#![feature(asm, lang_items, naked_functions)]
#![no_std]
#![no_main]

#[lang = "panic_fmt"]
#[no_mangle]
fn rust_begin_panic(_msg: core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

fn pin_output(pin: u32) {
    const P_DIR: u8 = 0;
    const P_INPUT: u8 = 1;
    const P_PULL: u8 = 2;
    const P_DRIVE: u8 = 8;
    const P_SENSE: u8 = 16;

    unsafe {
        let cnf = (0x50000700 + 4 * pin) as *mut u32;
        *cnf = (1 << P_DIR) | (1 << P_INPUT) | (0 << P_PULL) | (0 << P_DRIVE) | (0 << P_SENSE);
    }
}

fn pin_set(pin: u32, state: bool) {
    let outclear = 0x5000050C as *mut u32;
    let outset = 0x50000508 as *mut u32;

    unsafe {
        if state {
            *outclear = 1 << pin;
        } else {
            *outset = 1 << pin;
        }
    }
}

fn burn() {
    let mut x: i32 = 0;
    for i in 0..100000 {
        x += i;
        x -= i;
    }
}

#[naked]
pub fn main() {
    let mut led_state = false;

    loop {
        pin_output(17);
        pin_output(19);
        pin_set(17, led_state);
        pin_set(19, led_state);
        burn();
        led_state = led_state ^ true;
    }
}

#[link_section = ".reset_handler"]
static RESET: fn() = main;
