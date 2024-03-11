use crate::{SUBSEC_BITMASK, TimeValue};

pub trait IntoTimeValue
{
    fn years(self) -> TimeValue;
    fn months(self) -> TimeValue;
    fn weeks(self) -> TimeValue;
    fn days(self) -> TimeValue;
    fn hours(self) -> TimeValue;
    fn mins(self) -> TimeValue;
    fn secs(self) -> TimeValue;
    fn millis(self) -> TimeValue;
    fn micros(self) -> TimeValue;
    fn nanos(self) -> TimeValue;
    fn ticks(self) -> TimeValue;
}

impl IntoTimeValue for TimeValue
{
    #[inline] fn years(self) -> TimeValue {
        if self.is_strictly_negative() { self.ceil(1.years()) } else { self.floor(1.years())}
    }
    #[inline] fn months(self) -> TimeValue {
        if self.is_strictly_negative() { self.ceil(1.years()) } else { self.floor(1.years())}
    }
    #[inline] fn weeks(self) -> TimeValue {
        if self.is_strictly_negative() { self.ceil(1.weeks()) } else { self.floor(1.weeks())}
    }
    #[inline] fn days(self) -> TimeValue {
        if self.is_strictly_negative() { self.ceil(1.days()) } else { self.floor(1.days())}
    }
    #[inline] fn hours(self) -> TimeValue {
        if self.is_strictly_negative() { self.ceil(1.hours()) } else { self.floor(1.hours())}
    }
    #[inline] fn mins(self) -> TimeValue {
        if self.is_strictly_negative() { self.ceil(1.mins()) } else { self.floor(1.mins())}
    }
    #[inline] fn secs(mut self) -> TimeValue { self.0 &= !SUBSEC_BITMASK; self }

    #[inline] fn millis(self) -> TimeValue {
        if self.is_strictly_negative() { self.ceil(1.millis()) } else { self.floor(1.millis())}
    }
    #[inline] fn micros(self) -> TimeValue {
        if self.is_strictly_negative() { self.ceil(1.micros()) } else { self.floor(1.micros())}
    }
    #[inline] fn nanos(self) -> TimeValue {
        if self.is_strictly_negative() { self.ceil(1.nanos()) } else { self.floor(1.nanos())}
    }
    #[inline] fn ticks(self) -> TimeValue { self }
}


macro_rules! convert_safely {
    ($num:ty, $convert:expr) => {
        impl IntoTimeValue for $num {
            #[inline] fn years(self) -> TimeValue { TimeValue::from_years(($convert)(self)) }
            #[inline] fn months(self) -> TimeValue { TimeValue::from_months(($convert)(self)) }
            #[inline] fn weeks(self) -> TimeValue { TimeValue::from_weeks(($convert)(self)) }
            #[inline] fn days(self) -> TimeValue { TimeValue::from_days(($convert)(self)) }
            #[inline] fn hours(self) -> TimeValue { TimeValue::from_hours(($convert)(self)) }
            #[inline] fn mins(self) -> TimeValue { TimeValue::from_mins(($convert)(self)) }
            #[inline] fn secs(self) -> TimeValue { TimeValue::from_secs(($convert)(self)) }
            #[inline] fn millis(self) -> TimeValue { TimeValue::from_millis(($convert)(self)) }
            #[inline] fn micros(self) -> TimeValue { TimeValue::from_micros(($convert)(self)) }
            #[inline] fn nanos(self) -> TimeValue { TimeValue::from_nanos(($convert)(self)) }
            #[inline] fn ticks(self) -> TimeValue { TimeValue::from_ticks(($convert)(self)) }
        }
    }
}

convert_safely!(i8, |x| x as i64);
convert_safely!(u8, |x| x as i64);

convert_safely!(i16, |x| x as i64);
convert_safely!(u16, |x| x as i64);

convert_safely!(i32, |x| x as i64);
convert_safely!(u32, |x| x as i64);

convert_safely!(i64, |x| x);
convert_safely!(u64, |x| if x > i64::MAX as u64 { i64::MAX } else { x as i64 });

convert_safely!(u128, |x| if x > i64::MAX as u128 { i64::MAX } else { x as i64 });
convert_safely!(i128, |x| if x > i64::MAX as i128 { i64::MAX } else if x < i64::MIN as i128 { i64::MIN } else { x as i64 });

convert_safely!(isize, |x| x as i64);
convert_safely!(usize, |x| if x > i64::MAX as usize { i64::MAX } else { x as i64 });



#[cfg(test)]
mod tests {
    use crate::{SUBSEC_BITLEN, TimeValue};
    use crate::wins::convert::IntoTimeValue;

    #[test]
    fn convert() {
        assert_eq!(1.hours(), 3600.secs());
        assert_eq!(24.hours(), 1.days());
        assert_eq!(1_000_000.nanos(), 1000.micros());
        assert_eq!(1_000_000.nanos(), 1.millis());

        // frac << SUBSEC_BITLEN)/unit)
        let frac = 10;
        let unit = 1000;
        for t in 0..10 {
            let ticks = (frac as f64)*((1<<SUBSEC_BITLEN) as f64);
            println!("{} {ticks}", TimeValue::from_ticks((ticks/10.) as i64));
        }

        assert_eq!(10.millis().to_string(), "10ms");

        let t = 1.weeks() + 5.hours() + 7.mins() + 4.secs() + 42.millis() ;
        assert_eq!(t.days().to_string(),  "7d");
        assert_eq!(t.hours().to_string(), "7d 5h");
        assert_eq!(t.mins().to_string(),  "7d 5h 7min");
        assert_eq!(t.secs().to_string(),  "7d 5h 7min 4s");
    }
}