# SB Components Motor Shield for Raspberry Pi

Raspberry Pi GPIO/PWM setup for `sb-components-motor-shield-core`, using `rppal`.

This integration targets Linux on Raspberry Pi. Verify the GPIO mapping and
software-PWM behavior against the specific shield revision and Pi model.

### [rppal](https://lib.rs/crates/rppal)

Pi4 does not have hardware PWM on the pins used by the shield, so software pwm [is used instead](/src/shield_rpi.rs).
By its inclusion in @sbcshop's samples, softPwm is not a dealbreaker.
