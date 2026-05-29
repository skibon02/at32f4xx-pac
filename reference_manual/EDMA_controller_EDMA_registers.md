
Artery logo
AT32F435/437 Series Reference Manual

# 29.5 EDMA registers

Table 29-5 shows DMA register map and reset values.

These peripheral registers must be accessed by bytes (8 bits), half-words (16 bits) or words (32 bits).

Table 29-5 BPR register map and reset values

| Register       | Offset | Reset value |
| -------------- | ------ | ----------- |
| EDMA\_STS1     | 0x00   | 0x0000 0000 |
| EDMA\_STS2     | 0x04   | 0x0000 0000 |
| EDMA\_CLR1     | 0x08   | 0x0000 0000 |
| EDMA\_CLR2     | 0x0c   | 0x0000 0000 |
| EDMA\_S1CTRL   | 0x10   | 0x0000 0000 |
| EDMA\_S1DTCNT  | 0x14   | 0x0000 0000 |
| EDMA\_S1PADDR  | 0x18   | 0x0000 0000 |
| EDMA\_S1M0ADDR | 0x1c   | 0x0000 0000 |
| EDMA\_S1M1ADDR | 0x20   | 0x0000 0000 |
| EDMA\_S1FCTRL  | 0x24   | 0x0000 0000 |
| EDMA\_S2CTRL   | 0x28   | 0x0000 0000 |
| EDMA\_S2DTCNT  | 0x2c   | 0x0000 0000 |
| EDMA\_S2PADDR  | 0x30   | 0x0000 0000 |
| EDMA\_S2M0ADDR | 0x34   | 0x0000 0000 |
| EDMA\_S2M1ADDR | 0x38   | 0x0000 0000 |
| EDMA\_S2FCTRL  | 0x3c   | 0x0000 0000 |
| EDMA\_S3CTRL   | 0x40   | 0x0000 0000 |
| EDMA\_S3DTCNT  | 0x44   | 0x0000 0000 |
| EDMA\_S3PADDR  | 0x48   | 0x0000 0000 |
| EDMA\_S3M0ADDR | 0x4c   | 0x0000 0000 |
| EDMA\_S3M1ADDR | 0x50   | 0x0000 0000 |
| EDMA\_S3FCTRL  | 0x54   | 0x0000 0000 |
| EDMA\_S4CTRL   | 0x58   | 0x0000 0000 |
| EDMA\_S4DTCNT  | 0x5c   | 0x0000 0000 |
| EDMA\_S4PADDR  | 0x60   | 0x0000 0000 |
| EDMA\_S4M0ADDR | 0x64   | 0x0000 0000 |
| EDMA\_S4M1ADDR | 0x68   | 0x0000 0000 |
| EDMA\_S4FCTRL  | 0x6c   | 0x0000 0000 |
| EDMA\_S5CTRL   | 0x70   | 0x0000 0000 |
| EDMA\_S5DTCNT  | 0x74   | 0x0000 0000 |
| EDMA\_S5PADDR  | 0x78   | 0x0000 0000 |
| EDMA\_S5M0ADDR | 0x7c   | 0x0000 0000 |
| EDMA\_S5M1ADDR | 0x80   | 0x0000 0000 |
| EDMA\_S5FCTRL  | 0x84   | 0x0000 0000 |
| EDMA\_S6CTRL   | 0x88   | 0x0000 0000 |
| EDMA\_S6DTCNT  | 0x8c   | 0x0000 0000 |


2025.05.28
Page 699
Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

| EDMA\_S6PADDR  | 0x90  | 0x0000 0000 |
| -------------- | ----- | ----------- |
| EDMA\_S6M0ADDR | 0x94  | 0x0000 0000 |
| EDMA\_S6M1ADDR | 0x98  | 0x0000 0000 |
| EDMA\_S6FCTRL  | 0x9c  | 0x0000 0000 |
| EDMA\_S7CTRL   | 0xa0  | 0x0000 0000 |
| EDMA\_S7DTCNT  | 0xa4  | 0x0000 0000 |
| EDMA\_S7PADDR  | 0xa8  | 0x0000 0000 |
| EDMA\_S7M0ADDR | 0xac  | 0x0000 0000 |
| EDMA\_S7M1ADDR | 0xb0  | 0x0000 0000 |
| EDMA\_S7FCTRL  | 0xb4  | 0x0000 0000 |
| EDMA\_S8CTRL   | 0xb8  | 0x0000 0000 |
| EDMA\_S8DTCNT  | 0xbc  | 0x0000 0000 |
| EDMA\_S8PADDR  | 0xc0  | 0x0000 0000 |
| EDMA\_S8M0ADDR | 0xc4  | 0x0000 0000 |
| EDMA\_S8M1ADDR | 0xc8  | 0x0000 0000 |
| EDMA\_S8FCTRL  | 0xcc  | 0x0000 0000 |
| EDMA\_LLCTRL   | 0xd0  | 0x0000 0000 |
| EDMA\_S1LLP    | 0xd4  | 0x0000 0000 |
| EDMA\_S2LLP    | 0xd8  | 0x0000 0000 |
| EDMA\_S3LLP    | 0xdc  | 0x0000 0000 |
| EDMA\_S4LLP    | 0xe0  | 0x0000 0000 |
| EDMA\_S5LLP    | 0xe4  | 0x0000 0000 |
| EDMA\_S6LLP    | 0xe8  | 0x0000 0000 |
| EDMA\_S7LLP    | 0xec  | 0x0000 0000 |
| EDMA\_S8LLP    | 0xf0  | 0x0000 0000 |
| EDMA\_S2DCTRL  | 0xf4  | 0x0000 0000 |
| EDMA\_S12DCNT  | 0xf8  | 0x0000 0000 |
| EDMA\_S1STRIDE | 0xfc  | 0x0000 0000 |
| EDMA\_S22DCNT  | 0x100 | 0x0000 0000 |
| EDMA\_S2STRIDE | 0x104 | 0x0000 0000 |
| EDMA\_S32DCNT  | 0x108 | 0x0000 0000 |
| EDMA\_S3STRIDE | 0x10c | 0x0000 0000 |
| EDMA\_S42DCNT  | 0x110 | 0x0000 0000 |
| EDMA\_S4STRIDE | 0x114 | 0x0000 0000 |
| EDMA\_S52DCNT  | 0x118 | 0x0000 0000 |
| EDMA\_S5STRIDE | 0x11c | 0x0000 0000 |
| EDMA\_S62DCNT  | 0x120 | 0x0000 0000 |
| EDMA\_S6STRIDE | 0x124 | 0x0000 0000 |
| EDMA\_S72DCNT  | 0x128 | 0x0000 0000 |
| EDMA\_S7STRIDE | 0x12c | 0x0000 0000 |
| EDMA\_S82DCNT  | 0x130 | 0x0000 0000 |


2025.05.28
Page 700
Rev 2.07





Artery logo AT32F435/437 Series Reference Manual

| EDMA\_S8STRIDE   | 0x134 | 0x0000 0000 |
| ---------------- | ----- | ----------- |
| EDMA\_SYNCEN     | 0x138 | 0x0000 0000 |
| EDMA\_MUXSEL     | 0x13c | 0x0000 0000 |
| EDMA\_MUXS1CTRL  | 0x140 | 0x0000 0000 |
| EDMA\_MUXS2TRL   | 0x144 | 0x0000 0000 |
| EDMA\_MUXS3CTRL  | 0x148 | 0x0000 0000 |
| EDMA\_MUXS4CTRL  | 0x14c | 0x0000 0000 |
| EDMA\_MUXS5CTRL  | 0x150 | 0x0000 0000 |
| EDMA\_MUXS6CTRL  | 0x154 | 0x0000 0000 |
| EDMA\_MUXS7CTRL  | 0x158 | 0x0000 0000 |
| EDMA\_MUXS8CTRL  | 0x15c | 0x0000 0000 |
| EDMA\_MUXG1CTRL  | 0x160 | 0x0000 0000 |
| EDMA\_MUXG2CTRL  | 0x164 | 0x0000 0000 |
| EDMA\_MUXG3CTRL  | 0x168 | 0x0000 0000 |
| EDMA\_MUXG4CTRL  | 0x16c | 0x0000 0000 |
| EDMA\_MUXSYNCSTS | 0x170 | 0x0000 0000 |
| EDMA\_MUXSYNCCLR | 0x174 | 0x0000 0000 |
| EDMA\_MUXGSTS    | 0x178 | 0x0000 0000 |
| EDMA\_MUXGCLR    | 0x17c | 0x0000 0000 |


## 29.5.1 DMA status register 1 (DMA_STS1)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit        | Name     | Reset value | Type | Description                                        |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------- |
| Bit 31: 28 | Reserved | 0x0         | resd | Kept at its default value.                         |
| Bit 27     | FDTF4    | 0x0         | ro   | Stream4 full data transfer complete interrupt flag |
| Bit 26     | HDTF4    | 0x0         | ro   | Stream4 half data transfer complete interrupt flag |
| Bit 25     | DTERRF4  | 0x0         | ro   | Stream4 transfer error interrupt flag              |
| Bit 24     | DMERRF4  | 0x0         | ro   | Stream4 direct mode error interrupt flag           |
| Bit 23     | Reserved | 0x0         | ro   | Kept at its default value.                         |
| Bit 22     | FERRF4   | 0x0         | ro   | Stream4 fifo error interrupt flag                  |
| Bit 21     | FDTF3    | 0x0         | ro   | Stream3 full data transfer complete interrupt flag |
| Bit 20     | HDTF3    | 0x0         | ro   | Stream3 half data transfer complete interrupt flag |
| Bit 19     | DTERRF3  | 0x0         | ro   | Stream3 transfer error interrupt flag              |
| Bit 18     | DMERRF3  | 0x0         | ro   | Stream3 direct mode error interrupt flag           |
| Bit 17     | Reserved | 0x0         | ro   | Kept at its default value.                         |
| Bit 16     | FERRF3   | 0x0         | ro   | Stream3 fifo error interrupt flag                  |
| Bit 15: 12 | Reserved | 0x0         | resd | Kept at its default value.                         |


2025.05.28 Page 701 Rev 2.07





Artery logo
AT32F435/437 Series Reference Manual

| Bit 11 | FDTF2    | 0x0 | ro | Stream2 full data transfer complete interrupt flag |
| ------ | -------- | --- | -- | -------------------------------------------------- |
| Bit 10 | HDTF2    | 0x0 | ro | Stream2 half transfer complete interrupt flag      |
| Bit 9  | DTERRF2  | 0x0 | ro | Stream2 transfer error interrupt flag              |
| Bit 8  | DMERRF2  | 0x0 | ro | Stream2 direct mode error interrupt flag           |
| Bit 7  | Reserved | 0x0 | ro | Kept at its default value.                         |
| Bit 6  | FERRF2   | 0x0 | ro | Stream2 fifo error interrupt flag                  |
| Bit 5  | FDTF1    | 0x0 | ro | Stream1 full data transfer complete interrupt flag |
| Bit 4  | HDTF1    | 0x0 | ro | Stream1 half data transfer complete interrupt flag |
| Bit 3  | DTERRF1  | 0x0 | ro | Stream1 transfer error interrupt flag              |
| Bit 2  | DMERRF1  | 0x0 | ro | Stream1 direct mode error interrupt flag           |
| Bit 1  | Reserved | 0x0 | ro | Kept at its default value.                         |
| Bit 0  | FERRF1   | 0x0 | ro | Stream1 fifo error interrupt flag                  |


## 29.5.2 DMA status register 2 (DMA_STS2)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit        | Name     | Reset value | Type | Description                                        |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------- |
| Bit 31: 28 | Reserved | 0x0         | resd | Kept at its default value.                         |
| Bit 27     | FDTF8    | 0x0         | ro   | Stream6 full data transfer complete interrupt flag |
| Bit 26     | HDTF8    | 0x0         | ro   | Stream8 half transfer complete interrupt flag      |
| Bit 25     | DTERRF8  | 0x0         | ro   | Stream8 transfer error interrupt flag              |
| Bit 24     | DMERRF8  | 0x0         | ro   | Stream8 direct mode error interrupt flag           |
| Bit 23     | Reserved | 0x0         | ro   | Kept at its default value.                         |
| Bit 22     | FERRF8   | 0x0         | ro   | Stream8 fifo error interrupt flag                  |
| Bit 21     | FDTF7    | 0x0         | ro   | Stream7 full data transfer complete interrupt flag |
| Bit 20     | HDTF7    | 0x0         | ro   | Stream7 half data transfer complete interrupt flag |
| Bit 19     | DTERRF7  | 0x0         | ro   | Stream7 transfer error interrupt flag              |
| Bit 18     | DMERRF7  | 0x0         | ro   | Stream7 direct mode error interrupt flag           |
| Bit 17     | Reserved | 0x0         | ro   | Kept at its default value.                         |
| Bit 16     | FERRF7   | 0x0         | ro   | Stream7 fifo error interrupt flag                  |
| Bit 15: 12 | Reserved | 0x0         | resd | Kept at its default value.                         |
| Bit 11     | FDTF6    | 0x0         | ro   | Stream6 full data transfer complete interrupt flag |


2025.05.28
Page 702
Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

| Bit 10 | HDTF6    | 0x0 | ro | Stream6 half data transfer complete interrupt flag |
| ------ | -------- | --- | -- | -------------------------------------------------- |
| Bit 9  | DTERRF6  | 0x0 | ro | Stream6 transfer error interrupt flag              |
| Bit 8  | DMERRF6  | 0x0 | ro | Stream6 direct mode error interrupt flag           |
| Bit 7  | Reserved | 0x0 | ro | Kept at its default value.                         |
| Bit 6  | FERRF6   | 0x0 | ro | Stream6 fifo error interrupt flag                  |
| Bit 5  | FDTF5    | 0x0 | ro | Stream5 full data transfer complete interrupt flag |
| Bit 4  | HDTF5    | 0x0 | ro | Stream5 half data transfer complete interrupt flag |
| Bit 3  | DTERRF5  | 0x0 | ro | Stream5 transfer error interrupt flag              |
| Bit 2  | DMERRF5  | 0x0 | ro | Stream5 direct mode error interrupt flag           |
| Bit 1  | Reserved | 0x0 | ro | Kept at its default value.                         |
| Bit 0  | FERRF5   | 0x0 | ro | Stream5 fifo error interrupt flag                  |


# 29.5.3 DMA flag clear register 1 (DMA_CLR1)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit        | Name     | Reset value | Type | Description                                         |
| ---------- | -------- | ----------- | ---- | --------------------------------------------------- |
| Bit 31: 28 | Reserved | 0x0         | resd | Kept at its default value.                          |
| Bit 27     | FDTFC4   | 0x0         | w    | Stream4 clear transfer complete interrupt flag      |
| Bit 26     | HDTFC4   | 0x0         | w    | Stream4 clear half transfer complete interrupt flag |
| Bit 25     | DTERRFC4 | 0x0         | w    | Stream4 clear error interrupt flag                  |
| Bit 24     | DMERRFC4 | 0x0         | w    | Steam4 clear direct mode error interrupt flag       |
| Bit 23     | Reserved | 0x0         | resd | Kept at its default value.                          |
| Bit 22     | FERRFC4  | 0x0         | w    | Stream4 clear fifo error interrupt flag             |
| Bit 21     | FDTFC3   | 0x0         | w    | Stream3 clear transfer complete interrupt flag      |
| Bit 20     | HDTFC3   | 0x0         | w    | Stream3 clear half transfer complete interrupt flag |
| Bit 19     | DTERRFC3 | 0x0         | w    | Stream3 clear error interrupt flag                  |
| Bit 18     | DMERRFC3 | 0x0         | w    | Steam3 clear direct mode error interrupt flag       |
| Bit 17     | Reserved | 0x0         | resd | Kept at its default value.                          |
| Bit 16     | FERRFC3  | 0x0         | w    | Stream3 clear fifo error interrupt flag             |
| Bit 15: 12 | FDTFC2   | 0x0         | w    | Stream2 clear transfer complete interrupt flag      |
| Bit 11     | HDTFC2   | 0x0         | w    | Stream2 clear half transfer complete interrupt flag |
| Bit 10     | DTERRFC2 | 0x0         | w    | Stream2 clear error interrupt flag                  |
| Bit 9      | DMERRFC2 | 0x0         | w    | Steam2 clear direct mode error interrupt flag       |
| Bit 8      | Reserved | 0x0         | resd | Kept at its default value.                          |


2025.05.28
Page 703
Rev 2.07





ARTERY logo AT32F435/437 Series Reference Manual

| Bit 7 | FERRFC2  | 0x0 | w    | Stream2 clear fifo error interrupt flag             |
| ----- | -------- | --- | ---- | --------------------------------------------------- |
| Bit 6 | Reserved | 0x0 | resd | Kept at its default value.                          |
| Bit 5 | FDTFC1   | 0x0 | w    | Stream1 clear transfer complete interrupt flag      |
| Bit 4 | HDTFC1   | 0x0 | w    | Stream1 clear half transfer complete interrupt flag |
| Bit 3 | DTERRFC1 | 0x0 | w    | Stream1 clear error interrupt flag                  |
| Bit 2 | DMERRFC1 | 0x0 | w    | Steam1 clear direct mode error interrupt flag       |
| Bit 1 | Reserved | 0x0 | resd | Kept at its default value.                          |
| Bit 0 | FERRFC1  | 0x0 | w    | Stream1 clear fifo error interrupt flag             |


# 29.5.4 DMA flag clear register 2 (DMA_CLR2)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit        | Name     | Reset value | Type | Description                                         |
| ---------- | -------- | ----------- | ---- | --------------------------------------------------- |
| Bit 31: 28 | Reserved | 0x0         | resd | Kept at its default value.                          |
| Bit 27     | FDTFC8   | 0x0         | w    | Stream8 clear transfer complete interrupt flag      |
| Bit 26     | HDTFC8   | 0x0         | w    | Stream8 clear half transfer complete interrupt flag |
| Bit 25     | DTERRFC8 | 0x0         | w    | Stream8 clear error interrupt flag                  |
| Bit 24     | DMERRFC8 | 0x0         | w    | Steam8 clear direct mode error interrupt flag       |
| Bit 23     | Reserved | 0x0         | resd | Kept at its default value.                          |
| Bit 22     | FERRFC8  | 0x0         | w    | Stream8 clear fifo error interrupt flag             |
| Bit 21     | FDTFC7   | 0x0         | w    | Stream7 clear transfer complete interrupt flag      |
| Bit 20     | HDTFC7   | 0x0         | w    | Stream7 clear half transfer complete interrupt flag |
| Bit 19     | DTERRFC7 | 0x0         | w    | Stream7 clear error interrupt flag                  |
| Bit 18     | DMERRFC7 | 0x0         | w    | Steam7 clear direct mode error interrupt flag       |
| Bit 17     | Reserved | 0x0         | resd | Kept at its default value.                          |
| Bit 16     | FERRFC7  | 0x0         | w    | Stream7 clear fifo error interrupt flag             |
| Bit 15: 12 | FDTFC6   | 0x0         | w    | Stream6 clear transfer complete interrupt flag      |
| Bit 11     | HDTFC6   | 0x0         | w    | Stream6 clear half transfer complete interrupt flag |
| Bit 10     | DTERRFC6 | 0x0         | w    | Stream6 clear error interrupt flag                  |
| Bit 9      | DMERRFC6 | 0x0         | w    | Steam6 clear direct mode error interrupt flag       |
| Bit 8      | Reserved | 0x0         | resd | Kept at its default value.                          |
| Bit 7      | FERRFC6  | 0x0         | w    | Stream6 clear fifo error interrupt flag             |
| Bit 6      | Reserved | 0x0         | resd | Kept at its default value.                          |
| Bit 5      | FDTFC5   | 0x0         | w    | Stream5 clear transfer complete interrupt flag      |


2025.05.28 Page 704 Rev 2.07





Artery logo AT32F435/437 Series Reference Manual

| Bit 4 | HDTFC5   | 0x0 | w    | Stream5 clear half transfer complete interrupt flag |
| ----- | -------- | --- | ---- | --------------------------------------------------- |
| Bit 3 | DTERRFC5 | 0x0 | w    | Stream5 clear error interrupt flag                  |
| Bit 2 | DMERRFC5 | 0x0 | w    | Steam5 clear direct mode error interrupt flag       |
| Bit 1 | Reserved | 0x0 | resd | Kept at its default value.                          |
| Bit 0 | FERRFC5  | 0x0 | w    | Stream5 clear fifo error interrupt flag             |


# 29.5.5 DMA stream-x control register (DMA_SxCTRL) (x= 1...8)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                       |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 28 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                        |
| Bit 27: 25 | CHSEL    | 0x0         | rw   | channel select<br/>000: Channel 1 selected<br/>This field can be written only when SEN=0.                                                                                                                                                                                                                                                                                                                         |
| Bit 24: 23 | MBURST   | 0x0         | rw   | Memory burst transfer configuration<br/>This field defines the type of memory burst transfer.<br/>00: Single transfer<br/>01: INCR4 (4-beat increment burst)<br/>10: INCR8 (beat increment burst)<br/>11: INCR16 (16-beat increment burst)<br/>This field is forced to 00 when SEN=1.<br/>Note: This field can be written only when SEN=0.                                                                        |
| Bit 22: 21 | PBURST   | 0x0         | rw   | Peripheral burst transfer configuration<br/>This field defines the type of peripheral transfers.<br/>00: Single transfer<br/>01: INCR4 (4-beat increment burst)<br/>10: INCR8 (beat increment burst)<br/>11: INCR16 (16-beat increment burst)<br/>This field is forced to 00 when SEN=1.<br/>Note: This field can be written only when SEN=0.                                                                     |
| Bit 20     | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                        |
| Bit 19     | CM       | 0x0         | rw   | Current memory<br/>This bit is valid in dual buffer mode only.<br/>0: Current target is memory 0 (addressed by the DMA\_SxM0ADDR)<br/>1: Current target is memory 1 (addressed by the DMA\_SxM1ADDR)<br/>This bit can be written only when SEN=0 to indicate the target memory area of the first transfer.<br/>Once SEN=1, this bit is used as a status flag, indicating which is the current target memory area. |
| Bit 18     | DMM      | 0x0         | rw   | Double memory mode<br/>0: Double memory mode disabled<br/>1: Double memory mode enabled<br/>This bit can be written only when SEN=0.                                                                                                                                                                                                                                                                              |
| Bit 17: 16 | SPL      | 0x0         | rw   | Stream polarity<br/>00: Low<br/>01: Medium<br/>10: High<br/>11: Very high<br/>This field can be written only when SEN=0.                                                                                                                                                                                                                                                                                          |


2025.05.28 Page 705 Rev 2.07





Artery logo
# AT32F435/437 Series Reference Manual

| Bit 15     | PINCOS | 00x | rw | Peripheral increase offset<br/>0: Peripheral increase offset disabled<br/>1: Peripheral increase offset enabled<br/>If PINCOS bit is enabled, the offset size of the peripheral address calculation is fixed to 32-bit alignment. This bit has no meaning if PINCM=0.<br/>This bit is forced low by hardware when the direction mode is selected or if PBURST is different from 00.<br/>This bit can be written only when SEN=0. |
| ---------- | ------ | --- | -- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 14: 13 | MWIDTH | 0x0 | rw | Memory data width<br/>This field defines the HSIZE (AHB bus signal) in the memory controller.<br/>00: Byte (8-bit)<br/>01: Half-word (16-bit)<br/>10: Word (32-bit)<br/>11: Reserved<br/>In direct mode, this field is forced by hardware to be same value as PWIDTH when SEN=1.<br/>This field can be written only when SEN=0.                                                                                                  |
| Bit 12: 11 | PWIDTH | 00x | rw | peripheral data width<br/>This field defines the HSIZE (AHB bus signal) in the peripheral controller.<br/>00: Byte (8-bit)<br/>01: Half-word (16-bit)<br/>10: Word (32-bit)<br/>11: Reserved<br/>This field can be written only when SEN=0.                                                                                                                                                                                      |
| Bit 10     | MINCM  | 0x0 | rw | Memory increment mode<br/>0: Memory address pointer is fixed<br/>1: Memory address pointer is incremented<br/>This bit can be written only when SEN=0.                                                                                                                                                                                                                                                                           |
| Bit 9      | PINCM  | 0x0 | rw | peripheral increment mode<br/>0: Peripheral address pointer is fixed<br/>1: Peripheral address pointer is incremented<br/>This bit can be written only when SEN=0.                                                                                                                                                                                                                                                               |
| Bit 8      | LM     | 0x0 | rw | Loop mode<br/>0: Loop mode is disabled<br/>1: Loop mode is enabled<br/>This bit is forced flow by hardware if PFCTRL=1, as soon as SEN=1.<br/>This bit is forced high by hardware if DMM=1, as soon as SEN=1.                                                                                                                                                                                                                    |
| Bit 7: 6   | DTD    | 0x0 | rw | data transfer direction<br/>00: Peripheral to memory (P2M)<br/>01: Memory to peripheral (M2P)<br/>10: Memory to memory (M2M)<br/>11: Reserved<br/>This bit can be written only when SEN=0.                                                                                                                                                                                                                                       |
| Bit 5      | PFCTRL | 0x0 | rw | Peripheral flow controller<br/>0: DMA is the flow controller<br/>1: The peripheral is the flow controller<br/>This bit is forced low by hardware if DTD=10 (M2M).<br/>This bit can be written only when SEN=0.                                                                                                                                                                                                                   |
| Bit 4      | FDTIEN | 0x0 | rw | Full data transfer interrupt enable<br/>0: TC interrupt disabled<br/>1: TC interrupt enabled                                                                                                                                                                                                                                                                                                                                     |
| Bit 3      | HDTIEN | 0x0 | rw | Half data transfer interrupt enable<br/>0: HT interrupt disabled<br/>1: HT interrupt enabled                                                                                                                                                                                                                                                                                                                                     |


2025.05.28
Page 706
Rev 2.07




ARTERY logo # AT32F435/437 Series Reference Manual

| Bit 2 | DTERRIEN | 0x0 | rw | Data transfer error interrupt enable<br/>0: TE interrupt disabled<br/>1: TE interrupt enabled                                                                                                                                                                                                                                                                                                                                                             |
| ----- | -------- | --- | -- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 1 | DMERRIEN | 0x0 | rw | Direct mode error interrupt enable<br/>0: DME interrupt disabled<br/>1: DME interrupt enabled                                                                                                                                                                                                                                                                                                                                                             |
| Bit 0 | SEN      | 0x0 | rw | Stream enable<br/>0: Stream disabled<br/>1: Stream enabled<br/>This bit may be cleared by hardware under the following conditions:<br/>-DMA end of transfer<br/>-If a transfer error occurs on the AHB master buses<br/>-When the FIFO threshold on memory AHB port is not compatible with the burst size.<br/>Note: Before setting SEN=1 to start a new transfer, the event flags corresponding to the stream in DMA\_STS1 or DMA\_STS2 must be cleared. |


## 29.5.6 DMA stream-x data register (DMA_SxDTCNT) (x= 1...8)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Bit15: 0   | CNT      | 0x0000      | rw   | Number of data to be transferred 0\~65535<br/>This register can be written only when SEN=0.<br/>This register is read-only when SEN=1, indicating the remaining data to be transmitted. This register is decremented after each transfer.<br/>Typically, this register remains 0 after the completion of data transfer. But this register may be automatically reloaded with the previously programmed value in the following cases:<br/>·When the stream is configured to be loop mode<br/>·When the stream is enabled again by setting SEN=1.<br/>If this register is zero, no transaction can be served even if SEN=1 |


## 29.5.7 DMA stream-x peripheral address register (DMA_SxPADDR) (x= 1...8)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name  | Reset value | Type | Description                                                                                                 |
| --------- | ----- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | PADDR | 0x0000 0000 | rw   | Peripheral address<br/>Base address of the peripheral address.<br/>This field can be written only if SEN=0. |


2025.05.28 Page 707 Rev 2.07





Artery logo AT32F435/437 Series Reference Manual

### 29.5.8 DMA stream-x memory 0 address register (DMA_SxM0ADDR) (x= 1...8)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name   | Reset value | Type | Description                                                                                                                                                                               |
| --------- | ------ | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | M0ADDR | 0x0000 0000 | rw   | Memory 0 address<br/>Base address of memory area 0.<br/>This field can be written only if:<br/>·The stream is disabled (SEN =0)<br/>·The stream is enabled in dual buffer mode, and CM=1. |


\*The stream is enabled in dual buffer mode, and CM=1.

### 29.5.9 DMA stream-x memory 1 address register (DMA_SxM1ADDR) (x= 1...8)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name   | Reset value | Type | Description                                                                                                                                                                               |
| --------- | ------ | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | M1ADDR | 0x0000 0000 | rw   | Memory 1 address<br/>Base address of memory area 1.<br/>This field can be written only if:<br/>·The stream is disabled (SEN =0)<br/>·The stream is enabled in dual buffer mode, and CM=1. |


\*The stream is enabled in dual buffer mode, and CM=1.

### 29.5.10 DMA stream-x FIFO control register (DMA_SxFCTRL) (x= 1...8)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 8 | Reserved | 0x0000 00   | resd | Kept at its default value.                                                                                                                                                                                                                                 |
| Bit 7     | FERRIEN  | 0x0         | rw   | FIFO error interrupt enable<br/>0: FIFO error interrupt disabled<br/>1: FIFO error interrupt enabled                                                                                                                                                       |
| Bit 6     | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                 |
| Bit 5: 3  | FSTS     | 0x4         | rw   | FIFO status<br/>000: 0 \<fifo\_level <1/4<br/>001: 1/4≤fifo\_level <1/2<br/>010: 1/2≤fifo\_level <3/4<br/>011: 3/4≤fifo\_level < full<br/>100: FIFO empty<br/>101: FIFO full<br/>Others: No meaning<br/>These bits are not relevant in direct mode (FEN=0) |
| Bit 2     | FEN      | 0x0         | rw   | FIFO mode enable<br/>0: Direct mode<br/>1: FIFO mode<br/>This bit is set by hardware if memory-to-memory mode is selected (DTD=10) and SEN=1.                                                                                                              |
| Bit 1: 0  | FTHSEL   | 0x1         | rw   | FIFO threshold select<br/>00: 1/4 full FIFO<br/>01: 1/2 full FIFO<br/>10: 3/4 full FIFO<br/>11: full FIFO<br/>These bits are not used in direct mode (FEN=0).<br/>These bits cannot be written only if SEN=0.                                              |


2025.05.28 Page 708 Rev 2.07





Artery logo AT32F435/437 Series Reference Manual

# 29.5.11 DMA linked table control register (DMA_SxLLCTRL)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name     | Reset value | Type | Description                |
| --------- | -------- | ----------- | ---- | -------------------------- |
| Bit 31: 8 | Reserved | 0x0         | resd | Kept at its default value. |
| Bit 7     | S8LLEN   | 0x0         | rw   | Stream8 link list enable   |
| Bit 6     | S7LLEN   | 0x0         | rw   | Stream7 link list enable   |
| Bit 5     | S6LLEN   | 0x0         | rw   | Stream6 link list enable   |
| Bit 4     | S5LLEN   | 0x0         | rw   | Stream5 link list enable   |
| Bit 3     | S4LLEN   | 0x0         | rw   | Stream4 link list enable   |
| Bit 2     | S3LLEN   | 0x0         | rw   | Stream3 link list enable   |
| Bit 1     | S2LLEN   | 0x0         | rw   | Stream2 link list enable   |
| Bit 0     | S1LLEN   | 0x0         | rw   | Stream1 link list enable   |


# 29.5.12 DMA linked table pointer register (DMA_SxLLP) (x = 1...8)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name | Reset value | Type | Description                                                                                                                                             |
| --------- | ---- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | LLP  | 0x0000 0000 | rw   | link list pointer<br/>After the completion of the current descriptor, the flow controller uses this pointer as a target address to read the descriptor. |


# 29.5.13 DMA 2D transfer control register (DMA_S2DCTRL)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name     | Reset value | Type | Description                |
| --------- | -------- | ----------- | ---- | -------------------------- |
| Bit 31: 8 | Reserved | 0x0         | resd | Kept at its default value. |
| Bit 7     | S82DEN   | 0x0         | rw   | Stream8 2D enable          |
| Bit 6     | S72DEN   | 0x0         | rw   | Stream7 2D enable          |
| Bit 5     | S62DEN   | 0x0         | rw   | Stream6 2D enable          |
| Bit 4     | S52DEN   | 0x0         | rw   | Stream5 2D enable          |
| Bit 3     | S42DEN   | 0x0         | rw   | Stream4 2D enable          |
| Bit 2     | S32DEN   | 0x0         | rw   | Stream3 2D enable          |
| Bit 1     | S22DEN   | 0x0         | rw   | Stream2 2D enable          |
| Bit 0     | S12DEN   | 0x0         | rw   | Stream1 2D enable          |


# 29.5.14 DMA 2D transfer count register (DMA_S2DCNT)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit        | Name   | Reset value | Type | Description  |
| ---------- | ------ | ----------- | ---- | ------------ |
| Bit 31: 16 | YCOUNT | 0x0000      | rw   | C-axis count |
| Bit 15: 0  | XCOUNT | 0x0000      | rw   | X-axis count |


2025.05.28 Page 709 Rev 2.07




ARTERY logo

# AT32F435/437 Series Reference Manual

# 29.5.15 DMA 2D transfer stride register (DMA_STRIDE)( x = 1...8)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit        | Name   | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                       |        |        |    |                                                                                                                                                                                                                                                                                                                                                                                                              |
| ---------- | ------ | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ------ | -- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 16 | DSTSTD | 0x0000      | rw   | Destination stride<br/>This is a signed value (two’s complement).<br/>0x0000: 0<br/>...<br/>0x7FFF:32767<br/>0x8000:-32768<br/>0x8001:-32767<br/>...<br/>0xFFFF:-1<br/>The DST value is in terms of byte address. For example, if the destination stride is 0x8, the destination byte address is added with 0x8 before the next iteration.<br/>DSTSTD bit 1..0 must be 00, for the size of the destination position remains word. |        |        |    |                                                                                                                                                                                                                                                                                                                                                                                                              |
|            |        |             |      | Bit 15: 0                                                                                                                                                                                                                                                                                                                                                                                                                         | SRCSTD | 0x0000 | rw | Source stride<br/>This is a signed value (two’s complement).<br/>0x0000: 0<br/>...<br/>0x7FFF:32767<br/>0x8000:-32768<br/>0x8001:-32767<br/>...<br/>0xFFFF:-1<br/>The SRC value is in terms of byte address. For example, if the source stride is 0x8, the source byte address is added with 0x8 before the next iteration.<br/>SRCSTDbit 1..0 must be 00, for the size of the source position remains word. |


# 29.5.16 DMA synchronization enable (DMA_SYNCEN)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name     | Reset value | Type | Description                    |
| --------- | -------- | ----------- | ---- | ------------------------------ |
| Bit 31: 8 | Reserved | 0x0         | resd | Kept at its default value.     |
| Bit 7     | S8SYNC   | 0x0         | rw   | Stream8 synchronization enable |
| Bit 6     | S7SYNC   | 0x0         | rw   | Stream7 synchronization enable |
| Bit 5     | S6SYNC   | 0x0         | rw   | Stream6 synchronization enable |
| Bit 4     | S5SYNC   | 0x0         | rw   | Stream5 synchronization enable |
| Bit 3     | S4SYNC   | 0x0         | rw   | Stream4 synchronization enable |
| Bit 2     | S3SYNC   | 0x0         | rw   | Stream3 synchronization enable |
| Bit 1     | S2SYNC   | 0x0         | rw   | Stream2 synchronization enable |
| Bit 0     | S1SYNC   | 0x0         | rw   | Stream1 synchronization enable |


2025.05.28
Page 710
Rev 2.07





Artery logo AT32F435/437 Series Reference Manual

# 29.5.17 DMAMUX table select (DMA_MUXSEL)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name     | Reset value | Type | Description                                              |
| --------- | -------- | ----------- | ---- | -------------------------------------------------------- |
| Bit 31: 1 | Reserved | 0x0         | resd | Kept at its default value.                               |
| Bit 0     | TBL\_SEL | 0x0         | rw   | Multiplexer table select<br/>0x1: Flexible mapping table |


# 29.5.18 DMAMUX channel-x control register (DMA_MUXSxCTRL)
(x = 1...8)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit        | Name      | Reset value | Type | Description                                                                                                                                                                                              |
| ---------- | --------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 25 | Reserved  | 0x0         | resd | Kept at its default value.                                                                                                                                                                               |
| Bit 28: 24 | SYNCSEL   | 0x0         | rw   | Synchronization select                                                                                                                                                                                   |
| Bit 23: 19 | REQCNT    | 0x0         | rw   | DMA request count<br/>This field defines the number of DMA requests after synchronization events and/or before synchronization events.<br/>This field are reserved only if SYNCEN and EVTGEN both are 0. |
| Bit 18: 17 | SYNCPOL   | 0x0         | rw   | Synchronization polarity<br/>Defines the polarity of the selected synchronization input.<br/>0x0: No event<br/>0x1: Rising edge<br/>0x2: Falling edge<br/>0x3: Rising and falling edge                   |
| Bit 16     | SYNCEN    | 0x0         | rw   | Synchronization enable<br/>0: Synchronization disabled<br/>1: Synchronization enabled                                                                                                                    |
| Bit 15: 10 | Reserved  | 0x00        | resd | Kept at its default value.                                                                                                                                                                               |
| Bit 9      | EVTGEN    | 0x0         | rw   | Event generation enable<br/>0: Event generation disabled<br/>1: Event generation enabled                                                                                                                 |
| Bit 8      | SYNCOVIEN | 0x0         | rw   | Synchronization overrun interrupt enable<br/>0: Interrupt disabled<br/>1: Interrupt enabled                                                                                                              |
| Bit 7      | Reserved  | 0x0         | resd | Kept at its default value.                                                                                                                                                                               |
| Bit 6: 0   | REQSEL    | 0x00        | rw   | DMA request select<br/>Select a DMA request Refer to DMAMUX table for more information.                                                                                                                  |


2025.05.28 Page 711 Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

# 29.5.19 DMAMUX generator-x control register (DMA_MUXGxCTRL) (x = 1...8)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                            |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 24 | Reserved | 0x00        | resd | Kept at its default value.                                                                                                                                                             |
| Bit 23: 19 | GREQCNT  | 0x00        | rw   | DMA request generation count<br/>Defines the number of DMA requests (GREQCNT + 1) after an event occurs.<br/>This field is reserved only when GEN is disabled.                         |
| Bit 18: 17 | GPOL     | 0x0         | rw   | DMA request generation polarity<br/>Defines the polarity of the selected trigger inputs.<br/>0x0: No event<br/>0x1: Rising edge<br/>0x2: Falling edge<br/>0x3: Rising and falling edge |
| Bit 16     | GEN      | 0x0         | rw   | DMA request generation enable<br/>0: DMA request generation disabled<br/>1: DMA request generation enabled                                                                             |
| Bit 15: 9  | Reserved | 0x00        | resd | Kept at its default value.                                                                                                                                                             |
| Bit 8      | TRGOVIEN | 0x0         | rw   | Trigger overrun interrupt enable<br/>0: Interrupt disabled<br/>1: Interrupt enabled                                                                                                    |
| Bit 7: 5   | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                             |
| Bit 4: 0   | SIGSEL   | 0x00        | rw   | Signal select<br/>This field is used to select DMA signal for DMA request generation.                                                                                                  |


# 29.5.20 DMAMUX synchronization interrupt status register (DMA_MUXSYNCSTS)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name     | Reset value | Type | Description                                                                                                                                         |
| --------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 8 | Reserved | 0x0000 000  | resd | Kept at its default value.                                                                                                                          |
| Bit 7: 0  | SYNCOVF  | 0x00        | ro   | Synchronization overrun interrupt flag<br/>This bit is set when a new synchronization event occurs while the DMA request count is below the REQCNT. |


# 29.5.21 DMAMUX synchronization interrupt clear flag register (BPR_MUXSYNCCLR)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 8 | Reserved | 0x0000 000  | resd | Kept at its default value.                                                                                                                                 |
| Bit 7: 0  | SYNCOVFC | 0x00        | rw1c | Synchronization overrun interrupt flag clear<br/>Writing 1 to the corresponding bit clears the corresponding overrun flag SYNCOVF in DMA\_MUXCSR register. |


2025.05.28
Page 712
Rev 2.07




ARTERY logo **AT32F435/437 Series Reference Manual**

# 29.5.22 DMAMUX generator interrupt status register (DMA_MUXGSTS)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name     | Reset value | Type | Description                                                                                                                         |
| --------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 8 | Reserved | 0x0000 000  | resd | Kept at its default value.                                                                                                          |
| Bit 7: 0  | TRGOVF   | 0x00        | ro   | Trigger overrun interrupt flag<br/>This bit is set when a new trigger event occurs while the DMA request count is below the GNBREQ. |


# 29.5.23 DMAMUX generator interrupt clear flag register (DMA_MUXGCLR)

Access: 0 wait state, accessible by bytes, half-words or words.

| Bit       | Name     | Reset value | Type | Description                                                                                                                        |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 8 | Reserved | 0x0000 000  | resd | Kept at its default value.                                                                                                         |
| Bit 7: 0  | TRGOVFC  | 0x00        | rw1c | Trigger overrun interrupt flag clear<br/>Writing 1 to the corresponding bit clears the overrun flag TRGOVF in DMA\_MUXGS register. |


2025.05.28
Page 713
Rev 2.07
