use crate::{interner::IdentifierInterner, lexer::parse_to_lexer};
use std::{io::stdin, process};

mod ast;
mod core;
mod interner;
mod lexer;

fn main() {
    println!("parser playground; input code:");

    let mut interner = IdentifierInterner::new();

    for line in stdin()
        .lines()
        .map(|l| l.unwrap_or_else(|_| process::exit(0)))
    {
        match parse_to_lexer(line.as_str(), &mut interner) {
            Ok(lex) => println!("ok:  {lex:?}"),
            Err(e) => eprintln!("{}", e.to_error_msg(&line, "{playground}")),
        }
    }
}
