use std::ops::{BitAnd, BitAndAssign};
use crate::*;
use crate::iter::TimeIntersection;

impl<T:TimePoint,TW> BitAndAssign<TW> for TimeInterval<T>
    where TW: Into<TimeInterval<T>>
{
    fn bitand_assign(&mut self, tw: TW) {
        *self = self.bitand(tw)
    }
}

impl<T:TimePoint,TW> BitAndAssign<TW> for TimeSet<T>
    where TW: Into<TimeInterval<T>>
{
    fn bitand_assign(&mut self, tw: TW) {
        // todo: optimise cloning
        *self = self.clone().bitand(tw)
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



impl<T:TimePoint,TW> BitAnd<TW> for TimeInterval<T>
    where TW: Into<TimeInterval<T>>
{
    type Output = Self;
    #[inline] fn bitand(self, tw: TW) -> Self::Output { (&self).bitand(tw) }
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
    where TW: Into<TimeInterval<T>>
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
    where TW: Into<TimeInterval<T>>
{
    type Output = TimeSet<T>;

    #[inline]
    fn bitand(self, tw: TW) -> Self::Output {
        self.into_iter().intersection(tw.into()).collect()
    }
}


impl<T:TimePoint,TW> BitAnd<TW> for &TimeInterval<T>
    where TW: Into<TimeInterval<T>>
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