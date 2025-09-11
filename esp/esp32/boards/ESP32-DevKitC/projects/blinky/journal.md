# ESP32-DevKitC Blinky Project Journal

### 6 Sep 2025
- Completed issue #2: Add Journal
- Working issue #3: Assemble components on breadboard
- Locating the power pin and determining the output voltage so I can determine the required resistance
    - 3V3 pin on DevKitC board is connected to 3V3 pin on the ESP32-WROOM-32 package
    - 3V3 pin on ESP32-WROOM-32 package is connected to VDD33 per peripheral schematic on pg. 34 of ESP32-WROOM-32 datasheet
    - Table 13 on pg. 26 of ESP32-WROOM-32 datasheet indicates that VDD33 is typically 3.3V
    - Recommended current through LED is 20 mA, so R = 3.3V / 0.02A = 165 ohm
- TODO:
    - Connect 3V3 pin on DevKitC to power rail on breadboard
    - Connect GND pin on DevKitC to GND rail on breadboard
    - Determine GPIO pin to connect LED to
    - Put LED and resistor in series on breadboard

### 7 Sep 2025
- Working issue #3: Assemble components on breadboard
- Reading the ESP32 documentation to determine which GPIO pin to connect the LED to
- Downloaded the ESP32 technical reference manual and added it to the repo
- In the Espressif online documentation I found the [Embedded Rust (no_std) on Espressif](https://docs.espressif.com/projects/rust/no_std-training/) HTML book
    - It has a blinky example
- Spending some time browsing through the TRM, trying to gain a broad understanding of the chip
- Table 3.3-6 on pg. 71 of the TRM indicates the GPIO registers are in the address range 0x3FF4_4000 - 0x3FF4_4FFF
- Reading Section 6.3 of TRM "Peripheral Output via GPIO Matrix"
- Section 6.3.3 "Simple GPIO Output" is exactly what I need:
    - To configure a pin as a simple GPIO output:
        - Set the GPIO_FUNCx_OUT_SEL field in the GPIO_FUNCx_OUT_SEL_CFG register to the special value 0x100
    - Then to drive the GPIO pin high/low, set bit x in the GPIO_OUT_DATA register
- The GPIO_FUNCx_OUT_SEL_CFG register is detailed on pg. 146 of the TRM
- Pins I see labeled on the ESP32-DevKitC board:
    - VIN, GND, 3V3, GND
    - D2, D4, D5, D12, D13, D14, D15, D18, D19, D21, D22, D23, D25, D26, D27, D32, D33, D34, D35
    - RX0, TX0, RX2, TX2
    - UN, UP, EN
- GPIO pins not labeled:
    - D0, D1, D3, D6, D7, D8, D9, D10, D11, D16, D17, D20, D24, D28, D29, D30, D31
- The specific model of the ESP32-DevKitC board I purchased, [AITRIP 3PCS Type c 30pins CP2102 ESP-WROOM-32 ESP32)(https://www.amazon.com/AITRIP-ESP-WROOM-32-Development-Microcontroller-Integrated/dp/B0CR5Y2JVD/ref=sr_1_3?crid=322MUJASZNK8R&dib=eyJ2IjoiMSJ9.UdLxS8engRob9RiEzo8Gffis-O1Rs2BEJTjG2jb7tqqGwDCNIqTsveMEEHCUzU-ywqA0KULpdX9ha2s_v4hyc9jPUU9SaFCFVWf2qNRVwndljeITy13b8XYyXYEbRW_sCUxwtXASY23KGGbbQzXepj9z_fmBFecjNhrX9DjVTgsaxIvPQCt_Pav9OheR0A_S-gH-1Lyw2-rzxfPhvEsc4Lclwdymcy6c0EIvNsEH7JQ.JGVXYN8KKSjLOVCcAUKIW4EdlNryX9tvxNFAw7vNYaQ&dib_tag=se&keywords=esp32%2Busb-c&qid=1757279090&sprefix=esp32%2Busb-c%2Caps%2C175&sr=8-3&th=1), only exposes 30 of the ESP32-WROOM-32 package's 38 pins. The ESP32-DevKitC board sold by Espressif exposes all 38 pins.
- Table 6.10-1 IO_MUX Pin Summary on pg. 132 of the TRM shows the function of each of the 34 I/O pins on the ESP32 processor
- The pin labels on the ESP32-DevKitC board seem to be arbitrary. My suspicion was that the default function (function 0 in Table 6.10-1) of each pin was the label shown, but this is not the case.
- Shared I/O pin functions:
    - GPIO1 (U0TXD), GPIO3 (U0RXD)
    - GPIO6 - GPIO11 (SD / SPI)
    - GPIO16 (U2RXD), GPIO17 (U2TXD)
    - Pin 20 is not an I/O pin
    - Pin 24 is not an I/O pin
    - Pins 28 - 31 are not I/O pins
- Given the shared I/O pin functions and the pins that are not I/O pins, the pins not exposed on the ESP32-DevKitC board are:
    - GPIO0
    - GPIO36 - GPIO39

### 9 Sep 2025
- Working issue #3: Assemble components on breadboard
- Working on a pin mappings table

#### I/O Pin Mapping

**NOTE:** GPIO20, GPIO24, and GPIO28 - GPIO31 don't exist.

| ESP32-DevKitC | ESP32-WROOM-32 | ESP32 GPIO | ESP32 Pin Name | ESP32 Function 0 |
|:-------------:|:--------------:|:----------:|:--------------:|:----------------:|
| NC            | IO0            | 0          | GPIO0          | GPIO0            |
| TX0           | TXD0           | 1          | U0TXD          | U0TXD            |
| D2            | IO2            | 2          | GPIO2          | GPIO2            |
| RX0           | RXD0           | 3          | U0RXD          | U0RXD            |
| D4            | IO4            | 4          | GPIO4          | GPIO4            |
| D5            | IO5            | 5          | GPIO5          | GPIO5            |
| NC            | CLK            | 6          | SD_CLK         | SD_CLK           |
| NC            | SD0            | 7          | SD_DATA_0      | SD_DATA_0        |
| NC            | SD1            | 8          | SD_DATA_1      | SD_DATA_1        |
| NC            | SD2            | 9          | SD_DATA_2      | SD_DATA_2        |
| NC            | SD3            | 10         | SD_DATA_3      | SD_DATA_3        |
| NC            | CMD            | 11         | SD_CMD         | SD_CMD           |
| D12           | IO12           | 12         | MTDI           | MTDI             |
| D13           | IO13           | 13         | MTCK           | MTCK             |
| D14           | IO14           | 14         | MTMS           | MTMS             |
| D15           | IO15           | 15         | MTD0           | MTD0             |
| RX2           | IO16           | 16         | GPIO16         | GPIO16           |
| TX2           | IO17           | 17         | GPIO17         | GPIO17           |
| D18           | IO18           | 18         | GPIO18         | GPIO18           |
| D19           | IO19           | 19         | GPIO19         | GPIO19           |
| D21           | IO21           | 21         | GPIO21         | GPIO21           |
| D22           | IO22           | 22         | GPIO22         | GPIO22           |
| D23           | IO23           | 23         | GPIO23         | GPIO23           |
| D25           | IO25           | 25         | GPIO25         | GPIO25           |
| D26           | IO26           | 26         | GPIO26         | GPIO26           |
| D27           | IO27           | 27         | GPIO27         | GPIO27           |
| D32           | IO32           | 32         | 32K_XP         | GPIO32           |
| D33           | IO33           | 33         | 32K_XN         | GPIO33           |
| D34           | IO34           | 34         | VDET_1         | GPIO34           |
| D35           | IO35           | 35         | VDET_2         | GPIO35           |
| VP            | SENSOR_VP      | 36         | SENSOR_VP      | GPIO36           |
| NC            | NC             | 37         | SENSOR_CAPP    | GPIO37           |
| NC            | NC             | 38         | SENSOR_CAPN    | GPIO38           |
| VN            | SENSOR_VN      | 39         | SENSOR_VN      | GPIO39           |

### 10 Sep 2025
- Working issue #3: Assemble components on breadboard
- Filling out the I/O pin mappings table
