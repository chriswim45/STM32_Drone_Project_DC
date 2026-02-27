#![no_std]

// Steuerdaten von der Fernbedienung zur Drohne
pub struct TxToFc {
    pub throttle: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub buttons: u16,
}

// Statusdaten von der Drohne zurück zur Fernbedienung
pub struct FcToTx {
    pub battery_mv: u16,
    pub link_quality: u8,
}
