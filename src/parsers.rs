//! Parsers for dice notation expressions using the nom parsing library.
//!
//! This module contains all the parsing logic for converting string representations
//! of dice expressions into generator structures. The parsers are built using the
//! nom parser combinator library and can handle complex nested expressions.
//!
//! # Parser Hierarchy
//!
//! The parsers are organized in a hierarchical structure:
//!
//! 1. [`generator_parser`] - Top-level parser for complete expressions with comparisons
//! 2. [`succ_gen_parser`] - Parses success-based expressions
//! 3. [`hits_parser`] - Parses hit-counting expressions with target operations
//! 4. [`expr_parser`] - Parses arithmetic expressions with multiple terms
//! 5. [`term_parser`] - Parses individual terms (pools or constants)
//! 6. Various operation parsers for modifiers and operators
//!
//! # Examples
//!
//! ```rust
//! use dice_nom::parsers::generator_parser;
//!
//! // Parse a simple dice expression
//! let result = generator_parser("3d6+4");
//! assert!(result.is_ok());
//!
//! // Parse a complex expression with exploding dice and target
//! let result = generator_parser("4d6!![4] > 2d8+1");
//! assert!(result.is_ok());
//! ```

extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{char, digit0, digit1, space0},
    combinator::opt,
    multi::fold_many1,
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

use super::generators::{
    ArithOp, ArithTermGenerator, ComparisonOp, ExprGenerator, Generator, HitsGenerator,
    MulDivGenerator, MulDivOp, PoolGenerator, PoolOp, SuccGenerator, SuccessOp, TargetOp,
    TermGenerator,
};

/// Parses the top-level generator that can compare two sub-expressions.
///
/// This is the entry point for parsing complete dice expressions, including
/// comparisons between two sides. It handles expressions like "3d6 > 2d8+1".
///
/// # Arguments
///
/// * `input` - The string slice to parse
///
/// # Returns
///
/// Returns a `Result` containing the remaining input and the parsed `Generator`,
/// or a nom parsing error.
///
/// # Examples
///
/// ```rust
/// use dice_nom::parsers::generator_parser;
///
/// let (remaining, gen) = generator_parser("3d6+4").unwrap();
/// assert_eq!(remaining, "");
///
/// let (remaining, gen) = generator_parser("2d20 > 15").unwrap();
/// assert_eq!(remaining, "");
/// ```
/// * Examples
///
/// ```
/// use dice_nom::parsers::generator_parser;
/// use dice_nom::generators::*;
/// let (input, g) = generator_parser("3d8").unwrap();
/// assert_eq!(input, "");
/// assert_eq!(g.op, None);
///
/// let (input, g) = generator_parser("3d8 > 4d6").unwrap();
/// assert_eq!(input, "");
/// assert_eq!(g.op, Some(ComparisonOp::GT(
///     SuccGenerator{
///         hits: HitsGenerator{
///             expr: ExprGenerator{
///                 terms: vec![
///                     ArithTermGenerator{
///                         op: ArithOp::ImplicitAdd,
///                         term: TermGenerator::Pool(PoolGenerator {
///                             count: 4,
///                             range: 6,
///                             op: None
///                         })
///                     }
///                 ]
///             },
///             op: None
///         },
///         op: None
///     }
/// )));
/// ```
pub fn generator_parser(input: &str) -> IResult<&str, Generator> {
    match (mul_div_parser, opt(comparison_op_parser)).parse(input) {
        Ok((input, (mul_div, op))) => Ok((input, Generator { mul_div, op })),
        Err(e) => Err(e),
    }
}

pub fn mul_div_parser(input: &str) -> IResult<&str, MulDivGenerator> {
    match (succ_gen_parser, opt(mul_div_op_parser)).parse(input) {
        Ok((input, (succ, op))) => Ok((input, MulDivGenerator { succ, op })),
        Err(e) => Err(e),
    }
}

/// succ_parser builds a generator from the input that returns the
/// level of success of the sum of the rolled dice.
///
/// * Examples
///
/// ```
/// use dice_nom::parsers::succ_gen_parser;
/// use dice_nom::generators::*;
/// let (input, succ) = succ_gen_parser("3d8 {15}").unwrap();
/// assert_eq!(input, "");
/// assert_eq!(succ.op, Some(SuccessOp::TargetSucc(15)));
///
/// let (input, succ) = succ_gen_parser("(4d4** + 5 + 2d12)").unwrap();
/// assert_eq!(input, "");
/// assert_eq!(succ.op, None);
///
/// // roll 10d6, count those that rolled 4 or less, check to see if 3 or more.
/// let (input, succ) = succ_gen_parser("10d6(4){3, 2}").unwrap();
/// assert_eq!(input, "");
/// assert_eq!(succ.op, Some(SuccessOp::TargetSuccNext(3, 2)));
/// ```
pub fn succ_gen_parser(input: &str) -> IResult<&str, SuccGenerator> {
    match (hits_parser, opt(alt((succ_op_parser, succ_next_op_parser)))).parse(input) {
        Ok((input, (hits, op))) => Ok((input, SuccGenerator { hits, op })),
        Err(e) => Err(e),
    }
}

/// hits_parser generates an expression that returns the number of
/// times the rolled dice exceed (or are below) an expected value.
///
/// * Examples
///
/// ```
/// use dice_nom::parsers::hits_parser;
/// use dice_nom::generators::*;
/// let (input, hits) = hits_parser("( 2d4 + 3d6 + 2d8 )[4]").unwrap();
/// assert_eq!(input, "");
/// assert_eq!(hits.expr.terms.len(), 3);
/// assert_eq!(hits.op, Some(TargetOp::TargetHigh(4)));
///
/// let (input, hits) = hits_parser("d4 d8 d10 d12 (3)").unwrap();
/// assert_eq!(input, "");
/// assert_eq!(hits.expr.terms.len(), 4);
/// assert_eq!(hits.op, Some(TargetOp::TargetLow(3)));
/// ```
pub fn hits_parser(input: &str) -> IResult<&str, HitsGenerator> {
    match (pare_parser, opt(tgt_op_parser)).parse(input) {
        Ok((input, (expr, op))) => Ok((input, HitsGenerator { expr, op })),
        Err(e) => Err(e),
    }
}

fn pare_parser(input: &str) -> IResult<&str, ExprGenerator> {
    alt((
        delimited(
            (space0, char('('), space0),
            expr_parser,
            (space0, char(')'), space0),
        ),
        expr_parser,
    ))
    .parse(input)
}

/// expr_parser builds a vector of terms
///
/// * Examples
///
/// ```
/// use dice_nom::parsers::expr_parser;
/// use dice_nom::generators::*;
/// let (input, expr) = expr_parser("3d4 + 2d6 - d8").unwrap();
/// assert_eq!(input, "");
/// assert_eq!(expr.terms.len(), 3);
/// assert_eq!(expr.terms[0].op, ArithOp::ImplicitAdd);
/// assert_eq!(expr.terms[1].op, ArithOp::Add);
/// assert_eq!(expr.terms[2].op, ArithOp::Sub);
/// ```
pub fn expr_parser(input: &str) -> IResult<&str, ExprGenerator> {
    let result = fold_many1(
        arith_term_parser,
        Vec::new,
        |mut acc: Vec<_>, arith_term| {
            acc.push(arith_term);
            acc
        },
    )
    .parse(input);
    match result {
        Ok((input, terms)) => Ok((input, ExprGenerator { terms })),
        Err(e) => Err(e),
    }
}

fn implicit_term_parser(input: &str) -> IResult<&str, ArithTermGenerator> {
    match preceded(space0, term_parser).parse(input) {
        Ok((input, term)) => Ok((
            input,
            ArithTermGenerator {
                op: ArithOp::ImplicitAdd,
                term,
            },
        )),
        Err(e) => Err(e),
    }
}

fn add_term_parser(input: &str) -> IResult<&str, ArithTermGenerator> {
    match preceded(delimited(space0, char('+'), space0), term_parser).parse(input) {
        Ok((input, term)) => Ok((
            input,
            ArithTermGenerator {
                op: ArithOp::Add,
                term,
            },
        )),
        Err(e) => Err(e),
    }
}

fn sub_term_parser(input: &str) -> IResult<&str, ArithTermGenerator> {
    match preceded(delimited(space0, char('-'), space0), term_parser).parse(input) {
        Ok((input, term)) => Ok((
            input,
            ArithTermGenerator {
                op: ArithOp::Sub,
                term,
            },
        )),
        Err(e) => Err(e),
    }
}

fn arith_term_parser(input: &str) -> IResult<&str, ArithTermGenerator> {
    alt((implicit_term_parser, add_term_parser, sub_term_parser)).parse(input)
}

/// `term_parser` builds a `TermGenerator` from the given input.
///
/// # Examples
///
/// ```
/// use dice_nom::parsers::term_parser;
/// use dice_nom::generators::{TermGenerator, PoolGenerator, PoolOp};
/// assert_eq!(term_parser("10 "), Ok((" ", TermGenerator::Constant(10))));
/// assert_eq!(term_parser("2d6**"), Ok((
///     "",
///     TermGenerator::Pool(PoolGenerator{
///         count: 2,
///         range: 6,
///         op: Some(PoolOp::ExplodeEachUntil(None)) }))
/// ));
/// assert_eq!(term_parser("3d10!!4"), Ok((
///     "",
///     TermGenerator::Pool(PoolGenerator{
///         count: 3,
///         range: 10,
///         op: Some(PoolOp::ExplodeUntil(Some(4))) }))
/// ));
/// ```
pub fn term_parser(input: &str) -> IResult<&str, TermGenerator> {
    alt((pool_parser, const_parser)).parse(input)
}

fn const_parser(input: &str) -> IResult<&str, TermGenerator> {
    match preceded(space0, digit1).parse(input) {
        Ok((input, chars)) => Ok((
            input,
            TermGenerator::Constant(chars.parse::<i32>().unwrap()),
        )),
        Err(e) => Err(e),
    }
}

fn pool_parser(input: &str) -> IResult<&str, TermGenerator> {
    match (opt(digit1), is_a("dD"), range_parser, opt(pool_op_parser)).parse(input) {
        Ok((input, (count, _, range, op))) => {
            let count = match count {
                Some(chars) => {
                    let n = chars.parse::<i32>().unwrap();
                    if n > 100 {
                        100
                    } else {
                        n
                    }
                }
                None => 1,
            };

            Ok((
                input,
                TermGenerator::Pool(PoolGenerator { count, range, op }),
            ))
        }
        Err(e) => Err(e),
    }
}

/// range_parser handles the special case of using `%` to mean 100.
/// This is expanded to allow for any number of `%` to indicate a
/// larger number (until the maximum value in `i32` is reached).
///
/// # Examples
///
/// ```
/// use dice_nom::parsers::range_parser;
/// assert_eq!(range_parser("1234[12]"), Ok(("[12]", 1234)));
/// assert_eq!(range_parser("%[12]"), Ok(("[12]", 100)));
/// assert_eq!(range_parser("%%test"), Ok(("test", 1000)));
/// assert_eq!(range_parser("%%%4567"), Ok(("4567", 10000)));
/// ```
pub fn range_parser(input: &str) -> IResult<&str, i32> {
    match alt((digit1, is_a("%"))).parse(input) {
        Ok((input, chars)) => {
            if chars.starts_with('%') {
                let base = 10i32;
                let exp = chars.len() as u32;
                let n = match base.checked_pow(exp) {
                    Some(n) => clamp(10 * n, 100, 10000),
                    None => 100,
                };
                Ok((input, n))
            } else {
                Ok((input, clamp(chars.parse::<i32>().unwrap(), 1, 2000)))
            }
        }
        Err(e) => Err(e),
    }
}

fn tgt_high_parser(input: &str) -> IResult<&str, TargetOp> {
    match delimited((space0, char('['), space0), digit1, (space0, char(']'))).parse(input) {
        Ok((input, chars)) => Ok((
            input,
            TargetOp::TargetHigh(clamp(chars.parse::<i32>().unwrap(), 1, 1000)),
        )),
        Err(e) => Err(e),
    }
}

fn tgt_low_parser(input: &str) -> IResult<&str, TargetOp> {
    match delimited((space0, char('('), space0), digit1, (space0, char(')'))).parse(input) {
        Ok((input, chars)) => Ok((
            input,
            TargetOp::TargetLow(clamp(chars.parse::<i32>().unwrap(), 1, 1000)),
        )),
        Err(e) => Err(e),
    }
}

/// tgt_op_parser builds a target comparison operator
///
/// # Examples
///
/// ```
/// use dice_nom::parsers::tgt_op_parser;
/// use dice_nom::generators::TargetOp;
/// assert_eq!(tgt_op_parser("[12]"), Ok(("", TargetOp::TargetHigh(12))));
/// assert_eq!(tgt_op_parser("[ 12 ]"), Ok(("", TargetOp::TargetHigh(12))));
/// assert_eq!(tgt_op_parser("(12)"), Ok(("", TargetOp::TargetLow(12))));
/// assert_eq!(tgt_op_parser("( 12 )"), Ok(("", TargetOp::TargetLow(12))));
/// ```
pub fn tgt_op_parser(input: &str) -> IResult<&str, TargetOp> {
    alt((tgt_high_parser, tgt_low_parser)).parse(input)
}

/// succ_op_parser builds a success comparison operator
///
/// # Examples
///
/// ```
/// use dice_nom::parsers::succ_op_parser;
/// use dice_nom::generators::SuccessOp;
/// assert_eq!(succ_op_parser("{123}"), Ok(("", SuccessOp::TargetSucc(123))));
/// assert_eq!(succ_op_parser("{ 123 }"), Ok(("", SuccessOp::TargetSucc(123))));
/// ```
pub fn succ_op_parser(input: &str) -> IResult<&str, SuccessOp> {
    match delimited((space0, char('{'), space0), digit1, (space0, char('}'))).parse(input) {
        Ok((input, chars)) => Ok((
            input,
            SuccessOp::TargetSucc(clamp(chars.parse::<i32>().unwrap(), 1, 1000)),
        )),
        Err(e) => Err(e),
    }
}

/// succ_next_op_parser builds a success comparison operator
///
/// # Examples
///
/// ```
/// use dice_nom::parsers::succ_next_op_parser;
/// use dice_nom::generators::SuccessOp;
/// assert_eq!(succ_next_op_parser("{123,45}"), Ok(("", SuccessOp::TargetSuccNext(123, 45))));
/// assert_eq!(succ_next_op_parser("{ 123, 45 }"), Ok(("", SuccessOp::TargetSuccNext(123, 45))));
/// ```
pub fn succ_next_op_parser(input: &str) -> IResult<&str, SuccessOp> {
    match delimited(
        (char('{'), space0),
        separated_pair(digit1, (space0, char(','), space0), digit1),
        (space0, char('}')),
    )
    .parse(input)
    {
        Ok((input, (n, m))) => Ok((
            input,
            SuccessOp::TargetSuccNext(
                clamp(n.parse::<i32>().unwrap(), 1, 1000),
                clamp(m.parse::<i32>().unwrap(), 1, 1000),
            ),
        )),
        Err(e) => Err(e),
    }
}

/// pool_op_parser parses an operator that can act on pools of dice.
///
/// # Examples
///
/// ```
/// use dice_nom::parsers::pool_op_parser;
/// use dice_nom::generators::PoolOp;
/// assert_eq!(pool_op_parser("!"), Ok(("", PoolOp::Explode(None))));
/// assert_eq!(pool_op_parser(" ++ 3"), Ok(("", PoolOp::AddEach(Some(3)))));
/// assert_eq!(pool_op_parser(" ADV"), Ok(("", PoolOp::Advantage)));
/// ```
pub fn pool_op_parser(input: &str) -> IResult<&str, PoolOp> {
    alt((
        explode_until_op_parser,
        explode_op_parser,
        explode_each_until_op_parser,
        explode_each_op_parser,
        add_op_parser,
        sub_op_parser,
        take_mid_op_parser,
        take_high_op_parser,
        take_low_op_parser,
        command_op_parser,
    ))
    .parse(input)
}

/// optional_num_parser wraps `digit1` to return an optional i32.
///
/// # Arguments
///
/// `input` - a string slice to be parsed
///
/// # Examples
///
/// ```
/// use dice_nom::parsers::optional_num_parser;
/// assert_eq!(optional_num_parser("test"), Ok(("test", None)));
/// assert_eq!(optional_num_parser("123test"), Ok(("test", Some(123))));
/// assert_eq!(optional_num_parser("  123test"), Ok(("test", Some(123))));
/// ```
pub fn optional_num_parser(input: &str) -> IResult<&str, Option<i32>> {
    match (space0, digit0).parse(input) {
        Ok((input, (_, chars))) => {
            if !chars.is_empty() {
                Ok((input, Some(clamp(chars.parse::<i32>().unwrap(), 0, 1000))))
            } else {
                Ok((input, None))
            }
        }
        Err(e) => Err(e),
    }
}

fn explode_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match (tag("!"), optional_num_parser).parse(input) {
        Ok((input, (_, num))) => Ok((input, PoolOp::Explode(num))),
        Err(e) => Err(e),
    }
}

fn explode_until_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match (tag("!!"), optional_num_parser).parse(input) {
        Ok((input, (_, num))) => Ok((input, PoolOp::ExplodeUntil(num))),
        Err(e) => Err(e),
    }
}

fn explode_each_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match (tag("*"), optional_num_parser).parse(input) {
        Ok((input, (_, num))) => Ok((input, PoolOp::ExplodeEach(num))),
        Err(e) => Err(e),
    }
}

fn explode_each_until_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match (tag("**"), optional_num_parser).parse(input) {
        Ok((input, (_, num))) => Ok((input, PoolOp::ExplodeEachUntil(num))),
        Err(e) => Err(e),
    }
}

fn add_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match (space0, tag("++"), space0, optional_num_parser).parse(input) {
        Ok((input, (_, _, _, num))) => Ok((input, PoolOp::AddEach(num))),
        Err(e) => Err(e),
    }
}

fn sub_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match (space0, tag("--"), space0, optional_num_parser).parse(input) {
        Ok((input, (_, _, _, num))) => Ok((input, PoolOp::SubEach(num))),
        Err(e) => Err(e),
    }
}

fn take_mid_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match (char('~'), digit1).parse(input) {
        Ok((input, (_, chars))) => Ok((input, PoolOp::TakeMid(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e),
    }
}

fn take_high_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match (char('^'), digit1).parse(input) {
        Ok((input, (_, chars))) => Ok((input, PoolOp::TakeHigh(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e),
    }
}

fn take_low_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match (char('`'), digit1).parse(input) {
        Ok((input, (_, chars))) => Ok((input, PoolOp::TakeLow(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e),
    }
}

fn command_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match delimited(space0, alt((tag("ADV"), tag("DIS"), tag("Y"))), space0).parse(input) {
        Ok((input, op)) => match op {
            "ADV" => Ok((input, PoolOp::Advantage)),
            "DIS" => Ok((input, PoolOp::Disadvantage)),
            "Y" => Ok((input, PoolOp::BestGroup)),
            _ => panic!("unexpected tag in reroll op parser"),
        },
        Err(e) => Err(e),
    }
}

fn comparison_op_parser(input: &str) -> IResult<&str, ComparisonOp> {
    match alt((
        (delimited(space0, tag("<=>"), space0), succ_gen_parser),
        (delimited(space0, tag(">="), space0), succ_gen_parser),
        (delimited(space0, tag("<="), space0), succ_gen_parser),
        (delimited(space0, tag(">"), space0), succ_gen_parser),
        (delimited(space0, tag("<"), space0), succ_gen_parser),
        (delimited(space0, tag("="), space0), succ_gen_parser),
    ))
    .parse(input)
    {
        Ok((input, (tag, succ))) => match tag {
            "<=>" => Ok((input, ComparisonOp::CMP(succ))),
            ">=" => Ok((input, ComparisonOp::GE(succ))),
            "<=" => Ok((input, ComparisonOp::LE(succ))),
            ">" => Ok((input, ComparisonOp::GT(succ))),
            "<" => Ok((input, ComparisonOp::LT(succ))),
            "=" => Ok((input, ComparisonOp::EQ(succ))),
            _ => panic!("unexpected tag"),
        },
        Err(e) => Err(e),
    }
}

fn mul_div_op_parser(input: &str) -> IResult<&str, MulDivOp> {
    match alt((
        (space0, tag("x"), space0, digit1),
        (space0, tag("/"), space0, digit1),
    ))
    .parse(input)
    {
        Ok((input, (_, op, _, num))) => match op {
            "x" => Ok((
                input,
                MulDivOp::Mul(clamp(num.parse::<i32>().unwrap(), 0, 100)),
            )),
            "/" => Ok((
                input,
                MulDivOp::Div(clamp(num.parse::<i32>().unwrap(), 1, 100)),
            )),
            _ => panic!("unexpected op"),
        },
        Err(e) => Err(e),
    }
}

fn clamp(n: i32, min: i32, max: i32) -> i32 {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}
