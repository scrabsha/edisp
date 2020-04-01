//! Defines the behavior of a container.
//!
//! A container is any type that can hold values of a given type. This
//! includes vectors and any type defined in `std::collections`.
//!
//! Structures implementing this trait must be able to produce an empty
//! container (via `new`) and to add an element (via `add_element`). In some
//! situations, it may be required that the inner type to be a tuple.

/// Defines the behavior of a container.
///
/// A container is any type that can hold values of a given type. This
/// includes vectors and any type defined in `std::collections`.
///
/// Structures implementing this trait must be able to produce an empty
/// container (via `new`) and to add an element (via `add_element`).
///
/// There can be some requirements for `T`. For example, in the case of
/// `HashMap`, `T` must be a tuple of key-value pair.
pub trait Container<T> {
    /// Returns an empty container.
    fn new() -> Self;

    /// Adds an element to the container.
    fn add_element(&mut self, element: T);
}

impl<T> Container<T> for Vec<T> {
    fn new() -> Self {
        Vec::new()
    }

    fn add_element(&mut self, element: T) {
        self.push(element);
    }
}
