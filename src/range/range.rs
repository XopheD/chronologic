use std::ops::{Range, Add, Sub, AddAssign, SubAssign};
use crate::*;

impl<T:TimePoint> TimeWindow for Range<T>
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

impl<T:TimePoint> TimeConvex for Range<T> {}

impl<T:TimePoint+TimeTranslation> TimeTranslation for Range<T>
{
    fn translate(&self, t: TimeValue) -> TimeResult<Self>
    {
        if self.is_empty() {
            Err(TimeError::EmptyInterval)
        } else {
            let start = self.start.translate(t)?;
            let end = self.end.translate(t)?;
            if start.is_future_infinite() {
                Err(TimeError::FutureOverflow)
            } else if end.is_past_infinite() {
                Err(TimeError::PastOverflow)
            } else {
                Ok(start..end)
            }
        }
    }
}

impl<T:TimePoint> From<Range<T>> for TimeInterval<T>
{
    #[inline]
    fn from(range: Range<T>) -> Self {
        TimeInterval::new(range.start, range.end.just_before()).unwrap()
    }
}

//--------------------- TIME RANGE TRANSLATION -----------------------------------


impl<T> AddAssign<Range<TimeValue>> for TimeInterval<T>
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

impl<T> SubAssign<Range<TimeValue>> for TimeInterval<T>
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

impl<T> Add<Range<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn add(self, other: Range<TimeValue>) -> Self::Output {
        TimeInterval::new(
            self.lower+other.lower_bound(),
            self.upper+other.upper_bound()
        ).unwrap()
    }
}

impl<T> Sub<Range<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: Range<TimeValue>) -> Self::Output {
        TimeInterval::new(
            self.lower-other.upper_bound(),
            self.upper-other.lower_bound()
        ).unwrap()
    }
}

impl Add<Range<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: Range<Timestamp>) -> Self::Output {
        TimeInterval::new(
            self.lower+other.lower_bound(),
            self.upper+other.upper_bound()
        ).unwrap()
    }
}

impl Sub<Range<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: Range<Timestamp>) -> Self::Output {
        TimeInterval::new(
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
        *self += TimeSpan::from(other);
    }
}

impl<T> SubAssign<Range<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    #[inline]
    fn sub_assign(&mut self, other: Range<TimeValue>) {
        *self -= TimeSpan::from(other)
    }
}

impl<T> Add<Range<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = TimeSet<T>;
    #[inline]
    fn add(self, other: Range<TimeValue>) -> Self::Output {
        self + TimeSpan::from(other)
    }
}

impl<T> Sub<Range<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = TimeSet<T>;
    #[inline] fn sub(self, other: Range<TimeValue>) -> Self::Output {
        self - TimeSpan::from(other)
    }
}
