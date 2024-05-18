pub struct TryZip<A, B> {
    iter_a: A,
    iter_b: Option<B>,
}

impl<A, B> Iterator for TryZip<A, B>
where
    A: Iterator,
    B: Iterator,
{
    type Item = (A::Item, Option<B::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter_a.next() {
            Some(a) => Some((
                a,
                match &mut self.iter_b {
                    Some(ref mut iter_b) => iter_b.next(),
                    None => None,
                },
            )),
            None => None,
        }
    }
}

pub trait TryZipExt<A, B>: Iterator {
    fn try_zip(self, second: Option<B>) -> TryZip<Self, B>
    where Self: Sized,
    {
        TryZip { iter_a: self, iter_b: second }
    }
}

impl<I: Iterator, J: Iterator> TryZipExt<I, J> for I {}

#[cfg(test)]
mod tests {

    use super::*;

    use std::ops::Range;

    #[test]
    fn first_longer() {
        let iter_a = 0u16..10;
        let iter_b = 0i8..5;
        let result = iter_a.try_zip(Some(iter_b)).collect::<Vec<_>>();
        assert_eq!(result, vec![(0,Some(0)), (1,Some(1)), (2,Some(2)), (3,Some(3)), (4,Some(4)), (5,None), (6, None), (7, None), (8,None), (9,None)])
    }

    #[test]
    fn second_longer() {
        let iter_a = 0..5;
        let iter_b = 0..10;
        let result = iter_a.try_zip(Some(iter_b)).collect::<Vec<_>>();
        assert_eq!(result, vec![(0,Some(0)), (1,Some(1)), (2,Some(2)), (3,Some(3)), (4,Some(4))])
    }

    #[test]
    fn same_length() {
        let iter_a = 0..10;
        let iter_b = 0..10;
        let result = iter_a.try_zip(Some(iter_b)).collect::<Vec<_>>();
        assert_eq!(result, vec![(0,Some(0)), (1,Some(1)), (2,Some(2)), (3,Some(3)), (4,Some(4)), (5,Some(5)), (6, Some(6)), (7, Some(7)), (8,Some(8)), (9,Some(9))])
    }

    #[test]
    fn no_second() {
        let iter_a = 0usize..10;
        let result = iter_a.try_zip(None::<Range<u8>>).collect::<Vec<_>>();
        assert_eq!(result, vec![(0,None), (1,None), (2,None), (3,None), (4,None), (5,None), (6, None), (7, None), (8,None), (9,None)])
    }
}
