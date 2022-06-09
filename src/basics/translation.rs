use std::ops::*;
use crate::*;

//---------------------- TIMEVALUE OUTPUT ----------------------

impl Add for TimeValue
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output
    {
        if self.is_future_infinite() {
            if other.is_past_infinite() {
                panic!("can't add infinite time values +oo + -oo");
            }
            self
        } else if self.is_past_infinite() {
            if other.is_future_infinite() {
                panic!("can't add infinite time values -oo + +oo");
            }
            self
        } else if other.is_finite() {
            Self::from_ticks(self.0.saturating_add(other.0))
        } else {
            other
        }
    }
}

impl AddAssign for TimeValue
{
    #[inline]
    fn add_assign(&mut self, other: TimeValue) { *self = *self + other; }
}

impl Sub for TimeValue {
    type Output = Self;
    #[inline] fn sub(self, v: TimeValue) -> Self { self + (-v) }
}

impl SubAssign for TimeValue {
    #[inline] fn sub_assign(&mut self, v: TimeValue) { *self += -v }
}

impl Sub for Timestamp {
    type Output = TimeValue;
    /// Distance between two timestamps
    #[inline] fn sub(self, other: Self) -> Self::Output { self.0 - other.0 }
}

//---------------------- TIMESTAMP OUTPUT ----------------------

impl Add<TimeValue> for Timestamp
{
    type Output = Self;
    #[inline] fn add(self, other: TimeValue) -> Self::Output { Self(self.0+other) }
}

impl Sub<TimeValue> for Timestamp
{
    type Output = Self;
    #[inline] fn sub(self, other: TimeValue) -> Self::Output { Self(self.0-other) }
}

impl AddAssign<TimeValue> for Timestamp
{
    #[inline]
    fn add_assign(&mut self, other: TimeValue) { self.0 += other; }
}

impl SubAssign<TimeValue> for Timestamp
{
    #[inline]
    fn sub_assign(&mut self, other: TimeValue) { self.0 -= other; }
}

impl Add<Timestamp> for TimeValue {
    type Output = Timestamp;
    #[inline] fn add(self, tw: Self::Output) -> Self::Output { tw + self }
}

impl Sub<Timestamp> for TimeValue {
    type Output = Timestamp;
    #[inline] fn sub(self, tw: Self::Output) -> Self::Output { (-tw) + self }
}


//---------------------- TIMERANGE<T> OUTPUT ------------------------

impl<T> Add<TimeValue> for TimeInterval<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline] fn add(self, other: TimeValue) -> Self::Output {
        TimeInterval::new(self.lower + other, self.upper + other).unwrap()
    }
}

impl<T> Sub<TimeValue> for TimeInterval<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline] fn sub(self, other: TimeValue) -> Self::Output {
        TimeInterval::new(self.lower - other, self.upper - other).unwrap()
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
        TimeInterval::new(self.lower + other.lower, self.upper + other.upper).unwrap()
    }
}

impl<T> Sub<TimeSpan> for TimeInterval<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline] fn sub(self, other: TimeSpan) -> Self::Output {
        TimeInterval::new(self.lower - other.upper, self.upper - other.lower).unwrap()
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
        TimeSlot::new(self + other.lower, self + other.upper).unwrap()
    }
}

impl Sub<TimeSpan> for Timestamp {
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: TimeSpan) -> Self::Output {
        TimeSlot::new(self - other.upper, self - other.lower).unwrap()
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
        TimeInterval::new(
            self.lower - other.upper,
            self.upper - other.lower
        ).unwrap()
    }
}

impl Sub<Timestamp> for TimeSlot {
    type Output = TimeSpan;
    #[inline]
    fn sub(self, other: Timestamp) -> Self::Output {
        TimeInterval::new(
            self.lower - other,
            self.upper - other
        ).unwrap()
    }
}

impl Sub<TimeSlot> for Timestamp {
    type Output = TimeSpan;
    #[inline]
    fn sub(self, other: TimeSlot) -> Self::Output {
        TimeInterval::new(
            self - other.upper,
            self - other.lower
        ).unwrap()
    }
}

//------------------------ TIMESET<T> OUTPUT ------------------------

impl<T:TimePoint> Add<TimeValue> for TimeSet<T>
    where TimeInterval<T>: AddAssign<TimeValue>
{
    type Output = Self;
    #[inline]
    fn add(self, other: TimeValue) -> Self::Output {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl<T:TimePoint> Sub<TimeValue> for TimeSet<T>
    where TimeInterval<T>: SubAssign<TimeValue>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: TimeValue) -> Self::Output {
        let mut result = self.clone();
        result -= other;
        result
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
        self.iter()
            .map(|i| *i + other)
            .collect()
    }
}

impl Sub<Timestamp> for TimeSpans
{
    type Output = TimeSlots;
    #[inline]
    fn sub(self, other: Timestamp) -> Self::Output {
        self.iter()
            .map(|i| *i - other)
            .collect()
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
        other.iter()
            .map(|i| self - *i)
            .collect()
    }
}

impl<T> Add<TimeSpan> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
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
{
    #[inline]
    fn add_assign(&mut self, other: TimeSpan) {
        *self = self.clone() + other
    }
}

impl<T> SubAssign<TimeSpan> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
{
    #[inline]
    fn sub_assign(&mut self, other: TimeSpan) {
        *self = self.clone() - other
    }
}

impl<T> Add<TimeSpans> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
{
    type Output = Self;
    #[inline]
    fn add(self, other: TimeSpans) -> Self::Output
    {
        self.0.into_iter()
            .map(|i| other.iter().copied().map(move |j| (i,j)))
            .flatten()
            .map(|(a,b)| a+b)
            .collect()
    }
}

impl<T> Sub<TimeSpans> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
{
    type Output = Self;
    #[inline]
    fn sub(self, other: TimeSpans) -> Self::Output
    {
        self.0.into_iter()
            .map(|i| other.iter().copied().map(move |j| (i,j)))
            .flatten()
            .map(|(a,b)| a-b)
            .collect()
    }
}

impl<T> AddAssign<TimeSpans> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
{
    #[inline]
    fn add_assign(&mut self, other: TimeSpans) {
        *self = self.clone() + other
    }
}

impl<T> SubAssign<TimeSpans> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
{
    #[inline]
    fn sub_assign(&mut self, other: TimeSpans) {
        *self = self.clone() - other
    }
}


