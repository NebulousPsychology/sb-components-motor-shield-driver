# sb-components-motor-shield-driver

Rust driver for board components of SB Components' [Motorshield for the Raspberry Pi][product_page]

The documents and sample code for which are found:

- <https://github.com/sbcshop/MotorShield>
- particularly: <https://github.com/sbcshop/MotorShield/blob/master/PiMotor.py>

This crate provides an embedded-hal processor agnostic definition of the shield board, as well as an implementation for Rpi4 (rppal)

Any breakout which can feed the Rpi4 40pin [can map](pin_map.md) may define a board using `sbc_motor_shield::MotorShieldConfigurationBuilder`.
One such shield for Pico is `shield_pico::PicoGeeekpiSbcShield`, as mapped by a (possibly now discontinued?) breakout board
from [Geeekpi](https://thegeekpi.com). Breakouts by Waveshare may map similarly.
A niche scenario, but demonstrates that mappings for Pico are possible.

## Issues

The Features and cross-compilation issues degrade the build workflow, and are publish-blocking, but the crate remains accessible via git:
```toml
sb-components-motor-shield-driver = { git = "https://github.com/NebulousPsychology/sb-components-motor-shield-driver", branch = "main" }
```
- [ ] Features and cross-compilation. [consult: rust-cross](https://github.com/japaric/rust-cross)

Future Issues:

- [ ] Interrupts and Asynchronous operation (related to switching to an existing HC-SR04 crate )
- [ ] Lack of tests
- [x] Namespacing and project layout

## Build

```bash
cargo build --target thumbv6m-none-eabi -F sbc-pico --lib --no-default-features
cargo build --target aarch64-unknown-linux-gnu -F sbc-rpi --lib --no-default-features
```

## Other Considerations

### [l293x](https://lib.rs/crates/l293x)

ruled out as too bare-metal, cumbersome motor abstraction.

### [motor-driver-hal](https://lib.rs/crates/motor-driver-hal)

Enshrines a PWM-first direction logic not supported by the shield, requiring this project implement its traits for l293x.
See [](/src/sbc_motor_shield/motor.rs)

### [rppal](https://lib.rs/crates/rppal)

Pi4 does not have hardware PWM on the pins used by the shield, so software pwm [is used instead](/src/shield_rpi.rs).
By its inclusion in @sbcshop's samples, softPwm is not a dealbreaker.

### HC-SR04

the ultrasonic sensor is particularly sensitive to timing issues, the [current implementation](/src/sbc_motor_shield/sensor.rs)
makes virtually no effort to protect its accuracy. An existing crate is likely worth adopting in future.

- > <https://lib.rs/crates/hcsr04> (nostd, has async without advertising)
- > <https://lib.rs/crates/hcsr04_async> (nostd+async)
- ~~<https://lib.rs/crates/hc-sr04> (rpi)~~ (rejected non-agnostic)
- ~~<https://lib.rs/crates/hcsr04-gpio-cdev> (rpi5)~~ (rejected non-agnostic)

-----

[product_page]: https://shop.sb-components.co.uk/collections/hats-for-raspberry-pi/products/motorshield-for-raspberry-pi
