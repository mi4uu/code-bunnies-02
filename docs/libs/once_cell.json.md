# Crate Documentation

**Version:** 1.21.3

**Format Version:** 43

# Module `once_cell`

# Overview

`once_cell` provides two new cell-like types, [`unsync::OnceCell`] and
[`sync::OnceCell`]. A `OnceCell` might store arbitrary non-`Copy` types, can
be assigned to at most once and provides direct access to the stored
contents. The core API looks *roughly* like this (and there's much more
inside, read on!):

```rust,ignore
impl<T> OnceCell<T> {
    const fn new() -> OnceCell<T> { ... }
    fn set(&self, value: T) -> Result<(), T> { ... }
    fn get(&self) -> Option<&T> { ... }
}
```

Note that, like with [`RefCell`] and [`Mutex`], the `set` method requires
only a shared reference. Because of the single assignment restriction `get`
can return a `&T` instead of `Ref<T>` or `MutexGuard<T>`.

The `sync` flavor is thread-safe (that is, implements the [`Sync`] trait),
while the `unsync` one is not.

[`unsync::OnceCell`]: unsync/struct.OnceCell.html
[`sync::OnceCell`]: sync/struct.OnceCell.html
[`RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html

# Recipes

`OnceCell` might be useful for a variety of patterns.

## Safe Initialization of Global Data

```rust
use std::{env, io};

use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Logger {
    // ...
}
static INSTANCE: OnceCell<Logger> = OnceCell::new();

impl Logger {
    pub fn global() -> &'static Logger {
        INSTANCE.get().expect("logger is not initialized")
    }

    fn from_cli(args: env::Args) -> Result<Logger, std::io::Error> {
       // ...
#      Ok(Logger {})
    }
}

fn main() {
    let logger = Logger::from_cli(env::args()).unwrap();
    INSTANCE.set(logger).unwrap();
    // use `Logger::global()` from now on
}
```

## Lazy Initialized Global Data

This is essentially the `lazy_static!` macro, but without a macro.

```rust
use std::{sync::Mutex, collections::HashMap};

use once_cell::sync::OnceCell;

fn global_data() -> &'static Mutex<HashMap<i32, String>> {
    static INSTANCE: OnceCell<Mutex<HashMap<i32, String>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(13, "Spica".to_string());
        m.insert(74, "Hoyten".to_string());
        Mutex::new(m)
    })
}
```

There are also the [`sync::Lazy`] and [`unsync::Lazy`] convenience types to
streamline this pattern:

```rust
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    Mutex::new(m)
});

fn main() {
    println!("{:?}", GLOBAL_DATA.lock().unwrap());
}
```

Note that the variable that holds `Lazy` is declared as `static`, *not*
`const`. This is important: using `const` instead compiles, but works wrong.

[`sync::Lazy`]: sync/struct.Lazy.html
[`unsync::Lazy`]: unsync/struct.Lazy.html

## General purpose lazy evaluation

Unlike `lazy_static!`, `Lazy` works with local variables.

```rust
use once_cell::unsync::Lazy;

fn main() {
    let ctx = vec![1, 2, 3];
    let thunk = Lazy::new(|| {
        ctx.iter().sum::<i32>()
    });
    assert_eq!(*thunk, 6);
}
```

If you need a lazy field in a struct, you probably should use `OnceCell`
directly, because that will allow you to access `self` during
initialization.

```rust
use std::{fs, path::PathBuf};

use once_cell::unsync::OnceCell;

struct Ctx {
    config_path: PathBuf,
    config: OnceCell<String>,
}

impl Ctx {
    pub fn get_config(&self) -> Result<&str, std::io::Error> {
        let cfg = self.config.get_or_try_init(|| {
            fs::read_to_string(&self.config_path)
        })?;
        Ok(cfg.as_str())
    }
}
```

## Lazily Compiled Regex

This is a `regex!` macro which takes a string literal and returns an
*expression* that evaluates to a `&'static Regex`:

```
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}
```

This macro can be useful to avoid the "compile regex on every loop
iteration" problem.

## Runtime `include_bytes!`

The `include_bytes` macro is useful to include test resources, but it slows
down test compilation a lot. An alternative is to load the resources at
runtime:

```
use std::path::Path;

use once_cell::sync::OnceCell;

pub struct TestResource {
    path: &'static str,
    cell: OnceCell<Vec<u8>>,
}

impl TestResource {
    pub const fn new(path: &'static str) -> TestResource {
        TestResource { path, cell: OnceCell::new() }
    }
    pub fn bytes(&self) -> &[u8] {
        self.cell.get_or_init(|| {
            let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let path = Path::new(dir.as_str()).join(self.path);
            std::fs::read(&path).unwrap_or_else(|_err| {
                panic!("failed to load test resource: {}", path.display())
            })
        }).as_slice()
    }
}

static TEST_IMAGE: TestResource = TestResource::new("test_data/lena.png");

#[test]
fn test_sobel_filter() {
    let rgb: &[u8] = TEST_IMAGE.bytes();
    // ...
# drop(rgb);
}
```

## `lateinit`

`LateInit` type for delayed initialization. It is reminiscent of Kotlin's
`lateinit` keyword and allows construction of cyclic data structures:


```
use once_cell::sync::OnceCell;

pub struct LateInit<T> { cell: OnceCell<T> }

impl<T> LateInit<T> {
    pub fn init(&self, value: T) {
        assert!(self.cell.set(value).is_ok())
    }
}

impl<T> Default for LateInit<T> {
    fn default() -> Self { LateInit { cell: OnceCell::default() } }
}

impl<T> std::ops::Deref for LateInit<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.cell.get().unwrap()
    }
}

#[derive(Default)]
struct A<'a> {
    b: LateInit<&'a B<'a>>,
}

#[derive(Default)]
struct B<'a> {
    a: LateInit<&'a A<'a>>
}


fn build_cycle() {
    let a = A::default();
    let b = B::default();
    a.b.init(&b);
    b.a.init(&a);

    let _a = &a.b.a.b.a;
}
```

# Comparison with std

|`!Sync` types         | Access Mode            | Drawbacks                                     |
|----------------------|------------------------|-----------------------------------------------|
|`Cell<T>`             | `T`                    | requires `T: Copy` for `get`                  |
|`RefCell<T>`          | `RefMut<T>` / `Ref<T>` | may panic at runtime                          |
|`unsync::OnceCell<T>` | `&T`                   | assignable only once                          |

|`Sync` types          | Access Mode            | Drawbacks                                     |
|----------------------|------------------------|-----------------------------------------------|
|`AtomicT`             | `T`                    | works only with certain `Copy` types          |
|`Mutex<T>`            | `MutexGuard<T>`        | may deadlock at runtime, may block the thread |
|`sync::OnceCell<T>`   | `&T`                   | assignable only once, may block the thread    |

Technically, calling `get_or_init` will also cause a panic or a deadlock if
it recursively calls itself. However, because the assignment can happen only
once, such cases should be more rare than equivalents with `RefCell` and
`Mutex`.

# Minimum Supported `rustc` Version

If only the `std`, `alloc`, or `race` features are enabled, MSRV will be
updated conservatively, supporting at least latest 8 versions of the compiler.
When using other features, like `parking_lot`, MSRV might be updated more
frequently, up to the latest stable. In both cases, increasing MSRV is *not*
considered a semver-breaking change and requires only a minor version bump.

# Implementation details

The implementation is based on the
[`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs/) and
[`lazy_cell`](https://github.com/indiv0/lazycell/) crates and
[`std::sync::Once`]. In some sense, `once_cell` just streamlines and unifies
those APIs.

To implement a sync flavor of `OnceCell`, this crates uses either a custom
re-implementation of `std::sync::Once` or `parking_lot::Mutex`. This is
controlled by the `parking_lot` feature (disabled by default). Performance
is the same for both cases, but the `parking_lot` based `OnceCell<T>` is
smaller by up to 16 bytes.

This crate uses `unsafe`.

[`std::sync::Once`]: https://doc.rust-lang.org/std/sync/struct.Once.html

# F.A.Q.

**Should I use the sync or unsync flavor?**

Because Rust compiler checks thread safety for you, it's impossible to
accidentally use `unsync` where `sync` is required. So, use `unsync` in
single-threaded code and `sync` in multi-threaded. It's easy to switch
between the two if code becomes multi-threaded later.

At the moment, `unsync` has an additional benefit that reentrant
initialization causes a panic, which might be easier to debug than a
deadlock.

**Does this crate support async?**

No, but you can use
[`async_once_cell`](https://crates.io/crates/async_once_cell) instead.

**Does this crate support `no_std`?**

Yes, but with caveats. `OnceCell` is a synchronization primitive which
_semantically_ relies on blocking. `OnceCell` guarantees that at most one
`f` will be called to compute the value. If two threads of execution call
`get_or_init` concurrently, one of them has to wait.

Waiting fundamentally requires OS support. Execution environment needs to
understand who waits on whom to prevent deadlocks due to priority inversion.
You _could_ make code to compile by blindly using pure spinlocks, but the
runtime behavior would be subtly wrong.

Given these constraints, `once_cell` provides the following options:

- The `race` module provides similar, but distinct synchronization primitive
  which is compatible with `no_std`. With `race`, the `f` function can be
  called multiple times by different threads, but only one thread will win
  to install the value.
- `critical-section` feature (with a `-`, not `_`) uses `critical_section`
  to implement blocking.

**Can I bring my own mutex?**

There is [generic_once_cell](https://crates.io/crates/generic_once_cell) to
allow just that.

**Should I use `std::cell::OnceCell`, `once_cell`, or `lazy_static`?**

If you can use `std` version (your MSRV is at least 1.70, and you don't need
extra features `once_cell` provides), use `std`. Otherwise, use `once_cell`.
Don't use `lazy_static`.

# Related crates

* Most of this crate's functionality is available in `std` starting with
  Rust 1.70. See `std::cell::OnceCell` and `std::sync::OnceLock`.
* [double-checked-cell](https://github.com/niklasf/double-checked-cell)
* [lazy-init](https://crates.io/crates/lazy-init)
* [lazycell](https://crates.io/crates/lazycell)
* [mitochondria](https://crates.io/crates/mitochondria)
* [lazy_static](https://crates.io/crates/lazy_static)
* [async_once_cell](https://crates.io/crates/async_once_cell)
* [generic_once_cell](https://crates.io/crates/generic_once_cell) (bring
  your own mutex)

## Modules

## Module `unsync`

Single-threaded version of `OnceCell`.

```rust
pub mod unsync { /* ... */ }
```

### Types

#### Struct `OnceCell`

A cell which can be written to only once. It is not thread safe.

Unlike [`std::cell::RefCell`], a `OnceCell` provides simple `&`
references to the contents.

[`std::cell::RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html

# Example
```
use once_cell::unsync::OnceCell;

let cell = OnceCell::new();
assert!(cell.get().is_none());

let value: &String = cell.get_or_init(|| {
    "Hello, World!".to_string()
});
assert_eq!(value, "Hello, World!");
assert!(cell.get().is_some());
```

```rust
pub struct OnceCell<T> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub const fn new() -> OnceCell<T> { /* ... */ }
  ```
  Creates a new empty cell.

- ```rust
  pub const fn with_value(value: T) -> OnceCell<T> { /* ... */ }
  ```
  Creates a new initialized cell.

- ```rust
  pub fn get(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Gets a reference to the underlying value.

- ```rust
  pub fn get_mut(self: &mut Self) -> Option<&mut T> { /* ... */ }
  ```
  Gets a mutable reference to the underlying value.

- ```rust
  pub fn set(self: &Self, value: T) -> Result<(), T> { /* ... */ }
  ```
  Sets the contents of this cell to `value`.

- ```rust
  pub fn try_insert(self: &Self, value: T) -> Result<&T, (&T, T)> { /* ... */ }
  ```
  Like [`set`](Self::set), but also returns a reference to the final cell value.

- ```rust
  pub fn get_or_init<F>(self: &Self, f: F) -> &T
where
    F: FnOnce() -> T { /* ... */ }
  ```
  Gets the contents of the cell, initializing it with `f`

- ```rust
  pub fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<&T, E>
where
    F: FnOnce() -> Result<T, E> { /* ... */ }
  ```
  Gets the contents of the cell, initializing it with `f` if

- ```rust
  pub fn take(self: &mut Self) -> Option<T> { /* ... */ }
  ```
  Takes the value out of this `OnceCell`, moving it back to an uninitialized state.

- ```rust
  pub fn into_inner(self: Self) -> Option<T> { /* ... */ }
  ```
  Consumes the `OnceCell`, returning the wrapped value.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OnceCell<T> { /* ... */ }
    ```

  - ```rust
    fn clone_from(self: &mut Self, source: &Self) { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(t: never) -> T { /* ... */ }
    ```

  - ```rust
    fn from(value: T) -> Self { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

#### Struct `Lazy`

A value which is initialized on the first access.

# Example
```
use once_cell::unsync::Lazy;

let lazy: Lazy<i32> = Lazy::new(|| {
    println!("initializing");
    92
});
println!("ready");
println!("{}", *lazy);
println!("{}", *lazy);

// Prints:
//   ready
//   initializing
//   92
//   92
```

```rust
pub struct Lazy<T, F = fn() -> T> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(init: F) -> Lazy<T, F> { /* ... */ }
  ```
  Creates a new lazy value with the given initializing function.

- ```rust
  pub fn into_value(this: Lazy<T, F>) -> Result<T, F> { /* ... */ }
  ```
  Consumes this `Lazy` returning the stored value.

- ```rust
  pub fn force(this: &Lazy<T, F>) -> &T { /* ... */ }
  ```
  Forces the evaluation of this lazy value and returns a reference to

- ```rust
  pub fn force_mut(this: &mut Lazy<T, F>) -> &mut T { /* ... */ }
  ```
  Forces the evaluation of this lazy value and returns a mutable reference to

- ```rust
  pub fn get(this: &Lazy<T, F>) -> Option<&T> { /* ... */ }
  ```
  Gets the reference to the result of this lazy value if

- ```rust
  pub fn get_mut(this: &mut Lazy<T, F>) -> Option<&mut T> { /* ... */ }
  ```
  Gets the mutable reference to the result of this lazy value if

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Receiver**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Lazy<T> { /* ... */ }
    ```
    Creates a new lazy value using `Default` as the initializing function.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &T { /* ... */ }
    ```

## Module `sync`

**Attributes:**

- `#[<cfg>(any(feature = "std", feature = "critical-section"))]`

Thread-safe, blocking version of `OnceCell`.

```rust
pub mod sync { /* ... */ }
```

### Types

#### Struct `OnceCell`

A thread-safe cell which can be written to only once.

`OnceCell` provides `&` references to the contents without RAII guards.

Reading a non-`None` value out of `OnceCell` establishes a
happens-before relationship with a corresponding write. For example, if
thread A initializes the cell with `get_or_init(f)`, and thread B
subsequently reads the result of this call, B also observes all the side
effects of `f`.

# Example
```
use once_cell::sync::OnceCell;

static CELL: OnceCell<String> = OnceCell::new();
assert!(CELL.get().is_none());

std::thread::spawn(|| {
    let value: &String = CELL.get_or_init(|| {
        "Hello, World!".to_string()
    });
    assert_eq!(value, "Hello, World!");
}).join().unwrap();

let value: Option<&String> = CELL.get();
assert!(value.is_some());
assert_eq!(value.unwrap().as_str(), "Hello, World!");
```

```rust
pub struct OnceCell<T>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new() -> OnceCell<T> { /* ... */ }
  ```
  Creates a new empty cell.

- ```rust
  pub const fn with_value(value: T) -> OnceCell<T> { /* ... */ }
  ```
  Creates a new initialized cell.

- ```rust
  pub fn get(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Gets the reference to the underlying value.

- ```rust
  pub fn wait(self: &Self) -> &T { /* ... */ }
  ```
  Gets the reference to the underlying value, blocking the current

- ```rust
  pub fn get_mut(self: &mut Self) -> Option<&mut T> { /* ... */ }
  ```
  Gets the mutable reference to the underlying value.

- ```rust
  pub unsafe fn get_unchecked(self: &Self) -> &T { /* ... */ }
  ```
  Get the reference to the underlying value, without checking if the

- ```rust
  pub fn set(self: &Self, value: T) -> Result<(), T> { /* ... */ }
  ```
  Sets the contents of this cell to `value`.

- ```rust
  pub fn try_insert(self: &Self, value: T) -> Result<&T, (&T, T)> { /* ... */ }
  ```
  Like [`set`](Self::set), but also returns a reference to the final cell value.

- ```rust
  pub fn get_or_init<F>(self: &Self, f: F) -> &T
where
    F: FnOnce() -> T { /* ... */ }
  ```
  Gets the contents of the cell, initializing it with `f` if the cell

- ```rust
  pub fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<&T, E>
where
    F: FnOnce() -> Result<T, E> { /* ... */ }
  ```
  Gets the contents of the cell, initializing it with `f` if

- ```rust
  pub fn take(self: &mut Self) -> Option<T> { /* ... */ }
  ```
  Takes the value out of this `OnceCell`, moving it back to an uninitialized state.

- ```rust
  pub fn into_inner(self: Self) -> Option<T> { /* ... */ }
  ```
  Consumes the `OnceCell`, returning the wrapped value. Returns

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> OnceCell<T> { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> OnceCell<T> { /* ... */ }
    ```

  - ```rust
    fn clone_from(self: &mut Self, source: &Self) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(t: never) -> T { /* ... */ }
    ```

  - ```rust
    fn from(value: T) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OnceCell<T>) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Lazy`

A value which is initialized on the first access.

This type is thread-safe and can be used in statics.

# Example

```
use std::collections::HashMap;

use once_cell::sync::Lazy;

static HASHMAP: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    println!("initializing");
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    m
});

fn main() {
    println!("ready");
    std::thread::spawn(|| {
        println!("{:?}", HASHMAP.get(&13));
    }).join().unwrap();
    println!("{:?}", HASHMAP.get(&74));

    // Prints:
    //   ready
    //   initializing
    //   Some("Spica")
    //   Some("Hoyten")
}
```

```rust
pub struct Lazy<T, F = fn() -> T> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(f: F) -> Lazy<T, F> { /* ... */ }
  ```
  Creates a new lazy value with the given initializing

- ```rust
  pub fn into_value(this: Lazy<T, F>) -> Result<T, F> { /* ... */ }
  ```
  Consumes this `Lazy` returning the stored value.

- ```rust
  pub fn force(this: &Lazy<T, F>) -> &T { /* ... */ }
  ```
  Forces the evaluation of this lazy value and

- ```rust
  pub fn force_mut(this: &mut Lazy<T, F>) -> &mut T { /* ... */ }
  ```
  Forces the evaluation of this lazy value and

- ```rust
  pub fn get(this: &Lazy<T, F>) -> Option<&T> { /* ... */ }
  ```
  Gets the reference to the result of this lazy value if

- ```rust
  pub fn get_mut(this: &mut Lazy<T, F>) -> Option<&mut T> { /* ... */ }
  ```
  Gets the reference to the result of this lazy value if

###### Trait Implementations

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Receiver**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Lazy<T> { /* ... */ }
    ```
    Creates a new lazy value using `Default` as the initializing function.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

## Module `race`

**Attributes:**

- `#[<cfg>(feature = "race")]`

Thread-safe, non-blocking, "first one wins" flavor of `OnceCell`.

If two threads race to initialize a type from the `race` module, they
don't block, execute initialization function together, but only one of
them stores the result.

This module does not require `std` feature.

# Atomic orderings

All types in this module use `Acquire` and `Release`
[atomic orderings](Ordering) for all their operations. While this is not
strictly necessary for types other than `OnceBox`, it is useful for users as
it allows them to be certain that after `get` or `get_or_init` returns on
one thread, any side-effects caused by the setter thread prior to them
calling `set` or `get_or_init` will be made visible to that thread; without
it, it's possible for it to appear as if they haven't happened yet from the
getter thread's perspective. This is an acceptable tradeoff to make since
`Acquire` and `Release` have very little performance overhead on most
architectures versus `Relaxed`.

```rust
pub mod race { /* ... */ }
```

### Types

#### Struct `OnceNonZeroUsize`

A thread-safe cell which can be written to only once.

```rust
pub struct OnceNonZeroUsize {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub const fn new() -> Self { /* ... */ }
  ```
  Creates a new empty cell.

- ```rust
  pub fn get(self: &Self) -> Option<NonZeroUsize> { /* ... */ }
  ```
  Gets the underlying value.

- ```rust
  pub unsafe fn get_unchecked(self: &Self) -> NonZeroUsize { /* ... */ }
  ```
  Get the reference to the underlying value, without checking if the cell

- ```rust
  pub fn set(self: &Self, value: NonZeroUsize) -> Result<(), ()> { /* ... */ }
  ```
  Sets the contents of this cell to `value`.

- ```rust
  pub fn get_or_init<F>(self: &Self, f: F) -> NonZeroUsize
where
    F: FnOnce() -> NonZeroUsize { /* ... */ }
  ```
  Gets the contents of the cell, initializing it with `f` if the cell was

- ```rust
  pub fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<NonZeroUsize, E>
where
    F: FnOnce() -> Result<NonZeroUsize, E> { /* ... */ }
  ```
  Gets the contents of the cell, initializing it with `f` if

###### Trait Implementations

- **RefUnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> OnceNonZeroUsize { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `OnceBool`

A thread-safe cell which can be written to only once.

```rust
pub struct OnceBool {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub const fn new() -> Self { /* ... */ }
  ```
  Creates a new empty cell.

- ```rust
  pub fn get(self: &Self) -> Option<bool> { /* ... */ }
  ```
  Gets the underlying value.

- ```rust
  pub fn set(self: &Self, value: bool) -> Result<(), ()> { /* ... */ }
  ```
  Sets the contents of this cell to `value`.

- ```rust
  pub fn get_or_init<F>(self: &Self, f: F) -> bool
where
    F: FnOnce() -> bool { /* ... */ }
  ```
  Gets the contents of the cell, initializing it with `f` if the cell was

- ```rust
  pub fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<bool, E>
where
    F: FnOnce() -> Result<bool, E> { /* ... */ }
  ```
  Gets the contents of the cell, initializing it with `f` if

###### Trait Implementations

- **UnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> OnceBool { /* ... */ }
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

- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
#### Struct `OnceRef`

A thread-safe cell which can be written to only once.

```rust
pub struct OnceRef<''a, T> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub const fn new() -> Self { /* ... */ }
  ```
  Creates a new empty cell.

- ```rust
  pub fn get(self: &Self) -> Option<&''a T> { /* ... */ }
  ```
  Gets a reference to the underlying value.

- ```rust
  pub fn set(self: &Self, value: &''a T) -> Result<(), ()> { /* ... */ }
  ```
  Sets the contents of this cell to `value`.

- ```rust
  pub fn get_or_init<F>(self: &Self, f: F) -> &''a T
where
    F: FnOnce() -> &''a T { /* ... */ }
  ```
  Gets the contents of the cell, initializing it with `f` if the cell was

- ```rust
  pub fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<&''a T, E>
where
    F: FnOnce() -> Result<&''a T, E> { /* ... */ }
  ```
  Gets the contents of the cell, initializing it with `f` if

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

### Re-exports

#### Re-export `OnceBox`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use self::once_box::OnceBox;
```

