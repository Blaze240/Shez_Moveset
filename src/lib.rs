#![feature(
    concat_idents,
    proc_macro_hygiene
)]
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

static mut EDGE_EXIST : bool = false;

mod win1;
mod win2;
mod win3;

mod common;
mod slipatch;

#[skyline::main(name = "cloud_victory")]
pub fn main() {
	common::install();

    win1::install();
	win2::install();
	win3::install();

	slipatch::install();

}