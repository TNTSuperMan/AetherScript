#[derive(Debug)]
pub struct LexerError {
    pub at: usize,
    pub message: String,
}
