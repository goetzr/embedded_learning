SRC_DIR := src
BUILD_DIR := build

cxx-only: led.o delay.o main.o runtime.o vector_table.o
	arm-none-eabi-g++ -nostdlib -o $(BUILD_DIR)/cxx-only $(BUILD_DIR)/led.o $(BUILD_DIR)/delay.o $(BUILD_DIR)/main.o $(BUILD_DIR)/runtime.o $(BUILD_DIR)/vector_table.o -Wl,-Tlink.ld

led.o: $(SRC_DIR)/led.cpp
	arm-none-eabi-g++ -c -g -std=c++14 -nostdlib -mcpu=cortex-m4 -o $(BUILD_DIR)/led.o $(SRC_DIR)/led.cpp
delay.o: $(SRC_DIR)/delay.cpp
	arm-none-eabi-g++ -c -g -std=c++14 -nostdlib -mcpu=cortex-m4 -o $(BUILD_DIR)/delay.o $(SRC_DIR)/delay.cpp
main.o: $(SRC_DIR)/main.cpp
	arm-none-eabi-g++ -c -g -std=c++14 -nostdlib -mcpu=cortex-m4 -o $(BUILD_DIR)/main.o $(SRC_DIR)/main.cpp
runtime.o: $(SRC_DIR)/runtime.cpp
	arm-none-eabi-g++ -c -g -std=c++14 -nostdlib -mcpu=cortex-m4 -o $(BUILD_DIR)/runtime.o $(SRC_DIR)/runtime.cpp
vector_table.o: $(SRC_DIR)/vector_table.cpp
	arm-none-eabi-g++ -c -g -std=c++14 -nostdlib -mcpu=cortex-m4 -o $(BUILD_DIR)/vector_table.o $(SRC_DIR)/vector_table.cpp

test: led.o delay.o main.o runtime.o vector_table.o
	arm-none-eabi-g++ -Wl,--print-map -nostdlib -o $(BUILD_DIR)/cxx-only $(BUILD_DIR)/led.o $(BUILD_DIR)/delay.o $(BUILD_DIR)/main.o $(BUILD_DIR)/runtime.o $(BUILD_DIR)/vector_table.o -Wl,-Tlink.ld

.PHONY: clean
clean:
	rm -f $(BUILD_DIR)/*