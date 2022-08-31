use std::iter::*;
use std::mem::swap;
use super::*;


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
    pub(super) fn new(i:I, j:J) -> Self {

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
                        Some(mut i) if self.tmp.upper < i.lower  => {
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
                        Some(mut j) if self.tmp.upper < j.lower  => {
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

