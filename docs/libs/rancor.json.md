# Crate Documentation

**Version:** 0.1.0

**Format Version:** 43

# Module `rancor`

# rancor

rancor provides scalable and efficient error handling without using type
composition. This makes it best-suited for situations where:

- Programmatic error introspection is not useful
- Functions may error, but succeed most of the time
- Errors should provide as much useful detail as possible when emitted
- Use cases include both `no_std` and targets with support for `std`

## Features

- `alloc`: Provides the [`BoxedError`] type. Enabled by default.

## Types

### Struct `Strategy`

Equips a type with a `Fallible` implementation that chooses `E` as its error
type.

# Example

```
use rancor::{Failure, Fallible, Strategy};

trait Print<E = <Self as Fallible>::Error> {
    fn print(&self, message: &str) -> Result<(), E>;
}

impl<T: Print<E> + ?Sized, E> Print<E> for Strategy<T, E> {
    fn print(&self, message: &str) -> Result<(), E> {
        T::print(self, message)
    }
}

struct StdOut;

impl<E> Print<E> for StdOut {
    fn print(&self, message: &str) -> Result<(), E> {
        println!("{message}");
        Ok(())
    }
}

Strategy::<_, Failure>::wrap(&mut StdOut)
    .print("hello world")
    .unwrap();
```

```rust
pub struct Strategy<T: ?Sized, E> {
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
  pub fn wrap(inner: &mut T) -> &mut Self { /* ... */ }
  ```
  Wraps the given mutable reference, returning a mutable reference to a

##### Trait Implementations

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Receiver**
- **RefUnwindSafe**
- **Unpin**
- **Pointee**
- **Fallible**
### Enum `Panic`

An error type that does not occupy any space, panicking on creation instead.

Because panicking occurs immediately upon creation, this error type will not
print any additional trace information.

```rust
pub enum Panic {
}
```

#### Variants

#### Implementations

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Pointee**
- **UnwindSafe**
- **Sync**
- **Error**
- **Eq**
- **RefUnwindSafe**
- **Never**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Panic) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Trace**
  - ```rust
    fn trace<R>(self: Self, _: R) -> Self
where
    R: fmt::Debug + fmt::Display + Send + Sync + ''static { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Panic { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Panic) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, _: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Panic) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Source**
  - ```rust
    fn new<T: fmt::Display>(error: T) -> Self { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
### Struct `Failure`

An error type that only preserves success or failure, throwing away any more
detailed error messages.

```rust
pub struct Failure;
```

#### Implementations

##### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Failure) -> bool { /* ... */ }
    ```

- **Source**
  - ```rust
    fn new<T: error::Error + Send + Sync + ''static>(_: T) -> Self { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Failure) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Trace**
  - ```rust
    fn trace<R>(self: Self, _: R) -> Self
where
    R: fmt::Debug + fmt::Display + Send + Sync + ''static { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Failure) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Failure { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **Freeze**
- **Error**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **Pointee**
### Struct `Error`

A good general-purpose error type.

If `debug_assertions` and the `alloc` feature are enabled, then this error
will have the same behavior as [`BoxedError`]. Otherwise, it will behave
like [`Failure`].

```rust
pub struct Error {
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
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Trace**
  - ```rust
    fn trace<R>(self: Self, trace: R) -> Self
where
    R: fmt::Debug + fmt::Display + Send + Sync + ''static { /* ... */ }
    ```

- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Source**
  - ```rust
    fn new<T: error::Error + Send + Sync + ''static>(source: T) -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointee**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn error::Error + ''static> { /* ... */ }
    ```

## Traits

### Trait `Trace`

An error type which can add additional "trace" information to itself.

Some functions only add additional context to errors created by other
functions, rather than creating errors themselves. With generics, it's
therefore possible to have a generic function which can produce errors with
some type arguments but not with others. In these cases, `Trace` allows
those functions to add context if an error can occur, and compile out the
context if the error type is [`Infallible`] or [`Panic`].

# Example

```
use rancor::{ResultExt, Trace};

trait Print<E> {
    fn print(&self, message: &str) -> Result<(), E>;
}

fn print_hello_world<T: Print<E>, E: Trace>(printer: &T) -> Result<(), E> {
    printer.print("hello").trace("failed to print hello")?;
    printer.print("world").trace("failed to print world")?;
    Ok(())
}
```

```rust
pub trait Trace: Sized + Send + Sync + ''static {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `trace`: Adds an additional trace to this error, returning a new error.

#### Implementations

This trait is implemented for the following types:

- `BoxedError`
- `Infallible`
- `Panic`
- `Failure`
- `Error`

### Trait `Source`

An error type which can be uniformly constructed from an [`Error`] and
additional trace information.

# Example

```
use core::{error::Error, fmt};

use rancor::{fail, Source};

#[derive(Debug)]
struct DivideByZeroError;

impl fmt::Display for DivideByZeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "attempted to divide by zero")
    }
}

impl Error for DivideByZeroError {}

fn try_divide<E: Source>(a: i32, b: i32) -> Result<i32, E> {
    if b == 0 {
        fail!(DivideByZeroError);
    }
    Ok(a / b)
}
```

```rust
pub trait Source: Trace + error::Error {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `new`: Returns a new `Self` using the given [`Error`].

#### Implementations

This trait is implemented for the following types:

- `BoxedError`
- `Panic`
- `Failure`
- `Error`

### Trait `Fallible`

A type with fallible operations that return its associated error type.

`Fallible` turns an error type parameter into an associated type of another
parameter. You can equip an existing type with a `Fallible` implementation
by wrapping it in a [`Strategy`].

# Example

```
use rancor::{Failure, Fallible, Strategy};

trait Operator<E = <Self as Fallible>::Error> {
    fn operate(&self, lhs: i32, rhs: i32) -> Result<i32, E>;
}

impl<T: Operator<E> + ?Sized, E> Operator<E> for Strategy<T, E> {
    fn operate(&self, lhs: i32, rhs: i32) -> Result<i32, E> {
        T::operate(self, lhs, rhs)
    }
}

struct Add;

impl<E> Operator<E> for Add {
    fn operate(&self, lhs: i32, rhs: i32) -> Result<i32, E> {
        Ok(lhs + rhs)
    }
}

fn operate_one_one<T: Operator + Fallible>(
    operator: &T,
) -> Result<i32, T::Error> {
    operator.operate(1, 1)
}

assert_eq!(
    operate_one_one(Strategy::<_, Failure>::wrap(&mut Add)),
    Ok(2)
);
```

```rust
pub trait Fallible {
    /* Associated items */
}
```

#### Required Items

##### Associated Types

- `Error`: The error type associated with this type's operations.

#### Implementations

This trait is implemented for the following types:

- `Strategy<T, E>` with <T: ?Sized, E>

### Trait `ResultExt`

Helper methods for `Result`s.

```rust
pub trait ResultExt<T, E> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `into_error`: Returns a `Result` with this error type converted to `U`.
- `into_trace`: Returns a `Result` with this error type converted to `U` and with an
- `into_with_trace`: Returns a `Result` with this error type converted to `U` and with an
- `trace`: Adds an additional `trace` message to the error value of this type.
- `with_trace`: Adds an additional trace message to the error value of this type by
- `always_ok`: Safely unwraps a result that is always `Ok`.

#### Implementations

This trait is implemented for the following types:

- `Result<T, E>` with <T, E>

### Trait `OptionExt`

Helper methods for `Option`s.

```rust
pub trait OptionExt<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `into_error`: Returns a `Result` with an error indicating that `Some` was expected but
- `into_trace`: Returns a `Result` with an error indicating that `Some` was expected but
- `into_with_trace`: Returns a `Result` with an error indicating that `Some` was expected but

#### Implementations

This trait is implemented for the following types:

- `Option<T>` with <T>

### Trait `Never`

A type that can never be produced.

Never types include the unstable `!` type, enums with no variants, or any
type that always contains a never type (e.g. a struct with a `Never` field).

# Safety

It must be impossible to produce a value of this type.

```rust
pub unsafe trait Never {
    /* Associated items */
}
```

> This trait is unsafe to implement.

#### Implementations

This trait is implemented for the following types:

- `Infallible`
- `Panic`

## Functions

### Function `unreachable_checked`

**Attributes:**

- `#[inline(always)]`

Consumes a `Never` type, returning a primitive `!`.

This is a safe version of [`unreachable_unchecked`] for `Never` types.

# Example

```
use rancor::{unreachable_checked, Infallible};

let result = Ok::<i32, Infallible>(10);
match result {
    Ok(i) => println!("i"),
    Err(e) => unreachable_checked(e),
}
```

```rust
pub const fn unreachable_checked<T: Never>(_: T) -> never { /* ... */ }
```

## Macros

### Macro `fail`

**Attributes:**

- `#[macro_export]`

Returns the given error from this function.

The current function must return a `Result<_, E>` where `E` implements
[`Source`].

# Example

```
use core::{error::Error, fmt};

use rancor::{fail, Source};

#[derive(Debug)]
struct DivideByZeroError;

impl fmt::Display for DivideByZeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "attempted to divide by zero")
    }
}

impl Error for DivideByZeroError {}

fn divide<E: Source>(a: i32, b: i32) -> Result<i32, E> {
    if b == 0 {
        fail!(DivideByZeroError);
    }
    Ok(a / b)
}
```

```rust
pub macro_rules! fail {
    /* macro_rules! fail {
    ($($x:tt)*) => { ... };
} */
}
```

## Re-exports

### Re-export `Infallible`

A re-export of `core::convert::Infallible`.

```rust
pub use core::convert::Infallible;
```

### Re-export `BoxedError`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use boxed_error::BoxedError;
```

