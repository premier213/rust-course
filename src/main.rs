// use std::str::pattern;

use crate::{arrays::array_type, crates::arch::archive_file as arc, random::random};
use forloop::for_type;
use generic_traits::gn;
use hofs::hof_type;
use if_statement::statement;
use matches::{
    matching, matching_name,
    Level::{Easy, Hard, Middle},
};
use mutable_trait::mut_sum;
use pattern::pattern_type;
use rand::prelude::*;
use slices::slice_type;
use structures::struct_type;
use traits::tr;
use tuples::tuple_type;
use unsafes::unsafe_type;
use vectors::vector_type;
use while_loop::{loop_standalone, while_type};
mod arrays;
mod borrowing;
mod closures;
mod concurrency;
mod crates;
mod dynamic_dispatch;
mod enum_err;
mod enums;
mod error_handling;
mod files;
mod forloop;
mod generic_traits;
mod generics;
mod hofs;
mod if_statement;
mod input;
mod lifetime;
mod macros;
mod matches;
mod mutable_trait;
mod nested_mod;
mod operator;
mod overloading;
mod ownership;
mod pattern;
mod random;
mod reference;
mod slices;
mod static_dispatch;
mod strings;
mod structures;
mod trait_return;
mod traits;
mod tuples;
mod unsafes;
mod variable;
mod vectors;
mod while_loop;

fn main() {
    // let mut range = rand::thread_rng();
    // let a: u32 = range.gen();
    // println!("{a}");
    // arc("meysam");
    // random();
    // nested_mod::clean::clean_hdd();
    // nested_mod::clean::files::clean_files();
    // strings::print_line();
    // strings::strings();
    // input::input();
    // array_type();
    // vector_type()
    // slice_type();
    // tuple_type();
    // struct_type();
    // enums::enum_type();
    // generics::generic_types();
    // statement();
    // matching(Easy);
    // matching_name(String::from("ali"));
    // println!("{}", pattern_type(12));
    // pattern::tuple_pattern(5, -200);
    // for_type();
    // while_type(100);
    // loop_standalone(5000);
    // unsafe_type();
    // closures::closures_type();
    // let square = |a: i32| a * a;
    // hof_type(square, 2);
    // macros::macro_type();
    // traits::tr();
    // generic_traits::gn();
    // trait_return::trait_ret();
    // mutable_trait::mut_sum();
    // overloading::over();
    // static_dispatch::stat()
    // ownership::owner()
    // lifetime::life();
    // reference::reference();
    // files::files();
    // error_handling::err()
    // enum_err::err(Some(1));
    // operator::operator();
    // concurrency::threads::thread();
    // concurrency::channels::channel();
    concurrency::mutex::mutex();
}
