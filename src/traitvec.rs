use std::{
    cell::UnsafeCell,
    marker::{PhantomPinned, Unsize},
    ops::{Deref, DerefMut},
    pin::Pin,
};

/// This struct holds a Vector of trait objects `T`.
/// 
/// No instance of this struct can be made by a method,
/// use [`TraitVec`] instead.
pub struct InnerVec<T: ?Sized> {
    pub inner: UnsafeCell<Vec<Box<T>>>,
    pub _pinned: PhantomPinned,
}

impl<T: ?Sized> InnerVec<T> {
    fn new() -> Self {
        Self {
            inner: UnsafeCell::new(Vec::new()),
            _pinned: PhantomPinned,
        }
    }
    fn as_reference<'s>(self: Pin<&'s Self>) -> &'s Vec<Box<T>> {
        unsafe { &*self.inner.get() }
    }

    fn as_mutreference<'s>(self: Pin<&'s mut Self>) -> &'s mut Vec<Box<T>> {
        unsafe { &mut *self.inner.get() }
    }

    /// Pushes any type `U` implementing trait `T`, returning a mutable reference to `U`.
    pub fn push<'s, U>(self: Pin<&'s mut Self>, item: U) -> &'s mut U
    where
        U: Unsize<T>,
    {
        let boxed = Box::new(item);

        unsafe {
            let raw_ptr = Box::into_raw(boxed);

            let ret: &mut U = &mut *raw_ptr;

            let boxed = Box::from_raw(raw_ptr);

            (&mut *(self.inner.get())).push(boxed);

            ret
        }
    }

    /// Returns an Iterator with shared access to the trait object `T`
    pub fn iter<'a>(self: Pin<&'a Self>) -> impl Iterator<Item = &'a T> {
        self.as_reference().iter().map(|boxed| &**boxed)
    }

    /// Returns an Iterator with mutable access to the trait object `T`
    pub fn iter_mut<'a>(self: Pin<&'a mut Self>) -> impl Iterator<Item = &'a mut T> {
        self.as_mutreference().iter_mut().map(|boxed| &mut **boxed)
    }
}

pub struct TraitVec<T: ?Sized> {
    inner: InnerVec<T>,
}

impl<T: ?Sized> TraitVec<T> {
    /// Constructs a new `TraitVec`, wrapped within a [`Pin`].
    /// 
    /// The Pin is used to guarantee that the underlying [`InnerVec`] isn't moved.
    /// 
    /// Methods of the [`InnerVec`], like [`InnerVec::push`],
    /// can be accessed trough [`Pin::as_mut`] and [`Pin::as_ref`].
    pub fn new() -> Pin<Self> {
        unsafe {
            Pin::new_unchecked(TraitVec {
                inner: InnerVec::new(),
            })
        }
    }
}

impl<T: ?Sized> Deref for TraitVec<T> {
    type Target = InnerVec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: ?Sized> DerefMut for TraitVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: ?Sized> std::ops::Index<usize> for Pin<TraitVec<T>> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.as_ref().as_reference()[idx]
    }
}

impl<T: ?Sized> std::ops::IndexMut<usize> for Pin<TraitVec<T>> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.as_mut().as_mutreference()[idx]
    }
}
