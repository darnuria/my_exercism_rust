use std::iter::FromIterator;

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
        self.inner.as_ref().map(|e| {
            self.inner = &e.next;
            &e.val
        })
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
        self.list.take().map(|e| {
            self.length -= 1;
            self.list = e.next;
            e.val
        })
    }

    /// Returns a reference on the element at the head of the list.
    pub fn peek(&self) -> Option<&T> {
        self.list.as_ref().map(|e| &e.val)
    }

    /// Reverse the list
    /// Upside it's fast: Downside it's little bit mind-cursed.
    /// Inspired by Elsegahy solution (maybe inspired by others)
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        if self.is_empty() {
            return self;
        }
        let mut iter = self.list;
        let mut new_head = None;
        while let Some(mut e) = iter.take() {
            iter = e.next;
            e.next = new_head;
            new_head = Some(e);
        }
        self.list = new_head;
        self
    }
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self::new()
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
