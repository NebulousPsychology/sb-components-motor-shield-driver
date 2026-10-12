# SB Components Motor Shield Core

Portable `no_std` driver components for the SB Components Motor Shield, built
against `embedded-hal`. Board pin mappings and startup code are provided by the
Pico and Raspberry Pi integration crates in this repository.

The ultrasonic sensor accepts a caller-supplied microsecond clock so timer
selection remains platform-specific.
