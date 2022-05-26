// Copyright (c) 2016 macro-attr contributors.
// Copyright (c) 2020 Warlock <internalmike@gmail.com>.
// Copyright (c) 2020 Clint Armstrong <clint@clintarmstrong.net>.
//
// Licensed under the MIT license (see LICENSE or <http://opensource.org
// /licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
// <http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
// files in the project carrying such notice may not be copied, modified,
// or distributed except according to those terms.

#![deny(warnings)]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]
#![doc(test(attr(allow(unused_macros))))]
#![doc(test(attr(allow(unknown_lints, unused_macro_rules))))]

//! This crate provides the `macro_attr!` macro that enables the use of custom,
//! macro-based attributes and derivations.
//!
//! The `macro_attr!` macro should be used to wrap an entire *single* item
//! (`enum`, `struct`, *etc.*) declaration, including its attributes (both `derive` and others).
//! All attributes and derivations which whose names end with `!` will be assumed
//! to be implemented by macros, and treated accordingly.
//!
//! ```rust
//! use macro_attr_2018::macro_attr;
//!
//! // Define some traits to be derived.
//!
//! trait TypeName {
//!     fn type_name() -> &'static str;
//! }
//!
//! trait ReprType {
//!     type Repr;
//! }
//!
//! // Define macros which derive implementations of these macros.
//!
//! macro_rules! TypeName {
//!     // We can support any kind of item we want.
//!     (() $vis:vis enum $name:ident $($tail:tt)+) => { TypeName! { @impl $name } };
//!     (() $vis:vis struct $name:ident $($tail:tt)+) => { TypeName! { @impl $name } };
//!
//!     // Inner rule to cut down on repetition.
//!     (@impl $name:ident) => {
//!         impl TypeName for $name {
//!             fn type_name() -> &'static str { stringify!($name) }
//!         }
//!     };
//! }
//!
//! macro_rules! ReprType {
//!     // Note that we use a "derivation argument" here for the `$repr` type.
//!     (($repr:ty) $vis:vis enum $name:ident $($tail:tt)+) => {
//!         impl ReprType for $name {
//!             type Repr = $repr;
//!         }
//!     };
//! }
//!
//! // Derive.
//!
//! macro_attr! {
//!     #[derive(TypeName!, ReprType!(u16))]
//!     #[repr(u16)]
//!     enum SomeEnum { A, B, C, D }
//! }
//!
//! # fn main() {
//! assert_eq!(SomeEnum::type_name(), "SomeEnum");
//! assert_eq!(SomeEnum::A as <SomeEnum as ReprType>::Repr, 0u16);
//! # }
//! ```

#![no_std]

#[doc=include_str!("../README.md")]
type _DocTestReadme = ();

/// When given an item definition, including its attributes, this macro parses said attributes
/// and dispatches any derivations suffixed with `!` to user-defined macros.
/// This allows multiple macros to process the same item.
///
/// Given the following input:
///
/// ```ignore
/// #[derive(Copy, Name!(args...), Clone, Another!, Debug)]
/// struct Foo;
/// ```
///
/// `macro_attr!` will expand to the equivalent of:
///
/// ```ignore
/// #[derive(Copy, Clone, Debug)]
/// struct Foo;
///
/// Name!((args...) struct Foo;);
/// Another!(() struct Foo;);
/// ```
///
/// Note that macro derives may be mixed with regular derives,
/// or put in their own `#[derive(...)]` attribute.
/// Also note that macro derive invocations are *not* passed the other attributes on the item;
/// input will consist of the arguments provided to the derivation (*i.e.* `(args...)`
/// in this example), the item's visibility (if any), and the item definition itself.
///
/// A macro derivation invoked *without* arguments will be treated as though
/// it was invoked with empty parentheses.  *i.e.* `#[derive(Name!)]` is equivalent to `#[derive(Name!())]`.
///
/// A derivation macro may expand to any number of new items derived from the provided input.
#[macro_export]
macro_rules! macro_attr {
    (
        $(#[$($attrs:tt)+])*
        $(pub $(($($vis:tt)+))?)? enum $($it:tt)+
    ) => {
        $crate::macro_attr_impl! {
            @split_attrs [$(pub $(($($vis)+))?)? enum $($it)+]
            [] []
            [$([$($attrs)+])*]
        }
    };
    (
        $(#[$($attrs:tt)+])*
        $(pub $(($($vis:tt)+))?)? struct $($it:tt)+
    ) => {
        $crate::macro_attr_impl! {
            @split_attrs [$(pub $(($($vis)+))?)? struct $($it)+]
            [] []
            [$([$($attrs)+])*]
        }
    };
    (
        $(#[$($attrs:tt)+])*
        $(pub $(($($vis:tt)+))?)? trait $($it:tt)+
    ) => {
        $crate::macro_attr_impl! {
            @split_attrs [$(pub ($($vis)+))? trait $($it)+]
            [] []
            [$([$($attrs)+])*]
        }
    };
    (
        $(#[$($attrs:tt)+])*
        $vis:vis $keyword:ident $($it:tt)+
    ) => {
        $crate::macro_attr_impl! {
            @split_attrs [$vis $keyword $($it)+]
            [] []
            [$([$($attrs)+])*]
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! macro_attr_impl {
    (
        @split_attrs [$($it:tt)+]
        [$($derive_attrs:tt)*] [$([$other_attrs:meta])*]
        [[derive($($derive_attr:tt)+)] $([$($attrs:tt)+])*]
    ) => {
        $crate::macro_attr_impl! {
            @split_attrs [$($it)+]
            [$($derive_attrs)* [$($derive_attr)+]]
            [$([$other_attrs])*]
            [$([$($attrs)+])*]
        }
    };
    (
        @split_attrs [$($it:tt)+]
        [$($derive_attrs:tt)*] [$([$other_attrs:meta])*]
        [[$attr:meta] $([$($attrs:tt)+])*]
    ) => {
        $crate::macro_attr_impl! {
            @split_attrs [$($it)+]
            [$($derive_attrs)*]
            [$([$other_attrs])* [$attr]]
            [$([$($attrs)+])*]
        }
    };
    (
        @split_attrs [$($it:tt)+]
        [$($derive_attrs:tt)*] [$([$other_attrs:meta])*]
        []
    ) => {
        $crate::macro_attr_impl! {
            @split_derive_attrs [$($it)+] [$([$other_attrs])*]
            [] []
            [$($derive_attrs)*]
        }
    };
    (
        @split_derive_attrs [$($it:tt)+] [$([$other_attrs:meta])*]
        [$($macro_derives:tt)*] [$($std_derives:tt)*]
        [
            [$macro_derive:ident ! $(($($macro_derive_args:tt)*))? $(, $($other_inner_derives:tt)*)?]
            $([$($other_derives:tt)*])*
        ]
    ) => {
        $crate::macro_attr_impl! {
            @split_derive_attrs [$($it)+] [$([$other_attrs])*]
            [
                $($macro_derives)*
                [$macro_derive ( $($($macro_derive_args)*)? )]
            ]
            [
                $($std_derives)*
            ]
            [
                $([$($other_inner_derives)*])?
                $([$($other_derives)*])*
            ]
        }
    };
    (
        @split_derive_attrs [$($it:tt)+] [$([$other_attrs:meta])*]
        [$($macro_derives:tt)*] [$($std_derives:tt)*]
        [
            [$std_derive:ident $(($($std_derive_args:tt)*))? $(, $($other_inner_derives:tt)*)?]
            $([$($other_derives:tt)*])*
        ]
    ) => {
        $crate::macro_attr_impl! {
            @split_derive_attrs [$($it)+] [$([$other_attrs])*]
            [
                $($macro_derives)*
            ]
            [
                $($std_derives)*
                #[derive($std_derive $(($($std_derive_args)*))?)]
            ]
            [
                $([$($other_inner_derives)*])?
                $([$($other_derives)*])*
            ]
        }
    };
    (
        @split_derive_attrs [$($it:tt)+] [$([$other_attrs:meta])*]
        [$($macro_derives:tt)*] [$($std_derives:tt)*]
        [
            []
            $([$($other_derives:tt)*])*
        ]
    ) => {
        $crate::macro_attr_impl! {
            @split_derive_attrs [$($it)+] [$([$other_attrs])*]
            [
                $($macro_derives)*
            ]
            [
                $($std_derives)*
            ]
            [
                $([$($other_derives)*])*
            ]
        }
    };
    (
        @split_derive_attrs [$($it:tt)+] [$([$other_attrs:meta])*]
        [$([$macro_derive:ident ( $($macro_derive_args:tt)* )])*]
        [$($std_derives:tt)*]
        []
    ) => {
        $crate::macro_attr_impl! {
            @as_item
            $($std_derives)*
            $(#[$other_attrs])*
            $($it)+
        }
        $crate::macro_attr_impl! {
            @expand [$($it)+]
            [$([$macro_derive ( $($macro_derive_args)* )])*]
        }
    };
    (
        @expand [$($it:tt)+]
        [
            [$macro_derive:ident ( $($macro_derive_args:tt)* )]
            $([$other_macro_derives:ident ( $($other_macro_derives_args:tt)* )])*
        ]
    ) => {
        $macro_derive! {
            ( $($macro_derive_args)* )
            $($it)+
        }
        $crate::macro_attr_impl! {
            @expand [$($it)+]
            [$([$other_macro_derives ( $($other_macro_derives_args)* )])*]
        }
    };
    (
        @expand [$($it:tt)+]
        []
    ) => {
    };
    (@as_item $($i:item)*) => {$($i)*};
}
