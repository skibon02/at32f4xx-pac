

supplied.

The Standby mode is entered by the following procedures:

- Set the SLEEPDEE bit in the Cortex®-M4F system control register
- Set the LPSEL bit in the power control register (PWC_CTRL)
- Clear the SWEF bit in the power control/status register (PWC_CTRLSTS)
- Execute a WFI/WFE instruction

In Standby mode, all I/O pins remain in a high-impedance state except reset pins, TAMPER pins that are set as anti-tamper or calibration output, and the wakeup pins enabled.

The MCU leaves the Standby mode when an external reset (NRST pin), a WDT reset, ERTC periodic wakeup, ERTC timestamp, ERTC tamper event, a rising edge on the WKUP pin or the rising edge of an ERTC alarm event occurs.

**Debug mode**

By default, the debug connection is lost if the MCU enters Deepsleep mode or Standby mode while debugging. The reason is that the Cortex®-M4F core is no longer clocked. However, the software can be debugged even in the low-power mode by setting some configuration bits in the DEBUG register (DEBUG_CTRL).

# 3.7 PWC registers

The peripheral registers must be accessed by words (32 bits).

<u>Table 3-1 PWC register map and reset values</u>

| Register     | Offset | Reset value |
| ------------ | ------ | ----------- |
| PWC\_CTRL    | 0x00   | 0x0000 0000 |
| PWC\_CTRLSTS | 0x04   | 0x0000 0000 |
| PWC\_LDOOV   | 0x10   | 0x000X 0X00 |


## 3.7.1 Power control register (PWC_CTRL)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                                       |
| --------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 9 | Reserved | 0x000000    | resd | Kept at its default value.                                                                                                                                                                                                                        |
| Bit 8     | BPWEN    | 0x0         | rw   | Battery powered domain write enable<br/>0: Disabled<br/>1: Enabled<br/>Note:<br/>After reset, the battery powered domain write access is disabled. To write, this bit must be set.                                                                |
| Bit 7: 5  | PVMSEL   | 0x0         | rw   | Power voltage monitoring boundary select<br/>000: Unused, not configurable<br/>001: 2.3 V<br/>010: 2.4 V<br/>011: 2.5 V<br/>100: 2.6 V<br/>101: 2.7 V<br/>110: 2.8 V<br/>111: 2.9 V                                                               |
| Bit 4     | PVMEN    | 0x0         | rw   | Power voltage monitoring enable<br/>0: Disabled<br/>1: Enabled                                                                                                                                                                                    |
| Bit 3     | CLSEF    | 0x0         | wo   | Clear SEF flag<br/>0: No effect<br/>1: Clear the SEF flag<br/>Note: This bit is cleared by hardware after clearing the SEF flag. Reading this bit at any time will return all zero.                                                               |
| Bit 2     | CLSWEF   | 0x0         | wo   | Clear SWEF flag<br/>0: No effect<br/>1: Clear the SWEF flag<br/>Note:<br/>Clear the SWEF flag after two system clock cycles.<br/>This bit is cleared by hardware after clearing the SWEF flag. Reading this bit at any time will return all zero. |
| Bit 1     | LPSEL    | 0x0         | rw   | Low power mode select when Cortex®-M4F sleepdeep<br/>0: Enter DEEPSLEEP mode<br/>1: Enter Standby mode                                                                                                                                            |
| Bit 0     | VRSEL    | 0x0         | rw   | LDO state select in Deepsleep mode<br/>0: Enabled<br/>1: Low-power consumption mode                                                                                                                                                               |


## 3.7.2 Power control/status register (PWC_CTRLSTS)

Additional APB cycles are needed to read this register versus a standard APB read.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                  |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 10 | Reserved | 0x000000    | resd | Kept at its default value.                                                                                                                                                                                                                                   |
| Bit 9      | SWPEN2   | 0x0         | rw   | Standby wake-up pin2 enable<br/>0: Disabled (this pin is used for general-purpose I/O)<br/>1: Enabled (this pin is forced in input pull-down mode, and no longer used for general-purpose I/O)<br/>Note: This bit is cleared by hardware after system reset. |
| Bit 8      | SWPEN1   | 0x0         | rw   | Standby wake-up pin1 enable<br/>0: Disabled (this pin is used for general-purpose I/O)<br/>1: Enabled (this pin is forced in input pull-down mode, and no longer used for general-purpose I/O)<br/>Note: This bit is cleared by hardware after system reset. |
| Bit 7: 3   | Reserved | 0x00        | resd | Kept at its default value.                                                                                                                                                                                                                                   |
| Bit 2      | PVMOF    | 0x0         | ro   | Power voltage monitoring output flag<br/>0: Power voltage is higher than the threshold<br/>1: Power voltage is lower than the threshold<br/>Note: The power voltage monitor is stopped in Standby mode.                                                      |
| Bit 1      | SEF      | 0x0         | ro   | Standby mode entry flag<br/>0: Device is not in Standby mode                                                                                                                                                                                                 |
| Bit 0 | SWEF | 0x0         | ro   | 1: Device is in Standby mode<br/>Note: This bit is set by hardware (enter Standby mode) and cleared by POR/LVR or by setting the CLSEF bit.<br/>Standby wake-up event flag<br/>0: No wakeup event occurred<br/>1: A wakeup event occurred<br/>Note:<br/>This bit is set by hardware (on a wakeup event), and cleared by POR/LVR or by setting the CLSWEF bit.<br/>A wakeup event is generated by one of the following:<br/>When the rising edge on the Standby wakeup pin occurs;<br/>When the ERTC alarm event occurs;<br/>If the Standby wakeup pin is enabled when the Standby wakeup pin level is high. |


## 3.7.3 LDO output voltage select register (PWC_LDOOV)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                       |
| --------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 3 | Reserved | 0x0000 0000 | resd | Kept at its default value.                                                                                                                        |
| Bit 2: 0  | LDOOVSEL | 0x0         | rw   | LDO output voltage select<br/>000: 1.2V<br/>001: 1.3V<br/>010\~011: Unused, not configurable<br/>100: 1.1V<br/>101\~111: Unused, not configurable |


