

# 11.5 I²C interrupt requests

The following table lists all the I²C interrupt requests.

Table 11-7 I²C interrupt requests

| Interrupt event                          | Event flag | Enable control bit |
| ---------------------------------------- | ---------- | ------------------ |
| Address matched                          | ADDRF      | ADDRIEN            |
| Acknowledge failure                      | ACKFAIL    | ACKFAILIEN         |
| Stop condition received                  | STOPF      | STOPIEN            |
| Transmit interrupt state                 | TDIS       | TDIEN              |
| Receive data buffer full                 | RDBF       | RDIEN              |
| Transfer complete, wait for loading data | TCRLD      | TDCIEN             |
| Data transfer complete                   | TDC        |                    |
| SMBus alert                              | ALERTF     | ERRIEN             |
| Timeout error                            | TMOUT      |                    |
| PEC error                                | PECERR     |                    |
| Overrun/Underrun                         | OUF        |                    |
| Arbitration lost                         | ARLOST     |                    |
| Bus error                                | BUSERR     |                    |


# 11.6 I²C debug mode

When the microcontroller enters debug mode (Cortex®-M4 halted), the SMBUS timeout either continues to work or stops, depending on the I2Cx_SMBUS_TIMEOUT configuration bit in the DEBUG module.

# 11.7 I²C registers

These peripheral registers must be accessed by words (32 bits).

<u>Table 11-8 I²C register map and reset values</u>

| Register     | Offset | Reset value |
| ------------ | ------ | ----------- |
| I2C\_CTRL1   | 0x00   | 0x00000000  |
| I2C\_CTRL2   | 0x04   | 0x00000000  |
| I2C\_OADDR1  | 0x08   | 0x00000000  |
| I2C\_OADDR2  | 0x0C   | 0x00000000  |
| I2C\_CLKCTRL | 0x10   | 0x00000000  |
| I2C\_TIMEOUT | 0x14   | 0x00000000  |
| I2C\_STS     | 0x18   | 0x00000000  |
| I2C\_CLR     | 0x1C   | 0x00000000  |
| I2C\_PEC     | 0x20   | 0x00000000  |
| I2C\_RXDT    | 0x24   | 0x00000000  |
| I2C\_TXDT    | 0x28   | 0x00000000  |


## 11.7.1 Control register 1 (I2C_CTRL1)

| Bit        | Name      | Reset value | Type | Description                                                                                                                                                                                                                      |
| ---------- | --------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31:24  | Reserved  | 0x00        | res  | Kept at its default value.                                                                                                                                                                                                       |
| Bit 23     | PECEN     | 0x0         | rw   | PEC calculation enable<br/>0: PEC calculation disabled<br/>1: PEC calculation enabled                                                                                                                                            |
| Bit 22     | SMBALERT  | 0x0         | rw   | SMBus alert enable / pin set<br/>To enable SMBus master alert feature:<br/>0: SMBus alert disabled<br/>1: SMBus alert enabled<br/>To enable SMBus slave alert address:<br/>0: Pin high<br/>1: Pin low, response address 0001100x |
| Bit 21     | DEVADDREN | 0x0         | rw   | SMBus device default address enable<br/>0: SMBus device default address disabled<br/>1: SMBus device default address enabled, response device default address 1100001x                                                           |
| Bit 20     | HADDREN   | 0x0         | rw   | SMBus host default enable<br/>0: SMBus host address disabled<br/>1: SMBus host address enabled, response host address 0001000x                                                                                                   |
| Bit 19     | GCAEN     | 0x0         | rw   | General call address enable<br/>0: General call address disabled<br/>1: General call address enabled, response 0000000x                                                                                                          |
| Bit 18     | Reserved  | 0x0         | res  | Kept at its default value.                                                                                                                                                                                                       |
| Bit 17     | STRETCH   | 0x0         | rw   | Clock stretching mode<br/>0: Clock stretching mode enabled<br/>1: Clock stretching mode disabled<br/>Note: This bit can be set only when I2C is disabled (I2CEN=0)                                                               |
| Bit 16     | SCTRL     | 0x0         | rw   | Slave receive data control<br/>0: Slave receive data disabled<br/>1: Slave receive data enabled                                                                                                                                  |
| Bit 15     | DMAREN    | 0x0         | rw   | DMA receive data request enable<br/>0: DMA receive data request disabled<br/>1: DMA receive data request enabled                                                                                                                 |
| Bit 14     | DMATEN    | 0x0         | rw   | DMA Transmit data request enable<br/>0: DMA Transmit data request disabled<br/>1: DMA Transmit data request enabled                                                                                                              |
| Bit 13: 12 | Reserved  | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                       |
| Bit 11: 8  | DELT      | 0x0         | rw   | Digital filter value<br/>Filter time = DFLT x TI2C\_CLK<br/>The glitches less than the filter time on the SCL bus will be filtered.<br/>Note: This bit can be set only when I2C is disabled (I2CEN=0)                            |
| Bit 7      | ERRIEN    | 0x0         | rw   | Error interrupt enable<br/>0: Error interrupt disabled<br/>1: Error interrupt enabled                                                                                                                                            |
| Bit 6      | TDCIEN    | 0x0         | rw   | Data transfer complete interrupt enable<br/>0: Data transfer complete interrupt disabled<br/>1: Data transfer complete interrupt enabled                                                                                         |
| Bit 5      | STOPIEN   | 0x0         | rw   | Stop generation complete interrupt enable                                                                                                                                                                                        |
|       |            |     |      | 0: Stop generation complete interrupt disabled<br/>1: Stop generation complete interrupt enabled                       |
| Bit 4 | ACKFAILIEN | 0x0 | rw   | Acknowledge fail interrupt enable<br/>0: Acknowledge fail interrupt disabled<br/>1: Acknowledge fail interrupt enabled |
| Bit 3 | ADDRIEN    | 0x0 | rw   | Address match interrupt enable<br/>0: Address match interrupt disabled<br/>1: Address match interrupt enabled          |
| Bit 2 | RDIEN      | 0x0 | resd | Data receive interrupt enable<br/>0: Data receive interrupt disabled<br/>1: Data receive interrupt enabled             |
| Bit 1 | TDIEN      | 0x0 | rw   | Data transmit interrupt enable<br/>0: Data transmit interrupt disabled<br/>1: Data transmit interrupt enabled          |
| Bit 0 | I2CEN      | 0x0 | rw   | I2C peripheral enable<br/>0: Disabled<br/>1: Enabled                                                                   |


## 11.7.2 Control register 2 (I2C_CTRL2)

| Bit        | Name       | Reset value | Type | Description                                                                                                                                                                                                                |
| ---------- | ---------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 27 | Reserved   | 0x00        | res  | Kept at its default value.                                                                                                                                                                                                 |
| Bit 26     | PECTEN     | 0x0         | rw   | Request PEC transmission enable<br/>0: Transmission disabled<br/>1: Transmission enabled<br/>Note: This bit can be set only when I2C is enabled (I2CEN=1)                                                                  |
| Bit 25     | ASTOPEN    | 0x0         | rw   | Automatically send stop condition enable<br/>0: Disabled (Software sends STOP condition)<br/>1: Enabled (Automatically send STOP condition)                                                                                |
| Bit 24     | RLDEN      | 0x0         | rw   | Send data reload mode enable<br/>0: Send data reload mode disable<br/>1: Send data reload mode enabled                                                                                                                     |
| Bit 23: 16 | CNT\[7: 0] | 0x00        | rw   | Transmit data counter<br/>Note: These bits are invalid when SCTRL=0 in slave mode.<br/>Note: This bit can be set only when I2C is disabled (I2CEN=0)                                                                       |
| Bit 15     | NACKEN     | 0x0         | rw   | Not acknowledge enable<br/>0: Acknowledge enabled<br/>1: Acknowledge disabled<br/>Note: This bit can be set only when I2C is enabled (I2CEN=1)<br/>*Note: This bit can be reset when I2C is disabled (I2CEN=0)*            |
| Bit 14     | GENSTOP    | 0x0         | rw   | Generate stop condition<br/>0: No stop generation<br/>1: stop generation<br/>Note: This bit can be set only when I2C is enabled (I2CEN=1)<br/>Note: This bit is automatically reset when a stop condition is detected.     |
| Bit 13     | GENSTART   | 0x0         | rw   | Generate start condition<br/>0: No start generation<br/>1: Start generation<br/>Note: This bit can be set only when I2C is enabled (I2CEN=1)<br/>Note: This bit is automatically reset when a start condition is detected. |
| Bit 12     | READH10    | 0x0         | rw   | 10-bit address header read enable<br/>0: 10-bit address header read disabled                                                                                                                                               |
| Bit 11   | ADDR10       | 0x0   | rw | 1: 10-bit address header read enabled<br/>Host sends 10-bit address mode enable<br/>0: 7-bit address mode<br/>1: 10-bit address mode |
| Bit 10   | DIR          | 0x0   | rw | Master data transfer direction<br/>0: Transmit<br/>1: Receive                                                                        |
| Bit 9: 0 | SADDR\[9: 0] | 0x000 | rw | Slave address sent by the master<br/>In 7-bit address mode, BIT0 and BIT\[9: 8] don’t care.                                          |


## 11.7.3 Address register 1 (I2C_OADDR1)

| Bit        | Name         | Reset value | Type | Description                                                                                                                                      |
| ---------- | ------------ | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 16 | Reserved     | 0x0000      | rw   | Kept at its default value.                                                                                                                       |
| Bit 15     | ADDR1EN      | 0x0         | rw   | Own Address 1 enable<br/>0: Own Address 1 disabled<br/>1: Own Address 1 enabled                                                                  |
| Bit 14: 11 | Reserved     | 0x0         | res  | Kept at its default value.                                                                                                                       |
| Bit 10     | ADDR1MODE    | 0x0         | rw   | Own Address mode<br/>0: 7-bit address mode<br/>1: 10-bit address mode<br/>Note: This bit can be set only when ADDR1EN is enabled (ADDR1EN=1)     |
| Bit 9: 0   | ADDR1\[9: 0] | 0x000       | rw   | Own address1<br/>In 7-bit address mode, bit 0 and bit \[9: 8] don’t care.<br/>Note: This bit can be set only when ADDR1EN is enabled (ADDR1EN=1) |


## 11.7.4 Own address register 2 (I2C_OADDR2)

| Bit        | Name             | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                       |
| ---------- | ---------------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved         | 0x000       | res  | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                        |
| Bit 15     | ADDR2EN          | 0x0         | rw   | Own address 2 enable<br/>0: Own address 2 disabled<br/>1: Own address 2 enabled                                                                                                                                                                                                                                                                                                                                   |
| Bit 14: 11 | Reserved         | 0x0         | res  | Kept at its default value                                                                                                                                                                                                                                                                                                                                                                                         |
| Bit 10: 8  | ADDR2MASK\[2: 0] | 0x0         | rw   | Own address 2-bit mask<br/>000: Match Address bit \[7: 1]<br/>001: Match Address bit \[7: 2]<br/>010: Match address bit \[7: 3]<br/>011: Match address bit \[7: 4]<br/>100: Match address bit \[7: 5]<br/>101: Match address bit \[7: 6]<br/>110: Match address bit \[7]<br/>111: Response all addresses other than those reserved for I2C<br/>Note: This bit can be set only when ADDR2EN is enabled (ADDR2EN=1) |
| Bit 7: 1   | ADDR2\[7: 1]     | 0x00        | rw   | Own address 2<br/>7-bit address mode<br/>Note: This bit can be set only when ADDR2EN is enabled (ADDR2EN=1)                                                                                                                                                                                                                                                                                                       |
| Bit 0      | Reserved         | 0x0         | res  | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                        |


## 11.7.5 Timing register (I2C_CLKCTRL)

| Bit        | Name        | Reset value | Type | Description                                                     |
| ---------- | ----------- | ----------- | ---- | --------------------------------------------------------------- |
| Bit 31: 28 | DIVL\[3: 0] | 0x0         | rw   | Low 4 bits of clock divider value                               |
| Bit 27: 24 | DIVH\[7: 4] | 0x0         | rw   | High 4 bits of clock divider value<br/>DIV = (DIVH << 4) + DIVL |
| Bit 23: 20 | SCLD\[3: 0] | 0x0         | rw   | SCL output delay<br/>TSCLD = (SCLD + 1) x (DIV + 1) x TI2C\_CLK |
| Bit 19: 16 | SDAD\[3: 0] | 0x0         | rw   | SDA output delay<br/>TSDAD = (SDAD + 1) x (DIV + 1) x TI2C\_CLK |
| Bit 15: 8  | SCLH\[7: 0] | 0x00        | rw   | SCL high level                                                  |
| Bit 7: 0 | SCLL\[7: 0] | 0x00        | rw   | TSCLH = (SCLH + 1) x (DIV + 1) x TI2C\_CLK<br/>SCL low level<br/>TSCLL = (SCLL + 1) x (DIV + 1) x TI2C\_CLK |


## 11.7.6 Timeout register (I2C_TIMEOUT)

| Bit        | Name           | Reset value | Type | Description                                                                                                                                                                                                                                                                                                |
| ---------- | -------------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31     | EXTEN          | 0x0         | rw   | Cumulative clock low extend timeout enable<br/>0: Cumulative clock low extend timeout disabled<br/>1: Cumulative clock low extend timeout enabled<br/>Corresponds to TLOW:SEXT / TLOW:MEXT in SMBus                                                                                                        |
| Bit 30: 28 | Reserved       | 0x0         | res  | Kept at its default value.                                                                                                                                                                                                                                                                                 |
| Bit 27: 16 | EXTTIME\[11:0] | 0x000       | rw   | Cumulative clock low extend timeout value<br/>Timeout duration = (EXTTIME + 1) x 2048 x TI2C\_CLK.<br/>Note: These bits can be set only when EXTEN bit is enabled (EXTEN=1)                                                                                                                                |
| Bit 15     | TOEN           | 0x0         | rw   | Detect clock low/high timeout enable<br/>0: Clock low/high timeout detection disabled<br/>1: clock low/high timeout detection enabled<br/>Corresponds to TTIMEOUT in SMBus.                                                                                                                                |
| Bit 14: 13 | Reserved       | 0x0         | res  | Kept at its default value.                                                                                                                                                                                                                                                                                 |
| Bit 12     | TOMODE         | 0x0         | rw   | Clock timeout detection mode<br/>0: Clock low level detection<br/>1: Clock high level detection<br/>Note: This bit can be set only when TOEN is enabled (TOEN=1).                                                                                                                                          |
| Bit 11: 0  | TOTIME\[11:0]  | 0x000       | rw   | Clock timeout detection time<br/>For clock low level detection (TOMODE = 0):<br/>Timeout duration = (TOTIME + 1) x 2048 x TI2C\_CLK<br/>For clock high level detection (TOMODE = 1):<br/>Timeout duration = (TOTIME + 1) x 4 x TI2C\_CLK<br/>Note: This bit can be set only when TOEN is enabled (TOEN=1). |


## 11.7.7 Status register (I2C_STS)

| Bit        | Name        | Reset value | Type | Description                                                                                                                                                                                        |
| ---------- | ----------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 24 | Reserved    | 0x00        | res  | Kept at its default value.                                                                                                                                                                         |
| Bit 23: 17 | ADDR\[6: 0] | 0x00        | r    | Slave address matching value<br/>In 7-bit address mode: Slave address received<br/>In 10-bit address mode: 10-bit slave address header received                                                    |
| Bit 16     | SDIR        | 0x0         | r    | Slave data transfer direction<br/>0: Receive data<br/>1: Transmit data                                                                                                                             |
| Bit 15     | BUSYF       | 0x0         | r    | Bus busy flag transmission mode<br/>0: Bus idle<br/>1: Bus busy<br/>Once a START condition is detected, this bit is set;<br/>Once a STOP condition is detected, this bit is automatically cleared. |
| Bit 14     | Reserved    | 0x00        | res  | Kept at its default value.                                                                                                                                                                         |
| Bit 13     | ALERTF      | 0x0         | r    | SMBus alert flag<br/>SMBus host: This bit indicates the reception of an alert signal (ALERT pin changes from high to low)<br/>0: No alert signal received<br/>1: Alert signal received             |
| Bit 12     | TMOUT       | 0x0         | r    | SMBus timeout flag<br/>0: No timeout<br/>1: Timeout                                                                                                                                                |
| Bit 11     | PECERR      | 0x0         | r    | PEC receive error flag<br/>0: No PEC error<br/>1: PEC error                                                                                                                                        |
| Bit 10     | OUF         | 0x0         | r    | Overrun or underrun flag<br/>In transmission mode:                                                                                                                                                 |
|       |          |     |      | 0: No overrun or underrun<br/>1: Underrun<br/>In reception mode:<br/>0: No overrun or underrun<br/>1: Overrun                                                                                                                                                                                                                                                                                       |
| Bit 9 | ARLOST   | 0x0 | r    | Arbitration lost flag<br/>0: No arbitration lost detected.<br/>1: Arbitration lost detected.                                                                                                                                                                                                                                                                                                        |
| Bit 8 | BUSERR   | 0x0 | r    | Bus error flag<br/>0: No Bus error occurred<br/>1: Bus error occurred                                                                                                                                                                                                                                                                                                                               |
| Bit 7 | TCRLD    | 0x0 | r    | Data transfer complete, waiting for data load<br/>0: Data transfer is not complete yet<br/>1: Data transfer is complete<br/>This bit is set when data transfer is complete (CNT=1) and reload mode is enabled (RLDEN=1). It is automatically cleared when writing a CNT value.<br/>This bit is applicable in master mode or when SCTRL=1 in slave mode.                                             |
| Bit 6 | TDC      | 0x0 | r    | Data transfer complete flag<br/>0: Data transfer is not completed yet (the shift register still holds data)<br/>1: Data transfer is completed (shift register become empty and all data has been sent to the bus)<br/>This bit is set when ASTOPEN = 0, RLDEN = 0, CNT = 0.<br/>It is automatically cleared after a START or a STOP condition is received.                                          |
| Bit 5 | STOPF    | 0x0 | r    | Stop condition generation complete flag<br/>0: No Stop condition detected.<br/>1: Stop condition detected.                                                                                                                                                                                                                                                                                          |
| Bit 4 | ACKFAIL | 0x0 | r    | Acknowledge failure flag<br/>0: No acknowledge failure<br/>1: Acknowledge failure                                                                                                                                                                                                                                                                                                                   |
| Bit 3 | ADDRF | 0x0 | r    | 0\~7 bit address head match flag<br/>0: 0\~7 bit address head mismatch<br/>1: 0\~7 bit address head match                                                                                                                                                                                                                                                                                           |
| Bit 2 | RDBF     | 0x0 | r    | Receive data buffer full flag<br/>0: Data register has not received data yet<br/>1: Data register has received data                                                                                                                                                                                                                                                                                 |
| Bit 1 | TDIS     | 0x0 | rw1s | Transmit data interrupt status<br/>0: Data has been written to the I2C\_TXDT<br/>1: Data has been sent from the I2C\_TXDT to the shift register. I2C\_TXDT become empty, and thus the to-be-transferred data must be written to the I2C\_TXDT.<br/>When the clock stretching mode is disabled, a TDIS event is generated by writing 1 so that data is written to the I2C\_TXDT register in advance. |
| Bit 0 | TDBE     | 0x1 | r    | Transmit data buffer empty flag<br/>0: I2C\_TXDT not empty<br/>1: I2C\_TXDT empty<br/>This bit is only used to indicate the current status of the I2C\_TXDT register.                                                                                                                               |


# 11.7.8 Status clear register (I2C_CLR)

| Bit        | Name     | Reset value | Type | Description                                                                      |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------- |
| Bit 31: 14 | Reserved | 0x00000     | res  | Kept at its default value.                                                       |
| Bit 13     | ALERTC   | 0x0         | w    | Clear SMBus alert flag<br/>SMBus alert flag is cleared by writing 1.             |
| Bit 12     | TMOUTC   | 0x0         | w    | Clear SMBus timeout flag<br/>SMBus timeout flag is cleared by writing 1.         |
| Bit 11     | PECERRC  | 0x0         | w    | Clear PEC receive error flag<br/>PEC receive error flag is cleared by writing 1. |
| Bit 10   | OUFC     | 0x0 | w   | Clear overload / underload flag<br/>Overload / underload flag is cleared by writing 1.                             |
| Bit 9    | ARLOSTC  | 0x0 | w   | Clear arbitration lost flag<br/>Arbitration lost flag is cleared by writing 1.                                     |
| Bit 8    | BUSERRC  | 0x0 | w   | Clear bus error flag<br/>Bus error flag is cleared by writing 1                                                    |
| Bit 7: 6 | Reserved | 0x0 | res | Kept at its default value.                                                                                         |
| Bit 5    | STOPC    | 0x0 | w   | Clear stop condition generation complete flag<br/>Stop condition generation complete flag is cleared by writing 1. |
| Bit 4    | ACKFAILC | 0x0 | w   | Clear acknowledge failure flag<br/>Acknowledge failure flag is cleared by writing 1.                               |
| Bit 3    | ADDRC    | 0x0 | w   | Clear 0\~7 bit address match flag<br/>0\~7 bit address match flag is cleared by writing 1.                         |
| Bit 2: 0 | Reserved | 0x0 | res | Kept at its default value.                                                                                         |


## 11.7.9 PEC register (I2C_PEC)

| Bit       | Name          | Reset value | Type | Description                |
| --------- | ------------- | ----------- | ---- | -------------------------- |
| Bit 31: 8 | Reserved      | 0x000000    | res  | Kept at its default value. |
| Bit 7: 0  | PECVAL\[7: 0] | 0x00        | r    | PEC value                  |


## 11.7.10 Receive data register (I2C_RXDT)

| Bit       | Name      | Reset value | Type | Description                |
| --------- | --------- | ----------- | ---- | -------------------------- |
| Bit 31: 8 | Reserved  | 0x000000    | res  | Kept at its default value. |
| Bit 7: 0  | DT\[7: 0] | 0x00        | r    | Receive data register      |


Note: I2C_RXDT register is reset when I²C is disabled (I2CEN=0)

## 11.7.11 Transmit data register (I2C_TXDT)

| Bit       | Name      | Reset value | Type | Description                |
| --------- | --------- | ----------- | ---- | -------------------------- |
| Bit 31: 8 | Reserved  | 0x000000    | res  | Kept at its default value. |
| Bit 7: 0  | DT\[7: 0] | 0x00        | rw   | Transmit data register     |


