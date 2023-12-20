// cSpell:words peekable itertools jodel

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
    Iter: Iterator<Item = TheItem>,
{
    type Item = TheItem;
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;
    use rstest::*;

    #[test]
    fn compare_to_unique() {
        let input = vec![
            "asdf".to_string(),
            "asdf".to_string(),
            "asdf".to_string(),
            "jodel".to_string(),
            "asdf".to_string(),
            "jodel".to_string(),
            "1234".to_string(),
            "asdf".to_string(),
            "1234".to_string(),
            "1234".to_string(),
            "12345".to_string(),
        ];

        let unique_output = input
            .clone()
            .iter()
            .unique()
            .map(|s| -> String { s.to_string() })
            .sorted()
            .collect::<Vec<String>>();
        let adjacent_output = input
            .iter()
            .sorted()
            .adjacent_merge(
                |s| -> String { s.to_string() },
                |l, r| -> Option<String> {
                    if l == r {
                        Some(l.clone())
                    } else {
                        None
                    }
                },
            )
            .collect::<Vec<_>>();

        assert_eq!(unique_output, adjacent_output);
    }

    #[rstest]
    #[case(vec![])]
    #[case(vec![42])]
    #[case(vec![42, 3, 100, 101, 42, 3])]
    fn edge_cases(#[case] input: Vec<i32>) {
        let adjacent_output = input
            .iter()
            .adjacent_merge(
                |s| (*s.clone()),
                |l, r| {
                    if l == r {
                        Some(l.clone())
                    } else {
                        None
                    }
                },
            )
            .collect::<Vec<_>>();

        assert_eq!(input, adjacent_output);
    }

    #[test]
    fn merge_ranges() {
        let data = vec![
            (1..=5),
            (5..=9),
            (20..=22),
            (34..=39),
            (35..=42),
            (37..=41),
            (37..=45),
        ];
        let merged_should = vec![(1..=9), (20..=22), (34..=45)];

        let merged = data
            .iter()
            .adjacent_merge(
                |r| -> std::ops::RangeInclusive<i64> { (*r).clone() },
                |l, r| {
                    // I don't want to make this too complicated and assume a <= c
                    let (a, b) = l.clone().into_inner();
                    let (c, d) = r.clone().into_inner();

                    if c > b {
                        None
                    } else {
                        Some(a..=std::cmp::max(b, d))
                    }
                },
            )
            .collect::<Vec<_>>();

        assert_eq!(merged, merged_should);
    }
}
