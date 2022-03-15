// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
use std::{fmt::Display, ops::Rem};

/// a word be substituted in? If yes, which word?
pub type MatcherF<T> = Box<dyn Fn(T) -> bool>;

pub struct Matcher<T> {
    matcher: MatcherF<T>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<S, F>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: AsRef<str>,
    {
        Matcher {
            matcher: Box::new(matcher),
            subs: subs.as_ref().to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    fun: Vec<Matcher<T>>,
}

impl<T> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy { fun: Vec::new() }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.fun.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
        T: Copy + Display,
    {
        iter.map(move |item| {
            let mut result = String::new();
            for m in &self.fun {
                if (m.matcher)(item) {
                    result.push_str(&m.subs);
                }
            }
            if result.is_empty() {
                result = item.to_string()
            }
            result
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Copy + Default + PartialEq + From<u8> + Rem<Output = T>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|x| x % 3.into() == T::default(), "fizz"))
        .add_matcher(Matcher::new(|x| x % 5.into() == T::default(), "buzz"))
}
