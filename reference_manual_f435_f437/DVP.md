

Figure 27-18 YUV422 format to Y8 (Y-only) format

**PCDC = 1, PCDS = 0**
For YUV422 (YUYV) to Y8
* Input: Y0 | U0 | Y1 | V0 | Y2 | U1 | Y3 | V1 | ... | ...
* Output: Y0 | Y1 | Y2 | Y3 | ... | ...

**PCDC = 1, PCDS = 1**
For YUV422 (UYVY) to Y8
* Input: U0 | Y0 | V0 | Y1 | U1 | Y2 | V1 | Y3 | ... | ...
* Output: Y0 | Y1 | Y2 | Y3 | ... | ...

## 27.8 Registers

Table 27-5 shows the DVP register map and its reset values.

The peripheral registers can be accessed by words (32-bit).

Table 27-5 DVP register map and reset values

| Register   | Offset | Reset value |
| ---------- | ------ | ----------- |
| DVP\_CTRL  | 0x000  | 0x0000 0000 |
| DVP\_STS   | 0x004  | 0x0000 0000 |
| DVP\_ESTS  | 0x008  | 0x0000 0000 |
| DVP\_IENA  | 0x00C  | 0x0000 0000 |
| DVP\_ISTS  | 0x010  | 0x0000 0000 |
| DVP\_ICLR  | 0x014  | 0x0000 0000 |
| DVP\_SCR   | 0x018  | 0x0000 0000 |
| DVP\_SUR   | 0x01C  | 0x0000 0000 |
| DVP\_CWST  | 0x020  | 0x0000 0000 |
| DVP\_CWSZ  | 0x024  | 0x0000 0000 |
| DVP\_DT    | 0x028  | 0x0000 0000 |
| DVP\_ACTRL | 0x040  | 0x0000 0000 |
| DVP\_HSCF  | 0x048  | 0x0000 0000 |
| DVP\_VSCF  | 0x04C  | 0x0000 0000 |
| DVP\_FRF   | 0x050  | 0x0000 0000 |
| DVP\_BTH   | 0x054  | 0x0000 0000 |


### 27.8.1 DVP control register (DVP_CTRL)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                               |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 21 | Reserved | 0x000       | resd | Kept at its default value.                                                                                                                                                                |
| Bit 20     | LCDS     | 0x0         | rw   | Basic line capture/drop selection<br/>0: Capture the first line and drop the next<br/>1: Drop the first line and capture the next<br/>This register is valid when the LCDC=1 is asserted. |
| Bit 19     | LCDC     | 0x0         | rw   | Basic line capture/drop control<br/>0: All frames are captured or use enhanced image scaling resize feature<br/>1: Enable capture/drop control to capture one out of two                  |
|            |          |     |      | lines                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Bit 18     | PCDS     | 0x0 | rw   | Basic pixel capture/drop selection<br/>0: Capture the first group of data (one or two pixel data) and drop the next group<br/>1: Drop the first group of data (one or two pixel data) and capture the next group<br/>This register is valid when the PCDC=1/2/3 is asserted.                                                                                                                                                                                                 |
| Bit 17: 16 | PCDC     | 0x0 | rw   | Basic pixel capture/drop control<br/>0: All frames are captured or use enhanced image scaling resize feature<br/>1: Enable capture/drop control to capture one in two pixel data<br/>2: Enable capture/drop control to capture one in four pixel data<br/>3: Enable capture/drop control to capture two consecutive data in four pixel data                                                                                                                                  |
| Bit 15     | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Bit 14     | ENA      | 0x0 | rw   | DVP Enable<br/>0: DVP disabled<br/>1: DVP enabled<br/>The DVP register configuration must be completed before enabling DVP.                                                                                                                                                                                                                                                                                                                                                  |
| Bit 13: 12 | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Bit 11: 10 | PDL      | 0x0 | rw   | Pixel data length<br/>0: Interface captures 8-bit data<br/>1: Interface captures 10-bit data<br/>2: Interface captures 12-bit data<br/>3: Interface captures 14-bit data                                                                                                                                                                                                                                                                                                     |
| Bit 9: 8   | BFRC     | 0x0 | rw   | Basic frame rate control<br/>0: All frames are captured or use enhanced frame rate control feature<br/>1: Enable frame rate control, every alternate frame captured<br/>2: Enable frame rate control, one frame in 4 frames captured<br/>3: Reserved<br/>This feature is valid only when CFM=0 is asserted.                                                                                                                                                                  |
| Bit 7      | VSP      | 0x0 | rw   | DVP\_VSYNC polarity<br/>This bit defines the vertical synchronization signals.<br/>Frame start:<br/>0: DVP\_VSYNC low level indicates a Frame start signal<br/>1: DVP\_VSYNC high level indicates a Frame start signal<br/>Frame effective:<br/>0: DVP\_VSYNC low level indicates that the captured data is in vertical blanking<br/>1: DVP\_VSYNC high level indicates that the captured data is in vertical blanking<br/>This feature is valid only when SM=0 is asserted. |
| Bit 6      | HSP      | 0x0 | rw   | DVP\_HSYNC polarity<br/>0: DVP\_HSYNC high level indicates that the captured data is a valid pixel data, and Line start signal on the rising edge<br/>1: DVP\_HSYNC low level indicates that the captured data is a valid pixel data, and Line start signal on the falling edge<br/>This feature is valid only when SM=0 is asserted.                                                                                                                                        |
| Bit 5      | CKP      | 0x0 | rw   | DVP\_PCLK polarity<br/>0: DVP\_PCLK rising edge active<br/>1: DVP\_PCLK falling edge active                                                                                                                                                                                                                                                                                                                                                                                  |
| Bit 4      | SM       | 0x0 | rw   | Synchronization mode<br/>0: Hardware synchronization mode<br/>1: Embedded synchronization mode                                                                                                                                                                                                                                                                                                                                                                               |
| Bit 3      | JPEG     | 0x0 | rw   | JPEG format<br/>0: Uncompressed video format<br/>1: Compressed video format                                                                                                                                                                                                                                                                                                                                                                                                  |
| Bit 2<br/>Bit 1<br/>Bit 0 | CRP<br/>CFM<br/>CAP | 0x0<br/>0x0<br/>0x0 | rw<br/>rw<br/>rw | This feature is valid only when SM=0 is asserted.<br/>Cropping window function enable0: Cropping window function disabled1: Cropping window function enabled<br/>Capture function mode0: Continuous capture mode1: Single frame capture mode<br/>Capture function enable0: Capture function disabled1: Capture function enabledThe DMA controller and DVP register configurations must be programmed before enabling this bit.When CFM=1, after this bit is set, this register is automatically reset after the completion of a single frame capture.When CFM=0, after this bit is set, this register remains in set statue. After this bit is cleared by software, this register is automatically reset after the completion of the current frame capture. |


## 27.8.2 DVP status register (DVP_STS)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                    |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 3 | Reserved | 0x0000 0000 | resd | Kept at its default value.                                                                                                                                                                     |
| Bit 2     | OFNE     | 0x0         | ro   | Output data FIFO status<br/>0: FIFO empty<br/>1: FIFO has valid data                                                                                                                           |
| Bit 1     | VSYN     | 0x0         | ro   | Vertical synchronization status<br/>0: Vertical synchronization is not in blanking state<br/>1: Vertical synchronization is in blanking state<br/>This bit is valid when the CAP is set.       |
| Bit 0     | HSYN     | 0x0         | ro   | Horizontal synchronization status<br/>0: Horizontal synchronization is not in blanking state<br/>1: Horizontal synchronization is in blanking state<br/>This bit is valid when the CAP is set. |


## 27.8.3 DVP event status register (DVP_ESTS)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                                               |
| --------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 5 | Reserved | 0x0000 000  | resd | Kept at its default value.                                                                                                                                                                                                                                |
| Bit 4     | HSES     | 0x0         | ro   | Horizontal synchronization event status<br/>0: No horizontal synchronization status detected<br/>1: Horizontal synchronization status detected<br/>It is cleared by writing 1 to the HSIC bit in the DVP\_ICLR register.                                  |
| Bit 3     | VSES     | 0x0         | ro   | Vertical synchronization event status<br/>0: No vertical synchronization status detected<br/>1: Vertical synchronization status detected<br/>It is cleared by writing 1 to the VSIC bit in the DVP\_ICLR register.                                        |
| Bit 2     | ESEES    | 0x0         | ro   | Embedded synchronization error event status<br/>0: Embedded synchronization normal<br/>1: Embedded synchronization error<br/>It is cleared by writing 1 to the ESEIC bit in the DVP\_ICLR register.<br/>This feature is valid only when SM=1 is asserted. |
| Bit 1     | OVRES    | 0x0         | ro   | Output data FIFO overrun event status<br/>0: No data FIFO overrun event detected<br/>1: Data FIFO overrun event detected<br/>It is cleared by writing 1 to the OVRIC bit in the DVP\_ICLR register.                                                       |
| Bit 0     | CFDES    | 0x0         | ro   | Capture frame done raw event status<br/>0: A frame has not been captured<br/>1: A frame has been captured                                                                                                                                                 |


It is cleared by writing 1 to the CFDIC bit in the DVP_ICLR register.

## 27.8.4 DVP interrupt enable register (DVP_IENA)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                      |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 5 | Reserved | 0x0000000   | resd | Kept at its default value.                                                                                                                                       |
| Bit 4     | HSIE     | 0x0         | rw   | Horizontal synchronization interrupt enable<br/>0: Horizontal synchronization interrupt disabled<br/>1: Horizontal synchronization interrupt enabled             |
| Bit 3     | VSIE     | 0x0         | rw   | Vertical synchronization interrupt enable<br/>0: Vertical synchronization interrupt disabled<br/>1: Vertical synchronization interrupt enabled                   |
| Bit 2     | ESEIE    | 0x0         | rw   | Embedded synchronization error interrupt enable<br/>0: Embedded synchronization error interrupt disabled<br/>1: Embedded synchronization error interrupt enabled |
| Bit 1     | OVRIE    | 0x0         | rw   | Output data FIFO overrun interrupt enable<br/>0: Output data FIFO overrun interrupt disabled<br/>1: Output data FIFO overrun interrupt enabled                   |
| Bit 0     | CFDIE    | 0x0         | rw   | Capture frame done interrupt enable<br/>0: Capture frame done interrupt disabled<br/>1: Capture frame done interrupt enabled                                     |


## 27.8.5 DVP interrupt status register (DVP_ISTS)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                             |
| --------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 5 | Reserved | 0x0000 000  | resd | Kept at its default value.                                                                                                                                                                                                                                                                              |
| Bit 4     | HSIS     | 0x0         | ro   | Horizontal synchronization interrupt status<br/>0:No horizontal synchronization interrupt generated<br/>1: Horizontal synchronization interrupt generated<br/>It is cleared by writing 1 to the HSIC bit in the DVP\_ICLR register.                                                                     |
| Bit 3     | VSIS     | 0x0         | ro   | Vertical synchronization interrupt status<br/>0: No vertical synchronization interrupt generated<br/>1: Vertical synchronization interrupt generated<br/>It is cleared by writing 1 to the VSIC bit in the DVP\_ICLR register.                                                                          |
| Bit 2     | ESEIS    | 0x0         | ro   | Embedded synchronization error interrupt status<br/>0: No embedded synchronization error interrupt generated<br/>1: Embedded synchronization error interrupt generated<br/>It is cleared by writing 1 to the ESEIC bit in the DVP\_ICLR register.<br/>This feature is valid only when SM=1 is asserted. |
| Bit 1     | OVRIS    | 0x0         | ro   | Output data FIFO overrun interrupt status<br/>0: No FIFO overrun interrupt generated<br/>1: FIFO overrun interrupt generated<br/>It is cleared by writing 1 to the OVRIC bit in the DVP\_ICLR register.                                                                                                 |
| Bit 0     | CFDIS    | 0x0         | ro   | Capture frame done interrupt status<br/>0: A frame has not been captured<br/>1: Capture frame done interrupt generated<br/>It is cleared by writing 1 to the CFDIC bit in the DVP\_ICLR register.                                                                                                       |


## 27.8.6 DVP interrupt clear register (DVP_ICLR)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                            |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 5 | Reserved | 0x0000000   | resd | Kept at its default value.                                                                                                                                             |
| Bit 4     | HSIC     | 0x0         | wo   | Horizontal synchronization interrupt clear<br/>Writing 1 to this bit clears the HSES bit in the DVP\_ESTS register, and clears the HSIS bit in the DVP\_ISTS register. |
| Bit 3     | VSIC     | 0x0         | wo   | Vertical synchronization interrupt clear<br/>Writing 1 to this bit clears the VSES bit in the DVP\_ESTS register, and clears the VSIS bit in the DVP\_ISTS register.   |
| Bit 2     | ESEIC    | 0x0         | wo   | Embedded synchronization error interrupt clear                                                                                                                         |
|       |       |             |      | Writing 1 to this bit clears the ESEES bit in the DVP\_ESTS register, and clears the ESEIS bit in the DVP\_ISTS register.                                              |
| Bit 1 | OVRIC | 0x0         | wo   | Output data FIFO overrun interrupt clear<br/>Writing 1 to this bit clears the OVRES bit in the DVP\_ESTS register, and clears the OVRIS bit in the DVP\_ISTS register. |
| Bit 0 | CFDIC | 0x0         | wo   | Capture frame done interrupt clear<br/>Writing 1 to this bit clears the CFDES bit in the DVP\_ESTS register, and clears the CFDIS bit in the DVP\_ISTS register.       |


## 27.8.7 DVP embedded synchronization code register (DVP_SCR)

| Bit        | Name | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ---------- | ---- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 24 | FMEC | 0x00        | rw   | Frame end synchronization 4th code<br/>Frame end delimiter code consists of four consecutive data:<br/>All 1, all 0, all 0, FE4<br/>Frame end delimiter 4th code (FE4) is composed of FMEC.<br/>In embedded synchronization mode, if the FMEC is programmed to 0xFF, the decoder performs embedded synchronization for any frame end delimiter.<br/>All the undefined bytes other than FMSC, LNSC and LNEC are interpreted as frame end delimiter 4th code FE4 |
| Bit 23: 16 | LNEC | 0x00        | rw   | Line end synchronization 4th code<br/>The code consists of 4 consecutive data:<br/>All 1, all 0, all 0 and LE4<br/>Line end delimiter 4th data (LE4) is composed of LNEC.                                                                                                                                                                                                                                                                                      |
| Bit 15: 8  | LNSC | 0x00        | rw   | Line start synchronization 4th code<br/>The code consists of 4 consecutive data:<br/>All 1, all 0, all 0 and LS4<br/>Line start delimiter 4th data (LS4) is composed of LNSC.                                                                                                                                                                                                                                                                                  |
| Bit 7: 0   | FMSC | 0x00        | rw   | Frame start synchronization 4th code<br/>In embedded synchronization mode, if the FMSC and FMSU both are programmed to 0xFF, the decoder performs embedded synchronization for non-frame-start synchronization code.<br/>The first occurrence of line start synchronization code after a Frame end code will be interpreted as a start of frame synchronization code.                                                                                          |


## 27.8.8 DVP embedded synchronization unmask register (DVP_SUR)

| Bit        | Name | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                   |
| ---------- | ---- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 24 | FMEU | 0x00        | rw   | Frame end synchronization code unmask<br/>This field specifies the mask to be applied to the code of the frame end synchronization.<br/>PDL=0, set bit N =0 in the FMEU, the bit N is masked<br/>PDL=1, set bit N =0 in the FMEU, the bit N+2 is masked<br/>PDL=2, set bit N =0 in the FMEU, the bit N+4 is masked<br/>PDL=2, set bit N =0 in the FMEU, the bit N+6 is masked |
| Bit 23: 16 | LNEU | 0x00        | rw   | Line end synchronization code unmask<br/>This field specifies the mask to be applied to the code of the line end synchronization.<br/>PDL=0, set bit N =0 in the LNEU, the bit N is masked<br/>PDL=1, set bit N =0 in the LNEU, the bit N+2 is masked<br/>PDL=2, set bit N =0 in the LNEU, the bit N+4 is masked<br/>PDL=2, set bit N =0 in the LNEU, the bit N+6 is masked   |
| Bit 15: 8  | LNSU | 0x00        | rw   | Line start synchronization code unmask<br/>This field specifies the mask to be applied to the code of the line end synchronization.<br/>PDL=0, set bit N =0 in the LNSU, the bit N is masked                                                                                                                                                                                  |
|          |      |      |    | PDL=1, set bit N =0 in the LNSU, the bit N+2 is masked                                                                                                                                                                                |
|          |      |      |    | PDL=2, set bit N =0 in the LNSU, the bit N+4 is masked                                                                                                                                                                                |
|          |      |      |    | PDL=2, set bit N =0 in the LNSU, the bit N+6 is masked                                                                                                                                                                                |
|          |      |      |    | Frame start synchronization code unmask<br/>This field specifies the mask to be applied to the code of the line end synchronization.                                                                                                  |
| Bit 7: 0 | FMSU | 0x00 | rw | PDL=0, set bit N =0 in the FMSU, the bit N is masked<br/>PDL=1, set bit N =0 in the FMSU, the bit N+2 is masked<br/>PDL=2, set bit N =0 in the FMSU, the bit N+4 is masked<br/>PDL=2, set bit N =0 in the FMSU, the bit N+6 is masked |


## 27.8.9 DVP crop window start register (DVP_CWST)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                          |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 29 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                           |
| Bit 28: 16 | CVSTR    | 0x00        | rw   | Crop window vertical start line<br/>This field specifies the start position of crop window in vertical axis. The first line data captured after the Frame start is line 0, the second line data captured is line 1, and so on.       |
| Bit 15: 14 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                           |
| Bit 13: 0  | CHSTR    | 0x00        | rw   | Crop window horizontal start pixel<br/>This field specifies the start position of crop window in horizontal axis. The first pixel data captured after the Frame start is data 0, the second line data captured is data 1, and so on. |


## 27.8.10 DVP crop window size register (DVP_CWSZ)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                         |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 30 | Reserved | 0x00        | resd | Kept at its default value.                                                                                                                                                                                          |
| Bit 29: 16 | CVNUM    | 0x00        | rw   | Crop window vertical line number minus one<br/>This field specifies the number of lines of a crop window in vertical axis. CVNM1=N indicates that N+1 data has been cropped from the vertical axis.                 |
| Bit 15: 14 | Reserved | 0x00        | resd | Kept at its default value.                                                                                                                                                                                          |
| Bit 13: 0  | CHNUM    | 0x00        | rw   | Crop window horizontal pixel number minus one<br/>This field specifies the number of pixel data of a crop window in horizontal axis. CHNM1=N indicates that N+1 pixel data has been cropped from the vertical axis. |


## 27.8.11 DVP data register (DVP_DT)

| Bit       | Name | Reset value | Type | Description                                                                |
| --------- | ---- | ----------- | ---- | -------------------------------------------------------------------------- |
| Bit 31: 0 | DT   | 0x00000000  | ro   | Data Port<br/>This register is used by the DMA controller to pick up data. |


## 27.8.12 DVP advanced control register (DVP_ACTRL)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 18 | Reserved | 0x00        | resd | Kept at its default value.                                                                                                                                                                                                                 |
| Bit 17     | VSEID    | 0x0         | rw   | Vertical synchronization event and interrupt definition<br/>0: VSES and VEIS indicates frame end event and interrupt<br/>1: VSES and VEIS indicates frame start event and interrupt                                                        |
| Bit 16     | HSEID    | 0x0         | rw   | Horizontal synchronization event and interrupt definition<br/>0: HSES and HEIS indicates line end event and interrupt<br/>1: HSES and HEIS indicates line start event and interrupt                                                        |
| Bit 15: 13 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                 |
| Bit 12     | DMABT    | 0x0         | rw   | DMA burst transaction<br/>This register works with EDMA’s peripheral transfer configuration (PBURST)<br/>0: DMA burst transaction disabled. The EDMA’s peripheral transfer must be programmed as a single transfer (PBURST=0), or use DMA. |
|          |          |     |      | 1: DMA burst transaction enabled. The EDMA’s peripheral<br/>transfer must be programmed as INCR4 (PBURST=1).<br/>This configuration is enabled only when the EDMA is used<br/>for data transfer. If the DMA is used, this configuration<br/>must be disabled.                                                                                                                                                                                                                |
| Bit 11   | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Bit 10   | IDUS     | 0x0 | rw   | Input data un-used setting<br/>0: Unused data bit in the MSB<br/>1: Unused data bit in the LSB                                                                                                                                                                                                                                                                                                                                                                               |
| Bit 9: 8 | IDUN     | 0x0 | rw   | Input data un-used bit number<br/>0: No unused bits<br/>1: 2-bit unused data<br/>2: 4-bit unused data<br/>3: 6-bit unused data                                                                                                                                                                                                                                                                                                                                               |
| Bit 7    | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Bit 6    | EFDM     | 0x0 | rw   | Enhanced function data format management<br/>0: Enhanced function data format management disabled<br/>1: Enhanced function data format management enabled<br/>Enhanced function data format management must be<br/>enabled before enabling the enhanced image scaling<br/>resize and monochrome image binarization.                                                                                                                                                          |
| Bit 5: 4 | EFDF     | 0x0 | rw   | Enhanced function data format<br/>0: YUV422 (UYVY / VYUY) data format<br/>1: YUV422 (YUYV / YVYU) data format<br/>2: RGB565 and RGB555 data format<br/>3: Y8 (Y only) data format<br/>This configuration is valid when EFDM=1 is asserted.                                                                                                                                                                                                                                   |
| Bit 3    | PCDES    | 0x0 | rw   | Basic pixel capture/drop extended selection<br/>This register works with a basic pixel capture/drop<br/>selection (PCDS).<br/>PCDS=0:<br/>0: Capture the first pixel data and drop others<br/>1: Capture the second pixel data and drop others<br/>PCDS=1:<br/>0: Capture the third pixel data and drop others<br/>1: Capture the forth pixel data and drop others<br/>This configuration is valid only when the basic pixel<br/>capture/drop control is enabled and PCDC=2. |
| Bit 2    | MIBE     | 0x0 | rw   | Monochrome image binarization enable<br/>0: Monochrome image binarization disabled<br/>1: Monochrome image binarization enabled<br/>This configuration is valid only when EFDM=1 and EFDF<br/>is set based on CMOS video camera output format.                                                                                                                                                                                                                               |
| Bit 1    | EFRCE    | 0x0 | rw   | Enhanced frame rate control enable<br/>0: Enhanced frame rate control disabled<br/>1: Enhanced frame rate control enabled<br/>This configuration is valid only when CFM=0 and basic<br/>frame rate control is disabled.                                                                                                                                                                                                                                                      |
| Bit 0    | EISRE    | 0x0 | rw   | Enhanced image scaling resize enable<br/>0: Enhanced image scaling resize disabled<br/>1: Enhanced image scaling resize enabled<br/>This configuration is valid only when PCDC=0 and<br/>LCDC=0.<br/>This configuration works normally only when EFDM=1 and<br/>EFDT is set.                                                                                                                                                                                                 |


# 27.8.13 DVP enhanced horizontal scaling factor register (DVP_HSCF)

| Bit        | Name     | Reset value | Type | Description                                                                                                         |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------- |
| Bit 30: 29 | Reserved | 0x0         | resd | Kept at its default value.                                                                                          |
| Bit 28: 16 | HSRTF    | 0x00        | rw   | Horizontal scaling resize target factor<br/>When EISRE=1, this register must not be 0, or greater than HSRSF value. |
| Bit 15: 13 | Reserved | 0x0         | resd | Kept at its default value.                                                                                          |
| Bit 12: 0  | HSRSF    | 0x00        | rw   | Horizontal scaling resize source factor<br/>When EISRE=1, this register must not be 0,                              |


# 27.8.14 DVP enhanced vertical scaling factor register (DVP_VSCF)

| Bit        | Name     | Reset value | Type | Description                                                                                                       |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------- |
| Bit 30: 29 | Reserved | 0x0         | resd | Kept at its default value.                                                                                        |
| Bit 28: 16 | VSRTF    | 0x00        | rw   | Vertical scaling resize target factor<br/>When EISRE=1, this register must not be 0, or greater than VSRSF value. |
| Bit 15: 13 | Reserved | 0x0         | resd | Kept at its default value.                                                                                        |
| Bit 12: 0  | VSRSF    | 0x00        | rw   | Vertical scaling resize source factor<br/>When EISRE=1, this register must not be 0,                              |


# 27.8.15 DVP enhanced frame rate control factor register (DVP_FRF)

| Bit        | Name     | Reset value | Type | Description                                                                                                          |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------- |
| Bit 30: 13 | Reserved | 0x00000     | resd | Kept at its default value.                                                                                           |
| Bit 12: 8  | EFRCTF   | 0x00        | rw   | Enhanced frame rate control target factor<br/>When EFRE=1, this register must not be 0, or greater than EFRSF value. |
| Bit 7: 5   | Reserved | 0x0         | resd | Kept at its default value.                                                                                           |
| Bit 4: 0   | EFRCSF   | 0x00        | rw   | Enhanced frame rate control source factor<br/>When EFRCE=1, this register must not be 0                              |


# 27.8.16 DVP binarization threshold register (DVP_BTH)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                    |
| --------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 30: 8 | Reserved | 0x000000    | resd | Kept at its default value.                                                                                                                                                                                                                                     |
| Bit 7: 0  | MIBTHD   | 0x00        | rw   | Monochrome image binarization threshold<br/>Monochrome image is binarized based on this threshold.<br/>The value above the threshold is regarded as 1, while the value below the threshold is regarded as 0.<br/>This configuration is valid only when MIBE=1. |


