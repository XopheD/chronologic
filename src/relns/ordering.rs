use std::cmp::Ordering;
use crate::{TimeBounds, TimeInterval, TimePoint, TimeSet, TimeWindow};
use crate::iter::TimeConvexIterator;

impl<T:TimePoint,TW> PartialEq<TW> for TimeInterval<T>
    where
        TW: TimeWindow<TimePoint=T>
{
    #[inline]
    fn eq(&self, other: &TW) -> bool {
        other.is_convex()
            && self.lower == other.lower_bound()
            && self.upper == other.upper_bound()
    }
}

impl<T:TimePoint,TW> PartialEq<TW> for TimeSet<T>
    where
        TW: TimeWindow<TimePoint=T>
{
    #[inline]
    fn eq(&self, other: &TW) -> bool {
        self.iter().eq(other.iter())
    }
}

macro_rules! timepartialcmp {
    ($time:ident) => {
        impl<T:TimePoint,TW> PartialOrd<TW> for $time<T>
            where
                TW: TimeWindow<TimePoint=T>
        {
            #[inline]
            fn partial_cmp(&self, other: &TW) -> Option<Ordering>
            {
                if self.lt(other){
                    Some(Ordering::Less)
                } else if self.gt(other) {
                    Some(Ordering::Greater)
                } else if self.eq(other) {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }

            #[inline]
            fn lt(&self, other: &TW) -> bool {
                self.upper_bound() < other.lower_bound()
            }

            #[inline]
            fn le(&self, other: &TW) -> bool {
                self.lt(other) || self.eq(other)
            }

            #[inline]
            fn gt(&self, other: &TW) -> bool {
                self.lower_bound() > other.upper_bound()
            }

            #[inline]
            fn ge(&self, other: &TW) -> bool {
                self.gt(other) || self.eq(other)
            }
        }
    }
}

timepartialcmp!(TimeInterval);
timepartialcmp!(TimeSet);
