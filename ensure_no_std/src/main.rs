#![feature(start)]

#![deny(warnings)]

#![no_std]

#[cfg(windows)]
#[link(name="msvcrt")]
extern { }

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit_no_std::exit(99)
}

use macro_attr_2018::macro_attr;

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
