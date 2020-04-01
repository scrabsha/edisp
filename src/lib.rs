//! Allows to separate values in iterator by variant (especially for results)
//!
//! # Description
//!
//! This crate explores a new way to `collect` values from an `Iterator` by
//! dispatching it based on the variant used. It allows to quickly collect
//! different variants of the same enum into different collections. The most
//! obvious use case is to filter success and errors in iterator of results,
//! and collect each errors and each success in separate collections whereas,
//! with `collect` from `std`, the collecting process stops when the first
//! error is encountered.
//!
//! # Example
//!
//! This example shows how to implement the dispatching for a user-defined
//! enum.
//!
//! ```
//! use resep::prelude::*;
//!
//! enum CustomEnum<T> {
//!     Integer(u8),
//!     Custom(T),
//!     Char(char),
//! }
//!
//! // Implements required traits using a macro. Neat!
//! implement_dispatch!(CustomEnum<T>, Integer(u8), Custom(T), Char(char));
//!
//! // A practical use case.
//! let an_iterator = vec![
//!     CustomEnum::Custom("doggo"),
//!     CustomEnum::Integer(42),
//!     CustomEnum::Char('z'),
//!     CustomEnum::Char('w'),
//!     CustomEnum::Custom("horse"),
//! ].into_iter();
//!
//! let (some_integers, some_strs, some_chars): (Vec<_>, Vec<_>, Vec<_>) = CustomEnum::dispatch(an_iterator);
//!
//! assert_eq!(some_integers, vec![42]);
//! assert_eq!(some_strs, vec!["doggo", "horse"]);
//! assert_eq!(some_chars, vec!['z', 'w']);
//! ```

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
/// use resep::prelude::*;
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
    fn collect_result<C: Container<A>, D: Container<B>>(self) -> (C, D);
}

impl<T, E, I: Iterator<Item = Result<T, E>>> CollectResult<T, E> for I {
    fn collect_result<C: Container<T>, D: Container<E>>(self) -> (C, D) {
        use crate::prelude::*;

        Result::dispatch(self)
    }
}

