#[derive(Debug, PartialEq)]
pub struct LexerError {
    pub at: usize,
    pub message: String,
}
