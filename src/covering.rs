use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use crate::*;


pub trait TimeCovering<TW> {

    /// Checks if this time window covers the specified one.
    ///
    /// It means that the specified window is completely included in this time window.
    fn covers(&self, tw: &TW) -> bool;

    /// Checks if the specified time point is inside this time window.
    fn contains<I>(&self, item: &I) -> bool
        where
            Self::TimePoint: PartialOrd<I>,
            I: PartialOrd<Self::TimePoint>;
}

impl<TW:TimeSpan> TimeCovering<TW> for TimeSingleton<TW::TimePoint> {
    #[inline]
    fn covers(&self, tw: &TW) -> bool {
        tw.is_singleton() && self.get() == tw.lower_bound()
    }
}

impl<TW:TimeSpan> TimeCovering<TW> for TimeRange<TW::TimePoint> {
    #[inline]
    fn covers(&self, tw: &TW) -> bool {
        self.lower_bound() <= tw.lower_bound() && tw.upper_bound() <= self.upper_bound()
    }
    /*
    #[inline]
    fn contains<I>(&self, item: &I) -> bool
        where
            Self::TimePoint: PartialOrd<I>,
            I: PartialOrd<Self::TimePoint>
    {
        self.lower <= *item && *item <= self.upper
    }*/

}

impl<TW:TimeSpan> TimeCovering<TW> for RangeFrom<TW::TimePoint> {
    #[inline]
    fn covers(&self, tw: &TW) -> bool {
        self.lower_bound() <= tw.lower_bound()
    }
}

impl<TW:TimeSpan> TimeCovering<TW> for Range<TW::TimePoint> {
    #[inline]
    fn covers(&self, tw: &TW) -> bool {
        self.lower_bound() <= tw.lower_bound() && tw.upper_bound() < self.upper_bound()
    }
}
impl<TW:TimeSpan> TimeCovering<TW> for RangeInclusive<TW::TimePoint> {
    #[inline]
    fn covers(&self, tw: &TW) -> bool {
        self.lower_bound() <= tw.lower_bound() && tw.upper_bound() <= self.upper_bound()
    }
}

impl<TW:TimeSpan> TimeCovering<TW> for RangeTo<TW::TimePoint> {
    #[inline]
    fn covers(&self, tw: &TW) -> bool {
        tw.upper_bound() < self.upper_bound()
    }
}

impl<TW:TimeSpan> TimeCovering<TW> for RangeToInclusive<TW::TimePoint> {
    #[inline]
    fn covers(&self, tw: &TW) -> bool {
        tw.upper_bound() <= self.upper_bound()
    }
}

impl<TW:TimeSpan> TimeCovering<TW> for RangeFull {
    #[inline]
    fn covers(&self, _: &TW) -> bool { true }
}

impl<TW:TimeSpan> TimeCovering<RangeFull> for TW {
    #[inline]
    fn covers(&self, _: &RangeFull) -> bool {
        !self.is_low_bounded() && !self.is_up_bounded()
    }
}


impl<T:TimePoint> TimeCovering<TimeSingleton<T>> for TimeSet<T> {
    #[inline]
    fn covers(&self, tw: &TimeSingleton<T>) -> bool {
        self.iter()
            .skip_while(|ts| ts.upper_bound() < tw.get())
            .next()
            .map(|cvx| cvx.lower_bound() <= tw.get())
            .unwrap_or(false)
    }

/*
    fn contains<I>(&self, t: &I) -> bool
        where Self::TimePoint: PartialOrd<I>, I: PartialOrd<Self::TimePoint>
    {
        self.0.iter()
            .skip_while(|ts| ts.upper_bound() < *t)
            .next()
            .map(|ts| ts.lower_bound() <= *t)
            .unwrap_or(false)
    }
*/
}

impl<T:TimePoint> TimeCovering<TimeRange<T>> for TimeSet<T>  {
    #[inline]
    fn covers(&self, tw: &TimeRange<T>) -> bool {
        self.iter()
            .skip_while(|ts| ts.upper_bound() < tw.upper_bound())
            .next()
            .map(|cvx| cvx.lower_bound() <= tw.lower_bound())
            .unwrap_or(false)
    }
}

impl<T:TimePoint> TimeCovering<RangeFrom<T>> for TimeSet<T> {
    #[inline]
    fn covers(&self, tw: &RangeFrom<T>) -> bool {
        self.0.last()
            .map(|cvx| !cvx.is_up_bounded() && cvx.lower_bound() <= tw.lower_bound())
            .unwrap_or(false)
    }
}

impl<T:TimePoint> TimeCovering<Range<T>> for TimeSet<T> {
    #[inline]
    fn covers(&self, tw: &Range<T>) -> bool {
        self.lower_bound() <= tw.lower_bound() && tw.upper_bound() < self.upper_bound()
    }
}
impl<T:TimePoint> TimeCovering<RangeInclusive<T>> for TimeSet<T> {
    #[inline]
    fn covers(&self, tw: &RangeInclusive<T>) -> bool {
        self.lower_bound() <= tw.lower_bound() && tw.upper_bound() <= self.upper_bound()
    }
}

impl<T:TimePoint> TimeCovering<RangeTo<T>> for TimeSet<T> {
    #[inline]
    fn covers(&self, tw: &RangeTo<T>) -> bool {
        tw.upper_bound() < self.upper_bound()
    }
}

impl<T:TimePoint> TimeCovering<RangeToInclusive<T>> for TimeSet<T> {
    #[inline]
    fn covers(&self, tw: &RangeToInclusive<T>) -> bool {
        tw.upper_bound() <= self.upper_bound()
    }
}


impl<T:TimePoint> TimeCovering<Self> for TimeSet<T> {
    #[inline]
    fn covers(&self, tw: &Self) -> bool
    {
        // todo : should be more efficient (but it works)
        tw.0.iter().all(|tw| self.covers(tw))
    }
}