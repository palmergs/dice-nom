pub(self) mod parsers {
    use dice_nom::Roller;

    use nom::{
        IResult,
        error,
        sequence,
        sequence::{delimited, tuple},
        character::complete::{char, digit0, digit1},
        bytes::complete::is_not,
    };

    fn parens(input: &str) -> IResult<&str, &str> {
        delimited(char('('), is_not(")"), char(')'))(input)
    }

    fn die(input: &str) -> Option<Roller> {
        let (count, input) = match digit0::<&str, error::ParseError<&str>>(input) {
            Ok(tuple) => {
                if tuple.0 == "" {
                    (1u32, tuple.0)
                } else {
                    (tuple.1.parse::<u32>().unwrap(), tuple.0)
                }
            },
            _ => return None
        };

        let input = match char('d')(input) {
            Ok(tuple) => tuple.0,
            _ => return None
        };

        let (range, input) = match digit1(input) {
            Ok(tuple) => (tuple.1.parse::<u32>().unwrap(), tuple.0),
            _ => return None
        };

        Some(Roller{ count, range })
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_parens() {
            assert_eq!(parens("(test)"), Ok(("", "test")));
            assert_eq!(parens("(inner)after"), Ok(("after", "inner")));
            assert_eq!(
                parens("no match"), 
                Err(nom::Err::Error(error::Error { input: "no match", code: error::ErrorKind::Char })));
        }

        #[test]
        fn test_die() {
            assert_eq!(die("3d6"), Some(Roller{ count: 3, range: 6 }));
        }
    }
}
