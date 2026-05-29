

## 5.5.2 CRC calculation unit

CRC calculation can be performed on a sector level to check the security library code or user code inside bank 1 and bank 2.

* Generator polynomial: 0x4C11DB7,
that is, $X^{32} + X^{26} + X^{23} + X^{22} + X^{16} + X^{12} + X^{11} + X^{10} + X^8 + X^7 + X^5 + X^4 + X^2 + X + 1$
* CRC initial value: 0x00000000

Follow the procedure below:

* Check that no Flash memory operation is ongoing by checking the OBF bit in the FLASH_STS register
* Set the start sector and select the desired sector you wish to do CRC in the FLASH_CRC_CTRL register
* Enable CRC by setting the bit 31 in the FLASH_CRC_CTRL register
* Wait for the OBF bit to 0 (be cleared)
* Obtain CRC result by reading the FLASH_CRC_CHKR register

Note: Cross-boundary CRC check is forbidden for both the security library and user system data area.

## 5.6 Flash memory registers

Table 5-8 lists Flash register map and their reset values.

These peripheral registers must be accessed by words (32 bits).

<u>Table 5-8 Flash memory interface—Register map and reset value</u>

| Register           | Offset | Reset value |
| ------------------ | ------ | ----------- |
| FLASH\_PSR         | 0x00   | 0x0000 0330 |
| FLASH\_UNLOCK      | 0x04   | 0xXXXX XXXX |
| FLASH\_USD\_UNLOCK | 0x08   | 0xXXXX XXXX |
| FLASH\_STS         | 0x0C   | 0x0000 0000 |
| FLASH\_CTRL        | 0x10   | 0x0000 0080 |
| FLASH\_ADDR        | 0x14   | 0x0000 0000 |
| FLASH\_USD         | 0x1C   | 0x03FF FFFC |
| FLASH\_EPPS0       | 0x20   | 0xFFFF FFFF |
| FLASH\_EPPS1       | 0x2C   | 0xFFFF FFFF |
| FLASH\_UNLOCK2     | 0x44   | 0xXXXX XXXX |
| FLASH\_STS2        | 0x4C   | 0x0000 0000 |
| FLASH\_CTRL2       | 0x50   | 0x0000 0080 |
| FLASH\_ADDR2       | 0x54   | 0x0000 0000 |
| FLASH\_CONTR       | 0x58   | 0x0000 0080 |
| FLASH\_DIVR        | 0x60   | 0x0000 0022 |
| SLIB\_STS2         | 0xC8   | 0x0000 FFFF |
| SLIB\_STS0         | 0xCC   | 0x0000 0000 |
| SLIB\_STS1         | 0xD0   | 0xFFFF FFFF |
| SLIB\_PWD\_CLR     | 0xD4   | 0x0000 0000 |
| SLIB\_MISC\_STS    | 0xD8   | 0x0100 0000 |
| SLIB\_SET\_PWD     | 0xDC   | 0x0000 0000 |
| SLIB\_SET\_RANGE0  | 0xE0   | 0x0000 0000 |
| SLIB\_SET\_RANGE1  | 0xE4   | 0x0000 0000 |
| SLIB\_UNLOCK     | 0xF0 | 0x0000 0000 |
| FLASH\_CRC\_CTRL | 0xF4 | 0x0000 0000 |
| FLASH\_CRC\_CHKR | 0xF8 | 0x0000 0000 |


## 5.6.1 Flash performance select register (FLASH_PSR)

| Bit        | Name          | Reset value | Type | Description                                                                                                                                                                                                                                                                                                                                                                                               |
| ---------- | ------------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 14 | Reserved      | 0x00000     | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                |
| Bit 13     | NZW\_BST\_STS | 0x0         | ro   | Flash non-zero wait area boost status<br/>0: Flash non-zero wait area boost status disabled<br/>1: Flash non-zero wait area boost status enabled                                                                                                                                                                                                                                                          |
| Bit 12     | NZW\_BST      | 0x0         | rw   | Flash non-zero wait area boost<br/>0: Flash non-zero wait area boost disabled<br/>1: Flash non-zero wait area boost enabled<br/>Note:<br/>Enabling this feature will increase the operating efficiency of non-zero wait state Flash memory, but the system frequency will be limited. Refer to data sheet for more information.<br/>The setting code of this bit is located in the zero-wait state Flash. |
| Bit 11: 0  | Reserved      | 0x330       | resd | Kept at its default value.                                                                                                                                                                                                                                                                                                                                                                                |


## 5.6.2 Flash unlock register (FLASH_UNLOCK)

Only used in Flash memory bank 1.

| Bit       | Name  | Reset value | Type | Description                                                      |
| --------- | ----- | ----------- | ---- | ---------------------------------------------------------------- |
| Bit 31: 0 | UKVAL | 0xXXXX XXXX | wo   | Unlock key value<br/>This is used to unlock Flash memory bank 1. |


Note: All these bits are write-only, and return 0 when being read.

## 5.6.3 Flash user system data unlock register (FLASH_USD_UNLOCK)

| Bit       | Name       | Reset value | Type | Description                       |
| --------- | ---------- | ----------- | ---- | --------------------------------- |
| Bit 31: 0 | USD\_UKVAL | 0xXXXX XXXX | wo   | User system data Unlock key value |


Note: All these bits are write-only, and return 0 when being read.

## 5.6.4 Flash status register (FLASH_STS)

Only used in Flash memory bank 1.

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                     |
| --------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 6 | Reserved | 0x0000000   | resd | Kept at its default value                                                                                                                                       |
| Bit 5     | ODF      | 0x0         | rw   | Operation done flag<br/>This bit is set by hardware when Flash memory operations (program/erase) is completed. It is cleared by writing “1”.                    |
| Bit 4     | EPPERR   | 0x0         | rw   | Erase/program protection error<br/>This bit is set by hardware when programming the erase/program-protected Flash memory address. It is cleared by writing “1”. |
| Bit 3     | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                      |
| Bit 2     | PRGMERR  | 0x0         | rw   | Programming error<br/>When the programming address is in non-erase state, this bit is set by hardware. It is cleared by writing “1”.                            |
| Bit 1     | Reserved | 0x0         | resd | Kept at its default value.                                                                                                                                      |
| Bit 0     | OBF      | 0x0         | ro   | Operation busy flag<br/>When this bit is set, it indicates that Flash memory operation is in progress. It is cleared when operation is completed.               |


### 5.6.5 Flash control register (FLASH_CTRL)

Only used in Flash memory bank 1.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                     |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 13 | Reserved | 0x00000     | resd | Kept at its default value                                                                                                                                                                                                                                                       |
| Bit 12     | ODFIE    | 0x0         | rw   | Operation done flag interrupt enable<br/>0: Interrupt is disabled;<br/>1: Interrupt is enabled.                                                                                                                                                                                 |
| Bit 11,8   | Reserved | 0x0         | resd | Kept its default value                                                                                                                                                                                                                                                          |
| Bit 10     | ERRIE    | 0x0         | rw   | Error interrupt enable<br/>This bit enables EPPERR or PROGERR interrupt.<br/>0: Interrupt is disabled;<br/>1: Interrupt is enabled.                                                                                                                                             |
| Bit 9      | USDULKS  | 0x0         | rw   | User system data unlock success<br/>This bit is set by hardware when the user system data is unlocked properly, indicating that erase/program operation to the user system data is allowed. This bit is cleared by writing “0”, which will re-lock the user system data area.   |
| Bit 7      | OPLK     | 0x1         | rw   | Operation lock<br/>This bit is set by default, indicating that Flash memory is protected against operations. This bit is cleared by hardware after unlock, indicating that erase/program operation to Flash memory is allowed. Writing “1” can re-lock Flash memory operations. |
| Bit 6      | ERSTR    | 0x0         | rw   | Erase start<br/>An erase operation is triggered when this bit is set. This bit is cleared by hardware after the completion of the erase operation.                                                                                                                              |
| Bit 5      | USDERS   | 0x0         | rw   | User system data erase<br/>It indicates the user system data erase.                                                                                                                                                                                                             |
| Bit 4      | USDPRGM  | 0x0         | rw   | User system data program<br/>It indicates the user system data program.                                                                                                                                                                                                         |
| Bit 3      | BLKERS   | 0x0         | rw   | Block erase<br/>It indicates block erase operation.                                                                                                                                                                                                                             |
| Bit 2      | BANKERS  | 0x0         | rw   | Bank erase<br/>It indicates bank erase operation.                                                                                                                                                                                                                               |
| Bit 1      | SECERS   | 0x0         | rw   | Sector erase<br/>It indicates sector erase operation.                                                                                                                                                                                                                           |
| Bit 0      | FPRGM    | 0x0         | rw   | Flash program<br/>It indicates Flash program operation.                                                                                                                                                                                                                         |


### 5.6.6 Flash address register (FLASH_ADDR)

Only used in Flash memory bank 1.

| Bit       | Name | Reset value | Type | Description                                                              |
| --------- | ---- | ----------- | ---- | ------------------------------------------------------------------------ |
| Bit 31: 0 | FA   | 0x0000 0000 | wo   | Flash address<br/>Select the address of the blocks/sectors to be erased. |


### 5.6.7 User system data register (FLASH_USD)

| Bit        | Name     | Reset value | Type | Description                                                                                                     |
| ---------- | -------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------- |
| Bit 31: 26 | Reserved | 0x00        | resd | Kept at its default value                                                                                       |
| Bit 25: 18 | USER\_D1 | 0xFF        | ro   | User data 1                                                                                                     |
| Bit 17: 10 | USER\_D0 | 0xFF        | ro   | User data 0                                                                                                     |
| Bit 9: 2   | SSB      | 0xFF        | ro   | System setting byte<br/>Includes the system setting bytes in the loaded user system data area<br/>Bit 9: Unused |
|       |        |     |    | Bit 8: nSTDBY\_WDTBit 7: nDEPSLP\_WDTBit 6: UnusedBit 5: BTOPTBit 4: nSTDBY\_RSTBit 3: nDEPSLP\_RSTBit 2: nWDT\_ATO\_EN                                                                                                             |
| Bit 1 | FAP    | 0x0 | ro | Flash access protection<br/>Access to Flash memory is not allowed when this bit is set.                                                                                                                                             |
| Bit 0 | USDERR | 0x0 | ro | User system data error<br/>When this bit is set, it indicates that certain byte does not match its inverse code in the user system data area. At this point, this byte and its inverse code will be forced to 0xFF when being read. |


## 5.6.8 Erase/program protection status register 0 (FLASH_EPPS0)

| Bit       | Name | Reset value | Type | Description                                                                                                                         |
| --------- | ---- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | EPPS | 0xFFFF FFFF | ro   | Erase/Program protection status<br/>This register reflects the erase/program protection byte status in the loaded user system data. |


## 5.6.9 Erase/program protection status register 1 (FLASH_EPPS1)

| Bit       | Name | Reset value | Type | Description                                                                                                                         |
| --------- | ---- | ----------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | EPPS | 0xFFFF FFFF | ro   | Erase/Program protection status<br/>This register reflects the erase/program protection byte status in the loaded user system data. |


## 5.6.10 Flash unlock register 2 (FLASH_UNLOCK2)

Only used in Flash memory bank 2.

| Bit       | Name  | Reset value | Type | Description                                                               |
| --------- | ----- | ----------- | ---- | ------------------------------------------------------------------------- |
| Bit 31: 0 | UKVAL | 0xXXXX XXXX | wo   | Unlock key value<br/>This register is used to unlock Flash memory bank 2. |


Note: All these bits are write-only, and return 0 when being read.

## 5.6.11 Flash status register 2 (FLASH_STS2)

Only used in Flash memory bank 2.

| Bit       | Name     | Reset value | Type | Description                                                                                                                                                    |
| --------- | -------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 6 | Reserved | 0x0000000   | resd | Kept at its default value                                                                                                                                      |
| Bit 5     | ODF      | 0x0         | rw   | Operation done flag<br/>This bit is set by hardware when Flash memory operations (program/erase) is completed. It is cleared by writing “1”.                   |
| Bit 4     | EPPERR   | 0x0         | rw   | Erase/Program protection error<br/>This bit is set by hardware when programming the erase/program-protected Flash memory address. It is cleared by writing “1” |
| Bit 3     | Reserved | 0x0         | resd | Kept at its default value                                                                                                                                      |
| Bit 2     | PRGMERR  | 0x0         | rw   | Program error<br/>When the programming address is in non-erase state, this bit is set by hardware. It is cleared by writing “1”.                               |
| Bit 1     | Reserved | 0x0         | resd | Kept at its default value                                                                                                                                      |
| Bit 0     | OBF      | 0x0         | ro   | Operation busy flag<br/>When this bit is set, it indicates that Flash memory operation is in process. It is cleared when operation is completed.               |


# 5.6.12 Flash control register 2 (FLASH_CTRL2)

Only used in Flash memory bank 2.

| Bit        | Name     | Reset value | Type | Description                                                                                                                                                                                                                                                                     |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 13 | Reserved | 0x00000     | resd | Kept its default value                                                                                                                                                                                                                                                          |
| Bit 12     | ODFIE    | 0x0         | rw   | Operation done flag interrupt enable<br/>0: Interrupt is disabled;<br/>1: Interrupt is enabled.                                                                                                                                                                                 |
| Bit 11     | Reserved | 0x0         | resd | Kept its default value                                                                                                                                                                                                                                                          |
| Bit 10     | ERRIE    | 0x0         | rw   | Error interrupt enable<br/>This bit enables EPPERR or PROGERR interrupt.<br/>0: Interrupt is disabled;<br/>1: Interrupt is enabled.                                                                                                                                             |
| Bit 9,8    | Reserved | 0x0         | resd | Kept at its default value                                                                                                                                                                                                                                                       |
| Bit 7      | OPLK     | 0x1         | rw   | Operation lock<br/>This bit is set by default, indicating that Flash memory is protected against operations. This bit is cleared by hardware after unlock, indicating that erase/program operation to Flash memory is allowed. Writing “1” can re-lock Flash memory operations. |
| Bit 6      | ERSTR    | 0x0         | rw   | Erase start<br/>An erase operation is triggered when this bit is set. This bit is cleared by hardware after the completion of the erase operation.                                                                                                                              |
| Bit 5,4    | Reserved | 0x0         | resd | Kept its default value                                                                                                                                                                                                                                                          |
| Bit 3      | BLKERS   | 0x0         | rw   | Block erase<br/>It indicates block erase operation.                                                                                                                                                                                                                             |
| Bit 2      | BANKERS  | 0x0         | rw   | Bank erase<br/>It indicates bank erase operation.                                                                                                                                                                                                                               |
| Bit 1      | SECERS   | 0x0         | rw   | Sector erase<br/>It indicates sector erase operation.                                                                                                                                                                                                                           |
| Bit 0      | FPRGM    | 0x0         | rw   | Flash program<br/>It indicates Flash program operation.                                                                                                                                                                                                                         |


# 5.6.13 Flash address register 2 (FLASH_ADDR2)

Only used in Flash memory bank 2.

| Bit       | Name | Reset value | Type | Description                                                              |
| --------- | ---- | ----------- | ---- | ------------------------------------------------------------------------ |
| Bit 31: 0 | FA   | 0x0000 0000 | wo   | Flash address<br/>Select the address of the blocks/sectors to be erased. |


# 5.6.14 Flash continue read register (FLASH_CONTR)

| Bit       | Name       | Reset value | Type | Description                                                                                                                                                                                                                                               |
| --------- | ---------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | FCONTR\_EN | 0x0         | rw   | Flash continue read enable<br/>0: Flash continue read mode disabled<br/>1: Flash continue read mode enabled<br/>Setting this bit will enable the CPU to read Flash at a faster speed, but will also increase power consumption of Flash at the same time. |
| Bit 30: 0 | Reserved   | 0x0000 0080 | resd | Kept at its default value.                                                                                                                                                                                                                                |


# 5.6.15 Flash divider register (FLASH_DIVR)

| Bit       | Name      | Reset value | Type | Description                                                                                                                                                                                                                      |
| --------- | --------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 6 | Reserved  | 0x0000000   | resd | Kept at its default value.                                                                                                                                                                                                       |
| Bit 5: 4  | FDIV\_STS | 0x2         | ro   | Flash divider status<br/>This field indicates the division relationship between the current Flash interface and HCLK.<br/>0: HCLK/2<br/>1: HCLK/3<br/>Others: HCLK/4                                                             |
| Bit 3: 2  | Reserved  | 0x0         | resd | Kept at its default value.                                                                                                                                                                                                       |
| Bit 1: 0  | FDIV      | 0x2         | rw   | Flash divider<br/>This field is used to configure the division relationship between the Flash interface and HCLK.<br/>0: HCLK/2<br/>1: HCLK/3<br/>Others: HCLK/4<br/>Note: The Flash interface clock is not greater than 104MHz. |


# 5.6.16 Flash security library status register 2 (SLIB_STS2)

Only used in Flash security library.

| Bit        | Name           | Reset value | Type | Description                                                                                                            |
| ---------- | -------------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 16 | Reserved       | 0x0000      | resd | Kept at its default value                                                                                              |
| Bit 15: 0  | SLIB\_INST\_SS | 0xFFFF      | ro   | sLib instruction start sector<br/>0: Sector 0<br/>1: Sector 1<br/>2: Sector 2<br/>...<br/>0xFFFF: Non-sLib instruction |


# 5.6.17 Flash security library status register 0 (SLIB_STS0)

Only used in Flash security library.

| Bit       | Name      | Reset value | Type | Description                                                                                                                                                                                      |
| --------- | --------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 4 | Reserved  | 0x0000000   | resd | Kept at its default value                                                                                                                                                                        |
| Bit 3     | SLIB\_ENF | 0x0         | ro   | SLIB\_ENF: sLib enable flag<br/>When this bit is set, it indicates that the main Flash memory is partially or completely (depending on the setting of SLIB\_STS1) used as security library code. |
| Bit 2: 0  | Reserved  | 0x0         | resd | Kept at its default value                                                                                                                                                                        |


# 5.6.18 Flash security library status register 1 (SLIB_STS1)

Only used in Flash security library.

| Bit        | Name     | Reset value | Type | Description                                                                           |
| ---------- | -------- | ----------- | ---- | ------------------------------------------------------------------------------------- |
| Bit 31: 16 | SLIB\_ES | 0xFFFF      | ro   | Security library end sector<br/>0: Sector 0<br/>1: Sector 1<br/>2: Sector 2<br/>...   |
| Bit 15: 0  | SLIB\_SS | 0xFFFF      | ro   | Security library start sector<br/>0: Sector 0<br/>1: Sector 1<br/>2: Sector 2<br/>... |


# 5.6.19 Flash security library password clear register (SLIB_PWD_CLR)

<u>Only used</u> in Flash security library.

| Bit      | Name            | Reset value | Type | Description                                                                                                                                                                                                                             |
| -------- | --------------- | ----------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31:0 | SLIB\_PCLR\_VAL | 0x0000 0000 | wo   | Security library password clear value<br/>Entering correct security library password will unlock security library functions.<br/>The write status of this register is reflected in the bit 0 and bit 1 in the SLIB\_MISC\_STS register. |


# 5.6.20 Security library additional status register (SLIB_MISC_STS)

<u>Only used</u> in Flash security library.

| Bit        | Name           | Reset value | Type | Description                                                                                                                                                                                                                                                                                                          |
| ---------- | -------------- | ----------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31:25  | Reserved       | 0x00        | resd | Kept at its default value                                                                                                                                                                                                                                                                                            |
| Bit 24: 16 | SLIB\_RCNT     | 0x100       | ro   | Security library remaining count<br/>It is decremented from 256 to 0.                                                                                                                                                                                                                                                |
| Bit 15: 3  | Reserved       | 0x0000      | resd | Kept at its default value                                                                                                                                                                                                                                                                                            |
| Bit 2      | SLIB\_ULKF     | 0x0         | ro   | Security library unlock flag<br/>When this bit is set, it indicates that sLib-related setting registers can be configured.                                                                                                                                                                                           |
| Bit 1      | SLIB\_PWD\_OK  | 0x0         | ro   | Security library password ok<br/>This bit is set by hardware when the password is correct.                                                                                                                                                                                                                           |
| Bit 0      | SLIB\_PWD\_ERR | 0x0         | ro   | Security library password error<br/>This bit is set by hardware when the password is incorrect and the setting value of the password clear register is different from 0xFFFF FFFF.<br/>Note: When this bit is set, the hardware will no longer agree to re-program the password clear register until the next reset. |


# 5.6.21 Security library password setting register (SLIB_SET_PWD)

<u>Only used</u> in Flash security library.

| Bit       | Name            | Reset value | Type | Description                                                                                                                                                                                                                                    |
| --------- | --------------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31: 0 | SLIB\_PSET\_VAL | 0x0000 0000 | wo   | Security library password setting value<br/>Note: This register can be written only after unlocking security library lock. It is used to set up the startup password of security library. Values of 0xFFFF\_FFFF and 0x0000\_0000 are invalid. |


Note: All these bits are write-only, and return 0 when being read.

# 5.6.22 Security library address setting register0 (SLIB_SET_RANGE0)

<u>Only used</u> for Flash security library address <u>setting.</u>

| Bit        | Name          | Reset value | Type | Description                                                                                                                                                  |
| ---------- | ------------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 16 | SLIB\_ES\_SET | 0x000       | wo   | Security library end sector setting<br/>Theses bits are used to set the security library end sector.<br/>0: Sector 0<br/>1: Sector 1<br/>2: Sector 2<br/>... |
| Bit 10: 0  | SLIB\_SS\_SET | 0x000       | wo   | Security library start sector setting<br/>*These bits are used to set the security library start sector.*                                                    |


0: Sector 0
1: Sector 1
2: Sector 2
...
Note: All these bits are write-only, and return 0 when being read.
(1) This register can be written only after unlocking security library lock.
(2) Being out of the Flash address range is an invalid setting.

## 5.6.23 Security library address setting register 1 (SLIB_SET_RANGE1)

Only used for Flash security library address setting.

| Bit        | Name            | Reset value | Type | Description                                                                                                                                                                                                              |
| ---------- | --------------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Bit 31     | SET\_SLIB\_STRT | 0x0         | wo   | Setting sLib start<br/>This bit is used to enable security library information settings.<br/>It is cleared by hardware automatically when the security library configuration is enabled by hardware.                     |
| Bit 30: 16 | Reserved        | 0x0000      | resd | Kept at its default value                                                                                                                                                                                                |
| Bit 15: 0  | SLIB\_ISS\_SET  | 0x0000      | wo   | Security library instruction start sector setting<br/>These bits are used to set the security library instruction start sector.<br/>0: Sector 0<br/>1: Sector 1<br/>2: Sector 2<br/>...<br/>0xFFFF: Non-sLib instruction |


## 5.6.24 Security library unlock register (SLIB_UNLOCK)

Only used for Flash security library unlock setting.

| Bit       | Name        | Reset value | Type | Description                                                                                                              |
| --------- | ----------- | ----------- | ---- | ------------------------------------------------------------------------------------------------------------------------ |
| Bit 31: 0 | SLIB\_UKVAL | 0x0000 0000 | wo   | Security library unlock key value<br/>Fixed key value is 0xA35F\_6D24, used for security library setting register unlock |


Note: All these bits are write-only, and return 0 when being read.

## 5.6.25 Flash CRC calibration control register (FLASH_CRC_CTRL)

Only used in main Flash memory.

| Bit        | Name      | Reset value | Type | Description                                                                                                                                          |
| ---------- | --------- | ----------- | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bit 31     | CRC\_STRT | 0x0         | wo   | CRC start<br/>Set this bit to enable user code or security library code CRC check. This bit is cleared automatically after the hardware enables CRC. |
| Bit 30: 24 | Reserved  | 0x00        | wo   | Kept at its default value                                                                                                                            |
| Bit 23: 12 | CRC\_SN   | 0x000       | wo   | CRC sector number<br/>Set the number of the CRC calibration, in terms of sectors.                                                                    |
| Bit 11: 0  | CRC\_SS   | 0x000       | wo   | CRC calibration start sector<br/>Set the start sector for this CRC calibration.<br/>0x0: Sector 0<br/>0x1: Sector 1<br/>...                          |


Note: All these bits are write-only, and return 0 when being read.


# 5.6.26 Flash CRC check result register (FLASH_CRC_CHKR)

Only used in Flash or security library.

| Bit       | Name      | Reset value | Type | Description      |
| --------- | --------- | ----------- | ---- | ---------------- |
| Bit 31: 0 | CRC\_CHKR | 0x0000 0000 | ro   | CRC check result |


*Note: All these bits are read-only, and return 0 when being written.*

