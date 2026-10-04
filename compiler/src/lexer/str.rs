use crate::lexer::error::LexerError;
use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

// TODO: Resultを返す
pub fn parse_string(iter: &mut Peekable<Enumerate<Chars<'_>>>) -> Result<String, LexerError> {
    let mut str = String::new();

    while let Some((at, c)) = iter.next() {
        match c {
            '"' => {
                return Ok(str);
            }
            '\\' => {
                let ch = match iter
                    .next()
                    .ok_or_else(|| LexerError {
                        at,
                        message: "eof reached during string".to_string(),
                    })?
                    .1
                {
                    '0' => '\0',
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '"' => '"',
                    '\\' => '\\',
                    c => {
                        return Err(LexerError {
                            at,
                            message: format!("unknown string escape sequence: {c}"),
                        });
                    }
                };
                str.push(ch);
            }
            _ => str.push(c),
        }
    }

    Err(LexerError {
        at: usize::MAX,
        message: "eof reached during string".to_string(),
    })
}
