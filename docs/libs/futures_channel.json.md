# Crate Documentation

**Version:** 0.3.31

**Format Version:** 43

# Module `futures_channel`

Asynchronous channels.

Like threads, concurrent tasks sometimes need to communicate with each
other. This module contains two basic abstractions for doing so:

- [oneshot], a way of sending a single value from one task to another.
- [mpsc], a multi-producer, single-consumer channel for sending values
  between tasks, analogous to the similarly-named structure in the standard
  library.

All items are only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

## Modules

## Module `mpsc`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "std")]`

A multi-producer, single-consumer queue for sending values across
asynchronous tasks.

Similarly to the `std`, channel creation provides [`Receiver`] and
[`Sender`] handles. [`Receiver`] implements [`Stream`] and allows a task to
read values out of the channel. If there is no message to read from the
channel, the current task will be notified when a new value is sent.
[`Sender`] implements the `Sink` trait and allows a task to send messages into
the channel. If the channel is at capacity, the send will be rejected and
the task will be notified when additional capacity is available. In other
words, the channel provides backpressure.

Unbounded channels are also available using the `unbounded` constructor.

# Disconnection

When all [`Sender`] handles have been dropped, it is no longer
possible to send values into the channel. This is considered the termination
event of the stream. As such, [`Receiver::poll_next`]
will return `Ok(Ready(None))`.

If the [`Receiver`] handle is dropped, then messages can no longer
be read out of the channel. In this case, all further attempts to send will
result in an error.

# Clean Shutdown

If the [`Receiver`] is simply dropped, then it is possible for
there to be messages still in the channel that will not be processed. As
such, it is usually desirable to perform a "clean" shutdown. To do this, the
receiver will first call `close`, which will prevent any further messages to
be sent into the channel. Then, the receiver consumes the channel to
completion, at which point the receiver can be dropped.

[`Sender`]: struct.Sender.html
[`Receiver`]: struct.Receiver.html
[`Stream`]: ../../futures_core/stream/trait.Stream.html
[`Receiver::poll_next`]:
    ../../futures_core/stream/trait.Stream.html#tymethod.poll_next

```rust
pub mod mpsc { /* ... */ }
```

### Types

#### Struct `Sender`

The transmission end of a bounded mpsc channel.

This value is created by the [`channel`] function.

```rust
pub struct Sender<T>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn try_send(self: &mut Self, msg: T) -> Result<(), TrySendError<T>> { /* ... */ }
  ```
  Attempts to send a message on this `Sender`, returning the message

- ```rust
  pub fn start_send(self: &mut Self, msg: T) -> Result<(), SendError> { /* ... */ }
  ```
  Send a message on the channel.

- ```rust
  pub fn poll_ready(self: &mut Self, cx: &mut Context<''_>) -> Poll<Result<(), SendError>> { /* ... */ }
  ```
  Polls the channel to determine if there is guaranteed capacity to send

- ```rust
  pub fn is_closed(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether this channel is closed without needing a context.

- ```rust
  pub fn close_channel(self: &mut Self) { /* ... */ }
  ```
  Closes this channel from the sender side, preventing any new messages.

- ```rust
  pub fn disconnect(self: &mut Self) { /* ... */ }
  ```
  Disconnects this sender from the channel, closing it if there are no more senders left.

- ```rust
  pub fn same_receiver(self: &Self, other: &Self) -> bool { /* ... */ }
  ```
  Returns whether the senders send to the same receiver.

- ```rust
  pub fn is_connected_to(self: &Self, receiver: &Receiver<T>) -> bool { /* ... */ }
  ```
  Returns whether the sender send to this receiver.

- ```rust
  pub fn hash_receiver<H>(self: &Self, hasher: &mut H)
where
    H: std::hash::Hasher { /* ... */ }
  ```
  Hashes the receiver into the provided hasher

###### Trait Implementations

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sink**
  - ```rust
    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>> { /* ... */ }
    ```

  - ```rust
    fn start_send(self: Pin<&mut Self>, msg: T) -> Result<(), <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>> { /* ... */ }
    ```

  - ```rust
    fn poll_close(self: Pin<&mut Self>, _: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>> { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `UnboundedSender`

The transmission end of an unbounded mpsc channel.

This value is created by the [`unbounded`] function.

```rust
pub struct UnboundedSender<T>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn poll_ready(self: &Self, _: &mut Context<''_>) -> Poll<Result<(), SendError>> { /* ... */ }
  ```
  Check if the channel is ready to receive a message.

- ```rust
  pub fn is_closed(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether this channel is closed without needing a context.

- ```rust
  pub fn close_channel(self: &Self) { /* ... */ }
  ```
  Closes this channel from the sender side, preventing any new messages.

- ```rust
  pub fn disconnect(self: &mut Self) { /* ... */ }
  ```
  Disconnects this sender from the channel, closing it if there are no more senders left.

- ```rust
  pub fn start_send(self: &mut Self, msg: T) -> Result<(), SendError> { /* ... */ }
  ```
  Send a message on the channel.

- ```rust
  pub fn unbounded_send(self: &Self, msg: T) -> Result<(), TrySendError<T>> { /* ... */ }
  ```
  Sends a message along this channel.

- ```rust
  pub fn same_receiver(self: &Self, other: &Self) -> bool { /* ... */ }
  ```
  Returns whether the senders send to the same receiver.

- ```rust
  pub fn is_connected_to(self: &Self, receiver: &UnboundedReceiver<T>) -> bool { /* ... */ }
  ```
  Returns whether the sender send to this receiver.

- ```rust
  pub fn hash_receiver<H>(self: &Self, hasher: &mut H)
where
    H: std::hash::Hasher { /* ... */ }
  ```
  Hashes the receiver into the provided hasher

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Return the number of messages in the queue or 0 if channel is disconnected.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Return false is channel has no queued messages, true otherwise.

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Sink**
  - ```rust
    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>> { /* ... */ }
    ```

  - ```rust
    fn start_send(self: Pin<&mut Self>, msg: T) -> Result<(), <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>> { /* ... */ }
    ```

  - ```rust
    fn poll_close(self: Pin<&mut Self>, _: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>> { /* ... */ }
    ```

  - ```rust
    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>> { /* ... */ }
    ```

  - ```rust
    fn start_send(self: Pin<&mut Self>, msg: T) -> Result<(), <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>> { /* ... */ }
    ```

  - ```rust
    fn poll_close(self: Pin<&mut Self>, _: &mut Context<''_>) -> Poll<Result<(), <Self as >::Error>> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Receiver`

The receiving end of a bounded mpsc channel.

This value is created by the [`channel`] function.

```rust
pub struct Receiver<T> {
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
  pub fn close(self: &mut Self) { /* ... */ }
  ```
  Closes the receiving half of a channel, without dropping it.

- ```rust
  pub fn try_next(self: &mut Self) -> Result<Option<T>, TryRecvError> { /* ... */ }
  ```
  Tries to receive the next message without notifying a context if empty.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Stream**
  - ```rust
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Option<T>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **FusedStream**
  - ```rust
    fn is_terminated(self: &Self) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryStream**
  - ```rust
    fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<''_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>> { /* ... */ }
    ```

- **Sync**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

#### Struct `UnboundedReceiver`

The receiving end of an unbounded mpsc channel.

This value is created by the [`unbounded`] function.

```rust
pub struct UnboundedReceiver<T> {
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
  pub fn close(self: &mut Self) { /* ... */ }
  ```
  Closes the receiving half of a channel, without dropping it.

- ```rust
  pub fn try_next(self: &mut Self) -> Result<Option<T>, TryRecvError> { /* ... */ }
  ```
  Tries to receive the next message without notifying a context if empty.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Freeze**
- **TryStream**
  - ```rust
    fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<''_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **FusedStream**
  - ```rust
    fn is_terminated(self: &Self) -> bool { /* ... */ }
    ```

- **Stream**
  - ```rust
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Option<T>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **UnwindSafe**
#### Struct `SendError`

The error type for [`Sender`s](Sender) used as `Sink`s.

```rust
pub struct SendError {
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
  pub fn is_full(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this error is a result of the channel being full.

- ```rust
  pub fn is_disconnected(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this error is a result of the receiver being dropped.

###### Trait Implementations

- **Error**
- **Freeze**
- **UnwindSafe**
- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SendError) -> bool { /* ... */ }
    ```

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SendError { /* ... */ }
    ```

- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `TrySendError`

The error type returned from [`try_send`](Sender::try_send).

```rust
pub struct TrySendError<T> {
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
  pub fn is_full(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this error is a result of the channel being full.

- ```rust
  pub fn is_disconnected(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this error is a result of the receiver being dropped.

- ```rust
  pub fn into_inner(self: Self) -> T { /* ... */ }
  ```
  Returns the message that was attempted to be sent but failed.

- ```rust
  pub fn into_send_error(self: Self) -> SendError { /* ... */ }
  ```
  Drops the message and converts into a `SendError`.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Error**
- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TrySendError<T>) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TrySendError<T> { /* ... */ }
    ```

- **Sync**
- **Send**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Struct `TryRecvError`

The error type returned from [`try_next`](Receiver::try_next).

```rust
pub struct TryRecvError {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Sync**
- **Unpin**
- **RefUnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
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
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Error**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Functions

#### Function `channel`

Creates a bounded mpsc channel for communicating between asynchronous tasks.

Being bounded, this channel provides backpressure to ensure that the sender
outpaces the receiver by only a limited amount. The channel's capacity is
equal to `buffer + num-senders`. In other words, each sender gets a
guaranteed slot in the channel capacity, and on top of that there are
`buffer` "first come, first serve" slots available to all senders.

The [`Receiver`] returned implements the [`Stream`] trait, while [`Sender`]
implements `Sink`.

```rust
pub fn channel<T>(buffer: usize) -> (Sender<T>, Receiver<T>) { /* ... */ }
```

#### Function `unbounded`

Creates an unbounded mpsc channel for communicating between asynchronous
tasks.

A `send` on this channel will always succeed as long as the receive half has
not been closed. If the receiver falls behind, messages will be arbitrarily
buffered.

**Note** that the amount of available system memory is an implicit bound to
the channel. Using an `unbounded` channel has the ability of causing the
process to run out of memory. In this case, the process will be aborted.

```rust
pub fn unbounded<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>) { /* ... */ }
```

## Module `oneshot`

**Attributes:**

- `#[<cfg_attr>(target_os = "none", cfg(target_has_atomic = "ptr"))]`
- `#[<cfg>(feature = "alloc")]`

A channel for sending a single message between asynchronous tasks.

This is a single-producer, single-consumer channel.

```rust
pub mod oneshot { /* ... */ }
```

### Types

#### Struct `Receiver`

**Attributes:**

- `#[must_use = "futures do nothing unless you `.await` or poll them"]`

A future for a value that will be provided by another asynchronous task.

This is created by the [`channel`] function.

```rust
pub struct Receiver<T> {
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
  pub fn close(self: &mut Self) { /* ... */ }
  ```
  Gracefully close this receiver, preventing any subsequent attempts to

- ```rust
  pub fn try_recv(self: &mut Self) -> Result<Option<T>, Canceled> { /* ... */ }
  ```
  Attempts to receive a message outside of the context of a task.

###### Trait Implementations

- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Result<T, Canceled>> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **TryFuture**
  - ```rust
    fn try_poll(self: Pin<&mut F>, cx: &mut Context<''_>) -> Poll<<F as Future>::Output> { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **Send**
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

- **FusedFuture**
  - ```rust
    fn is_terminated(self: &Self) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Sender`

A means of transmitting a single value to another task.

This is created by the [`channel`] function.

```rust
pub struct Sender<T> {
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
  pub fn send(self: Self, t: T) -> Result<(), T> { /* ... */ }
  ```
  Completes this oneshot with a successful result.

- ```rust
  pub fn poll_canceled(self: &mut Self, cx: &mut Context<''_>) -> Poll<()> { /* ... */ }
  ```
  Polls this `Sender` half to detect whether its associated

- ```rust
  pub fn cancellation(self: &mut Self) -> Cancellation<''_, T> { /* ... */ }
  ```
  Creates a future that resolves when this `Sender`'s corresponding

- ```rust
  pub fn is_canceled(self: &Self) -> bool { /* ... */ }
  ```
  Tests to see whether this `Sender`'s corresponding `Receiver`

- ```rust
  pub fn is_connected_to(self: &Self, receiver: &Receiver<T>) -> bool { /* ... */ }
  ```
  Tests to see whether this `Sender` is connected to the given `Receiver`. That is, whether

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Send**
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
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `Cancellation`

**Attributes:**

- `#[must_use = "futures do nothing unless you `.await` or poll them"]`

A future that resolves when the receiving end of a channel has hung up.

This is an `.await`-friendly interface around [`poll_canceled`](Sender::poll_canceled).

```rust
pub struct Cancellation<''a, T> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<()> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

#### Struct `Canceled`

Error returned from a [`Receiver`] when the corresponding [`Sender`] is
dropped.

```rust
pub struct Canceled;
```

##### Implementations

###### Trait Implementations

- **Unpin**
- **Send**
- **RefUnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Canceled) -> bool { /* ... */ }
    ```

- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **Error**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Canceled { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Functions

#### Function `channel`

Creates a new one-shot channel for sending a single value across asynchronous tasks.

The channel works for a spsc (single-producer, single-consumer) scheme.

This function is similar to Rust's channel constructor found in the standard
library. Two halves are returned, the first of which is a `Sender` handle,
used to signal the end of a computation and provide its value. The second
half is a `Receiver` which implements the `Future` trait, resolving to the
value that was given to the `Sender` handle.

Each half can be separately owned and sent across tasks.

# Examples

```
use futures::channel::oneshot;
use std::{thread, time::Duration};

let (sender, receiver) = oneshot::channel::<i32>();

thread::spawn(|| {
    println!("THREAD: sleeping zzz...");
    thread::sleep(Duration::from_millis(1000));
    println!("THREAD: i'm awake! sending.");
    sender.send(3).unwrap();
});

println!("MAIN: doing some useful stuff");

futures::executor::block_on(async {
    println!("MAIN: waiting for msg...");
    println!("MAIN: got: {:?}", receiver.await)
});
```

```rust
pub fn channel<T>() -> (Sender<T>, Receiver<T>) { /* ... */ }
```

