use std::ops::{BitOr, BitOrAssign};
use crate::*;
use crate::iter::TimeUnion;


impl<T:TimePoint,TW> BitOrAssign<TW> for TimeSet<T>
    where TW: Into<TimeInterval<T>>
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

impl<T:TimePoint,TW> BitOr<TW> for TimeInterval<T>
    where TW: Into<TimeInterval<T>>
{
    type Output = TimeSet<T>;
    #[inline] fn bitor(self, tw: TW) -> Self::Output { (&self).bitor(tw) }
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
    where TW: Into<TimeInterval<T>>
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
    where TW: Into<TimeInterval<T>>
{
    type Output = TimeSet<T>;

    #[inline]
    fn bitor(self, tw: TW) -> Self::Output {
        self.into_iter().union(tw.into()).collect()
    }
}


impl<T:TimePoint,TW> BitOr<TW> for &TimeInterval<T>
    where TW: Into<TimeInterval<T>>
{
    type Output = TimeSet<T>;

    fn bitor(self, tw: TW) -> Self::Output
    {
        let i = tw.into();
        if self.is_empty() {
            if i.is_empty() {
                TimeSet::empty()
            } else {
                i.into()
            }
        } else {
            if i.is_empty() {
                (*self).into()
            } else {
                if i.upper < self.lower.just_before() {
                    TimeSet(vec![i, *self])
                } else if self.upper < i.lower.just_before() {
                    TimeSet(vec![*self, i])
                } else {
                    TimeSet(vec![
                        TimeInterval {
                            lower: self.lower.min(i.lower),
                            upper: self.upper.max(i.upper)
                        }
                    ])
                }
            }
        }
    }
}