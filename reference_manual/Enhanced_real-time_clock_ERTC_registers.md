ARTERY logo **AT32F435/437 Series Reference Manual**

Table 17-2 ERTC low-power mode wakeup

| Clock sources | Events                    | Wake up Sleep | Wake up Deepsleep | Wakeup Standby |
| ------------- | ------------------------- | ------------- | ----------------- | -------------- |
| HEXT          | Alarm clock A             | √             | ×                 | ×              |
|               | Alarm clock B             | √             | ×                 | ×              |
|               | Periodic automatic wakeup | √             | ×                 | ×              |
|               | Time stamp                | √             | ×                 | ×              |
|               | Tamper event              | √             | ×                 | ×              |
| LICK          | Alarm clock A             | √             | √                 | √              |
|               | Alarm clock B             | √             | √                 | √              |
|               | Periodic automatic wakeup | √             | √                 | √              |
|               | Time stamp                | √             | √                 | √              |
|               | Tamper event              | √             | √                 | √              |
| LEXT          | Alarm clock A             | √             | √                 | √              |
|               | Alarm clock B             | √             | √                 | √              |
|               | Periodic automatic wakeup | √             | √                 | √              |
|               | Time stamp                | √             | √                 | √              |
|               | Tamper event              | √             | √                 | √              |


Table 17-3 Interrupt control bits

| Interrupt events          | Event flag | Interrupt enable bit | EXINT line |
| ------------------------- | ---------- | -------------------- | ---------- |
| Alarm clock A             | ALAF       | ALAIEN               | 17         |
| Alarm clock B             | ALBF       | ALBIEN               | 17         |
| Periodic automatic wakeup | WATF       | WATIEN               | 22         |
| Time stamp                | TSF        | TSIEN                | 21         |
| Tamper event              | TP1F/TP2F  | TPIEN                | 21         |


# 17.4 ERTC registers

These peripheral registers must be accessed by words (32 bits).
ERTC registers are 32-bit addressable registers.

Table 17-4 ERTC register map and reset values

| Register    | Offset | Reset value |
| ----------- | ------ | ----------- |
| ERTC\_TIME  | 0x00   | 0x0000 0000 |
| ERTC\_DATE  | 0x04   | 0x0000 2101 |
| ERTC\_CTRL  | 0x08   | 0x0000 0000 |
| ERTC\_STS   | 0x0C   | 0x0000 0007 |
| ERTC\_DIV   | 0x10   | 0x007F 00FF |
| ERTC\_WAT   | 0x14   | 0x0000 FFFF |
| ERTC\_CCAL  | 0x18   | 0x0000 0000 |
| ERTC\_ALA   | 0x1C   | 0x0000 0000 |
| ERTC\_ALB   | 0x20   | 0x0000 0000 |
| ERTC\_WP    | 0x24   | 0x0000 0000 |
| ERTC\_SBS   | 0x28   | 0x0000 0000 |
| ERTC\_TADJ  | 0x2C   | 0x0000 0000 |
| ERTC\_TSTM  | 0x30   | 0x0000 0000 |
| ERTC\_TSDT  | 0x34   | 0x0000 000D |
| ERTC\_TSSBS | 0x38   | 0x0000 0000 |
| ERTC\_SCAL  | 0x3C   | 0x0000 0000 |


2025.05.28 Page 352 Rev 2.07





Artery logo
AT32F435/437 Series Reference Manual

| ERTC\_TAMP   | 0x40      | 0x0000 0000 |
| ------------ | --------- | ----------- |
| ERTC\_ALASBS | 0x44      | 0x0000 0000 |
| ERTC\_ALBSBS | 0x48      | 0x0000 0000 |
| ERTC\_BPRx   | 0x50-0x9C | 0x0000 0000 |


# 17.4.1 ERTC time register (ERTC_TIME)

| Bit        | Name     | Reset value | Type | Description                                                                                                         |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 23 | Reserved | 0x000       | resd | Kept at its default value.                                                                                          |
| Bit 22     | AMPM     | 0x0         | rw   | AM/PM<br/>0: AM<br/>1: PM<br/>Note: This bit is applicable for 12-hr format only. It is 0 for 24-hr format instead. |
| Bit 21: 20 | HT       | 0x0         | rw   | Hour tens                                                                                                           |
| Bit 19: 16 | HU       | 0x0         | rw   | Hour units                                                                                                          |
| Bit 15     | Reserved | 0x0         | resd | Kept at its default value.                                                                                          |
| Bit 14: 12 | MT       | 0x0         | rw   | Minute tens                                                                                                         |
| Bit 11: 8  | MU       | 0x0         | rw   | Minute units                                                                                                        |
| Bit 7      | Reserved | 0x0         | resd | Kept at its default value.                                                                                          |
| Bit 6: 4   | ST       | 0x0         | rw   | Second tens                                                                                                         |
| Bit 3: 0   | SU       | 0x0         | rw   | Second units                                                                                                        |


# 17.4.2 ERTC date register (ERTC_DATE)

| Bit        | Name     | Reset value | Type | Description                                                                                                                         |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 24 | Reserved | 0x00        | resd | Kept at its default value.                                                                                                          |
| Bit 23: 20 | YT       | 0x0         | rw   | Year tens                                                                                                                           |
| Bit 19: 16 | YU       | 0x0         | rw   | Year units                                                                                                                          |
| Bit 15: 13 | WK       | 0x1         | rw   | Week day<br/>0: Forbidden<br/>1: Monday<br/>2: Tuesday<br/>3: Wednesday<br/>4: Thursday<br/>5: Friday<br/>6: Saturday<br/>7: Sunday |
| Bit 12     | MT       | 0x0         | rw   | Month tens                                                                                                                          |
| Bit 11: 8  | MU       | 0x1         | rw   | Month units                                                                                                                         |
| Bit 7: 6   | Reserved | 0x0         | resd | Kept at its default value.                                                                                                          |
| Bit 5: 4   | DT       | 0x0         | rw   | Date tens                                                                                                                           |
| Bit 3: 0   | DU       | 0x1         | rw   | Date units                                                                                                                          |


# 17.4.3 ERTC control register (ERTC_CTRL)

| Bit        | Name     | Reset value | Type | Description                                                                                                              |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 24 | Reserved | 0x00        | resd | Kept at its default value.                                                                                               |
| Bit 23     | CALOEN   | 0x0         | rw   | Calibration output enable<br/>0: Calibration output disabled<br/>1: Calibration output enabled                           |
| Bit 22: 21 | OUTSEL   | 0x0         | rw   | Output source selection<br/>00: Output source disabled<br/>01: Alarm clock A<br/>10: Alarm clock B<br/>11: Wakeup events |


2025.05.28
Page 353
Rev 2.07





ARTERY logo
AT32F435/437 Series Reference Manual

| Bit 20 | OUTP    | 0x0 | rw | Output polarity<br/>0: High<br/>1: Low                                                                                                                                                                                                                                                                                                                               |
| ------ | ------- | --- | -- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 19 | CALOSEL | 0x0 | rw | Calibration output selection<br/>0: 512Hz<br/>1: 1Hz                                                                                                                                                                                                                                                                                                                 |
| Bit 18 | BPR     | 0x0 | rw | Battery powered domain data register<br/>This bit in the battery powered domain is not affected by a system reset. It is used to store the daylight saving time change or others that need to be saved permanently.                                                                                                                                                  |
| Bit 17 | DEC1H   | 0x0 | wo | Decrease 1 hour<br/>0: No effect<br/>1: Subtract 1 hour<br/>Note: This bit is applicable only when the current hour is not 0. The next second takes effect when this bit is set (don’t set this bit when the hour is being incremented)                                                                                                                              |
| Bit 16 | ADD1H   | 0x0 | wo | Add 1 hour<br/>0: No effect<br/>1: Add 1 hour<br/>Note: The next second takes effect when this bit is set (don’t set this bit when the hour is being incremented)                                                                                                                                                                                                    |
| Bit 15 | TSIEN   | 0x0 | rw | Timestamp interrupt enable<br/>0: Timestamp interrupt disabled<br/>1: Timestamp interrupt enabled                                                                                                                                                                                                                                                                    |
| Bit 14 | WATIEN  | 0x0 | rw | Wakeup timer interrupt enable<br/>0: Wakeup timer interrupt disable<br/>1: Wakeup timer interrupt enabled                                                                                                                                                                                                                                                            |
| Bit 13 | ALBIEN  | 0x0 | rw | Alarm B interrupt enable<br/>0: Alarm B interrupt disabled<br/>1: Alarm B interrupt enabled                                                                                                                                                                                                                                                                          |
| Bit 12 | ALAIEN  | 0x0 | rw | Alarm A interrupt enable<br/>0: Alarm A interrupt disabled<br/>1: Alarm A interrupt enabled                                                                                                                                                                                                                                                                          |
| Bit 11 | TSEN    | 0x0 | rw | Timestamp enable<br/>0: Timestamp disabled<br/>1: Timestamp enabled                                                                                                                                                                                                                                                                                                  |
| Bit 10 | WATEN   | 0x0 | rw | Wakeup timer disabled<br/>0: Wakeup timer disabled<br/>1: Wakeup timer enabled                                                                                                                                                                                                                                                                                       |
| Bit 9  | ALBEN   | 0x0 | rw | Alarm B enable<br/>0: Alarm B disabled<br/>1: Alarm B enabled                                                                                                                                                                                                                                                                                                        |
| Bit 8  | ALAEN   | 0x0 | rw | Alarm A enable<br/>0: Alarm A disabled<br/>1: Alarm A enabled                                                                                                                                                                                                                                                                                                        |
| Bit 7  | CCALEN  | 0x0 | rw | Coarse calibration enable<br/>0: Coarse calibration disabled<br/>1: Coarse calibration enabled                                                                                                                                                                                                                                                                       |
| Bit 6  | HM      | 0x0 | rw | Hour mode<br/>0: 24-hour format<br/>1: 12-hour format                                                                                                                                                                                                                                                                                                                |
| Bit 5  | DREN    | 0x0 | rw | Date/time register direct read enable<br/>0: Date/time register direct read disabled. ERTC\_TIME, ERTC\_DATE and ERTC\_SBS values are taken from the synchronized registers, which are updated once every two ERTC\_CLK cycles<br/>1: Date/time register direct read enabled. ERTC\_TIME, ERTC\_DATE and ERTC\_SBS values are taken from the battery powered domain. |
| Bit 4  | RCDEN   | 0x0 | rw | Reference clock detection enable<br/>0: Reference clock detection disabled<br/>1: Reference clock detection enabled                                                                                                                                                                                                                                                  |
| Bit 3  | TSEDG   | 0x0 | rw | Timestamp trigger edge<br/>0: Rising edge                                                                                                                                                                                                                                                                                                                            |


2025.05.28
Page 354
Rev 2.07




ARTERY logo # AT32F435/437 Series Reference Manual

| Bit 2: 0 | WATCLK | 0x0 | rw | 1: Falling edge<br/>Wakeup timer clock selection<br/>000: ERTC\_CLK/16<br/>001: ERTC\_CLK/8<br/>010: ERTC\_CLK/4<br/>011: ERTC\_CLK/2<br/>10x: ck\_b<br/>11x: ck\_b is selected. 216 is added to the wakeup counter value, and wakeup time =ERTC\_WAT+216.<br/>Note: The write access to this field is supported when WATEN=0 and WATWF=1. |
| -------- | ------ | --- | -- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |


## 17.4.4 ERTC initialization and status register (ERTC_STS)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                             |
| ---------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 17 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                              |
| Bit 16     | CALUPDF  | 0x0         | ro   | Calibration value update complete flag<br/>0: Calibration value update is complete<br/>1: Calibration value update is in progress<br/>This bit is automatically set when software writes to the ERTC\_SCAL register. It is automatically cleared when a new calibration value is taking into account. When this bit is set, the write access to the ERTC\_SCAL register is not allowed. |
| Bit 15     | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                              |
| Bit 14     | TP2F     | 0x0         | rw0c | Tamper detection 2 flag<br/>0: No tamper event<br/>1: Tamper event occurs                                                                                                                                                                                                                                                                                                               |
| Bit 13     | TP1F     | 0x0         | rw0c | Tamper detection 1 flag<br/>0: No tamper event<br/>1: Tamper event occurs                                                                                                                                                                                                                                                                                                               |
| Bit 12     | TSOF     | 0x0         | rw0c | Timestamp overflow flag<br/>0: No timestamp overflow<br/>1: Timestamp overflow occurs<br/>If a new time stamp event is detected when time stamp flag (TSF) is already set, this bit will be set by hardware.                                                                                                                                                                            |
| Bit 11     | TSF      | 0x0         | rw0c | Timestamp flag<br/>0: No timestamp event<br/>1: Timestamp event occurs<br/>It is recommended to double check the TSOF flag after reading a timestamp and clearing the TSF. Otherwise, a new timestamp event may be detected while clearing the TSF.<br/>Note: The clearing operation of this bit takes effect after two APB\_CLK cycles.                                                |
| Bit 10     | WATF     | 0x0         | rw0c | Wakeup timer flag<br/>0: No wakeup timer event<br/>1: Wakeup timer event occurs<br/>Note: The clearing operation of this bit takes effect after two APB\_CLK cycles.                                                                                                                                                                                                                    |
| Bit 9      | ALBF     | 0x0         | rw0c | Alarm clock B flag<br/>0: No alarm clock event<br/>1: Alarm clock event occurs<br/>Note: The clearing operation of this bit takes effect after two APB\_CLK cycles.                                                                                                                                                                                                                     |
| Bit 8      | ALAF     | 0x0         | rw0c | Alarm clock A flag<br/>0: No alarm clock event<br/>1: Alarm clock event occurs<br/>Note: The clearing operation of this bit takes effect after two APB\_CLK cycles.                                                                                                                                                                                                                     |
| Bit 7      | IMEN     | 0x0         | rw   | Initialization mode enable<br/>0: Initialization mode disabled<br/>1: Initialization mode enabled<br/>When an initialization mode is entered, the calendar stops running.                                                                                                                                                                                                               |


2025.05.28 Page 355 Rev 2.07




ARTERY logo

# AT32F435/437 Series Reference Manual

| Bit 6 | IMF   | 0x0 | ro   | Enter initialization mode flag<br/>0: Initialization mode is not entered<br/>1: Initialization mode is entered<br/>The ERTC\_TIME, ERTC\_DATE and ERTC\_DIV registers can be modified only when an initialization mode is enabled (INITEN=1) and entered (INITEF=1).                          |
| ----- | ----- | --- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 5 | UPDF  | 0x0 | rw0c | Calendar update flag<br/>0: Calendar update is in progress<br/>1: Calendar update is complete<br/>The UPDF bit is set each time the shadow register is synchronized with the ERTC calendar value located in the battery powered domain. The synchronization is performed every two ERTC\_CLK. |
| Bit 4 | INITF | 0x0 | ro   | Calendar initialization flag<br/>0: Calendar has not been initialized<br/>1: Calendar has been initialized<br/>This bit is set when the calendar year filed (ERTC\_DATE) is different from 0. It is cleared when the year is 0.                                                               |
| Bit 3 | TADJF | 0x0 | ro   | Time adjustment flag<br/>0: No time adjustment<br/>1: Time adjustment is in progress<br/>This bit is automatically set when a write access to the ERTC\_TADJ register is performed. It is automatically cleared at the end of time adjustment.                                                |
| Bit 2 | WATWF | 0x1 | ro   | Wakeup timer register allows write flag<br/>0: Wakeup timer register configuration not allowed<br/>1: Wakeup timer register configuration allowed                                                                                                                                             |
| Bit 1 | ALBWF | 0x1 | ro   | Alarm B register allows write flag<br/>0: Alarm B register write operation not allowed<br/>1: Alarm B register write operation allowed                                                                                                                                                        |
| Bit 0 | ALAWF | 0x1 | ro   | Alarm A register allows write flag<br/>0: Alarm A register write operation not allowed<br/>1: Alarm A register write operation allowed                                                                                                                                                        |


## 17.4.5 ERTC divider register (ERTC_DIV)

| Bit        | Name     | Reset value | Type | Description                                                  |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------ |
| Bit 31: 23 | Reserved | 0x000       | resd | Kept at its default value.                                   |
| Bit 22: 16 | DIVA     | 0x7F        | rw   | Divider A                                                    |
| Bit 15     | Reserved | 0x0         | resd | Kept at its default value.                                   |
| Bit 14: 0  | DIVB     | 0x00FF      | rw   | Divider B<br/>Calendar clock = ERTC\_CLK/((DIVA+1)x(DIVB+1)) |


## 17.4.6 ERTC wakeup timer register (ERTC_WAT)

| Bit        | Name     | Reset value | Type | Description                |
| ---------- | -------- | ----------- | ---- | -------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value. |
| Bit 15: 0  | VAL      | 0xFFFF      | rw   | Wakeup timer reload value  |


## 17.4.7 ERTC coarse calibration register (ERTC_CCAL)

| Bit       | Name     | Reset value | Type | Description                                                                                      |
| --------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------ |
| Bit 31: 8 | Reserved | 0x000000    | resd | Kept at its default value.                                                                       |
| Bit 7     | CALDIR   | 0x0         | rw   | Calibration direction<br/>0: Positive calibration<br/>1: Negative calibration                    |
| Bit 6: 5  | Reserved | 0x0         | resd | Kept at its default value.                                                                       |
| Bit 4: 0  | CALVAL   | 0x00        | rw   | Calibration value<br/>Positive calibration<br/>00000: +0 ppm<br/>00001: +4 ppm<br/>00010: +8 ppm |


2025.05.28
Page 356
Rev 2.07





ARTERY logo AT32F435/437 Series Reference Manual

11111: +126 ppm
Negative calibration
00000: -0 ppm
00001: -2 ppm
00010: -4 ppm
...
11111: - 63 ppm

## 17.4.8 ERTC alarm clock A register (ERTC_ALA)

| Bit        | Name  | Reset value | Type | Description                                                                                                     |
| ---------- | ----- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------- |
| Bit 31     | MASK4 | 0x0         | rw   | Date/week day mask<br/>0: Date/week day is not masked<br/>1: Alarm clock doesn’t care about date/week day       |
| Bit 30     | WKSEL | 0x0         | rw   | Date/week day select<br/>0: Date<br/>1: Week day (DT\[1: 0] is not used)                                        |
| Bit 29: 28 | DT    | 0x0         | rw   | Date tens                                                                                                       |
| Bit 27: 24 | DU    | 0x0         | rw   | Date/week day units                                                                                             |
| Bit 23     | MASK3 | 0x0         | rw   | Hour mask<br/>0: No hour mask<br/>1: Alarm clock doesn’t care about hours                                       |
| Bit 22     | AMPM  | 0x0         | rw   | AM/PM<br/>0: AM<br/>1: PM<br/>Note: This bit is applicable for 12-hour format only. It is 0 for 24-hour format. |
| Bit 21: 20 | HT    | 0x0         | rw   | Hour tens                                                                                                       |
| Bit 19: 16 | HU    | 0x0         | rw   | Hour units                                                                                                      |
| Bit 15     | MASK2 | 0x0         | rw   | Minute mask<br/>0: No minute mask<br/>1: Alarm clock doesn’t care about minutes                                 |
| Bit 14: 12 | MT    | 0x0         | rw   | Minute tens                                                                                                     |
| Bit 11: 8  | MU    | 0x0         | rw   | Minute units                                                                                                    |
| Bit 7      | MASK1 | 0x0         | rw   | Second mask<br/>0: No second mask<br/>1: Alarm clock doesn’t care about seconds                                 |
| Bit 6: 4   | ST    | 0x0         | rw   | Second tens                                                                                                     |
| Bit 3: 0   | SU    | 0x0         | rw   | Second units                                                                                                    |


## 17.4.9 ERTC alarm clock B register (ERTC_ALB)

| Bit        | Name  | Reset value | Type | Description                                                                                                     |
| ---------- | ----- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------- |
| Bit 31     | MASK4 | 0x0         | rw   | Date/week day mask<br/>0: Date/week day is not masked<br/>1: Alarm clock doesn’t care about date/week day       |
| Bit 30     | WKSEL | 0x0         | rw   | Date/week day select<br/>0: Date<br/>1: Week day (DT\[1: 0] is not used)                                        |
| Bit 29: 28 | DT    | 0x0         | rw   | Date tens                                                                                                       |
| Bit 27: 24 | DU    | 0x0         | rw   | Date/week day units                                                                                             |
| Bit 23     | MASK3 | 0x0         | rw   | Hour mask<br/>0: No hour mask<br/>1: Alarm clock doesn’t care about hours                                       |
| Bit 22     | AMPM  | 0x0         | rw   | AM/PM<br/>0: AM<br/>1: PM<br/>Note: This bit is applicable for 12-hour format only. It is 0 for 24-hour format. |
| Bit 21: 20 | HT    | 0x0         | rw   | Hour tens                                                                                                       |
| Bit 19: 16 | HU    | 0x0         | rw   | Hour units                                                                                                      |
| Bit 15     | MASK2 | 0x0         | rw   | Minute mask<br/>0: No minute mask                                                                               |


2025.05.28 Page 357 Rev 2.07





ARTERY logo AT32F435/437 Series Reference Manual

| Bit 14: 12 | MT    | 0x0 | rw | 1: Alarm clock doesn’t care about minutes<br/>Minute tens                       |
| ---------- | ----- | --- | -- | ------------------------------------------------------------------------------- |
| Bit 11: 8  | MU    | 0x0 | rw | Minute units                                                                    |
| Bit 7      | MASK1 | 0x0 | rw | Second mask<br/>0: No second mask<br/>1: Alarm clock doesn’t care about seconds |
| Bit 6: 4   | ST    | 0x0 | rw | Second tens                                                                     |
| Bit 3: 0   | SU    | 0x0 | rw | Second units                                                                    |


## 17.4.10 ERTC write protection register (ERTC_WP)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                              |
| --------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 8 | Reserved | 0x000000    | resd | Kept at its default value                                                                                                                                |
| Bit 7: 0  | CMD      | 0x00        | wo   | Command register<br/>All ERTC register write protection is unlocked by writing 0xCA and 0x53. Writing any other value will re-activate write protection. |


## 17.4.11 ERTC subsecond register (ERTC_SBS)

| Bit        | Name     | Reset value | Type | Description                                                                                           |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value                                                                             |
| Bit 15: 0  | SBS      | 0x0000      | ro   | Sub-second value<br/>Subsecond is the value in the DIVB counter. Clock frequency = ERTC\_CLK/(DIVA+1) |


## 17.4.12 ERTC time adjustment register (ERTC_TADJ)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                              |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31     | ADD1S    | 0x0         | wo   | Add 1 second<br/>0: No effect<br/>1: Add one second<br/>This bit can be written only when TADJF=0. It is intended to be used with DECSBS in order to fine-tune the time. |
| Bit 30: 15 | Reserved | 0x0000      | resd | Kept at its default value                                                                                                                                                |
| Bit 14: 0  | DECSBS   | 0x0000      | wo   | DECSBS\[14: 0]: Decrease sub-second value<br/>Delay (ADD1S=0): Delay = DECSBS/(DIVB+1)<br/>Advance (ADD1S=1) : Advance =1-(DECSBS/(DIVB+1))                              |


## 17.4.13 ERTC time stamp time register (ERTC_TSTM)

| Bit        | Name     | Reset value | Type | Description                                                                                                     |
| ---------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------- |
| Bit 31: 23 | Reserved | 0x000       | resd | Kept at its default value                                                                                       |
| Bit 22     | AMPM     | 0x0         | ro   | AM/PM<br/>0: AM<br/>1: PM<br/>Note: This bit is applicable for 12-hour format only. It is 0 for 24-hour format. |
| Bit 21: 20 | HT       | 0x0         | ro   | Hour tens                                                                                                       |
| Bit 19: 16 | HU       | 0x0         | ro   | Hour units                                                                                                      |
| Bit 15     | Reserved | 0x0         | resd | Kept at its default value                                                                                       |
| Bit 14: 12 | MT       | 0x0         | ro   | Minute tens                                                                                                     |
| Bit 11: 8  | MU       | 0x0         | ro   | Minute units                                                                                                    |
| Bit 7      | Reserved | 0x0         | resd | Kept at its default value                                                                                       |
| Bit 6: 4   | ST       | 0x0         | ro   | Second tens                                                                                                     |
| Bit 3: 0   | SU       | 0x0         | ro   | Second units                                                                                                    |


\*Note: The content of this register is valid only when the TSF is set in the ERTC_STS register. It is cleared when TSF bit is reset.

2025.05.28 Page 358 Rev 2.07





ARTERY logo AT32F435/437 Series Reference Manual

# 17.4.14 ERTC time stamp date register (ERTC_TSDT)

| Bit        | Name     | Reset value | Type | Description               |
| ---------- | -------- | ----------- | ---- | ------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value |
| Bit 15: 13 | WK       | 0x0         | ro   | Week day                  |
| Bit 12     | MT       | 0x0         | ro   | Month tens                |
| Bit 11: 8  | MU       | 0x0         | ro   | Month units               |
| Bit 7: 6   | Reserved | 0x0         | resd | Kept at its default value |
| Bit 5: 4   | DT       | 0x0         | ro   | Date tens                 |
| Bit 3: 0   | DU       | 0x0         | ro   | Date units                |


Note: The content of this register is valid only when the TSF is set in the ERTC_STS register. It is cleared when TSF bit is reset.

# 17.4.15 ERTC time stamp subsecond register (ERTC_TSSBS)

| Bit        | Name     | Reset value | Type | Description               |
| ---------- | -------- | ----------- | ---- | ------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value |
| Bit 15: 0  | SBS      | 0x0000      | ro   | Sub-second value          |


Note: The content of this register is valid only when the TSF is set in the ERTC_STS register. It is cleared when TSF bit is reset.

# 17.4.16 ERTC smooth calibration register (ERTC_SCAL)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                       |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved | 0x0000      | resd | Kept at its default value                                                                                                                                                                                                                         |
| Bit 15     | ADD      | 0x0         | rw   | Add ERTC clock<br/>0: No ERTC clock added<br/>1: One ERTC\_CLK is inserted every 211 ERTC\_CLK cycles                                                                                                                                             |
| Bit 14     | CAL8     | 0x0         | rw   | 8-second calibration period<br/>0: No effect<br/>1: 8-second calibration                                                                                                                                                                          |
| Bit 13     | CAL16    | 0x0         | rw   | 16 second calibration period<br/>0: No effect<br/>1: 16-second calibration                                                                                                                                                                        |
| Bit 12: 9  | Reserved | 0x0         | resd | Kept at its default value                                                                                                                                                                                                                         |
| Bit 8: 0   | DEC      | 0x000       | rw   | Decrease ERTC clock<br/>DEC out of ERTC\_CLK cycles are masked during the 220 ERTC\_CLK periods. This bit is usually used with ADD. When the ADD is set, the actual number of ERTC\_CLK is equal to 220+512-DEC during the 220 ERTC\_CLK periods. |


# 17.4.17 ERTC tamper configuration register (ERTC_TAMP)

| Bit        | Name     | Reset value | Type | Description                                                            |
| ---------- | -------- | ----------- | ---- | ---------------------------------------------------------------------- |
| Bit 31: 19 | Reserved | 0x0000      | resd | Kept at its default value                                              |
| Bit 18     | OUTTYPE  | 0x0         | rw   | Output type<br/>0: Open-drain output<br/>1: Push-pull output           |
| Bit 17     | TSPIN    | 0x0         | rw   | Time stamp detection pin selection<br/>0: ERTC\_MUX1<br/>1: ERTC\_MUX2 |
| Bit 16     | TP1PIN   | 0x0         | rw   | Tamper detection pin selection<br/>0: ERTC\_MUX1<br/>1: ERTC\_MUX2     |
| Bit 15     | TPPU     | 0x0         | rw   | Tamper detection pull-up<br/>0: Tamper detection pull-up enabled       |


2025.05.28 Page 359 Rev 2.07




ARTERY logo

# AT32F435/437 Series Reference Manual

| Bit 14: 13 | TPPR     | 0x0 | rw   | 1: Tamper detection pull-up disabled<br/>Tamper detection pre-charge time<br/>0: 1 ERTC\_CLK cycle<br/>1: 2 ERTC\_CLK cycles<br/>2: 4 ERTC\_CLK cycles<br/>3: 8 ERTC\_CLK cycles                                |
| ---------- | -------- | --- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 12: 11 | TPFLT    | 0x0 | rw   | Tamper detection filter time<br/>0: No filter<br/>1: Tamper is detected after 2 consecutive samples<br/>2: Tamper is detected after 4 consecutive samples<br/>3: Tamper is detected after 8 consecutive samples |
| Bit 10: 8  | TPFREQ   | 0x0 | rw   | Tamper detection frequency<br/>0: ERTC\_CLK/32768<br/>1: ERTC\_CLK/16384<br/>2: ERTC\_CLK/8192<br/>3: ERTC\_CLK/4096<br/>4: ERTC\_CLK/2048<br/>5: ERTC\_CLK/1024<br/>6: ERTC\_CLK/512<br/>7: ERTC\_CLK/256      |
| Bit 7      | TPTSEN   | 0x0 | rw   | Tamper detection timestamp enable<br/>0: Tamper detection timestamp disabled<br/>1: Tamper detection timestamp enabled. Save timestamp on a tamper event.                                                       |
| Bit 6: 5   | Reserved | 0x0 | resd | Kept at its default value                                                                                                                                                                                       |
| Bit 4      | TP2EDG   | 0x0 | rw   | Tamper detection 2 valid edge<br/>If TPFLT=0:<br/>0: Rising edge<br/>1: Falling edge<br/>If TPFLT>0:<br/>0: Low<br/>1: High                                                                                     |
| Bit 3      | TP2EN    | 0x0 | rw   | Tamper detection 2 enable<br/>0: Tamper detection 2 disabled<br/>1: Tamper detection 2 enabled                                                                                                                  |
| Bit 2      | TPIEN    | 0x0 | rw   | Tamper detection interrupt enable<br/>0: Tamper detection interrupt disabled<br/>1: Tamper detection interrupt enabled                                                                                          |
| Bit 1      | TP1EDG   | 0x0 | rw   | Tamper detection 1 valid edge<br/>If TPFLT=0:<br/>0: Rising edge<br/>1: Falling edge<br/>If TPFLT>0:<br/>0: Low<br/>1: High                                                                                     |
| Bit 0      | TP1EN    | 0x0 | rw   | Tamper detection 1 enable<br/>0: Tamper detection 1 disabled<br/>1: Tamper detection 1 enabled                                                                                                                  |


2025.05.28
Page 360
Rev 2.07





Artery logo AT32F435/437 Series Reference Manual

### 17.4.18 ERTC alarm clock A subsecond register (ERTC_ALASBS)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                     |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 28 | Reserved | 0x0         | resd | Kept at its default value                                                                                                                                                                                                                       |
| Bit 27: 24 | SBSMSK   | 0x0         | rw   | Sub-second mask<br/>0: No comparison. Alarm A doesn’t care about subseconds.<br/>1: SBS\[0] is compared<br/>2: SBS\[1: 0] are compared<br/>3: SBS\[2: 0] are compared<br/>...<br/>14: SBS\[13: 0] are compared<br/>15: SBS\[14: 0] are compared |
| Bit 23: 15 | Reserved | 0x000       | rw   | Kept at its default value                                                                                                                                                                                                                       |
| Bit 14: 0  | SBS      | 0x0000      | rw   | Sub-second value                                                                                                                                                                                                                                |


### 17.4.19 ERTC alarm clock B subsecond register (ERTC_ALBSBS)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                     |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 28 | Reserved | 0x0         | resd | Kept at its default value                                                                                                                                                                                                                       |
| Bit 27: 24 | SBSMSK   | 0x0         | rw   | Sub-second mask<br/>0: No comparison. Alarm B doesn’t care about subseconds.<br/>1: SBS\[0] is compared<br/>2: SBS\[1: 0] are compared<br/>3: SBS\[2: 0] are compared<br/>...<br/>14: SBS\[13: 0] are compared<br/>15: SBS\[14: 0] are compared |
| Bit 23: 15 | Reserved | 0x000       | rw   | Kept at its default value                                                                                                                                                                                                                       |
| Bit 14: 0  | SBS      | 0x0000      | rw   | Sub-second value                                                                                                                                                                                                                                |


### 17.4.20 ERTC battery powered domain data register (ERTC_BPRx)

| Bit       | Name | Reset value | Type | Description                                                                                                                                                                                         |
| --------- | ---- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | DT   | 0x0000 0000 | rw   | Battery powered domain data<br/>BPR\_DTx registers are powered on by VBAT so that they are not reset by a system reset. They are reset on a tamper event or when a battery powered domain is reset. |


2025.05.28 Page 361 Rev 2.07
