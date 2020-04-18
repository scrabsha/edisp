# edisp

## Edisp

Dispatch-on-collect for Rust enums. This crate allows to dispatch enums
yielded from an iterator, depending on their variants, with no runtime
costs.

**Note:** This file describes what *should* be done, not the current state
of the crate. Every feature documented here will be implemented prior first
beta release.

## Details

### On `std` enums

This crate provides dispatching for enums defined in `std`. Values can be
collected in any type that implements both
[`Default`](https://doc.rust-lang.org/std/default/trait.Default.html) and
[`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html) traits.
(see below) traits. This dispatching consists in a trait generated for each
enum, which can be called on every `Iterator`, like so:

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
let (some_successes, some_errors): (Vec<_>, Vec<_>) = iter.dispatch_result();

assert_eq!(some_successes, vec![42, 0]);
assert_eq!(some_errors, vec!["User not found", "System error"]);
```

### On other crate's enums

Dispatching code is generated with either `derive` macro or with declarative
macro. The first method allows to quickly generate boilerplate without
needing to write the enum name and variants twice. The second allows to get
rid of the procedural macro dependencies, `syn` and `quote`, and reduce
compilation time.

Values can then be collected in any type that implements both
[`Default`](https://doc.rust-lang.org/std/default/trait.Default.html) and
[`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html) traits.

#### Using `derive` macro

**Note:** This feature is not currently avalaible. It will be implemented
before first beta release.

This crate provides a custom `derive` macro allowing which automatically
implements traits required for dispatching, as shown in the following code
snippet:

```rust
use edisp::prelude::*;

#[derive(Dispatch)]
enum MyOwnEnum<T> {
    Character(char),
    Custom(T),
}

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

**Note:** This feature is not currently implemented, and as such can't be
turned off.

The custom derive feature can be disabled by disabling `derive` feature.

#### Using declarative macro

**Note:** `CollectDispatch2`, `CollectDispatch3`, and so on still have to be
merged toghether.

This crate provides a macro entitled `implement_dispatch`. It allows to
generate traits required for dispatching. Everything wraps up like this:

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

License: MIT OR Apache-2.0
