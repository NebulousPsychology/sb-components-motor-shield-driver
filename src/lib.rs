pub mod sbc_motor_shield;

#[cfg(all(feature = "rp-pico"))]
pub mod shield_pico;

#[cfg(all(
    feature = "sbc-rpi",
    feature = "std",
    not(feature = "sbc-pico"),
    any(target_arch = "arm", target_arch = "aarch64"),
    // any(target_family = "unix")
))]
pub mod shield_rpi;
