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
//! collected in any type that implements both [`Default`] and [`Extend`] traits.
//! This dispatching consists in a trait generated for each enum, which can be
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
//! **Note:** This feature is not currently avalaible. It will be implemented
//! before first beta release.
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
//! **Note:** This feature is not currently implemented, and as such can't be
//! turned off.
//!
//! The custom derive feature can be disabled by disabling `derive` feature.
//!
//! ## Using declarative macro
//!
//! **Note:** `CollectDispatch2`, `CollectDispatch3`, and so on still have to be
//! merged toghether.
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

pub mod dispatchers;
pub mod prelude;
pub mod std_enums;

/// Implements a given dispatcher trait for a given enum.
///
/// This macro is meant to be used internally, and should **not** be called
/// by the user. It does not bring any new feature, and won't be faster or
/// whetever.
#[macro_export]
macro_rules! implement_dispatcher_trait {
    (
        $enum_name:ident ( $( $ty_arg:tt ),* $( , )? ),
        $( (
            $variant_name:ident,
            $inner_type:ty,
            $container_name:ident,
            $container_letter:ident
        ) ),+ $( , )?
    ) => {
        impl<
            $( $ty_arg, )*
            $( $container_letter, )+
        > $crate::dispatchers::Dispatch<( $( $container_letter, )+ )> for $enum_name< $( $ty_arg, )* >
        where
        $(
            $container_letter: Default + Extend<$inner_type>,
        )+
        {
            fn dispatch<I>(iter: I) -> ( $( $container_letter, )+ )
            where
                I: Iterator<Item = $enum_name< $( $ty_arg, )* >>,
            {
                $(
                    let mut $container_name = $container_letter::default();
                )+

                use $enum_name::*;
                for element in iter {
                    match element {
                        $(
                            $variant_name(value) => $container_name.extend(Some(value)),
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
    ($_:ident $( < $( $__:tt ),+ $( , )? > )? $( , )? ) => {
        compile_error!("It is not necessary to implement `Dispatch` on an empty enum.");
    };

    ($_:ident $( < $( $__:tt),+ $( , )? > )?,
     $___: ident ($____: ty) $( , )?
    ) => {
        compile_error!("It is not necessary to implement `Dispatch` on a single-variant enum. You can use `map` and then collect instead.");
    };


    ($enum_name:ident $( < $( $ty_arg:tt ),+ $( , )? > )?,
     $variant1_name: ident ($variant1_it: ty),
     $variant2_name: ident ($variant2_it: ty) $( , )?
    ) => {
        implement_dispatcher_trait!(
            $enum_name( $( $( $ty_arg, )+ )? ),
            ($variant1_name, $variant1_it, container_a, A),
            ($variant2_name, $variant2_it, container_b, B),
        );
    };

    ($enum_name:ident $( < $( $ty_arg:tt ),+ $( , )? > )?,
     $variant1_name: ident ($variant1_it: ty),
     $variant2_name: ident ($variant2_it: ty),
     $variant3_name: ident ($variant3_it: ty) $( , )?
    ) => {
        implement_dispatcher_trait!(
            $enum_name( $( $( $ty_arg, )+ )? ),
            ($variant1_name, $variant1_it, container_1, A),
            ($variant2_name, $variant2_it, container_2, B),
            ($variant3_name, $variant3_it, container_3, C),
        );
    };

    ($enum_name:ident $( < $( $ty_arg:tt ),+ $( , )? > )?,
     $variant1_name: ident ($variant1_it: ty),
     $variant2_name: ident ($variant2_it: ty),
     $variant3_name: ident ($variant3_it: ty),
     $variant4_name: ident ($variant4_it: ty) $( , )?
    ) => {
        implement_dispatcher_trait!(
            $enum_name( $( $( $ty_arg, )+ )? ),
            ($variant1_name, $variant1_it, container_1, A),
            ($variant2_name, $variant2_it, container_2, B),
            ($variant3_name, $variant3_it, container_3, C),
            ($variant4_name, $variant4_it, container_4, D),
        );
    };

    ($enum_name:ident $( < $( $ty_arg:tt ),+ $( , )? > )?,
     $variant1_name: ident ($variant1_it: ty),
     $variant2_name: ident ($variant2_it: ty),
     $variant3_name: ident ($variant3_it: ty),
     $variant4_name: ident ($variant4_it: ty),
     $variant5_name: ident ($variant5_it: ty) $( , )?
    ) => {
        implement_dispatcher_trait!(
            $enum_name( $( $( $ty_arg, )+ )? ),
            ($variant1_name, $variant1_it, container_1, A),
            ($variant2_name, $variant2_it, container_2, B),
            ($variant3_name, $variant3_it, container_3, C),
            ($variant4_name, $variant4_it, container_4, D),
            ($variant5_name, $variant5_it, container_5, E),
        );
    };

    ($enum_name:ident $( < $( $ty_arg:tt ),+ $( , )? > )?,
     $variant1_name: ident ($variant1_it: ty),
     $variant2_name: ident ($variant2_it: ty),
     $variant3_name: ident ($variant3_it: ty),
     $variant4_name: ident ($variant4_it: ty),
     $variant5_name: ident ($variant5_it: ty),
     $variant6_name: ident ($variant6_it: ty) $( , )?
    ) => {
        implement_dispatcher_trait!(
            $enum_name( $( $( $ty_arg, )+ )? ),
            ($variant1_name, $variant1_it, container_1, A),
            ($variant2_name, $variant2_it, container_2, B),
            ($variant3_name, $variant3_it, container_3, C),
            ($variant4_name, $variant4_it, container_4, D),
            ($variant5_name, $variant5_it, container_5, E),
            ($variant6_name, $variant6_it, container_6, F),
        );
    };

    ($enum_name:ident $( < $( $ty_arg:tt ),+ $( , )? > )?,
     $variant1_name: ident ($variant1_it: ty),
     $variant2_name: ident ($variant2_it: ty),
     $variant3_name: ident ($variant3_it: ty),
     $variant4_name: ident ($variant4_it: ty),
     $variant5_name: ident ($variant5_it: ty),
     $variant6_name: ident ($variant6_it: ty),
     $variant7_name: ident ($variant7_it: ty) $( , )?
    ) => {
        implement_dispatcher_trait!(
            $enum_name( $( $( $ty_arg, )+ )? ),
            ($variant1_name, $variant1_it, container_1, A),
            ($variant2_name, $variant2_it, container_2, B),
            ($variant3_name, $variant3_it, container_3, C),
            ($variant4_name, $variant4_it, container_4, D),
            ($variant5_name, $variant5_it, container_5, E),
            ($variant6_name, $variant6_it, container_6, F),
            ($variant7_name, $variant7_it, container_7, G),
        );
    };

    ($enum_name:ident $( < $( $ty_arg:tt ),+ $( , )? > )?,
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
            $enum_name( $( $( $ty_arg, )+ )? ),
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a dispatching test.
    ///
    /// This allows to generate tests for the `implement_dispatch` macro. These
    /// tests run on *n*-variants enums, with concrete type and no lifetime
    /// parameter. They are here to check that macro expansion are correct in
    /// the simplest case.
    ///
    /// This macro is used internally to write tests. `edisp` users should not
    /// use it in their code.
    ///
    /// The syntax of this macro proceeds as follow:
    ///   - the name of the generated test,
    ///   - a list of values, separated by comas, surrounded by square braces,
    ///   designating the content of an iterator,
    /// Then, for each variant used:
    ///   - the name of the variant,
    ///   - the type it contains, surrounded by parenthesis,
    ///   - the name of its container (`c1`, `c2`, `c3`...),
    ///   - the `Container` type which will be used to collect values (if
    ///   you're unsure, simply use `Vec<_>`),
    ///   - the expected content of the container.
    macro_rules! implement_and_test_dispatching {
        (
            $test_name:ident,
            // The values which will be yielded by the iterator
            [ $( $input_value:expr ),* $( , )? ],
            // Informations about each variant
            $( (
                // The name of the variant
                $v_name:ident
                // Its inner type
                ($v_type:ty),
                // The name of its container
                $c_name:ident,
                // The type of its container
                $collect_type:ty,
                // The expected content of the container
                $c_content:tt $( , )?
            ) ),* $( , )?
        ) => {
            #[test]
            fn $test_name() {
                use crate::prelude::*;

                // Enum declaration
                enum Enum {
                    $( $v_name($v_type) ),*
                }

                // Allows caller not to specify the enum name for each variant
                use Enum::*;

                // Implements dispatch for the genrated enum
                implement_dispatch!(
                    Enum,
                    $( $v_name($v_type) ),*
                );

                // Testing:
                //   - Creation of the iterator
                let iter = vec![ $( $input_value ),* ].into_iter();
                //   - Dispatching
                let ( $( $c_name ),* ): ( $( $collect_type ),* ) = Enum::dispatch(iter);
                //   - Conformity check
                $(
                    assert_eq!($c_name, $c_content);
                )*
            }
        };
    }

    // Generates a test for a two-variants enum.
    implement_and_test_dispatching! {
        dispatch_enum2,
        [V1(42), V2("manatee")],
        (V1(usize), c1, Vec<_>, [42]),
        (V2(&'static str), c2, Vec<_>, ["manatee"]),
    }

    // Generates a test for a three-variants enum.
    implement_and_test_dispatching! {
        dispatch_enum3,
        [V1(42), V2("manatee"), V3('!')],
        (V1(usize), c1, Vec<_>, [42]),
        (V2(&'static str), c2, Vec<_>, ["manatee"]),
        (V3(char), c3, Vec<_>, ['!']),
    }

    // Generates a test for a four-variants enum.
    implement_and_test_dispatching! {
        dispatch_enum4,
        [V1(42), V2("manatee"), V3('!'), V4(true)],
        (V1(usize), c1, Vec<_>, [42]),
        (V2(&'static str), c2, Vec<_>, ["manatee"]),
        (V3(char), c3, Vec<_>, ['!']),
        (V4(bool), c4, Vec<_>, [true]),
    }

    // Generates a test for a five-variants enum.
    implement_and_test_dispatching! {
        dispatch_enum5,
        [V1(42), V2("manatee"), V3('!'), V4(true), V5(1.618)],
        (V1(usize), c1, Vec<_>, [42]),
        (V2(&'static str), c2, Vec<_>, ["manatee"]),
        (V3(char), c3, Vec<_>, ['!']),
        (V4(bool), c4, Vec<_>, [true]),
        (V5(f64), c5, Vec<_>, [1.618]),
    }

    // Generates a test for a six-variants enum.
    implement_and_test_dispatching! {
        dispatch_enum6,
        [V1(42), V2("manatee"), V3('!'), V4(true), V5(1.618), V6(-1)],
        (V1(usize), c1, Vec<_>, [42]),
        (V2(&'static str), c2, Vec<_>, ["manatee"]),
        (V3(char), c3, Vec<_>, ['!']),
        (V4(bool), c4, Vec<_>, [true]),
        (V5(f64), c5, Vec<_>, [1.618]),
        (V6(isize), c6, Vec<_>, [-1]),
    }

    // Generates a test for a seven-variants enum.
    implement_and_test_dispatching! {
        dispatch_enum7,
        [V1(42), V2("manatee"), V3('!'), V4(true), V5(1.618), V6(-1), V7(101)],
        (V1(usize), c1, Vec<_>, [42]),
        (V2(&'static str), c2, Vec<_>, ["manatee"]),
        (V3(char), c3, Vec<_>, ['!']),
        (V4(bool), c4, Vec<_>, [true]),
        (V5(f64), c5, Vec<_>, [1.618]),
        (V6(isize), c6, Vec<_>, [-1]),
        (V7(u8), c7, Vec<_>, [101]),
    }

    // Generates a test for a eight-variants enum.
    implement_and_test_dispatching! {
        dispatch_enum8,
        [V1(42), V2("manatee"), V3('!'), V4(true), V5(1.618), V6(-1), V7(101), V8('§')],
        (V1(usize), c1, Vec<_>, [42]),
        (V2(&'static str), c2, Vec<_>, ["manatee"]),
        (V3(char), c3, Vec<_>, ['!']),
        (V4(bool), c4, Vec<_>, [true]),
        (V5(f64), c5, Vec<_>, [1.618]),
        (V6(isize), c6, Vec<_>, [-1]),
        (V7(u8), c7, Vec<_>, [101]),
        (V8(char), c8, Vec<_>, ['§']),
    }
}
