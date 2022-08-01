//! Shoot yourself in the foot the easy way.

pub fn footgun<A, B>(mut a: A) -> B {
    let a_ptr = &mut a as *mut A;
    let b_ptr = a_ptr as *mut B;

    // SAFETY: TODO
    unsafe { std::ptr::read(b_ptr) }
}
