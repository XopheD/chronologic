use std::ops::{BitAnd, BitAndAssign};
use crate::*;
use crate::iter::TimeIntersection;

//------------ TIME POINTS ------------


impl<TW> BitAnd<TW> for TimeValue
    where TW: TimeConvex<TimePoint=TimeValue>
{
    type Output = TimeInterval<TimeValue>;
    #[inline] fn bitand(self, tw: TW) -> Self::Output { TimeInterval::singleton(self).bitand(tw) }
}

impl<TW> BitAnd<TW> for Timestamp
    where TW: TimeConvex<TimePoint=Timestamp>
{
    type Output = TimeInterval<Timestamp>;
    #[inline] fn bitand(self, tw: TW) -> Self::Output { TimeInterval::singleton(self).bitand(tw) }
}

//------------ TIME INTERVALS ------------

impl<T:TimePoint,TW> BitAndAssign<TW> for TimeInterval<T>
    where TW: TimeConvex<TimePoint=T>
{
    #[inline]
    fn bitand_assign(&mut self, tw: TW) {
        if self.lower < tw.lower_bound() { self.lower = tw.lower_bound(); }
        if self.upper > tw.upper_bound() { self.upper = tw.upper_bound(); }
    }
}

impl<T:TimePoint,TW> BitAnd<TW> for TimeInterval<T>
    where TW: TimeConvex<TimePoint=T>
{
    type Output = Self;
    #[inline]
    fn bitand(self, tw: TW) -> Self::Output { (&self).bitand(tw) }
}


impl<T:TimePoint,TW> BitAnd<TW> for &TimeInterval<T>
    where TW: TimeConvex<TimePoint=T>
{
    type Output = TimeInterval<T>;

    #[inline]
    fn bitand(self, tw: TW) -> Self::Output {
        let i = tw.into();
        TimeInterval {
            lower: self.lower.max(i.lower),
            upper: self.upper.min(i.upper)
        }
    }
}

//----------------- TIME SETS ------------------------

impl<T:TimePoint,TW> BitAndAssign<TW> for TimeSet<T>
    where TW: TimeConvex<TimePoint=T>
{
    fn bitand_assign(&mut self, tw: TW) {
        let _ = self.truncate_after(tw.upper_bound());
        let _ = self.truncate_before(tw.lower_bound());
    }
}

impl<T:TimePoint> BitAndAssign<Self> for TimeSet<T>
{
    fn bitand_assign(&mut self, tw: Self) {
        // fixme: suppress cloning
        *self = self.clone().bitand(tw)
    }
}

impl<T:TimePoint> BitAndAssign<&Self> for TimeSet<T>
{
    fn bitand_assign(&mut self, tw: &Self) {
        // fixme: suppress cloning
        *self = self.clone().bitand(tw)
    }
}




impl<T:TimePoint> BitAnd<Self> for TimeSet<T>
{
    type Output = Self;
    #[inline] fn bitand(self, tw: Self) -> Self::Output { (&self).bitand(tw) }
}

impl<T:TimePoint> BitAnd<&Self> for TimeSet<T>
{
    type Output = Self;
    #[inline] fn bitand(self, tw: &Self) -> Self::Output { (&self).bitand(tw) }
}

impl<T:TimePoint, TW> BitAnd<TW> for TimeSet<T>
    where TW: TimeConvex<TimePoint=T>
{
    type Output = Self;
    #[inline] fn bitand(self, tw: TW) -> Self::Output { (&self).bitand(tw) }
}


impl<T:TimePoint> BitAnd<TimeSet<T>> for &TimeSet<T>
{
    type Output = TimeSet<T>;

    #[inline]
    fn bitand(self, tw: TimeSet<T>) -> Self::Output {
        self.into_iter().intersection(tw.into_iter()).collect()
    }
}

impl<T:TimePoint> BitAnd<Self> for &TimeSet<T>
{
    type Output = TimeSet<T>;

    #[inline]
    fn bitand(self, tw: &TimeSet<T>) -> Self::Output {
        self.into_iter().intersection(tw.into_iter()).collect()
    }
}


impl<T:TimePoint, TW> BitAnd<TW> for &TimeSet<T>
    where TW: TimeConvex<TimePoint=T>
{
    type Output = TimeSet<T>;

    #[inline]
    fn bitand(self, tw: TW) -> Self::Output {
        self.into_iter().intersection(tw.into()).collect()
    }
}

