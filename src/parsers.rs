extern crate nom;

use nom::{
    IResult,
    bytes::complete::{tag, is_a},
    character::complete::{char, space0, digit0, digit1},
    combinator::opt,
    sequence::{tuple, delimited, separated_pair},
    branch::alt,
};

use super::generators::{
    Generator,
    SuccGenerator,
    ExprGenerator,
    HitsGenerator,
    TermGenerator,
    PoolGenerator,
    SuccessOp,
    PoolOp,
    ComparisonOp,
    TargetOp,
};

pub fn generator_parser(input: &str) -> IResult<&str, Generator> {
    match tuple((succ_gen_parser, opt(comparison_op_parser)))(input) {
        Ok((input, (succ, op))) => Ok((input, Generator{ succ, op })),
        Err(e) => Err(e)
    }
}

fn succ_gen_parser(input: &str) -> IResult<&str, SuccGenerator> {
    match tuple((
        expr_parser, 
        opt(alt((succ_op_parser, succ_next_op_parser)))
    ))(input) {
        Ok((input, (expr, op))) => Ok((input, SuccGenerator{ expr, op })),
        Err(e) => Err(e)
    }
}

fn pare_parser(input: &str) -> IResult<&str, ExprGenerator> {
    alt((
        delimited(
            tuple((space0, char('('), space0)),
            expr_parser, 
            tuple((space0, char(')'), space0))
        ),
        expr_parser
    ))(input)
}

fn expr_parser(input: &str) -> IResult<&str, ExprGenerator> {
   Ok((input, ExprGenerator{})) 
}

fn hits_parser(input: &str) -> IResult<&str, HitsGenerator> {
    match tuple((term_parser, opt(tgt_op_parser)))(input) {
        Ok((input, (term, op))) => Ok((input, HitsGenerator{ term, op })),
        Err(e) => Err(e)
    }
}

fn term_parser(input: &str) -> IResult<&str, TermGenerator> {
    alt((pool_parser, const_parser))(input)
}

fn const_parser(input: &str) -> IResult<&str, TermGenerator> {
    match digit1(input) {
        Ok((input, chars)) => Ok((
            input, 
            TermGenerator::Constant(chars.parse::<i32>().unwrap())
        )),
        Err(e) => Err(e)
    }
}

fn pool_parser(input: &str) -> IResult<&str, TermGenerator> {
    match tuple((
        opt(digit1),
        is_a("dD"),
        range_parser,
        opt(pool_op_parser)
    ))(input) {
        Ok((input, (count, _, range, op))) => {
            let count = match count {
                Some(chars) => chars.parse::<i32>().unwrap(),
                None => 1
            };
            Ok((input, TermGenerator::Pool(PoolGenerator{ count, range, op })))
        },
        Err(e) => Err(e)
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
                    None => 100
                };
                Ok((input, n))
            } else {
                Ok((input, chars.parse::<i32>().unwrap()))
            }
        },
        Err(e) => Err(e)
    }
}

fn tgt_high_parser(input: &str) -> IResult<&str, TargetOp> {
    match delimited(
        tuple((char('['), space0)),
        digit1, 
        tuple((space0, char(']')))
    )(input) {
        Ok((input, chars)) => Ok((
            input, 
            TargetOp::TargetHigh(chars.parse::<i32>().unwrap())
        )),
        Err(e) => Err(e),
    }
}

fn tgt_low_parser(input: &str) -> IResult<&str, TargetOp> {
    match delimited(
        tuple((char('('), space0)),
        digit1, 
        tuple((space0, char(')')))
    )(input) {
        Ok((input, chars)) => Ok((
            input, 
            TargetOp::TargetLow(chars.parse::<i32>().unwrap())
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
        tuple((char('{'), space0)), 
        digit1, 
        tuple((space0, char('}')))
    )(input) {

        Ok((input, chars)) => Ok((
            input, 
            SuccessOp::TargetSucc(chars.parse::<i32>().unwrap())
        )),
        Err(e) => Err(e) 
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
        tuple((space0, char('}')))
    )(input) {
        Ok((input, (n, m))) => Ok((
            input, 
            SuccessOp::TargetSuccNext(n.parse::<i32>().unwrap(), m.parse::<i32>().unwrap())
        )),
        Err(e) => Err(e)
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
        explode_op_parser, 
        explode_until_op_parser,
        explode_each_op_parser,
        explode_each_until_op_parser,
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
        },
        Err(e) => Err(e)
    }
}

fn explode_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((tag("!"), optional_num_parser))(input) {
        Ok((input, (_, num))) => Ok((input, PoolOp::Explode(num))),
        Err(e) => Err(e)
    }
}

fn explode_until_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((tag("!!"), optional_num_parser))(input) {
        Ok((input, (_, num))) => Ok((input, PoolOp::ExplodeUntil(num))),
        Err(e) => Err(e)
    }
}

fn explode_each_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((tag("*"), optional_num_parser))(input) {
        Ok((input, (_, num))) => Ok((input, PoolOp::ExplodeEach(num))),
        Err(e) => Err(e)
    }
}

fn explode_each_until_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((tag("**"), optional_num_parser))(input) {
        Ok((input, (_, num))) => Ok((input, PoolOp::ExplodeEachUntil(num))),
        Err(e) => Err(e)
    }
}

fn add_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((space0, tag("++"), space0, optional_num_parser))(input) {
        Ok((input, (_, _, _, num))) => Ok((input, PoolOp::AddEach(num))),
        Err(e) => Err(e)
    }
}

fn sub_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((space0, tag("--"), space0, optional_num_parser))(input) {
        Ok((input, (_, _, _, num))) => Ok((input, PoolOp::SubEach(num))),
        Err(e) => Err(e)
    }
}

fn take_mid_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((char('~'), digit1))(input) {
        Ok((input, (_, chars))) => Ok((
            input, 
            PoolOp::TakeMid(chars.parse::<i32>().unwrap())
        )),
        Err(e) => Err(e)
    }
}

fn take_high_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((char('^'), digit1))(input) {
        Ok((input, (_, chars))) => Ok((
            input, 
            PoolOp::TakeHigh(chars.parse::<i32>().unwrap())
        )),
        Err(e) => Err(e)
    }
}

fn take_low_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match tuple((char('`'), digit1))(input) {
        Ok((input, (_, chars))) => Ok((
            input, 
            PoolOp::TakeLow(chars.parse::<i32>().unwrap())
        )),
        Err(e) => Err(e)
    }
}

fn command_op_parser(input: &str) -> IResult<&str, PoolOp> {
    match delimited(
        space0,
        alt((tag("ADV"), tag("DIS"), tag("Y"))),
        space0
    )(input) {
        Ok((input, op)) => match op {
            "ADV" => Ok((input, PoolOp::Advantage)),
            "DIS" => Ok((input, PoolOp::Disadvantage)),
            "Y" => Ok((input, PoolOp::BestGroup)),
            _ => panic!("unexpected tag in reroll op parser")
        },
        Err(e) => Err(e)
    }
}

fn comparison_op_parser(input: &str) -> IResult<&str, ComparisonOp> {
    match alt((
        tuple((tag(">="), succ_gen_parser)),
        tuple((tag("<="), succ_gen_parser)),
        tuple((tag(">"), succ_gen_parser)),
        tuple((tag("<"), succ_gen_parser)),
        tuple((tag("="), succ_gen_parser))
    ))(input) {
        Ok((input, (tag, succ))) => match tag {
            ">=" => Ok((input, ComparisonOp::GE(succ))),
            "<=" => Ok((input, ComparisonOp::LE(succ))),
            ">"  => Ok((input, ComparisonOp::GT(succ))),
            "<"  => Ok((input, ComparisonOp::LT(succ))),
            "="  => Ok((input, ComparisonOp::EQ(succ))),
            _    => panic!("unexpected tag")
        }
        Err(e) => Err(e)
    }
}
