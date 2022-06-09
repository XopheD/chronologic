//! # Time relations
use super::*;

pub trait TimeRelation: TimeWindow {

    /// Checks if a timepoint is inside this time window
    fn contains<I>(&self, item: &I) -> bool
        where Self::TimePoint: PartialOrd<I>, I: PartialOrd<Self::TimePoint>;
}

impl<TW:TimeConvex> TimeRelation for TW
{
    fn contains<I>(&self, item: &I) -> bool where Self::TimePoint: PartialOrd<I>, I: PartialOrd<Self::TimePoint> {
        self.lower_bound() <= *item && *item <= self.upper_bound()
    }
}

impl<T:TimePoint> TimeRelation for TimeSet<T> {

    fn contains<I>(&self, t: &I) -> bool
        where Self::TimePoint: PartialOrd<I>, I: PartialOrd<Self::TimePoint>
    {
        self.0.iter()
            .skip_while(|ts| ts.upper_bound() < *t)
            .next()
            .map(|ts| ts.lower_bound() <= *t)
            .unwrap_or(false)
    }
}


#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use crate::*;


    fn checktw<T:Debug>(check:&str, x:&T) {
        assert_eq!( check, &format!("{:?}", x));
    }

    #[test]
    fn contains()
    {
        let t1 = TimeValue::from_ticks(1);
        let t5 = TimeValue::from_ticks(5);
        let t10 = TimeValue::from_ticks(10);
        let tw10 = TimeSpan::centered(t10, t5).unwrap();
        let tw = !t1 & !t5 & !tw10;
        checktw( "]-oo,0]U[2,4]U[16,+oo[", &tw);

        assert!( tw.contains(&TimeValue::from_ticks(3)));
        assert!( tw.contains(&TimeValue::from_ticks(100)));
        assert!( tw.contains(&TimeValue::from_ticks(-15)));
        assert!(!tw.contains(&TimeValue::from_ticks(10)));
    }

}

