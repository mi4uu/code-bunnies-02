# Crate Documentation

**Version:** 0.7.4

**Format Version:** 43

# Module `clap_lex`

Minimal, flexible command-line parser

As opposed to a declarative parser, this processes arguments as a stream of tokens.  As lexing
a command-line is not context-free, we rely on the caller to decide how to interpret the
arguments.

# Examples

```rust
use std::path::PathBuf;
use std::ffi::OsStr;

type BoxedError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
struct Args {
    paths: Vec<PathBuf>,
    color: Color,
    verbosity: usize,
}

#[derive(Debug)]
enum Color {
    Always,
    Auto,
    Never,
}

impl Color {
    fn parse(s: Option<&OsStr>) -> Result<Self, BoxedError> {
        let s = s.map(|s| s.to_str().ok_or(s));
        match s {
            Some(Ok("always")) | Some(Ok("")) | None => {
                Ok(Color::Always)
            }
            Some(Ok("auto")) => {
                Ok(Color::Auto)
            }
            Some(Ok("never")) => {
                Ok(Color::Never)
            }
            Some(invalid) => {
                Err(format!("Invalid value for `--color`, {invalid:?}").into())
            }
        }
    }
}

fn parse_args(
    raw: impl IntoIterator<Item=impl Into<std::ffi::OsString>>
) -> Result<Args, BoxedError> {
    let mut args = Args {
        paths: Vec::new(),
        color: Color::Auto,
        verbosity: 0,
    };

    let raw = clap_lex::RawArgs::new(raw);
    let mut cursor = raw.cursor();
    raw.next(&mut cursor);  // Skip the bin
    while let Some(arg) = raw.next(&mut cursor) {
        if arg.is_escape() {
            args.paths.extend(raw.remaining(&mut cursor).map(PathBuf::from));
        } else if arg.is_stdio() {
            args.paths.push(PathBuf::from("-"));
        } else if let Some((long, value)) = arg.to_long() {
            match long {
                Ok("verbose") => {
                    if let Some(value) = value {
                        return Err(format!("`--verbose` does not take a value, got `{value:?}`").into());
                    }
                    args.verbosity += 1;
                }
                Ok("color") => {
                    args.color = Color::parse(value)?;
                }
                _ => {
                    return Err(
                        format!("Unexpected flag: --{}", arg.display()).into()
                    );
                }
            }
        } else if let Some(mut shorts) = arg.to_short() {
            while let Some(short) = shorts.next_flag() {
                match short {
                    Ok('v') => {
                        args.verbosity += 1;
                    }
                    Ok('c') => {
                        let value = shorts.next_value_os();
                        args.color = Color::parse(value)?;
                    }
                    Ok(c) => {
                        return Err(format!("Unexpected flag: -{c}").into());
                    }
                    Err(e) => {
                        return Err(format!("Unexpected flag: -{}", e.to_string_lossy()).into());
                    }
                }
            }
        } else {
            args.paths.push(PathBuf::from(arg.to_value_os().to_owned()));
        }
    }

    Ok(args)
}

let args = parse_args(["bin", "--hello", "world"]);
println!("{args:?}");
```

## Types

### Struct `RawArgs`

Command-line arguments

```rust
pub struct RawArgs {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn from_args() -> Self { /* ... */ }
  ```

- ```rust
  pub fn new</* synthetic */ impl Into<OsString>: Into<OsString>, /* synthetic */ impl IntoIterator<Item = impl Into<OsString>>: IntoIterator<Item = impl Into<OsString>>>(iter: impl IntoIterator<Item = impl Into<OsString>>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn cursor(self: &Self) -> ArgCursor { /* ... */ }
  ```
  Create a cursor for walking the arguments

- ```rust
  pub fn next(self: &Self, cursor: &mut ArgCursor) -> Option<ParsedArg<''_>> { /* ... */ }
  ```
  Advance the cursor, returning the next [`ParsedArg`]

- ```rust
  pub fn next_os(self: &Self, cursor: &mut ArgCursor) -> Option<&OsStr> { /* ... */ }
  ```
  Advance the cursor, returning a raw argument value.

- ```rust
  pub fn peek(self: &Self, cursor: &ArgCursor) -> Option<ParsedArg<''_>> { /* ... */ }
  ```
  Return the next [`ParsedArg`]

- ```rust
  pub fn peek_os(self: &Self, cursor: &ArgCursor) -> Option<&OsStr> { /* ... */ }
  ```
  Return a raw argument value.

- ```rust
  pub fn remaining(self: &Self, cursor: &mut ArgCursor) -> impl Iterator<Item = &OsStr> { /* ... */ }
  ```
  Return all remaining raw arguments, advancing the cursor to the end

- ```rust
  pub fn seek(self: &Self, cursor: &mut ArgCursor, pos: SeekFrom) { /* ... */ }
  ```
  Adjust the cursor's position

- ```rust
  pub fn insert</* synthetic */ impl Into<OsString>: Into<OsString>, /* synthetic */ impl IntoIterator<Item = impl Into<OsString>>: IntoIterator<Item = impl Into<OsString>>>(self: &mut Self, cursor: &ArgCursor, insert_items: impl IntoIterator<Item = impl Into<OsString>>) { /* ... */ }
  ```
  Inject arguments before the [`RawArgs::next`]

- ```rust
  pub fn is_end(self: &Self, cursor: &ArgCursor) -> bool { /* ... */ }
  ```
  Any remaining args?

##### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RawArgs { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RawArgs) -> bool { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(val: I) -> Self { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> RawArgs { /* ... */ }
    ```

### Struct `ArgCursor`

Position within [`RawArgs`]

```rust
pub struct ArgCursor {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArgCursor) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArgCursor) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArgCursor) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArgCursor { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Struct `ParsedArg`

Command-line Argument

```rust
pub struct ParsedArg<''s> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Argument is length of 0

- ```rust
  pub fn is_stdio(self: &Self) -> bool { /* ... */ }
  ```
  Does the argument look like a stdio argument (`-`)

- ```rust
  pub fn is_escape(self: &Self) -> bool { /* ... */ }
  ```
  Does the argument look like an argument escape (`--`)

- ```rust
  pub fn is_negative_number(self: &Self) -> bool { /* ... */ }
  ```
  Does the argument look like a negative number?

- ```rust
  pub fn to_long(self: &Self) -> Option<(Result<&str, &OsStr>, Option<&OsStr>)> { /* ... */ }
  ```
  Treat as a long-flag

- ```rust
  pub fn is_long(self: &Self) -> bool { /* ... */ }
  ```
  Can treat as a long-flag

- ```rust
  pub fn to_short(self: &Self) -> Option<ShortFlags<''_>> { /* ... */ }
  ```
  Treat as a short-flag

- ```rust
  pub fn is_short(self: &Self) -> bool { /* ... */ }
  ```
  Can treat as a short-flag

- ```rust
  pub fn to_value_os(self: &Self) -> &OsStr { /* ... */ }
  ```
  Treat as a value

- ```rust
  pub fn to_value(self: &Self) -> Result<&str, &OsStr> { /* ... */ }
  ```
  Treat as a value

- ```rust
  pub fn display(self: &Self) -> impl std::fmt::Display + ''_ { /* ... */ }
  ```
  Safely print an argument that may contain non-UTF8 content

##### Trait Implementations

- **Unpin**
- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ParsedArg<''s>) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ParsedArg<''s>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ParsedArg<''s>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParsedArg<''s> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `ShortFlags`

Walk through short flags within a [`ParsedArg`]

```rust
pub struct ShortFlags<''s> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn advance_by(self: &mut Self, n: usize) -> Result<(), usize> { /* ... */ }
  ```
  Move the iterator forward by `n` short flags

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  No short flags left

- ```rust
  pub fn is_negative_number(self: &Self) -> bool { /* ... */ }
  ```
  Does the short flag look like a number

- ```rust
  pub fn next_flag(self: &mut Self) -> Option<Result<char, &''s OsStr>> { /* ... */ }
  ```
  Advance the iterator, returning the next short flag on success

- ```rust
  pub fn next_value_os(self: &mut Self) -> Option<&''s OsStr> { /* ... */ }
  ```
  Advance the iterator, returning everything left as a value

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ShortFlags<''s> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **Unpin**
## Re-exports

### Re-export `SeekFrom`

```rust
pub use std::io::SeekFrom;
```

### Re-export `OsStrExt`

```rust
pub use ext::OsStrExt;
```

