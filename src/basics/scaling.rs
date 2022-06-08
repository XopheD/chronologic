use super::*;
use std::ops::{Div, DivAssign, Mul, MulAssign};

fn from_f64(t:f64) -> TimeValue
{
    if t >= INFINITE_TIME_VALUE as f64 {
        TimeValue::INFINITE
    } else if t <= -INFINITE_TIME_VALUE as f64 {
        -TimeValue::INFINITE
    } else {
        unsafe { TimeValue::from_ticks_unchecked(t as i64) }
    }
}

impl Mul<i64> for TimeValue {
    type Output = Self;
    #[inline] fn mul(self, n: i64) -> Self::Output {
        TimeValue::from_ticks(self.0.saturating_mul(n))
    }
}

impl Div<i64> for TimeValue {
    type Output = Self;
    #[inline] fn div(self, n: i64) -> Self::Output {
        TimeValue::from_ticks(self.0.saturating_div(n))
    }
}

impl Mul<usize> for TimeValue {
    type Output = Self;
    #[inline] fn mul(self, n: usize) -> Self::Output {
        if n > i64::MAX as usize {
            if self.is_strictly_positive() {
                TimeValue::INFINITE
            } else if self.is_strictly_negative() {
                - TimeValue::INFINITE
            } else {
                TimeValue::default()
            }
        } else {
            TimeValue::from_ticks(self.0.saturating_mul(n as i64))
        }
    }
}

impl Div<usize> for TimeValue {
    type Output = Self;
    #[inline] fn div(self, n: usize) -> Self::Output {
        if n > i64::MAX as usize {
            TimeValue::default()
        } else {
            TimeValue::from_ticks(self.0.saturating_mul(n as i64))
        }
    }
}


impl Mul<f64> for TimeValue {
    type Output = Self;
    #[inline] fn mul(self, factor: f64) -> Self::Output {
        from_f64((self.0 as f64) * factor)
    }
}

impl Div<f64> for TimeValue {
    type Output = Self;
    #[inline] fn div(self, factor: f64) -> Self::Output {
        from_f64((self.0 as f64) / factor)
    }
}

impl MulAssign<i64> for TimeValue {
    #[inline] fn mul_assign(&mut self, n: i64) { *self = *self * n }
}

impl DivAssign<i64> for TimeValue {
    #[inline] fn div_assign(&mut self, n: i64) { *self = *self / n }
}

impl MulAssign<usize> for TimeValue {
    #[inline] fn mul_assign(&mut self, n: usize) { *self = *self * n }
}

impl DivAssign<usize> for TimeValue {
    #[inline] fn div_assign(&mut self, n: usize) { *self = *self / n }
}

impl MulAssign<f64> for TimeValue {
    #[inline] fn mul_assign(&mut self, n: f64) { *self = *self * n }
}

impl DivAssign<f64> for TimeValue {
    #[inline] fn div_assign(&mut self, n: f64) { *self = *self / n }
}

impl Mul<TimeValue> for i64 {
    type Output = TimeValue;
    #[inline] fn mul(self, t: TimeValue) -> Self::Output { t * self }
}

impl Mul<TimeValue> for usize {
    type Output = TimeValue;
    #[inline] fn mul(self, t: TimeValue) -> Self::Output { t * self }
}

impl Mul<TimeValue> for f64 {
    type Output = TimeValue;
    #[inline] fn mul(self, t: TimeValue) -> Self::Output { t * self }
}

////////////////////////////////////////////////////////////////////////////////////////////


impl Mul<i64> for TimeInterval {
    type Output = TimeInterval;

    #[inline]
    fn mul(self, n: i64) -> Self::Output {
        let (lower, upper) = if n < 0 {
            // negative factor -> reverse interval
            (self.upper * n, self.lower * n)
        } else {
            (self.lower * n, self.upper * n)
        };
        assert! (!lower.is_future_infinite(), "future overflow");
        assert! (!upper.is_past_infinite(), "past overflow");
        TimeInterval { lower, upper }
    }
}

impl Div<i64> for TimeInterval {
    type Output = TimeInterval;

    #[inline]
    fn div(self, n: i64) -> Self::Output {
        let (lower, upper) = if n < 0 {
            // negative factor -> reverse interval
            (self.upper / n, self.lower / n)
        } else {
            (self.lower / n, self.upper / n)
        };
        TimeInterval { lower, upper }
    }
}


impl Mul<usize> for TimeInterval {
    type Output = TimeInterval;

    #[inline]
    fn mul(self, n: usize) -> Self::Output {
        let lower = self.lower * n;
        let upper = self.upper * n;
        assert! (!lower.is_future_infinite(), "future overflow");
        assert! (!upper.is_past_infinite(), "past overflow");
        TimeInterval { lower, upper }
    }
}

impl Div<usize> for TimeInterval {
    type Output = TimeInterval;

    #[inline]
    fn div(self, n: usize) -> Self::Output {
        TimeInterval { lower: self.lower/n, upper: self.upper/n }
    }
}

impl Mul<f64> for TimeInterval {
    type Output = TimeInterval;

    #[inline]
    fn mul(self, n: f64) -> Self::Output {
        let (lower, upper) = if n < 0. {
            // negative factor -> reverse interval
            (self.upper * n, self.lower * n)
        } else {
            (self.lower * n, self.upper * n)
        };
        assert! (!lower.is_future_infinite(), "future overflow");
        assert! (!upper.is_past_infinite(), "past overflow");
        TimeInterval { lower, upper }
    }
}

impl Div<f64> for TimeInterval {
    type Output = TimeInterval;

    #[inline]
    fn div(self, n: f64) -> Self::Output {
        let (lower, upper) = if n < 0. {
            // negative factor -> reverse interval
            (self.upper / n, self.lower / n)
        } else {
            (self.lower / n, self.upper / n)
        };
        assert! (!lower.is_future_infinite(), "future overflow");
        assert! (!upper.is_past_infinite(), "past overflow");
        TimeInterval { lower, upper }
    }
}

impl MulAssign<i64> for TimeInterval {
    #[inline] fn mul_assign(&mut self, n: i64) { *self = *self * n }
}

impl DivAssign<i64> for TimeInterval {
    #[inline] fn div_assign(&mut self, n: i64) { *self = *self / n }
}

impl MulAssign<usize> for TimeInterval {
    #[inline] fn mul_assign(&mut self, n: usize) { *self = *self * n }
}

impl DivAssign<usize> for TimeInterval {
    #[inline] fn div_assign(&mut self, n: usize) { *self = *self / n }
}

impl MulAssign<f64> for TimeInterval {
    #[inline] fn mul_assign(&mut self, n: f64) { *self = *self * n }
}

impl DivAssign<f64> for TimeInterval {
    #[inline] fn div_assign(&mut self, n: f64) { *self = *self / n }
}

impl Mul<TimeInterval> for i64 {
    type Output = TimeInterval;
    #[inline] fn mul(self, t: TimeInterval) -> Self::Output { t * self }
}

impl Mul<TimeInterval> for f64 {
    type Output = TimeInterval;
    #[inline] fn mul(self, t: TimeInterval) -> Self::Output { t * self }
}

////////////////////////////////////////////////////////////////////////////////////

fn check_joined_inners(inners: &mut Vec<TimeInterval>)
{
    let mut i = 1;
    while i < inners.len() {
        let previous = unsafe { inners.get_unchecked(i-1) };
        let current = unsafe { inners.get_unchecked(i) };
        if previous.upper_bound() >= current.lower_bound().just_before() {
            unsafe { inners.get_unchecked_mut(i-1) }.upper = current.upper_bound();
            inners.remove(i);
        } else {
            i += 1;
        }
    }
}

impl Mul<i64> for TimeWindow
{
    type Output = TimeWindow;
    #[inline]
    fn mul(self, factor: i64) -> Self::Output
    {
        if factor == 0 {
            if self.is_empty() {
                TimeWindow::empty()
            } else {
                TimeWindow::singleton(TimeValue::default())
            }
        } else {
            // since factor is non null integer, the scale increases distances between
            // successive intervals so they remain disjoints
            let mut inners = self.0.iter()
                .map(|tw| *tw * factor)
                .filter(|tw| !tw.is_empty())
                .collect::<Vec<_>>();
            if factor < 0 { inners.reverse() }
            Self(inners)
        }
    }
}

impl Div<i64> for TimeWindow
{
    type Output = TimeWindow;
    #[inline]
    fn div(self, factor: i64) -> Self::Output {
        let mut copy = self.clone();
        copy /= factor; copy
    }
}

impl Mul<usize> for TimeWindow
{
    type Output = TimeWindow;
    #[inline]
    fn mul(self, factor: usize) -> Self::Output
    {
        if factor == 0 {
            if self.is_empty() {
                TimeWindow::empty()
            } else {
                TimeWindow::singleton(TimeValue::default())
            }
        } else {
            // since factor is non null integer, the scale increases distances between
            // successive intervals so they remain disjoints
            let mut inners = self.0.iter()
                .map(|tw| *tw * factor)
                .filter(|tw| !tw.is_empty())
                .collect::<Vec<_>>();
            if factor < 0 { inners.reverse() }
            Self(inners)
        }
    }
}

impl Div<usize> for TimeWindow
{
    type Output = TimeWindow;
    #[inline]
    fn div(self, factor: usize) -> Self::Output {
        let mut copy = self.clone();
        copy /= factor; copy
    }
}

impl Mul<f64> for TimeWindow
{
    type Output = TimeWindow;
    #[inline]
    fn mul(self, factor: f64) -> Self::Output {
        let mut inners = self.0.iter()
            .map(|tw| *tw * factor)
            .collect::<Vec<_>>();
        if factor < 0. { inners.reverse() }
        check_joined_inners(&mut inners);
        Self(inners)
    }
}

impl Div<f64> for TimeWindow
{
    type Output = TimeWindow;
    #[inline]
    fn div(self, factor: f64) -> Self::Output {
        let mut copy = self.clone();
        copy /= factor; copy
    }
}

impl MulAssign<i64> for TimeWindow
{
    fn mul_assign(&mut self, factor: i64)
    {
        if factor == 0 {
            if !self.is_empty() {
                self.0.clear();
                self.0.push(TimeInterval {
                    lower: TimeValue::default(), // 0
                    upper: TimeValue::default()  // 0
                });
            }
        } else {
            self.0.iter_mut().for_each(|tw| *tw *= factor);
            if factor < 0 { self.0.reverse() }
            // NOTE: since factor is non null integer, the scale increases distances between
            // successive intervals so they remain disjoints (don’t need check_joined_inners)
        }
    }
}

impl DivAssign<i64> for TimeWindow
{
    fn div_assign(&mut self, factor: i64)
    {
        assert_ne!(factor, 0, "div. by zero");
        self.0.iter_mut().for_each(|tw| *tw /= factor);
        if factor < 0 { self.0.reverse() }
        check_joined_inners(&mut self.0)
    }
}


impl MulAssign<usize> for TimeWindow
{
    fn mul_assign(&mut self, factor: usize)
    {
        if factor == 0 {
            if !self.is_empty() {
                self.0.clear();
                self.0.push(TimeInterval {
                    lower: TimeValue::default(), // 0
                    upper: TimeValue::default()  // 0
                });
            }
        } else {
            self.0.iter_mut().for_each(|tw| *tw *= factor);
            if factor < 0 { self.0.reverse() }
            // NOTE: since factor is non null integer, the scale increases distances between
            // successive intervals so they remain disjoints (don’t need check_joined_inners)
        }
    }
}

impl DivAssign<usize> for TimeWindow
{
    fn div_assign(&mut self, factor: usize)
    {
        assert_ne!(factor, 0, "div. by zero");
        self.0.iter_mut().for_each(|tw| *tw /= factor);
        if factor < 0 { self.0.reverse() }
        check_joined_inners(&mut self.0)
    }
}

impl MulAssign<f64> for TimeWindow
{
    #[inline]
    fn mul_assign(&mut self, factor: f64) {
        self.0.iter_mut().for_each(|tw| *tw *= factor);
        if factor < 0. { self.0.reverse() }
        if factor.abs() < 1. { check_joined_inners(&mut self.0) }
    }
}

impl DivAssign<f64> for TimeWindow {
    #[inline] fn div_assign(&mut self, factor: f64) { *self *= 1./factor }
}

impl Mul<TimeWindow> for i64 {
    type Output = TimeWindow;
    #[inline] fn mul(self, t: TimeWindow) -> Self::Output { t * self }
}

impl Mul<TimeWindow> for f64 {
    type Output = TimeWindow;
    #[inline] fn mul(self, t: TimeWindow) -> Self::Output { t * self }
}
