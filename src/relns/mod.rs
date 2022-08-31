//! # Time relations
mod overlapping;

pub use overlapping::*;


#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use crate::*;


    fn checktw<T:Debug>(check:&str, x:&T) {
        assert_eq!( check, &format!("{:?}", x));
    }
/*
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

    #[test]
    fn contains2()
    {
        let t1 = Timestamp::default()+TimeValue::from_ticks(1);
        let t5 = Timestamp::default()+TimeValue::from_ticks(5);
        let t10 = Timestamp::default()+TimeValue::from_ticks(10);
        let tw : TimeSlots = !t1 & !t5 & !t10;
        dbg!(&tw);
        assert!( !tw.contains(&t5));
    }*/

}

