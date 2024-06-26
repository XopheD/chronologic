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
    where TW: TimeConvex<TimePoint=T> + Debug
{
    fn bitor_assign(&mut self, tw: TW)
    {
        if !tw.is_empty() { // nothing to do for union with an empty interval
            let first = self.0.iter().rposition(|i| i.lower_bound() <= tw.upper_bound().just_after());
            let last = self.0.iter().position(|i| i.upper_bound() >= tw.lower_bound().just_before());
            match (first, last) {
                (None, None) => { // self was initially empty
                    *self = TimeSet::from(tw)
                }
                (Some(first), None) => {
                    // self:            |---|   |----|   |---|  |----|
                    // tw:               |------------|
                    // tw:      |-----------------------|
                    // tw:      |----|
                    // SAFETY: first is valid since it is returned by rposition
                    let w1 = unsafe { self.0.get_unchecked_mut(first) };
                    if w1.lower_bound().just_before() > tw.upper_bound() {
                        // the new interval should be inserted before
                        if first > 0 {
                            // SAFETY: first > 0 so an (obsolete) element exists just before
                            unsafe { *self.0.get_unchecked_mut(first-1) = tw.into() }
                            self.0.drain(0..first-1);
                        } else {
                            // no available cell, all the elements should be shifted
                            self.0.insert(0, tw.into())
                        }
                    } else {
                        // the new interval is merged with the first one
                        if w1.upper < tw.upper_bound() { w1.upper = tw.upper_bound(); }
                        w1.lower = tw.lower_bound();
                        self.0.drain(0..first); // drop the obsolete ones
                    }
                }
                (None, Some(last)) =>  {
                    // self:   |---|   |----|   |---|
                    // tw:                 |--------------|
                    // tw:                    |------------|
                    // tw:                                   |-----|
                    // SAFETY: first is valid since it is returned by rposition
                    let w2 = unsafe { self.0.get_unchecked_mut(last) };
                    if w2.upper_bound().just_after() < tw.lower_bound() {
                        // the new interval should be inserted at the end
                        self.0.push(tw.into())
                    } else {
                        w2.upper = tw.upper_bound();
                        if w2.lower > tw.lower_bound() { w2.lower = tw.lower_bound(); }
                        self.0.truncate(last + 1);
                    }
                }
                (Some(first), Some(last)) => {
                    // self:   |---|   |----|   |---|  |----|   |--|
                    // tw:               |--------------------|
                    let w1 = unsafe { self.0.get_unchecked_mut(first) };
                    if w1.lower > tw.lower_bound() { w1.lower = tw.lower_bound(); }
                    let w2 = unsafe { self.0.get_unchecked_mut(last) };
                    if w2.upper < tw.upper_bound() { w2.upper = tw.upper_bound(); }
                    if first < last { self.0.drain(first+1..last); }
                }
            }
        }
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

impl<T:TimePoint,TW> BitOr<TW> for TimeSet<T>
    where TW: TimeConvex<TimePoint=T> + Debug
{
    type Output = Self;
    #[inline] fn bitor(mut self, tw: TW) -> Self::Output {
        self |= tw; self
    }
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
    where TW: TimeConvex<TimePoint=T> + Debug
{
    type Output = TimeSet<T>;

    #[inline]
    fn bitor(self, tw: TW) -> Self::Output {
        self.clone() | tw
    }
}

