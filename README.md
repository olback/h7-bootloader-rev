# [WIP] Arduino Portenta H7 Bootloader Reverse Engineering Project

## I²C Devices

| Address (7bit) | Device |
| -------------- | ------ |
| 0x08           | [NXP PF1550 PMIC](https://www.nxp.com/docs/en/data-sheet/PF1550.pdf) |
| 0x36           | Unknown? |
| 0x3f           | ANX7625** |
| 0x48           | [NXP SE050C2](https://www.nxp.com/docs/en/data-sheet/SE050-DATASHEET.pdf) |
| 0x60           | Microchip ATECC608A* |

\* No datasheet available to the general public.
\** Does not respond with ACK when writing data using [STM32h7xx-hal](github.com/stm32-rs/stm32h7xx-hal)?

## Boot I²C data

![i2c boot pmic data](/images/boot_i2c_pmic.png)

| Packet | Address (7bit) | R/W | Data | Description |
| ------ | -------------- | --- | ---- | ----------- |
| 1      | 0x08           | W   | 0x4f, 0x00 | LDO2_VOLT: 1.80V |
| 2      | 0x08           | W   | 0x50, 0x0f | LDO2_CTRL: VLDO2_EN = 1, VLDO2_STBY_EN = 1, VLDO2_OMODE = 1, VLDO2_LPWR = 1 |
| 3      | 0x08           | W   | 0x4c, 0x05 | LDO1_VOLT: 1.00V |
| 4      | 0x08           | W   | 0x4d, 0x03 | LDO1_CTRL: VLDO1_EN = 1, VLDO1_STBY_EN = 1 |
| 5      | 0x08           | W   | 0x52, 0x09 | LDO3_VOLT: 1.20V |
| 6      | 0x08           | W   | 0x53, 0x0f | LDO3_CTRL: VLDO3_EN = 1, VLDO3_STBY_EN = 1, VLDO3_OMODE = 1, VLDO3_LPWR = 1 |
| ...    | 0x08           | W   |            | There are 12 I²C write calls in the disassembled firmware, this list is incomplete. |
