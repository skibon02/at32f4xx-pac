
ARTERY logo
AT32F435/437 Series Reference Manual

* For ADC peripherals, the pins of analog channels should be configured as analog mode.
* For I²C peripherals that intend to use pins as bidirectional functions, open-drain mode is required.
* For USB OTGFS_ID pin, configure corresponding IOMUX and enable corresponding clocks in CRM, there is no need of GPIO status configuration.

## 6.2.11 IOMUX map priority

The unique peripheral multiplexed function can be configured through the GPIOx_MUXL/GPIOx_MUXH register, except individual pins that may be directly owned by hardware.

Some pins have been directly owned by specific hardware feature, whatever GPIO configuration.

### Table 6-9 Pins owned by hardware

| Pin  | Enable bit                                                                                 | Description                                           |
| ---- | ------------------------------------------------------------------------------------------ | ----------------------------------------------------- |
| PA0  | PWC\_CTRLSTS\[8] =1<br/>(ERTC\_CTRL\[11]=1&<br/>ERTC\_TAMP\[17]=1)                         | Once enabled, PA0 pin acts as WKUP1 function of PWC.  |
| PA0  | (ERTC\_TAMP\[0]=1&<br/>ERTC\_TAMP\[16]=1)\|<br/>(ERTC\_TAMP\[3]=1)                         | Once enabled, PA0 pin is used as TAMPER2\_BPR.        |
| PA4  | DAC\_CTRL\[2] =1                                                                           | Once enabled, PA4 pin acts as DAC1 analog channel.    |
| PA5  | DAC\_CTRL\[18] =1                                                                          | Once enabled, PA5 pin acts as DAC2 analog channel.    |
| PC13 | PWC\_CTRLSTS\[9] = 1<br/>(ERTC\_CTRL\[23]=1)\|<br/>(ERTC\_CTRL\[22:21]!=00)                | Once enabled, PC13 pin acts as WKUP2 function of PWC. |
| PC13 | (ERTC\_CTRL\[11]=1&<br/>ERTC\_TAMP\[17]=0)\|<br/>(ERTC\_TAMP\[0]=1&<br/>ERTC\_TAMP\[16]=0) | Once enabled, PC13 pin acts as RTC channel.           |
| PC14 | CRM\_BPDC\[0]=1                                                                            | Once enabled, PC14 pin acts as LEXT channel.          |
| PC15 | CRM\_BPDC\[0]=1 &<br/>CRM\_BPDC\[2]=0                                                      | Once enabled, PC15 pin acts as LEXT channel.          |
| PH0  | CRM\_CTRL\[16]=1                                                                           | Once enabled, PH0 pin acts as HEXT channel.           |
| PH1  | CRM\_CTRL\[16]=1&<br/>CRM\_CTRL\[18]=0                                                     | Once enabled, PH1 pin acts as HEXT channel.           |


Note: Either PA0 or PC13 cannot be used as TAMPER_BPR and WKUP of PWC simultaneously.

## 6.2.12 External interrupt/wake-up lines

Each pin can be used as an external interrupt input. The corresponding pin should be configured as input mode.

## 6.3 GPIO registers

Table 6-10 lists GPIO register map and their reset values. These peripheral registers must be accessed by bytes (8 bits), half-words (16 bits) or words (32 bits).

### Table 6-10 GPIO register map and reset values

| Register               | Offset | Reset value                    |
| ---------------------- | ------ | ------------------------------ |
| GPIOA\_CFGR            | 0x00   | 0xA800 0000                    |
| GPIOx\_CFGR(x =B,C,F)  | 0x00   | 0x0000 0280(B)<br/>0x0000 0000 |
| GPIOx\_OMODER          | 0x04   | 0x0000 0000                    |
| GPIOx\_ODRVR           | 0x08   | 0x0000 00C0(B)<br/>0x0000 0000 |
| GPIOA\_PULL            | 0x0C   | 0x6400 0000(A)                 |
| GPIOx\_PULL(x = B,C,F) | 0x0C   | 0x0000 0100(B)<br/>0x0000 0000 |
| GPIOx\_IDT             | 0x10   | 0x0000 XXXX                    |
| GPIOx\_ODT             | 0x14   | 0x0000 0000                    |


2025.05.28
Page 139
Rev 2.07




ARTERY logo # AT32F435/437 Series Reference Manual

| GPIOx\_SCR  | 0x18 | 0x0000 0000 |
| ----------- | ---- | ----------- |
| GPIOx\_WPR  | 0x1C | 0x0000 0000 |
| GPIOx\_MUXL | 0x20 | 0x0000 0000 |
| GPIOx\_MUXH | 0x24 | 0x0000 0000 |
| GPIOx\_CLR  | 0x28 | 0x0000 0000 |
| GPIOx\_HDRV | 0x3C | 0x0000 0000 |


## 6.3.1 GPIO configuration register (GPIOx_CFGR) (x=A..H )

**Address offset**: 0x00

**Reset values**: 0xa8000000 for port A, 0x0000 0280 for port B, 0x00000000 for other ports

| Bit          | Name  | Reset value | Type | Description                                                                                                                                                   |
| ------------ | ----- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 2y+1: 2y | IOMCy | 0xA800 0000 | rw   | GPIOx mode configuration (y=0\~15)<br/>00: Input mode (reset state)<br/>01: General-purpose output mode<br/>10: Multiplexed function mode<br/>11: Analog mode |


## 6.3.2 GPIO output mode register (GPIOx_OMODE) (x=A..H)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                         |
| ---------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Always 0.                                                                                                                                           |
| Bit 15: 0  | OM       | 0x0000      | rw   | GPIOx output mode configuration<br/>These field is used to configure the output mode of the GPIOx:<br/>0: Push-pull (reset state)<br/>1: Open-drain |


## 6.3.3 GPIO drive capability register (GPIOx_ODRVR) (x=A..H)

**Address offset**: 0x08

**Reset values**: 0x0000 00C0 for port B 0x00000000 for other ports

| Bit          | Name  | Reset value | Type | Description                                                                                                                                                                                                                 |
| ------------ | ----- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 2y+1: 2y | ODRVy | 0x0000 0000 | rw   | GPIOx drive capability (y=0…15)<br/>This field is used to configure the IO port drive capability.<br/>x0: Normal sourcing/sinking strength<br/>01: Large sourcing/sinking strength<br/>11: Normal sourcing/sinking strength |


## 6.3.4 GPIO pull-up/pull -down register (GPIOx_PULL) (x=A..H)

**Address offset**: 0x0C

**Reset values**: 0x6400 0000 for port A, 0x0000 0100 for port B, 0x0000000 for other ports

| Bit          | Name  | Reset value | Type | Description                                                                                                                                                                              |
| ------------ | ----- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 2y+1: 2y | PULLy | 0x6400 0000 | rw   | GPIOx pull-up/pull-down configuration (y=0…15)<br/>This field is used to configure the pull-up/pull-down of the IO port.<br/>00: No pull-up, pull-down<br/>01: Pull-up<br/>10: Pull-down |


## 6.3.5 GPIO input register (GPIOx_IDH) (x=A..H)

| Bit        | Name     | Reset value | Type | Description                                                                                  |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value.                                                                   |
| Bit 15: 0  | IDT      | 0xXXXX      | ro   | GPIOx input data<br/>Indicates the input status of I/O port. Each bit corresponds to an I/O. |


 2025.05.28
 Page 140
 Rev 2.07




ARTERY logo **AT32F435/437 Series Reference Manual**

## 6.3.6 GPIO output register (GPIOx_IDH) (x=A..H)

| Bit        | Name     | Reset value | Type | Description                                                                                                                   |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                                    |
| Bit 15: 0  | ODT      | 0x0000      | rw   | GPIOx output data<br/>Each bit represents an I/O port.<br/>It indicates the output status of I/O port.<br/>0: Low<br/>1: High |


## 6.3.7 GPIO set/clear register (GPIOx_SCR) (x=A..H)

| Bit        | Name | Reset value | Type | Description                                                                                                                                                                                                                                                                                        |
| ---------- | ---- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | IOCB | 0x0000      | wo   | GPIOx clear bit<br/>The corresponding ODT register bit is cleared by writing “1” to these bits. Otherwise, the corresponding ODT register bit remains unchanged, which acts as ODT register bit operations.<br/>0: No action to the corresponding ODT bits<br/>1: Clear the corresponding ODT bits |
| Bit 15: 0  | IOSB | 0x0000      | wo   | GPIOx set bit<br/>The corresponding ODT register bit is set by writing “1” to these bits. Otherwise, the corresponding ODT register bit remains unchanged, which acts as ODT register bit operations.<br/>0: No action to the corresponding ODT bits<br/>1: Set the corresponding ODT bits         |


## 6.3.8 GPIO write protection register (GPIOx_WPR) (x=A..H)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                           |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 17 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                            |
| Bit 16     | WPSEQ    | 0x0         | rw   | Write protect sequence<br/>Write protect enable sequence bit and WPEN bit must be enabled at the same time to achieve write protection for some I/O bits.<br/>Write protect enable bit is executed four times in the order below: write “1” -> write “0” -> write “1” -> read. Note that the value of WPEN bit cannot be modified during this period. |
| Bit 15: 0  | WPEN     | 0x0000      | rw   | Write protect enable<br/>Each bit corresponds to an I/O port.<br/>0: No effect.<br/>1: Write protect                                                                                                                                                                                                                                                  |


2025.05.28 Page 141 Rev 2.07




ARTERY logo

# AT32F435/437 Series Reference Manual

## 6.3.9 GPIO multiplexed function low register (GPIOx_MUXL) (x=A..H)

**Address offset**: 0x20

**Reset value**: 0x00000000

| Bit          | Name  | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                               |
| ------------ | ----- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 4y+3: 4y | MUXLy | 0x0         | rw   | Multiplexed function select for GPIOx pin y (y=0...7)<br/>This field is used to configure multiplexed function IOs.<br/>0000: MUX0<br/>0001: MUX1<br/>0010: MUX2<br/>0011: MUX3<br/>0100: MUX4<br/>0101: MUX5<br/>0110: MUX6<br/>0111: MUX7<br/>1000: MUX8<br/>1001: MUX9<br/>1010: MUX10<br/>1011: MUX11<br/>1100: MUX12<br/>1101: MUX13<br/>1110: MUX14<br/>1111: MUX15 |


## 6.3.10 GPIO multiplexed function high register (GPIOx_MUXH) (x=A..H)

| Bit          | Name  | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                               |
| ------------ | ----- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 4y+3: 4y | MUXHy | 0x0         | rw   | Multiplexed function select for GPIOx pin y (y=8...15)<br/>This field is used to configure multiplexed function IOs<br/>0000: MUX0<br/>0001: MUX1<br/>0010: MUX2<br/>0011: MUX3<br/>0100: MUX4<br/>0101: MUX5<br/>0110: MUX6<br/>0111: MUX7<br/>1000: MUX8<br/>1001: MUX9<br/>1010: MUX10<br/>1011: MUX11<br/>1100: MUX12<br/>1101: MUX13<br/>1110: MUX14<br/>1111: MUX15 |


## 6.3.11 GPIO port bit clear register (GPIOx_CLR) (x=A..H)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                        |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                                                                                                                                                                                                         |
| Bit 15: 0  | IOCB     | 0x0000      | wo   | GPIOx clear bit<br/>The corresponding ODT register bit is cleared by writing “1” to these bits. Otherwise, the corresponding ODT register bit remains unchanged, which acts as ODT register bit operations.<br/>0: No action to the corresponding ODT bits<br/>1: Clear the corresponding ODT bits |


2025.05.28
Page 142
Rev 2.07
