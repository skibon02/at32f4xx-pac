

# 30 Debug (DEBUG)

## 30.1 Debug introduction

Cortex®-M4F core provides powerful debugging features including halt and single step support, as well as trace function that is used for checking the details of the program execution. The debug features are implemented with two interfaces: serial wire debug (SWD) and JTAG debug port. Trace information is collected by a single-wire serial wire view interface, or by TRACE interface when a larger trace bandwidth is needed. Trace and debugging interfaces can be combined into one interface.

ARM Cortex®-M4F reference documentation:

* Cortex®-M4 Technical Reference Manual (TRM)
* ARM Debug Interface V5
* ARM CoreSight Design Kit revision r1p0 Technical Reference Manual

## 30.2 Debug and Trace

It is possible to support debugging for different peripherals, and configure the status of peripherals during debugging. For timers and watchdogs, the user can select whether or not to stop or continue counting during debugging; For CAN, the user can select whether or not to stop or continue updating receive registers during debugging; For I²C, the user can select whether or not to stop or continue SMBUS timeout counting.

In addition, code debugging is supported in Low-power mode. In Sleep mode, the clock programmed by code remains active for HCLK and FCLK to continue to work. In DeepSleep mode, HICK oscillator is enabled to feed FCLK and HCLK.

There are several ID codes inside the MCU, which is accessible by the debugger using the DEBUG_IDCODE at address 0xE0042000. It is part of the DEBUG and is mapped on the external PPB bus. These codes are accessible using the JTAG debug port or the SWD debug port or by the user software. They are even accessible while the MCU is under system reset.

Two trace interface modes supported: single-pin mode for serial wire view and multi-pin trace interface.

## 30.3 I/O pin control

SWJ-DP debug is supported in different packages of AT32F435/437. It uses 5 general-purpose I/O ports. After reset, the SWJ-DP can be immediately used by the debugger as a default function. To ensure that JTAG input pins are not floating (particularly SWCLK/JTCK), the JTAG input pins are embedded with internal pull-up or pull-down feature, NJTRST, JTDI and JTMS/SWDIO with internal pull-up feature, and JTCK/SWCLK with internal pull-down feature.

GPIO and IOMUX registers can be configured to allow users to switch between debug ports or disable debug feature.

## 30.4 DEGUB registers

Table 30-1 shows DEBUG register map and reset values.

These peripheral registers must be accessed by words (32 bits).

Table 30-1 DEBUG register address and reset value

| Register           | Offset      | Reset value |
| ------------------ | ----------- | ----------- |
| DEBUG\_IDCODE      | 0xE004 2000 | 0xXXXX XXXX |
| DEBUG\_CTRL        | 0xE004 2004 | 0x0000 0000 |
| DEBUG\_APB1\_PAUSE | 0xE004 2008 | 0x0000 0000 |
| DEBUG\_APB2\_PAUSE | 0xE004 200C | 0x0000 0000 |
| DEBUG\_SER\_ID     | 0xE004 2020 | 0x0000 XX0X |


# 30.4.1 DEBUG device ID (DEBUG_IDCODE)

MCU integrates an ID code that is used to identify MCU’s revision code. The DEBUG_IDCODE register is mapped on the external PPB bus at address 0xE0042000. This code is accessible by the JTAG debug port or SW debug port or by the user code.

| Bit       | Name | Reset value | Type | Description     |
| --------- | ---- | ----------- | ---- | --------------- |
| Bit 31: 0 | PID  | 0xXXXX XXXX | ro   | PID information |


| PID \[31: 0] | AT32 part number | FLASH size | Packages |
| ------------ | ---------------- | ---------- | -------- |
| 0x7008\_4540 | AT32F435ZMT7     | 4032KB     | LQFP144  |
| 0x7008\_3341 | AT32F435ZGT7     | 1024KB     | LQFP144  |
| 0x7008\_4598 | AT32F435ZDT7     | 448KB      | LQFP144  |
| 0x7008\_3242 | AT32F435ZCT7     | 256KB      | LQFP144  |
| 0x7008\_4543 | AT32F435VMT7     | 4032KB     | LQFP100  |
| 0x7008\_3344 | AT32F435VGT7     | 1024KB     | LQFP100  |
| 0x7008\_4599 | AT32F435VDT7     | 448KB      | LQFP100  |
| 0x7008\_3245 | AT32F435VCT7     | 256KB      | LQFP100  |
| 0x7008\_4546 | AT32F435RMT7     | 4032KB     | LQFP64   |
| 0x7008\_3347 | AT32F435RGT7     | 1024KB     | LQFP64   |
| 0x7008\_459A | AT32F435RDT7     | 448KB      | LQFP64   |
| 0x7008\_3248 | AT32F435RCT7     | 256KB      | LQFP64   |
| 0x7008\_4549 | AT32F435CMT7     | 4032KB     | LQFP48   |
| 0x7008\_334A | AT32F435CGT7     | 1024KB     | LQFP48   |
| 0x7008\_459B | AT32F435CDT7     | 448KB      | LQFP48   |
| 0x7008\_324B | AT32F435CCT7     | 256KB      | LQFP48   |
| 0x7008\_454C | AT32F435CMU7     | 4032KB     | QFN48    |
| 0x7008\_334D | AT32F435CGU7     | 1024KB     | QFN48    |
| 0x7008\_459C | AT32F435CDU7     | 448KB      | QFN48    |
| 0x7008\_324E | AT32F435CCU7     | 256KB      | QFN48    |
| 0x7008\_454F | AT32F437ZMT7     | 4032KB     | LQFP144  |
| 0x7008\_3350 | AT32F437ZGT7     | 1024KB     | LQFP144  |
| 0x7008\_459D | AT32F437ZDT7     | 448KB      | LQFP144  |
| 0x7008\_3251 | AT32F437ZCT7     | 256KB      | LQFP144  |
| 0x7008\_4552 | AT32F437VMT7     | 4032KB     | LQFP100  |
| 0x7008\_3353 | AT32F437VGT7     | 1024KB     | LQFP100  |
| 0x7008\_459E | AT32F437VDT7     | 448KB      | LQFP100  |
| 0x7008\_3254 | AT32F437VCT7     | 256KB      | LQFP100  |
| 0x7008\_4555 | AT32F437RMT7     | 4032KB     | LQFP64   |
| 0x7008\_3356 | AT32F437RGT7     | 1024KB     | LQFP64   |
| 0x7008\_459F | AT32F437RDT7     | 448KB      | LQFP64   |
| 0x7008\_3257 | AT32F437RCT7     | 256KB      | LQFP64   |


## 30.4.2 DEBUG control register (DEBUG_CTRL)

This register is asynchronously reset by POR Reset (not reset by system reset). It can be written by the debugger under reset.

| Bit      | Name             | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| -------- | ---------------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31:3 | Reserved         | 0x0000 0000 | resd | Always 0.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| Bit 2    | STANDBY\_DEBUG   | 0x0         | rw   | Debug Standby mode control bit<br/>0: The whole 1.2V digital circuit is unpowered in Standby mode<br/>1: The whole 1.2V digital circuit is not unpowered in Standby mode, and the system clock is provided by the internal RC oscillator (HICK)                                                                                                                                                                                                                                                                                                                                          |
| Bit 1    | DEEPSLEEP\_DEBUG | 0x0         | rw   | Debug Deepsleep mode control bit<br/>0: In Deepsleep mode, all clocks in the 1.2V domain are disabled. When exiting from Deepsleep mode, the internal RC oscillator (HICK) is enabled, and HICK is used as the system clock source, and the software must reprogram the system clock according to application requirements.<br/>1: In Deepsleep mode, system clock is provided by the internal RC oscillator (HICK). When exiting from Deepsleep mode, HICK is used as the system clock source, and the software must reprogram the system clock. According to application requirements. |
| Bit 0    | SLEEP\_DEBUG     | 0x0         | rw   | Debug Sleep mode control bit<br/>0: When entering Sleep mode, CPU HCLK clock is disabled, but other clocks remain active. When exiting from Sleep mode, it is not necessary to reprogram the clock system.<br/>1: When entering Sleep mode, all clocks keep running.                                                                                                                                                                                                                                                                                                                     |


## 30.4.3 DEBUG APB1 pause register (DEBUG_ APB1_PAUSE)

This register is asynchronously reset by POR Reset (not reset by system reset). It can be written by the <u>debugger</u> under reset.

| Bit        | Name                 | Reset value | Type | Description                                                                                                             |
| ---------- | -------------------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 29 | Reserved             | 0x0         | resd | Kept at its default value.                                                                                              |
| Bit 28     | I2C3\_SMBUS\_TIMEOUT | 0x0         | rw   | I2C3 pause control bit<br/>0: I2C3 SMBUS timeout control works normally<br/>1: I2C3 SMBUS timeout control stops running |
| Bit 27     | I2C2\_SMBUS\_TIMEOUT | 0x0         | rw   | I2C2 pause control bit<br/>0: I2C2 SMBUS timeout control works normally<br/>1: I2C2 SMBUS timeout control stops running |
| Bit 26     | CAN2\_PAUSE          | 0x0         | rw   | CAN2 pause control bit<br/>0: CAN2 works normally<br/>1: CAN2 receive register pauses (does not receive data)           |
| Bit 25     | CAN1\_PAUSE          | 0x0         | rw   | CAN1 pause control bit<br/>0: CAN1 works normally<br/>1: CAN1 receive register pauses (does not receive data)           |
| Bit 24     | I2C1\_SMBUS\_TIMEOUT | 0x0  | rw   | I²C1 pause control bit<br/>0: I²C1 SMBUS timeout control works normally<br/>1: I²C1 SMBUS timeout control stops running |
| Bit 23: 16 | Reserved             | 0x00 | resd | Kept at its default value.                                                                                              |
| Bit 15     | ERTC\_512\_PAUSE     | 0x0  | rw   | ERTC 512Hz output clock pause control bit<br/>0: ERTC 512Hz output clock works normally<br/>1: Froze 512Hz output clock |
| Bit 14: 13 | Reserved             | 0x0  | rw   | Kept at its default value.                                                                                              |
| Bit 12     | WDT\_PAUSE           | 0x0  | rw   | WDT pause control bit<br/>0: WDT works normally<br/>1: WDT stops running                                                |
| Bit 11     | WWDT\_PAUSE          | 0x0  | rw   | WWDT pause control bit<br/>0: WWDT works normally<br/>1: WWDT stops running                                             |
| Bit 10     | ERTC\_PAUSE          | 0x0  | rw   | ERTC pause control bit<br/>0: ERTC works normally<br/>1: ERTC stops running                                             |
| Bit 9      | Reserved             | 0x0  | rw   | Kept at its default value.                                                                                              |
| Bit 8      | TMR14\_PAUSE         | 0x0  | rw   | TMR14 pause control bit<br/>0: TMR14 works normally<br/>1: TMR14 stops running                                          |
| Bit 7      | TMR13\_PAUSE         | 0x0  | rw   | TMR13 pause control bit<br/>0: TMR13 works normally<br/>1: TMR13 stops running                                          |
| Bit 6      | TMR12\_PAUSE         | 0x0  | rw   | TMR12 pause control bit<br/>0: TMR12 works normally<br/>1: TMR12 stops running                                          |
| Bit 5      | TMR7\_PAUSE          | 0x0  | rw   | TMR7 pause control bit<br/>0: TMR7 works normally<br/>1: TMR7 stops running                                             |
| Bit 4      | TMR6\_PAUSE          | 0x0  | rw   | TMR6 pause control bit<br/>0: TMR6 works normally<br/>1: TMR6 stops running                                             |
| Bit 3      | TMR5\_PAUSE          | 0x0  | rw   | TMR5 pause control bit<br/>0: TMR5 works normally<br/>1: TMR5 stops running                                             |
| Bit 2      | TMR4\_PAUSE          | 0x0  | rw   | TMR4 pause control bit<br/>0: TMR4 works normally<br/>1: TMR4 stops running                                             |
| Bit 1      | TMR3\_PAUSE          | 0x0  | rw   | TMR3 pause control bit<br/>0: TMR3 works normally<br/>1: TMR3 stops running                                             |
| Bit 0      | TMR2\_PAUSE          | 0x0  | rw   | TMR2 pause control bit<br/>0: TMR2 works normally<br/>1: TMR2 stops running                                             |


### 30.4.4 DEBUG APB2 pause register (DEBUG_ APB2_PAUSE)

This register is asynchronously reset by POR Reset (not reset by system reset). It can be written by the <u>debugger</u> under reset.

| Bit        | Name         | Reset value | Type | Description                                                                    |
| ---------- | ------------ | ----------- | ---- | ------------------------------------------------------------------------------ |
| Bit 31: 19 | Reserved     | 0x0000      | resd | Kept at its default value.                                                     |
| Bit 18     | TMR11\_PAUSE | 0x0         | rw   | TMR11 pause control bit<br/>0: TMR11 works normally<br/>1: TMR11 stops running |
| Bit 17     | TMR10\_PAUSE | 0x0         | rw   | TMR10 pause control bit<br/>0: TMR10 works normally<br/>1: TMR10 stops running |
| Bit 16     | TMR9\_PAUSE  | 0x0         | rw   | TMR9 pause control bit<br/>0: TMR9 works normally<br/>1: TMR9 stops running    |
| Bit 15: 7  | Reserved     | 0x000       | resd | Kept at its default value.                                                     |
| Bit 6      | TMR20\_PAUSE | 0x0         | rw   | TMR20 pause control bit<br/>0: TMR20 works normally<br/>1: TMR20 stops running |
| Bit 5: 2   | Reserved     | 0x0         | resd | Kept at its default value.                                                     |
| Bit 1      | TMR8\_PAUSE  | 0x0         | rw   | TMR8 pause control bit<br/>0: TMR8 works normally<br/>1: TMR8 stops running    |
| Bit 0      | TMR1\_PAUSE  | 0x0         | rw   | TMR1 pause control bit<br/>0: TMR2 works normally<br/>1: TMR2 stops running    |


### 30.4.5 DEBUG SERIES ID register (DEBUG_ SER_ID)

DEBUG_SIE_ID register is used to identify MCU part number and its revision code. The DEBUG_IDCODE register is mapped on the external PPB bus. This register is asynchronously reset by POR Reset (not reset by system reset). This code is accessible by the JTAG debug port or SW debug <u>port or by</u> the user code.

| Bit        | Name     | Reset value | Type | Description                                              |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value.                               |
| Bit 15: 8  | SER\_ID  | 0xXX        | ro   | MCU part number ID<br/>AT32F435: 0x0D<br/>AT32F437: 0x0E |
| Bit 7: 3   | Reserved | 0x0X        | resd | Kept at its default value.                               |
| Bit 2: 0   | REV\_ID  | 0xX         | ro   | Revision code<br/>0x0: Revision A                        |


