#![feature(concat_idents, proc_macro_hygiene)]
#![allow(
    unused_imports,
    unused_macros,
    unused_variables,
    unused_assignments,
    unused_unsafe,
    non_upper_case_globals,
    non_snake_case,
    clippy::borrow_interior_mutable_const
)]
use the_csk_collection_api::*;
static mut MASTER_EXIST: bool = false;

mod win1;
mod win2;
mod win3;

mod common;
mod slipatch;

#[skyline::main(name = "shez_moveset")]
pub fn main() {
    let version = the_csk_collection_api::
    if version.major < 2 {
        println!("You're using an outdated build of the CSK Collection plugin! Please make sure to update to the latest.");
    }

    common::install();

    win1::install();
    win2::install();
    win3::install();

    slipatch::install();
}
