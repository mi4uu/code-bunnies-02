# Crate Documentation

**Version:** 0.5.9

**Format Version:** 43

# Module `socket2`

Utilities for creating and using sockets.

The goal of this crate is to create and use a socket using advanced
configuration options (those that are not available in the types in the
standard library) without using any unsafe code.

This crate provides as direct as possible access to the system's
functionality for sockets, this means little effort to provide
cross-platform utilities. It is up to the user to know how to use sockets
when using this crate. *If you don't know how to create a socket using
libc/system calls then this crate is not for you*. Most, if not all,
functions directly relate to the equivalent system call with no error
handling applied, so no handling errors such as [`EINTR`]. As a result using
this crate can be a little wordy, but it should give you maximal flexibility
over configuration of sockets.

[`EINTR`]: std::io::ErrorKind::Interrupted

# Examples

```no_run
# fn main() -> std::io::Result<()> {
use std::net::{SocketAddr, TcpListener};
use socket2::{Socket, Domain, Type};

// Create a TCP listener bound to two addresses.
let socket = Socket::new(Domain::IPV6, Type::STREAM, None)?;

socket.set_only_v6(false)?;
let address: SocketAddr = "[::1]:12345".parse().unwrap();
socket.bind(&address.into())?;
socket.listen(128)?;

let listener: TcpListener = socket.into();
// ...
# drop(listener);
# Ok(()) }
```

## Features

This crate has a single feature `all`, which enables all functions even ones
that are not available on all OSs.

## Types

### Struct `Domain`

Specification of the communication domain for a socket.

This is a newtype wrapper around an integer which provides a nicer API in
addition to an injection point for documentation. Convenience constants such
as [`Domain::IPV4`], [`Domain::IPV6`], etc, are provided to avoid reaching
into libc for various constants.

This type is freely interconvertible with C's `int` type, however, if a raw
value needs to be provided.

```rust
pub struct Domain(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn for_address(address: SocketAddr) -> Domain { /* ... */ }
  ```
  Returns the correct domain for `address`.

##### Trait Implementations

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(d: c_int) -> Domain { /* ... */ }
    ```

  - ```rust
    fn from(d: Domain) -> c_int { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Domain) -> bool { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Domain { /* ... */ }
    ```

- **Send**
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

### Struct `Type`

Specification of communication semantics on a socket.

This is a newtype wrapper around an integer which provides a nicer API in
addition to an injection point for documentation. Convenience constants such
as [`Type::STREAM`], [`Type::DGRAM`], etc, are provided to avoid reaching
into libc for various constants.

This type is freely interconvertible with C's `int` type, however, if a raw
value needs to be provided.

```rust
pub struct Type(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn nonblocking(self: Self) -> Type { /* ... */ }
  ```
  Set `SOCK_NONBLOCK` on the `Type`.

- ```rust
  pub const fn cloexec(self: Self) -> Type { /* ... */ }
  ```
  Set `SOCK_CLOEXEC` on the `Type`.

##### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Type) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **StructuralPartialEq**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(t: c_int) -> Type { /* ... */ }
    ```

  - ```rust
    fn from(t: Type) -> c_int { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **Unpin**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Type { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Struct `Protocol`

Protocol specification used for creating sockets via `Socket::new`.

This is a newtype wrapper around an integer which provides a nicer API in
addition to an injection point for documentation.

This type is freely interconvertible with C's `int` type, however, if a raw
value needs to be provided.

```rust
pub struct Protocol(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Protocol { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(p: c_int) -> Protocol { /* ... */ }
    ```

  - ```rust
    fn from(p: Protocol) -> c_int { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Protocol) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `RecvFlags`

**Attributes:**

- `#[<cfg>(not(target_os = "redox"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(not(target_os = "redox"))))]`

Flags for incoming messages.

Flags provide additional information about incoming messages.

```rust
pub struct RecvFlags(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn is_end_of_record(self: Self) -> bool { /* ... */ }
  ```
  Check if the message terminates a record.

- ```rust
  pub const fn is_out_of_band(self: Self) -> bool { /* ... */ }
  ```
  Check if the message contains out-of-band data.

- ```rust
  pub const fn is_confirm(self: Self) -> bool { /* ... */ }
  ```
  Check if the confirm flag is set.

- ```rust
  pub const fn is_dontroute(self: Self) -> bool { /* ... */ }
  ```
  Check if the don't route flag is set.

- ```rust
  pub const fn is_truncated(self: Self) -> bool { /* ... */ }
  ```
  Check if the message contains a truncated datagram.

##### Trait Implementations

- **Eq**
- **RefUnwindSafe**
- **UnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RecvFlags) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
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
    fn clone(self: &Self) -> RecvFlags { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Struct `MaybeUninitSlice`

A version of [`IoSliceMut`] that allows the buffer to be uninitialised.

[`IoSliceMut`]: std::io::IoSliceMut

```rust
pub struct MaybeUninitSlice<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn new(buf: &''a mut [MaybeUninit<u8>]) -> MaybeUninitSlice<''a> { /* ... */ }
  ```
  Creates a new `MaybeUninitSlice` wrapping a byte slice.

##### Trait Implementations

- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Receiver**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &[MaybeUninit<u8>] { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut [MaybeUninit<u8>] { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Send**
### Struct `TcpKeepalive`

Configures a socket's TCP keepalive parameters.

See [`Socket::set_tcp_keepalive`].

```rust
pub struct TcpKeepalive {
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
  pub const fn new() -> TcpKeepalive { /* ... */ }
  ```
  Returns a new, empty set of TCP keepalive parameters.

- ```rust
  pub const fn with_time(self: Self, time: Duration) -> Self { /* ... */ }
  ```
  Set the amount of time after which TCP keepalive probes will be sent on

- ```rust
  pub const fn with_interval(self: Self, interval: Duration) -> Self { /* ... */ }
  ```
  Set the value of the `TCP_KEEPINTVL` option. On Windows, this sets the

- ```rust
  pub const fn with_retries(self: Self, retries: u32) -> Self { /* ... */ }
  ```
  Set the value of the `TCP_KEEPCNT` option.

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TcpKeepalive { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
### Struct `MsgHdr`

**Attributes:**

- `#[<cfg>(not(target_os = "redox"))]`

Configuration of a `sendmsg(2)` system call.

This wraps `msghdr` on Unix and `WSAMSG` on Windows. Also see [`MsgHdrMut`]
for the variant used by `recvmsg(2)`.

```rust
pub struct MsgHdr<''addr, ''bufs, ''control> {
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
  pub fn new() -> MsgHdr<''addr, ''bufs, ''control> { /* ... */ }
  ```
  Create a new `MsgHdr` with all empty/zero fields.

- ```rust
  pub fn with_addr(self: Self, addr: &''addr SockAddr) -> Self { /* ... */ }
  ```
  Set the address (name) of the message.

- ```rust
  pub fn with_buffers(self: Self, bufs: &''bufs [IoSlice<''_>]) -> Self { /* ... */ }
  ```
  Set the buffer(s) of the message.

- ```rust
  pub fn with_control(self: Self, buf: &''control [u8]) -> Self { /* ... */ }
  ```
  Set the control buffer of the message.

- ```rust
  pub fn with_flags(self: Self, flags: libc::c_int) -> Self { /* ... */ }
  ```
  Set the flags of the message.

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Struct `MsgHdrMut`

**Attributes:**

- `#[<cfg>(not(target_os = "redox"))]`

Configuration of a `recvmsg(2)` system call.

This wraps `msghdr` on Unix and `WSAMSG` on Windows. Also see [`MsgHdr`] for
the variant used by `sendmsg(2)`.

```rust
pub struct MsgHdrMut<''addr, ''bufs, ''control> {
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
  pub fn new() -> MsgHdrMut<''addr, ''bufs, ''control> { /* ... */ }
  ```
  Create a new `MsgHdrMut` with all empty/zero fields.

- ```rust
  pub fn with_addr(self: Self, addr: &''addr mut SockAddr) -> Self { /* ... */ }
  ```
  Set the mutable address (name) of the message.

- ```rust
  pub fn with_buffers(self: Self, bufs: &''bufs mut [MaybeUninitSlice<''_>]) -> Self { /* ... */ }
  ```
  Set the mutable buffer(s) of the message.

- ```rust
  pub fn with_control(self: Self, buf: &''control mut [MaybeUninit<u8>]) -> Self { /* ... */ }
  ```
  Set the mutable control buffer of the message.

- ```rust
  pub fn flags(self: &Self) -> RecvFlags { /* ... */ }
  ```
  Returns the flags of the message.

- ```rust
  pub fn control_len(self: &Self) -> usize { /* ... */ }
  ```
  Gets the length of the control buffer.

##### Trait Implementations

- **Unpin**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **RefUnwindSafe**
## Re-exports

### Re-export `SockAddr`

```rust
pub use sockaddr::SockAddr;
```

### Re-export `Socket`

```rust
pub use socket::Socket;
```

### Re-export `SockRef`

```rust
pub use sockref::SockRef;
```

### Re-export `InterfaceIndexOrAddress`

**Attributes:**

- `#[<cfg>(not(any(target_os = "haiku", target_os = "illumos", target_os =
"netbsd", target_os = "redox", target_os = "solaris",)))]`

```rust
pub use socket::InterfaceIndexOrAddress;
```

