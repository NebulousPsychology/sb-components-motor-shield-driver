# sb-components-motor-shield-driver

Rust driver for board components of SB Components' [Motorshield for the Raspberry Pi][product_page]

The documents and sample code for which are found:

- <https://github.com/sbcshop/MotorShield>
- particularly: <https://github.com/sbcshop/MotorShield/blob/master/PiMotor.py>

This crate provides an embedded-hal processor agnostic definition of the shield board, as well as an implementation for Rpi4 (rppal)

Any breakout which can feed the Rpi4 40pin [can map](pin_map.md) may define a board using `sbc_motor_shield::MotorShieldConfigurationBuilder`

`shield_pico::PicoGeeekpiSbcShield` defines the shield for Pico, as mapped by a
(possibly now discontinued?) breakout board from [Geeekpi](https://thegeekpi.com). Breakouts by Waveshare may map similarly.
Niche, but demonstrates that mappings for Pico are possible.

## Issues

The following issues prevent this crate from being `publish = true`

- [ ] Poor features and cross compilation management. See [Build](#build)
- [ ] Lack of tests
- [ ] Namespacing could be better thought out

## Build

```bash
cargo build --target thumbv6m-none-eabi -F sbc-pico --lib --no-default-features
cargo build --target aarch64-unknown-linux-gnu -F sbc-rpi --lib --no-default-features
```

## other considerations

### [l293x](https://lib.rs/crates/l293x)

ruled out as too bare-metal, cumbersome motor abstraction.

### [motor-driver-hal](https://lib.rs/crates/motor-driver-hal)

Enshrines a PWM-first direction logic not supported by the shield, requiring this project implement its traits for l293x.
See [](/src/sbc_motor_shield/motor.rs)

### [rppal](https://lib.rs/crates/rppal)

Pi4 does not have hardware PWM on the pins used by the shield, so software pwm [is used instead](/src/shield_rpi.rs).
By its inclusion in @sbcshop's samples, softPwm is not a dealbreaker.

### [cross compile](https://github.com/japaric/rust-cross)

possible route for improving the build workflow.

-----

[product_page]: https://shop.sb-components.co.uk/collections/hats-for-raspberry-pi/products/motorshield-for-raspberry-pi
