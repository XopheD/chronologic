use std::ops::*;
use crate::*;
use crate::iter::*;

// Convenient implementation to use range notation in Rust code

impl<T:TimePoint> TimeBounds for Range<T>
{
    type TimePoint = T;
    #[inline] fn is_empty(&self) -> bool { Range::is_empty(self) }
    #[inline] fn is_bounded(&self) -> bool { !self.is_empty() && self.lower_bound().is_finite() && self.upper_bound().is_finite() }
    #[inline] fn is_low_bounded(&self) -> bool { !self.is_empty() && self.lower_bound().is_finite() }
    #[inline] fn is_up_bounded(&self) -> bool { !self.is_empty() && self.upper_bound().is_finite()  }
    #[inline] fn lower_bound(&self) -> Self::TimePoint { if self.is_empty() { T::INFINITE } else { self.start } }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { if self.is_empty() { -T::INFINITE} else { self.end.just_before() } }// not inclusive
}

impl<T:TimePoint> TimeBounds for RangeFrom<T>
{
    type TimePoint = T;
    #[inline] fn is_empty(&self) -> bool { false }
    #[inline] fn is_bounded(&self) -> bool { false }
    #[inline] fn is_low_bounded(&self) -> bool { self.lower_bound().is_finite() }
    #[inline] fn is_up_bounded(&self) -> bool { false }
    #[inline] fn lower_bound(&self) -> Self::TimePoint { self.start }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { Self::TimePoint::INFINITE }
}

impl<T:TimePoint> TimeBounds for RangeInclusive<T>
{
    type TimePoint = T;
    #[inline] fn is_empty(&self) -> bool { RangeInclusive::is_empty(self) }
    #[inline] fn is_bounded(&self) -> bool { !self.is_empty() && self.lower_bound().is_finite() && self.upper_bound().is_finite() }
    #[inline] fn is_low_bounded(&self) -> bool { !self.is_empty() && self.lower_bound().is_finite() }
    #[inline] fn is_up_bounded(&self) -> bool { !self.is_empty() && self.upper_bound().is_finite()  }
    #[inline] fn lower_bound(&self) -> Self::TimePoint { if self.is_empty() { T::INFINITE} else { *self.start() } }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { if self.is_empty() { -T::INFINITE } else { *self.end() } } // inclusive
}

impl<T:TimePoint> TimeBounds for RangeTo<T>
{
    type TimePoint = T;
    #[inline] fn is_empty(&self) -> bool { false }
    #[inline] fn is_bounded(&self) -> bool { false }
    #[inline] fn is_low_bounded(&self) -> bool { false }
    #[inline] fn is_up_bounded(&self) -> bool { self.upper_bound().is_finite()  }
    #[inline] fn lower_bound(&self) -> Self::TimePoint { - Self::TimePoint::INFINITE }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { self.end.just_before() } // not inclusive
}

impl<T:TimePoint> TimeBounds for RangeToInclusive<T>
{
    type TimePoint = T;
    #[inline] fn is_empty(&self) -> bool { false }
    #[inline] fn is_bounded(&self) -> bool { false }
    #[inline] fn is_low_bounded(&self) -> bool { false }
    #[inline] fn is_up_bounded(&self) -> bool { self.upper_bound().is_finite()  }
    #[inline] fn lower_bound(&self) -> Self::TimePoint { - Self::TimePoint::INFINITE }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { self.end } // inclusive
}

// not really useful, just for fun...
impl<T:TimePoint> From<RangeFull> for TimeInterval<T> {
    #[inline] fn from(_: RangeFull) -> Self { TimeInterval::all() }
}


macro_rules! timerange {
    ($range:ident) => {
        impl<T:TimePoint> TimeConvex for $range<T> { }
        impl<T:TimePoint> TimeComplementary for $range<T> {
            type Output = <TimeInterval<T> as TimeComplementary>::Output;
            #[inline] fn complementary(self) -> Self::Output {
                Into::<TimeInterval<_>>::into(self).complementary()
            }
        }
        impl<T:TimePoint,TW> TimeUnion<TW> for $range<T>
            where TimeInterval<T>: TimeUnion<TW>
        {
            type Output = <TimeInterval<T> as TimeUnion<TW>>::Output;
            #[inline] fn union(self, tw: TW) -> Self::Output {
                Into::<TimeInterval<_>>::into(self).union(tw)
            }
        }
        impl<T:TimePoint,TW> TimeIntersection<TW> for $range<T>
            where TimeInterval<T>: TimeIntersection<TW>
        {
            type Output = <TimeInterval<T> as TimeIntersection<TW>>::Output;
            #[inline] fn intersection(self, tw: TW) -> Self::Output {
                Into::<TimeInterval<_>>::into(self).intersection(tw)
            }
        }
        impl<T:TimePoint> TimeUnion<TimeSet<T>> for $range<T>
        {
            type Output = TimeSet<T>;
            #[inline] fn union(self, tw: TimeSet<T>) -> Self::Output { tw.union(self) }
        }
        impl<T:TimePoint> TimeUnion<&TimeSet<T>> for $range<T>
        {
            type Output = TimeSet<T>;
            #[inline] fn union(self, tw: &TimeSet<T>) -> Self::Output { tw.union(self) }
        }
        impl<T:TimePoint> TimeIntersection<TimeSet<T>> for $range<T>
        {
            type Output = TimeSet<T>;
            #[inline] fn intersection(self, tw: TimeSet<T>) -> Self::Output { tw.intersection(self) }
        }
        impl<T:TimePoint> TimeIntersection<&TimeSet<T>> for $range<T>
        {
            type Output = TimeSet<T>;
            #[inline] fn intersection(self, tw: &TimeSet<T>) -> Self::Output { tw.intersection(self) }
        }
        impl<TW:TimeConvexIterator> TimeUnion<$range<TW::TimePoint>> for TW {
            type Output = UnionIter<Self,<TimeInterval<TW::TimePoint> as IntoIterator>::IntoIter>;
            #[inline] fn union(self, tw: $range<TW::TimePoint>) -> Self::Output {
                self.union(Into::<TimeInterval<_>>::into(tw).into_iter())
            }
        }
        impl<TW:TimeConvexIterator> TimeIntersection<$range<TW::TimePoint>> for TW {
            type Output = InterIter<Self,<TimeInterval<TW::TimePoint> as IntoIterator>::IntoIter>;
            #[inline] fn intersection(self, tw: $range<TW::TimePoint>) -> Self::Output {
                self.intersection(Into::<TimeInterval<_>>::into(tw).into_iter())
            }
        }
        impl<T:TimePoint> From<$range<T>> for TimeInterval<T> {
            #[inline]
            fn from(range: $range<T>) -> Self {
                TimeInterval { lower: range.lower_bound(), upper: range.upper_bound() }
            }
        }
    }
}

timerange!(Range);
timerange!(RangeFrom);
timerange!(RangeInclusive);
timerange!(RangeTo);
timerange!(RangeToInclusive);
