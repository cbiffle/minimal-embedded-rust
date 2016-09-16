#![no_std]
#![feature(lang_items)]
#![feature(start)]

extern {
    /// This symbol is exported by the linker script, and defines the initial
    /// stack pointer.
    static __STACK_BASE: u32;

    /// We plug the compiler-generated main shim into the vector table as reset.
    /// This is technically wrong.  The shim expects to receive argc/argv in
    /// r0/r1, and the reset vector receives those registers uninitialized.  But
    /// since we ignore them anyway, it's safe in practice. (TODO)
    #[no_mangle]
    fn main(argc: isize, argv: *const *const u8) -> isize;
}

#[start]
#[inline(never)]  // for inspection
pub fn start(_: isize, _: *const *const u8) -> isize {
    loop {}
}

extern "C" fn trap() { loop {} }

type Handler = extern fn();

#[repr(C, packed)]
pub struct ExceptionTable {
    initial_stack: *const u32,
    reset: unsafe extern fn(argc: isize, argv: *const *const u8) -> isize,

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

#[link_section=".isr_vector"]
pub static ISR_VECTORS : ExceptionTable = ExceptionTable {
    initial_stack: unsafe { &__STACK_BASE },
    reset: main,

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

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {loop {}}

#[no_mangle]
pub extern "C" fn abort() -> ! { loop {} }

