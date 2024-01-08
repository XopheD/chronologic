use std::iter::{Fuse, FusedIterator};
use crate::*;
use crate::iter::*;

/// # The complementary iterator of a time set
pub trait TimeComplementary: TimeConvexIterator {
    type Output:TimeConvexIterator<TimePoint=Self::TimePoint>;
    fn complementary(self) -> Self::Output;
}


impl<TW> TimeComplementary for TW
    where
        TW: TimeConvexIterator
{
    type Output = IterComplementary<Self>;

    #[inline]
    fn complementary(self) -> Self::Output {
        IterComplementary::new(self)
    }
}


pub struct IterComplementary<I:TimeConvexIterator>
{
    iter: Fuse<I>,
    lower: I::TimePoint
}

impl<I:TimeConvexIterator> IterComplementary<I>
{
    fn new(iter: I) -> Self {
        Self {
            iter: iter.fuse(),
            lower: -I::TimePoint::INFINITE
        }
    }
}

impl<I:TimeConvexIterator> TimeConvexIterator for IterComplementary<I> {
    type TimePoint = I::TimePoint;
}

impl<I:TimeConvexIterator> FusedIterator for IterComplementary<I> { }


impl<I:TimeConvexIterator> Iterator for IterComplementary<I>
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
        for next in self.iter.by_ref()
        {
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
