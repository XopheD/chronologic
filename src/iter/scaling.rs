use std::ops::Mul;
use crate::*;
use crate::iter::*;


/// # Time window scaling iterator
pub trait TimeScaling<S:Copy>: TimeConvexIterator
{
    type Output:TimeConvexIterator<TimePoint=Self::TimePoint>;
    fn scaling(self, scale: S) -> Self::Output;
}


impl<I:TimeConvexIterator,S:Copy> TimeScaling<S> for I
    where
        I::Item: Mul<S,Output=I::Item>
{
    type Output = TimeValueScaleIter<I,S>;

    fn scaling(self, scale: S) -> Self::Output {
        TimeValueScaleIter { scale, iter: self }
    }
}

pub struct TimeValueScaleIter<I:TimeConvexIterator,S:Copy> {
    scale: S,
    iter: I
}

impl<I:TimeConvexIterator,S:Copy> Iterator for TimeValueScaleIter<I,S>
    where
        I::Item: Mul<S,Output=I::Item>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|tw| tw*self.scale)
            .and_then(|tw| if tw.is_empty() { None } else { Some(tw) })
    }
}


impl<I:TimeConvexIterator,S:Copy> TimeConvexIterator for TimeValueScaleIter<I,S>
    where
        I::Item: Mul<S,Output=I::Item>
{
    type TimePoint = I::TimePoint;
}

