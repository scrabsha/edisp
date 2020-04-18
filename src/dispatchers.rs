//! Contains dispatchers for various kind of n-variants enums.
//!
//! This crate contains several versions of the `CollectDispatch*` trait. Each
//! trait correspond to an enum with a given number of members. For instance,
//! `CollectDispatchFour` allows to collect and dispatch for four-membered
//! tuples.
//!
//! This kind of implementation is not as fancy as expected, but it is required
//! because the arity of the returned tuple of `Container` is equal to the
//! number of tuple variants.
//!
//! No dispatcher is implemented for single-variant enum because guessing which
//! variant is used is trivial in this specific case.

/// Generates a n-variants enum dispatcher trait.
///
/// The first argument is the name of the generated trait. The nomenclature is
/// to name it `CollectDispatchn`, where n is the number of variants of the
/// tuple.
///
/// Then there is a list of pair of letters with parenthesis before and after
/// them, separated by commas. The number of pairs is the number of enum
/// variants. It should be ensured as much as possible that each letter is not
/// repeated.
///
/// The first letter of a pair designates a container which is able to hold
/// a value whose type is the second letter. These letters have to be written
/// by the programmer because the compiler does not allow to create arbitrary
/// label in macros.
///
/// # Example
///
/// The following code snippet declares a trait called `CollectDispatchTest`,
/// which allows to dispatch two-variants enums.
///
/// ```
/// use edisp::create_n_collect_dispatcher;
///
/// create_n_collect_dispatcher!(CollectDispatchTest, (A, B), (C, D));
/// ```
#[macro_export]
macro_rules! create_n_collect_dispatcher {
    ($name:ident, $( ($high_letter:ident, $low_letter:ident) ),+ ) => {
        /// A dispatcher for n-variants enum.
        ///
        /// Note that due to limitations of the Rust compiler, it is not
        /// possible to replace "n" written above with the correct number.
        /// Feel free to mentaly replace it with the last word in the trait
        /// name (ScreamingCamelCase is your friend).
        pub trait $name<$( $high_letter, )+> where Self: Sized {
            /// Performs dispatching.
            fn dispatch<$( $low_letter: Default + Extend<$high_letter>, )+ I: Iterator<Item = Self>>(iter: I) -> ( $( $low_letter, )+ );
        }
    }
}

create_n_collect_dispatcher!(CollectDispatch2, (L, A), (M, B));
create_n_collect_dispatcher!(CollectDispatch3, (L, A), (M, B), (N, C));
create_n_collect_dispatcher!(CollectDispatch4, (L, A), (M, B), (N, C), (O, D));
create_n_collect_dispatcher!(CollectDispatch5, (L, A), (M, B), (N, C), (O, D), (P, E));
create_n_collect_dispatcher!(
    CollectDispatch6,
    (L, A),
    (M, B),
    (N, C),
    (O, D),
    (P, E),
    (Q, F)
);
create_n_collect_dispatcher!(
    CollectDispatch7,
    (L, A),
    (M, B),
    (N, C),
    (O, D),
    (P, E),
    (Q, F),
    (R, G)
);
create_n_collect_dispatcher!(
    CollectDispatch8,
    (L, A),
    (M, B),
    (N, C),
    (O, D),
    (P, E),
    (Q, F),
    (R, G),
    (S, H)
);
