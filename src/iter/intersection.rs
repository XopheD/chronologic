use std::iter::{Fuse, FusedIterator};
use std::mem::swap;
use crate::*;


impl<TW1:TimeConvexIterator,TW2> TimeIntersection<TW2> for TW1
    where
        TW2: TimeConvexIterator<TimePoint=TW1::TimePoint>
{
    type Output = InterIter<Self,TW2>;

    #[inline]
    fn intersection(self, tw: TW2) -> Self::Output {
        InterIter::new(self, tw)
    }
}

impl<TW:TimeConvexIterator> TimeIntersection<TimeInterval<TW::TimePoint>> for TW
{
    type Output = InterIter<Self,<TimeInterval<TW::TimePoint> as IntoIterator>::IntoIter>;

    #[inline]
    fn intersection(self, tw: TimeInterval<TW::TimePoint>) -> Self::Output {
        InterIter::new(self, tw.into_iter())
    }
}

impl<TW:TimeConvexIterator> TimeIntersection<&TimeInterval<TW::TimePoint>> for TW
{
    type Output = InterIter<Self,<TimeInterval<TW::TimePoint> as IntoIterator>::IntoIter>;

    #[inline]
    fn intersection(self, tw: &TimeInterval<TW::TimePoint>) -> Self::Output {
        InterIter::new(self, tw.into_iter())
    }
}


impl<TW:TimeConvexIterator> TimeIntersection<TimeSet<TW::TimePoint>> for TW
{
    type Output = InterIter<Self,<TimeSet<TW::TimePoint> as IntoIterator>::IntoIter>;

    #[inline]
    fn intersection(self, tw: TimeSet<TW::TimePoint>) -> Self::Output {
        InterIter::new(self, tw.into_iter())
    }
}

impl<TW:TimeConvexIterator> TimeIntersection<&TimeSet<TW::TimePoint>> for TW
{
    type Output = InterIter<Self,<TimeSet<TW::TimePoint> as IntoIterator>::IntoIter>;

    #[inline]
    fn intersection(self, tw: &TimeSet<TW::TimePoint>) -> Self::Output {
        // todo: suppress the clone call
        InterIter::new(self, tw.clone().into_iter())
    }
}






#[derive(Copy,Clone,Debug)]
enum InterState {
    Init, // computation didn’t start yet
    WaitI, // I should be next, J is temporary
    WaitJ, // J should be next, I is temporary
    End // nothing more to do
}


pub struct InterIter<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint>
{
    i: Fuse<I>, j: Fuse<J>, state: InterState, tmp: TimeInterval<I::TimePoint>
}

impl<I,J> InterIter<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint>
{
    fn new(i:I, j:J) -> Self {
        Self { i: i.fuse(), j: j.fuse(), state: InterState::Init, tmp:TimeInterval::all() }
    }
}


impl<I,J> Iterator for InterIter<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint>
{
    type Item = TimeInterval<I::TimePoint>;

    fn next(&mut self) -> Option<Self::Item>
    {
        loop {  //dbg!(self.state); dbg!(self.tmp);
            match self.state {
                InterState::Init => {
                    match (self.i.next(), self.j.next()) {
                        (None,_)|(_,None) => { self.state = InterState::End; return None; }

                        (Some(i), Some(j)) if i.upper < j.lower => {
                            // i:       [------------------]
                            // j:                                  [--------]
                            //=>tmp:                               [--------]
                            self.state = InterState::WaitI;
                            self.tmp=j;
                        },

                        (Some(i), Some(j)) if j.upper < i.lower  => {
                            // i:                          [------------------]
                            // j:          [--------]
                            self.state = InterState::WaitJ;
                            self.tmp=i;
                        },
                        (Some(mut i), Some(j)) if i.upper < j.upper  => {
                            // i:     [------------------]       or           [-----------]
                            // j:                  [--------]    or    [----------------------]
                            self.state = InterState::WaitI;
                            if j.lower > i.lower { i.lower = j.lower; }
                            self.tmp = j; return Some(i);
                        },
                        (Some(i), Some(mut j)) => {
                            // i:     [------------------]      or           [----------------]
                            // j:           [--------]          or     [------------------]
                            self.state = InterState::WaitJ;
                            if i.lower > j.lower { j.lower = i.lower; }
                            self.tmp = i; return Some(j);
                        },
                    }
                }
                InterState::WaitI => {
                    match self.i.next() {
                        None => {
                            /* end of the iterator over i...*/
                            self.state = InterState::End;
                            return None;
                        },
                        Some(i) if i.upper < self.tmp.lower => {
                            // i:       [------------------]
                            // tmp:                                [--------]
                        },
                        Some(i) if self.tmp.upper < i.lower  => {
                            // i:                          [------------------]
                            // tmp:        [--------]
                            self.state = InterState::WaitJ;
                            self.tmp = i;
                        },
                        Some(mut i) if i.upper <= self.tmp.upper => {
                            // i:     [------------------]       or           [-----------]
                            // tmp:                [--------]    or    [----------------------]
                            if self.tmp.lower > i.lower { i.lower = self.tmp.lower; }
                            return Some(i);
                        },
                        Some(mut i) => {
                            // i:     [------------------]      or           [----------------]
                            // tmp:         [--------]          or     [------------------]
                            self.state = InterState::WaitJ;
                            if self.tmp.lower < i.lower { self.tmp.lower = i.lower; }
                            swap(&mut i, &mut self.tmp);
                            return Some(i)
                        },
                    }
                }
                InterState::WaitJ => {
                    match self.j.next() {
                        None => {
                            /* end of the iterator over i...*/
                            self.state = InterState::End;
                            return None;
                        },
                        Some(j) if j.upper < self.tmp.lower => {
                            // tmp:                                [--------]
                            // j:       [------------------]
                        },
                        Some(j) if self.tmp.upper < j.lower  => {
                            // tmp:        [--------]
                            // j:                          [------------------]
                            self.state = InterState::WaitI;
                            self.tmp = j;
                        },
                        Some(mut j) if j.upper <= self.tmp.upper => {
                            // tmp:                [--------]    or    [----------------------]
                            // j:     [------------------]       or           [-----------]
                            if self.tmp.lower > j.lower { j.lower = self.tmp.lower; }
                            return Some(j);
                        }
                        Some(mut j) => {
                            // tmp:         [--------]          or     [------------------]
                            // j:     [------------------]      or           [----------------]
                            self.state = InterState::WaitI;
                            if self.tmp.lower < j.lower { self.tmp.lower = j.lower; }
                            swap(&mut j, &mut self.tmp);
                            return Some(j);
                        }
                    }
                }
                InterState::End => { return None; }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        match self.state {
            InterState::End => { (0, Some(0)) }
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

impl<I,J> TimeConvexIterator for InterIter<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint>
{
    type TimePoint = I::TimePoint;
}

impl<I,J> FusedIterator for InterIter<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint> {}



#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    fn inter()
    {
        let i:TimeInterval<_> = (TimeValue::from_ticks(0) ..= TimeValue::from_ticks(200)).into();

        let k = (TimeValue::from_ticks(100) ..= TimeValue::from_ticks(155))
           // .complementary()
            .intersection(i)
            .intersection(TimeValue::from_ticks(0) ..= TimeValue::from_ticks(200))
            //.complementary()
            ;
        dbg!(k);
        //  | (TimeValue::from_ticks(7) ..= TimeValue::from_ticks(10))
        // | (TimeValue::from_ticks(10) ..= TimeValue::from_ticks(25));

    }
}
