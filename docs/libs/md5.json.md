# Crate Documentation

**Version:** 0.7.0

**Format Version:** 43

# Module `md5`

The [MD5] hash function.

## Example

```
let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
```

## Security Warning

The package is provided for the purposes of interoperability with protocols
and systems that mandate the use of MD5. However, MD5 should be considered
[cryptographically broken and unsuitable for further use][VU836068].
Collision attacks against MD5 are both practical and trivial, and
[theoretical attacks against MD5 have been found][ACM1724151].

[RFC6151] advises no new protocols to be designed with any MD5-based
constructions, including HMAC-MD5.

[MD5]: https://en.wikipedia.org/wiki/MD5

[ACM1724151]: https://dl.acm.org/citation.cfm?id=1724151
[RFC6151]: https://tools.ietf.org/html/rfc6151
[VU836068]: https://www.kb.cert.org/vuls/id/836068

## Types

### Struct `Digest`

A digest.

```rust
pub struct Digest(pub [u8; 16]);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `[u8; 16]` |  |

#### Implementations

##### Trait Implementations

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Digest) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Digest { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Receiver**
- **UnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(digest: Digest) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(context: Context) -> Digest { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **Send**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Struct `Context`

A context.

```rust
pub struct Context {
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
  pub fn new() -> Context { /* ... */ }
  ```
  Create a context for computing a digest.

- ```rust
  pub fn consume<T: AsRef<[u8]>>(self: &mut Self, data: T) { /* ... */ }
  ```
  Consume data.

- ```rust
  pub fn compute(self: Self) -> Digest { /* ... */ }
  ```
  Finalize and return the digest.

##### Trait Implementations

- **Freeze**
- **Unpin**
- **UnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, data: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Context { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

  - ```rust
    fn from(context: Context) -> Digest { /* ... */ }
    ```

- **RefUnwindSafe**
## Functions

### Function `compute`

**Attributes:**

- `#[inline]`

Compute the digest of data.

```rust
pub fn compute<T: AsRef<[u8]>>(data: T) -> Digest { /* ... */ }
```

