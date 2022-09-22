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
//! Library for the game

use std::{
	io::BufRead,
	ops::Neg,
};

/// # Errors
///
/// This function will return an error if .
pub fn read_move<const DIMENSIONS: usize>(input_source: &mut impl BufRead) -> Result<Coordinates<DIMENSIONS>, String>
{
	let mut input = String::new();
	if input_source.read_line(&mut input).is_ok()
	{
		let mut validated_parsed_tokens = Coordinates([0; DIMENSIONS]);
		let mut parsed_tokens = input.trim().split(' ').map(|value| value.parse::<i32>()).enumerate();
		let mut counter: usize = 0;
		while let Some((i, Ok(element))) = parsed_tokens.next()
		{
			if i >= DIMENSIONS
			{
				return Err("Invalid Movement. 🤕 Too many arguments.".to_string())
			}
			validated_parsed_tokens.0[i] = element;
			counter = i;
		}
		if counter < DIMENSIONS - 1
		{
			return Err("Invalid Movement. 🤕 Too few arguments.".to_string())
		}
		Ok(validated_parsed_tokens)
	}
	else
	{
		// Show helpful error when user input is invalid.
		Err("Invalid Movement. 🤕 Movements must be in the format of: x, y, dx, dy".to_string())
	}
}

/// Chess piece
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Piece<const DIMENSIONS: usize>(pub Coordinates<DIMENSIONS>);

/// Cordinates representing a point or vector in space on the game board
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Coordinates<const DIMENSIONS: usize>(pub [i32; DIMENSIONS]);

impl<const DIMENSIONS: usize> Neg for Coordinates<DIMENSIONS>
{
	type Output = Coordinates<DIMENSIONS>;

	fn neg(self) -> Self::Output
	{
		let mut output: Self = self;
		for (i, element) in self.0.iter().enumerate()
		{
			output.0[i] = -element;
		}
		output
	}
}

impl<const DIMENSIONS: usize> Piece<DIMENSIONS>
{
	/// Adds vector to piece's position
	pub fn translate(
		&mut self,
		d_position: Coordinates<DIMENSIONS>,
	) -> &mut Self
	{
		for (i, element) in d_position.0.iter().enumerate().take(DIMENSIONS)
		{
			self.0 .0[i] += element
		}
		self
	}
}

#[cfg(test)]
mod tests
{
	use {
		super::read_move,
		crate::{
			Coordinates,
			Piece,
		},
		test_case::test_case,
	};

	enum IntendedTestCaseResult
	{
		Ok,
		Err,
	}

	#[test_case(Coordinates([0, 1, 2, 3, 4, 5, 6, 7, 8]), Coordinates([0, 1, 2, 3, 4, 5, 6, 7, 8]),    IntendedTestCaseResult::Ok;  "9 dimensional space with 9 inputs")]
	#[test_case(Coordinates([0, 1, 2, 3, 4, 5, 6, 7, 8]), Coordinates([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), IntendedTestCaseResult::Err; "9 dimensional space with 10 inputs")]
	#[test_case(Coordinates([0, 1, 2, 3, 4, 5, 6, 7, 8]), Coordinates([0, 1, 2, 3, 4, 5, 6, 7]),       IntendedTestCaseResult::Err; "9 dimensional space with 8 inputs")]
	fn test_bad_get_input_move_too_little<const DIMENSIONS: usize, const INPUT_SIZE: usize>(
		_space: Coordinates<DIMENSIONS>,
		input: Coordinates<INPUT_SIZE>,
		result: IntendedTestCaseResult,
	)
	{
		let input_string = format!("{}\n", input.0.map(|i| -> String { i.to_string() }).join(" "));
		match result
		{
			IntendedTestCaseResult::Ok => assert!(read_move::<DIMENSIONS>(&mut input_string.as_bytes()).is_ok()),
			IntendedTestCaseResult::Err => assert!(read_move::<DIMENSIONS>(&mut input_string.as_bytes()).is_err()),
		}
	}

	#[test]
	fn test_translate()
	{
		let input_string = "5 1 2 3\n";
		if let Ok(movement) = read_move(&mut input_string.as_bytes())
		{
			let mut piece = Piece(Coordinates([-2, 0, -6, 1]));
			_ = piece.translate(movement);
			assert_eq!(piece, Piece(Coordinates([3, 1, -4, 4])));
		}
	}

	#[test]
	fn test_translate2()
	{
		let input_string = "5 1 2 3\n";
		if let Ok(movement) = read_move(&mut input_string.as_bytes())
		{
			let mut piece = Piece(Coordinates([-2, 0, -6, 1]));
			_ = piece.translate(movement).translate(-movement);
			assert_eq!(piece, Piece(Coordinates([-2, 0, -6, 1])));
		}
	}
}
