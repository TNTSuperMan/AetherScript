#[derive(Debug)]
pub enum LexerError {
    UnknownChar(char),
    Syntax(String, usize),
}
