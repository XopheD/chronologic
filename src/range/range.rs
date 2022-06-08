use std::ops::{Range, Add, Sub, AddAssign, SubAssign};
use crate::*;
use crate::error::TimeError;

impl<T:TimePoint> TimeSpan for Range<T>
{
    type TimePoint = T;
    #[inline] fn is_empty(&self) -> bool { Range::is_empty(self) }
    #[inline] fn is_singleton(&self) -> bool { !self.is_empty() && self.lower_bound() == self.upper_bound() }
    #[inline] fn is_bounded(&self) -> bool { !self.is_empty() && self.lower_bound().is_finite() && self.upper_bound().is_finite() }
    #[inline] fn is_low_bounded(&self) -> bool { !self.is_empty() && self.lower_bound().is_finite() }
    #[inline] fn is_up_bounded(&self) -> bool { !self.is_empty() && self.upper_bound().is_finite()  }
    #[inline] fn is_convex(&self) -> bool { true }
    #[inline] fn lower_bound(&self) -> Self::TimePoint { self.start }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { self.end.just_before() } // not inclusive
}

impl<T:TimePoint> TimeConvex for Range<T> {
    #[inline]
    fn to_timerange(&self) -> TimeRange<T> {
        TimeRange::new(self.start, self.end.just_before()).unwrap()
    }
}

impl<T:TimePoint> TryFrom<Range<T>> for TimeRange<T>
{
    type Error = TimeError;

    #[inline]
    fn try_from(range: Range<T>) -> Result<Self, Self::Error> {
        TimeRange::new(range.start, range.end.just_before())
    }
}

//--------------------- TIME RANGE TRANSLATION -----------------------------------


impl<T> AddAssign<Range<TimeValue>> for TimeRange<T>
    where T:TimePoint+AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, other: Range<TimeValue>)
    {
        assert! [ !other.is_empty() ];
        self.lower += other.lower_bound();
        assert!( !self.lower.is_future_infinite() );
        self.upper += other.upper_bound();
        assert!( !self.upper.is_past_infinite() );
    }
}

impl<T> SubAssign<Range<TimeValue>> for TimeRange<T>
    where T:TimePoint+SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, other: Range<TimeValue>)
    {
        assert! [ !other.is_empty() ];
        self.lower -= other.upper_bound();
        assert!( !self.lower.is_future_infinite() );
        self.upper -= other.lower_bound();
        assert!( !self.upper.is_past_infinite() );
    }
}

impl<T> Add<Range<TimeValue>> for TimeRange<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn add(self, other: Range<TimeValue>) -> Self::Output {
        TimeRange::new(
            self.lower+other.lower_bound(),
            self.upper+other.upper_bound()
        ).unwrap()
    }
}

impl<T> Sub<Range<TimeValue>> for TimeRange<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: Range<TimeValue>) -> Self::Output {
        TimeRange::new(
            self.lower-other.upper_bound(),
            self.upper-other.lower_bound()
        ).unwrap()
    }
}

impl Add<Range<Timestamp>> for TimeInterval
{
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: Range<Timestamp>) -> Self::Output {
        TimeRange::new(
            self.lower+other.lower_bound(),
            self.upper+other.upper_bound()
        ).unwrap()
    }
}

impl Sub<Range<Timestamp>> for TimeInterval
{
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: Range<Timestamp>) -> Self::Output {
        TimeRange::new(
            self.lower-other.upper_bound(),
            self.upper-other.lower_bound()
        ).unwrap()
    }
}


//--------------------- TIME SET TRANSLATION -----------------------------------


impl<T> AddAssign<Range<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    #[inline]
    fn add_assign(&mut self, other: Range<TimeValue>) {
        *self += other.to_timerange();
    }
}

impl<T> SubAssign<Range<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    #[inline]
    fn sub_assign(&mut self, other: Range<TimeValue>) {
        *self -= other.to_timerange()
    }
}

impl<T> Add<Range<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = TimeSet<T>;
    #[inline]
    fn add(self, other: Range<TimeValue>) -> Self::Output {
        self + other.to_timerange()
    }
}

impl<T> Sub<Range<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = TimeSet<T>;
    #[inline] fn sub(self, other: Range<TimeValue>) -> Self::Output {
        self - other.to_timerange()
    }
}
