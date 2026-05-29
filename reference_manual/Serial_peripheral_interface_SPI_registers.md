
ARTERY logo AT32F435/437 Series Reference Manual

# 13.4 SPI registers

These peripheral registers must be accessed by words (32 bits).

Table 13-2 SPI register map and reset value

| Register     | Offset | Reset value |
| ------------ | ------ | ----------- |
| SPI\_CTRL1   | 0x00   | 0x0000      |
| SPI\_CTRL2   | 0x04   | 0x0000      |
| SPI\_STS     | 0x08   | 0x0002      |
| SPI\_DT      | 0x0C   | 0x0000      |
| SPI\_CPOLY   | 0x10   | 0x0007      |
| SPI\_RCRC    | 0x14   | 0x0000      |
| SPI\_TCRC    | 0x18   | 0x0000      |
| SPI\_I2SCTRL | 0x1C   | 0x0000      |
| SPI\_I2SCLKP | 0x20   | 0x0002      |


## 13.4.1 SPI control register 1 (SPI_CTRL1) (Not used in I<sup>2</sup>S mode)

| Bit    | Name   | Reset value | Type | Description                                                                                                                                                                                                                                  |
| ------ | ------ | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 15 | SLBEN  | 0x0         | rw   | Single line bidirectional half-duplex enable<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                  |
| Bit 14 | SLBTD  | 0x0         | rw   | Single line bidirectional half-duplex transmission direction<br/>This bit and the SLBEN bit together determine the data output direction in “Single line bidirectional half-duplex” mode.<br/>0: Receive-only mode<br/>1: Transmit-only mode |
| Bit 13 | CCEN   | 0x0         | rw   | CRC calculation enable<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                                        |
| Bit 12 | NTC    | 0x0         | rw   | Transmit CRC next<br/>When this bit is set, it indicates that the next data transferred is CRC value.<br/>0: Next transmitted data is the normal value<br/>1: Next transmitted data is CRC value                                             |
| Bit 11 | FBN    | 0x0         | rw   | Frame bit num<br/>This bit is used to configure the number of data frame bit for transmission/reception.<br/>0: 8-bit data frame<br/>1: 16-bit data frame                                                                                    |
| Bit 10 | ORA    | 0x0         | rw   | Receive-only active<br/>In two-wire unidirectional mode, when this bit is set, it indicates that Receive-only is active, but the transmit is not allowed.<br/>0: Transmission and reception<br/>1: Receive-only mode                         |
| Bit 9  | SWCSEN | 0x0         | rw   | Software CS enable<br/>When this bit is set, the CS pin level is determined by the SWCSIL bit. The status of I/O level on the CK pin is invalid.<br/>0: Disabled<br/>1: Enabled                                                              |
| Bit 8  | SWCSIL | 0x0         | rw   | Software CS internal level<br/>This bit is valid only when the SWCSEN is set. It determines the level on the CS pin.<br/>In master mode, this bit must be set.<br/>0: Low level<br/>1: High level                                            |


2025.05.28 Page 248 Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

| Bit 7    | LTF    | 0x0 | rw | LSB transmit first<br/>This bit is used to select for MST transfer first or LSB transfer first.<br/>0: MSB<br/>1: LSB                                                                                                                                                                                                                                                                                                                  |
| -------- | ------ | --- | -- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 6    | SPIEN  | 0x0 | rw | SPI enable<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                                                                                                                                                                                                                                              |
| Bit 5: 3 | MDIV   | 0x0 | rw | Master clock frequency division<br/>In master mode, the peripheral clock divided by the prescaler is used as SPI clock. The MDIV\[3] bit is in the SPI\_CTRL2 register, MDIV\[3: 0]:<br/>0000: Divided by 2<br/>0001: Divided by 4<br/>0010: Divided by 8<br/>0011: Divided by 16<br/>0100: Divided by 32<br/>0101: Divided by 64<br/>0110: Divided by 128<br/>0111: Divided by 256<br/>1000: Divided by 512<br/>1001: Divided by 1024 |
| Bit 2    | MSTEN  | 0x0 | rw | Master enable<br/>0: Disabled (Slave)<br/>1: Enabled (Master)                                                                                                                                                                                                                                                                                                                                                                          |
| Bit 1    | CLKPOL | 0x0 | rw | Clock polarity<br/>Indicates the polarity of clock output in idle state.<br/>0: Low level<br/>1: High level                                                                                                                                                                                                                                                                                                                            |
| Bit 0    | CLKPHA | 0x0 | rw | Clock phase<br/>0: Data capture starts from the first clock edge<br/>1: Data capture starts from the second clock edge                                                                                                                                                                                                                                                                                                                 |


Note: The SPI_CTRL1 register must be 0 in I²S mode.

## 13.4.2 SPI control register 2 (SPI_CTRL2)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                          |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 15: 10 | Reserved | 0x00        | resd | Forced 0 by hardware.                                                                                                                                                                                                                                                                |
| Bit 9      | MDIV3EN  | 0x0         | rw   | Master clock frequency divided by 3 enable<br/>0: Disabled<br/>1: Enabled<br/>Note: When this bit is set, the MDIV\[3: 0] becomes invalid, and the SPI clock is forced to be PCLK/3.<br/>When SPI/3 and PCLK/SYSCLK≠1, SPI does not meet 50% duty cycle. See data sheet for details. |
| Bit 8      | MDIV\[3] | 0x0         | rw   | Master clock frequency division<br/>Refer to the MDIV\[2: 0] of the SPI\_CTRL1 register.                                                                                                                                                                                             |
| Bit 7      | TDBEIE   | 0x0         | rw   | Transmit data buffer empty interrupt enable<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                                                           |
| Bit 6      | RDBFIE   | 0x0         | rw   | Receive data buffer full interrupt enable<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                                                             |
| Bit 5      | ERRIE    | 0x0         | rw   | Error interrupt enable<br/>This bit controls interrupt generation when errors occur (CCERR, MMERR, ROERR, TUERR and CSPAS)<br/>0: Disabled<br/>1: Enabled                                                                                                                            |
| Bit 4      | TIEN     | 0x0         | rw   | TI mode enable                                                                                                                                                                                                                                                                       |


2025.05.28
Page 249
Rev 2.07




ARTERY logo

# AT32F435/437 Series Reference Manual

0: TI mode disabled (Motorola mode)

1: TI mode enabled (TI mode)

Note: This mode is not used in I2S mode. It must be 0 in I2S mode.

| Bit 3 | Reserved | 0x0 | resd | Kept at its default value                                                                                                                                                                                                        |
| ----- | -------- | --- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 2 | HWCSOE   | 0x0 | rw   | Hardware CS output enable<br/>This bit is valid only in master mode. When this bit is set, the I/O output on the CS pin is low; when this bit is 0, the I/O input on the CS pin must be set high.<br/>0: Disabled<br/>1: Enabled |
| Bit 1 | DMATEN   | 0x0 | rw   | DMA transmit enable<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                               |
| Bit 0 | DMAREN   | 0x0 | rw   | DMA receive enable<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                                |


## 13.4.3 SPI status register (SPI_STS)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                   |
| --------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 15: 9 | Reserved | 0x00        | resd | Forced 0 by hardware                                                                                                                                                                                                          |
| Bit 8     | CSPAS    | 0x0         | ro   | CS pulse abnormal setting flag<br/>0: CS pulse flag normal<br/>1: CS pulse flag is set abnormally<br/>Note: This bit is used for TI slave mode. It is cleared by reading the STS register.                                    |
| Bit 7     | BF       | 0x0         | ro   | Busy flag<br/>0: SPI is not busy.<br/>1: SPI is busy.                                                                                                                                                                         |
| Bit 6     | ROERR    | 0x0         | ro   | Receiver overflow error<br/>0: No overflow error<br/>1: Overflow error occurs.                                                                                                                                                |
| Bit 5     | MMERR    | 0x0         | ro   | Master mode error<br/>This bit is set by hardware and cleared by software (read/write access to the SPI\_STS register, followed by write operation to the SPI\_CTRL1 register)<br/>0: No mode error<br/>1: Mode error occurs. |
| Bit 4     | CCERR    | 0x0         | rw0c | CRC error<br/>Set by hardware, and cleared by software.<br/>0: No CRC error<br/>1: CRC error occurs.                                                                                                                          |
| Bit 3     | TUERR    | 0x0         | ro   | Transmitter underload error<br/>Set by hardware, and cleared by software (read the SPI\_STS register).<br/>0: No underload error<br/>1: Underload error occurs.<br/>Note: This bit is only used in I2S mode.                  |
| Bit 2     | ACS      | 0x0         | ro   | Audio channel state<br/>This bit indicates the status of the current audio channel.<br/>0: Left channel<br/>1: Right channel<br/>Note: This bit is only used in I2S mode.                                                     |
| Bit 1     | TDBE     | 0x1         | ro   | Transmit data buffer empty                                                                                                                                                                                                    |


2025.05.28
Page 250
Rev 2.07




ARTERY logo # AT32F435/437 Series Reference Manual

| Bit   | Name | Reset value | Type | Description                                                                                            |
| ----- | ---- | ----------- | ---- | ------------------------------------------------------------------------------------------------------ |
|       |      |             |      | 0: Transmit data buffer is not empty.<br/>1: Transmit data buffer is empty.                            |
| Bit 0 | RDBF | 0x0         | ro   | Receive data buffer full<br/>0: Transmit data buffer is not full.<br/>1: Transmit data buffer is full. |


## 13.4.4 SPI data register (SPI_DT)

| Bit       | Name | Reset value | Type | Description                                                                                                                              |
| --------- | ---- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 15: 0 | DT   | 0x0000      | rw   | Data value<br/>This register controls read and write operations. When the data bit is set as 8 bit, only the 8-bit LSB \[7: 0] is valid. |


## 13.4.5 SPICRC register (SPI_CPOLY) (Not used in I<sup>2</sup>S mode)

| Bit       | Name  | Reset value | Type | Description                                                                                                                           |
| --------- | ----- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 15: 0 | CPOLY | 0x0007      | rw   | CRC polynomial<br/>This register contains the polynomial used for CRC calculation.<br/>Note: This register is valid only in SPI mode. |


## 13.4.6 SPICRC receive register (SPI_RCRC) (Not used in I<sup>2</sup>S mode)

| Bit       | Name | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| --------- | ---- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 15: 0 | RCRC | 0x0000      | ro   | Receive CRC<br/>When CRC calculation is enabled, this register contains the CRC value computed based on the received data. This register is reset when the CCEN bit in the SPI\_CTRL1 register is cleared.<br/>When the data frame format is set to 8-bit data, only the 8-bit LSB (\[7: 0]) are calculated based on CRC8 standard; when 16-bit data bit is selected, follow CRC16 standard.<br/>Note: This register is only used in SPI mode. |


## 13.4.7 SPICRC transmit register (SPI_TCRC) (Not used in I<sup>2</sup>S mode)

| Bit       | Name | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| --------- | ---- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 15: 0 | TCRC | 0x0000      | ro   | Transmit CRC<br/>When CRC calculation is enabled, this register contains the CRC value computed based on the transmitted data. This register is reset when the CCEN bit in the SPI\_CTRL1 register is cleared.<br/>When the data frame format is set to 8-bit data, only the 8-bit LSB (\[7: 0]) are calculated based on CRC8 standard; when 16-bit data bit is selected, follow CRC16 standard.<br/>Note: This register is only used in SPI mode. |


## 13.4.8 SPI_I2S register (SPI_I2SCTRL)

| Bit        | Name     | Reset value | Type | Description                                     |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------- |
| Bit 15: 12 | Reserved | 0x0         | resd | Forced 0 by hardware.                           |
| Bit 11     | I2SMSEL  | 0x0         | rw   | I2S mode select<br/>0: SPI mode<br/>1: I2S mode |
| Bit 10     | I2SEN    | 0x0         | rw   | I2S enable<br/>0: Disabled<br/>1: Enabled       |
| Bit 9: 8   | OPERSEL  | 0x0         | rw   | I2S operation mode select                       |


2025.05.28 Page 251 Rev 2.07





ARTERY logo AT32F435/437 Series Reference Manual

|          |           |     |      | 00: Slave transmission<br/>01: Slave reception<br/>10: Master transmission<br/>11: Master reception                                                                             |
| -------- | --------- | --- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 7    | PCMFSSEL  | 0x0 | rw   | PCM frame synchronization<br/>This bit is valid only when the PCM standard is used.<br/>0: Short frame synchronization<br/>1: Long frame synchronization                        |
| Bit 6    | Reserved  | 0x0 | resd | Kept at its default value                                                                                                                                                       |
| Bit 5: 4 | STDSEL    | 0x0 | rw   | I2S standard select<br/>00: Philips standard<br/>01: MSB-aligned standard (left-aligned)<br/>10: LSB-aligned standard (right-aligned)<br/>11: PCM standard                      |
| Bit 3    | I2SCLKPOL | 0x0 | rw   | I2S clock polarity<br/>This bit indicates the clock polarity on the clock pin in idle state.<br/>0: Low<br/>1: High                                                             |
| Bit 2: 1 | I2SDBN    | 0x0 | rw   | I2S data bit num<br/>00: 16-bit data length<br/>01: 24-bit data length<br/>10: 32-bit data length<br/>11: Not allowed.                                                          |
| Bit 0    | I2SCBN    | 0x0 | rw   | I2S channel bit num<br/>This bit can be configured only when the I2S is set to 16-bit data; otherwise, it is fixed to 32-bit by hardware.<br/>0: 16-bit wide<br/>1: 32-bit wide |


## 13.4.9 SPI_I2S prescaler register (SPI_I2SCLKP)

| Bit                     | Name      | Reset value | Type | Description                                                                                                   |
| ----------------------- | --------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------- |
| Bit 15: 12              | Reserved  | 0x0         | resd | Forced 0 by hardware.                                                                                         |
| Bit 9                   | I2SMCLKOE | 0x0         | rw   | I2S Master clock output enable<br/>0: Disabled<br/>1: Enabled                                                 |
| Bit 8                   | I2SODD    | 0x0         | rw   | Odd factor for I2S division<br/>0: Actual divider factor =I2SDIV*2<br/>1: Actual divider factor =(I2SDIV*2)+1 |
| Bit 11: 10<br/>Bit 7: 0 | I2SDIV    | 0x02        | rw   | I2S division<br/>It is not allowed to configure I2SDIV\[9: 0]=0 or I2SDIV\[9: 0]=1                            |


2025.05.28 Page 252 Rev 2.07
