use std::{iter::FusedIterator, str::Chars};

#[derive(Debug, Clone)]
pub struct Stream<T: FusedIterator<Item = char>> {
    iterator: T,
    buffer: Vec<char>,
    pub position: usize,
    end: Option<usize>,
}

impl<T: FusedIterator<Item = char>> Stream<T> {
    pub fn new(iterator: T) -> Self {
        Self {
            iterator,
            buffer: vec![],
            position: 0,
            end: None,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        if let Some(end) = self.end {
            if self.position == end {
                return None;
            }
        }

        if let Some(char) = self.buffer.get(self.position) {
            Some(*char)
        } else {
            if let Some(char) = self.iterator.next() {
                self.buffer.push(char);
                Some(char)
            } else {
                None
            }
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

    pub fn until<'a, R, F: FnOnce(Stream<&'a mut Stream<T>>) -> R>(&'a mut self, until: usize, func: F) -> R {
        func(Stream {
            iterator: self,
            buffer: vec![],
            position: 0,
            end: Some(until),
        })
    }
}

impl<T: FusedIterator<Item = char>> Iterator for Stream<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<T: FusedIterator<Item = char>> FusedIterator for Stream<T> {}

impl<'a> From<&'a str> for Stream<Chars<'a>> {
    fn from(value: &'a str) -> Self {
        Stream::new(value.chars())
    }
}

impl<'a> From<&'a String> for Stream<Chars<'a>> {
    fn from(value: &'a String) -> Self {
        Stream::new(value.chars())
    }
}
