use std::iter::Peekable;

pub struct IterWhile<'a, I: Iterator, F: FnMut(&I::Item) -> bool> {
    peekable: &'a mut Peekable<I>,
    f: F,
}
impl<'a, I: Iterator, F: FnMut(&I::Item) -> bool> IterWhile<'a, I, F> {
    pub fn new(peekable: &'a mut Peekable<I>, f: F) -> Self {
        Self { peekable, f }
    }
}

impl<'a, I: Iterator, F: FnMut(&I::Item) -> bool> Iterator for IterWhile<'a, I, F> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.peekable.next_if(&mut self.f)
    }
}
