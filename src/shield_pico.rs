#[cfg(feature = "std")] // Use no_std if std feature is disabled
compile_error!("pico should not be defined for std environments");

#[cfg(feature = "rp-pico")]
pub mod shield_pico {

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

    /// pins not used by sb-components motor shield
    pub struct UnusedPins {
        pub voltage_monitor: rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio29,
            rp_pico::hal::gpio::FunctionNull,
            rp_pico::hal::gpio::PullDown,
        >,
        pub vbus_detect: rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio24,
            rp_pico::hal::gpio::FunctionNull,
            rp_pico::hal::gpio::PullDown,
        >,
        pub b_power_save: rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio23,
            rp_pico::hal::gpio::FunctionNull,
            rp_pico::hal::gpio::PullDown,
        >,
        pub led: rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio25,
            rp_pico::hal::gpio::FunctionNull,
            rp_pico::hal::gpio::PullDown,
        >,
        /// Recommended use: UART0TX (not gp16 uart0/i2c0)
        pub gpio0: rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio0,
            rp_pico::hal::gpio::FunctionNull,
            rp_pico::hal::gpio::PullDown,
        >,
        /// Recommended use: UART0RX (not gp17 uart0/i2c0)
        pub gpio1: rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio1,
            rp_pico::hal::gpio::FunctionNull,
            rp_pico::hal::gpio::PullDown,
        >,
        /// Recommended use: sonic2_tr
        pub gpio15: rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio15,
            rp_pico::hal::gpio::FunctionNull,
            rp_pico::hal::gpio::PullDown,
        >,
        /// Recommended use: sonic2_en
        pub gpio16: rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio16,
            rp_pico::hal::gpio::FunctionNull,
            rp_pico::hal::gpio::PullDown,
        >,
        /// Recommended use: 12C0 SDA
        pub gpio20: rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio20,
            rp_pico::hal::gpio::FunctionNull,
            rp_pico::hal::gpio::PullDown,
        >,
        /// Recommended use: 12C0 SCL
        pub gpio21: rp_pico::hal::gpio::Pin<
            rp_pico::hal::gpio::bank0::Gpio21,
            rp_pico::hal::gpio::FunctionNull,
            rp_pico::hal::gpio::PullDown,
        >,
    }

    /// Initialize the sb-components motor shield.
    /// Mapped according to the geekpi pico-to-rpi-40pin-hat breakout board
    pub fn create(
        pins: rp_pico::Pins,
        pwm_slices: rp_pico::hal::pwm::Slices,
    ) -> Result<(PicoGeeekpiSbcShield, UnusedPins), sbc_motor_shield::MotorShieldError> {
        // ? 20+21 (i2c0) | 25 (picoled) | 15+16 (unused, another sio sonar?)
        let unused = UnusedPins {
            voltage_monitor: pins.voltage_monitor, //29/adc vref?
            b_power_save: pins.b_power_save,       //23/run?
            vbus_detect: pins.vbus_detect,         //24
            led: pins.led,
            gpio0: pins.gpio0,
            gpio1: pins.gpio1,
            gpio15: pins.gpio15,
            gpio16: pins.gpio16,
            gpio20: pins.gpio20,
            gpio21: pins.gpio21,
        };

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
            .build()?;

        return Ok((pico_shield, unused));
    }
}
