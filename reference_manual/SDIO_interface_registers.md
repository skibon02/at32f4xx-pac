

**SDIO interrupts**

There is a pin with interrupt feature on the SD interface in order to enable the SD I/O card to interrupt the MultiMedia card/SD module. In 4-bit SD mode, this pin is SDIO_D1. The SD I/O interrupts are detected when the level is active. In other words, the interrupt signal line must be active (low) before it is recognized and responded by the MultiMedia card/SD module, and will remain inactive (high) at the end of the interrupt routine.

When the SDIO_DTCTRL [11] bit is set, the SDIO interrupts are detected on the SDIO_D1 signal line.

# 25.4 SDIO registers

The device communicates with the system through 32-bit control registers accessible via AHB.

The peripheral registers must be accessed by words (32-bit).

Table 25-24 A summary of the SDIO registers

| Register      | Offset | Reset value |
| ------------- | ------ | ----------- |
| SDIO\_PWRCTRL | 0x00   | 0x0000 0000 |
| SDIO\_CLKCTRL | 0x04   | 0x0000 0000 |
| SDIO\_ARG     | 0x08   | 0x0000 0000 |
| SDIO\_CMD     | 0x0C   | 0x0000 0000 |
| SDIO\_RSPCMD  | 0x10   | 0x0000 0000 |
| SDIO\_RSP1    | 0x14   | 0x0000 0000 |
| SDIO\_RSP2    | 0x18   | 0x0000 0000 |
| SDIO\_RSP3    | 0x1C   | 0x0000 0000 |
| SDIO\_RSP4    | 0x20   | 0x0000 0000 |
| SDIO\_DTTMR   | 0x24   | 0x0000 0000 |
| SDIO\_DTLEN   | 0x28   | 0x0000 0000 |
| SDIO\_DTCTRL  | 0x2C   | 0x0000 0000 |
| SDIO\_DTCNTR  | 0x30   | 0x0000 0000 |
| SDIO\_STS     | 0x34   | 0x0000 0000 |
| SDIO\_INTCLR  | 0x38   | 0x0000 0000 |
| SDIO\_INTEN   | 0x3C   | 0x0000 0000 |
| SDIO\_BUFCNTR | 0x48   | 0x0000 0000 |
| SDIO\_BUF     | 0x80   | 0x0000 0000 |


## 25.4.1 SDIO power control register (SDIO_ PWRCTRL)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                                            |
| --------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 2 | Reserved | 0x0000 0000 | resd | Kept at its default value.                                                                                                                                                                                                                             |
| Bit 1: 0  | PS       | 0x0         | rw   | Power switch<br/>These bits are set or cleared by software. They are used to define the current status of the card clock.<br/>00: Power-off, the card clock is stopped.<br/>01: Reserved<br/>10: Reserved<br/>11: Power-on, the card clock is started. |


Note: Write access to this register is not allowed within seven HCLK clock periods after data is written.


# 25.4.2 SDIO clock control register (SDIO_ CLKCTRL)

The SDIO_CLKCTRL register controls the SDIO_CK output clock.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                        |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 17 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                         |
| Bit 16: 15 | CLKDIV   | 0x0         | rw   | Clock division<br/>This field is set or cleared by software. It defines the clock division relations between the SDIOCLK and the SDIO\_CK: SDIO\_CK frequency=SDIOCLK / \[CLKDIV\[9: 0] + 2].                                                                                                                                                      |
| Bit 14     | HFCEN    | 0x0         | rw   | Hardware flow control enable<br/>This bit is set or cleared by software.<br/>0: Hardware flow control disabled<br/>1: Hardware flow control enabled<br/>Note: When hardware flow control is enabled, refer to the SDIO\_STS register for the meaning of the TXBUF\_E and RXBUF\_F interrupt signals.                                               |
| Bit 13     | CLKEGS   | 0x0         | rw   | SDIO\_CK edge selection<br/>This bit is set or cleared by software.<br/>0: SDIO\_CK generated on the rising edge of the master clock SDIOCLK<br/>1: SDIO\_CK generated on the falling edge of the master clock SDIOCLK                                                                                                                             |
| Bit 12: 11 | BUSWS    | 0x0         | rw   | Bus width selection<br/>This bit is set or cleared by software.<br/>00: Default bus mode, SDIO\_D0 used<br/>01: 4-bit bus mode, SDIO\_D\[3: 0] used<br/>10: 8-bit bus mode, SDIO\_D\[7: 0] used                                                                                                                                                    |
| Bit 10     | BYPSEN   | 0x0         | rw   | Clock divider bypass enable bit<br/>This bit is set or cleared by software. When disabled, the SDIO\_CK output signal is driven by the SDIOCLK that is divided according to the CLKDIV value. When enabled, the SDIO\_CK output signal is directly driven by the SDIOCLK.<br/>0: Clock divider bypass disabled<br/>1: Clock divider bypass enabled |
| Bit 9      | PWRSVEN  | 0x0         | rw   | Power saving mode enable<br/>This bit is set or cleared by software. When disabled, the SDIO\_CK is always output; when enabled, the SDIO\_CK is only output when the bus is active.<br/>0: Power saving mode disabled<br/>1: Power saving mode enabled                                                                                            |
| Bit 8      | CLKOEN   | 0x0         | rw   | Clock output enable<br/>This bit is set or cleared by software.<br/>0: Clock output disabled<br/>1: Clock output enabled                                                                                                                                                                                                                           |
| Bit 7: 0   | CLKDIV   | 0x00        | rw   | Clock division<br/>This field is set or cleared by software. It defines the clock division relations between the SDIOCLK and the SDIO\_CK: SDIO\_CK frequency=SDIOCLK / \[CLKDIV\[9: 0] + 2].                                                                                                                                                      |


Note: 1. While the SD/SDIO card or MultiMedia car is in identification mode, the SDIO_CK frequency must be less than 400kHz.

2. When all card are assigned with relative card addresses, the clock frequency can be changed to the maximum card frequency.

3. This register cannot be written within seven HCLK clock periods after data is written. The SDIO_CK can be stopped during the read wait period for SD I/O cards. In this case, the SDIO_ CLKCTRL register does not control the SDIO_CK.


## 25.4.3 SDIO argument register (SDIO_ARG)

The SDIO_ARG register contains 32-bit command argument, which is sent to a card as part of a <u>command.</u>

| Bit       | Name | Reset value | Type | Description                                                                                                                                                                                             |
| --------- | ---- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | ARGU | 0x0000 0000 | rw   | Command argument<br/>Command argument is sent to a card as part of a command. If a command contains an argument, it must be loaded into this register before writing a command to the command register. |


## 25.4.4 SDIO command register (SDIO_CMD)

The SDIO_CMD register contains the command index and command type bits. The command index is sent to a card as part of a command. The command type bits control the command channel state machine (CCSM).

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                             |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 12 | Reserved | 0x00000     | resd | Kept at its default value.                                                                                                                                                                                                                                              |
| Bit 11     | IOSUSP   | 0x0         | rw   | SD I/O suspend command<br/>This bit is set or cleared by software. If this bit is set, the command to be sent is a suspend command (used for SDIO only).<br/>0: SD I/O suspend command disabled<br/>1: SD I/O suspend command enabled                                   |
| Bit 10     | CCSMEN   | 0x0         | rw   | Command channel state machine (CCSM) enable bit<br/>This bit is set or cleared by software.<br/>0: Command channel state machine disabled<br/>1: Command channel state machine enabled                                                                                  |
| Bit 9      | PNDWT    | 0x0         | rw   | CCSM Waits for ends of data transfer (CmdPend internal signal)<br/>This bit is set or cleared by software. If this bit is set, the CCSM waits for the end of data transfer before it starts sending a command.<br/>0: Disabled<br/>1: Enabled                           |
| Bit 8      | INTWT    | 0x0         | rw   | CCSM waits for interrupt request<br/>This bit is set or cleared by software. If this bit is set, the CCSM disables command timeout and waits for an interrupt request.<br/>0: Disabled<br/>1: Enabled                                                                   |
| Bit 7: 6   | RSPWT    | 0x0         | rw   | Wait for response bits<br/>This bit is set or cleared by software. This bit indicates whether the CCSM is to wait for a response, and if yes, it will indicates the response type.<br/>00: No response<br/>01: Short response<br/>10: No response<br/>11: Long response |
| Bit 5: 0   | CMDIDX   | 0x00        | rw   | Command index<br/>The command index is sent to a card as part of a command.                                                                                                                                                                                             |


Note: 1.This register cannot be written within seven HCLK clock periods after data is written.
2.MultiMedia card can sent two types of responses: 48-bit short response or 136-bit short response. The SD card and SD I/O card can send only short responses, and the argument can vary according to the type of response. The software will distinguish the type of response according to the command sent.


### 25.4.5 SDIO command response register (SDIO_RSPCMD)

The SDIO_RSPCMD register contains the command index of the last command response received. If the command response transmission does not contain the command index (long or OCR response), the SDIO_RSPCMD field is unknown, although it should have contained 111111b (the value of the reserved field from a response)

| Bit       | Name     | Reset value | Type | Description                                                                                        |
| --------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------- |
| Bit 31: 6 | Reserved | 0x0000000   | resd | Kept at its default value.                                                                         |
| Bit 5: 0  | RSPCMD   | 0x00        | ro   | Response command index<br/>This field contains the command index of the command response received. |


### 25.4.6 SDIO response 1..4 register (SDIO_RSPx)

The SDIO_RSPx (x=1..4) register contains the status of a card, which is part of the response received.

| Bit       | Name     | Reset value | Type | Description     |
| --------- | -------- | ----------- | ---- | --------------- |
| Bit 31: 0 | CARDSTSx | 0x0000 0000 | ro   | See Table 23-25 |


\*The card status size is 32 or 127 bits, depending on the response type.

Table 25-25 Response type and SDIO_RSPx register

| Register   | Short response       | Long response          |
| ---------- | -------------------- | ---------------------- |
| SDIO\_RSP1 | Card status \[31: 0] | Card status \[127: 96] |
| SDIO\_RSP2 | Unused               | Card status \[95: 64]  |
| SDIO\_RSP3 | Unused               | Card status \[63: 32]  |
| SDIO\_RSP4 | Unused               | Card status \[31: 1]   |


\*The most significant bit of the card status is always received first. The least significant bit of the SDIO_RSP4 register is always 0.

### 25.4.7 SDIO data timer register (SDIO_DTTMR)

The SDIO_DTTMR register contains the data timeout period in the unit of card bus clock periods. A counter loads the value from the SDIO_DTTMR register and starts decrementing when the DCSM enters the Wait_R or busy state. If the counter reaches 0 while the DCSM is in either of these states, a timeout status flag will be set.

| Bit       | Name    | Reset value | Type | Description                                                           |
| --------- | ------- | ----------- | ---- | --------------------------------------------------------------------- |
| Bit 31: 0 | TIMEOUT | 0x0000 0000 | rw   | Data timeout period<br/>Data timeout period in card bus clock cycles. |


*Note: A data transfer must be written to the SDIO_DTCNTR and the SDIO_DTLEN register before being written to the SDIO data control register (SDIO_DTCTRL).*

### 25.4.8 SDIO data length register (SDIO_DTLEN)

The SDIO_DTLEN register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.

| Bit        | Name     | Reset value | Type | Description                                                   |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------- |
| Bit 31: 25 | Reserved | 0x00        | resd | Kept at its default value.                                    |
| Bit 24: 0  | DTLEN    | 0x00000000  | rw   | Data length value<br/>Number of data bytes to be transferred. |


*Note: For a block data transfer, the value in the SDIO_DTLEN must be a multiple of the block data size.*
*A data transfer must be written to the SDIO_DTCNTR and the SDIO_DTLEN register before being written to the SDIO data control register (SDIO_DTCTRL).*


# 25.4.9 SDIO data control register (SDIO_DTCTRL)

The SDIO_DTCTRL register controls the data channel statue machine (DCSM).

| Bit        | Name      | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ---------- | --------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 12 | Reserved  | 0x00000     | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Bit 11     | IOEN      | 0x0         | rw   | SD I/O enable functions<br/>This bit is set or cleared by software. If the bit is set, the DCSM performs an SD IO card-specific operation.<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Bit 10     | RDWTMODE  | 0x0         | rw   | Read wait mode<br/>This bit is set or cleared by software. If disabled, the SDIO\_D2 controls the read wait; if enabled, the SDIO\_CK controls the read wait.<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| Bit 9      | RDWTSTOP  | 0x0         | rw   | Read wait stop<br/>This bit is set or cleared by software. While the RDWTSTART is set, If this bit is set, it indicates that read wait is stopped; if this bit cleared, it indicates that the read wait is in progress.<br/>0: Read wait is in progress if the RDWTSTART is set.<br/>1: Read wait is stopped if the RDWTSTART is set.                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Bit 8      | RDWTSTART | 0x0         | rw   | Read wait start<br/>This bit is set or cleared by software. When this bit is set, read wait starts; when this bit is cleared, no actions occurs.<br/>0: Read wait disabled<br/>1: Read wait enabled                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Bit 7: 4   | BLKSIZE   | 0x0         | rw   | Data block size<br/>This bit is set or cleared by software. This field defines the length of data block when the block data transfer is selected.<br/>0000: block length = 2⁰ = 1 byte<br/>0001: block length = 2¹ = 2 bytes<br/>0010: block length = 2² = 4 bytes<br/>0011: block length = 2³ = 8 bytes<br/>0100: block length = 2⁴ = 16 bytes<br/>0101: block length = 2⁵ = 32 bytes<br/>0110: block length = 2⁶ = 64 bytes<br/>0111: block length = 2⁷ = 128 bytes<br/>1000: block length = 2⁸ = 256 bytes<br/>1001: block length = 2⁹ = 512 bytes<br/>1010: block length = 2¹⁰ = 1024 bytes<br/>1011: block length = 2¹¹ = 2048 bytes<br/>1100: block length = 2¹² = 4096 bytes<br/>1101: block length = 2¹³ = 8192 bytes<br/>1110: block length = 2¹⁴ = 16384 bytes<br/>1111: Reserved |
| Bit 3      | DMAEN     | 0x0         | rw   | DMA enable bit<br/>This bit is set or cleared by software.<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Bit 2      | TFRMODE   | 0x0         | rw   | Data transfer mode selection<br/>This bit is set or cleared by software. If this bit is set, it indicates stream data transfer; if this bit cleared, it indicates block data transfer.<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Bit 1      | TFRDIR    | 0x0         | rw   | Data transfer direction selection<br/>This bit is set or cleared by software. If this bit is set, data transfer is from a card to a controller; if this bit is cleared, data transfer is from a controller to a card.<br/>0: Disabled                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Bit 0 | TFREN | 0x0         | rw   | 1: Enabled<br/>Data transfer enabled bit<br/>This bit is set or cleared by software. If this bit is set, data transfer starts. The DCSM enters the Wait\_S or Wait\_R state, depending on the direction bit TFRDIR. The DCSM goes to the read wait state if the RDWTSTART bit is set from the beginning of the transfer. It is not necessary to clear the enable bit after the end of data transfer but the SDIO\_DTCTRL must be updated to enable a new data transfer.<br/>0: Disabled<br/>1: Enabled |


Note: This register cannot be written within seven HCLK clock periods after data is written.

# 25.4.10 SDIO data counter register (SDIO_DTCNTR)

The SDIO_DTCNTR register loads the value from the SDIO_DTLEN register when the DCSM moves from the idle state to the Wait_R or Wait_S state. During the data transfer, the counter value decrements to 0, and then the DCSM enters the idle state and sets the data status end flag bit DTCMPL.

| Bit        | Name     | Reset value | Type | Description                                                                                                                          |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 25 | Reserved | 0x00        | resd | Kept at its default value.                                                                                                           |
| Bit 24: 0  | CNT      | 0x00000000  | ro   | Data count value<br/>When this register is read, the number of data bytes to be transferred is returned. Write access has no effect. |


Note: This register can be read only when the data transfer is complete.

# 25.4.11 SDIO status register (SDIO_STS)

The SDIO_STS is a read-only register, containing two types of flags:

* Static flags (bits [23: 22, 10: 0]): These bits can be cleared by writing to the SDIO_INTCLR register.

* Dynamic flags (bit [21: 11]): These bit status changes with the state of the corresponding logic (for example, BUT full or empty flag is set or cleared as data written to the BUF).

| Bit        | Name       | Reset value | Type | Description                                                                                                                    |
| ---------- | ---------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 23 | Reserved   | 0x000       | resd | Kept at its default value.                                                                                                     |
| Bit 22     | IOIF       | 0x0         | ro   | SD I/O interrupt received                                                                                                      |
| Bit 21     | RXBUF      | 0x0         | ro   | Data available in receive BUF                                                                                                  |
| Bit 20     | TXBUF      | 0x0         | ro   | Data available in transmit BUF                                                                                                 |
| Bit 19     | RXBUFE     | 0x0         | ro   | Receive BUF empty                                                                                                              |
| Bit 18     | TXBUFE     | 0x0         | ro   | Transmit BUF empty<br/>If hardware flow control is enabled, the TXBUF\_E signal becomes valid when the BUF contains two words. |
| Bit 17     | RXBUFF     | 0x0         | ro   | Receive BUF full<br/>If hardware flow control is enabled, the RXBUF\_F becomes valid two words before the BUF is full.         |
| Bit 16     | TXBUFF     | 0x0         | ro   | Transmit BUF full                                                                                                              |
| Bit 15     | RXBUFH     | 0x0         | ro   | Receive BUF half full<br/>There are at least 8 words in the BUF. This flag bit can be used as DMA request.                     |
| Bit 14     | TXBUFH     | 0x0         | ro   | Transmit BUF half empty:<br/>At least 8 words can be written to the BUF. This flag bit can be used as DMA request.             |
| Bit 13     | DORX       | 0x0         | ro   | Data receive in progress                                                                                                       |
| Bit 12     | DOTX       | 0x0         | ro   | Data transmit in progress                                                                                                      |
| Bit 11     | DOCMD      | 0x0         | ro   | Command transfer in progress                                                                                                   |
| Bit 10     | DTBLKCMPL  | 0x0         | ro   | Data block sent/received CRC check passed)                                                                                     |
| Bit 9      | SBITERR    | 0x0         | ro   | *Start bit not detected on all data signals in wide bus mode*                                                                  |
| Bit 8      | DTCMPL     | 0x0         | ro   | Data end (data counter, SDIO CNT, is zero)                                                                                     |
| Bit 7      | CMDCMPL    | 0x0         | ro   | Command sent (no response required)                                                                                            |
| Bit 6      | CMDRSPCMPL | 0x0         | ro   | Command response (CRC check passed)                                                                                            |
| Bit 5      | RXERRO     | 0x0         | ro   | Received BUF overrun error                                                                                                     |
| Bit 4 | TXERRU     | 0x0 | ro | Transmit BUF underrun error                                                                     |
| Bit 3 | DTTIMEOUT  | 0x0 | ro | Data timeout                                                                                    |
| Bit 2 | CMDTIMEOUT | 0x0 | ro | Command response timeout<br/>The command timeout is a fixed value of 64 SDIO\_CK clock periods. |
| Bit 1 | DTFAIL     | 0x0 | ro | Data block sent/received (CRC check failed)                                                     |
| Bit 0 | CMDFAIL    | 0x0 | ro | Command response received (CRC check failed)                                                    |


# 25.4.12 SDIO clear interrupt register (SDIO_INTCLR)

The SDIO_INTCLR is a read-only register. Writing 1 to the corresponding register bit will clear the correspond bit in the SDIO_STS register.

| Bit        | Name       | Reset value | Type | Description                                                                             |
| ---------- | ---------- | ----------- | ---- | --------------------------------------------------------------------------------------- |
| Bit 31: 23 | Reserved   | 0x000       | resd | Kept at its default value.                                                              |
| Bit 22     | IOIF       | 0x0         | rw   | SD I/O interface flag clear bit<br/>This bit is set by software to clear the IOIF flag. |
| Bit 21: 11 | Reserved   | 0x000       | resd | Kept at its default value.                                                              |
| Bit 10     | DTBLKCMPL  | 0x0         | rw   | DTBLKCMPL flag clear bit<br/>This bit is set by software to clear the DTBLKCMPL flag.   |
| Bit 9      | SBITERR    | 0x0         | rw   | SBITERR flag clear bit<br/>This bit is set to clear the SBITERR flag.                   |
| Bit 8      | DTCMPL     | 0x0         | rw   | DTCMPL flag clear bit<br/>This bit is set by software to clear the DTCMPL flag.         |
| Bit 7      | CMDCMPL    | 0x0         | rw   | CMDCMPL flag clear bit<br/>This bit is set by software to clear the CMDCMPL flag.       |
| Bit 6      | CMDRSPCMPL | 0x0         | rw   | MDRSPCMPL flag clear bit<br/>This bit is set by software to clear the CMDRSPCMPL flag.  |
| Bit 5      | RXERRO     | 0x0         | rw   | RXERRO flag clear bit<br/>This bit is set by software to clear the RXERRO flag.         |
| Bit 4      | TXERRU     | 0x0         | rw   | TXERRU flag clear bit<br/>This bit is set by software to clear the TXERRU flag.         |
| Bit 3      | DTTIMEOUT  | 0x0         | rw   | DTTIMEOUT flag clear bit<br/>This bit is set by software to clear the DTTIMEOUT flag.   |
| Bit 2      | CMDTIMEOUT | 0x0         | rw   | CMDTIMEOUT flag clear bit<br/>This bit is set by software to clear the CMDTIMEOUT flag. |
| Bit 1      | DTFAIL     | 0x0         | rw   | DTFAIL flag clear bit<br/>This bit is set by software to clear the DTFAIL flag.         |
| Bit 0      | CMDFAIL    | 0x0         | rw   | CMDFAIL flag clear bit<br/>This bit is set by software to clear the CMDFAIL flag.       |


# 25.4.13 SDIO interrupt mask register (SDIO_INTEN)

The SDIO_INTEN register determines which status bit generates an interrupt by setting the corresponding bit.

| Bit        | Name      | Reset value | Type | Description                                                                                                                                                                    |
| ---------- | --------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 23 | Reserved  | 0x000       | resd | Kept at its default value.                                                                                                                                                     |
| Bit 22     | IOIFIEN   | 0x0         | rw   | SD I/O mode received interrupt enable<br/>This bit is set or cleared by software to enable/disable the SD I/O mode received interrupt function.<br/>0: Disabled<br/>1: Enabled |
| Bit 21     | RXBUFIEN  | 0x0         | rw   | Data available in RxBUF interrupt enable<br/>This bit is set or cleared by software to enable/disable the Data Available in RxBUF Interrupt.<br/>0: Disabled<br/>1: Enabled    |
| Bit 20     | TXBUFIEN  | 0x0         | rw   | Data available in TxBUF interrupt enable<br/>This bit is set or cleared by software to enable/disable the Data Available in TxBUF Interrupt.<br/>0: Disabled<br/>1: Enabled    |
| Bit 19     | RXBUFEIEN | 0x0         | rw   | RxBUF empty interrupt enable<br/>This bit is set or cleared by software to enable/disable the                                                                                  |
|        |               |             | RxBUF empty interrupt.<br/>0: Disabled<br/>1: Enabled |                                                                                                                                                                       |
| Bit 18 | TXBUFEIEN     | 0x0         | rw                                                    | TxBUF empty interrupt enable<br/>This bit is set or cleared by software to enable/disable the TxBUF empty interrupt.<br/>0: Disabled<br/>1: Enabled                   |
| Bit 17 | RXBUFFIEN     | 0x0         | rw                                                    | RxBUF full interrupt enable<br/>This bit is set or cleared by software to enable/disable the RxBUF full interrupt.<br/>0: Disabled<br/>1: Enabled                     |
| Bit 16 | TXBUFFIEN     | 0x0         | rw                                                    | TxBUF full interrupt enable<br/>This bit is set or cleared by software to enable/disable the TxBUF full interrupt.<br/>0: Disabled<br/>1: Enabled                     |
| Bit 15 | RXBUFHIEN     | 0x0         | rw                                                    | RxBUF half full interrupt enable<br/>This bit is set or cleared by software to enable/disable the RxBUF half full interrupt.<br/>0: Disabled<br/>1: Enabled           |
| Bit 14 | TXBUFHIEN     | 0x0         | rw                                                    | TxBUF half empty interrupt enable<br/>This bit is set or cleared by software to enable/disable the TxBUF half empty interrupt.<br/>0: Disabled<br/>1: Enabled         |
| Bit 13 | DORXIEN       | 0x0         | rw                                                    | Data receive acting interrupt enable<br/>This bit is set or cleared by software to enable/disable the Data receive acting interrupt.<br/>0: Disabled<br/>1: Enabled   |
| Bit 12 | DOTXIEN       | 0x0         | rw                                                    | Data transmit acting interrupt enable<br/>This bit is set or cleared by software to enable/disable the Data transmit acting interrupt.<br/>0: Disabled<br/>1: Enabled |
| Bit 11 | DOCMDIEN      | 0x0         | rw                                                    | Command acting interrupt enable<br/>This bit is set or cleared by software to enable/disable the Command acting interrupt.<br/>0: Disabled<br/>1: Enabled             |
| Bit 10 | DTBLKCMPLIEN  | 0x0         | rw                                                    | Data block end interrupt enable<br/>This bit is set or cleared by software to enable/disable the Data block end interrupt.<br/>0: Disabled<br/>1: Enabled             |
| Bit 9  | SBITERRIEN    | 0x0         | rw                                                    | Start bit error interrupt enable<br/>This bit is set or cleared by software to enable/disable the Start bit error interrupt.<br/>0: Disabled<br/>1: Enabled           |
| Bit 8  | DTCMPLIEN     | 0x0         | rw                                                    | Data end interrupt enable<br/>This bit is set or cleared by software to enable/disable the Data end interrupt.<br/>0: Disabled<br/>1: Enabled                         |
| Bit 7  | CMDCMPLIEN    | 0x0         | rw                                                    | Command sent interrupt enable<br/>This bit is set or cleared by software to enable/disable the Command sent interrupt.<br/>0: Disabled<br/>1: Enabled                 |
| Bit 6  | CMDRSPCMPLIEN | 0x0         | rw                                                    | Command response received interrupt enable                                                                                                                            |
|       |               |             |      | This bit is set or cleared by software to enable/disable the<br/>Command response received interrupt.<br/>0: Disabled<br/>1: Enabled                                      |
| Bit 5 | RXERROIEN     | 0x0         | rw   | RxBUF overrun error interrupt enable<br/>This bit is set or cleared by software to enable/disable the<br/>RxBUF overrun error interrupt.<br/>0: Disabled<br/>1: Enabled   |
| Bit 4 | TXERRUIEN     | 0x0         | rw   | TxBUF underrun error interrupt enable<br/>This bit is set or cleared by software to enable/disable the<br/>TxBUF underrun error interrupt.<br/>0: Disabled<br/>1: Enabled |
| Bit 3 | DTTIMEOUTIEN  | 0x0         | rw   | Data timeout interrupt enable<br/>This bit is set or cleared by software to enable/disable the<br/>Data timeout interrupt.<br/>0: Disabled<br/>1: Enabled                 |
| Bit 2 | CMDTIMEOUTIEN | 0x0         | rw   | Command timeout interrupt enable<br/>This bit is set or cleared by software to enable/disable the<br/>Command timeout interrupt.<br/>0: Disabled<br/>1: Enabled           |
| Bit 1 | DTFAILIEN     | 0x0         | rw   | Data CRC fail interrupt enable<br/>This bit is set or cleared by software to enable/disable the<br/>Data CRC fail interrupt.<br/>0: Disabled<br/>1: Enabled               |
| Bit 0 | CMDFAILIEN    | 0x0         | rw   | Command CRC fail interrupt enable<br/>This bit is set or cleared by software to enable/disable the<br/>Command CRC fail interrupt.<br/>0: Disabled<br/>1: Enabled         |


# 25.4.14 SDIOBUF counter register (SDIO_BUFCNTR)

The SDIO_BUFCNTR register contains the number of words to be written to or read from the BUF. The BUF counter loads the value from the SDIO_DTLEN register when the data transfer bit TFREN is set in the SDIO_DTCTRL register. If the data length is not word-aligned, the remaining 1 to 3 bytes are regarded as a word.

| Bit        | Name     | Reset value | Type | Description                                            |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------ |
| Bit 31: 24 | Reserved | 0x00        | resd | Kept at its default value.                             |
| Bit 23: 0  | CNT      | 0x000000    | ro   | Number of words to be written to or read from the BUF. |


# 25.4.15 SDIO data BUF register (SDIO_BUF)

The receive and data BUF is group of a 32-bit wide registers that can be written or read. The BUF contains 32 registers on 32 sequential addresses. The CPU can use BUF for read/write multiple operations.

| Bit       | Name | Reset value | Type | Description                                                                                                                     |
| --------- | ---- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | DT   | 0x0000 0000 | rw   | Receive and transmit BUF data<br/>The BUF data occupies 32x 32-bit words, the address:<br/>SDIO base + 0x80 to SDIO base + 0xFC |


