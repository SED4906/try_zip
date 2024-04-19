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

impl<A, B> TryZip<A, B> {
    pub fn try_zip(iter_a: A, iter_b: Option<B>) -> TryZip<A, B> {
        TryZip { iter_a, iter_b }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;

    #[test]
    fn first_longer() {
        let iter_a = 0..10;
        let iter_b = 0..5;
        println!(
            "{:?}",
            TryZip::try_zip(iter_a, Some(iter_b)).collect::<Vec<_>>()
        )
    }

    #[test]
    fn second_longer() {
        let iter_a = 0..5;
        let iter_b = 0..10;
        println!(
            "{:?}",
            TryZip::try_zip(iter_a, Some(iter_b)).collect::<Vec<_>>()
        )
    }

    #[test]
    fn same_length() {
        let iter_a = 0..10;
        let iter_b = 0..10;
        println!(
            "{:?}",
            TryZip::try_zip(iter_a, Some(iter_b)).collect::<Vec<_>>()
        )
    }

    #[test]
    fn no_second() {
        let iter_a = 0..10;
        println!(
            "{:?}",
            TryZip::try_zip(iter_a, None::<Range<u8>>).collect::<Vec<_>>()
        )
    }
}
