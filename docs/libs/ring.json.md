# Crate Documentation

**Version:** 0.17.14

**Format Version:** 43

# Module `ring`

# Feature Flags

<table>
<tr><th>Feature
    <th>Description
<tr><td><code>alloc (default)</code>
    <td>Enable features that require use of the heap, RSA in particular.
<tr><td><code>less-safe-getrandom-custom-or-rdrand</code>
    <td>Treat user-provided ("custom") and RDRAND-based <code>getrandom</code>
        implementations as secure random number generators (see
        <code>SecureRandom</code>). This feature only works with
        <code>os = "none"</code> targets. See
        <a href="https://docs.rs/getrandom/0.2.10/getrandom/macro.register_custom_getrandom.html">
            <code>register_custom_getrandom</code>
        </a> and <a href="https://docs.rs/getrandom/0.2.10/getrandom/#rdrand-on-x86">
            RDRAND on x86
        </a> for additional details.
<tr><td><code>less-safe-getrandom-espidf</code>
    <td>Treat getrandom as a secure random number generator (see
        <code>SecureRandom</code>) on the esp-idf target. While the esp-idf
        target does have hardware RNG, it is beyond the scope of ring to
        ensure its configuration. This feature allows ring to build
        on esp-idf despite the likelihood that RNG is not secure.
        This feature only works with <code>os = espidf</code> targets.
        See <a href="https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/random.html">
<tr><td><code>std</code>
    <td>Enable features that use libstd, in particular
        <code>std::error::Error</code> integration. Implies `alloc`.
<tr><td><code>wasm32_unknown_unknown_js</code>
    <td>When this feature is enabled, for the wasm32-unknown-unknown target,
        Web APIs will be used to implement features like `ring::rand` that
        require an operating environment of some kind. This has no effect
        for any other target. This enables the `getrandom` crate's `js`
        feature.
</table>

## Modules

## Module `aead`

Authenticated Encryption with Associated Data (AEAD).

See [Authenticated encryption: relations among notions and analysis of the
generic composition paradigm][AEAD] for an introduction to the concept of
AEADs.

[AEAD]: https://eprint.iacr.org/2000/025.pdf
[`crypto.cipher.AEAD`]: https://golang.org/pkg/crypto/cipher/#AEAD

```rust
pub mod aead { /* ... */ }
```

### Modules

## Module `chacha20_poly1305_openssh`

The [chacha20-poly1305@openssh.com] AEAD-ish construct.

This should only be used by SSH implementations. It has a similar, but
different API from `ring::aead` because the construct cannot use the same
API as `ring::aead` due to the way the construct handles the encrypted
packet length.

The concatenation of a and b is denoted `a||b`. `K_1` and `K_2` are defined
in the [chacha20-poly1305@openssh.com] specification. `packet_length`,
`padding_length`, `payload`, and `random padding` are defined in
[RFC 4253]. The term `plaintext` is used as a shorthand for
`padding_length||payload||random padding`.

[chacha20-poly1305@openssh.com]:
   http://cvsweb.openbsd.org/cgi-bin/cvsweb/src/usr.bin/ssh/PROTOCOL.chacha20poly1305?annotate=HEAD
[RFC 4253]: https://tools.ietf.org/html/rfc4253

```rust
pub mod chacha20_poly1305_openssh { /* ... */ }
```

### Types

#### Struct `SealingKey`

A key for sealing packets.

```rust
pub struct SealingKey {
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
  pub fn new(key_material: &[u8; 64]) -> Self { /* ... */ }
  ```
  Constructs a new `SealingKey`.

- ```rust
  pub fn seal_in_place(self: &Self, sequence_number: u32, plaintext_in_ciphertext_out: &mut [u8], tag_out: &mut [u8; 16]) { /* ... */ }
  ```
  Seals (encrypts and signs) a packet.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `OpeningKey`

A key for opening packets.

```rust
pub struct OpeningKey {
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
  pub fn new(key_material: &[u8; 64]) -> Self { /* ... */ }
  ```
  Constructs a new `OpeningKey`.

- ```rust
  pub fn decrypt_packet_length(self: &Self, sequence_number: u32, encrypted_packet_length: [u8; 4]) -> [u8; 4] { /* ... */ }
  ```
  Returns the decrypted, but unauthenticated, packet length.

- ```rust
  pub fn open_in_place<''a>(self: &Self, sequence_number: u32, ciphertext_in_plaintext_out: &''a mut [u8], tag: &[u8; 16]) -> Result<&''a [u8], error::Unspecified> { /* ... */ }
  ```
  Opens (authenticates and decrypts) a packet.

###### Trait Implementations

- **Freeze**
- **UnwindSafe**
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

- **Send**
- **RefUnwindSafe**
- **Sync**
### Constants and Statics

#### Constant `KEY_LEN`

The length of key.

```rust
pub const KEY_LEN: usize = _;
```

#### Constant `PACKET_LENGTH_LEN`

The length in bytes of the `packet_length` field in a SSH packet.

```rust
pub const PACKET_LENGTH_LEN: usize = 4;
```

#### Constant `TAG_LEN`

The length in bytes of an authentication tag.

```rust
pub const TAG_LEN: usize = super::TAG_LEN;
```

## Module `quic`

QUIC Header Protection.

See draft-ietf-quic-tls.

```rust
pub mod quic { /* ... */ }
```

### Types

#### Struct `HeaderProtectionKey`

A key for generating QUIC Header Protection masks.

```rust
pub struct HeaderProtectionKey {
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
  pub fn new(algorithm: &''static Algorithm, key_bytes: &[u8]) -> Result<Self, error::Unspecified> { /* ... */ }
  ```
  Create a new header protection key.

- ```rust
  pub fn new_mask(self: &Self, sample: &[u8]) -> Result<[u8; 5], error::Unspecified> { /* ... */ }
  ```
  Generate a new QUIC Header Protection mask.

- ```rust
  pub fn algorithm(self: &Self) -> &''static Algorithm { /* ... */ }
  ```
  The key's algorithm.

###### Trait Implementations

- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(okm: hkdf::Okm<''_, &''static Algorithm>) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Type Alias `Sample`

QUIC sample for new key masks

```rust
pub type Sample = [u8; 16];
```

#### Struct `Algorithm`

A QUIC Header Protection Algorithm.

```rust
pub struct Algorithm {
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
  pub fn key_len(self: &Self) -> usize { /* ... */ }
  ```
  The length of the key.

- ```rust
  pub fn sample_len(self: &Self) -> usize { /* ... */ }
  ```
  The required sample length.

###### Trait Implementations

- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> Result<(), ::core::fmt::Error> { /* ... */ }
    ```

- **KeyType**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Constants and Statics

#### Static `AES_128`

AES-128.

```rust
pub static AES_128: Algorithm = _;
```

#### Static `AES_256`

AES-256.

```rust
pub static AES_256: Algorithm = _;
```

#### Static `CHACHA20`

ChaCha20.

```rust
pub static CHACHA20: Algorithm = _;
```

### Types

#### Struct `Aad`

The additionally authenticated data (AAD) for an opening or sealing
operation. This data is authenticated but is **not** encrypted.

The type `A` could be a byte slice `&[u8]`, a byte array `[u8; N]`
for some constant `N`, `Vec<u8>`, etc.

```rust
pub struct Aad<A>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from(aad: A) -> Self { /* ... */ }
  ```
  Construct the `Aad` from the given bytes.

- ```rust
  pub fn empty() -> Self { /* ... */ }
  ```
  Construct an empty `Aad`.

###### Trait Implementations

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Aad<A> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
#### Struct `Tag`

**Attributes:**

- `#[must_use]`
- `#[repr(C)]`

A possibly valid authentication tag.

```rust
pub struct Tag(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

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

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: &[u8]) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Send**
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

  - ```rust
    fn from(value: [u8; 16]) -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Tag { /* ... */ }
    ```

### Traits

#### Trait `NonceSequence`

A sequences of unique nonces.

A given `NonceSequence` must never return the same `Nonce` twice from
`advance()`.

A simple counter is a reasonable (but probably not ideal) `NonceSequence`.

Intentionally not `Clone` or `Copy` since cloning would allow duplication
of the sequence.

```rust
pub trait NonceSequence {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `advance`: Returns the next nonce in the sequence.

#### Trait `BoundKey`

An AEAD key bound to a nonce sequence.

```rust
pub trait BoundKey<N: NonceSequence>: core::fmt::Debug {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `new`: Constructs a new key from the given `UnboundKey` and `NonceSequence`.
- `algorithm`: The key's AEAD algorithm.

##### Implementations

This trait is implemented for the following types:

- `OpeningKey<N>` with <N: NonceSequence>
- `SealingKey<N>` with <N: NonceSequence>

### Constants and Statics

#### Constant `MAX_TAG_LEN`

The maximum length of a tag for the algorithms in this module.

```rust
pub const MAX_TAG_LEN: usize = TAG_LEN;
```

### Re-exports

#### Re-export `Algorithm`

```rust
pub use self::algorithm::Algorithm;
```

#### Re-export `AES_128_GCM`

```rust
pub use self::algorithm::AES_128_GCM;
```

#### Re-export `AES_256_GCM`

```rust
pub use self::algorithm::AES_256_GCM;
```

#### Re-export `CHACHA20_POLY1305`

```rust
pub use self::algorithm::CHACHA20_POLY1305;
```

#### Re-export `LessSafeKey`

```rust
pub use self::less_safe_key::LessSafeKey;
```

#### Re-export `Nonce`

```rust
pub use self::nonce::Nonce;
```

#### Re-export `NONCE_LEN`

```rust
pub use self::nonce::NONCE_LEN;
```

#### Re-export `OpeningKey`

```rust
pub use self::opening_key::OpeningKey;
```

#### Re-export `SealingKey`

```rust
pub use self::sealing_key::SealingKey;
```

#### Re-export `UnboundKey`

```rust
pub use self::unbound_key::UnboundKey;
```

## Module `agreement`

Key Agreement: ECDH, including X25519.

# Example

Note that this example uses X25519, but ECDH using NIST P-256/P-384 is done
exactly the same way, just substituting
`agreement::ECDH_P256`/`agreement::ECDH_P384` for `agreement::X25519`.

```
use ring::{agreement, rand};

let rng = rand::SystemRandom::new();

let my_private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;

// Make `my_public_key` a byte slice containing my public key. In a real
// application, this would be sent to the peer in an encoded protocol
// message.
let my_public_key = my_private_key.compute_public_key()?;

let peer_public_key_bytes = {
    // In a real application, the peer public key would be parsed out of a
    // protocol message. Here we just generate one.
    let peer_private_key =
        agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;
    peer_private_key.compute_public_key()?
};

let peer_public_key = agreement::UnparsedPublicKey::new(
    &agreement::X25519,
    peer_public_key_bytes);

agreement::agree_ephemeral(
    my_private_key,
    &peer_public_key,
    |_key_material| {
        // In a real application, we'd apply a KDF to the key material and the
        // public keys (as recommended in RFC 7748) and then derive session
        // keys from the result. We omit all that here.
    },
)?;

# Ok::<(), ring::error::Unspecified>(())
```

```rust
pub mod agreement { /* ... */ }
```

### Types

#### Struct `Algorithm`

A key agreement algorithm.

```rust
pub struct Algorithm {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> Result<(), ::core::fmt::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `EphemeralPrivateKey`

An ephemeral private key for use (only) with `agree_ephemeral`. The
signature of `agree_ephemeral` ensures that an `EphemeralPrivateKey` can be
used for at most one key agreement.

```rust
pub struct EphemeralPrivateKey {
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
  pub fn generate(alg: &''static Algorithm, rng: &dyn rand::SecureRandom) -> Result<Self, error::Unspecified> { /* ... */ }
  ```
  Generate a new ephemeral private key for the given algorithm.

- ```rust
  pub fn compute_public_key(self: &Self) -> Result<PublicKey, error::Unspecified> { /* ... */ }
  ```
  Computes the public key from the private key.

- ```rust
  pub fn algorithm(self: &Self) -> &''static Algorithm { /* ... */ }
  ```
  The algorithm for the private key.

###### Trait Implementations

- **Sync**
- **Freeze**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> Result<(), ::core::fmt::Error> { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `PublicKey`

A public key for key agreement.

```rust
pub struct PublicKey {
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
  pub fn algorithm(self: &Self) -> &''static Algorithm { /* ... */ }
  ```
  The algorithm for the public key.

###### Trait Implementations

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> Result<(), core::fmt::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PublicKey { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
#### Struct `UnparsedPublicKey`

An unparsed, possibly malformed, public key for key agreement.

```rust
pub struct UnparsedPublicKey<B> {
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
  pub fn new(algorithm: &''static Algorithm, bytes: B) -> Self { /* ... */ }
  ```
  Constructs a new `UnparsedPublicKey`.

- ```rust
  pub fn algorithm(self: &Self) -> &''static Algorithm { /* ... */ }
  ```
  The algorithm for the public key.

- ```rust
  pub fn bytes(self: &Self) -> &B { /* ... */ }
  ```
  TODO: doc

###### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> Result<(), core::fmt::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Send**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> UnparsedPublicKey<B> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Functions

#### Function `agree_ephemeral`

**Attributes:**

- `#[inline]`

Performs a key agreement with an ephemeral private key and the given public
key.

`my_private_key` is the ephemeral private key to use. Since it is moved, it
will not be usable after calling `agree_ephemeral`, thus guaranteeing that
the key is used for only one key agreement.

`peer_public_key` is the peer's public key. `agree_ephemeral` will return
`Err(error_value)` if it does not match `my_private_key's` algorithm/curve.
`agree_ephemeral` verifies that it is encoded in the standard form for the
algorithm and that the key is *valid*; see the algorithm's documentation for
details on how keys are to be encoded and what constitutes a valid key for
that algorithm.

After the key agreement is done, `agree_ephemeral` calls `kdf` with the raw
key material from the key agreement operation and then returns what `kdf`
returns.

```rust
pub fn agree_ephemeral<B: AsRef<[u8]>, R, /* synthetic */ impl FnOnce(&[u8]) -> R: FnOnce(&[u8]) -> R>(my_private_key: EphemeralPrivateKey, peer_public_key: &UnparsedPublicKey<B>, kdf: impl FnOnce(&[u8]) -> R) -> Result<R, error::Unspecified> { /* ... */ }
```

### Re-exports

#### Re-export `X25519`

```rust
pub use crate::ec::curve25519::x25519::X25519;
```

#### Re-export `ECDH_P256`

```rust
pub use crate::ec::suite_b::ecdh::ECDH_P256;
```

#### Re-export `ECDH_P384`

```rust
pub use crate::ec::suite_b::ecdh::ECDH_P384;
```

## Module `io`

Serialization and deserialization.

```rust
pub mod io { /* ... */ }
```

### Re-exports

#### Re-export `Positive`

```rust
pub use self::positive::Positive;
```

## Module `digest`

SHA-2 and the legacy SHA-1 digest algorithm.

If all the data is available in a single contiguous slice then the `digest`
function should be used. Otherwise, the digest can be calculated in
multiple steps using `Context`.

```rust
pub mod digest { /* ... */ }
```

### Types

#### Struct `Context`

A context for multi-step (Init-Update-Finish) digest calculations.

# Examples

```
use ring::digest;

let one_shot = digest::digest(&digest::SHA384, b"hello, world");

let mut ctx = digest::Context::new(&digest::SHA384);
ctx.update(b"hello");
ctx.update(b", ");
ctx.update(b"world");
let multi_part = ctx.finish();

assert_eq!(&one_shot.as_ref(), &multi_part.as_ref());
```

```rust
pub struct Context {
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
  pub fn new(algorithm: &''static Algorithm) -> Self { /* ... */ }
  ```
  Constructs a new context.

- ```rust
  pub fn update(self: &mut Self, data: &[u8]) { /* ... */ }
  ```
  Updates the digest with all the data in `data`.

- ```rust
  pub fn finish(self: Self) -> Digest { /* ... */ }
  ```
  Finalizes the digest calculation and returns the digest value.

- ```rust
  pub fn algorithm(self: &Self) -> &''static Algorithm { /* ... */ }
  ```
  The algorithm that this context is using.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Context { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Freeze**
- **Send**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `Digest`

A calculated digest value.

Use [`Self::as_ref`] to get the value as a `&[u8]`.

```rust
pub struct Digest {
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
  pub fn algorithm(self: &Self) -> &''static Algorithm { /* ... */ }
  ```
  The algorithm that was used to calculate the digest value.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Sync**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Digest { /* ... */ }
    ```

- **Send**
#### Struct `Algorithm`

A digest algorithm.

```rust
pub struct Algorithm {
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
  pub fn block_len(self: &Self) -> usize { /* ... */ }
  ```
  The internal block length.

- ```rust
  pub fn chaining_len(self: &Self) -> usize { /* ... */ }
  ```
  The size of the chaining value of the digest function, in bytes.

- ```rust
  pub fn output_len(self: &Self) -> usize { /* ... */ }
  ```
  The length of a finalized digest.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> Result<(), ::core::fmt::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
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

- **Unpin**
- **Freeze**
- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Sync**
- **RefUnwindSafe**
- **UnwindSafe**
### Functions

#### Function `digest`

Returns the digest of `data` using the given digest algorithm.

```rust
pub fn digest(algorithm: &''static Algorithm, data: &[u8]) -> Digest { /* ... */ }
```

### Constants and Statics

#### Static `SHA1_FOR_LEGACY_USE_ONLY`

SHA-1 as specified in [FIPS 180-4]. Deprecated.

[FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf

```rust
pub static SHA1_FOR_LEGACY_USE_ONLY: Algorithm = _;
```

#### Static `SHA256`

SHA-256 as specified in [FIPS 180-4].

[FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf

```rust
pub static SHA256: Algorithm = _;
```

#### Static `SHA384`

SHA-384 as specified in [FIPS 180-4].

[FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf

```rust
pub static SHA384: Algorithm = _;
```

#### Static `SHA512`

SHA-512 as specified in [FIPS 180-4].

[FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf

```rust
pub static SHA512: Algorithm = _;
```

#### Static `SHA512_256`

SHA-512/256 as specified in [FIPS 180-4].

This is *not* the same as just truncating the output of SHA-512, as
SHA-512/256 has its own initial state distinct from SHA-512's initial
state.

[FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf

```rust
pub static SHA512_256: Algorithm = _;
```

#### Constant `MAX_BLOCK_LEN`

The maximum block length ([`Algorithm::block_len()`]) of all the algorithms
in this module.

```rust
pub const MAX_BLOCK_LEN: usize = _;
```

#### Constant `MAX_OUTPUT_LEN`

The maximum output length ([`Algorithm::output_len()`]) of all the
algorithms in this module.

```rust
pub const MAX_OUTPUT_LEN: usize = _;
```

#### Constant `MAX_CHAINING_LEN`

The maximum chaining length ([`Algorithm::chaining_len()`]) of all the
algorithms in this module.

```rust
pub const MAX_CHAINING_LEN: usize = MAX_OUTPUT_LEN;
```

#### Constant `SHA1_OUTPUT_LEN`

The length of the output of SHA-1, in bytes.

```rust
pub const SHA1_OUTPUT_LEN: usize = _;
```

#### Constant `SHA256_OUTPUT_LEN`

The length of the output of SHA-256, in bytes.

```rust
pub const SHA256_OUTPUT_LEN: usize = _;
```

#### Constant `SHA384_OUTPUT_LEN`

The length of the output of SHA-384, in bytes.

```rust
pub const SHA384_OUTPUT_LEN: usize = _;
```

#### Constant `SHA512_OUTPUT_LEN`

The length of the output of SHA-512, in bytes.

```rust
pub const SHA512_OUTPUT_LEN: usize = _;
```

#### Constant `SHA512_256_OUTPUT_LEN`

The length of the output of SHA-512/256, in bytes.

```rust
pub const SHA512_256_OUTPUT_LEN: usize = _;
```

## Module `error`

Error reporting.

```rust
pub mod error { /* ... */ }
```

### Re-exports

#### Re-export `KeyRejected`

```rust
pub use self::key_rejected::KeyRejected;
```

#### Re-export `Unspecified`

```rust
pub use self::unspecified::Unspecified;
```

## Module `hkdf`

HMAC-based Extract-and-Expand Key Derivation Function.

HKDF is specified in [RFC 5869].

[RFC 5869]: https://tools.ietf.org/html/rfc5869

```rust
pub mod hkdf { /* ... */ }
```

### Types

#### Struct `Algorithm`

An HKDF algorithm.

```rust
pub struct Algorithm(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn hmac_algorithm(self: &Self) -> hmac::Algorithm { /* ... */ }
  ```
  The underlying HMAC algorithm.

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Algorithm) -> bool { /* ... */ }
    ```

- **Send**
- **KeyType**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Algorithm { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **Copy**
#### Struct `Salt`

A salt for HKDF operations.

```rust
pub struct Salt(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(algorithm: Algorithm, value: &[u8]) -> Self { /* ... */ }
  ```
  Constructs a new `Salt` with the given value based on the given digest

- ```rust
  pub fn extract(self: &Self, secret: &[u8]) -> Prk { /* ... */ }
  ```
  The [HKDF-Extract] operation.

- ```rust
  pub fn algorithm(self: &Self) -> Algorithm { /* ... */ }
  ```
  The algorithm used to derive this salt.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

  - ```rust
    fn from(okm: Okm<''_, Algorithm>) -> Self { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Freeze**
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Prk`

A HKDF PRK (pseudorandom key).

```rust
pub struct Prk(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new_less_safe(algorithm: Algorithm, value: &[u8]) -> Self { /* ... */ }
  ```
  Construct a new `Prk` directly with the given value.

- ```rust
  pub fn expand<''a, L: KeyType>(self: &''a Self, info: &''a [&''a [u8]], len: L) -> Result<Okm<''a, L>, error::Unspecified> { /* ... */ }
  ```
  The [HKDF-Expand] operation.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(okm: Okm<''_, Algorithm>) -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Prk { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

#### Struct `Okm`

An HKDF OKM (Output Keying Material)

Intentionally not `Clone` or `Copy` as an OKM is generally only safe to
use once.

```rust
pub struct Okm<''a, L: KeyType> {
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
  pub fn len(self: &Self) -> &L { /* ... */ }
  ```
  The `OkmLength` given to `Prk::expand()`.

- ```rust
  pub fn fill(self: Self, out: &mut [u8]) -> Result<(), error::Unspecified> { /* ... */ }
  ```
  Fills `out` with the output of the HKDF-Expand operation for the given

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
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
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(okm: hkdf::Okm<''_, &''static Algorithm>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(okm: hkdf::Okm<''_, &''static Algorithm>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(okm: Okm<''_, Algorithm>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(okm: Okm<''_, Algorithm>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(okm: hkdf::Okm<''_, Algorithm>) -> Self { /* ... */ }
    ```

### Traits

#### Trait `KeyType`

The length of the OKM (Output Keying Material) for a `Prk::expand()` call.

```rust
pub trait KeyType {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `len`: The length that `Prk::expand()` should expand its input to.

##### Implementations

This trait is implemented for the following types:

- `&''static Algorithm`
- `&''static Algorithm`
- `Algorithm`
- `Algorithm`

### Constants and Statics

#### Static `HKDF_SHA1_FOR_LEGACY_USE_ONLY`

HKDF using HMAC-SHA-1. Obsolete.

```rust
pub static HKDF_SHA1_FOR_LEGACY_USE_ONLY: Algorithm = _;
```

#### Static `HKDF_SHA256`

HKDF using HMAC-SHA-256.

```rust
pub static HKDF_SHA256: Algorithm = _;
```

#### Static `HKDF_SHA384`

HKDF using HMAC-SHA-384.

```rust
pub static HKDF_SHA384: Algorithm = _;
```

#### Static `HKDF_SHA512`

HKDF using HMAC-SHA-512.

```rust
pub static HKDF_SHA512: Algorithm = _;
```

## Module `hmac`

HMAC is specified in [RFC 2104].

After a `Key` is constructed, it can be used for multiple signing or
verification operations. Separating the construction of the key from the
rest of the HMAC operation allows the per-key precomputation to be done
only once, instead of it being done in every HMAC operation.

Frequently all the data to be signed in a message is available in a single
contiguous piece. In that case, the module-level `sign` function can be
used. Otherwise, if the input is in multiple parts, `Context` should be
used.

# Examples:

## Signing a value and verifying it wasn't tampered with

```
use ring::{hmac, rand};

let rng = rand::SystemRandom::new();
let key = hmac::Key::generate(hmac::HMAC_SHA256, &rng)?;

let msg = "hello, world";

let tag = hmac::sign(&key, msg.as_bytes());

// [We give access to the message to an untrusted party, and they give it
// back to us. We need to verify they didn't tamper with it.]

hmac::verify(&key, msg.as_bytes(), tag.as_ref())?;

# Ok::<(), ring::error::Unspecified>(())
```

## Using the one-shot API:

```
use ring::{digest, hmac, rand};
use ring::rand::SecureRandom;

let msg = "hello, world";

// The sender generates a secure key value and signs the message with it.
// Note that in a real protocol, a key agreement protocol would be used to
// derive `key_value`.
let rng = rand::SystemRandom::new();
let key_value: [u8; digest::SHA256_OUTPUT_LEN] = rand::generate(&rng)?.expose();

let s_key = hmac::Key::new(hmac::HMAC_SHA256, key_value.as_ref());
let tag = hmac::sign(&s_key, msg.as_bytes());

// The receiver (somehow!) knows the key value, and uses it to verify the
// integrity of the message.
let v_key = hmac::Key::new(hmac::HMAC_SHA256, key_value.as_ref());
hmac::verify(&v_key, msg.as_bytes(), tag.as_ref())?;

# Ok::<(), ring::error::Unspecified>(())
```

## Using the multi-part API:
```
use ring::{digest, hmac, rand};
use ring::rand::SecureRandom;

let parts = ["hello", ", ", "world"];

// The sender generates a secure key value and signs the message with it.
// Note that in a real protocol, a key agreement protocol would be used to
// derive `key_value`.
let rng = rand::SystemRandom::new();
let mut key_value: [u8; digest::SHA384_OUTPUT_LEN] = rand::generate(&rng)?.expose();

let s_key = hmac::Key::new(hmac::HMAC_SHA384, key_value.as_ref());
let mut s_ctx = hmac::Context::with_key(&s_key);
for part in &parts {
    s_ctx.update(part.as_bytes());
}
let tag = s_ctx.sign();

// The receiver (somehow!) knows the key value, and uses it to verify the
// integrity of the message.
let v_key = hmac::Key::new(hmac::HMAC_SHA384, key_value.as_ref());
let mut msg = Vec::<u8>::new();
for part in &parts {
    msg.extend(part.as_bytes());
}
hmac::verify(&v_key, &msg.as_ref(), tag.as_ref())?;

# Ok::<(), ring::error::Unspecified>(())
```

[RFC 2104]: https://tools.ietf.org/html/rfc2104

```rust
pub mod hmac { /* ... */ }
```

### Types

#### Struct `Algorithm`

An HMAC algorithm.

```rust
pub struct Algorithm(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn digest_algorithm(self: &Self) -> &''static digest::Algorithm { /* ... */ }
  ```
  The digest algorithm this HMAC algorithm is based on.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **KeyType**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Algorithm { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Algorithm) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Unpin**
- **Sync**
- **Freeze**
- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Tag`

An HMAC tag.

For a given tag `t`, use `t.as_ref()` to get the tag value as a byte slice.

```rust
pub struct Tag(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Tag { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **Copy**
#### Struct `Key`

A key to use for HMAC signing.

```rust
pub struct Key {
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
  pub fn generate(algorithm: Algorithm, rng: &dyn rand::SecureRandom) -> Result<Self, error::Unspecified> { /* ... */ }
  ```
  Generate an HMAC signing key using the given digest algorithm with a

- ```rust
  pub fn new(algorithm: Algorithm, key_value: &[u8]) -> Self { /* ... */ }
  ```
  Construct an HMAC signing key using the given digest algorithm and key

- ```rust
  pub fn algorithm(self: &Self) -> Algorithm { /* ... */ }
  ```
  The digest algorithm for the key.

###### Trait Implementations

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Key { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(okm: hkdf::Okm<''_, Algorithm>) -> Self { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> Result<(), core::fmt::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Context`

A context for multi-step (Init-Update-Finish) HMAC signing.

Use `sign` for single-step HMAC signing.

```rust
pub struct Context {
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
  pub fn with_key(signing_key: &Key) -> Self { /* ... */ }
  ```
  Constructs a new HMAC signing context using the given digest algorithm

- ```rust
  pub fn update(self: &mut Self, data: &[u8]) { /* ... */ }
  ```
  Updates the HMAC with all the data in `data`. `update` may be called

- ```rust
  pub fn sign(self: Self) -> Tag { /* ... */ }
  ```
  Finalizes the HMAC calculation and returns the HMAC value. `sign`

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Context { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> Result<(), core::fmt::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Send**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Functions

#### Function `sign`

Calculates the HMAC of `data` using the key `key` in one step.

Use `Context` to calculate HMACs where the input is in multiple parts.

It is generally not safe to implement HMAC verification by comparing the
return value of `sign` to a tag. Use `verify` for verification instead.

```rust
pub fn sign(key: &Key, data: &[u8]) -> Tag { /* ... */ }
```

#### Function `verify`

Calculates the HMAC of `data` using the signing key `key`, and verifies
whether the resultant value equals `tag`, in one step.

This is logically equivalent to, but more efficient than, constructing a
`Key` with the same value as `key` and then using `verify`.

The verification will be done in constant time to prevent timing attacks.

```rust
pub fn verify(key: &Key, data: &[u8], tag: &[u8]) -> Result<(), error::Unspecified> { /* ... */ }
```

### Constants and Statics

#### Static `HMAC_SHA1_FOR_LEGACY_USE_ONLY`

HMAC using SHA-1. Obsolete.

```rust
pub static HMAC_SHA1_FOR_LEGACY_USE_ONLY: Algorithm = _;
```

#### Static `HMAC_SHA256`

HMAC using SHA-256.

```rust
pub static HMAC_SHA256: Algorithm = _;
```

#### Static `HMAC_SHA384`

HMAC using SHA-384.

```rust
pub static HMAC_SHA384: Algorithm = _;
```

#### Static `HMAC_SHA512`

HMAC using SHA-512.

```rust
pub static HMAC_SHA512: Algorithm = _;
```

## Module `pbkdf2`

PBKDF2 derivation and verification.

Use `derive` to derive PBKDF2 outputs. Use `verify` to verify secret
against previously-derived outputs.

PBKDF2 is specified in [RFC 2898 Section 5.2] with test vectors given in
[RFC 6070]. See also [NIST Special Publication 800-132].

[RFC 2898 Section 5.2]: https://tools.ietf.org/html/rfc2898#section-5.2
[RFC 6070]: https://tools.ietf.org/html/rfc6070
[NIST Special Publication 800-132]:
   http://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-132.pdf

# Examples

## Password Database Example

```
use ring::{digest, pbkdf2};
use std::{collections::HashMap, num::NonZeroU32};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

enum Error {
    WrongUsernameOrPassword
}

struct PasswordDatabase {
    pbkdf2_iterations: NonZeroU32,
    db_salt_component: [u8; 16],

    // Normally this would be a persistent database.
    storage: HashMap<String, Credential>,
}

impl PasswordDatabase {
    pub fn store_password(&mut self, username: &str, password: &str) {
        let salt = self.salt(username);
        let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(PBKDF2_ALG, self.pbkdf2_iterations, &salt,
                       password.as_bytes(), &mut to_store);
        self.storage.insert(String::from(username), to_store);
    }

    pub fn verify_password(&self, username: &str, attempted_password: &str)
                           -> Result<(), Error> {
        match self.storage.get(username) {
           Some(actual_password) => {
               let salt = self.salt(username);
               pbkdf2::verify(PBKDF2_ALG, self.pbkdf2_iterations, &salt,
                              attempted_password.as_bytes(),
                              actual_password)
                    .map_err(|_| Error::WrongUsernameOrPassword)
           },

           None => Err(Error::WrongUsernameOrPassword)
        }
    }

    // The salt should have a user-specific component so that an attacker
    // cannot crack one password for multiple users in the database. It
    // should have a database-unique component so that an attacker cannot
    // crack the same user's password across databases in the unfortunate
    // but common case that the user has used the same password for
    // multiple systems.
    fn salt(&self, username: &str) -> Vec<u8> {
        let mut salt = Vec::with_capacity(self.db_salt_component.len() +
                                          username.as_bytes().len());
        salt.extend(self.db_salt_component.as_ref());
        salt.extend(username.as_bytes());
        salt
    }
}

fn main() {
    // Normally these parameters would be loaded from a configuration file.
    let mut db = PasswordDatabase {
        pbkdf2_iterations: NonZeroU32::new(100_000).unwrap(),
        db_salt_component: [
            // This value was generated from a secure PRNG.
            0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52,
            0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39, 0x01, 0x8a
        ],
        storage: HashMap::new(),
    };

    db.store_password("alice", "@74d7]404j|W}6u");

    // An attempt to log in with the wrong password fails.
    assert!(db.verify_password("alice", "wrong password").is_err());

    // Normally there should be an expoentially-increasing delay between
    // attempts to further protect against online attacks.

    // An attempt to log in with the right password succeeds.
    assert!(db.verify_password("alice", "@74d7]404j|W}6u").is_ok());
}

```rust
pub mod pbkdf2 { /* ... */ }
```

### Types

#### Struct `Algorithm`

A PBKDF2 algorithm.

```rust
pub struct Algorithm(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Algorithm) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Algorithm { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **Unpin**
- **Sync**
- **Send**
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

- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

### Functions

#### Function `derive`

Fills `out` with the key derived using PBKDF2 with the given inputs.

Do not use `derive` as part of verifying a secret; use `verify` instead, to
minimize the effectiveness of timing attacks.

`out.len()` must be no larger than the digest length * (2**32 - 1), per the
PBKDF2 specification.

| Parameter   | RFC 2898 Section 5.2 Term
|-------------|-------------------------------------------
| digest_alg  | PRF (HMAC with the given digest algorithm)
| iterations  | c (iteration count)
| salt        | S (salt)
| secret      | P (password)
| out         | dk (derived key)
| out.len()   | dkLen (derived key length)

# Panics

Panics if `out.len() > u32::MAX * digest_alg.output_len()`, where
`digest_alg` is the underlying HMAC/digest algorithm.

Panics if `salt` is so astronomically gigantic that it isn't a valid input
to the underlying digest function.

Panics if `secret` is so astronomically gigantic that it isn't a valid
input to the underlying digest function.

```rust
pub fn derive(algorithm: Algorithm, iterations: core::num::NonZeroU32, salt: &[u8], secret: &[u8], out: &mut [u8]) { /* ... */ }
```

#### Function `verify`

Verifies that a previously-derived (e.g., using `derive`) PBKDF2 value
matches the PBKDF2 value derived from the other inputs.

The comparison is done in constant time to prevent timing attacks. The
comparison will fail if `previously_derived` is empty (has a length of
zero).

| Parameter                  | RFC 2898 Section 5.2 Term
|----------------------------|--------------------------------------------
| digest_alg                 | PRF (HMAC with the given digest algorithm).
| `iterations`               | c (iteration count)
| `salt`                     | S (salt)
| `secret`                   | P (password)
| `previously_derived`       | dk (derived key)
| `previously_derived.len()` | dkLen (derived key length)

```rust
pub fn verify(algorithm: Algorithm, iterations: core::num::NonZeroU32, salt: &[u8], secret: &[u8], previously_derived: &[u8]) -> Result<(), error::Unspecified> { /* ... */ }
```

### Constants and Statics

#### Static `PBKDF2_HMAC_SHA1`

PBKDF2 using HMAC-SHA1.

```rust
pub static PBKDF2_HMAC_SHA1: Algorithm = _;
```

#### Static `PBKDF2_HMAC_SHA256`

PBKDF2 using HMAC-SHA256.

```rust
pub static PBKDF2_HMAC_SHA256: Algorithm = _;
```

#### Static `PBKDF2_HMAC_SHA384`

PBKDF2 using HMAC-SHA384.

```rust
pub static PBKDF2_HMAC_SHA384: Algorithm = _;
```

#### Static `PBKDF2_HMAC_SHA512`

PBKDF2 using HMAC-SHA512.

```rust
pub static PBKDF2_HMAC_SHA512: Algorithm = _;
```

## Module `pkcs8`

PKCS#8 is specified in [RFC 5958].

[RFC 5958]: https://tools.ietf.org/html/rfc5958

```rust
pub mod pkcs8 { /* ... */ }
```

### Types

#### Struct `Document`

A generated PKCS#8 document.

```rust
pub struct Document {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Sync**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
## Module `rand`

Cryptographic pseudo-random number generation.

*ring* functions that generate random bytes take a `&dyn SecureRandom`
parameter to make it clear which functions are non-deterministic.

```rust
pub mod rand { /* ... */ }
```

### Types

#### Struct `Random`

A random value constructed from a `SecureRandom` that hasn't been exposed
through any safe Rust interface.

Intentionally does not implement any traits other than `Sized`.

```rust
pub struct Random<T: RandomlyConstructable>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn expose(self: Self) -> T { /* ... */ }
  ```
  Expose the random value.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **UnwindSafe**
- **Sync**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
#### Struct `SystemRandom`

A secure random number generator where the random values come directly
from the operating system.

"Directly from the operating system" here presently means "whatever the
`getrandom` crate does" but that may change in the future. That roughly
means calling libc's `getrandom` function or whatever is analogous to that;
see the `getrandom` crate's documentation for more info.

A single `SystemRandom` may be shared across multiple threads safely.

`new()` is guaranteed to always succeed and to have low latency; it won't
try to open or read from a file or do similar things. The first call to
`fill()` may block a substantial amount of time since any and all
initialization is deferred to it. Therefore, it may be a good idea to call
`fill()` once at a non-latency-sensitive time to minimize latency for
future calls.

```rust
pub struct SystemRandom(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Constructs a new `SystemRandom`.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SystemRandom { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **SecureRandom**
  - ```rust
    fn fill(self: &Self, dest: &mut [u8]) -> Result<(), Unspecified> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Sync**
### Traits

#### Trait `SecureRandom`

A secure random number generator.

```rust
pub trait SecureRandom: sealed::SecureRandom {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `fill`: Fills `dest` with random bytes.

##### Implementations

This trait is implemented for the following types:

- `T` with <T>

#### Trait `RandomlyConstructable`

A type that can be returned by `ring::rand::generate()`.

```rust
pub trait RandomlyConstructable: sealed::RandomlyConstructable {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `T` with <T>

### Functions

#### Function `generate`

**Attributes:**

- `#[inline]`

Generate the new random value using `rng`.

```rust
pub fn generate<T: RandomlyConstructable>(rng: &dyn SecureRandom) -> Result<Random<T>, error::Unspecified> { /* ... */ }
```

## Module `rsa`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

RSA.

```rust
pub mod rsa { /* ... */ }
```

### Types

#### Struct `RsaParameters`

Parameters for RSA verification.

```rust
pub struct RsaParameters {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **VerificationAlgorithm**
  - ```rust
    fn verify(self: &Self, public_key: untrusted::Input<''_>, msg: untrusted::Input<''_>, signature: untrusted::Input<''_>) -> Result<(), error::Unspecified> { /* ... */ }
    ```

### Re-exports

#### Re-export `KeyPair`

```rust
pub use self::keypair::KeyPair;
```

#### Re-export `KeyPairComponents`

```rust
pub use self::keypair_components::KeyPairComponents;
```

#### Re-export `PublicKey`

```rust
pub use self::public_key::PublicKey;
```

#### Re-export `PublicKeyComponents`

```rust
pub use self::public_key_components::PublicKeyComponents;
```

## Module `signature`

Public key signatures: signing and verification.

Use the `verify` function to verify signatures, passing a reference to the
algorithm that identifies the algorithm. See the documentation for `verify`
for examples.

For signature verification, this API treats each combination of parameters
as a separate algorithm. For example, instead of having a single "RSA"
algorithm with a verification function that takes a bunch of parameters,
there are `RSA_PKCS1_2048_8192_SHA256`, `RSA_PKCS1_2048_8192_SHA384`, etc.,
which encode sets of parameter choices into objects. This is designed to
reduce the risks of algorithm agility and to provide consistency with ECDSA
and EdDSA.

Currently this module does not support digesting the message to be signed
separately from the public key operation, as it is currently being
optimized for Ed25519 and for the implementation of protocols that do not
requiring signing large messages. An interface for efficiently supporting
larger messages may be added later.


# Algorithm Details

## `ECDSA_*_ASN1` Details: ASN.1-encoded ECDSA Signatures

The signature is a ASN.1 DER-encoded `Ecdsa-Sig-Value` as described in
[RFC 3279 Section 2.2.3]. This is the form of ECDSA signature used in
X.509-related structures and in TLS's `ServerKeyExchange` messages.

The public key is encoding in uncompressed form using the
Octet-String-to-Elliptic-Curve-Point algorithm in
[SEC 1: Elliptic Curve Cryptography, Version 2.0].

During verification, the public key is validated using the ECC Partial
Public-Key Validation Routine from Section 5.6.2.3.3 of
[NIST Special Publication 800-56A, revision 2] and Appendix A.3 of the
NSA's [Suite B implementer's guide to FIPS 186-3]. Note that, as explained
in the NSA guide, ECC Partial Public-Key Validation is equivalent to ECC
Full Public-Key Validation for prime-order curves like this one.

## `ECDSA_*_FIXED` Details: Fixed-length (PKCS#11-style) ECDSA Signatures

The signature is *r*||*s*, where || denotes concatenation, and where both
*r* and *s* are both big-endian-encoded values that are left-padded to the
maximum length. A P-256 signature will be 64 bytes long (two 32-byte
components) and a P-384 signature will be 96 bytes long (two 48-byte
components). This is the form of ECDSA signature used PKCS#11 and DNSSEC.

The public key is encoding in uncompressed form using the
Octet-String-to-Elliptic-Curve-Point algorithm in
[SEC 1: Elliptic Curve Cryptography, Version 2.0].

During verification, the public key is validated using the ECC Partial
Public-Key Validation Routine from Section 5.6.2.3.3 of
[NIST Special Publication 800-56A, revision 2] and Appendix A.3 of the
NSA's [Suite B implementer's guide to FIPS 186-3]. Note that, as explained
in the NSA guide, ECC Partial Public-Key Validation is equivalent to ECC
Full Public-Key Validation for prime-order curves like this one.

## `RSA_PKCS1_*` Details: RSA PKCS#1 1.5 Signatures

The signature is an RSASSA-PKCS1-v1_5 signature as described in
[RFC 3447 Section 8.2].

The public key is encoded as an ASN.1 `RSAPublicKey` as described in
[RFC 3447 Appendix-A.1.1]. The public key modulus length, rounded *up* to
the nearest (larger) multiple of 8 bits, must be in the range given in the
name of the algorithm. The public exponent must be an odd integer of 2-33
bits, inclusive.


## `RSA_PSS_*` Details: RSA PSS Signatures

The signature is an RSASSA-PSS signature as described in
[RFC 3447 Section 8.1].

The public key is encoded as an ASN.1 `RSAPublicKey` as described in
[RFC 3447 Appendix-A.1.1]. The public key modulus length, rounded *up* to
the nearest (larger) multiple of 8 bits, must be in the range given in the
name of the algorithm. The public exponent must be an odd integer of 2-33
bits, inclusive.

During verification, signatures will only be accepted if the MGF1 digest
algorithm is the same as the message digest algorithm and if the salt
length is the same length as the message digest. This matches the
requirements in TLS 1.3 and other recent specifications.

During signing, the message digest algorithm will be used as the MGF1
digest algorithm. The salt will be the same length as the message digest.
This matches the requirements in TLS 1.3 and other recent specifications.
Additionally, the entire salt is randomly generated separately for each
signature using the secure random number generator passed to `sign()`.


[SEC 1: Elliptic Curve Cryptography, Version 2.0]:
    http://www.secg.org/sec1-v2.pdf
[NIST Special Publication 800-56A, revision 2]:
    http://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-56Ar2.pdf
[Suite B implementer's guide to FIPS 186-3]:
    https://github.com/briansmith/ring/blob/main/doc/ecdsa.pdf
[RFC 3279 Section 2.2.3]:
    https://tools.ietf.org/html/rfc3279#section-2.2.3
[RFC 3447 Section 8.2]:
    https://tools.ietf.org/html/rfc3447#section-7.2
[RFC 3447 Section 8.1]:
    https://tools.ietf.org/html/rfc3447#section-8.1
[RFC 3447 Appendix-A.1.1]:
    https://tools.ietf.org/html/rfc3447#appendix-A.1.1


# Examples

## Signing and verifying with Ed25519

```
use ring::{
    rand,
    signature::{self, KeyPair},
};

# fn main() -> Result<(), ring::error::Unspecified> {
// Generate a key pair in PKCS#8 (v2) format.
let rng = rand::SystemRandom::new();
let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;

// Normally the application would store the PKCS#8 file persistently. Later
// it would read the PKCS#8 file from persistent storage to use it.

let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())?;

// Sign the message "hello, world".
const MESSAGE: &[u8] = b"hello, world";
let sig = key_pair.sign(MESSAGE);

// Normally an application would extract the bytes of the signature and
// send them in a protocol message to the peer(s). Here we just get the
// public key key directly from the key pair.
let peer_public_key_bytes = key_pair.public_key().as_ref();

// Verify the signature of the message using the public key. Normally the
// verifier of the message would parse the inputs to this code out of the
// protocol message(s) sent by the signer.
let peer_public_key =
    signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);
peer_public_key.verify(MESSAGE, sig.as_ref())?;

# Ok(())
# }
```

## Signing and verifying with RSA (PKCS#1 1.5 padding)

By default OpenSSL writes RSA public keys in SubjectPublicKeyInfo format,
not RSAPublicKey format, and Base64-encodes them (“PEM” format).

To convert the PEM SubjectPublicKeyInfo format (“BEGIN PUBLIC KEY”) to the
binary RSAPublicKey format needed by `verify()`, use:

```sh
openssl rsa -pubin \
            -in public_key.pem \
            -inform PEM \
            -RSAPublicKey_out \
            -outform DER \
            -out public_key.der
```

To extract the RSAPublicKey-formatted public key from an ASN.1 (binary)
DER-encoded RSAPrivateKey format private key file, use:

```sh
openssl rsa -in private_key.der \
            -inform DER \
            -RSAPublicKey_out \
            -outform DER \
            -out public_key.der
```

```
# #[cfg(feature = "std")]
use ring::{rand, rsa, signature};

# #[cfg(feature = "std")]
fn sign_and_verify_rsa(private_key_path: &std::path::Path,
                       public_key_path: &std::path::Path)
                       -> Result<(), MyError> {
// Create an RSA keypair from the DER-encoded bytes. This example uses
// a 2048-bit key, but larger keys are also supported.
let private_key_der = read_file(private_key_path)?;
let key_pair = rsa::KeyPair::from_der(&private_key_der)
    .map_err(|_| MyError::BadPrivateKey)?;

// Sign the message "hello, world", using PKCS#1 v1.5 padding and the
// SHA256 digest algorithm.
const MESSAGE: &'static [u8] = b"hello, world";
let rng = rand::SystemRandom::new();
let mut signature = vec![0; key_pair.public().modulus_len()];
key_pair.sign(&signature::RSA_PKCS1_SHA256, &rng, MESSAGE, &mut signature)
    .map_err(|_| MyError::OOM)?;

// Verify the signature.
let public_key =
    signature::UnparsedPublicKey::new(&signature::RSA_PKCS1_2048_8192_SHA256,
                                      read_file(public_key_path)?);
public_key.verify(MESSAGE, &signature)
    .map_err(|_| MyError::BadSignature)
}

#[derive(Debug)]
enum MyError {
#  #[cfg(feature = "std")]
   IO(std::io::Error),
   BadPrivateKey,
   OOM,
   BadSignature,
}

# #[cfg(feature = "std")]
fn read_file(path: &std::path::Path) -> Result<Vec<u8>, MyError> {
    use std::io::Read;

    let mut file = std::fs::File::open(path).map_err(|e| MyError::IO(e))?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).map_err(|e| MyError::IO(e))?;
    Ok(contents)
}
#
# #[cfg(not(feature = "std"))]
# fn sign_and_verify_rsa(_private_key_path: &std::path::Path,
#                        _public_key_path: &std::path::Path)
#                        -> Result<(), ()> {
#     Ok(())
# }
#
# fn main() {
#     let private_key_path =
#         std::path::Path::new("src/rsa/signature_rsa_example_private_key.der");
#     let public_key_path =
#         std::path::Path::new("src/rsa/signature_rsa_example_public_key.der");
#     sign_and_verify_rsa(&private_key_path, &public_key_path).unwrap()
# }
```

```rust
pub mod signature { /* ... */ }
```

### Types

#### Type Alias `RsaKeyPair`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

An RSA key pair, used for signing.

```rust
pub type RsaKeyPair = crate::rsa::KeyPair;
```

#### Struct `Signature`

A public key signature returned from a signing operation.

```rust
pub struct Signature {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Signature { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **RefUnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `UnparsedPublicKey`

An unparsed, possibly malformed, public key for signature verification.

```rust
pub struct UnparsedPublicKey<B> {
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
  pub fn new(algorithm: &''static dyn VerificationAlgorithm, bytes: B) -> Self { /* ... */ }
  ```
  Construct a new `UnparsedPublicKey`.

- ```rust
  pub fn verify(self: &Self, message: &[u8], signature: &[u8]) -> Result<(), error::Unspecified>
where
    B: AsRef<[u8]> { /* ... */ }
  ```
  Parses the public key and verifies `signature` is a valid signature of

###### Trait Implementations

- **RefUnwindSafe**
- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> Result<(), core::fmt::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UnparsedPublicKey<B> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Traits

#### Trait `KeyPair`

Key pairs for signing messages (private key and public key).

```rust
pub trait KeyPair: core::fmt::Debug + Send + Sized + Sync {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `PublicKey`: The type of the public key.

###### Required Methods

- `public_key`: The public key for the key pair.

##### Implementations

This trait is implemented for the following types:

- `Ed25519KeyPair`
- `EcdsaKeyPair`
- `KeyPair`

#### Trait `VerificationAlgorithm`

A signature verification algorithm.

```rust
pub trait VerificationAlgorithm: core::fmt::Debug + Sync + sealed::Sealed {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `verify`: Verify the signature `signature` of message `msg` with the public key

##### Implementations

This trait is implemented for the following types:

- `EdDSAParameters`
- `EcdsaVerificationAlgorithm`
- `super::RsaParameters`

### Re-exports

#### Re-export `Ed25519KeyPair`

```rust
pub use crate::ec::curve25519::ed25519::signing::Ed25519KeyPair;
```

#### Re-export `EdDSAParameters`

```rust
pub use crate::ec::curve25519::ed25519::verification::EdDSAParameters;
```

#### Re-export `ED25519`

```rust
pub use crate::ec::curve25519::ed25519::verification::ED25519;
```

#### Re-export `ED25519_PUBLIC_KEY_LEN`

```rust
pub use crate::ec::curve25519::ed25519::ED25519_PUBLIC_KEY_LEN;
```

#### Re-export `EcdsaKeyPair`

```rust
pub use crate::ec::suite_b::ecdsa::signing::EcdsaKeyPair;
```

#### Re-export `EcdsaSigningAlgorithm`

```rust
pub use crate::ec::suite_b::ecdsa::signing::EcdsaSigningAlgorithm;
```

#### Re-export `ECDSA_P256_SHA256_ASN1_SIGNING`

```rust
pub use crate::ec::suite_b::ecdsa::signing::ECDSA_P256_SHA256_ASN1_SIGNING;
```

#### Re-export `ECDSA_P256_SHA256_FIXED_SIGNING`

```rust
pub use crate::ec::suite_b::ecdsa::signing::ECDSA_P256_SHA256_FIXED_SIGNING;
```

#### Re-export `ECDSA_P384_SHA384_ASN1_SIGNING`

```rust
pub use crate::ec::suite_b::ecdsa::signing::ECDSA_P384_SHA384_ASN1_SIGNING;
```

#### Re-export `ECDSA_P384_SHA384_FIXED_SIGNING`

```rust
pub use crate::ec::suite_b::ecdsa::signing::ECDSA_P384_SHA384_FIXED_SIGNING;
```

#### Re-export `EcdsaVerificationAlgorithm`

```rust
pub use crate::ec::suite_b::ecdsa::verification::EcdsaVerificationAlgorithm;
```

#### Re-export `ECDSA_P256_SHA256_ASN1`

```rust
pub use crate::ec::suite_b::ecdsa::verification::ECDSA_P256_SHA256_ASN1;
```

#### Re-export `ECDSA_P256_SHA256_FIXED`

```rust
pub use crate::ec::suite_b::ecdsa::verification::ECDSA_P256_SHA256_FIXED;
```

#### Re-export `ECDSA_P256_SHA384_ASN1`

```rust
pub use crate::ec::suite_b::ecdsa::verification::ECDSA_P256_SHA384_ASN1;
```

#### Re-export `ECDSA_P384_SHA256_ASN1`

```rust
pub use crate::ec::suite_b::ecdsa::verification::ECDSA_P384_SHA256_ASN1;
```

#### Re-export `ECDSA_P384_SHA384_ASN1`

```rust
pub use crate::ec::suite_b::ecdsa::verification::ECDSA_P384_SHA384_ASN1;
```

#### Re-export `ECDSA_P384_SHA384_FIXED`

```rust
pub use crate::ec::suite_b::ecdsa::verification::ECDSA_P384_SHA384_FIXED;
```

#### Re-export `RsaEncoding`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::padding::RsaEncoding;
```

#### Re-export `RSA_PKCS1_SHA256`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::padding::RSA_PKCS1_SHA256;
```

#### Re-export `RSA_PKCS1_SHA384`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::padding::RSA_PKCS1_SHA384;
```

#### Re-export `RSA_PKCS1_SHA512`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::padding::RSA_PKCS1_SHA512;
```

#### Re-export `RSA_PSS_SHA256`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::padding::RSA_PSS_SHA256;
```

#### Re-export `RSA_PSS_SHA384`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::padding::RSA_PSS_SHA384;
```

#### Re-export `RSA_PSS_SHA512`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::padding::RSA_PSS_SHA512;
```

#### Re-export `RsaPublicKeyComponents`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::verification::RsaPublicKeyComponents;
```

#### Re-export `RSA_PKCS1_1024_8192_SHA1_FOR_LEGACY_USE_ONLY`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::verification::RSA_PKCS1_1024_8192_SHA1_FOR_LEGACY_USE_ONLY;
```

#### Re-export `RSA_PKCS1_1024_8192_SHA256_FOR_LEGACY_USE_ONLY`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::verification::RSA_PKCS1_1024_8192_SHA256_FOR_LEGACY_USE_ONLY;
```

#### Re-export `RSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::verification::RSA_PKCS1_1024_8192_SHA512_FOR_LEGACY_USE_ONLY;
```

#### Re-export `RSA_PKCS1_2048_8192_SHA1_FOR_LEGACY_USE_ONLY`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::verification::RSA_PKCS1_2048_8192_SHA1_FOR_LEGACY_USE_ONLY;
```

#### Re-export `RSA_PKCS1_2048_8192_SHA256`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::verification::RSA_PKCS1_2048_8192_SHA256;
```

#### Re-export `RSA_PKCS1_2048_8192_SHA384`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::verification::RSA_PKCS1_2048_8192_SHA384;
```

#### Re-export `RSA_PKCS1_2048_8192_SHA512`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::verification::RSA_PKCS1_2048_8192_SHA512;
```

#### Re-export `RSA_PKCS1_3072_8192_SHA384`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::verification::RSA_PKCS1_3072_8192_SHA384;
```

#### Re-export `RSA_PSS_2048_8192_SHA256`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::verification::RSA_PSS_2048_8192_SHA256;
```

#### Re-export `RSA_PSS_2048_8192_SHA384`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::verification::RSA_PSS_2048_8192_SHA384;
```

#### Re-export `RSA_PSS_2048_8192_SHA512`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::verification::RSA_PSS_2048_8192_SHA512;
```

#### Re-export `RsaParameters`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crate::rsa::RsaParameters;
```

## Macros

### Macro `test_file`

**Attributes:**

- `#[macro_export]`

**⚠️ Deprecated**: internal API that will be removed

References a test input file.

```rust
pub macro_rules! test_file {
    /* macro_rules! test_file {
    ($file_name:expr) => { ... };
} */
}
```

