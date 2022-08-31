use crate::*;
use crate::wins::*;


/// Complementary set
pub trait TimeComplementary: TimeBounds {
    type Output;
    fn complementary(&self) -> Self::Output;
}

impl<TW:TimeConvex> TimeComplementary for TW {

    type Output = TimeSet<TW::TimePoint>;

    fn complementary(&self) -> Self::Output
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
                    TimeSet(vec![TimeInterval { lower: cut2, upper: TW::TimePoint::INFINITE }])
                }
            } else {
                if cut2.is_future_infinite() {
                    TimeSet(vec![TimeInterval { lower: -TW::TimePoint::INFINITE, upper: cut1 }])
                } else {
                    TimeSet(vec![
                        TimeInterval { lower: -TW::TimePoint::INFINITE, upper: cut1 },
                        TimeInterval { lower: cut2, upper: TW::TimePoint::INFINITE },
                    ])
                }
            }
        }
    }
}

