//! NOTE: Consider adopting L293X package or motor-driver-hal from crates.io

use embedded_hal::{
    digital::{self, Error},
    pwm,
};

#[deprecated]
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
}
#[deprecated]
pub enum MotorError {
    PwmError,
    PinError(digital::ErrorKind),
    IllegalPercentage(),
}
#[deprecated]
pub trait IMotor {
    /// Drive motor forward at given speed (0-100)
    fn forward(&mut self, percent: u8) -> Result<(), MotorError>;

    /// Drive motor backward at given speed
    fn backward(&mut self, speed: u8) -> Result<(), MotorError>;

    /// Stop motor
    fn stop(&mut self) -> Result<(), MotorError>;
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
        }
    }
}

impl<TFwdPin, TBakPin, TPwmPin> IMotor for MotorL293D<TFwdPin, TBakPin, TPwmPin>
where
    TFwdPin: digital::OutputPin,
    TBakPin: digital::OutputPin,
    TPwmPin: pwm::SetDutyCycle,
{
    /// Drive motor forward at given speed (0-65535, where 65535 is max)
    fn forward(&mut self, percent: u8) -> Result<(), MotorError> {
        if percent > 100 {
            return Err(MotorError::IllegalPercentage());
        }
        self.fwd_pin
            .set_high()
            .map_err(|e| MotorError::PinError(e.kind()))?;
        self.bak_pin
            .set_low()
            .map_err(|e| MotorError::PinError(e.kind()))?;

        match percent {
            x if x <= 100 => self.enable.set_duty_cycle_percent(percent),
            _ => self.enable.set_duty_cycle_fully_on(),
        }
        .map_err(|_pwm_err| MotorError::PwmError)?;
        Ok(())
    }

    /// Drive motor backward at given speed
    fn backward(&mut self, speed: u8) -> Result<(), MotorError> {
        if speed > 100 {
            return Err(MotorError::IllegalPercentage());
        }
        self.fwd_pin
            .set_low()
            .map_err(|e| MotorError::PinError(e.kind()))?;
        self.bak_pin
            .set_high()
            .map_err(|e| MotorError::PinError(e.kind()))?;
        self.enable
            .set_duty_cycle_percent(speed)
            .map_err(|_pwm_err| MotorError::PwmError)?;
        Ok(())
    }

    /// Stop motor
    fn stop(&mut self) -> Result<(), MotorError> {
        self.fwd_pin
            .set_low()
            .map_err(|e| MotorError::PinError(e.kind()))?;
        self.bak_pin
            .set_low()
            .map_err(|e| MotorError::PinError(e.kind()))?;
        self.enable
            .set_duty_cycle_fully_off()
            .map_err(|_pwm_err| MotorError::PwmError)?;
        Ok(())
    }
}
