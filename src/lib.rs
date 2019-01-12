#![feature(unsize)]

pub mod traitvec;

use std::pin::Pin;

use crate::traitvec::TraitVec;


trait Defer {
    fn call(self: Box<Self>);
}

impl<U, F: FnOnce() -> U> Defer for F {
    fn call(self: Box<Self>) {
        (self)();
    }
}

#[test]
fn main() {
    let mut v: Pin<TraitVec<dyn std::fmt::Debug>> = TraitVec::new();
    {
        let n = 0;
        let x = v.as_mut().push(1);

        let vec = vec![1, 2, 3];

        let mut vec = v.as_mut().push(vec);
        vec.extend(0..100);
        vec.extend(0..100);
    }
    drop(v);
}
