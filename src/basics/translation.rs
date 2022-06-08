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

impl<T> Add<TimeValue> for TimeRange<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline] fn add(self, other: TimeValue) -> Self::Output {
        TimeRange::new(self.lower + other, self.upper + other).unwrap()
    }
}

impl<T> Sub<TimeValue> for TimeRange<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline] fn sub(self, other: TimeValue) -> Self::Output {
        TimeRange::new(self.lower - other, self.upper - other).unwrap()
    }
}

impl<T> AddAssign<TimeValue> for TimeRange<T>
    where T:TimePoint+AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, other: TimeValue) {
        self.lower += other;
        self.upper += other;
        assert!( (self.lower != self.upper) || !self.lower.is_finite() );
    }
}

impl<T> SubAssign<TimeValue> for TimeRange<T>
    where T:TimePoint+SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, other: TimeValue) {
        self.lower -= other;
        self.upper -= other;
        assert!( (self.lower != self.upper) || !self.lower.is_finite() );
    }
}

impl<TW> Add<TW> for TimeValue
    where TW: Into<TimeInterval>
{
    type Output = TimeInterval;
    #[inline] fn add(self, other: TW) -> Self::Output { other.into() + self}
}

impl<T> Sub<TimeRange<T>> for TimeValue
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = TimeRange<T>;
    #[inline] fn sub(self, other: TimeRange<T>) -> Self::Output { (-other) + self }
}


impl<T> Add<TimeInterval> for TimeRange<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline] fn add(self, other: TimeInterval) -> Self::Output {
        TimeRange::new(self.lower + other.lower, self.upper + other.upper).unwrap()
    }
}

impl<T> Sub<TimeInterval> for TimeRange<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline] fn sub(self, other: TimeInterval) -> Self::Output {
        TimeRange::new(self.lower - other.upper, self.upper - other.lower).unwrap()
    }
}

impl<T> AddAssign<TimeInterval> for TimeRange<T>
    where T:TimePoint+AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, other: TimeInterval) {
        self.lower += other.lower;
        self.upper += other.upper;
        assert!( (self.lower != self.upper) || !self.lower.is_finite() );
    }
}

impl<T> SubAssign<TimeInterval> for TimeRange<T>
    where T:TimePoint+SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, other: TimeInterval) {
        self.lower -= other.upper;
        self.upper -= other.lower;
        assert!( (self.lower != self.upper) || !self.lower.is_finite() );
    }
}

impl Add<TimeInterval> for Timestamp {
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: TimeInterval) -> Self::Output {
        TimeSlot::new(self + other.lower, self + other.upper).unwrap()
    }
}

impl Sub<TimeInterval> for Timestamp {
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: TimeInterval) -> Self::Output {
        TimeSlot::new(self - other.upper, self - other.lower).unwrap()
    }
}

impl Sub for TimeSlot {
    type Output = TimeInterval;
    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        TimeRange::new(
            self.lower - other.upper,
            self.upper - other.lower
        ).unwrap()
    }
}

impl Sub<Timestamp> for TimeSlot {
    type Output = TimeInterval;
    #[inline]
    fn sub(self, other: Timestamp) -> Self::Output {
        TimeRange::new(
            self.lower - other,
            self.upper - other
        ).unwrap()
    }
}

impl Sub<TimeSlot> for Timestamp {
    type Output = TimeInterval;
    #[inline]
    fn sub(self, other: TimeSlot) -> Self::Output {
        TimeRange::new(
            self - other.upper,
            self - other.lower
        ).unwrap()
    }
}

//------------------------ TIMESET<T> OUTPUT ------------------------

impl<T:TimePoint> Add<TimeValue> for TimeSet<T>
    where TimeRange<T>: AddAssign<TimeValue>
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
    where TimeRange<T>: SubAssign<TimeValue>
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
    where TimeRange<T>: AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, t: TimeValue) {
        // adding a constant preserves the structure (order and distance
        // between successive intervals -> no new overlapping to manage)
        self.0.iter_mut().for_each(|tw| *tw += t)
    }
}

impl<T:TimePoint> SubAssign<TimeValue> for TimeSet<T>
    where TimeRange<T>: SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, t: TimeValue) {
        // adding a constant preserves the structure (order and distance
        // between successive intervals -> no new overlapping to manage)
        self.0.iter_mut().for_each(|tw| *tw -= t)
    }
}

impl<T> Add<TimeInterval> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
{
    type Output = Self;
    #[inline]
    fn add(self, other: TimeInterval) -> Self::Output {
        self.0.into_iter()
            .fold(Self::empty(), |r,i| r|(i+other))
    }
}

impl<T> Sub<TimeInterval> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
{
    type Output = Self;
    #[inline]
    fn sub(self, other: TimeInterval) -> Self::Output
    {
        self.0.into_iter()
            .fold(Self::empty(), |r,i| r|(i-other))
    }
}

impl<T> AddAssign<TimeInterval> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
{
    #[inline]
    fn add_assign(&mut self, other: TimeInterval) {
        *self = self.clone() + other
    }
}

impl<T> SubAssign<TimeInterval> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
{
    #[inline]
    fn sub_assign(&mut self, other: TimeInterval) {
        *self = self.clone() - other
    }
}

impl<T> Add<TimeWindow> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
{
    type Output = Self;
    #[inline]
    fn add(self, other: TimeWindow) -> Self::Output
    {
        self.0.into_iter()
            .map(|i| other.iter().copied().map(move |j| (i,j)))
            .flatten()
            .map(|(a,b)| a+b)
            .collect()
    }
}

impl<T> Sub<TimeWindow> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
{
    type Output = Self;
    #[inline]
    fn sub(self, other: TimeWindow) -> Self::Output
    {
        self.0.into_iter()
            .map(|i| other.iter().copied().map(move |j| (i,j)))
            .flatten()
            .map(|(a,b)| a-b)
            .collect()
    }
}

impl<T> AddAssign<TimeWindow> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
{
    #[inline]
    fn add_assign(&mut self, other: TimeWindow) {
        *self = self.clone() + other
    }
}

impl<T> SubAssign<TimeWindow> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
{
    #[inline]
    fn sub_assign(&mut self, other: TimeWindow) {
        *self = self.clone() - other
    }
}


