use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

pub struct CharsWhile<'i, 'c, F: FnMut(char) -> bool> {
    iter: &'i mut Peekable<Enumerate<Chars<'c>>>,
    f: F,
}
impl<'i, 'c, F: FnMut(char) -> bool> CharsWhile<'i, 'c, F> {
    pub fn new(iter: &'i mut Peekable<Enumerate<Chars<'c>>>, f: F) -> Self {
        Self { iter, f }
    }
}

impl<'i, 'c, F: FnMut(char) -> bool> Iterator for CharsWhile<'i, 'c, F> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        self.iter.next_if(|(_, c)| (self.f)(*c)).map(|(_, c)| c)
    }
}
