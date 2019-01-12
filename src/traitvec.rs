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
    inner: UnsafeCell<Vec<Box<T>>>,
    _pinned: PhantomPinned,
}

impl<T: ?Sized> InnerVec<T> {
    fn new() -> Self {
        Self {
            inner: UnsafeCell::new(Vec::new()),

            // This struct may not move,
            // as long as there is still a reference returned from push() alive.
            _pinned: PhantomPinned,
        }
    }
    fn as_reference<'s>(self: Pin<&'s Self>) -> Pin<&'s Vec<Box<T>>> {
        Pin::new(unsafe { &*self.inner.get() })
    }

    fn as_mutreference<'s>(self: Pin<&'s mut Self>) -> Pin<&'s mut Vec<Box<T>>> {
        Pin::new(unsafe { &mut *self.inner.get() })
    }

    /// Pushes any type `U` implementing trait `T`, returning a mutable reference to `U`.
    pub fn push<'s, U>(self: Pin<&'s Self>, item: U) -> Pin<&'s mut U>
    where
        U: Unsize<T>,
    {
        let boxed = Box::new(item);

        unsafe {
            let raw_ptr = Box::into_raw(boxed);

            let ret: &mut U = &mut *raw_ptr;

            let boxed = Box::from_raw(raw_ptr);

            (&mut *(self.inner.get())).push(boxed);

            Pin::new_unchecked(ret)
        }
    }

    /// Returns an Iterator with shared access to the trait object `T`
    pub fn iter<'s>(self: Pin<&'s Self>) -> impl Iterator<Item = &'s T> {
        Pin::get_ref(self.as_reference()).iter().map(|boxed| &**boxed)
    }

    /// Returns an Iterator with mutable access to the trait object `T`
    pub fn iter_mut<'s>(self: Pin<&'s mut Self>) -> impl Iterator<Item = &'s mut T> {
        Pin::get_mut(self.as_mutreference()).iter_mut().map(|boxed| &mut **boxed)
    }

    /// Returns a draining Iterator that removes the specified range in the vector,
    /// and yields the removed items.
    pub fn drain<'s, R>(self: Pin<&'s mut Self>, range: R) -> impl Iterator<Item = Box<T>> + 's
    where
        R: std::ops::RangeBounds<usize>, 
    {
        Pin::get_mut(self.as_mutreference()).drain(range)
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
        &Pin::get_ref(self.as_ref().as_reference())[idx]
    }
}

impl<T: ?Sized> std::ops::IndexMut<usize> for Pin<TraitVec<T>> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut Pin::get_mut(self.as_mut().as_mutreference())[idx]
    }
}
