// Copyright (c) 2016 macro-attr contributors.
// Copyright (c) 2020 Warlock <internalmike@gmail.com>.
//
// Licensed under the MIT license (see LICENSE or <http://opensource.org
// /licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
// <http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
// files in the project carrying such notice may not be copied, modified,
// or distributed except according to those terms.

/*!
This crate provides the `macro_attr!` macro that enables the use of custom, macro-based attributes and derivations.

The `macro_attr!` macro should be used to wrap an entire *single* item (`enum`, `struct`, *etc.*) declaration, including its attributes (both `derive` and others).  All attributes and derivations which whose names end with `!` will be assumed to be implemented by macros, and treated accordingly.

```rust
use macro_attr_2018::macro_attr;

// Define some traits to be derived.

trait TypeName {
    fn type_name() -> &'static str;
}

trait ReprType {
    type Repr;
}

// Define macros which derive implementations of these macros.

macro_rules! TypeName {
    // We can support any kind of item we want.
    (() $vis:vis enum $name:ident $($tail:tt)+) => { TypeName! { @impl $name } };
    (() $vis:vis struct $name:ident $($tail:tt)+) => { TypeName! { @impl $name } };

    // Inner rule to cut down on repetition.
    (@impl $name:ident) => {
        impl TypeName for $name {
            fn type_name() -> &'static str { stringify!($name) }
        }
    };
}

macro_rules! ReprType {
    // Note that we use a "derivation argument" here for the `$repr` type.
    (($repr:ty) $vis:vis enum $name:ident $($tail:tt)+) => {
        impl ReprType for $name {
            type Repr = $repr;
        }
    };
}
```
*/

#![no_std]
#![deny(warnings)]

/**
When given an item definition, including its attributes, this macro parses said attributes and dispatches any attributes or derivations suffixed with `!` to user-defined macros.  This allows multiple macros to process the same item.

Given the following input:

```ignore
#[derive(Copy, Name!(args...), Clone, Another!, Debug)]
struct Foo;
```

`macro_attr!` will expand to the equivalent of:

```ignore
#[derive(Copy, Clone, Debug)]
struct Foo;

Name!((args...) struct Foo;);
Another!(() struct Foo;);
```

Note that macro derives may be mixed with regular derives, or put in their own `#[derive(...)]` attribute.  Also note that macro derive invocations are *not* passed the other attributes on the item; input will consist of the arguments provided to the derivation (*i.e.* `(args...)` in this example), the item's visibility (if any), and the item definition itself.

A macro derivation invoked *without* arguments will be treated as though it was invoked with empty parentheses.  *i.e.* `#[derive(Name!)]` is equivalent to `#[derive(Name!())]`.

A derivation macro may expand to any number of new items derived from the provided input.
*/
#[macro_export]
macro_rules! macro_attr {
    ($($item:tt)*) => {
        $crate::macro_attr_impl! { $($item)* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! macro_attr_impl {
    /*

    > **Convention**: a capture named `$fixed` is used for any part of a recursive rule that is needed in the terminal case, but is not actually being *used* for the recursive part.  This avoids having to constantly repeat the full capture pattern (and makes changing it easier).

    # Primary Invocation Forms

    These need to catch any valid form of item.

    */
    (
        $(#[$($attrs:tt)+])*
        enum $($it:tt)+
    ) => {
        $crate::macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)+],)*), (), (),
            (enum $($it)+)
        }
    };

    (
        $(#[$($attrs:tt)+])*
        struct $($it:tt)+
    ) => {
        $crate::macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)+],)*), (), (),
            (struct $($it)+)
        }
    };

    (
        $(#[$($attrs:tt)+])*
        trait $($it:tt)+
    ) => {
        $crate::macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)+],)*), (), (),
            (trait $($it)+)
        }
    };

    (
        $(#[$($attrs:tt)+])*
        pub $(($($vis:tt)+))? enum $($it:tt)+
    ) => {
        $crate::macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)+],)*), (), (),
            (pub $(($($vis)+))? enum $($it)+)
        }
    };

    (
        $(#[$($attrs:tt)+])*
        pub $(($($vis:tt)+))? struct $($it:tt)+
    ) => {
        $crate::macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)+],)*), (), (),
            (pub $(($($vis)+))? struct $($it)+)
        }
    };

    (
        $(#[$($attrs:tt)+])*
        pub $(($($vis:tt)+))? trait $($it:tt)+
    ) => {
        $crate::macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)+],)*), (), (),
            (pub $(($($vis)+))? trait $($it)+)
        }
    };

    /*

    # `@split_attrs`

    This is responsible for dividing all attributes on an item into two groups:

    - `#[derive(...)]`
    - Everything else.

    As part of this, it also explodes `#[derive(A, B(..), C, ...)]` into `A, B(..), C, ...`.  This is to simplify the next stage.

    */
    (
        @split_attrs
        (),
        $non_derives:tt,
        $derives:tt,
        $it:tt
    ) => {
        $crate::macro_attr_impl! {
            @split_derive_attrs
            { $non_derives, $it },
            $derives,
            (),
            ()
        }
    };

    (
        @split_attrs
        (#[derive($($new_drvs:tt)*)], $(#[$($attrs:tt)*],)*),
        $non_derives:tt,
        ($($derives:tt)*),
        $it:tt
    ) => {
        $crate::macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            ($($derives)* $($new_drvs)*,),
            $it
        }
    };

    (
        @split_attrs
        (#[$new_attr:meta], $(#[$($attrs:tt)*],)*),
        ($($non_derives:tt)*),
        $derives:tt,
        $it:tt
    ) => {
        $crate::macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            ($($non_derives)* #[$new_attr],),
            $derives,
            $it
        }
    };

    /*

    # `@split_derive_attrs`

    This is responsible for taking the list of derivation attributes and splitting them into "built-in" and "custom" groups.

    A custom attribute is any which has a `!` after the name, like a macro.
    */

    (@split_derive_attrs
        { ($(#[$($non_derives:tt)*],)*), ($($it:tt)*) },
        ($(,)*), (), ($($user_drvs:tt)*)
    ) => {
        $crate::macro_attr_impl! {
            @as_item
            $(#[$($non_derives)*])*
            $($it)*
        }

        $crate::macro_attr_impl! {
            @expand_user_drvs
            ($($user_drvs)*), ($($it)*)
        }
    };

    (@split_derive_attrs
        { ($(#[$($non_derives:tt)*],)*), ($($it:tt)*) },
        ($(,)*), ($($bi_drvs:ident,)+), ($($user_drvs:tt)*)
    ) => {
        $crate::macro_attr_impl! {
            @as_item
            #[derive($($bi_drvs,)+)]
            $(#[$($non_derives)*])*
            $($it)*
        }

        $crate::macro_attr_impl! {
            @expand_user_drvs
            ($($user_drvs)*), ($($it)*)
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (,, $($tail:tt)*), $bi_drvs:tt, $user_drvs:tt
    ) => {
        $crate::macro_attr_impl! {
            @split_derive_attrs
            $fixed, ($($tail)*), $bi_drvs, $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (, $($tail:tt)*), $bi_drvs:tt, $user_drvs:tt
    ) => {
        $crate::macro_attr_impl! {
            @split_derive_attrs
            $fixed, ($($tail)*), $bi_drvs, $user_drvs
        }
    };

    /*

    ## Custom Derivations

    Now we can handle the custom derivations.  There are two forms we care about: those *with* an argument, and those *without*.

    The *reason* we care is that, in order to simplify the derivation macros, we want to detect the argument-less case and generate an empty pair of parens.

    */
    (@split_derive_attrs
        $fixed:tt,
        ($new_user:ident ! ($($new_user_args:tt)*), $($tail:tt)*), $bi_drvs:tt, ($($user_drvs:tt)*)
    ) => {
        $crate::macro_attr_impl! {
            @split_derive_attrs
            $fixed, ($($tail)*), $bi_drvs, ($($user_drvs)* $new_user($($new_user_args)*),)
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        ($new_user:ident !, $($tail:tt)*), $bi_drvs:tt, ($($user_drvs:tt)*)
    ) => {
        $crate::macro_attr_impl! {
            @split_derive_attrs
            $fixed, ($($tail)*), $bi_drvs, ($($user_drvs)* $new_user(),)
        }
    };

    /*

    ## Non-Macro Derivations

    All the rest.

    */
    (@split_derive_attrs
        $fixed:tt,
        ($drv:ident, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        $crate::macro_attr_impl! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* $drv,), $user_drvs
        }
    };

    /*

    # `@expand_user_drvs`

    Finally, we have a recursive rule for expanding user derivations.  This is basically just using the derivation name as a macro identifier.

    This *has* to be recursive because we need to expand two independent repetition sequences simultaneously, and this causes `macro_rules!` to throw a wobbly.  Don't want that.  So, recursive it is.

    */
    (@expand_user_drvs
        (), ($($it:tt)*)
    ) => {};

    (@expand_user_drvs
        ($user_drv:ident $arg:tt, $($tail:tt)*), ($($it:tt)*)
    ) => {
        $user_drv! { $arg $($it)* }
        $crate::macro_attr_impl! {
            @expand_user_drvs
            ($($tail)*), ($($it)*)
        }
    };

    /*

    # Miscellaneous Rules

    */
    (@as_item $($i:item)*) => {$($i)*};
}
