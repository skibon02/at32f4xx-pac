

8-bit right alignment: load data into the DAC_DDTH8R [7: 0] and DAC_DDTH8R [15: 8]
12-bit left alignment: load data into the DAC_DDTH12L [15: 4] and DAC_DDTH12L [31: 20]
12-bit right alignment: load data into the DAC_DDTH12R [11: 0] and DAC_DDTH12R [27: 16]
The loaded 8-bit data corresponds to the DHRx[11:4] and the loaded 12-bit data corresponds to the DHRx[11: 0]

# 19.5 DAC registers

These peripheral registers must be accessed by words (32 bits).

Table 19-2 DAC register map and reset values

| Register      | Offset | Reset value |
| ------------- | ------ | ----------- |
| DAC\_CTRL     | 000h   | 0x0000 0000 |
| DAC\_SWTRG    | 004h   | 0x0000 0000 |
| DAC\_D1DTH12R | 008h   | 0x0000 0000 |
| DAC\_D1DTH12L | 00Ch   | 0x0000 0000 |
| DAC\_D1DTH8R  | 010h   | 0x0000 0000 |
| DAC\_D2DTH12R | 014h   | 0x0000 0000 |
| DAC\_D2DTH12L | 018h   | 0x0000 0000 |
| DAC\_D2DTH8R  | 01Ch   | 0x0000 0000 |
| DAC\_DDTH12R  | 020h   | 0x0000 0000 |
| DAC\_DDTH12L  | 024h   | 0x0000 0000 |
| DAC\_DDTH8R   | 028h   | 0x0000 0000 |
| DAC\_D1ODT    | 02Ch   | 0x0000 0000 |
| DAC\_D2ODT    | 030h   | 0x0000 0000 |
| DAC\_STS      | 034h   | 0x0000 0000 |


## 19.5.1 DAC control register (DAC_CTRL)

| Bit        | Name        | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ---------- | ----------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 30 | Reserved    | 0x0         | resd | Kept at its default value                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Bit 29     | D2DMAUDRIEN | 0x0         | rw   | DAC2 DMA transfer underrun interrupt enable<br/>This bit is set and cleared by software.<br/>0: DAC2 DMA transfer underrun interrupt disabled<br/>1: DAC2 DMA transfer underrun interrupt enabled                                                                                                                                                                                                                                                                                                                                                             |
| Bit 28     | D2DMAEN     | 0x0         | rw   | DAC2 DMA transfer enable<br/>This bit is set and cleared by software.<br/>0: DAC2 DMA mode disabled<br/>1: DAC2 DMA mode enabled                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Bit 27: 24 | D2NBSEL     | 0x0         | rw   | DAC2 noise bit select<br/>These bits are used to select the mark bit in noise generation mode or amplitude in triangular-wave generation mode.<br/>0000: Unmask LSFR bit0 /Triangle amplitude is equal to 1<br/>0001: Unmask LSFR bit\[1: 0] /Triangle amplitude is equal to 3<br/>0010: Unmask LSFR bit\[2: 0] /Triangle amplitude is equal to 7<br/>0011: Unmask LSFR bit\[3: 0] /Triangle amplitude is equal to 15<br/>0100: Unmask LSFR bit\[4: 0] /Triangle amplitude is equal to 31<br/>0101: Unmask LSFR bit\[5: 0] /Triangle amplitude is equal to 63 |
|            |             |     |      | 0110: Unmask LSFR bit\[6: 0] /Triangle amplitude is equal to 127<br/>0111: Unmask LSFR bit\[7: 0] /Triangle amplitude is equal to 255<br/>1000: Unmask LSFR bit\[8: 0] /Triangle amplitude is equal to 511<br/>1001: Unmask LSFR bit\[9: 0] /Triangle amplitude is equal to 1023<br/>1010: Unmask LSFR bit\[10:0] /Triangle amplitude is equal to 2047<br/>≥1011: Unmask LSFR bit\[11: 0] /Triangle amplitude is equal to 4095                                                                                                                                                               |
| Bit 23: 22 | D2NM        | 0x0 | rw   | DAC2 noise mode<br/>00: Wave generation disabled<br/>01: Noise wave generation enabled<br/>1x: Triangular wave generation enabled                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Bit 21: 19 | D2TRGSEL    | 0x0 | rw   | DAC2 trigger select<br/>000: TMR6 TRGOUT event<br/>001: TMR8 TRGOUT event<br/>010: TMR7 TRGOUT event<br/>011: TMR5 TRGOUT event<br/>100: TMR2 TRGOUT event<br/>101: TMR4 TRGOUT event<br/>110: External interrupt line 9<br/>111: Software trigger<br/>Note: These bits can be valid only when D2TRGEN = 1.                                                                                                                                                                                                                                                                                  |
| Bit 18     | D2TRGEN     | 0x0 | rw   | DAC2 trigger enable<br/>0: DAC2 trigger disabled<br/>1: DAC2 trigger enabled<br/>Note:<br/>When the DAC2 trigger is disabled, the data written into the DAC\_D2DTHx register is transferred into the DAC\_D2ODT register after one APB1 clock cycle.<br/>When the DAC2 trigger is enabled, the data written into the DAC\_D2DTHx register is transferred into the DAC\_D2ODT register after three APB1 clock cycles.<br/>If the software trigger is selected, it takes one APB1 clock cycle to have the data written into the DAC\_D2DTHx register transferred into the DAC\_D2ODT register. |
| Bit 17     | D2OBDIS     | 0x0 | rw   | DAC2 output buffer disable<br/>0: DAC2 output buffer enabled<br/>1: DAC2 output buffer disabled                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Bit 16     | D2EN        | 0x0 | rw   | DAC2 enable<br/>0: DAC2 disabled<br/>1: DAC2 enabled                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Bit 15: 14 | Reserved    | 0x0 | resd | Kept at its default value                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Bit 13     | D1DMAUDRIEN | 0x0 | rw   | DAC1 DMA transfer underrun interrupt enable<br/>This bit is set and cleared by software.<br/>0: DAC1 DMA transfer underrun interrupt disabled<br/>1: DAC1 DMA transfer underrun interrupt enabled                                                                                                                                                                                                                                                                                                                                                                                            |
| Bit 12     | D1DMAEN     | 0x0 | rw   | DAC1 DMA transfer enable<br/>0: DAC1 DMA transfer disabled<br/>1: DAC1 DMA transfer enabled                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Bit 11: 8  | D1NBSEL     | 0x0 | rw   | DAC1 noise bit select<br/>These bits are used to select the mark bit in noise generation mode or amplitude in triangular-wave generation mode.<br/>0000: Unmask LSFR bit0/Triangle amplitude is equal to 1<br/>0001: Unmask LSFR bit\[1:0]/Triangle amplitude is equal to 3<br/>0010: Unmask LSFR bit\[2: 0]/Triangle amplitude is equal to 7<br/>0011: Unmask LSFR bit\[3: 0]/Triangle amplitude is equal to 15                                                                                                                                                                             |
|          |          |     |    | 0100: Unmask LSFR bit\[4: 0]/Triangle amplitude is equal to 31<br/>0101: Unmask LSFR bit\[5: 0]/Triangle amplitude is equal to 63<br/>0110: Unmask LSFR bit\[6: 0]/Triangle amplitude is equal to 127<br/>0111: Unmask LSFR bit\[7: 0]/Triangle amplitude is equal to 255<br/>1000: Unmask LSFR bit\[8: 0]/Triangle amplitude is equal to 511<br/>1001: Unmask LSFR bit\[9: 0]/Triangle amplitude is equal to 1023<br/>1010: Unmask LSFR bit\[10: 0]/Triangle amplitude is equal to 2047<br/>≥1011: Unmask LSFR bit\[11:0]/Triangle amplitude is equal to 4095                               |
| Bit 7: 6 | D1NM     | 0x0 | rw | DAC1 noise mode<br/>00: Wave generation disabled<br/>01: Noise wave generation enabled<br/>1x: Triangular wave generation enabled                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Bit 5: 3 | D1TRGSEL | 0x0 | rw | DAC1 trigger select<br/>000: TMR6 TRGOUT event<br/>001: TMR8 TRGOUT event<br/>010: TMR7 TRGOUT event<br/>011: TMR5 TRGOUT event<br/>100: TMR2 TRGOUT event<br/>101: TMR4 TRGOUT event<br/>110: External interrupt line 9<br/>111: Software trigger<br/>Note: These bits can be valid only when D1TRGEN = 1.                                                                                                                                                                                                                                                                                  |
| Bit 2    | D1TRGEN  | 0x0 | rw | DAC1 trigger enable<br/>0: DAC1 trigger disabled<br/>1: DAC1 trigger enabled<br/>Note:<br/>When the DAC1 trigger is disabled, the data written into the DAC\_D1DTHx register is transferred into the DAC\_D1ODT register after one APB1 clock cycle.<br/>When the DAC1 trigger is enabled, the data written into the DAC\_D1DTHx register is transferred into the DAC\_D1ODT register after three APB1 clock cycles<br/>If the software trigger is selected, it takes one APB1 clock cycle to have the data written into the DAC\_D1DTHx register transferred into the DAC\_ D1ODT register. |
| Bit 1    | D1OBDIS  | 0x0 | rw | DAC1 output buffer disable<br/>0: DAC1 output buffer enabled<br/>1: DAC1 output buffer disabled                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Bit 0    | D1EN     | 0x0 | rw | DAC1 enable<br/>0: DAC1 disabled<br/>1: DAC1 enabled                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |


### 19.5.2 DAC software trigger register (DAC_SWTRG)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                              |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 2 | Reserved | 0x0000 0000 | resd | Kept at its default value                                                                                                                                                                                                                |
| Bit 1     | D2SWTRG  | 0x0         | rw   | DAC2 software trigger<br/>0: DAC2 software trigger disabled<br/>1: DAC2 software trigger enabled<br/>Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC\_D2DTH data is loaded into the DAC\_D2ODT register. |
| Bit 0     | D1SWTRG  | 0x0         | rw   | DAC1 software trigger<br/>0: DAC1 software trigger disabled<br/>1: DAC1 software trigger enabled<br/>Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC\_D1DTH data is loaded into the DAC\_D1ODT register. |


### 19.5.3 DAC1 12-bit right-aligned data holding register (DAC_D1DTH12R)

| Bit        | Name     | Reset value | Type | Description                    |
| ---------- | -------- | ----------- | ---- | ------------------------------ |
| Bit 31: 12 | Reserved | 0x00000     | resd | Kept at its default value      |
| Bit 11: 0  | D1DT12R  | 0x000       | rw   | DAC1 12-bit right-aligned data |


### 19.5.4 DAC1 12-bit left-aligned data holding register (DAC_D1DTH12L)

| Bit        | Name     | Reset value | Type | Description                   |
| ---------- | -------- | ----------- | ---- | ----------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value     |
| Bit 15: 4  | D1DT12L  | 0x000       | rw   | DAC1 12-bit left-aligned data |
| Bit 3: 0   | Reserved | 0x0         | resd | Kept at its default value     |


### 19.5.5 DAC1 8-bit right-aligned data holding register (DAC_D1DTH8R)

| Bit       | Name     | Reset value | Type | Description                   |
| --------- | -------- | ----------- | ---- | ----------------------------- |
| Bit 31: 8 | Reserved | 0x000000    | resd | Kept at its default value     |
| Bit 7: 0  | D1DT8R   | 0x00        | rw   | DAC1 8-bit right-aligned data |


### 19.5.6 DAC2 12-bit right-aligned data holding register (DAC_D2DTH12R)

| Bit        | Name     | Reset value | Type | Description                    |
| ---------- | -------- | ----------- | ---- | ------------------------------ |
| Bit 31: 12 | Reserved | 0x00000     | resd | Kept at its default value      |
| Bit 11: 0  | D2DT12R  | 0x000       | rw   | DAC2 12-bit right-aligned data |


### 19.5.7 DAC2 12-bit left-aligned data holding register (DAC_D2DTH12L)

| Bit        | Name     | Reset value | Type | Description                   |
| ---------- | -------- | ----------- | ---- | ----------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value     |
| Bit 15: 4  | D2DT12L  | 0x000       | rw   | DAC2 12-bit left-aligned data |
| Bit 3: 0   | Reserved | 0x0         | resd | Kept at its default value     |


### 19.5.8 DAC2 8-bit right-aligned data holding register (DAC_ D2DTH8R)

| Bit       | Name     | Reset value | Type | Description                   |
| --------- | -------- | ----------- | ---- | ----------------------------- |
| Bit 31: 8 | Reserved | 0x000000    | resd | Kept at its default value     |
| Bit 7: 0  | D2DT8R   | 0x00        | rw   | DAC2 8-bit right-aligned data |


### 19.5.9 Dual DAC 12-bit right-aligned data holding register (DAC_ DDTH12R)

| Bit        | Name     | Reset value | Type | Description                    |
| ---------- | -------- | ----------- | ---- | ------------------------------ |
| Bit 31: 28 | Reserved | 0x0         | resd | Kept at its default value      |
| Bit 27: 16 | DD2DT12R | 0x000       | rw   | DAC2 12-bit right-aligned data |
| Bit 15: 12 | Reserved | 0x0         | resd | Kept at its default value      |
| Bit 11: 0  | DD1DT12R | 0x000       | rw   | DAC1 12-bit right-aligned data |


### 19.5.10 Dual DAC 12-bit left-aligned data holding register (DAC_ DDTH12L)

| Bit        | Name     | Reset value | Type | Description                   |
| ---------- | -------- | ----------- | ---- | ----------------------------- |
| Bit 31: 20 | DD2DT12L | 0x000       | rw   | DAC2 12-bit left-aligned data |
| Bit 19: 16 | Reserved | 0x0         | resd | Kept at its default value     |
| Bit 15: 4  | DD1DT12L | 0x000       | rw   | DAC1 12-bit left-aligned data |
| Bit 3: 0   | Reserved | 0x0         | resd | Kept at its default value     |


### 19.5.11 Dual DAC 8-bit right-aligned data holding register (DAC_ DDTH8R)

| Bit        | Name     | Reset value | Type | Description                   |
| ---------- | -------- | ----------- | ---- | ----------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value     |
| Bit 15: 8  | DD2DT8R  | 0x00        | rw   | DAC2 8-bit right-aligned data |
| Bit 7: 0   | DD1DT8R  | 0x00        | rw   | DAC1 8-bit right-aligned data |


### 19.5.12 DAC1 data output register (DAC_ D1ODT)

| Bit        | Name     | Reset value | Type | Description               |
| ---------- | -------- | ----------- | ---- | ------------------------- |
| Bit 31: 12 | Reserved | 0x00000     | resd | Kept at its default value |
| Bit 11: 0  | D1ODT    | 0x000       | rw   | DAC1 output data          |


### 19.5.13 DAC2 data output register (DAC_ D2ODT)

| Bit        | Name     | Reset value | Type | Description               |
| ---------- | -------- | ----------- | ---- | ------------------------- |
| Bit 31: 12 | Reserved | 0x00000     | resd | Kept at its default value |
| Bit 11: 0  | D2ODT    | 0x000       | rw   | DAC2 output data          |


