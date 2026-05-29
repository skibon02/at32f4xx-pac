
ARTERY logo AT32F435/437 Series Reference Manual

# 12.12 USART registers

These peripheral registers must be accessed by words (32 bits).

Table 12-6 USART register map and reset value

| Register     | Offset | Reset value |
| ------------ | ------ | ----------- |
| USART\_STS   | 0x00   | 0x0000 00C0 |
| USART\_DT    | 0x04   | 0x0000 0000 |
| USART\_BAUDR | 0x08   | 0x0000 0000 |
| USART\_CTRL1 | 0x0C   | 0x0000 0000 |
| USART\_CTRL2 | 0x10   | 0x0000 0000 |
| USART\_CTRL3 | 0x14   | 0x0000 0000 |
| USART\_GTP   | 0x18   | 0x0000 0000 |


## 12.12.1 Status register (USART_STS)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                    |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 10 | Reserved | 0x000000    | resd | Forced 0 by hardware.                                                                                                                                                                                                                                                                                          |
| Bit 9      | CTSCF    | 0x0         | rw0c | CTS change flag<br/>This bit is set by hardware when the CTS status line changes. It is cleared by software.<br/>0: No change on the CTS status line<br/>1: A change occurs on the CTS status line.                                                                                                            |
| Bit 8      | BFF      | 0x0         | rw0c | Break frame flag<br/>This bit is set by hardware when a break frame is detected. It is cleared by software.<br/>0: Break frame is not detected.<br/>1: Break frame is detected.                                                                                                                                |
| Bit 7      | TDBE     | 0x1         | ro   | Transmit data buffer empty<br/>This bit is set by hardware when the transmit data buffer is empty. It is cleared by a USART\_DT register write operation.<br/>0: Data is not transferred to the shift register.<br/>1: Data is transferred to the shift register.                                              |
| Bit 6      | TDC      | 0x1         | rw0c | Transmit data complete<br/>This bit is set by hardware at the end of transmission. It is cleared by software. (Option 1: read access to USART\_STS register followed by a USART\_DT write operation; Option 2: Write “0” to this bit )<br/>0: Transmission is not completed.<br/>1: Transmission is completed. |
| Bit 5      | RDBF     | 0x0         | rw0c | Receive data buffer full<br/>This bit is set by hardware when the data is transferred from the shift register to the USART\_DT register. It is cleared by software. (Option 1: read USART\_DT register; Option 2: write “0” to this bit)<br/>0: Data is not received.<br/>1: Data is received.                 |
| Bit 4      | IDLEF    | 0x0         | ro   | Idle flag<br/>This bit is set by hardware when an idle line is detected. It is cleared by software. (Read USART\_DT register followed by a USART\_DT read operation)<br/>0: No idle line is detected.<br/>1: Idle line is detected.                                                                            |
| Bit 3      | ROERR    | 0x0         | ro   | Receiver overflow error<br/>This bit is set by hardware when the data is received while the RDNE is still set. It is cleared by software. (Read USART\_STS register followed by a USART\_DT read operation)<br/>0: No overflow error                                                                           |


2025.05.28 Page 221 Rev 2.07





ARTERY logo AT32F435/437 Series Reference Manual

| Bit   | Name | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                         |
| ----- | ---- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 2 | NERR | 0x0         | ro   | 1: Overflow error is detected.<br/>Note: When this bit is set, the DT register content will not be lost, but the subsequent data will be overwritten.<br/>Noise error<br/>This bit is set by hardware when noise is detect on a received frame. It is cleared by software. (Read USART\_STS register followed by a USART\_DT read operation)<br/>0: No noise is detected.<br/>1: Noise is detected. |
| Bit 1 | FERR | 0x0         | ro   | Framing error<br/>This bit is set by hardware when a stop bit error (low), excessive noise or break frame is detected. It is cleared by software. USART\_STS register followed by a USART\_DT read operation)<br/>0: No framing error is detected.<br/>1: Framing error is detected.                                                                                                                |
| Bit 0 | PERR | 0x0         | ro   | Parity error<br/>This bit is set by hardware when parity error occurs. It is cleared by software. USART\_STS register followed by a USART\_DT read operation)<br/>0: No parity error occurs.<br/>1: Parity error occurs.                                                                                                                                                                            |


# 12.12.2 Data register (USART_DT)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                         |
| --------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 9 | Reserved | 0x000000    | resd | Kept at its default value.                                                                                                                                                                                                                                                          |
| Bit 8: 0  | DT       | 0x00        | rw   | Data value<br/>This register provides read and write function. When transmitting with the parity bit enabled, the value written in the MSB bit will be replaced by the parity bit. When receiving with the parity bit enabled, the value in the MSB bit is the received parity bit. |


# 12.12.3 Baud rate register (USART_BAUDR)

<u>Note: If the TEN and REN are both disabled, the baud counter stops counting.</u>

| Bit        | Name     | Reset value | Type | Description                                      |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------ |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value.                       |
| Bit 15: 0  | DIV      | 0x0000      | rw   | Divider<br/>This field define the USART divider. |


# 12.12.4 Control register 1 (USART_CTRL1)

| Bit         | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                       |
| ----------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 29  | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                        |
| Bit 28      | DBN1     | 0x0         | rw   | Data bit num<br/>This bit, along with the DBN0 bit, is used to program the number of data bits.<br/>10: 7 data bits<br/>00: 8 data bits<br/>01: 9 data bits<br/>11: Write operation forbidden.<br/>Note: When parity check is enabled, the number of the valid data bit is decremented by one bit, and the MSB of the data bit is replaced with parity check bit. |
| Bit 27: 26  | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                        |
| Bit 25 : 21 | TSDT     | 0x00        | rw   | Transmit start delay time<br/>In RS485 mode, the first data (in sequential transmit mode) is transmitted after a period of time of being written so as to ensure that the transfer direction of the external transmitter/receiver to switch back to transmit. This time depends on the TSDT value, in unit of 1/16 baud rate.                                     |
| Bit 20 : 16 | TCDT     | 0x00        | rw   | transmit complete delay time                                                                                                                                                                                                                                                                                                                                      |


2025.05.28 Page 222 Rev 2.07




Artery logo

AT32F435/437 Series Reference Manual

In RS485 mode, a period of time (delay) is needed before the last data transfer is complete even if the last STOP bit has been transferred. This time duration allows the transfer direction of the external receiver/transmitter to switch back to receive. This time depends on the TCDT value, in unit of 1/16 baud rate.

| Bit 15: 14 | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                 |
| ---------- | -------- | --- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 13     | UEN      | 0x0 | rw   | USART enable<br/>0: USART is disabled.<br/>1: USART is enable.                                                                                                                                                                                                                                                                                                                             |
| Bit 12     | DBN0     | 0x0 | rw   | Data bit num<br/>This bit, along with DBN1, is used to program the number of data bits.<br/>10: 7 data bits<br/>00: 8 data bits<br/>01: 9 data bits<br/>11: Write operation forbidden.<br/>Note: When parity check is enabled, the number of the valid data bist is decremented by one bit, and the MSB of the data bits is replaced with parity check bit.                                |
| Bit 11     | WUM      | 0x0 | rw   | Wakeup mode<br/>This bit determines the way to wake up silent mode.<br/>0: Waken up by idle line<br/>1: Waken up by ID match                                                                                                                                                                                                                                                               |
| Bit 10     | PEN      | 0x0 | rw   | Parity enable<br/>This bit is used to enable hardware parity control (generation of parity bit for transmission; detection of parity bit for reception). When this bit is enabled, the MSB bit of the transmitted data is replaced with the parity bit; Check whether the parity bit of the received data is correct.<br/>0: Parity control is disabled.<br/>1: Parity control is enabled. |
| Bit 9      | PSEL     | 0x0 | rw   | Parity selection<br/>This bit selects the odd or even parity after the parity control is enabled.<br/>0: Even parity<br/>1: Odd parity                                                                                                                                                                                                                                                     |
| Bit 8      | PERRIEN  | 0x0 | rw   | PERR interrupt enable<br/>0: Interrupt is disabled.<br/>1: Interrupt is enabled.                                                                                                                                                                                                                                                                                                           |
| Bit 7      | TDBEIEN  | 0x0 | rw   | TDBE interrupt enable<br/>0: Interrupt is disabled.<br/>1: Interrupt is enabled.                                                                                                                                                                                                                                                                                                           |
| Bit 6      | TDCIEN   | 0x0 | rw   | TDC interrupt enable<br/>0: Interrupt is disabled.<br/>1: Interrupt is enabled.                                                                                                                                                                                                                                                                                                            |
| Bit 5      | RDBFIEN  | 0x0 | rw   | RDBF interrupt enable<br/>0: Interrupt is disabled.<br/>1: Interrupt is enabled.                                                                                                                                                                                                                                                                                                           |
| Bit 4      | IDLEIEN  | 0x0 | rw   | IDLE interrupt enable<br/>0: Interrupt is disabled.<br/>1: Interrupt is enabled.                                                                                                                                                                                                                                                                                                           |
| Bit 3      | TEN      | 0x0 | rw   | Transmitter enable<br/>This bit enables the transmitter.<br/>0: Transmitter is disabled.<br/>1: Transmitter is enabled.                                                                                                                                                                                                                                                                    |
| Bit 2      | REN      | 0x0 | rw   | Receiver enable<br/>This bit enables the receiver.<br/>0: Receiver is disabled.<br/>1: Receiver is enabled.                                                                                                                                                                                                                                                                                |
| Bit 1      | RM       | 0x0 | rw   | Receiver mute<br/>This bit determines if the receiver is in mute mode or not. It is set or cleared by software. When the idle line is used to wake up from mute mode, this bit is cleared by                                                                                                                                                                                               |


2025.05.28
Page 223
Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

|       |     |     |    | hardware after wake up. When the address match is used to wake up from mute mode, it is cleared by hardware after wake up. When address mismatches, this bit is set by hardware to enter mute mode again.<br/>0: Receiver is in active mode.<br/>1: Receiver is in mute mode.         |
| ----- | --- | --- | -- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 0 | SBF | 0x0 | rw | Send break frame<br/>This bit is used to send a break frame. It can be set or cleared by software. Generally speaking, it is set by software and cleared by hardware at the end of break frame transmission.<br/>0: No break frame is transmitted.<br/>1: Break frame is transmitted. |


# 12.12.5 Control register 2 (USART_CTRL2)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                     |
| ---------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 28 | IDH      | 0x0         | rw   | USART identification<br/>This field holds the upper four bits of USART ID. It is configurable.                                                                                                                                                                                                                  |
| Bit 27: 16 | Reserved | 0x000       | resd | Kept at its default value.                                                                                                                                                                                                                                                                                      |
| Bit 15     | TRPSWAP  | 0x0         | rw   | Transmit/receive pin swap<br/>0: Transmit/receive pin is not swappable<br/>1: Transmit/receive pin is swappable                                                                                                                                                                                                 |
| Bit 14     | LINEN    | 0x0         | rw   | LIN mode enable<br/>0: LIN mode is disabled.<br/>1: LIN mode is enabled.                                                                                                                                                                                                                                        |
| Bit 13: 12 | STOPBN   | 0x0         | rw   | STOP bit num<br/>These bits are used to program the number of stop bits.<br/>00: 1 stop bit<br/>01: 0.5 stop bit<br/>10: 2 stop bits<br/>11: 1.5 stop bits                                                                                                                                                      |
| Bit 11     | CLKEN    | 0x0         | rw   | Clock enable<br/>This bit is used to enable the clock pin for synchronous mode or Smartcard mode.<br/>0: Clock is disabled.<br/>1: Clock is enabled.                                                                                                                                                            |
| Bit 10     | CLKPOL   | 0x0         | rw   | Clock polarity<br/>In synchronous mode or Smartcard mode, this bit is used to select the polarity of the clock output on the clock pin in idle state.<br/>0: Clock output low<br/>1: Clock output high                                                                                                          |
| Bit 9      | CLKPHA   | 0x0         | rw   | Clock phase<br/>This bit is used to select the phase of the clock output on the clock pin in synchronous mode or Smartcard mode.<br/>0: Data capture is done on the first clock edge.<br/>1: Data capture is done on the second clock edge.                                                                     |
| Bit 8      | LBCP     | 0x0         | rw   | Last bit clock pulse<br/>This bit is used to select whether the clock pulse of the last data bit transmitted is output on the clock pin in synchronous mode.<br/>0: The clock pulse of the last data bit is no output on the clock pin.<br/>1: The clock pulse of the last data bit is output on the clock pin. |
| Bit 7      | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                      |
| Bit 6      | BFIEN    | 0x0         | rw   | Break frame interrupt enable<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                                                                                                     |
| Bit 5      | BFBN     | 0x0         | rw   | Break frame bit num<br/>This bit is used to select 11-bit or 10-bit break frame.<br/>0: 10-bit break frame<br/>1: 11-bit break frame                                                                                                                                                                            |


2025.05.28
Page 224
Rev 2.07




ARTERY logo AT32F435/437 Series Reference Manual

| Bit      | Name | Reset value | Type | Description                                                                                                                                                                                                                                                                                                 |
| -------- | ---- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 4    | IDBN | 0x0         | rw   | Identification bit num<br/>This bit is used to select ID bit number.<br/>0: 4 bit<br/>1: Data bit - 1 bit<br/>Note: When this bit is set, in 7, 8 or 8-bit data mode, the ID bit number is the lower 6, 7 or 8 bit, respectively. If the parity check is enabled, the ID bit number is then 5, 6 or 7 bits. |
| Bit 3: 0 | IDL  | 0x0         | rw   | USART identification<br/>This field holds the lower four bits of USART ID. It is configurable.                                                                                                                                                                                                              |


Note: These three bits (CLKPOL, CLKPHA and LBCP) should not be changed while the transmission is enabled.

# 12.12.6 Control register 3 (USART_CTRL3)

| Bit         | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                |
| ----------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16  | Reserved | 0x0000      | resd | Forced 0 by hardware.                                                                                                                                                                                                                                                                                                                                      |
| Bit 15      | DEP      | 0x0         | rw   | DE polarity selection<br/>0: High level active<br/>1: Low level active                                                                                                                                                                                                                                                                                     |
| Bit 14      | RS485EN  | 0x0         | rw   | RS485 enable<br/>This bit is used to enable RS485 mode. In RS485 mode, the USART controls the transfer direction of the external receiver/transmitter through the DE signal.<br/>0: RS485 mode disabled. The control signal DE output is disabled. RTS pin is used in RS232 mode.<br/>1: RS485 mode enabled. The control signal DE outputs on the RTS pin. |
| Bit 13 : 11 | Reserved | 0x0         | resd | Forced 0 by hardware.                                                                                                                                                                                                                                                                                                                                      |
| Bit 10      | CTSCFIEN | 0x0         | rw   | CTSCF interrupt enable<br/>0: CTSCF interrupt disabled<br/>1: CTSCF interrupt enabled                                                                                                                                                                                                                                                                      |
| Bit 9       | CTSEN    | 0x0         | rw   | CTS enable<br/>0: CTS is disabled.<br/>1: CTS is enabled.                                                                                                                                                                                                                                                                                                  |
| Bit 8       | RTSEN    | 0x0         | rw   | RTS enable<br/>0: RTS is disabled.<br/>1: RTS is enabled.                                                                                                                                                                                                                                                                                                  |
| Bit 7       | DMATEN   | 0x0         | rw   | DMA transmitter enable<br/>0: DMA transmitter is disabled.<br/>1: DMA transmitter is enabled.                                                                                                                                                                                                                                                              |
| Bit 6       | DMAREN   | 0x0         | rw   | DMA receiver enable<br/>0: DMA receiver is disabled.<br/>1: DMA receiver is enabled.                                                                                                                                                                                                                                                                       |
| Bit 5       | SCMEN    | 0x0         | rw   | Smartcard mode enable<br/>0: Smartcard mode is disabled.<br/>1: Smartcard mode is enabled.                                                                                                                                                                                                                                                                 |
| Bit 4       | SCNACKEN | 0x0         | rw   | Smartcard NACK enable<br/>This bit is used to send NACK when parity error occurs.<br/>0: NACK is disabled when parity error occurs.<br/>1: NACK is enabled when parity error occurs.                                                                                                                                                                       |
| Bit 3       | SLBEN    | 0x0         | rw   | Single-wire bidirectional half-duplex enable<br/>0: Single-wire bidirectional half-duplex is disabled.<br/>1: Single-wire bidirectional half-duplex is enabled.                                                                                                                                                                                            |
| Bit 2       | IRDALP   | 0x0         | rw   | IrDA low-power mode<br/>This bit is used to configure IrDA low-power mode.<br/>0: IrDA low-power mode is disabled.<br/>1: IrDA low-power mode is enabled.                                                                                                                                                                                                  |
| Bit 1       | IRDAEN   | 0x0         | rw   | IrDA enable<br/>0: IrDA is disabled.<br/>1: IrDA is enabled.                                                                                                                                                                                                                                                                                               |
| Bit 0       | ERRIEN   | 0x0         | rw   | Error interrupt enable<br/>An interrupt is generated when a framing error, overflow                                                                                                                                                                                                                                                                        |


2025.05.28 | Page 225 | Rev 2.07




ARTERY logo AT32F435/437 Series Reference Manual

error or noise error occurs.
0: Error interrupt is disabled.
1: Error interrupt is enabled.

## 12.12.7 Guard time and divider register (USART_GDIV)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Forced 0 by hardware.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Bit 15: 8  | SCGT     | 0x00        | rw   | Smartcard guard time value<br/>This field specifies the guard time value. The transmission complete flag is set after this guard time in smartcard mode.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Bit 7: 0   | ISDIV    | 0x00        | rw   | IrDA/smartcard division<br/>In IrDA mode:<br/>8 bit \[7: 0] is valid. It is valid in common mode and must be set to 00000001. In low-power mode, it divides the peripheral clock to serve as the period base of the pulse width;<br/>00000000: Reserved–Do not write.<br/>00000001: Divided by 1<br/>00000010: Divided by 2<br/>......<br/>Smartcard mode:<br/>The lower 5 bit \[4: 0] is valid. This division is used to divide the peripheral clock to provide clock for the Smartcard.<br/>Configured as follows:<br/>00000: Reserved–Do not write.<br/>00001: Divided by 2<br/>00010: Divided by 4<br/>00011: Divided by 6<br/>...... |


2025.05.28 Page 226 Rev 2.07
