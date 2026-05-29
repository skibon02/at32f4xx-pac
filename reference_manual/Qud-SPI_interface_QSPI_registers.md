
ARTERY logo AT32F435/437 Series Reference Manual

# 28.4 QSPI registers

These registers must be accessed by bytes (8-bit), half-words (16-bit) or words (32-bit).

<u>Table 28-1 QSPI register map and reset values</u>

| Register    | Offset | Reset value  |
| ----------- | ------ | ------------ |
| CMD\_W0     | 0x0    | 0x0000 0000  |
| CMD\_W1     | 0x4    | 0x0100 0003  |
| CMD\_W2     | 0x8    | 0x0000 0000  |
| CMD\_W3     | 0xC    | 0x0000 0000  |
| CTRL        | 0x10   | 0x0010 0083  |
| FIFOSTS     | 0x18   | 0x0000 0001  |
| CTRL2       | 0x20   | 0x0000 0000  |
| CMDSTS      | 0x24   | 0x0000 0000  |
| RSTS        | 0x28   | 0x0000 0000  |
| FSIZE       | 0x2C   | 0xF0000 0000 |
| XIP CMD\_W0 | 0x30   | 0x0000 3000  |
| XIP CMD\_W1 | 0x34   | 0x0000 2000  |
| XIP CMD\_W2 | 0x38   | 0x0F01 0F01  |
| XIP CMD\_W3 | 0x3C   | 0x0000 0000  |
| CTRL3       | 0x40   | 0x0000 0000  |
| REV         | 0x50   | 0x0001 0500  |
| DT          | 0x100  | 0x0000 0000  |


## 28.4.1 Command word 0 (CMD_W0)

No-wait states, assessable by bytes, half-words and words.

| Bit       | Name   | Reset value | Type | Description                                                                                                                                                                       |
| --------- | ------ | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | SPIADR | 0x0         | rw   | **SPI Flash address**<br/>This register defines the values of SPI Flash addresses, and sends them to SPI Flash. The address byte is based on the bit \[2: 0] of CMD\_W1 register. |


## 28.4.2 Command word 1 (CMD_W1)

No-wait states, assessable by words

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                   |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 29 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                    |
| Bit 28     | PEMEN    | 0x0         | rw   | **Performance enhanced mode enable**<br/>Locates between the address and the second dummy state. In this mode, the command status after the second read command can be removed. Do not set this bit when CMD\_W2=0.<br/>0: Performance enhanced mode disabled<br/>1: 1-byte Performance enhanced mode enabled |
| Bit 27: 26 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                    |


2025.05.28 Page 679 Rev 2.07





Artery logo AT32F435/437 Series Reference Manual

| Bit 25: 24 | INSLEN   | 0x1 | rw   | Instruction code length<br/>Instruction code is required for SPI Flash command execution. The instruction code length varies from SPI Flash supplier to SPI Flash supplier. Thus this register can be used to program the desired instruction code length. Typically, the instruction code is one-byte length. However, if the user sets two-byte instruction code, the host controller sends this instruction code twice.<br/>00: No instruction code. It cannot be used until the continuous read mode command is completed.<br/>01: 1-byte instruction code<br/>10: 2-byte instruction code (repeated instruction code)<br/>11: Reserved |
| ---------- | -------- | --- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 23: 16 | DUM2     | 0x0 | rw   | Second dummy state cycle<br/>The second dummy cycle is located between the address and data state, excluding performance enhanced mode status. The user can check there is a dummy state between the address and data status in SPI Flash specification. The host controller sends logic 1 in a dummy cycle.<br/>0: No second dummy state<br/>1\~32: 1 dummy second period\~32 dummy second period                                                                                                                                                                                                                                          |
| Bit 15: 3  | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Bit 2: 0   | ADRLEN   | 0x3 | rw   | SPI address length<br/>This field defines the number of bytes of the SPI Flash address, ranging from one to four bytes.<br/>000: No address state<br/>001: 1-byte address<br/>010: 2-byte address<br/>011: 3-byte address<br/>100: 4-byte address<br/>Others: Reserved                                                                                                                                                                                                                                                                                                                                                                      |


## 28.4.3 Command word 2 (CMD_W2)

No-wait states, accessible words

| Bit       | Name | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                    |
| --------- | ---- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 0 | DCNT | 0x0         | rw   | Read/Write data counter<br/>This bit must be set to 0 when executing read status command.<br/>0: No read/write data<br/>1\~FFFFFFFF: 1\~FFFFFFFF byte data<br/>Note: This register must not be padded with 0 for data read or write. However, for “read status” or “write enable” instruction, this register must be set to 0. |


## 28.4.4 Command word 3 (CMD_W3)

No-wait states, accessible by words.

| Bit        | Name   | Reset value | Type | Description                                                                                                                                                                                                                             |
| ---------- | ------ | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 24 | INSC   | 0x00        | rw   | Instruction code<br/>This code is set to enable SPI Flash command.                                                                                                                                                                      |
| Bit 23: 16 | PEMOPC | 0x00        | rw   | Performance enhanced mode operation code<br/>This field works with the PEMEN bit. This code can be padded to execute performance enhanced mode. Follow the corresponding Flash specification document to write the corresponding value. |


2025.05.28 Page 680 Rev 2.07





ARTERY logo
# AT32F435/437 Series Reference Manual

| Bit 15: 10 | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                 |
| ---------- | -------- | --- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 7: 5   | OPMODE   | 0x0 | rw   | SPI Operation mode<br/>000: Serial mode (1-1-1)<br/>001: Dual mode (1-1-2)<br/>010: Quad mode (1-1-4)<br/>011: Dual I/O mode (1-2-2)<br/>100: Quad I/O mode (1-4-4)<br/>101: DPI mode (2-2-2)<br/>110: QPI mode (4-4-4)<br/>Others: Reserved                                                                                                                                               |
| Bit 4      | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                 |
| Bit 3      | RSTSC    | 0x0 | rw   | Read SPI status configuration<br/>This bit is valid only when read state and write is enabled.<br/>The user must send a SPI read state command.<br/>0: Hardware read. The controller keeps polling until the state is ready (not busy) and feedbacks to the status register.<br/>1: Software read. Read status ones and feedback to the status register until the user is able to read it. |
| Bit 2      | RSTSEN   | 0x0 | rw   | Read SPI status enable<br/>This bit is valid when WEN =“0”, and the user must send SPI read status command.<br/>0: Read SPI status disabled<br/>1: Read SPI status enabled                                                                                                                                                                                                                 |
| Bit 1      | WEN      | 0x0 | rw   | Write data enable<br/>This bit is used to enable SPI write data, excluding read data or read status (read data return path); the user must set write enable bit=1 for other SPI commands.<br/>Note: Write enable must be set to 1 in data write or Flash erase command. The write enable must be set to 0 only in read data or read status command.<br/>0: Disabled<br/>1: Enabled         |
| Bit 0      | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                 |


## 28.4.5 Control register (CTRL)

No-wait states, accessible by bytes, half-words and words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                |
| ---------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 22 | Reserved | 0x000       | resd | Kept at its default value.                                                                                                                                                                                                                                                                                 |
| Bit 21     | KEYEN    | 0x0         | rw   | SPI data encryption key enable<br/>0: SPI data encryption key disabled<br/>1: SPI data encryption key enabled<br/>When this bit is enabled, raw data is converted into ciphertext and written into the QSPI peripheral through QSPIKEY. While read, data is decrypted into plaintext and sends to the CPU. |
| Bit 20     | XIPSEL   | 0x1         | rw   | XIP port selection<br/>Read SPI Flash data from the following ports:<br/>0: Command slave port<br/>1: XIP port<br/>When this bit is switched, the QSPI sends automatically an Abort signal. The user can send a command only after the completion of Abort function.                                       |
| Bit 19     | XIPRCMDF | 0x0         | rw   | XIP read command flush                                                                                                                                                                                                                                                                                     |


2025.05.28 | Page 681 | Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

| Bit 18: 16 | BUSY     | 0x0  | rw   | Busy bit of SPI status<br/>The host polls this busy bit and remains in hardware read state.<br/>000\~111: bit 0\~bit7                                                                                                                        |
| ---------- | -------- | ---- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 15: 9  | Reserved | 0x00 | resd | Kept at its default value.                                                                                                                                                                                                                   |
| Bit 8      | ABORT    | 0x0  | rw   | Refresh all commands/FIFOs and reset state machine<br/>When an Abort event occurs, this bit must be written (This bit is automatically cleared to 0).<br/>0: No effect<br/>1: Enabled                                                        |
| Bit 7      | XIPIDLE  | 0x1  | ro   | XIP port idle status<br/>0: XIP port is busy<br/>1: XIP port is idle                                                                                                                                                                         |
| Bit 6: 5   | Reserved | 0x0  | resd | Kept at its default value.                                                                                                                                                                                                                   |
| Bit 4      | SCKMODE  | 0x0  | rw   | Sckout mode<br/>0: For mode 0, sck\_out is low in idle state, with data capture on the first edge<br/>1: For mode 3, sck\_out is high in idle state, with data capture on the second edge                                                    |
| Bit 3      | Reserved | 0x0  | resd | Kept at its default value.                                                                                                                                                                                                                   |
| Bit 2: 0   | CLKDIV   | 0x3  | rrw  | Clk divider<br/>This field is used to divide the spi\_clk.<br/>000: Divided by 2<br/>001: Divided by 4<br/>010: Divided by 6<br/>011: Divided by 8<br/>100: Divided by 3<br/>101: Divided by 5<br/>110: Divided by 10<br/>111: Divided by 12 |


## 28.4.6 FIFO status register (FIFOSTS)

No-wait states, accessible by bytes, half-words and words.

| Bit       | Name      | Reset value | Type | Description                                                                                                                                                                        |
| --------- | --------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 2 | Reserved  | 0x0000 0000 | resd | Kept at its default value.                                                                                                                                                         |
| Bit 1     | RXFIFORDY | 0x0         | ro   | RxFIFO ready status<br/>When this bit is set, it indicates that RxFIFO is full or that the remaining data in the RxFIFO is less than the depth of RxFIFO, but it is the last data. |
| Bit 0     | TXFIFORDY | 0x1         | ro   | TxFIFO ready status<br/>When the TxFIFO is set, it indicates that the TxFIFO will get empty so that data can be transmitted into it until it becomes full.                         |


## 28.4.7 Control register 2 (CTRL2)

No-wait states, accessible by bytes, half-words and words.

| Bit        | Name     | Reset value | Type | Description                |
| ---------- | -------- | ----------- | ---- | -------------------------- |
| Bit 31: 14 | Reserved | 0x0000 0    | resd | Kept at its default value. |


2025.05.28
Page 682
Rev 2.07





ARTERY logo AT32F435/437 Series Reference Manual

| Bit 13: 12 | RXFIFO THOD | 0x0  | rw   | This field is used to program the level value to trigger RxFIFO threshold interrupt for DMA handshake mode. The value is in terms of word.<br/>The trigger value is the data in the RxFIFO.<br/>00: 8 WORD<br/>01: 16 WORD<br/>10: 24 WORD<br/>11: Reserved |
| ---------- | ----------- | ---- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 11: 10 | Reserved    | 0x0  | resd | Kept at its default value.                                                                                                                                                                                                                                  |
| Bit 9: 8   | TXFIFO THOD | 0x0  | rw   | This field is used to program the level value to trigger TxFIFO threshold interrupt for DMA handshake mode. The value is in terms of word.<br/>The trigger value is the data in the TxFIFO.<br/>00: 8 WORD<br/>01: 16 WORD<br/>10: 24 WORD<br/>11: Reserved |
| Bit 7: 2   | Reserved    | 0x00 | resd | Kept at its default value.                                                                                                                                                                                                                                  |
| Bit 1      | CMDIE       | 0x0  | rw   | Command complete Interrupt enable<br/>0: Command complete Interrupt disabled<br/>1: Command complete Interrupt enabled                                                                                                                                      |
| Bit 0      | DMAEN       | 0x0  | rw   | DMA enable<br/>Note: This bit must be disabled before moving from command-based slave port to XIP port.                                                                                                                                                     |


# 28.4.8 Command status register (CMDSTS)

No-wait states, accessible by bytes, half-words and words.

| Bit       | Name     | Reset value | Type | Description                                              |
| --------- | -------- | ----------- | ---- | -------------------------------------------------------- |
| Bit 31: 1 | Reserved | 0x0000 0000 | resd | Kept at its default value.                               |
| Bit 0     | CMDSTS   | 0x0         | rw1c | Command complete status<br/>Set at the end of a command. |


# 28.4.9 Read status register (RSTS)

No-wait states, accessible by bytes, half-words and words.

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                  |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 1 | Reserved | 0x0000 00   | resd | Kept at its default value.                                                                                                                                                   |
| Bit 7: 0  | SPISTS   | 0x00        | ro   | SPI Read status<br/>The host sends a read SPI Flash status command and stores the returned data in this register. By reading it, the user can check the status of SPI Flash. |


# 28.4.10 Flash size register (FSIZE)

No-wait states, accessible by bytes, half-words and words.

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                              |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | SPIFSIZE | 0xF000 0000 | rw   | SPI Flash Size<br/>In direct address map mode, system address is always greater than that of SPI Flash. The user must mask the upper bits of the system address to match SPI Flash size. |


2025.05.28 Page 683 Rev 2.07





ARTERY logo AT32F435/437 Series Reference Manual

# 28.4.11 XIP command word 0 (XIP CMD_W0)

No-wait states, accessible by words.

| Bit        | Name         | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                |
| ---------- | ------------ | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 20 | Reserved     | 0x000       | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                 |
| Bit 19: 12 | XIPR\_INSC   | 0x03        | rw   | XIP read instruction code<br/>This field is set to execute SPI Flash command of XPI read.                                                                                                                                                                                                                                                                                                                  |
| Bit 11     | XIPR\_ADRLEN | 0x0         | rw   | XIP read address length<br/>This bit defines the number of bytes of SPI Flash address.<br/>The user can uses this bit to program a 3-byte or 4-byte XIP read address.<br/>0: 3-byte address<br/>1: 4-byte address                                                                                                                                                                                          |
| Bit 10: 8  | XIPR\_OPMODE | 0x0         | rw   | XIP read Operation mode<br/>000: Serial mode (1-1-1)<br/>001: Dual mode (1-1-2)<br/>010: Quad mode (1-1-4)<br/>011: Dual IO mode (1-2-2)<br/>100: Quad IO mode (1-4-4)<br/>101: DPI mode (2-2-2)<br/>110: QPI mode (4-4-4)<br/>111: Reserved                                                                                                                                                               |
| Bit 7: 0   | XIPR\_DUM2   | 0x00        | rw   | XIP Read second dummy cycle<br/>The second dummy state is located between the address and data status, excluding continuous read mode status.<br/>The user can check if there is a dummy state between the address and data status in SPI Flash specification. The host controller issues logic 1 in a dummy cycle.<br/>0: No second dummy state<br/>1\~32: 1 dummy second period\~32 dummy second periods |


# 28.4.12 XIP command word 1 (XIP CMD_W1)

No-wait states, accessible by words.

| Bit        | Name         | Reset value | Type | Description                                                                                                                                                                                                                                   |
| ---------- | ------------ | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 18 | Reserved     | 0x000       | resd | Kept at its default value.                                                                                                                                                                                                                    |
| Bit 19: 12 | XIPW\_INSC   | 0x02        | rw   | XIP write instruction code<br/>The user can set this field to enable a SPI Flash command of XIP write.                                                                                                                                        |
| Bit 11     | XIPW\_ADRLEN | 0x0         | rw   | XIP write address length<br/>This bit defines the number of bytes of SPI Flash address.<br/>The user can uses this bit to program a 3-byte or 4-byte XIP write address.<br/>0: 3-byte address<br/>1: 4-byte address                           |
| Bit 10: 8  | XIPW\_OPMODE | 0x0         | rw   | XIP write operation mode<br/>000: Serial mode (1-1-1)<br/>001: Dual mode (1-1-2)<br/>010: Quad mode (1-1-4)<br/>011: Dual IO mode (1-2-2)<br/>100: Quad IO mode (1-4-4)<br/>101: DPI mode (2-2-2)<br/>110: QPI mode (4-4-4)<br/>111: Reserved |


2025.05.28 Page 684 Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

| Bit 7: 0 | XIPW\_DUM2 | 0x00 | rw | XIP Write second dummy cycle<br/>The second dummy state is located between the address and data status, excluding continuous read mode status. The user can check if there is a dummy state between the address and data status in SPI Flash specification. The host controller issues logic 1 in a dummy cycle.<br/>0: No second dummy state<br/>1\~32: 1 dummy second period\~32 dummy second periods |
| -------- | ---------- | ---- | -- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |


## 28.4.13 XIP command word 2 (XIP CMD_W2)

No-wait states, accessible by words.

| Bit        | Name       | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ---------- | ---------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31     | XIPW\_SEL  | 0x0         | rw   | XIP write mode select<br/>0: Mode D<br/>1: Mode T<br/>The XIP slave port can be used for the improvement of read/write process and performance.<br/>Mode D: When data are written to the consecutive addresses, put a limit on the maximum data count (DCNT) in a single write operation<br/>Mode T: When two consecutive data are written and the addresses are also continuous, and the interval is less than a specified time (TCNT), then they are combined into one command. |
| Bit 30: 24 | XIPW\_TCNT | 0x0F        | rw   | This indicates the time counter that is used to judge time interval in mode T.<br/>Value is in terms of sck\_out period.<br/>This counter is valid when mode T is selected.                                                                                                                                                                                                                                                                                                       |
| Bit 23: 22 | Reserved   | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Bit 21: 16 | XIPW\_DCNT | 0x01        | rw   | This indicates the time counter that is used to judge the maximum data count in mode D.<br/>Value is in terms of word, and must not be 0.<br/>This counter is valid when mode D is selected.                                                                                                                                                                                                                                                                                      |
| Bit 15     | XIPR\_SEL  | 0x0         | rw   | XIP read mode select<br/>0: Mode D<br/>1: Mode T<br/>The XIP slave port can be used for the improvement of read/write process and performance.<br/>Mode D: When data are read from the consecutive addresses, put a limit on the maximum data count (DCNT) in a single write operation<br/>Mode T: When two consecutive data are read and the addresses are also continuous, and the interval is less than a specified time (TCNT), then they are combined into one command.      |
| Bit 14: 8  | XIPR\_TCNT | 0x0F        | rw   | This indicates the time counter that is used to judge time interval in mode T.<br/>Value is in terms of sck\_out period.<br/>This counter is valid when mode T is selected.                                                                                                                                                                                                                                                                                                       |
| Bit 7: 6   | Reserved   | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Bit 5: 0   | XIPR\_DCNT | 0x01        | rw   | This indicates the time counter that is used to judge the maximum data count in mode D.<br/>Value is in terms of word, and must not be 0.<br/>This counter is valid when mode D is selected.                                                                                                                                                                                                                                                                                      |


2025.05.28
Page 685
Rev 2.07





Artery logo AT32F435/437 Series Reference Manual

### 28.4.14 XIP command word 3 (XIP CMD_W3)

No-wait states, accessible by bytes, half-words and words.

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                  |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 4 | Reserved | 0x0000 000  | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                   |
| Bit 3     | CSTS     | 0x0         | r    | Cache Status<br/>0: Cache verified<br/>1: Cache failed                                                                                                                                                                                                                                                                       |
| Bit 2: 1  | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                   |
| Bit 0     | BYPASSC  | 0x0         | rw   | Bypass Cache Function<br/>When this bit is set, the high-speed cache feature is deactivated, and all read transfers do not check high-speed cache.<br/>Cache is only applicable to XIP Read only application (extended external Flash). For XIP Read/write application (extended external PSRAM), this bit must be set to 1. |


### 28.4.15 Control register (CTRL3)

No-wait states, accessible by bytes, half-words and words.

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                              |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 9 | Reserved | 0x0000 000  | resd | Kept at its default value.                                                                                                                                                                                                               |
| Bit 8     | ISPC     | 0x0         | rw   | Input sampling phase correction enable<br/>This bit is used to correct sampling phase according to input sampling phase delay and SPI Flash output timing.<br/>0: Fixed sampling phase<br/>1: Input sampling phase correction is enabled |
| Bit 7: 6  | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                               |
| Bit 0     | ISPD     | 0x00        | rw   | Input sampling phase delay<br/>When ISPC is set, this bit setting is taken into account.                                                                                                                                                 |


### 28.4.16 Revision register (REV)

No-wait states, accessible by bytes, half-words and words.

| Bit       | Name | Reset value | Type | Description           |
| --------- | ---- | ----------- | ---- | --------------------- |
| Bit 31: 0 | REV  | 0x0001 0500 | ro   | Indicates IP version. |


### 28.4.17 Data port register (DT)

No-wait states, accessible by bytes, half-words and words.

| Bit       | Name | Reset value | Type | Description                                                      |
| --------- | ---- | ----------- | ---- | ---------------------------------------------------------------- |
| Bit 31: 0 | DT   | 0x0000 0000 | rw   | Data port register<br/>This port is used for data read or write. |


2025.05.28 Page 686 Rev 2.07
