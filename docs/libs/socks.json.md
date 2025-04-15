# Crate Documentation

**Version:** 0.3.4

**Format Version:** 43

# Module `socks`

SOCKS proxy clients

## Types

### Enum `TargetAddr`

A description of a connection target.

```rust
pub enum TargetAddr {
    Ip(std::net::SocketAddr),
    Domain(String, u16),
}
```

#### Variants

##### `Ip`

Connect to an IP address.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::net::SocketAddr` |  |

##### `Domain`

Connect to a fully qualified domain name.

The domain name will be passed along to the proxy server and DNS lookup
will happen there.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |
| 1 | `u16` |  |

#### Implementations

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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
- **RefUnwindSafe**
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

- **ToTargetAddr**
  - ```rust
    fn to_target_addr(self: &Self) -> io::Result<TargetAddr> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **ToSocketAddrs**
  - ```rust
    fn to_socket_addrs(self: &Self) -> io::Result<Iter> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TargetAddr { /* ... */ }
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

### Struct `Iter`

An iterator over `SocketAddr`s associated with a `TargetAddr`.

```rust
pub struct Iter(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

- **RefUnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<SocketAddr> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
## Traits

### Trait `ToTargetAddr`

A trait for objects that can be converted to `TargetAddr`.

```rust
pub trait ToTargetAddr {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `to_target_addr`: Converts the value of `self` to a `TargetAddr`.

#### Implementations

This trait is implemented for the following types:

- `TargetAddr`
- `std::net::SocketAddr`
- `std::net::SocketAddrV4`
- `std::net::SocketAddrV6`
- `(std::net::Ipv4Addr, u16)`
- `(std::net::Ipv6Addr, u16)`
- `(&''a str, u16)` with <''a>
- `&''a str` with <''a>

## Re-exports

### Re-export `Socks4Stream`

```rust
pub use v4::Socks4Stream;
```

### Re-export `Socks4Listener`

```rust
pub use v4::Socks4Listener;
```

### Re-export `Socks5Stream`

```rust
pub use v5::Socks5Stream;
```

### Re-export `Socks5Listener`

```rust
pub use v5::Socks5Listener;
```

### Re-export `Socks5Datagram`

```rust
pub use v5::Socks5Datagram;
```

