//! A dispatch trait for various kind of n-variants enums.
//!
//! This crate contains the trait `Dispatch` which defines the prototype of the
//! `dispatch` associated function. It is very general and fits for every enum,
//! regardless the number of variants it contains. This trait should however be
//! implemented for enums with two variants or more.

/// A dispatcher trait.
///
/// This trait is general enough to be usable on every enum, regardless of the
/// number of variants it has.
///
/// The generic type `O` should be a tuple whose arity is equal to the number
/// of variants of the implementor, and should contain only types which
/// implement `Default` and `Extend`.
pub trait Dispatch<O>
where
    Self: Sized,
{
    /// Performs dispatching.
    fn dispatch<I: Iterator<Item = Self>>(iter: I) -> O;
}
