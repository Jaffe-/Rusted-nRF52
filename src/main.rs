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

macro_rules! breakpoint {
    ($arg:expr) => (
        unsafe { asm!("BKPT $0" : : "i"($arg) : : "volatile") }
    )
}

extern {
    static __StackTop: u32;
}

fn pin_output() {
    const P_DIR: u8 = 0;
    const P_INPUT: u8 = 1;
    const P_PULL: u8 = 2;
    const P_DRIVE: u8 = 8;
    const P_SENSE: u8 = 16;

    let cnf = 0x50000744 as *mut u32;

    unsafe {
        *cnf = (1 << P_DIR) | (1 << P_INPUT) | (0 << P_PULL) | (0 << P_DRIVE) | (0 << P_SENSE);
    }
}

fn pin_outset() {
    let outset = 0x50000508 as *mut u32;

    unsafe {
        *outset = 1 << 17;
    }
}

#[naked]
pub fn main() {
    // Set up stack
    unsafe {
        asm!("mov sp, $0"::"r"(__StackTop)::"volatile");
    }

    pin_output();
    pin_outset();
    breakpoint!(1);
}

#[link_section = ".reset_handler"]
static RESET: fn() = main;
