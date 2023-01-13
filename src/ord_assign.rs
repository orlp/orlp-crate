use core::ptr;

struct PreventUnwind;
impl Drop for PreventUnwind {
    fn drop(&mut self) {
        panic!("intentional double-panic to abort, not sound to unwind")
    }
}

/// An extension trait for mutating in-place versions of `Ord`'s `min`/`max`/`clamp`.
/// 
/// All implementations call `abort()` if the `Ord` implementation panics.
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

#[cfg(test)]
mod test {
    use super::OrdAssign;

    #[test]
    fn test_at_least() {
        let mut x = 0;
        x.bound_at_least(42);
        assert_eq!(x, 42);
    }

    #[test]
    fn test_at_most() {
        let mut x = 42;
        x.bound_at_most(0);
        assert_eq!(x, 0);
    }

    #[test]
    fn test_bound() {
        let mut x = 0;
        x.bound(50, 100);
        assert_eq!(x, 50);
        x.bound(0, 25);
        assert_eq!(x, 25);
    }
}