#![no_std]
#![no_main]

use bsp::entry;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};



// https://doc.rust-lang.org/core/ffi/index.html
use core::ffi::{c_int, c_uint, c_void};

#[panic_handler]
fn oh_no(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}


mod c {
    use core::ffi::{c_int, c_uint, c_void};
    #[link(name = "blink", kind = "static")]
    extern "C" {
        pub(super) static LED_PIN: c_uint;

        // https://www.raspberrypi.com/documentation/pico-sdk/high_level.html
        pub(super) fn gpio_init(gpio: c_uint) -> c_void;
        pub(super) fn c_gpio_set_dir(gpio: c_uint, on_off: bool) -> c_void;
        pub(super) fn c_gpio_put(gpio: c_uint, on_off: bool) -> c_void;

        pub(super) fn sleep_us(us: u64) -> c_void;
        pub(super) fn sleep_ms(ms: u32) -> c_void;


        pub(super) fn c_main() -> c_void;
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


#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    //use embedded_hal::watchdog::WatchdogEnable;
    //watchdog.start(10);

    //rs_main();
    unsafe { c::c_main() };
    loop{}
}
