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
            rp_pico::hal::gpio::FunctionSioOutput,
            rp_pico::hal::gpio::PullDown,
        >,
    >;

    #[deprecated]
    /// Initialize the sb-components motor shield.
    /// Mapped according to the geekpi pico-to-rpi-40pin-hat breakout board
    pub fn create_with_extras(pins: rp_pico::Pins, pwm_slices: rp_pico::hal::pwm::Slices) {}

    /// Initialize the sb-components motor shield.
    /// Mapped according to the geekpi pico-to-rpi-40pin-hat breakout board
    pub fn create(
        pins: rp_pico::Pins,
        pwm_slices: rp_pico::hal::pwm::Slices,
    ) -> Result<PicoGeeekpiSbcShield, sbc_motor_shield::MotorShieldError> {
        // ? 20+21 (i2c0) | 25 (picoled) | 15+16 (unused, another sio sonar?)

        // do pwm configurations <see rp2040-hal::pwm>
        let mut pwm_m1_3b = pwm_slices.pwm3.channel_b;
        let mut pwm_m2_3a = pwm_slices.pwm3.channel_a;
        let mut pwm_m3_1b = pwm_slices.pwm1.channel_b;
        let mut pwm_m4_1a = pwm_slices.pwm1.channel_a;
        pwm_m1_3b.output_to(pins.gpio7);
        pwm_m2_3a.output_to(pins.gpio22);
        pwm_m3_1b.output_to(pins.gpio3);
        pwm_m4_1a.output_to(pins.gpio18);
        // ? what purpose is the alias rp_pico::Gp7Pwm3B
        /*
        from  <see rp2040-hal::pwm>
        [
        Pwm0: (0, [Gpio0, Gpio1, Gpio16, Gpio17], 0),
        Pwm1: (1, [Gpio2, Gpio3, Gpio18, Gpio19], 1),
        Pwm2: (2, [Gpio4, Gpio5, Gpio20, Gpio21], 2),
        Pwm3: (3, [Gpio6, Gpio7, Gpio22, Gpio23], 3),
        Pwm4: (4, [Gpio8, Gpio9, Gpio24, Gpio25], 4),
        Pwm5: (5, [Gpio10, Gpio11, Gpio26, Gpio27], 5),
        Pwm6: (6, [Gpio12, Gpio13, Gpio28, Gpio29], 6),
        Pwm7: (7, [Gpio14, Gpio15], 7)
        ]
        chA (even pin, pwmout only),  chB (odd pin, pwmio)
         */

        // Produce the motor shield
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
                pins.gpio14.into_push_pull_output(), // F
                pins.gpio12.into_push_pull_output(), // B
                pins.gpio13.into_push_pull_output(), // L
                pins.gpio17.into_push_pull_output(), // R
            )
            .with_motor1(
                // 5(gp3) is available?
                pins.gpio9.into_push_pull_output(),
                pins.gpio8.into_push_pull_output(),
                pwm_m1_3b, // TODO: find gp7 among slices [6,7,22,23]:pwm3
                Some(1000),
            )
            .with_motor2(
                pins.gpio27.into_push_pull_output(),
                pins.gpio26.into_push_pull_output(),
                pwm_m2_3a, // TODO: find gp 22 among slices ! also pwm3?
                Some(1000),
            )
            // chA (even pin, pwmout only),  chB (odd pin, pwmio)
            .with_motor3(
                pins.gpio4.into_push_pull_output(),
                pins.gpio2.into_push_pull_output(),
                pwm_m3_1b, // TODO: find gp 3 among slices {2,3,18,19}: pwm1
                Some(1000),
            )
            .with_motor4(
                pins.gpio5.into_push_pull_output(),
                pins.gpio19.into_push_pull_output(),
                pwm_m4_1a, // TODO: find gp 18 among slices !also pwm1 {2,3,18,19}: pwm1
                Some(1000),
            )
            // .with_i2c(sda:pins.gpio20, scl:pins.gpio21) // I2C #0
            .build();
        return pico_shield;
    }
}

#[cfg(feature = "sbc-rpi")]
mod rpi_shield {}
