use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

// TODO: Resultを返す
pub fn parse_string(iter: &mut Peekable<Enumerate<Chars<'_>>>) -> Option<String> {
    let mut str = String::new();

    while let Some((_, c)) = iter.next() {
        match c {
            '"' => {
                return Some(str);
            }
            '\\' => {
                let ch = match iter.next()?.1 {
                    '0' => '\0',
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '"' => '"',
                    '\\' => '\\',
                    _ => return None,
                };
                str.push(ch);
            }
            _ => str.push(c),
        }
    }

    None
}
