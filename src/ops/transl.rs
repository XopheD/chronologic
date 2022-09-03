use std::ops::*;
use crate::*;


//---------------------- TIMERANGE<T> OUTPUT ------------------------

impl<T> Add<TimeValue> for TimeInterval<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline] fn add(self, other: TimeValue) -> Self::Output {
        let tw = TimeInterval::new(self.lower + other, self.upper + other);
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

impl<T> Sub<TimeValue> for TimeInterval<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline] fn sub(self, other: TimeValue) -> Self::Output {
        let tw = TimeInterval::new(self.lower - other, self.upper - other);
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

impl<T> AddAssign<TimeValue> for TimeInterval<T>
    where T:TimePoint+AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, other: TimeValue) {
        self.lower += other;
        self.upper += other;
        assert!( (self.lower != self.upper) || !self.lower.is_finite() );
    }
}

impl<T> SubAssign<TimeValue> for TimeInterval<T>
    where T:TimePoint+SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, other: TimeValue) {
        self.lower -= other;
        self.upper -= other;
        assert!( (self.lower != self.upper) || !self.lower.is_finite() );
    }
}

impl<T> Add<TimeInterval<T>> for TimeValue
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = TimeInterval<T>;
    #[inline] fn add(self, other: TimeInterval<T>) -> Self::Output { other + self }
}

impl<T> Sub<TimeInterval<T>> for TimeValue
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = TimeInterval<T>;
    #[inline] fn sub(self, other: TimeInterval<T>) -> Self::Output { (-other) + self }
}

impl<T> Add<TimeSpan> for TimeInterval<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline] fn add(self, other: TimeSpan) -> Self::Output {
        let tw = TimeInterval::new(self.lower + other.lower, self.upper + other.upper);
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

impl<T> Sub<TimeSpan> for TimeInterval<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline] fn sub(self, other: TimeSpan) -> Self::Output {
        let tw = TimeInterval::new(self.lower - other.upper, self.upper - other.lower);
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

impl<T> AddAssign<TimeSpan> for TimeInterval<T>
    where T:TimePoint+AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, other: TimeSpan) {
        self.lower += other.lower;
        self.upper += other.upper;
        assert!( (self.lower != self.upper) || !self.lower.is_finite() );
    }
}

impl<T> SubAssign<TimeSpan> for TimeInterval<T>
    where T:TimePoint+SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, other: TimeSpan) {
        self.lower -= other.upper;
        self.upper -= other.lower;
        assert!( (self.lower != self.upper) || !self.lower.is_finite() );
    }
}

impl Add<TimeSpan> for Timestamp {
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: TimeSpan) -> Self::Output {
        let tw = TimeSlot::new(self + other.lower, self + other.upper);
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

impl Sub<TimeSpan> for Timestamp {
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: TimeSpan) -> Self::Output {
        let tw = TimeSlot::new(self - other.upper, self - other.lower);
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

impl Add<Timestamp> for TimeSpan {
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: Timestamp) -> Self::Output { other + self }
}

impl Sub<Timestamp> for TimeSpan {
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: Timestamp) -> Self::Output { (-other) + self }
}

impl Sub for TimeSlot {
    type Output = TimeSpan;
    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        let tw = TimeInterval::new(self.lower - other.upper,self.upper - other.lower);
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

impl Sub<Timestamp> for TimeSlot {
    type Output = TimeSpan;
    #[inline]
    fn sub(self, other: Timestamp) -> Self::Output {
        let tw = TimeInterval::new(self.lower - other,self.upper - other);
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

impl Sub<TimeSlot> for Timestamp {
    type Output = TimeSpan;
    #[inline]
    fn sub(self, other: TimeSlot) -> Self::Output {
        let tw = TimeInterval::new(self - other.upper,self - other.lower);
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

//------------------------ TIMESET<T> OUTPUT ------------------------

impl<T:TimePoint> Add<TimeValue> for TimeSet<T>
    where TimeInterval<T>: Add<TimeValue,Output=TimeInterval<T>>
{
    type Output = Self;
    #[inline]
    fn add(self, other: TimeValue) -> Self::Output {
        // adding a constant preserves the structure (order and distance
        // between successive intervals -> no new overlapping to manage)
        Self(self.0.iter().map(|tw| *tw + other).collect())
    }
}

impl<T:TimePoint> Sub<TimeValue> for TimeSet<T>
    where TimeInterval<T>: Sub<TimeValue,Output=TimeInterval<T>>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: TimeValue) -> Self::Output {
        // adding a constant preserves the structure (order and distance
        // between successive intervals -> no new overlapping to manage)
        Self(self.0.iter().map(|tw| *tw - other).collect())
    }
}

impl<T:TimePoint> AddAssign<TimeValue> for TimeSet<T>
    where TimeInterval<T>: AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, t: TimeValue) {
        // adding a constant preserves the structure (order and distance
        // between successive intervals -> no new overlapping to manage)
        self.0.iter_mut().for_each(|tw| *tw += t)
    }
}

impl<T:TimePoint> SubAssign<TimeValue> for TimeSet<T>
    where TimeInterval<T>: SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, t: TimeValue) {
        // adding a constant preserves the structure (order and distance
        // between successive intervals -> no new overlapping to manage)
        self.0.iter_mut().for_each(|tw| *tw -= t)
    }
}

impl Add<Timestamp> for TimeSpans
{
    type Output = TimeSlots;
    #[inline]
    fn add(self, other: Timestamp) -> Self::Output {
        // adding a constant preserves the structure (order and distance
        // between successive intervals -> no new overlapping to manage)
        TimeSet(self.0.iter().map(|tw| *tw + other).collect())
    }
}

impl Sub<Timestamp> for TimeSpans
{
    type Output = TimeSlots;
    #[inline]
    fn sub(self, other: Timestamp) -> Self::Output {
        // adding a constant preserves the structure (order and distance
        // between successive intervals -> no new overlapping to manage)
        TimeSet(self.0.iter().map(|tw| *tw - other).collect())
    }
}

impl Add<TimeSpans> for Timestamp
{
    type Output = TimeSlots;
    #[inline]
    fn add(self, other: TimeSpans) -> Self::Output { other + self }
}

impl Sub<TimeSpans> for Timestamp
{
    type Output = TimeSlots;
    #[inline]
    fn sub(self, other: TimeSpans) -> Self::Output {
        // adding a constant preserves the structure (order and distance
        // between successive intervals -> no new overlapping to manage)
        // but as we subtracts the intervals, the list should be reversed
        TimeSet(other.0.into_iter().rev().map(|i| self - i).collect())
    }
}

impl<T> Add<TimeSpan> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
        Self: BitOr<TimeInterval<T>,Output=Self>
{
    type Output = Self;
    #[inline]
    fn add(self, other: TimeSpan) -> Self::Output {
        self.0.into_iter()
            .fold(Self::empty(), |r,i| r|(i+other))
    }
}

impl<T> Sub<TimeSpan> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
        Self: BitOr<TimeInterval<T>,Output=Self>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: TimeSpan) -> Self::Output
    {
        self.0.into_iter()
            .fold(Self::empty(), |r,i| r|(i-other))
    }
}

impl<T> AddAssign<TimeSpan> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
        Self: Add<TimeSpan,Output=Self>
{
    #[inline]
    fn add_assign(&mut self, other: TimeSpan) {
        *self = self.clone() + other
    }
}

impl<T> SubAssign<TimeSpan> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
        Self: Sub<TimeSpan,Output=Self>
{
    #[inline]
    fn sub_assign(&mut self, other: TimeSpan) {
        *self = self.clone() - other
    }
}

impl<T> Add<TimeSpans> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
        TimeInterval<T>: Add<TimeInterval<TimeValue>,Output=TimeInterval<T>>,
        Self: BitOr<TimeInterval<T>,Output=Self>+TimeBounds<TimePoint=T>
{
    type Output = Self;
    #[inline]
    fn add(self, other: TimeSpans) -> Self::Output
    {
        self.0.into_iter()
            .map(|i| other.0.iter().copied().map(move |j| (i,j)))
            .flatten()
            .map(|(a,b)| a+b)
            .collect()
    }
}

impl<T> Sub<TimeSpans> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
        TimeInterval<T>: Sub<TimeInterval<TimeValue>,Output=TimeInterval<T>>,
        Self: BitOr<TimeInterval<T>,Output=Self>+TimeBounds<TimePoint=T>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: TimeSpans) -> Self::Output
    {
        self.0.into_iter()
            .map(|i| other.0.iter().copied().map(move |j| (i,j)))
            .flatten()
            .map(|(a,b)| a-b)
            .collect()
    }
}

impl<T> AddAssign<TimeSpans> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
        Self: Add<TimeSpans, Output=Self>
{
    #[inline]
    fn add_assign(&mut self, other: TimeSpans) {
        *self = self.clone() + other
    }
}

impl<T> SubAssign<TimeSpans> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
        Self: Sub<TimeSpans, Output=Self>
{
    #[inline]
    fn sub_assign(&mut self, other: TimeSpans) {
        *self = self.clone() - other
    }
}


