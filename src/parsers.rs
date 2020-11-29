extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{char, digit0, digit1, space0},
    combinator::opt,
    multi::fold_many1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

use super::generators::{
    ArithOp, ArithTermGenerator, ComparisonOp, ExprGenerator, Generator, HitsGenerator,
    PoolGenerator, PoolOp, SuccGenerator, SuccessOp, TargetOp, TermGenerator,
};

/// generator_parser is the top level parser and builds a generator
/// that can compare the relative values of two sub expressions.
///
/// * Examples
///
/// ```
/// use dice_nom::parsers::generator_parser;
/// use dice_nom::generators::*;
/// let (input, gen) = generator_parser("3d8").unwrap();
/// assert_eq!(input, "");
/// assert_eq!(gen.op, None);
///
/// let (input, gen) = generator_parser("3d8 > 4d6").unwrap();
/// assert_eq!(input, "");
/// assert_eq!(gen.op, Some(ComparisonOp::GT(
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
    match tuple((succ_gen_parser, opt(comparison_op_parser)))(input) {
        Ok((input, (succ, op))) => Ok((input, Generator { succ, op })),
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
    match tuple((hits_parser, opt(alt((succ_op_parser, succ_next_op_parser)))))(input) {
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
    match tuple((pare_parser, opt(tgt_op_parser)))(input) {
        Ok((input, (expr, op))) => Ok((input, HitsGenerator { expr, op })),
        Err(e) => Err(e),
    }
}

fn pare_parser(input: &str) -> IResult<&str, ExprGenerator> {
    alt((
        delimited(
            tuple((space0, char('('), space0)),
            expr_parser,
            tuple((space0, char(')'), space0)),
        ),
        expr_parser,
    ))(input)
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
    match fold_many1(
        arith_term_parser,
        Vec::new(),
        |mut acc: Vec<_>, arith_term| {
            acc.push(arith_term);
            acc
        },
    )(input)
    {
        Ok((input, terms)) => Ok((input, ExprGenerator { terms })),
        Err(e) => Err(e),
    }
}

fn implicit_term_parser(input: &str) -> IResult<&str, ArithTermGenerator> {
    match preceded(space0, term_parser)(input) {
        Ok((input, term)) => Ok((
            input,
            ArithTermGenerator {
                op: ArithOp::ImplicitAdd,
                term: term,
            },
        )),
        Err(e) => Err(e),
    }
}

fn add_term_parser(input: &str) -> IResult<&str, ArithTermGenerator> {
    match preceded(delimited(space0, char('+'), space0), term_parser)(input) {
        Ok((input, term)) => Ok((
            input,
            ArithTermGenerator {
                op: ArithOp::Add,
                term: term,
            },
        )),
        Err(e) => Err(e),
    }
}

fn sub_term_parser(input: &str) -> IResult<&str, ArithTermGenerator> {
    match preceded(delimited(space0, char('-'), space0), term_parser)(input) {
        Ok((input, term)) => Ok((
            input,
            ArithTermGenerator {
                op: ArithOp::Sub,
                term: term,
            },
        )),
        Err(e) => Err(e),
    }
}

fn arith_term_parser(input: &str) -> IResult<&str, ArithTermGenerator> {
    alt((implicit_term_parser, add_term_parser, sub_term_parser))(input)
}

/// term_parser builds a TermGenerator from the given input.
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
    alt((pool_parser, const_parser))(input)
}

fn const_parser(input: &str) -> IResult<&str, TermGenerator> {
    match preceded(space0, digit1)(input) {
        Ok((input, chars)) => Ok((
            input,
            TermGenerator::Constant(chars.parse::<i32>().unwrap()),
        )),
        Err(e) => Err(e),
    }
}

fn pool_parser(input: &str) -> IResult<&str, TermGenerator> {
    match tuple((opt(digit1), is_a("dD"), range_parser, opt(pool_op_parser)))(input) {
        Ok((input, (count, _, range, op))) => {
            let count = match count {
                Some(chars) => chars.parse::<i32>().unwrap(),
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
    match alt((digit1, is_a("%")))(input) {
        Ok((input, chars)) => {
            if chars.chars().nth(0).unwrap() == '%' {
                let base = 10i32;
                let exp = chars.len() as u32;
                let n = match base.checked_pow(exp) {
                    Some(n) => 10 * n,
                    None => 100,
                };
                Ok((input, n))
            } else {
                Ok((input, chars.parse::<i32>().unwrap()))
            }
        }
        Err(e) => Err(e),
    }
}

fn tgt_high_parser(input: &str) -> IResult<&str, TargetOp> {
    match delimited(
        tuple((space0, char('['), space0)),
        digit1,
        tuple((space0, char(']'))),
    )(input)
    {
        Ok((input, chars)) => Ok((input, TargetOp::TargetHigh(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e),
    }
}

fn tgt_low_parser(input: &str) -> IResult<&str, TargetOp> {
    match delimited(
        tuple((space0, char('('), space0)),
        digit1,
        tuple((space0, char(')'))),
    )(input)
    {
        Ok((input, chars)) => Ok((input, TargetOp::TargetLow(chars.parse::<i32>().unwrap()))),
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
    alt((tgt_high_parser, tgt_low_parser))(input)
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
    match delimited(
        tuple((space0, char('{'), space0)),
        digit1,
        tuple((space0, char('}'))),
    )(input)
    {
        Ok((input, chars)) => Ok((input, SuccessOp::TargetSucc(chars.parse::<i32>().unwrap()))),
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
        tuple((char('{'), space0)),
        separated_pair(digit1, tuple((space0, char(','), space0)), digit1),
        tuple((space0, char('}'))),
    )(input)
    {
        Ok((input, (n, m))) => Ok((
            input,
            SuccessOp::TargetSuccNext(n.parse::<i32>().unwrap(), m.parse::<i32>().unwrap()),
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
    ))(input)
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
    match tuple((space0, digit0))(input) {
        Ok((input, (_, chars))) => {
            if chars.len() > 0 {
                Ok((input, Some(chars.parse::<i32>().unwrap())))
            } else {
                Ok((input, None))
            }
        }
        Err(e) => Err(e),
    }
}

fn explode_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((tag("!"), optional_num_parser))(input) {
        Ok((input, (_, num))) => Ok((input, PoolOp::Explode(num))),
        Err(e) => Err(e),
    }
}

fn explode_until_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((tag("!!"), optional_num_parser))(input) {
        Ok((input, (_, num))) => Ok((input, PoolOp::ExplodeUntil(num))),
        Err(e) => Err(e),
    }
}

fn explode_each_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((tag("*"), optional_num_parser))(input) {
        Ok((input, (_, num))) => Ok((input, PoolOp::ExplodeEach(num))),
        Err(e) => Err(e),
    }
}

fn explode_each_until_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((tag("**"), optional_num_parser))(input) {
        Ok((input, (_, num))) => Ok((input, PoolOp::ExplodeEachUntil(num))),
        Err(e) => Err(e),
    }
}

fn add_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((space0, tag("++"), space0, optional_num_parser))(input) {
        Ok((input, (_, _, _, num))) => Ok((input, PoolOp::AddEach(num))),
        Err(e) => Err(e),
    }
}

fn sub_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((space0, tag("--"), space0, optional_num_parser))(input) {
        Ok((input, (_, _, _, num))) => Ok((input, PoolOp::SubEach(num))),
        Err(e) => Err(e),
    }
}

fn take_mid_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((char('~'), digit1))(input) {
        Ok((input, (_, chars))) => Ok((input, PoolOp::TakeMid(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e),
    }
}

fn take_high_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((char('^'), digit1))(input) {
        Ok((input, (_, chars))) => Ok((input, PoolOp::TakeHigh(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e),
    }
}

fn take_low_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((char('`'), digit1))(input) {
        Ok((input, (_, chars))) => Ok((input, PoolOp::TakeLow(chars.parse::<i32>().unwrap()))),
        Err(e) => Err(e),
    }
}

fn command_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match delimited(space0, alt((tag("ADV"), tag("DIS"), tag("Y"))), space0)(input) {
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
        tuple((delimited(space0, tag("<=>"), space0), succ_gen_parser)),
        tuple((delimited(space0, tag(">="), space0), succ_gen_parser)),
        tuple((delimited(space0, tag("<="), space0), succ_gen_parser)),
        tuple((delimited(space0, tag(">"), space0), succ_gen_parser)),
        tuple((delimited(space0, tag("<"), space0), succ_gen_parser)),
        tuple((delimited(space0, tag("="), space0), succ_gen_parser)),
    ))(input)
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
