use crate::lexer::{chars_while::CharsWhile, error::LexerError};
use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

#[derive(Debug, PartialEq)]
pub enum NumlikeTok {
    Smi(i32),
    Bigint(String),
    PlusSymbol,
    MinusSymbol,
}

fn get_ints(iter: &mut Peekable<Enumerate<Chars<'_>>>, radix: u32) -> String {
    CharsWhile::new(iter, |c| c.is_digit(radix)).collect::<String>()
}

fn calc_exp_num(num: i32, exp: u32) -> Option<i32> {
    10i32.checked_pow(exp)?.checked_mul(num)
}

pub fn try_iter_to_num(
    first: char,
    iter: &mut Peekable<Enumerate<Chars<'_>>>,
) -> Result<NumlikeTok, LexerError> {
    let is_negative = first == '-';

    let first_n = if first == '-' || first == '+' {
        match iter.peek().copied() {
            Some((_, c)) if c.is_ascii_digit() => {
                iter.next();
                c
            }
            _ => {
                return Ok(if first == '-' {
                    NumlikeTok::MinusSymbol
                } else {
                    NumlikeTok::PlusSymbol
                });
            }
        }
    } else {
        first
    };

    let radix: u32 = if first_n == '0'
        && let Some((_, c)) = iter.peek()
    {
        match c {
            'x' => {
                iter.next();
                16
            }
            'o' => {
                iter.next();
                8
            }
            'b' => {
                iter.next();
                2
            }
            _ if c.is_ascii_digit() => {
                eprintln!(
                    "warn: javascript-like octet syntax(e.g. `0547`) is not supported, parses as decimal"
                );
                10
            }
            _ => return Ok(NumlikeTok::Smi(0)),
        }
    } else {
        10
    };

    let mut int = String::new();
    if is_negative {
        int.push('-');
    }
    if radix == 10 {
        int.push(first_n);
    }
    int.push_str(&get_ints(iter, radix));

    if int.is_empty() {
        return Err(LexerError {
            at: iter.peek().map(|(i, _)| *i).unwrap_or(usize::MAX),
            message: "radix syntax must have num part like: 0x1".to_string(),
        });
    }

    match iter.peek().copied() {
        Some((at, '.')) => {
            return Err(LexerError {
                at,
                message: "float syntax not supported".to_string(),
            });
        }
        Some((_, 'n')) => Ok(NumlikeTok::Bigint(int)),
        Some((i, _)) => match i32::from_str_radix(&int, radix) {
            Ok(n) => Ok(NumlikeTok::Smi(n)),
            Err(e) => Err(LexerError {
                at: i,
                message: e.to_string(),
            }),
        },
        None => match i32::from_str_radix(&int, radix) {
            Ok(n) => Ok(NumlikeTok::Smi(n)),
            Err(e) => Err(LexerError {
                at: usize::MAX,
                message: e.to_string(),
            }),
        },
    }
    /*/

    let float = if let Some(&(i, _)) = iter.peek() {
        iter.next();
        let float = CharsWhile::new(iter, |c| c.is_digit(radix)).collect::<String>();
        if float.is_empty() {
            return TryToNumstrResult::IncorrectSyntax("invalid char after '.'".to_string(), i);
        }
        Some(float)
    } else {
        None
    };*/
}

#[cfg(test)]
mod tests {
    use super::*;

    fn try_parse(source: &str) -> Result<NumlikeTok, LexerError> {
        let mut iter = source.chars().enumerate().peekable();
        let (_, first_char) = iter.next().expect("testcase err: need first char ;)");
        try_iter_to_num(first_char, &mut iter)
    }

    #[test]
    fn plain_test() {
        let cases: &[(&'static str, i32)] = &[
            ("1048", 1048),
            ("-3015", -3015),
            ("-0x14", -0x14),
            ("0b1011", 0b1011),
            ("0o107", 0o107),
            ("0101", 0101),
        ];
        for (s, n) in cases {
            assert_eq!(try_parse(s), Ok(NumlikeTok::Smi(*n)));
        }
    }

    #[test]
    fn decimal() {
        for i in 0u8..4 {
            let neg = i & 0b00000001 != 0;
            let spc = i & 0b00000010 != 0;

            let mut str = String::new();
            if neg {
                str.push('-');
            }
            str.push_str("42");
            if spc {
                str.push(' ');
            }
            let mut expect_val: i32 = 42;
            if neg {
                expect_val = -expect_val;
            }
            assert_eq!(try_parse(&str), Ok(NumlikeTok::Smi(expect_val)));
        }
    }
    // TODO: hex/oct/binの網羅テスト
}
