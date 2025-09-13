//! # dice-nom
//!
//! A comprehensive dice rolling library that utilizes the nom parser for randomly generating
//! numbers to support role-playing games. This library provides parsing capabilities for
//! complex dice expressions including exploding dice, target numbers, success levels,
//! and various modifiers commonly used in tabletop RPGs.
//!
//! ## Features
//!
//! - **Complex Dice Expressions**: Parse strings like `"3d6+2"`, `"4d8!"`, or `"2d20^1"`
//! - **Dice Operations**: Exploding dice, advantage/disadvantage, target numbers
//! - **Pool Operations**: Keep highest/lowest, take middle values, group matching
//! - **Arithmetic**: Addition, subtraction with proper precedence
//! - **Comparison Operations**: Compare dice pools with various operators
//! - **Success Systems**: Target-based and threshold-based success counting
//!
//! ## Quick Start
//!
//! ```rust
//! use dice_nom::{parse, roller};
//! use rand::prelude::*;
//!
//! let mut rng = rand::thread_rng();
//!
//! // Simple dice rolling
//! let simple_roller = roller(2, 6, None);
//! let result = simple_roller.generate(&mut rng);
//! println!("2d6: {}", result);
//!
//! // Parse complex expressions
//! let generator = parse("3d6+4").unwrap();
//! let result = generator.generate(&mut rng);
//! println!("3d6+4: {}", result);
//! ```
//!
//! ## Dice Operators
//!
//! | Operator | Description | Example |
//! |----------|-------------|---------|
//! | `!` | Explode on max | `3d6!` |
//! | `!!` | Explode until not max | `2d8!!` |
//! | `*` | Explode each die on max | `4d6*` |
//! | `**` | Explode each until not max | `3d10**` |
//! | `++n` | Add n to each die | `2d4++2` |
//! | `--n` | Subtract n from each die | `3d6--1` |
//! | `` `n `` | Keep lowest n dice | `4d6`3` |
//! | `^n` | Keep highest n dice | `4d6^3` |
//! | `~n` | Keep middle n dice | `5d6~3` |
//! | `ADV` | Roll twice, keep higher | `1d20ADV` |
//! | `DIS` | Roll twice, keep lower | `1d20DIS` |
//! | `Y` | Keep largest group | `5d6Y` |
//!
//! ## Target and Success Operators
//!
//! | Operator | Description | Example |
//! |----------|-------------|---------|
//! | `[n]` | Count dice ≥ n as hits | `6d6[4]` |
//! | `(n)` | Count dice ≤ n as hits | `4d10(3)` |
//! | `{n}` | Success if total ≥ n | `3d6{12}` |
//! | `{n,m}` | Success levels every m over n | `2d10{15,5}` |
//!
//! ## Comparison Operators
//!
//! Compare two dice expressions:
//!
//! ```rust
//! use dice_nom::parse;
//! use rand::prelude::*;
//!
//! let mut rng = rand::thread_rng();
//! let contest = parse("3d6 > 2d8+1").unwrap();
//! let result = contest.generate(&mut rng);
//! println!("Contest result: {}", result.sum()); // 1 if left wins, 0 if right wins
//! ```
//!
//! ## Module Organization
//!
//! - [`generators`]: Core generator types and implementations
//! - [`results`]: Result types for dice rolls and pools
//! - [`parsers`]: Nom-based parsers for dice expressions

pub mod results;

pub mod generators;
use generators::{Generator, PoolGenerator};

pub mod parsers;

/// roller builds a simple `PoolGenerator` that can randomly generate dice rolls.
///
/// * Examples
///
/// ```
/// use rand::prelude::*;
/// let mut rng = rand::rng();
/// let roller = dice_nom::roller(3, 6, Some("**"));
/// assert_eq!(roller.count, 3);
/// assert_eq!(roller.range, 6);
/// assert_eq!(roller.op, Some(dice_nom::generators::PoolOp::ExplodeEachUntil(None)));
///
/// let pool = roller.generate(&mut rng);
/// assert!(pool.count() >= 3);
/// assert!(pool.sum() >= 3);
/// ```
pub fn roller(count: i32, range: i32, op: Option<&str>) -> PoolGenerator {
    let op = match op {
        Some(s) => match parsers::pool_op_parser(s) {
            Ok((_, op)) => Some(op),
            Err(_) => None,
        },
        None => None,
    };
    PoolGenerator { count, range, op }
}

/// parse builds a generator from the given input string. If any of the string
/// can be parsed a generator is returned. If no generator can be built then
/// an error is returned with the input string.
///
/// * Examples
///
/// ```
/// use rand::prelude::*;
/// let mut rng = rand::rng();
/// let g = dice_nom::parse("2d4! + 2d6! < 3d8!");
/// assert!(g.is_ok());
/// if let Ok(g) = g {
///     let results = g.generate(&mut rng);
///     assert!(!results.rhs.is_none());
/// }
///
/// let g = dice_nom::parse("attack badger");
/// assert!(!g.is_ok());
/// assert_eq!(g, Err("attack badger"));
/// ```
pub fn parse(input: &str) -> Result<Generator, &str> {
    match parsers::generator_parser(input) {
        Ok((_, g)) => Ok(g),
        Err(_) => Err(input),
    }
}
