#![no_std]

// pub mod sbc_motor_shield;
pub(crate) mod sbc_motor_shield;

#[cfg(feature = "rp-pico")]
pub mod pico_shield {

    use crate::sbc_motor_shield;

    pub type PicoGeeekpiSbcShield = sbc_motor_shield::MotorShield<
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio6,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioInput>,
            rp_pico::hal::gpio::PullUp,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio28,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioInput>,
            rp_pico::hal::gpio::PullUp,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio11,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioInput>,
            rp_pico::hal::gpio::PullNone,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio10,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioOutput>,
            rp_pico::hal::gpio::PullDown,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio9,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioOutput>,
            rp_pico::hal::gpio::PullDown,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio8,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioOutput>,
            rp_pico::hal::gpio::PullDown,
        >,
        rp_pico::hal::pwm::Channel<
            rp_pico::hal::pwm::Slice<rp_pico::hal::pwm::Pwm3, rp_pico::hal::pwm::FreeRunning>,
            rp_pico::hal::pwm::B,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio27,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioOutput>,
            rp_pico::hal::gpio::PullDown,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio26,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioOutput>,
            rp_pico::hal::gpio::PullDown,
        >,
        rp_pico::hal::pwm::Channel<
            rp_pico::hal::pwm::Slice<rp_pico::hal::pwm::Pwm3, rp_pico::hal::pwm::FreeRunning>,
            rp_pico::hal::pwm::A,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio4,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioOutput>,
            rp_pico::hal::gpio::PullDown,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio2,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioOutput>,
            rp_pico::hal::gpio::PullDown,
        >,
        rp_pico::hal::pwm::Channel<
            rp_pico::hal::pwm::Slice<rp_pico::hal::pwm::Pwm1, rp_pico::hal::pwm::FreeRunning>,
            rp_pico::hal::pwm::B,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio5,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioOutput>,
            rp_pico::hal::gpio::PullDown,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio19,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioOutput>,
            rp_pico::hal::gpio::PullDown,
        >,
        rp_pico::hal::pwm::Channel<
            rp_pico::hal::pwm::Slice<rp_pico::hal::pwm::Pwm1, rp_pico::hal::pwm::FreeRunning>,
            rp_pico::hal::pwm::A,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio14,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioOutput>,
            rp_pico::hal::gpio::PullDown,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio12,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioOutput>,
            rp_pico::hal::gpio::PullDown,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio13,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioOutput>,
            rp_pico::hal::gpio::PullDown,
        >,
        rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio17,
            rp_pico::hal::gpio::FunctionSio<rp_pico::hal::gpio::SioOutput>,
            rp_pico::hal::gpio::PullDown,
        >,
    >;
    // chA (even pin, pwmout only),  chB (odd pin, pwmio)
    // impl  sbc_motor_shield::MotorShieldConfigurationBuilder<> {

    // }
    /// Initialize the sb-components motor shield.
    /// Mapped according to the geekpi pico-to-rpi-40pin-hat breakout board
    pub fn create(
        pins: rp_pico::Pins,
        pwm_slices: rp_pico::hal::pwm::Slices,
    ) -> Result<PicoGeeekpiSbcShield, sbc_motor_shield::MotorShieldError> {
        let pico_shield = sbc_motor_shield::MotorShieldConfigurationBuilder::new()
            .with_ir1(pins.gpio6.into_pull_up_input())
            .with_ir2(pins.gpio28.into_pull_up_input())
            .with_sonic(
                pins.gpio10.into_push_pull_output(),
                pins.gpio11.into_floating_input(), // pull type not specified?
            )
            // .with_sonic2(
            //     pins.gpio15.into_push_pull_output(),
            //     pins.gpio16.into_floating_input(), // pull type not specified?
            // )
            .with_lights(
                pins.gpio14.into_push_pull_output(), // f
                pins.gpio12.into_push_pull_output(), // b
                pins.gpio13.into_push_pull_output(), // L
                pins.gpio17.into_push_pull_output(), // R
            )
            .with_motor1(
                // 5(gp3) is available?
                pins.gpio9.into_push_pull_output(),
                pins.gpio8.into_push_pull_output(),
                // pwm_slices.pwm3.channel_b.output_to(pins.gpio7), // TODO: find gp7 among slices [6,7,22,23]:pwm3
                pwm_slices.pwm3.channel_b, // TODO: find gp7 among slices [6,7,22,23]:pwm3
                Some(1000),
            )
            .with_motor2(
                pins.gpio27.into_push_pull_output(),
                pins.gpio26.into_push_pull_output(),
                pwm_slices.pwm3.channel_a, // TODO: find gp 22 among slices ! also pwm3?
                Some(1000),
            )
            // chA (even pin, pwmout only),  chB (odd pin, pwmio)
            .with_motor3(
                pins.gpio4.into_push_pull_output(),
                pins.gpio2.into_push_pull_output(),
                pwm_slices.pwm1.channel_b, // TODO: find gp 3 among slices {2,3,18,19}: pwm1
                Some(1000),
            )
            .with_motor4(
                pins.gpio5.into_push_pull_output(),
                pins.gpio19.into_push_pull_output(),
                pwm_slices.pwm1.channel_a, // TODO: find gp 18 among slices !also pwm1 {2,3,18,19}: pwm1
                Some(1000),
            )
            // .with_i2c(sda:pins.gpio20, scl:pins.gpio21.i2c) // I2C #0
            .build();
        return pico_shield;
    }
}

#[cfg(feature = "sbc-rpi")]
mod rpi_shield {}
