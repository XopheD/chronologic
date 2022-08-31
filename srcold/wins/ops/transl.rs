use crate::*;
use crate::wins::*;

/// # A trait for time translation of time window
pub trait TimeTranslation<TW> : TimeWindow
    where
        TW: TimeWindow<TimePoint=TimeValue>
{

    type Output : TimeWindow<TimePoint=Self::TimePoint>;
    /// Translate the time window
    ///
    /// If the translation is not possible (e.g. +oo + -oo),
    /// an error is returned.
    fn translate(self, t: &TW) -> TimeResult<Self::Output>;
}

impl TimeTranslation<TimeValue> for TimeValue
{
    type Output = Self;

    fn translate(self, other: &TimeValue) -> Self
    {
        if self.is_future_infinite() {
            if other.is_past_infinite() {
                Err(TimeError::UndefinedValue)
            } else {
                Ok(self)
            }
        } else if self.is_past_infinite() {
            if other.is_future_infinite() {
                Err(TimeError::UndefinedValue)
            } else {
                Ok(self)
            }
        } else if other.is_finite() {
            Ok(Self::from_ticks(self.0.saturating_add(other.0)))
        } else {
            Ok(*other)
        }
    }
}

impl TimeTranslation<TimeValue> for Timestamp
{
    type Output = Self;
    #[inline]
    fn translate(self, t: &TimeValue) -> TimeResult<Self> {
        self.0.translate(t).map(|t| Self(t))
    }
}

impl<T,TW> TimeTranslation<TW> for TimeInterval<T>
    where
        T: TimePoint+TimeTranslation<TimeValue,Output=T>,
        TW: TimeConvex<TimePoint=TimeValue>
{
    type Output = Self;

    #[inline]
    fn translate(mut self, tw: &TW) -> TimeResult<Self>
    {
        if tw.is_empty() {
            Err(TimeError::EmptyInterval)
        } else {
            self.lower = self.lower.translate(&tw.lower_bound())?;
            self.upper = self.upper.translate(&tw.upper_bound())?;
            Ok(self)
        }
    }
}


impl<T:TimePoint> TimeTranslation<TimeValue> for TimeSet<T>
    where
        TimeInterval<T>:TimeTranslation<TimeValue,Output=TimeInterval<T>>
{
    type Output = Self;

    fn translate(mut self, t: &TimeValue) -> TimeResult<Self>
    {
        // adding a constant preserves the structure (order and distance
        // between successive intervals -> nothing to manage)
        // except if infinite values are reached
        self.0.iter_mut()
            .try_for_each(|i| Ok( *i = i.translate(t)? ))?;

        // suppress repeated {+oo,+oo} at the end
        while let Some(end) = self.0.get(self.0.len()-2) {
            if end.is_up_bounded() { break; }
            self.0.pop();
        }

        // suppress repeated {-oo,-oo} at the beginning
        while let Some(begin) = self.0.get(1) {
            if begin.is_low_bounded() { break; }
            self.0.remove(0);
        }

        Ok(self)
    }
}
