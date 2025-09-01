SRC_DIR := src
BUILD_DIR := build

asm-only: led.o delay.o main.o runtime.o vector_table.o
	arm-none-eabi-ld -o $(BUILD_DIR)/asm-only $(BUILD_DIR)/led.o $(BUILD_DIR)/delay.o $(BUILD_DIR)/main.o $(BUILD_DIR)/runtime.o $(BUILD_DIR)/vector_table.o -T link.ld

led.o: $(SRC_DIR)/led.S
	arm-none-eabi-as -g -mcpu=cortex-m4 -o $(BUILD_DIR)/led.o $(SRC_DIR)/led.S
delay.o: $(SRC_DIR)/delay.S
	arm-none-eabi-as -g -mcpu=cortex-m4 -o $(BUILD_DIR)/delay.o $(SRC_DIR)/delay.S
main.o: $(SRC_DIR)/main.S
	arm-none-eabi-as -g -mcpu=cortex-m4 -o $(BUILD_DIR)/main.o $(SRC_DIR)/main.S
runtime.o: $(SRC_DIR)/runtime.S
	arm-none-eabi-as -g -mcpu=cortex-m4 -o $(BUILD_DIR)/runtime.o $(SRC_DIR)/runtime.S
vector_table.o: $(SRC_DIR)/vector_table.S
	arm-none-eabi-as -g -mcpu=cortex-m4 -o $(BUILD_DIR)/vector_table.o $(SRC_DIR)/vector_table.S

.PHONY: clean
clean:
	rm -f $(BUILD_DIR)/*