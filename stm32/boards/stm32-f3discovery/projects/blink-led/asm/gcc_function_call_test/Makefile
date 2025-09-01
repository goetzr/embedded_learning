all:
	arm-none-eabi-gcc -mthumb -S test.c -o build/test.S
	arm-none-eabi-gcc -mthumb -c test.c -o build/test.o

.PHONY: clean
clean:
	rm -fr build/*