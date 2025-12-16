//! NOTE: Consider adopting L293X package or motor-driver-hal from crates.io
// #![cfg(not(feature = "motor-driver-hal"))] // Use no_std if std feature is disabled
mod sbc_motor_hal {

    use embedded_hal::{
        digital::{self, Error},
        pwm,
    };
    use motor_driver_hal::MotorDirection;

    pub struct MotorL293D<TFwdPin, TBakPin, TPwmPin>
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
        pub enable: TPwmPin,
        direction: MotorDirection,
    }

    pub enum MotorError {
        PwmError,
        PinError(digital::ErrorKind),
        IllegalPercentage(),
        NotImplemented,
    }

    impl<TFwdPin, TBakPin, TPwmPin> MotorL293D<TFwdPin, TBakPin, TPwmPin>
    where
        TFwdPin: digital::OutputPin,
        TBakPin: digital::OutputPin,
        TPwmPin: pwm::SetDutyCycle,
    {
        /// Create a motor from three arbitrary pins
        pub fn new(fwd_pin: TFwdPin, bak_pin: TBakPin, enable_pin: TPwmPin) -> Self {
            Self {
                fwd_pin,
                bak_pin,
                enable: enable_pin,
                direction: MotorDirection::Forward,
            }
        }
        #[inline]
        fn pinset(a: &mut impl digital::OutputPin, d: bool) -> Result<(), MotorError> {
            if d { a.set_low() } else { a.set_high() }
                .map_err(|o| MotorError::PinError(o.kind()))?;
            Ok(())
        }
    }

    impl<TFwdPin, TBakPin, TPwmPin> motor_driver_hal::MotorDriver
        for MotorL293D<TFwdPin, TBakPin, TPwmPin>
    where
        TFwdPin: digital::OutputPin,
        TBakPin: digital::OutputPin,
        TPwmPin: pwm::SetDutyCycle,
    {
        type Error = MotorError;

        fn initialize(&mut self) -> Result<(), Self::Error> {
            Self::pinset(&mut self.bak_pin, false)?;
            self.bak_pin
                .set_low()
                .map_err(|o| MotorError::PinError(o.kind()))?;
            self.fwd_pin
                .set_low()
                .map_err(|o| MotorError::PinError(o.kind()))?;
            self.enable
                .set_duty_cycle_fully_off()
                .map_err(|o| MotorError::PwmError)?;
            Ok(())
        }

        fn set_speed(&mut self, speed: i16) -> Result<(), Self::Error> {
            self.enable
                .set_duty_cycle_fraction(speed as u16, 100)
                .map_err(|o| MotorError::PwmError)?;
            Ok(())
        }

        fn set_direction(&mut self, forward: bool) -> Result<(), Self::Error> {
            self.direction = match forward {
                true => motor_driver_hal::MotorDirection::Forward,
                _ => MotorDirection::Reverse,
            };
            Ok(())
        }

        fn stop(&mut self) -> Result<(), Self::Error> {
            // coasting is pwm off, fwd/bak any differing state
            self.direction = MotorDirection::Coast;
            self.enable
                .set_duty_cycle_fully_off()
                .map_err(|o| MotorError::PwmError)?;
            Ok(())
        }

        fn brake(&mut self) -> Result<(), Self::Error> {
            // enable high, fwd/bak any same state
            self.direction = MotorDirection::Brake;

            self.enable
                .set_duty_cycle_percent(50)
                .map_err(|o| MotorError::PwmError)?;
            Self::pinset(&mut self.bak_pin, false)?;
            self.bak_pin
                .set_low()
                .map_err(|o| MotorError::PinError(o.kind()))?;
            self.bak_pin
                .set_low()
                .map_err(|o| MotorError::PinError(o.kind()))?;
            Ok(())
        }

        fn enable(&mut self) -> Result<(), Self::Error> {
            todo!()
        }

        fn disable(&mut self) -> Result<(), Self::Error> {
            todo!()
        }

        fn get_direction(&self) -> Result<bool, Self::Error> {
            Ok(MotorDirection::Forward == self.direction)
        }

        fn check_ppr(&mut self) -> Result<(), Self::Error> {
            Err(MotorError::NotImplemented)
        }
        fn set_ppr(&mut self, ppr: i16) -> Result<bool, Self::Error> {
            Err(MotorError::NotImplemented)
        }

        fn get_speed(&self) -> Result<i16, Self::Error> {
            Err(MotorError::NotImplemented)
        }
        fn get_current(&self) -> Result<f32, Self::Error> {
            Err(MotorError::NotImplemented)
        }

        fn get_voltage(&self) -> Result<f32, Self::Error> {
            Err(MotorError::NotImplemented)
        }

        fn get_temperature(&self) -> Result<f32, Self::Error> {
            Err(MotorError::NotImplemented)
        }

        fn get_fault_status(&self) -> Result<u8, Self::Error> {
            todo!()
        }
        // /// Drive motor forward at given speed (0-65535, where 65535 is max)
        // fn forward(&mut self, percent: u8) -> Result<(), MotorError> {
        //     if percent > 100 {
        //         return Err(MotorError::IllegalPercentage());
        //     }
        //     self.fwd_pin
        //         .set_high()
        //         .map_err(|e| MotorError::PinError(e.kind()))?;
        //     self.bak_pin
        //         .set_low()
        //         .map_err(|e| MotorError::PinError(e.kind()))?;

        //     match percent {
        //         x if x <= 100 => self.enable.set_duty_cycle_percent(percent),
        //         _ => self.enable.set_duty_cycle_fully_on(),
        //     }
        //     .map_err(|_pwm_err| MotorError::PwmError)?;
        //     Ok(())
        // }

        // /// Drive motor backward at given speed
        // fn backward(&mut self, speed: u8) -> Result<(), MotorError> {
        //     if speed > 100 {
        //         return Err(MotorError::IllegalPercentage());
        //     }
        //     self.fwd_pin
        //         .set_low()
        //         .map_err(|e| MotorError::PinError(e.kind()))?;
        //     self.bak_pin
        //         .set_high()
        //         .map_err(|e| MotorError::PinError(e.kind()))?;
        //     self.enable
        //         .set_duty_cycle_percent(speed)
        //         .map_err(|_pwm_err| MotorError::PwmError)?;
        //     Ok(())
        // }

        // /// Stop motor
        // fn stop(&mut self) -> Result<(), MotorError> {
        //     self.fwd_pin
        //         .set_low()
        //         .map_err(|e| MotorError::PinError(e.kind()))?;
        //     self.bak_pin
        //         .set_low()
        //         .map_err(|e| MotorError::PinError(e.kind()))?;
        //     self.enable
        //         .set_duty_cycle_fully_off()
        //         .map_err(|_pwm_err| MotorError::PwmError)?;
        //     Ok(())
        // }
    }
}
