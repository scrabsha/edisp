# Edisp

Dispatch-on-collect for Rust enums. This crate allows to dispatch enums yielded
from an iterator, depending on their variants, with no runtime costs.

# Details

## On `std` enums

**Note:** This paragraph describes what *should* be done, not the current state
of the crate. As of today, dispatching is implemented for `Result`.

This crate provides dispatching for enums defined in `std`. Values can be
collected in any type that implement `Container` (see below). This dispatching
consists in a trait generated for each enum, which can be called on every
`Iterator`, like so:

```rust
use edisp::prelude::*;

// Use your regular iterator
let iter = vec![
    Ok(42),
    Ok(0),
    Err("User not found"),
    Err("System error"),
].into_iter();

// Call the correct method, and that's all!
let (some_success, some_errors): (Vec<_>, Vec<_>) = iter.dispatch_result();

assert_eq!(some_success, vec![42, 0]);
assert_eq!(some_errors, vec!["User not found", "System error"]);
```

## On other crate's enums

This crate provides traits entitled `CollectDispatch2`, `CollectDispatch3`, and
so on. These traits are implemented on-demand with a specific macro. Values can
be collected in any type that implement `Container` (see below).

Everything wraps up like this:

```rust
use edisp::prelude::*;

enum MyOwnEnum<T> {
    Character(char),
    Custom(T),
}

// Implements the required trait (in this case, CollectDispatch2)
implement_dispatch!(
    MyOwnEnum<T>,
    Character(char),
    Custom(T),
);

// Practical use-case:
// First, create an iterator of `MyOwnEnum<&'static str>`
let iter = vec![
    MyOwnEnum::Character('λ'),
    MyOwnEnum::Custom("horse"),
    MyOwnEnum::Custom("manatee"),
    MyOwnEnum::Character('!'),
].into_iter();

// Then call it
let (some_characters, some_strs): (Vec<_>, Vec<_>) = MyOwnEnum::dispatch(iter);

// And it does what you expect!
assert_eq!(
    some_characters,
    vec!['λ', '!'],
);

assert_eq!(
    some_strs,
    vec!["horse", "manatee"],
);
```


## The `Container` trait

Values contained in enum variants are collected on objects which implement a
`Container` trait. This trait is fairly simple, and *may be* implemented by
every collection in the standard library. Additionaly, this trait is made
public so that other rustaceans can implement it for their own collections.

So far, this traits consists of two methods:
  - creating a new `container`,
  - adding an element to it.

There can however be some specific requirements. For instance, `HashMap` may
implement `Collect`, but only for `(K, V)` tuples.

