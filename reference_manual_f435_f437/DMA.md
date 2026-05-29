

Figure 9-8 DMAMUX event generation

Timing diagram showing DMAMUX event generation with signals CLK, Selected all_req

SYNCEN = 0, EVTGEN = 1, REQCNT = 2

# 9.5 DMA registers

Table 9-5 shows DMA register map and their reset values.

These peripheral registers must be accessed by bytes (8 bits), half-words (16 bits) or words (32 bits).

<u>Table 9-5 DMA register map and reset value</u>

| Register     | Offset | Reset value |
| ------------ | ------ | ----------- |
| DMA\_STS     | 0x00   | 0x0000 0000 |
| DMA\_CLR     | 0x04   | 0x0000 0000 |
| DMA\_C1CTRL  | 0x08   | 0x0000 0000 |
| DMA\_C1DTCNT | 0x0c   | 0x0000 0000 |
| DMA\_C1PADDR | 0x10   | 0x0000 0000 |
| DMA\_C1MADDR | 0x14   | 0x0000 0000 |
| DMA\_C2CTRL  | 0x1c   | 0x0000 0000 |
| DMA\_C2DTCNT | 0x20   | 0x0000 0000 |
| DMA\_C2PADDR | 0x24   | 0x0000 0000 |
| DMA\_C2MADDR | 0x28   | 0x0000 0000 |
| DMA\_C3CTRL  | 0x30   | 0x0000 0000 |
| DMA\_C3DTCNT | 0x34   | 0x0000 0000 |
| DMA\_C3PADDR | 0x38   | 0x0000 0000 |
| DMA\_C3MADDR | 0x3c   | 0x0000 0000 |
| DMA\_C4CTRL  | 0x44   | 0x0000 0000 |
| DMA\_C4DTCNT | 0x48   | 0x0000 0000 |
| DMA\_C4PADDR | 0x4c   | 0x0000 0000 |
| DMA\_C4MADDR | 0x50   | 0x0000 0000 |
| DMA\_C5CTRL  | 0x58   | 0x0000 0000 |
| DMA\_C5DTCNT | 0x5c   | 0x0000 0000 |
| DMA\_C5PADDR | 0x60   | 0x0000 0000 |
| DMA\_C5MADDR | 0x64   | 0x0000 0000 |
| DMA\_C6CTRL  | 0x6c   | 0x0000 0000 |
| DMA\_C6DTCNT    | 0x70  | 0x0000 0000 |
| DMA\_C6PADDR    | 0x74  | 0x0000 0000 |
| DMA\_C6MADDR    | 0x78  | 0x0000 0000 |
| DMA\_C7CTRL     | 0x80  | 0x0000 0000 |
| DMA\_C7DTCNT    | 0x84  | 0x0000 0000 |
| DMA\_C7PADDR    | 0x88  | 0x0000 0000 |
| DMA\_C7MADDR    | 0x8c  | 0x0000 0000 |
| DMA\_MUXSEL     | 0x100 | 0x0000 0000 |
| DMA\_MUXC1CTRL  | 0x104 | 0x0000 0000 |
| DMA\_MUXC2CTRL  | 0x108 | 0x0000 0000 |
| DMA\_MUXC3CTRL  | 0x10c | 0x0000 0000 |
| DMA\_MUXC4CTRL  | 0x110 | 0x0000 0000 |
| DMA\_MUXC5CTRL  | 0x114 | 0x0000 0000 |
| DMA\_MUXC6CTRL  | 0x118 | 0x0000 0000 |
| DMA\_MUXC7CTRL  | 0x11c | 0x0000 0000 |
| DMA\_MUXG1CTRL  | 0x120 | 0x0000 0000 |
| DMA\_MUXG2CTRL  | 0x124 | 0x0000 0000 |
| DMA\_MUXG3CTRL  | 0x128 | 0x0000 0000 |
| DMA\_MUXG4CTRL  | 0x12c | 0x0000 0000 |
| DMA\_MUXSYNCSTS | 0x130 | 0x0000 0000 |
| DMA\_MUXSYNCCLR | 0x134 | 0x0000 0000 |
| DMA\_MUXGSTS    | 0x138 | 0x0000 0000 |
| DMA\_MUXGCLR    | 0x13c | 0x0000 0000 |


# 9.5.1 DMA interrupt status register (DMA_STS)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit    | Name     | Reset value | Type | Description                                                                                                                                                                       |
| ------ | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 31: 28 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                        |
| Bit 27 | DTERRF7  | 0x0         | ro   | Channel 7 data transfer error event flag<br/>0: No transfer error occurred.<br/>1: Transfer error occurred.                                                                       |
| Bit 26 | HDTF7    | 0x0         | ro   | Channel7 half transfer event flag<br/>0: No half-transfer event occurred.<br/>1: Half-transfer event occurred.                                                                    |
| Bit 25 | FDTF7    | 0x0         | ro   | Channel 7 transfer complete event flag<br/>0: No transfer complete event occurred.<br/>1: Transfer complete event occurred.                                                       |
| Bit 24 | GF7      | 0x0         | ro   | Channel7 global event flag<br/>0: No transfer error, half transfer or transfer complete event occurred.<br/>1: Transfer error, half transfer or transfer complete event occurred. |
| Bit 23 | DTERRF6  | 0x0         | ro   | Channel 6 data transfer error event flag<br/>0: No transfer error occurred.<br/>1: Transfer error occurred.                                                                       |
| Bit 22 | HDTF6   | 0x0 | ro | Channel 6 half transfer event flag<br/>0: No half-transfer event occurred.<br/>1: Half-transfer event occurred.                                                          |
| Bit 21 | FDTF6   | 0x0 | ro | Channel 6 transfer complete event flag<br/>0: No transfer complete event occurred.<br/>1: Transfer complete event occurred.                                              |
| Bit 20 | GF6     | 0x0 | ro | Channel 6 global event flag<br/>0: No transfer error, half transfer or transfer complete event occurred.<br/>1: Transfer error, half transfer or transfer complete event |
| Bit 19 | DTERRF5 | 0x0 | ro | Channel 5 data transfer error event flag<br/>0: No transfer error occurred.<br/>1: Transfer error occurred.                                                              |
| Bit 18 | HDTF5   | 0x0 | ro | Channel 5 half transfer event flag<br/>0: No half-transfer event occurred.<br/>1: Half-transfer event occurred.                                                          |
| Bit 17 | FDTF5   | 0x0 | ro | Channel 5 transfer complete event flag<br/>0: No transfer complete event occurred.<br/>1: Transfer complete event occurred.                                              |
| Bit 16 | GF5     | 0x0 | ro | Channel 5 global event flag<br/>0: No transfer error, half transfer or transfer complete event occurred.<br/>1: Transfer error, half transfer or transfer complete event |
| Bit 15 | DTERRF4 | 0x0 | ro | Channel 4 data transfer error event flag<br/>0: No transfer error occurred.<br/>1: Transfer error occurred.                                                              |
| Bit 14 | HDTF4   | 0x0 | ro | Channel 4 half transfer event flag<br/>0: No half-transfer event occurred.<br/>1: Half-transfer event occurred.                                                          |
| Bit 13 | FDTF4   | 0x0 | ro | Channel 4 transfer complete event flag<br/>0: No transfer complete event occurred.<br/>1: Transfer complete event occurred.                                              |
| Bit 12 | GF4     | 0x0 | ro | Channel 4 global event flag<br/>0: No transfer error, half transfer or transfer complete event occurred.<br/>1: Transfer error, half transfer or transfer complete event |
| Bit 11 | DTERRF3 | 0x0 | ro | Channel 3 data transfer error event flag<br/>0: No transfer error occurred.<br/>1: Transfer error occurred.                                                              |
| Bit 10 | HDTF3   | 0x0 | ro | Channel 3 half transfer event flag<br/>0: No half-transfer event occurred.<br/>1: Half-transfer event occurred.                                                          |
| Bit 9  | FDTF3   | 0x0 | ro | Channel 3 transfer complete event flag<br/>0: No transfer complete event occurred.<br/>1: Transfer complete event occurred.                                              |
| Bit 8  | GF3     | 0x0 | ro | Channel 3 global event flag<br/>0: No transfer error, half transfer or transfer complete event occurred.<br/>1: Transfer error, half transfer or transfer complete event |
| Bit 7  | DTERRF2 | 0x0 | ro | Channel 2 data transfer error event flag<br/>0: No transfer error occurred.<br/>1: Transfer error occurred.                                                              |
| Bit 6  | HDTF2   | 0x0 | ro | Channel 2 half transfer event flag<br/>0: No half-transfer event occurred.<br/>1: Half-transfer event occurred.                                                          |
| Bit 5 | FDTF2   | 0x0 | ro | Channel 2 transfer complete event flag<br/>0: No transfer complete event occurred.<br/>1: Transfer complete event occurred.                                              |
| Bit 4 | GF2     | 0x0 | ro | Channel 2 global event flag<br/>0: No transfer error, half transfer or transfer complete event occurred.<br/>1: Transfer error, half transfer or transfer complete event |
| Bit 3 | DTERRF1 | 0x0 | ro | Channel 1 data transfer error event flag<br/>0: No transfer error occurred.<br/>1: Transfer error occurred.                                                              |
| Bit 2 | HDTF1   | 0x0 | ro | Channel 1 half transfer event flag<br/>0: No half-transfer event occurred.<br/>1: Half-transfer event occurred.                                                          |
| Bit 1 | FDTF1   | 0x0 | ro | Channel 1 transfer complete event flag<br/>0: No transfer complete event occurred.<br/>1: Transfer complete event occurred.                                              |
| Bit 0 | GF1     | 0x0 | ro | Channel 1 global event flag<br/>0: No transfer error, half transfer or transfer complete event occurred.<br/>1: Transfer error, half transfer or transfer complete event |


## 9.5.2 DMA interrupt flag clear register (DMA_CLR)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit    | Name     | Reset value | Type | Description                                                                                                                         |
| ------ | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------- |
| 31: 28 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                          |
| Bit 27 | DTERRFC7 | 0x0         | rw1c | Channel 7 data transfer error flag clear<br/>0: No effect<br/>1: Clear the DTERRF flag in the DMA\_STS register                     |
| Bit 26 | HDTFC7   | 0x0         | rw1c | Channel 7 half transfer flag clear<br/>0: No effect<br/>1: Clear the HDTF7 flag in the DMA\_STS register                            |
| Bit 25 | FDTFC7   | 0x0         | rw1c | Channel 7 transfer complete flag clear<br/>0: No effect<br/>1: Clear the FDTF7 flag in the DMA\_STS register                        |
| Bit 24 | GFC7     | 0x0         | rw1c | Channel 7 global interrupt flag clear<br/>0: No effect<br/>1: Clear the DTERRF7, HDTF7, FDTF7 and GF7 flag in the DMA\_STS register |
| Bit 23 | DTERRFC6 | 0x0         | rw1c | Channel 6 data transfer error flag clear<br/>0: No effect<br/>1: Clear the DTERRF6 flag in the DMA\_STS register                    |
| Bit 22 | HDTFC6   | 0x0         | rw1c | Channel 6 half transfer flag clear<br/>0: No effect<br/>1: Clear the HDTF6 flag in the DMA\_STS register                            |
| Bit 21 | FDTFC6   | 0x0         | rw1c | Channel 6 transfer complete flag clear<br/>0: No effect<br/>1: Clear the FDTF6 flag in the DMA\_STS register                        |
| Bit 20 | GFC6     | 0x0         | rw1c | Channel 6 global interrupt flag clear<br/>0: No effect<br/>1: Clear the DTERRF6, HDTF6, FDTF6 and GF6 flag in the DMA\_STS register |
| Bit 19 | DTERRFC5 | 0x0         | rw1c | Channel 5 data transfer error flag clear<br/>0: No effect<br/>1: Clear the DTERRF5 flag in the DMA\_STS register                    |
| Bit 18 | HDTFC5   | 0x0 | rw1c | Channel 5 half transfer flag clear<br/>0: No effect<br/>1: Clear the HDTF5 flag in the DMA\_STS register                            |
| Bit 17 | FDTFC5   | 0x0 | rw1c | Channel 5 transfer complete flag clear<br/>0: No effect<br/>1: Clear the FDTF5 flag in the DMA\_STS register                        |
| Bit 16 | GFC5     | 0x0 | rw1c | Channel 5 global interrupt flag clear<br/>0: No effect<br/>1: Clear the DTERRF5, HDTF5, FDTF5 and GF5 in the DMA\_STS register      |
| Bit 15 | DTERRFC4 | 0x0 | rw1c | Channel 4 data transfer error flag clear<br/>0: No effect<br/>1: Clear the DTERRF4 flag in the DMA\_STS register                    |
| Bit 14 | HDTFC4   | 0x0 | rw1c | Channel 4 half transfer flag clear<br/>0: No effect<br/>1: Clear the HDTF4 flag in the DMA\_STS register                            |
| Bit 13 | FDTFC4   | 0x0 | rw1c | Channel 4 transfer complete flag clear<br/>0: No effect<br/>1: Clear the FDTF4 flag in the DMA\_STS register                        |
| Bit 12 | GFC4     | 0x0 | rw1c | Channel 4 global interrupt flag clear<br/>0: No effect<br/>1: Clear the DTERRF4, HDTF4, FDTF4 and GF4 flag in the DMA\_STS register |
| Bit 11 | DTERRFC3 | 0x0 | rw1c | Channel 3 data transfer error flag clear<br/>0: No effect<br/>1: Clear the DTERRF3 flag in the DMA\_STS register                    |
| Bit 10 | HDTFC3   | 0x0 | rw1c | Channel 3 half transfer flag clear<br/>0: No effect<br/>1: Clear the HDTF3 flag in the DMA\_STS register                            |
| Bit 9  | FDTFC3   | 0x0 | rw1c | Channel 3 transfer complete flag clear<br/>0: No effect<br/>1: Clear the FDTF3 flag in the DMA\_STS register                        |
| Bit 8  | GFC3     | 0x0 | rw1c | Channel 3 global interrupt flag clear<br/>0: No effect<br/>1: Clear the DTERRF3, HDTF3, FDTF3 and GF3 flag in the DMA\_STS register |
| Bit 7  | DTERRFC2 | 0x0 | rw1c | Channel 2 data transfer error flag clear<br/>0: No effect<br/>1: Clear the DTERRF2 flag in the DMA\_STS register                    |
| Bit 6  | HDTFC2   | 0x0 | rw1c | Channel 2 half transfer flag clear<br/>0: No effect<br/>1: Clear the HDTF2 flag in the DMA\_STS register                            |
| Bit 5  | FDTFC2   | 0x0 | rw1c | Channel 2 transfer complete flag clear<br/>0: No effect<br/>1: Clear the FDTF2 flag in the DMA\_STS register                        |
| Bit 4  | GFC2     | 0x0 | rw1c | Channel 2 global interrupt flag clear<br/>0: No effect<br/>1: Clear the DTERRF2, HDTF2, FDTF2 and GF2 in the DMA\_STS register      |
| Bit 3  | DTERRFC1 | 0x0 | rw1c | Channel 1 data transfer error flag clear<br/>0: No effect<br/>1: Clear the DTERRF1 flag in the DMA\_STS register                    |
| Bit 2  | HDTFC1   | 0x0 | rw1c | Channel 1 half transfer flag clear<br/>0: No effect<br/>1: Clear the HDTF1 flag in the DMA\_STS register                            |
| Bit 1  | FDTFC1   | 0x0 | rw1c | Channel 1 transfer complete flag clear<br/>0: No effect<br/>1: Clear the FDTF1 flag in the DMA\_STS register                        |


**Bit 0**: **GFC1** (Reset value: 0x0, Type: rw1c)
Channel 1 global interrupt flag clear
0: No effect
1: Clear the DTERRF1, HDTF1, FDTF1 and GF1 in the DMA_STS register

# 9.5.3 DMA channel-x configuration register (DMA_CxCTRL)

(x = 1...7)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit        | Name     | Reset value | Type | Description                                                                              |
| ---------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------- |
| Bit 31: 15 | Reserved | 0x00000     | resd | Kept at its default value.                                                               |
| Bit 14     | M2M      | 0x0         | rw   | Memory to memory mode<br/>0: Disabled<br/>1: Enabled.                                    |
| Bit 13: 12 | CHPL     | 0x0         | rw   | Channel priority level<br/>00: Low<br/>01: Medium<br/>10: High<br/>11: Very high         |
| Bit 11: 10 | MWIDTH   | 0x0         | rw   | Memory data bit width<br/>00: 8 bits<br/>01: 16 bits<br/>10: 32 bits<br/>11: Reserved    |
| Bit 9: 8   | PWIDTH   | 0x0         | rw   | Peripheral data bit width<br/>00: 8 bits<br/>01: 16 bits<br/>10: 32 bits<br/>1: Reserved |
| Bit 7      | MINCM    | 0x0         | rw   | Memory address increment mode<br/>0: Disabled<br/>1: Enabled.                            |
| Bit 6      | PINCM    | 0x0         | rw   | Peripheral address increment mode<br/>0: Disabled<br/>1: Enabled.                        |
| Bit 5      | LM       | 0x0         | rw   | Circular mode<br/>0: Disabled<br/>1: Enabled.                                            |
| Bit 4      | DTD      | 0x0         | rw   | Data transfer direction<br/>0: Read from peripherals<br/>1: Read from memory             |
| Bit 3      | DTERRIEN | 0x0         | rw   | Data transfer error interrupt enable<br/>0: Disabled<br/>1: Enabled.                     |
| Bit 2      | HDTIEN   | 0x0         | rw   | Half-transfer interrupt enable<br/>0: Disabled<br/>1: Enabled.                           |
| Bit 1      | FDTIEN   | 0x0         | rw   | Transfer complete interrupt enable<br/>0: Disabled<br/>1: Enabled.                       |
| Bit 0      | CHEN     | 0x0         | rw   | Channel enable<br/>0: Disabled<br/>1: Enabled.                                           |


# 9.5.4 DMA channel-x number of data register (DMA_CxDTCNT) (x = 1...7)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                           |
| ---------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                            |
| Bit 15: 0  | CNT      | 0x0000      | rw   | Number of data to transfer<br/>The number of data to transfer is from 0x0 to 0xFFFF. This register can only written when the CHEN bit in the corresponding channel is set 0. The value is decremented after each DMA transfer.<br/>Note: This register holds the number of data to transfer, instead of transfer size. The transfer size is calculated by data width. |


# 9.5.5 DMA channel-x peripheral address register (DMA_CxPADDR) (x = 1...7)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name  | Reset value | Type | Description                                                                                                                                                                                                             |
| --------- | ----- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | PADDR | 0x0000 0000 | rw   | Peripheral base address<br/>Base address of peripheral data register is the source or destination of data transfer.<br/>Note: The register can only be written when the CHEN bit in the corresponding channel is set 0. |


# 9.5.6 DMA channel-x memory address register (DMA_CxMADDR) (x = 1...7)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name  | Reset value | Type | Description                                                                                                                                                                               |
| --------- | ----- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | MADDR | 0x0000 0000 | rw   | Memory base address<br/>Memory address is the source or destination of data transfer.<br/>Note: The register can only be written when the CHEN bit in the corresponding channel is set 0. |


# 9.5.7 DMAMUX selection register (DMA_MUXSEL)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name     | Reset value | Type | Description                                              |
| --------- | -------- | ----------- | ---- | -------------------------------------------------------- |
| Bit 31: 0 | Reserved | 0x0000 0000 | resd | Kept at its default value.                               |
| Bit 0     | TBL\_SEL | 0x0         | rw   | Multiplexer table select<br/>0x1: Flexible mapping table |


# 9.5.8 DMAMUX channel-x control register (DMA_MUXCxCTRL) (x = 1...7)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit        | Name     | Reset value | Type | Description                |
| ---------- | -------- | ----------- | ---- | -------------------------- |
| Bit 31: 25 | Reserved | 0x00        | resd | Kept at its default value. |
| Bit 28: 24 | SYNCSEL  | 0x00        | rw   | Synchronization select     |
| Bit 23: 19 | REQCNT    | 0x00 | rw   | DMA request count<br/>These bits indicate the number of DMA requests sent to the DMA controller after synchronization is enabled, and/or DMA request count before event output is generated.<br/>These bits are reserved only when both SYNCEN and EVTGEN bits are low. |
| Bit 18: 17 | SYNCPOL   | 0x0  | rw   | Synchronization polarity<br/>This field defines the polarity of the selected synchronization input.<br/>0x0: No events<br/>0x1: Rising edge<br/>0x2: Falling edge<br/>0x3: Rising edge and falling edge                                                                 |
| Bit 16     | SYNCEN    | 0x0  | rw   | Synchronization enable<br/>0: Synchronization disabled<br/>1: Synchronization enabled                                                                                                                                                                                   |
| Bit 15: 10 | Reserved  | 0x00 | resd | Kept at its default value.                                                                                                                                                                                                                                              |
| Bit 9      | EVTGEN    | 0x0  | rw   | Event generation enable<br/>0: Event generation is disabled<br/>1: Event generation is enabled                                                                                                                                                                          |
| Bit 8      | SYNCOVIEN | 0x0  | rw   | Synchronization overrun interrupt enable<br/>0: Interrupt disabled<br/>1: Interrupt enabled                                                                                                                                                                             |
| Bit 7      | Reserved  | 0x0  | resd | Kept at its default value.                                                                                                                                                                                                                                              |
| Bit 6: 0   | REQSEL    | 0x00 | rw   | DMA request select<br/>Select DMA request. Refer to DMAMUX table for more information.                                                                                                                                                                                  |


## 9.5.9 DMAMUX generator x control register (DMA_MUXGxCTRL) (x = 1...4)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                           |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 24 | Reserved | 0x00        | resd | Kept at its default value.                                                                                                                                                                            |
| Bit 23: 19 | GREQCNT  | 0x00        | rw   | DMA request generation count<br/>These bits define the number of DMA requests (GNBREQ + 1) to be generated when a trigger event occurs.<br/>This field is reserved only when the GEN bit is disabled. |
| Bit 18: 17 | GPOL     | 0x0         | rw   | DMA request generation polarity<br/>This field defines the polarity of the selected trigger input.<br/>0x0: No events<br/>0x1: Rising edge<br/>0x2: Falling edge<br/>0x3: Rising and falling edges    |
| Bit 16     | GEN      | 0x0         | rw   | DMA request generation enable<br/>0: DMA request generation is disabled<br/>1: DMA request generation is enabled                                                                                      |
| Bit 15: 9 | Reserved | 0x00 | resd | Kept at its default value.                                                                       |
| Bit 8     | TRGOVIEN | 0x0  | rw   | Trigger overrun interrupt enable<br/>0: Interrupt disabled<br/>1: Interrupt enabled              |
| Bit 7: 5  | Reserved | 0x0  | resd | Kept at its default value.                                                                       |
| Bit 4: 0  | SIGSEL   | 0x00 | rw   | Signal select<br/>This field is used to select the DMA trigger input for DMA request generation. |


## 9.5.10 DMAMUX channel synchronization status register (DMA_MUXSYNCSTS)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name     | Reset value | Type | Description                                                                                                                                          |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 8 | Reserved | 0x0000 00   | resd | Kept at its default value.                                                                                                                           |
| Bit 7: 0  | SYNCOVF  | 0x00        | ro   | Synchronization overrun interrupt flag<br/>When the DMA request count is less than REQCNT, this bit is set while a new synchronization event occurs. |


## 9.5.11 DMAMUX channel interrupt clear flag register (DMA_MUXSYNCCLR)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name     | Reset value | Type | Description                                                                                                                                |
| --------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 8 | Reserved | 0x0000 00   | resd | Kept at its default value.                                                                                                                 |
| Bit 7: 0  | SYNCOVFC | 0x00        | rw1c | Synchronization overrun interrupt flag clear<br/>Writing 1 to the corresponding bit can clear the SYNCOVF flag in the MUXSYNCSTS register. |


## 9.5.12 DMAMUX generator interrupt status register (DMA_MUXGSTS)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name     | Reset value | Type | Description                                                                                                                              |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 4 | Reserved | 0x0000 000  | resd | Kept at its default value.                                                                                                               |
| Bit 3: 0  | TRGOVF   | 0x00        | ro   | Trigger overrun interrupt flag<br/>When the DMA request count is lower than GREQCNT, this field is set while a new trigger event occurs. |


# 9.5.13 DMAMUX generator interrupt flag clear register (DMA_MUXGCLR)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit      | Register | Reset value | Type | Description                                                                                                                         |
| -------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31:4 | reserved | 0x0000 000  | resd | Kept at its default value.                                                                                                          |
| Bit 3: 0 | TRGOVFC  | 0x00        | rw1c | Trigger overrun interrupt flag clear<br/>Writing 1 to the corresponding bit can clear the TRGOVF flag in the DMA\_MUXGSTS register. |


