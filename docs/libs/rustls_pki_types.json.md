# Crate Documentation

**Version:** 1.11.0

**Format Version:** 43

# Module `rustls_pki_types`

This crate provides types for representing X.509 certificates, keys and other types as
commonly used in the rustls ecosystem. It is intended to be used by crates that need to work
with such X.509 types, such as [rustls](https://crates.io/crates/rustls),
[rustls-webpki](https://crates.io/crates/rustls-webpki),
[rustls-pemfile](https://crates.io/crates/rustls-pemfile), and others.

Some of these crates used to define their own trivial wrappers around DER-encoded bytes.
However, in order to avoid inconvenient dependency edges, these were all disconnected. By
using a common low-level crate of types with long-term stable API, we hope to avoid the
downsides of unnecessary dependency edges while providing good interoperability between crates.

## DER and PEM

Many of the types defined in this crate represent DER-encoded data. DER is a binary encoding of
the ASN.1 format commonly used in web PKI specifications. It is a binary encoding, so it is
relatively compact when stored in memory. However, as a binary format, it is not very easy to
work with for humans and in contexts where binary data is inconvenient. For this reason,
many tools and protocols use a ASCII-based encoding of DER, called PEM. In addition to the
base64-encoded DER, PEM objects are delimited by header and footer lines which indicate the type
of object contained in the PEM blob.

Types here can be created from:

- DER using (for example) [`PrivatePkcs8KeyDer::from()`].
- PEM using (for example) [`pem::PemObject::from_pem_slice()`].

The [`pem::PemObject`] trait contains the full selection of ways to construct
these types from PEM encodings.  That includes ways to open and read from a file,
from a slice, or from an `std::io` stream.

There is also a lower-level API that allows a given PEM file to be fully consumed
in one pass, even if it contains different data types: see the implementation of
the [`pem::PemObject`] trait on the `(pem::SectionKind, Vec<u8>)` tuple.

## Creating new certificates and keys

This crate does not provide any functionality for creating new certificates or keys. However,
the [rcgen](https://docs.rs/rcgen) crate can be used to create new certificates and keys.

## Cloning private keys

This crate intentionally **does not** implement `Clone` on private key types in
order to minimize the exposure of private key data in memory.

If you want to extend the lifetime of a `PrivateKeyDer<'_>`, consider [`PrivateKeyDer::clone_key()`].
Alternatively  since these types are immutable, consider wrapping the `PrivateKeyDer<'_>` in a [`Rc`]
or an [`Arc`].

[`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`PrivateKeyDer::clone_key()`]: https://docs.rs/rustls-pki-types/latest/rustls_pki_types/enum.PrivateKeyDer.html#method.clone_key

## Target `wasm32-unknown-unknown` with the `web` feature

[`std::time::SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html)
is unavailable in `wasm32-unknown-unknown` targets, so calls to
[`UnixTime::now()`](https://docs.rs/rustls-pki-types/latest/rustls_pki_types/struct.UnixTime.html#method.now),
otherwise enabled by the [`std`](https://docs.rs/crate/rustls-pki-types/latest/features#std) feature,
require building instead with the [`web`](https://docs.rs/crate/rustls-pki-types/latest/features#web)
feature. It gets time by calling [`Date.now()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now)
in the browser.

## Modules

## Module `alg_id`

The PKIX [`AlgorithmIdentifier`] type, and common values.

If you need to use an [`AlgorithmIdentifier`] not defined here,
you can define it locally.

```rust
pub mod alg_id { /* ... */ }
```

### Types

#### Struct `AlgorithmIdentifier`

A DER encoding of the PKIX AlgorithmIdentifier type:

```ASN.1
AlgorithmIdentifier  ::=  SEQUENCE  {
    algorithm               OBJECT IDENTIFIER,
    parameters              ANY DEFINED BY algorithm OPTIONAL  }
                               -- contains a value of the type
                               -- registered for use with the
                               -- algorithm object identifier value
```
(from <https://www.rfc-editor.org/rfc/rfc5280#section-4.1.1.2>)

The outer sequence encoding is *not included*, so this is the DER encoding
of an OID for `algorithm` plus the `parameters` value.

For example, this is the `rsaEncryption` algorithm (but prefer to use the constant
[`RSA_ENCRYPTION`] instead):

```
let rsa_encryption = rustls_pki_types::AlgorithmIdentifier::from_slice(
    &[
        // algorithm: 1.2.840.113549.1.1.1
        0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01,
        // parameters: NULL
        0x05, 0x00
    ]
);
assert_eq!(rustls_pki_types::alg_id::RSA_ENCRYPTION, rsa_encryption);
```

Common values for this type are provided in this module.

```rust
pub struct AlgorithmIdentifier(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_slice(bytes: &''static [u8]) -> Self { /* ... */ }
  ```
  Makes a new `AlgorithmIdentifier` from a static octet slice.

###### Trait Implementations

- **Eq**
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Receiver**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Send**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AlgorithmIdentifier { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AlgorithmIdentifier) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Constants and Statics

#### Constant `ECDSA_P256`

AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp256r1`.

This is:

```text
# ecPublicKey
OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }
# secp256r1
OBJECT_IDENTIFIER { 1.2.840.10045.3.1.7 }
```

```rust
pub const ECDSA_P256: AlgorithmIdentifier = _;
```

#### Constant `ECDSA_P384`

AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp384r1`.

This is:

```text
# ecPublicKey
OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }
# secp384r1
OBJECT_IDENTIFIER { 1.3.132.0.34 }
```

```rust
pub const ECDSA_P384: AlgorithmIdentifier = _;
```

#### Constant `ECDSA_P521`

AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp521r1`.

This is:

```text
# ecPublicKey
OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }
# secp521r1
OBJECT_IDENTIFIER { 1.3.132.0.35 }
```

```rust
pub const ECDSA_P521: AlgorithmIdentifier = _;
```

#### Constant `ECDSA_SHA256`

AlgorithmIdentifier for `ecdsa-with-SHA256`.

This is:

```text
# ecdsa-with-SHA256
OBJECT_IDENTIFIER { 1.2.840.10045.4.3.2 }
```

```rust
pub const ECDSA_SHA256: AlgorithmIdentifier = _;
```

#### Constant `ECDSA_SHA384`

AlgorithmIdentifier for `ecdsa-with-SHA384`.

This is:

```text
# ecdsa-with-SHA384
OBJECT_IDENTIFIER { 1.2.840.10045.4.3.3 }
```

```rust
pub const ECDSA_SHA384: AlgorithmIdentifier = _;
```

#### Constant `ECDSA_SHA512`

AlgorithmIdentifier for `ecdsa-with-SHA512`.

This is:

```text
# ecdsa-with-SHA512
OBJECT_IDENTIFIER { 1.2.840.10045.4.3.4 }
```

```rust
pub const ECDSA_SHA512: AlgorithmIdentifier = _;
```

#### Constant `RSA_ENCRYPTION`

AlgorithmIdentifier for `rsaEncryption`.

This is:

```text
# rsaEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.1 }
NULL {}
```

```rust
pub const RSA_ENCRYPTION: AlgorithmIdentifier = _;
```

#### Constant `RSA_PKCS1_SHA256`

AlgorithmIdentifier for `sha256WithRSAEncryption`.

This is:

```text
# sha256WithRSAEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.11 }
NULL {}
```

```rust
pub const RSA_PKCS1_SHA256: AlgorithmIdentifier = _;
```

#### Constant `RSA_PKCS1_SHA384`

AlgorithmIdentifier for `sha384WithRSAEncryption`.

This is:

```text
# sha384WithRSAEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.12 }
NULL {}
```

```rust
pub const RSA_PKCS1_SHA384: AlgorithmIdentifier = _;
```

#### Constant `RSA_PKCS1_SHA512`

AlgorithmIdentifier for `sha512WithRSAEncryption`.

This is:

```text
# sha512WithRSAEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.13 }
NULL {}
```

```rust
pub const RSA_PKCS1_SHA512: AlgorithmIdentifier = _;
```

#### Constant `RSA_PSS_SHA256`

AlgorithmIdentifier for `rsassaPss` with:

- hashAlgorithm: sha256
- maskGenAlgorithm: mgf1 with sha256
- saltLength: 32

This is:

```text
# rsassa-pss
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.10 }
SEQUENCE {
  # hashAlgorithm:
  [0] {
    SEQUENCE {
      # sha256
      OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.1 }
      NULL {}
    }
  }
  # maskGenAlgorithm:
  [1] {
    SEQUENCE {
      # mgf1
      OBJECT_IDENTIFIER { 1.2.840.113549.1.1.8 }
      SEQUENCE {
        # sha256
        OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.1 }
        NULL {}
      }
    }
  }
  # saltLength:
  [2] {
    INTEGER { 32 }
  }
}
```

See <https://datatracker.ietf.org/doc/html/rfc4055#section-3.1> for
the meaning of the context-specific tags.

```rust
pub const RSA_PSS_SHA256: AlgorithmIdentifier = _;
```

#### Constant `RSA_PSS_SHA384`

AlgorithmIdentifier for `rsassaPss` with:

- hashAlgorithm: sha384
- maskGenAlgorithm: mgf1 with sha384
- saltLength: 48

This is:

```text
# rsassa-pss
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.10 }
SEQUENCE {
  # hashAlgorithm:
  [0] {
    SEQUENCE {
      # sha384
      OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.2 }
      NULL {}
    }
  }
  # maskGenAlgorithm:
  [1] {
    SEQUENCE {
      # mgf1
      OBJECT_IDENTIFIER { 1.2.840.113549.1.1.8 }
      SEQUENCE {
        # sha384
        OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.2 }
        NULL {}
      }
    }
  }
  # saltLength:
  [2] {
    INTEGER { 48 }
  }
}
```

See <https://datatracker.ietf.org/doc/html/rfc4055#section-3.1> for
the meaning of the context-specific tags.

```rust
pub const RSA_PSS_SHA384: AlgorithmIdentifier = _;
```

#### Constant `RSA_PSS_SHA512`

AlgorithmIdentifier for `rsassaPss` with:

- hashAlgorithm: sha512
- maskGenAlgorithm: mgf1 with sha512
- saltLength: 64

This is:

```text
# rsassa-pss
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.10 }
SEQUENCE {
  # hashAlgorithm:
  [0] {
    SEQUENCE {
      # sha512
      OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.3 }
      NULL {}
    }
  }
  # maskGenAlgorithm:
  [1] {
    SEQUENCE {
      # mgf1
      OBJECT_IDENTIFIER { 1.2.840.113549.1.1.8 }
      SEQUENCE {
        # sha512
        OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.3 }
        NULL {}
      }
    }
  }
  # saltLength:
  [2] {
    INTEGER { 64 }
  }
}
```

See <https://datatracker.ietf.org/doc/html/rfc4055#section-3.1> for
the meaning of the context-specific tags.

```rust
pub const RSA_PSS_SHA512: AlgorithmIdentifier = _;
```

#### Constant `ED25519`

AlgorithmIdentifier for `ED25519`.

This is:

```text
# ed25519
OBJECT_IDENTIFIER { 1.3.101.112 }
```

```rust
pub const ED25519: AlgorithmIdentifier = _;
```

## Module `pem`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

Low-level PEM decoding APIs.

These APIs allow decoding PEM format in an iterator, which means you
can load multiple different types of PEM section from a file in a single
pass.

```rust
pub mod pem { /* ... */ }
```

### Types

#### Struct `ReadIter`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Extract and return all PEM sections by reading `rd`.

```rust
pub struct ReadIter<R, T> {
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
  pub fn new(rd: R) -> Self { /* ... */ }
  ```
  Create a new iterator.

###### Trait Implementations

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `SliceIter`

Iterator over all PEM sections in a `&[u8]` slice.

```rust
pub struct SliceIter<''a, T> {
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
  pub fn new(current: &''a [u8]) -> Self { /* ... */ }
  ```
  Create a new iterator.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Send**
- **Freeze**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Enum `SectionKind`

**Attributes:**

- `#[non_exhaustive]`

A single recognised section in a PEM file.

```rust
pub enum SectionKind {
    Certificate,
    PublicKey,
    RsaPrivateKey,
    PrivateKey,
    EcPrivateKey,
    Crl,
    Csr,
    EchConfigList,
}
```

##### Variants

###### `Certificate`

A DER-encoded x509 certificate.

Appears as "CERTIFICATE" in PEM files.

###### `PublicKey`

A DER-encoded Subject Public Key Info; as specified in RFC 7468.

Appears as "PUBLIC KEY" in PEM files.

###### `RsaPrivateKey`

A DER-encoded plaintext RSA private key; as specified in PKCS #1/RFC 3447

Appears as "RSA PRIVATE KEY" in PEM files.

###### `PrivateKey`

A DER-encoded plaintext private key; as specified in PKCS #8/RFC 5958

Appears as "PRIVATE KEY" in PEM files.

###### `EcPrivateKey`

A Sec1-encoded plaintext private key; as specified in RFC 5915

Appears as "EC PRIVATE KEY" in PEM files.

###### `Crl`

A Certificate Revocation List; as specified in RFC 5280

Appears as "X509 CRL" in PEM files.

###### `Csr`

A Certificate Signing Request; as specified in RFC 2986

Appears as "CERTIFICATE REQUEST" in PEM files.

###### `EchConfigList`

An EchConfigList structure, as specified in
<https://www.ietf.org/archive/id/draft-farrell-tls-pemesni-05.html>.

Appears as "ECHCONFIG" in PEM files.

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SectionKind) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **StructuralPartialEq**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

  - ```rust
    fn try_from(value: &[u8]) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
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

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SectionKind { /* ... */ }
    ```

- **Send**
#### Enum `Error`

**Attributes:**

- `#[non_exhaustive]`

Errors that may arise when parsing the contents of a PEM file

```rust
pub enum Error {
    MissingSectionEnd {
        end_marker: alloc::vec::Vec<u8>,
    },
    IllegalSectionStart {
        line: alloc::vec::Vec<u8>,
    },
    Base64Decode(alloc::string::String),
    Io(io::Error),
    NoItemsFound,
}
```

##### Variants

###### `MissingSectionEnd`

a section is missing its "END marker" line

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `end_marker` | `alloc::vec::Vec<u8>` | the expected "END marker" line that was not found |

###### `IllegalSectionStart`

syntax error found in the line that starts a new section

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `line` | `alloc::vec::Vec<u8>` | line that contains the syntax error |

###### `Base64Decode`

base64 decode error

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `Io`

I/O errors, from APIs that accept `std::io` types.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `io::Error` |  |

###### `NoItemsFound`

No items found of desired type

##### Implementations

###### Trait Implementations

- **Send**
- **Error**
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

- **Freeze**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Traits

#### Trait `PemObject`

Items that can be decoded from PEM data.

```rust
pub trait PemObject: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `from_pem`: Conversion from a PEM [`SectionKind`] and body data.

##### Provided Methods

- ```rust
  fn from_pem_slice(pem: &[u8]) -> Result<Self, Error> { /* ... */ }
  ```
  Decode the first section of this type from PEM contained in

- ```rust
  fn pem_slice_iter(pem: &[u8]) -> SliceIter<''_, Self> { /* ... */ }
  ```
  Iterate over all sections of this type from PEM contained in

- ```rust
  fn from_pem_file</* synthetic */ impl AsRef<std::path::Path>: AsRef<std::path::Path>>(file_name: impl AsRef<std::path::Path>) -> Result<Self, Error> { /* ... */ }
  ```
  Decode the first section of this type from the PEM contents of the named file.

- ```rust
  fn pem_file_iter</* synthetic */ impl AsRef<std::path::Path>: AsRef<std::path::Path>>(file_name: impl AsRef<std::path::Path>) -> Result<ReadIter<io::BufReader<File>, Self>, Error> { /* ... */ }
  ```
  Iterate over all sections of this type from the PEM contents of the named file.

- ```rust
  fn from_pem_reader</* synthetic */ impl std::io::Read: std::io::Read>(rd: impl std::io::Read) -> Result<Self, Error> { /* ... */ }
  ```
  Decode the first section of this type from PEM read from an [`io::Read`].

- ```rust
  fn pem_reader_iter<R: std::io::Read>(rd: R) -> ReadIter<io::BufReader<R>, Self> { /* ... */ }
  ```
  Iterate over all sections of this type from PEM present in an [`io::Read`].

##### Implementations

This trait is implemented for the following types:

- `T` with <T: PemObjectFilter + From<alloc::vec::Vec<u8>>>
- `(SectionKind, alloc::vec::Vec<u8>)`
- `PrivateKeyDer<''static>`

### Functions

#### Function `from_buf`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Extract and decode the next supported PEM section from `rd`.

- Ok(None) is returned if there is no PEM section read from `rd`.
- Underlying IO errors produce a `Err(...)`
- Otherwise each decoded section is returned with a `Ok(Some(...))`

```rust
pub fn from_buf(rd: &mut dyn io::BufRead) -> Result<Option<(SectionKind, alloc::vec::Vec<u8>)>, Error> { /* ... */ }
```

## Types

### Enum `PrivateKeyDer`

**Attributes:**

- `#[non_exhaustive]`

A DER-encoded X.509 private key, in one of several formats

See variant inner types for more detailed information.

This can load several types of PEM-encoded private key, and then reveal
which types were found:

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivateKeyDer, pem::PemObject};

// load from a PEM file
let pkcs8 = PrivateKeyDer::from_pem_file("tests/data/nistp256key.pkcs8.pem").unwrap();
let pkcs1 = PrivateKeyDer::from_pem_file("tests/data/rsa1024.pkcs1.pem").unwrap();
let sec1 = PrivateKeyDer::from_pem_file("tests/data/nistp256key.pem").unwrap();
assert!(matches!(pkcs8, PrivateKeyDer::Pkcs8(_)));
assert!(matches!(pkcs1, PrivateKeyDer::Pkcs1(_)));
assert!(matches!(sec1, PrivateKeyDer::Sec1(_)));
# }
```

```rust
pub enum PrivateKeyDer<''a> {
    Pkcs1(PrivatePkcs1KeyDer<''a>),
    Sec1(PrivateSec1KeyDer<''a>),
    Pkcs8(PrivatePkcs8KeyDer<''a>),
}
```

#### Variants

##### `Pkcs1`

An RSA private key

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `PrivatePkcs1KeyDer<''a>` |  |

##### `Sec1`

A Sec1 private key

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `PrivateSec1KeyDer<''a>` |  |

##### `Pkcs8`

A PKCS#8 private key

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `PrivatePkcs8KeyDer<''a>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn clone_key(self: &Self) -> PrivateKeyDer<''static> { /* ... */ }
  ```
  Clone the private key to a `'static` value

- ```rust
  pub fn secret_der(self: &Self) -> &[u8] { /* ... */ }
  ```
  Yield the DER-encoded bytes of the private key

##### Trait Implementations

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PrivateKeyDer<''a>) -> bool { /* ... */ }
    ```

- **Eq**
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

  - ```rust
    fn from(key: PrivatePkcs1KeyDer<''a>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(key: PrivateSec1KeyDer<''a>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(key: PrivatePkcs8KeyDer<''a>) -> Self { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **PemObject**
  - ```rust
    fn from_pem(kind: SectionKind, value: Vec<u8>) -> Option<Self> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(key: &''a [u8]) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(key: Vec<u8>) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

### Struct `PrivatePkcs1KeyDer`

A DER-encoded plaintext RSA private key; as specified in PKCS#1/RFC 3447

RSA private keys are identified in PEM context as `RSA PRIVATE KEY` and when stored in a
file usually use a `.pem` or `.key` extension.

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivatePkcs1KeyDer, pem::PemObject};

// load from a PEM file
PrivatePkcs1KeyDer::from_pem_file("tests/data/rsa1024.pkcs1.pem").unwrap();

// or from a PEM byte slice...
# let byte_slice = include_bytes!("../tests/data/rsa1024.pkcs1.pem");
PrivatePkcs1KeyDer::from_pem_slice(byte_slice).unwrap();
# }
```

```rust
pub struct PrivatePkcs1KeyDer<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn clone_key(self: &Self) -> PrivatePkcs1KeyDer<''static> { /* ... */ }
  ```
  Clone the private key to a `'static` value

- ```rust
  pub fn secret_pkcs1_der(self: &Self) -> &[u8] { /* ... */ }
  ```
  Yield the DER-encoded bytes of the private key

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

  - ```rust
    fn from(key: PrivatePkcs1KeyDer<''a>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(slice: &''a [u8]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(vec: Vec<u8>) -> Self { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PrivatePkcs1KeyDer<''a>) -> bool { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **PemObject**
  - ```rust
    fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T> { /* ... */ }
    ```

### Struct `PrivateSec1KeyDer`

A Sec1-encoded plaintext private key; as specified in RFC 5915

Sec1 private keys are identified in PEM context as `EC PRIVATE KEY` and when stored in a
file usually use a `.pem` or `.key` extension. For more on PEM files, refer to the crate
documentation.

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivateSec1KeyDer, pem::PemObject};

// load from a PEM file
PrivateSec1KeyDer::from_pem_file("tests/data/nistp256key.pem").unwrap();

// or from a PEM byte slice...
# let byte_slice = include_bytes!("../tests/data/nistp256key.pem");
PrivateSec1KeyDer::from_pem_slice(byte_slice).unwrap();
# }
```

```rust
pub struct PrivateSec1KeyDer<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn clone_key(self: &Self) -> PrivateSec1KeyDer<''static> { /* ... */ }
  ```
  Clone the private key to a `'static` value

- ```rust
  pub fn secret_sec1_der(self: &Self) -> &[u8] { /* ... */ }
  ```
  Yield the DER-encoded bytes of the private key

##### Trait Implementations

- **StructuralPartialEq**
- **Eq**
- **Freeze**
- **Sync**
- **PemObject**
  - ```rust
    fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(key: PrivateSec1KeyDer<''a>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(slice: &''a [u8]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(vec: Vec<u8>) -> Self { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PrivateSec1KeyDer<''a>) -> bool { /* ... */ }
    ```

### Struct `PrivatePkcs8KeyDer`

A DER-encoded plaintext private key; as specified in PKCS#8/RFC 5958

PKCS#8 private keys are identified in PEM context as `PRIVATE KEY` and when stored in a
file usually use a `.pem` or `.key` extension. For more on PEM files, refer to the crate
documentation.

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{PrivatePkcs8KeyDer, pem::PemObject};

// load from a PEM file
PrivatePkcs8KeyDer::from_pem_file("tests/data/nistp256key.pkcs8.pem").unwrap();
PrivatePkcs8KeyDer::from_pem_file("tests/data/rsa1024.pkcs8.pem").unwrap();

// or from a PEM byte slice...
# let byte_slice = include_bytes!("../tests/data/nistp256key.pkcs8.pem");
PrivatePkcs8KeyDer::from_pem_slice(byte_slice).unwrap();
# }
```

```rust
pub struct PrivatePkcs8KeyDer<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn clone_key(self: &Self) -> PrivatePkcs8KeyDer<''static> { /* ... */ }
  ```
  Clone the private key to a `'static` value

- ```rust
  pub fn secret_pkcs8_der(self: &Self) -> &[u8] { /* ... */ }
  ```
  Yield the DER-encoded bytes of the private key

##### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PrivatePkcs8KeyDer<''a>) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PemObject**
  - ```rust
    fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(key: PrivatePkcs8KeyDer<''a>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(slice: &''a [u8]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(vec: Vec<u8>) -> Self { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **Send**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

### Struct `TrustAnchor`

A trust anchor (a.k.a. root CA)

Traditionally, certificate verification libraries have represented trust anchors as full X.509
root certificates. However, those certificates contain a lot more data than is needed for
verifying certificates. The [`TrustAnchor`] representation allows an application to store
just the essential elements of trust anchors.

The most common way to get one of these is to call [`rustls_webpki::anchor_from_trusted_cert()`].

[`rustls_webpki::anchor_from_trusted_cert()`]: https://docs.rs/rustls-webpki/latest/webpki/fn.anchor_from_trusted_cert.html

```rust
pub struct TrustAnchor<''a> {
    pub subject: Der<''a>,
    pub subject_public_key_info: Der<''a>,
    pub name_constraints: Option<Der<''a>>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `subject` | `Der<''a>` | Value of the `subject` field of the trust anchor |
| `subject_public_key_info` | `Der<''a>` | Value of the `subjectPublicKeyInfo` field of the trust anchor |
| `name_constraints` | `Option<Der<''a>>` | Value of DER-encoded `NameConstraints`, containing name constraints to the trust anchor, if any |

#### Implementations

##### Methods

- ```rust
  pub fn to_owned(self: &Self) -> TrustAnchor<''static> { /* ... */ }
  ```
  Yield a `'static` lifetime of the `TrustAnchor` by allocating owned `Der` variants

##### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TrustAnchor<''a> { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TrustAnchor<''a>) -> bool { /* ... */ }
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

### Struct `CertificateRevocationListDer`

A Certificate Revocation List; as specified in RFC 5280

Certificate revocation lists are identified in PEM context as `X509 CRL` and when stored in a
file usually use a `.crl` extension. For more on PEM files, refer to the crate documentation.

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{CertificateRevocationListDer, pem::PemObject};

// load several from a PEM file
let crls: Vec<_> = CertificateRevocationListDer::pem_file_iter("tests/data/crl.pem")
    .unwrap()
    .collect();
assert!(crls.len() >= 1);

// or one from a PEM byte slice...
# let byte_slice = include_bytes!("../tests/data/crl.pem");
CertificateRevocationListDer::from_pem_slice(byte_slice).unwrap();

// or several from a PEM byte slice
let crls: Vec<_> = CertificateRevocationListDer::pem_slice_iter(byte_slice)
    .collect();
assert!(crls.len() >= 1);
# }
```

```rust
pub struct CertificateRevocationListDer<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CertificateRevocationListDer<''a> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Receiver**
- **StructuralPartialEq**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(slice: &''a [u8]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(vec: Vec<u8>) -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PemObject**
  - ```rust
    fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CertificateRevocationListDer<''a>) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Struct `CertificateSigningRequestDer`

A Certificate Signing Request; as specified in RFC 2986

Certificate signing requests are identified in PEM context as `CERTIFICATE REQUEST` and when stored in a
file usually use a `.csr` extension. For more on PEM files, refer to the crate documentation.

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{CertificateSigningRequestDer, pem::PemObject};

// load from a PEM file
CertificateSigningRequestDer::from_pem_file("tests/data/csr.pem").unwrap();

// or from a PEM byte slice...
# let byte_slice = include_bytes!("../tests/data/csr.pem");
CertificateSigningRequestDer::from_pem_slice(byte_slice).unwrap();
# }
```

```rust
pub struct CertificateSigningRequestDer<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(slice: &''a [u8]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(vec: Vec<u8>) -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Receiver**
- **Sync**
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

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PemObject**
  - ```rust
    fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CertificateSigningRequestDer<''a>) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CertificateSigningRequestDer<''a> { /* ... */ }
    ```

### Struct `CertificateDer`

A DER-encoded X.509 certificate; as specified in RFC 5280

Certificates are identified in PEM context as `CERTIFICATE` and when stored in a
file usually use a `.pem`, `.cer` or `.crt` extension. For more on PEM files, refer to the
crate documentation.

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{CertificateDer, pem::PemObject};

// load several from a PEM file
let certs: Vec<_> = CertificateDer::pem_file_iter("tests/data/certificate.chain.pem")
    .unwrap()
    .collect();
assert_eq!(certs.len(), 3);

// or one from a PEM byte slice...
# let byte_slice = include_bytes!("../tests/data/certificate.chain.pem");
CertificateDer::from_pem_slice(byte_slice).unwrap();

// or several from a PEM byte slice
let certs: Vec<_> = CertificateDer::pem_slice_iter(byte_slice)
    .collect();
assert_eq!(certs.len(), 3);
# }
```

```rust
pub struct CertificateDer<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_slice(bytes: &''a [u8]) -> Self { /* ... */ }
  ```
  A const constructor to create a `CertificateDer` from a slice of DER.

- ```rust
  pub fn into_owned(self: Self) -> CertificateDer<''static> { /* ... */ }
  ```
  Converts this certificate into its owned variant, unfreezing borrowed content (if any)

##### Trait Implementations

- **Send**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CertificateDer<''a>) -> bool { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **PemObject**
  - ```rust
    fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T> { /* ... */ }
    ```

- **Freeze**
- **Eq**
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
    fn from(slice: &''a [u8]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(vec: Vec<u8>) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CertificateDer<''a> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **UnwindSafe**
- **Receiver**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Type Alias `SubjectPublicKeyInfo`

**⚠️ Deprecated since 1.7.0**: Prefer `SubjectPublicKeyInfoDer` instead

A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280.

```rust
pub type SubjectPublicKeyInfo<''a> = SubjectPublicKeyInfoDer<''a>;
```

### Struct `SubjectPublicKeyInfoDer`

A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280.

Public keys are identified in PEM context as a `PUBLIC KEY`.

```rust
# #[cfg(all(feature = "alloc", feature = "std"))] {
use rustls_pki_types::{SubjectPublicKeyInfoDer, pem::PemObject};

// load from a PEM file
SubjectPublicKeyInfoDer::from_pem_file("tests/data/spki.pem").unwrap();

// or from a PEM byte slice...
# let byte_slice = include_bytes!("../tests/data/spki.pem");
SubjectPublicKeyInfoDer::from_pem_slice(byte_slice).unwrap();
# }
```

```rust
pub struct SubjectPublicKeyInfoDer<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn into_owned(self: Self) -> SubjectPublicKeyInfoDer<''static> { /* ... */ }
  ```
  Converts this SubjectPublicKeyInfo into its owned variant, unfreezing borrowed content (if any)

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SubjectPublicKeyInfoDer<''a> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **PemObject**
  - ```rust
    fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SubjectPublicKeyInfoDer<''a>) -> bool { /* ... */ }
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
- **RefUnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(slice: &''a [u8]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(vec: Vec<u8>) -> Self { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Receiver**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `EchConfigListBytes`

A TLS-encoded Encrypted Client Hello (ECH) configuration list (`ECHConfigList`); as specified in
[draft-ietf-tls-esni-18 §4](https://datatracker.ietf.org/doc/html/draft-ietf-tls-esni-18#section-4)

```rust
pub struct EchConfigListBytes<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn into_owned(self: Self) -> EchConfigListBytes<''static> { /* ... */ }
  ```
  Converts this config into its owned variant, unfreezing borrowed content (if any)

- ```rust
  pub fn config_and_key_from_iter</* synthetic */ impl Iterator<Item = Result<(SectionKind, Vec<u8>), pem::Error>>: Iterator<Item = Result<(SectionKind, Vec<u8>), pem::Error>>>(iter: impl Iterator<Item = Result<(SectionKind, Vec<u8>), pem::Error>>) -> Result<(Self, PrivatePkcs8KeyDer<''static>), pem::Error> { /* ... */ }
  ```
  Convert an iterator over PEM items into an `EchConfigListBytes` and private key.

##### Trait Implementations

- **StructuralPartialEq**
- **UnwindSafe**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> EchConfigListBytes<''a> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Freeze**
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

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PemObject**
  - ```rust
    fn from_pem(kind: SectionKind, der: Vec<u8>) -> Option<T> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(slice: &''a [u8]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(vec: Vec<u8>) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &EchConfigListBytes<''a>) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Receiver**
### Struct `InvalidSignature`

A detail-less error when a signature is not valid.

```rust
pub struct InvalidSignature;
```

#### Implementations

##### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> InvalidSignature { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Send**
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

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Struct `UnixTime`

A timestamp, tracking the number of non-leap seconds since the Unix epoch.

The Unix epoch is defined January 1, 1970 00:00:00 UTC.

```rust
pub struct UnixTime(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn now() -> Self { /* ... */ }
  ```
  The current time, as a `UnixTime`

- ```rust
  pub fn since_unix_epoch(duration: Duration) -> Self { /* ... */ }
  ```
  Convert a `Duration` since the start of 1970 to a `UnixTime`

- ```rust
  pub fn as_secs(self: &Self) -> u64 { /* ... */ }
  ```
  Number of seconds since the Unix epoch

##### Trait Implementations

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

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &UnixTime) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Freeze**
- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UnixTime { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **UnwindSafe**
- **Unpin**
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

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &UnixTime) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &UnixTime) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

### Struct `Der`

DER-encoded data, either owned or borrowed

This wrapper type is used to represent DER-encoded data in a way that is agnostic to whether
the data is owned (by a `Vec<u8>`) or borrowed (by a `&[u8]`). Support for the owned
variant is only available when the `alloc` feature is enabled.

```rust
pub struct Der<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub const fn from_slice(der: &''a [u8]) -> Self { /* ... */ }
  ```
  A const constructor to create a `Der` from a borrowed slice

##### Trait Implementations

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Receiver**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Der<''a> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Der<''a>) -> bool { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(slice: &''a [u8]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(vec: Vec<u8>) -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

## Traits

### Trait `SignatureVerificationAlgorithm`

An abstract signature verification algorithm.

One of these is needed per supported pair of public key type (identified
with `public_key_alg_id()`) and `signatureAlgorithm` (identified with
`signature_alg_id()`).  Note that both of these `AlgorithmIdentifier`s include
the parameters encoding, so separate `SignatureVerificationAlgorithm`s are needed
for each possible public key or signature parameters.

Debug implementations should list the public key algorithm identifier and
signature algorithm identifier in human friendly form (i.e. not encoded bytes),
along with the name of the implementing library (to distinguish different
implementations of the same algorithms).

```rust
pub trait SignatureVerificationAlgorithm: Send + Sync + fmt::Debug {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `verify_signature`: Verify a signature.
- `public_key_alg_id`: Return the `AlgorithmIdentifier` that must equal a public key's
- `signature_alg_id`: Return the `AlgorithmIdentifier` that must equal the `signatureAlgorithm` value

#### Provided Methods

- ```rust
  fn fips(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if this is backed by a FIPS-approved implementation.

## Re-exports

### Re-export `AlgorithmIdentifier`

```rust
pub use alg_id::AlgorithmIdentifier;
```

### Re-export `AddrParseError`

```rust
pub use server_name::AddrParseError;
```

### Re-export `DnsName`

```rust
pub use server_name::DnsName;
```

### Re-export `InvalidDnsNameError`

```rust
pub use server_name::InvalidDnsNameError;
```

### Re-export `IpAddr`

```rust
pub use server_name::IpAddr;
```

### Re-export `Ipv4Addr`

```rust
pub use server_name::Ipv4Addr;
```

### Re-export `Ipv6Addr`

```rust
pub use server_name::Ipv6Addr;
```

### Re-export `ServerName`

```rust
pub use server_name::ServerName;
```

