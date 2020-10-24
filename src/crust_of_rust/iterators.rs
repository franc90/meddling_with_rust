pub trait IteratorExt: Iterator + Sized {
    fn custom_flatten(self) -> Flatten<Self>
        where
            Self::Item: IntoIterator {
        custom_flatten(self)
    }
}

impl<T> IteratorExt for T
    where T: Iterator + Sized {
    fn custom_flatten(self) -> Flatten<Self>
        where
            Self::Item: IntoIterator {
        custom_flatten(self)
    }
}

pub fn custom_flatten<O>(iter: O) -> Flatten<O::IntoIter>
    where
        O: IntoIterator,
        O::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
    where
        O: Iterator,
        O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
    where
        O: Iterator,
        O::Item: IntoIterator,
{
    pub fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
    where O: Iterator,
          O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut front_iter) = self.front_iter {
                if let Some(i) = front_iter.next() {
                    return Some(i);
                }
                self.front_iter = None;
            }
            if let Some(front_iter) = self.outer.next() {
                self.front_iter = Some(front_iter.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
    where O: Iterator + DoubleEndedIterator,
          O::Item: IntoIterator,
          <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_iter {
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
                self.back_iter = None;
            }
            if let Some(back_iter) = self.outer.next_back() {
                self.back_iter = Some(back_iter.into_iter());
            } else {
                return self.front_iter.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(custom_flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }

    #[test]
    fn empty_wide() {
        assert_eq!(custom_flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0)
    }

    #[test]
    fn one() {
        assert_eq!(custom_flatten(std::iter::once(vec!["a"])).count(), 1)
    }

    #[test]
    fn two() {
        assert_eq!(custom_flatten(std::iter::once(vec!["a", "b"])).count(), 2)
    }

    #[test]
    fn two_wide() {
        assert_eq!(custom_flatten(vec![vec!["a"], vec!["b"]]).count(), 2)
    }

    #[test]
    fn reverse() {
        assert_eq!(custom_flatten(std::iter::once(vec!["a", "b"]))
                       .rev().collect::<Vec<_>>(),
                   vec!["b", "a"])
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(custom_flatten(vec![vec!["a"], vec!["b"]])
                       .rev().collect::<Vec<_>>(),
                   vec!["b", "a"])
    }

    #[test]
    fn both_ends() {
        let mut iter = custom_flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn inf() {
        let mut iter = custom_flatten((0..).map(|i| 0..i));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        // can go on, it's infinite
    }

    #[test]
    fn ext() {
        assert_eq!(vec![vec![0, 1]].into_iter().custom_flatten().count(), 2);
    }
}