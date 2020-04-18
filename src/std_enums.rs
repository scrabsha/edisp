//! Contains implementation of the `CollectDispatch` trait for every enum
//! defined in the standard library.
//!
//! Every `std` enum should additionaly have a custom trait entitled `CollectE`
//! (`E` being the name of the enum), which can be used as an iterator adapter.

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
