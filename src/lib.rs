#![feature(unsize)]

pub mod traitvec;

#[test]
fn test() {
    use crate::traitvec::TraitVec;
    use std::pin::Pin;

    let v: Pin<TraitVec<dyn std::fmt::Debug>> = TraitVec::new();
    {
        let x = v.as_ref().push(1);

        let mut vec = v.as_ref().push(vec![1, 2, 3]);
        vec.push(4);

        assert!(*x == 1);

        assert!(*vec == vec![1, 2, 3, 4]);
    }
    drop(v);
}
