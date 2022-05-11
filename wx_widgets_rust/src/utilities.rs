
use std::fmt;




pub fn to_addr<T>(x: &T) -> usize {
    let p : *const &T = std::ptr::addr_of!(x);
    p as usize
}

pub unsafe fn from_addr<'a, T>(address: usize) -> &'a T {
    &*(address as *const &T)
}

pub struct Fmt<F>(pub F) where F: Fn(&mut fmt::Formatter) -> fmt::Result;

impl<F> fmt::Debug for Fmt<F>
    where F: Fn(&mut fmt::Formatter) -> fmt::Result
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(f)
    }
}