#![allow(dead_code, unused_variables)]

use embedded_hal::{digital, pwm};

mod light;
pub mod motor;
pub mod sensor;

pub struct LightArray<
    TLightFore: digital::OutputPin,
    TLightBack: digital::OutputPin,
    TLightLeft: digital::OutputPin,
    TLightRight: digital::OutputPin,
> {
    pub fore: TLightFore,
    pub back: TLightBack,
    pub left: TLightLeft,
    pub right: TLightRight,
}

pub struct MotorArray<
    TM1F: digital::OutputPin,
    TM1B: digital::OutputPin,
    TM1E: pwm::SetDutyCycle,
    TM2F: digital::OutputPin,
    TM2B: digital::OutputPin,
    TM2E: pwm::SetDutyCycle,
    TM3F: digital::OutputPin,
    TM3B: digital::OutputPin,
    TM3E: pwm::SetDutyCycle,
    TM4F: digital::OutputPin,
    TM4B: digital::OutputPin,
    TM4E: pwm::SetDutyCycle,
> {
    pub motor1: motor_driver_hal::MotorDriverWrapper<TM1F, TM1B, TM1E, ()>,
    pub motor2: motor_driver_hal::MotorDriverWrapper<TM2F, TM2B, TM2E, ()>,
    pub motor3: motor_driver_hal::MotorDriverWrapper<TM3F, TM3B, TM3E, ()>,
    pub motor4: motor_driver_hal::MotorDriverWrapper<TM4F, TM4B, TM4E, ()>,
}

pub struct MotorShield<
    TIR1: digital::InputPin,
    TIR2: digital::InputPin,
    TSonicEcho: digital::InputPin,
    TSonicTrig: digital::OutputPin,
    TM1F: digital::OutputPin,
    TM1B: digital::OutputPin,
    TM1E: pwm::SetDutyCycle,
    TM2F: digital::OutputPin,
    TM2B: digital::OutputPin,
    TM2E: pwm::SetDutyCycle,
    TM3F: digital::OutputPin,
    TM3B: digital::OutputPin,
    TM3E: pwm::SetDutyCycle,
    TM4F: digital::OutputPin,
    TM4B: digital::OutputPin,
    TM4E: pwm::SetDutyCycle,
    TLightFore: digital::OutputPin,
    TLightBack: digital::OutputPin,
    TLightLeft: digital::OutputPin,
    TLightRight: digital::OutputPin,
> {
    pub sensor_ir1: sensor::infrared::SensorIR<TIR1>,
    pub sensor_ir2: sensor::infrared::SensorIR<TIR2>,
    pub sensor_sonic: sensor::ultrasonic::Sonar<TSonicEcho, TSonicTrig>,
    pub motors: MotorArray<TM1F, TM1B, TM1E, TM2F, TM2B, TM2E, TM3F, TM3B, TM3E, TM4F, TM4B, TM4E>,
    pub lights: LightArray<TLightFore, TLightBack, TLightLeft, TLightRight>,
}

impl<
        TIR1: digital::InputPin,
        TIR2: digital::InputPin,
        TSonicEcho: digital::InputPin,
        TSonicTrig: digital::OutputPin,
        // TSonicEcho2: digital::InputPin,
        // TSonicTrig2: digital::OutputPin,
        TM1F: digital::OutputPin,
        TM1B: digital::OutputPin,
        TM1E: pwm::SetDutyCycle,
        TM2F: digital::OutputPin,
        TM2B: digital::OutputPin,
        TM2E: pwm::SetDutyCycle,
        TM3F: digital::OutputPin,
        TM3B: digital::OutputPin,
        TM3E: pwm::SetDutyCycle,
        TM4F: digital::OutputPin,
        TM4B: digital::OutputPin,
        TM4E: pwm::SetDutyCycle,
        TLightFore: digital::OutputPin,
        TLightBack: digital::OutputPin,
        TLightLeft: digital::OutputPin,
        TLightRight: digital::OutputPin,
    >
    MotorShield<
        TIR1,
        TIR2,
        TSonicEcho,
        TSonicTrig,
        TM1F,
        TM1B,
        TM1E,
        TM2F,
        TM2B,
        TM2E,
        TM3F,
        TM3B,
        TM3E,
        TM4F,
        TM4B,
        TM4E,
        TLightFore,
        TLightBack,
        TLightLeft,
        TLightRight,
    >
{
    pub fn new(
        ir1: sensor::infrared::SensorIR<TIR1>,
        ir2: sensor::infrared::SensorIR<TIR2>,
        sonic: sensor::ultrasonic::Sonar<TSonicEcho, TSonicTrig>,
        motors: MotorArray<TM1F, TM1B, TM1E, TM2F, TM2B, TM2E, TM3F, TM3B, TM3E, TM4F, TM4B, TM4E>,
        lights: LightArray<TLightFore, TLightBack, TLightLeft, TLightRight>,
    ) -> Self {
        Self {
            sensor_ir1: ir1,
            sensor_ir2: ir2,
            sensor_sonic: sonic,
            // sensor_sonic_x: sensor::ultrasonic::Sonar::new(),
            motors,
            lights,
        }
    }
}

pub struct MotorShieldConfigurationBuilder<
    TIR1: digital::InputPin,
    TIR2: digital::InputPin,
    TSonicEcho: digital::InputPin,
    TSonicTrig: digital::OutputPin,
    TM1F: digital::OutputPin,
    TM1B: digital::OutputPin,
    TM1E: pwm::SetDutyCycle,
    TM2F: digital::OutputPin,
    TM2B: digital::OutputPin,
    TM2E: pwm::SetDutyCycle,
    TM3F: digital::OutputPin,
    TM3B: digital::OutputPin,
    TM3E: pwm::SetDutyCycle,
    TM4F: digital::OutputPin,
    TM4B: digital::OutputPin,
    TM4E: pwm::SetDutyCycle,
    TLightFore: digital::OutputPin,
    TLightBack: digital::OutputPin,
    TLightLeft: digital::OutputPin,
    TLightRight: digital::OutputPin,
> {
    pub sensor_ir1: Option<sensor::infrared::SensorIR<TIR1>>,
    pub sensor_ir2: Option<sensor::infrared::SensorIR<TIR2>>,
    pub sensor_sonic: Option<sensor::ultrasonic::Sonar<TSonicEcho, TSonicTrig>>,
    // pub motors:
    //     Option<MotorArray<TM1F, TM1B, TM1E, TM2F, TM2B, TM2E, TM3F, TM3B, TM3E, TM4F, TM4B, TM4E>>,
    pub m1: Option<motor_driver_hal::MotorDriverWrapper<TM1F, TM1B, TM1E, ()>>,
    pub m2: Option<motor_driver_hal::MotorDriverWrapper<TM2F, TM2B, TM2E, ()>>,
    pub m3: Option<motor_driver_hal::MotorDriverWrapper<TM3F, TM3B, TM3E, ()>>,
    pub m4: Option<motor_driver_hal::MotorDriverWrapper<TM4F, TM4B, TM4E, ()>>,
    pub lights: Option<LightArray<TLightFore, TLightBack, TLightLeft, TLightRight>>,
}

#[derive(Debug)]
pub enum MotorShieldError {
    Unspecified,
    ConfigurationInvalid,
    /// Fixed-size list of missing configuration field names.
    MissingFields(MissingFieldsError),
}

/// Small fixed-size container for missing field names.
/// Avoids heap allocation so it's compatible with `no_std` crates.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MissingFieldsError {
    pub names: [&'static str; MISSING_FIELDS_ERROR_CAPACITY],
    pub count: usize,
}
const MISSING_FIELDS_ERROR_CAPACITY: usize = 8;
impl MissingFieldsError {
    pub const fn new() -> Self {
        Self {
            names: [""; MISSING_FIELDS_ERROR_CAPACITY],
            count: 0,
        }
    }

    pub fn push(&mut self, name: &'static str) {
        if self.count < MISSING_FIELDS_ERROR_CAPACITY {
            self.names[self.count] = name;
        } else if self.count == MISSING_FIELDS_ERROR_CAPACITY {
            self.names[MISSING_FIELDS_ERROR_CAPACITY - 1] = "...";
        }
        self.count += 1;
    }

    pub fn push_if_none<T>(&mut self, value: &Option<T>, name: &'static str) {
        if value.is_none() {
            self.push(name);
        }
    }

    pub fn as_slice(&self) -> &[&'static str] {
        if self.count < MISSING_FIELDS_ERROR_CAPACITY {
            &self.names[..self.count]
        } else {
            &self.names[..MISSING_FIELDS_ERROR_CAPACITY]
        }
    }
    pub fn check_validity(self) -> Result<(), MotorShieldError> {
        match self.count {
            0 => Ok(()),
            _ => Err(MotorShieldError::MissingFields(self)),
        }
    }
}

impl<
        TIR1: digital::InputPin,
        TIR2: digital::InputPin,
        TSonicEcho: digital::InputPin,
        TSonicTrig: digital::OutputPin,
        TM1F: digital::OutputPin,
        TM1B: digital::OutputPin,
        TM1E: pwm::SetDutyCycle,
        TM2F: digital::OutputPin,
        TM2B: digital::OutputPin,
        TM2E: pwm::SetDutyCycle,
        TM3F: digital::OutputPin,
        TM3B: digital::OutputPin,
        TM3E: pwm::SetDutyCycle,
        TM4F: digital::OutputPin,
        TM4B: digital::OutputPin,
        TM4E: pwm::SetDutyCycle,
        TLightFore: digital::OutputPin,
        TLightBack: digital::OutputPin,
        TLightLeft: digital::OutputPin,
        TLightRight: digital::OutputPin,
    >
    MotorShieldConfigurationBuilder<
        TIR1,
        TIR2,
        TSonicEcho,
        TSonicTrig,
        TM1F,
        TM1B,
        TM1E,
        TM2F,
        TM2B,
        TM2E,
        TM3F,
        TM3B,
        TM3E,
        TM4F,
        TM4B,
        TM4E,
        TLightFore,
        TLightBack,
        TLightLeft,
        TLightRight,
    >
{
    pub fn new() -> Self {
        Self {
            lights: None,
            sensor_ir1: None,
            sensor_ir2: None,
            sensor_sonic: None,
            m1: None,
            m2: None,
            m3: None,
            m4: None,
        }
    }
    fn create_motor<F, B, E>(
        p_f: F,
        p_b: B,
        p_e: E,
        duty: Option<u16>,
    ) -> motor_driver_hal::MotorDriverWrapper<F, B, E, ()>
    where
        F: digital::OutputPin,
        B: digital::OutputPin,
        E: pwm::SetDutyCycle,
    {
        return motor_driver_hal::MotorDriverBuilder::new()
            .with_dual_enable(p_f, p_b)
            .with_pwm_channels(motor_driver_hal::wrapper::PwmChannels::Single(p_e))
            .with_max_duty(duty.unwrap_or_else(|| 1000))
            .build();
    }

    pub fn with_ir1(mut self, t: TIR1) -> Self {
        self.sensor_ir1 = Some(sensor::infrared::SensorIR::new(t));
        self
    }
    pub fn with_ir2(mut self, t: TIR2) -> Self {
        self.sensor_ir2 = Some(sensor::infrared::SensorIR::new(t));
        self
    }

    pub fn with_sonic(mut self, trigger_out: TSonicTrig, echo_in: TSonicEcho) -> Self {
        self.sensor_sonic = Some(sensor::ultrasonic::Sonar::new(trigger_out, echo_in));
        self
    }

    pub fn with_lights(
        mut self,
        f: TLightFore,
        b: TLightBack,
        l: TLightLeft,
        r: TLightRight,
    ) -> Self {
        self.lights = Some(LightArray {
            fore: f,
            back: b,
            left: l,
            right: r,
        });
        self
    }

    pub fn with_motor1(mut self, p_f: TM1F, p_b: TM1B, p_e: TM1E, duty: Option<u16>) -> Self {
        self.m1 = Some(Self::create_motor(p_f, p_b, p_e, duty));
        return self;
    }

    pub fn with_motor2(mut self, p_f: TM2F, p_b: TM2B, p_e: TM2E, duty: Option<u16>) -> Self {
        self.m2 = Some(Self::create_motor(p_f, p_b, p_e, duty));
        return self;
    }

    pub fn with_motor3(mut self, p_f: TM3F, p_b: TM3B, p_e: TM3E, duty: Option<u16>) -> Self {
        self.m3 = Some(Self::create_motor(p_f, p_b, p_e, duty));
        return self;
    }

    pub fn with_motor4(self, p_f: TM4F, p_b: TM4B, p_e: TM4E, duty: Option<u16>) -> Self {
        return self.with_motor4_driver(Self::create_motor(p_f, p_b, p_e, duty));
    }

    /// (allows direct configuration from RppalMotorDriverBuilder)
    pub fn with_motor4_driver(
        mut self,
        driver: motor_driver_hal::MotorDriverWrapper<TM4F, TM4B, TM4E, ()>,
    ) -> Self {
        self.m4 = Some(driver);
        return self;
    }

    /// perform final checks and return a descriptive MissingFields error
    fn validity(&self) -> Result<(), MotorShieldError> {
        let mut missing = MissingFieldsError::new();
        missing.push_if_none(&self.sensor_ir1, "sensor_ir1");
        missing.push_if_none(&self.sensor_ir2, "sensor_ir2");
        missing.push_if_none(&self.sensor_sonic, "sonic");
        missing.push_if_none(&self.lights, "lights");

        missing.push_if_none(&self.m1, "motor 1");
        missing.push_if_none(&self.m2, "motor 2");
        missing.push_if_none(&self.m3, "motor 3");
        missing.push_if_none(&self.m4, "motor 4");
        missing.check_validity()
    }

    pub fn build(
        self,
    ) -> Result<
        MotorShield<
            TIR1,
            TIR2,
            TSonicEcho,
            TSonicTrig,
            TM1F,
            TM1B,
            TM1E,
            TM2F,
            TM2B,
            TM2E,
            TM3F,
            TM3B,
            TM3E,
            TM4F,
            TM4B,
            TM4E,
            TLightFore,
            TLightBack,
            TLightLeft,
            TLightRight,
        >,
        MotorShieldError,
    > {
        // perform final checks and return a descriptive MissingFields error
        self.validity()?;

        Ok(MotorShield {
            sensor_ir1: self.sensor_ir1.unwrap(),
            sensor_ir2: self.sensor_ir2.unwrap(),
            sensor_sonic: self.sensor_sonic.unwrap(),
            motors: MotorArray {
                motor1: self.m1.unwrap(),
                motor2: self.m2.unwrap(),
                motor3: self.m3.unwrap(),
                motor4: self.m4.unwrap(),
            },
            lights: self.lights.unwrap(),
        })
    }

    pub fn build_and_init(
        self,
    ) -> Result<
        MotorShield<
            TIR1,
            TIR2,
            TSonicEcho,
            TSonicTrig,
            TM1F,
            TM1B,
            TM1E,
            TM2F,
            TM2B,
            TM2E,
            TM3F,
            TM3B,
            TM3E,
            TM4F,
            TM4B,
            TM4E,
            TLightFore,
            TLightBack,
            TLightLeft,
            TLightRight,
        >,
        MotorShieldError,
    > {
        let mut board = self.build()?;
        // &(&((&board).motors).motor1).
        // let &mut m1: motor_driver_hal::MotorDriverWrapper<TM1F, TM1B, TM1E, ()> =
        //     &mut board.motors.motor1;
        return Ok(board);
    }
}
