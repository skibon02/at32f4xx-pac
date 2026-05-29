

### 24.7.2 Self-refresh mode and Power-down mode

The SDRAM controller supports two low-power modes: self-refresh mode and power-down mode.

**Self-refresh mode**

This mode is selected by setting CMD=101 and by configuring the Target Bank bits (BK1 and/or BK2) in the SDRAM_CMD register.

The SDRAM clock stops running after a TRAS delay. The internal refresh time stops counting if one of the following conditions is present:

When both SDRAM devices have been initialized, a self-refresh command is issued to both devices; or when one of the devices is not initialized, a self-refresh command is issued to one of the device.

After entering the self-refresh mode, the SDRAM device must remain in this mode for a minimum period of time of TRAS, which is determined by the SDRAM device characteristics. To guarantee this minimum period, the BUSY flag will be set after the self-refresh is entered. The BUSY flag is cleared automatically only after a TRAS time. This bit can be used to judge whether to exit self-refresh mode.

It is possible to leave self-refresh mode by setting CMD=000 or by read/write access to the SDRAM.

**Power-down mode**

This mode is selected by setting CMD=110 and by configuring the Target Bank bits (BK1 and/or BK2) in the SDRAM_CMD register.

During power-down mode, the SDRAM device can also perform auto-refresh operation. When an auto-refresh command occurs, the SDRAM exists from the power-down mode before performing auto-refresh operation. After that, the SDRAM will enter power-down mode again.

It is possible to leave power-down mode by setting CMD=000 or by read/write access to the SDRAM.

## 24.8 XMC registers

These peripheral registers must be accessed by words (32 bits).

**Table 24-39 XMC register address mapping**

| Register      | Offset | Reset value |
| ------------- | ------ | ----------- |
| XMC\_BK1CTRL1 | 0x000  | 0x0000 30DB |
| XMC\_BK1TMG1  | 0x004  | 0x0FFF FFFF |
| XMC\_BK1CTRL2 | 0x008  | 0x0000 30D2 |
| XMC\_BK1TMG2  | 0x00C  | 0x0FFF FFFF |
| XMC\_BK1CTRL3 | 0x010  | 0x0000 30D2 |
| XMC\_BK1TMG3  | 0x014  | 0x0FFF FFFF |
| XMC\_BK1CTRL4 | 0x018  | 0x0000 30D2 |
| XMC\_BK1TMG4  | 0x01C  | 0x0FFF FFFF |
| XMC\_BK2CTRL  | 0x060  | 0x0000 0018 |
| XMC\_BK2IS    | 0x064  | 0x0000 0040 |
| XMC\_BK2TMGRG | 0x068  | 0xFCFC FCFC |
| XMC\_BK2TMGSP | 0x06C  | 0xFCFC FCFC |
| XMC\_BK2ECC   | 0x074  | 0x0000 0000 |
| XMC\_BK3CTRL  | 0x080  | 0x0000 0018 |
| XMC\_BK3IS    | 0x084  | 0x0000 0040 |
| XMC\_BK3TMGRG | 0x088  | 0xFCFC FCFC |
| XMC\_BK3TMGSP | 0x08C  | 0xFCFC FCFC |
| XMC\_BK3ECC   | 0x094  | 0x0000 0000 |
| XMC\_BK4CTRL  | 0x0A0  | 0x0000 0018 |
| XMC\_BK4IS    | 0x0A4  | 0x0000 0040 |
| XMC\_BK4TMGCM  | 0x0A8 | 0xFCFC FCFC |
| XMC\_BK4TMGAT  | 0x0AC | 0xFCFC FCFC |
| XMC\_BK4TMGIO  | 0xB0  | 0xFCFC FCFC |
| XMC\_BK1TMGWR1 | 0x104 | 0x0FFF FFFF |
| XMC\_BK1TMGWR2 | 0x10C | 0x0FFF FFFF |
| XMC\_BK1TMGWR3 | 0x114 | 0x0FFF FFFF |
| XMC\_BK1TMGWR4 | 0x11C | 0x0FFF FFFF |
| XMC\_EXT1      | 0x220 | 0x0000 0808 |
| XMC\_EXT2      | 0x224 | 0x0000 0808 |
| XMC\_EXT3      | 0x228 | 0x0000 0808 |
| XMC\_EXT4      | 0x22C | 0x0000 0808 |
| SDRAM\_CTRL1   | 0x140 | 0x0000 02D0 |
| SDRAM\_CTRL2   | 0x144 | 0x0000 02D0 |
| SDRAM\_TM1     | 0x148 | 0x0FFF FFFF |
| SDRAM\_TM2     | 0x14C | 0x0FFF FFFF |
| SDRAM\_CMD     | 0x150 | 0x0000 0007 |
| SDRAM\_RCNT    | 0x154 | 0x0000 0000 |
| SDRAM\_STS     | 0x158 | 0x0000 0000 |


# 24.8.1 NOR Flash and PSRAM control registers
## 24.8.1.1 SRAM/NOR Flash chip select control register 1 (XMC_BK1CTRL1)

<u>Accessed by words</u>

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 20 | Reserved | 0x000       | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                     |
| Bit 19     | MWMC     | 0x0         | rw   | Memory write mode control<br/>0: Write operations are performed in asynchronous mode<br/>1: Write operations are performed in synchronous mode                                                                                                                                                                                                                                                                                 |
| Bit 18: 16 | CRPGS    | 0x0         | rw   | CRAM page size<br/>Cellular RAM 1.5 does not allow synchronous access to cross the address boundaries between pages. When these bits are configured in synchronous mode, the XMC will automatically split the access when the page size is reached.<br/>000: No split access when crossing address boundary (default value)<br/>001: 128 bytes<br/>010: 256 bytes<br/>011: 512 bytes<br/>100: 1024 bytes<br/>Others: Reserved. |
| Bit 15     | NWASEN   | 0x0         | rw   | NWAIT in asynchronous transfer enable<br/>0: NWAIT signal is disabled<br/>1: NWAIT signal is enable                                                                                                                                                                                                                                                                                                                            |
| Bit 14     | RWTD     | 0x0         | rw   | Read-write timing different<br/>Different timings are used for read and write operations, that is, the XMC\_BK1TMGWR1 register is enabled.<br/>0: Same timings for read and write operations<br/>1: Different timings for read and write operations                                                                                                                                                                            |
| Bit 13     | NWSEN    | 0x1         | rw   | NWAIT enable during synchronous transfer<br/>0: NWAIT signal is disabled                                                                                                                                                                                                                                                                                                                                                       |
| Bit 12   | WEN      | 0x1 | rw   | 1: NWAIT signal is enabled<br/>Write enable<br/>0: Disabled<br/>1: Enabled                                                                                                                                 |
| Bit 11   | NWTCFG   | 0x0 | rw   | NWAIT timing configuration<br/>It is valid only in synchronous mode.<br/>0: NWAIT signal is active one data cycle before the wait state<br/>1: NWAIT signal is active one data cycle during the wait state |
| Bit 10   | WRAPEN   | 0x0 | rw   | Wrapped enable<br/>This bit defines whether the XMC will split a wrapped AHB access into two accesses.<br/>0: Direct wrapped access is not allowed<br/>1: Direct wrapped access is allowed                 |
| Bit 9    | NWPOL    | 0x0 | rw   | NWAIT polarity<br/>This bit defines the polarity of the NWAIT signal in synchronous mode.<br/>0: NWAIT active low<br/>1: NWAIT active high                                                                 |
| Bit 8    | SYNCBEN  | 0x0 | rw   | Synchronous burst enable<br/>This bit allows synchronous access to Flash memories.<br/>0: Synchronous burst disabled<br/>1: Synchronous burst enabled                                                      |
| Bit 7    | Reserved | 0x1 | resd | Kept at its default value.                                                                                                                                                                                 |
| Bit 6    | NOREN    | 0x1 | rw   | Nor flash access enable<br/>0: Nor flash access is disabled<br/>1: Nor flash access is enabled                                                                                                             |
| Bit 5: 4 | EXTMDBW  | 0x1 | rw   | External memory data bus width<br/>This field defines the external memory data bus width.<br/>00: 8 bits<br/>01: 16 bits<br/>10: Reserved<br/>11: Reserved                                                 |
| Bit 3: 2 | DEV      | 0x2 | rw   | Memory device type<br/>00: SRAM/ROM<br/>01: PSRAM (Cellular RAM or CRAM)<br/>10: NOR Flash<br/>11: Reserved                                                                                                |
| Bit 1    | ADMUXEN  | 0x1 | rw   | Address/data multiplexing enable<br/>0: Address/data not multiplexed<br/>1: Address/data multiplexed                                                                                                       |
| Bit 0    | EN       | 0x1 | rw   | Memory bank enable<br/>0: Memory bank disabled<br/>1: Memory bank enabled                                                                                                                                  |


# 24.8.1.2 SRAM/NOR Flash chip select control register x (x=2, 3, 4)

Accessed by words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                          |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 20 | Reserved | 0x000       | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                           |
| Bit 19     | MWMC     | 0x0         | rw   | Memory write mode control<br/>0: Write operations are performed in asynchronous mode<br/>1: Write operations are performed in synchronous mode                                                                                                                                                                                                                                       |
| Bit 18: 16 | CRPGS    | 0x0         | rw   | CRAM page size<br/>Cellular RAM 1.5 does not allow synchronous access to cross the address boundaries between pages. When these bits are configured in synchronous mode, the XMC will automatically split the access when the page size is reached.<br/>000: No split access when crossing address boundary (default value)<br/>001: 128 bytes<br/>010: 256 bytes<br/>011: 512 bytes |
|          |          |     |      | 100: 1024 bytes<br/>Others: Reserved.                                                                                                                                                                                                                   |
| Bit 15   | NWASEN   | 0x0 | rw   | NWAIT enable during asynchronous transfer<br/>0: NWAIT signal is disabled<br/>1: NWAIT signal is enable                                                                                                                                                 |
| Bit 14   | RWTD     | 0x0 | rw   | Read-write timing different<br/>Different timings are used for read and write operations,<br/>that is, the XMC\_BK1TMGWRx register is enabled.<br/>0: Same timings for read and write operations<br/>1: Different timings for read and write operations |
| Bit 13   | NWSEN    | 0x1 | rw   | NWAIT enable during synchronous transfer<br/>0: NWAIT signal is disabled<br/>1: NWAIT signal is enabled                                                                                                                                                 |
| Bit 12   | WEN      | 0x1 | rw   | Write enable<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                                                             |
| Bit 11   | NWTCFG   | 0x0 | rw   | NWAIT timing configuration<br/>It is valid only in synchronous mode.<br/>0: NWAIT signal is active one data cycle before the wait<br/>state<br/>1: NWAIT signal is active one data cycle during the wait<br/>state                                      |
| Bit 10   | WRAPEN   | 0x0 | rw   | Wrapped enable<br/>This bit defines whether the XMC will split a wrapped AHB<br/>access into two accesses.<br/>0: Direct wrapped access is not allowed<br/>1: Direct wrapped access is allowed                                                          |
| Bit 9    | NWPOL    | 0x0 | rw   | NWAIT polarity<br/>This bit defines the polarity of the NWAIT signal in<br/>synchronous mode.<br/>0: NWAIT active low<br/>1: NWAIT active high                                                                                                          |
| Bit 8    | SYNCBEN  | 0x0 | rw   | Synchronous burst enable<br/>This bit allows synchronous access to Flash memories.<br/>0: Synchronous burst disabled<br/>1: Synchronous burst enabled                                                                                                   |
| Bit 7    | Reserved | 0x1 | resd | Kept at its default value.                                                                                                                                                                                                                              |
| Bit 6    | NOREN    | 0x1 | rw   | Nor flash access enable<br/>0: Nor flash access is disabled<br/>1: Nor flash access is enabled                                                                                                                                                          |
| Bit 5: 4 | EXTMDBW  | 0x1 | rw   | External memory data bus width<br/>This field defines the external memory data bus width.<br/>00: 8 bits<br/>01: 16 bits<br/>10: Reserved<br/>11: Reserved                                                                                              |
| Bit 3: 2 | DEV      | 0x0 | rw   | Memory device type<br/>00: SRAM/ROM<br/>01: PSRAM (Cellular RAM or CRAM)<br/>10: NOR Flash<br/>11: Reserved                                                                                                                                             |
| Bit 1    | ADMUXEN  | 0x1 | rw   | Address/data multiplexing enable<br/>0: Address/data not multiplexed<br/>1: Address/data multiplexed                                                                                                                                                    |
| Bit 0    | EN       | 0x0 | rw   | Memory bank enable<br/>0: Memory bank disabled<br/>1: Memory bank enabled                                                                                                                                                                               |


# 24.8.1.3 SRAM/NOR Flash chip select timing register x (XMC_BK1TMGx) (x=1,2,3,4)

<u>Accessed</u> by words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 30 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                 |
| Bit 29: 28 | ASYNCM   | 0x0         | rw   | Asynchronous mode<br/>This field is valid only when the RWTD bit is enabled.<br/>00: Mode A<br/>01: Mode B<br/>10: Mode C<br/>11: Mode D                                                                                                                                                   |
| Bit 27: 24 | DTLAT    | 0xF         | rw   | Data latency<br/>This field is valid only in synchronous mode.<br/><br/>0000: 1 extra XMC\_CLK cycle is inserted<br/>0001: 2 extra XMC\_CLK cycles are inserted<br/>......<br/>1111: 16 extra XMC\_CLK cycles are inserted                                                                 |
| Bit 23: 20 | CLKPSC   | 0xF         | rw   | Clock prescaler<br/>This field is valid only in synchronous mode. It defines the frequency of the XMC\_CLK clock.<br/>0000: Reserved<br/>0001: XMC\_CLK cycle= 2 x HCLK clock cycles<br/>0010: XMC\_CLK cycle =3 x HCLK clock cycles<br/>......<br/>1111: XMC\_CLK cycle = 6 x HCLK cycles |
| Bit 19: 16 | BUSLAT   | 0xF         | rw   | Bus latency<br/>To avoid data bus conflict, a latency is inserted on the data bus after one read operation in multiplexed or synchronous mode.<br/>0000: 1 HCLK cycle is inserted<br/>0001: 2 HCLK cycles are inserted<br/>......<br/>1111: 16 HCLK cycles are inserted                    |
| Bit 15: 8  | DTST     | 0xFF        | rw   | Data setup time<br/>00000000: 1 extra HCLK cycle is inserted<br/>00000001: 2 extra HCLK cycles are inserted<br/>......<br/>11111111: 256 extra HCLK cycles are inserted                                                                                                                    |
| Bit 7: 4   | ADDRHT   | 0xF         | rw   | Address-hold time<br/>0000: 1 HCLK cycle is inserted<br/>0001: 2 extra HCLK cycles are inserted<br/>......<br/>1111: 16 extra HCLK cycles are inserted                                                                                                                                     |
| Bit 3: 0   | ADDRST   | 0xF         | rw   | Address setup time<br/>0000: 1 HCLK cycle is inserted<br/>0001: 2 extra HCLK cycles are inserted<br/>......<br/>1111: 16 extra HCLK cycles are inserted                                                                                                                                    |


# 24.8.1.4 SRAM/NOR Flash write timing register x (XMC_BK1TMGWRx), x=1,2,3,4

<u>Accessed</u> by words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                              |
| ---------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 30 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                               |
| Bit 29: 28 | ASYNCM   | 0x0         | rw   | Asynchronous mode<br/>This field is valid only when the RWTD bit is enabled.<br/>00: Mode A<br/>01: Mode B<br/>10: Mode C<br/>11: Mode D |
| Bit 27: 20 | Reserved | 0xFF        | resd | Kept at its default value.                                                                                                               |
| Bit 19: 16 | BUSLAT | 0xF  | rw | Bus latency<br/>To avoid data bus conflict, a latency is inserted on the data bus after one read operation in multiplexed or synchronous mode.<br/>0000: 1 HCLK cycle is inserted<br/>0001: 2 HCLK cycles are inserted<br/>......<br/>1111: 16 HCLK cycles are inserted |
| Bit 15: 8  | DTST   | 0xFF | rw | Data setup time<br/>00000000: 1 extra HCLK cycle is inserted<br/>00000001: 2 extra HCLK cycles are inserted<br/>......<br/>11111111: 256 extra HCLK cycles are inserted                                                                                                 |
| Bit 7: 4   | ADDRHT | 0xF  | rw | Address-hold time<br/>0000: 1 extra HCLK cycle is inserted<br/>0001: 2 extra HCLK cycles are inserted<br/>......<br/>1111: 16 extra HCLK cycles are inserted                                                                                                            |
| Bit 3: 0   | ADDRST | 0xF  | rw | Address setup time<br/>0000: 1 extra HCLK cycle is inserted<br/>0001: 2 extra HCLK cycles are inserted<br/>......<br/>1111: 16 extra HCLK cycles are inserted                                                                                                           |


# 24.8.1.5 SRAM/NOR Flash extra timing register x (XMC_EXTx) (x=1,2,3,4)

<u>Accessed by words.</u>

| Bit        | Name      | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ---------- | --------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved  | 0x0000      | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Bit 15: 8  | BUSLATR2R | 0x08        | rw   | Bus turnaround phase for consecutive read duration<br/>This field is used to define the bus turnaround phase duration for consecutive read operations. A delay is inserted between two consecutive read operations in order to avoid bus conflicts.<br/>00000000: 1 HCLK cycle is inserted for consecutive read operations<br/>00000001: 2 HCLK cycles are inserted for consecutive read operations<br/>......<br/>00001000: 9 HCLK cycles are inserted for consecutive read operations (default value)<br/>......<br/>11111111: 256 HCLK cycles are inserted for consecutive read operations        |
| Bit 7: 0   | BUSLATW2W | 0x08        | rw   | Bus turnaround phase for consecutive write duration<br/>This field is used to define the bus turnaround phase duration for consecutive write operations. A delay is inserted between two consecutive write operations in order to avoid bus conflicts.<br/>00000000: 1 HCLK cycle is inserted for consecutive write operations<br/>00000001: 2 HCLK cycles are inserted for consecutive write operations<br/>......<br/>00001000: 9 HCLK cycles are inserted for consecutive write operations (default value)<br/>......<br/>11111111: 256 HCLK cycles are inserted for consecutive write operations |


# 24.8.2 NAND Flash control registers

## 24.8.2.1 NAND Flash control register x (XMC_BKxCTRL) (x=2,3)

Accessed by words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                       |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 20 | Reserved | 0x000       | resd | Kept at its default value.                                                                                                                                        |
| Bit 19: 17 | ECCPGS   | 0x0         | rw   | ECC page size<br/>000: 256 bytes<br/>001: 512 bytes<br/>010: 1024 bytes<br/>011: 2048 bytes<br/>100: 4096 bytes<br/>101: 8192 bytes                               |
| Bit 16: 13 | TAR      | 0x0         | rw   | ALE to RE delay<br/>This field specifies the delay from the falling edge of the ALE to that of the RE.<br/>0000: 1 HCLK cycle<br/>......<br/>1111: 16 HCLK cycles |
| Bit 12: 9  | TCR      | 0x0         | rw   | CLE to RE delay<br/>This field specifies the delay from the falling edge of the CLE to that of the RE.<br/>0000: 1 HCLK cycle<br/>......<br/>1111: 16 HCLK cycles |
| Bit 8: 7   | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                        |
| Bit 6      | ECCEN    | 0x0         | rw   | ECC enable<br/>0: ECC disabled<br/>1: ECC enabled                                                                                                                 |
| Bit 5: 4   | EXTMDBW  | 0x1         | rw   | External memory data bus width<br/>This field specifies the external NAND Flash width.<br/>00: 8 bits<br/>01: 16 bits<br/>10: Reserved<br/>11: Reserved           |
| Bit 3      | DEV      | 0x1         | rw   | Memory device type<br/>0: Reserved<br/>1: NAND Flash                                                                                                              |
| Bit 2      | EN       | 0x0         | rw   | Memory bank enable<br/>0: Memory bank disabled<br/>1: Memory bank enabled                                                                                         |
| Bit 1      | NWEN     | 0x0         | rw   | Wait feature enable<br/>This bit is used to enable NAND Flash wait function.<br/>0: Disabled<br/>1: Enabled                                                       |
| Bit 0      | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                        |


## 24.8.2.2 Interrupt enable and FIFO status register x (XMC_BKxIS) (x=2,3)

Accessed by words.

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                            |
| --------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 7 | Reserved | 0x000000    | resd | Kept at its default value.                                                                                                                                                             |
| Bit 6     | FIFOE    | 0x1         | rw   | FIFO empty<br/>This bit is set by hardware when the FIFO is empty.<br/>0: FIFO is not empty<br/>1: FIFO is empty<br/>XMC FIFO size is 16 words. It is used to store the data from AHB. |
| Bit 5     | FEIEN    | 0x0         | rw   | Falling edge interrupt enable<br/>0: Falling edge interrupt disabled<br/>1: Falling edge interrupt enabled                                                                             |
| Bit 4     | HLIEN    | 0x0         | rw   | High-level interrupt enable                                                                                                                                                            |
|       |       |             |      | 0: High-level interrupt disabled<br/>1: High-level interrupt enabled                                                                                            |
| Bit 3 | REIEN | 0x0         | rw   | Rising edge interrupt enable<br/>0: Rising edge interrupt disabled<br/>1: Rising edge interrupt enabled                                                         |
| Bit 2 | FES   | 0x0         | rw   | Falling edge status<br/>This bit is set by hardware and cleared by software.<br/>0: No falling edge interrupt generated<br/>1: Falling edge interrupt generated |
| Bit 1 | HLS   | 0x0         | rw   | High-level status<br/>This bit is set by hardware and cleared by software.<br/>0: No high level interrupt generated<br/>1: High level interrupt generated       |
| Bit 0 | RES   | 0x0         | rw   | Rising edge status<br/>This bit is set by hardware and cleared by software.<br/>0: No rising edge interrupt generated<br/>1: Rising edge interrupt generated    |


# 24.8.2.3 Regular memory timing register x (XMC_ BKxTMGRG) (x=2,3)

<u>Accessed</u> by words.

| Bit        | Name    | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                      |
| ---------- | ------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 24 | RGDHIZT | 0xFC        | rw   | Regular memory databus High resistance time<br/>This field defines the databus high resistance duration when write access to NAND Flash is started in a regular space.<br/>00000000: 0 HCLK cycle is inserted<br/>00000001: 1 additional HCLK cycle is inserted<br/>......<br/>11111111: 255 additional HCLK cycles are inserted |
| Bit 23: 16 | RGHT    | 0xFC        | rw   | Regular memory hold time<br/>This field defines the databus hold time when access to NAND Flash in a regular memory.<br/>00000000: Reserved<br/>00000001: 1 HCLK cycle is inserted<br/>......<br/>11111111: 255 HCLK cycles are inserted                                                                                         |
| Bit 15: 8  | RGWT    | 0xFC        | rw   | Regular memory wait time<br/>Specifies the regular memory wait time when the XMC\_NWE and XMC\_NOE is low.<br/>00000000: 0 HCLK cycle is inserted<br/>00000001: 1 additional HCLK cycle is inserted<br/>......<br/>11111111: 255 additional HCLK cycles are inserted                                                             |
| Bit 7: 0   | RGST    | 0xFC        | rw   | Regular memory setup time<br/>This field defines the address setup time when access to NAND Flash in a regular memory.<br/>00000000: 0 HCLK cycle is inserted<br/>00000001: 1 additional HCLK cycle is inserted<br/>......<br/>11111111: 255 additional HCLK cycles are inserted                                                 |


# 24.8.2.4 Special memory timing register x (XMC_ BKxTMGSP) (x=2,3)

<u>Accessed</u> by words.

| Bit        | Name    | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                      |
| ---------- | ------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 24 | SPDHIZT | 0xFC        | rw   | Special memory databus High resistance time<br/>This field defines the databus high resistance duration when write access to NAND Flash is started in a special space.<br/>00000000: 0 HCLK cycle is inserted<br/>00000001: 1 additional HCLK cycle is inserted<br/>......<br/>11111111: 255 additional HCLK cycles are inserted |
| Bit 23: 16 | SPHT    | 0xFC        | rw   | Special memory hold time                                                                                                                                                                                                                                                                                                         |
| Bit 15: 8<br/>Bit 7: 0 | SPWT<br/>SPST | 0xFC<br/>0xFC | rw<br/>rw | This field defines the databus hold time when access to NAND Flash in a special memory.00000000: Reserved00000001: 1 HCLK cycle is inserted......11111111: 255 HCLK cycles are inserted<br/>Special memory wait timeSpecifies the special memory wait time when the XMC\_NWE and XMC\_NOE is low\.00000000: 0 HCLK cycle is inserted00000001: 1 additional HCLK cycle is inserted......11111111: 255 additional HCLK cycles are inserted<br/>Special memory setup timeThis field defines the address setup time when access to NAND Flash in a special memory.00000000: 0 HCLK cycle is inserted00000001: 1 additional HCLK cycle is inserted......11111111: 255 additional HCLK cycles are inserted |


## 24.8.2.5 ECC value register x (XMC_ BKxCC) (x=2,3)

Accessed by words.

| Bit<br/>Bit 31: 0 | Name<br/>ECC | Reset value<br/>0x0000 0000 | Type<br/>ro | Description<br/>EECC valueThis field contains the computed ECC value. |
| ----------------- | ------------ | --------------------------- | ----------- | --------------------------------------------------------------------- |


## 24.8.3 PC card controller registers

### 24.8.3.1 PC card control register (XMC_BK4CTRL)

Accessed by words.

| Bit<br/>Bit 31: 3<br/>Bit 2<br/>Bit 1<br/>Bit 0 | Name<br/>Reserved<br/>EN<br/>NWEN<br/>Reserved | Reset value<br/>0x000<br/>0x0<br/>0x0<br/>0x0 | Type<br/>resd<br/>rw<br/>rw<br/>resd | Description<br/>Kept at its default value.<br/>Memory bank enable0: Memory bank disabled1: Memory bank enabled<br/>Wait feature enableThis bit is used to enable PC card memory bank wait feature.0: Wait feature disabled1: Wait feature enabled<br/>Kept at its default value. |
| ----------------------------------------------- | ---------------------------------------------- | --------------------------------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |


### 24.8.3.2 Interrupt enable and FIFO status register 4 (XMC_BK4IS)

Accessed by words.

| Bit | Name | Reset value | Type | Description |
| --------------------------------------------------------- | --------------------------------------------------------- | -------------------------------------------------------- | ----------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 7<br/>Bit 6<br/>Bit 5<br/>Bit 4<br/>Bit 3 | Reserved<br/>FIFOE<br/>FEIEN<br/>HLIEN<br/>REIEN | 0x000000<br/>0x1<br/>0x0<br/>0x0<br/>0x0 | resd<br/>rw<br/>rw<br/>rw<br/>rw |Kept at its default value.<br/>FIFO emptyThis bit is set by hardware when the FIFO is empty.0: FIFO is not empty1: FIFO is emptyXMC FIFO size is 16 words. It is used to store the data from AHB.<br/>Falling edge interrupt enable0: Falling edge interrupt disabled1: Falling edge interrupt enabled<br/>High-level interrupt enable0: High-level interrupt disabled1: High-level interrupt enabled<br/>Rising edge interrupt enable0: Rising edge interrupt disabled1: Rising edge interrupt enabled |
| Bit 2 | FES | 0x0 | rw | Falling edge status<br/>This bit is set by hardware and cleared by software.<br/>0: No falling edge interrupt generated<br/>1: Falling edge interrupt generated |
| Bit 1 | HLS | 0x0 | rw | High-level status<br/>This bit is set by hardware and cleared by software.<br/>0: No high level interrupt generated<br/>1: High level interrupt generated       |
| Bit 0 | RES | 0x0 | rw | Rising edge status<br/>This bit is set by hardware and cleared by software.<br/>0: No rising edge interrupt generated<br/>1: Rising edge interrupt generated    |


## 24.8.3.3 Common memory timing register 4 (XMC_ BK4TMGCM)

<u>Accessed</u> by words.

| Bit        | Name    | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                    |
| ---------- | ------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 24 | CMDHIZT | 0xFC        | rw   | Common memory databus High resistance time<br/>This field defines the databus high resistance duration when write access to NAND Flash is started in a common space.<br/>00000000: 0 HCLK cycle is inserted<br/>00000001: 1 additional HCLK cycle is inserted<br/>......<br/>11111111: 255 additional HCLK cycles are inserted |
| Bit 23: 16 | CMHT    | 0xFC        | rw   | Common memory hold time<br/>This field defines the databus hold time when access to NAND Flash in a common space<br/>00000000: Reserved<br/>00000001: 1 HCLK cycle is inserted<br/>......<br/>11111111: 255 HCLK cycles are inserted                                                                                           |
| Bit 15: 8  | CMWT    | 0xFC        | rw   | Common memory wait time<br/>Specifies the common memory wait time when the XMC\_NWE and XMC\_NOE is low.<br/>00000000: 0 HCLK cycle is inserted<br/>00000001: 1 additional HCLK cycle is inserted<br/>......<br/>11111111: 255 additional HCLK cycles are inserted                                                             |
| Bit 7: 0   | CMST    | 0xFC        | rw   | Common memory setup time<br/>This field defines the address setup time when access to NAND Flash in a regular memory.<br/>00000000: 0 HCLK cycle is inserted<br/>00000001: 1 additional HCLK cycle is inserted<br/>......<br/>11111111: 255 additional HCLK cycles are inserted                                                |


## 24.8.3.4 Attribute memory timing register 4 (XMC_ BK4TMGAT)

<u>Accessed</u> by words.

| Bit        | Name    | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                           |
| ---------- | ------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 24 | ATDHIZT | 0xFC        | rw   | Attribute memory databus High resistance time<br/>This field defines the databus high resistance duration when write access to NAND Flash is started in an attribute space.<br/>00000000: 0 HCLK cycle is inserted<br/>00000001: 1 additional HCLK cycle is inserted<br/>......<br/>11111111: 255 additional HCLK cycles are inserted |
| Bit 23: 16 | ATHT    | 0xFC        | rw   | Attribute memory hold time<br/>This field defines the databus hold time when access to NAND Flash in an attribute space.<br/>00000000: Reserved<br/>00000001: 1 HCLK cycle is inserted<br/>......<br/>11111111: 255 HCLK cycles are inserted                                                                                          |
| Bit 15: 8 | ATWT | 0xFC | rw | Attribute memory wait time<br/>Specifies the attribute memory wait time when the XMC\_NWE and XMC\_NOE is low.<br/>00000000: 0 HCLK cycle is inserted<br/>00000001: 1 additional HCLK cycle is inserted<br/>......<br/>11111111: 255 additional HCLK cycles are inserted             |
| Bit 7: 0  | ATST | 0xFC | rw | Attribute memory setup time<br/>This field defines the address setup time when access to NAND Flash in an attribute space.<br/>00000000: 0 HCLK cycle is inserted<br/>00000001: 1 additional HCLK cycle is inserted<br/>......<br/>11111111: 255 additional HCLK cycles are inserted |


## 24.8.3.5 IO space timing register 4 (XMC_ BK4TMGIO)

<u>Accessed</u> by words.

| Bit        | Name    | Reset value | Type | Description                                                                                                                                                                                                                                                                                                           |
| ---------- | ------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 24 | IODHIZT | 0xFC        | rw   | I/O space databus High resistance time<br/>This field defines the databus high resistance duration when write access to NAND Flash is started in an IO space.<br/>00000000: 0 HCLK cycle is inserted<br/>00000001: 1 additional HCLK cycle is inserted<br/>......<br/>11111111: 255 additional HCLK cycle is inserted |
| Bit 23: 16 | IOHT    | 0xFC        | rw   | I/O space hold time<br/>This field defines the databus hold time when access to NAND Flash in an IO space.<br/>00000000: Reserved<br/>00000001: 1 HCLK cycle is inserted<br/>......<br/>11111111: 255 HCLK cycles are inserted                                                                                        |
| Bit 15: 8  | ATWT    | 0xFC        | rw   | Attribute memory wait time<br/>Specifies the IO wait time when the XMC\_NWE and XMC\_NOE is low.<br/>00000000: 0 HCLK cycle is inserted<br/>00000001: 1 additional HCLK cycle is inserted<br/>......<br/>11111111: 255 additional HCLK cycles are inserted                                                            |
| Bit 7: 0   | IOST    | 0xFC        | rw   | IO space setup time<br/>This field defines the address setup time when access to NAND Flash in an IO space.<br/>00000000: 0 HCLK cycle is inserted<br/>00000001: 1 additional HCLK cycle is inserted<br/>......<br/>11111111: 255 additional HCLK cycles are inserted                                                 |


## 24.8.4 SDRAM controller registers

### 24.8.4.1 SDRAM control register 1, 2 (SDRAM_CTRL1,SDRAM_CTRL2)

This register contains the control parameters for each SDRAM memory bank.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                          |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 15 | Reserved | 0x00000     | resd | Kept at its default value.                                                                                                                                                                                                                                                                                           |
| Bit 14: 13 | RD       | 0x0         | rw   | Read delay<br/>This field defines the delay (in HCLK clock cycles) for reading data after CAS latency.<br/>00: No HCLK clock cycle delay<br/>01: 1 HCLK clock cycle delay<br/>10: 2 HCLK clock cycle delay<br/>11: Reserved, do not use.<br/>Note: The corresponding bits in the CTRL2 register are "don't care bit" |
| Bit 12     | BSTR     | 0x0         | rw   | Burst read                                                                                                                                                                                                                                                                                                           |
|            |        |     |    | When this bit is set, it indicates that single AHB requests (single or burst) are processed as bursts. Several data will be prefetched and stored into the FIFO.<br/>0: Single read requests are not processed as bursts<br/>1: ingle read requests are always processed as bursts<br/>Note: The corresponding bits in the CTRL2 register are “don’t care bit” |
| Bit 11: 10 | CLKDIV | 0x0 | rw | Clock division configuration<br/>SDRAM clock configuration.<br/>00: SDCLK clock disabled<br/>01: HCLK/4<br/>10: HCLK/2<br/>11: HCLK/3<br/>Note: The corresponding bits in the CTRL2 register are “don’t care bit”                                                                                                                                              |
| Bit 9      | WRP    | 0x0 | rw | Write protection<br/>This bit is set to enable the SDRAM write protection.<br/>0: Write access allowed<br/>1: Write access forbidden                                                                                                                                                                                                                           |
| Bit 8: 7   | CAS    | 0x0 | rw | CAS latency<br/>This field is used to select CAS latency.<br/>00: Reserved, do not use.<br/>01: 1 cycle<br/>10: 2 cycles<br/>11: 3 cycles                                                                                                                                                                                                                      |
| Bit 6      | INBK   | 0x0 | rw | Internal banks<br/>This bit is used to define the number of internal banks.<br/>0: 2 internal BANKs<br/>1: 4 internal BANKs                                                                                                                                                                                                                                    |
| Bit 5: 4   | DB     | 0x0 | rw | SDRAM data bus<br/>This field enables 8-bit or 16-bit data bus width.<br/>00: 8 bits<br/>01: 16 bits<br/>10: Reserved, do not use.<br/>11: Reserved, do not use.                                                                                                                                                                                               |
| Bit 3: 2   | RA     | 0x0 | rw | Row address<br/>This field defines the number of a row address, including 11 bits, 12 bits and 13 bits.<br/>00: 11 bits<br/>01: 12 bits<br/>10: 13 bits<br/>11: Reserved, do not use.                                                                                                                                                                          |
| Bit 1: 0   | CA     | 0x0 | rw | Column address<br/>This field defines the number of a column address, including 8 bits, 9 bits, 10 bits and 13 bits.<br/>00: 8 bits<br/>01: 9 bits<br/>10: 10 bits<br/>11: 11 bits                                                                                                                                                                             |


# 24.8.4.2 SDRAM timing register 1, 2 (SDRAM_TM1,SDRAM_TM2)

This register contains the timing parameters of each SDRAM bank.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                        |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 28 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                         |
| Bit 27: 24 | TRCD     | 0xF         | rw   | Row active to Read/Write delay<br/>This field defines the delay between the activate command and a read/write command in number of clock cycles.<br/>0000: 1 cycle<br/>0001: 2 cycles<br/>....<br/>1111: 16 cycles |
| Bit 23:20  | TRP      | 0xF         | rw   | Precharge to active delay<br/>This field defines the delay between a precharge command and another command in number of clock                                                                                      |
|            |      |     |    | cycles.<br/>0000: 1 cycle<br/>0001: 2 cycles<br/>....<br/>1111: 16 cycles                                                                                                                                                                        |
| Bit 19: 16 | TWR  | 0xF | rw | Write Recovery delay<br/>This field defines the delay between a write command and a precharge command in number of clock cycles.<br/>0000: 1 cycle<br/>0001: 2 cycles<br/>....<br/>1111: 16 cycles                                               |
| Bit 15: 12 | TRC  | 0xF | rw | Refresh to active delay<br/>This field defines the delay between the refresh command and the activate command, as well as the delay between two consecutive refresh commands.<br/>0000: 1 cycle<br/>0001: 2 cycles<br/>....<br/>1111: 16 cycles  |
| Bit 11: 8  | TRAS | 0xF | rw | Self refresh time<br/>This field defines the minimum self-refresh period in umber of clock cycles.<br/>0000: 1 cycle<br/>0001: 2 cycles<br/>....<br/>1111: 16 cycles                                                                             |
| Bit 7: 4   | TXSR | 0xF | rw | Exit Self-refresh to active delay<br/>This field defines the delay from exiting the self-refresh command to issuing an activate command in number of clock cycles.<br/>0000: 1 cycle<br/>0001: 2 cycles<br/>....<br/>1111: 16 cycles             |
| Bit 3: 0   | TMRD | 0xF | rw | Load mode register program to active delay<br/>This field defines the delay between a load mode register command and an activate or refresh command in number of clock cycles.<br/>0000: 1 cycle<br/>0001: 2 cycles<br/>....<br/>1111: 16 cycles |


Note: If two SDRAM devices are used, the TRP and TRC timings must be programmed with the timings of the slowest devices.


# 24.8.4.3 SDRAM command register (SDRAM_CMD)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                              |
| ---------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 22 | Reserved | 0x000       | resd | Kept at its default value.                                                                                                                                                                                                                                                               |
| Bit 22: 9  | MRD      | 0x0000      | rw   | Mode Register data<br/>Refer to the SDRAM specifications for details.                                                                                                                                                                                                                    |
| Bit 8: 5   | ART      | 0x0         | rw   | Auto-refresh times<br/>This field defines the number of consecutive auto-refresh commands issued when MODE =“011”.<br/>0000: 1 auto-refresh cycle<br/>0001: 2 auto-refresh cycles<br/>....<br/>1110: 15 auto-refresh cycles<br/>1111: Reserved                                           |
| Bit 4      | BK1      | 0x0         | wo   | SDRAM Bank 1<br/>This bit indicates whether the command will be sent to the SDRAM Bank 1.<br/>0: Command not sent to SDRAM Bank 1<br/>1: Command sent to SDRAM Bank 1                                                                                                                    |
| Bit 3      | BK2      | 0x0         | wo   | SDRAM Bank 2<br/>This bit indicates whether the command will be sent to the SDRAM Bank 2.<br/>0: Command not sent to SDRAM Bank 2<br/>1: Command sent to SDRAM Bank 2                                                                                                                    |
| Bit 2: 0   | CMD      | 0x0         | wo   | SDRAM Command<br/>This field defines the command issued to the SDRAM device.<br/>000: Normal mode<br/>001: Clock configuration enable<br/>010: Precharge all banks<br/>011: Auto refresh<br/>100: Load mode register<br/>101: Self refresh<br/>110: Power-down command<br/>111: Reserved |


# 24.8.4.4 SDRAM refresh timer register (SDRAM_RCNT)

This register is used to set the refresh rate of the SDRAM, in number of SDRAM CLK clock cycles.

The RC has to be configured as a non-zero value in order to perform a correct refresh operation. The RC value cannot be changed after initialization.

Refresh operation has priority over a read/write operation. However, if a read/write operation is in progress while a new refresh request occurs, the refresh operation starts only after the read/write operation is complete. It is recommended that the RC value is smaller than the calculated value in order to ensure the anticipated refresh rate.

This register is shared by the SDRAM Bank 1 and Bank 2.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                        |
| ---------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 15 | Reserved | 0x00000     | resd | Kept at its default value.                                                                                                                                                                                                                                                         |
| Bit 14     | ERIEN    | 0x0         | rw   | Error interrupt enable<br/>0: Error interrupt disabled<br/>1: Error interrupt enabled                                                                                                                                                                                              |
| Bit 13: 1  | RC       | 0x0000      | rw   | Refresh counter<br/>This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of clock cycles. It must be set at least to 41 clock cycles.<br/>Refresh rate = (RC + 1) x SDRAM clock frequency<br/>RC = (SDRAM refresh period/number of rows) - 20 |
| Bit 0      | ERRC     | 0x0         | wo   | Error flag clear<br/>This bit is used to clear the error flag (ER) in the status register.<br/>0: No effect<br/>1: Refresh error flag is cleared.                                                                                                                                  |

Note: The programmed COUNT value must not be equal to the sum of the following timings: TWR+TRP+TRC+TRCD+4 memory clock cycles.

## 24.8.4.5 SDRAM status register (SDRAM_STS)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                     |
| --------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 6 | Reserved | 0x0000000   | resd | Kept at its default value.                                                                                                                      |
| Bit 5     | BUSY     | 0x0         | ro   | Busy<br/>This bit indicates the status of the current SDRAM controller.<br/>0: Idle<br/>1: Busy                                                 |
| Bit 4: 3  | BK2STS   | 0x0         | ro   | Bank2 status<br/>This field defines the status mode of the SDRAM Bank 2.<br/>00: Normal mode<br/>01: Auto-refresh mode<br/>10: Power-down mode  |
| Bit 2: 1  | BK1STS   | 0x0         | ro   | Bank 1 Status<br/>This field defines the status mode of the SDRAM Bank 1.<br/>00: Normal mode<br/>01: Auto-refresh mode<br/>10: Power-down mode |
| Bit 0     | ERR      | 0x0         | ro   | Error flag<br/>0: No refresh error has been detected<br/>1: A refresh error has been detected                                                   |


