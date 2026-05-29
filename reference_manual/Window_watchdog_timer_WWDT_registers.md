

Table 15-1 Minimum and maximum timeout value when PCLK1=72 MHz

| Prescaler | Min. Timeout value | Max. Timeout value |
| --------- | ------------------ | ------------------ |
| 0         | 56.5μs             | 3.64ms             |
| 1         | 113.5μs            | 7.28ms             |
| 2         | 227.5μs            | 14.56ms            |
| 3         | 455μs              | 29.12ms            |


Figure 15-2 Window watchdog timing diagram

| CNT\[6: 0] | ... | 55                  | 54 | 53 | 52 | 51 | 50 | 4F | 4E                 | 4D | 4C | 4B | 4A | ... | 41 | 40 | 3F | 55 |
| ---------- | --- | ------------------- | -- | -- | -- | -- | -- | -- | ------------------ | -- | -- | -- | -- | --- | -- | -- | -- | -- |
| WIN\[6: 0] |     |                     |    |    |    |    |    | 4F |                    |    |    |    |    |     |    |    |    |    |
| RLD\[6: 0] |     |                     |    |    |    |    |    | 55 |                    |    |    |    |    |     |    |    |    |    |
|            |     | Refresh not allowed |    |    |    |    |    |    | Refresh the window |    |    |    |    |     |    |    |    |    |
| CNT\[6]    |     |                     |    |    |    |    |    |    |                    |    |    |    |    |     |    |    |    |    |
| Reset      |     |                     |    |    |    |    |    |    |                    |    |    |    |    |     |    |    |    |    |


# 15.4 Debug mode

When the microcontroller enters debug mode (Cortex®-M4F core halted), the WWDT counter stops counting by setting the WWDT_PAUSE in the DEBUG module. Refer to Chapter 30.2 for more information.

# 15.5 WWDT registers

These peripheral registers must be accessed by words (32 bits).

Table 15-2 WWDT register map and reset value

| Register   | Offset | Reset value |
| ---------- | ------ | ----------- |
| WWDT\_CTRL | 0x00   | 0x0000 007F |
| WWDT\_CFG  | 0x04   | 0x0000 007F |
| WWDT\_STS  | 0x08   | 0x0000 0000 |


## 15.5.1 Control register (WWDT_CTRL)

| Bit       | Name     | Reset value | Type | Description                                                                                                                 |
| --------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 8 | Reserved | 0x000000    | resd | Kept at its default value.                                                                                                  |
| Bit 7     | WWDTEN   | 0x0         | rw1s | Window watchdog enable<br/>0: Disabled<br/>1: Enabled<br/>This bit is set by software, but can be cleared only after reset. |
| Bit 6: 0  | CNT      | 0x7F        | rw   | Downcounter<br/>When the counter counts down to 0x3F, a reset is generated.                                                 |


## 15.5.2 Configuration register (WWDT_CFG)

| Bit        | Name     | Reset value | Type | Description                                                                      |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------- |
| Bit 31: 10 | Reserved | 0x000000    | resd | Kept at its default value.                                                       |
| Bit 9      | RLDIEN   | 0x0         | rw1s | Reload counter interrupt<br/>0: Disabled<br/>1: Enabled                          |
| Bit 8: 7   | DIV      | 0x0         | rw   | Clock division value<br/>00: PCLK1 divided by 4096<br/>01: PCLK1 divided by 8192 |
|          |     |      |    | 10: PCLK1 divided by 16384<br/>11: PCLK1 divided by 32768                                                                                                                              |
| Bit 6: 0 | WIN | 0x7F | rw | Window value<br/>If the counter is reloaded while its value is greater than the window register value, a reset is generated. The counter must be reloaded between 0x40 and WIN\[6: 0]. |


## 15.5.3 Status register (WWDT_STS)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                     |
| --------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 1 | Reserved | 0x0000 0000 | resd | Kept at its default value.                                                                                                                      |
| Bit 0     | RLDF     | 0x0         | rw0c | Reload counter interrupt flag<br/>This flag is set when the downcounter reaches 0x40.<br/>'This bit is set by hardware and cleared by software. |


