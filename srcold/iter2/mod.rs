mod compl;
mod inter;
mod union;

use std::array::IntoIter;
use std::cmp::Ordering;
use std::iter::FusedIterator;
use crate::*;
use compl::ComplIter;
use inter::InterIter;
use crate::iter::union::iter::UnionIter;


/// An iterator over sorted and distinct time intervals
///
/// This trait specifies the type of the base temporal data (date or duration)
/// and ensures that it exists an iterator over convex parts (time intervals) of the time window.
///
/// By contract, this iterator should produce its time intervals in a sorted manner.
/// It means also that no time intervals overlap
pub trait TimeConvexIterator: Iterator<Item=TimeInterval<Self::TimePoint>>+Sized
{
    /// The type of the underlying time data.
    ///
    /// This is also the type of the element managed by a time window.
    /// Typically, the timepoint is [`Timestamp`] when dealing with dates and
    /// [`TimeValue`]  when dealing with durations.
    type TimePoint: TimePoint;

    #[inline]
    fn complementary(self) -> ComplIter<Self> { ComplIter::new(self) }

    #[inline]
    fn intersection_old<I>(self, other:I) -> InterIter<Self,I::IntoIter>
        where
            I: IntoIterator<Item=TimeInterval<Self::TimePoint>>,
            I::IntoIter: TimeConvexIterator<TimePoint=Self::TimePoint>
    {
        InterIter::new(self, other.into_iter())
    }

    #[inline]
    fn union<I>(self, other:I) -> UnionIter<Self,I::IntoIter>
        where
            I: IntoIterator<Item=TimeInterval<Self::TimePoint>>,
            I::IntoIter: TimeConvexIterator<TimePoint=Self::TimePoint>
    {
        UnionIter::new(self, other.into_iter())
    }
}

pub trait IntoTimeConvexIterator {

    type TimePoint:TimePoint;
    type IntoIter: TimeConvexIterator<TimePoint=Self::TimePoint>;

    /// An iterator over the convex parts
    fn into_iter(self) -> Self::IntoIter;
}

pub trait IntoTimeConvexIterator2: IntoIterator<Item=TimeInterval<Self::TimePoint>>
    where
        Self::IntoIter : TimeConvexIterator<TimePoint=Self::TimePoint>
{
    type TimePoint:TimePoint;
}

impl<I:TimeConvexIterator> IntoTimeConvexIterator for I {
    type TimePoint = I::TimePoint;
    type IntoIter = I;

    #[inline] fn into_iter(self) -> Self::IntoIter { self }
}

impl<T:TimePoint> IntoIterator for TimeInterval<T>
{
    type Item =  Self;
    type IntoIter = std::option::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter
    {
        if self.is_empty() {
            None.into_iter()
        } else {
            Some(TimeInterval {
                lower:self.lower_bound(),
                upper:self.upper_bound()
            }).into_iter()
        }
    }
}
/*

impl<TW:TimeConvex> IntoTimeConvexIterator for TW
{
    type TimePoint =  TW::TimePoint;
    type IntoIter = std::option::IntoIter<TimeInterval<Self::TimePoint>>;

    #[inline]
    fn into_convex_iter(self) -> Self::IntoIter
    {
        if self.is_empty() {
            None.into_iter()
        } else {
            Some(TimeInterval {
                lower:self.lower_bound(),
                upper:self.upper_bound()
            }).into_iter()
        }
    }
}*/


impl<T:TimePoint> TimeConvexIterator for std::option::IntoIter<TimeInterval<T>> {
    type TimePoint = T;
}


impl<'a,T:TimePoint> IntoIterator for &'a TimeSet<T>
{
    type Item = TimeInterval<T>;
    type IntoIter = timeset::IntoConvexIter<T,std::iter::Copied<std::slice::Iter<'a,TimeInterval<T>>>>;

    #[inline] fn into_iter(self) -> Self::IntoIter { timeset::IntoConvexIter(self.0.iter().copied()) }

}

mod timeset {
    use std::iter::FusedIterator;
    use crate::*;
    use crate::iter::TimeConvexIterator;

    pub struct IntoConvexIter<T: TimePoint, I: Iterator<Item=TimeInterval<T>>>(pub(super) I);

    impl<T: TimePoint, I: Iterator<Item=TimeInterval<T>>> TimeConvexIterator for IntoConvexIter<T, I> {
        type TimePoint = T;
    }

    impl<T:TimePoint, I> FusedIterator for IntoConvexIter<T, I>
        where I: FusedIterator + Iterator<Item=TimeInterval<T>>
    {}

    impl<T: TimePoint, I: Iterator<Item=TimeInterval<T>>> Iterator for IntoConvexIter<T, I> {
        type Item = I::Item;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> { self.0.next() }
        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
        #[inline]
        fn count(self) -> usize where Self: Sized { self.0.count() }
        #[inline]
        fn last(self) -> Option<Self::Item> where Self: Sized { self.0.last() }
        #[inline]
        fn nth(&mut self, n: usize) -> Option<Self::Item> { self.0.nth(n) }
    }
}

impl<I> TimeConvexIterator for std::iter::StepBy<I>
    where
        I:TimeConvexIterator
{
    type TimePoint = I::TimePoint;
}

impl<I,P> TimeConvexIterator for std::iter::Filter<I,P>
    where
        I:TimeConvexIterator+Sized,
        P: FnMut(&I::Item)->bool
{
    type TimePoint = I::TimePoint;
}

impl<I> TimeConvexIterator for std::iter::Peekable<I>
    where
        I:TimeConvexIterator
{
    type TimePoint = I::TimePoint;
}

impl<I,P> TimeConvexIterator for std::iter::SkipWhile<I,P>
    where
        I:TimeConvexIterator+Sized,
        P: FnMut(&I::Item)->bool
{
    type TimePoint = I::TimePoint;
}

impl<I,P> TimeConvexIterator for std::iter::TakeWhile<I,P>
    where
        I:TimeConvexIterator+Sized,
        P: FnMut(&I::Item)->bool
{
    type TimePoint = I::TimePoint;
}

impl<I> TimeConvexIterator for std::iter::Skip<I>
    where
        I:TimeConvexIterator
{
    type TimePoint = I::TimePoint;
}

impl<I> TimeConvexIterator for std::iter::Take<I>
    where
        I:TimeConvexIterator
{
    type TimePoint = I::TimePoint;
}

impl<I,F> TimeConvexIterator for std::iter::Inspect<I,F>
    where
        I:TimeConvexIterator,
        F: FnMut(&I::Item), I: Sized
{
    type TimePoint = I::TimePoint;
}


impl<T:TimePoint> IntoIterator for TimeSet<T>
{
    type Item = TimeInterval<T>;
    type IntoIter = timeset::IntoConvexIter<T,std::vec::IntoIter<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        timeset::IntoConvexIter(self.0.into_iter())
    }
}


impl<T:TimePoint> TimeConvexIterator for std::vec::IntoIter<TimeInterval<T>> {
    type TimePoint = T;
}

