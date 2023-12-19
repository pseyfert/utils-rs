use std::iter::FusedIterator;


// largely taken from the standard library Cycle implementation

#[derive(Clone, Debug)]
pub struct CountedCycle<I> {
    orig: I,
    iter: I,
    cycle: usize,
}

impl<I: Clone> CountedCycle<I> {
    pub fn new(iter: I) -> CountedCycle<I> {
        CountedCycle {
            orig: iter.clone(),
            iter,
            cycle: 0,
        }
    }
}

impl<I> Iterator for CountedCycle<I>
where
    I: Clone + Iterator,
{
    type Item = (usize, <I as Iterator>::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => {
                self.iter = self.orig.clone();
                self.cycle += 1;
                match self.iter.next() {
                    None => None,
                    Some(y) => Some((self.cycle, y)),
                }
            }
            Some(y) => Some((self.cycle, y)),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // the cycle iterator is either empty or infinite
        match self.orig.size_hint() {
            sz @ (0, Some(0)) => sz,
            (0, _) => (0, None),
            _ => (usize::MAX, None),
        }
    }
}

impl<I> FusedIterator for CountedCycle<I> where I: Clone + Iterator {}

pub trait CycledIterator {
    type Item;
    #[inline]
    fn cycle_counted(self) -> CountedCycle<Self>
    where
        Self: Sized + Clone,
    {
        CountedCycle::new(self)
    }
}

impl<Iter, TheItem> CycledIterator for Iter
where
    Iter: Clone + Iterator<Item = TheItem>,
{
    type Item = TheItem;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_edge_cases() {
        let items: Vec<usize> = vec![];
        assert_eq!(items.iter().cycle_counted().count(), 0);

        (0..100)
            .cycle_counted()
            .enumerate()
            .take(100)
            .for_each(|(i, (cy, o))| {
                assert_eq!(cy, 0);
                assert_eq!(i, o);
            });

        let items = vec!['k'];
        items
            .iter()
            .enumerate()
            .cycle_counted()
            .enumerate()
            .for_each(|(i, (cy, (inner, c)))| {
                assert_eq!(cy, i);
                assert_eq!('k', *c);
                assert_eq!(0, inner);
            })
    }
}
