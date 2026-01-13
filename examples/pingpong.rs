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

    // Set the LED to be an output
    let mut led_pin = pins.led.into_push_pull_output();
    let mut alt_led_pin = pins.gpio14.into_push_pull_output();

    // Init PWMs
    let pwm_slices: hal::pwm::Slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);
    let _m = components::motor::MotorL293D::new(
        pins.gpio0.into_push_pull_output(),
        pins.gpio1.into_push_pull_output(),
        pwm_slices.pwm0.channel_a,
    );
    let m2 = components::motor::MotorL293D::new(
        pins.gpio2.into_push_pull_output(),
        pins.gpio3.into_push_pull_output(),
        pwm_slices.pwm0.channel_b,
    );
    //  pins.gpio2.into_push_pull_output(), pins.gpio1.into_push_pull_output(), rp_pico::Gp0Pwm0A.);

    loop {
        led_pin.set_high().unwrap();
        alt_led_pin.set_low().unwrap();
        delay.delay_ms(500);

        led_pin.set_low().unwrap();
        alt_led_pin.set_high().unwrap();
        delay.delay_ms(500);
    }
}
