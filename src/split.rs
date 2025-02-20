use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct Splittable<R, W> {
    r: R,
    w: W,
}

impl<R, W> Splittable<R, W> {
    pub fn new(r: R, w: W) -> Self {
        Self {
            r,
            w,
        }
    }

    pub fn split(self) -> (RHalf<R, W>, WHalf<R, W>) {
        let key = Arc::new(0);
        (
            RHalf::new(self.r, key.clone()),
            WHalf::new(self.w, key),
        )
    }

    pub fn get_r(&self) -> &R {
        &self.r
    }

    pub fn get_w(&self) -> &W {
        &self.w
    }

    pub fn get_r_mut(&mut self) -> &mut R {
        &mut self.r
    }

    pub fn get_w_mut(&mut self) -> &mut W {
        &mut self.w
    }
}

impl <R, W> Deref for Splittable<R, W> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.r
    }
}

impl <R, W> DerefMut for Splittable<R, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.r
    }
}

pub struct RHalf<R, W> {
    k: Arc<u8>,
    r: R,
    _p: std::marker::PhantomData<W>,
}

impl<R, W> RHalf<R, W> {
    pub(crate) fn new(r: R, k: Arc<u8>) -> Self {
        Self {
            k,
            r,
            _p: Default::default(),
        }
    }

    pub fn is_pair_of(&self, w: &WHalf<R, W>) -> bool {
        Arc::ptr_eq(&self.k, &w.k)
    }

    pub fn unsplit(self, w: WHalf<R, W>) -> Splittable<R, W> {
        if !self.is_pair_of(&w) {
            panic!("not a pair");
        }

        Splittable::new(self.r, w.w)
    }
}

impl<R, W> Deref for RHalf<R, W> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.r
    }
}

impl<R, W> DerefMut for RHalf<R, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.r
    }
}

pub struct WHalf<R, W> {
    k: Arc<u8>,
    w: W,
    _p: std::marker::PhantomData<R>,
}

impl<R, W> WHalf<R, W> {
    pub(crate) fn new(w: W, k: Arc<u8>) -> Self {
        Self {
            k,
            w,
            _p: Default::default(),
        }
    }

    pub fn is_pair_of(&self, r: &RHalf<R, W>) -> bool {
        r.is_pair_of(self)
    }

    pub fn unsplit(self, r: RHalf<R, W>) -> Splittable<R, W> {
        r.unsplit(self)
    }
}

impl<R, W> DerefMut for WHalf<R, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.w
    }
}

impl<R, W> Deref for WHalf<R, W> {
    type Target = W;
    fn deref(&self) -> &Self::Target {
        &self.w
    }
}

#[cfg(test)]
mod Test {
    #[test]
    fn test() {
        pub struct TestRead {

        }

        pub struct TestWrite {

        }

        let s1 = super::Splittable::new(TestRead{}, TestWrite{});
        let (r1, w1) = s1.split();
        let s2 = super::Splittable::new(TestRead{}, TestWrite{});
        let (r2, w2) = s2.split();
        assert!(r1.is_pair_of(&w1));
        assert!(w1.is_pair_of(&r1));
        assert!(r2.is_pair_of(&w2));
        assert!(w2.is_pair_of(&r2));
        assert!(!w1.is_pair_of(&r2));
        assert!(!w2.is_pair_of(&r1));

        let s1 = r1.unsplit(w1);
        let s2 = r2.unsplit(w2);
    }
}
