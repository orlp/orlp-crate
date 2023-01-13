use core::ptr;

struct PreventUnwind;
impl Drop for PreventUnwind {
    fn drop(&mut self) {
        panic!("intentional double-panic to abort, not sound to unwind")
    }
}

/// A mutating in-place version of Ord::min/max/clamp.
pub trait OrdAssign {
    fn bound_at_least(&mut self, other: Self);
    fn bound_at_most(&mut self, other: Self);
    fn bound(&mut self, lo: Self, hi: Self);
}

impl<T: Ord> OrdAssign for T {
    fn bound_at_least(&mut self, other: Self) {
        unsafe {
            let pu = PreventUnwind;
            ptr::write(self as *mut Self, ptr::read(self as *mut Self).max(other));
            core::mem::forget(pu);
        }
    }

    fn bound_at_most(&mut self, other: Self) {
        unsafe {
            let pu = PreventUnwind;
            ptr::write(self as *mut Self, ptr::read(self as *mut Self).min(other));
            core::mem::forget(pu);
        }
    }

    fn bound(&mut self, lo: Self, hi: Self) {
        unsafe {
            let pu = PreventUnwind;
            ptr::write(
                self as *mut Self,
                ptr::read(self as *mut Self).clamp(lo, hi),
            );
            core::mem::forget(pu);
        }
    }
}
