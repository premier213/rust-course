use crate::{arrays::array_type, crates::arch::archive_file as arc, random::random};
use rand::prelude::*;
use vectors::vector_type;
mod arrays;
mod borrowing;
mod crates;
mod input;
mod nested_mod;
mod random;
mod strings;
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
    vector_type()
}
