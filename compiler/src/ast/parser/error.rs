pub struct AstError {
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

impl AstError {
    pub fn to_error_msg(&self, code: &str, fpath: &str) -> String {
        let (ln, col) = charat_to_lncol(self.at, code);
        format!("AstError: {} at {fpath}:{ln}:{col}", self.message)
    }
    pub fn unex<T>(at: usize) -> Result<T, AstError> {
        Err(AstError {
            at,
            message: format!("unknown token"),
        })
    }
    pub fn todo<T>(name: &str) -> Result<T, AstError> {
        Err(AstError {
            at: usize::MAX,
            message: format!("{name} is todo"),
        })
    }
}

pub fn err<T>(at: usize, msg: &str) -> Result<T, AstError> {
    Err(AstError {
        at,
        message: msg.to_string(),
    })
}
