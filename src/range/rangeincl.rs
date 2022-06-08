use std::ops::{RangeInclusive, Add, Sub, AddAssign, SubAssign};
use crate::*;
use crate::error::TimeError;


impl<T:TimePoint> TimeSpan for RangeInclusive<T>
{
    type TimePoint = T;
    #[inline] fn is_empty(&self) -> bool { RangeInclusive::is_empty(self) }
    #[inline] fn is_singleton(&self) -> bool { !self.is_empty() && self.lower_bound() == self.upper_bound() }
    #[inline] fn is_bounded(&self) -> bool { !self.is_empty() && self.lower_bound().is_finite() && self.upper_bound().is_finite() }
    #[inline] fn is_low_bounded(&self) -> bool { !self.is_empty() && self.lower_bound().is_finite() }
    #[inline] fn is_up_bounded(&self) -> bool { !self.is_empty() && self.upper_bound().is_finite()  }
    #[inline] fn is_convex(&self) -> bool { true }
    #[inline] fn lower_bound(&self) -> Self::TimePoint { *self.start() }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { *self.end() } // inclusive
}

impl<T:TimePoint> TimeConvex for RangeInclusive<T> {
    #[inline]
    fn to_timerange(&self) -> TimeRange<T> {
        TimeRange::new(self.lower_bound(), self.upper_bound()).unwrap()
    }
}

impl<T:TimePoint> TryFrom<RangeInclusive<T> > for TimeRange<T>
{
    type Error = TimeError;
    #[inline]
    fn try_from(range: RangeInclusive<T>) -> Result<Self, Self::Error> {
        TimeRange::new(range.lower_bound(), range.upper_bound())
    }
}


//--------------------- TIME RANGE TRANSLATION -----------------------------------

impl<T> AddAssign<RangeInclusive<TimeValue>> for TimeRange<T>
    where T:TimePoint+AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, other: RangeInclusive<TimeValue>)
    {
        assert! [ !other.is_empty() ];
        self.lower += other.lower_bound();
        assert!( !self.lower.is_future_infinite() );
        self.upper += other.upper_bound();
        assert!( !self.upper.is_past_infinite() );
    }
}


impl<T> SubAssign<RangeInclusive<TimeValue>> for TimeRange<T>
    where T:TimePoint+SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeInclusive<TimeValue>)
    {
        assert! [ !other.is_empty() ];
        self.lower -= other.upper_bound();
        assert!( !self.lower.is_future_infinite() );
        self.upper -= other.lower_bound();
        assert!( !self.upper.is_past_infinite() );
    }
}

impl<T> Add<RangeInclusive<TimeValue>> for TimeRange<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn add(self, other: RangeInclusive<TimeValue>) -> Self::Output {
        TimeRange::new(
            self.lower+other.lower_bound(),
            self.upper+other.upper_bound()
        ).unwrap()
    }
}

impl<T> Sub<RangeInclusive<TimeValue>> for TimeRange<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: RangeInclusive<TimeValue>) -> Self::Output {
        TimeRange::new(
            self.lower-other.upper_bound(),
            self.upper-other.lower_bound()
        ).unwrap()
    }
}

impl Add<RangeInclusive<Timestamp>> for TimeInterval
{
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: RangeInclusive<Timestamp>) -> Self::Output {
        TimeRange::new(
            self.lower+other.lower_bound(),
            self.upper+other.upper_bound()
        ).unwrap()
    }
}

impl Sub<RangeInclusive<Timestamp>> for TimeInterval
{
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: RangeInclusive<Timestamp>) -> Self::Output {
        TimeRange::new(
            self.lower-other.upper_bound(),
            self.upper-other.lower_bound()
        ).unwrap()
    }
}


//--------------------- TIME SET TRANSLATION -----------------------------------


impl<T> AddAssign<RangeInclusive<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    #[inline]
    fn add_assign(&mut self, other: RangeInclusive<TimeValue>) {
        *self += other.to_timerange();
    }
}

impl<T> SubAssign<RangeInclusive<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeInclusive<TimeValue>) {
        *self -= other.to_timerange()
    }
}

impl<T> Add<RangeInclusive<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = TimeSet<T>;
    #[inline]
    fn add(self, other: RangeInclusive<TimeValue>) -> Self::Output {
        self + other.to_timerange()
    }
}

impl<T> Sub<RangeInclusive<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = TimeSet<T>;
    #[inline] fn sub(self, other: RangeInclusive<TimeValue>) -> Self::Output {
        self - other.to_timerange()
    }
}
