
---

### 2.2 `docs/architecture.md` – grobes Skelett

```markdown
# Software-Architektur

Die Firmware ist in mehrere Schichten aufgeteilt, um Hardware und Logik zu trennen
und das Projekt gut wartbar zu halten.

## Schichtenmodell

1. **Application Layer (Firmware)**
   - `firmware/flight_controller`
   - `firmware/remote`
   - Enthält die eigentliche Anwendungslogik:
     - Flight-Control-Loop
     - Auswertung von Joysticks
     - Zustandsautomaten, Failsafe, etc.

2. **Hardware Abstraction Layer (HAL)**
   - `crates/drone-hal`
   - Definiert plattformunabhängige Traits, z.B.:
     - `Motor`, `MotorSet`
     - später: `Radio`, `Imu`
   - Wird von den Firmwares implementiert / genutzt.

3. **Hardware-spezifische Implementierung**
   - im Moment direkt in `firmware/flight_controller/src/board.rs`
   - nutzt `stm32f1xx-hal`, um:
     - GPIO, Timer (PWM), I2C, SPI zu konfigurieren
     - konkrete Pins zuzuweisen (z.B. PC13 = LED)

## Datenfluss (geplant)

```text
Remote (Joystick) ──NRF24──> Flight Controller ──PWM──> Motoren
                                 │
                                IMU
                                 │
                              Regelung
