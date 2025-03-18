use std::iter::{Fuse, FusedIterator};
use std::mem::swap;
use crate::*;
use crate::iter::*;

/// # Time window union iterator
pub trait TimeUnion<TW>: TimeConvexIterator
{
    type Output:TimeConvexIterator<TimePoint=Self::TimePoint>;
    fn union(self, tw: TW) -> Self::Output;
}


impl<TW1,TW2> TimeUnion<TW2> for TW1
    where
        TW1: TimeConvexIterator,
        TW2: TimeConvexIterator<TimePoint=TW1::TimePoint>
{
    type Output = IterUnion<Self,TW2>;

    #[inline]
    fn union(self, tw: TW2) -> Self::Output {
        IterUnion::new(self, tw)
    }
}

impl<TW:TimeConvexIterator> TimeUnion<TimeInterval<TW::TimePoint>> for TW
{
    type Output = IterUnion<Self,<TimeInterval<TW::TimePoint> as IntoIterator>::IntoIter>;

    #[inline]
    fn union(self, tw: TimeInterval<TW::TimePoint>) -> Self::Output {
        IterUnion::new(self, tw.into_iter())
    }
}

impl<TW:TimeConvexIterator> TimeUnion<&TimeInterval<TW::TimePoint>> for TW
{
    type Output = IterUnion<Self,<TimeInterval<TW::TimePoint> as IntoIterator>::IntoIter>;

    #[inline]
    fn union(self, tw: &TimeInterval<TW::TimePoint>) -> Self::Output {
        IterUnion::new(self, tw.into_iter())
    }
}


impl<TW:TimeConvexIterator> TimeUnion<TimeSet<TW::TimePoint>> for TW
{
    type Output = IterUnion<Self,<TimeSet<TW::TimePoint> as IntoIterator>::IntoIter>;

    #[inline]
    fn union(self, tw: TimeSet<TW::TimePoint>) -> Self::Output {
        IterUnion::new(self, tw.into_iter())
    }
}

impl<TW:TimeConvexIterator> TimeUnion<&TimeSet<TW::TimePoint>> for TW
{
    type Output = IterUnion<Self,<TimeSet<TW::TimePoint> as IntoIterator>::IntoIter>;

    #[inline]
    fn union(self, tw: &TimeSet<TW::TimePoint>) -> Self::Output {
        // todo: suppress the clone function
        IterUnion::new(self, tw.clone().into_iter())
    }
}



#[derive(Copy,Clone,Debug)]
enum UnionState {
    Init, // computation didn’t start yet
    WaitI, // I should be next, J is temporary
    WaitJ, // J should be next, I is temporary
    OnlyI, // I should be next, J is empty (only I remains)
    OnlyJ, // J should be next, I is empty (only J remains)
    End // nothing more to do
}


pub struct IterUnion<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint>
{
    i: Fuse<I>, j: Fuse<J>, state: UnionState, tmp: TimeInterval<I::TimePoint>
}

impl<I,J> IterUnion<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint>
{
    fn new(i:I, j:J) -> Self {
        Self { i: i.fuse(), j: j.fuse(), state: UnionState::Init, tmp:TimeInterval::all() }
    }
}


impl<I,J> Iterator for IterUnion<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint>
{
    type Item = TimeInterval<I::TimePoint>;

    fn next(&mut self) -> Option<Self::Item>
    {
        loop {
            match self.state {
                UnionState::Init => {
                    match (self.i.next(), self.j.next()) {
                        (None,None) => { self.state = UnionState::End; return None; }
                        (Some(i), None) => { self.state = UnionState::OnlyI; return Some(i); },
                        (None, Some(j)) => { self.state = UnionState::OnlyJ; return Some(j); },

                        (Some(i), Some(j)) if i.upper < j.lower.just_before() => {
                            // i:       [------------------]
                            // j:                                  [--------]
                            //=>tmp:                               [--------]
                            self.state = UnionState::WaitI;
                            self.tmp=j; return Some(i);
                        },

                        (Some(i), Some(j)) if j.upper < i.lower.just_before()  => {
                            // i:                          [------------------]
                            // j:          [--------]
                            //=>tmp:                       [------------------]
                            self.state = UnionState::WaitJ;
                            self.tmp=i; return Some(j);
                        },
                        (Some(i), Some(j)) if i.upper <= j.upper  => {
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
                        Some(i) if i.upper < self.tmp.lower.just_before() => {
                            // i:       [------------------]
                            // tmp:                                [--------]
                            //=>tmp:                               [--------]
                            return Some(i);
                        },
                        Some(mut i) if self.tmp.upper < i.lower.just_before()  => {
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
                        Some(j) if j.upper < self.tmp.lower.just_before() => {
                            // tmp:                                [--------]
                            // j:       [------------------]
                            //=>tmp:                               [--------]
                            return Some(j);
                        },
                        Some(mut j) if self.tmp.upper < j.lower.just_before()  => {
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

impl<I,J> TimeConvexIterator for IterUnion<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint>
{
    type TimePoint = I::TimePoint;
}

impl<I,J> FusedIterator for IterUnion<I,J>
    where
        I:TimeConvexIterator,
        J:TimeConvexIterator<TimePoint=I::TimePoint> {}


