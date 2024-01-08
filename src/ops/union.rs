use std::ops::{BitOr, BitOrAssign};
use crate::*;
use crate::iter::TimeUnion;

//------------ TIME POINTS ------------


impl<TW> BitOr<TW> for TimeValue
    where TW: TimeConvex<TimePoint=TimeValue>
{
    type Output = TimeSet<TimeValue>;
    #[inline] fn bitor(self, tw: TW) -> Self::Output { TimeInterval::singleton(self).bitor(tw) }
}

impl<TW> BitOr<TW> for Timestamp
    where TW: TimeConvex<TimePoint=Timestamp>
{
    type Output = TimeSet<Timestamp>;
    #[inline] fn bitor(self, tw: TW) -> Self::Output { TimeInterval::singleton(self).bitor(tw) }
}

//------------ TIME INTERVALS ------------


impl<T:TimePoint,TW> BitOr<TW> for TimeInterval<T>
    where TW: TimeConvex<TimePoint=T>
{
    type Output = TimeSet<T>;
    #[inline] fn bitor(self, tw: TW) -> Self::Output { (&self).bitor(tw) }
}

impl<T:TimePoint,TW> BitOr<TW> for &TimeInterval<T>
    where TW: TimeConvex<TimePoint=T>
{
    type Output = TimeSet<T>;

    fn bitor(self, tw: TW) -> Self::Output
    {
        if self.is_empty() {
            TimeSet::from(tw)

        } else if tw.is_empty() {
            (*self).into()

        } else if tw.upper_bound() < self.lower.just_before() {
            TimeSet(vec![tw.into(), *self])

        } else if self.upper < tw.lower_bound().just_before() {
            TimeSet(vec![*self, tw.into()])

        } else {
            TimeSet(vec![
                TimeInterval {
                    lower: self.lower.min(tw.lower_bound()),
                    upper: self.upper.max(tw.upper_bound())
                }
            ])
        }
    }
}

//------------ TIME SETS ------------

impl<T:TimePoint,TW> BitOrAssign<TW> for TimeSet<T>
    where TW: TimeConvex<TimePoint=T>
{
    fn bitor_assign(&mut self, tw: TW) {
        // todo: optimise cloning
        *self = self.clone().bitor(tw)
    }
}

impl<T:TimePoint> BitOrAssign<Self> for TimeSet<T>
{
    fn bitor_assign(&mut self, tw: Self) {
        // fixme: suppress cloning
        *self = self.clone().bitor(tw)
    }
}

impl<T:TimePoint> BitOrAssign<&Self> for TimeSet<T>
{
    fn bitor_assign(&mut self, tw: &Self) {
        // fixme: suppress cloning
        *self = self.clone().bitor(tw)
    }
}

impl<T:TimePoint> BitOr<Self> for TimeSet<T>
{
    type Output = Self;
    #[inline] fn bitor(self, tw: Self) -> Self::Output { (&self).bitor(tw) }
}

impl<T:TimePoint> BitOr<&Self> for TimeSet<T>
{
    type Output = Self;
    #[inline] fn bitor(self, tw: &Self) -> Self::Output { (&self).bitor(tw) }
}

impl<T:TimePoint, TW> BitOr<TW> for TimeSet<T>
    where TW: TimeConvex<TimePoint=T>
{
    type Output = Self;
    #[inline] fn bitor(self, tw: TW) -> Self::Output { (&self).bitor(tw) }
}


impl<T:TimePoint> BitOr<TimeSet<T>> for &TimeSet<T>
{
    type Output = TimeSet<T>;

    #[inline]
    fn bitor(self, tw: TimeSet<T>) -> Self::Output {
        self.into_iter().union(tw.into_iter()).collect()
    }
}

impl<T:TimePoint> BitOr<Self> for &TimeSet<T>
{
    type Output = TimeSet<T>;

    #[inline]
    fn bitor(self, tw: &TimeSet<T>) -> Self::Output {
        self.into_iter().union(tw.into_iter()).collect()
    }
}


impl<T:TimePoint, TW> BitOr<TW> for &TimeSet<T>
    where TW: TimeConvex<TimePoint=T>
{
    type Output = TimeSet<T>;

    #[inline]
    fn bitor(self, tw: TW) -> Self::Output {
        self.into_iter().union(tw.into()).collect()
    }
}

