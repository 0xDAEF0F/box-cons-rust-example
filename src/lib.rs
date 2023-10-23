use crate::List::{Cons, Nil};

#[derive(Debug, Clone)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

pub struct ListIterator<'a, T> {
    list: &'a List<T>,
}

impl<'a, T> List<T> {
    pub fn iter(&'a self) -> ListIterator<'a, T> {
        ListIterator { list: self }
    }
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list {
            Nil => None,
            Cons(value, next) => {
                self.list = next;
                Some(value)
            }
        }
    }
}
