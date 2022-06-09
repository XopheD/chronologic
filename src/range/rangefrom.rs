use std::ops::{RangeFrom, Add, Sub, AddAssign, SubAssign};
use crate::*;
use crate::error::TimeError;

impl<T:TimePoint> TimeWindow for RangeFrom<T>
{
    type TimePoint = T;
    #[inline] fn is_empty(&self) -> bool { self.lower_bound().is_future_infinite() }
    #[inline] fn is_singleton(&self) -> bool { false }
    #[inline] fn is_bounded(&self) -> bool { false }
    #[inline] fn is_low_bounded(&self) -> bool { self.lower_bound().is_finite() }
    #[inline] fn is_up_bounded(&self) -> bool { false }
    #[inline] fn is_convex(&self) -> bool { true }
    #[inline] fn lower_bound(&self) -> Self::TimePoint { self.start }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { Self::TimePoint::INFINITE }
}

impl<T:TimePoint> From<RangeFrom<T> > for TimeInterval<T>
{
    #[inline]
    fn from(range: RangeFrom<T>) -> Self {
        assert!( !range.start.is_future_infinite() );
        TimeInterval { lower: range.start, upper: T::INFINITE}
    }
}

//--------------------- TIME RANGE TRANSLATION -----------------------------------

impl<T> AddAssign<RangeFrom<TimeValue>> for TimeInterval<T>
    where T:TimePoint+AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, other: RangeFrom<TimeValue>) {
        self.lower += other.lower_bound();
        assert!( !self.lower.is_future_infinite() );
        self.upper = T::INFINITE;
    }
}

impl<T> SubAssign<RangeFrom<TimeValue>> for TimeInterval<T>
    where T:TimePoint+SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeFrom<TimeValue>) {
        self.upper -= other.lower_bound();
        assert!( !self.upper.is_past_infinite() );
        self.lower = -T::INFINITE;
    }
}

impl<T> Add<RangeFrom<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn add(self, other: RangeFrom<TimeValue>) -> Self::Output {
        TimeInterval::after(self.lower + other.lower_bound()).unwrap()
    }
}

impl<T> Sub<RangeFrom<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: RangeFrom<TimeValue>) -> Self::Output {
        TimeInterval::before(self.upper - other.lower_bound()).unwrap()
    }
}

impl Add<RangeFrom<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: RangeFrom<Timestamp>) -> Self::Output {
        TimeInterval::after(self.lower + other.lower_bound()).unwrap()
    }
}

impl Sub<RangeFrom<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: RangeFrom<Timestamp>) -> Self::Output {
        TimeInterval::before(self.upper - other.lower_bound()).unwrap()
    }
}


//--------------------- TIME SET TRANSLATION -----------------------------------


impl<T> AddAssign<RangeFrom<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    #[inline]
    fn add_assign(&mut self, other: RangeFrom<TimeValue>) {
        *self += TimeSpan::from(other)
    }
}

impl<T> SubAssign<RangeFrom<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeFrom<TimeValue>) {
        *self -= TimeSpan::from(other)
    }
}

impl<T> Add<RangeFrom<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = TimeInterval<T>;
    #[inline]
    fn add(self, other: RangeFrom<TimeValue>) -> Self::Output {
        TimeInterval::after(self.lower_bound() + other.lower_bound()).unwrap()
    }
}

impl<T> Sub<RangeFrom<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = TimeInterval<T>;
    #[inline] fn sub(self, other: RangeFrom<TimeValue>) -> Self::Output {
        TimeInterval::before(self.upper_bound() - other.upper_bound()).unwrap()
    }
}
