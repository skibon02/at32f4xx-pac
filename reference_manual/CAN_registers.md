

# 20.7 CAN registers

These peripheral registers must be accessed by words (32 bits).

Table 20-1 CAN register map and reset values

| Register | Offset     | Reset value |
| -------- | ---------- | ----------- |
| MCTRL    | 000h       | 0x0001 0002 |
| MSTS     | 004h       | 0x0000 0C02 |
| TSTS     | 008h       | 0x1C00 0000 |
| RF0      | 00Ch       | 0x0000 0000 |
| FR1      | 010h       | 0x0000 0000 |
| INTEN    | 014h       | 0x0000 0000 |
| ESTS     | 018h       | 0x0000 0000 |
| BTMG     | 01Ch       | 0x0123 0000 |
| Reserved | 020h\~17Fh | xx          |
| TMI0     | 180h       | 0xXXXX XXXX |
| TMC0     | 184h       | 0xXXXX XXXX |
| TMDTL0   | 188h       | 0xXXXX XXXX |
| TMDTH0   | 18Ch       | 0xXXXX XXXX |
| TMI1     | 190h       | 0xXXXX XXXX |
| TMC1     | 194h       | 0xXXXX XXXX |
| TMDTL1   | 198h       | 0xXXXX XXXX |
| TMDTH1   | 19Ch       | 0xXXXX XXXX |
| TMI2     | 1A0h       | 0xXXXX XXXX |
| TMC2     | 1A4h       | 0xXXXX XXXX |
| TMDTL2   | 1A8h       | 0xXXXX XXXX |
| TMDTH2   | 1ACh       | 0xXXXX XXXX |
| RFI0     | 1B0h       | 0xXXXX XXXX |
| RFC0     | 1B4h       | 0xXXXX XXXX |
| RFDTL0   | 1B8h       | 0xXXXX XXXX |
| RFDTH0   | 1BCh       | 0xXXXX XXXX |
| RFI1     | 1C0h       | 0xXXXX XXXX |
| RFC1     | 1C4h       | 0xXXXX XXXX |
| RFDTL1   | 1C8h       | 0xXXXX XXXX |
| RFDTH1   | 1CCh       | 0xXXXX XXXX |
| Reserved | 1D0h\~1FFh | xx          |
| FCTRL    | 200h       | 0x2A1C 0E01 |
| FMCFG    | 204h       | 0x0000 0000 |
| Reserved | 208h       | xx          |
| FSCFG    | 20Ch       | 0x0000 0000 |
| Reserved | 210h       | xx          |
| FRF      | 214h       | 0x0000 0000 |
| Reserved | 218h       | xx          |
| FACFG    | 21Ch       | 0x0000 0000 |
| Reserved | 220h\~23Fh | xx          |
| FB0F1    | 240h       | 0xXXXX XXXX |
| FB0F2    | 244h       | 0xXXXX XXXX |
| FB1F1    | 248h       | 0xXXXX XXXX |
| FB1F2    | 24Ch       | 0xXXXX XXXX |
| ...      | ...        | ...         |
| FB27F1   | 318h       | 0xXXXX XXXX |
| FB27F2   | 31Ch       | 0xXXXX XXXX |


# 20.7.1 CAN control and status registers

## 20.7.1.1 CAN master control register (CAN_MCTRL)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ---------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 17 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Bit 16     | PTD      | 0x1         | rw   | Prohibit trans when debug<br/>0: Transmission works during debug<br/>1: Transmission is prohibited during debug. Receive FIFO can be still accessible normally.<br/>Note: Transmission can be disabled only when PTD and CANx\_PAUSE bits in the DEBUG\_CTRL register are set simultaneously.                                                                                                                                                                                     |
| Bit 15     | SPRST    | 0x0         | rw1s | Software partial reset<br/>0: Normal<br/>1: Software partial reset<br/>Note:<br/>SPRST only reset receive FIFO and MCTRL register.<br/>The CAN enters Sleep mode after reset. Then this bit is automatically cleared by hardware.                                                                                                                                                                                                                                                 |
| Bit 14: 8  | Reserved | 0x00        | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Bit 7      | TTCEN    | 0x0         | rw   | Time triggered communication mode enable<br/>0: Time triggered communication mode disabled<br/>1: Time triggered communication mode enabled                                                                                                                                                                                                                                                                                                                                       |
| Bit 6      | AEBOEN   | 0x0         | rw   | Automatic exit bus-off enable<br/>0: Automatic exit bus-off disabled<br/>1: Automatic exit bus-off enabled<br/>Note:<br/>When Automatic exit bus-off mode is enabled, the hardware will automatically leave bus-off mode as soon as an exit timing is detected on the CAN bus.<br/>When Automatic exit bus-off mode is disabled, the software must enter/leave the freeze mode once more, and then the bus-off state is left only when an exit timing is detected on the CAN bus. |
| Bit 5      | AEDEN    | 0x0         | rw   | Automatic exit doze mode enable<br/>0: Automatic exit sleep mode disabled<br/>1: Automatic exit sleep mode enabled<br/>Note:<br/>When Automatic exit sleep mode is disabled, the sleep mode is left by software clearing the sleep request command.<br/>When Automatic exit sleep mode is enabled, the sleep mode is left without the need of software intervention as soon as a message is monitored on the CAN bus.                                                             |
| Bit 4      | PRSFEN   | 0x0         | rw   | Prohibit retransmission enable when sending fails enable<br/>0: Retransmission is enabled.<br/>1: Retransmission is disabled.                                                                                                                                                                                                                                                                                                                                                     |
| Bit 3      | MDRSEL   | 0x0         | rw   | Message discard rule select when overflow<br/>0: The previous message is discarded.                                                                                                                                                                                                                                                                                                                                                                                               |
| Bit 2 | MMSSR | 0x0 | rw | 1: The new incoming message is discarded.<br/>Multiple message transmit sequence rule<br/>0: The message with the smallest identifier is first transmitted.<br/>1: The message with the first request order is first transmitted.                                                                                                                                                                                                                                                                                  |
| Bit 1 | DZEN  | 0x1 | rw | Doze mode enable<br/>0: Sleep mode is disabled.<br/>1: Sleep mode is enabled.<br/>Note:<br/>The hardware will automatically leave sleep mode when the AEDEN is set and a message is monitored on the CAN bus.<br/>After CAN reset or partial software reset, this bit is forced to be set by hardware, that is, the CAN will keep in sleep mode, by default.                                                                                                                                                       |
| Bit 0 | FZEN  | 0x0 | rw | Freeze mode enable<br/>0: Freeze mode disabled<br/>1: Freeze mode enabled<br/>Note:<br/>The CAN leaves Freeze mode once 11 consecutive recessive bits have been detected on the RX pin. For this reason, the software acknowledges the entry of Freeze mode after the FZC bit is cleared by hardware.<br/>The Freeze mode is entered only when the current CAN activity (transmission or reception) is completed. Thus the software acknowledges the exit of Freeze mode after the FZC bit is cleared by hardware. |


## 20.7.1.2 CAN master status register (CAN_MSTS)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                   |
| ---------- | -------- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 12 | Reserved | 0x00000     | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                    |
| Bit 11     | REALRX   | 0x1         | ro   | Real time level on RX pin<br/>0: Low<br/>1: High                                                                                                                                                                                                                                                                                                                              |
| Bit 10     | LSAMPRX  | 0x1         | ro   | Last sample level on RX pin<br/>0: Low<br/>1: High<br/>Note: This value keeps updating with the REALRX.                                                                                                                                                                                                                                                                       |
| Bit 9      | CURS     | 0x0         | ro   | Current receive status<br/>0: No reception occurs<br/>1: Reception is in progress<br/>Note: This bit is set by hardware when the CAN reception starts, and it is cleared by hardware at the end of reception.                                                                                                                                                                 |
| Bit 8      | CUSS     | 0x0         | ro   | Current transmit status<br/>0: No transmit occurs<br/>1: Transmit is in progress<br/>Note: This bit is set by hardware when the CAN transmission starts, and it is cleared by hardware at the end of transmission.                                                                                                                                                            |
| Bit 7: 5   | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                    |
| Bit 4      | EDZIF    | 0x0         | rw1c | Enter doze mode interrupt flag<br/>0: Sleep mode is not entered or no condition for flag set.<br/>1: Sleep mode is entered.<br/>Note:<br/>This bit is set by hardware only when EDZIEN=1 and the CAN enters Sleep mode. When set, this bit will generate a status change interrupt. This bit is cleared by software (writing 1 to itself) or by hardware when DZC is cleared. |
| Bit 3      | QDZIF    | 0x0         | rw1c | Exit doze mode interrupt flag<br/>0: Sleep mode is not left or no condition for exit.<br/>1: Sleep mode has been left or exit condition has generated.                                                                                                                                                                                                                        |
|       |      |     |      | Note:<br/>This bit is cleared by software (writing 1 to itself)<br/>Sleep mode is left when a SOF is detected on the bus.<br/>When QDZIEN=1, this bit will generate a status change<br/>interrupt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Bit 2 | EOIF | 0x0 | rw1c | Error occur interrupt flag<br/>0: No error interrupt or no condition for error interrupt flag<br/>1: Error interrupt is generated.<br/>Note:<br/>This bit is cleared by software (writing 1 to itself).<br/>This bit is set by hardware only when the corresponding<br/>bit is set in the CAN\_ESTS register and the corresponding<br/>interrupt enable bit in the CAN\_INTEN register is enabled.<br/>*When set, this bit will generate a status change interrupt.*                                                                                                                                                                                                                                                         |
| Bit 1 | DZC  | 0x1 | ro   | Doze mode acknowledge<br/>0: The CAN is not in Sleep mode.<br/>1: CAN is in Sleep mode.<br/>Note:<br/>This bit is used to decide whether the CAN is in Sleep<br/>mode or not. This bit acknowledges the Sleep mode<br/>request generated by software.<br/>The Sleep mode can be entered only when the current<br/>CAN activity (transmission or reception) is completed. For<br/>this reason, the software acknowledges the entry of Sleep<br/>mode after this bit is set by hardware.<br/>The Sleep mode is left only once 11 consecutive recessive<br/>bits have been detect on the CAN RX pin. For this reason,<br/>the software acknowledges the exit of Sleep mode after<br/>this bit is cleared by hardware.           |
| Bit 0 | FZC  | 0x0 | ro   | Freeze mode confirm<br/>0: The CAN is not in Freeze mode.<br/>1: The CAN is in Freeze mode.<br/>Note:<br/>This bit is used to decide whether the CAN is in Freeze<br/>mode or not. This bit acknowledges the Freeze mode<br/>request generated by software.<br/>The Freeze mode can be entered only when the current<br/>CAN activity (transmission or reception) is completed. For<br/>this reason, the software acknowledges the entry of<br/>Freeze mode after this bit is set by hardware.<br/>The Freeze mode is left only once 11 consecutive<br/>recessive bits have been detect on the CAN RX pin. For<br/>this reason, the software acknowledges the exit of Freeze<br/>mode after this bit is cleared by hardware. |


### 20.7.1.3 CAN transmit status register (CAN_TSTS)

| Bit    | Name   | Reset value | Type | Description                                                                                                                                                                                                                                  |
| ------ | ------ | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31 | TM2LPF | 0x0         | ro   | Transmit mailbox 2 lowest priority flag<br/>0: Mailbox 2 is not given the lowest priority.<br/>1: Lowest priority (This indicates that more than one<br/>mailboxes are pending for transmission, the mailbox 2 has<br/>the lowest priority.) |
| Bit 30 | TM1LPF | 0x0         | ro   | Transmit mailbox 1 lowest priority flag<br/>0: Mailbox 1 is not given the lowest priority.<br/>1: Lowest priority (This indicates that more than one<br/>mailboxes are pending for transmission, the mailbox 1 has<br/>the lowest priority.) |
| Bit 29 | TM0LPF | 0x0         | ro   | Transmit mailbox 0 lowest priority flag<br/>0: Mailbox 0 is not given the lowest priority.<br/>1: Lowest priority (This indicates that more than one<br/>mailboxes are pending for transmission, the mailbox 0 has<br/>the lowest priority.) |
| Bit 28 | TM2EF  | 0x1         | ro   | Transmit mailbox 2 empty flag<br/>*This bit is set by hardware when no transmission is*                                                                                                                                                      |
|            |          |     |      | pending in the mailbox 2.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Bit 27     | TM1EF    | 0x1 | ro   | Transmit mailbox 1 empty flag<br/>This bit is set by hardware when no transmission is pending in the mailbox 1.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Bit 26     | TM0EF    | 0x1 | ro   | Transmit mailbox 0 empty flag<br/>This bit is set by hardware when no transmission is pending in the mailbox 0.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Bit 25: 24 | TMNR     | 0x0 | ro   | Transmit Mailbox number record<br/>Note:<br/>If the transmit mailbox is free, these two bits refer to the number of the next transmit mailbox free.<br/>For example, in case of free CAN, the value of these two bit becomes 01 after a message transmit request is written.<br/>If the transmit box is full, these two bits refer to the number of the transmit mailbox with the lowest priority.<br/>For example, when there are three messages are pending for transmission, the identifiers of mailbox 0, mailbox 1 and mailbox 2 are 0x400, 0x433 and 0x411 respectively, and the value of these two bits becomes 01. |
| Bit 23     | TM2CT    | 0x0 | rw1c | Transmit mailbox 2 cancel transmit<br/>0: No effect<br/>1: Transmission is cancelled.<br/>Note: Software sets this bit to abort the transmission of mailbox 2. This bit is cleared by hardware when the transmit message in the mailbox 2 is cleared. Setting this bit has no effect if the mailbox 2 is free.                                                                                                                                                                                                                                                                                                             |
| Bit 22: 20 | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| Bit 19     | TM2TEF   | 0x0 | rw1c | Transmit mailbox 2 transmission error flag<br/>0: No error<br/>1: Mailbox 2 transmission error<br/>Note:<br/>This bit is set when the mailbox 2 transmission error occurred.<br/>It is cleared by software writing 1 or by hardware at the start of the next transmission                                                                                                                                                                                                                                                                                                                                                  |
| Bit 18     | TM2ALF   | 0x0 | rw1c | Transmit mailbox 2 arbitration lost flag<br/>0: No arbitration lost<br/>1: Transmit mailbox 2 arbitration lost<br/>Note:<br/>This bit is set when the mailbox 2 transmission failed due to an arbitration lost.<br/>It is cleared by software writing 1 or by hardware at the start of the next transmission                                                                                                                                                                                                                                                                                                               |
| Bit 17     | TM2TSF   | 0x0 | rw1c | Transmit mailbox 2 transmission success flag<br/>0: Transmission failed<br/>1: Transmission was successful.<br/>Note:<br/>This bit indicates whether the mailbox 2 transmission is successful or not. It is cleared by software writing 1.                                                                                                                                                                                                                                                                                                                                                                                 |
| Bit 16     | TM2TCF   | 0x0 | rw1c | Transmit mailbox 2 transmission completed flag<br/>0: Transmission is in progress<br/>1: Transmission is completed<br/>Note:<br/>This bit is set by hardware when the transmission/abort request on mailbox 2 has been completed.<br/>It is cleared by software writing 1 or by hardware when a new transmission request is received.<br/>Clearing this bit will clear the TM2TSF, TM2ALF and TM2TEF bits of mailbox 2.                                                                                                                                                                                                    |
| Bit 15     | TM1CT    | 0x0 | rw1s | Transmit mailbox 1 cancel transmit<br/>0: No effect<br/>1: Mailbox 1 cancel transmit<br/>Note: This bit is set by software to abort the transmission                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|            |          |     |      | request on mailbox 1. Clearing the message transmission on mailbox 1 will clear this bit. Setting by this software has no effect when the mailbox 1 is free.                                                                                                                                                                                                                                                            |
| Bit 14: 12 | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                              |
| Bit 11     | TM1TEF   | 0x0 | rw1c | Transmit mailbox 1 transmission error flag<br/>0: No error<br/>1: Mailbox 1 transmission error<br/>Note:<br/>This bit is set when the mailbox 1 transmission error occurred.<br/>It is cleared by software writing 1 or by hardware at the start of the next transmission                                                                                                                                               |
| Bit 10     | TM1ALF   | 0x0 | rw1c | Transmit mailbox 1 arbitration lost flag<br/>0: No arbitration lost<br/>1: Transmit mailbox 1 arbitration lost<br/>Note:<br/>This bit is set when the mailbox 1 transmission failed due to an arbitration lost.<br/>It is cleared by software writing 1 or by hardware at the start of the next transmission                                                                                                            |
| Bit 9      | TM1TSF   | 0x0 | rw1c | Transmit mailbox 1 transmission success flag<br/>0: Transmission failed<br/>1: Transmission was successful.<br/>Note:<br/>This bit indicates whether the mailbox 1 transmission is successful or not. It is cleared by software writing 1.                                                                                                                                                                              |
| Bit 8      | TM1TCF   | 0x0 | rw1c | Transmit mailbox 1 transmission completed flag<br/>0: Transmission is in progress<br/>1: Transmission is completed<br/>Note:<br/>This bit is set by hardware when the transmission/abort request on mailbox 1 has been completed.<br/>It is cleared by software writing 1 or by hardware when a new transmission request is received.<br/>Clearing this bit will clear the TM1TSF, TM1ALF and TM1TEF bits of mailbox 1. |
| Bit 7      | TM0CT    | 0x0 | rw1s | Transmit mailbox 0 cancel transmit<br/>0: No effect<br/>1: Mailbox 0 cancel transmit<br/>Note: This bit is set by software to abort the transmission request on mailbox 0. Clearing the message transmission on mailbox 0 will clear this bit. Setting by this software has no effect when the mailbox 0 is free.                                                                                                       |
| Bit 6: 4   | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                              |
| Bit 3      | TM0TEF   | 0x0 | rw1c | Transmit mailbox 0 transmission error flag<br/>0: No error<br/>1: Mailbox 0 transmission error<br/>Note:<br/>This bit is set when the mailbox 0 transmission error occurred.<br/>It is cleared by software writing 0 or by hardware at the start of the next transmission                                                                                                                                               |
| Bit 2      | TM0ALF   | 0x0 | rw1c | Transmit mailbox 0 arbitration lost flag<br/>0: No arbitration lost<br/>1: Transmit mailbox 0 arbitration lost<br/>Note:<br/>This bit is set when the mailbox 0 transmission failed due to an arbitration lost.<br/>It is cleared by software writing 1 or by hardware at the start of the next transmission                                                                                                            |
| Bit 1      | TM0TSF   | 0x0 | rw1c | Transmit mailbox 0 transmission success flag<br/>0: Transmission failed<br/>1: Transmission was successful.<br/>Note:                                                                                                                                                                                                                                                                                                   |
| Bit 0 | TM0TCF | 0x0         | rw1c | This bit indicates whether the mailbox 0 transmission is successful or not. It is cleared by software writing 1.<br/>Transmit mailbox 0 transmission completed flag<br/>0: Transmission is in progress<br/>1: Transmission is completed<br/>Note:<br/>This bit is set by hardware when the transmission/abort request on mailbox 0 has been completed.<br/>It is cleared by software writing 1 or by hardware when a new transmission request is received.<br/>Clearing this bit will clear the TM0TSF, TM0ALF and TM0TEF bits of mailbox 0. |


# 20.7.1.4 CAN receive FIFO 0 register (CAN_RF0)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                  |
| --------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 6 | Reserved | 0x0000000   | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                   |
| Bit 5     | RF0R     | 0x0         | rw1s | Receive FIFO 0 release<br/>0: No effect<br/>1: Release FIFO<br/>Note:<br/>This bit is set by software to release FIFO 0. It is cleared by hardware when the FIFO 0 is released.<br/>Setting this bit by software has no effect when the FIFO 0 is empty.<br/>If there are more than two messages pending in the FIFO 0, the software has to release the FIFO 0 to access the second message. |
| Bit 4     | RF0OF    | 0x0         | rw1c | Receive FIFO 0 overflow flag<br/>0: No overflow<br/>1: Receive FIFO 0 overflow<br/>Note:<br/>This bit is set by hardware when a new message has been received and passed the filter while the FIFO 0 is full.<br/>It is cleared by software by writing 1.                                                                                                                                    |
| Bit 3     | RF0FF    | 0x0         | rw1c | Receive FIFO 0 full flag<br/>0: Receive FIFO 0 is not full<br/>1: Receive FIFO 0 is full<br/>Note:<br/>This bit is set by hardware when three messages are pending in the FIFO 0.<br/>It is cleared by software by writing 1.                                                                                                                                                                |
| Bit 2     | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                   |
| Bit 1: 0  | RF0MN    | 0x0         | ro   | Receive FIFO 0 message num<br/>Note:<br/>These two bits indicate how many messages are pending in the FIFO 0.<br/>RF0ML bit is incremented by one each time a new message has been received and passed the filter while the FIFO 0 is not full.<br/>RF0ML bit is decremented by one each time the software releases the receive FIFO 0 by writing 1 to the RF0R bit.                         |


# 20.7.1.5 CAN receive FIFO 1 register (CAN_RF1)

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                                              |
| --------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 6 | Reserved | 0x0000000   | resd | Kept at its default value.                                                                                                                                                                                                                               |
| Bit 5     | RF1R     | 0x0         | rw1s | Receive FIFO 1 release<br/>0: No effect<br/>1: Release FIFO<br/>Note:<br/>This bit is set by software to release FIFO 1. It is cleared by hardware when the FIFO 1 is released.<br/>Setting this bit by software has no effect when the FIFO 1 is empty. |
|          |          |             |      | If there are more than two messages pending in the FIFO 0, the software has to release the FIFO 1 to access the second message.                                                                                                                                                                                                                                      |
| Bit 4    | RF1OF    | 0x0         | rw1c | Receive FIFO 1 overflow flag<br/>0: No overflow<br/>1: Receive FIFO 1 overflow<br/>Note:<br/>This bit is set by hardware when a new message has been received and passed the filter while the FIFO 1 is full.<br/>It is cleared by software by writing 1.                                                                                                            |
| Bit 3    | RF1FF    | 0x0         | rw1c | Receive FIFO 1 full flag<br/>0: Receive FIFO 1 is not full<br/>1: Receive FIFO 1 is full<br/>Note:<br/>This bit is set by hardware when three messages are pending in the FIFO 1.<br/>It is cleared by software by writing 1.                                                                                                                                        |
| Bit 2    | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                           |
| Bit 1: 0 | RF1MN    | 0x0         | ro   | Receive FIFO 1 message num<br/>Note:<br/>These two bits indicate how many messages are pending in the FIFO 1.<br/>RF1ML bit is incremented by one each time a new message has been received and passed the filter while the FIFO 1 is not full.<br/>RF1ML bit is decremented by one each time the software releases the receive FIFO 1 by writing 1 to the RF1R bit. |


## 20.7.1.6 CAN interrupt enable register (CAN_INTEN)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                       |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 18 | Reserved | 0x0000      | resd | Kept at its default value.                                                                                                                                                                                                                        |
| Bit 17     | EDZIEN   | 0x0         | rw   | Enter doze mode interrupt enable<br/>0: Enter sleep mode interrupt disabled<br/>1: Enter sleep mode interrupt enabled<br/>Note: EDZIF flag bit corresponds to this interrupt. An interrupt is generated when both this bit and EDZIF bit are set. |
| Bit 16     | QDZIEN   | 0x0         | rw   | Quit doze mode interrupt enable<br/>0: Quit sleep mode interrupt disabled<br/>1: Quit sleep mode interrupt enabled<br/>Note: The flag bit of this interrupt is the QDZIF bit. An interrupt is generated when both this bit and QDZIF bit are set. |
| Bit 15     | EOIEN    | 0x0         | rw   | Error occur interrupt enable<br/>0: Error interrupt disabled<br/>1: Error interrupt enabled<br/>Note: The flag bit of this interrupt is the EOIF bit. An interrupt is generated when both this bit and EOIF bit are set.                          |
| Bit 14: 12 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                        |
| Bit 11     | ETRIEN   | 0x0         | rw   | Error type record interrupt enable<br/>0: Error type record interrupt disabled<br/>1: Error type record interrupt enabled<br/>Note: EOIF is set only when this interrupt is enabled and the ETR\[2: 0] is set by hardware.                        |
| Bit 10     | BOIEN    | 0x0         | rw   | Bus-off interrupt enable<br/>0: Bus-off interrupt disabled<br/>1: Bus-off interrupt enabled<br/>Note: EOIF is set only when this interrupt is enabled and the BOF is set by hardware.                                                             |
| Bit 9      | EPIEN    | 0x0         | rw   | Error passive interrupt enable<br/>0: Error passive interrupt disabled<br/>1: Error passive interrupt enabled<br/>Note: EOIF is set only when this interrupt is enabled and                                                                       |
|       |          |     |      | the EPF is set by hardware.                                                                                                                                                                                                                                           |
| Bit 8 | EAIEN    | 0x0 | rw   | Error active interrupt enable<br/>0: Error warning interrupt disabled<br/>1: Error warning interrupt enabled<br/>Note: EOIF is set only when this interrupt is enabled and the EAF is set by hardware.                                                                |
| Bit 7 | Reserved | 0x0 | resd | Kept at its default value.                                                                                                                                                                                                                                            |
| Bit 6 | RF1OIEN  | 0x0 | rw   | Receive FIFO 1 overflow interrupt enable<br/>0: Receive FIFO 1 overflow interrupt disabled<br/>1: Receive FIFO 1 overflow interrupt enabled<br/>Note: The flag bit of this interrupt is the RF1OF bit. An interrupt is generated when this bit and RF1OF bit are set. |
| Bit 5 | RF1FIEN  | 0x0 | rw   | Receive FIFO 1 full interrupt enable<br/>0: Receive FIFO 1 full interrupt disabled<br/>1: Receive FIFO 1 full interrupt enabled<br/>Note: The flag bit of this interrupt is the RF1FF bit. An interrupt is generated when this bit and RF1FF bit are set.             |
| Bit 4 | RF1MIEN  | 0x0 | rw   | FIFO 1 receive message interrupt enable<br/>0: FIFO 1 receive message interrupt disabled<br/>1: FIFO 1 receive message interrupt enabled<br/>Note: The flag bit of this interrupt is RF1MN bit, so an interrupt is generated when this bit and RF1MN bit are set.     |
| Bit 3 | RF0OIEN  | 0x0 | rw   | Receive FIFO 0 overflow interrupt enable<br/>0: Receive FIFO 0 overflow interrupt disabled<br/>1: Receive FIFO 0 overflow interrupt enabled<br/>Note: The flag bit of this interrupt is RF0OF bit, so an interrupt is generated when this bit and RF0OF bit are set.  |
| Bit 2 | RF0FIEN  | 0x0 | rw   | Receive FIFO 0 full interrupt enable<br/>0: Receive FIFO 0 full interrupt disabled<br/>1: Receive FIFO 0 full interrupt enabled<br/>Note: The flag bit of this interrupt is the RF0FF bit. An interrupt is generated when this bit and RF0FF bit are set.             |
| Bit 1 | RF0MIEN  | 0x0 | rw   | FIFO 0 receive message interrupt enable<br/>0: FIFO 0 receive message interrupt disabled<br/>1: FIFO 0 receive message interrupt enabled<br/>Note: The flag bit of this interrupt is the RF0MN bit. An interrupt is generated when this bit and RF0MN bit are set.    |
| Bit 0 | TCIEN    | 0x0 | rw   | Transmit mailbox empty interrupt enable<br/>0: Transmit mailbox empty interrupt disabled<br/>1: Transmit mailbox empty interrupt enabled<br/>Note: The flag bit of this interrupt is the TMxTCF bit. An interrupt is generated when this bit and TMxTCF bit are set   |


# 20.7.1.7 CAN error status register (CAN_ESTS)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                               |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 24 | REC      | 0x00        | ro   | Receive error counter<br/>This counter is implemented in accordance with the receive part of the fault confinement mechanism of the CAN protocol.                                                                         |
| Bit 23: 16 | TEC      | 0x00        | ro   | Transmit error counter<br/>This counter is implemented in accordance with the transmit part of the fault confinement mechanism of the CAN protocol.                                                                       |
| Bit 15: 7  | Reserved | 0x00        | resd | Kept at its default value.                                                                                                                                                                                                |
| Bit 6: 4   | ETR      | 0x00        | rw   | Error type record<br/>000: No error<br/>001: Bit stuffing error<br/>010: Format error<br/>011: Acknowledgement error<br/>100: Recessive bit error<br/>101: Dominant bit error<br/>110: CRC error<br/>111: Set by software |
|       |          |             |      | Note:<br/>This field is used to indicate the current error type. It is set by hardware according to the error condition detected on the CAN bus. It is cleared by hardware when a message has been transmitted or received successfully.<br/>If the error code 7 is not used by hardware, this field can be set by software to monitor the code update. |
| Bit 3 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                              |
| Bit 2 | BOF      | 0x0         | ro   | Bus-off flag<br/>0: Bus-off state is not entered.<br/>1: Bus-off state is entered.<br/>Note: When the TEC is greater than 255, the bus-off state is entered, and this bit is set by hardware.                                                                                                                                                           |
| Bit 1 | EPF      | 0x0         | ro   | Error passive flag<br/>0: Error passive state is not entered<br/>1: Error passive state is entered<br/>Note: This bit is set by hardware when the current error times has reached the Error passive state limit (Receive Error Counter or Transmit Error Counter >127)                                                                                  |
| Bit 0 | EAF      | 0x0         | ro   | Error active flag<br/>0: Error active state is not entered<br/>1: Error active state is entered<br/>Note: This bit is set by hardware when the current error times has reached the Error active state limit (Receive Error Counter or Transmit Error Counter ≥96)                                                                                       |


## 20.7.1.8 CAN bit timing register (CAN_BTMG)

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                          |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31     | LOEN     | 0x0         | rw   | Listen-Only mode<br/>0: Listen-Only mode disabled<br/>1: Listen-Only mode enabled                                                                                                    |
| Bit 30     | LBEN     | 0x0         | rw   | Loop back mode<br/>0: Loop back mode disabled<br/>1: Loop back mode enabled                                                                                                          |
| Bit 29: 26 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                           |
| Bit 25: 24 | RSAW     | 0x1         | rw   | Resynchronization width<br/>tRSAW = tCAN x (RSAW\[1: 0] + 1)<br/>Note: This field defines the maximum of time unit that the CAN hardware is allowed to lengthen or shorten in a bit. |
| Bit 23     | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                           |
| Bit 22: 20 | BTS2     | 0x2         | rw   | Bit time segment 2<br/>tBTS2 = tCAN x (BTS2\[2: 0] + 1)<br/>Note: This field defines the number of time unit in Bit time segment 2.                                                  |
| Bit 19: 16 | BTS1     | 0x3         | rw   | Bit time segment 1<br/>tBTS1 = tCAN x (BTS1\[3: 0] + 1)<br/>Note: This field defines the number of time unit in Bit time segment 1.                                                  |
| Bit 15: 12 | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                                           |
| Bit 11: 0  | BRDIV    | 0x000       | rw   | Baud rate division<br/>tq = (BRDIV\[11: 0]+1) x tPCLK<br/>Note: This field defines the length of a time unit (tq).                                                                   |


## 20.7.2 CAN mailbox registers

This section describes the registers of the transmit and receive mailboxes. Refer to 20.6.5 for more information on register map.

Transmit and receive mailboxes are the same except:

* RFFMN field in the CAN_RFCx register
* A receive mailbox is read only
* A transmit mailbox can be written only when empty. TMxEF=1 in the CAN_TSTS register indicates that the mailbox is empty.


There are three transmit mailboxes and two receive mailboxes. Each receive mailbox has 3-level depth of FIFO, and can only access to the first received message in the FIFO.

Each mailbox contains four registers.

Figure 20-14 Transmit and receive mailboxes

Diagram of Transmit and receive mailboxes showing FIFO0, FIFO1, and Transmit mailboxes with their respective registers

## 20.7.2.1 Transmit mailbox identifier register (CAN_TMIx) (x=0..2)

Note: 1. This register is write protected when its mailboxes are pending for transmission.
2. This register implements the Transmit Request control (bit 0) — reset value 0.

| Bit        | Name         | Reset value | Type | Description                                                                                                                                                                           |
| ---------- | ------------ | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 21 | TMSID/ TMEID | 0xXXX       | rw   | Transmit mailbox standard identifier or extended identifier high bytes<br/>Note: This field defines the 11-bit high bytes of the standard identifier or extended identifier.          |
| Bit 20: 3  | TMEID        | 0xXXXXX     | rw   | Transmit mailbox extended identifier<br/>Note: This field defines the 18-bit low bytes of the extended identifier.                                                                    |
| Bit 2      | TMIDSEL      | 0xX         | rw   | Transmit mailbox identifier type select<br/>0: Standard identifier<br/>1: Extended identifier                                                                                         |
| Bit 1      | TMFRSEL      | 0xX         | rw   | Transmit mailbox frame type select<br/>0: Data frame<br/>1: Remote frame                                                                                                              |
| Bit 0      | TMSR         | 0x0         | rw   | Transmit mailbox send request<br/>0: No effect<br/>1: Transmit request<br/>Note: This bit is cleared by hardware when the transmission has been completed (The mailbox becomes empty) |


## 20.7.2.2 Transmit mailbox data length and time stamp register (CAN_TMCx) (x=0..2)

<u>All the bits in the register are write protected when</u> the mailbox is not in empty state.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                         |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | TMTS     | 0xXXXX      | rw   | Transmit mailbox time stamp<br/>Note: This field contains the value of the CAN timer sampled at the SOF transmission.                                                                                                                                                                                                                                               |
| Bit 15: 9  | Reserved | 0xXX        | resd | Kept at its default value                                                                                                                                                                                                                                                                                                                                           |
| Bit 8      | TMTSTEN  | 0xX         | rw   | Transmit mailbox time stamp transmit enable<br/>0: Time stamp is not sent<br/>1: Time stamp is sent<br/>Note:<br/>This bit is valid only when the time-triggered communication mode is enabled.<br/>In the time stamp MTS\[15: 0], the MTS\[7: 0] is stored in the TMDT7, and MTS\[15: 8] in the TMDT6. The data length must be programmed as 8 to send time stamp. |
| Bit 7: 4   | Reserved | 0xX         | resd | Kept at its default value                                                                                                                                                                                                                                                                                                                                           |
| Bit 3: 0   | TMDTBL   | 0xX         | rw   | Transmit mailbox data byte length<br/>Note: This field defines the data length of a transmit                                                                                                                                                                                                                                                                        |
message. A transmit message can contain from 0 to 8 data bytes.

# 20.7.2.3 Transmit mailbox data low register (CAN_TMDTLx) (x=0..2)

All the bits in the register are write protected when the mailbox is not in empty state.

| Bit        | Name  | Reset value | Type | Description                  |
| ---------- | ----- | ----------- | ---- | ---------------------------- |
| Bit 31: 24 | TMDT3 | 0xXX        | rw   | Transmit mailbox data byte 3 |
| Bit 23: 16 | TMDT2 | 0xXX        | rw   | Transmit mailbox data byte 2 |
| Bit 15: 8  | TMDT1 | 0xXX        | rw   | Transmit mailbox data byte 1 |
| Bit 7: 0   | TMDT0 | 0xXX        | rw   | Transmit mailbox data byte 0 |


# 20.7.2.4 Transmit mailbox data high register (CAN_TMDTHx) (x=0..2)

All the bits in the register are write protected when the mailbox is not in empty state.

| Bit        | Name  | Reset value | Type | Description                                                                                                                                                                                     |
| ---------- | ----- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 24 | TMDT7 | 0xXX        | rw   | Transmit mailbox data byte 7                                                                                                                                                                    |
| Bit 23: 16 | TMDT6 | 0xXX        | rw   | Transmit mailbox data byte 6<br/>Note: This field will be replaced with MTS\[15: 8] when the time-triggered communication mode is enabled and the corresponding time stamp transmit is enabled. |
| Bit 15: 8  | TMDT5 | 0xXX        | rw   | Transmit mailbox data byte 5                                                                                                                                                                    |
| Bit 7: 0   | TMDT4 | 0xXX        | rw   | Transmit mailbox data byte 4                                                                                                                                                                    |


# 20.7.2.5 Receive FIFO mailbox identifier register (CAN_RFIx) (x=0..1)

Note: All the receive mailbox registers are read-only.

| Bit        | Name        | Reset value | Type | Description                                                                                                                                                                |
| ---------- | ----------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 21 | RFSID/RFEID | 0xXXX       | ro   | Receive FIFO standard identifier or receive FIFO extended identifier<br/>Note: This field defines the 11-bit high bytes of the standard identifier or extended identifier. |
| Bit 20: 3  | RFEID       | 0xXXXXX     | ro   | Receive FIFO extended identifier<br/>Note: This field defines the 18-bit low bytes of the extended identifier.                                                             |
| Bit 2      | RFIDI       | 0xX         | ro   | Receive FIFO identifier type indication<br/>0: Standard identifier<br/>1: Extended identifier                                                                              |
| Bit 1      | RFFRI       | 0xX         | Ro   | Receive FIFO frame type indication<br/>0: Data frame<br/>1: Remote frame                                                                                                   |
| Bit 0      | Reserved    | 0x0         | resd | Kept at its default value                                                                                                                                                  |


# 20.7.2.6 Receive FIFO mailbox data length and time stamp register (CAN_RFCx) (x=0..1)

Note: All the receive mailbox registers are read-only.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                              |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | RFTS     | 0xXXXX      | ro   | Receive FIFO time stamp<br/>Note: This field contains the value of the CAN timer sampled at the start of a receive frame.                                                                                |
| Bit 15: 8  | RFFMN    | 0xXX        | ro   | Receive FIFO filter match number<br/>Note: This field contains the filter number that a message has passed through.                                                                                      |
| Bit 7: 4   | Reserved | 0xX         | resd | Kept at its default value                                                                                                                                                                                |
| Bit 3: 0   | RFDTL    | 0xX         | ro   | Receive FIFO data length<br/>Note: This field defines the data length of a receive message. A transmit message can contain from 0 to 8 data bytes. For a remote frame, its data length RFDTl is fixed 0. |


## 20.7.2.7 Receive FIFO mailbox data low register (CAN_RFDTLx) (x=0..1)

Note: All the receive mailbox registers are read-only.

| Bit        | Name  | Reset value | Type | Description              |
| ---------- | ----- | ----------- | ---- | ------------------------ |
| Bit 31: 24 | RFDT3 | 0xXX        | ro   | Receive FIFO data byte 3 |
| Bit 23: 16 | RFDT2 | 0xXX        | ro   | Receive FIFO data byte 2 |
| Bit 15: 8  | RFDT1 | 0xXX        | ro   | Receive FIFO data byte 1 |
| Bit 7: 0   | RFDT0 | 0xXX        | ro   | Receive FIFO data byte 0 |


## 20.7.2.8 Receive FIFO mailbox data high register (CAN_RFDTHx) (x=0..1)

Note: All the receive mailbox registers are read-only.

| Bit        | Name  | Reset value | Type | Description              |
| ---------- | ----- | ----------- | ---- | ------------------------ |
| Bit 31: 24 | RFDT7 | 0xXX        | ro   | Receive FIFO data byte 7 |
| Bit 23: 16 | RFDT6 | 0xXX        | ro   | Receive FIFO data byte 6 |
| Bit 15: 8  | RFDT5 | 0xXX        | ro   | Receive FIFO data byte 5 |
| Bit 7: 0   | RFDT4 | 0xXX        | ro   | Receive FIFO data byte 4 |


## 20.7.3 CAN filter registers

### 20.7.3.1 CAN filter control register (CAN_FCTRL)

Note: All the non-reserved bits of this register are controlled by software completely.

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                                                                                        |
| --------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 1 | Reserved | 0x150E0700  | resd | Kept at its default value                                                                                                                                                                                                          |
| Bit 0     | FCS      | 0x1         | rw   | Filter configuration switch<br/>0: Disabled (Filter bank is active)<br/>1: Enabled (Filter bank is in configuration mode)<br/>Note: The initialization of the filter bank can be configured only when it is in configuration mode. |


### 20.7.3.2 CAN filter mode configuration register (CAN_FMCFG)

Note: This register can be written only when FCS=1 in the CAN_FCTRL register (The filter is in configuration mode).

| Bit        | Name     | Reset value | Type | Description                                                                                                           |
| ---------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 28 | Reserved | 0x00000     | resd | Kept at its default value                                                                                             |
| Bit 27: 0  | FMSELx   | 0x0000      | rw   | Filter mode select<br/>Each bit corresponds to a filter bank.<br/>0: Identifier mask mode<br/>1: Identifier list mode |


### 20.7.3.3 CAN filter bit width configuration register (CAN_FBWCFG)

Note: This register can be written only when FCS=1 in the CAN_FCTRL register (The filter is in configuration mode).

| Bit        | Name     | Reset value | Type | Description                                                                                                |
| ---------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------- |
| Bit 31: 28 | Reserved | 0x00000     | resd | Kept at its default value                                                                                  |
| Bit 27: 0  | FBWSELx  | 0x0000      | rw   | Filter bit width select<br/>Each bit corresponds to a filter bank.<br/>0: Dual 16-bit<br/>1: Single 32-bit |


## 20.7.3.4 CAN filter FIFO association register (CAN_ FRF)

Note: This register can be written only when FCS=1 in the CAN_FCTRL register (The filter is in configuration mode).

| Bit        | Name     | Reset value | Type | Description                                                                                                                      |
| ---------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 28 | Reserved | 0x00000     | resd | Kept at its default value                                                                                                        |
| Bit 27: 0  | FRFSELx  | 0x0000      | rw   | Filter relation FIFO select<br/>Each bit corresponds to a filter bank.<br/>0: Associated with FIFO0<br/>1: Associated with FIFO1 |


## 20.7.3.5 CAN filter activation control register (CAN_ FACFG)

| Bit        | Name     | Reset value | Type | Description                                                                                    |
| ---------- | -------- | ----------- | ---- | ---------------------------------------------------------------------------------------------- |
| Bit 31: 28 | Reserved | 0x00000     | resd | Kept at its default value                                                                      |
| Bit 27: 0  | FAENx    | 0x0000      | rw   | Filter active enable<br/>Each bit corresponds to a filter bank.<br/>0: Disabled<br/>1: Enabled |


## 20.7.3.6 CAN filter bank i filter bit register (CAN_ FiFBx) (i=0..13; x=1..2)

Note: There are 14 filter banks (i=0..13). Each filter bank consists of two 32-bit registers, CAN_FiFB[2: 1]. This register can be modified only when the FAENx bit of the CAN_FACFG register is cleared or the FCS bit of the CAN_FCTRL register is set.

| Bit       | Name | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| --------- | ---- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | FFDB | 0xXXXX XXXX | rw   | Filters filter data bit<br/>Identifier list mode:<br/>The configuration value of the register matches with the level of the corresponding bit of the data received on the bus (If it is a standard frame, the value of the corresponding bit of the extended frame is neglected.)<br/>Identifier mark mode:<br/>Only the bit with its register configuration value 1 can match with the level of the corresponding bit of the data received on the bus. It don’t care when the register value is 0. |


