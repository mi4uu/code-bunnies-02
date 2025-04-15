# Crate Documentation

**Version:** 0.3.1

**Format Version:** 43

# Module `want`

A Futures channel-like utility to signal when a value is wanted.

Futures are supposed to be lazy, and only starting work if `Future::poll`
is called. The same is true of `Stream`s, but when using a channel as
a `Stream`, it can be hard to know if the receiver is ready for the next
value.

Put another way, given a `(tx, rx)` from `futures::sync::mpsc::channel()`,
how can the sender (`tx`) know when the receiver (`rx`) actually wants more
work to be produced? Just because there is room in the channel buffer
doesn't mean the work would be used by the receiver.

This is where something like `want` comes in. Added to a channel, you can
make sure that the `tx` only creates the message and sends it when the `rx`
has `poll()` for it, and the buffer was empty.

# Example

```nightly
# //#![feature(async_await)]
extern crate want;

# fn spawn<T>(_t: T) {}
# fn we_still_want_message() -> bool { true }
# fn mpsc_channel() -> (Tx, Rx) { (Tx, Rx) }
# struct Tx;
# impl Tx { fn send<T>(&mut self, _: T) {} }
# struct Rx;
# impl Rx { async fn recv(&mut self) -> Option<Expensive> { Some(Expensive) } }

// Some message that is expensive to produce.
struct Expensive;

// Some futures-aware MPSC channel...
let (mut tx, mut rx) = mpsc_channel();

// And our `want` channel!
let (mut gv, mut tk) = want::new();


// Our receiving task...
spawn(async move {
    // Maybe something comes up that prevents us from ever
    // using the expensive message.
    //
    // Without `want`, the "send" task may have started to
    // produce the expensive message even though we wouldn't
    // be able to use it.
    if !we_still_want_message() {
        return;
    }

    // But we can use it! So tell the `want` channel.
    tk.want();

    match rx.recv().await {
        Some(_msg) => println!("got a message"),
        None => println!("DONE"),
    }
});

// Our sending task
spawn(async move {
    // It's expensive to create a new message, so we wait until the
    // receiving end truly *wants* the message.
    if let Err(_closed) = gv.want().await {
        // Looks like they will never want it...
        return;
    }

    // They want it, let's go!
    tx.send(Expensive);
});

# fn main() {}
```

## Types

### Struct `Giver`

An entity that gives a value when wanted.

```rust
pub struct Giver {
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
  pub fn want(self: &mut Self) -> impl Future<Output = Result<(), Closed>> + ''_ { /* ... */ }
  ```
  Returns a `Future` that fulfills when the `Taker` has done some action.

- ```rust
  pub fn poll_want(self: &mut Self, cx: &mut task::Context<''_>) -> Poll<Result<(), Closed>> { /* ... */ }
  ```
  Poll whether the `Taker` has registered interest in another value.

- ```rust
  pub fn give(self: &Self) -> bool { /* ... */ }
  ```
  Mark the state as idle, if the Taker currently is wanting.

- ```rust
  pub fn is_wanting(self: &Self) -> bool { /* ... */ }
  ```
  Check if the `Taker` has called `want()` without parking a task.

- ```rust
  pub fn is_canceled(self: &Self) -> bool { /* ... */ }
  ```
  Check if the `Taker` has canceled interest without parking a task.

- ```rust
  pub fn shared(self: Self) -> SharedGiver { /* ... */ }
  ```
  Converts this into a `SharedGiver`.

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
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
### Struct `Taker`

An entity that wants a value.

```rust
pub struct Taker {
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
  pub fn cancel(self: &mut Self) { /* ... */ }
  ```
  Signal to the `Giver` that the want is canceled.

- ```rust
  pub fn want(self: &mut Self) { /* ... */ }
  ```
  Signal to the `Giver` that a value is wanted.

##### Trait Implementations

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
### Struct `SharedGiver`

A cloneable `Giver`.

It differs from `Giver` in that you cannot poll for `want`. It's only
usable as a cancellation watcher.

```rust
pub struct SharedGiver {
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
  pub fn is_wanting(self: &Self) -> bool { /* ... */ }
  ```
  Check if the `Taker` has called `want()` without parking a task.

- ```rust
  pub fn is_canceled(self: &Self) -> bool { /* ... */ }
  ```
  Check if the `Taker` has canceled interest without parking a task.

##### Trait Implementations

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SharedGiver { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Freeze**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `Closed`

The `Taker` has canceled its interest in a value.

```rust
pub struct Closed {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
## Functions

### Function `new`

Create a new `want` channel.

```rust
pub fn new() -> (Giver, Taker) { /* ... */ }
```

