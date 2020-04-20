//! Dispatch-on-collect for Rust enums.
//!
//! This crate allows to dispatch enums
//! yielded from an iterator, depending on their variants, with no runtime
//! costs.
//!
//! **Note:** This documentation describes what *should* be done, not the
//! current state of the crate. Every feature documented here will be
//! implemented prior first beta release.
//!
//! # Dispatching on `std` enums
//!
//! This crate provides dispatching for enums defined in `std`. Values can be
//! collected in any type that implements both [`Default`] and [`Extend`]
//! traits. This dispatching consists in a trait generated for each enum,
//! which can be called on every `Iterator`, like so:
//!
//! ```
//! use edisp::prelude::*;
//!
//! // Use your regular iterator
//! let iter = vec![
//!     Ok(42),
//!     Ok(0),
//!     Err("User not found"),
//!     Err("System error"),
//! ].into_iter();
//!
//! // Call the correct method, and that's all!
//! let (some_successes, some_errors): (Vec<_>, Vec<_>) = iter.dispatch_result();
//!
//! assert_eq!(some_successes, vec![42, 0]);
//! assert_eq!(some_errors, vec!["User not found", "System error"]);
//! ```
//!
//! # Dispatching on other crate's enums
//!
//! Dispatching code is generated with either `derive` macro or with declarative
//! macro. The first method allows to quickly generate boilerplate without
//! needing to write the enum name and variants twice. The second allows to get
//! rid of the procedural macro dependencies, `syn` and `quote`, and reduces
//! compilation time.
//!
//! Values can then be collected in any type that implements both [`Default`]
//! and [`Extend`] traits.
//!
//! ## Using `derive` macro
//!
//! This crate provides a custom `derive` macro allowing which automatically
//! implements traits required for dispatching, as shown in the following code
//! snippet:
//!
//! ```rust
//! use edisp::prelude::*;
//!
//! #[derive(Dispatch)]
//! enum MyOwnEnum<T> {
//!     Character(char),
//!     Custom(T),
//! }
//!
//! // Practical use-case:
//! // First, create an iterator of `MyOwnEnum<&'static str>`
//! let iter = vec![
//!     MyOwnEnum::Character('λ'),
//!     MyOwnEnum::Custom("horse"),
//!     MyOwnEnum::Custom("manatee"),
//!     MyOwnEnum::Character('!'),
//! ].into_iter();
//!
//! // Then call it
//! let (some_characters, some_strs): (Vec<_>, Vec<_>) = MyOwnEnum::dispatch(iter);
//!
//! // And it does what you expect!
//! assert_eq!(
//!     some_characters,
//!     vec!['λ', '!'],
//! );
//!
//! assert_eq!(
//!     some_strs,
//!     vec!["horse", "manatee"],
//! );
//! ```
//!
//! The custom derive feature can be disabled by disabling `default-features`
//! in the cargo manifest.
//!
//! ## Using declarative macro
//!
//! This crate provides a macro entitled `implement_dispatch`. It allows to
//! generate traits required for dispatching. Everything wraps up like this:
//!
//! ```rust
//! use edisp::prelude::*;
//!
//! enum MyOwnEnum<T> {
//!     Character(char),
//!     Custom(T),
//! }
//!
//! // Implements the required trait (in this case, CollectDispatch2)
//! implement_dispatch!(
//!     MyOwnEnum<T>,
//!     Character(char),
//!     Custom(T),
//! );
//!
//! // Practical use-case:
//! // First, create an iterator of `MyOwnEnum<&'static str>`
//! let iter = vec![
//!     MyOwnEnum::Character('λ'),
//!     MyOwnEnum::Custom("horse"),
//!     MyOwnEnum::Custom("manatee"),
//!     MyOwnEnum::Character('!'),
//! ].into_iter();
//!
//! // Then call it
//! let (some_characters, some_strs): (Vec<_>, Vec<_>) = MyOwnEnum::dispatch(iter);
//!
//! // And it does what you expect!
//! assert_eq!(
//!     some_characters,
//!     vec!['λ', '!'],
//! );
//!
//! assert_eq!(
//!     some_strs,
//!     vec!["horse", "manatee"],
//! );
//! ```
//!
//! [`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
//! [`Extend`]: https://doc.rust-lang.org/std/iter/trait.Extend.html

#![forbid(missing_docs)]

pub mod prelude;

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn simple_derive() {
        #[allow(dead_code)]
        #[derive(Dispatch)]
        enum E {
            Var1(usize),
            Var2(&'static str),
            Var3(()),
        }
    }

    #[test]
    fn derive_with_empty_variant() {
        #[allow(dead_code)]
        #[derive(Dispatch)]
        enum E {
            Nothing,
            Var1(usize),
        }
    }

    #[test]
    fn derive_with_discriminant_and_attributes() {
        #[allow(dead_code)]
        #[derive(Dispatch)]
        enum E {
            A = 0,
            /// Some documentation comments
            B,
        }
    }

    #[test]
    fn derive_with_generics() {
        #[allow(dead_code)]
        #[derive(Dispatch)]
        enum E<'a, T> {
            A(&'a usize),
            B(T),
            C(T),
        }
    }
}
