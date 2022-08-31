use std::iter::*;
use crate::*;
use crate::iter::*;


pub struct ComplIter<I:TimeConvexIterator>
{
    iter: Fuse<I>,
    lower: I::TimePoint
}

impl<I:TimeConvexIterator> ComplIter<I>
{
    pub(super) fn new(iter: I) -> Self {
        Self {
            iter: iter.fuse(),
            lower: -I::TimePoint::INFINITE
        }
    }
}

impl<I:TimeConvexIterator> TimeConvexIterator for ComplIter<I> {
    type TimePoint = I::TimePoint;
}

impl<I:TimeConvexIterator> FusedIterator for ComplIter<I> { }


impl<I:TimeConvexIterator> Iterator for ComplIter<I>
{
    type Item = TimeInterval<I::TimePoint>;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.lower.is_past_infinite() {
            // just starting the iteration
            let start = self.iter.next()
                .and_then(|next| {
                    let upper = next.lower_bound().just_before();
                    self.lower = next.upper_bound().just_after();
                    if upper == -I::TimePoint::INFINITE {
                        None
                    } else {
                        Some(TimeInterval { lower: -I::TimePoint::INFINITE, upper})
                    }
                });
            if start.is_some() { return start; }
        }
        while let Some(next) = self.iter.next() {
            if self.lower < next.lower_bound() {
                let result = TimeInterval {
                    lower: self.lower,
                    upper: next.lower_bound().just_before()
                };
                self.lower = next.upper_bound().just_after();
                return Some(result);
            }
        }
        if self.lower.is_future_infinite() {
            None
        } else {
            let result = TimeInterval {
                lower: self.lower,
                upper: I::TimePoint::INFINITE
            };
            self.lower = I::TimePoint::INFINITE;
            Some(result)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let (min,max) = self.iter.size_hint();
        if self.lower.is_finite() {
            (min.saturating_sub(1), max)
        } else {
            (min.saturating_sub(1), max.map(|i| i + 1))
        }
    }
}


#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    fn compl()
    {
        let k = (TimeValue::from_ticks(0) ..= TimeValue::from_ticks(5))
            .into_iter()
            .complementary()
            .complementary()
            .for_each(|f| { eprintln!("k {:?}", f); });
          //  | (TimeValue::from_ticks(7) ..= TimeValue::from_ticks(10))
           // | (TimeValue::from_ticks(10) ..= TimeValue::from_ticks(25));

    }
}