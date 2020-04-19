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
        > $crate::dispatch::Dispatch<( $( $container_letter, )+ )> for $enum_name< $( $ty_arg, )* >
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
/// use edisp_core::prelude::*;
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
