
ARTERY logo
AT32F435/437 Series Reference Manual

### Figure 18-18 Regular shift mode and DMA mode 2

Diagram showing ADC regular shift mode and DMA mode 2 timing and data flow

## 18.6 ADC registers

Table 18-5 lists ADC register map and their reset values.

These peripheral registers must be accessed by words (32 bits).

Table 18-5 ADC register map and reset values

| Register     | Offset | Reset value |
| ------------ | ------ | ----------- |
| ADC1\_STS    | 0x000  | 0x0000 0000 |
| ADC1\_CTRL1  | 0x004  | 0x0000 0000 |
| ADC1\_CTRL2  | 0x008  | 0x0000 0000 |
| ADC1\_SPT1   | 0x00C  | 0x0000 0000 |
| ADC1\_SPT2   | 0x010  | 0x0000 0000 |
| ADC1\_PCDTO1 | 0x014  | 0x0000 0000 |
| ADC1\_PCDTO2 | 0x018  | 0x0000 0000 |
| ADC1\_PCDTO3 | 0x01C  | 0x0000 0000 |
| ADC1\_PCDTO4 | 0x020  | 0x0000 0000 |
| ADC1\_VMHB   | 0x024  | 0x0000 FFFF |
| ADC1\_VMLB   | 0x028  | 0x0000 0000 |
| ADC1\_OSQ1   | 0x02C  | 0x0000 0000 |
| ADC1\_OSQ2   | 0x030  | 0x0000 0000 |
| ADC1\_OSQ3   | 0x034  | 0x0000 0000 |
| ADC1\_PSQ    | 0x038  | 0x0000 0000 |
| ADC1\_PDT1   | 0x03C  | 0x0000 0000 |
| ADC1\_PDT2   | 0x040  | 0x0000 0000 |
| ADC1\_PDT3   | 0x044  | 0x0000 0000 |
| ADC1\_PDT4   | 0x048  | 0x0000 0000 |
| ADC1\_ODT    | 0x04C  | 0x0000 0000 |
| ADC1\_OVSP   | 0x080  | 0x0000 0000 |
| ADC1\_CALVAL | 0x0B4  | 0x0000 0000 |


2025.05.28
Page 378
Rev 2.07





Artery logo
AT32F435/437 Series Reference Manual

| ADC2\_STS    | 0x100 | 0x0000 0000 |
| ------------ | ----- | ----------- |
| ADC2\_CTRL1  | 0x104 | 0x0000 0000 |
| ADC2\_CTRL2  | 0x108 | 0x0000 0000 |
| ADC2\_SPT1   | 0x10C | 0x0000 0000 |
| ADC2\_SPT2   | 0x110 | 0x0000 0000 |
| ADC2\_PCDTO1 | 0x114 | 0x0000 0000 |
| ADC2\_PCDTO2 | 0x118 | 0x0000 0000 |
| ADC2\_PCDTO3 | 0x11C | 0x0000 0000 |
| ADC2\_PCDTO4 | 0x120 | 0x0000 0000 |
| ADC2\_VMHB   | 0x124 | 0x0000 FFFF |
| ADC2\_VMLB   | 0x128 | 0x0000 0000 |
| ADC2\_OSQ1   | 0x12C | 0x0000 0000 |
| ADC2\_OSQ2   | 0x130 | 0x0000 0000 |
| ADC2\_OSQ3   | 0x134 | 0x0000 0000 |
| ADC2\_PSQ    | 0x138 | 0x0000 0000 |
| ADC2\_PDT1   | 0x13C | 0x0000 0000 |
| ADC2\_PDT2   | 0x140 | 0x0000 0000 |
| ADC2\_PDT3   | 0x144 | 0x0000 0000 |
| ADC2\_PDT4   | 0x148 | 0x0000 0000 |
| ADC2\_ODT    | 0x14C | 0x0000 0000 |
| ADC2\_OVSP   | 0x180 | 0x0000 0000 |
| ADC2\_CALVAL | 0x1B4 | 0x0000 0000 |
| ADC3\_STS    | 0x200 | 0x0000 0000 |
| ADC3\_CTRL1  | 0x204 | 0x0000 0000 |
| ADC3\_CTRL2  | 0x208 | 0x0000 0000 |
| ADC3\_SPT1   | 0x20C | 0x0000 0000 |
| ADC3\_SPT2   | 0x210 | 0x0000 0000 |
| ADC3\_PCDTO1 | 0x214 | 0x0000 0000 |
| ADC3\_PCDTO2 | 0x218 | 0x0000 0000 |
| ADC3\_PCDTO3 | 0x21C | 0x0000 0000 |
| ADC3\_PCDTO4 | 0x220 | 0x0000 0000 |
| ADC3\_VMHB   | 0x224 | 0x0000 FFFF |
| ADC3\_VMLB   | 0x228 | 0x0000 0000 |
| ADC3\_OSQ1   | 0x22C | 0x0000 0000 |
| ADC3\_OSQ2   | 0x230 | 0x0000 0000 |
| ADC3\_OSQ3   | 0x234 | 0x0000 0000 |
| ADC3\_PSQ    | 0x238 | 0x0000 0000 |
| ADC3\_PDT1   | 0x23C | 0x0000 0000 |
| ADC3\_PDT2   | 0x240 | 0x0000 0000 |
| ADC3\_PDT3   | 0x244 | 0x0000 0000 |
| ADC3\_PDT4   | 0x248 | 0x0000 0000 |


2025.05.28
Page 379
Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

| ADC3\_ODT    | 0x24C | 0x0000 0000 |
| ------------ | ----- | ----------- |
| ADC3\_OVSP   | 0x280 | 0x0000 0000 |
| ADC3\_CALVAL | 0x2B4 | 0x0000 0000 |
| ADC\_CSTS    | 0x300 | 0x0000 0000 |
| ADC\_CCTRL   | 0x304 | 0x0000 0000 |
| ADC\_CODT    | 0x308 | 0x0000 0000 |


# 18.6.1 ADC status register (ADC_STS)

Accessed by words.

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 7 | Reserved | 0x0000000   | resd | Kept at its default value.                                                                                                                                                                                                                                 |
| Bit 6     | RDY      | 0x0         | rw0c | ADC conversion ready flag<br/>This bit is read only. It is set by hardware when the ADC is powered.<br/>0: Not ready for ADC conversion<br/>1: Ready for ADC conversion                                                                                    |
| Bit 5     | OCCO     | 0x0         | rw0c | Ordinary channel conversion overflow flag)<br/>This bit is set by hardware and cleared by software (writing 0).<br/>0: No overflow occurred<br/>1: Overflow occurred<br/>Overflow detection is applicable to the case of DMA transfer enable or EOCSFEN =1 |
| Bit 4     | OCCS     | 0x0         | rw0c | Ordinary channel conversion start flag<br/>This bit is set by hardware and cleared by software (writing 0).<br/>0: No ordinary channel conversion started<br/>1: Ordinary channel conversion has started                                                   |
| Bit 3     | PCCS     | 0x0         | rw0c | Preempted channel conversion start flag<br/>This bit is set by hardware and cleared by software (writing 0).<br/>0: No preempted channel conversion started<br/>1: Preempted channel conversion has started                                                |
| Bit 2     | PCCE     | 0x0         | rw0c | Preempted channel end of conversion flag<br/>This bit is set by hardware and cleared by software (writing 0).<br/>0: Conversion is not complete<br/>1: Conversion is complete                                                                              |
| Bit 1     | OCCE     | 0x0         | rw0c | End of conversion flag<br/>This bit is set by hardware. It is cleared by software (writing 0) or by reading the ADC\_ODT register.<br/>0: Conversion is not complete<br/>1: Conversion is complete                                                         |
| Bit 0     | VMOR     | 0x0         | rw0c | Voltage monitoring out of range flag<br/>This bit is set by hardware and cleared by software (writing 0).<br/>0: Voltage is within the value programmed<br/>1: Voltage is outside the value programmed                                                     |


2025.05.28
Page 380
Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

# 18.6.2 ADC control register 1 (ADC_CTRL1)

Accessed by words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                               |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 27 | Reserved | 0x00        | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                |
| Bit 26     | OCCOIE   | 0x0         | rw   | Ordinary channel conversion overflow interrupt enable<br/>0: Ordinary channel conversion overflow interrupt disabled<br/>1: Ordinary channel conversion overflow interrupt enabled                                                                                                                                                        |
| Bit 25: 24 | CRSEL    | 0x0         | rw   | Conversion resolution select<br/>00: 12-bit<br/>01: 10-bit<br/>10: 8-bit<br/>11: 6-bit                                                                                                                                                                                                                                                    |
| Bit 23     | OCVMEN   | 0x0         | rw   | Voltage monitoring enable on ordinary channels<br/>0: Voltage monitoring disabled on ordinary channels<br/>1: Voltage monitoring enabled on ordinary channels                                                                                                                                                                             |
| Bit 22     | PCVMEN   | 0x0         | rw   | Voltage monitoring enable on preempted channels<br/>0: Voltage monitoring disabled on preempted channels<br/>1: Voltage monitoring enabled on preempted channels                                                                                                                                                                          |
| Bit 21: 16 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                |
| Bit 15: 13 | OCPCNT   | 0x0         | rw   | Partitioned mode conversion count of ordinary channels<br/>000: 1 channel<br/>001: 2 channels<br/>......<br/>111: 8 channels<br/>Note: In this mode, the preempted group converts only one channel at each trigger.                                                                                                                       |
| Bit 12     | PCPEN    | 0x0         | rw   | Partitioned mode enable on preempted channels<br/>0: Partitioned mode disabled on preempted channels<br/>1: Partitioned mode enabled on preempted channels                                                                                                                                                                                |
| Bit 11     | OCPEN    | 0x0         | rw   | Partitioned mode enable on ordinary channels<br/>This is set and cleared by software to enable or disable partitioned mode on ordinary channels.<br/>0: Partitioned mode disabled on ordinary channels<br/>1: Partitioned mode enabled on ordinary channels                                                                               |
| Bit 10     | PCAUTOEN | 0x0         | rw   | Preempted group automatic conversion enable after ordinary group<br/>0: Preempted group automatic conversion disabled<br/>1: Preempted group automatic conversion enabled                                                                                                                                                                 |
| Bit 9      | VMSGEN   | 0x0         | rw   | Voltage monitoring enable on a single channel<br/>0: Disabled (Voltage monitoring enabled on all channels)<br/>1: Enabled (Voltage monitoring enabled a single channel)                                                                                                                                                                   |
| Bit 8      | SQEN     | 0x0         | rw   | Sequence mode enable<br/>0: Sequence mode disabled (a single channel is converted)<br/>1: Sequence mode enabled (the selected multiple channels are converted)<br/>Note: If multip-channel conversion is enabled and the CCEIEN/PCCEIEN is set, an OCCE or PCCE interrupt is generated only at the end of conversion of the last channel. |


2025.05.28
Page 381
Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

| Bit 7    | PCCEIEN | 0x0  | rw | Conversion end interrupt enable on Preempted channels<br/>0: Conversion end interrupt disabled on Preempted channels<br/>1: Conversion end interrupt enabled on Preempted channels                                                                                                                           |
| -------- | ------- | ---- | -- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 6    | VMORIEN | 0x0  | rw | Voltage monitoring out of range interrupt enable<br/>0: Voltage monitoring out of range interrupt disabled<br/>1: Voltage monitoring out of range interrupt enabled                                                                                                                                          |
| Bit 5    | CCEIEN  | 0x0  | rw | Channel conversion end interrupt enable<br/>0: Channel conversion end interrupt disabled<br/>1: Channel conversion end interrupt enabled                                                                                                                                                                     |
| Bit 4: 0 | VMCSEL  | 0x00 | rw | Voltage monitoring channel select<br/>This filed is valid only when the VMSGEN is enabled.<br/>00000: ADC\_IN0 channel<br/>00001: ADC\_IN1 channel<br/>......<br/>01111: ADC\_IN15 channel<br/>10000: ADC\_IN16 channel<br/>10001: ADC\_IN17 channel<br/>10010\~11111: Unused, configuration is not allowed. |


# 18.6.3 ADC control register 2 (ADC_CTRL2)

Accessed by words.

| Bit                   | Name     | Reset value | Type | Description                                                                                                                                                                                                                                      |
| --------------------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 30: 26            | Reserved | 0x00        | resd | Kept at its default value                                                                                                                                                                                                                        |
| Bit 30                | OCSWTRG  | 0x0         | rw   | Conversion of ordinary channels triggered by software<br/>0: Conversion of ordinary channels not triggered<br/>1: Conversion of ordinary channels triggered (This bit is cleared by software or by hardware as soon as the conversion starts)    |
| Bit 29: 28            | OCETE    | 0x0         | rw   | Ordinary channel external trigger edge select<br/>00: Edge trigger forbidden<br/>01: Rising edge<br/>01: Falling edge<br/>11: Any edge                                                                                                           |
| Bit 31<br/>Bit 27: 24 | OCTESEL  | 0x0         | rw   | Ordinary channel conversion trigger event select<br/>Note:<br/>Refer to section 18.4.1.1 for details on bits.                                                                                                                                    |
| Bit 22                | PCSWTRG  | 0x0         | rw   | Conversion of preempted channels triggered by software<br/>0: Conversion of preempted channels not triggered<br/>1: Conversion of preempted channels triggered (This bit is cleared by software or by hardware as soon as the conversion starts) |
| Bit 21: 20            | PCETE    | 0x0         | rw   | Preempted channel external trigger edge select<br/>00: Edge trigger forbidden<br/>01: Rising edge<br/>01: Falling edge<br/>11: Any edge                                                                                                          |
| Bit 23<br/>Bit 19: 16 | PCTESEL  | 0x0         | rw   | Preempted channel conversion trigger event select<br/>Note:<br/>Refer to section 18.4.1.1 for details on bits.                                                                                                                                   |
| Bit 15: 12            | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                       |


2025.05.28
Page 382
Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

| Bit 11   | DTALIGN   | 0x0 | rw   | Data alignment<br/>0: Right alignment<br/>1: Left alignment                                                                                                                                                                                                                                                                                                                                                                                                         |
| -------- | --------- | --- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 10   | EOCSFEN   | 0x0 | rw   | Each ordinary channel conversion OCCE flag enable)<br/>0: Disabled<br/>1: Enabled<br/>Note: Overflow detection is enabled automatically when this bit is set.                                                                                                                                                                                                                                                                                                       |
| Bit 9    | OCDRCEN   | 0x0 | rw   | Ordinary channel DMA request continue enable for independent mode)<br/>0: Disabled (After the completion of the programmed number of DMA transfers, no DMA request generated at the end of ordinary conversion)<br/>1: Enabled (Don’t care about the programmed number of DMA transfers, Each ordinary channel sends DMA request at the end of ordinary conversion)<br/>Note:<br/>This bit is set only in non-master/slave mode with OCDMAEN = 1.                   |
| Bit 8    | OCDMAEN   | 0x0 | rw   | DMA transfer enable of ordinary channels<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                                                                                                                                                                                                                                             |
| Bit 7: 5 | Reserved  | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| Bit 4    | ADABRT    | 0x0 | rw   | ADC conversion abort<br/>0: ADC conversion is not aborted<br/>1: ADC conversion is aborted<br/>Note: This bit is cleared by hardware after ADC conversion is aborted. After this bit is cleared, re-triggering ADC conversion is allowed. If the ADABRT is set, it is necessary to wait until it is cleared before starting related ADC opertions.                                                                                                                  |
| Bit 3    | ADCALINIT | 0x0 | rw   | Initialize A/D calibration<br/>This bit is set by software and cleared by hardware. It is cleared after the calibration registers are initialized.<br/>0: No initialization occurred or initialization completed<br/>1: Enable initialization or initializations is ongoing                                                                                                                                                                                         |
| Bit 2    | ADCAL     | 0x0 | rw   | A/D Calibration<br/>0: No calibration occurred or calibration completed<br/>1: Enable calibration or calibration is in process                                                                                                                                                                                                                                                                                                                                      |
| Bit 1    | RPEN      | 0x0 | rw   | Repetition mode enable<br/>0: Repetition mode disabled<br/>When SQEN=0, a single conversion is done each time when a trigger event arrives; when SQEN=1, a group of repetition is done each timer when a trigger event arrives.<br/>1: Repetition mode enabled<br/>When SQEN =0, continuous conversion mode on a single channel is enabled at each trigger event; when SQEN =1, continuous conversion mode on a group of channels is enabled at each trigger event. |


2025.05.28
Page 383
Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

**Bit 0** **ADCEN** **0x0** **rw**
A/D converter enable
0: A/D converter disabled (ADC goes to power-down mode)
1: A/D converter enabled
Note:
When this bit is in OFF state, writing an ON command will wake up ADC from power-down mode.
Note that there is a delay of t<sub>STAB</sub> between power on and start of conversion.

# 18.6.4 ADC sampling time register 1 (ADC_SPT1)

<u>Accessed by words.</u>

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 27 | Reserved | 0x00        | resd | Kept at its default value.                                                                                                                                                                                                                 |
| Bit 26: 24 | CSPT18   | 0x0         | rw   | Sample time selection of channel ADC\_IN18<br/>000: Reserved<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles                           |
| Bit 23: 21 | CSPT17   | 0x0         | rw   | Sample time selection of channel ADC\_IN17<br/>000: Reserved<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles ~~111: 239.5 cycles~~ |
| Bit 20: 18 | CSPT16   | 0x0         | rw   | Sample time selection of channel ADC\_IN16<br/>000: Reserved<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles                           |
| Bit 17: 15 | CSPT15   | 0x0         | rw   | Sample time selection of channel ADC\_IN15<br/>000: Reserved<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles                           |


2025.05.28
Page 384
Rev 2.07





ARTERY logo
# AT32F435/437 Series Reference Manual

| Bit 14: 12 | CSPT14 | 0x0 | rw | Sample time selection of channel ADC\_IN14<br/>000: Reserved<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles   |
| ---------- | ------ | --- | -- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 11: 9  | CSPT13 | 0x0 | rw | Sample time selection of channel ADC\_IN13<br/>000: 2.5 cycles<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles |
| Bit 8: 6   | CSPT12 | 0x0 | rw | Sample time selection of channel ADC\_IN12<br/>000: 2.5 cycles<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles |
| Bit 5: 3   | CSPT11 | 0x0 | rw | Sample time selection of channel ADC\_IN11<br/>000: 2.5 cycles<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles |
| Bit 2: 0   | CSPT10 | 0x0 | rw | Sample time selection of channel ADC\_IN10<br/>000: 2.5 cycles<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles |


2025.05.28
Page 385
Rev 2.07





ARTERY logo AT32F435/437 Series Reference Manual

# 18.6.5 ADC sampling time register 2 (ADC_SPT2)

<u>Accessed</u> by words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                     |
| ---------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 30 | Reserved | 0x0         | resd | Kept at its default value                                                                                                                                                                                       |
| Bit 29: 27 | CSPT9    | 0x0         | rw   | Sample time selection of channel ADC\_IN9<br/>000: Reserved<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles |
| Bit 26: 24 | CSPT8    | 0x0         | rw   | Sample time selection of channel ADC\_IN8<br/>000: Reserved<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles |
| Bit 23: 21 | CSPT7    | 0x0         | rw   | Sample time selection of channel ADC\_IN7<br/>000: Reserved<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles |
| Bit 20: 18 | CSPT6    | 0x0         | rw   | Sample time selection of channel ADC\_IN6<br/>000: Reserved<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles |
| Bit 17: 15 | CSPT5    | 0x0         | rw   | Sample time selection of channel ADC\_IN5<br/>000: Reserved<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles |


2025.05.28 Page 386 Rev 2.07





ARTERY logo
# AT32F435/437 Series Reference Manual

| Bit 14: 12 | CSPT4 | 0x0 | rw | Sample time selection of channel ADC\_IN4<br/>000: Reserved<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles   |
| ---------- | ----- | --- | -- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 11: 9  | CSPT3 | 0x0 | rw | Sample time selection of channel ADC\_IN3<br/>000: Reserved<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles   |
| Bit 8: 6   | CSPT2 | 0x0 | rw | Sample time selection of channel ADC\_IN2<br/>000: Reserved<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles   |
| Bit 5: 3   | CSPT1 | 0x0 | rw | Sample time selection of channel ADC\_IN1<br/>000: 2.5 cycles<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles |
| Bit 2: 0   | CSPT0 | 0x0 | rw | Sample time selection of channel ADC\_IN0<br/>000: 2.5 cycles<br/>001: 6.5 cycles<br/>010: 12.5 cycles<br/>011: 24.5 cycles<br/>100: 47.5 cycles<br/>101: 92.5 cycles<br/>110: 247.5 cycles<br/>111: 640.5 cycles |


2025.05.28
Page 387
Rev 2.07





Artery logo AT32F435/437 Series Reference Manual

## 18.6.6 ADC preempted channel data offset register x (ADC_PCDTOx) (x=1..4)

Accessed by words.

| Bit        | Name     | Reset value | Type | Description                                                                                                       |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------- |
| Bit 31: 12 | Reserved | 0x00000     | resd | Kept at its default value                                                                                         |
| Bit 11: 0  | PCDTOx   | 0x000       | rw   | Data offset for Preempted channel x<br/>Converted data stored in the ADC\_PDTx = Raw converted data – ADC\_PCDTOx |


## 18.6.7 ADC voltage monitor high threshold register (ADC_VWHB)

Accessed by words.

| Bit        | Name     | Reset value | Type | Description                      |
| ---------- | -------- | ----------- | ---- | -------------------------------- |
| Bit 31: 16 | Reserved | 0x00000     | resd | Kept at its default value        |
| Bit 15: 0  | VMHB     | 0xFFFF      | rw   | Voltage monitoring high boundary |


## 18.6.8 ADC voltage monitor low threshold register (ADC_VWLB)

Accessed by words.

| Bit        | Name     | Reset value | Type | Description                     |
| ---------- | -------- | ----------- | ---- | ------------------------------- |
| Bit 31: 16 | Reserved | 0x00000     | resd | Kept at its default value       |
| Bit 15: 0  | VMLB     | 0x0000      | rw   | Voltage monitoring low boundary |


## 18.6.9 ADC ordinary sequence register 1 (ADC_OSQ1)

Accessed by words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                             |
| ---------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 24 | Reserved | 0x00        | resd | Kept at its default value                                                                                                                                                               |
| Bit 23: 20 | OCLEN    | 0x0         | rw   | Ordinary conversion sequence length<br/>0000: 1 conversion<br/>0001: 2 conversions<br/>......<br/>1111: 16 conversions                                                                  |
| Bit 19: 15 | OSN16    | 0x00        | rw   | Number of 16th conversion in ordinary sequence                                                                                                                                          |
| Bit 14: 10 | OSN15    | 0x00        | rw   | Number of 15th conversion in ordinary sequence                                                                                                                                          |
| Bit 9: 5   | OSN14    | 0x00        | rw   | Number of 14th conversion in ordinary sequence                                                                                                                                          |
| Bit 4: 0   | OSN13    | 0x00        | rw   | Number of 13th conversion in ordinary sequence<br/>Note: The number can be from 0 to 17. For example, if the number is set to 3, it means that the 13th conversion is ADC\_IN3 channel. |


## 18.6.10 ADC ordinary sequence register 2 (ADC_OSQ2)

Accessed by words.

| Bit        | Name     | Reset value | Type | Description                                    |
| ---------- | -------- | ----------- | ---- | ---------------------------------------------- |
| Bit 31: 30 | Reserved | 0x0         | resd | Kept at its default value                      |
| Bit 29: 25 | OSN12    | 0x00        | rw   | Number of 12th conversion in ordinary sequence |
| Bit 24: 20 | OSN11    | 0x00        | rw   | Number of 11th conversion in ordinary sequence |
| Bit 19: 15 | OSN10    | 0x00        | rw   | Number of 10th conversion in ordinary sequence |
| Bit 14: 10 | OSN9     | 0x00        | rw   | Number of 9th conversion in ordinary sequence  |


2025.05.28 Page 388 Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

| Bit      | Name | Reset value | Type | Description                                                                                                                                                                           |
| -------- | ---- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 9: 5 | OSN8 | 0x00        | rw   | Number of 8th conversion in ordinary sequence                                                                                                                                         |
| Bit 4: 0 | OSN7 | 0x00        | rw   | Number of 7th conversion in ordinary sequence<br/>Note: The number can be from 0 to 17. For example, if the number is set to 8, it means that the 7th conversion is ADC\_IN8 channel. |


# 18.6.11 ADC ordinary sequence register 3 (ADC_ OSQ3)

Accessed by words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                            |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 30 | Reserved | 0x0         | resd | Kept at its default value                                                                                                                                                              |
| Bit 29: 25 | OSN6     | 0x00        | rw   | Number of 6th conversion in ordinary sequence                                                                                                                                          |
| Bit 24: 20 | OSN5     | 0x00        | rw   | Number of 5th conversion in ordinary sequence                                                                                                                                          |
| Bit 19: 15 | OSN4     | 0x00        | rw   | Number of 4th conversion in ordinary sequence                                                                                                                                          |
| Bit 14: 10 | OSN3     | 0x00        | rw   | Number of 3rd conversion in ordinary sequence                                                                                                                                          |
| Bit 9: 5   | OSN2     | 0x00        | rw   | Number of 2nd conversion in ordinary sequence                                                                                                                                          |
| Bit 4: 0   | OSN1     | 0x00        | rw   | Number of 1st conversion in ordinary sequence<br/>Note: The number can be from 0 to 17. For example, if the number is set to 8, it means that the 1st conversion is ADC\_IN17 channel. |


# 18.6.12 ADC preempted sequence register (ADC_ PSQ)

Accessed by words.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                        |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 30 | Reserved | 0x0         | resd | Kept at its default value                                                                                                                                                                                                                                                                                                                                                                          |
| Bit 21: 20 | PCLEN    | 0x0         | rw   | Preempted conversion sequence length<br/>00: 1 conversion<br/>01: 2 conversions<br/>10: 3 conversions<br/>11: 4 conversions                                                                                                                                                                                                                                                                        |
| Bit 19: 15 | PSN4     | 0x00        | rw   | Number of 4th conversion in preempted sequence                                                                                                                                                                                                                                                                                                                                                     |
| Bit 14: 10 | PSN3     | 0x00        | rw   | Number of 3rd conversion in preempted sequence                                                                                                                                                                                                                                                                                                                                                     |
| Bit 9: 5   | PSN2     | 0x00        | rw   | Number of 2nd conversion in preempted sequence                                                                                                                                                                                                                                                                                                                                                     |
| Bit 4: 0   | PSN1     | 0x00        | rw   | Number of 1st conversion in preempted sequence<br/>Note: The number can be from 0 to 17. For example, if the number is set to 3, it refers to the ADC\_IN3 channel.<br/>If PCLEN is less than 4, the conversion sequence starts from 4-PCLEN. For example, when ADC\_PSQ (\[21: 0]) =10 00110 00101 00100 00011, it indicates that the scan conversion follows the sequence: 4, 5, 6, not 3, 4, 5. |


# 18.6.13 ADC preempted data register x (ADC_ PDTx) (x=1..4)

Accessed by words.

| Bit        | Name     | Reset value | Type | Description                            |
| ---------- | -------- | ----------- | ---- | -------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value              |
| Bit 15: 0  | PDTx     | 0x0000      | rw   | Conversion data from preempted channel |


2025.05.28
Page 389
Rev 2.07





ARTERY logo AT32F435/437 Series Reference Manual

# 18.6.14 ADC ordinary data register (ADC_ ODT)

<u>Accessed by words.</u>

| Bit        | Name    | Reset value | Type | Description                                                                                                                                                                                                                           |
| ---------- | ------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | ADC2ODT | 0x0000      | ro   | ADC2 conversion data of ordinary channel<br/>Note:<br/>These bits are reserved in ADC2 and ADC3.<br/>In ADC1, these bits are valid only in master/slave mode, and they contain the conversion result from the ADC2 ordinary channels. |
| Bit 15: 0  | ODT     | 0x0000      | ro   | Conversion data of ordinary channel                                                                                                                                                                                                   |


# 18.6.15 ADC oversampling register (ADC_ OVSP)

<u>Accessed by words.</u>

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                               |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 11 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                |
| Bit 10     | OOSRSEL  | 0x0         | rw   | Ordinary oversampling restart mode select<br/>When the ordinary oversampling is interrupted by preempted conversions, this bit can be used to select where to resume ordinary conversions.<br/>0: Continuous mode (ordinary oversampling buffer will be reserved)<br/>1: Restart mode (ordinary oversampling buffer will be cleared, that is, the previously oversampled times are reset) |
| Bit 9      | OOSTREN  | 0x0         | rw   | Ordinary oversampling trigger mode enable<br/>0: Disabled (only one trigger is needed for all oversampling conversions)<br/>1: Enabled (Each oversampling conversion needs a trigger)                                                                                                                                                                                                     |
| Bit 8: 5   | OSSSEL   | 0x0         | rw   | Oversampling shift select<br/>This field is used to define the number of right-shift used in the oversampling results.<br/>0000: No shift<br/>0001: 1 bit<br/>0010: 2 bits<br/>0011: 3 bits<br/>0100: 4 bits<br/>0101: 5 bits<br/>0110: 6 bits<br/>0111: 7 bits<br/>1000: 8 bits<br/>1001\~1111: Unused. Do not configure.                                                                |
| Bit 4: 2   | OSRSEL   | 0x0         | rw   | Oversampling ratio select<br/>000: 2x<br/>001: 4x<br/>010: 8x<br/>011: 16x<br/>100: 32x<br/>101: 64x<br/>110: 128x<br/>111: 256x                                                                                                                                                                                                                                                          |
| Bit 1      | POSEN    | 0x0         | rw   | Preempted oversampling enable<br/>0: Preempted oversampling disabled<br/>1: Preempted oversampling enabled                                                                                                                                                                                                                                                                                |


2025.05.28 Page 390 Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

| Bit 0 | OOSEN | 0x0 | rw | Ordinary oversampling enable<br/>0:Ordinary oversampling disabled<br/>1: Ordinary oversampling enabled |
| ----- | ----- | --- | -- | ------------------------------------------------------------------------------------------------------ |


# 18.6.16 ADC calibration value register (ADC_CALVAL)

Accessed by words.

| Bit       | Name     | Reset value | Type | Description                |
| --------- | -------- | ----------- | ---- | -------------------------- |
| Bit 31: 7 | Reserved | 0x0000      | resd | Kept at its default value. |
| Bit 6: 0  | CALVAL   | 0x0         | rw   | A/D Calibration value      |


# 18.6.17 ADC common status register (ADC_CSTS)

Accessed by words.

| Bit        | Name     | Reset value | Type | Description                                                                                                               |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 23 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                                |
| Bit 22     | RDY3     | 0x0         | ro   | ADC3 conversion ready flag<br/>This bit is the mapping bit of the RDY bit in the ADC3\_STS register.                      |
| Bit 21     | OCCO3    | 0x0         | ro   | ADC3 ordinary channel conversion overflow flag<br/>This bit is the mapping bit of the OCCO bit in the ADC3\_STS register. |
| Bit 20     | OCCS3    | 0x0         | ro   | ADC3 ordinary channel conversion start flag<br/>This bit is the mapping bit of the OCCS bit in the ADC3\_STS register.    |
| Bit 19     | PCCS3    | 0x0         | ro   | ADC3 Preempted channel conversion start flag<br/>This bit is the mapping bit of the PCCS bit in the ADC3\_STS register.   |
| Bit 18     | PCCE3    | 0x0         | ro   | ADC3 preempted channels conversion end flag<br/>This bit is the mapping bit of the PCCE bit in the ADC3\_STS register.    |
| Bit 17     | OCCE3    | 0x0         | ro   | ADC3 ordinary channels conversion end flag<br/>This bit is the mapping bit of the OCCE bit in the ADC3\_STS register.     |
| Bit 16     | VMOR3    | 0x0         | ro   | ADC3 voltage monitoring out of range flag<br/>This bit is the mapping bit of the VMOR bit in the ADC3\_STS register.      |
| Bit 15     | Reserved | 0x0         | resd | Kept at its default value.                                                                                                |
| Bit 14     | RDY2     | 0x0         | ro   | ADC2 conversion ready flag<br/>This bit is the mapping bit of the RDY bit in the ADC2\_STS register.                      |
| Bit 13     | OCCO2    | 0x0         | ro   | ADC2 ordinary channel conversion overflow flag<br/>This bit is the mapping bit of the OCCO bit in the ADC2\_STS register. |
| Bit 12     | OCCS2    | 0x0         | ro   | ADC2 ordinary channel conversion start flag<br/>This bit is the mapping bit of the OCCS bit in the ADC2\_STS register.    |
| Bit 11     | PCCS2    | 0x0         | ro   | ADC2 preempted channel conversion start flag<br/>This bit is the mapping bit of the PCCS bit in the ADC2\_STS register.   |


2025.05.28
Page 391
Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

| Bit 10 | PCCE2    | 0x0 | ro   | **ADC2 Preempted channels conversion end flag**<br/>This bit is the mapping bit of the PCCE bit in the ADC2\_STS register.    |
| ------ | -------- | --- | ---- | ----------------------------------------------------------------------------------------------------------------------------- |
| Bit 9  | OCCE2    | 0x0 | ro   | **ADC2 ordinary channels conversion end flag**<br/>This bit is the mapping bit of the OCCE bit in the ADC2\_STS register.     |
| Bit 8  | VMOR2    | 0x0 | ro   | **ADC2 voltage monitoring out of range flag**<br/>This bit is the mapping bit of the VMOR bit in the ADC2\_STS register.      |
| Bit 7  | Reserved | 0x0 | resd | Kept at its default value.                                                                                                    |
| Bit 6  | RDY1     | 0x0 | ro   | **ADC1 conversion ready flag**<br/>This bit is the mapping bit of the RDY bit in the ADC1\_STS register.                      |
| Bit 5  | OCCO1    | 0x0 | ro   | **ADC1 ordinary channel conversion overflow flag**<br/>This bit is the mapping bit of the OCCO bit in the ADC1\_STS register. |
| Bit 4  | OCCS1    | 0x0 | ro   | **ADC1 ordinary channel conversion start flag**<br/>This bit is the mapping bit of the OCCS bit in the ADC1\_STS register.    |
| Bit 3  | PCCS1    | 0x0 | ro   | **ADC1 Preempted channel conversion start flag**<br/>This bit is the mapping bit of the PCCS bit in the ADC1\_STS register.   |
| Bit 2  | PCCE1    | 0x0 | ro   | **ADC1 preempted channels conversion end flag**<br/>This bit is the mapping bit of the PCCE bit in the ADC1\_STS register.    |
| Bit 1  | OCCE1    | 0x0 | ro   | **ADC1 ordinary channels conversion end flag**<br/>This bit is the mapping bit of the OCCE bit in the ADC1\_STS register.     |
| Bit 0  | VMOR1    | 0x0 | ro   | **ADC1 voltage monitoring out of range flag**<br/>This bit is the mapping bit of the VMOR bit in the ADC1\_STS register.      |


# 18.6.18 ADC common control register (ADC_CCTRL)

Accessed by words.

| Bit        | Name     | Reset value | Type | Description                                                                      |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------- |
| Bit 31: 24 | Reserved | 0x0000      | resd | Kept at its default value.                                                       |
| Bit 23     | ITSRVEN  | 0x0         | rw   | **Internal temperature sensor and VINTRV enable**<br/>0: Disabled<br/>1: Enabled |
| Bit 22     | VBATEN   | 0x0         | rw   | **VBAT enable**<br/>0: Disabled<br/>1: Enabled                                   |
| Bit 21: 20 | Reserved | 0x0         | resd | Kept at its default value.                                                       |


2025.05.28
Page 392
Rev 2.07




ARTERY logo

# AT32F435/437 Series Reference Manual

| Bit 19: 16            | ADCDIV   | 0x0 | rw   | ADC division<br/>0000: HCLK/2<br/>0001: HCLK/3<br/>...<br/>1111: HCLK/17<br/>Note:<br/>The clock divided by this field are used by all ADCs.<br/>The maximum value of the ADCCLK is 80 MHz. After this division, the ADCCLK cannot be higher than PCLK2.                                                                                                                                                                                                                               |
| --------------------- | -------- | --- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 28<br/>Bit 15: 14 | MSDMASEL | 0x0 | rw   | Ordinary channel DMA transfer mode select in master/slave mode<br/>MSDMASEL\[2] is the 28th bit in the ADC\_CCTRL register.<br/>MSDMASEL\[2: 0] is defined as follows:<br/>000: No DMA transfer<br/>001: DMA mode 1<br/>010: DMA mode 2<br/>011: DMA mode 3<br/>100: DMA mode 4<br/>101: DMA mode 5<br/>110\~111: Unused. Do not configure.<br/>Note:<br/>This field is applicable in master/slave mode. Refer to Section 18.5.1 for details on DMA mode x.                            |
| Bit 13                | MSDRCEN  | 0x0 | rw   | Ordinary channel DMA request continuation enable in master/slave mode<br/>0: Disabled (After the completion of the programmed number of DMA transfers, no DMA request generated at the end of ordinary conversion)<br/>1: Enabled (Don't care about the programmed number of DMA transfers, Each ordinary channel sends DMA request at the end of ordinary conversion)<br/>Note: This bit is applicable in master/slave mode and when DMA mode x is selected through the MSDMASEL bit. |
| Bit 12                | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Bit 11: 8             | ASISEL   | 0x0 | rw   | Adjacent ADC sampling interval select in ordinary shift mode<br/>0000: 5 \* TADCCLK<br/>0001: 6 \* TADCCLK<br/>0010: 7 \* TADCCLK<br/>...<br/>1111: 20 \* TADCCLK<br/>Note:<br/>This field is used to configure the conversion interval between adjacent ADCs in ordinary shift mode. Refer to Section 18.5.4 for details.                                                                                                                                                             |
| Bit 7: 5              | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                             |


2025.05.28
Page 393
Rev 2.07





Artery logo
AT32F435/437 Series Reference Manual

| Bit 4: 0 | MSSEL | 0x00 | rw | Combined master/slave mode select<br/>00000: Non-combined mode<br/>00001: Combined ordinary simultaneous+preempted simultaneous modes (single slave)<br/>00010: Combined ordinary simultaneous+alternate preempted trigger modes (single slave)<br/>00011\~00100: Unused. Do not configure<br/>00101: Preempted simultaneous mode (single slave)<br/>00110: Ordinary simultaneous mode (single slave)<br/>00111: Ordinary shift mode (single slave)<br/>01000: Unused. Do not configure<br/>01001: Alternate preempted trigger mode (single slave)<br/>01010\~10000: Unused. Do not configure<br/>10001: Combined ordinary simultaneous+preempted simultaneous modes (dual slave)<br/>10010: Combined ordinary simultaneous+alternate preempted trigger modes (dual slave)<br/>10011\~10100: Unused. Do not configure<br/>10101: Preempted simultaneous mode (dual slave)<br/>10110: Ordinary simultaneous mode (dual slave)<br/>10111: Ordinary shift mode (dual slave)<br/>11000: Unused. Do not configure<br/>11001: Alternate preempted trigger mode (dual slave)<br/>11010\~11111: Unused. Do not configure<br/>Note: In combined master/slave mode, the change of configuration will cause loss of synchronization between master and slave. Thus it is recommended to disable combined master/slave mode before changing. |
| -------- | ----- | ---- | -- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |


2025.05.28
Page 394
Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

# 18.6.19 ADC common data register (ADC_CODT)

Accessed by words.

| Bit        | Name  | Reset value | Type | Description                                                                                                                                                                   |
| ---------- | ----- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | CODTH | 0x0000      | rw   | Ordinary conversion high halfword data in master slave mode<br/>Note: The meanings of data in this field vary from DMA mode to DMA mode. Refer to Section 18.5.1 for details. |
| Bit 15: 0  | CODTL | 0x0000      | rw   | Ordinary conversion low halfword data in master slave mode<br/>Note: The meanings of data in this field vary from DMA mode to DMA mode. Refer to Section 18.5.1 for details.  |


2025.05.28
Page 395
Rev 2.07
