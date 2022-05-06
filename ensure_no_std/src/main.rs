#![feature(default_alloc_error_handler)]
#![feature(start)]

#![deny(warnings)]

#![no_std]

use core::alloc::Layout;
use core::panic::PanicInfo;
use macro_attr_2018::macro_attr;
#[cfg(not(windows))]
use libc::exit;
use libc_alloc::LibcAlloc;
#[cfg(windows)]
use winapi::shared::minwindef::UINT;
#[cfg(windows)]
use winapi::um::processthreadsapi::ExitProcess;

#[cfg(windows)]
#[link(name="msvcrt")]
extern { }

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[cfg(windows)]
unsafe fn exit(code: UINT) -> ! {
    ExitProcess(code);
    loop { }
}

#[panic_handler]
pub extern fn panic(_info: &PanicInfo) -> ! {
    unsafe { exit(99) }
}

#[no_mangle]
pub fn rust_oom(_layout: Layout) -> ! {
    unsafe { exit(98) }
}

trait TypeName {
    fn type_name() -> &'static str;
}

macro_rules! TypeName {
    (() $vis:vis enum $name:ident $($tail:tt)+) => { TypeName! { @impl $name } };
    (() $vis:vis struct $name:ident $($tail:tt)+) => { TypeName! { @impl $name } };
    (@impl $name:ident) => {
        impl TypeName for $name {
            fn type_name() -> &'static str { stringify!($name) }
        }
    };
}

macro_attr! {
    #[derive(TypeName!)]
    struct X;
}

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    assert_eq!(X::type_name(), "X");
    0
}
