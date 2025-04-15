# Crate Documentation

**Version:** 0.3.0

**Format Version:** 43

# Module `better_panic`

`better-panic` gives you pretty backtraces for panics.

It is inspired by Python tracebacks and tries to replicate them as well
as possible.  This is what it looks like:

<img src="https://github.com/mitsuhiko/better-panic/raw/master/screenshot.png">

Some of the code is based on the
[color-backtrace](https://crates.io/crates/color-backtrace) library.

## Usage

The most common way to use it is to invoke the `install` function
which installs a panic handler.  In debug builds the backtrace is shown
automatically, in release builds it's hidden by default.

```
better_panic::install();
```

For more configuration see the `Settings` object.

## Features

- Colorize backtraces to be easier on the eyes
- Show source snippets if source files are found on disk
- Hide all the frames after the panic was already initiated

## Types

### Enum `Verbosity`

Defines how verbose the backtrace is supposed to be.

```rust
pub enum Verbosity {
    Minimal,
    Medium,
    Full,
}
```

#### Variants

##### `Minimal`

Print a small message including the panic payload and the panic location.

##### `Medium`

Everything in `Minimal` and additionally print a backtrace.

##### `Full`

Everything in `Medium` plus source snippets for all backtrace locations.

#### Implementations

##### Methods

- ```rust
  pub fn from_env() -> Self { /* ... */ }
  ```
  Get the verbosity level from the `RUST_BACKTRACE` env variable.

##### Trait Implementations

- **Unpin**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Verbosity) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
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
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Verbosity { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Verbosity) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Verbosity) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Struct `Settings`

Configuration for panic printing.

```rust
pub struct Settings {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Alias for `Settings::default`.

- ```rust
  pub fn debug() -> Self { /* ... */ }
  ```
  Common settings for debugging.

- ```rust
  pub fn auto() -> Self { /* ... */ }
  ```
  In release builds this is `new`, in debug builds this is `debug`.

- ```rust
  pub fn message</* synthetic */ impl Into<String>: Into<String>>(self: Self, message: impl Into<String>) -> Self { /* ... */ }
  ```
  Controls the "greeting" message of the panic.

- ```rust
  pub fn verbosity(self: Self, v: Verbosity) -> Self { /* ... */ }
  ```
  Controls the verbosity level.

- ```rust
  pub fn backtrace_first(self: Self, value: bool) -> Self { /* ... */ }
  ```
  Controls the backtrace position.

- ```rust
  pub fn most_recent_first(self: Self, value: bool) -> Self { /* ... */ }
  ```
  Controls the most-recent-first behavior.

- ```rust
  pub fn lineno_suffix(self: Self, value: bool) -> Self { /* ... */ }
  ```
  Append the line number as suffix to the filename.

- ```rust
  pub fn create_panic_handler(self: Self) -> Box<dyn Fn(&PanicInfo<''_>) + Sync + Send + ''static> { /* ... */ }
  ```
  Consumes the settings and creates a panic handler.

- ```rust
  pub fn install(self: Self) { /* ... */ }
  ```
  Installs the panic handler.

##### Trait Implementations

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Settings { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

## Functions

### Function `install`

Installs the panic handler with `Settings::auto`.

```rust
pub fn install() { /* ... */ }
```

### Function `debug_install`

Installs the panic handler with debug settings.

```rust
pub fn debug_install() { /* ... */ }
```

