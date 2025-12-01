#![allow(dead_code, unused_variables)]

use embedded_hal::{digital, pwm};

mod light;
pub mod motor;
mod sensor;

/// Macro to generate motor type aliases for a given board
/// Usage: define_motor_types!(pico, 1, 2, 0, a);
/// Expands to: type PicoMotor1 = MotorL293D<Pin<Gpio1, ...>, Pin<Gpio2, ...>, Channel<Pwm0, A>>;
#[cfg(feature = "rp-pico")]
#[macro_export]
macro_rules! define_motor_types {
    (
        $board:ident,
        $($motor_num:expr => ($fwd_gpio:expr, $bak_gpio:expr, $pwm_slice_idx:expr, $pwm_ch:ident)),+
        $(,)?
    ) => {
        $(
            paste::paste! {
                pub type [<SbcMotor $motor_num>]= motor_driver_hal::MotorDriverWrapper<
                    rp_pico::hal::gpio::Pin<
                        rp_pico::hal::gpio::bank0::[<Gpio $fwd_gpio>],
                        rp_pico::hal::gpio::FunctionSioOutput,
                        rp_pico::hal::gpio::PullDown,
                    >,
                    rp_pico::hal::gpio::Pin<
                        rp_pico::hal::gpio::bank0::[<Gpio $bak_gpio>],
                        rp_pico::hal::gpio::FunctionSioOutput,
                        rp_pico::hal::gpio::PullDown,
                    >,
                    rp_pico::hal::pwm::Channel<
                        rp_pico::hal::pwm::Slice<
                            rp_pico::hal::pwm::[<Pwm $pwm_slice_idx>],
                            rp_pico::hal::pwm::FreeRunning
                            >,
                        rp_pico::hal::pwm::[<$pwm_ch:upper>],
                    >, ()>;
            }

            paste::paste! {
                pub type [<$board:camel Motor $motor_num>] = motor::MotorL293D<
                    rp_pico::hal::gpio::Pin<
                        rp_pico::hal::gpio::bank0::[<Gpio $fwd_gpio>],
                        rp_pico::hal::gpio::FunctionSioOutput,
                        rp_pico::hal::gpio::PullDown,
                    >,
                    rp_pico::hal::gpio::Pin<
                        rp_pico::hal::gpio::bank0::[<Gpio $bak_gpio>],
                        rp_pico::hal::gpio::FunctionSioOutput,
                        rp_pico::hal::gpio::PullDown,
                    >,
                    rp_pico::hal::pwm::Channel<
                        rp_pico::hal::pwm::Slice<
                            rp_pico::hal::pwm::[<Pwm $pwm_slice_idx>],
                            rp_pico::hal::pwm::FreeRunning
                            >,
                        rp_pico::hal::pwm::[<$pwm_ch:upper>],
                    >,
                >;
            }

        )+

        paste::paste! {
            /// FIXME: rp_pico implies $board=pico
            // #[cfg(feature = "board-pico")]
            pub fn [<setup_ $board:lower _motors>](
                pins: rp_pico::Pins,
                pwm_slices: rp_pico::hal::pwm::Slices,
            ) -> (
                $([<$board:camel Motor $motor_num>],)+
            ) {
                (
                    $(
                        paste::paste! {
                            motor::MotorL293D::new(
                                pins.[<gpio $fwd_gpio>].into_push_pull_output(),
                                pins.[<gpio $bak_gpio>].into_push_pull_output(),
                                pwm_slices.[<pwm $pwm_slice_idx>].[<channel_ $pwm_ch:lower>],
                            )
                        }
                    ),+
                )
            }
        }
    };
}

// Use the macro to define all motor types for Pico board:
// m => (fwd,bak, pwm_id, pwm_ch)
#[cfg(feature = "rp-pico")]
define_motor_types!(pico,
    1 => (0, 1, 0, a),
    2 => (2, 3, 0, b),
    3 => (4, 5, 1, a),
    4 => (6, 7, 1, b)
);

#[cfg(feature = "rp-pico")]
#[macro_export]
macro_rules! define_led_types {
    (
        $board:ident,
        $($light_name:expr => $gpio_num:expr),+
        $(,)?
    ) => {
        $(
            paste::paste! {
                pub type [< SbcLight $light_name>] = rp_pico::hal::gpio::Pin<
                        rp_pico::hal::gpio::bank0::[<Gpio $gpio_num>],
                        rp_pico::hal::gpio::FunctionSioOutput,
                        rp_pico::hal::gpio::PullDown,
                    >;
            }
        )+

        paste::paste! {

        }
        };
}

#[cfg(feature = "rp-pico")]
define_led_types!(pico, Fore => 0, Back => 2, Left => 4, Right => 6);
#[cfg(feature = "sbc-rpi")]
define_led_types!(rpi, Fore => 0, Back => 2, Left => 4, Right => 6);

pub struct LightArray<TLightFore, TLightBack, TLightLeft, TLightRight>
where
    TLightFore: digital::OutputPin,
    TLightBack: digital::OutputPin,
    TLightLeft: digital::OutputPin,
    TLightRight: digital::OutputPin,
{
    pub fore: TLightFore,
    pub back: TLightBack,
    pub left: TLightLeft,
    pub right: TLightRight,
}

pub struct MotorArray<TM1F, TM1B, TM1E, TM2F, TM2B, TM2E, TM3F, TM3B, TM3E, TM4F, TM4B, TM4E>
where
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
{
    pub motor1: motor::MotorL293D<TM1F, TM1B, TM1E>,
    pub motor2: motor::MotorL293D<TM2F, TM2B, TM2E>,
    pub motor3: motor::MotorL293D<TM3F, TM3B, TM3E>,
    pub motor4: motor::MotorL293D<TM4F, TM4B, TM4E>,
}

pub struct MotorShield<
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
> where
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
{
    pub sensor_ir1: sensor::infrared::SensorIR<TIR1>,
    pub sensor_ir2: sensor::infrared::SensorIR<TIR2>,
    pub sensor_sonic: sensor::ultrasonic::Sonar<TSonicEcho, TSonicTrig>,
    pub motors: MotorArray<TM1F, TM1B, TM1E, TM2F, TM2B, TM2E, TM3F, TM3B, TM3E, TM4F, TM4B, TM4E>,
    pub m1: motor_driver_hal::MotorDriverWrapper<TM1F, TM1B, TM1E, ()>,
    pub m2: motor_driver_hal::MotorDriverWrapper<TM2F, TM2B, TM2E, ()>,
    pub m3: motor_driver_hal::MotorDriverWrapper<TM3F, TM3B, TM3E, ()>,
    pub m4: motor_driver_hal::MotorDriverWrapper<TM4F, TM4B, TM4E, ()>,
    pub lights: LightArray<TLightFore, TLightBack, TLightLeft, TLightRight>,
}

impl<
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
where
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
{
    // pub fn new() -> Self {
    //     Self {
    //         sensor_ir1: sensor::SensorIR::new(),
    //         sensor_ir2: sensor::SensorIR::new(),
    //         sensor_sonic: sensor::Ultrasonic::new(),
    //         sensor_sonic_x: sensor::Ultrasonic::new(),
    //         motor1: motor::MotorL293D::new(fwd_pin, bak_pin, enable_pin)
    //     }
    // }
}

pub struct MotorShieldConfigurationBuilder<
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
> where
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
{
    pub sensor_ir1: Option<sensor::infrared::SensorIR<TIR1>>,
    pub sensor_ir2: Option<sensor::infrared::SensorIR<TIR2>>,
    pub sensor_sonic: Option<sensor::ultrasonic::Sonar<TSonicEcho, TSonicTrig>>,
    pub motors:
        Option<MotorArray<TM1F, TM1B, TM1E, TM2F, TM2B, TM2E, TM3F, TM3B, TM3E, TM4F, TM4B, TM4E>>,
    pub m1: Option<motor_driver_hal::MotorDriverWrapper<TM1F, TM1B, TM1E, ()>>,
    pub m2: Option<motor_driver_hal::MotorDriverWrapper<TM2F, TM2B, TM2E, ()>>,
    pub m3: Option<motor_driver_hal::MotorDriverWrapper<TM3F, TM3B, TM3E, ()>>,
    pub m4: Option<motor_driver_hal::MotorDriverWrapper<TM4F, TM4B, TM4E, ()>>,
    pub lights: Option<LightArray<TLightFore, TLightBack, TLightLeft, TLightRight>>,
}

enum MotorShieldError {
    Unspecified,
    ConfigurationInvalid,
}

impl<
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
where
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
{
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

    pub fn with_sonic(mut self, trigger: TSonicTrig, echo_receiver: TSonicEcho) -> Self {
        self.sensor_sonic = Some(sensor::ultrasonic::Sonar::new(trigger, echo_receiver));
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

    fn build(
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
        // todo!("perform final checks");
        let data = MotorShield {
            sensor_ir1: self.sensor_ir1.unwrap(),
            sensor_ir2: self.sensor_ir2.unwrap(),
            sensor_sonic: self.sensor_sonic.unwrap(),
            motors: self.motors.unwrap(),
            m1: self.m1.unwrap(),
            m2: self.m2.unwrap(),
            m3: self.m3.unwrap(),
            m4: self.m4.unwrap(),
            lights: self.lights.unwrap(),
        };
        return Ok(data);
    }
    fn unused(di: usize) {
        return;
    }
}
