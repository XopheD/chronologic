use std::ops::{RangeInclusive, Add, Sub, AddAssign, SubAssign};
use crate::*;


impl<T:TimePoint> TimeWindow for RangeInclusive<T>
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

impl<T:TimePoint> TimeConvex for RangeInclusive<T> {}

impl<T:TimePoint+TimeTranslation> TimeTranslation for RangeInclusive<T>
{
    fn translate(&self, t: TimeValue) -> TimeResult<Self>
    {
        if self.is_empty() {
            Err(TimeError::EmptyInterval)
        } else {
            let lower = self.lower_bound().translate(t)?;
            let upper = self.upper_bound().translate(t)?;
            if lower.is_future_infinite() {
                Err(TimeError::FutureOverflow)
            } else if upper.is_past_infinite() {
                Err(TimeError::PastOverflow)
            } else {
                Ok(lower..=upper)
            }
        }
    }
}

impl<T:TimePoint> From<RangeInclusive<T> > for TimeInterval<T>
{
    #[inline]
    fn from(range: RangeInclusive<T>) -> Self {
        TimeInterval::new(range.lower_bound(), range.upper_bound()).unwrap()
    }
}


//--------------------- TIME RANGE TRANSLATION -----------------------------------

impl<T> AddAssign<RangeInclusive<TimeValue>> for TimeInterval<T>
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


impl<T> SubAssign<RangeInclusive<TimeValue>> for TimeInterval<T>
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

impl<T> Add<RangeInclusive<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn add(self, other: RangeInclusive<TimeValue>) -> Self::Output {
        TimeInterval::new(
            self.lower+other.lower_bound(),
            self.upper+other.upper_bound()
        ).unwrap()
    }
}

impl<T> Sub<RangeInclusive<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: RangeInclusive<TimeValue>) -> Self::Output {
        TimeInterval::new(
            self.lower-other.upper_bound(),
            self.upper-other.lower_bound()
        ).unwrap()
    }
}

impl Add<RangeInclusive<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: RangeInclusive<Timestamp>) -> Self::Output {
        TimeInterval::new(
            self.lower+other.lower_bound(),
            self.upper+other.upper_bound()
        ).unwrap()
    }
}

impl Sub<RangeInclusive<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: RangeInclusive<Timestamp>) -> Self::Output {
        TimeInterval::new(
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
        *self += TimeSpan::from(other);
    }
}

impl<T> SubAssign<RangeInclusive<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeInclusive<TimeValue>) {
        *self -= TimeSpan::from(other);
    }
}

impl<T> Add<RangeInclusive<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = TimeSet<T>;
    #[inline]
    fn add(self, other: RangeInclusive<TimeValue>) -> Self::Output {
        self + TimeSpan::from(other)
    }
}

impl<T> Sub<RangeInclusive<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = TimeSet<T>;
    #[inline] fn sub(self, other: RangeInclusive<TimeValue>) -> Self::Output {
        self - TimeSpan::from(other)
    }
}
