#[derive(Debug, PartialEq)]
pub struct LexerError {
    pub at: usize,
    pub message: String,
}

fn charat_to_lncol(at: usize, code: &str) -> (usize, usize) {
    let mut lines = 1usize;
    let mut cols = 1usize;

    for (c, _) in code.chars().zip(0..at) {
        if c == '\n' {
            lines += 1;
            cols = 1;
        } else {
            cols += 1;
        }
    }

    (lines, cols)
}

impl LexerError {
    pub fn to_error_msg(&self, code: &str, fpath: &str) -> String {
        let (ln, col) = charat_to_lncol(self.at, code);
        format!("LexerError: {} at {fpath}:{ln}:{col}", self.message)
    }
}
