# Crate Documentation

**Version:** 0.4.3

**Format Version:** 43

# Module `munge`

Munge makes it easy and safe to destructure `MaybeUninit`s, `Cell`s,
`UnsafeCell`s, `ManuallyDrop`s and more.

Just use the `munge!` macro to destructure opaque types the same way you'd
destructure a value. The `munge!` macro may be used to perform either borrow
destructuring (e.g. `let (a, b) = c` where `c` is a reference) or move
destructuring (e.g. `let (a, b) = c` where `c` is a value) depending on the
type.

Munge has no features and is always `#![no_std]`.

## Examples
Initialize `MaybeUninit`s:

```rust
use core::mem::MaybeUninit;
use munge::munge;

pub struct Example {
    a: u32,
    b: (char, f32),
}

let mut mu = MaybeUninit::<Example>::uninit();

munge!(let Example { a, b: (c, mut f) } = &mut mu);
assert_eq!(a.write(10), &10);
assert_eq!(c.write('x'), &'x');
assert_eq!(f.write(3.14), &3.14);
// Note that `mut` bindings can be reassigned like you'd expect:
f = &mut MaybeUninit::uninit();

// SAFETY: `mu` is completely initialized.
let init = unsafe { mu.assume_init() };
assert_eq!(init.a, 10);
assert_eq!(init.b.0, 'x');
assert_eq!(init.b.1, 3.14);
```

Destructure `Cell`s:

```rust
use core::cell::Cell;
use munge::munge;

pub struct Example {
    a: u32,
    b: (char, f32),
}

let value = Example {
    a: 10,
    b: ('x', 3.14),
};
let cell = Cell::<Example>::new(value);

munge!(let Example { a, b: (c, f) } = &cell);
assert_eq!(a.get(), 10);
a.set(42);
assert_eq!(c.get(), 'x');
c.set('!');
assert_eq!(f.get(), 3.14);
f.set(1.41);

let value = cell.into_inner();
assert_eq!(value.a, 42);
assert_eq!(value.b.0, '!');
assert_eq!(value.b.1, 1.41);
```

You can even extend munge to work with your own types by implementing its
`Destructure` and `Restructure` traits:

```rust
use munge::{Destructure, Restructure, Move, munge};

pub struct Invariant<T>(T);

impl<T> Invariant<T> {
    /// # Safety
    ///
    /// `value` must uphold my custom invariant.
    pub unsafe fn new_unchecked(value: T) -> Self {
        Self(value)
    }

    pub fn unwrap(self) -> T {
        self.0
    }
}

// SAFETY:
// - `Invariant<T>` is destructured by move, so its `Destructuring` type is
//   `Move`.
// - `underlying` returns a pointer to its inner type, so it is guaranteed
//   to be non-null, properly aligned, and valid for reads.
unsafe impl<T> Destructure for Invariant<T> {
    type Underlying = T;
    type Destructuring = Move;

    fn underlying(&mut self) -> *mut Self::Underlying {
        &mut self.0 as *mut Self::Underlying
    }
}

// SAFETY: `restructure` returns an `Invariant<U>` that takes ownership of
// the restructured field because `Invariant<T>` is destructured by move.
unsafe impl<T, U> Restructure<U> for Invariant<T> {
    type Restructured = Invariant<U>;

    unsafe fn restructure(&self, ptr: *mut U) -> Self::Restructured {
        // SAFETY: The caller has guaranteed that `ptr` is a pointer to a
        // subfield of some `T`, so it must be properly aligned, valid for
        // reads, and initialized. We may move the fields because the
        // destructuring type for `Invariant<T>` is `Move`.
        let value = unsafe { ptr.read() };
        Invariant(value)
    }
}

// SAFETY: `(1, 2, 3)` upholds my custom invariant.
let value = unsafe { Invariant::new_unchecked((1, 2, 3)) };
munge!(let (one, two, three) = value);
assert_eq!(one.unwrap(), 1);
assert_eq!(two.unwrap(), 2);
assert_eq!(three.unwrap(), 3);
```

## Types

### Struct `Borrow`

Destructuring by borrow, e.g. `let (a, b) = c` where `c` is a reference.

Borrow destructuring leaves the original value intact, only borrowing from
the destructured value. Borrow destructuring may use rest patterns (`..`)
because the original value is not moved and so it is safe to restructure
only some of the fields of the destructured value.

```rust
pub struct Borrow;
```

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Struct `Move`

Destructuring by move, e.g. `let (a, b) = c` where `c` is a value.

Move destructuring forgets the original value and moves each destructured
field during restructuring. Move destructuring may not use rest patterns
(`..`) because every field of the original value must be restructured, else
they will be forgotten.

```rust
pub struct Move;
```

#### Implementations

##### Trait Implementations

- **RefUnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
## Traits

### Trait `Destructure`

A type that can be destructured into its constituent parts.

See the [crate docs](index.html#examples) for an example of implementing
`Destructure` and `Restructure`.

# Safety

- [`Destructuring`](Destructure::Destructuring) must reflect the type of
  destructuring allowed for the type:
  - [`Borrow`] if the type is restructured by creating disjoint borrows of
    the fields of `Underlying`.
  - [`Move`] if the type may be restructured by moving the fields out of the
    destructured `Underlying`.
- [`underlying`](Destructure::underlying) must return a pointer that is
  non-null, properly aligned, and valid for reads.

```rust
pub unsafe trait Destructure: Sized {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `Underlying`: The underlying type that is destructured.
- `Destructuring`: The type of destructuring to perform.

##### Required Methods

- `underlying`: Returns a mutable pointer to the underlying type.

#### Implementations

This trait is implemented for the following types:

- `core::mem::MaybeUninit<T>` with <T>
- `&core::mem::MaybeUninit<T>` with <T>
- `&mut core::mem::MaybeUninit<T>` with <T>
- `core::cell::Cell<T>` with <T>
- `&core::cell::Cell<T>` with <T: ?Sized>
- `&mut core::cell::Cell<T>` with <T: ?Sized>
- `core::cell::UnsafeCell<T>` with <T>
- `&core::cell::UnsafeCell<T>` with <T: ?Sized>
- `&mut core::cell::UnsafeCell<T>` with <T: ?Sized>
- `core::mem::ManuallyDrop<T>` with <T>
- `&core::mem::ManuallyDrop<T>` with <T: ?Sized>
- `&mut core::mem::ManuallyDrop<T>` with <T: ?Sized>

### Trait `Restructure`

A type that can be "restructured" as a field of some containing type.

See the [crate docs](index.html#examples) for an example of implementing
`Destructure` and `Restructure`.

# Safety

[`restructure`](Restructure::restructure) must return a valid
[`Restructured`](Restructure::Restructured) that upholds the invariants for
its [`Destructuring`](Destructure::Destructuring):
- If the type is destructured [by borrow](Borrow), then the `Restructured`
  value must behave as a disjoint borrow of a field of the underlying type.
- If the type is destructured [by move](Move), then the `Restructured` value
  must move the fields out of the underlying type.

```rust
pub unsafe trait Restructure<T: ?Sized>: Destructure {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `Restructured`: The restructured version of this type.

##### Required Methods

- `restructure`: Restructures a pointer to this type into the target type.

#### Implementations

This trait is implemented for the following types:

- `core::mem::MaybeUninit<T>` with <T, U>
- `&''a core::mem::MaybeUninit<T>` with <''a, T, U: ''a>
- `&''a mut core::mem::MaybeUninit<T>` with <''a, T, U: ''a>
- `core::cell::Cell<T>` with <T, U>
- `&''a core::cell::Cell<T>` with <''a, T: ?Sized, U: ''a + ?Sized>
- `&''a mut core::cell::Cell<T>` with <''a, T: ?Sized, U: ''a + ?Sized>
- `core::cell::UnsafeCell<T>` with <T, U>
- `&''a core::cell::UnsafeCell<T>` with <''a, T, U>
- `&''a mut core::cell::UnsafeCell<T>` with <''a, T, U>
- `core::mem::ManuallyDrop<T>` with <T, U>
- `&''a core::mem::ManuallyDrop<T>` with <''a, T, U>
- `&''a mut core::mem::ManuallyDrop<T>` with <''a, T, U>

## Macros

### Macro `munge`

**Attributes:**

- `#[macro_export]`

Destructures a type into

# Example

```
# use core::mem::MaybeUninit;
# use munge::munge;
pub struct Example {
    a: u32,
    b: (char, f32),
}

let mut mu = MaybeUninit::<Example>::uninit();

munge!(let Example { a, b: (c, mut f) } = &mut mu);
assert_eq!(a.write(10), &10);
assert_eq!(c.write('x'), &'x');
assert_eq!(f.write(3.14), &3.14);
// Note that `mut` bindings can be reassigned like you'd expect:
let mut new_f = MaybeUninit::uninit();
f = &mut new_f;

// SAFETY: `mu` is completely initialized.
let init = unsafe { mu.assume_init() };
assert_eq!(init.a, 10);
assert_eq!(init.b.0, 'x');
assert_eq!(init.b.1, 3.14);
```

```rust
pub macro_rules! munge {
    /* macro_rules! munge {
    ($($t:tt)*) => { ... };
} */
}
```

