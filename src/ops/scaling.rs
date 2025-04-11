use crate::*;
use std::ops::{Div, DivAssign, Mul, MulAssign};

//-------------- TIMEVALUE SCALING -----------------------------

// GROUP 1: scale factor is always less than i64::MAX (abs value)
macro_rules! timevalscalingsmall {
    ($scale: ty) => {
        impl Mul<$scale> for TimeValue {
            type Output = Self;
            #[inline] fn mul(self, n: $scale) -> Self::Output {
                TimeValue::from_ticks(self.0.saturating_mul(n as i64))
            }
        }
        impl Div<$scale> for TimeValue {
            type Output = Self;
            #[inline] fn div(self, n: $scale) -> Self::Output {
                TimeValue::from_ticks(self.0.saturating_div(n as i64))
            }
        }
    };
}
timevalscalingsmall!(i8);
timevalscalingsmall!(u8);
timevalscalingsmall!(i16);
timevalscalingsmall!(u16);
timevalscalingsmall!(i32);
timevalscalingsmall!(u32);
timevalscalingsmall!(i64);

// GROUP 2: scale factor is signed and could be greater than i64::MAX (abs value)

macro_rules! timevalscalingbig {
    ($time: ty) => {
        impl Mul<$time> for TimeValue {
            type Output = Self;
            #[inline] fn mul(self, n: $time) -> Self::Output {
                if n > INFINITE_TIME_VALUE as $time {
                    TimeValue::INFINITE
                } else if n < -INFINITE_TIME_VALUE as $time {
                    -TimeValue::INFINITE
                } else {
                    TimeValue::from_ticks(self.0.saturating_mul(n as i64))
                }
            }
        }
        impl Div<$time> for TimeValue {
            type Output = Self;
            #[inline] fn div(self, n: $time) -> Self::Output {
                if n > INFINITE_TIME_VALUE as $time || n < -INFINITE_TIME_VALUE as $time {
                    TimeValue::default() // zero
                } else {
                    TimeValue::from_ticks(self.0.saturating_div(n as i64))
                }
            }
        }
    };
}

timevalscalingbig!(i128);
timevalscalingbig!(isize);

// GROUP 3: scale factor is unsigned and could be greater than i64::MAX (abs value)

macro_rules! timevalscalingubig {
    ($time: ty) => {
        impl Mul<$time> for TimeValue {
            type Output = Self;
            #[inline] fn mul(self, n: $time) -> Self::Output {
                if n > INFINITE_TIME_VALUE as $time {
                    TimeValue::INFINITE
                } else {
                    TimeValue::from_ticks(self.0.saturating_mul(n as i64))
                }
            }
        }
        impl Div<$time> for TimeValue {
            type Output = Self;
            #[inline] fn div(self, n: $time) -> Self::Output {
                if n > INFINITE_TIME_VALUE as $time {
                    TimeValue::default() // zero
                } else {
                    TimeValue::from_ticks(self.0.saturating_div(n as i64))
                }
            }
        }
    };
}
timevalscalingubig!(u64);
timevalscalingubig!(u128);
timevalscalingubig!(usize);


// GROUP 4: scale factor is a float (could be less than 1.0)

macro_rules! timevalscalingfloat {
    ($time: ty) => {
        impl Mul<$time> for TimeValue {
            type Output = Self;
            #[inline]
            fn mul(self, n: $time) -> Self::Output {
                let t = self.0 as $time * n;
                if t > INFINITE_TIME_VALUE as $time {
                    TimeValue::INFINITE
                } else if t < -INFINITE_TIME_VALUE as $time {
                    -TimeValue::INFINITE
                } else {
                    TimeValue::from_ticks(t as i64)
                }
            }
        }
        impl Div<$time> for TimeValue {
            type Output = Self;
            #[inline]
            fn div(self, n: $time) -> Self::Output {
                self.mul(1. / n)
            }
        }
    };
}

timevalscalingfloat!(f32);
timevalscalingfloat!(f64);

macro_rules! timescalingassign {
    ($time:ty, $scale:ty) => {
        impl MulAssign<$scale> for $time {
            #[inline] fn mul_assign(&mut self, n: $scale) { *self = *self * n }
        }
        impl DivAssign<$scale> for $time {
            #[inline] fn div_assign(&mut self, n: $scale) { *self = *self / n }
        }
    };
}
timescalingassign!(TimeValue,u8);
timescalingassign!(TimeValue,i8);
timescalingassign!(TimeValue,u16);
timescalingassign!(TimeValue,i16);
timescalingassign!(TimeValue,u32);
timescalingassign!(TimeValue,i32);
timescalingassign!(TimeValue,u64);
timescalingassign!(TimeValue,i64);
timescalingassign!(TimeValue,u128);
timescalingassign!(TimeValue,i128);
timescalingassign!(TimeValue,usize);
timescalingassign!(TimeValue,isize);
timescalingassign!(TimeValue,f32);
timescalingassign!(TimeValue,f64);

////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! timespanscalingint {
    ($scale:ty) => {
        impl Mul<$scale> for TimeSpan {
            type Output = TimeSpan;
            #[inline]
            #[allow(unused_comparisons)]
            fn mul(self, n: $scale) -> Self::Output {
                let (lower, upper) = if n < 0 {
                    // negative factor -> reverse interval
                    (self.upper * n, self.lower * n)
                } else {
                    (self.lower * n, self.upper * n)
                };
                assert! (!lower.is_future_infinite(), "future overflow");
                assert! (!upper.is_past_infinite(), "past overflow");
                TimeSpan { lower, upper }
            }
        }
        impl Div<$scale> for TimeSpan {
            type Output = TimeSpan;
            #[inline]
            #[allow(unused_comparisons)]
            fn div(self, n: $scale) -> Self::Output {
                let (lower, upper) = if n < 0 {
                    // negative factor -> reverse interval
                    (self.upper / n, self.lower / n)
                } else {
                    (self.lower / n, self.upper / n)
                };
                TimeSpan { lower, upper }
            }
        }

    };
}
timespanscalingint!(i8);
timespanscalingint!(u8);
timespanscalingint!(i16);
timespanscalingint!(u16);
timespanscalingint!(i32);
timespanscalingint!(u32);
timespanscalingint!(i64);
timespanscalingint!(u64);
timespanscalingint!(i128);
timespanscalingint!(u128);
timespanscalingint!(isize);
timespanscalingint!(usize);


macro_rules! timespanscalingfloat {
    ($scale:ty) => {
    impl Mul<$scale> for TimeSpan {
            type Output = TimeSpan;
            #[inline]
            fn mul(self, f: $scale) -> Self::Output {
                let (lower, upper) = if f < 0. {
                    // negative factor -> reverse interval
                    (self.upper * f, self.lower * f)
                } else {
                    (self.lower * f, self.upper * f)
                };
                assert! (!lower.is_future_infinite(), "future overflow");
                assert! (!upper.is_past_infinite(), "past overflow");
                TimeSpan { lower, upper }
            }
        }
        impl Div<$scale> for TimeSpan {
            type Output = TimeSpan;
            #[inline]
            fn div(self, f: $scale) -> Self::Output {
                let (lower, upper) = if f < 0. {
                    // negative factor -> reverse interval
                    (self.upper / f, self.lower / f)
                } else {
                    (self.lower / f, self.upper / f)
                };
                assert! (!lower.is_future_infinite(), "future overflow");
                assert! (!upper.is_past_infinite(), "past overflow");
                TimeSpan { lower, upper }
            }
        }
    }
}
timespanscalingfloat!(f32);
timespanscalingfloat!(f64);

timescalingassign!(TimeSpan,u8);
timescalingassign!(TimeSpan,i8);
timescalingassign!(TimeSpan,u16);
timescalingassign!(TimeSpan,i16);
timescalingassign!(TimeSpan,u32);
timescalingassign!(TimeSpan,i32);
timescalingassign!(TimeSpan,u64);
timescalingassign!(TimeSpan,i64);
timescalingassign!(TimeSpan,u128);
timescalingassign!(TimeSpan,i128);
timescalingassign!(TimeSpan,usize);
timescalingassign!(TimeSpan,isize);
timescalingassign!(TimeSpan,f32);
timescalingassign!(TimeSpan,f64);

////////////////////////////////////////////////////////////////////////////////////

macro_rules! timespansscaleint {
    ($scale:ty) => {
        impl Mul<$scale> for TimeSpans {
            type Output = TimeSpans;
            #[inline]
            fn mul(self, n: $scale) -> Self::Output {
                if n == 0 {
                    if self.is_empty() {
                        TimeSpans::empty()
                    } else {
                        Self(vec![ TimeSpan::default() ])
                    }
                } else {
                    // since factor is non null integer, the scale increases distances between
                    // successive intervals so they remain disjoints
                    let mut inners = self.0.iter()
                        .map(|tw| *tw * n)
                        .filter(|tw| !tw.is_empty())
                        .collect::<Vec<_>>();
                    #[allow(unused_comparisons)] if n < 0 { inners.reverse() }
                    Self(inners)
                }
            }
        }
        impl Div<$scale> for TimeSpans {
            type Output = TimeSpans;
            #[inline]
            fn div(self, n: $scale) -> Self::Output {
                let mut copy = self.clone();
                copy /= n; copy
            }
        }
        impl MulAssign<$scale> for TimeSpans {
            fn mul_assign(&mut self, n: $scale) {
                if n == 0 {
                    if !self.is_empty() {
                        self.0.clear();
                        self.0.push(TimeSpan::default());
                    }
                } else {
                    self.0.iter_mut().for_each(|tw| *tw *= n);
                    #[allow(unused_comparisons)] if n < 0 { self.0.reverse() }
                    // NOTE: since factor is non null integer, the scale increases distances between
                    // successive intervals so they remain disjoints (don’t need check_joined_inners)
                }
            }
        }
        impl DivAssign<$scale> for TimeSpans {
            fn div_assign(&mut self, n: $scale) {
                assert_ne!(n, 0, "div. by zero");
                self.0.iter_mut().for_each(|tw| *tw /= n);
                #[allow(unused_comparisons)] if n < 0 { self.0.reverse() }
                check_joined_inners(&mut self.0)
            }
        }

    };
}

timespansscaleint!(u8);
timespansscaleint!(i8);
timespansscaleint!(u16);
timespansscaleint!(i16);
timespansscaleint!(u32);
timespansscaleint!(i32);
timespansscaleint!(u64);
timespansscaleint!(i64);
timespansscaleint!(u128);
timespansscaleint!(i128);
timespansscaleint!(usize);
timespansscaleint!(isize);

macro_rules! timespansscalefloat {
    ($scale:ty) => {
        impl Mul<$scale> for TimeSpans {
            type Output = TimeSpans;
            #[inline]
            fn mul(self, f: $scale) -> Self::Output {
                let mut inners = self.0.iter()
                    .map(|tw| *tw * f)
                    .collect::<Vec<_>>();
                if f < 0. { inners.reverse() }
                if f.abs() > 1. { check_joined_inners(&mut inners); }
                Self(inners)
            }
        }
        impl Div<$scale> for TimeSpans {
            type Output = TimeSpans;
            #[inline] fn div(self, f: $scale) -> Self::Output { self * (1./f) }
        }
        impl MulAssign<$scale> for TimeSpans {
            #[inline]
            fn mul_assign(&mut self, f: $scale) {
                self.0.iter_mut().for_each(|tw| *tw *= f);
                if f < 0. { self.0.reverse() }
                if f.abs() < 1. { check_joined_inners(&mut self.0) }
            }
        }
        impl DivAssign<$scale> for TimeSpans {
            #[inline] fn div_assign(&mut self, f: $scale) { *self *= 1./f }
        }
    };
}

timespansscalefloat!(f32);
timespansscalefloat!(f64);

fn check_joined_inners(inners: &mut Vec<TimeSpan>)
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



macro_rules! timerevmul {
    ($time:ty, $scale:ty) => {
        impl Mul<$time> for $scale {
            type Output = $time;
            #[inline] fn mul(self, t: $time) -> Self::Output { t * self }
        }
    };
}
macro_rules! timerevmulall {
    ($scale:ty) => {
        timerevmul!(TimeValue,$scale);
        timerevmul!(TimeSpan,$scale);
        timerevmul!(TimeSpans,$scale);
    };
}

timerevmulall!(u8);
timerevmulall!(i8);
timerevmulall!(u16);
timerevmulall!(i16);
timerevmulall!(u32);
timerevmulall!(i32);
timerevmulall!(u64);
timerevmulall!(i64);
timerevmulall!(u128);
timerevmulall!(i128);
timerevmulall!(usize);
timerevmulall!(isize);
timerevmulall!(f32);
timerevmulall!(f64);
