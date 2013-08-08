use std::sys;
use std::cast;
use std::libc;
use std::unstable::intrinsics;

pub struct ReferenceCount {
    value: *mut int
}

impl ReferenceCount {
    pub fn new() -> ReferenceCount { unsafe {
        let result = libc::calloc(sys::size_of::<int>() as libc::size_t, 1) as *mut int;

        *result = 1;

        return ReferenceCount {
            value: result
        };
    }}

    #[inline(always)] pub fn retain(&self) { unsafe {
        intrinsics::atomic_xadd(cast::transmute(self.value), 1);
    }}

    #[inline(always)] pub fn release(&self) -> bool { unsafe {
        return intrinsics::atomic_xsub(cast::transmute(self.value), 1) == 1;
    }}
}

impl Clone for ReferenceCount {
    pub fn clone(&self) -> ReferenceCount {
        self.retain();
        
        return ReferenceCount {
            value: self.value
        }
    }
}

#[cfg(test)]
mod tests {
    use refcount::ReferenceCount;

    #[test]
    fn test_new() {
        let r0 = ReferenceCount::new();

        assert_eq!(unsafe { *r0.value }, 1);
    }

    #[test]
    fn test_clone() {
        let r0 = ReferenceCount::new();
        let r1 = r0.clone();

        assert_eq!(unsafe { *r0.value }, 2);
        assert_eq!(unsafe { *r1.value }, 2);
    }

    #[test]
    fn test_retain() {
        let r0 = ReferenceCount::new();

        assert_eq!(unsafe { *r0.value }, 1);
        r0.retain();
        assert_eq!(unsafe { *r0.value }, 2);
    }

    #[test]
    fn test_release() {
        let r0 = ReferenceCount::new();

        assert_eq!(unsafe { *r0.value }, 1);
        r0.retain();
        assert_eq!(unsafe { *r0.value }, 2);
        assert_eq!(r0.release(), false);
        assert_eq!(unsafe { *r0.value }, 1);
        assert_eq!(r0.release(), true);
        assert_eq!(unsafe { *r0.value }, 0);
    }
}