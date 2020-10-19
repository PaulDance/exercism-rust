/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T: Clone> {
    matcher: Box<dyn Fn(T) -> bool>,
    substitute: String,
}

impl<T: Clone> Matcher<T> {
    /// The matcher function has to live long enough to be stored in a Box.
    /// The substitute can be anything, as long as it can produce a String.
    pub fn new<F, S>(matcher: F, substitute: S) -> Self
    where
        F: 'static + Fn(T) -> bool,
        S: ToString,
    {
        Self {
            matcher: Box::new(matcher),
            substitute: substitute.to_string(),
        }
    }

    /// Maps the given element to the matcher's substitute if the matching
    /// function yields `true`, returns `None` otherwise.
    fn map(&self, elt: T) -> Option<String> {
        if (self.matcher)(elt) {
            Some(self.substitute.clone())
        } else {
            None
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
pub struct Fizzy<T: Clone> {
    set: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: Clone + ToString,
{
    /// Builds a new Fizzy with an empty set of matchers.
    pub fn new() -> Self {
        Self { set: Vec::new() }
    }

    /// Adds a new given `matcher` to this Fizzy's set and returns the modified Fizzy.
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.set.push(matcher);
        self
    }

    /// Maps the Fizzy onto every element of an iterator, returning a new iterator.
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        // Closure: concatenate all the substitutes of the element's matching
        // mappings into one string and return it not empty, return the element's
        // textual representation otherwise.
        iter.map(move |elt| {
            match self
                .set
                .iter()
                .filter_map(|matcher| matcher.map(elt.clone()))
                .collect::<String>()
            {
                s if s.is_empty() => elt.to_string(),
                s => s,
            }
        })
    }
}

use std::ops::Rem;

/// Convenience function: returns a Fizzy which applies the standard fizz-buzz rules.
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    // Quite a bit of paramater constraints in order to achieve generality.
    T: Clone + ToString + Rem<T> + From<u8>,
    <T as Rem>::Output: PartialEq<T>,
{
    // Classic FizzBuzz rules taken from the tests.
    Fizzy::new()
        .add_matcher(Matcher::new(|n| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n| n % 5.into() == 0.into(), "buzz"))
}
