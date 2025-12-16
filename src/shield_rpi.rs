// #![cfg_attr(not(feature = "std"), no_std)] // Use no_std if std feature is disabled

#[cfg(all(
    feature = "sbc-rpi",
    feature = "std",
    not(feature = "sbc-pico"),
    any(target_arch = "arm", target_arch = "aarch64"),
))]
mod shield_rpi {
    //
    use crate::sbc_motor_shield;
    use fugit::RateExtU32;
    use rppal::gpio::{self, Gpio};

    #[derive(Debug)]
    struct POWError {}
    impl embedded_hal::pwm::Error for POWError {
        fn kind(&self) -> embedded_hal::pwm::ErrorKind {
            embedded_hal::pwm::ErrorKind::Other
        }
    }

    struct PwmOutputWrapper {
        pub pin: gpio::OutputPin,
        /// is specified in hertz (Hz).
        pub frequency: f64,
    }
    impl embedded_hal::pwm::SetDutyCycle for PwmOutputWrapper {
        fn max_duty_cycle(&self) -> u16 {
            100
        }

        fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
            let duty_cycle: f64 = duty as f64 / self.max_duty_cycle() as f64;
            self.pin.set_pwm_frequency(self.frequency, duty_cycle);
            Ok(())
        }
    }
    impl embedded_hal::pwm::ErrorType for PwmOutputWrapper {
        type Error = POWError;
    }
    impl PwmOutputWrapper {
        pub fn new(p: rppal::gpio::Pin, f: f64) -> Self {
            Self {
                pin: p.into_output_low(),
                frequency: f,
            }
        }
    }

    type RppalSbcBoard = crate::sbc_motor_shield::MotorShield<
        gpio::InputPin,
        gpio::InputPin,
        //IR1, TIR2,
        gpio::InputPin,
        gpio::OutputPin,
        //TSonicEcho, TSonicTrig,
        gpio::OutputPin,
        gpio::OutputPin,
        PwmOutputWrapper, //pwm
        // TM1F, TM1B, TM1E,
        gpio::OutputPin,
        gpio::OutputPin,
        PwmOutputWrapper, //pwm
        // TM2F, TM2B, TM2E,
        gpio::OutputPin,
        gpio::OutputPin,
        PwmOutputWrapper, //pwm
        //TM3F, TM3B, TM3E,
        gpio::OutputPin,
        gpio::OutputPin,
        PwmOutputWrapper, // pwm
        // TM4F, TM4B, TM4E,
        gpio::OutputPin,
        gpio::OutputPin,
        gpio::OutputPin,
        gpio::OutputPin,
        // TLightFore, TLightBack, TLightLeft, TLightRight>
    >;

    // /// define setdutycycle for all software output pins
    // impl embedded_hal::pwm::SetDutyCycle for gpio::OutputPin {
    //     fn max_duty_cycle(&self) -> u16 {
    //         todo!()
    //     }

    //     fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
    //         // self.set_pwm(period, pulse_width);
    //         self.set_pwm_frequency(frequency, duty_cycle)
    //     }
    // }

    pub fn create_rpi(gp: &Gpio) -> Result<RppalSbcBoard, rppal::gpio::Error> {
        let motor_frequency: f64 = 50.0;
        let max_duty: u16 = 100; // todo: confirm actual cycle
                                 // select pin/channels, according to rppal docs
        #[cfg(feature = "rp5")]
        let channels = (); //12,13,18,19
        #[cfg(not(feature = "rp5"))]
        let channels = (); // pwm0=12/18, pwm1=13/19
                           // ! pwm pins (gpio.board) according to https://github.com/sbcshop/MotorShield/blob/master/PiMotor.py: 11,22,19,32(phys)

        let x = gp.get(12).unwrap().into_output();
        let (period, pulse_width) = (1u32, 2u32);
        // x.set_pwm(period.millis(), pulse_width.millis()).unwrap();

        let p0 = rppal::pwm::Pwm::new(rppal::pwm::Channel::Pwm0).unwrap();
        let pwr = motor_driver_hal::PwmWrapper::new(p0, 100);

        // !! why we're not using motor_driver_hal's Rppal builders and wrappers !!
        // ! the reason it's so hard to coax rppal to configure pwm for a particular pin is that pwm pins are defined by the rpi pwm overlay config files of the sysfs interface
        // https://www.kernel.org/doc/html/v5.10/driver-api/pwm.html#using-pwms-with-the-sysfs-interface
        // https://docs.golemparts.com/rppal/0.20.0/rppal/pwm/
        // meanwhile, rpi.gpio uses software pwm : https://pypi.org/project/RPi.GPIO/

        // TODO: streamline the api around duty and frequency
        let mfreq: f64 = 100.0;
        let duty = Some(100);
        let board: RppalSbcBoard = sbc_motor_shield::MotorShieldConfigurationBuilder::new()
            .with_motor1(
                gp.get(22)?.into_output_low(),
                gp.get(27)?.into_output_low(),
                PwmOutputWrapper {
                    pin: gp.get(17)?.into_output_low(),
                    frequency: mfreq,
                },
                duty,
            )
            .with_motor2(
                gp.get(23)?.into_output(),
                gp.get(24)?.into_output(),
                PwmOutputWrapper::new(gp.get(25)?, mfreq),
                duty,
            )
            .with_motor3(
                gp.get(9)?.into_output(),
                gp.get(11)?.into_output(),
                PwmOutputWrapper::new(gp.get(10)?, mfreq),
                duty,
            )
            .with_motor4(
                gp.get(8)?.into_output(),
                gp.get(7)?.into_output(),
                PwmOutputWrapper::new(gp.get(12)?, mfreq),
                duty,
            )
            .with_ir1(gp.get(4)?.into_input_pullup())
            .with_ir2(gp.get(18)?.into_input_pullup())
            .with_sonic(
                gp.get(5).unwrap().into_output(),
                gp.get(6).unwrap().into_input(),
            )
            .with_lights(
                gp.get(26)?.into_output_low(), //f
                gp.get(13)?.into_output_low(), //b
                gp.get(19)?.into_output_low(), //l
                gp.get(16)?.into_output_low(), //r
            )
            .build()
            .map_err(|e| rppal::gpio::Error::UnknownModel)?;
        // ! alt sonic 38 in (or 8); 40 out (or 10)
        //       pins 8,10,38,40 are unused

        Ok(board)
    }
}
