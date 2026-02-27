#![no_std]

// Dieses Crate definiert abstrakte Hardware-Schnittstellen,
// die von Firmware-Projekten (Flight Controller, Remote) benutzt werden sollen.

pub mod motor;

pub use motor::Motor;