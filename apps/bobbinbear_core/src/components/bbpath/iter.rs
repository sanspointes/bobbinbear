use bevy::prelude::*;

use super::{bbpath::TessPathVerb, BBPathEvent};

/// Manually implemented to avoid iterator overhead when skipping over
/// several points where the custom attributes are stored.
///
/// It makes an unfortunately large difference (the simple iterator
/// benchmarks are 2 to 3 times faster).
#[derive(Copy, Clone)]
struct PointIter<'l> {
    ptr: *const Vec2,
    end: *const Vec2,
    _marker: core::marker::PhantomData<&'l Vec2>,
}

#[allow(dead_code)]
impl<'l> PointIter<'l> {
    fn new(slice: &'l [Vec2]) -> Self {
        let ptr = slice.as_ptr();
        let end = unsafe { ptr.add(slice.len()) };
        PointIter {
            ptr,
            end,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    fn remaining_len(&self) -> usize {
        (self.end as usize - self.ptr as usize) / core::mem::size_of::<Vec2>()
    }

    #[inline]
    fn next(&mut self) -> Vec2 {
        // Don't bother panicking here. calls to next
        // are always followed by advance_n which will
        // catch the issue and panic.
        if self.ptr >= self.end {
            return Vec2::new(f32::NAN, f32::NAN);
        }

        unsafe {
            let output = *self.ptr;
            self.ptr = self.ptr.offset(1);

            output
        }
    }

    #[inline]
    fn advance_n(&mut self, n: usize) {
        unsafe {
            assert!(self.remaining_len() >= n);
            self.ptr = self.ptr.add(n);
        }
    }
}

pub struct BBPathIter<'l> {
    points: PointIter<'l>,
    verbs: ::core::slice::Iter<'l, TessPathVerb>,
    current: Vec2,
    first: Vec2,
}
impl<'l> BBPathIter<'l> {
    pub fn new(points: &'l [Vec2], verbs: &'l [TessPathVerb]) -> Self {
        BBPathIter {
            points: PointIter::new(points),
            verbs: verbs.iter(),
            current: Vec2::ZERO,
            first: Vec2::ZERO,
        }
    }
}
impl<'l> Iterator for BBPathIter<'l> {
    type Item = BBPathEvent;
    #[inline]
    fn next(&mut self) -> Option<BBPathEvent> {
        match self.verbs.next() {
            Some(&TessPathVerb::Begin) => {
                self.current = self.points.next();
                // self.skip_attributes();
                self.first = self.current;
                Some(BBPathEvent::Begin { from: self.current })
            }
            Some(&TessPathVerb::LineTo) => {
                let from = self.current;
                self.current = self.points.next();
                // self.skip_attributes();
                Some(BBPathEvent::Line {
                    from,
                    to: self.current,
                })
            }
            Some(&TessPathVerb::QuadraticTo) => {
                let from = self.current;
                let ctrl1 = self.points.next();
                self.current = self.points.next();
                // self.skip_attributes();
                Some(BBPathEvent::Quadratic {
                    from,
                    ctrl1,
                    to: self.current,
                })
            }
            Some(&TessPathVerb::CubicTo) => {
                let from = self.current;
                let ctrl1 = self.points.next();
                let ctrl2 = self.points.next();
                self.current = self.points.next();
                // self.skip_attributes();
                Some(BBPathEvent::Cubic {
                    from,
                    ctrl1,
                    ctrl2,
                    to: self.current,
                })
            }
            Some(&TessPathVerb::Close) => {
                let last = self.current;
                let _ = self.points.next();
                Some(BBPathEvent::End {
                    last,
                    first: self.first,
                    close: true,
                })
            }
            Some(&TessPathVerb::End) => {
                let last = self.current;
                self.current = self.first;
                Some(BBPathEvent::End {
                    last,
                    first: self.first,
                    close: false,
                })
            }
            None => None,
        }
    }
}
