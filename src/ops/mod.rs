mod union;
mod intersect;
mod compl;

mod scaling;
mod transl;
mod range;



#[cfg(test)]
mod tests {
    use crate::*;

    fn instants() -> Box<[TimeValue]> {
        (0..100).map(|t| TimeValue::from_secs(t)).collect()
    }

    fn dates() -> Box<[Timestamp]> {
        instants().iter().map(|t| Timestamp::from_origin(*t)).collect()
    }

    #[test]
    pub fn union()
    {
        let t = instants();
        assert_eq!( "[1s,5s]", (TimeInterval::new(t[1],t[4]) | (t[3]..=t[5])).to_string() );
        assert_eq!( "[1s,3s]U{7s}", (t[7] | (t[1]..=t[3])).to_string() );

        let d = dates();
        assert_eq!( "[00:01,00:03]U{01:10}".to_string(), (d[70] | (d[1]..=d[3])).format_timeset("%M:%S") );

    }

    #[test]
    pub fn union2()
    {
        let a1 = TimeValue::from_ticks(82);
        let a2 = TimeValue::from_ticks(178);
        let b1 = TimeValue::from_ticks(179);
        let b2 = TimeValue::from_ticks(279);

        let mut a = TimeSet::convex(a1,a2);
        let b = TimeSet::convex(b1,b2);

        assert_eq!(a|b, TimeInterval::new(a1,b2));
    }

    #[test]
    pub fn union3()
    {
        let a1 = TimeValue::from_ticks(1);
        let a2 = TimeValue::from_ticks(5);
        let b1 = TimeValue::from_ticks(10);
        let b2 = TimeValue::from_ticks(20);

        let mut w = TimeSpans::empty();
        dbg!(&w);
        w |= TimeInterval::new(a1,a2);
        dbg!(&w);
        w |= TimeInterval::new(b1,b2);
        dbg!(&w);
    }

    #[test]
    pub fn intersection()
    {

    }

    #[test]
    pub fn complementary()
    {

    }

    #[test]
    pub fn mix()
    {

    }
}