# Crate Documentation

**Version:** 1.2.0

**Format Version:** 43

# Module `scopeguard`

A scope guard will run a given closure when it goes out of scope,
even if the code between panics.
(as long as panic doesn't abort)

# Examples

## Hello World

This example creates a scope guard with an example function:

```
extern crate scopeguard;

fn f() {
    let _guard = scopeguard::guard((), |_| {
        println!("Hello Scope Exit!");
    });

    // rest of the code here.

    // Here, at the end of `_guard`'s scope, the guard's closure is called.
    // It is also called if we exit this scope through unwinding instead.
}
# fn main() {
#    f();
# }
```

## `defer!`

Use the `defer` macro to run an operation at scope exit,
either regular scope exit or during unwinding from a panic.

```
#[macro_use(defer)] extern crate scopeguard;

use std::cell::Cell;

fn main() {
    // use a cell to observe drops during and after the scope guard is active
    let drop_counter = Cell::new(0);
    {
        // Create a scope guard using `defer!` for the current scope
        defer! {
            drop_counter.set(1 + drop_counter.get());
        }

        // Do regular operations here in the meantime.

        // Just before scope exit: it hasn't run yet.
        assert_eq!(drop_counter.get(), 0);

        // The following scope end is where the defer closure is called
    }
    assert_eq!(drop_counter.get(), 1);
}
```

## Scope Guard with Value

If the scope guard closure needs to access an outer value that is also
mutated outside of the scope guard, then you may want to use the scope guard
with a value. The guard works like a smart pointer, so the inner value can
be accessed by reference or by mutable reference.

### 1. The guard owns a file

In this example, the scope guard owns a file and ensures pending writes are
synced at scope exit.

```
extern crate scopeguard;

use std::fs::*;
use std::io::{self, Write};
# // Mock file so that we don't actually write a file
# struct MockFile;
# impl MockFile {
#     fn create(_s: &str) -> io::Result<Self> { Ok(MockFile) }
#     fn write_all(&self, _b: &[u8]) -> io::Result<()> { Ok(()) }
#     fn sync_all(&self) -> io::Result<()> { Ok(()) }
# }
# use self::MockFile as File;

fn try_main() -> io::Result<()> {
    let f = File::create("newfile.txt")?;
    let mut file = scopeguard::guard(f, |f| {
        // ensure we flush file at return or panic
        let _ = f.sync_all();
    });
    // Access the file through the scope guard itself
    file.write_all(b"test me\n").map(|_| ())
}

fn main() {
    try_main().unwrap();
}

```

### 2. The guard restores an invariant on scope exit

```
extern crate scopeguard;

use std::mem::ManuallyDrop;
use std::ptr;

// This function, just for this example, takes the first element
// and inserts it into the assumed sorted tail of the vector.
//
// For optimization purposes we temporarily violate an invariant of the
// Vec, that it owns all of its elements.
//
// The safe approach is to use swap, which means two writes to memory,
// the optimization is to use a “hole” which uses only one write of memory
// for each position it moves.
//
// We *must* use a scope guard to run this code safely. We
// are running arbitrary user code (comparison operators) that may panic.
// The scope guard ensures we restore the invariant after successful
// exit or during unwinding from panic.
fn insertion_sort_first<T>(v: &mut Vec<T>)
    where T: PartialOrd
{
    struct Hole<'a, T: 'a> {
        v: &'a mut Vec<T>,
        index: usize,
        value: ManuallyDrop<T>,
    }

    unsafe {
        // Create a moved-from location in the vector, a “hole”.
        let value = ptr::read(&v[0]);
        let mut hole = Hole { v: v, index: 0, value: ManuallyDrop::new(value) };

        // Use a scope guard with a value.
        // At scope exit, plug the hole so that the vector is fully
        // initialized again.
        // The scope guard owns the hole, but we can access it through the guard.
        let mut hole_guard = scopeguard::guard(hole, |hole| {
            // plug the hole in the vector with the value that was // taken out
            let index = hole.index;
            ptr::copy_nonoverlapping(&*hole.value, &mut hole.v[index], 1);
        });

        // run algorithm that moves the hole in the vector here
        // move the hole until it's in a sorted position
        for i in 1..hole_guard.v.len() {
            if *hole_guard.value >= hole_guard.v[i] {
                // move the element back and the hole forward
                let index = hole_guard.index;
                hole_guard.v.swap(index, index + 1);
                hole_guard.index += 1;
            } else {
                break;
            }
        }

        // When the scope exits here, the Vec becomes whole again!
    }
}

fn main() {
    let string = String::from;
    let mut data = vec![string("c"), string("a"), string("b"), string("d")];
    insertion_sort_first(&mut data);
    assert_eq!(data, vec!["a", "b", "c", "d"]);
}

```


# Crate Features

- `use_std`
  + Enabled by default. Enables the `OnUnwind` and `OnSuccess` strategies.
  + Disable to use `no_std`.

# Rust Version

This version of the crate requires Rust 1.20 or later.

The scopeguard 1.x release series will use a carefully considered version
upgrade policy, where in a later 1.x version, we will raise the minimum
required Rust version.

## Types

### Enum `Always`

Always run on scope exit.

“Always” run: on regular exit from a scope or on unwinding from a panic.
Can not run on abort, process exit, and other catastrophic events where
destructors don’t run.

```rust
pub enum Always {
}
```

#### Variants

#### Implementations

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

- **Sync**
- **Strategy**
  - ```rust
    fn should_run() -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `ScopeGuard`

`ScopeGuard` is a scope guard that may own a protected value.

If you place a guard in a local variable, the closure can
run regardless how you leave the scope — through regular return or panic
(except if panic or other code aborts; so as long as destructors run).
It is run only once.

The `S` parameter for [`Strategy`](trait.Strategy.html) determines if
the closure actually runs.

The guard's closure will be called with the held value in the destructor.

The `ScopeGuard` implements `Deref` so that you can access the inner value.

```rust
pub struct ScopeGuard<T, F, S = Always> {
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
  pub fn with_strategy(v: T, dropfn: F) -> ScopeGuard<T, F, S> { /* ... */ }
  ```
  Create a `ScopeGuard` that owns `v` (accessible through deref) and calls

- ```rust
  pub fn into_inner(guard: Self) -> T { /* ... */ }
  ```
  “Defuse” the guard and extract the value without calling the closure.

##### Trait Implementations

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Receiver**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Send**
- **UnwindSafe**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Traits

### Trait `Strategy`

Controls in which cases the associated code should be run

```rust
pub trait Strategy {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `should_run`: Return `true` if the guard’s associated code should run

#### Implementations

This trait is implemented for the following types:

- `Always`

## Functions

### Function `guard`

**Attributes:**

- `#[inline]`
- `#[must_use]`

Create a new `ScopeGuard` owning `v` and with deferred closure `dropfn`.

```rust
pub fn guard<T, F>(v: T, dropfn: F) -> ScopeGuard<T, F, Always>
where
    F: FnOnce(T) { /* ... */ }
```

## Macros

### Macro `defer`

**Attributes:**

- `#[macro_export]`

Macro to create a `ScopeGuard` (always run).

The macro takes statements, which are the body of a closure
that will run when the scope is exited.

```rust
pub macro_rules! defer {
    /* macro_rules! defer {
    ($($t:tt)*) => { ... };
} */
}
```

