# BharatOS Power Management

## CPU Power

- **Speed Shift** (Intel) — frequency transition in µs
- **CPPC** (AMD) — Collaborative Processor Performance Control
- **C-states** — C1 (halt), C1E, C3, C6, C7 (package)
- **P-states** — CPU frequency/voltage pairs
- **Turbo Boost** — sustained above-base frequency

## Display Power

- **DPMS**: Display Power Management Signaling
- **Adaptive brightness** based on ambient light
- **HDR tone mapping** — ST2084 PQ curve

## Battery

- **Charge monitoring**: % and mAh via ACPI/EC
- **Discharge rate**: estimated via voltage + mAh delta
- **Estimated remaining**: battery_level / discharge_rate

## Profiles

| Profile | CPU | GPU | Brightness | Polling |
|---------|-----|-----|-----------|---------|
| Performance | 100% | 100% | 100% | 30 Hz |
| Balanced | 80% | 80% | 50% + adaptive | 1 Hz |
| Battery | 50% | 50% | dim | 0.5 Hz |
| Eco | 30% | 30% | minimum | 0.1 Hz |
| Flight | 50% | off | minimum | 0.1 Hz |
| Presentation | 100% | 100% | 80% | admin |

## Suspend/Resume

- S3: Suspend to RAM
- S4: Hibernate
- Modern Standby (InstantGo)
- Wake sources: keyboard, mouse, LAN, RTC, power button

## TLP Integration

BharatOS uses an integrated power management daemon (bharat-powerd):
- Reads ACPI _PSV (state value) and _PCF (processor control)
- Applies governor policies
- Manages runtime power management (runtime PM)
