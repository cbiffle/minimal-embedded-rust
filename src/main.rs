#![no_std]
#![feature(lang_items)]
#![feature(start)]

extern {
    /// This symbol is exported by the linker script, and defines the initial
    /// stack pointer.  Because we put it into the vector table --- which we
    /// model as an array of fn pointers --- we pun it as a function symbol
    /// here.  This is kind of a hack, but a reasonable one that I've done in
    /// C/C++ before too.
    fn __STACK_BASE();

    // TODO: model the ISRs as a proper struct and quit doing this.
}

#[start]
#[inline(never)]  // for inspection
pub fn start(_argc: isize, _argv: *const *const u8) -> isize {
    loop {}
}

unsafe extern "C" fn trap() { loop {} }

#[link_section=".isr_vector"]
pub static ISR_VECTORS: [Option<unsafe extern fn()>; 16] = [
    Some(__STACK_BASE),
    Some(trap),

    Some(trap),             // NMI
    Some(trap),             // Hard Fault
    Some(trap),             // MemMang
    Some(trap),             // BusFault
    Some(trap),             // UsageFault
    None,
    None,
    None,
    None,
    Some(trap),             // sv_call
    Some(trap),             // DebugMon
    None,
    Some(trap),             // PendSV
    Some(trap),             // SysTick
];

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {loop {}}

#[no_mangle]
pub extern "C" fn abort() -> ! { loop {} }

