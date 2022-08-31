//mod compl;
//mod inter;
//mod union;
mod translation;
mod scaling;
/*
pub use compl::*;
pub use inter::*;
pub use union::*;
*/

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use crate::*;


    fn checktw<T:Debug>(check:&str, x:&T) {
        assert_eq!( check, &format!("{:?}", x));
    }


    #[test]
    fn complement()
    {
        let t1 = TimeValue::from_ticks(1);
        let t5 = TimeValue::from_ticks(5);
        let tw = TimeSpan::new(t1, t5);
        let tw = tw.complementary();
        dbg!(&tw);
        let a = &tw;
        let b = &tw;
        let b: TimeSpans = b.complementary();
        let c = tw.complementary();
        dbg!(tw.complementary());
    }

    #[test]
    fn intersection()
    {
        let t1 = TimeValue::from_ticks(1);
        let t5 = TimeValue::from_ticks(5);
        let t10 = TimeValue::from_ticks(10);
        let t10bis = TimeSpan::centered(t10, t5).unwrap();


        let intersection = t1.iter().complementary()
            .intersection_old(t5.complementary())
            .intersection_old(t10bis.complementary())
            .collect::<TimeSet<_>>();
        checktw( "]-oo,0]U[2,4]U[16,+oo[", &intersection);
    }

    #[test]
    fn union()
    {
        let a : TimeInterval<_> = (TimeValue::from_ticks(1)..=TimeValue::from_ticks(10)).try_into().unwrap();
        let b: TimeInterval<_>  = (TimeValue::from_ticks(15)..=TimeValue::from_ticks(18)).try_into().unwrap();
        let c: TimeInterval<_> = (TimeValue::from_ticks(8)..=TimeValue::from_ticks(14)).try_into().unwrap();

        checktw( "[1,18]", &(a.union(b).union(c)));

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
