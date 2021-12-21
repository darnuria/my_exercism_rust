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
    /// Allocate a new empty list
    pub fn new() -> Self {
        SimpleLinkedList {
            length: 0,
            list: None,
        }
    }

    /// Not tested attempt at implementing Iterator.
    fn iter<'a>(&'a self) -> IterSimpleLinkedList<'a, T> {
        IterSimpleLinkedList { inner: &self.list }
    }

    /// Return true if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.list.is_none()
    }

    /// Returns the size of the list.
    /// In constant time because we keep in the list handle a counter.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Push front and allocate on heap a value in list.
    pub fn push(&mut self, val: T) {
        self.length += 1;
        self.list = Some(Box::new(Cell {
            val,
            next: self.list.take(),
        }));
    }

    /// Returns the last element (pop front) deallocate the node on heap.
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

    /// Returns a reference on the element at the head of the list.
    pub fn peek(&self) -> Option<&T> {
        match self.list {
            None => None,
            Some(ref e) => Some(&e.val),
        }
    }

    /// Reverse the list
    /// Downside we allocate
    /// Upside it's dead simple.
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        // Maybe it can be done without reallocating at cost of some CPU usage
        // With memory cursed manipulation.
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
    /// Converts an iterator into a `SimpleLinkedList`
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


impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    /// Make a `Vec` out of a SimpleLinkedList
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v = std::collections::VecDeque::with_capacity(linked_list.len());
        while let Some(e) = linked_list.pop() {
            v.push_front(e)
        }
        v.into()
    }
}
