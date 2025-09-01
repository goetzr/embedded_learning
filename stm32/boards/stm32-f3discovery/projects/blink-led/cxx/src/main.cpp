#include "led.h"
#include "delay.h"

static LedState led_state __attribute__ ((section (".data.main"))) = LedState::ON;

int main() __attribute__ ((section (".text.main")));
int main() {
    init_led();
    init_delay();

    while (true) {
        set_led(led_state);
        delay_ms(1000);
        led_state = toggle_led_state(led_state);
    }

    return 0;
}