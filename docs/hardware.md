
---

### 2.3 `docs/hardware.md` – Teileliste & grober Aufbau

```markdown
# Hardware

## Stückliste (Bill of Materials)

- 2× STM32F103C8T6 ("BluePill")
- 4× brushed DC-Motoren (zweiadrig)
- 4× Propeller
- 10× MOSFET IRLZ34N (N-Kanal, logic level)
- 100× Dioden 1N4148 (Freilauf / Schutz)
- 1× MPU6500 IMU (Gyroskop + Beschleunigungssensor)
- 2× NRF24-Module (Drohne / Fernbedienung)
- 2× PS3-Style Joysticks
- mehrere Taster / Buttons
- 3,7 V LiPo-Akku

## Grundaufbau

```text
LiPo 3,7 V
   │
   ├─> Flight Controller (BluePill)
   │     ├─ PWM-Ausgänge (Timer) ─> MOSFETs ─> DC-Motoren
   │     ├─ I2C/SPI ─> MPU6500
   │     └─ SPI ─> NRF24 (Funk)
   │
   └─> Remote (BluePill)
         ├─ ADC ─> Joysticks
         ├─ GPIO ─> Buttons
         └─ SPI ─> NRF24 (Funk)
