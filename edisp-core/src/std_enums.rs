//! Contains implementation of the `Dispatch` trait for every enum defined in
//! the standard library.
//!
//! Every `std` enum should additionaly have a custom trait entitled `CollectE`
//! (`E` being the name of the enum), which can be used as an iterator adapter.
//!
//! The following list contains every enum available in the standard library
//! and whether if the `Dispatch` trait has been implemented for it:
//!   - `Cow` (done),
//!   - `Entry` (both in `hash_map` and in `btree_map`) (todo),
//!   - `VarError` (todo),
//!   - `SeekFrom` (todo),
//!   - `IpAddr` (todo),
//!   - `SocketAddr` (todo),
//!   - `Bound` (todo),
//!   - `Option` (todo),
//!   - `Component` (todo),
//!   - `Prefix` (todo),
//!   - `Result` (done),
//!   - `TryLockError` (todo),
//!   - `Poll` (todo),
//!
//! Some enums may not benefit from the implementation of `Dispatch` as such,
//! they have been removed from this list.

use std::borrow::Cow;

use crate::prelude::*;

implement_dispatch!(Result<T, E>, Ok(T), Err(E));

/// Allows to collect values from an iterator by dispatching `Ok` variants
/// and `Err` variants in two different containers.
pub trait CollectResult<A, B> {
    /// Collects values and dispatch them.
    fn dispatch_result<C: Default + Extend<A>, D: Default + Extend<B>>(self) -> (C, D);
}

impl<T, E, I: Iterator<Item = Result<T, E>>> CollectResult<T, E> for I {
    fn dispatch_result<C: Default + Extend<T>, D: Default + Extend<E>>(self) -> (C, D) {
        use crate::prelude::*;

        Result::dispatch(self)
    }
}

impl<'a, B: 'a + ToOwned + ?Sized, C, D> Dispatch<(C, D)> for Cow<'a, B>
where
    Self: Sized,
    C: Default + Extend<&'a B>,
    D: Default + Extend<<B as ToOwned>::Owned>,
{
    fn dispatch<I: Iterator<Item = Self>>(iter: I) -> (C, D) {
        let mut c = C::default();
        let mut d = D::default();

        for element in iter {
            match element {
                Cow::Borrowed(v) => c.extend(Some(v)),
                Cow::Owned(v) => d.extend(Some(v)),
            }
        }

        (c, d)
    }
}

/// Allows to collect owned values and borrowed values separately.
///
/// This may be usefull. The first value inside the tuple contains the borrowed
/// data while the second one contains the owned data.
pub trait CollectCow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
{
    /// Collects values and dispatch them.
    fn dispatch_cow<C, D>(self) -> (C, D)
    where
        C: Default + Extend<&'a B>,
        D: Default + Extend<<B as ToOwned>::Owned>;
}

impl<'a, B, I> CollectCow<'a, B> for I
where
    B: 'a + ToOwned + ?Sized,
    I: Iterator<Item = Cow<'a, B>>,
{
    fn dispatch_cow<C, D>(self) -> (C, D)
    where
        C: Default + Extend<&'a B>,
        D: Default + Extend<<B as ToOwned>::Owned>,
    {
        Cow::dispatch(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_result_impl() {
        let i = vec![Ok(42), Err("foo"), Ok(101), Err("bar")].into_iter();
        let (some_oks, some_errs): (Vec<_>, Vec<_>) = i.dispatch_result();

        assert_eq!(some_oks, vec![42, 101],);
        assert_eq!(some_errs, vec!["foo", "bar"],);
    }

    #[test]
    fn collect_cow_impl() {
        let i = vec![Cow::Owned(42), Cow::Borrowed(&-1), Cow::Owned(101)].into_iter();
        let (some_borrowed, some_owned): (Vec<&i8>, Vec<_>) = i.dispatch_cow();

        assert_eq!(some_borrowed, vec![&-1]);
        assert_eq!(some_owned, vec![42, 101]);
    }
}
