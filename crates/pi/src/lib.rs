use rppal::gpio::{self, Gpio};
use sb_components_motor_shield_core::components as sbc_motor_shield;

#[derive(Debug)]
pub struct PwmOutputError;
impl embedded_hal::pwm::Error for PwmOutputError {
    fn kind(&self) -> embedded_hal::pwm::ErrorKind {
        embedded_hal::pwm::ErrorKind::Other
    }
}

pub struct PwmOutputWrapper {
    pin: gpio::OutputPin,
    /// is specified in hertz (Hz).
    pub frequency: f64,
}
impl embedded_hal::pwm::SetDutyCycle for PwmOutputWrapper {
    fn max_duty_cycle(&self) -> u16 {
        100
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        let duty_cycle: f64 = duty as f64 / self.max_duty_cycle() as f64;
        self.pin
            .set_pwm_frequency(self.frequency, duty_cycle)
            .map_err(|_| PwmOutputError)
    }
}
impl embedded_hal::pwm::ErrorType for PwmOutputWrapper {
    type Error = PwmOutputError;
}
impl PwmOutputWrapper {
    pub fn new(p: rppal::gpio::Pin, f: f64) -> Self {
        Self {
            pin: p.into_output_low(),
            frequency: f,
        }
    }
}

pub type RppalSbcBoard = sbc_motor_shield::MotorShield<
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

#[derive(Debug)]
pub enum CreateRpiError {
    Gpio(gpio::Error),
    Configuration(sbc_motor_shield::MotorShieldError),
}

    // !! why we're not using motor_driver_hal's Rppal builders and wrappers !!
    // ! the reason it's so hard to coax rppal to configure pwm for a particular pin is that pwm pins are defined by the rpi pwm overlay config files of the sysfs interface
    // https://www.kernel.org/doc/html/v5.10/driver-api/pwm.html#using-pwms-with-the-sysfs-interface
    // https://docs.golemparts.com/rppal/0.20.0/rppal/pwm/
    // meanwhile, rpi.gpio uses software pwm : https://pypi.org/project/RPi.GPIO/

impl From<gpio::Error> for CreateRpiError {
    fn from(error: gpio::Error) -> Self {
        Self::Gpio(error)
    }
}

pub fn create_rpi(gp: &Gpio) -> Result<RppalSbcBoard, CreateRpiError> {
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
        .with_sonic(gp.get(5)?.into_output(), gp.get(6)?.into_input())
        .with_lights(
            gp.get(26)?.into_output_low(), //f
            gp.get(13)?.into_output_low(), //b
            gp.get(19)?.into_output_low(), //l
            gp.get(16)?.into_output_low(), //r
        )
        .build()
        .map_err(CreateRpiError::Configuration)?;
    // ! alt sonic 38 in (or 8); 40 out (or 10)
    //       pins 8,10,38,40 are unused

    Ok(board)
}
