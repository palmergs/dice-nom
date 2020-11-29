pub mod results;

pub mod generators;
use generators::{ Generator, PoolGenerator };

pub mod parsers;

/// roller builds a simple PoolGenerator that can randomly generate dice rolls.
///
/// * Examples
/// 
/// ```
/// let roller = dice_nom::roller(3, 6, Some("**"));
/// assert_eq!(roller.count, 3);
/// assert_eq!(roller.range, 6);
/// assert_eq!(roller.op, Some(dice_nom::generators::PoolOp::ExplodeEachUntil(None)));
/// 
/// let pool = roller.generate();
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
    PoolGenerator{ count, range, op}
}

/// parse builds a generator from the given input string. If any of the string
/// can be parsed a generator is returned. If no generator can be built then
/// an error is returned with the input string.
/// 
/// * Examples
/// 
/// ```
/// let gen = dice_nom::parse("2d4! + 2d6! < 3d8!");
/// assert!(gen.is_ok());
/// if let Ok(gen) = gen {
///     let results = gen.generate();
///     assert!(!results.rhs.is_none());
/// }
/// ```
pub fn parse(input: &str) -> Result<Generator, &str> {
    match parsers::generator_parser(input) {
        Ok((_, gen)) => Ok(gen),
        Err(_) => Err(input),
    }
}