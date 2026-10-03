use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use crate::lexer::chars_while::CharsWhile;

#[derive(Debug, PartialEq)]
pub enum TryToNumstrResult {
    Int(i64),
    PlusSymbol,
    MinusSymbol,
    IncorrectSyntax(String, usize),
}

fn get_ints(iter: &mut Peekable<Enumerate<Chars<'_>>>, radix: u32) -> String {
    CharsWhile::new(iter, |c| c.is_digit(radix)).collect::<String>()
}

fn calc_exp_num(num: i64, exp: u32) -> Option<i64> {
    10i64.checked_pow(exp)?.checked_mul(num)
}

pub fn try_iter_to_num(
    first: char,
    iter: &mut Peekable<Enumerate<Chars<'_>>>,
) -> TryToNumstrResult {
    let is_negative = first == '-';

    let first_n = if first == '-' || first == '+' {
        match iter.peek().copied() {
            Some((_, c)) if c.is_ascii_digit() => {
                iter.next();
                c
            }
            _ => {
                return if first == '-' {
                    TryToNumstrResult::MinusSymbol
                } else {
                    TryToNumstrResult::PlusSymbol
                };
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
            _ => return TryToNumstrResult::Int(0),
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
        return TryToNumstrResult::IncorrectSyntax(
            "radix syntax must have num part like: 0x1".to_string(),
            iter.peek().map(|(i, _)| *i).unwrap_or(usize::MAX),
        );
    }

    match iter.peek().copied() {
        Some((i, '.')) => {
            return TryToNumstrResult::IncorrectSyntax("float syntax not supported".to_string(), i);
        }
        Some((i, 'e' | 'E')) => {
            iter.next();
            if radix != 10 {
                return TryToNumstrResult::IncorrectSyntax(
                    "radix syntax cant use with exp".to_string(),
                    i,
                );
            }
            let exp = get_ints(iter, 10);
            if exp.is_empty() {
                return TryToNumstrResult::IncorrectSyntax(
                    "exp syntax must have num part like: 0x1".to_string(),
                    i,
                );
            }
            let Ok(exp_num) = exp.parse::<u32>() else {
                return TryToNumstrResult::IncorrectSyntax("exp num too big".to_string(), i);
            };
            let Ok(n) = int.parse::<i64>() else {
                return TryToNumstrResult::IncorrectSyntax("num too big".to_string(), i);
            };
            match calc_exp_num(n, exp_num) {
                Some(n) => TryToNumstrResult::Int(n),
                None => TryToNumstrResult::IncorrectSyntax("number too big".to_string(), i),
            }
        }
        Some((i, _)) => match i64::from_str_radix(&int, radix) {
            Ok(n) => TryToNumstrResult::Int(n),
            Err(e) => TryToNumstrResult::IncorrectSyntax(e.to_string(), i),
        },
        None => match i64::from_str_radix(&int, radix) {
            Ok(n) => TryToNumstrResult::Int(n),
            Err(e) => TryToNumstrResult::IncorrectSyntax(e.to_string(), usize::MAX),
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

    fn try_parse(source: &str) -> TryToNumstrResult {
        let mut iter = source.chars().enumerate().peekable();
        let (_, first_char) = iter.next().expect("testcase err: need first char ;)");
        try_iter_to_num(first_char, &mut iter)
    }

    #[test]
    fn plain_test() {
        let cases: &[(&'static str, i64)] = &[
            ("1048", 1048),
            ("-3015", -3015),
            ("1004e7", 10040000000),
            ("-0x14", -0x14),
            ("0b1011", 0b1011),
            ("0o107", 0o107),
            ("0101", 0101),
        ];
        for (s, n) in cases {
            assert_eq!(try_parse(s), TryToNumstrResult::Int(*n));
        }
    }

    #[test]
    fn decimal_and_exp() {
        for i in 0u8..8 {
            let neg = i & 0b00000001 != 0;
            let exp = i & 0b00000010 != 0;
            let spc = i & 0b00000100 != 0;

            let mut str = String::new();
            if neg {
                str.push('-');
            }
            str.push_str("42");
            if exp {
                str.push_str("e13");
            }
            if spc {
                str.push(' ');
            }
            let mut expect_val: i64 = 42;
            if neg {
                expect_val = -expect_val;
            }
            if exp {
                expect_val = expect_val * 10000000000000;
            }
            assert_eq!(try_parse(&str), TryToNumstrResult::Int(expect_val));
        }
    }
    // TODO: hex/oct/binの網羅テスト
}
