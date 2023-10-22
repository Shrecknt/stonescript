use std::{iter::FusedIterator, str::Chars};

#[derive(Debug)]
pub struct Stream<'a, T: FusedIterator<Item = char>> {
    iterator: &'a mut T,
    buffer: Vec<char>,
    pub position: usize,
}

impl<'a, T: FusedIterator<Item = char>> Stream<'a, T> {
    pub fn new(iterator: &'a mut T) -> Self {
        Self {
            iterator,
            buffer: vec![],
            position: 0,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        if let Some(char) = self.buffer.get(self.position) {
            Some(*char)
        } else if let Some(char) = self.iterator.next() {
            self.buffer.push(char);
            self.peek()
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let ret = self.peek();
        self.advance();
        ret
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }
}

impl<'a, T: FusedIterator<Item = char>> Iterator for Stream<'a, T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<'a, T: FusedIterator<Item = char>> FusedIterator for Stream<'a, T> {}

impl<'a> From<&'a mut Chars<'a>> for Stream<'a, Chars<'a>> {
    fn from(value: &'a mut Chars<'a>) -> Self {
        Stream::new(value)
    }
}
