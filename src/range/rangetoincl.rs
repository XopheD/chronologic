use std::ops::{RangeToInclusive, Add, Sub, AddAssign, SubAssign};
use crate::*;

impl<T:TimePoint> TimeWindow for RangeToInclusive<T>
{
    type TimePoint = T;
    #[inline] fn is_empty(&self) -> bool { self.upper_bound().is_past_infinite() }
    #[inline] fn is_singleton(&self) -> bool { false }
    #[inline] fn is_bounded(&self) -> bool { false }
    #[inline] fn is_low_bounded(&self) -> bool { false }
    #[inline] fn is_up_bounded(&self) -> bool { self.upper_bound().is_finite()  }
    #[inline] fn is_convex(&self) -> bool { true }
    #[inline] fn lower_bound(&self) -> Self::TimePoint { - Self::TimePoint::INFINITE }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { self.end } // inclusive
}

impl<T:TimePoint> TimeConvex for RangeToInclusive<T> {}

impl<T:TimePoint+TimeTranslation> TimeTranslation for RangeToInclusive<T>
{
    fn translate(&self, t: TimeValue) -> TimeResult<Self>
    {
        let end = self.end.translate(t)?;
        if end.is_past_infinite() {
            Err(TimeError::PastOverflow)
        } else {
            Ok(..=end)
        }
    }
}


impl<T:TimePoint> From<RangeToInclusive<T> > for TimeInterval<T>
{
    #[inline]
    fn from(range: RangeToInclusive<T>) -> Self {
        assert!( !range.end.is_past_infinite() );
        TimeInterval { lower: -T::INFINITE, upper: range.end }
    }
}

//--------------------- TIME RANGE TRANSLATION -----------------------------------

impl<T> AddAssign<RangeToInclusive<TimeValue>> for TimeInterval<T>
    where T:TimePoint+AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, other: RangeToInclusive<TimeValue>) {
        self.upper += other.upper_bound();
        assert!( !self.upper.is_past_infinite() );
        self.lower = -T::INFINITE;
    }
}

impl<T> SubAssign<RangeToInclusive<TimeValue>> for TimeInterval<T>
    where T:TimePoint+SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeToInclusive<TimeValue>) {
        self.lower -= other.upper_bound();
        assert!( !self.lower.is_future_infinite() );
        self.upper = T::INFINITE;
    }
}

impl<T> Add<RangeToInclusive<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn add(self, other: RangeToInclusive<TimeValue>) -> Self::Output {
        TimeInterval::before(self.upper + other.upper_bound()).unwrap()
    }
}

impl<T> Sub<RangeToInclusive<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: RangeToInclusive<TimeValue>) -> Self::Output {
        TimeInterval::after(self.lower - other.upper_bound()).unwrap()
    }
}

impl Add<RangeToInclusive<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: RangeToInclusive<Timestamp>) -> Self::Output {
        TimeInterval::before(self.upper + other.upper_bound()).unwrap()
    }
}

impl Sub<RangeToInclusive<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: RangeToInclusive<Timestamp>) -> Self::Output {
        TimeInterval::after(self.lower - other.upper_bound()).unwrap()
    }
}


//--------------------- TIME SET TRANSLATION -----------------------------------


impl<T> AddAssign<RangeToInclusive<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    #[inline]
    fn add_assign(&mut self, other: RangeToInclusive<TimeValue>) {
        *self += TimeSpan::from(other);
    }
}

impl<T> SubAssign<RangeToInclusive<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeToInclusive<TimeValue>) {
        *self -= TimeSpan::from(other)
    }
}

impl<T> Add<RangeToInclusive<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = TimeInterval<T>;
    #[inline]
    fn add(self, other: RangeToInclusive<TimeValue>) -> Self::Output {
        TimeInterval::before(self.upper_bound() + other.upper_bound()).unwrap()
    }
}

impl<T> Sub<RangeToInclusive<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = TimeInterval<T>;
    #[inline] fn sub(self, other: RangeToInclusive<TimeValue>) -> Self::Output {
        TimeInterval::after(self.lower_bound() - other.upper_bound()).unwrap()
    }
}
