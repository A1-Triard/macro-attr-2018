// Copyright (c) 2015 macro-attr contributors.
// Copyright (c) 2020 Warlock <internalmike@gmail.com>.
//
// Licensed under the MIT license (see LICENSE or <http://opensource.org
// /licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
// <http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
// files in the project carrying such notice may not be copied, modified,
// or distributed except according to those terms.

#![deny(warnings)]

use macro_attr_2018::macro_attr;

macro_rules! EnumIterator {
    (() $(pub)* enum $name:ident { $($body:tt)* }) => {
        EnumIterator! {
            @collect_variants ($name),
            ($($body)*,) -> ()
        }
    };

    (
        @collect_variants ($name:ident),
        ($(,)*) -> ($($var_names:ident,)*)
    ) => {
        type NameIter = ::std::vec::IntoIter<&'static str>;
        type VariantIter = ::std::vec::IntoIter<$name>;

        impl $name {
            #[allow(dead_code)]
            pub fn iter_variants() -> VariantIter {
                vec![$($name::$var_names),*].into_iter()
            }

            #[allow(dead_code)]
            pub fn iter_variant_names() -> NameIter {
                vec![$(stringify!($var_names)),*].into_iter()
            }
        }
    };

    (
        @collect_variants $fixed:tt,
        ($var:ident $(= $_val:expr)*, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        EnumIterator! {
            @collect_variants $fixed,
            ($($tail)*) -> ($($var_names)* $var,)
        }
    };
}

macro_attr! {
    #[derive(Debug, PartialEq, EnumIterator!)]
    enum Get { Up, Down, AllAround }
}

#[test]
fn test_enum_iterator() {
    let vs: Vec<_> = Get::iter_variant_names().zip(Get::iter_variants()).collect();
    assert_eq!(&*vs, &[("Up", Get::Up), ("Down", Get::Down), ("AllAround", Get::AllAround)]);
}
