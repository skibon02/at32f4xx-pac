

* Select an trigger mode by setting EXINT_POLCFG1 and EXINT_POLCFG2 register.
* Enable interrupt or event by setting EXINT_INTEN and EXINT_EVTEN register.
* Generate software trigger by setting EXINT_SWTRG register (This is applied to only software trigger interrupt).

**Interrupt clear procedure**

* Writing “1” to the EXINT_INTSTS register to clear the interrupts generated, and the corresponding bits in the EXINT_SWTRG register.

## 8.3 EXINT registers

These peripheral registers must be accessed by words (32 bits).

Table 8-1 shows EXINT register map and their reset value.

Table 8-1 External interrupt/Event controller register map and reset value

| Register        | Offset | Reset value |
| --------------- | ------ | ----------- |
| EXINT\_INTEN    | 0x00   | 0x0000 0000 |
| EXINT\_EVTEN    | 0x04   | 0x0000 0000 |
| EXINT\_ POLCFG1 | 0x08   | 0x0000 0000 |
| EXINT\_ POLCFG2 | 0x0C   | 0x0000 0000 |
| EXINT\_ SWTRG   | 0x10   | 0x0000 0000 |
| EXINT\_ INTSTS  | 0x14   | 0x0000 0000 |


### 8.3.1 Interrupt enable register (EXINT_INTEN)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                   |
| ---------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 23 | Reserved | 0x000       | resd | Forced to 0 by hardware.                                                                                                                                                                      |
| Bit 22: 0  | INTENx   | 0x00000     | rw   | Interrupt enable or disable on line x<br/>0: Interrupt request is disabled.<br/>1: Interrupt request is enabled.<br/>Note: Bit 19 is applied to only AT32F434/437A and is reserved otherwise. |


### 8.3.2 Event enable register (EXINT_EVTEN)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                       |
| ---------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 23 | Reserved | 0x000       | resd | Forced to 0 by hardware.                                                                                                                                                          |
| Bit 22: 0  | EVTENx   | 0x00000     | rw   | Event enable or disable on line x<br/>0: Event request is disabled.<br/>1: Event request is enabled.<br/>Note: Bit 19 is applied to only AT32F437/437a and is reserved otherwise. |


### 8.3.3 Polarity configuration register 1 (EXINT_ POLCFG1)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                   |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 23 | Reserved | 0x000       | resd | Forced to 0 by hardware.                                                                                                                                                                                                                                                                                      |
| Bit 22: 0  | RPx      | 0x00000     | rw   | Rising polarity configuration bit on line x<br/>These bits are used to select a rising edge to trigger an interrupt and event on line x.<br/>0: Rising trigger on line x is disabled.<br/>1: Rising trigger on line x is enable.<br/>Note: Bit 19 is applied to only AT32F437/437A and is reserved otherwise. |


## 8.3.4 Polarity configuration register 2 (EXINT_ POLCFG2)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                        |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 23 | Reserved | 0x000       | resd | Forced to be 0 by hardware.                                                                                                                                                                                                                                                                                        |
| Bit 22: 0  | FPx      | 0x00000     | rw   | Falling polarity configuration bit on line x<br/>These bits are used to select a falling edge to trigger an interrupt and event on line x.<br/>0: Falling trigger on line x is disabled.<br/>1: Falling trigger on line x is enabled.<br/>Note: Bit 19 is applied to only AT32F437/437A and is reserved otherwise. |


## 8.3.5 Software trigger register (EXINT_ SWTRG)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 23 | Reserved | 0x000       | resd | Forced to 0 by hardware.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| Bit 22: 0  | SWTx     | 0x00000     | rw   | Software trigger on line x<br/>If the corresponding bit in EXINT\_INTEN register is 1, the software writes to this bit. The hardware sets the corresponding bit in the EXINT\_INTSTS automatically to generate an interrupt.<br/>If the corresponding bit in the EXINT\_EVTEN register is 1, the software writes to this bit. The hardware generates an event on the corresponding interrupt line automatically.<br/>0: Default value<br/>1: Software trigger generated<br/>Note: This bit is cleared by writing 1 to the corresponding bit in the EXINT\_INTSTS register.<br/>Note: Bit 19 is applied to only AT32F437/437A and is reserved otherwise. |


## 8.3.6 Interrupt status register (EXINT_ INTSTS)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                               |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 23 | Reserved | 0x000       | resd | Forced to 0 by hardware.                                                                                                                                                                  |
| Bit 22: 0  | LINEx    | 0x00000     | Rw1c | Line x status bit<br/>0: No interrupt occurred.<br/>1: Interrupt occurred.<br/>Note: This bit is cleared by writing 1. Bit 19 is applied to only AT32F437/437A and is reserved otherwise. |


