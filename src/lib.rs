#![cfg_attr(not(feature = "std"), no_std)] // Use no_std if std feature is disabled

pub mod components;

#[cfg(all(feature = "rp-pico"))]
pub mod shield_pico;

#[cfg(all(
    feature = "sbc-rpi",
    feature = "std",
    not(feature = "sbc-pico"),
    any(target_arch = "arm", target_arch = "aarch64"),
))]
pub mod shield_rpi;
