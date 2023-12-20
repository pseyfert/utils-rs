pub struct AdjacentMerge<I: Iterator, V, B, P>
where
    B: Fn(&V, &V) -> Option<V>,
    I: Iterator,
    P: Fn(&I::Item) -> V,
{
    iter: std::iter::Peekable<I>,
    p: P,
    b: B,
}

impl<I, V, B, P> Iterator for AdjacentMerge<I, V, B, P>
where
    B: Fn(&V, &V) -> Option<V>,
    I: Iterator,
    P: Fn(&I::Item) -> V,
{
    type Item = V;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (low, hi) = self.iter.size_hint();
        ((low > 0) as usize, hi)
    }

    fn next(&mut self) -> Option<V> {
        let Some(mut acc) = self.iter.next().as_ref().map(&self.p) else {
            return None;
        };
        while let Some(n) = self.iter.peek() {
            match (self.b)(&acc, &(self.p)(n)) {
                Some(a) => {
                    acc = a;
                    self.iter.next();
                }
                None => {
                    return Some(acc);
                }
            }
        }
        Some(acc)
    }
}

pub trait AdjacentMergeIterator {
    type Item;

    #[inline]
    fn adjacent_merge<V, B, P>(self, p: P, b: B) -> AdjacentMerge<Self, V, B, P>
    where
        B: Fn(&V, &V) -> Option<V>,
        P: Fn(&<Self as Iterator>::Item) -> V,
        Self: Sized + Iterator,
    {
        AdjacentMerge {
            iter: self.peekable(),
            p,
            b,
        }
    }
}

impl<Iter, TheItem> AdjacentMergeIterator for Iter
where
    Iter: Clone + Iterator<Item = TheItem>,
{
    type Item = TheItem;
}
