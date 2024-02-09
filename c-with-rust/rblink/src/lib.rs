#![no_std]

// https://doc.rust-lang.org/core/ffi/index.html
use core::ffi::{c_int, c_uint, c_void};

#[panic_handler]
fn oh_no(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod c {
    use core::ffi::{c_int, c_uint, c_void};
    extern "C" {
        pub(super) static LED_PIN: c_uint;

        // https://www.raspberrypi.com/documentation/pico-sdk/high_level.html
        pub(super) fn gpio_init(gpio: c_uint) -> c_void;
        pub(super) fn c_gpio_set_dir(gpio: c_uint, on_off: bool) -> c_void;
        pub(super) fn c_gpio_put(gpio: c_uint, on_off: bool) -> c_void;

        pub(super) fn sleep_us(us: u64) -> c_void;
        pub(super) fn sleep_ms(ms: u32) -> c_void;
    }
}

static LED_PIN: &c_uint = unsafe { &c::LED_PIN };

fn gpio_init(gpio: c_uint) {
    unsafe { c::gpio_init(gpio) };
}
fn gpio_set_dir(gpio: c_uint, on_off: bool) {
    unsafe { c::c_gpio_set_dir(gpio, on_off) };
}
fn gpio_put(gpio: c_uint, on_off: bool) {
    unsafe { c::c_gpio_put(gpio, on_off) };
}

fn sleep_us(us: u64) {
    unsafe { c::sleep_us(us) };
}
fn sleep_ms(ms: u32) {
    unsafe { c::sleep_ms(ms) };
}

// this is now the entry point
#[no_mangle]
pub extern "C" fn main() -> c_int {
    rs_main()
}

#[no_mangle]
pub extern "C" fn rs_main() -> c_int {
    let led_pin = *LED_PIN;
    gpio_init(led_pin);
    gpio_set_dir(led_pin, true);
    let mut sleep = 250;
    loop {
        gpio_put(led_pin, true);
        sleep_ms(sleep);
        gpio_put(led_pin, false);
        sleep_ms(sleep);
        sleep = add_half(sleep);
    }
}

#[no_mangle]
pub extern "C" fn add_half(num: u32) -> u32 {
    let mut half = num / 2;
    if half == 0 {
        half = 1;
    }
    num.wrapping_add(half)
}
