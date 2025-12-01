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

fn init_motors<P1F, P1B, P1E, P2F, P2B, P2E>(
    p_1f: P1F,
    p_1b: P1B,
    p_1e: P1E,
    p_2f: P2F,
    p_2b: P2B,
    p_2e: P2E,
) -> (
    motor_driver_hal::MotorDriverWrapper<P1F, P1B, P1E, ()>,
    motor_driver_hal::MotorDriverWrapper<P2F, P2B, P2E, ()>,
)
where
    P1F: embedded_hal::digital::OutputPin,
    P1B: embedded_hal::digital::OutputPin,
    P1E: embedded_hal::pwm::SetDutyCycle,
    P2F: embedded_hal::digital::OutputPin,
    P2B: embedded_hal::digital::OutputPin,
    P2E: embedded_hal::pwm::SetDutyCycle,
{
    // IN1[2]+IN2[7]+PWM_EN1[1] -> OUT1[3]+OUT2[6] -> Motor1
    let m1: motor_driver_hal::MotorDriverWrapper<P1F, P1B, P1E, ()> =
        motor_driver_hal::MotorDriverBuilder::new()
            .with_dual_enable(p_1f, p_1b)
            .with_pwm_channels(motor_driver_hal::wrapper::PwmChannels::Single(p_1e))
            .with_max_duty(1000)
            .build();
    // IN3[10]+IN4[15]+PWM_EN2[9] -> OUT3[11]+OUT4[14] -> Motor2
    let m2: motor_driver_hal::MotorDriverWrapper<P2F, P2B, P2E, ()> =
        motor_driver_hal::MotorDriverBuilder::new()
            .with_dual_enable(p_2f, p_2b)
            .with_pwm_channels(motor_driver_hal::wrapper::PwmChannels::Single(p_2e))
            .with_max_duty(1000)
            .build();

    // let sonic: sensor::ultrasonic::Sonar<_, _>::new(echo_receiver_pin, trigger_pin);
    // Keep or return m1/m2 if you need them to live beyond this function:
    return (m1, m2);
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
    data: MotorShield<
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
    fn with_motor1(
        self,
        p_f: impl digital::OutputPin,
        p_b: impl digital::OutputPin,
        p_e: impl pwm::SetDutyCycle,
    ) -> Self {
        // todo
        let m1: motor_driver_hal::MotorDriverWrapper<_, _, _, ()> =
            motor_driver_hal::MotorDriverBuilder::new()
                .with_dual_enable(p_f, p_b)
                .with_pwm_channels(motor_driver_hal::wrapper::PwmChannels::Single(p_e))
                .with_max_duty(1000)
                .build();
        return self;
    }

    fn with_pico_pins(self, pins: rp_pico::Pins, pwm_slices: rp_pico::hal::pwm::Slices) -> Self {
        let (a, b, c, d) = setup_pico_motors(pins, pwm_slices);
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
        return Ok(self.data);
    }
    fn unused(di: usize) {
        return;
    }
}
