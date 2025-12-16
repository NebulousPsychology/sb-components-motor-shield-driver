//! NOTE: Consider adopting L293X package or motor-driver-hal from crates.io
// #![cfg(not(feature = "motor-driver-hal"))] // Use no_std if std feature is disabled
mod sbc_motor_hal {
    use embedded_hal::{
        digital::{self},
        pwm::{self},
    };
    use motor_driver_hal::{HBridgeMotorDriver, MotorDirection, MotorDriver, MotorDriverError};

    /// Provide a MotorDriver for the L293x's "Single EN PWM/Dual Direction Pin" arrangement
    ///
    /// > [!WARNING]
    /// > There are deep electrical reasons why "Single EN Pin/Dual Direction PWM"
    /// > is better at handling state changes while braking.
    /// > - jitter
    /// > - torque ripple
    /// > - back-EMF
    /// > - impedence/current spikes
    /// >
    /// > For this reason, the `brake` method is discouraged.
    ///
    /// In any event, the official code for the sb components shield
    /// endorses the L293x control patterns, and so
    pub struct MotorL293x<TFwdPin, TBakPin, TPwmPin>
    where
        TFwdPin: digital::OutputPin,
        TBakPin: digital::OutputPin,
        TPwmPin: pwm::SetDutyCycle,
    {
        /// Direction pin 1 (IN1 on L293D)
        pub fwd_pin: TFwdPin,
        /// Direction pin 2 (IN2 on L293D)
        pub bak_pin: TBakPin,
        /// PWM-capable pin for enable (EN on L293D)
        pub en_pwm: TPwmPin,
        max_duty: u16,
        current_speed: i16,
        direction: MotorDirection,
        initialized: bool,
    }

    impl<TFwdPin: digital::OutputPin, TBakPin: digital::OutputPin, TPwmPin: pwm::SetDutyCycle>
        MotorL293x<TFwdPin, TBakPin, TPwmPin>
    {
        const DEFAULT_MAX_DUTY_CYCLE: u16 = 100;
        /// Create a motor from three arbitrary pins
        pub fn new(
            fwd_pin: TFwdPin,
            bak_pin: TBakPin,
            enable_pin: TPwmPin,
            duty: Option<u16>,
        ) -> Self {
            Self {
                fwd_pin,
                bak_pin,
                en_pwm: enable_pin,
                max_duty: match duty {
                    None => Self::DEFAULT_MAX_DUTY_CYCLE,
                    Some(d) if d > 0 => d,
                    _ => Self::DEFAULT_MAX_DUTY_CYCLE, // Alternatively, error
                },
                current_speed: 0,
                direction: MotorDirection::Forward,
                initialized: false,
            }
        }

        /// Convenience for setting a digital output, and remapping any error
        #[inline]
        fn pinset(pin: &mut impl digital::OutputPin, state: bool) -> Result<(), MotorDriverError> {
            return if state { pin.set_high() } else { pin.set_low() }
                .map_err(|o| MotorDriverError::GpioError);
        }

        /// Convenience for checking the init flag
        #[inline]
        fn validate_initialize(&self) -> Result<(), MotorDriverError> {
            if self.initialized {
                Ok(())
            } else {
                Err(MotorDriverError::NotInitialized)
            }
        }

        ///Convenience for setting pwm throttle, as a fraction of the max_duty
        /// (like MotorDriverWrapper::control_enable, but controls both directions individually)
        #[inline]
        fn set_pwm(&mut self, duty: u16) -> Result<(), MotorDriverError> {
            assert_ne!(0, self.max_duty);
            self.en_pwm
                .set_duty_cycle_fraction(duty, self.max_duty)
                .map_err(|_| MotorDriverError::PwmError)
        }

        ///Convenience for setting control pins
        /// (like MotorDriverWrapper::control_enable, but controls both directions individually)
        #[inline]
        fn set_pins(&mut self, fore: bool, back: bool) -> Result<(), MotorDriverError> {
            Self::pinset(&mut self.bak_pin, back)?;
            Self::pinset(&mut self.fwd_pin, fore)?;
            Ok(())
        }

        /// Called as the final step in any state change to direction or throttle.
        /// As in MotorDriverWrapper::update_pwm, interpret the speed and direction and set pins.
        /// However, here the L293x patterns are used.
        fn update_pwm(&mut self) -> Result<(), MotorDriverError> {
            let duty = self.current_speed.unsigned_abs().min(self.max_duty);

            // original would hold the EN pin, and set one pwm 0 while providing trottle with the other

            // here, setspeed will have assigned cspeed, and dir:Forward/Reverse if nonzero, UNCHANGED if zero
            // here, stop sets speed 0, dir:coast  (duty: 0, pins opposed)
            // here, brake sets speed 0, dir:brake (duty: max, pins together)

            // init sets speed 0 | duty 0,0 | pins: 0,0

            let (newduty, ctrl) = match self.direction {
                MotorDirection::Brake => (self.max_duty, Some((false, false))),
                MotorDirection::Coast => (0, None),
                MotorDirection::Forward => (duty, Some((true, false))),
                MotorDirection::Reverse => (duty, Some((false, true))),
            };
            // apply the settings
            self.set_pwm(newduty)?;
            if let Some((f, b)) = ctrl {
                self.set_pins(f, b)?;
            };
            Ok(())
        }
    }

    impl<TFwdPin, TBakPin, TPwmPin> motor_driver_hal::MotorDriver
        for MotorL293x<TFwdPin, TBakPin, TPwmPin>
    where
        TFwdPin: digital::OutputPin,
        TBakPin: digital::OutputPin,
        TPwmPin: pwm::SetDutyCycle,
    {
        type Error = MotorDriverError;

        fn initialize(&mut self) -> Result<(), Self::Error> {
            // in motordriverwrapper:
            // control_enable(false)
            //      set dir pins: double low
            // dutycycle zero
            self.set_pins(false, false)?; // fighting
            self.set_pwm(0)?;
            self.initialized = true;
            Ok(())
        }

        fn set_speed(&mut self, speed: i16) -> Result<(), Self::Error> {
            self.validate_initialize()?;

            if speed.unsigned_abs() > self.max_duty {
                return Err(MotorDriverError::InvalidSpeed);
            }

            self.current_speed = speed;
            self.direction = if speed < 0 {
                MotorDirection::Reverse
            } else if speed > 0 {
                MotorDirection::Forward
            } else {
                self.direction
            };

            return self.update_pwm();
        }

        fn set_direction(&mut self, forward: bool) -> Result<(), Self::Error> {
            self.validate_initialize()?;
            self.direction = match forward {
                true => MotorDirection::Forward,
                false => MotorDirection::Reverse,
            };
            return self.update_pwm();
        }

        fn stop(&mut self) -> Result<(), Self::Error> {
            self.validate_initialize()?;
            // coasting is pwm off, fwd/bak any differing state
            self.current_speed = 0;
            self.direction = MotorDirection::Coast;
            return self.update_pwm();
        }

        fn brake(&mut self) -> Result<(), Self::Error> {
            self.validate_initialize()?;
            // enable high, fwd/bak any same state
            self.current_speed = 0;
            self.direction = MotorDirection::Brake;
            return self.update_pwm();
        }

        fn enable(&mut self) -> Result<(), Self::Error> {
            self.validate_initialize()?;
            return self.set_pins(true, true);
            // TODO: both-high will produce braking during high pwm
        }

        fn disable(&mut self) -> Result<(), Self::Error> {
            self.validate_initialize()?;
            return self.set_pins(false, false);
        }

        fn get_speed(&self) -> Result<i16, Self::Error> {
            self.validate_initialize()?;
            Ok(self.current_speed)
        }
        fn get_direction(&self) -> Result<bool, Self::Error> {
            self.validate_initialize()?;
            Ok(MotorDirection::Forward == self.direction)
        }

        // following methods are not supported through the shield

        fn check_ppr(&mut self) -> Result<(), Self::Error> {
            Err(MotorDriverError::HardwareFault) // MotorDriverWrapper does, we don't
        }
        fn set_ppr(&mut self, ppr: i16) -> Result<bool, Self::Error> {
            Err(MotorDriverError::HardwareFault) // MotorDriverWrapper does, we don't
        }
        fn get_current(&self) -> Result<f32, Self::Error> {
            Err(MotorDriverError::HardwareFault)
        }
        fn get_voltage(&self) -> Result<f32, Self::Error> {
            Err(MotorDriverError::HardwareFault)
        }
        fn get_temperature(&self) -> Result<f32, Self::Error> {
            Err(MotorDriverError::HardwareFault)
        }
        fn get_fault_status(&self) -> Result<u8, Self::Error> {
            self.validate_initialize()?;
            Ok(0)
        }
    }
}
