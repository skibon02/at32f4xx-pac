
ARTERY logo
AT32F435/437 Series Reference Manual

Figure 22-3 Cross-return algorithm

Diagram showing three vertical lines representing thresholds C1 = 7980, C2 = 8000, and C3 = 8020 on a horizontal axis.

From the above figure, auto calibration function will adjust the HICKCAL or HICKTRIM according to the specified step as soon as the condition for triggering auto calibration is reached.

**Cross:**

If the auto calibration condition is met, the actual sampling data in the first 1ms period will be either less than C2, or greater than C2.

When this value is less than C2, the auto calibration module will start increasing either the HICKCAL or HICKTRIM according to the step definition until the actual sampling value is greater than C2. In this way, the actual value will cross over C2 from small to large.

When this value is greater than C2, the auto calibration module will start decrease either the HICKCAL or HICKTRIM according to the step definition until the actual sampling value become less than C1. In this way, the actual value will cross over C2 from large to small.

**Return:**

After cross operation is completed, the actual value closest to C2 can be obtained by comparing the difference (calculated as absolute value) between the actual sampling value and C2 before and after crossing C2 so as to get the best calibration value HICKCAL or HICKTRIM.

If the difference after crossing is less than the one before crossing C2, the calibration value after crossing prevails, and stops the calibration process until the next condition for auto calibration appears.

If the difference after crossing is greater than the one before crossing C2, the calibration value before crossing prevails, and it will return by one step to the one before crossing, and stops the calibration process until the next condition for auto calibration appears.

According to the cross-return strategy, in theory, it is possible to get the frequency accuracy that is 0.5 steps away from the center frequency.

Four conditions for enabling auto calibration function are as follows:

1. The rising edge of the CANLON (from 0 to 1)
2. When CALON=1, reference signal is lost and restored
3. When the sample counter is less than C1
4. When the sample counter is greater than C3

Even though the sampling counter is between C1 and C3, at the rising edge the CANLON, the auto calibration module can also be activated so that the HICK frequency can be adjusted to be within a range of 0.5 steps of the center frequency as soon as the CANLON is enabled.

Under one of the above-mentioned circumstances, the HICK frequency can be calibrated to be within 0.5 steps of the center frequency. To achieve the best calibration accuracy, it is recommended to remain step as 1 (default value). If the step is set to 0, either HICKCAL or HICKTRIM will not be able to be calibrated.

## 22.6 Register description

Refer to the list of abbreviations used in register descriptions.

These peripheral registers must be accessed by words (32 bits).

2025.05.28
Page 505
Rev 2.07





ARTERY logo AT32F435/437 Series Reference Manual

# 22.6.1 ACC register map

Table 22-2 ACC register map and reset values

| Register   | Offset | Reset value  |
| ---------- | ------ | ------------ |
| ACC\_STS   | 0x00   | 0x0000 000   |
| ACC\_CTRL1 | 0x04   | 0x0000 0100  |
| ACC\_CTRL2 | 0x08   | 0x0000 2080  |
| ACC\_C1    | 0x0C   | 0x0000 1F2C  |
| ACC\_C2    | 0x10   | 0x0000 1F40  |
| ACC\_C3    | 0x14   | 0x00000 1F54 |


# 22.6.2 Status register (ACC_STS)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 2 | Reserved | 0x0000000   | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Bit 1     | RSLOST   | 0x0         | ro   | Reference Signal Lost<br/>0: Reference Signal is not lost<br/>1: Reference Signal is lost<br/>Note: During the calibration, when the sample counter of the calibration module is twice that of C2, if a SOF reference signal is not detected, it means that the reference signal is lost. The internal statue machine will move to the idle state unless another SOF signal is detected; otherwise, the internal clock sample counter remains 0. The RSLOST bit is immediately cleared after the CALON bit is cleared or when the RSLOST is written with 0. Reference signal detection occurs only when CALON=1. |
| Bit 0     | CALRDY   | 0x0         | ro   | Internal high-speed clock calibration ready<br/>0: Internal 8MHz oscillator calibration is not ready<br/>1: Internal 8MHz oscillator calibration is ready<br/>Note: This bit is set by hardware to indicate that internal 8MHz oscillator has been calibrated to the frequency closest to 8MHz. The CALRDY is immediately cleared after the CALON bit is cleared or when the CALRDY is written with 0.                                                                                                                                                                                                           |


# 22.6.3 Control register 1 (ACC_CTRL1)

| Bit        | Name      | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ---------- | --------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 12 | Reserved  | 0x00000     | resd | Forced by hardware to 0                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Bit 11: 8  | STEP      | 0x1         | rw   | Calibrated step<br/>This field defines the value after each calibration.<br/>Note: It is recommended to set the step bit in order to get a more accurate calibration result. While ENTRIM=0, only the HICKCAL is calibrated. If the step is incremented or decremented by one, the HICKCAL will be incremented or decremented by one accordingly, and the HICK frequency will increase or decrease by 40KHz (design value). This is a positive relationship.<br/>While ENTRIM=1, only the HICKTRIM is calibrated. If the step is incremented or decremented by one, the HICKTRIM will be incremented or decremented by one accordingly, and the HICK frequency will increase or decrease by 20KHz (design value). This is a positive relationship. |
| Bit 7: 6   | Reserved  | 0x0         | rw   | Forced by hardware to 0                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Bit 5      | CALRDYIEN | 0x0         | rw   | CALRDY interrupt enable<br/>This bit is set or cleared by software.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |


2025.05.28 Page 506 Rev 2.07





Artery logo
AT32F435/437 Series Reference Manual

| Bit   | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                               |
| ----- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|       |          |             |      | 0: Interrupt generation disabled<br/>1: ACC interrupt is generated when CALRDY=1 in the ACC\_STS register                                                                                                                                                                                                                                                 |
| Bit 4 | EIEN     | 0x0         | rw   | RSLOST error interrupt enable<br/>This bit is set or cleared by software.<br/>0: Interrupt generation disabled<br/>1: ACC interrupt is generated when RSLOST=1 in the ACC\_STS register                                                                                                                                                                   |
| Bit 3 | Reserved | 0x0         | rw   | Forced by hardware to 0                                                                                                                                                                                                                                                                                                                                   |
| Bit 2 | SOFSEL   | 0x0         | rw   | SOF select<br/>This bit is set or cleared by software.<br/>0: USB1 as SOF signal source<br/>1: USB2 as SOF signal source                                                                                                                                                                                                                                  |
| Bit 1 | ENTRIM   | 0x0         | rw   | Enable trim<br/>This bit is set or cleared by software.<br/>0: HICKCAL is calibrated.<br/>1: HICKTRIM is calibrated.<br/>Note: It is recommended to set ENTRIM=1 in order to get higher calibration accuracy.                                                                                                                                             |
| Bit 0 | CALON    | 0x0         | rw   | Calibration on<br/>This bit is set or cleared by software.<br/>0: Calibration disabled<br/>1: Calibration enabled, and starts searching for a pulse on the USB\_SOF.<br/>Note: This module cannot be used without the USB\_SOF reference signal. If there are no requirements on the accuracy of the HICK clock, it is unnecessary to enable this module. |


## 22.6.4 Control register 2 (ACC_CTRL2)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 14 | Reserved | 0x00000     | resd | Forced to 0 by hardware                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| Bit 13: 8  | HICKTRIM | 0x20        | ro   | Internal high-speed auto clock trimming<br/>This field is read only, but not written.<br/>Internal high-speed clock is adjusted by ACC module, which is added to the ACC\_HICKCAL\[7: 0] bit. These bits allow the users to input a trimming value to adjust the frequency of the HICKRC oscillator according to the variations in voltage and temperature.<br/>The default value is 32, which can trim the HICK to 8MHz±0.25%. The trimming value is 20kHz (design value) between two consecutive ACC\_HICKTRIM steps. |
| Bit 7: 0   | HICKCAL  | 0x80        | ro   | Internal high-speed auto clock calibration<br/>This field is read only, but not written.<br/>Internal high-speed clock is adjusted by ACC module. These bits allow the users to input a trimming value to adjust the frequency of the HICKPC oscillator according to the variations in voltage and temperature.<br/>The default value is 128, which can trim the HICK to 8MHz±0.25%. The trimming value is 40kHz (design value) between two consecutive ACC\_HICKCAL steps.                                             |


## 22.6.5 Compare value 1 (ACC_C1)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                   |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Forced to 0 by hardware                                                                                                                                                                                                                                                                                                                                                       |
| Bit 15: 0  | C1       | 0x1F2C      | rw   | Compare 1<br/>This value is the lower boundary for triggering calibration, and its default value is 7980. When the number of clocks sampled by ACC in 1ms period is less than or equal to C1, auto calibration is triggered automatically.<br/>When the actual sampling value (number of clocks in 1ms) is greater than C1 but less than C3, auto calibration is not enabled. |


2025.05.28
Page 507
Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

## 22.6.6 Compare value 2 (ACC_C2)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Forced to 0 by hardware                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Bit 15: 0  | C2       | 0x1F40      | rw   | Compare 2<br/>This value defines the number of clocks sampled for 8MHz (ideal frequency) clock in 1ms period , and its default value is 8000 (theoretical value).<br/>As a center point of cross-return strategy, this value is used to calculate the calibration value closest to the theoretical value. In theory, the actual frequency after calibration can be trimmed to be within an accuracy of 0.5 steps from the target frequency (8MHz). |


## 22.6.7 Compare value 3 (ACC_C3)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                             |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Forced to 0 by hardware                                                                                                                                                                                                                                                                                                                                 |
| Bit 15: 0  | C3       | 0x1F54      | rw   | Compare 3<br/>This value is the upper boundary for triggering calibration. When the number of clock sampled by ACC in 1ms period is greater than or equal to C3, auto calibration is triggered automatically.<br/>When the actual sampling value (number of clocks in 1ms period) is greater than C1 but less than C3, auto calibration is not enabled. |


2025.05.28
Page 508
Rev 2.07
