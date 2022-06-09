mod timewin;
mod timevalue;
mod timeinterval;
mod timeset;

mod translation;
mod scaling;
mod setops;

pub use timewin::*;
pub use timevalue::*;
pub use timeinterval::*;
pub use timeset::*;


const INFINITE_TIME_VALUE : i64 = i64::MAX;

const SUBSEC_BITLEN: usize = 30; // more than nanosecond precision
// could be set to 20 for microseconds precision, to 10 for millisecond
// and set to 0 to get only second precision
// (but we kept nanos to be compliant with std::time precision)

// fractional part mask
const SUBSEC_BITMASK: i64 = !((!0) << SUBSEC_BITLEN);

// max of seconds according to fract. part precision
const MAX_SEC: i64 = i64::MAX >> SUBSEC_BITLEN;



#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use crate::*;

    fn checktw<T:Debug>(check:&str, x:&T) {
        assert_eq!( check, &format!("{:?}", x));
    }


    #[test]
    fn intersection()
    {
        let t1 = TimeValue::from_ticks(1);
        let t5 = TimeValue::from_ticks(5);
        let t10 = TimeValue::from_ticks(10);

        let t10bis = TimeSpan::centered(t10, t5).unwrap();

        let intersection = !t1 & !t5 & !t10bis;
        dbg!(!intersection.clone());
        checktw( "]-oo,0]U[2,4]U[16,+oo[", &intersection);
    }

    #[test]
    fn union()
    {
        let a : TimeInterval<_> = (TimeValue::from_ticks(1)..=TimeValue::from_ticks(10)).try_into().unwrap();
        let b: TimeInterval<_>  = (TimeValue::from_ticks(15)..=TimeValue::from_ticks(18)).try_into().unwrap();
        let c: TimeInterval<_> = (TimeValue::from_ticks(8)..=TimeValue::from_ticks(14)).try_into().unwrap();

        checktw( "[1,18]", &(a|b|c));

        // dbg!((a|b) + c);
    }

    #[test]
    fn translation()
    {
        let t1 = TimeValue::from_ticks(1);
        let t5 = TimeValue::from_ticks(5);
        let tw : TimeSpan = (t1..t5).try_into().unwrap();

        let now = Timestamp::now();
        assert_eq!( tw + now, now + tw);
    }
}
