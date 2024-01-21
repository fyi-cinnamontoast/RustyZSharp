use std::{ops, ptr};
use std::ffi::c_char;
use std::fmt::{Debug, Formatter, write};
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl ops::BitOr<Span> for Span {
    type Output = Span;

    fn bitor(self, rhs: Span) -> Self::Output {
        Span {
            lo: self.lo,
            hi: rhs.hi
        }
    }
}

macro_rules! raw {
    ($val:expr) => {
        Raw::new(&mut $val)
    };
}

pub(crate) use raw;

pub struct Raw<T> {
    p: *mut T
}

impl<T> Copy for Raw<T> {}
impl<T> Clone for Raw<T> {
    fn clone(&self) -> Self { Raw { p: self.p } }
}

impl<T> Raw<T> {
    pub fn null() -> Self {
        Self {p: ptr::null_mut()}
    }

    pub fn new(n: &mut T) -> Self {
        Self {p: n as *mut T}
    }

    pub fn as_immut<'a>(&self) -> Option<&'a T> {
        unsafe { self.p.as_ref() }
    }

    pub fn as_mut<'a>(&mut self) -> Option<&'a mut T> {
        unsafe { self.p.as_mut() }
    }
}

pub fn to_c_char_array(s: &str) -> *const c_char {
    s.to_string().add("\0").as_str().as_ptr() as _
}