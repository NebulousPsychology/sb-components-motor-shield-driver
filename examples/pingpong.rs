#![no_std]
#![no_main]
#![allow(unused_variables)]

// The macro for our start-up function
use embedded_hal::digital::OutputPin;

use panic_halt as _;
use rp_pico::entry;
// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

use fugit::RateExtU32;
use sb_components_motor_shield_driver::*;

#[entry]
fn ping_pong() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    let pins: rp_pico::Pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Init PWMs
    let pwm_slices: hal::pwm::Slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);
    let (board, spare) = shield_pico::create(pins, pwm_slices).unwrap();

    // Set the LED to be an output
    let mut led_pin = spare.led.into_push_pull_output();

    // setup UART0
    let u = hal::uart::UartPeripheral::new(
        pac.UART0,
        (spare.gpio0.into_function(), spare.gpio1.into_function()),
        &mut pac.RESETS,
    )
    .enable(
        hal::uart::UartConfig::new(
            9600.Hz(),
            hal::uart::DataBits::Eight,
            None,
            hal::uart::StopBits::One,
        ),
        clocks.peripheral_clock.freq(),
    )
    .ok()
    .unwrap();

    // Setup an additional sonar sensor
    let sonar2 = components::sensor::ultrasonic::Sonar::new(
        spare.gpio15.into_push_pull_output(),
        spare.gpio16.into_pull_down_input(),
    );

    // setup i2c
    let i2c_peripheral = hal::I2C::i2c0(
        pac.I2C0,
        spare.gpio20.into_pull_up_disabled().into_function(),
        spare.gpio21.into_pull_up_disabled().into_function(),
        1234.Hz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(500);

        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
