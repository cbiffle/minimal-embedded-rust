#![no_std]
#![feature(lang_items)]
#![feature(start)]

#[start]
pub fn start(_argc: isize, _argv: *const *const u8) -> isize {
    loop {}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {loop {}}
