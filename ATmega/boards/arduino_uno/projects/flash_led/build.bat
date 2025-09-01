set TOOLCHAIN_DIR="C:\Program Files\WindowsApps\ArduinoLLC.ArduinoIDE_1.8.14.0_x86__mdqgnx93n4wtt\hardware\tools\avr\bin"
%TOOLCHAIN_DIR%\avr-gcc -Os -DF_CPU=16000000UL -mmcu=atmega328p -o flash_led.elf flash_led.c
if not errorlevel 1 (
    %TOOLCHAIN_DIR%\avr-objcopy -O ihex -R .eeprom flash_led.elf flash_led.hex
    %TOOLCHAIN_DIR%\avrdude -C %TOOLCHAIN_DIR%\..\etc\avrdude.conf -F -V -c arduino -p m328p -P COM3 -b 115200 -U flash:w:flash_led.hex
)