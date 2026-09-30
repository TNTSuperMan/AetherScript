use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use crate::lexer::chars_while::CharsWhile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Radix {
    Hex,
    Decimal,
    Octet,
    Binary,
}
impl Radix {
    pub fn radix(&self) -> u32 {
        match self {
            Self::Hex => 16,
            Self::Decimal => 10,
            Self::Octet => 8,
            Self::Binary => 2,
        }
    }
    pub fn is_correct_char(&self, c: char) -> bool {
        c.is_digit(self.radix())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumString {
    pub sign: Sign,
    pub radix: Radix,
    pub int: String,
    pub float: Option<String>,
}

pub enum TryToNumstrResult {
    Ok(NumString),
    MinusSymbol,
    IncorrectSyntax(String, usize),
}

pub fn try_iter_to_numstr(
    first: char,
    iter: &mut Peekable<Enumerate<Chars<'_>>>,
) -> TryToNumstrResult {
    let sign = if first == '-' {
        Sign::Negative
    } else {
        Sign::Positive
    };

    let first_n = if first == '-' {
        match iter.peek() {
            Some((_, c)) if c.is_ascii_digit() => *c,
            _ => return TryToNumstrResult::MinusSymbol,
        }
    } else {
        first
    };

    let radix = if first_n == '0'
        && let Some((_, c)) = iter.peek()
    {
        match c {
            'x' => {
                iter.next();
                Radix::Hex
            }
            'b' => {
                iter.next();
                Radix::Binary
            }
            _ if c.is_ascii_digit() => Radix::Octet,
            _ => {
                return TryToNumstrResult::Ok(NumString {
                    sign,
                    radix: Radix::Decimal,
                    int: String::from('0'),
                    float: None,
                });
            }
        }
    } else {
        Radix::Decimal
    };

    let int = CharsWhile::new(iter, |c| radix.is_correct_char(c)).collect::<String>();

    let float = if let Some(&(i, _)) = iter.peek() {
        iter.next();
        let float = CharsWhile::new(iter, |c| radix.is_correct_char(c)).collect::<String>();
        if float.is_empty() {
            return TryToNumstrResult::IncorrectSyntax("invalid char after '.'".to_string(), i);
        }
        Some(float)
    } else {
        None
    };

    TryToNumstrResult::Ok(NumString {
        sign,
        radix,
        int,
        float,
    })
}
