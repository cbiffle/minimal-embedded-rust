#![no_std]
#![feature(lang_items)]

extern {
    /// This symbol is exported by the linker script, and defines the initial
    /// stack pointer.
    static __STACK_BASE: u32;
}

#[no_mangle]
pub unsafe extern fn reset_handler() -> ! {
    loop {}
}

extern "C" fn trap() { loop {} }

type Handler = extern fn();

#[repr(C, packed)]
pub struct ExceptionTable {
    initial_stack: *const u32,
    reset: unsafe extern fn() -> !,

    nmi: Option<Handler>,
    hard_fault: Option<Handler>,
    mm_fault: Option<Handler>,
    bus_fault: Option<Handler>,
    usage_fault: Option<Handler>,

    _reserved0: Option<Handler>,
    _reserved1: Option<Handler>,
    _reserved2: Option<Handler>,
    _reserved3: Option<Handler>,

    sv_call: Option<Handler>,
    debug_mon: Option<Handler>,

    _reserved4: Option<Handler>,

    pend_sv: Option<Handler>,
    sys_tick: Option<Handler>,
}

/// Const pointers are not inherently Sync.  We must be Sync to be static.
/// Sigh.
unsafe impl Sync for ExceptionTable {}

#[no_mangle]
#[link_section=".isr_vector"]
pub static ISR_VECTORS : ExceptionTable = ExceptionTable {
    initial_stack: unsafe { &__STACK_BASE },
    reset: reset_handler,

    nmi: Some(trap),
    hard_fault: Some(trap),
    mm_fault: Some(trap),
    bus_fault: Some(trap),
    usage_fault: Some(trap),
    _reserved0: None,
    _reserved1: None,
    _reserved2: None,
    _reserved3: None,
    sv_call: Some(trap),
    debug_mon: Some(trap),
    _reserved4: None,
    pend_sv: Some(trap),
    sys_tick: Some(trap),
};

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {loop {}}

#[no_mangle]
pub extern "C" fn abort() -> ! { loop {} }
