use std::iter::*;
use std::mem::swap;
use crate::iter::*;

#[derive(Copy,Clone,Debug)]
enum UnionState {
    Init, // computation didn’t start yet
    WaitI, // I should be next, J is temporary
    WaitJ, // J should be next, I is temporary
    OnlyI, // I should be next, J is empty (only I remains)
    OnlyJ, // J should be next, I is empty (only J remains)
    End // nothing more to do
}


pub struct UnionIter<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint>
{
    i: Fuse<I>, j: Fuse<J>, state: UnionState, tmp: TimeInterval<I::TimePoint>
}

impl<I,J> UnionIter<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint>
{
    pub(crate) fn new(i:I, j:J) -> Self {

        Self { i: i.fuse(), j: j.fuse(), state: UnionState::Init, tmp:TimeInterval::all() }
    }
}


impl<I,J> Iterator for UnionIter<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint>
{
    type Item = TimeInterval<I::TimePoint>;

    fn next(&mut self) -> Option<Self::Item>
    {
        loop {  //dbg!(self.state); dbg!(self.tmp);
            match self.state {
                UnionState::Init => {
                    match (self.i.next(), self.j.next()) {
                        (None,None) => { self.state = UnionState::End; return None; }
                        (Some(i), None) => { self.state = UnionState::OnlyI; return Some(i); },
                        (None, Some(j)) => { self.state = UnionState::OnlyJ; return Some(j); },

                        (Some(i), Some(j)) if i.upper < j.lower => {
                            // i:       [------------------]
                            // j:                                  [--------]
                            //=>tmp:                               [--------]
                            self.state = UnionState::WaitI;
                            self.tmp=j; return Some(i);
                        },

                        (Some(i), Some(j)) if j.upper < i.lower  => {
                            // i:                          [------------------]
                            // j:          [--------]
                            //=>tmp:                       [------------------]
                            self.state = UnionState::WaitJ;
                            self.tmp=i; return Some(j);
                        },
                        (Some(i), Some(j)) if i.upper < j.upper  => {
                            // i:     [------------------]       or           [-----------]
                            // j:                  [--------]    or    [----------------------]
                            //=>tmp:  [---------------------]    or    [----------------------]
                            self.state = UnionState::WaitI;
                            self.tmp = TimeInterval { lower: i.lower.min(j.lower), upper: j.upper };
                        },
                        (Some(i), Some(j)) => {
                            // i:     [------------------]      or           [----------------]
                            // j:           [--------]          or     [------------------]
                            //=>tmp   [------------------]      or     [----------------------]
                            self.state = UnionState::WaitJ;
                            self.tmp = TimeInterval { lower: i.lower.min(j.lower), upper: i.upper };
                        },
                     }
                }
                UnionState::WaitI => {
                    match self.i.next() {
                        None => {
                            /* end of the iterator over i...*/
                            self.state = UnionState::OnlyJ;
                            return Some(self.tmp);
                        },
                        Some(i) if i.upper < self.tmp.lower => {
                            // i:       [------------------]
                            // tmp:                                [--------]
                            //=>tmp:                               [--------]
                            return Some(i);
                        },
                        Some(mut i) if self.tmp.upper < i.lower  => {
                            // i:                          [------------------]
                            // tmp:        [--------]
                            //=>tmp:                       [------------------]
                            self.state = UnionState::WaitJ;
                            swap(&mut self.tmp, &mut i);
                            return Some(i);
                        },
                        Some(i) if i.upper <= self.tmp.upper => {
                            // i:     [------------------]       or           [-----------]
                            // tmp:                [--------]    or    [----------------------]
                            //=>tmp:  [---------------------]    or    [----------------------]
                            if self.tmp.lower > i.lower { self.tmp.lower = i.lower; }
                        },
                        Some(i) => {
                            // i:     [------------------]      or           [----------------]
                            // tmp:         [--------]          or     [------------------]
                            //=>tmp   [------------------]      or     [----------------------]
                            self.state = UnionState::WaitJ;
                            if self.tmp.lower > i.lower { self.tmp.lower = i.lower; }
                            self.tmp.upper = i.upper;
                        },
                    }
                }
                UnionState::WaitJ => {
                    match self.j.next() {
                        None => {
                            /* end of the iterator over i...*/
                            self.state = UnionState::OnlyJ;
                            return Some(self.tmp);
                        },
                        Some(j) if j.upper < self.tmp.lower => {
                            // tmp:                                [--------]
                            // j:       [------------------]
                            //=>tmp:                               [--------]
                            return Some(j);
                        },
                        Some(mut j) if self.tmp.upper < j.lower  => {
                            // tmp:        [--------]
                            // j:                          [------------------]
                            //=>tmp:                       [------------------]
                            self.state = UnionState::WaitI;
                            swap(&mut self.tmp, &mut j);
                            return Some(j);
                        },
                        Some(j) if j.upper <= self.tmp.upper => {
                            // tmp:                [--------]    or    [----------------------]
                            // j:     [------------------]       or           [-----------]
                            //=>tmp:  [---------------------]    or    [----------------------]
                            if self.tmp.lower > j.lower { self.tmp.lower = j.lower; }
                        }
                        Some(j) => {
                            // tmp:         [--------]          or     [------------------]
                            // j:     [------------------]      or           [----------------]
                            //=>tmp   [------------------]      or     [----------------------]
                            self.state = UnionState::WaitI;
                            if self.tmp.lower > j.lower { self.tmp.lower = j.lower; }
                            self.tmp.upper = j.upper;
                        }
                    }
                }
                UnionState::OnlyI => { return self.i.next(); }
                UnionState::OnlyJ => { return self.j.next(); }
                UnionState::End => { return None; }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        match self.state {
            UnionState::OnlyI => { self.i.size_hint() }
            UnionState::OnlyJ => { self.j.size_hint() }
            UnionState::End => { (0, Some(0)) }
            _ => {
                if let (_,Some(imax)) = self.i.size_hint() {
                    if let (_,Some(jmax)) = self.j.size_hint() {
                        return (0, Some(imax.saturating_add(jmax)));
                    }
                }
                (0, None)
            }
        }
    }
}

impl<I,J> TimeConvexIterator for UnionIter<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint>
{
    type TimePoint = I::TimePoint;
}

impl<I,J> FusedIterator for UnionIter<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint> {}


#[cfg(test)]
pub mod tests {
    //extern crate test; use test::Bencher;

    use std::fmt::Debug;
    use crate::*;

    fn checktw<T:Debug>(check:&str, x:&T) {
        assert_eq!( check, &format!("{:?}", x));
    }
/*
    #[bench]
    fn union_iterator(bencher: &mut Bencher)
    {
        let kk = TimeSet::from(TimeValue::from_ticks(100) ..= TimeValue::from_ticks(155))
            .complementary()
            .union(TimeValue::from_ticks(200) ..= TimeValue::from_ticks(205))
            .complementary()
            .union(TimeValue::from_ticks(50) ..= TimeValue::from_ticks(70))
            .complementary()
            .union(TimeValue::from_ticks(65) ..= TimeValue::from_ticks(105))
            ;

        bencher.iter(|| {
            let k = (TimeValue::from_ticks(100) ..= TimeValue::from_ticks(155))
                .into_convex_iter()
                .complementary()
                .union(TimeValue::from_ticks(200) ..= TimeValue::from_ticks(205))
                .complementary()
                .union(TimeValue::from_ticks(50) ..= TimeValue::from_ticks(70))
                .complementary()
                .union(kk.clone())
                .collect::<TimeSet<_>>();

            //checktw( "]-oo,49]U[65,105]U[156,+oo[", &k);
        })
    }

    #[bench]
    fn union_iterator2(bencher: &mut Bencher)
    {
        let kk = TimeSet::from(TimeValue::from_ticks(100) ..= TimeValue::from_ticks(155))
            .complementary()
            .union(TimeValue::from_ticks(200) ..= TimeValue::from_ticks(205))
            .complementary()
            .union(TimeValue::from_ticks(50) ..= TimeValue::from_ticks(70))
            .complementary()
            .union(TimeValue::from_ticks(65) ..= TimeValue::from_ticks(105))
            ;
        bencher.iter(|| {
            let k = TimeSet::from(TimeValue::from_ticks(100) ..= TimeValue::from_ticks(155))
                .complementary()
                .union(TimeValue::from_ticks(200) ..= TimeValue::from_ticks(205))
                .complementary()
                .union(TimeValue::from_ticks(50) ..= TimeValue::from_ticks(70))
                .complementary()
                .union(kk.clone());

            //checktw( "]-oo,49]U[65,105]U[156,+oo[", &k);
        })
    }*/
/*
    #[bench]
    fn union_op(bencher: &mut Bencher)
    {
        bencher.iter(|| {
            let k = TimeSet::from(TimeValue::from_ticks(100) ..= TimeValue::from_ticks(155));
            let k = !k;
            let k = k | TimeSet::from((TimeValue::from_ticks(200) ..= TimeValue::from_ticks(205)));
            let k = !k;
            let k = k | TimeSet::from((TimeValue::from_ticks(50) ..= TimeValue::from_ticks(70)));
            let k = !k;
            let k = k | TimeSet::from((TimeValue::from_ticks(65) ..= TimeValue::from_ticks(105)));

            /*       .into_convex_iter()
                   .complementary()
                   .union(TimeValue::from_ticks(200) ..= TimeValue::from_ticks(205))
                   .complementary()
                   .union(TimeValue::from_ticks(50) ..= TimeValue::from_ticks(70))
                   .complementary()
                   .union(TimeValue::from_ticks(65) ..= TimeValue::from_ticks(105))
                   .collect::<TimeSet<_>>();*/

            // checktw( "]-oo,49]U[65,105]U[156,+oo[", &k);
        })
    }
    #[bench]
    fn union_op2(bencher: &mut Bencher)
    {
        bencher.iter(|| {
            let k = !(!((!TimeSet::from(TimeValue::from_ticks(100) ..= TimeValue::from_ticks(155)))
                                 | TimeSet::from((TimeValue::from_ticks(200) ..= TimeValue::from_ticks(205))))
                                     | TimeSet::from((TimeValue::from_ticks(50) ..= TimeValue::from_ticks(70))))
                | TimeSet::from((TimeValue::from_ticks(65) ..= TimeValue::from_ticks(105)));

            /*       .into_convex_iter()
                   .complementary()
                   .union(TimeValue::from_ticks(200) ..= TimeValue::from_ticks(205))
                   .complementary()
                   .union(TimeValue::from_ticks(50) ..= TimeValue::from_ticks(70))
                   .complementary()
                   .union(TimeValue::from_ticks(65) ..= TimeValue::from_ticks(105))
                   .collect::<TimeSet<_>>();*/

             //checktw( "]-oo,49]U[65,105]U[156,+oo[", &k);
        })
    }
*/
    #[test]
    fn union()
    {
        let k = TimeSet::from(TimeValue::from_ticks(100) ..= TimeValue::from_ticks(155))
            .complementary()
            .union(TimeValue::from_ticks(200) ..= TimeValue::from_ticks(205))
            .complementary()
            .union(TimeValue::from_ticks(50) ..= TimeValue::from_ticks(70))
            .complementary()
            .union(TimeValue::from_ticks(65) ..= TimeValue::from_ticks(105))
            ;

        //checktw( "]-oo,49]U[65,105]U[156,+oo[", &k);



    }
}