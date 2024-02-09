/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include "pico/stdlib.h"

// https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html
// bool is compatible
#define i8  int8_t
#define i16 int16_t
#define i32 int32_t

#define u8  uint8_t
#define u16 uint16_t
#define u32 uint32_t

#define i64 int64_t
#define u64 uint64_t

#define isize intptr_t
#define usize uintptr_t


// used from rust
const uint LED_PIN = PICO_DEFAULT_LED_PIN;
uint led_pin() {
    return PICO_DEFAULT_LED_PIN;
}
// without this: undefined reference to `gpio_set_dir'
void c_gpio_set_dir(uint gpio, bool out) {
    return gpio_set_dir(gpio, out);
}
// without this: undefined reference to `gpio_put'
void c_gpio_put(uint gpio, bool value) {
    return gpio_put(gpio, value);
}

// defined in rust
u32 add_half(u32 num);
int rs_main();

int c_main() {
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);
    int sleep = 250;
    while (true) {
        gpio_put(LED_PIN, 1);
        sleep_ms(sleep);
        gpio_put(LED_PIN, 0);
        sleep_ms(sleep);
        sleep = add_half(sleep);
        if(sleep > 2000) {
            reset_usb_boot(LED_PIN, 0);
        }
    }
}

// rename to main() to enable this one
int main_disabled() {
    return rs_main();
    //return c_main();
}
