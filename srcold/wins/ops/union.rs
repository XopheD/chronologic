use crate::*;

/// Make union in place
pub trait TimeUnion<Rhs>
    where
        Self: TimeWindow
{
    type Output;
    fn union(self, rhs: Rhs) -> Self::Output;
}



impl<TW1,TW2> TimeUnion<TW1> for TW2
    where
        TW1: TimeConvex,
        TW2: TimeConvex<TimePoint=TW1::TimePoint>,
        TW1: Into<TimeInterval<TW1::TimePoint>>,
        TW2: Into<TimeInterval<TW1::TimePoint>>
{
    type Output = TimeSet<TW1::TimePoint>;

    fn union(self, rhs: TW1) -> Self::Output
    {
        if self.is_empty() {
            if rhs.is_empty() {
                TimeSet::empty()
            } else {
                TimeSet(vec![rhs.into()])
            }
        } else if rhs.is_empty() {
            TimeSet(vec![self.into()])
        } else if self.upper_bound() < rhs.lower_bound().just_before() {
            TimeSet(vec![self.into(), rhs.into()])
        } else if rhs.upper_bound() < self.lower_bound().just_before() {
            TimeSet(vec![rhs.into(), self.into()])
        } else {
            TimeSet(vec![ TimeInterval {
                lower: self.lower_bound().min(rhs.lower_bound()),
                upper: self.upper_bound().max(rhs.upper_bound())
            }])
        }
    }
}


impl<T:TimePoint,TW> TimeUnion<TW> for TimeSet<T>
    where
        TW: TimeConvex<TimePoint=T>+Into<TimeInterval<T>>,
        Self: TimeComplementary<Output=Self>+TimeIntersection<Self,Output=Self>+TimeBounds<TimePoint=T>
{
    type Output = Self;

    fn union(mut self, rhs: TW) -> Self::Output
    {
        if rhs.lower_bound() > self.upper_bound() {
            self.0.push(rhs.into());
            self
        } else {
            // quick and dirty... (todo)
            self.complementary()
                .intersection(rhs.complementary())
                .complementary()
        }
    }
}
