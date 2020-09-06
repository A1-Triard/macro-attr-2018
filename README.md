# `macro-attr-2018`

The [`macro-attr`](https://crates.io/crates/macro-attr) modern fork.

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
