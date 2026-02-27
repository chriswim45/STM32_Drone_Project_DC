//! Abstrakte Schnittstellen für Motoren.
//!
//! Diese Traits sind komplett plattformunabhängig. Die konkrete
//! Implementierung (STM32, MOSFETs, PWM-Kanäle usw.) passiert z.B.
//! im flight_controller-Projekt.

/// einzelner Motor (z.B. ein DC-Motor mit MOSFET + PWM)
pub trait Motor {
    /// duty: 0.0 = aus, 1.0 = volle Leistung.
    fn set_throttle(&mut self, duty: f32);
}

/// Menge von vier Motoren (Quadcopter).
pub trait MotorSet {
    /// Setzt für alle Motoren denselben Duty Cycle.
    fn set_all(&mut self, duty: f32);
    // später: fn set_individual(...)
}
