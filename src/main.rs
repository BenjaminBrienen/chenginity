#![warn(clippy::all)]
#![warn(absolute_paths_not_starting_with_crate)]
#![warn(box_pointers)]
#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(fuzzy_provenance_casts)]
#![warn(keyword_idents)]
#![warn(lossy_provenance_casts)]
#![warn(macro_use_extern_crate)]
#![warn(meta_variable_misuse)]
#![warn(missing_abi)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(must_not_suspend)]
#![warn(non_ascii_idents)]
#![warn(non_exhaustive_omitted_patterns)]
#![warn(noop_method_call)]
#![warn(pointer_structural_match)]
#![warn(rust_2021_incompatible_closure_captures)]
#![warn(rust_2021_incompatible_or_patterns)]
#![warn(rust_2021_prefixes_incompatible_syntax)]
#![warn(rust_2021_prelude_collisions)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unstable_features)]
#![allow(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_macro_rules)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![allow(incomplete_features)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::implicit_return)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(unstable_features)]
#![feature(try_trait_v2)]
#![feature(let_chains)]
#![feature(adt_const_params)]
#![feature(strict_provenance)]
#![feature(must_not_suspend)]
#![feature(non_exhaustive_omitted_patterns_lint)]
//! Chess game

use {
	chenginity::{
		read_move,
		Coordinates,
		Piece,
	},
	colored::Colorize,
	std::{
		error::Error,
		io::{
			stdin,
			stdout,
			BufRead,
			Write,
		},
	},
};

#[allow(box_pointers)]
fn main() -> Result<(), Box<dyn Error>>
{
	let mut input_handle = stdin().lock();
	let mut input_buffer: String = String::new();
	let mut output_handle = stdout();
	_ = input_handle.read_line(&mut input_buffer)?;
	// let dimensions = input_buffer.parse::<i32>();
	loop
	{
		print!("{}", "\nInput move: ".green());
		_ = output_handle.flush();
		let input = read_move::<2>(&mut input_handle)?;
		let mut pawn = Piece(Coordinates([0, 0]));
		println!("{:?}", pawn.translate(input).translate(input));
	}
}
