# STM32_Drone_Project_DC

Embedded Rust Projekt für eine kleine DIY-Quadcopter-Drohne mit brushed DC-Motoren
auf Basis von zwei STM32F103 ("BluePill") Boards.

## Ziele

- Rust-Firmware für:
  - **Flight Controller** (Drohne)
  - **Remote** (Fernbedienung mit Joysticks)
- Saubere Softwarearchitektur:
  - Trennung von Hardware-Abstraktion, Protokoll und Anwendungslogik
  - Wiederverwendbare Crates (`drone-hal`, `drone-protocol`)
- Lern-/Portfolio-Projekt:
  - Zeigt Erfahrung mit Embedded Rust, HAL, STM32, NRF24, MPU, GitHub-Workflow

## Projektstruktur

```text
STM32_Drone_Project_DC/
├─ Cargo.toml              # Workspace-Definition
├─ memory.x                # Linker-Memory-Layout für STM32F103C8
├─ .cargo/
│  └─ config.toml          # Build-Target & probe-rs Runner
├─ docs/
│  ├─ architecture.md      # Software-Architektur
│  ├─ hardware.md          # Hardware-Design (Motoren, MOSFETs, LiPo, Sensoren)
│  └─ protocol.md          # Kommunikationsprotokoll (NRF24)
├─ crates/
│  ├─ drone-hal/           # abstrakte Hardware-Traits (Motor, Radio, Sensoren)
│  └─ drone-protocol/      # Datenstrukturen für Funkprotokoll
└─ firmware/
   ├─ flight_controller/   # Firmware für die Drohne (STM32F103)
   └─ remote/              # Firmware für die Fernbedienung
