# Crate Documentation

**Version:** 0.3.2

**Format Version:** 43

# Module `glob`

Support for matching file paths against Unix shell style patterns.

The `glob` and `glob_with` functions allow querying the filesystem for all
files that match a particular pattern (similar to the libc `glob` function).
The methods on the `Pattern` type provide functionality for checking if
individual paths match a particular pattern (similar to the libc `fnmatch`
function).

For consistency across platforms, and for Windows support, this module
is implemented entirely in Rust rather than deferring to the libc
`glob`/`fnmatch` functions.

# Examples

To print all jpg files in `/media/` and all of its subdirectories.

```rust,no_run
use glob::glob;

for entry in glob("/media/**/*.jpg").expect("Failed to read glob pattern") {
    match entry {
        Ok(path) => println!("{:?}", path.display()),
        Err(e) => println!("{:?}", e),
    }
}
```

To print all files containing the letter "a", case insensitive, in a `local`
directory relative to the current working directory. This ignores errors
instead of printing them.

```rust,no_run
use glob::glob_with;
use glob::MatchOptions;

let options = MatchOptions {
    case_sensitive: false,
    require_literal_separator: false,
    require_literal_leading_dot: false,
};
for entry in glob_with("local/*a*", options).unwrap() {
    if let Ok(path) = entry {
        println!("{:?}", path.display())
    }
}
```

## Types

### Struct `Paths`

An iterator that yields `Path`s from the filesystem that match a particular
pattern.

Note that it yields `GlobResult` in order to report any `IoErrors` that may
arise during iteration. If a directory matches but is unreadable,
thereby preventing its contents from being checked for matches, a
`GlobError` is returned to express this.

See the `glob` function for more details.

```rust
pub struct Paths {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<GlobResult> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
### Struct `GlobError`

A glob iteration error.

This is typically returned when a particular path cannot be read
to determine if its contents match the glob pattern. This is possible
if the program lacks the appropriate permissions, for example.

```rust
pub struct GlobError {
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
  pub fn path(self: &Self) -> &Path { /* ... */ }
  ```
  The Path that the error corresponds to.

- ```rust
  pub fn error(self: &Self) -> &io::Error { /* ... */ }
  ```
  The error in question.

- ```rust
  pub fn into_error(self: Self) -> io::Error { /* ... */ }
  ```
  Consumes self, returning the _raw_ underlying `io::Error`

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn cause(self: &Self) -> Option<&dyn Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Send**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Type Alias `GlobResult`

An alias for a glob iteration result.

This represents either a matched path or a glob iteration error,
such as failing to read a particular directory's contents.

```rust
pub type GlobResult = Result<std::path::PathBuf, GlobError>;
```

### Struct `PatternError`

**Attributes:**

- `#[allow(missing_copy_implementations)]`

A pattern parsing error.

```rust
pub struct PatternError {
    pub pos: usize,
    pub msg: &''static str,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `pos` | `usize` | The approximate character index of where the error occurred. |
| `msg` | `&''static str` | A message describing the error. |

#### Implementations

##### Trait Implementations

- **Send**
- **Freeze**
- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `Pattern`

A compiled Unix shell style pattern.

- `?` matches any single character.

- `*` matches any (possibly empty) sequence of characters.

- `**` matches the current directory and arbitrary
  subdirectories. To match files in arbitrary subdiretories, use
  `**/*`.

  This sequence **must** form a single path component, so both
  `**a` and `b**` are invalid and will result in an error.  A
  sequence of more than two consecutive `*` characters is also
  invalid.

- `[...]` matches any character inside the brackets.  Character sequences
  can also specify ranges of characters, as ordered by Unicode, so e.g.
  `[0-9]` specifies any character between 0 and 9 inclusive. An unclosed
  bracket is invalid.

- `[!...]` is the negation of `[...]`, i.e. it matches any characters
  **not** in the brackets.

- The metacharacters `?`, `*`, `[`, `]` can be matched by using brackets
  (e.g. `[?]`).  When a `]` occurs immediately following `[` or `[!` then it
  is interpreted as being part of, rather then ending, the character set, so
  `]` and NOT `]` can be matched by `[]]` and `[!]]` respectively.  The `-`
  character can be specified inside a character sequence pattern by placing
  it at the start or the end, e.g. `[abc-]`.

```rust
pub struct Pattern {
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
  pub fn new(pattern: &str) -> Result<Self, PatternError> { /* ... */ }
  ```
  This function compiles Unix shell style patterns.

- ```rust
  pub fn escape(s: &str) -> String { /* ... */ }
  ```
  Escape metacharacters within the given string by surrounding them in

- ```rust
  pub fn matches(self: &Self, str: &str) -> bool { /* ... */ }
  ```
  Return if the given `str` matches this `Pattern` using the default

- ```rust
  pub fn matches_path(self: &Self, path: &Path) -> bool { /* ... */ }
  ```
  Return if the given `Path`, when converted to a `str`, matches this

- ```rust
  pub fn matches_with(self: &Self, str: &str, options: MatchOptions) -> bool { /* ... */ }
  ```
  Return if the given `str` matches this `Pattern` using the specified

- ```rust
  pub fn matches_path_with(self: &Self, path: &Path, options: MatchOptions) -> bool { /* ... */ }
  ```
  Return if the given `Path`, when converted to a `str`, matches this

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Access the original glob pattern.

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **RefUnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Pattern { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Pattern) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Pattern) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Pattern { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Self, PatternError> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Pattern) -> bool { /* ... */ }
    ```

- **Send**
### Struct `MatchOptions`

**Attributes:**

- `#[allow(missing_copy_implementations)]`

Configuration options to modify the behaviour of `Pattern::matches_with(..)`.

```rust
pub struct MatchOptions {
    pub case_sensitive: bool,
    pub require_literal_separator: bool,
    pub require_literal_leading_dot: bool,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `case_sensitive` | `bool` | Whether or not patterns should be matched in a case-sensitive manner.<br>This currently only considers upper/lower case relationships between<br>ASCII characters, but in future this might be extended to work with<br>Unicode. |
| `require_literal_separator` | `bool` | Whether or not path-component separator characters (e.g. `/` on<br>Posix) must be matched by a literal `/`, rather than by `*` or `?` or<br>`[...]`. |
| `require_literal_leading_dot` | `bool` | Whether or not paths that contain components that start with a `.`<br>will require that `.` appears literally in the pattern; `*`, `?`, `**`,<br>or `[...]` will not match. This is useful because such files are<br>conventionally considered hidden on Unix systems and it might be<br>desirable to skip them when listing files. |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Constructs a new `MatchOptions` with default field values. This is used

##### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> MatchOptions { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MatchOptions) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &MatchOptions) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MatchOptions { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &MatchOptions) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

## Functions

### Function `glob`

Return an iterator that produces all the `Path`s that match the given
pattern using default match options, which may be absolute or relative to
the current working directory.

This may return an error if the pattern is invalid.

This method uses the default match options and is equivalent to calling
`glob_with(pattern, MatchOptions::new())`. Use `glob_with` directly if you
want to use non-default match options.

When iterating, each result is a `GlobResult` which expresses the
possibility that there was an `IoError` when attempting to read the contents
of the matched path.  In other words, each item returned by the iterator
will either be an `Ok(Path)` if the path matched, or an `Err(GlobError)` if
the path (partially) matched _but_ its contents could not be read in order
to determine if its contents matched.

See the `Paths` documentation for more information.

# Examples

Consider a directory `/media/pictures` containing only the files
`kittens.jpg`, `puppies.jpg` and `hamsters.gif`:

```rust,no_run
use glob::glob;

for entry in glob("/media/pictures/*.jpg").unwrap() {
    match entry {
        Ok(path) => println!("{:?}", path.display()),

        // if the path matched but was unreadable,
        // thereby preventing its contents from matching
        Err(e) => println!("{:?}", e),
    }
}
```

The above code will print:

```ignore
/media/pictures/kittens.jpg
/media/pictures/puppies.jpg
```

If you want to ignore unreadable paths, you can use something like
`filter_map`:

```rust
use glob::glob;
use std::result::Result;

for path in glob("/media/pictures/*.jpg").unwrap().filter_map(Result::ok) {
    println!("{}", path.display());
}
```
Paths are yielded in alphabetical order.

```rust
pub fn glob(pattern: &str) -> Result<Paths, PatternError> { /* ... */ }
```

### Function `glob_with`

Return an iterator that produces all the `Path`s that match the given
pattern using the specified match options, which may be absolute or relative
to the current working directory.

This may return an error if the pattern is invalid.

This function accepts Unix shell style patterns as described by
`Pattern::new(..)`.  The options given are passed through unchanged to
`Pattern::matches_with(..)` with the exception that
`require_literal_separator` is always set to `true` regardless of the value
passed to this function.

Paths are yielded in alphabetical order.

```rust
pub fn glob_with(pattern: &str, options: MatchOptions) -> Result<Paths, PatternError> { /* ... */ }
```

