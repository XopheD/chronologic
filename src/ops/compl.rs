use crate::*;


/// # The complementary of a time set
pub trait TimeComplementary {
    type Output;
    fn complementary(self) -> Self::Output;
}


impl<T:TimePoint> TimeComplementary for TimeSet<T>
{
    type Output = Self;

    fn complementary(self) -> Self::Output
    {
        if let Some(first) = self.0.first() {
            let mut compl = Vec::with_capacity(self.0.len() + 1);
            if !first.lower.just_before().is_past_infinite() {
                compl.push((..first.lower).into());
            }
            let mut previous = first.upper.just_after();
            self.0.iter().skip(1)
                .for_each(|tw| {
                    compl.push(TimeInterval { lower: previous, upper: tw.lower.just_before() });
                    previous = tw.upper.just_after();
                });
            if !previous.is_future_infinite() {
                compl.push((previous..).into() );
            }
            TimeSet(compl)
        } else {
            TimeSet::all()
        }
    }
}


impl<T:TimePoint> TimeComplementary for TimeInterval<T> {

    type Output = TimeSet<T>;

    fn complementary(self) -> Self::Output
    {
        if self.is_empty() {
            TimeSet::all()
        } else {
            let cut1 = self.lower_bound().just_before();
            let cut2 = self.upper_bound().just_after();
            if cut1.is_past_infinite() {
                if cut2.is_future_infinite() {
                    TimeSet::empty()
                } else {
                    TimeSet(vec![TimeInterval { lower: cut2, upper: T::INFINITE }])
                }
            } else {
                if cut2.is_future_infinite() {
                    TimeSet(vec![TimeInterval { lower: -T::INFINITE, upper: cut1 }])
                } else {
                    TimeSet(vec![
                        TimeInterval { lower: -T::INFINITE, upper: cut1 },
                        TimeInterval { lower: cut2, upper: T::INFINITE },
                    ])
                }
            }
        }
    }
}



macro_rules! timepoint {
    ($timepoint:ty) => {
        impl TimeComplementary for $timepoint {
            type Output = TimeSet<$timepoint>;
            fn complementary(self) -> Self::Output {
                let first = TimeInterval::before(self.just_before());
                let second = TimeInterval::after(self.just_after());
                if first.is_empty() {
                    if second.is_empty() {
                        TimeSet(vec![])
                    } else {
                        TimeSet(vec![second])
                    }
                } else {
                    if second.is_empty() {
                        TimeSet(vec![first])
                    } else {
                        TimeSet(vec![first, second])
                    }
                }
            }
        }
    }
}

timepoint!(TimeValue);
timepoint!(Timestamp);