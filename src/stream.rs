use std::any::TypeId;

#[derive(Debug, Clone)]
pub struct Stream<T> {
    pub contents: Vec<T>,
    pub position: usize,
    pub left_col: Option<u64>,
    pub row: Option<u64>,
    pub col: Option<u64>,
    pub is_chars: bool,
}

impl<T: Clone + 'static> Stream<T> {
    pub fn new() -> Self {
        Self {
            contents: Vec::new(),
            position: 0,
            left_col: None,
            row: None,
            col: None,
            is_chars: false,
        }
    }

    pub fn eof(&self) -> bool {
        self.position >= self.contents.len()
    }

    pub fn peek(&self, distance: usize) -> Option<T> {
        self.contents.get(self.position + distance).cloned()
    }

    pub fn next(&mut self) -> T {
        let item = self.contents[self.position].clone();
        self.position += 1;
        if self.is_chars {
            let item = unsafe { *std::mem::transmute::<&T, &char>(&item) };
            // let item = (Box::new(item.clone()) as Box<dyn Any>)
            //     .downcast::<char>()
            //     .unwrap();
            self.col = Some(self.col.unwrap() + 1);
            if item == '\n' {
                self.row = Some(self.row.unwrap() + 1);
                self.col = self.left_col;
            }
            // the transmuted `item` should be dropped before the original one
            // this code isn't needed for that but I'm trying to make it as explicit as possible
            drop(item);
        }
        item
    }

    pub fn until(&mut self, condition: &dyn Fn(T) -> bool, including: bool) -> Vec<T> {
        let mut res = vec![];
        while !self.eof() && !condition(self.peek(0).unwrap()) {
            res.push(self.next());
        }
        if including {
            if self.eof() {
                self.yeet("Expected ';', found EOF".to_string());
                unreachable!()
            }
            self.skip()
        }
        res
    }

    pub fn skip(&mut self) {
        self.next();
    }

    pub fn yeet(&self, msg: String) -> ! {
        if TypeId::of::<T>() == TypeId::of::<char>() {
            panic!("{} ({}:{})", msg, self.row.unwrap(), self.col.unwrap())
        } else {
            panic!("{} ({})", msg, self.position)
        }
    }
}

impl<T: Clone + 'static> Iterator for Stream<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.eof() {
            return Some(self.next());
        }
        None
    }
}

impl From<String> for Stream<char> {
    fn from(value: String) -> Self {
        Self {
            contents: value.chars().into_iter().collect(),
            position: 0,
            left_col: Some(0),
            row: Some(1),
            col: Some(0),
            is_chars: true,
        }
    }
}

impl<T> From<Vec<T>> for Stream<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            contents: value,
            position: 0,
            left_col: None,
            row: None,
            col: None,
            is_chars: false,
        }
    }
}

impl<T> From<&mut dyn Iterator<Item = T>> for Stream<T> {
    fn from(value: &mut dyn Iterator<Item = T>) -> Self {
        Self {
            contents: value.collect::<Vec<T>>(),
            position: 0,
            left_col: None,
            row: None,
            col: None,
            is_chars: false,
        }
    }
}
