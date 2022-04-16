




pub fn to_addr<T>(x: &T) -> usize {
    let p : *const T = std::ptr::addr_of!(*x);
    p as usize
}

pub unsafe fn from_addr<'a, T>(address: usize) -> &'a T {
    &*(address as *const T)
}

