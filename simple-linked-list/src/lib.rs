use std::{collections::linked_list, iter::FromIterator};

struct Cell<T> {
    val: T,
    next: Option<Box<Cell<T>>>,
}

pub struct SimpleLinkedList<T> {
    length: usize,
    list: Option<Box<Cell<T>>>,
}

struct IterSimpleLinkedList<'a, T: 'a> {
    inner: &'a Option<Box<Cell<T>>>,
}

impl<'a, T> Iterator for IterSimpleLinkedList<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref elem) = self.inner {
            self.inner = &elem.next;
            return Some(&elem.val);
        }
        None
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            length: 0,
            list: None,
        }
    }

    fn iter<'a>(&'a self) -> IterSimpleLinkedList<'a, T> {
        IterSimpleLinkedList { inner: &self.list }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, val: T) {
        self.length += 1;
        self.list = Some(Box::new(Cell {
            val,
            next: self.list.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.list.take() {
            None => None,
            Some(e) => {
                self.length -= 1;
                self.list = e.next;
                Some(e.val)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.list {
            None => None,
            Some(ref e) => Some(&e.val),
        }
    }

    // Maybe it can be done without reallocating at cost of some CPU usage
    // With memory cursed manipulation.
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        // Downside we allocate
        // Upside it's dead simple.
        let mut lst = SimpleLinkedList::new();
        if self.is_empty() {
            return lst;
        }
        while let Some(e) = self.pop() {
            lst.push(e);
        }
        lst
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut acc = SimpleLinkedList::new();
        for e in iter {
            acc.push(e);
        }
        acc
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v = std::collections::VecDeque::with_capacity(linked_list.len());
        while let Some(e) = linked_list.pop() {
            v.push_front(e)
        }
        v.into()
    }
}
