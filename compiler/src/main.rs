use std::{io::stdin, process};

use crate::lexer::parse_to_lexer;

mod ast;
mod lexer;

fn main() {
    println!("parser playground; input code:");

    for line in stdin()
        .lines()
        .map(|l| l.unwrap_or_else(|_| process::exit(0)))
    {
        match parse_to_lexer(line.as_str()) {
            Ok((lex, _)) => println!("ok:  {lex:?}"),
            Err(e) => eprintln!("err: {e:?}"),
        }
    }
}
