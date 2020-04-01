//! Allows to separate values in iterator by variant (especially for results)
//!
//! # Edisp
//!
//! Dispatch-on-collect for Rust enums. This crate allows to dispatch enums
//! yielded from an iterator, depending on their variants, with no runtime
//! costs.
//!
//! # Details
//!
//! ## On `std` enums
//!
//! **Note:** This paragraph describes what *should* be done, not the current
//! state of the crate. As of today, dispatching is implemented for `Result`.
//!
//! This crate provides dispatching for enums defined in `std`. Values can be
//! collected in any type that implement `Container` (see below). This
//! dispatching consists in a trait generated for each enum, which can be
//! called on every `Iterator`, like so:
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
//! let (some_success, some_errors): (Vec<_>, Vec<_>) = iter.dispatch_result();
//!
//! assert_eq!(some_success, vec![42, 0]);
//! assert_eq!(some_errors, vec!["User not found", "System error"]);
//! ```
//!
//! ## On other crate's enums
//!
//! This crate provides traits entitled `CollectDispatch2`, `CollectDispatch3`,
//! and so on. These traits are implemented on-demand with a specific macro.
//! Values can be collected in any type that implement `Container` (see below).
//!
//! Everything wraps up like this:
//!
//! ```
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
//! ## The `Container` trait
//!
//! Values contained in enum variants are collected on objects which implement a
//! `Container` trait. This trait is fairly simple, and *may be* implemented by
//! every collection in the standard library. Additionaly, this trait is made
//! public so that other rustaceans can implement it for their own collections.
//!
//! So far, this traits consists of two methods:
//!   - creating a new `container`,
//!   - adding an element to it.
//!
//! There can however be some specific requirements. For instance, `HashMap`
//! may implement `Collect`, but only for `(K, V)` tuples.

#![forbid(missing_docs)]

pub mod container;
pub mod dispatchers;
pub mod prelude;

use container::Container;

/// Implements a given dispatcher trait for a given enum.
///
/// This macro is meant to be used internally, and should **not** be called
/// by the user. It does not bring any new feature, and won't be faster or
/// whetever.
#[macro_export]
macro_rules! implement_dispatcher_trait {
    (
        $enum_name:ident< $( $ty_arg:ident, )* >,
        $trait_name:ident,
        $( (
            $variant_name:ident,
            $inner_type:ty,
            $container_name:ident,
            $container_letter:ident
        ), )+
    ) => {
        impl< $( $ty_arg, )* > $crate::dispatchers::$trait_name< $( $inner_type, )+ > for $enum_name< $( $ty_arg, )* > {
            fn dispatch< $( $container_letter, )* I>(iter:I) -> ( $( $container_letter, )+ )
                where $( $container_letter: $crate::container::Container<$inner_type>, )+
                      I: Iterator<Item = $enum_name< $( $ty_arg, )* >>
            {
                $(
                    let mut $container_name = $container_letter::new();
                )+

                use $enum_name::*;

                for element in iter {
                    match element {
                        $(
                            $variant_name(value) => $container_name.add_element(value),
                        )+
                    }
                }

                (
                    $(
                        $container_name,
                    )+
                )
            }
        }
    }
}

/// Implements the dispatch for an enum.
///
/// ```
/// use edisp::prelude::*;
///
/// enum MyResult<T, E> {
///     MyOk(T),
///     MyErr(E)
/// }
///
/// implement_dispatch!(MyResult<T, E>, MyOk(T), MyErr(E));
///
/// enum MyEnum {
///     Integer(u8),
///     Other(char),
/// }
///
/// implement_dispatch!(MyEnum, Integer(u8), Other(char));
/// ```
#[macro_export]
macro_rules! implement_dispatch {
    ($_:ident $( $( $__:ident ),+ > )? $( , )? ) => {
        compile_error!("It is not necessary to implement `Dispatch` on an empty enum.");
    };

    ($_:ident $( < $($__:ident),+> )?,
     $___: ident ($____: ty) $( , )?
    ) => {
        compile_error!("It is not necessary to implement `Dispatch` on a single-variant enum. You can use `map` and then collect instead.");
    };


    ($enum_name:ident $( < $($ty_arg:ident),+> )?,
     $variant1_name: ident ($variant1_it: ty),
     $variant2_name: ident ($variant2_it: ty) $( , )?
    ) => {
        implement_dispatcher_trait!(
            $enum_name< $( $( $ty_arg, )+ )?>,
            CollectDispatch2,
            ($variant1_name, $variant1_it, container_a, A),
            ($variant2_name, $variant2_it, container_b, B),
        );
    };

    ($enum_name:ident $( < $($ty_arg:ident),+> )?,
     $variant1_name: ident ($variant1_it: ty),
     $variant2_name: ident ($variant2_it: ty),
     $variant3_name: ident ($variant3_it: ty) $( , )?
    ) => {
        implement_dispatcher_trait!(
            $enum_name< $( $( $ty_arg, )+ )?>,
            CollectDispatch3,
            ($variant1_name, $variant1_it, container_1, A),
            ($variant2_name, $variant2_it, container_2, B),
            ($variant3_name, $variant3_it, container_3, C),
        );
    };

    ($enum_name:ident $( < $($ty_arg:ident),+> )?,
     $variant1_name: ident ($variant1_it: ty),
     $variant2_name: ident ($variant2_it: ty),
     $variant3_name: ident ($variant3_it: ty),
     $variant4_name: ident ($variant4_it: ty) $( , )?
    ) => {
        implement_dispatcher_trait!(
            $enum_name< $( $( $ty_arg, )+ )?>,
            CollectDispatch4,
            ($variant1_name, $variant1_it, container_1, A),
            ($variant2_name, $variant2_it, container_2, B),
            ($variant3_name, $variant3_it, container_3, C),
            ($variant4_name, $variant4_it, container_4, D),
        );
    };

    ($enum_name:ident $( < $($ty_arg:ident),+> )?,
     $variant1_name: ident ($variant1_it: ty),
     $variant2_name: ident ($variant2_it: ty),
     $variant3_name: ident ($variant3_it: ty),
     $variant4_name: ident ($variant4_it: ty),
     $variant5_name: ident ($variant5_it: ty) $( , )?
    ) => {
        implement_dispatcher_trait!(
            $enum_name< $( $( $ty_arg, )+ )?>,
            CollectDispatch5,
            ($variant1_name, $variant1_it, container_1, A),
            ($variant2_name, $variant2_it, container_2, B),
            ($variant3_name, $variant3_it, container_3, C),
            ($variant4_name, $variant4_it, container_4, D),
            ($variant5_name, $variant5_it, container_5, E),
        );
    };

    ($enum_name:ident $( < $($ty_arg:ident),+> )?,
     $variant1_name: ident ($variant1_it: ty),
     $variant2_name: ident ($variant2_it: ty),
     $variant3_name: ident ($variant3_it: ty),
     $variant4_name: ident ($variant4_it: ty),
     $variant5_name: ident ($variant5_it: ty),
     $variant6_name: ident ($variant6_it: ty) $( , )?
    ) => {
        implement_dispatcher_trait!(
            $enum_name< $( $( $ty_arg, )+ )?>,
            CollectDispatch6,
            ($variant1_name, $variant1_it, container_1, A),
            ($variant2_name, $variant2_it, container_2, B),
            ($variant3_name, $variant3_it, container_3, C),
            ($variant4_name, $variant4_it, container_4, D),
            ($variant5_name, $variant5_it, container_5, E),
            ($variant6_name, $variant6_it, container_6, F),
        );
    };

    ($enum_name:ident $( < $($ty_arg:ident),+> )?,
     $variant1_name: ident ($variant1_it: ty),
     $variant2_name: ident ($variant2_it: ty),
     $variant3_name: ident ($variant3_it: ty),
     $variant4_name: ident ($variant4_it: ty),
     $variant5_name: ident ($variant5_it: ty),
     $variant6_name: ident ($variant6_it: ty),
     $variant7_name: ident ($variant7_it: ty) $( , )?
    ) => {
        implement_dispatcher_trait!(
            $enum_name< $( $( $ty_arg, )+ )?>,
            CollectDispatch7,
            ($variant1_name, $variant1_it, container_1, A),
            ($variant2_name, $variant2_it, container_2, B),
            ($variant3_name, $variant3_it, container_3, C),
            ($variant4_name, $variant4_it, container_4, D),
            ($variant5_name, $variant5_it, container_5, E),
            ($variant6_name, $variant6_it, container_6, F),
            ($variant7_name, $variant7_it, container_7, G),
        );
    };

    ($enum_name:ident $( < $($ty_arg:ident),+> )?,
     $variant1_name: ident ($variant1_it: ty),
     $variant2_name: ident ($variant2_it: ty),
     $variant3_name: ident ($variant3_it: ty),
     $variant4_name: ident ($variant4_it: ty),
     $variant5_name: ident ($variant5_it: ty),
     $variant6_name: ident ($variant6_it: ty),
     $variant7_name: ident ($variant7_it: ty),
     $variant8_name: ident ($variant8_it: ty) $( , )?
    ) => {
        implement_dispatcher_trait!(
            $enum_name< $( $( $ty_arg, )+ )?>,
            CollectDispatch8,
            ($variant1_name, $variant1_it, container_1, A),
            ($variant2_name, $variant2_it, container_2, B),
            ($variant3_name, $variant3_it, container_3, C),
            ($variant4_name, $variant4_it, container_4, D),
            ($variant5_name, $variant5_it, container_5, E),
            ($variant6_name, $variant6_it, container_6, F),
            ($variant7_name, $variant7_it, container_7, G),
            ($variant8_name, $variant8_it, container_8, H),
        );
    };
}

implement_dispatch!(Result<T, E>, Ok(T), Err(E));

/// Allows to collect values from an iterator by dispatching `Ok` variants
/// and `Err` variants to two different containers.
pub trait CollectResult<A, B> {
    /// Collects values and dispatch them.
    fn dispatch_result<C: Container<A>, D: Container<B>>(self) -> (C, D);
}

impl<T, E, I: Iterator<Item = Result<T, E>>> CollectResult<T, E> for I {
    fn dispatch_result<C: Container<T>, D: Container<E>>(self) -> (C, D) {
        use crate::prelude::*;

        Result::dispatch(self)
    }
}
