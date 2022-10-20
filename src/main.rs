// use std::str::pattern;

use crate::{arrays::array_type, crates::arch::archive_file as arc, random::random};
use if_statement::statement;
use matches::{
    matching, matching_name,
    Level::{Easy, Hard, Middle},
};
use pattern::pattern_type;
use rand::prelude::*;
use slices::slice_type;
use structures::struct_type;
use tuples::tuple_type;
use vectors::vector_type;
mod arrays;
mod borrowing;
mod crates;
mod enums;
mod generics;
mod if_statement;
mod input;
mod matches;
mod nested_mod;
mod pattern;
mod random;
mod slices;
mod strings;
mod structures;
mod tuples;
mod variable;
mod vectors;
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
    pattern::tuple_pattern(5, -200);
}
