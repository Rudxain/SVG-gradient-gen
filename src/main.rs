#![warn(
	unused,
	future_incompatible,
	clippy::exit,
	clippy::unwrap_used,
	clippy::cargo,
	clippy::pedantic,
	clippy::nursery,
	clippy::shadow_unrelated,
	clippy::string_to_string,
	clippy::decimal_literal_representation,
	clippy::unseparated_literal_suffix,
	clippy::empty_structs_with_brackets,
	clippy::format_push_string
)]
#![forbid(
	unsafe_code,
	clippy::mem_forget,
	clippy::large_include_file,
	clippy::fn_to_numeric_cast_any,
	clippy::cast_precision_loss,
	clippy::float_arithmetic,
	clippy::excessive_precision,
	clippy::lossy_float_literal,
	clippy::float_cmp,
	clippy::float_cmp_const
)]

use core::str::FromStr;
use sv3g::*;

#[derive(Debug, PartialEq)]
enum SubCmds {
	Help,
	Custom,
	Wb,
	Rainbow,
	Rgb,
	Sky,
	Mint,
	Fire,
}

impl FromStr for SubCmds {
	type Err = ();

	fn from_str(input: &str) -> Result<SubCmds, Self::Err> {
		match input {
			"help" => Ok(SubCmds::Help),
			_ => Err(()),
		}
	}
}

fn main() {}
