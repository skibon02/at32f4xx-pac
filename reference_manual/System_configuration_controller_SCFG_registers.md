

### 6.3.12 GPIO huge current control register (GPIOx_HDRV) (x=A..E)

| Bit        | Name     | Reset value | Type | Description                                                                                                             |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                              |
| Bit 15:0   | HDRV     | 0x0000      | rw   | Huge sourcing/sinking strength control<br/>0: Not active<br/>1: GPIO is configured as maximum sourcing/sinking strength |


# 7 System configuration controller (SCFG)

## 7.1 Introduction

This device contains a set of system configuration register. The system configuration controller is mainly used to:

* Manage the external interrupts connected to the GPIOs

* Control the memory mapping mode

* Manage IRTMR/EMAC GPIO configurations

## 7.2 IOMUX registers

Table 7-1 shows SCFG register map and their reset values.

These peripheral registers must be accessed by words (32 bits).

Table 7-1 SCFG register map and reset values

| Register      | Offset | Reset value |
| ------------- | ------ | ----------- |
| SCFG\_CFG1    | 0x00   | 0x0000 000X |
| SCFG\_CFG2    | 0x04   | 0x0000 0000 |
| SCFG\_EXINTC1 | 0x08   | 0x0000 0000 |
| SCFG\_EXINTC2 | 0x0C   | 0x0000 0000 |
| SCFG\_EXINTC3 | 0x10   | 0x0000 0000 |
| SCFG\_EXINTC4 | 0x14   | 0x0000 0000 |
| SCFG\_UHDRV   | 0x2C   | 0x0000 0000 |


### 7.2.1 SCFG configuration register 1 (SCFG_CFG1 )

| Bit        | Name         | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ---------- | ------------ | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 12 | Reserved     | 0x00000     | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Bit 11: 10 | SWAP\_XMC    | 0x0         | rw   | XMC address mapping swap<br/>00: No XMC address mapping swap<br/>01: SDRAM addresses are mapped at 0x6000 0000 and 0x7000 0000. NOR/PSRAM /SRAM/NAND2 memory addresses are mapped at 0xC000 00000 and 0xD000 0000.<br/>10: QSPI2 memory addresses are mapped at 0x8000 0000. NAND3 memory is mapped at 0xB000 0000.<br/>11: SDRAM memory addresses are mapped at 0x6000 0000 and 0x7000 0000. NOR/PSRAM /SRAM/NAND2 memory addresses are mapped at 0xC000 00000 and 0xD000 0000. QSPI2 memory addresses are mapped at *0x8000 0000. NAND3 memory is mapped at 0xB000 0000.* |
| Bit 9: 8   | Reserved     | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Bit 7: 6   | IR\_SRC\_SEL | 0x0         | rw   | Infrared modulation envelope signal source selection<br/>This field is used to select the infrared modulation envelope signal source.<br/>00: TMR10                                                                                                                                                                                                                                                                                                                                                                                                                         |
|          |               |     |      | 01: Reserved<br/>10: Reserved<br/>11: Reserved                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Bit 5    | IR\_POL       | 0x0 | rw   | Infrared output polarity selection<br/>0: Infrared output (IR\_OUT) is not inversed<br/>1: Infrared output (IR\_OUT) is inversed                                                                                                                                                                                                                                                                                                                                                                                                                                |
| Bit 4: 3 | Reserved      | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Bit 2: 0 | MEM\_MAP\_SEL | 0xX | rw   | Memory address mapping selection<br/>This field defines the memory address mapping at 0x0000 0000. After reset, the reset value of this field is the same as that of the BOOT0 and BOOT1 pins. After changing this field, the user can decide which of the following memory to be mapped at 0x0000 0000.<br/>000: Main Flash memory mapped at 0x0000 0000<br/>001: Boot memory mapped at 0x0000 0000<br/>010: XMC BANK1 mapped at 0x0000 0000<br/>011: Embedded SRAM mapped at 0x0000 0000<br/>100: XMC SDRAM BANK1 mapped at 0x0000 0000<br/>Others: Reserved. |


## 7.2.2 SCFG configuration register 2 (SCFG_CFG2)

| Bit        | Name           | Reset value | Type | Description                                                                                                                                                  |
| ---------- | -------------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 24 | Reserved       | 0x00        | resd | Kept at its default value.                                                                                                                                   |
| Bit 23     | MII\_RMII\_SEL | 0x0         | rw   | MII or RMII selection<br/>This bit is used to select the Ethernet MII or RMII interface.<br/>0: MII<br/>1: RMII<br/>Note: This bit applies to AT32F437 only. |
| Bit 22: 0  | Reserved       | 0x00000X    | resd | Kept at its default value.                                                                                                                                   |


## 7.2.3 SCFG external interrupt configuration register 1 (SCFG_EXINTC1)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                          |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                                                                                                                                                                                                                           |
| Bit 15: 12 | EXINT3   | 0x0000      | rw   | EXINT3 input source configuration<br/>These bits are used to select the input source for the EXINT3 external interrupt.<br/>0000: GPIOA pin3<br/>0001: GPIOB pin3<br/>0010: GPIOC pin3<br/>0011: GPIOD pin3<br/>0100: GPIOE pin3<br/>0101: GPIOF pin3<br/>0110: GPIOG pin3<br/>0111: GPIOH pin3<br/>Others: Reserved |
| Bit 11: 8  | EXINT2   | 0x0000      | rw   | EXINT2 input source configuration<br/>These bits are used to select the input source for the EXINT2 external interrupt.<br/>0000: GPIOA pin2<br/>0001: GPIOB pin2<br/>0010: GPIOC pin2<br/>0011: GPIOD pin2<br/>0100: GPIOE pin2                                                                                     |
|          |        |        |    | 0101: GPIOF pin2<br/>0110: GPIOG pin2<br/>0111: GPIOH pin2<br/>Others: Reserved                                                                                                                                                                                                                                      |
| Bit 7: 4 | EXINT1 | 0x0000 | rw | EXINT1 input source configuration<br/>These bits are used to select the input source for the EXINT1 external interrupt.<br/>0000: GPIOA pin1<br/>0001: GPIOB pin1<br/>0010: GPIOC pin1<br/>0011: GPIOD pin1<br/>0100: GPIOE pin1<br/>0101: GPIOF pin1<br/>0110: GPIOG pin1<br/>0111: GPIOH pin1<br/>Others: Reserved |
| Bit 3: 0 | EXINT0 | 0x0000 | rw | EXINT0 input source configuration<br/>These bits are used to select the input source for the EXINT0 external interrupt.<br/>0000: GPIOA pin0<br/>0001: GPIOB pin0<br/>0010: GPIOC pin0<br/>0011: GPIOD pin0<br/>0100: GPIOE pin0<br/>0101: GPIOF pin0<br/>0110: GPIOG pin0<br/>0111: GPIOH pin0<br/>Others: Reserved |


## 7.2.4 SCFG external interrupt configuration register 2 (SCFG_ EXINTC2)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                     |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                                                                                                                                                                                                      |
| Bit 15: 12 | EXINT7   | 0x0000      | rw   | EXINT7 input source configuration<br/>These bits are used to select the input source for the EXINT7 external interrupt.<br/>0000: GPIOA pin7<br/>0001: GPIOB pin7<br/>0010: GPIOC pin7<br/>0011: GPIOD pin7<br/>0100: GPIOE pin7<br/>0101: GPIOF pin7<br/>0110: GPIOG pin7<br/>Others: Reserved |
| Bit 11: 8  | EXINT6   | 0x0000      | rw   | EXINT6 input source configuration<br/>These bits are used to select the input source for the EXINT6 external interrupt.<br/>0000: GPIOA pin6                                                                                                                                                    |
|          |        |        |    | 0001: GPIOB pin6<br/>0010: GPIOC pin6<br/>0011: GPIOD pin6<br/>0100: GPIOE pin6<br/>0101: GPIOF pin6<br/>0110: GPIOG pin6<br/>Others: Reserved                                                                                                                                                      |
| Bit 7: 4 | EXINT5 | 0x0000 | rw | **EXINT5 input source configuration**<br/>These bits are used to select the input source for the EXINT5 external interrupt.<br/>0000: GPIOA pin5<br/>0001: GPIOB pin5<br/>0010: GPIOC pin5<br/>0011: GPIOD pin5<br/>0100: GPIOE pin5<br/>0101: GPIOF pin5<br/>0110: GPIOG pin5<br/>Others: Reserved |
| Bit 3: 0 | EXINT4 | 0x0000 | rw | **EXINT4 input source configuration**<br/>These bits are used to select the input source for the EXINT4 external interrupt.<br/>0000: GPIOA pin4<br/>0001: GPIOB pin4<br/>0010: GPIOC pin4<br/>0011: GPIOD pin4<br/>0100: GPIOE pin4<br/>0101: GPIOF pin4<br/>0110: GPIOG pin4<br/>Others: Reserved |


## 7.2.5 SCFG external interrupt configuration register 3 (SCFG_ EXINTC3)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                  |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                                                                                                                                                                                                                   |
| Bit 15: 12 | EXINT11  | 0x0000      | rw   | **EXINT11 input source configuration**<br/>These bits are used to select the input source for the EXINT11 external interrupt.<br/>0000: GPIOA pin11<br/>0001: GPIOB pin11<br/>0010: GPIOC pin11<br/>0011: GPIOD pin11<br/>0100: GPIOE pin11<br/>0101: GPIOF pin11<br/>0110: GPIOG pin11<br/>Others: Reserved |
| Bit 11: 8  | EXINT10  | 0x0000      | rw   | **EXINT10 input source configuration**<br/>These bits are used to select the input source for the EXINT10 external interrupt.                                                                                                                                                                                |
|          |        |        |    | 0000: GPIOA pin10<br/>0001: GPIOB pin10<br/>0010: GPIOC pin10<br/>0011: GPIOD pin10<br/>0100: GPIOE pin10<br/>0101: GPIOF pin10<br/>0110: GPIOG pin10<br/>Others: Reserved                                                                                                                      |
| Bit 7: 4 | EXINT9 | 0x0000 | rw | EXINT9 input source configuration<br/>These bits are used to select the input source for the EXINT9 external interrupt.<br/>0000: GPIOA pin9<br/>0001: GPIOB pin9<br/>0010: GPIOC pin9<br/>0011: GPIOD pin9<br/>0100: GPIOE pin9<br/>0101: GPIOF pin9<br/>0110: GPIOG pin9<br/>Others: Reserved |
| Bit 3: 0 | EXINT8 | 0x0000 | rw | EXINT8 input source configuration<br/>These bits are used to select the input source for the EXINT8 external interrupt.<br/>0000: GPIOA pin8<br/>0001: GPIOB pin8<br/>0010: GPIOC pin8<br/>0011: GPIOD pin8<br/>0100: GPIOE pin8<br/>0101: GPIOF pin8<br/>0110: GPIOG pin8<br/>Others: Reserved |


# 7.2.6 SCFG external interrupt configuration register 4 (SCFG_ EXINTC4)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                              |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value                                                                                                                                                                                                                                                                                |
| Bit 15: 12 | EXINT15  | 0x0000      | rw   | EXINT15 input source configuration<br/>These bits are used to select the input source for the EXINT15 external interrupt.<br/>0000: GPIOA pin15<br/>0001: GPIOB pin15<br/>0010: GPIOC pin15<br/>0011: GPIOD pin15<br/>0100: GPIOE pin15<br/>0101: GPIOF pin15<br/>0110: GPIOG pin15<br/>Others: Reserved |
| Bit 11: 8  | EXINT14  | 0x0000      | rw   | EXINT14 input source configuration<br/>These bits are used to select the input source for the EXINT14 external interrupt.                                                                                                                                                                                |
|          |         |        |    | 0000: GPIOA pin14<br/>0001: GPIOB pin14<br/>0010: GPIOC pin14<br/>0011: GPIOD pin14<br/>0100: GPIOE pin14<br/>0101: GPIOF pin14<br/>0110: GPIOG pin14<br/>Others: Reserved                                                                                                                               |
| Bit 7:4  | EXINT13 | 0x0000 | rw | EXINT13 input source configuration<br/>These bits are used to select the input source for the EXINT13 external interrupt.<br/>0000: GPIOA pin13<br/>0001: GPIOB pin13<br/>0010: GPIOC pin13<br/>0011: GPIOD pin13<br/>0100: GPIOE pin13<br/>0101: GPIOF pin13<br/>0110: GPIOG pin13<br/>Others: Reserved |
| Bit 3: 0 | EXINT12 | 0x0000 | rw | EXINT12 input source configuration<br/>These bits are used to select the input source for the EXINT12 external interrupt.<br/>0000: GPIOA pin12<br/>0001: GPIOB pin12<br/>0010: GPIOC pin12<br/>0011: GPIOD pin12<br/>0100: GPIOE pin12<br/>0101: GPIOF pin12<br/>0110: GPIOG pin12<br/>Others: Reserved |


## 7.2.7 SCFG ultra high sourcing/sinking strength (SCFG_UHDRV)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                              |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 11 | Reserved | 0x0000 00   | resd | Kept at its default value                                                                                                                                                                                                                                                                                                |
| Bit 10     | PF15\_UH | 0x0         | rw   | PF15 Ultra high sourcing/sinking strength<br/>This bit is written by software to control the PF15 PAD sourcing/sinking strength.<br/>0: Not active<br/>1: Corresponding GPIO is switched to ultra-high sourcing/sinking strength<br/>When this bit is set, the control bits of GPIOx\_ODRVR\&GPIOx\_HDRV become invalid. |
| Bit 9      | PF14\_UH | 0x0         | rw   | PF14 Ultra high sourcing/sinking strength<br/>This bit is written by software to control the PF14 PAD sourcing/sinking strength.<br/>0: Not active<br/>1: Corresponding GPIO is switched to ultra-high sourcing/sinking strength<br/>When this bit is set, the control bits of GPIOx\_ODRVR\&GPIOx\_HDRV become invalid. |
| Bit 8      | PD15\_UH | 0x0         | rw   | PD15 Ultra high sourcing/sinking strength<br/>This bit is written by software to control the PD15 PAD sourcing/sinking strength.<br/>0: Not active<br/>1: Corresponding GPIO is switched to ultra-high sourcing/sinking strength<br/>When this bit is set, the control bits of GPIOx\_ODRVR\&GPIOx\_HDRV become invalid. |
| Bit 7      | PD14\_UH | 0x0         | rw   | PD14 Ultra high sourcing/sinking strength<br/>This bit is written by software to control the PD14 PAD sourcing/sinking strength.<br/>0: Not active<br/>1: Corresponding GPIO is switched to ultra-high sourcing/sinking strength<br/>When this bit is set, the control bits of GPIOx\_ODRVR\&GPIOx\_HDRV become invalid. |
| Bit 6      | PD13\_UH | 0x0         | rw   | PD13 Ultra high sourcing/sinking strength<br/>This bit is written by software to control the PD13 PAD sourcing/sinking strength.<br/>0: Not active<br/>1: Corresponding GPIO is switched to ultra-high sourcing/sinking strength<br/>When this bit is set, the control bits of GPIOx\_ODRVR\&GPIOx\_HDRV become invalid. |
| Bit 5      | PD12\_UH | 0x0         | rw   | PD12 Ultra high sourcing/sinking strength<br/>This bit is written by software to control the PD12 PAD sourcing/sinking strength.<br/>0: Not active<br/>1: Corresponding GPIO is switched to ultra-high sourcing/sinking strength<br/>When this bit is set, the control bits of GPIOx\_ODRVR\&GPIOx\_HDRV become invalid. |
| Bit 4: 3   | Reserved | 0x0         | rw   | Kept at its default value                                                                                                                                                                                                                                                                                                |
| Bit 2      | PB10\_UH | 0x0         | rw   | PB10 Ultra high sourcing/sinking strength<br/>This bit is written by software to control the PB10 PAD sourcing/sinking strength.<br/>0: Not active<br/>1: Corresponding GPIO is switched to ultra-high                                                                                                                   |


sourcing/sinking strength
When this bit is set, the control bits of GPIOx_ODRVR&GPIOx_HDRV become invalid.

| Bit 1 | PB9\_UH | 0x0 | rw | PB9 Ultra high sourcing/sinking strength<br/>This bit is written by software to control the PB9 PAD sourcing/sinking strength.<br/>0: Not active<br/>1: Corresponding GPIO is switched to ultra-high sourcing/sinking strength<br/>When this bit is set, the control bits of GPIOx\_ODRVR\&GPIOx\_HDRV become invalid. |
| ----- | ------- | --- | -- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 0 | PB3\_UH | 0x0 | rw | PB3 Ultra high sourcing/sinking strength<br/>This bit is written by software to control the PB3 PAD sourcing/sinking strength.<br/>0: Not active<br/>1: Corresponding GPIO is switched to ultra-high sourcing/sinking strength<br/>When this bit is set, the control bits of GPIOx\_ODRVR\&GPIOx\_HDRV become invalid. |


