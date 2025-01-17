//! LP Parser - A Linear Programming File Parser
//!
//! This crate provides robust parsing capabilities for Linear Programming (LP)
//! files using nom parser combinators. It supports multiple industry-standard
//! LP file formats and offers comprehensive features for optimization problems.
//!
//! # Features
//!
//! - Zero-copy parsing with lifetime management
//! - Support for multiple LP file format specifications
//! - Comprehensive parsing of all standard LP file components
//! - Optional serialization and diff tracking
//!
//! # Quick Start
//!
//! ```rust
//! use std::path::Path;
//!
//! use lp_parser::{parser::parse_file, LpProblem};
//!
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let content = parse_file(Path::new("problem.lp"))?;
//!     let problem = LpProblem::parse(&content)?;
//!     println!("Problem name: {:?}", problem.name());
//!     Ok(())
//! }
//! ```
//!
//! # Module Organization
//!
//! - `model`: Core data structures for LP problems
//! - `parser`: File parsing utilities
//! - `parsers`: Component-specific parsers
//! - `lp_problem`: Main problem representation and parsing
//!

// #![deny(missing_docs)]

#[cfg(feature = "csv")]
pub mod csv;
pub mod problem;
pub mod model;
pub mod parser;
pub mod parsers;

use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    error::{Error, ErrorKind},
    Err, IResult,
};

/// Headers that indicate the beginning of a constraint section in an LP file.
pub const CONSTRAINT_HEADERS: [&str; 5] = ["subject to", "such that", "s.t.", "st:", "st"];

/// All possible section headers that can appear in an LP file's bounds section.
pub const ALL_BOUND_HEADERS: [&str; 14] = [
    "bounds",
    "bound",
    "generals",
    "general",
    "gen",
    "integers",
    "integer",
    "binaries",
    "binary",
    "bin",
    "semi-continuous",
    "semis",
    "semi",
    "end",
];

/// Headers that indicate the beginning of a binary variable section.
pub const BINARY_HEADERS: [&str; 4] = ["binaries", "binary", "bin", "end"];

/// Header marking the end of an LP file or section.
pub const END_HEADER: [&str; 1] = ["end"];

/// Headers that indicate the beginning of a general integer variable section.
pub const GENERAL_HEADERS: [&str; 4] = ["generals", "general", "gen", "end"];

/// Headers that indicate the beginning of an integer variable section.
pub const INTEGER_HEADERS: [&str; 3] = ["integers", "integer", "end"];

/// Headers that indicate the beginning of a semi-continuous variable section.
pub const SEMI_HEADERS: [&str; 4] = ["semi-continuous", "semis", "semi", "end"];

/// Headers that indicate the beginning of a Special Ordered Set (SOS) constraint section.
pub const SOS_HEADERS: [&str; 2] = ["sos", "end"];

/// Valid characters that can appear in LP file elements.
///
/// These characters are allowed in addition to alphanumeric
/// characters in names and other elements of LP files.
pub const VALID_LP_FILE_CHARS: [char; 18] = ['!', '#', '$', '%', '&', '(', ')', '_', ',', '.', ';', '?', '@', '\\', '{', '}', '~', '\''];

#[inline]
pub(crate) fn log_unparsed_content(prefix: &str, remaining: &str) {
    if !remaining.trim().is_empty() {
        log::debug!("{prefix}: {remaining}");
    }
}

#[inline]
/// Returns a closure that takes an input string and searches for the specified
/// tag, ignoring case. The closure returns a result containing a tuple with the
/// part of the input after the tag and the part before the tag if the tag is
/// found. If the tag is not found, it returns an error.
///
/// # Arguments
///
/// * `tag` - A string slice that represents the tag to search for in the input.
///
/// # Returns
///
/// A closure that takes a string slice and returns an `IResult` containing a
/// tuple of string slices or an error if the tag is not found.
///
fn take_until_cased<'a>(tag: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &str| {
        let mut index = 0;
        let tag_lower = tag.to_lowercase();
        let chars: Vec<char> = input.chars().collect();

        if chars.len() < tag.len() {
            return Err(Err::Error(Error::new(input, ErrorKind::TakeUntil)));
        }

        while index <= chars.len() - tag.len() {
            let window: String = chars[index..index + tag.len()].iter().collect();
            if window.to_lowercase() == tag_lower {
                return Ok((&input[index..], &input[..index]));
            }
            index += 1;
        }

        Err(Err::Error(Error::new(input, ErrorKind::TakeUntil)))
    }
}

#[allow(clippy::manual_try_fold)]
#[inline]
pub(crate) fn take_until_parser<'a>(tags: &'a [&'a str]) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> + 'a {
    move |input| {
        tags.iter()
            .map(|&tag| take_until_cased(tag))
            .fold(Err(Err::Error(Error::new(input, ErrorKind::Alt))), |acc, parser| acc.map_or_else(|_| parser(input), Ok))
    }
}

#[inline]
/// Checks if the input string starts with a binary section header.
pub fn is_binary_section(input: &str) -> IResult<&str, &str> {
    alt((tag_no_case("binaries"), tag_no_case("binary"), tag_no_case("bin")))(input)
}

#[inline]
/// Checks if the input string starts with a bounds section header.
pub fn is_bounds_section(input: &str) -> IResult<&str, &str> {
    alt((tag_no_case("bounds"), tag_no_case("bound")))(input)
}

#[inline]
/// Checks if the input string starts with a generals section header.
pub fn is_generals_section(input: &str) -> IResult<&str, &str> {
    alt((tag_no_case("generals"), tag_no_case("general"), tag_no_case("gen")))(input)
}

#[inline]
/// Checks if the input string starts with a integers section header.
pub fn is_integers_section(input: &str) -> IResult<&str, &str> {
    alt((tag_no_case("integers"), tag_no_case("integer")))(input)
}

#[inline]
/// Checks if the input string starts with a semi-continuous section header.
pub fn is_semi_section(input: &str) -> IResult<&str, &str> {
    alt((tag_no_case("semis"), tag_no_case("semi")))(input)
}

#[inline]
/// Checks if the input string starts with a SOS constraints section header.
pub fn is_sos_section(input: &str) -> IResult<&str, &str> {
    tag_no_case("sos")(input)
}
