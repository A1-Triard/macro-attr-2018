// Copyright (c) 2015 macro-attr contributors.
// Copyright (c) 2020 Warlock <internalmike@gmail.com>.
// Copyright (c) 2020 Clint Armstrong <clint@clintarmstrong.net>.
//
// Licensed under the MIT license (see LICENSE or <http://opensource.org
// /licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
// <http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
// files in the project carrying such notice may not be copied, modified,
// or distributed except according to those terms.

use macro_attr_2018::macro_attr;

pub trait UseVis {
    fn print_vis() -> &'static str;
}

macro_rules! UseVis {
    (() $vis:vis $ty:ident $id:ident $($it:tt)+) => {
        impl UseVis for $id {
            fn print_vis() -> &'static str {
                stringify!($vis).trim()
            }
        }
    };
}

macro_rules! NestedVis {
    ($vis:vis $ty:ident $id:ident $($it:tt)+) => {
        macro_attr!{
            #[derive(UseVis!)]
            $vis $ty $id $($it)+
        }
    };
}

macro_rules! UseVisTT {
    (() $(pub $(($($vis:tt)+))?)? enum $id:ident $($it:tt)+) => {
        impl UseVis for $id {
            fn print_vis() -> &'static str {
                stringify!($(pub $(($($vis)*))?)?)
            }
        }
    };
    (() $(pub $(($($vis:tt)+))?)? struct $id:ident $($it:tt)+) => {
        impl UseVis for $id {
            fn print_vis() -> &'static str {
                stringify!($(pub $(($($vis)*))?)?)
            }
        }
    };
}

macro_rules! NestedVisTT {
    ($(pub $(($($vis:tt)+))?)? enum $id:ident $($it:tt)+) => {
        macro_attr!{
            #[derive(UseVis!)]
            $(pub $(($($vis)*))?)? enum $id $($it)+
        }
    };
    ($(pub $(($($vis:tt)+))?)? struct $id:ident $($it:tt)+) => {
        macro_attr!{
            #[derive(UseVisTT!)]
            $(pub $(($($vis)*))?)? struct $id $($it)+
        }
    };
}

macro_attr! {
    #[derive(UseVis!)]
    enum PrivE {}
}

macro_attr! {
    #[derive(UseVis!)]
    pub enum PubE {}
}

macro_attr! {
    #[derive(UseVis!)]
    pub(crate) enum CrateE {}
}

macro_attr! {
    #[derive(UseVis!)]
    struct PrivS {}
}

macro_attr! {
    #[derive(UseVis!)]
    pub struct PubS {}
}

macro_attr! {
    #[derive(UseVis!)]
    pub(crate) struct CrateS {}
}

#[test]
fn test_vis() {
    assert_eq!("", PrivE::print_vis());
    assert_eq!("pub", PubE::print_vis());
    assert_eq!("pub(crate)", CrateE::print_vis());
    assert_eq!("", PrivS::print_vis());
    assert_eq!("pub", PubS::print_vis());
    assert_eq!("pub(crate)", CrateS::print_vis());
}

macro_attr! {
    #[derive(UseVisTT!)]
    enum PrivETT {}
}

macro_attr! {
    #[derive(UseVisTT!)]
    pub enum PubETT {}
}

macro_attr! {
    #[derive(UseVisTT!)]
    pub(crate) enum CrateETT {}
}

macro_attr! {
    #[derive(UseVisTT!)]
    struct PrivSTT {}
}

macro_attr! {
    #[derive(UseVisTT!)]
    pub struct PubSTT {}
}

macro_attr! {
    #[derive(UseVisTT!)]
    pub(crate) struct CrateSTT {}
}

#[test]
fn test_vis_tt() {
    assert_eq!("", PrivETT::print_vis());
    assert_eq!("pub", PubETT::print_vis());
    assert_eq!("pub(crate)", CrateETT::print_vis());
    assert_eq!("", PrivSTT::print_vis());
    assert_eq!("pub", PubSTT::print_vis());
    assert_eq!("pub(crate)", CrateSTT::print_vis());
}

NestedVis!{
    enum NestedPrivE {}
}

NestedVis!{
    pub enum NestedPubE {}
}

NestedVis!{
    pub(crate) enum NestedCrateE {}
}

NestedVis!{
    struct NestedPrivS {}
}

NestedVis!{
    pub struct NestedPubS {}
}

NestedVis!{
    pub(crate) struct NestedCrateS {}
}

#[test]
fn test_nested_vis() {
    assert_eq!("", NestedPrivE::print_vis());
    assert_eq!("pub", NestedPubE::print_vis());
    assert_eq!("pub(crate)", NestedCrateE::print_vis());
    assert_eq!("", NestedPrivS::print_vis());
    assert_eq!("pub", NestedPubS::print_vis());
    assert_eq!("pub(crate)", NestedCrateS::print_vis());
}

NestedVisTT!{
    enum NestedPrivTTE {}
}

NestedVisTT!{
    pub enum NestedPubTTE {}
}

NestedVisTT!{
    pub(crate) enum NestedCrateTTE {}
}

NestedVisTT!{
    struct NestedPrivTTS {}
}

NestedVisTT!{
    pub struct NestedPubTTS {}
}

NestedVisTT!{
    pub(crate) struct NestedCrateTTS {}
}

#[test]
fn test_nested_vis_tt() {
    assert_eq!("", NestedPrivTTE::print_vis());
    assert_eq!("pub", NestedPubTTE::print_vis());
    assert_eq!("pub(crate)", NestedCrateTTE::print_vis());
    assert_eq!("", NestedPrivTTS::print_vis());
    assert_eq!("pub", NestedPubTTS::print_vis());
    assert_eq!("pub(crate)", NestedCrateTTS::print_vis());
}