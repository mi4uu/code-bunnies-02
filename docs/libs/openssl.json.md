# Crate Documentation

**Version:** 0.10.72

**Format Version:** 43

# Module `openssl`

Bindings to OpenSSL

This crate provides a safe interface to the popular OpenSSL cryptography library. OpenSSL versions 1.0.1 through
3.x.x and LibreSSL versions 2.5 through 3.7.x are supported.

# Building

Both OpenSSL libraries and headers are required to build this crate. There are multiple options available to locate
OpenSSL.

## Vendored

If the `vendored` Cargo feature is enabled, the `openssl-src` crate will be used to compile and statically link to
a copy of OpenSSL. The build process requires a C compiler, perl (and perl-core), and make. The OpenSSL version will generally track
the newest OpenSSL release, and changes to the version are *not* considered breaking changes.

```toml
[dependencies]
openssl = { version = "0.10", features = ["vendored"] }
```

The vendored copy will be configured to automatically find a configuration and root certificates at `/usr/local/ssl`.
This path can be overridden with an environment variable (see the manual section below).
Alternatively, the `openssl-probe` crate can be used to find root certificates at runtime.

## Automatic

The `openssl-sys` crate will automatically detect OpenSSL installations via Homebrew on macOS and vcpkg on Windows.
Additionally, it will use `pkg-config` on Unix-like systems to find the system installation.

```not_rust
# macOS (Homebrew)
$ brew install openssl@3

# macOS (MacPorts)
$ sudo port install openssl

# macOS (pkgsrc)
$ sudo pkgin install openssl

# Arch Linux
$ sudo pacman -S pkgconf openssl

# Debian and Ubuntu
$ sudo apt-get install pkg-config libssl-dev

# Fedora
$ sudo dnf install pkgconf perl-FindBin perl-IPC-Cmd openssl-devel

# Alpine Linux
$ apk add pkgconf openssl-dev

# openSUSE
$ sudo zypper in libopenssl-devel
```

## Manual

A set of environment variables can be used to point `openssl-sys` towards an OpenSSL installation. They will
override the automatic detection logic.

* `OPENSSL_DIR` - If specified, the directory of an OpenSSL installation. The directory should contain `lib` and
  `include` subdirectories containing the libraries and headers respectively.
* `OPENSSL_LIB_DIR` and `OPENSSL_INCLUDE_DIR` - If specified, the directories containing the OpenSSL libraries and
  headers respectively. This can be used if the OpenSSL installation is split in a nonstandard directory layout.
* `OPENSSL_STATIC` - If set, the crate will statically link to OpenSSL rather than dynamically link.
* `OPENSSL_LIBS` - If set, a `:`-separated list of library names to link to (e.g. `ssl:crypto`). This can be used
  if nonstandard library names were used for whatever reason.
* `OPENSSL_NO_VENDOR` - If set, always find OpenSSL in the system, even if the `vendored` feature is enabled.

If the `vendored` Cargo feature is enabled, the following environment variable can also be used to further configure
the OpenSSL build.

* `OPENSSL_CONFIG_DIR` - If set, the copy of OpenSSL built by the `openssl-src` crate will be configured to look for
  configuration files and root certificates in this directory.

Additionally, these variables can be prefixed with the upper-cased target architecture (e.g.
    `X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR`), which can be useful when cross compiling.

# Feature Detection

APIs have been added to and removed from the various supported OpenSSL versions, and this library exposes the
functionality available in the version being linked against. This means that methods, constants, and even modules
will be present when building against one version of OpenSSL but not when building against another! APIs will
document any version-specific availability restrictions.

A build script can be used to detect the OpenSSL or LibreSSL version at compile time if needed. The `openssl-sys`
crate propagates the version via the `DEP_OPENSSL_VERSION_NUMBER` and `DEP_OPENSSL_LIBRESSL_VERSION_NUMBER`
environment variables to build scripts. The version format is a hex-encoding of the OpenSSL release version:
`0xMNNFFPPS`. For example, version 1.0.2g's encoding is `0x1_00_02_07_0`.

For example, let's say we want to adjust the TLSv1.3 cipher suites used by a client, but also want to compile
against OpenSSL versions that don't support TLSv1.3:

Cargo.toml:

```toml
[dependencies]
openssl-sys = "0.9"
openssl = "0.10"
```

build.rs:

```
use std::env;

fn main() {
    if let Ok(v) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&v, 16).unwrap();

        if version >= 0x1_01_01_00_0 {
            println!("cargo:rustc-cfg=openssl111");
        }
    }
}
```

lib.rs:

```
use openssl::ssl::{SslConnector, SslMethod};

let mut ctx = SslConnector::builder(SslMethod::tls()).unwrap();

// set_ciphersuites was added in OpenSSL 1.1.1, so we can only call it when linking against that version
#[cfg(openssl111)]
ctx.set_ciphersuites("TLS_AES_256_GCM_SHA384:TLS_AES_128_GCM_SHA256").unwrap();
```

## Modules

## Module `aes`

**Attributes:**

- `#[<cfg_attr>(all(not(boringssl), not(awslc),
not(osslconf = "OPENSSL_NO_DEPRECATED_3_0")), doc =
r#"\
## AES IGE
```rust
use openssl::aes::{AesKey, aes_ige};
use openssl::symm::Mode;

let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
let plaintext = b"\x12\x34\x56\x78\x90\x12\x34\x56\x12\x34\x56\x78\x90\x12\x34\x56";
let mut iv = *b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\
                \x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F";

 let key = AesKey::new_encrypt(key).unwrap();
 let mut output = [0u8; 16];
 aes_ige(plaintext, &mut output, &key, &mut iv, Mode::Encrypt);
 assert_eq!(output, *b"\xa6\xad\x97\x4d\x5c\xea\x1d\x36\xd2\xf3\x67\x98\x09\x07\xed\x32");
```"#)]`

Low level AES IGE and key wrapping functionality

AES ECB, CBC, XTS, CTR, CFB, GCM and other conventional symmetric encryption
modes are found in [`symm`].  This is the implementation of AES IGE and key wrapping

Advanced Encryption Standard (AES) provides symmetric key cipher that
the same key is used to encrypt and decrypt data.  This implementation
uses 128, 192, or 256 bit keys.  This module provides functions to
create a new key with [`new_encrypt`] and perform an encryption/decryption
using that key with [`aes_ige`].

[`new_encrypt`]: struct.AesKey.html#method.new_encrypt
[`aes_ige`]: fn.aes_ige.html

The [`symm`] module should be used in preference to this module in most cases.
The IGE block cipher is a non-traditional cipher mode.  More traditional AES
encryption methods are found in the [`Crypter`] and [`Cipher`] structs.

[`symm`]: ../symm/index.html
[`Crypter`]: ../symm/struct.Crypter.html
[`Cipher`]: ../symm/struct.Cipher.html

# Examples
\
## AES IGE
```rust
use openssl::aes::{AesKey, aes_ige};
use openssl::symm::Mode;

let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
let plaintext = b"\x12\x34\x56\x78\x90\x12\x34\x56\x12\x34\x56\x78\x90\x12\x34\x56";
let mut iv = *b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\
                \x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F";

 let key = AesKey::new_encrypt(key).unwrap();
 let mut output = [0u8; 16];
 aes_ige(plaintext, &mut output, &key, &mut iv, Mode::Encrypt);
 assert_eq!(output, *b"\xa6\xad\x97\x4d\x5c\xea\x1d\x36\xd2\xf3\x67\x98\x09\x07\xed\x32");
```

## Key wrapping
```rust
use openssl::aes::{AesKey, unwrap_key, wrap_key};

let kek = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
let key_to_wrap = b"\x00\x11\x22\x33\x44\x55\x66\x77\x88\x99\xAA\xBB\xCC\xDD\xEE\xFF";

let enc_key = AesKey::new_encrypt(kek).unwrap();
let mut ciphertext = [0u8; 24];
wrap_key(&enc_key, None, &mut ciphertext, &key_to_wrap[..]).unwrap();
let dec_key = AesKey::new_decrypt(kek).unwrap();
let mut orig_key = [0u8; 16];
unwrap_key(&dec_key, None, &mut orig_key, &ciphertext[..]).unwrap();

assert_eq!(&orig_key[..], &key_to_wrap[..]);
```


```rust
pub mod aes { /* ... */ }
```

### Types

#### Struct `KeyError`

Provides Error handling for parsing keys.

```rust
pub struct KeyError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
#### Struct `AesKey`

The key used to encrypt or decrypt cipher blocks.

```rust
pub struct AesKey(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new_encrypt(key: &[u8]) -> Result<AesKey, KeyError> { /* ... */ }
  ```
   Prepares a key for encryption.

- ```rust
  pub fn new_decrypt(key: &[u8]) -> Result<AesKey, KeyError> { /* ... */ }
  ```
   Prepares a key for decryption.

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
### Functions

#### Function `aes_ige`

**Attributes:**

- `#[doc(alias = "AES_ige_encrypt")]`

 Performs AES IGE encryption or decryption

 AES IGE (Infinite Garble Extension) is a form of AES block cipher utilized in
 OpenSSL.  Infinite Garble refers to propagating forward errors.  IGE, like other
 block ciphers implemented for AES requires an initialization vector.  The IGE mode
 allows a stream of blocks to be encrypted or decrypted without having the entire
 plaintext available.  For more information, visit [AES IGE Encryption].

 This block cipher uses 16 byte blocks.  The rust implementation will panic
 if the input or output does not meet this 16-byte boundary.  Attention must
 be made in this low level implementation to pad the value to the 128-bit boundary.

 [AES IGE Encryption]: http://www.links.org/files/openssl-ige.pdf

 # Panics

 Panics if `in_` is not the same length as `out`, if that length is not a multiple of 16, or if
 `iv` is not at least 32 bytes.

This corresponds to [`AES_ige_encrypt`](https://www.openssl.org/docs/manmaster/man3/AES_ige_encrypt.html).

```rust
pub fn aes_ige(in_: &[u8], out: &mut [u8], key: &AesKey, iv: &mut [u8], mode: crate::symm::Mode) { /* ... */ }
```

#### Function `wrap_key`

**Attributes:**

- `#[doc(alias = "AES_wrap_key")]`

 Wrap a key, according to [RFC 3394](https://tools.ietf.org/html/rfc3394)

 * `key`: The key-encrypting-key to use. Must be a encrypting key
 * `iv`: The IV to use. You must use the same IV for both wrapping and unwrapping
 * `out`: The output buffer to store the ciphertext
 * `in_`: The input buffer, storing the key to be wrapped

 Returns the number of bytes written into `out`

 # Panics

 Panics if either `out` or `in_` do not have sizes that are a multiple of 8, or if
 `out` is not 8 bytes longer than `in_`

This corresponds to [`AES_wrap_key`](https://www.openssl.org/docs/manmaster/man3/AES_wrap_key.html).

```rust
pub fn wrap_key(key: &AesKey, iv: Option<[u8; 8]>, out: &mut [u8], in_: &[u8]) -> Result<usize, KeyError> { /* ... */ }
```

#### Function `unwrap_key`

**Attributes:**

- `#[doc(alias = "AES_unwrap_key")]`

 Unwrap a key, according to [RFC 3394](https://tools.ietf.org/html/rfc3394)

 * `key`: The key-encrypting-key to decrypt the wrapped key. Must be a decrypting key
 * `iv`: The same IV used for wrapping the key
 * `out`: The buffer to write the unwrapped key to
 * `in_`: The input ciphertext

 Returns the number of bytes written into `out`

 # Panics

 Panics if either `out` or `in_` do not have sizes that are a multiple of 8, or
 if `in_` is not 8 bytes longer than `out`

This corresponds to [`AES_unwrap_key`](https://www.openssl.org/docs/manmaster/man3/AES_unwrap_key.html).

```rust
pub fn unwrap_key(key: &AesKey, iv: Option<[u8; 8]>, out: &mut [u8], in_: &[u8]) -> Result<usize, KeyError> { /* ... */ }
```

## Module `asn1`

**Attributes:**

- `#[deny(missing_docs)]`

Defines the format of certificates

This module is used by [`x509`] and other certificate building functions
to describe time, strings, and objects.

Abstract Syntax Notation One is an interface description language.
The specification comes from [X.208] by OSI, and rewritten in X.680.
ASN.1 describes properties of an object with a type set.  Those types
can be atomic, structured, choice, and other (CHOICE and ANY).  These
types are expressed as a number and the assignment operator ::=  gives
the type a name.

The implementation here provides a subset of the ASN.1 types that OpenSSL
uses, especially in the properties of a certificate used in HTTPS.

[X.208]: https://www.itu.int/rec/T-REC-X.208-198811-W/en
[`x509`]: ../x509/struct.X509Builder.html

## Examples

```
use openssl::asn1::Asn1Time;
let tomorrow = Asn1Time::days_from_now(1);
```

```rust
pub mod asn1 { /* ... */ }
```

### Types

#### Struct `Asn1GeneralizedTime`

Non-UTC representation of time

If a time can be represented by UTCTime, UTCTime is used
otherwise, ASN1_GENERALIZEDTIME is used.  This would be, for
example outside the year range of 1950-2049.

[ASN1_GENERALIZEDTIME_set] documentation from OpenSSL provides
further details of implementation.  Note: these docs are from the master
branch as documentation on the 1.1.0 branch did not include this page.

[ASN1_GENERALIZEDTIME_set]: https://www.openssl.org/docs/manmaster/man3/ASN1_GENERALIZEDTIME_set.html

```rust
pub struct Asn1GeneralizedTime(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1GeneralizedTimeRef { /* ... */ }
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

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::ASN1_GENERALIZEDTIME) -> Asn1GeneralizedTime { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::ASN1_GENERALIZEDTIME { /* ... */ }
    ```

- **Freeze**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Asn1GeneralizedTimeRef { /* ... */ }
    ```

- **Send**
- **Sync**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut Asn1GeneralizedTimeRef { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1GeneralizedTimeRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Receiver**
#### Struct `Asn1GeneralizedTimeRef`

Reference to a [`Asn1GeneralizedTime`]

[`Asn1GeneralizedTime`]: struct.Asn1GeneralizedTime.html

```rust
pub struct Asn1GeneralizedTimeRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **ForeignTypeRef**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1GeneralizedTimeRef { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1GeneralizedTimeRef { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Asn1Type`

The type of an ASN.1 value.

```rust
pub struct Asn1Type(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_raw(value: c_int) -> Self { /* ... */ }
  ```
  Constructs an `Asn1Type` from a raw OpenSSL value.

- ```rust
  pub fn as_raw(self: &Self) -> c_int { /* ... */ }
  ```
  Returns the raw OpenSSL value represented by this type.

###### Trait Implementations

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Asn1Type) -> bool { /* ... */ }
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

- **Sync**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Asn1Type { /* ... */ }
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

- **Freeze**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `TimeDiff`

**Attributes:**

- `#[<cfg>(any(ossl102, boringssl, awslc))]`

Difference between two ASN1 times.

This `struct` is created by the [`diff`] method on [`Asn1TimeRef`]. See its
documentation for more.

[`diff`]: struct.Asn1TimeRef.html#method.diff
[`Asn1TimeRef`]: struct.Asn1TimeRef.html

```rust
pub struct TimeDiff {
    pub days: libc::c_int,
    pub secs: libc::c_int,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `days` | `libc::c_int` | Difference in days |
| `secs` | `libc::c_int` | Difference in seconds.<br><br>This is always less than the number of seconds in a day. |

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TimeDiff) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TimeDiff { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `Asn1Time`

Time storage and comparison

Asn1Time should be used to store and share time information
using certificates.  If Asn1Time is set using a string, it must
be in either YYMMDDHHMMSSZ, YYYYMMDDHHMMSSZ, or another ASN.1 format.

[ASN_TIME_set] documentation at OpenSSL explains the ASN.1 implementation
used by OpenSSL.

[ASN_TIME_set]: https://www.openssl.org/docs/manmaster/crypto/ASN1_TIME_set.html

```rust
pub struct Asn1Time(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn days_from_now(days: u32) -> Result<Asn1Time, ErrorStack> { /* ... */ }
  ```
  Creates a new time on specified interval in days from now

- ```rust
  pub fn from_unix(time: time_t) -> Result<Asn1Time, ErrorStack> { /* ... */ }
  ```
   Creates a new time from the specified `time_t` value

- ```rust
  pub fn from_str(s: &str) -> Result<Asn1Time, ErrorStack> { /* ... */ }
  ```
   Creates a new time corresponding to the specified ASN1 time string.

- ```rust
  pub fn from_str_x509(s: &str) -> Result<Asn1Time, ErrorStack> { /* ... */ }
  ```
   Creates a new time corresponding to the specified X509 time string.

###### Trait Implementations

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Asn1Time) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Asn1Time) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Asn1Time) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Asn1TimeRef) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Asn1TimeRef) -> bool { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1TimeRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Asn1Time) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Asn1Time) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Asn1Time) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Asn1TimeRef) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Asn1TimeRef) -> Option<Ordering> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Asn1TimeRef { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1TimeRef { /* ... */ }
    ```

- **Send**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut Asn1TimeRef { /* ... */ }
    ```

- **UnwindSafe**
- **Receiver**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::ASN1_TIME) -> Asn1Time { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::ASN1_TIME { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `Asn1TimeRef`

Reference to an [`Asn1Time`]

[`Asn1Time`]: struct.Asn1Time.html

```rust
pub struct Asn1TimeRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn diff(self: &Self, compare: &Self) -> Result<TimeDiff, ErrorStack> { /* ... */ }
  ```
   Find difference between two times

- ```rust
  pub fn compare(self: &Self, other: &Self) -> Result<Ordering, ErrorStack> { /* ... */ }
  ```
   Compare two times

###### Trait Implementations

- **RefUnwindSafe**
- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Asn1TimeRef) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Asn1Time) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Asn1Time) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Asn1TimeRef) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Asn1TimeRef) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ForeignTypeRef**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1TimeRef { /* ... */ }
    ```

- **Send**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1TimeRef { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Asn1TimeRef) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Asn1Time) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Asn1Time) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Asn1TimeRef) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Asn1TimeRef) -> Option<Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Asn1String`

Primary ASN.1 type used by OpenSSL

Almost all ASN.1 types in OpenSSL are represented by ASN1_STRING
structures.  This implementation uses [ASN1_STRING-to_UTF8] to preserve
compatibility with Rust's String.

[ASN1_STRING-to_UTF8]: https://www.openssl.org/docs/manmaster/crypto/ASN1_STRING_to_UTF8.html

```rust
pub struct Asn1String(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Asn1StringRef { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1StringRef { /* ... */ }
    ```

- **UnwindSafe**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut Asn1StringRef { /* ... */ }
    ```

- **Receiver**
- **Freeze**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::ASN1_STRING) -> Asn1String { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::ASN1_STRING { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1StringRef { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Asn1StringRef`

A reference to an [`Asn1String`].

```rust
pub struct Asn1StringRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn as_utf8(self: &Self) -> Result<OpensslString, ErrorStack> { /* ... */ }
  ```
   Converts the ASN.1 underlying format to UTF8

- ```rust
  pub fn as_slice(self: &Self) -> &[u8] { /* ... */ }
  ```
   Return the string as an array of bytes.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
   Returns the number of bytes in the string.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Determines if the string is empty.

###### Trait Implementations

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1StringRef { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1StringRef { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ForeignTypeRef**
- **RefUnwindSafe**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

#### Struct `Asn1Integer`

Numeric representation

Integers in ASN.1 may include BigNum, int64 or uint64.  BigNum implementation
can be found within [`bn`] module.

OpenSSL documentation includes [`ASN1_INTEGER_set`].

[`bn`]: ../bn/index.html
[`ASN1_INTEGER_set`]: https://www.openssl.org/docs/manmaster/crypto/ASN1_INTEGER_set.html

```rust
pub struct Asn1Integer(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_bn(bn: &BigNumRef) -> Result<Self, ErrorStack> { /* ... */ }
  ```
  Converts a bignum to an `Asn1Integer`.

###### Trait Implementations

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut Asn1IntegerRef { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::ASN1_INTEGER) -> Asn1Integer { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::ASN1_INTEGER { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Asn1IntegerRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Asn1Integer) -> bool { /* ... */ }
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

  - ```rust
    fn borrow(self: &Self) -> &Asn1IntegerRef { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Asn1Integer) -> Option<Ordering> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1IntegerRef { /* ... */ }
    ```

- **Eq**
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

- **Receiver**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
#### Struct `Asn1IntegerRef`

A reference to an [`Asn1Integer`].

```rust
pub struct Asn1IntegerRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn get(self: &Self) -> i64 { /* ... */ }
  ```

- ```rust
  pub fn to_bn(self: &Self) -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Converts the integer to a `BigNum`.

- ```rust
  pub fn set(self: &mut Self, value: i32) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the ASN.1 value to the value of a signed 32-bit integer, for larger numbers

- ```rust
  pub fn to_owned(self: &Self) -> Result<Asn1Integer, ErrorStack> { /* ... */ }
  ```
   Creates a new Asn1Integer with the same value.

###### Trait Implementations

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1IntegerRef { /* ... */ }
    ```

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Asn1IntegerRef) -> bool { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> Ordering { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1IntegerRef { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Asn1IntegerRef) -> Option<Ordering> { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ForeignTypeRef**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `Asn1BitString`

Sequence of bytes

Asn1BitString is used in [`x509`] certificates for the signature.
The bit string acts as a collection of bytes.

[`x509`]: ../x509/struct.X509.html#method.signature

```rust
pub struct Asn1BitString(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1BitStringRef { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::ASN1_BIT_STRING) -> Asn1BitString { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::ASN1_BIT_STRING { /* ... */ }
    ```

- **Unpin**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Asn1BitStringRef { /* ... */ }
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

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut Asn1BitStringRef { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Receiver**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1BitStringRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

#### Struct `Asn1BitStringRef`

A reference to an [`Asn1BitString`].

```rust
pub struct Asn1BitStringRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn as_slice(self: &Self) -> &[u8] { /* ... */ }
  ```
   Returns the Asn1BitString as a slice.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
   Returns the number of bytes in the string.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Determines if the string is empty.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1BitStringRef { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **ForeignTypeRef**
- **UnwindSafe**
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

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1BitStringRef { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `Asn1OctetString`

ASN.1 OCTET STRING type

```rust
pub struct Asn1OctetString(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new_from_bytes(value: &[u8]) -> Result<Self, ErrorStack> { /* ... */ }
  ```
  Creates an Asn1OctetString from bytes

###### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1OctetStringRef { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Receiver**
- **Send**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::ASN1_OCTET_STRING) -> Asn1OctetString { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::ASN1_OCTET_STRING { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1OctetStringRef { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Asn1OctetStringRef { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut Asn1OctetStringRef { /* ... */ }
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
#### Struct `Asn1OctetStringRef`

A reference to an [`Asn1OctetString`].

```rust
pub struct Asn1OctetStringRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn as_slice(self: &Self) -> &[u8] { /* ... */ }
  ```
   Returns the octet string as an array of bytes.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
   Returns the number of bytes in the octet string.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Determines if the string is empty.

###### Trait Implementations

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

- **Freeze**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ForeignTypeRef**
- **Unpin**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1OctetStringRef { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1OctetStringRef { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Asn1Object`

Object Identifier

Represents an ASN.1 Object.  Typically, NIDs, or numeric identifiers
are stored as a table within the [`Nid`] module.  These constants are
used to determine attributes of a certificate, such as mapping the
attribute "CommonName" to "CN" which is represented as the OID of 13.
This attribute is a constant in the [`nid::COMMONNAME`].

OpenSSL documentation at [`OBJ_nid2obj`]

[`Nid`]: ../nid/index.html
[`nid::COMMONNAME`]: ../nid/constant.COMMONNAME.html
[`OBJ_nid2obj`]: https://www.openssl.org/docs/manmaster/crypto/OBJ_obj2nid.html

```rust
pub struct Asn1Object(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_str(txt: &str) -> Result<Asn1Object, ErrorStack> { /* ... */ }
  ```
   Constructs an ASN.1 Object Identifier from a string representation of the OID.

- ```rust
  pub fn as_slice(self: &Self) -> &[u8] { /* ... */ }
  ```
   Return the OID as an DER encoded array of bytes. This is the ASN.1

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1ObjectRef { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::ASN1_OBJECT) -> Asn1Object { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::ASN1_OBJECT { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Asn1ObjectRef { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Receiver**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut Asn1ObjectRef { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1ObjectRef { /* ... */ }
    ```

- **Stackable**
- **UnwindSafe**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Asn1Object { /* ... */ }
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

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Asn1ObjectRef`

A reference to an [`Asn1Object`].

```rust
pub struct Asn1ObjectRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn nid(self: &Self) -> Nid { /* ... */ }
  ```
  Returns the NID associated with this OID.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> Asn1Object { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1ObjectRef { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Send**
- **UnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ForeignTypeRef**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1ObjectRef { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Struct `Asn1Enumerated`

An ASN.1 enumerated.

```rust
pub struct Asn1Enumerated(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::ASN1_ENUMERATED) -> Asn1Enumerated { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::ASN1_ENUMERATED { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut Asn1EnumeratedRef { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1EnumeratedRef { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Send**
- **Sync**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Asn1EnumeratedRef { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1EnumeratedRef { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Receiver**
- **Freeze**
#### Struct `Asn1EnumeratedRef`

A reference to an [`Asn1Enumerated`].

```rust
pub struct Asn1EnumeratedRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn get_i64(self: &Self) -> Result<i64, ErrorStack> { /* ... */ }
  ```
   Get the value, if it fits in the required bounds.

###### Trait Implementations

- **UnwindSafe**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Asn1EnumeratedRef { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ForeignTypeRef**
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

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Asn1EnumeratedRef { /* ... */ }
    ```

- **Send**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `base64`

Base64 encoding support.

```rust
pub mod base64 { /* ... */ }
```

### Functions

#### Function `encode_block`

**Attributes:**

- `#[doc(alias = "EVP_EncodeBlock")]`

 Encodes a slice of bytes to a base64 string.

 # Panics

 Panics if the input length or computed output length overflow a signed C integer.

This corresponds to [`EVP_EncodeBlock`](https://www.openssl.org/docs/manmaster/man3/EVP_EncodeBlock.html).

```rust
pub fn encode_block(src: &[u8]) -> String { /* ... */ }
```

#### Function `decode_block`

**Attributes:**

- `#[doc(alias = "EVP_DecodeBlock")]`

 Decodes a base64-encoded string to bytes.

 # Panics

 Panics if the input length or computed output length overflow a signed C integer.

This corresponds to [`EVP_DecodeBlock`](https://www.openssl.org/docs/manmaster/man3/EVP_DecodeBlock.html).

```rust
pub fn decode_block(src: &str) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
```

## Module `bn`

BigNum implementation

Large numbers are important for a cryptographic library.  OpenSSL implementation
of BigNum uses dynamically assigned memory to store an array of bit chunks.  This
allows numbers of any size to be compared and mathematical functions performed.

OpenSSL wiki describes the [`BIGNUM`] data structure.

# Examples

```
use openssl::bn::BigNum;
use openssl::error::ErrorStack;

fn main() -> Result<(), ErrorStack> {
  let a = BigNum::new()?; // a = 0
  let b = BigNum::from_dec_str("1234567890123456789012345")?;
  let c = &a * &b;
  assert_eq!(a, c);
  Ok(())
}
```

[`BIGNUM`]: https://wiki.openssl.org/index.php/Manual:Bn_internal(3)

```rust
pub mod bn { /* ... */ }
```

### Types

#### Struct `MsbOption`

Options for the most significant bits of a randomly generated `BigNum`.

```rust
pub struct MsbOption(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

###### Trait Implementations

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Send**
- **Sync**
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
#### Struct `BigNumContext`

Temporary storage for BigNums on the secure heap

BigNum values are stored dynamically and therefore can be expensive
to allocate.  BigNumContext and the OpenSSL [`BN_CTX`] structure are used
internally when passing BigNum values between subroutines.

[`BN_CTX`]: https://www.openssl.org/docs/manmaster/crypto/BN_CTX_new.html

```rust
pub struct BigNumContext(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Result<BigNumContext, ErrorStack> { /* ... */ }
  ```
   Returns a new `BigNumContext`.

- ```rust
  pub fn new_secure() -> Result<BigNumContext, ErrorStack> { /* ... */ }
  ```
   Returns a new secure `BigNumContext`.

###### Trait Implementations

- **RefUnwindSafe**
- **Receiver**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::BN_CTX) -> BigNumContext { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::BN_CTX { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &BigNumContextRef { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &BigNumContextRef { /* ... */ }
    ```

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

- **UnwindSafe**
- **Unpin**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &BigNumContextRef { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut BigNumContextRef { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `BigNumContextRef`

Reference to [`BigNumContext`]

[`BigNumContext`]: struct.BigNumContext.html

```rust
pub struct BigNumContextRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &BigNumContextRef { /* ... */ }
    ```

- **ForeignTypeRef**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **UnwindSafe**
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

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &BigNumContextRef { /* ... */ }
    ```

- **Sync**
#### Struct `BigNum`

Dynamically sized large number implementation

Perform large number mathematics.  Create a new BigNum
with [`new`].  Perform standard mathematics on large numbers using
methods from [`Dref<Target = BigNumRef>`]

OpenSSL documentation at [`BN_new`].

[`new`]: struct.BigNum.html#method.new
[`Dref<Target = BigNumRef>`]: struct.BigNum.html#deref-methods
[`BN_new`]: https://www.openssl.org/docs/manmaster/crypto/BN_new.html

# Examples
```
use openssl::bn::BigNum;
# use openssl::error::ErrorStack;
# fn bignums() -> Result< (), ErrorStack > {
let little_big = BigNum::from_u32(std::u32::MAX)?;
assert_eq!(*&little_big.num_bytes(), 4);
# Ok(())
# }
# fn main () { bignums(); }
```

```rust
pub struct BigNum(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Creates a new `BigNum` with the value 0.

- ```rust
  pub fn new_secure() -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Returns a new secure `BigNum`.

- ```rust
  pub fn from_u32(n: u32) -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Creates a new `BigNum` with the given value.

- ```rust
  pub fn from_dec_str(s: &str) -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Creates a `BigNum` from a decimal string.

- ```rust
  pub fn from_hex_str(s: &str) -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Creates a `BigNum` from a hexadecimal string.

- ```rust
  pub fn get_rfc2409_prime_768() -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Returns a constant used in IKE as defined in [`RFC 2409`].  This prime number is in

- ```rust
  pub fn get_rfc2409_prime_1024() -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Returns a constant used in IKE as defined in [`RFC 2409`].  This prime number is in

- ```rust
  pub fn get_rfc3526_prime_1536() -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Returns a constant used in IKE as defined in [`RFC 3526`].  The prime is in the order

- ```rust
  pub fn get_rfc3526_prime_2048() -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Returns a constant used in IKE as defined in [`RFC 3526`].  The prime is in the order

- ```rust
  pub fn get_rfc3526_prime_3072() -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Returns a constant used in IKE as defined in [`RFC 3526`].  The prime is in the order

- ```rust
  pub fn get_rfc3526_prime_4096() -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Returns a constant used in IKE as defined in [`RFC 3526`].  The prime is in the order

- ```rust
  pub fn get_rfc3526_prime_6144() -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Returns a constant used in IKE as defined in [`RFC 3526`].  The prime is in the order

- ```rust
  pub fn get_rfc3526_prime_8192() -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Returns a constant used in IKE as defined in [`RFC 3526`].  The prime is in the order

- ```rust
  pub fn from_slice(n: &[u8]) -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Creates a new `BigNum` from an unsigned, big-endian encoded number of arbitrary length.

- ```rust
  pub fn copy_from_slice(self: &mut Self, n: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Copies data from a slice overwriting what was in the BigNum.

###### Trait Implementations

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &BigNumRef { /* ... */ }
    ```

- **Freeze**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::BIGNUM) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::BIGNUM { /* ... */ }
    ```

- **Unpin**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut BigNumRef { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, oth: &BigNum) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, oth: &BigNum) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, oth: &BigNumRef) -> bool { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, oth: &BigNumRef) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Ord**
  - ```rust
    fn cmp(self: &Self, oth: &BigNum) -> Ordering { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, n: i32) -> BigNum { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, oth: &BigNumRef) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

- **UnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Receiver**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, oth: &BigNumRef) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &BigNumRef { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, n: i32) -> BigNum { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, oth: &BigNumRef) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, oth: &BigNumRef) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &BigNumRef { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, oth: &BigNum) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, oth: &BigNum) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, oth: &BigNumRef) -> Option<Ordering> { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn neg(self: Self) -> BigNum { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
#### Struct `BigNumRef`

Reference to a [`BigNum`]

[`BigNum`]: struct.BigNum.html

```rust
pub struct BigNumRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
   Erases the memory used by this `BigNum`, resetting its value to 0.

- ```rust
  pub fn add_word(self: &mut Self, w: u32) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Adds a `u32` to `self`.

- ```rust
  pub fn sub_word(self: &mut Self, w: u32) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Subtracts a `u32` from `self`.

- ```rust
  pub fn mul_word(self: &mut Self, w: u32) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Multiplies a `u32` by `self`.

- ```rust
  pub fn div_word(self: &mut Self, w: u32) -> Result<u64, ErrorStack> { /* ... */ }
  ```
   Divides `self` by a `u32`, returning the remainder.

- ```rust
  pub fn mod_word(self: &Self, w: u32) -> Result<u64, ErrorStack> { /* ... */ }
  ```
   Returns the result of `self` modulo `w`.

- ```rust
  pub fn rand_range(self: &Self, rnd: &mut BigNumRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places a cryptographically-secure pseudo-random nonnegative

- ```rust
  pub fn pseudo_rand_range(self: &Self, rnd: &mut BigNumRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   The cryptographically weak counterpart to `rand_in_range`.

- ```rust
  pub fn set_bit(self: &mut Self, n: i32) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets bit `n`. Equivalent to `self |= (1 << n)`.

- ```rust
  pub fn clear_bit(self: &mut Self, n: i32) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Clears bit `n`, setting it to 0. Equivalent to `self &= ~(1 << n)`.

- ```rust
  pub fn is_bit_set(self: &Self, n: i32) -> bool { /* ... */ }
  ```
   Returns `true` if the `n`th bit of `self` is set to 1, `false` otherwise.

- ```rust
  pub fn mask_bits(self: &mut Self, n: i32) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Truncates `self` to the lowest `n` bits.

- ```rust
  pub fn lshift1(self: &mut Self, a: &BigNumRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places `a << 1` in `self`.  Equivalent to `self * 2`.

- ```rust
  pub fn rshift1(self: &mut Self, a: &BigNumRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places `a >> 1` in `self`. Equivalent to `self / 2`.

- ```rust
  pub fn checked_add(self: &mut Self, a: &BigNumRef, b: &BigNumRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places `a + b` in `self`.  [`core::ops::Add`] is also implemented for `BigNumRef`.

- ```rust
  pub fn checked_sub(self: &mut Self, a: &BigNumRef, b: &BigNumRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places `a - b` in `self`. [`core::ops::Sub`] is also implemented for `BigNumRef`.

- ```rust
  pub fn lshift(self: &mut Self, a: &BigNumRef, n: i32) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places `a << n` in `self`.  Equivalent to `a * 2 ^ n`.

- ```rust
  pub fn rshift(self: &mut Self, a: &BigNumRef, n: i32) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places `a >> n` in `self`. Equivalent to `a / 2 ^ n`.

- ```rust
  pub fn to_owned(self: &Self) -> Result<BigNum, ErrorStack> { /* ... */ }
  ```
   Creates a new BigNum with the same value.

- ```rust
  pub fn set_negative(self: &mut Self, negative: bool) { /* ... */ }
  ```
   Sets the sign of `self`.  Pass true to set `self` to a negative.  False sets

- ```rust
  pub fn ucmp(self: &Self, oth: &BigNumRef) -> Ordering { /* ... */ }
  ```
   Compare the absolute values of `self` and `oth`.

- ```rust
  pub fn is_negative(self: &Self) -> bool { /* ... */ }
  ```
   Returns `true` if `self` is negative.

- ```rust
  pub fn is_even(self: &Self) -> bool { /* ... */ }
  ```
   Returns `true` is `self` is even.

- ```rust
  pub fn is_odd(self: &Self) -> bool { /* ... */ }
  ```
   Returns `true` is `self` is odd.

- ```rust
  pub fn num_bits(self: &Self) -> i32 { /* ... */ }
  ```
   Returns the number of significant bits in `self`.

- ```rust
  pub fn num_bytes(self: &Self) -> i32 { /* ... */ }
  ```
  Returns the size of `self` in bytes. Implemented natively.

- ```rust
  pub fn rand(self: &mut Self, bits: i32, msb: MsbOption, odd: bool) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Generates a cryptographically strong pseudo-random `BigNum`, placing it in `self`.

- ```rust
  pub fn pseudo_rand(self: &mut Self, bits: i32, msb: MsbOption, odd: bool) -> Result<(), ErrorStack> { /* ... */ }
  ```
   The cryptographically weak counterpart to `rand`.  Not suitable for key generation.

- ```rust
  pub fn generate_prime(self: &mut Self, bits: i32, safe: bool, add: Option<&BigNumRef>, rem: Option<&BigNumRef>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Generates a prime number, placing it in `self`.

- ```rust
  pub fn checked_mul(self: &mut Self, a: &BigNumRef, b: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the result of `a * b` in `self`.

- ```rust
  pub fn checked_div(self: &mut Self, a: &BigNumRef, b: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the result of `a / b` in `self`. The remainder is discarded.

- ```rust
  pub fn checked_rem(self: &mut Self, a: &BigNumRef, b: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the result of `a % b` in `self`.

- ```rust
  pub fn div_rem(self: &mut Self, rem: &mut BigNumRef, a: &BigNumRef, b: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the result of `a / b` in `self` and `a % b` in `rem`.

- ```rust
  pub fn sqr(self: &mut Self, a: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the result of `a²` in `self`.

- ```rust
  pub fn nnmod(self: &mut Self, a: &BigNumRef, m: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the result of `a mod m` in `self`.  As opposed to `div_rem`

- ```rust
  pub fn mod_add(self: &mut Self, a: &BigNumRef, b: &BigNumRef, m: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the result of `(a + b) mod m` in `self`.

- ```rust
  pub fn mod_sub(self: &mut Self, a: &BigNumRef, b: &BigNumRef, m: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the result of `(a - b) mod m` in `self`.

- ```rust
  pub fn mod_mul(self: &mut Self, a: &BigNumRef, b: &BigNumRef, m: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the result of `(a * b) mod m` in `self`.

- ```rust
  pub fn mod_sqr(self: &mut Self, a: &BigNumRef, m: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the result of `a² mod m` in `self`.

- ```rust
  pub fn mod_sqrt(self: &mut Self, a: &BigNumRef, p: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places into `self` the modular square root of `a` such that `self^2 = a (mod p)`

- ```rust
  pub fn exp(self: &mut Self, a: &BigNumRef, p: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the result of `a^p` in `self`.

- ```rust
  pub fn mod_exp(self: &mut Self, a: &BigNumRef, p: &BigNumRef, m: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the result of `a^p mod m` in `self`.

- ```rust
  pub fn mod_inverse(self: &mut Self, a: &BigNumRef, n: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the inverse of `a` modulo `n` in `self`.

- ```rust
  pub fn gcd(self: &mut Self, a: &BigNumRef, b: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the greatest common denominator of `a` and `b` in `self`.

- ```rust
  pub fn is_prime(self: &Self, checks: i32, ctx: &mut BigNumContextRef) -> Result<bool, ErrorStack> { /* ... */ }
  ```
   Checks whether `self` is prime.

- ```rust
  pub fn is_prime_fasttest(self: &Self, checks: i32, ctx: &mut BigNumContextRef, do_trial_division: bool) -> Result<bool, ErrorStack> { /* ... */ }
  ```
   Checks whether `self` is prime with optional trial division.

- ```rust
  pub fn to_vec(self: &Self) -> Vec<u8> { /* ... */ }
  ```
   Returns a big-endian byte vector representation of the absolute value of `self`.

- ```rust
  pub fn to_vec_padded(self: &Self, pad_to: i32) -> Result<Vec<u8>, ErrorStack> { /* ... */ }
  ```
   Returns a big-endian byte vector representation of the absolute value of `self` padded

- ```rust
  pub fn to_dec_str(self: &Self) -> Result<OpensslString, ErrorStack> { /* ... */ }
  ```
   Returns a decimal string representation of `self`.

- ```rust
  pub fn to_hex_str(self: &Self) -> Result<OpensslString, ErrorStack> { /* ... */ }
  ```
   Returns a hexadecimal string representation of `self`.

- ```rust
  pub fn to_asn1_integer(self: &Self) -> Result<Asn1Integer, ErrorStack> { /* ... */ }
  ```
   Returns an `Asn1Integer` containing the value of `self`.

- ```rust
  pub fn set_const_time(self: &mut Self) { /* ... */ }
  ```
   Force constant time computation on this value.

- ```rust
  pub fn is_const_time(self: &Self) -> bool { /* ... */ }
  ```
   Returns true if `self` is in const time mode.

- ```rust
  pub fn is_secure(self: &Self) -> bool { /* ... */ }
  ```
   Returns true if `self` was created with [`BigNum::new_secure`].

###### Trait Implementations

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, oth: &BigNumRef) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, oth: &BigNum) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, oth: &BigNumRef) -> Option<Ordering> { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> BigNum { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Shl**
  - ```rust
    fn shl(self: Self, n: i32) -> BigNum { /* ... */ }
    ```

- **ForeignTypeRef**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, oth: &BigNumRef) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, oth: &BigNumRef) -> BigNum { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, oth: &BigNumRef) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, oth: &BigNumRef) -> BigNum { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Rem**
  - ```rust
    fn rem(self: Self, oth: &''b BigNumRef) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, oth: &BigNumRef) -> BigNum { /* ... */ }
    ```

- **Sync**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &BigNumRef { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Eq**
- **Div**
  - ```rust
    fn div(self: Self, oth: &''b BigNumRef) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, oth: &BigNumRef) -> BigNum { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Ord**
  - ```rust
    fn cmp(self: &Self, oth: &BigNumRef) -> Ordering { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, oth: &BigNumRef) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, oth: &BigNum) -> BigNum { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, oth: &BigNumRef) -> BigNum { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &BigNumRef { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, oth: &BigNumRef) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, oth: &BigNum) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, oth: &BigNumRef) -> bool { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **Shr**
  - ```rust
    fn shr(self: Self, n: i32) -> BigNum { /* ... */ }
    ```

## Module `cipher`

Symmetric ciphers.

```rust
pub mod cipher { /* ... */ }
```

### Types

#### Struct `Cipher`

A symmetric cipher.

```rust
pub struct Cipher(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_nid(nid: Nid) -> Option<&''static CipherRef> { /* ... */ }
  ```
   Looks up the cipher for a certain nid.

- ```rust
  pub fn fetch(ctx: Option<&LibCtxRef>, algorithm: &str, properties: Option<&str>) -> Result<Self, ErrorStack> { /* ... */ }
  ```
   Fetches a cipher object corresponding to the specified algorithm name and properties.

- ```rust
  pub fn aes_128_ecb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_128_cbc() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_128_xts() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_256_xts() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_128_ctr() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_128_cfb1() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_128_cfb128() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_128_cfb8() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_128_gcm() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_128_ccm() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_128_ofb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_128_ocb() -> &''static CipherRef { /* ... */ }
  ```
  Requires OpenSSL 1.1.0 or newer.

- ```rust
  pub fn aes_128_wrap() -> &''static CipherRef { /* ... */ }
  ```
  Requires OpenSSL 1.0.2 or newer.

- ```rust
  pub fn aes_128_wrap_pad() -> &''static CipherRef { /* ... */ }
  ```
  Requires OpenSSL 1.1.0 or newer.

- ```rust
  pub fn aes_192_ecb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_192_cbc() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_192_ctr() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_192_cfb1() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_192_cfb128() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_192_cfb8() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_192_gcm() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_192_ccm() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_192_ofb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_192_ocb() -> &''static CipherRef { /* ... */ }
  ```
  Requires OpenSSL 1.1.0 or newer.

- ```rust
  pub fn aes_192_wrap() -> &''static CipherRef { /* ... */ }
  ```
  Requires OpenSSL 1.0.2 or newer.

- ```rust
  pub fn aes_192_wrap_pad() -> &''static CipherRef { /* ... */ }
  ```
  Requires OpenSSL 1.1.0 or newer.

- ```rust
  pub fn aes_256_ecb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_256_cbc() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_256_ctr() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_256_cfb1() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_256_cfb128() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_256_cfb8() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_256_gcm() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_256_ccm() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_256_ofb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn aes_256_ocb() -> &''static CipherRef { /* ... */ }
  ```
  Requires OpenSSL 1.1.0 or newer.

- ```rust
  pub fn aes_256_wrap() -> &''static CipherRef { /* ... */ }
  ```
  Requires OpenSSL 1.0.2 or newer.

- ```rust
  pub fn aes_256_wrap_pad() -> &''static CipherRef { /* ... */ }
  ```
  Requires OpenSSL 1.1.0 or newer.

- ```rust
  pub fn bf_cbc() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn bf_ecb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn bf_cfb64() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn bf_ofb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn des_cbc() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn des_ecb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn des_ede3() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn des_ede3_ecb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn des_ede3_cbc() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn des_ede3_cfb8() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn des_ede3_cfb64() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn des_ede3_ofb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn rc4() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn camellia128_cfb128() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn camellia128_ecb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn camellia128_cbc() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn camellia128_ofb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn camellia192_cfb128() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn camellia192_ecb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn camellia192_cbc() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn camellia192_ofb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn camellia256_cfb128() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn camellia256_ecb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn camellia256_cbc() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn camellia256_ofb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn cast5_cfb64() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn cast5_ecb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn cast5_cbc() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn cast5_ofb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn chacha20() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn chacha20_poly1305() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn seed_cbc() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn seed_cfb128() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn seed_ecb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn seed_ofb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn sm4_ecb() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn sm4_cbc() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn sm4_ctr() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn sm4_cfb128() -> &''static CipherRef { /* ... */ }
  ```

- ```rust
  pub fn sm4_ofb() -> &''static CipherRef { /* ... */ }
  ```

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Send**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Receiver**
- **Unpin**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut <Self as >::CType) -> Self { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut <Self as >::CType { /* ... */ }
    ```

#### Struct `CipherRef`

A reference to a [`Cipher`].

```rust
pub struct CipherRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn nid(self: &Self) -> Nid { /* ... */ }
  ```
   Returns the cipher's Nid.

- ```rust
  pub fn key_length(self: &Self) -> usize { /* ... */ }
  ```
   Returns the length of keys used with this cipher.

- ```rust
  pub fn iv_length(self: &Self) -> usize { /* ... */ }
  ```
   Returns the length of the IV used with this cipher.

- ```rust
  pub fn block_size(self: &Self) -> usize { /* ... */ }
  ```
   Returns the block size of the cipher.

###### Trait Implementations

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ForeignTypeRef**
- **Sync**
## Module `cipher_ctx`

**Attributes:**

- `#[warn(missing_docs)]`

The symmetric encryption context.

# Examples

Encrypt data with AES128 CBC

```
use openssl::cipher::Cipher;
use openssl::cipher_ctx::CipherCtx;

let cipher = Cipher::aes_128_cbc();
let data = b"Some Crypto Text";
let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

let mut ctx = CipherCtx::new().unwrap();
ctx.encrypt_init(Some(cipher), Some(key), Some(iv)).unwrap();

let mut ciphertext = vec![];
ctx.cipher_update_vec(data, &mut ciphertext).unwrap();
ctx.cipher_final_vec(&mut ciphertext).unwrap();

assert_eq!(
    b"\xB4\xB9\xE7\x30\xD6\xD6\xF7\xDE\x77\x3F\x1C\xFF\xB3\x3E\x44\x5A\x91\xD7\x27\x62\x87\x4D\
      \xFB\x3C\x5E\xC4\x59\x72\x4A\xF4\x7C\xA1",
    &ciphertext[..],
);
```

Decrypt data with AES128 CBC

```
use openssl::cipher::Cipher;
use openssl::cipher_ctx::CipherCtx;

let cipher = Cipher::aes_128_cbc();
let data = b"\xB4\xB9\xE7\x30\xD6\xD6\xF7\xDE\x77\x3F\x1C\xFF\xB3\x3E\x44\x5A\x91\xD7\x27\x62\
             \x87\x4D\xFB\x3C\x5E\xC4\x59\x72\x4A\xF4\x7C\xA1";
let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

let mut ctx = CipherCtx::new().unwrap();
ctx.decrypt_init(Some(cipher), Some(key), Some(iv)).unwrap();

let mut plaintext = vec![];
ctx.cipher_update_vec(data, &mut plaintext).unwrap();
ctx.cipher_final_vec(&mut plaintext).unwrap();

assert_eq!(b"Some Crypto Text", &plaintext[..]);
```

```rust
pub mod cipher_ctx { /* ... */ }
```

### Types

#### Struct `CipherCtx`

A context object used to perform symmetric encryption operations.

```rust
pub struct CipherCtx(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Result<Self, ErrorStack> { /* ... */ }
  ```
   Creates a new context.

###### Trait Implementations

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &CipherCtxRef { /* ... */ }
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

- **Receiver**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &CipherCtxRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &CipherCtxRef { /* ... */ }
    ```

- **UnwindSafe**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut CipherCtxRef { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::EVP_CIPHER_CTX) -> CipherCtx { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::EVP_CIPHER_CTX { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `CipherCtxRef`

A reference to a [`CipherCtx`].

```rust
pub struct CipherCtxRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn copy(self: &mut Self, src: &CipherCtxRef) -> Result<(), ErrorStack> { /* ... */ }
  ```

- ```rust
  pub fn encrypt_init(self: &mut Self, type_: Option<&CipherRef>, key: Option<&[u8]>, iv: Option<&[u8]>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Initializes the context for encryption.

- ```rust
  pub fn decrypt_init(self: &mut Self, type_: Option<&CipherRef>, key: Option<&[u8]>, iv: Option<&[u8]>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Initializes the context for decryption.

- ```rust
  pub fn seal_init<T>(self: &mut Self, type_: Option<&CipherRef>, pub_keys: &[PKey<T>], encrypted_keys: &mut [Vec<u8>], iv: Option<&mut [u8]>) -> Result<(), ErrorStack>
where
    T: HasPublic { /* ... */ }
  ```
   Initializes the context to perform envelope encryption.

- ```rust
  pub fn open_init<T>(self: &mut Self, type_: Option<&CipherRef>, encrypted_key: &[u8], iv: Option<&[u8]>, priv_key: Option<&PKeyRef<T>>) -> Result<(), ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
   Initializes the context to perform envelope decryption.

- ```rust
  pub fn block_size(self: &Self) -> usize { /* ... */ }
  ```
   Returns the block size of the context's cipher.

- ```rust
  pub fn key_length(self: &Self) -> usize { /* ... */ }
  ```
   Returns the key length of the context's cipher.

- ```rust
  pub fn rand_key(self: &Self, buf: &mut [u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Generates a random key based on the configured cipher.

- ```rust
  pub fn set_key_length(self: &mut Self, len: usize) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the length of the key expected by the context.

- ```rust
  pub fn iv_length(self: &Self) -> usize { /* ... */ }
  ```
   Returns the length of the IV expected by this context.

- ```rust
  pub fn num(self: &Self) -> usize { /* ... */ }
  ```
   Returns the `num` parameter of the cipher.

- ```rust
  pub fn set_iv_length(self: &mut Self, len: usize) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the length of the IV expected by this context.

- ```rust
  pub fn tag_length(self: &Self) -> usize { /* ... */ }
  ```
   Returns the length of the authentication tag expected by this context.

- ```rust
  pub fn tag(self: &Self, tag: &mut [u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Retrieves the calculated authentication tag from the context.

- ```rust
  pub fn set_tag_length(self: &mut Self, len: usize) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the length of the generated authentication tag.

- ```rust
  pub fn set_tag(self: &mut Self, tag: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the authentication tag for verification during decryption.

- ```rust
  pub fn set_padding(self: &mut Self, padding: bool) { /* ... */ }
  ```
   Enables or disables padding.

- ```rust
  pub fn set_data_len(self: &mut Self, len: usize) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the total length of plaintext data.

- ```rust
  pub fn set_flags(self: &mut Self, flags: CipherCtxFlags) { /* ... */ }
  ```
   Set ctx flags.

- ```rust
  pub fn cipher_update(self: &mut Self, input: &[u8], output: Option<&mut [u8]>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Writes data into the context.

- ```rust
  pub unsafe fn cipher_update_unchecked(self: &mut Self, input: &[u8], output: Option<&mut [u8]>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Writes data into the context.

- ```rust
  pub fn cipher_update_vec(self: &mut Self, input: &[u8], output: &mut Vec<u8>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Like [`Self::cipher_update`] except that it appends output to a [`Vec`].

- ```rust
  pub fn cipher_update_inplace(self: &mut Self, data: &mut [u8], inlen: usize) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Like [`Self::cipher_update`] except that it writes output into the

- ```rust
  pub fn cipher_final(self: &mut Self, output: &mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Finalizes the encryption or decryption process.

- ```rust
  pub unsafe fn cipher_final_unchecked(self: &mut Self, output: &mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Finalizes the encryption or decryption process.

- ```rust
  pub fn cipher_final_vec(self: &mut Self, output: &mut Vec<u8>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Like [`Self::cipher_final`] except that it appends output to a [`Vec`].

###### Trait Implementations

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &CipherCtxRef { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &CipherCtxRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ForeignTypeRef**
- **RefUnwindSafe**
- **Sync**
#### Struct `CipherCtxFlags`

Flags for `EVP_CIPHER_CTX`.

```rust
pub struct CipherCtxFlags(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> c_int { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: c_int) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: c_int) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: c_int) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<CipherCtxFlags> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<CipherCtxFlags> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **Unpin**
- **RefUnwindSafe**
- **Flags**
  - ```rust
    fn bits(self: &Self) -> c_int { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: c_int) -> CipherCtxFlags { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Send**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PublicFlags**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: CipherCtxFlags) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Sync**
- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `cms`

**Attributes:**

- `#[<cfg>(all(not(libressl), not(osslconf = "OPENSSL_NO_CMS")))]`

SMIME implementation using CMS

CMS (PKCS#7) is an encryption standard.  It allows signing and encrypting data using
X.509 certificates.  The OpenSSL implementation of CMS is used in email encryption
generated from a `Vec` of bytes.  This `Vec` follows the smime protocol standards.
Data accepted by this module will be smime type `enveloped-data`.

```rust
pub mod cms { /* ... */ }
```

### Types

#### Struct `CMSOptions`

```rust
pub struct CMSOptions(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> c_uint { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: c_uint) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: c_uint) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: c_uint) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<CMSOptions> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<CMSOptions> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &CMSOptions) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CMSOptions { /* ... */ }
    ```

- **Unpin**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &CMSOptions) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CMSOptions) -> bool { /* ... */ }
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
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: CMSOptions) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Send**
- **PublicFlags**
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

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Flags**
  - ```rust
    fn bits(self: &Self) -> c_uint { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: c_uint) -> CMSOptions { /* ... */ }
    ```

#### Struct `CmsContentInfo`

High level CMS wrapper

CMS supports nesting various types of data, including signatures, certificates,
encrypted data, smime messages (encrypted email), and data digest.  The ContentInfo
content type is the encapsulation of all those content types.  [`RFC 5652`] describes
CMS and OpenSSL follows this RFC's implementation.

[`RFC 5652`]: https://tools.ietf.org/html/rfc5652#page-6

```rust
pub struct CmsContentInfo(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn smime_read_cms(smime: &[u8]) -> Result<CmsContentInfo, ErrorStack> { /* ... */ }
  ```
   Parses a smime formatted `vec` of bytes into a `CmsContentInfo`.

- ```rust
  pub fn from_der(der: &[u8]) -> Result<CmsContentInfo, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a DER-encoded ContentInfo structure.

- ```rust
  pub fn from_pem(pem: &[u8]) -> Result<CmsContentInfo, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a PEM-encoded ContentInfo structure.

- ```rust
  pub fn sign<T>(signcert: Option<&X509Ref>, pkey: Option<&PKeyRef<T>>, certs: Option<&StackRef<X509>>, data: Option<&[u8]>, flags: CMSOptions) -> Result<CmsContentInfo, ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
   Given a signing cert `signcert`, private key `pkey`, a certificate stack `certs`,

- ```rust
  pub fn encrypt(certs: &StackRef<X509>, data: &[u8], cipher: Cipher, flags: CMSOptions) -> Result<CmsContentInfo, ErrorStack> { /* ... */ }
  ```
   Given a certificate stack `certs`, data `data`, cipher `cipher` and flags `flags`,

- ```rust
  pub fn verify(self: &mut Self, certs: Option<&StackRef<X509>>, store: Option<&X509StoreRef>, detached_data: Option<&[u8]>, output_data: Option<&mut Vec<u8>>, flags: CMSOptions) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Verify this CmsContentInfo's signature,

###### Trait Implementations

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &CmsContentInfoRef { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &CmsContentInfoRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &CmsContentInfoRef { /* ... */ }
    ```

- **Receiver**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::CMS_ContentInfo) -> CmsContentInfo { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::CMS_ContentInfo { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut CmsContentInfoRef { /* ... */ }
    ```

- **Unpin**
#### Struct `CmsContentInfoRef`

Reference to [`CMSContentInfo`]

[`CMSContentInfo`]:struct.CmsContentInfo.html

```rust
pub struct CmsContentInfoRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn decrypt<T>(self: &Self, pkey: &PKeyRef<T>, cert: &X509) -> Result<Vec<u8>, ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
   Given the sender's private key, `pkey` and the recipient's certificate, `cert`,

- ```rust
  pub fn decrypt_without_cert_check<T>(self: &Self, pkey: &PKeyRef<T>) -> Result<Vec<u8>, ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
   Given the sender's private key, `pkey`,

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes this CmsContentInfo using DER.

- ```rust
  pub fn to_pem(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes this CmsContentInfo using DER.

###### Trait Implementations

- **Sync**
- **UnwindSafe**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &CmsContentInfoRef { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &CmsContentInfoRef { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ForeignTypeRef**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `conf`

Interface for processing OpenSSL configuration files.

```rust
pub mod conf { /* ... */ }
```

### Types

#### Struct `Conf`

```rust
pub struct Conf(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(method: ConfMethod) -> Result<Conf, ErrorStack> { /* ... */ }
  ```
   Create a configuration parser.

###### Trait Implementations

- **Sync**
- **RefUnwindSafe**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::CONF) -> Conf { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::CONF { /* ... */ }
    ```

- **Unpin**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &ConfRef { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Freeze**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &ConfRef { /* ... */ }
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

- **Receiver**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut ConfRef { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &ConfRef { /* ... */ }
    ```

#### Struct `ConfRef`

```rust
pub struct ConfRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ForeignTypeRef**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **Send**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &ConfRef { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &ConfRef { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
### Re-exports

#### Re-export `methods::*`

**Attributes:**

- `#[<cfg>(not(any(boringssl, libressl400, awslc)))]`

```rust
pub use methods::*;
```

## Module `derive`

Shared secret derivation.

# Example

The following example implements [ECDH] using `NIST P-384` keys:

```
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# use std::convert::TryInto;
use openssl::bn::BigNumContext;
use openssl::pkey::PKey;
use openssl::derive::Deriver;
use openssl::ec::{EcGroup, EcKey, EcPoint, PointConversionForm};
use openssl::nid::Nid;

let group = EcGroup::from_curve_name(Nid::SECP384R1)?;

let first: PKey<_> = EcKey::generate(&group)?.try_into()?;

// second party generates an ephemeral key and derives
// a shared secret using first party's public key
let shared_key = EcKey::generate(&group)?;
// shared_public is sent to first party
let mut ctx = BigNumContext::new()?;
let shared_public = shared_key.public_key().to_bytes(
       &group,
       PointConversionForm::COMPRESSED,
       &mut ctx,
   )?;

let shared_key: PKey<_> = shared_key.try_into()?;
let mut deriver = Deriver::new(&shared_key)?;
deriver.set_peer(&first)?;
// secret can be used e.g. as a symmetric encryption key
let secret = deriver.derive_to_vec()?;
# drop(deriver);

// first party derives the same shared secret using
// shared_public
let point = EcPoint::from_bytes(&group, &shared_public, &mut ctx)?;
let recipient_key: PKey<_> = EcKey::from_public_key(&group, &point)?.try_into()?;
let mut deriver = Deriver::new(&first)?;
deriver.set_peer(&recipient_key)?;
let first_secret = deriver.derive_to_vec()?;

assert_eq!(secret, first_secret);
# Ok(()) }
```

[ECDH]: https://wiki.openssl.org/index.php/Elliptic_Curve_Diffie_Hellman

```rust
pub mod derive { /* ... */ }
```

### Types

#### Struct `Deriver`

A type used to derive a shared secret between two keys.

```rust
pub struct Deriver<''a>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new<T>(key: &''a PKeyRef<T>) -> Result<Deriver<''a>, ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
  Creates a new `Deriver` using the provided private key.

- ```rust
  pub fn set_peer<T>(self: &mut Self, key: &''a PKeyRef<T>) -> Result<(), ErrorStack>
where
    T: HasPublic { /* ... */ }
  ```
   Sets the peer key used for secret derivation.

- ```rust
  pub fn set_peer_ex<T>(self: &mut Self, key: &''a PKeyRef<T>, validate_peer: bool) -> Result<(), ErrorStack>
where
    T: HasPublic { /* ... */ }
  ```
   Sets the peer key used for secret derivation along with optionally validating the peer public key.

- ```rust
  pub fn len(self: &mut Self) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Returns the size of the shared secret.

- ```rust
  pub fn derive(self: &mut Self, buf: &mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Derives a shared secret between the two keys, writing it into the buffer.

- ```rust
  pub fn derive_to_vec(self: &mut Self) -> Result<Vec<u8>, ErrorStack> { /* ... */ }
  ```
  A convenience function which derives a shared secret and returns it in a new buffer.

###### Trait Implementations

- **Unpin**
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

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
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
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **RefUnwindSafe**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
## Module `dh`

Diffie-Hellman key agreement.

```rust
pub mod dh { /* ... */ }
```

### Types

#### Struct `Dh`

```rust
pub struct Dh<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_params(p: BigNum, g: BigNum, q: BigNum) -> Result<Dh<Params>, ErrorStack> { /* ... */ }
  ```

- ```rust
  pub fn from_pqg(prime_p: BigNum, prime_q: Option<BigNum>, generator: BigNum) -> Result<Dh<Params>, ErrorStack> { /* ... */ }
  ```
   Creates a DH instance based upon the given primes and generator params.

- ```rust
  pub fn set_public_key(self: Self, pub_key: BigNum) -> Result<Dh<Public>, ErrorStack> { /* ... */ }
  ```
  Sets the public key on the DH object.

- ```rust
  pub fn set_private_key(self: Self, priv_key: BigNum) -> Result<Dh<Private>, ErrorStack> { /* ... */ }
  ```
  Sets the private key on the DH object and recomputes the public key.

- ```rust
  pub fn set_key(self: Self, pub_key: BigNum, priv_key: BigNum) -> Result<Dh<Private>, ErrorStack> { /* ... */ }
  ```
  Sets the public and private keys on the DH object.

- ```rust
  pub fn generate_params(prime_len: u32, generator: u32) -> Result<Dh<Params>, ErrorStack> { /* ... */ }
  ```
   Generates DH params based on the given `prime_len` and a fixed `generator` value.

- ```rust
  pub fn generate_key(self: Self) -> Result<Dh<Private>, ErrorStack> { /* ... */ }
  ```
   Generates a public and a private key based on the DH params.

- ```rust
  pub fn params_from_pem(pem: &[u8]) -> Result<Dh<Params>, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a PEM-encoded PKCS#3 DHpararameters structure.

- ```rust
  pub fn params_from_der(der: &[u8]) -> Result<Dh<Params>, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a DER-encoded PKCS#3 DHparameters structure.

- ```rust
  pub fn get_1024_160() -> Result<Dh<Params>, ErrorStack> { /* ... */ }
  ```
   Requires OpenSSL 1.0.2 or newer.

- ```rust
  pub fn get_2048_224() -> Result<Dh<Params>, ErrorStack> { /* ... */ }
  ```
   Requires OpenSSL 1.0.2 or newer.

- ```rust
  pub fn get_2048_256() -> Result<Dh<Params>, ErrorStack> { /* ... */ }
  ```
   Requires OpenSSL 1.0.2 or newer.

- ```rust
  pub fn prime_p(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns the prime `p` from the DH instance.

- ```rust
  pub fn prime_q(self: &Self) -> Option<&BigNumRef> { /* ... */ }
  ```
   Returns the prime `q` from the DH instance.

- ```rust
  pub fn generator(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns the generator from the DH instance.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(dh: Dh<T>) -> Result<PKey<T>, ErrorStack> { /* ... */ }
    ```

  - ```rust
    fn try_from(pkey: PKey<T>) -> Result<Dh<T>, ErrorStack> { /* ... */ }
    ```

- **Receiver**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::DH) -> Dh<T> { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::DH { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &DhRef<T> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut DhRef<T> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &DhRef<T> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &DhRef<T> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Freeze**
#### Struct `DhRef`

```rust
pub struct DhRef<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn params_to_pem(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the parameters into a PEM-encoded PKCS#3 DHparameter structure.

- ```rust
  pub fn params_to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the parameters into a DER-encoded PKCS#3 DHparameter structure.

- ```rust
  pub fn check_key(self: &Self) -> Result<bool, ErrorStack> { /* ... */ }
  ```
   Validates DH parameters for correctness

- ```rust
  pub fn public_key(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns the public key from the DH instance.

- ```rust
  pub fn compute_key(self: &Self, public_key: &BigNumRef) -> Result<Vec<u8>, ErrorStack> { /* ... */ }
  ```
   Computes a shared secret from the own private key and the given `public_key`.

- ```rust
  pub fn private_key(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns the private key from the DH instance.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &DhRef<T> { /* ... */ }
    ```

- **UnwindSafe**
- **ForeignTypeRef**
- **Unpin**
- **Freeze**
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

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &DhRef<T> { /* ... */ }
    ```

## Module `dsa`

Digital Signatures

DSA ensures a message originated from a known sender, and was not modified.
DSA uses asymmetrical keys and an algorithm to output a signature of the message
using the private key that can be validated with the public key but not be generated
without the private key.

```rust
pub mod dsa { /* ... */ }
```

### Types

#### Struct `Dsa`

Object representing DSA keys.

A DSA object contains the parameters p, q, and g.  There is a private
and public key.  The values p, g, and q are:

* `p`: DSA prime parameter
* `q`: DSA sub-prime parameter
* `g`: DSA base parameter

These values are used to calculate a pair of asymmetrical keys used for
signing.

OpenSSL documentation at [`DSA_new`]

[`DSA_new`]: https://www.openssl.org/docs/manmaster/crypto/DSA_new.html

# Examples

```
use openssl::dsa::Dsa;
use openssl::error::ErrorStack;
use openssl::pkey::Private;

fn create_dsa() -> Result<Dsa<Private>, ErrorStack> {
    let sign = Dsa::generate(2048)?;
    Ok(sign)
}
# fn main() {
#    create_dsa();
# }
```

```rust
pub struct Dsa<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_pqg(p: BigNum, q: BigNum, g: BigNum) -> Result<Dsa<Params>, ErrorStack> { /* ... */ }
  ```
   Creates a DSA params based upon the given parameters.

- ```rust
  pub fn generate_params(bits: u32) -> Result<Dsa<Params>, ErrorStack> { /* ... */ }
  ```
   Generates DSA params based on the given number of bits.

- ```rust
  pub fn generate_key(self: Self) -> Result<Dsa<Private>, ErrorStack> { /* ... */ }
  ```
   Generates a private key based on the DSA params.

- ```rust
  pub fn generate(bits: u32) -> Result<Dsa<Private>, ErrorStack> { /* ... */ }
  ```
  Generate a DSA key pair.

- ```rust
  pub fn from_private_components(p: BigNum, q: BigNum, g: BigNum, priv_key: BigNum, pub_key: BigNum) -> Result<Dsa<Private>, ErrorStack> { /* ... */ }
  ```
  Create a DSA key pair with the given parameters

- ```rust
  pub fn public_key_from_pem(pem: &[u8]) -> Result<Dsa<Public>, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a PEM-encoded SubjectPublicKeyInfo structure containing a DSA key.

- ```rust
  pub fn public_key_from_der(der: &[u8]) -> Result<Dsa<Public>, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a DER-encoded SubjectPublicKeyInfo structure containing a DSA key.

- ```rust
  pub fn from_public_components(p: BigNum, q: BigNum, g: BigNum, pub_key: BigNum) -> Result<Dsa<Public>, ErrorStack> { /* ... */ }
  ```
  Create a new DSA key with only public components.

###### Trait Implementations

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &DsaRef<T> { /* ... */ }
    ```

- **Unpin**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &DsaRef<T> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Dsa<T> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
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

  - ```rust
    fn borrow(self: &Self) -> &DsaRef<T> { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::DSA) -> Dsa<T> { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::DSA { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Receiver**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(dsa: Dsa<T>) -> Result<PKey<T>, ErrorStack> { /* ... */ }
    ```

  - ```rust
    fn try_from(pkey: PKey<T>) -> Result<Dsa<T>, ErrorStack> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Send**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut DsaRef<T> { /* ... */ }
    ```

#### Struct `DsaRef`

Reference to [`Dsa`].

[`Dsa`]: struct.Dsa.html

```rust
pub struct DsaRef<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn public_key_to_pem(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the public key into a PEM-encoded SubjectPublicKeyInfo structure.

- ```rust
  pub fn public_key_to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the public key into a DER-encoded SubjectPublicKeyInfo structure.

- ```rust
  pub fn pub_key(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns a reference to the public key component of `self`.

- ```rust
  pub fn private_key_to_pem(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the private key to a PEM-encoded DSAPrivateKey structure.

- ```rust
  pub fn private_key_to_pem_passphrase(self: &Self, cipher: crate::symm::Cipher, passphrase: &[u8]) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the private key to a PEM-encoded encrypted DSAPrivateKey structure.

- ```rust
  pub fn private_key_to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the private_key to a DER-encoded `DSAPrivateKey` structure.

- ```rust
  pub fn priv_key(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns a reference to the private key component of `self`.

- ```rust
  pub fn size(self: &Self) -> u32 { /* ... */ }
  ```
   Returns the maximum size of the signature output by `self` in bytes.

- ```rust
  pub fn p(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns the DSA prime parameter of `self`.

- ```rust
  pub fn q(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns the DSA sub-prime parameter of `self`.

- ```rust
  pub fn g(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns the DSA base parameter of `self`.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &DsaRef<T> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ForeignTypeRef**
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
    fn as_ref(self: &Self) -> &DsaRef<T> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> Dsa<T> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
- **Sync**
#### Struct `DsaSig`

Object representing DSA signature.

DSA signatures consist of two components: `r` and `s`.

# Examples

```
use std::convert::TryInto;

use openssl::bn::BigNum;
use openssl::dsa::{Dsa, DsaSig};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sign::{Signer, Verifier};

const TEST_DATA: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
let dsa_ref = Dsa::generate(1024).unwrap();

let pub_key: PKey<_> = dsa_ref.clone().try_into().unwrap();
let priv_key: PKey<_> = dsa_ref.try_into().unwrap();

let mut signer = if let Ok(signer) = Signer::new(MessageDigest::sha256(), &priv_key) {
    signer
} else {
    // DSA signing is not supported (eg. BoringSSL)
    return;
};

signer.update(TEST_DATA).unwrap();

let signature = signer.sign_to_vec().unwrap();
// Parse DER-encoded DSA signature
let signature = DsaSig::from_der(&signature).unwrap();

// Extract components `r` and `s`
let r = BigNum::from_slice(&signature.r().to_vec()).unwrap();
let s = BigNum::from_slice(&signature.s().to_vec()).unwrap();

// Construct new DSA signature from components
let signature = DsaSig::from_private_components(r, s).unwrap();

// Serialize DSA signature to DER
let signature = signature.to_der().unwrap();

let mut verifier = Verifier::new(MessageDigest::sha256(), &pub_key).unwrap();
verifier.update(TEST_DATA).unwrap();
assert!(verifier.verify(&signature[..]).unwrap());
```

```rust
pub struct DsaSig(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_private_components(r: BigNum, s: BigNum) -> Result<Self, ErrorStack> { /* ... */ }
  ```
   Returns a new `DsaSig` by setting the `r` and `s` values associated with an DSA signature.

- ```rust
  pub fn from_der(der: &[u8]) -> Result<DsaSig, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a DER-encoded DSA signature.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut DsaSigRef { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &DsaSigRef { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &DsaSigRef { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::DSA_SIG) -> DsaSig { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::DSA_SIG { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &DsaSigRef { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Receiver**
- **UnwindSafe**
#### Struct `DsaSigRef`

Reference to a [`DsaSig`].

```rust
pub struct DsaSigRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the DSA signature into a DER-encoded `DSASignature` structure.

- ```rust
  pub fn r(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns internal component `r` of an `DsaSig`.

- ```rust
  pub fn s(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns internal component `s` of an `DsaSig`.

###### Trait Implementations

- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &DsaSigRef { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &DsaSigRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ForeignTypeRef**
- **Send**
## Module `ec`

Elliptic Curve

Cryptography relies on the difficulty of solving mathematical problems, such as the factor
of large integers composed of two large prime numbers and the discrete logarithm of a
random elliptic curve.  This module provides low-level features of the latter.
Elliptic Curve protocols can provide the same security with smaller keys.

There are 2 forms of elliptic curves, `Fp` and `F2^m`.  These curves use irreducible
trinomial or pentanomial.  Being a generic interface to a wide range of algorithms,
the curves are generally referenced by [`EcGroup`].  There are many built-in groups
found in [`Nid`].

OpenSSL Wiki explains the fields and curves in detail at [Elliptic Curve Cryptography].

[`EcGroup`]: struct.EcGroup.html
[`Nid`]: ../nid/struct.Nid.html
[Elliptic Curve Cryptography]: https://wiki.openssl.org/index.php/Elliptic_Curve_Cryptography

```rust
pub mod ec { /* ... */ }
```

### Types

#### Struct `PointConversionForm`

Compressed or Uncompressed conversion

Conversion from the binary value of the point on the curve is performed in one of
compressed, uncompressed, or hybrid conversions.  The default is compressed, except
for binary curves.

Further documentation is available in the [X9.62] standard.

[X9.62]: http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.202.2977&rep=rep1&type=pdf

```rust
pub struct PointConversionForm(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

###### Trait Implementations

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
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PointConversionForm { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `Asn1Flag`

Named Curve or Explicit

This type acts as a boolean as to whether the `EcGroup` is named or explicit.

```rust
pub struct Asn1Flag(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Unpin**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Asn1Flag { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Asn1Flag) -> bool { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
#### Struct `EcGroup`

Describes the curve

A curve can be of the named curve type.  These curves can be discovered
using openssl binary `openssl ecparam -list_curves`.  Other operations
are available in the [wiki].  These named curves are available in the
[`Nid`] module.

Curves can also be generated using prime field parameters or a binary field.

Prime fields use the formula `y^2 mod p = x^3 + ax + b mod p`.  Binary
fields use the formula `y^2 + xy = x^3 + ax^2 + b`.  Named curves have
assured security.  To prevent accidental vulnerabilities, they should
be preferred.

[wiki]: https://wiki.openssl.org/index.php/Command_Line_Elliptic_Curve_Operations
[`Nid`]: ../nid/index.html

```rust
pub struct EcGroup(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_curve_name(nid: Nid) -> Result<EcGroup, ErrorStack> { /* ... */ }
  ```
   Returns the group of a standard named curve.

- ```rust
  pub fn from_components(p: BigNum, a: BigNum, b: BigNum, ctx: &mut BigNumContextRef) -> Result<EcGroup, ErrorStack> { /* ... */ }
  ```
   Returns the group for given parameters

###### Trait Implementations

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &EcGroupRef { /* ... */ }
    ```

- **Unpin**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Receiver**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &EcGroupRef { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &EcGroupRef { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::EC_GROUP) -> EcGroup { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::EC_GROUP { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut EcGroupRef { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `EcGroupRef`

Reference to [`EcGroup`]

[`EcGroup`]: struct.EcGroup.html

```rust
pub struct EcGroupRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn components_gfp(self: &Self, p: &mut BigNumRef, a: &mut BigNumRef, b: &mut BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the components of a curve over a prime field in the provided `BigNum`s.

- ```rust
  pub fn components_gf2m(self: &Self, p: &mut BigNumRef, a: &mut BigNumRef, b: &mut BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the components of a curve over a binary field in the provided `BigNum`s.

- ```rust
  pub fn cofactor(self: &Self, cofactor: &mut BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the cofactor of the group in the provided `BigNum`.

- ```rust
  pub fn degree(self: &Self) -> u32 { /* ... */ }
  ```
   Returns the degree of the curve.

- ```rust
  pub fn order_bits(self: &Self) -> u32 { /* ... */ }
  ```
   Returns the number of bits in the group order.

- ```rust
  pub fn generator(self: &Self) -> &EcPointRef { /* ... */ }
  ```
   Returns the generator for the given curve as an [`EcPoint`].

- ```rust
  pub fn set_generator(self: &mut Self, generator: EcPoint, order: BigNum, cofactor: BigNum) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the generator point for the given curve

- ```rust
  pub fn order(self: &Self, order: &mut BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places the order of the curve in the provided `BigNum`.

- ```rust
  pub fn set_asn1_flag(self: &mut Self, flag: Asn1Flag) { /* ... */ }
  ```
   Sets the flag determining if the group corresponds to a named curve or must be explicitly

- ```rust
  pub fn asn1_flag(self: &Self) -> Asn1Flag { /* ... */ }
  ```
   Gets the flag determining if the group corresponds to a named curve.

- ```rust
  pub fn curve_name(self: &Self) -> Option<Nid> { /* ... */ }
  ```
   Returns the name of the curve, if a name is associated.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &EcGroupRef { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **RefUnwindSafe**
- **ForeignTypeRef**
- **Freeze**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &EcGroupRef { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `EcPoint`

Represents a point on the curve

```rust
pub struct EcPoint(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(group: &EcGroupRef) -> Result<EcPoint, ErrorStack> { /* ... */ }
  ```
   Creates a new point on the specified curve.

- ```rust
  pub fn from_bytes(group: &EcGroupRef, buf: &[u8], ctx: &mut BigNumContextRef) -> Result<EcPoint, ErrorStack> { /* ... */ }
  ```
   Creates point from a binary representation

- ```rust
  pub fn from_hex_str(group: &EcGroupRef, s: &str, ctx: &mut BigNumContextRef) -> Result<EcPoint, ErrorStack> { /* ... */ }
  ```
   Creates point from a hexadecimal string representation

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &EcPointRef { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &EcPointRef { /* ... */ }
    ```

- **UnwindSafe**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &EcPointRef { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Receiver**
- **RefUnwindSafe**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut EcPointRef { /* ... */ }
    ```

- **Freeze**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::EC_POINT) -> EcPoint { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::EC_POINT { /* ... */ }
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

- **Sync**
#### Struct `EcPointRef`

A reference a borrowed [`EcPoint`].

```rust
pub struct EcPointRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn add(self: &mut Self, group: &EcGroupRef, a: &EcPointRef, b: &EcPointRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Computes `a + b`, storing the result in `self`.

- ```rust
  pub fn mul(self: &mut Self, group: &EcGroupRef, q: &EcPointRef, m: &BigNumRef, ctx: &BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Computes `q * m`, storing the result in `self`.

- ```rust
  pub fn mul_generator(self: &mut Self, group: &EcGroupRef, n: &BigNumRef, ctx: &BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Computes `generator * n`, storing the result in `self`.

- ```rust
  pub fn mul_full(self: &mut Self, group: &EcGroupRef, n: &BigNumRef, q: &EcPointRef, m: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Computes `generator * n + q * m`, storing the result in `self`.

- ```rust
  pub fn invert(self: &mut Self, group: &EcGroupRef, ctx: &BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Inverts `self`.

- ```rust
  pub fn to_bytes(self: &Self, group: &EcGroupRef, form: PointConversionForm, ctx: &mut BigNumContextRef) -> Result<Vec<u8>, ErrorStack> { /* ... */ }
  ```
   Serializes the point to a binary representation.

- ```rust
  pub fn to_hex_str(self: &Self, group: &EcGroupRef, form: PointConversionForm, ctx: &mut BigNumContextRef) -> Result<OpensslString, ErrorStack> { /* ... */ }
  ```
   Serializes the point to a hexadecimal string representation.

- ```rust
  pub fn to_owned(self: &Self, group: &EcGroupRef) -> Result<EcPoint, ErrorStack> { /* ... */ }
  ```
   Creates a new point on the specified curve with the same value.

- ```rust
  pub fn eq(self: &Self, group: &EcGroupRef, other: &EcPointRef, ctx: &mut BigNumContextRef) -> Result<bool, ErrorStack> { /* ... */ }
  ```
   Determines if this point is equal to another.

- ```rust
  pub fn affine_coordinates(self: &Self, group: &EcGroupRef, x: &mut BigNumRef, y: &mut BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places affine coordinates of a curve over a prime field in the provided

- ```rust
  pub fn affine_coordinates_gfp(self: &Self, group: &EcGroupRef, x: &mut BigNumRef, y: &mut BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places affine coordinates of a curve over a prime field in the provided

- ```rust
  pub fn set_affine_coordinates_gfp(self: &mut Self, group: &EcGroupRef, x: &BigNumRef, y: &BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets affine coordinates of a curve over a prime field using the provided

- ```rust
  pub fn affine_coordinates_gf2m(self: &Self, group: &EcGroupRef, x: &mut BigNumRef, y: &mut BigNumRef, ctx: &mut BigNumContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Places affine coordinates of a curve over a binary field in the provided

- ```rust
  pub fn is_infinity(self: &Self, group: &EcGroupRef) -> bool { /* ... */ }
  ```
   Checks if point is infinity

- ```rust
  pub fn is_on_curve(self: &Self, group: &EcGroupRef, ctx: &mut BigNumContextRef) -> Result<bool, ErrorStack> { /* ... */ }
  ```
   Checks if point is on a given curve

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **ForeignTypeRef**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &EcPointRef { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &EcPointRef { /* ... */ }
    ```

#### Struct `EcKey`

Public and optional private key on the given curve.

```rust
pub struct EcKey<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_curve_name(nid: Nid) -> Result<EcKey<Params>, ErrorStack> { /* ... */ }
  ```
   Constructs an `EcKey` corresponding to a known curve.

- ```rust
  pub fn from_group(group: &EcGroupRef) -> Result<EcKey<Params>, ErrorStack> { /* ... */ }
  ```
   Constructs an `EcKey` corresponding to a curve.

- ```rust
  pub fn from_public_key(group: &EcGroupRef, public_key: &EcPointRef) -> Result<EcKey<Public>, ErrorStack> { /* ... */ }
  ```
   Constructs an `EcKey` from the specified group with the associated [`EcPoint`]: `public_key`.

- ```rust
  pub fn from_public_key_affine_coordinates(group: &EcGroupRef, x: &BigNumRef, y: &BigNumRef) -> Result<EcKey<Public>, ErrorStack> { /* ... */ }
  ```
   Constructs a public key from its affine coordinates.

- ```rust
  pub fn public_key_from_pem(pem: &[u8]) -> Result<EcKey<Public>, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a PEM-encoded SubjectPublicKeyInfo structure containing a EC key.

- ```rust
  pub fn public_key_from_der(der: &[u8]) -> Result<EcKey<Public>, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a DER-encoded SubjectPublicKeyInfo structure containing a EC key.

- ```rust
  pub fn generate(group: &EcGroupRef) -> Result<EcKey<Private>, ErrorStack> { /* ... */ }
  ```
   Generates a new public/private key pair on the specified curve.

- ```rust
  pub fn from_private_components(group: &EcGroupRef, private_number: &BigNumRef, public_key: &EcPointRef) -> Result<EcKey<Private>, ErrorStack> { /* ... */ }
  ```
   Constructs an public/private key pair given a curve, a private key and a public key point.

- ```rust
  pub fn private_key_from_pem(pem: &[u8]) -> Result<EcKey<Private>, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a private key from a PEM-encoded ECPrivateKey structure.

- ```rust
  pub fn private_key_from_pem_passphrase(pem: &[u8], passphrase: &[u8]) -> Result<EcKey<Private>, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a private key from a PEM-encoded encrypted ECPrivateKey structure.

- ```rust
  pub fn private_key_from_pem_callback<F>(pem: &[u8], callback: F) -> Result<EcKey<Private>, crate::error::ErrorStack>
where
    F: FnOnce(&mut [u8]) -> Result<usize, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a private key from a PEM-encoded encrypted ECPrivateKey structure.

- ```rust
  pub fn private_key_from_der(der: &[u8]) -> Result<EcKey<Private>, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a DER-encoded elliptic curve private key structure.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(ec_key: EcKey<T>) -> Result<PKey<T>, ErrorStack> { /* ... */ }
    ```

  - ```rust
    fn try_from(pkey: PKey<T>) -> Result<EcKey<T>, ErrorStack> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &EcKeyRef<T> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &EcKeyRef<T> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &EcKeyRef<T> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Receiver**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> EcKey<T> { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::EC_KEY) -> EcKey<T> { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::EC_KEY { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut EcKeyRef<T> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `EcKeyRef`

A reference to an [`EcKey`].

```rust
pub struct EcKeyRef<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn private_key_to_pem(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the private key to a PEM-encoded ECPrivateKey structure.

- ```rust
  pub fn private_key_to_pem_passphrase(self: &Self, cipher: crate::symm::Cipher, passphrase: &[u8]) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the private key to a PEM-encoded encrypted ECPrivateKey structure.

- ```rust
  pub fn private_key_to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the private key into a DER-encoded ECPrivateKey structure.

- ```rust
  pub fn private_key(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns the private key value.

- ```rust
  pub fn public_key(self: &Self) -> &EcPointRef { /* ... */ }
  ```
   Returns the public key.

- ```rust
  pub fn public_key_to_pem(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the public key into a PEM-encoded SubjectPublicKeyInfo structure.

- ```rust
  pub fn public_key_to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the public key into a DER-encoded SubjectPublicKeyInfo structure.

- ```rust
  pub fn group(self: &Self) -> &EcGroupRef { /* ... */ }
  ```
   Returns the key's group.

- ```rust
  pub fn check_key(self: &Self) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Checks the key for validity.

###### Trait Implementations

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

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &EcKeyRef<T> { /* ... */ }
    ```

- **RefUnwindSafe**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &EcKeyRef<T> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> EcKey<T> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ForeignTypeRef**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
## Module `ecdsa`

Low level Elliptic Curve Digital Signature Algorithm (ECDSA) functions.

```rust
pub mod ecdsa { /* ... */ }
```

### Types

#### Struct `EcdsaSig`

A low level interface to ECDSA.

```rust
pub struct EcdsaSig(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn sign<T>(data: &[u8], eckey: &EcKeyRef<T>) -> Result<EcdsaSig, ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
   Computes a digital signature of the hash value `data` using the private EC key eckey.

- ```rust
  pub fn from_private_components(r: BigNum, s: BigNum) -> Result<EcdsaSig, ErrorStack> { /* ... */ }
  ```
   Returns a new `EcdsaSig` by setting the `r` and `s` values associated with an ECDSA signature.

- ```rust
  pub fn from_der(der: &[u8]) -> Result<EcdsaSig, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a DER-encoded ECDSA signature.

###### Trait Implementations

- **Unpin**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::ECDSA_SIG) -> EcdsaSig { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::ECDSA_SIG { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut EcdsaSigRef { /* ... */ }
    ```

- **Receiver**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &EcdsaSigRef { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &EcdsaSigRef { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &EcdsaSigRef { /* ... */ }
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
#### Struct `EcdsaSigRef`

A reference to an [`EcdsaSig`].

```rust
pub struct EcdsaSigRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the ECDSA signature into a DER-encoded ECDSASignature structure.

- ```rust
  pub fn verify<T>(self: &Self, data: &[u8], eckey: &EcKeyRef<T>) -> Result<bool, ErrorStack>
where
    T: HasPublic { /* ... */ }
  ```
   Verifies if the signature is a valid ECDSA signature using the given public key.

- ```rust
  pub fn r(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns internal component: `r` of an `EcdsaSig`. (See X9.62 or FIPS 186-2)

- ```rust
  pub fn s(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns internal components: `s` of an `EcdsaSig`. (See X9.62 or FIPS 186-2)

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **ForeignTypeRef**
- **Unpin**
- **Freeze**
- **RefUnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &EcdsaSigRef { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &EcdsaSigRef { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `encrypt`

Message encryption.

The [`Encrypter`] allows for encryption of data given a public key. The [`Decrypter`] can be
used with the corresponding private key to decrypt the data.

# Examples

Encrypt and decrypt data given an RSA keypair:

```rust
use openssl::encrypt::{Encrypter, Decrypter};
use openssl::rsa::{Rsa, Padding};
use openssl::pkey::PKey;

// Generate a keypair
let keypair = Rsa::generate(2048).unwrap();
let keypair = PKey::from_rsa(keypair).unwrap();

let data = b"hello, world!";

// Encrypt the data with RSA PKCS1
let mut encrypter = Encrypter::new(&keypair).unwrap();
encrypter.set_rsa_padding(Padding::PKCS1).unwrap();
// Create an output buffer
let buffer_len = encrypter.encrypt_len(data).unwrap();
let mut encrypted = vec![0; buffer_len];
// Encrypt and truncate the buffer
let encrypted_len = encrypter.encrypt(data, &mut encrypted).unwrap();
encrypted.truncate(encrypted_len);

// Decrypt the data
let mut decrypter = Decrypter::new(&keypair).unwrap();
decrypter.set_rsa_padding(Padding::PKCS1).unwrap();
// Create an output buffer
let buffer_len = decrypter.decrypt_len(&encrypted).unwrap();
let mut decrypted = vec![0; buffer_len];
// Encrypt and truncate the buffer
let decrypted_len = decrypter.decrypt(&encrypted, &mut decrypted).unwrap();
decrypted.truncate(decrypted_len);
assert_eq!(&*decrypted, data);
```

```rust
pub mod encrypt { /* ... */ }
```

### Types

#### Struct `Encrypter`

A type which encrypts data.

```rust
pub struct Encrypter<''a> {
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
  pub fn new<T>(pkey: &''a PKeyRef<T>) -> Result<Encrypter<''a>, ErrorStack>
where
    T: HasPublic { /* ... */ }
  ```
  Creates a new `Encrypter`.

- ```rust
  pub fn rsa_padding(self: &Self) -> Result<Padding, ErrorStack> { /* ... */ }
  ```
  Returns the RSA padding mode in use.

- ```rust
  pub fn set_rsa_padding(self: &mut Self, padding: Padding) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Sets the RSA padding mode.

- ```rust
  pub fn set_rsa_mgf1_md(self: &mut Self, md: MessageDigest) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Sets the RSA MGF1 algorithm.

- ```rust
  pub fn set_rsa_oaep_md(self: &mut Self, md: MessageDigest) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Sets the RSA OAEP algorithm.

- ```rust
  pub fn set_rsa_oaep_label(self: &mut Self, label: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Sets the RSA OAEP label.

- ```rust
  pub fn encrypt(self: &Self, from: &[u8], to: &mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Performs public key encryption.

- ```rust
  pub fn encrypt_len(self: &Self, from: &[u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Gets the size of the buffer needed to encrypt the input data.

###### Trait Implementations

- **Send**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
#### Struct `Decrypter`

A type which decrypts data.

```rust
pub struct Decrypter<''a> {
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
  pub fn new<T>(pkey: &''a PKeyRef<T>) -> Result<Decrypter<''a>, ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
  Creates a new `Decrypter`.

- ```rust
  pub fn rsa_padding(self: &Self) -> Result<Padding, ErrorStack> { /* ... */ }
  ```
  Returns the RSA padding mode in use.

- ```rust
  pub fn set_rsa_padding(self: &mut Self, padding: Padding) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Sets the RSA padding mode.

- ```rust
  pub fn set_rsa_mgf1_md(self: &mut Self, md: MessageDigest) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Sets the RSA MGF1 algorithm.

- ```rust
  pub fn set_rsa_oaep_md(self: &mut Self, md: MessageDigest) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Sets the RSA OAEP algorithm.

- ```rust
  pub fn set_rsa_oaep_label(self: &mut Self, label: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Sets the RSA OAEP label.

- ```rust
  pub fn decrypt(self: &Self, from: &[u8], to: &mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Performs public key decryption.

- ```rust
  pub fn decrypt_len(self: &Self, from: &[u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Gets the size of the buffer needed to decrypt the input data.

###### Trait Implementations

- **RefUnwindSafe**
- **Unpin**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `envelope`

**Attributes:**

- `#[<cfg>(not(any(boringssl, awslc)))]`

Envelope encryption.

# Example

```rust
use openssl::rsa::Rsa;
use openssl::envelope::Seal;
use openssl::pkey::PKey;
use openssl::symm::Cipher;

let rsa = Rsa::generate(2048).unwrap();
let key = PKey::from_rsa(rsa).unwrap();

let cipher = Cipher::aes_256_cbc();
let mut seal = Seal::new(cipher, &[key]).unwrap();

let secret = b"My secret message";
let mut encrypted = vec![0; secret.len() + cipher.block_size()];

let mut enc_len = seal.update(secret, &mut encrypted).unwrap();
enc_len += seal.finalize(&mut encrypted[enc_len..]).unwrap();
encrypted.truncate(enc_len);
```

```rust
pub mod envelope { /* ... */ }
```

### Types

#### Struct `Seal`

Represents an EVP_Seal context.

```rust
pub struct Seal {
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
  pub fn new<T>(cipher: Cipher, pub_keys: &[PKey<T>]) -> Result<Seal, ErrorStack>
where
    T: HasPublic { /* ... */ }
  ```
  Creates a new `Seal`.

- ```rust
  pub fn iv(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
  Returns the initialization vector, if the cipher uses one.

- ```rust
  pub fn encrypted_keys(self: &Self) -> &[Vec<u8>] { /* ... */ }
  ```
  Returns the encrypted keys.

- ```rust
  pub fn update(self: &mut Self, input: &[u8], output: &mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Feeds data from `input` through the cipher, writing encrypted bytes into `output`.

- ```rust
  pub fn finalize(self: &mut Self, output: &mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Finishes the encryption process, writing any remaining data to `output`.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Open`

Represents an EVP_Open context.

```rust
pub struct Open {
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
  pub fn new<T>(cipher: Cipher, priv_key: &PKeyRef<T>, iv: Option<&[u8]>, encrypted_key: &[u8]) -> Result<Open, ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
  Creates a new `Open`.

- ```rust
  pub fn update(self: &mut Self, input: &[u8], output: &mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Feeds data from `input` through the cipher, writing decrypted bytes into `output`.

- ```rust
  pub fn finalize(self: &mut Self, output: &mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Finishes the decryption process, writing any remaining data to `output`.

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `error`

Errors returned by OpenSSL library.

OpenSSL errors are stored in an `ErrorStack`.  Most methods in the crate
returns a `Result<T, ErrorStack>` type.

# Examples

```
use openssl::error::ErrorStack;
use openssl::bn::BigNum;

let an_error = BigNum::from_dec_str("Cannot parse letters");
match an_error {
    Ok(_)  => (),
    Err(e) => println!("Parsing Error: {:?}", e),
}
```

```rust
pub mod error { /* ... */ }
```

### Types

#### Struct `ErrorStack`

Collection of [`Error`]s from OpenSSL.

[`Error`]: struct.Error.html

```rust
pub struct ErrorStack(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn get() -> ErrorStack { /* ... */ }
  ```
  Returns the contents of the OpenSSL error stack.

- ```rust
  pub fn put(self: &Self) { /* ... */ }
  ```
  Pushes the errors back onto the OpenSSL error stack.

- ```rust
  pub fn errors(self: &Self) -> &[Error] { /* ... */ }
  ```
  Returns the errors in the stack.

###### Trait Implementations

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Error**
- **UnwindSafe**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(e: ErrorStack) -> io::Error { /* ... */ }
    ```

  - ```rust
    fn from(_: ErrorStack) -> fmt::Error { /* ... */ }
    ```

  - ```rust
    fn from(e: ErrorStack) -> Error { /* ... */ }
    ```

  - ```rust
    fn from(e: ErrorStack) -> HandshakeError<S> { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ErrorStack { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Error`

An error reported from OpenSSL.

```rust
pub struct Error {
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
  pub fn get() -> Option<Error> { /* ... */ }
  ```
  Returns the first error on the OpenSSL error stack.

- ```rust
  pub fn put(self: &Self) { /* ... */ }
  ```
  Pushes the error back onto the OpenSSL error stack.

- ```rust
  pub fn code(self: &Self) -> libc::c_ulong { /* ... */ }
  ```
  Returns the raw OpenSSL error code for this error.

- ```rust
  pub fn library(self: &Self) -> Option<&''static str> { /* ... */ }
  ```
  Returns the name of the library reporting the error, if available.

- ```rust
  pub fn library_code(self: &Self) -> libc::c_int { /* ... */ }
  ```
  Returns the raw OpenSSL error constant for the library reporting the

- ```rust
  pub fn function(self: &Self) -> Option<&''_ str> { /* ... */ }
  ```
  Returns the name of the function reporting the error.

- ```rust
  pub fn reason(self: &Self) -> Option<&''static str> { /* ... */ }
  ```
  Returns the reason for the error.

- ```rust
  pub fn reason_code(self: &Self) -> libc::c_int { /* ... */ }
  ```
  Returns the raw OpenSSL error constant for the reason for the error.

- ```rust
  pub fn file(self: &Self) -> &''_ str { /* ... */ }
  ```
  Returns the name of the source file which encountered the error.

- ```rust
  pub fn line(self: &Self) -> u32 { /* ... */ }
  ```
  Returns the line in the source file which encountered the error.

- ```rust
  pub fn data(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Returns additional data describing the error.

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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Sync**
- **Error**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

## Module `ex_data`

```rust
pub mod ex_data { /* ... */ }
```

### Types

#### Struct `Index`

A slot in a type's "extra data" structure.

It is parameterized over the type containing the extra data as well as the
type of the data in the slot.

```rust
pub struct Index<T, U>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub unsafe fn from_raw(idx: c_int) -> Index<T, U> { /* ... */ }
  ```
  Creates an `Index` from a raw integer index.

- ```rust
  pub fn as_raw(self: &Self) -> c_int { /* ... */ }
  ```

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Index<T, U> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
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

## Module `hash`

Message digest (hash) computation support.

# Examples

Calculate a hash in one go:

```
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use openssl::hash::{hash, MessageDigest};

let data = b"\x42\xF4\x97\xE0";
let spec = b"\x7c\x43\x0f\x17\x8a\xef\xdf\x14\x87\xfe\xe7\x14\x4e\x96\x41\xe2";
let res = hash(MessageDigest::md5(), data)?;
assert_eq!(&*res, spec);
# Ok(()) }
```

Supply the input in chunks:

```
use openssl::hash::{Hasher, MessageDigest};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut hasher = Hasher::new(MessageDigest::sha256())?;
hasher.update(b"test")?;
hasher.update(b"this")?;
let digest: &[u8] = &hasher.finish()?;

let expected = hex::decode("9740e652ab5b4acd997a7cca13d6696702ccb2d441cca59fc6e285127f28cfe6")?;
assert_eq!(digest, expected);
# Ok(()) }
```

```rust
pub mod hash { /* ... */ }
```

### Types

#### Struct `MessageDigest`

A message digest algorithm.

```rust
pub struct MessageDigest(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub unsafe fn from_ptr(x: *const ffi::EVP_MD) -> Self { /* ... */ }
  ```
  Creates a `MessageDigest` from a raw OpenSSL pointer.

- ```rust
  pub fn from_nid(type_: Nid) -> Option<MessageDigest> { /* ... */ }
  ```
   Returns the `MessageDigest` corresponding to an `Nid`.

- ```rust
  pub fn from_name(name: &str) -> Option<MessageDigest> { /* ... */ }
  ```
   Returns the `MessageDigest` corresponding to an algorithm name.

- ```rust
  pub fn null() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn md5() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn sha1() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn sha224() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn sha256() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn sha384() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn sha512() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn sha3_224() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn sha3_256() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn sha3_384() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn sha3_512() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn shake_128() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn shake_256() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn ripemd160() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn sm3() -> MessageDigest { /* ... */ }
  ```

- ```rust
  pub fn as_ptr(self: &Self) -> *const ffi::EVP_MD { /* ... */ }
  ```

- ```rust
  pub fn block_size(self: &Self) -> usize { /* ... */ }
  ```
  The block size of the digest in bytes.

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
  The size of the digest in bytes.

- ```rust
  pub fn type_(self: &Self) -> Nid { /* ... */ }
  ```
  The name of the digest.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MessageDigest { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MessageDigest) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Send**
- **Sync**
#### Struct `Hasher`

Provides message digest (hash) computation.

# Examples

```
use openssl::hash::{Hasher, MessageDigest};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let data = [b"\x42\xF4", b"\x97\xE0"];
let spec = b"\x7c\x43\x0f\x17\x8a\xef\xdf\x14\x87\xfe\xe7\x14\x4e\x96\x41\xe2";
let mut h = Hasher::new(MessageDigest::md5())?;
h.update(data[0])?;
h.update(data[1])?;
let res = h.finish()?;
assert_eq!(&*res, spec);
# Ok(()) }
```

# Warning

Don't actually use MD5 and SHA-1 hashes, they're not secure anymore.

Don't ever hash passwords, use the functions in the `pkcs5` module or bcrypt/scrypt instead.

For extendable output functions (XOFs, i.e. SHAKE128/SHAKE256),
you must use [`Hasher::finish_xof`] instead of [`Hasher::finish`]
and provide a `buf` to store the hash. The hash will be as long as
the `buf`.

```rust
pub struct Hasher {
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
  pub fn new(ty: MessageDigest) -> Result<Hasher, ErrorStack> { /* ... */ }
  ```
  Creates a new `Hasher` with the specified hash type.

- ```rust
  pub fn update(self: &mut Self, data: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Feeds data into the hasher.

- ```rust
  pub fn finish(self: &mut Self) -> Result<DigestBytes, ErrorStack> { /* ... */ }
  ```
  Returns the hash of the data written and resets the non-XOF hasher.

- ```rust
  pub fn finish_xof(self: &mut Self, buf: &mut [u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Writes the hash of the data into the supplied buf and resets the XOF hasher.

###### Trait Implementations

- **Freeze**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Hasher { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

#### Struct `DigestBytes`

The resulting bytes of a digest.

This type derefs to a byte slice - it exists to avoid allocating memory to
store the digest data.

```rust
pub struct DigestBytes {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Receiver**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DigestBytes { /* ... */ }
    ```

### Functions

#### Function `hash`

Computes the hash of the `data` with the non-XOF hasher `t`.

# Examples

```
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use openssl::hash::{hash, MessageDigest};

let data = b"\x42\xF4\x97\xE0";
let spec = b"\x7c\x43\x0f\x17\x8a\xef\xdf\x14\x87\xfe\xe7\x14\x4e\x96\x41\xe2";
let res = hash(MessageDigest::md5(), data)?;
assert_eq!(&*res, spec);
# Ok(()) }
```

```rust
pub fn hash(t: MessageDigest, data: &[u8]) -> Result<DigestBytes, crate::error::ErrorStack> { /* ... */ }
```

#### Function `hash_xof`

**Attributes:**

- `#[<cfg>(any(ossl111, awslc))]`

Computes the hash of the `data` with the XOF hasher `t` and stores it in `buf`.

# Examples

```
use openssl::hash::{hash_xof, MessageDigest};

let data = b"\x41\x6c\x6c\x20\x79\x6f\x75\x72\x20\x62\x61\x73\x65\x20\x61\x72\x65\x20\x62\x65\x6c\x6f\x6e\x67\x20\x74\x6f\x20\x75\x73";
let spec = b"\x49\xd0\x69\x7f\xf5\x08\x11\x1d\x8b\x84\xf1\x5e\x46\xda\xf1\x35";
let mut buf = vec![0; 16];
hash_xof(MessageDigest::shake_128(), data, buf.as_mut_slice()).unwrap();
assert_eq!(buf, spec);
```


```rust
pub fn hash_xof(t: MessageDigest, data: &[u8], buf: &mut [u8]) -> Result<(), crate::error::ErrorStack> { /* ... */ }
```

## Module `kdf`

```rust
pub mod kdf { /* ... */ }
```

## Module `lib_ctx`

**Attributes:**

- `#[<cfg>(ossl300)]`

```rust
pub mod lib_ctx { /* ... */ }
```

### Types

#### Struct `LibCtx`

```rust
pub struct LibCtx(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Result<Self, ErrorStack> { /* ... */ }
  ```

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &LibCtxRef { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &LibCtxRef { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::OSSL_LIB_CTX) -> LibCtx { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::OSSL_LIB_CTX { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Receiver**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut LibCtxRef { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &LibCtxRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

#### Struct `LibCtxRef`

```rust
pub struct LibCtxRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &LibCtxRef { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ForeignTypeRef**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Send**
- **RefUnwindSafe**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &LibCtxRef { /* ... */ }
    ```

- **Freeze**
- **Sync**
## Module `md`

Message digest algorithms.

```rust
pub mod md { /* ... */ }
```

### Types

#### Struct `Md`

A message digest algorithm.

```rust
pub struct Md(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_nid(type_: Nid) -> Option<&''static MdRef> { /* ... */ }
  ```
   Returns the `Md` corresponding to an [`Nid`].

- ```rust
  pub fn fetch(ctx: Option<&LibCtxRef>, algorithm: &str, properties: Option<&str>) -> Result<Self, ErrorStack> { /* ... */ }
  ```
   Fetches an `Md` object corresponding to the specified algorithm name and properties.

- ```rust
  pub fn null() -> &''static MdRef { /* ... */ }
  ```

- ```rust
  pub fn md5() -> &''static MdRef { /* ... */ }
  ```

- ```rust
  pub fn sha1() -> &''static MdRef { /* ... */ }
  ```

- ```rust
  pub fn sha224() -> &''static MdRef { /* ... */ }
  ```

- ```rust
  pub fn sha256() -> &''static MdRef { /* ... */ }
  ```

- ```rust
  pub fn sha384() -> &''static MdRef { /* ... */ }
  ```

- ```rust
  pub fn sha512() -> &''static MdRef { /* ... */ }
  ```

- ```rust
  pub fn sha3_224() -> &''static MdRef { /* ... */ }
  ```

- ```rust
  pub fn sha3_256() -> &''static MdRef { /* ... */ }
  ```

- ```rust
  pub fn sha3_384() -> &''static MdRef { /* ... */ }
  ```

- ```rust
  pub fn sha3_512() -> &''static MdRef { /* ... */ }
  ```

- ```rust
  pub fn shake128() -> &''static MdRef { /* ... */ }
  ```

- ```rust
  pub fn shake256() -> &''static MdRef { /* ... */ }
  ```

- ```rust
  pub fn ripemd160() -> &''static MdRef { /* ... */ }
  ```

- ```rust
  pub fn sm3() -> &''static MdRef { /* ... */ }
  ```

###### Trait Implementations

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut <Self as >::CType) -> Self { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut <Self as >::CType { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Receiver**
- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
#### Struct `MdRef`

A reference to an [`Md`].

```rust
pub struct MdRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn block_size(self: &Self) -> usize { /* ... */ }
  ```
   Returns the block size of the digest in bytes.

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
   Returns the size of the digest in bytes.

- ```rust
  pub fn type_(self: &Self) -> Nid { /* ... */ }
  ```
   Returns the [`Nid`] of the digest.

###### Trait Implementations

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ForeignTypeRef**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `md_ctx`

**Attributes:**

- `#[<cfg_attr>(not(any(boringssl, awslc)), doc =
r#"\
Compute and verify an HMAC-SHA256

```
use openssl::md::Md;
use openssl::md_ctx::MdCtx;
use openssl::memcmp;
use openssl::pkey::PKey;

// Create a key with the HMAC secret.
let key = PKey::hmac(b"my secret").unwrap();

let text = b"Some Crypto Text";

// Compute the HMAC.
let mut ctx = MdCtx::new().unwrap();
ctx.digest_sign_init(Some(Md::sha256()), &key).unwrap();
ctx.digest_sign_update(text).unwrap();
let mut hmac = vec![];
ctx.digest_sign_final_to_vec(&mut hmac).unwrap();

// Verify the HMAC. You can't use MdCtx to do this; instead use a constant time equality check.
# let target = hmac.clone();
let valid = memcmp::eq(&hmac, &target);
assert!(valid);
```"#)]`

The message digest context.

# Examples

Compute the SHA256 checksum of data

```
use openssl::md::Md;
use openssl::md_ctx::MdCtx;

let mut ctx = MdCtx::new().unwrap();
ctx.digest_init(Md::sha256()).unwrap();
ctx.digest_update(b"Some Crypto Text").unwrap();
let mut digest = [0; 32];
ctx.digest_final(&mut digest).unwrap();

assert_eq!(
    digest,
    *b"\x60\x78\x56\x38\x8a\xca\x5c\x51\x83\xc4\xd1\x4d\xc8\xf9\xcc\xf2\
       \xa5\x21\xb3\x10\x93\x72\xfa\xd6\x7c\x55\xf5\xc9\xe3\xd1\x83\x19",
);
```

Sign and verify data with RSA and SHA256

```
use openssl::md::Md;
use openssl::md_ctx::MdCtx;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;

// Generate a random RSA key.
let key = Rsa::generate(4096).unwrap();
let key = PKey::from_rsa(key).unwrap();

let text = b"Some Crypto Text";

// Create the signature.
let mut ctx = MdCtx::new().unwrap();
ctx.digest_sign_init(Some(Md::sha256()), &key).unwrap();
ctx.digest_sign_update(text).unwrap();
let mut signature = vec![];
ctx.digest_sign_final_to_vec(&mut signature).unwrap();

// Verify the signature.
let mut ctx = MdCtx::new().unwrap();
ctx.digest_verify_init(Some(Md::sha256()), &key).unwrap();
ctx.digest_verify_update(text).unwrap();
let valid = ctx.digest_verify_final(&signature).unwrap();
assert!(valid);
```

\
Compute and verify an HMAC-SHA256

```
use openssl::md::Md;
use openssl::md_ctx::MdCtx;
use openssl::memcmp;
use openssl::pkey::PKey;

// Create a key with the HMAC secret.
let key = PKey::hmac(b"my secret").unwrap();

let text = b"Some Crypto Text";

// Compute the HMAC.
let mut ctx = MdCtx::new().unwrap();
ctx.digest_sign_init(Some(Md::sha256()), &key).unwrap();
ctx.digest_sign_update(text).unwrap();
let mut hmac = vec![];
ctx.digest_sign_final_to_vec(&mut hmac).unwrap();

// Verify the HMAC. You can't use MdCtx to do this; instead use a constant time equality check.
# let target = hmac.clone();
let valid = memcmp::eq(&hmac, &target);
assert!(valid);
```

```rust
pub mod md_ctx { /* ... */ }
```

### Types

#### Struct `MdCtx`

```rust
pub struct MdCtx(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Result<Self, ErrorStack> { /* ... */ }
  ```
   Creates a new context.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &MdCtxRef { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &MdCtxRef { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut MdCtxRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Unpin**
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

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::EVP_MD_CTX) -> MdCtx { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::EVP_MD_CTX { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &MdCtxRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Receiver**
#### Struct `MdCtxRef`

A reference to an [`MdCtx`].

```rust
pub struct MdCtxRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn digest_init(self: &mut Self, digest: &MdRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Initializes the context to compute the digest of data.

- ```rust
  pub fn digest_sign_init<''a, T>(self: &''a mut Self, digest: Option<&MdRef>, pkey: &PKeyRef<T>) -> Result<&''a mut PkeyCtxRef<T>, ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
   Initializes the context to compute the signature of data.

- ```rust
  pub fn digest_verify_init<''a, T>(self: &''a mut Self, digest: Option<&MdRef>, pkey: &PKeyRef<T>) -> Result<&''a mut PkeyCtxRef<T>, ErrorStack>
where
    T: HasPublic { /* ... */ }
  ```
   Initializes the context to verify the signature of data.

- ```rust
  pub fn digest_update(self: &mut Self, data: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Updates the context with more data.

- ```rust
  pub fn digest_sign_update(self: &mut Self, data: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Updates the context with more data.

- ```rust
  pub fn digest_verify_update(self: &mut Self, data: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Updates the context with more data.

- ```rust
  pub fn digest_final(self: &mut Self, out: &mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Copies the computed digest into the buffer, returning the number of bytes written.

- ```rust
  pub fn digest_final_xof(self: &mut Self, out: &mut [u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Copies the computed digest into the buffer.

- ```rust
  pub fn digest_sign_final(self: &mut Self, out: Option<&mut [u8]>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Signs the computed digest.

- ```rust
  pub fn digest_sign_final_to_vec(self: &mut Self, out: &mut Vec<u8>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Like [`Self::digest_sign_final`] but appends the signature to a [`Vec`].

- ```rust
  pub fn digest_verify_final(self: &mut Self, signature: &[u8]) -> Result<bool, ErrorStack> { /* ... */ }
  ```
   Verifies the provided signature.

- ```rust
  pub fn digest_sign(self: &mut Self, from: &[u8], to: Option<&mut [u8]>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Computes the signature of the data in `from`.

- ```rust
  pub fn digest_sign_to_vec(self: &mut Self, from: &[u8], to: &mut Vec<u8>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Like [`Self::digest_sign`] but appends the signature to a [`Vec`].

- ```rust
  pub fn digest_verify(self: &mut Self, data: &[u8], signature: &[u8]) -> Result<bool, ErrorStack> { /* ... */ }
  ```
   Verifies the signature of the data in `data`.

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
   Returns the size of the message digest, i.e. the size of the hash

- ```rust
  pub fn reset(self: &mut Self) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Resets the underlying EVP_MD_CTX instance

###### Trait Implementations

- **ForeignTypeRef**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &MdCtxRef { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &MdCtxRef { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
## Module `memcmp`

Utilities to safely compare cryptographic values.

Extra care must be taken when comparing values in
cryptographic code. If done incorrectly, it can lead
to a [timing attack](https://en.wikipedia.org/wiki/Timing_attack).
By analyzing the time taken to execute parts of a cryptographic
algorithm, and attacker can attempt to compromise the
cryptosystem.

The utilities in this module are designed to be resistant
to this type of attack.

# Examples

To perform a constant-time comparison of two arrays of the same length but different
values:

```
use openssl::memcmp::eq;

// We want to compare `a` to `b` and `c`, without giving
// away through timing analysis that `c` is more similar to `a`
// than `b`.
let a = [0, 0, 0];
let b = [1, 1, 1];
let c = [0, 0, 1];

// These statements will execute in the same amount of time.
assert!(!eq(&a, &b));
assert!(!eq(&a, &c));
```

```rust
pub mod memcmp { /* ... */ }
```

### Functions

#### Function `eq`

**Attributes:**

- `#[doc(alias = "CRYPTO_memcmp")]`

 Returns `true` iff `a` and `b` contain the same bytes.

 This operation takes an amount of time dependent on the length of the two
 arrays given, but is independent of the contents of a and b.

 # Panics

 This function will panic the current task if `a` and `b` do not have the same
 length.

 # Examples

 To perform a constant-time comparison of two arrays of the same length but different
 values:

 ```
 use openssl::memcmp::eq;

 // We want to compare `a` to `b` and `c`, without giving
 // away through timing analysis that `c` is more similar to `a`
 // than `b`.
 let a = [0, 0, 0];
 let b = [1, 1, 1];
 let c = [0, 0, 1];

 // These statements will execute in the same amount of time.
 assert!(!eq(&a, &b));
 assert!(!eq(&a, &c));
 ```

This corresponds to [`CRYPTO_memcmp`](https://www.openssl.org/docs/manmaster/man3/CRYPTO_memcmp.html).

```rust
pub fn eq(a: &[u8], b: &[u8]) -> bool { /* ... */ }
```

## Module `nid`

A collection of numerical identifiers for OpenSSL objects.

```rust
pub mod nid { /* ... */ }
```

### Types

#### Struct `SignatureAlgorithms`

The digest and public-key algorithms associated with a signature.

```rust
pub struct SignatureAlgorithms {
    pub digest: Nid,
    pub pkey: Nid,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `digest` | `Nid` | The signature's digest.<br><br>If the signature does not specify a digest, this will be `NID::UNDEF`. |
| `pkey` | `Nid` | The signature's public-key. |

##### Implementations

###### Trait Implementations

- **Sync**
- **Unpin**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Send**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Nid`

A numerical identifier for an OpenSSL object.

Objects in OpenSSL can have a short name, a long name, and
a numerical identifier (NID). For convenience, objects
are usually represented in source code using these numeric
identifiers.

Users should generally not need to create new `Nid`s.

# Examples

To view the integer representation of a `Nid`:

```
use openssl::nid::Nid;

assert!(Nid::AES_256_GCM.as_raw() == 901);
```

# External Documentation

The following documentation provides context about `Nid`s and their usage
in OpenSSL.

- [Obj_nid2obj](https://www.openssl.org/docs/manmaster/crypto/OBJ_create.html)

```rust
pub struct Nid(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_raw(raw: c_int) -> Nid { /* ... */ }
  ```
  Create a `Nid` from an integer representation.

- ```rust
  pub const fn as_raw(self: &Self) -> c_int { /* ... */ }
  ```
  Return the integer representation of a `Nid`.

- ```rust
  pub fn create(oid: &str, sn: &str, ln: &str) -> Result<Nid, ErrorStack> { /* ... */ }
  ```
   Creates a new `Nid` for the `oid` with short name `sn` and long name `ln`.

- ```rust
  pub fn signature_algorithms(self: &Self) -> Option<SignatureAlgorithms> { /* ... */ }
  ```
   Returns the `Nid`s of the digest and public key algorithms associated with a signature ID.

- ```rust
  pub fn long_name(self: &Self) -> Result<&''static str, ErrorStack> { /* ... */ }
  ```
   Returns the string representation of a `Nid` (long).

- ```rust
  pub fn short_name(self: &Self) -> Result<&''static str, ErrorStack> { /* ... */ }
  ```
   Returns the string representation of a `Nid` (short).

###### Trait Implementations

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Copy**
- **Eq**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Nid) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **Sync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Nid { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `ocsp`

**Attributes:**

- `#[<cfg>(not(osslconf = "OPENSSL_NO_OCSP"))]`

```rust
pub mod ocsp { /* ... */ }
```

### Types

#### Struct `OcspFlag`

```rust
pub struct OcspFlag(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> c_ulong { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: c_ulong) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: c_ulong) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: c_ulong) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<OcspFlag> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<OcspFlag> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **UnwindSafe**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Copy**
- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **Flags**
  - ```rust
    fn bits(self: &Self) -> c_ulong { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: c_ulong) -> OcspFlag { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &OcspFlag) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Freeze**
- **RefUnwindSafe**
- **Unpin**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: OcspFlag) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OcspFlag) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &OcspFlag) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **Sync**
- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OcspFlag { /* ... */ }
    ```

- **PublicFlags**
#### Struct `OcspResponseStatus`

```rust
pub struct OcspResponseStatus(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_raw(raw: c_int) -> OcspResponseStatus { /* ... */ }
  ```

- ```rust
  pub fn as_raw(self: &Self) -> c_int { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OcspResponseStatus) -> bool { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> OcspResponseStatus { /* ... */ }
    ```

- **RefUnwindSafe**
- **StructuralPartialEq**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **Sync**
- **Freeze**
#### Struct `OcspCertStatus`

```rust
pub struct OcspCertStatus(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_raw(raw: c_int) -> OcspCertStatus { /* ... */ }
  ```

- ```rust
  pub fn as_raw(self: &Self) -> c_int { /* ... */ }
  ```

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> OcspCertStatus { /* ... */ }
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
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OcspCertStatus) -> bool { /* ... */ }
    ```

#### Struct `OcspRevokedStatus`

```rust
pub struct OcspRevokedStatus(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_raw(raw: c_int) -> OcspRevokedStatus { /* ... */ }
  ```

- ```rust
  pub fn as_raw(self: &Self) -> c_int { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OcspRevokedStatus) -> bool { /* ... */ }
    ```

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> OcspRevokedStatus { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `OcspStatus`

```rust
pub struct OcspStatus<''a> {
    pub status: OcspCertStatus,
    pub reason: OcspRevokedStatus,
    pub revocation_time: Option<&''a crate::asn1::Asn1GeneralizedTimeRef>,
    pub this_update: &''a crate::asn1::Asn1GeneralizedTimeRef,
    pub next_update: &''a crate::asn1::Asn1GeneralizedTimeRef,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `OcspCertStatus` | The overall status of the response. |
| `reason` | `OcspRevokedStatus` | If `status` is `CERT_STATUS_REVOKED`, the reason for the revocation. |
| `revocation_time` | `Option<&''a crate::asn1::Asn1GeneralizedTimeRef>` | If `status` is `CERT_STATUS_REVOKED`, the time at which the certificate was revoked. |
| `this_update` | `&''a crate::asn1::Asn1GeneralizedTimeRef` | The time that this revocation check was performed. |
| `next_update` | `&''a crate::asn1::Asn1GeneralizedTimeRef` | The time at which this revocation check expires. |

##### Implementations

###### Methods

- ```rust
  pub fn check_validity(self: &Self, nsec: u32, maxsec: Option<u32>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Checks validity of the `this_update` and `next_update` fields.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
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

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
#### Struct `OcspBasicResponse`

```rust
pub struct OcspBasicResponse(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut OcspBasicResponseRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &OcspBasicResponseRef { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &OcspBasicResponseRef { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &OcspBasicResponseRef { /* ... */ }
    ```

- **Freeze**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::OCSP_BASICRESP) -> OcspBasicResponse { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::OCSP_BASICRESP { /* ... */ }
    ```

- **Receiver**
#### Struct `OcspBasicResponseRef`

```rust
pub struct OcspBasicResponseRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn verify(self: &Self, certs: &StackRef<X509>, store: &X509StoreRef, flags: OcspFlag) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Verifies the validity of the response.

- ```rust
  pub fn find_status<''a>(self: &''a Self, id: &OcspCertIdRef) -> Option<OcspStatus<''a>> { /* ... */ }
  ```
   Looks up the status for the specified certificate ID.

###### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &OcspBasicResponseRef { /* ... */ }
    ```

- **Sync**
- **ForeignTypeRef**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &OcspBasicResponseRef { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `OcspCertId`

```rust
pub struct OcspCertId(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_cert(digest: MessageDigest, subject: &X509Ref, issuer: &X509Ref) -> Result<OcspCertId, ErrorStack> { /* ... */ }
  ```
   Constructs a certificate ID for certificate `subject`.

###### Trait Implementations

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &OcspCertIdRef { /* ... */ }
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

  - ```rust
    fn borrow(self: &Self) -> &OcspCertIdRef { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::OCSP_CERTID) -> OcspCertId { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::OCSP_CERTID { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &OcspCertIdRef { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Receiver**
- **Send**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut OcspCertIdRef { /* ... */ }
    ```

#### Struct `OcspCertIdRef`

```rust
pub struct OcspCertIdRef(/* private field */);
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

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **RefUnwindSafe**
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

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &OcspCertIdRef { /* ... */ }
    ```

- **ForeignTypeRef**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &OcspCertIdRef { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `OcspResponse`

```rust
pub struct OcspResponse(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn create(status: OcspResponseStatus, body: Option<&OcspBasicResponseRef>) -> Result<OcspResponse, ErrorStack> { /* ... */ }
  ```
   Creates an OCSP response from the status and optional body.

- ```rust
  pub fn from_der(der: &[u8]) -> Result<OcspResponse, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a DER-encoded OCSP response.

###### Trait Implementations

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &OcspResponseRef { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut OcspResponseRef { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::OCSP_RESPONSE) -> OcspResponse { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::OCSP_RESPONSE { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &OcspResponseRef { /* ... */ }
    ```

- **Unpin**
- **Receiver**
- **UnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &OcspResponseRef { /* ... */ }
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

- **RefUnwindSafe**
- **Send**
- **Sync**
#### Struct `OcspResponseRef`

```rust
pub struct OcspResponseRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the response to its standard DER encoding.

- ```rust
  pub fn status(self: &Self) -> OcspResponseStatus { /* ... */ }
  ```
   Returns the status of the response.

- ```rust
  pub fn basic(self: &Self) -> Result<OcspBasicResponse, ErrorStack> { /* ... */ }
  ```
   Returns the basic response.

###### Trait Implementations

- **Send**
- **Sync**
- **UnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &OcspResponseRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &OcspResponseRef { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ForeignTypeRef**
#### Struct `OcspRequest`

```rust
pub struct OcspRequest(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Result<OcspRequest, ErrorStack> { /* ... */ }
  ```

- ```rust
  pub fn from_der(der: &[u8]) -> Result<OcspRequest, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a DER-encoded OCSP request.

###### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut OcspRequestRef { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Receiver**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &OcspRequestRef { /* ... */ }
    ```

- **Unpin**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::OCSP_REQUEST) -> OcspRequest { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::OCSP_REQUEST { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &OcspRequestRef { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &OcspRequestRef { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
#### Struct `OcspRequestRef`

```rust
pub struct OcspRequestRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the request to its standard DER encoding.

- ```rust
  pub fn add_id(self: &mut Self, id: OcspCertId) -> Result<&mut OcspOneReqRef, ErrorStack> { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &OcspRequestRef { /* ... */ }
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
    fn as_ref(self: &Self) -> &OcspRequestRef { /* ... */ }
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

- **Send**
- **UnwindSafe**
- **Unpin**
- **ForeignTypeRef**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `OcspOneReq`

```rust
pub struct OcspOneReq(/* private field */);
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

- **Send**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Receiver**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &OcspOneReqRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &OcspOneReqRef { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut OcspOneReqRef { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &OcspOneReqRef { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::OCSP_ONEREQ) -> OcspOneReq { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::OCSP_ONEREQ { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `OcspOneReqRef`

```rust
pub struct OcspOneReqRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &OcspOneReqRef { /* ... */ }
    ```

- **Sync**
- **ForeignTypeRef**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &OcspOneReqRef { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **UnwindSafe**
## Module `pkcs12`

PKCS #12 archives.

```rust
pub mod pkcs12 { /* ... */ }
```

### Types

#### Struct `Pkcs12`

```rust
pub struct Pkcs12(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_der(der: &[u8]) -> Result<Pkcs12, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a DER-encoded PKCS#12 archive.

- ```rust
  pub fn builder() -> Pkcs12Builder { /* ... */ }
  ```
  Creates a new builder for a protected pkcs12 certificate.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Pkcs12Ref { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Receiver**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Pkcs12Ref { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Pkcs12Ref { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut Pkcs12Ref { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Unpin**
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

- **Send**
- **Freeze**
- **RefUnwindSafe**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::PKCS12) -> Pkcs12 { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::PKCS12 { /* ... */ }
    ```

#### Struct `Pkcs12Ref`

```rust
pub struct Pkcs12Ref(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the `Pkcs12` to its standard DER encoding.

- ```rust
  pub fn parse(self: &Self, pass: &str) -> Result<ParsedPkcs12, ErrorStack> { /* ... */ }
  ```
  Deprecated.

- ```rust
  pub fn parse2(self: &Self, pass: &str) -> Result<ParsedPkcs12_2, ErrorStack> { /* ... */ }
  ```
   Extracts the contents of the `Pkcs12`.

###### Trait Implementations

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

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **ForeignTypeRef**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Pkcs12Ref { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Pkcs12Ref { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `ParsedPkcs12`

**⚠️ Deprecated since 0.10.46**: Use ParsedPkcs12_2 instead

```rust
pub struct ParsedPkcs12 {
    pub pkey: crate::pkey::PKey<crate::pkey::Private>,
    pub cert: crate::x509::X509,
    pub chain: Option<crate::stack::Stack<crate::x509::X509>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `pkey` | `crate::pkey::PKey<crate::pkey::Private>` |  |
| `cert` | `crate::x509::X509` |  |
| `chain` | `Option<crate::stack::Stack<crate::x509::X509>>` |  |

##### Implementations

###### Trait Implementations

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

- **Sync**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
#### Struct `ParsedPkcs12_2`

```rust
pub struct ParsedPkcs12_2 {
    pub pkey: Option<crate::pkey::PKey<crate::pkey::Private>>,
    pub cert: Option<crate::x509::X509>,
    pub ca: Option<crate::stack::Stack<crate::x509::X509>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `pkey` | `Option<crate::pkey::PKey<crate::pkey::Private>>` |  |
| `cert` | `Option<crate::x509::X509>` |  |
| `ca` | `Option<crate::stack::Stack<crate::x509::X509>>` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
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

- **RefUnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Pkcs12Builder`

```rust
pub struct Pkcs12Builder {
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
  pub fn name(self: &mut Self, name: &str) -> &mut Self { /* ... */ }
  ```
  The `friendlyName` used for the certificate and private key.

- ```rust
  pub fn pkey<T>(self: &mut Self, pkey: &PKeyRef<T>) -> &mut Self
where
    T: HasPrivate { /* ... */ }
  ```
  The private key.

- ```rust
  pub fn cert(self: &mut Self, cert: &X509Ref) -> &mut Self { /* ... */ }
  ```
  The certificate.

- ```rust
  pub fn ca(self: &mut Self, ca: Stack<X509>) -> &mut Self { /* ... */ }
  ```
  An additional set of certificates to include in the archive beyond the one provided to

- ```rust
  pub fn key_algorithm(self: &mut Self, nid: Nid) -> &mut Self { /* ... */ }
  ```
  The encryption algorithm that should be used for the key

- ```rust
  pub fn cert_algorithm(self: &mut Self, nid: Nid) -> &mut Self { /* ... */ }
  ```
  The encryption algorithm that should be used for the cert

- ```rust
  pub fn key_iter(self: &mut Self, iter: u32) -> &mut Self { /* ... */ }
  ```
  Key iteration count, default is 2048 as of this writing

- ```rust
  pub fn mac_iter(self: &mut Self, mac_iter: u32) -> &mut Self { /* ... */ }
  ```
  MAC iteration count, default is the same as key_iter.

- ```rust
  pub fn mac_md(self: &mut Self, md: MessageDigest) -> &mut Self { /* ... */ }
  ```
  MAC message digest type

- ```rust
  pub fn build<T>(self: Self, password: &str, friendly_name: &str, pkey: &PKeyRef<T>, cert: &X509Ref) -> Result<Pkcs12, ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
  Deprecated.

- ```rust
  pub fn build2(self: &Self, password: &str) -> Result<Pkcs12, ErrorStack> { /* ... */ }
  ```
   Builds the PKCS#12 object.

###### Trait Implementations

- **UnwindSafe**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
## Module `pkcs5`

```rust
pub mod pkcs5 { /* ... */ }
```

### Types

#### Struct `KeyIvPair`

```rust
pub struct KeyIvPair {
    pub key: Vec<u8>,
    pub iv: Option<Vec<u8>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `key` | `Vec<u8>` |  |
| `iv` | `Option<Vec<u8>>` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> KeyIvPair { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &KeyIvPair) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
### Functions

#### Function `bytes_to_key`

**Attributes:**

- `#[allow(clippy::useless_conversion)]`
- `#[doc(alias = "EVP_BytesToKey")]`

 Derives a key and an IV from various parameters.

 If specified, `salt` must be 8 bytes in length.

 If the total key and IV length is less than 16 bytes and MD5 is used then
 the algorithm is compatible with the key derivation algorithm from PKCS#5
 v1.5 or PBKDF1 from PKCS#5 v2.0.

 New applications should not use this and instead use
 `pbkdf2_hmac` or another more modern key derivation algorithm.

This corresponds to [`EVP_BytesToKey`](https://www.openssl.org/docs/manmaster/man3/EVP_BytesToKey.html).

```rust
pub fn bytes_to_key(cipher: crate::symm::Cipher, digest: crate::hash::MessageDigest, data: &[u8], salt: Option<&[u8]>, count: i32) -> Result<KeyIvPair, crate::error::ErrorStack> { /* ... */ }
```

#### Function `pbkdf2_hmac`

**Attributes:**

- `#[doc(alias = "PKCS5_PBKDF2_HMAC")]`

 Derives a key from a password and salt using the PBKDF2-HMAC algorithm with a digest function.

This corresponds to [`PKCS5_PBKDF2_HMAC`](https://www.openssl.org/docs/manmaster/man3/PKCS5_PBKDF2_HMAC.html).

```rust
pub fn pbkdf2_hmac(pass: &[u8], salt: &[u8], iter: usize, hash: crate::hash::MessageDigest, key: &mut [u8]) -> Result<(), crate::error::ErrorStack> { /* ... */ }
```

#### Function `scrypt`

**Attributes:**

- `#[allow(clippy::useless_conversion)]`
- `#[doc(alias = "EVP_PBE_scrypt")]`

 Derives a key from a password and salt using the scrypt algorithm.

 Requires OpenSSL 1.1.0 or newer.

This corresponds to [`EVP_PBE_scrypt`](https://www.openssl.org/docs/manmaster/man3/EVP_PBE_scrypt.html).

```rust
pub fn scrypt(pass: &[u8], salt: &[u8], n: u64, r: u64, p: u64, maxmem: u64, key: &mut [u8]) -> Result<(), crate::error::ErrorStack> { /* ... */ }
```

## Module `pkcs7`

**Attributes:**

- `#[<cfg>(not(any(boringssl, awslc)))]`

```rust
pub mod pkcs7 { /* ... */ }
```

### Types

#### Struct `Pkcs7SignerInfo`

```rust
pub struct Pkcs7SignerInfo(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Pkcs7SignerInfoRef { /* ... */ }
    ```

- **Stackable**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Pkcs7SignerInfoRef { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Unpin**
- **Receiver**
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

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::PKCS7_SIGNER_INFO) -> Pkcs7SignerInfo { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::PKCS7_SIGNER_INFO { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut Pkcs7SignerInfoRef { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Pkcs7SignerInfoRef { /* ... */ }
    ```

#### Struct `Pkcs7SignerInfoRef`

```rust
pub struct Pkcs7SignerInfoRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Send**
- **Sync**
- **ForeignTypeRef**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Pkcs7SignerInfoRef { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Pkcs7SignerInfoRef { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
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

#### Struct `Pkcs7`

A PKCS#7 structure.

Contains signed and/or encrypted data.

```rust
pub struct Pkcs7(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_pem(pem: &[u8]) -> Result<Pkcs7, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a PEM-encoded PKCS#7 signature

- ```rust
  pub fn from_der(der: &[u8]) -> Result<Pkcs7, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a DER-encoded PKCS#7 signature

- ```rust
  pub fn from_smime(input: &[u8]) -> Result<(Pkcs7, Option<Vec<u8>>), ErrorStack> { /* ... */ }
  ```
   Parses a message in S/MIME format.

- ```rust
  pub fn encrypt(certs: &StackRef<X509>, input: &[u8], cipher: Cipher, flags: Pkcs7Flags) -> Result<Pkcs7, ErrorStack> { /* ... */ }
  ```
   Creates and returns a PKCS#7 `envelopedData` structure.

- ```rust
  pub fn sign<PT>(signcert: &X509Ref, pkey: &PKeyRef<PT>, certs: &StackRef<X509>, input: &[u8], flags: Pkcs7Flags) -> Result<Pkcs7, ErrorStack>
where
    PT: HasPrivate { /* ... */ }
  ```
   Creates and returns a PKCS#7 `signedData` structure.

###### Trait Implementations

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Pkcs7Ref { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::PKCS7) -> Pkcs7 { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::PKCS7 { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Pkcs7Ref { /* ... */ }
    ```

- **Receiver**
- **Freeze**
- **Unpin**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut Pkcs7Ref { /* ... */ }
    ```

- **Send**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Pkcs7Ref { /* ... */ }
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

#### Struct `Pkcs7Ref`

Reference to `Pkcs7`

```rust
pub struct Pkcs7Ref(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn to_smime(self: &Self, input: &[u8], flags: Pkcs7Flags) -> Result<Vec<u8>, ErrorStack> { /* ... */ }
  ```
   Converts PKCS#7 structure to S/MIME format

- ```rust
  pub fn to_pem(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the data into a PEM-encoded PKCS#7 structure.

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the data into a DER-encoded PKCS#7 structure.

- ```rust
  pub fn decrypt<PT>(self: &Self, pkey: &PKeyRef<PT>, cert: &X509Ref, flags: Pkcs7Flags) -> Result<Vec<u8>, ErrorStack>
where
    PT: HasPrivate { /* ... */ }
  ```
   Decrypts data using the provided private key.

- ```rust
  pub fn verify(self: &Self, certs: &StackRef<X509>, store: &X509StoreRef, indata: Option<&[u8]>, out: Option<&mut Vec<u8>>, flags: Pkcs7Flags) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Verifies the PKCS#7 `signedData` structure contained by `&self`.

- ```rust
  pub fn signers(self: &Self, certs: &StackRef<X509>, flags: Pkcs7Flags) -> Result<Stack<X509>, ErrorStack> { /* ... */ }
  ```
   Retrieve the signer's certificates from the PKCS#7 structure without verifying them.

- ```rust
  pub fn type_(self: &Self) -> Option<&Asn1ObjectRef> { /* ... */ }
  ```
  Return the type of a PKCS#7 structure as an Asn1Object

- ```rust
  pub fn signed(self: &Self) -> Option<&Pkcs7SignedRef> { /* ... */ }
  ```
  Get the signed data of a PKCS#7 structure of type PKCS7_SIGNED

###### Trait Implementations

- **Freeze**
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

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Pkcs7Ref { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **ForeignTypeRef**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Pkcs7Ref { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Pkcs7Signed`

A PKCS#7 signed data structure.

Contains signed data.

```rust
pub struct Pkcs7Signed(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::PKCS7_SIGNED) -> Pkcs7Signed { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::PKCS7_SIGNED { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Pkcs7SignedRef { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Pkcs7SignedRef { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Sync**
- **Unpin**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Pkcs7SignedRef { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut Pkcs7SignedRef { /* ... */ }
    ```

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

- **Receiver**
- **UnwindSafe**
#### Struct `Pkcs7SignedRef`

Reference to `Pkcs7Signed`

```rust
pub struct Pkcs7SignedRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn certificates(self: &Self) -> Option<&StackRef<X509>> { /* ... */ }
  ```
  Get the stack of certificates from the PKCS7_SIGNED object

###### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Pkcs7SignedRef { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ForeignTypeRef**
- **Unpin**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Pkcs7SignedRef { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Pkcs7Flags`

```rust
pub struct Pkcs7Flags(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> c_int { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: c_int) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: c_int) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: c_int) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<Pkcs7Flags> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<Pkcs7Flags> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **Copy**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Pkcs7Flags { /* ... */ }
    ```

- **Flags**
  - ```rust
    fn bits(self: &Self) -> c_int { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: c_int) -> Pkcs7Flags { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Pkcs7Flags) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Pkcs7Flags) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PublicFlags**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: Pkcs7Flags) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Pkcs7Flags) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Freeze**
## Module `pkey`

**Attributes:**

- `#[allow(clippy::missing_safety_doc)]`

Public/private key processing.

Asymmetric public key algorithms solve the problem of establishing and sharing
secret keys to securely send and receive messages.
This system uses a pair of keys: a public key, which can be freely
distributed, and a private key, which is kept to oneself. An entity may
encrypt information using a user's public key. The encrypted information can
only be deciphered using that user's private key.

This module offers support for five popular algorithms:

* RSA

* DSA

* Diffie-Hellman

* Elliptic Curves

* HMAC

These algorithms rely on hard mathematical problems - namely integer factorization,
discrete logarithms, and elliptic curve relationships - that currently do not
yield efficient solutions. This property ensures the security of these
cryptographic algorithms.

# Example

Generate a 2048-bit RSA public/private key pair and print the public key.

```rust
use openssl::rsa::Rsa;
use openssl::pkey::PKey;
use std::str;

let rsa = Rsa::generate(2048).unwrap();
let pkey = PKey::from_rsa(rsa).unwrap();

let pub_key: Vec<u8> = pkey.public_key_to_pem().unwrap();
println!("{:?}", str::from_utf8(pub_key.as_slice()).unwrap());
```

```rust
pub mod pkey { /* ... */ }
```

### Types

#### Enum `Params`

A tag type indicating that a key only has parameters.

```rust
pub enum Params {
}
```

##### Variants

##### Implementations

###### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **HasParams**
- **Unpin**
- **RefUnwindSafe**
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

- **UnwindSafe**
#### Enum `Public`

A tag type indicating that a key only has public components.

```rust
pub enum Public {
}
```

##### Variants

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **HasPublic**
- **Send**
- **RefUnwindSafe**
- **HasParams**
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

- **Sync**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Enum `Private`

A tag type indicating that a key has private components.

```rust
pub enum Private {
}
```

##### Variants

##### Implementations

###### Trait Implementations

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **HasPublic**
- **HasParams**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **HasPrivate**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `Id`

An identifier of a kind of key.

```rust
pub struct Id(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_raw(value: c_int) -> Id { /* ... */ }
  ```
  Creates a `Id` from an integer representation.

- ```rust
  pub fn as_raw(self: &Self) -> c_int { /* ... */ }
  ```
  Returns the integer representation of the `Id`.

###### Trait Implementations

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Id { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Id) -> bool { /* ... */ }
    ```

- **Freeze**
- **StructuralPartialEq**
- **Send**
- **Unpin**
- **Eq**
- **Sync**
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

- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `PKey`

A public or private key.

```rust
pub struct PKey<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_rsa(rsa: Rsa<T>) -> Result<PKey<T>, ErrorStack> { /* ... */ }
  ```
   Creates a new `PKey` containing an RSA key.

- ```rust
  pub fn from_dsa(dsa: Dsa<T>) -> Result<PKey<T>, ErrorStack> { /* ... */ }
  ```
   Creates a new `PKey` containing a DSA key.

- ```rust
  pub fn from_dh(dh: Dh<T>) -> Result<PKey<T>, ErrorStack> { /* ... */ }
  ```
   Creates a new `PKey` containing a Diffie-Hellman key.

- ```rust
  pub fn from_dhx(dh: Dh<T>) -> Result<PKey<T>, ErrorStack> { /* ... */ }
  ```
  Creates a new `PKey` containing a Diffie-Hellman key with type DHX.

- ```rust
  pub fn from_ec_key(ec_key: EcKey<T>) -> Result<PKey<T>, ErrorStack> { /* ... */ }
  ```
   Creates a new `PKey` containing an elliptic curve key.

- ```rust
  pub fn hmac(key: &[u8]) -> Result<PKey<Private>, ErrorStack> { /* ... */ }
  ```
   Creates a new `PKey` containing an HMAC key.

- ```rust
  pub fn cmac(cipher: &Cipher, key: &[u8]) -> Result<PKey<Private>, ErrorStack> { /* ... */ }
  ```
  Creates a new `PKey` containing a CMAC key.

- ```rust
  pub fn generate_x25519() -> Result<PKey<Private>, ErrorStack> { /* ... */ }
  ```
  Generates a new private X25519 key.

- ```rust
  pub fn generate_x448() -> Result<PKey<Private>, ErrorStack> { /* ... */ }
  ```
  Generates a new private X448 key.

- ```rust
  pub fn generate_ed25519() -> Result<PKey<Private>, ErrorStack> { /* ... */ }
  ```
  Generates a new private Ed25519 key.

- ```rust
  pub fn generate_ed448() -> Result<PKey<Private>, ErrorStack> { /* ... */ }
  ```
  Generates a new private Ed448 key.

- ```rust
  pub fn ec_gen(curve: &str) -> Result<PKey<Private>, ErrorStack> { /* ... */ }
  ```
   Generates a new EC key using the provided curve.

- ```rust
  pub fn private_key_from_pem(pem: &[u8]) -> Result<PKey<Private>, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a private key from a PEM-encoded key type specific format.

- ```rust
  pub fn private_key_from_pem_passphrase(pem: &[u8], passphrase: &[u8]) -> Result<PKey<Private>, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a private key from a PEM-encoded encrypted key type specific format.

- ```rust
  pub fn private_key_from_pem_callback<F>(pem: &[u8], callback: F) -> Result<PKey<Private>, crate::error::ErrorStack>
where
    F: FnOnce(&mut [u8]) -> Result<usize, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a private key from a PEM-encoded encrypted key type specific format.

- ```rust
  pub fn private_key_from_der(der: &[u8]) -> Result<PKey<Private>, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a DER-encoded private key.

- ```rust
  pub fn private_key_from_pkcs8(der: &[u8]) -> Result<PKey<Private>, ErrorStack> { /* ... */ }
  ```
  Deserializes a DER-formatted PKCS#8 unencrypted private key.

- ```rust
  pub fn private_key_from_pkcs8_callback<F>(der: &[u8], callback: F) -> Result<PKey<Private>, ErrorStack>
where
    F: FnOnce(&mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Deserializes a DER-formatted PKCS#8 private key, using a callback to retrieve the password

- ```rust
  pub fn private_key_from_pkcs8_passphrase(der: &[u8], passphrase: &[u8]) -> Result<PKey<Private>, ErrorStack> { /* ... */ }
  ```
   Deserializes a DER-formatted PKCS#8 private key, using the supplied password if the key is

- ```rust
  pub fn private_key_from_raw_bytes(bytes: &[u8], key_type: Id) -> Result<PKey<Private>, ErrorStack> { /* ... */ }
  ```
   Creates a private key from its raw byte representation

- ```rust
  pub fn public_key_from_pem(pem: &[u8]) -> Result<PKey<Public>, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a PEM-encoded SubjectPublicKeyInfo structure.

- ```rust
  pub fn public_key_from_pem_passphrase(pem: &[u8], passphrase: &[u8]) -> Result<PKey<Public>, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a PEM-encoded SubjectPublicKeyInfo structure.

- ```rust
  pub fn public_key_from_pem_callback<F>(pem: &[u8], callback: F) -> Result<PKey<Public>, crate::error::ErrorStack>
where
    F: FnOnce(&mut [u8]) -> Result<usize, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a PEM-encoded SubjectPublicKeyInfo structure.

- ```rust
  pub fn public_key_from_der(der: &[u8]) -> Result<PKey<Public>, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a DER-encoded SubjectPublicKeyInfo structure.

- ```rust
  pub fn public_key_from_raw_bytes(bytes: &[u8], key_type: Id) -> Result<PKey<Public>, ErrorStack> { /* ... */ }
  ```
   Creates a public key from its raw byte representation

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::EVP_PKEY) -> PKey<T> { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::EVP_PKEY { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut PKeyRef<T> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
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

  - ```rust
    fn borrow(self: &Self) -> &PKeyRef<T> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &PKeyRef<T> { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PKey<T> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &PKeyRef<T> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(ec_key: EcKey<T>) -> Result<PKey<T>, ErrorStack> { /* ... */ }
    ```

  - ```rust
    fn try_from(pkey: PKey<T>) -> Result<EcKey<T>, ErrorStack> { /* ... */ }
    ```

  - ```rust
    fn try_from(rsa: Rsa<T>) -> Result<PKey<T>, ErrorStack> { /* ... */ }
    ```

  - ```rust
    fn try_from(pkey: PKey<T>) -> Result<Rsa<T>, ErrorStack> { /* ... */ }
    ```

  - ```rust
    fn try_from(dsa: Dsa<T>) -> Result<PKey<T>, ErrorStack> { /* ... */ }
    ```

  - ```rust
    fn try_from(pkey: PKey<T>) -> Result<Dsa<T>, ErrorStack> { /* ... */ }
    ```

  - ```rust
    fn try_from(dh: Dh<T>) -> Result<PKey<T>, ErrorStack> { /* ... */ }
    ```

  - ```rust
    fn try_from(pkey: PKey<T>) -> Result<Dh<T>, ErrorStack> { /* ... */ }
    ```

- **Receiver**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `PKeyRef`

Reference to `PKey`.

```rust
pub struct PKeyRef<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn rsa(self: &Self) -> Result<Rsa<T>, ErrorStack> { /* ... */ }
  ```
   Returns a copy of the internal RSA key.

- ```rust
  pub fn dsa(self: &Self) -> Result<Dsa<T>, ErrorStack> { /* ... */ }
  ```
   Returns a copy of the internal DSA key.

- ```rust
  pub fn dh(self: &Self) -> Result<Dh<T>, ErrorStack> { /* ... */ }
  ```
   Returns a copy of the internal DH key.

- ```rust
  pub fn ec_key(self: &Self) -> Result<EcKey<T>, ErrorStack> { /* ... */ }
  ```
   Returns a copy of the internal elliptic curve key.

- ```rust
  pub fn id(self: &Self) -> Id { /* ... */ }
  ```
   Returns the `Id` that represents the type of this key.

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
   Returns the maximum size of a signature in bytes.

- ```rust
  pub fn public_key_to_pem(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the public key into a PEM-encoded SubjectPublicKeyInfo structure.

- ```rust
  pub fn public_key_to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the public key into a DER-encoded SubjectPublicKeyInfo structure.

- ```rust
  pub fn bits(self: &Self) -> u32 { /* ... */ }
  ```
   Returns the size of the key.

- ```rust
  pub fn security_bits(self: &Self) -> u32 { /* ... */ }
  ```
  Returns the number of security bits.

- ```rust
  pub fn public_eq<U>(self: &Self, other: &PKeyRef<U>) -> bool
where
    U: HasPublic { /* ... */ }
  ```
   Compares the public component of this key with another.

- ```rust
  pub fn raw_public_key(self: &Self) -> Result<Vec<u8>, ErrorStack> { /* ... */ }
  ```
   Raw byte representation of a public key.

- ```rust
  pub fn private_key_to_pem_pkcs8(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the private key to a PEM-encoded PKCS#8 PrivateKeyInfo structure.

- ```rust
  pub fn private_key_to_pem_pkcs8_passphrase(self: &Self, cipher: crate::symm::Cipher, passphrase: &[u8]) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the private key to a PEM-encoded PKCS#8 EncryptedPrivateKeyInfo structure.

- ```rust
  pub fn private_key_to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the private key to a DER-encoded key type specific format.

- ```rust
  pub fn raw_private_key(self: &Self) -> Result<Vec<u8>, ErrorStack> { /* ... */ }
  ```
   Raw byte representation of a private key.

- ```rust
  pub fn private_key_to_pkcs8(self: &Self) -> Result<Vec<u8>, ErrorStack> { /* ... */ }
  ```
   Serializes a private key into an unencrypted DER-formatted PKCS#8

- ```rust
  pub fn private_key_to_pkcs8_passphrase(self: &Self, cipher: Cipher, passphrase: &[u8]) -> Result<Vec<u8>, ErrorStack> { /* ... */ }
  ```
   Serializes a private key into a DER-formatted PKCS#8, using the supplied password to

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> PKey<T> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &PKeyRef<T> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **UnwindSafe**
- **Unpin**
- **ForeignTypeRef**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &PKeyRef<T> { /* ... */ }
    ```

- **Freeze**
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

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Traits

#### Trait `HasParams`

A trait indicating that a key has parameters.

```rust
pub unsafe trait HasParams {
    /* Associated items */
}
```

> This trait is unsafe to implement.

##### Implementations

This trait is implemented for the following types:

- `Params`
- `T` with <T>

#### Trait `HasPublic`

A trait indicating that a key has public components.

```rust
pub unsafe trait HasPublic {
    /* Associated items */
}
```

> This trait is unsafe to implement.

##### Implementations

This trait is implemented for the following types:

- `Public`
- `T` with <T>

#### Trait `HasPrivate`

A trait indicating that a key has private components.

```rust
pub unsafe trait HasPrivate {
    /* Associated items */
}
```

> This trait is unsafe to implement.

##### Implementations

This trait is implemented for the following types:

- `Private`

## Module `pkey_ctx`

**Attributes:**

- `#[<cfg_attr>(not(any(boringssl, awslc)), doc =
r#"\
Generate a CMAC key

```
use openssl::pkey_ctx::PkeyCtx;
use openssl::pkey::Id;
use openssl::cipher::Cipher;

let mut ctx = PkeyCtx::new_id(Id::CMAC).unwrap();
ctx.keygen_init().unwrap();
ctx.set_keygen_cipher(Cipher::aes_128_cbc()).unwrap();
ctx.set_keygen_mac_key(b"0123456789abcdef").unwrap();
let cmac_key = ctx.keygen().unwrap();
```"#)]`

The asymmetric encryption context.

# Examples

Encrypt data with RSA

```
use openssl::rsa::Rsa;
use openssl::pkey::PKey;
use openssl::pkey_ctx::PkeyCtx;

let key = Rsa::generate(4096).unwrap();
let key = PKey::from_rsa(key).unwrap();

let mut ctx = PkeyCtx::new(&key).unwrap();
ctx.encrypt_init().unwrap();

let data = b"Some Crypto Text";
let mut ciphertext = vec![];
ctx.encrypt_to_vec(data, &mut ciphertext).unwrap();
```
\
Generate a CMAC key

```
use openssl::pkey_ctx::PkeyCtx;
use openssl::pkey::Id;
use openssl::cipher::Cipher;

let mut ctx = PkeyCtx::new_id(Id::CMAC).unwrap();
ctx.keygen_init().unwrap();
ctx.set_keygen_cipher(Cipher::aes_128_cbc()).unwrap();
ctx.set_keygen_mac_key(b"0123456789abcdef").unwrap();
let cmac_key = ctx.keygen().unwrap();
```

Sign and verify data with RSA

```
use openssl::pkey_ctx::PkeyCtx;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;

// Generate a random RSA key.
let key = Rsa::generate(4096).unwrap();
let key = PKey::from_rsa(key).unwrap();

let text = b"Some Crypto Text";

// Create the signature.
let mut ctx = PkeyCtx::new(&key).unwrap();
ctx.sign_init().unwrap();
let mut signature = vec![];
ctx.sign_to_vec(text, &mut signature).unwrap();

// Verify the signature.
let mut ctx = PkeyCtx::new(&key).unwrap();
ctx.verify_init().unwrap();
let valid = ctx.verify(text, &signature).unwrap();
assert!(valid);
```

```rust
pub mod pkey_ctx { /* ... */ }
```

### Types

#### Struct `HkdfMode`

**Attributes:**

- `#[<cfg>(any(ossl111, libressl360))]`

HKDF modes of operation.

```rust
pub struct HkdfMode(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

###### Trait Implementations

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Sync**
- **Freeze**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

#### Struct `PkeyCtx`

A context object which can perform asymmetric cryptography operations.

```rust
pub struct PkeyCtx<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(pkey: &PKeyRef<T>) -> Result<Self, ErrorStack> { /* ... */ }
  ```
   Creates a new pkey context using the provided key.

- ```rust
  pub fn new_id(id: Id) -> Result<Self, ErrorStack> { /* ... */ }
  ```
   Creates a new pkey context for the specified algorithm ID.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &PkeyCtxRef<T> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::EVP_PKEY_CTX) -> PkeyCtx<T> { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::EVP_PKEY_CTX { /* ... */ }
    ```

- **Unpin**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut PkeyCtxRef<T> { /* ... */ }
    ```

- **Receiver**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &PkeyCtxRef<T> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &PkeyCtxRef<T> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
#### Struct `PkeyCtxRef`

A reference to a [`PkeyCtx`].

```rust
pub struct PkeyCtxRef<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn encrypt_init(self: &mut Self) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Prepares the context for encryption using the public key.

- ```rust
  pub fn verify_init(self: &mut Self) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Prepares the context for signature verification using the public key.

- ```rust
  pub fn verify_recover_init(self: &mut Self) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Prepares the context for signature recovery using the public key.

- ```rust
  pub fn encrypt(self: &mut Self, from: &[u8], to: Option<&mut [u8]>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Encrypts data using the public key.

- ```rust
  pub fn encrypt_to_vec(self: &mut Self, from: &[u8], out: &mut Vec<u8>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Like [`Self::encrypt`] but appends ciphertext to a [`Vec`].

- ```rust
  pub fn verify(self: &mut Self, data: &[u8], sig: &[u8]) -> Result<bool, ErrorStack> { /* ... */ }
  ```
   Verifies the signature of data using the public key.

- ```rust
  pub fn verify_recover(self: &mut Self, sig: &[u8], to: Option<&mut [u8]>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Recovers the original data signed by the private key. You almost

- ```rust
  pub fn decrypt_init(self: &mut Self) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Prepares the context for decryption using the private key.

- ```rust
  pub fn sign_init(self: &mut Self) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Prepares the context for signing using the private key.

- ```rust
  pub fn derive_set_peer<U>(self: &mut Self, key: &PKeyRef<U>) -> Result<(), ErrorStack>
where
    U: HasPublic { /* ... */ }
  ```
   Sets the peer key used for secret derivation.

- ```rust
  pub fn decrypt(self: &mut Self, from: &[u8], to: Option<&mut [u8]>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Decrypts data using the private key.

- ```rust
  pub fn decrypt_to_vec(self: &mut Self, from: &[u8], out: &mut Vec<u8>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Like [`Self::decrypt`] but appends plaintext to a [`Vec`].

- ```rust
  pub fn sign(self: &mut Self, data: &[u8], sig: Option<&mut [u8]>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Signs the contents of `data`.

- ```rust
  pub fn sign_to_vec(self: &mut Self, data: &[u8], sig: &mut Vec<u8>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Like [`Self::sign`] but appends the signature to a [`Vec`].

- ```rust
  pub fn derive_init(self: &mut Self) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Prepares the context for shared secret derivation.

- ```rust
  pub fn keygen_init(self: &mut Self) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Prepares the context for key generation.

- ```rust
  pub fn set_signature_md(self: &Self, md: &MdRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets which algorithm was used to compute the digest used in a

- ```rust
  pub fn rsa_padding(self: &Self) -> Result<Padding, ErrorStack> { /* ... */ }
  ```
   Returns the RSA padding mode in use.

- ```rust
  pub fn set_rsa_padding(self: &mut Self, padding: Padding) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the RSA padding mode.

- ```rust
  pub fn set_rsa_pss_saltlen(self: &mut Self, len: RsaPssSaltlen) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the RSA PSS salt length.

- ```rust
  pub fn set_rsa_mgf1_md(self: &mut Self, md: &MdRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the RSA MGF1 algorithm.

- ```rust
  pub fn set_rsa_oaep_md(self: &mut Self, md: &MdRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the RSA OAEP algorithm.

- ```rust
  pub fn set_rsa_oaep_label(self: &mut Self, label: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the RSA OAEP label.

- ```rust
  pub fn set_keygen_cipher(self: &mut Self, cipher: &CipherRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the cipher used during key generation.

- ```rust
  pub fn set_keygen_mac_key(self: &mut Self, key: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the key MAC key used during key generation.

- ```rust
  pub fn set_hkdf_md(self: &mut Self, digest: &MdRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the digest used for HKDF derivation.

- ```rust
  pub fn set_hkdf_mode(self: &mut Self, mode: HkdfMode) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the HKDF mode of operation.

- ```rust
  pub fn set_hkdf_key(self: &mut Self, key: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the input material for HKDF generation as the "key".

- ```rust
  pub fn set_hkdf_salt(self: &mut Self, salt: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the salt value for HKDF generation.

- ```rust
  pub fn add_hkdf_info(self: &mut Self, info: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Appends info bytes for HKDF generation.

- ```rust
  pub fn derive(self: &mut Self, buf: Option<&mut [u8]>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Derives a shared secret between two keys.

- ```rust
  pub fn derive_to_vec(self: &mut Self, buf: &mut Vec<u8>) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Like [`Self::derive`] but appends the secret to a [`Vec`].

- ```rust
  pub fn keygen(self: &mut Self) -> Result<PKey<Private>, ErrorStack> { /* ... */ }
  ```
   Generates a new public/private keypair.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &PkeyCtxRef<T> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &PkeyCtxRef<T> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **ForeignTypeRef**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Send**
- **Freeze**
- **RefUnwindSafe**
## Module `provider`

**Attributes:**

- `#[<cfg>(ossl300)]`

```rust
pub mod provider { /* ... */ }
```

### Types

#### Struct `Provider`

```rust
pub struct Provider(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn load(ctx: Option<&LibCtxRef>, name: &str) -> Result<Self, ErrorStack> { /* ... */ }
  ```
   Loads a new provider into the specified library context, disabling the fallback providers.

- ```rust
  pub fn try_load(ctx: Option<&LibCtxRef>, name: &str, retain_fallbacks: bool) -> Result<Self, ErrorStack> { /* ... */ }
  ```
   Loads a new provider into the specified library context, disabling the fallback providers if `retain_fallbacks`

- ```rust
  pub fn set_default_search_path(ctx: Option<&LibCtxRef>, path: &str) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Specifies the default search path that is to be used for looking for providers in the specified library context.

###### Trait Implementations

- **Freeze**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &ProviderRef { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut ProviderRef { /* ... */ }
    ```

- **Sync**
- **Receiver**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &ProviderRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &ProviderRef { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Unpin**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::OSSL_PROVIDER) -> Provider { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::OSSL_PROVIDER { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

#### Struct `ProviderRef`

A reference to a [`Provider`].

```rust
pub struct ProviderRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **ForeignTypeRef**
- **UnwindSafe**
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

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &ProviderRef { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &ProviderRef { /* ... */ }
    ```

## Module `rand`

Utilities for secure random number generation.

# Examples

To generate a buffer with cryptographically strong bytes:

```
use openssl::rand::rand_bytes;

let mut buf = [0; 256];
rand_bytes(&mut buf).unwrap();
```

```rust
pub mod rand { /* ... */ }
```

### Functions

#### Function `rand_bytes`

**Attributes:**

- `#[doc(alias = "RAND_bytes")]`

 Fill buffer with cryptographically strong pseudo-random bytes.

 # Examples

 To generate a buffer with cryptographically strong random bytes:

 ```
 use openssl::rand::rand_bytes;

 let mut buf = [0; 256];
 rand_bytes(&mut buf).unwrap();
 ```

This corresponds to [`RAND_bytes`](https://www.openssl.org/docs/manmaster/man3/RAND_bytes.html).

```rust
pub fn rand_bytes(buf: &mut [u8]) -> Result<(), crate::error::ErrorStack> { /* ... */ }
```

#### Function `rand_priv_bytes`

**Attributes:**

- `#[doc(alias = "RAND_priv_bytes")]`

 Fill buffer with cryptographically strong pseudo-random bytes. It is
 intended to be used for generating values that should remain private.

 # Examples

 To generate a buffer with cryptographically strong random bytes:

 ```
 use openssl::rand::rand_priv_bytes;

 let mut buf = [0; 256];
 rand_priv_bytes(&mut buf).unwrap();
 ```

 Requires OpenSSL 1.1.1 or newer.

This corresponds to [`RAND_priv_bytes`](https://www.openssl.org/docs/manmaster/man3/RAND_priv_bytes.html).

```rust
pub fn rand_priv_bytes(buf: &mut [u8]) -> Result<(), crate::error::ErrorStack> { /* ... */ }
```

#### Function `keep_random_devices_open`

**Attributes:**

- `#[doc(alias = "RAND_keep_random_devices_open")]`

 Controls random device file descriptor behavior.

 Requires OpenSSL 1.1.1 or newer.

This corresponds to [`RAND_keep_random_devices_open`](https://www.openssl.org/docs/manmaster/man3/RAND_keep_random_devices_open.html).

```rust
pub fn keep_random_devices_open(keep: bool) { /* ... */ }
```

## Module `rsa`

Rivest–Shamir–Adleman cryptosystem

RSA is one of the earliest asymmetric public key encryption schemes.
Like many other cryptosystems, RSA relies on the presumed difficulty of a hard
mathematical problem, namely factorization of the product of two large prime
numbers. At the moment there does not exist an algorithm that can factor such
large numbers in reasonable time. RSA is used in a wide variety of
applications including digital signatures and key exchanges such as
establishing a TLS/SSL connection.

The RSA acronym is derived from the first letters of the surnames of the
algorithm's founding trio.

# Example

Generate a 2048-bit RSA key pair and use the public key to encrypt some data.

```rust
use openssl::rsa::{Rsa, Padding};

let rsa = Rsa::generate(2048).unwrap();
let data = b"foobar";
let mut buf = vec![0; rsa.size() as usize];
let encrypted_len = rsa.public_encrypt(data, &mut buf, Padding::PKCS1).unwrap();
```

```rust
pub mod rsa { /* ... */ }
```

### Types

#### Struct `Padding`

Type of encryption padding to use.

Random length padding is primarily used to prevent attackers from
predicting or knowing the exact length of a plaintext message that
can possibly lead to breaking encryption.

```rust
pub struct Padding(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_raw(value: c_int) -> Padding { /* ... */ }
  ```
  Creates a `Padding` from an integer representation.

- ```rust
  pub fn as_raw(self: &Self) -> c_int { /* ... */ }
  ```
  Returns the integer representation of `Padding`.

###### Trait Implementations

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Padding { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Padding) -> bool { /* ... */ }
    ```

#### Struct `Rsa`

An RSA key.

```rust
pub struct Rsa<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_public_components(n: BigNum, e: BigNum) -> Result<Rsa<Public>, ErrorStack> { /* ... */ }
  ```
  Creates a new RSA key with only public components.

- ```rust
  pub fn public_key_from_pem(pem: &[u8]) -> Result<Rsa<Public>, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a PEM-encoded SubjectPublicKeyInfo structure containing an RSA key.

- ```rust
  pub fn public_key_from_pem_pkcs1(pem: &[u8]) -> Result<Rsa<Public>, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a PEM-encoded PKCS#1 RSAPublicKey structure.

- ```rust
  pub fn public_key_from_der(der: &[u8]) -> Result<Rsa<Public>, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a DER-encoded SubjectPublicKeyInfo structure containing an RSA key.

- ```rust
  pub fn public_key_from_der_pkcs1(der: &[u8]) -> Result<Rsa<Public>, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a DER-encoded PKCS#1 RSAPublicKey structure.

- ```rust
  pub fn from_private_components(n: BigNum, e: BigNum, d: BigNum, p: BigNum, q: BigNum, dmp1: BigNum, dmq1: BigNum, iqmp: BigNum) -> Result<Rsa<Private>, ErrorStack> { /* ... */ }
  ```
  Creates a new RSA key with private components (public components are assumed).

- ```rust
  pub fn generate(bits: u32) -> Result<Rsa<Private>, ErrorStack> { /* ... */ }
  ```
   Generates a public/private key pair with the specified size.

- ```rust
  pub fn generate_with_e(bits: u32, e: &BigNumRef) -> Result<Rsa<Private>, ErrorStack> { /* ... */ }
  ```
   Generates a public/private key pair with the specified size and a custom exponent.

- ```rust
  pub fn private_key_from_pem(pem: &[u8]) -> Result<Rsa<Private>, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a private key from a PEM-encoded PKCS#1 RSAPrivateKey structure.

- ```rust
  pub fn private_key_from_pem_passphrase(pem: &[u8], passphrase: &[u8]) -> Result<Rsa<Private>, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a private key from a PEM-encoded encrypted PKCS#1 RSAPrivateKey structure.

- ```rust
  pub fn private_key_from_pem_callback<F>(pem: &[u8], callback: F) -> Result<Rsa<Private>, crate::error::ErrorStack>
where
    F: FnOnce(&mut [u8]) -> Result<usize, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a private key from a PEM-encoded encrypted PKCS#1 RSAPrivateKey structure.

- ```rust
  pub fn private_key_from_der(der: &[u8]) -> Result<Rsa<Private>, crate::error::ErrorStack> { /* ... */ }
  ```
   Decodes a DER-encoded PKCS#1 RSAPrivateKey structure.

###### Trait Implementations

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &RsaRef<T> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Rsa<T> { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut RsaRef<T> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &RsaRef<T> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &RsaRef<T> { /* ... */ }
    ```

- **Sync**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::RSA) -> Rsa<T> { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::RSA { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(rsa: Rsa<T>) -> Result<PKey<T>, ErrorStack> { /* ... */ }
    ```

  - ```rust
    fn try_from(pkey: PKey<T>) -> Result<Rsa<T>, ErrorStack> { /* ... */ }
    ```

- **Receiver**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

#### Struct `RsaRef`

Reference to `RSA`

```rust
pub struct RsaRef<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn private_key_to_pem(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the private key to a PEM-encoded PKCS#1 RSAPrivateKey structure.

- ```rust
  pub fn private_key_to_pem_passphrase(self: &Self, cipher: crate::symm::Cipher, passphrase: &[u8]) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the private key to a PEM-encoded encrypted PKCS#1 RSAPrivateKey structure.

- ```rust
  pub fn private_key_to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the private key to a DER-encoded PKCS#1 RSAPrivateKey structure.

- ```rust
  pub fn private_decrypt(self: &Self, from: &[u8], to: &mut [u8], padding: Padding) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Decrypts data using the private key, returning the number of decrypted bytes.

- ```rust
  pub fn private_encrypt(self: &Self, from: &[u8], to: &mut [u8], padding: Padding) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Encrypts data using the private key, returning the number of encrypted bytes.

- ```rust
  pub fn d(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns a reference to the private exponent of the key.

- ```rust
  pub fn p(self: &Self) -> Option<&BigNumRef> { /* ... */ }
  ```
   Returns a reference to the first factor of the exponent of the key.

- ```rust
  pub fn q(self: &Self) -> Option<&BigNumRef> { /* ... */ }
  ```
   Returns a reference to the second factor of the exponent of the key.

- ```rust
  pub fn dmp1(self: &Self) -> Option<&BigNumRef> { /* ... */ }
  ```
   Returns a reference to the first exponent used for CRT calculations.

- ```rust
  pub fn dmq1(self: &Self) -> Option<&BigNumRef> { /* ... */ }
  ```
   Returns a reference to the second exponent used for CRT calculations.

- ```rust
  pub fn iqmp(self: &Self) -> Option<&BigNumRef> { /* ... */ }
  ```
   Returns a reference to the coefficient used for CRT calculations.

- ```rust
  pub fn check_key(self: &Self) -> Result<bool, ErrorStack> { /* ... */ }
  ```
   Validates RSA parameters for correctness

- ```rust
  pub fn public_key_to_pem(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the public key into a PEM-encoded SubjectPublicKeyInfo structure.

- ```rust
  pub fn public_key_to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the public key into a DER-encoded SubjectPublicKeyInfo structure.

- ```rust
  pub fn public_key_to_pem_pkcs1(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the public key into a PEM-encoded PKCS#1 RSAPublicKey structure.

- ```rust
  pub fn public_key_to_der_pkcs1(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the public key into a DER-encoded PKCS#1 RSAPublicKey structure.

- ```rust
  pub fn size(self: &Self) -> u32 { /* ... */ }
  ```
   Returns the size of the modulus in bytes.

- ```rust
  pub fn public_decrypt(self: &Self, from: &[u8], to: &mut [u8], padding: Padding) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Decrypts data using the public key, returning the number of decrypted bytes.

- ```rust
  pub fn public_encrypt(self: &Self, from: &[u8], to: &mut [u8], padding: Padding) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Encrypts data using the public key, returning the number of encrypted bytes.

- ```rust
  pub fn n(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns a reference to the modulus of the key.

- ```rust
  pub fn e(self: &Self) -> &BigNumRef { /* ... */ }
  ```
   Returns a reference to the public exponent of the key.

###### Trait Implementations

- **Sync**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &RsaRef<T> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> Rsa<T> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **ForeignTypeRef**
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
- **Freeze**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &RsaRef<T> { /* ... */ }
    ```

#### Struct `RsaPrivateKeyBuilder`

```rust
pub struct RsaPrivateKeyBuilder {
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
  pub fn new(n: BigNum, e: BigNum, d: BigNum) -> Result<RsaPrivateKeyBuilder, ErrorStack> { /* ... */ }
  ```
  Creates a new `RsaPrivateKeyBuilder`.

- ```rust
  pub fn set_factors(self: Self, p: BigNum, q: BigNum) -> Result<RsaPrivateKeyBuilder, ErrorStack> { /* ... */ }
  ```
   Sets the factors of the Rsa key.

- ```rust
  pub fn set_crt_params(self: Self, dmp1: BigNum, dmq1: BigNum, iqmp: BigNum) -> Result<RsaPrivateKeyBuilder, ErrorStack> { /* ... */ }
  ```
   Sets the Chinese Remainder Theorem params of the Rsa key.

- ```rust
  pub fn build(self: Self) -> Rsa<Private> { /* ... */ }
  ```
  Returns the Rsa key.

###### Trait Implementations

- **Unpin**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

## Module `sha`

The SHA family of hashes.

SHA, or Secure Hash Algorithms, are a family of cryptographic hashing algorithms published by
the National Institute of Standards and Technology (NIST).  Hash algorithms such as those in
the SHA family are used to map data of an arbitrary size to a fixed-size string of bytes.
As cryptographic hashing algorithms, these mappings have the property of being irreversible.
This property makes hash algorithms like these excellent for uses such as verifying the
contents of a file- if you know the hash you expect beforehand, then you can verify that the
data you have is correct if it hashes to the same value.

# Examples

When dealing with data that becomes available in chunks, such as while buffering data from IO,
you can create a hasher that you can repeatedly update to add bytes to.

```rust
use openssl::sha;

let mut hasher = sha::Sha256::new();

hasher.update(b"Hello, ");
hasher.update(b"world");

let hash = hasher.finish();
println!("Hashed \"Hello, world\" to {}", hex::encode(hash));
```

On the other hand, if you already have access to all of the data you would like to hash, you
may prefer to use the slightly simpler method of simply calling the hash function corresponding
to the algorithm you want to use.

```rust
use openssl::sha::sha256;

let hash = sha256(b"your data or message");
println!("Hash = {}", hex::encode(hash));
```

```rust
pub mod sha { /* ... */ }
```

### Types

#### Struct `Sha1`

An object which calculates a SHA1 hash of some data.

# Warning

SHA1 is known to be insecure - it should not be used unless required for
compatibility with existing systems.

```rust
pub struct Sha1(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Sha1 { /* ... */ }
  ```
   Creates a new hasher.

- ```rust
  pub fn update(self: &mut Self, buf: &[u8]) { /* ... */ }
  ```
   Feeds some data into the hasher.

- ```rust
  pub fn finish(self: Self) -> [u8; 20] { /* ... */ }
  ```
   Returns the hash of the data.

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> Sha1 { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
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
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Sha1 { /* ... */ }
    ```

#### Struct `Sha224`

An object which calculates a SHA224 hash of some data.

```rust
pub struct Sha224(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Sha224 { /* ... */ }
  ```
   Creates a new hasher.

- ```rust
  pub fn update(self: &mut Self, buf: &[u8]) { /* ... */ }
  ```
   Feeds some data into the hasher.

- ```rust
  pub fn finish(self: Self) -> [u8; 28] { /* ... */ }
  ```
   Returns the hash of the data.

###### Trait Implementations

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Sha224 { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Sha224 { /* ... */ }
    ```

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

#### Struct `Sha256`

An object which calculates a SHA256 hash of some data.

```rust
pub struct Sha256(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Sha256 { /* ... */ }
  ```
   Creates a new hasher.

- ```rust
  pub fn update(self: &mut Self, buf: &[u8]) { /* ... */ }
  ```
   Feeds some data into the hasher.

- ```rust
  pub fn finish(self: Self) -> [u8; 32] { /* ... */ }
  ```
   Returns the hash of the data.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Sha256 { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Freeze**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Sha256 { /* ... */ }
    ```

#### Struct `Sha384`

An object which calculates a SHA384 hash of some data.

```rust
pub struct Sha384(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Sha384 { /* ... */ }
  ```
   Creates a new hasher.

- ```rust
  pub fn update(self: &mut Self, buf: &[u8]) { /* ... */ }
  ```
   Feeds some data into the hasher.

- ```rust
  pub fn finish(self: Self) -> [u8; 48] { /* ... */ }
  ```
   Returns the hash of the data.

###### Trait Implementations

- **UnwindSafe**
- **Send**
- **Freeze**
- **Default**
  - ```rust
    fn default() -> Sha384 { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Sha384 { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
#### Struct `Sha512`

An object which calculates a SHA512 hash of some data.

```rust
pub struct Sha512(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Sha512 { /* ... */ }
  ```
   Creates a new hasher.

- ```rust
  pub fn update(self: &mut Self, buf: &[u8]) { /* ... */ }
  ```
   Feeds some data into the hasher.

- ```rust
  pub fn finish(self: Self) -> [u8; 64] { /* ... */ }
  ```
   Returns the hash of the data.

###### Trait Implementations

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Default**
  - ```rust
    fn default() -> Sha512 { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Sha512 { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Freeze**
- **Sync**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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
- **Send**
### Functions

#### Function `sha1`

**Attributes:**

- `#[inline]`
- `#[doc(alias = "SHA1")]`

 Computes the SHA1 hash of some data.

 # Warning

 SHA1 is known to be insecure - it should not be used unless required for
 compatibility with existing systems.

This corresponds to [`SHA1`](https://www.openssl.org/docs/manmaster/man3/SHA1.html).

```rust
pub fn sha1(data: &[u8]) -> [u8; 20] { /* ... */ }
```

#### Function `sha224`

**Attributes:**

- `#[inline]`
- `#[doc(alias = "SHA224")]`

 Computes the SHA224 hash of some data.

This corresponds to [`SHA224`](https://www.openssl.org/docs/manmaster/man3/SHA224.html).

```rust
pub fn sha224(data: &[u8]) -> [u8; 28] { /* ... */ }
```

#### Function `sha256`

**Attributes:**

- `#[inline]`
- `#[doc(alias = "SHA256")]`

 Computes the SHA256 hash of some data.

This corresponds to [`SHA256`](https://www.openssl.org/docs/manmaster/man3/SHA256.html).

```rust
pub fn sha256(data: &[u8]) -> [u8; 32] { /* ... */ }
```

#### Function `sha384`

**Attributes:**

- `#[inline]`
- `#[doc(alias = "SHA384")]`

 Computes the SHA384 hash of some data.

This corresponds to [`SHA384`](https://www.openssl.org/docs/manmaster/man3/SHA384.html).

```rust
pub fn sha384(data: &[u8]) -> [u8; 48] { /* ... */ }
```

#### Function `sha512`

**Attributes:**

- `#[inline]`
- `#[doc(alias = "SHA512")]`

 Computes the SHA512 hash of some data.

This corresponds to [`SHA512`](https://www.openssl.org/docs/manmaster/man3/SHA512.html).

```rust
pub fn sha512(data: &[u8]) -> [u8; 64] { /* ... */ }
```

## Module `sign`

**Attributes:**

- `#[<cfg_attr>(not(any(boringssl, awslc)), doc =
r#"\

Compute an HMAC:

```rust
use openssl::hash::MessageDigest;
use openssl::memcmp;
use openssl::pkey::PKey;
use openssl::sign::Signer;

// Create a PKey
let key = PKey::hmac(b"my secret").unwrap();

let data = b"hello, world!";
let data2 = b"hola, mundo!";

// Compute the HMAC
let mut signer = Signer::new(MessageDigest::sha256(), &key).unwrap();
signer.update(data).unwrap();
signer.update(data2).unwrap();
let hmac = signer.sign_to_vec().unwrap();

// `Verifier` cannot be used with HMACs; use the `memcmp::eq` function instead
//
// Do not simply check for equality with `==`!
# let target = hmac.clone();
assert!(memcmp::eq(&hmac, &target));
```"#)]`

Message signatures.

The `Signer` allows for the computation of cryptographic signatures of
data given a private key. The `Verifier` can then be used with the
corresponding public key to verify the integrity and authenticity of that
data given the signature.

# Examples

Sign and verify data given an RSA keypair:

```rust
use openssl::sign::{Signer, Verifier};
use openssl::rsa::Rsa;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;

// Generate a keypair
let keypair = Rsa::generate(2048).unwrap();
let keypair = PKey::from_rsa(keypair).unwrap();

let data = b"hello, world!";
let data2 = b"hola, mundo!";

// Sign the data
let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
signer.update(data).unwrap();
signer.update(data2).unwrap();
let signature = signer.sign_to_vec().unwrap();

// Verify the data
let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
verifier.update(data).unwrap();
verifier.update(data2).unwrap();
assert!(verifier.verify(&signature).unwrap());
```
\

Compute an HMAC:

```rust
use openssl::hash::MessageDigest;
use openssl::memcmp;
use openssl::pkey::PKey;
use openssl::sign::Signer;

// Create a PKey
let key = PKey::hmac(b"my secret").unwrap();

let data = b"hello, world!";
let data2 = b"hola, mundo!";

// Compute the HMAC
let mut signer = Signer::new(MessageDigest::sha256(), &key).unwrap();
signer.update(data).unwrap();
signer.update(data2).unwrap();
let hmac = signer.sign_to_vec().unwrap();

// `Verifier` cannot be used with HMACs; use the `memcmp::eq` function instead
//
// Do not simply check for equality with `==`!
# let target = hmac.clone();
assert!(memcmp::eq(&hmac, &target));
```

```rust
pub mod sign { /* ... */ }
```

### Types

#### Struct `RsaPssSaltlen`

Salt lengths that must be used with `set_rsa_pss_saltlen`.

```rust
pub struct RsaPssSaltlen(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn custom(val: c_int) -> RsaPssSaltlen { /* ... */ }
  ```
  Sets the salt length to the given value.

###### Trait Implementations

- **UnwindSafe**
- **Unpin**
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

- **RefUnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Signer`

A type which computes cryptographic signatures of data.

```rust
pub struct Signer<''a> {
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
  pub fn new<''a, T>(type_: MessageDigest, pkey: &PKeyRef<T>) -> Result<Signer<''a>, ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
   Creates a new `Signer`.

- ```rust
  pub fn new_without_digest<''a, T>(pkey: &PKeyRef<T>) -> Result<Signer<''a>, ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
   Creates a new `Signer` without a digest.

- ```rust
  pub fn rsa_padding(self: &Self) -> Result<Padding, ErrorStack> { /* ... */ }
  ```
   Returns the RSA padding mode in use.

- ```rust
  pub fn set_rsa_padding(self: &mut Self, padding: Padding) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the RSA padding mode.

- ```rust
  pub fn set_rsa_pss_saltlen(self: &mut Self, len: RsaPssSaltlen) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the RSA PSS salt length.

- ```rust
  pub fn set_rsa_mgf1_md(self: &mut Self, md: MessageDigest) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the RSA MGF1 algorithm.

- ```rust
  pub fn update(self: &mut Self, buf: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Feeds more data into the `Signer`.

- ```rust
  pub fn len(self: &Self) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Computes an upper bound on the signature length.

- ```rust
  pub fn sign(self: &Self, buf: &mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Writes the signature into the provided buffer, returning the number of bytes written.

- ```rust
  pub fn sign_to_vec(self: &Self) -> Result<Vec<u8>, ErrorStack> { /* ... */ }
  ```
  Returns the signature.

- ```rust
  pub fn sign_oneshot(self: &mut Self, sig_buf: &mut [u8], data_buf: &[u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
   Signs the data in `data_buf` and writes the signature into the buffer `sig_buf`, returning the

- ```rust
  pub fn sign_oneshot_to_vec(self: &mut Self, data_buf: &[u8]) -> Result<Vec<u8>, ErrorStack> { /* ... */ }
  ```
  Returns the signature.

###### Trait Implementations

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

- **RefUnwindSafe**
- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
#### Struct `Verifier`

A type which can be used to verify the integrity and authenticity
of data given the signature.

```rust
pub struct Verifier<''a> {
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
  pub fn new<T>(type_: MessageDigest, pkey: &''a PKeyRef<T>) -> Result<Verifier<''a>, ErrorStack>
where
    T: HasPublic { /* ... */ }
  ```
   Creates a new `Verifier`.

- ```rust
  pub fn new_without_digest<T>(pkey: &''a PKeyRef<T>) -> Result<Verifier<''a>, ErrorStack>
where
    T: HasPublic { /* ... */ }
  ```
   Creates a new `Verifier` without a digest.

- ```rust
  pub fn rsa_padding(self: &Self) -> Result<Padding, ErrorStack> { /* ... */ }
  ```
   Returns the RSA padding mode in use.

- ```rust
  pub fn set_rsa_padding(self: &mut Self, padding: Padding) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the RSA padding mode.

- ```rust
  pub fn set_rsa_pss_saltlen(self: &mut Self, len: RsaPssSaltlen) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the RSA PSS salt length.

- ```rust
  pub fn set_rsa_mgf1_md(self: &mut Self, md: MessageDigest) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the RSA MGF1 algorithm.

- ```rust
  pub fn update(self: &mut Self, buf: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Feeds more data into the `Verifier`.

- ```rust
  pub fn verify(self: &Self, signature: &[u8]) -> Result<bool, ErrorStack> { /* ... */ }
  ```
   Determines if the data fed into the `Verifier` matches the provided signature.

- ```rust
  pub fn verify_oneshot(self: &mut Self, signature: &[u8], buf: &[u8]) -> Result<bool, ErrorStack> { /* ... */ }
  ```
   Determines if the data given in `buf` matches the provided signature.

###### Trait Implementations

- **Sync**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **UnwindSafe**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `srtp`

```rust
pub mod srtp { /* ... */ }
```

### Types

#### Struct `SrtpProtectionProfile`

```rust
pub struct SrtpProtectionProfile(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &SrtpProtectionProfileRef { /* ... */ }
    ```

- **Send**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &SrtpProtectionProfileRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Receiver**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Stackable**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut SrtpProtectionProfileRef { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &SrtpProtectionProfileRef { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::SRTP_PROTECTION_PROFILE) -> SrtpProtectionProfile { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::SRTP_PROTECTION_PROFILE { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `SrtpProtectionProfileRef`

Reference to `SrtpProtectionProfile`.

```rust
pub struct SrtpProtectionProfileRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn id(self: &Self) -> SrtpProfileId { /* ... */ }
  ```

- ```rust
  pub fn name(self: &Self) -> &''static str { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &SrtpProtectionProfileRef { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **UnwindSafe**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &SrtpProtectionProfileRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **ForeignTypeRef**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `SrtpProfileId`

An identifier of an SRTP protection profile.

```rust
pub struct SrtpProfileId(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_raw(value: c_ulong) -> SrtpProfileId { /* ... */ }
  ```
  Creates a `SrtpProfileId` from an integer representation.

- ```rust
  pub fn as_raw(self: &Self) -> c_ulong { /* ... */ }
  ```
  Returns the integer representation of `SrtpProfileId`.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SrtpProfileId) -> bool { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
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

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SrtpProfileId { /* ... */ }
    ```

## Module `ssl`

SSL/TLS support.

`SslConnector` and `SslAcceptor` should be used in most cases - they handle
configuration of the OpenSSL primitives for you.

# Examples

To connect as a client to a remote server:

```no_run
use openssl::ssl::{SslMethod, SslConnector};
use std::io::{Read, Write};
use std::net::TcpStream;

let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

let stream = TcpStream::connect("google.com:443").unwrap();
let mut stream = connector.connect("google.com", stream).unwrap();

stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
let mut res = vec![];
stream.read_to_end(&mut res).unwrap();
println!("{}", String::from_utf8_lossy(&res));
```

To accept connections as a server from remote clients:

```no_run
use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;


let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
acceptor.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
acceptor.set_certificate_chain_file("certs.pem").unwrap();
acceptor.check_private_key().unwrap();
let acceptor = Arc::new(acceptor.build());

let listener = TcpListener::bind("0.0.0.0:8443").unwrap();

fn handle_client(stream: SslStream<TcpStream>) {
    // ...
}

for stream in listener.incoming() {
    match stream {
        Ok(stream) => {
            let acceptor = acceptor.clone();
            thread::spawn(move || {
                let stream = acceptor.accept(stream).unwrap();
                handle_client(stream);
            });
        }
        Err(e) => { /* connection failed */ }
    }
}
```

```rust
pub mod ssl { /* ... */ }
```

### Types

#### Struct `SslOptions`

Options controlling the behavior of an `SslContext`.

```rust
pub struct SslOptions(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> u64 { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: u64) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: u64) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: u64) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<SslOptions> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<SslOptions> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **Copy**
- **Unpin**
- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **StructuralPartialEq**
- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Flags**
  - ```rust
    fn bits(self: &Self) -> u64 { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: u64) -> SslOptions { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Freeze**
- **PublicFlags**
- **Send**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SslOptions { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: SslOptions) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SslOptions) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SslOptions) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SslOptions) -> bool { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

#### Struct `SslMode`

Options controlling the behavior of an `SslContext`.

```rust
pub struct SslMode(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> libc::c_long { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: libc::c_long) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: libc::c_long) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: libc::c_long) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<SslMode> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<SslMode> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Sync**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SslMode) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **PublicFlags**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Flags**
  - ```rust
    fn bits(self: &Self) -> libc::c_long { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: libc::c_long) -> SslMode { /* ... */ }
    ```

- **Eq**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: SslMode) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SslMode { /* ... */ }
    ```

- **StructuralPartialEq**
- **Copy**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **RefUnwindSafe**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SslMode) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SslMode) -> bool { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

#### Struct `SslMethod`

A type specifying the kind of protocol an `SslContext` will speak.

```rust
pub struct SslMethod(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn tls() -> SslMethod { /* ... */ }
  ```
   Support all versions of the TLS protocol.

- ```rust
  pub fn dtls() -> SslMethod { /* ... */ }
  ```
   Support all versions of the DTLS protocol.

- ```rust
  pub fn tls_client() -> SslMethod { /* ... */ }
  ```
   Support all versions of the TLS protocol, explicitly as a client.

- ```rust
  pub fn tls_server() -> SslMethod { /* ... */ }
  ```
   Support all versions of the TLS protocol, explicitly as a server.

- ```rust
  pub fn dtls_client() -> SslMethod { /* ... */ }
  ```
   Support all versions of the DTLS protocol, explicitly as a client.

- ```rust
  pub fn dtls_server() -> SslMethod { /* ... */ }
  ```
   Support all versions of the DTLS protocol, explicitly as a server.

- ```rust
  pub unsafe fn from_ptr(ptr: *const ffi::SSL_METHOD) -> SslMethod { /* ... */ }
  ```
  Constructs an `SslMethod` from a pointer to the underlying OpenSSL value.

- ```rust
  pub fn as_ptr(self: &Self) -> *const ffi::SSL_METHOD { /* ... */ }
  ```
  Returns a pointer to the underlying OpenSSL value.

###### Trait Implementations

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SslMethod { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
- **Send**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `SslVerifyMode`

Options controlling the behavior of certificate verification.

```rust
pub struct SslVerifyMode(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> i32 { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: i32) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: i32) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: i32) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<SslVerifyMode> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<SslVerifyMode> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Flags**
  - ```rust
    fn bits(self: &Self) -> i32 { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: i32) -> SslVerifyMode { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SslVerifyMode) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **PublicFlags**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SslVerifyMode) -> bool { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SslVerifyMode) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SslVerifyMode { /* ... */ }
    ```

- **Eq**
- **StructuralPartialEq**
- **Unpin**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: SslVerifyMode) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Freeze**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **Sync**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

#### Struct `SslSessionCacheMode`

Options controlling the behavior of session caching.

```rust
pub struct SslSessionCacheMode(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> libc::c_long { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: libc::c_long) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: libc::c_long) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: libc::c_long) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<SslSessionCacheMode> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<SslSessionCacheMode> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SslSessionCacheMode) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SslSessionCacheMode) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: SslSessionCacheMode) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

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
- **StructuralPartialEq**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SslSessionCacheMode { /* ... */ }
    ```

- **Flags**
  - ```rust
    fn bits(self: &Self) -> libc::c_long { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: libc::c_long) -> SslSessionCacheMode { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SslSessionCacheMode) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **PublicFlags**
- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Struct `ExtensionContext`

Which messages and under which conditions an extension should be added or expected.

```rust
pub struct ExtensionContext(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> c_uint { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: c_uint) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: c_uint) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: c_uint) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<ExtensionContext> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<ExtensionContext> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Flags**
  - ```rust
    fn bits(self: &Self) -> c_uint { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: c_uint) -> ExtensionContext { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ExtensionContext) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExtensionContext { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ExtensionContext) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ExtensionContext) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **PublicFlags**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: ExtensionContext) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

#### Struct `SslFiletype`

An identifier of the format of a certificate or key file.

```rust
pub struct SslFiletype(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_raw(raw: c_int) -> SslFiletype { /* ... */ }
  ```
  Constructs an `SslFiletype` from a raw OpenSSL value.

- ```rust
  pub fn as_raw(self: &Self) -> c_int { /* ... */ }
  ```
  Returns the raw OpenSSL value represented by this type.

###### Trait Implementations

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Freeze**
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

- **Sync**
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

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SslFiletype { /* ... */ }
    ```

#### Struct `StatusType`

An identifier of a certificate status type.

```rust
pub struct StatusType(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_raw(raw: c_int) -> StatusType { /* ... */ }
  ```
  Constructs a `StatusType` from a raw OpenSSL value.

- ```rust
  pub fn as_raw(self: &Self) -> c_int { /* ... */ }
  ```
  Returns the raw OpenSSL value represented by this type.

###### Trait Implementations

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StatusType { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
#### Struct `NameType`

An identifier of a session name type.

```rust
pub struct NameType(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_raw(raw: c_int) -> StatusType { /* ... */ }
  ```
  Constructs a `StatusType` from a raw OpenSSL value.

- ```rust
  pub fn as_raw(self: &Self) -> c_int { /* ... */ }
  ```
  Returns the raw OpenSSL value represented by this type.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NameType { /* ... */ }
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

- **RefUnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Sync**
#### Struct `SniError`

An error returned from the SNI callback.

```rust
pub struct SniError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Copy**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SniError) -> bool { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SniError { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `SslAlert`

An SSL/TLS alert.

```rust
pub struct SslAlert(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

###### Trait Implementations

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SslAlert { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SslAlert) -> bool { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `AlpnError`

**Attributes:**

- `#[<cfg>(any(ossl102, libressl261, boringssl, awslc))]`

An error returned from an ALPN selection callback.

Requires AWS-LC or BoringSSL or OpenSSL 1.0.2 or LibreSSL 2.6.1 or newer.

```rust
pub struct AlpnError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **UnwindSafe**
- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Unpin**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> AlpnError { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AlpnError) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `ClientHelloResponse`

**Attributes:**

- `#[<cfg>(ossl111)]`

The result of a client hello callback.

Requires OpenSSL 1.1.1 or newer.

```rust
pub struct ClientHelloResponse(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClientHelloResponse) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClientHelloResponse { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
#### Struct `SslVersion`

An SSL/TLS protocol version.

```rust
pub struct SslVersion(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SslVersion) -> bool { /* ... */ }
    ```

- **Eq**
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

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SslVersion { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `SslContextBuilder`

A builder for `SslContext`s.

```rust
pub struct SslContextBuilder(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(method: SslMethod) -> Result<SslContextBuilder, ErrorStack> { /* ... */ }
  ```
   Creates a new `SslContextBuilder`.

- ```rust
  pub unsafe fn from_ptr(ctx: *mut ffi::SSL_CTX) -> SslContextBuilder { /* ... */ }
  ```
  Creates an `SslContextBuilder` from a pointer to a raw OpenSSL value.

- ```rust
  pub fn as_ptr(self: &Self) -> *mut ffi::SSL_CTX { /* ... */ }
  ```
  Returns a pointer to the raw OpenSSL value.

- ```rust
  pub fn set_verify(self: &mut Self, mode: SslVerifyMode) { /* ... */ }
  ```
   Configures the certificate verification method for new connections.

- ```rust
  pub fn set_verify_callback<F>(self: &mut Self, mode: SslVerifyMode, verify: F)
where
    F: Fn(bool, &mut X509StoreContextRef) -> bool + ''static + Sync + Send { /* ... */ }
  ```
   Configures the certificate verification method for new connections and

- ```rust
  pub fn set_servername_callback<F>(self: &mut Self, callback: F)
where
    F: Fn(&mut SslRef, &mut SslAlert) -> Result<(), SniError> + ''static + Sync + Send { /* ... */ }
  ```
   Configures the server name indication (SNI) callback for new connections.

- ```rust
  pub fn set_verify_depth(self: &mut Self, depth: u32) { /* ... */ }
  ```
   Sets the certificate verification depth.

- ```rust
  pub fn set_verify_cert_store(self: &mut Self, cert_store: X509Store) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets a custom certificate store for verifying peer certificates.

- ```rust
  pub fn set_cert_store(self: &mut Self, cert_store: X509Store) { /* ... */ }
  ```
   Replaces the context's certificate store.

- ```rust
  pub fn set_read_ahead(self: &mut Self, read_ahead: bool) { /* ... */ }
  ```
   Controls read ahead behavior.

- ```rust
  pub fn set_mode(self: &mut Self, mode: SslMode) -> SslMode { /* ... */ }
  ```
   Sets the mode used by the context, returning the previous mode.

- ```rust
  pub fn set_tmp_dh(self: &mut Self, dh: &DhRef<Params>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the parameters to be used during ephemeral Diffie-Hellman key exchange.

- ```rust
  pub fn set_tmp_dh_callback<F>(self: &mut Self, callback: F)
where
    F: Fn(&mut SslRef, bool, u32) -> Result<Dh<Params>, ErrorStack> + ''static + Sync + Send { /* ... */ }
  ```
   Sets the callback which will generate parameters to be used during ephemeral Diffie-Hellman

- ```rust
  pub fn set_tmp_ecdh(self: &mut Self, key: &EcKeyRef<Params>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the parameters to be used during ephemeral elliptic curve Diffie-Hellman key exchange.

- ```rust
  pub fn set_default_verify_paths(self: &mut Self) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Use the default locations of trusted certificates for verification.

- ```rust
  pub fn set_ca_file<P: AsRef<Path>>(self: &mut Self, file: P) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Loads trusted root certificates from a file.

- ```rust
  pub fn load_verify_locations(self: &mut Self, ca_file: Option<&Path>, ca_path: Option<&Path>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Loads trusted root certificates from a file and/or a directory.

- ```rust
  pub fn set_client_ca_list(self: &mut Self, list: Stack<X509Name>) { /* ... */ }
  ```
   Sets the list of CA names sent to the client.

- ```rust
  pub fn add_client_ca(self: &mut Self, cacert: &X509Ref) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Add the provided CA certificate to the list sent by the server to the client when

- ```rust
  pub fn set_session_id_context(self: &mut Self, sid_ctx: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Set the context identifier for sessions.

- ```rust
  pub fn set_certificate_file<P: AsRef<Path>>(self: &mut Self, file: P, file_type: SslFiletype) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Loads a leaf certificate from a file.

- ```rust
  pub fn set_certificate_chain_file<P: AsRef<Path>>(self: &mut Self, file: P) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Loads a certificate chain from a file.

- ```rust
  pub fn set_certificate(self: &mut Self, cert: &X509Ref) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the leaf certificate.

- ```rust
  pub fn add_extra_chain_cert(self: &mut Self, cert: X509) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Appends a certificate to the certificate chain.

- ```rust
  pub fn set_private_key_file<P: AsRef<Path>>(self: &mut Self, file: P, file_type: SslFiletype) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Loads the private key from a file.

- ```rust
  pub fn set_private_key<T>(self: &mut Self, key: &PKeyRef<T>) -> Result<(), ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
   Sets the private key.

- ```rust
  pub fn set_cipher_list(self: &mut Self, cipher_list: &str) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the list of supported ciphers for protocols before TLSv1.3.

- ```rust
  pub fn set_ciphersuites(self: &mut Self, cipher_list: &str) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the list of supported ciphers for the TLSv1.3 protocol.

- ```rust
  pub fn set_options(self: &mut Self, option: SslOptions) -> SslOptions { /* ... */ }
  ```
   Sets the options used by the context, returning the old set.

- ```rust
  pub fn options(self: &Self) -> SslOptions { /* ... */ }
  ```
   Returns the options used by the context.

- ```rust
  pub fn clear_options(self: &mut Self, option: SslOptions) -> SslOptions { /* ... */ }
  ```
   Clears the options used by the context, returning the old set.

- ```rust
  pub fn set_min_proto_version(self: &mut Self, version: Option<SslVersion>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the minimum supported protocol version.

- ```rust
  pub fn set_max_proto_version(self: &mut Self, version: Option<SslVersion>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the maximum supported protocol version.

- ```rust
  pub fn min_proto_version(self: &mut Self) -> Option<SslVersion> { /* ... */ }
  ```
   Gets the minimum supported protocol version.

- ```rust
  pub fn max_proto_version(self: &mut Self) -> Option<SslVersion> { /* ... */ }
  ```
   Gets the maximum supported protocol version.

- ```rust
  pub fn set_alpn_protos(self: &mut Self, protocols: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the protocols to sent to the server for Application Layer Protocol Negotiation (ALPN).

- ```rust
  pub fn set_tlsext_use_srtp(self: &mut Self, protocols: &str) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Enables the DTLS extension "use_srtp" as defined in RFC5764.

- ```rust
  pub fn set_alpn_select_callback<F>(self: &mut Self, callback: F)
where
    F: for<''a> Fn(&mut SslRef, &''a [u8]) -> Result<&''a [u8], AlpnError> + ''static + Sync + Send { /* ... */ }
  ```
   Sets the callback used by a server to select a protocol for Application Layer Protocol

- ```rust
  pub fn check_private_key(self: &Self) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Checks for consistency between the private key and certificate.

- ```rust
  pub fn cert_store(self: &Self) -> &X509StoreBuilderRef { /* ... */ }
  ```
   Returns a shared reference to the context's certificate store.

- ```rust
  pub fn cert_store_mut(self: &mut Self) -> &mut X509StoreBuilderRef { /* ... */ }
  ```
   Returns a mutable reference to the context's certificate store.

- ```rust
  pub fn verify_param(self: &Self) -> &X509VerifyParamRef { /* ... */ }
  ```
   Returns a reference to the X509 verification configuration.

- ```rust
  pub fn verify_param_mut(self: &mut Self) -> &mut X509VerifyParamRef { /* ... */ }
  ```
   Returns a mutable reference to the X509 verification configuration.

- ```rust
  pub fn set_status_callback<F>(self: &mut Self, callback: F) -> Result<(), ErrorStack>
where
    F: Fn(&mut SslRef) -> Result<bool, ErrorStack> + ''static + Sync + Send { /* ... */ }
  ```
   Sets the callback dealing with OCSP stapling.

- ```rust
  pub fn set_psk_client_callback<F>(self: &mut Self, callback: F)
where
    F: Fn(&mut SslRef, Option<&[u8]>, &mut [u8], &mut [u8]) -> Result<usize, ErrorStack> + ''static + Sync + Send { /* ... */ }
  ```
   Sets the callback for providing an identity and pre-shared key for a TLS-PSK client.

- ```rust
  pub fn set_psk_callback<F>(self: &mut Self, callback: F)
where
    F: Fn(&mut SslRef, Option<&[u8]>, &mut [u8], &mut [u8]) -> Result<usize, ErrorStack> + ''static + Sync + Send { /* ... */ }
  ```

- ```rust
  pub fn set_psk_server_callback<F>(self: &mut Self, callback: F)
where
    F: Fn(&mut SslRef, Option<&[u8]>, &mut [u8]) -> Result<usize, ErrorStack> + ''static + Sync + Send { /* ... */ }
  ```
   Sets the callback for providing an identity and pre-shared key for a TLS-PSK server.

- ```rust
  pub fn set_new_session_callback<F>(self: &mut Self, callback: F)
where
    F: Fn(&mut SslRef, SslSession) + ''static + Sync + Send { /* ... */ }
  ```
   Sets the callback which is called when new sessions are negotiated.

- ```rust
  pub fn set_remove_session_callback<F>(self: &mut Self, callback: F)
where
    F: Fn(&SslContextRef, &SslSessionRef) + ''static + Sync + Send { /* ... */ }
  ```
   Sets the callback which is called when sessions are removed from the context.

- ```rust
  pub unsafe fn set_get_session_callback<F>(self: &mut Self, callback: F)
where
    F: Fn(&mut SslRef, &[u8]) -> Option<SslSession> + ''static + Sync + Send { /* ... */ }
  ```
   Sets the callback which is called when a client proposed to resume a session but it was not

- ```rust
  pub fn set_keylog_callback<F>(self: &mut Self, callback: F)
where
    F: Fn(&SslRef, &str) + ''static + Sync + Send { /* ... */ }
  ```
   Sets the TLS key logging callback.

- ```rust
  pub fn set_session_cache_mode(self: &mut Self, mode: SslSessionCacheMode) -> SslSessionCacheMode { /* ... */ }
  ```
   Sets the session caching mode use for connections made with the context.

- ```rust
  pub fn set_stateless_cookie_generate_cb<F>(self: &mut Self, callback: F)
where
    F: Fn(&mut SslRef, &mut [u8]) -> Result<usize, ErrorStack> + ''static + Sync + Send { /* ... */ }
  ```
   Sets the callback for generating an application cookie for TLS1.3

- ```rust
  pub fn set_stateless_cookie_verify_cb<F>(self: &mut Self, callback: F)
where
    F: Fn(&mut SslRef, &[u8]) -> bool + ''static + Sync + Send { /* ... */ }
  ```
   Sets the callback for verifying an application cookie for TLS1.3

- ```rust
  pub fn set_cookie_generate_cb<F>(self: &mut Self, callback: F)
where
    F: Fn(&mut SslRef, &mut [u8]) -> Result<usize, ErrorStack> + ''static + Sync + Send { /* ... */ }
  ```
   Sets the callback for generating a DTLSv1 cookie

- ```rust
  pub fn set_cookie_verify_cb<F>(self: &mut Self, callback: F)
where
    F: Fn(&mut SslRef, &[u8]) -> bool + ''static + Sync + Send { /* ... */ }
  ```
   Sets the callback for verifying a DTLSv1 cookie

- ```rust
  pub fn set_ex_data<T>(self: &mut Self, index: Index<SslContext, T>, data: T) { /* ... */ }
  ```
   Sets the extra data at the specified index.

- ```rust
  pub fn add_custom_ext<AddFn, ParseFn, T>(self: &mut Self, ext_type: u16, context: ExtensionContext, add_cb: AddFn, parse_cb: ParseFn) -> Result<(), ErrorStack>
where
    AddFn: Fn(&mut SslRef, ExtensionContext, Option<(usize, &X509Ref)>) -> Result<Option<T>, SslAlert> + ''static + Sync + Send,
    T: AsRef<[u8]> + ''static + Sync + Send,
    ParseFn: Fn(&mut SslRef, ExtensionContext, &[u8], Option<(usize, &X509Ref)>) -> Result<(), SslAlert> + ''static + Sync + Send { /* ... */ }
  ```
   Adds a custom extension for a TLS/DTLS client or server for all supported protocol versions.

- ```rust
  pub fn set_max_early_data(self: &mut Self, bytes: u32) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the maximum amount of early data that will be accepted on incoming connections.

- ```rust
  pub fn set_client_hello_callback<F>(self: &mut Self, callback: F)
where
    F: Fn(&mut SslRef, &mut SslAlert) -> Result<ClientHelloResponse, ErrorStack> + ''static + Sync + Send { /* ... */ }
  ```
   Sets a callback which will be invoked just after the client's hello message is received.

- ```rust
  pub fn set_session_cache_size(self: &mut Self, size: i32) -> i64 { /* ... */ }
  ```
   Sets the context's session cache size limit, returning the previous limit.

- ```rust
  pub fn set_sigalgs_list(self: &mut Self, sigalgs: &str) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the context's supported signature algorithms.

- ```rust
  pub fn set_groups_list(self: &mut Self, groups: &str) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the context's supported elliptic curve groups.

- ```rust
  pub fn set_num_tickets(self: &mut Self, num_tickets: usize) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the number of TLS 1.3 session tickets that will be sent to a client after a full

- ```rust
  pub fn set_security_level(self: &mut Self, level: u32) { /* ... */ }
  ```
   Set the context's security level to a value between 0 and 5, inclusive.

- ```rust
  pub fn build(self: Self) -> SslContext { /* ... */ }
  ```
  Consumes the builder, returning a new `SslContext`.

###### Trait Implementations

- **Freeze**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `SslContext`

A context object for TLS streams.

Applications commonly configure a single `SslContext` that is shared by all of its
`SslStreams`.

```rust
pub struct SslContext(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn builder(method: SslMethod) -> Result<SslContextBuilder, ErrorStack> { /* ... */ }
  ```
  Creates a new builder object for an `SslContext`.

- ```rust
  pub fn new_ex_index<T>() -> Result<Index<SslContext, T>, ErrorStack>
where
    T: ''static + Sync + Send { /* ... */ }
  ```
   Returns a new extra data index.

###### Trait Implementations

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &SslContextRef { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &SslContextRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Receiver**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &SslContextRef { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **UnwindSafe**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut SslContextRef { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::SSL_CTX) -> SslContext { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::SSL_CTX { /* ... */ }
    ```

#### Struct `SslContextRef`

Reference to [`SslContext`]

[`SslContext`]: struct.SslContext.html

```rust
pub struct SslContextRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn certificate(self: &Self) -> Option<&X509Ref> { /* ... */ }
  ```
   Returns the certificate associated with this `SslContext`, if present.

- ```rust
  pub fn private_key(self: &Self) -> Option<&PKeyRef<Private>> { /* ... */ }
  ```
   Returns the private key associated with this `SslContext`, if present.

- ```rust
  pub fn cert_store(self: &Self) -> &X509StoreRef { /* ... */ }
  ```
   Returns a shared reference to the certificate store used for verification.

- ```rust
  pub fn extra_chain_certs(self: &Self) -> &StackRef<X509> { /* ... */ }
  ```
   Returns a shared reference to the stack of certificates making up the chain from the leaf.

- ```rust
  pub fn ex_data<T>(self: &Self, index: Index<SslContext, T>) -> Option<&T> { /* ... */ }
  ```
   Returns a reference to the extra data at the specified index.

- ```rust
  pub fn max_early_data(self: &Self) -> u32 { /* ... */ }
  ```
   Gets the maximum amount of early data that will be accepted on incoming connections.

- ```rust
  pub unsafe fn add_session(self: &Self, session: &SslSessionRef) -> bool { /* ... */ }
  ```
   Adds a session to the context's cache.

- ```rust
  pub unsafe fn remove_session(self: &Self, session: &SslSessionRef) -> bool { /* ... */ }
  ```
   Removes a session from the context's cache and marks it as non-resumable.

- ```rust
  pub fn session_cache_size(self: &Self) -> i64 { /* ... */ }
  ```
   Returns the context's session cache size limit.

- ```rust
  pub fn verify_mode(self: &Self) -> SslVerifyMode { /* ... */ }
  ```
   Returns the verify mode that was set on this context from [`SslContextBuilder::set_verify`].

- ```rust
  pub fn num_tickets(self: &Self) -> usize { /* ... */ }
  ```
   Gets the number of TLS 1.3 session tickets that will be sent to a client after a full

- ```rust
  pub fn security_level(self: &Self) -> u32 { /* ... */ }
  ```
   Get the context's security level, which controls the allowed parameters

###### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &SslContextRef { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ForeignTypeRef**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &SslContextRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> <Self as >::Owned { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `CipherBits`

Information about the state of a cipher.

```rust
pub struct CipherBits {
    pub secret: i32,
    pub algorithm: i32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `secret` | `i32` | The number of secret bits used for the cipher. |
| `algorithm` | `i32` | The number of bits processed by the chosen algorithm. |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Sync**
- **Unpin**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `SslCipher`

Information about a cipher.

```rust
pub struct SslCipher(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Receiver**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut SslCipherRef { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &SslCipherRef { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Stackable**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::SSL_CIPHER) -> SslCipher { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::SSL_CIPHER { /* ... */ }
    ```

#### Struct `SslCipherRef`

Reference to an [`SslCipher`].

[`SslCipher`]: struct.SslCipher.html

```rust
pub struct SslCipherRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn name(self: &Self) -> &''static str { /* ... */ }
  ```
   Returns the name of the cipher.

- ```rust
  pub fn standard_name(self: &Self) -> Option<&''static str> { /* ... */ }
  ```
   Returns the RFC-standard name of the cipher, if one exists.

- ```rust
  pub fn version(self: &Self) -> &''static str { /* ... */ }
  ```
   Returns the SSL/TLS protocol version that first defined the cipher.

- ```rust
  pub fn bits(self: &Self) -> CipherBits { /* ... */ }
  ```
   Returns the number of bits used for the cipher.

- ```rust
  pub fn description(self: &Self) -> String { /* ... */ }
  ```
   Returns a textual description of the cipher.

- ```rust
  pub fn handshake_digest(self: &Self) -> Option<MessageDigest> { /* ... */ }
  ```
   Returns the handshake digest of the cipher.

- ```rust
  pub fn cipher_nid(self: &Self) -> Option<Nid> { /* ... */ }
  ```
   Returns the NID corresponding to the cipher.

###### Trait Implementations

- **RefUnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **ForeignTypeRef**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `CipherLists`

A stack of selected ciphers, and a stack of selected signalling cipher suites

```rust
pub struct CipherLists {
    pub suites: crate::stack::Stack<SslCipher>,
    pub signalling_suites: crate::stack::Stack<SslCipher>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `suites` | `crate::stack::Stack<SslCipher>` |  |
| `signalling_suites` | `crate::stack::Stack<SslCipher>` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `SslSession`

An encoded SSL session.

These can be cached to share sessions across connections.

```rust
pub struct SslSession(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_der(der: &[u8]) -> Result<SslSession, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a DER-encoded session structure.

###### Trait Implementations

- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &SslSessionRef { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut SslSessionRef { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SslSession { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &SslSessionRef { /* ... */ }
    ```

- **Receiver**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::SSL_SESSION) -> SslSession { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::SSL_SESSION { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Freeze**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &SslSessionRef { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `SslSessionRef`

Reference to [`SslSession`].

[`SslSession`]: struct.SslSession.html

```rust
pub struct SslSessionRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn id(self: &Self) -> &[u8] { /* ... */ }
  ```
   Returns the SSL session ID.

- ```rust
  pub fn master_key_len(self: &Self) -> usize { /* ... */ }
  ```
   Returns the length of the master key.

- ```rust
  pub fn master_key(self: &Self, buf: &mut [u8]) -> usize { /* ... */ }
  ```
   Copies the master key into the provided buffer.

- ```rust
  pub fn max_early_data(self: &Self) -> u32 { /* ... */ }
  ```
   Gets the maximum amount of early data that can be sent on this session.

- ```rust
  pub fn time(self: &Self) -> libc::c_long { /* ... */ }
  ```
   Returns the time at which the session was established, in seconds since the Unix epoch.

- ```rust
  pub fn timeout(self: &Self) -> i64 { /* ... */ }
  ```
   Returns the sessions timeout, in seconds.

- ```rust
  pub fn protocol_version(self: &Self) -> SslVersion { /* ... */ }
  ```
   Returns the session's TLS protocol version.

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the session into a DER-encoded structure.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> SslSession { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &SslSessionRef { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &SslSessionRef { /* ... */ }
    ```

- **Send**
- **ForeignTypeRef**
- **Freeze**
- **RefUnwindSafe**
#### Struct `Ssl`

The state of an SSL/TLS session.

`Ssl` objects are created from an [`SslContext`], which provides configuration defaults.
These defaults can be overridden on a per-`Ssl` basis, however.

[`SslContext`]: struct.SslContext.html

```rust
pub struct Ssl(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new_ex_index<T>() -> Result<Index<Ssl, T>, ErrorStack>
where
    T: ''static + Sync + Send { /* ... */ }
  ```
   Returns a new extra data index.

- ```rust
  pub fn new(ctx: &SslContextRef) -> Result<Ssl, ErrorStack> { /* ... */ }
  ```
   Creates a new `Ssl`.

- ```rust
  pub fn connect<S>(self: Self, stream: S) -> Result<SslStream<S>, HandshakeError<S>>
where
    S: Read + Write { /* ... */ }
  ```
   Initiates a client-side TLS handshake.

- ```rust
  pub fn accept<S>(self: Self, stream: S) -> Result<SslStream<S>, HandshakeError<S>>
where
    S: Read + Write { /* ... */ }
  ```
   Initiates a server-side TLS handshake.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &SslRef { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &SslRef { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut SslRef { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::SSL) -> Ssl { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::SSL { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Receiver**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &SslRef { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Sync**
#### Struct `SslRef`

Reference to an [`Ssl`].

[`Ssl`]: struct.Ssl.html

```rust
pub struct SslRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn set_connect_state(self: &mut Self) { /* ... */ }
  ```
   Configure as an outgoing stream from a client.

- ```rust
  pub fn set_accept_state(self: &mut Self) { /* ... */ }
  ```
   Configure as an incoming stream to a server.

- ```rust
  pub fn set_verify(self: &mut Self, mode: SslVerifyMode) { /* ... */ }
  ```
   Like [`SslContextBuilder::set_verify`].

- ```rust
  pub fn verify_mode(self: &Self) -> SslVerifyMode { /* ... */ }
  ```
   Returns the verify mode that was set using `set_verify`.

- ```rust
  pub fn set_verify_callback<F>(self: &mut Self, mode: SslVerifyMode, verify: F)
where
    F: Fn(bool, &mut X509StoreContextRef) -> bool + ''static + Sync + Send { /* ... */ }
  ```
   Like [`SslContextBuilder::set_verify_callback`].

- ```rust
  pub fn set_tmp_dh(self: &mut Self, dh: &DhRef<Params>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Like [`SslContextBuilder::set_tmp_dh`].

- ```rust
  pub fn set_tmp_dh_callback<F>(self: &mut Self, callback: F)
where
    F: Fn(&mut SslRef, bool, u32) -> Result<Dh<Params>, ErrorStack> + ''static + Sync + Send { /* ... */ }
  ```
   Like [`SslContextBuilder::set_tmp_dh_callback`].

- ```rust
  pub fn set_tmp_ecdh(self: &mut Self, key: &EcKeyRef<Params>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Like [`SslContextBuilder::set_tmp_ecdh`].

- ```rust
  pub fn set_alpn_protos(self: &mut Self, protocols: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Like [`SslContextBuilder::set_alpn_protos`].

- ```rust
  pub fn current_cipher(self: &Self) -> Option<&SslCipherRef> { /* ... */ }
  ```
   Returns the current cipher if the session is active.

- ```rust
  pub fn state_string(self: &Self) -> &''static str { /* ... */ }
  ```
   Returns a short string describing the state of the session.

- ```rust
  pub fn state_string_long(self: &Self) -> &''static str { /* ... */ }
  ```
   Returns a longer string describing the state of the session.

- ```rust
  pub fn set_hostname(self: &mut Self, hostname: &str) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the host name to be sent to the server for Server Name Indication (SNI).

- ```rust
  pub fn peer_certificate(self: &Self) -> Option<X509> { /* ... */ }
  ```
   Returns the peer's certificate, if present.

- ```rust
  pub fn peer_cert_chain(self: &Self) -> Option<&StackRef<X509>> { /* ... */ }
  ```
   Returns the certificate chain of the peer, if present.

- ```rust
  pub fn verified_chain(self: &Self) -> Option<&StackRef<X509>> { /* ... */ }
  ```
   Returns the verified certificate chain of the peer, including the leaf certificate.

- ```rust
  pub fn certificate(self: &Self) -> Option<&X509Ref> { /* ... */ }
  ```
   Like [`SslContext::certificate`].

- ```rust
  pub fn private_key(self: &Self) -> Option<&PKeyRef<Private>> { /* ... */ }
  ```
   Like [`SslContext::private_key`].

- ```rust
  pub fn version(self: &Self) -> &str { /* ... */ }
  ```

- ```rust
  pub fn version2(self: &Self) -> Option<SslVersion> { /* ... */ }
  ```
   Returns the protocol version of the session.

- ```rust
  pub fn version_str(self: &Self) -> &''static str { /* ... */ }
  ```
   Returns a string describing the protocol version of the session.

- ```rust
  pub fn selected_alpn_protocol(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
   Returns the protocol selected via Application Layer Protocol Negotiation (ALPN).

- ```rust
  pub fn set_tlsext_use_srtp(self: &mut Self, protocols: &str) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Enables the DTLS extension "use_srtp" as defined in RFC5764.

- ```rust
  pub fn srtp_profiles(self: &Self) -> Option<&StackRef<SrtpProtectionProfile>> { /* ... */ }
  ```
   Gets all SRTP profiles that are enabled for handshake via set_tlsext_use_srtp

- ```rust
  pub fn selected_srtp_profile(self: &Self) -> Option<&SrtpProtectionProfileRef> { /* ... */ }
  ```
   Gets the SRTP profile selected by handshake.

- ```rust
  pub fn pending(self: &Self) -> usize { /* ... */ }
  ```
   Returns the number of bytes remaining in the currently processed TLS record.

- ```rust
  pub fn servername(self: &Self, type_: NameType) -> Option<&str> { /* ... */ }
  ```
   Returns the servername sent by the client via Server Name Indication (SNI).

- ```rust
  pub fn servername_raw(self: &Self, type_: NameType) -> Option<&[u8]> { /* ... */ }
  ```
   Returns the servername sent by the client via Server Name Indication (SNI).

- ```rust
  pub fn set_ssl_context(self: &mut Self, ctx: &SslContextRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Changes the context corresponding to the current connection.

- ```rust
  pub fn ssl_context(self: &Self) -> &SslContextRef { /* ... */ }
  ```
   Returns the context corresponding to the current connection.

- ```rust
  pub fn param_mut(self: &mut Self) -> &mut X509VerifyParamRef { /* ... */ }
  ```
   Returns a mutable reference to the X509 verification configuration.

- ```rust
  pub fn verify_result(self: &Self) -> X509VerifyResult { /* ... */ }
  ```
   Returns the certificate verification result.

- ```rust
  pub fn session(self: &Self) -> Option<&SslSessionRef> { /* ... */ }
  ```
   Returns a shared reference to the SSL session.

- ```rust
  pub fn client_random(self: &Self, buf: &mut [u8]) -> usize { /* ... */ }
  ```
   Copies the `client_random` value sent by the client in the TLS handshake into a buffer.

- ```rust
  pub fn server_random(self: &Self, buf: &mut [u8]) -> usize { /* ... */ }
  ```
   Copies the `server_random` value sent by the server in the TLS handshake into a buffer.

- ```rust
  pub fn export_keying_material(self: &Self, out: &mut [u8], label: &str, context: Option<&[u8]>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Derives keying material for application use in accordance to RFC 5705.

- ```rust
  pub fn export_keying_material_early(self: &Self, out: &mut [u8], label: &str, context: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Derives keying material for application use in accordance to RFC 5705.

- ```rust
  pub unsafe fn set_session(self: &mut Self, session: &SslSessionRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the session to be used.

- ```rust
  pub fn session_reused(self: &Self) -> bool { /* ... */ }
  ```
   Determines if the session provided to `set_session` was successfully reused.

- ```rust
  pub fn set_status_type(self: &mut Self, type_: StatusType) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the status response a client wishes the server to reply with.

- ```rust
  pub fn extms_support(self: &Self) -> Option<bool> { /* ... */ }
  ```
   Determines if current session used Extended Master Secret

- ```rust
  pub fn ocsp_status(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
   Returns the server's OCSP response, if present.

- ```rust
  pub fn set_ocsp_status(self: &mut Self, response: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the OCSP response to be returned to the client.

- ```rust
  pub fn is_server(self: &Self) -> bool { /* ... */ }
  ```
   Determines if this `Ssl` is configured for server-side or client-side use.

- ```rust
  pub fn set_ex_data<T>(self: &mut Self, index: Index<Ssl, T>, data: T) { /* ... */ }
  ```
   Sets the extra data at the specified index.

- ```rust
  pub fn ex_data<T>(self: &Self, index: Index<Ssl, T>) -> Option<&T> { /* ... */ }
  ```
   Returns a reference to the extra data at the specified index.

- ```rust
  pub fn ex_data_mut<T>(self: &mut Self, index: Index<Ssl, T>) -> Option<&mut T> { /* ... */ }
  ```
   Returns a mutable reference to the extra data at the specified index.

- ```rust
  pub fn set_max_early_data(self: &mut Self, bytes: u32) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the maximum amount of early data that will be accepted on this connection.

- ```rust
  pub fn max_early_data(self: &Self) -> u32 { /* ... */ }
  ```
   Gets the maximum amount of early data that can be sent on this connection.

- ```rust
  pub fn finished(self: &Self, buf: &mut [u8]) -> usize { /* ... */ }
  ```
   Copies the contents of the last Finished message sent to the peer into the provided buffer.

- ```rust
  pub fn peer_finished(self: &Self, buf: &mut [u8]) -> usize { /* ... */ }
  ```
   Copies the contents of the last Finished message received from the peer into the provided

- ```rust
  pub fn is_init_finished(self: &Self) -> bool { /* ... */ }
  ```
   Determines if the initial handshake has been completed.

- ```rust
  pub fn client_hello_isv2(self: &Self) -> bool { /* ... */ }
  ```
   Determines if the client's hello message is in the SSLv2 format.

- ```rust
  pub fn client_hello_legacy_version(self: &Self) -> Option<SslVersion> { /* ... */ }
  ```
   Returns the legacy version field of the client's hello message.

- ```rust
  pub fn client_hello_random(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
   Returns the random field of the client's hello message.

- ```rust
  pub fn client_hello_session_id(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
   Returns the session ID field of the client's hello message.

- ```rust
  pub fn client_hello_ciphers(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
   Returns the ciphers field of the client's hello message.

- ```rust
  pub fn bytes_to_cipher_list(self: &Self, bytes: &[u8], isv2format: bool) -> Result<CipherLists, ErrorStack> { /* ... */ }
  ```
   Decodes a slice of wire-format cipher suite specification bytes. Unsupported cipher suites

- ```rust
  pub fn client_hello_compression_methods(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
   Returns the compression methods field of the client's hello message.

- ```rust
  pub fn set_mtu(self: &mut Self, mtu: u32) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the MTU used for DTLS connections.

- ```rust
  pub fn psk_identity_hint(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
   Returns the PSK identity hint used during connection setup.

- ```rust
  pub fn psk_identity(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
   Returns the PSK identity used during connection setup.

- ```rust
  pub fn add_chain_cert(self: &mut Self, chain: X509) -> Result<(), ErrorStack> { /* ... */ }
  ```

- ```rust
  pub fn set_method(self: &mut Self, method: SslMethod) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Sets a new default TLS/SSL method for SSL objects

- ```rust
  pub fn set_private_key_file<P: AsRef<Path>>(self: &mut Self, path: P, ssl_file_type: SslFiletype) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Loads the private key from a file.

- ```rust
  pub fn set_private_key(self: &mut Self, pkey: &PKeyRef<Private>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the private key.

- ```rust
  pub fn set_certificate(self: &mut Self, cert: &X509Ref) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the certificate

- ```rust
  pub fn set_certificate_chain_file<P: AsRef<Path>>(self: &mut Self, path: P) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Loads a certificate chain from a file.

- ```rust
  pub fn add_client_ca(self: &mut Self, cacert: &X509Ref) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets ca certificate that client trusted

- ```rust
  pub fn set_client_ca_list(self: &mut Self, list: Stack<X509Name>) { /* ... */ }
  ```

- ```rust
  pub fn set_min_proto_version(self: &mut Self, version: Option<SslVersion>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the minimum supported protocol version.

- ```rust
  pub fn set_max_proto_version(self: &mut Self, version: Option<SslVersion>) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the maximum supported protocol version.

- ```rust
  pub fn set_ciphersuites(self: &mut Self, cipher_list: &str) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the list of supported ciphers for the TLSv1.3 protocol.

- ```rust
  pub fn set_cipher_list(self: &mut Self, cipher_list: &str) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the list of supported ciphers for protocols before TLSv1.3.

- ```rust
  pub fn set_verify_cert_store(self: &mut Self, cert_store: X509Store) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Set the certificate store used for certificate verification

- ```rust
  pub fn set_num_tickets(self: &mut Self, num_tickets: usize) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the number of TLS 1.3 session tickets that will be sent to a client after a full

- ```rust
  pub fn num_tickets(self: &Self) -> usize { /* ... */ }
  ```
   Gets the number of TLS 1.3 session tickets that will be sent to a client after a full

- ```rust
  pub fn set_security_level(self: &mut Self, level: u32) { /* ... */ }
  ```
   Set the context's security level to a value between 0 and 5, inclusive.

- ```rust
  pub fn security_level(self: &Self) -> u32 { /* ... */ }
  ```
   Get the connection's security level, which controls the allowed parameters

- ```rust
  pub fn peer_tmp_key(self: &Self) -> Result<PKey<Public>, ErrorStack> { /* ... */ }
  ```
   Get the temporary key provided by the peer that is used during key

- ```rust
  pub fn tmp_key(self: &Self) -> Result<PKey<Private>, ErrorStack> { /* ... */ }
  ```
   Returns the temporary key from the local end of the connection that is

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &SslRef { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ForeignTypeRef**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &SslRef { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `MidHandshakeSslStream`

An SSL stream midway through the handshake process.

```rust
pub struct MidHandshakeSslStream<S> {
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
  pub fn get_ref(self: &Self) -> &S { /* ... */ }
  ```
  Returns a shared reference to the inner stream.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut S { /* ... */ }
  ```
  Returns a mutable reference to the inner stream.

- ```rust
  pub fn ssl(self: &Self) -> &SslRef { /* ... */ }
  ```
  Returns a shared reference to the `Ssl` of the stream.

- ```rust
  pub fn error(self: &Self) -> &Error { /* ... */ }
  ```
  Returns the underlying error which interrupted this handshake.

- ```rust
  pub fn into_error(self: Self) -> Error { /* ... */ }
  ```
  Consumes `self`, returning its error.

- ```rust
  pub fn handshake(self: Self) -> Result<SslStream<S>, HandshakeError<S>> { /* ... */ }
  ```
   Restarts the handshake process.

###### Trait Implementations

- **Freeze**
- **Sync**
- **Send**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `SslStream`

A TLS session over a stream.

```rust
pub struct SslStream<S> {
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
  pub fn new(ssl: Ssl, stream: S) -> Result<Self, ErrorStack> { /* ... */ }
  ```
   Creates a new `SslStream`.

- ```rust
  pub unsafe fn from_raw_parts(ssl: *mut ffi::SSL, stream: S) -> Self { /* ... */ }
  ```
  Constructs an `SslStream` from a pointer to the underlying OpenSSL `SSL` struct.

- ```rust
  pub fn read_early_data(self: &mut Self, buf: &mut [u8]) -> Result<usize, Error> { /* ... */ }
  ```
   Read application data transmitted by a client before handshake completion.

- ```rust
  pub fn write_early_data(self: &mut Self, buf: &[u8]) -> Result<usize, Error> { /* ... */ }
  ```
   Send data to the server without blocking on handshake completion.

- ```rust
  pub fn connect(self: &mut Self) -> Result<(), Error> { /* ... */ }
  ```
   Initiates a client-side TLS handshake.

- ```rust
  pub fn accept(self: &mut Self) -> Result<(), Error> { /* ... */ }
  ```
   Initiates a server-side TLS handshake.

- ```rust
  pub fn do_handshake(self: &mut Self) -> Result<(), Error> { /* ... */ }
  ```
   Initiates the handshake.

- ```rust
  pub fn stateless(self: &mut Self) -> Result<bool, ErrorStack> { /* ... */ }
  ```
   Perform a stateless server-side handshake.

- ```rust
  pub fn read_uninit(self: &mut Self, buf: &mut [MaybeUninit<u8>]) -> io::Result<usize> { /* ... */ }
  ```
   Like `read`, but takes a possibly-uninitialized slice.

- ```rust
  pub fn ssl_read(self: &mut Self, buf: &mut [u8]) -> Result<usize, Error> { /* ... */ }
  ```
   Like `read`, but returns an `ssl::Error` rather than an `io::Error`.

- ```rust
  pub fn ssl_read_uninit(self: &mut Self, buf: &mut [MaybeUninit<u8>]) -> Result<usize, Error> { /* ... */ }
  ```
   Like `read_ssl`, but takes a possibly-uninitialized slice.

- ```rust
  pub fn ssl_write(self: &mut Self, buf: &[u8]) -> Result<usize, Error> { /* ... */ }
  ```
   Like `write`, but returns an `ssl::Error` rather than an `io::Error`.

- ```rust
  pub fn ssl_peek(self: &mut Self, buf: &mut [u8]) -> Result<usize, Error> { /* ... */ }
  ```
   Reads data from the stream, without removing it from the queue.

- ```rust
  pub fn shutdown(self: &mut Self) -> Result<ShutdownResult, Error> { /* ... */ }
  ```
   Shuts down the session.

- ```rust
  pub fn get_shutdown(self: &mut Self) -> ShutdownState { /* ... */ }
  ```
   Returns the session's shutdown state.

- ```rust
  pub fn set_shutdown(self: &mut Self, state: ShutdownState) { /* ... */ }
  ```
   Sets the session's shutdown state.

- ```rust
  pub fn get_ref(self: &Self) -> &S { /* ... */ }
  ```
  Returns a shared reference to the underlying stream.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut S { /* ... */ }
  ```
  Returns a mutable reference to the underlying stream.

- ```rust
  pub fn ssl(self: &Self) -> &SslRef { /* ... */ }
  ```
  Returns a shared reference to the `Ssl` object associated with this stream.

###### Trait Implementations

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
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

- **RefUnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Read**
  - ```rust
    fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `SslStreamBuilder`

**⚠️ Deprecated since 0.10.32**: use the methods directly on Ssl/SslStream instead

A partially constructed `SslStream`, useful for unusual handshakes.

```rust
pub struct SslStreamBuilder<S> {
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
  pub fn new(ssl: Ssl, stream: S) -> Self { /* ... */ }
  ```
  Begin creating an `SslStream` atop `stream`

- ```rust
  pub fn stateless(self: &mut Self) -> Result<bool, ErrorStack> { /* ... */ }
  ```
   Perform a stateless server-side handshake

- ```rust
  pub fn set_connect_state(self: &mut Self) { /* ... */ }
  ```
   Configure as an outgoing stream from a client.

- ```rust
  pub fn set_accept_state(self: &mut Self) { /* ... */ }
  ```
   Configure as an incoming stream to a server.

- ```rust
  pub fn connect(self: Self) -> Result<SslStream<S>, HandshakeError<S>> { /* ... */ }
  ```
  See `Ssl::connect`

- ```rust
  pub fn accept(self: Self) -> Result<SslStream<S>, HandshakeError<S>> { /* ... */ }
  ```
  See `Ssl::accept`

- ```rust
  pub fn handshake(self: Self) -> Result<SslStream<S>, HandshakeError<S>> { /* ... */ }
  ```
   Initiates the handshake.

- ```rust
  pub fn read_early_data(self: &mut Self, buf: &mut [u8]) -> Result<usize, Error> { /* ... */ }
  ```
   Read application data transmitted by a client before handshake

- ```rust
  pub fn write_early_data(self: &mut Self, buf: &[u8]) -> Result<usize, Error> { /* ... */ }
  ```
   Send data to the server without blocking on handshake completion.

- ```rust
  pub fn get_ref(self: &Self) -> &S { /* ... */ }
  ```
  Returns a shared reference to the underlying stream.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut S { /* ... */ }
  ```
  Returns a mutable reference to the underlying stream.

- ```rust
  pub fn ssl(self: &Self) -> &SslRef { /* ... */ }
  ```
  Returns a shared reference to the `Ssl` object associated with this builder.

- ```rust
  pub fn set_dtls_mtu_size(self: &mut Self, mtu_size: usize) { /* ... */ }
  ```
  Set the DTLS MTU size.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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
- **RefUnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Enum `ShutdownResult`

The result of a shutdown request.

```rust
pub enum ShutdownResult {
    Sent,
    Received,
}
```

##### Variants

###### `Sent`

A close notify message has been sent to the peer.

###### `Received`

A close notify response message has been received from the peer.

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ShutdownResult { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ShutdownResult) -> bool { /* ... */ }
    ```

- **Copy**
- **Eq**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Sync**
- **Freeze**
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

- **Unpin**
#### Struct `ShutdownState`

The shutdown state of a session.

```rust
pub struct ShutdownState(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> c_int { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: c_int) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: c_int) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: c_int) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<ShutdownState> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<ShutdownState> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

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

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **Flags**
  - ```rust
    fn bits(self: &Self) -> c_int { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: c_int) -> ShutdownState { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Unpin**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ShutdownState { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: ShutdownState) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ShutdownState) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ShutdownState) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ShutdownState) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **PublicFlags**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Functions

#### Function `cipher_name`

**Attributes:**

- `#[doc(alias = "OPENSSL_cipher_name")]`

 Returns the OpenSSL name of a cipher corresponding to an RFC-standard cipher name.

 If the cipher has no corresponding OpenSSL name, the string `(NONE)` is returned.

 Requires OpenSSL 1.1.1 or newer.

This corresponds to [`OPENSSL_cipher_name`](https://www.openssl.org/docs/manmaster/man3/OPENSSL_cipher_name.html).

```rust
pub fn cipher_name(std_name: &str) -> &''static str { /* ... */ }
```

#### Function `select_next_proto`

**Attributes:**

- `#[doc(alias = "SSL_select_next_proto")]`

 A standard implementation of protocol selection for Application Layer Protocol Negotiation
 (ALPN).

 `server` should contain the server's list of supported protocols and `client` the client's. They
 must both be in the ALPN wire format. See the documentation for
 [`SslContextBuilder::set_alpn_protos`] for details.

 It will select the first protocol supported by the server which is also supported by the client.

 [`SslContextBuilder::set_alpn_protos`]: struct.SslContextBuilder.html#method.set_alpn_protos

This corresponds to [`SSL_select_next_proto`](https://www.openssl.org/docs/manmaster/man3/SSL_select_next_proto.html).

```rust
pub fn select_next_proto<''a>(server: &''a [u8], client: &''a [u8]) -> Option<&''a [u8]> { /* ... */ }
```

### Re-exports

#### Re-export `ConnectConfiguration`

```rust
pub use crate::ssl::connector::ConnectConfiguration;
```

#### Re-export `SslAcceptor`

```rust
pub use crate::ssl::connector::SslAcceptor;
```

#### Re-export `SslAcceptorBuilder`

```rust
pub use crate::ssl::connector::SslAcceptorBuilder;
```

#### Re-export `SslConnector`

```rust
pub use crate::ssl::connector::SslConnector;
```

#### Re-export `SslConnectorBuilder`

```rust
pub use crate::ssl::connector::SslConnectorBuilder;
```

#### Re-export `Error`

```rust
pub use crate::ssl::error::Error;
```

#### Re-export `ErrorCode`

```rust
pub use crate::ssl::error::ErrorCode;
```

#### Re-export `HandshakeError`

```rust
pub use crate::ssl::error::HandshakeError;
```

## Module `stack`

```rust
pub mod stack { /* ... */ }
```

### Types

#### Struct `Stack`

An owned stack of `T`.

```rust
pub struct Stack<T: Stackable>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Result<Stack<T>, ErrorStack> { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &StackRef<T> { /* ... */ }
    ```

- **UnwindSafe**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut <T as >::StackType) -> Stack<T> { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut <T as >::StackType { /* ... */ }
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

  - ```rust
    fn borrow(self: &Self) -> &StackRef<T> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> IntoIter<T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> Iter<''a, T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> IterMut<''a, T> { /* ... */ }
    ```

- **Receiver**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Unpin**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **RefUnwindSafe**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut StackRef<T> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &StackRef<T> { /* ... */ }
    ```

#### Struct `IntoIter`

```rust
pub struct IntoIter<T: Stackable> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<T> { /* ... */ }
    ```

- **ExactSizeIterator**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **RefUnwindSafe**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
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

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

#### Struct `StackRef`

```rust
pub struct StackRef<T: Stackable>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of items in the stack.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Determines if the stack is empty.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, T> { /* ... */ }
  ```

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, T> { /* ... */ }
  ```

- ```rust
  pub fn get(self: &Self, idx: usize) -> Option<&<T as >::Ref> { /* ... */ }
  ```
  Returns a reference to the element at the given index in the

- ```rust
  pub fn get_mut(self: &mut Self, idx: usize) -> Option<&mut <T as >::Ref> { /* ... */ }
  ```
  Returns a mutable reference to the element at the given index in the

- ```rust
  pub fn push(self: &mut Self, data: T) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Pushes a value onto the top of the stack.

- ```rust
  pub fn pop(self: &mut Self) -> Option<T> { /* ... */ }
  ```
  Removes the last element from the stack and returns it.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &StackRef<T> { /* ... */ }
    ```

- **Sync**
- **ForeignTypeRef**
- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: usize) -> &mut <T as >::Ref { /* ... */ }
    ```

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

- **Index**
  - ```rust
    fn index(self: &Self, index: usize) -> &<T as >::Ref { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> Iter<''a, T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> IterMut<''a, T> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &StackRef<T> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Iter`

An iterator over the stack's contents.

```rust
pub struct Iter<''a, T: Stackable> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a <T as >::Ref> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<&''a <T as >::Ref> { /* ... */ }
    ```

- **ExactSizeIterator**
- **UnwindSafe**
- **RefUnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `IterMut`

A mutable iterator over the stack's contents.

```rust
pub struct IterMut<''a, T: Stackable> {
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
- **UnwindSafe**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<&''a mut <T as >::Ref> { /* ... */ }
    ```

- **ExactSizeIterator**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

- **RefUnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a mut <T as >::Ref> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Traits

#### Trait `Stackable`

Trait implemented by types which can be placed in a stack.

It should not be implemented for any type outside of this crate.

```rust
pub trait Stackable: ForeignType {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `StackType`: The C stack type for this element.

##### Implementations

This trait is implemented for the following types:

- `Asn1Object`
- `Pkcs7SignerInfo`
- `SrtpProtectionProfile`
- `SslCipher`
- `OpensslString`
- `X509`
- `X509Extension`
- `X509Name`
- `X509Revoked`
- `GeneralName`
- `DistPoint`
- `AccessDescription`
- `X509Object`

## Module `string`

```rust
pub mod string { /* ... */ }
```

### Types

#### Struct `OpensslString`

```rust
pub struct OpensslString(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Receiver**
- **RefUnwindSafe**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &OpensslStringRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &OpensslStringRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &OpensslStringRef { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut c_char) -> OpensslString { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut c_char { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut OpensslStringRef { /* ... */ }
    ```

- **Stackable**
- **Freeze**
#### Struct `OpensslStringRef`

```rust
pub struct OpensslStringRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &OpensslStringRef { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Receiver**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &OpensslStringRef { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &str { /* ... */ }
    ```

- **Freeze**
- **ForeignTypeRef**
- **Unpin**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

## Module `symm`

High level interface to certain symmetric ciphers.

# Examples

Encrypt data in AES128 CBC mode

```
use openssl::symm::{encrypt, Cipher};

let cipher = Cipher::aes_128_cbc();
let data = b"Some Crypto Text";
let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
let ciphertext = encrypt(
    cipher,
    key,
    Some(iv),
    data).unwrap();

assert_eq!(
    b"\xB4\xB9\xE7\x30\xD6\xD6\xF7\xDE\x77\x3F\x1C\xFF\xB3\x3E\x44\x5A\x91\xD7\x27\x62\x87\x4D\
      \xFB\x3C\x5E\xC4\x59\x72\x4A\xF4\x7C\xA1",
    &ciphertext[..]);
```

Encrypting an asymmetric key with a symmetric cipher

```
use openssl::rsa::{Padding, Rsa};
use openssl::symm::Cipher;

// Generate keypair and encrypt private key:
let keypair = Rsa::generate(2048).unwrap();
let cipher = Cipher::aes_256_cbc();
let pubkey_pem = keypair.public_key_to_pem_pkcs1().unwrap();
let privkey_pem = keypair.private_key_to_pem_passphrase(cipher, b"Rust").unwrap();
// pubkey_pem and privkey_pem could be written to file here.

// Load private and public key from string:
let pubkey = Rsa::public_key_from_pem_pkcs1(&pubkey_pem).unwrap();
let privkey = Rsa::private_key_from_pem_passphrase(&privkey_pem, b"Rust").unwrap();

// Use the asymmetric keys to encrypt and decrypt a short message:
let msg = b"Foo bar";
let mut encrypted = vec![0; pubkey.size() as usize];
let mut decrypted = vec![0; privkey.size() as usize];
let len = pubkey.public_encrypt(msg, &mut encrypted, Padding::PKCS1).unwrap();
assert!(len > msg.len());
let len = privkey.private_decrypt(&encrypted, &mut decrypted, Padding::PKCS1).unwrap();
let output_string = String::from_utf8(decrypted[..len].to_vec()).unwrap();
assert_eq!("Foo bar", output_string);
println!("Decrypted: '{}'", output_string);
```

```rust
pub mod symm { /* ... */ }
```

### Types

#### Enum `Mode`

```rust
pub enum Mode {
    Encrypt,
    Decrypt,
}
```

##### Variants

###### `Encrypt`

###### `Decrypt`

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Mode { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
#### Struct `Cipher`

Represents a particular cipher algorithm.

See OpenSSL doc at [`EVP_EncryptInit`] for more information on each algorithms.

[`EVP_EncryptInit`]: https://www.openssl.org/docs/manmaster/crypto/EVP_EncryptInit.html

```rust
pub struct Cipher(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_nid(nid: Nid) -> Option<Cipher> { /* ... */ }
  ```
   Looks up the cipher for a certain nid.

- ```rust
  pub fn nid(self: &Self) -> Nid { /* ... */ }
  ```
   Returns the cipher's Nid.

- ```rust
  pub fn aes_128_ecb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_128_cbc() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_128_xts() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_128_ctr() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_128_cfb1() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_128_cfb128() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_128_cfb8() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_128_gcm() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_128_ccm() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_128_ofb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_128_ocb() -> Cipher { /* ... */ }
  ```
  Requires OpenSSL 1.1.0 or newer.

- ```rust
  pub fn aes_192_ecb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_192_cbc() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_192_ctr() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_192_cfb1() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_192_cfb128() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_192_cfb8() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_192_gcm() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_192_ccm() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_192_ofb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_192_ocb() -> Cipher { /* ... */ }
  ```
  Requires OpenSSL 1.1.0 or newer.

- ```rust
  pub fn aes_256_ecb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_256_cbc() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_256_xts() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_256_ctr() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_256_cfb1() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_256_cfb128() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_256_cfb8() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_256_gcm() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_256_ccm() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_256_ofb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn aes_256_ocb() -> Cipher { /* ... */ }
  ```
  Requires OpenSSL 1.1.0 or newer.

- ```rust
  pub fn bf_cbc() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn bf_ecb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn bf_cfb64() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn bf_ofb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn des_cbc() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn des_ecb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn des_ede3() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn des_ede3_cbc() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn des_ede3_ecb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn des_ede3_cfb64() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn des_ede3_cfb8() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn des_ede3_ofb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn rc4() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn camellia_128_cbc() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn camellia_128_ecb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn camellia_128_ofb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn camellia_128_cfb128() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn camellia_192_cbc() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn camellia_192_ecb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn camellia_192_ofb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn camellia_192_cfb128() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn camellia_256_cbc() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn camellia_256_ecb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn camellia_256_ofb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn camellia_256_cfb128() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn cast5_cbc() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn cast5_ecb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn cast5_ofb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn cast5_cfb64() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn chacha20() -> Cipher { /* ... */ }
  ```
  Requires OpenSSL 1.1.0 or newer.

- ```rust
  pub fn chacha20_poly1305() -> Cipher { /* ... */ }
  ```
  Requires OpenSSL 1.1.0 or newer.

- ```rust
  pub fn seed_cbc() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn seed_cfb128() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn seed_ecb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn seed_ofb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn sm4_ecb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn sm4_cbc() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn sm4_ctr() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn sm4_cfb128() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn sm4_ofb() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn rc2_cbc() -> Cipher { /* ... */ }
  ```

- ```rust
  pub fn rc2_40_cbc() -> Cipher { /* ... */ }
  ```

- ```rust
  pub unsafe fn from_ptr(ptr: *const ffi::EVP_CIPHER) -> Cipher { /* ... */ }
  ```
  Creates a `Cipher` from a raw pointer to its OpenSSL type.

- ```rust
  pub fn as_ptr(self: &Self) -> *const ffi::EVP_CIPHER { /* ... */ }
  ```

- ```rust
  pub fn key_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the length of keys used with this cipher.

- ```rust
  pub fn iv_len(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the length of the IV used with this cipher, or `None` if the

- ```rust
  pub fn block_size(self: &Self) -> usize { /* ... */ }
  ```
  Returns the block size of the cipher.

###### Trait Implementations

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Cipher) -> bool { /* ... */ }
    ```

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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **UnwindSafe**
- **Freeze**
- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Cipher { /* ... */ }
    ```

- **Sync**
- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
#### Struct `Crypter`

Represents a symmetric cipher context.

Padding is enabled by default.

# Examples

Encrypt some plaintext in chunks, then decrypt the ciphertext back into plaintext, in AES 128
CBC mode.

```
use openssl::symm::{Cipher, Mode, Crypter};

let plaintexts: [&[u8]; 2] = [b"Some Stream of", b" Crypto Text"];
let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
let data_len = plaintexts.iter().fold(0, |sum, x| sum + x.len());

// Create a cipher context for encryption.
let mut encrypter = Crypter::new(
    Cipher::aes_128_cbc(),
    Mode::Encrypt,
    key,
    Some(iv)).unwrap();

let block_size = Cipher::aes_128_cbc().block_size();
let mut ciphertext = vec![0; data_len + block_size];

// Encrypt 2 chunks of plaintexts successively.
let mut count = encrypter.update(plaintexts[0], &mut ciphertext).unwrap();
count += encrypter.update(plaintexts[1], &mut ciphertext[count..]).unwrap();
count += encrypter.finalize(&mut ciphertext[count..]).unwrap();
ciphertext.truncate(count);

assert_eq!(
    b"\x0F\x21\x83\x7E\xB2\x88\x04\xAF\xD9\xCC\xE2\x03\x49\xB4\x88\xF6\xC4\x61\x0E\x32\x1C\xF9\
      \x0D\x66\xB1\xE6\x2C\x77\x76\x18\x8D\x99",
    &ciphertext[..]
);


// Let's pretend we don't know the plaintext, and now decrypt the ciphertext.
let data_len = ciphertext.len();
let ciphertexts = [&ciphertext[..9], &ciphertext[9..]];

// Create a cipher context for decryption.
let mut decrypter = Crypter::new(
    Cipher::aes_128_cbc(),
    Mode::Decrypt,
    key,
    Some(iv)).unwrap();
let mut plaintext = vec![0; data_len + block_size];

// Decrypt 2 chunks of ciphertexts successively.
let mut count = decrypter.update(ciphertexts[0], &mut plaintext).unwrap();
count += decrypter.update(ciphertexts[1], &mut plaintext[count..]).unwrap();
count += decrypter.finalize(&mut plaintext[count..]).unwrap();
plaintext.truncate(count);

assert_eq!(b"Some Stream of Crypto Text", &plaintext[..]);
```

```rust
pub struct Crypter {
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
  pub fn new(t: Cipher, mode: Mode, key: &[u8], iv: Option<&[u8]>) -> Result<Crypter, ErrorStack> { /* ... */ }
  ```
  Creates a new `Crypter`.  The initialisation vector, `iv`, is not necessary for certain

- ```rust
  pub fn pad(self: &mut Self, padding: bool) { /* ... */ }
  ```
  Enables or disables padding.

- ```rust
  pub fn set_tag(self: &mut Self, tag: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Sets the tag used to authenticate ciphertext in AEAD ciphers such as AES GCM.

- ```rust
  pub fn set_tag_len(self: &mut Self, tag_len: usize) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Sets the length of the authentication tag to generate in AES CCM.

- ```rust
  pub fn set_data_len(self: &mut Self, data_len: usize) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Feeds total plaintext length to the cipher.

- ```rust
  pub fn aad_update(self: &mut Self, input: &[u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Feeds Additional Authenticated Data (AAD) through the cipher.

- ```rust
  pub fn update(self: &mut Self, input: &[u8], output: &mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Feeds data from `input` through the cipher, writing encrypted/decrypted

- ```rust
  pub unsafe fn update_unchecked(self: &mut Self, input: &[u8], output: &mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Feeds data from `input` through the cipher, writing encrypted/decrypted

- ```rust
  pub fn finalize(self: &mut Self, output: &mut [u8]) -> Result<usize, ErrorStack> { /* ... */ }
  ```
  Finishes the encryption/decryption process, writing any remaining data

- ```rust
  pub fn get_tag(self: &Self, tag: &mut [u8]) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Retrieves the authentication tag used to authenticate ciphertext in AEAD ciphers such

###### Trait Implementations

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

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
### Functions

#### Function `encrypt`

Encrypts data in one go, and returns the encrypted data.

Data is encrypted using the specified cipher type `t` in encrypt mode with the specified `key`
and initialization vector `iv`. Padding is enabled.

This is a convenient interface to `Crypter` to encrypt all data in one go.  To encrypt a stream
of data incrementally , use `Crypter` instead.

# Examples

Encrypt data in AES128 CBC mode

```
use openssl::symm::{encrypt, Cipher};

let cipher = Cipher::aes_128_cbc();
let data = b"Some Crypto Text";
let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
let ciphertext = encrypt(
    cipher,
    key,
    Some(iv),
    data).unwrap();

assert_eq!(
    b"\xB4\xB9\xE7\x30\xD6\xD6\xF7\xDE\x77\x3F\x1C\xFF\xB3\x3E\x44\x5A\x91\xD7\x27\x62\x87\x4D\
      \xFB\x3C\x5E\xC4\x59\x72\x4A\xF4\x7C\xA1",
    &ciphertext[..]);
```

```rust
pub fn encrypt(t: Cipher, key: &[u8], iv: Option<&[u8]>, data: &[u8]) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
```

#### Function `decrypt`

Decrypts data in one go, and returns the decrypted data.

Data is decrypted using the specified cipher type `t` in decrypt mode with the specified `key`
and initialization vector `iv`. Padding is enabled.

This is a convenient interface to `Crypter` to decrypt all data in one go.  To decrypt a  stream
of data incrementally , use `Crypter` instead.

# Examples

Decrypt data in AES128 CBC mode

```
use openssl::symm::{decrypt, Cipher};

let cipher = Cipher::aes_128_cbc();
let data = b"\xB4\xB9\xE7\x30\xD6\xD6\xF7\xDE\x77\x3F\x1C\xFF\xB3\x3E\x44\x5A\x91\xD7\x27\x62\
             \x87\x4D\xFB\x3C\x5E\xC4\x59\x72\x4A\xF4\x7C\xA1";
let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
let ciphertext = decrypt(
    cipher,
    key,
    Some(iv),
    data).unwrap();

assert_eq!(
    b"Some Crypto Text",
    &ciphertext[..]);
```

```rust
pub fn decrypt(t: Cipher, key: &[u8], iv: Option<&[u8]>, data: &[u8]) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
```

#### Function `encrypt_aead`

Like `encrypt`, but for AEAD ciphers such as AES GCM.

Additional Authenticated Data can be provided in the `aad` field, and the authentication tag
will be copied into the `tag` field.

The size of the `tag` buffer indicates the required size of the tag. While some ciphers support
a range of tag sizes, it is recommended to pick the maximum size. For AES GCM, this is 16 bytes,
for example.

```rust
pub fn encrypt_aead(t: Cipher, key: &[u8], iv: Option<&[u8]>, aad: &[u8], data: &[u8], tag: &mut [u8]) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
```

#### Function `decrypt_aead`

Like `decrypt`, but for AEAD ciphers such as AES GCM.

Additional Authenticated Data can be provided in the `aad` field, and the authentication tag
should be provided in the `tag` field.

```rust
pub fn decrypt_aead(t: Cipher, key: &[u8], iv: Option<&[u8]>, aad: &[u8], data: &[u8], tag: &[u8]) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
```

## Module `version`

Build and version information.

```rust
pub mod version { /* ... */ }
```

### Functions

#### Function `number`

**Attributes:**

- `#[doc(alias = "OpenSSL_version_num")]`

 OPENSSL_VERSION_NUMBER is a numeric release version identifier:

 `MNNFFPPS: major minor fix patch status`

 The status nibble has one of the values 0 for development, 1 to e for betas 1 to 14, and f for release.

 for example

 `0x000906000 == 0.9.6 dev`
 `0x000906023 == 0.9.6b beta 3`
 `0x00090605f == 0.9.6e release`

This corresponds to [`OpenSSL_version_num`](https://www.openssl.org/docs/manmaster/man3/OpenSSL_version_num.html).

```rust
pub fn number() -> i64 { /* ... */ }
```

#### Function `version`

**Attributes:**

- `#[doc(alias = "OpenSSL_version")]`

 The text variant of the version number and the release date. For example, "OpenSSL 0.9.5a 1 Apr 2000".

This corresponds to [`OpenSSL_version`](https://www.openssl.org/docs/manmaster/man3/OpenSSL_version.html).

```rust
pub fn version() -> &''static str { /* ... */ }
```

#### Function `c_flags`

**Attributes:**

- `#[doc(alias = "OpenSSL_version")]`

 The compiler flags set for the compilation process in the form "compiler: ..." if available or
 "compiler: information not available" otherwise.

This corresponds to [`OpenSSL_version`](https://www.openssl.org/docs/manmaster/man3/OpenSSL_version.html).

```rust
pub fn c_flags() -> &''static str { /* ... */ }
```

#### Function `built_on`

**Attributes:**

- `#[doc(alias = "OpenSSL_version")]`

 The date of the build process in the form "built on: ..." if available or "built on: date not available" otherwise.

This corresponds to [`OpenSSL_version`](https://www.openssl.org/docs/manmaster/man3/OpenSSL_version.html).

```rust
pub fn built_on() -> &''static str { /* ... */ }
```

#### Function `platform`

**Attributes:**

- `#[doc(alias = "OpenSSL_version")]`

 The "Configure" target of the library build in the form "platform: ..." if available or "platform: information not available" otherwise.

This corresponds to [`OpenSSL_version`](https://www.openssl.org/docs/manmaster/man3/OpenSSL_version.html).

```rust
pub fn platform() -> &''static str { /* ... */ }
```

#### Function `dir`

**Attributes:**

- `#[doc(alias = "OpenSSL_version")]`

 The "OPENSSLDIR" setting of the library build in the form "OPENSSLDIR: "..."" if available or "OPENSSLDIR: N/A" otherwise.

This corresponds to [`OpenSSL_version`](https://www.openssl.org/docs/manmaster/man3/OpenSSL_version.html).

```rust
pub fn dir() -> &''static str { /* ... */ }
```

## Module `x509`

The standard defining the format of public key certificates.

An `X509` certificate binds an identity to a public key, and is either
signed by a certificate authority (CA) or self-signed. An entity that gets
a hold of a certificate can both verify your identity (via a CA) and encrypt
data with the included public key. `X509` certificates are used in many
Internet protocols, including SSL/TLS, which is the basis for HTTPS,
the secure protocol for browsing the web.

```rust
pub mod x509 { /* ... */ }
```

### Modules

## Module `verify`

**Attributes:**

- `#[<cfg>(any(ossl102, boringssl, libressl261, awslc))]`

```rust
pub mod verify { /* ... */ }
```

### Types

#### Struct `X509CheckFlags`

Flags used to check an `X509` certificate.

```rust
pub struct X509CheckFlags(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> c_uint { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: c_uint) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: c_uint) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: c_uint) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<X509CheckFlags> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<X509CheckFlags> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Flags**
  - ```rust
    fn bits(self: &Self) -> c_uint { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: c_uint) -> X509CheckFlags { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Eq**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> X509CheckFlags { /* ... */ }
    ```

- **Send**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &X509CheckFlags) -> bool { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
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

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &X509CheckFlags) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PublicFlags**
- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &X509CheckFlags) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: X509CheckFlags) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Copy**
#### Struct `X509VerifyFlags`

Flags used to verify an `X509` certificate chain.

```rust
pub struct X509VerifyFlags(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> c_ulong { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: c_ulong) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: c_ulong) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: c_ulong) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags value (`&!`).

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<X509VerifyFlags> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<X509VerifyFlags> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **PublicFlags**
- **StructuralPartialEq**
- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> X509VerifyFlags { /* ... */ }
    ```

- **Copy**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Flags**
  - ```rust
    fn bits(self: &Self) -> c_ulong { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: c_ulong) -> X509VerifyFlags { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &X509VerifyFlags) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: X509VerifyFlags) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &X509VerifyFlags) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &X509VerifyFlags) -> bool { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `X509VerifyParam`

Adjust parameters associated with certificate verification.

```rust
pub struct X509VerifyParam(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Result<X509VerifyParam, ErrorStack> { /* ... */ }
  ```
   Create an X509VerifyParam

###### Trait Implementations

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509_VERIFY_PARAM) -> X509VerifyParam { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509_VERIFY_PARAM { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509VerifyParamRef { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Receiver**
- **Send**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509VerifyParamRef { /* ... */ }
    ```

- **Freeze**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Unpin**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509VerifyParamRef { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509VerifyParamRef { /* ... */ }
    ```

#### Struct `X509VerifyParamRef`

Reference to `X509VerifyParam`.

```rust
pub struct X509VerifyParamRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn set_hostflags(self: &mut Self, hostflags: X509CheckFlags) { /* ... */ }
  ```
   Set the host flags.

- ```rust
  pub fn set_flags(self: &mut Self, flags: X509VerifyFlags) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Set verification flags.

- ```rust
  pub fn clear_flags(self: &mut Self, flags: X509VerifyFlags) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Clear verification flags.

- ```rust
  pub fn flags(self: &mut Self) -> X509VerifyFlags { /* ... */ }
  ```
   Gets verification flags.

- ```rust
  pub fn set_host(self: &mut Self, host: &str) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Set the expected DNS hostname.

- ```rust
  pub fn set_email(self: &mut Self, email: &str) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Set the expected email address.

- ```rust
  pub fn set_ip(self: &mut Self, ip: IpAddr) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Set the expected IPv4 or IPv6 address.

- ```rust
  pub fn set_time(self: &mut Self, time: time_t) { /* ... */ }
  ```
   Set the verification time, where time is of type time_t, traditionaly defined as seconds since the epoch

- ```rust
  pub fn set_depth(self: &mut Self, depth: c_int) { /* ... */ }
  ```
   Set the verification depth

- ```rust
  pub fn set_auth_level(self: &mut Self, lvl: c_int) { /* ... */ }
  ```
   Sets the authentication security level to auth_level

- ```rust
  pub fn auth_level(self: &Self) -> i32 { /* ... */ }
  ```
   Gets the current authentication security level

- ```rust
  pub fn set_purpose(self: &mut Self, purpose: X509PurposeId) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the verification purpose

###### Trait Implementations

- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

  - ```rust
    fn borrow(self: &Self) -> &X509VerifyParamRef { /* ... */ }
    ```

- **Send**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509VerifyParamRef { /* ... */ }
    ```

- **Sync**
- **ForeignTypeRef**
## Module `extension`

Add extensions to an `X509` certificate or certificate request.

The extensions defined for X.509 v3 certificates provide methods for
associating additional attributes with users or public keys and for
managing relationships between CAs. The extensions created using this
module can be used with `X509v3Context` objects.

# Example

```rust
use openssl::x509::extension::BasicConstraints;
use openssl::x509::X509Extension;

let mut bc = BasicConstraints::new();
let bc = bc.critical().ca().pathlen(1);

let extension: X509Extension = bc.build().unwrap();
```

```rust
pub mod extension { /* ... */ }
```

### Types

#### Struct `BasicConstraints`

An extension which indicates whether a certificate is a CA certificate.

```rust
pub struct BasicConstraints {
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
  pub fn new() -> BasicConstraints { /* ... */ }
  ```
  Construct a new `BasicConstraints` extension.

- ```rust
  pub fn critical(self: &mut Self) -> &mut BasicConstraints { /* ... */ }
  ```
  Sets the `critical` flag to `true`. The extension will be critical.

- ```rust
  pub fn ca(self: &mut Self) -> &mut BasicConstraints { /* ... */ }
  ```
  Sets the `ca` flag to `true`.

- ```rust
  pub fn pathlen(self: &mut Self, pathlen: u32) -> &mut BasicConstraints { /* ... */ }
  ```
  Sets the `pathlen` to an optional non-negative value. The `pathlen` is the

- ```rust
  pub fn build(self: &Self) -> Result<X509Extension, ErrorStack> { /* ... */ }
  ```
  Return the `BasicConstraints` extension as an `X509Extension`.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> BasicConstraints { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Sync**
- **Unpin**
#### Struct `KeyUsage`

An extension consisting of a list of names of the permitted key usages.

```rust
pub struct KeyUsage {
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
  pub fn new() -> KeyUsage { /* ... */ }
  ```
  Construct a new `KeyUsage` extension.

- ```rust
  pub fn critical(self: &mut Self) -> &mut KeyUsage { /* ... */ }
  ```
  Sets the `critical` flag to `true`. The extension will be critical.

- ```rust
  pub fn digital_signature(self: &mut Self) -> &mut KeyUsage { /* ... */ }
  ```
  Sets the `digitalSignature` flag to `true`.

- ```rust
  pub fn non_repudiation(self: &mut Self) -> &mut KeyUsage { /* ... */ }
  ```
  Sets the `nonRepudiation` flag to `true`.

- ```rust
  pub fn key_encipherment(self: &mut Self) -> &mut KeyUsage { /* ... */ }
  ```
  Sets the `keyEncipherment` flag to `true`.

- ```rust
  pub fn data_encipherment(self: &mut Self) -> &mut KeyUsage { /* ... */ }
  ```
  Sets the `dataEncipherment` flag to `true`.

- ```rust
  pub fn key_agreement(self: &mut Self) -> &mut KeyUsage { /* ... */ }
  ```
  Sets the `keyAgreement` flag to `true`.

- ```rust
  pub fn key_cert_sign(self: &mut Self) -> &mut KeyUsage { /* ... */ }
  ```
  Sets the `keyCertSign` flag to `true`.

- ```rust
  pub fn crl_sign(self: &mut Self) -> &mut KeyUsage { /* ... */ }
  ```
  Sets the `cRLSign` flag to `true`.

- ```rust
  pub fn encipher_only(self: &mut Self) -> &mut KeyUsage { /* ... */ }
  ```
  Sets the `encipherOnly` flag to `true`.

- ```rust
  pub fn decipher_only(self: &mut Self) -> &mut KeyUsage { /* ... */ }
  ```
  Sets the `decipherOnly` flag to `true`.

- ```rust
  pub fn build(self: &Self) -> Result<X509Extension, ErrorStack> { /* ... */ }
  ```
  Return the `KeyUsage` extension as an `X509Extension`.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> KeyUsage { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
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

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Sync**
#### Struct `ExtendedKeyUsage`

An extension consisting of a list of usages indicating purposes
for which the certificate public key can be used for.

```rust
pub struct ExtendedKeyUsage {
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
  pub fn new() -> ExtendedKeyUsage { /* ... */ }
  ```
  Construct a new `ExtendedKeyUsage` extension.

- ```rust
  pub fn critical(self: &mut Self) -> &mut ExtendedKeyUsage { /* ... */ }
  ```
  Sets the `critical` flag to `true`. The extension will be critical.

- ```rust
  pub fn server_auth(self: &mut Self) -> &mut ExtendedKeyUsage { /* ... */ }
  ```
  Sets the `serverAuth` flag to `true`.

- ```rust
  pub fn client_auth(self: &mut Self) -> &mut ExtendedKeyUsage { /* ... */ }
  ```
  Sets the `clientAuth` flag to `true`.

- ```rust
  pub fn code_signing(self: &mut Self) -> &mut ExtendedKeyUsage { /* ... */ }
  ```
  Sets the `codeSigning` flag to `true`.

- ```rust
  pub fn email_protection(self: &mut Self) -> &mut ExtendedKeyUsage { /* ... */ }
  ```
  Sets the `emailProtection` flag to `true`.

- ```rust
  pub fn time_stamping(self: &mut Self) -> &mut ExtendedKeyUsage { /* ... */ }
  ```
  Sets the `timeStamping` flag to `true`.

- ```rust
  pub fn ms_code_ind(self: &mut Self) -> &mut ExtendedKeyUsage { /* ... */ }
  ```
  Sets the `msCodeInd` flag to `true`.

- ```rust
  pub fn ms_code_com(self: &mut Self) -> &mut ExtendedKeyUsage { /* ... */ }
  ```
  Sets the `msCodeCom` flag to `true`.

- ```rust
  pub fn ms_ctl_sign(self: &mut Self) -> &mut ExtendedKeyUsage { /* ... */ }
  ```
  Sets the `msCTLSign` flag to `true`.

- ```rust
  pub fn ms_sgc(self: &mut Self) -> &mut ExtendedKeyUsage { /* ... */ }
  ```
  Sets the `msSGC` flag to `true`.

- ```rust
  pub fn ms_efs(self: &mut Self) -> &mut ExtendedKeyUsage { /* ... */ }
  ```
  Sets the `msEFS` flag to `true`.

- ```rust
  pub fn ns_sgc(self: &mut Self) -> &mut ExtendedKeyUsage { /* ... */ }
  ```
  Sets the `nsSGC` flag to `true`.

- ```rust
  pub fn other(self: &mut Self, other: &str) -> &mut ExtendedKeyUsage { /* ... */ }
  ```
  Sets a flag not already defined.

- ```rust
  pub fn build(self: &Self) -> Result<X509Extension, ErrorStack> { /* ... */ }
  ```
  Return the `ExtendedKeyUsage` extension as an `X509Extension`.

###### Trait Implementations

- **Freeze**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ExtendedKeyUsage { /* ... */ }
    ```

#### Struct `SubjectKeyIdentifier`

An extension that provides a means of identifying certificates that contain a
particular public key.

```rust
pub struct SubjectKeyIdentifier {
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
  pub fn new() -> SubjectKeyIdentifier { /* ... */ }
  ```
  Construct a new `SubjectKeyIdentifier` extension.

- ```rust
  pub fn critical(self: &mut Self) -> &mut SubjectKeyIdentifier { /* ... */ }
  ```
  Sets the `critical` flag to `true`. The extension will be critical.

- ```rust
  pub fn build(self: &Self, ctx: &X509v3Context<''_>) -> Result<X509Extension, ErrorStack> { /* ... */ }
  ```
  Return a `SubjectKeyIdentifier` extension as an `X509Extension`.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> SubjectKeyIdentifier { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `AuthorityKeyIdentifier`

An extension that provides a means of identifying the public key corresponding
to the private key used to sign a CRL.

```rust
pub struct AuthorityKeyIdentifier {
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
  pub fn new() -> AuthorityKeyIdentifier { /* ... */ }
  ```
  Construct a new `AuthorityKeyIdentifier` extension.

- ```rust
  pub fn critical(self: &mut Self) -> &mut AuthorityKeyIdentifier { /* ... */ }
  ```
  Sets the `critical` flag to `true`. The extension will be critical.

- ```rust
  pub fn keyid(self: &mut Self, always: bool) -> &mut AuthorityKeyIdentifier { /* ... */ }
  ```
  Sets the `keyid` flag.

- ```rust
  pub fn issuer(self: &mut Self, always: bool) -> &mut AuthorityKeyIdentifier { /* ... */ }
  ```
  Sets the `issuer` flag.

- ```rust
  pub fn build(self: &Self, ctx: &X509v3Context<''_>) -> Result<X509Extension, ErrorStack> { /* ... */ }
  ```
  Return a `AuthorityKeyIdentifier` extension as an `X509Extension`.

###### Trait Implementations

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

- **Freeze**
- **Sync**
- **Default**
  - ```rust
    fn default() -> AuthorityKeyIdentifier { /* ... */ }
    ```

- **Send**
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

#### Struct `SubjectAlternativeName`

An extension that allows additional identities to be bound to the subject
of the certificate.

```rust
pub struct SubjectAlternativeName {
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
  pub fn new() -> SubjectAlternativeName { /* ... */ }
  ```
  Construct a new `SubjectAlternativeName` extension.

- ```rust
  pub fn critical(self: &mut Self) -> &mut SubjectAlternativeName { /* ... */ }
  ```
  Sets the `critical` flag to `true`. The extension will be critical.

- ```rust
  pub fn email(self: &mut Self, email: &str) -> &mut SubjectAlternativeName { /* ... */ }
  ```
  Sets the `email` flag.

- ```rust
  pub fn uri(self: &mut Self, uri: &str) -> &mut SubjectAlternativeName { /* ... */ }
  ```
  Sets the `uri` flag.

- ```rust
  pub fn dns(self: &mut Self, dns: &str) -> &mut SubjectAlternativeName { /* ... */ }
  ```
  Sets the `dns` flag.

- ```rust
  pub fn rid(self: &mut Self, rid: &str) -> &mut SubjectAlternativeName { /* ... */ }
  ```
  Sets the `rid` flag.

- ```rust
  pub fn ip(self: &mut Self, ip: &str) -> &mut SubjectAlternativeName { /* ... */ }
  ```
  Sets the `ip` flag.

- ```rust
  pub fn dir_name(self: &mut Self, _dir_name: &str) -> &mut SubjectAlternativeName { /* ... */ }
  ```
  Sets the `dirName` flag.

- ```rust
  pub fn other_name(self: &mut Self, _other_name: &str) -> &mut SubjectAlternativeName { /* ... */ }
  ```
  Sets the `otherName` flag.

- ```rust
  pub fn other_name2(self: &mut Self, oid: Asn1Object, content: &[u8]) -> &mut SubjectAlternativeName { /* ... */ }
  ```
  Sets the `otherName` flag.

- ```rust
  pub fn build(self: &Self, _ctx: &X509v3Context<''_>) -> Result<X509Extension, ErrorStack> { /* ... */ }
  ```
  Return a `SubjectAlternativeName` extension as an `X509Extension`.

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> SubjectAlternativeName { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
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

- **Sync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `store`

Describe a context in which to verify an `X509` certificate.

The `X509` certificate store holds trusted CA certificates used to verify
peer certificates.

# Example

```rust
use openssl::x509::store::{X509StoreBuilder, X509Store};
use openssl::x509::{X509, X509Name};
use openssl::asn1::Asn1Time;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use openssl::rsa::Rsa;
use openssl::nid::Nid;

let rsa = Rsa::generate(2048).unwrap();
let pkey = PKey::from_rsa(rsa).unwrap();

let mut name = X509Name::builder().unwrap();
name.append_entry_by_nid(Nid::COMMONNAME, "foobar.com").unwrap();
let name = name.build();

// Sep 27th, 2016
let sample_time = Asn1Time::from_unix(1474934400).unwrap();

let mut builder = X509::builder().unwrap();
builder.set_version(2).unwrap();
builder.set_subject_name(&name).unwrap();
builder.set_issuer_name(&name).unwrap();
builder.set_pubkey(&pkey).unwrap();
builder.set_not_before(&sample_time);
builder.set_not_after(&sample_time);
builder.sign(&pkey, MessageDigest::sha256()).unwrap();

let certificate: X509 = builder.build();

let mut builder = X509StoreBuilder::new().unwrap();
let _ = builder.add_cert(certificate);

let store: X509Store = builder.build();
```

```rust
pub mod store { /* ... */ }
```

### Types

#### Struct `X509StoreBuilder`

A builder type used to construct an `X509Store`.

```rust
pub struct X509StoreBuilder(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Result<X509StoreBuilder, ErrorStack> { /* ... */ }
  ```
   Returns a builder for a certificate store.

- ```rust
  pub fn build(self: Self) -> X509Store { /* ... */ }
  ```
  Constructs the `X509Store`.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509StoreBuilderRef { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **RefUnwindSafe**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509StoreBuilderRef { /* ... */ }
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

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509StoreBuilderRef { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509StoreBuilderRef { /* ... */ }
    ```

- **Unpin**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509_STORE) -> X509StoreBuilder { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509_STORE { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Receiver**
#### Struct `X509StoreBuilderRef`

A reference to an [`X509StoreBuilder`].

```rust
pub struct X509StoreBuilderRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn add_cert(self: &mut Self, cert: X509) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Adds a certificate to the certificate store.

- ```rust
  pub fn set_default_paths(self: &mut Self) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Load certificates from their default locations.

- ```rust
  pub fn add_lookup<T>(self: &mut Self, method: &''static X509LookupMethodRef<T>) -> Result<&mut X509LookupRef<T>, ErrorStack> { /* ... */ }
  ```
   Adds a lookup method to the store.

- ```rust
  pub fn set_flags(self: &mut Self, flags: X509VerifyFlags) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets certificate chain validation related flags.

- ```rust
  pub fn set_purpose(self: &mut Self, purpose: X509PurposeId) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the certificate purpose.

- ```rust
  pub fn set_param(self: &mut Self, param: &X509VerifyParamRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets certificate chain validation related parameters.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Freeze**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509StoreBuilderRef { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **ForeignTypeRef**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509StoreBuilderRef { /* ... */ }
    ```

#### Struct `X509Lookup`

Information used by an `X509Store` to look up certificates and CRLs.

```rust
pub struct X509Lookup<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn hash_dir() -> &''static X509LookupMethodRef<HashDir> { /* ... */ }
  ```
   Lookup method that loads certificates and CRLs on demand and caches

- ```rust
  pub fn file() -> &''static X509LookupMethodRef<File> { /* ... */ }
  ```
   Lookup method loads all the certificates or CRLs present in a file

###### Trait Implementations

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509LookupRef<T> { /* ... */ }
    ```

- **UnwindSafe**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509LookupRef<T> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Receiver**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509LookupRef<T> { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509LookupRef<T> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
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

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509_LOOKUP) -> X509Lookup<T> { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509_LOOKUP { /* ... */ }
    ```

- **Send**
#### Struct `X509LookupRef`

A reference to an [`X509Lookup`].

```rust
pub struct X509LookupRef<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn add_dir(self: &mut Self, name: &str, file_type: SslFiletype) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Specifies a directory from which certificates and CRLs will be loaded

- ```rust
  pub fn load_cert_file<P: AsRef<Path>>(self: &mut Self, file: P, file_type: SslFiletype) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Specifies a file from which certificates will be loaded

- ```rust
  pub fn load_crl_file<P: AsRef<Path>>(self: &mut Self, file: P, file_type: SslFiletype) -> Result<i32, ErrorStack> { /* ... */ }
  ```
   Specifies a file from which certificate revocation lists will be loaded

###### Trait Implementations

- **Sync**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509LookupRef<T> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509LookupRef<T> { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ForeignTypeRef**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `HashDir`

Marker type corresponding to the [`X509_LOOKUP_hash_dir`] lookup method.

[`X509_LOOKUP_hash_dir`]: https://www.openssl.org/docs/manmaster/crypto/X509_LOOKUP_hash_dir.html

```rust
pub struct HashDir;
```

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **RefUnwindSafe**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
#### Struct `File`

Marker type corresponding to the [`X509_LOOKUP_file`] lookup method.

[`X509_LOOKUP_file`]: https://www.openssl.org/docs/man1.1.1/man3/X509_LOOKUP_file.html

```rust
pub struct File;
```

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **RefUnwindSafe**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `X509LookupMethod`

Method used to look up certificates and CRLs.

```rust
pub struct X509LookupMethod<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509LookupMethodRef<T> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Receiver**
- **UnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509LookupMethodRef<T> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509LookupMethodRef<T> { /* ... */ }
    ```

- **Freeze**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509_LOOKUP_METHOD) -> X509LookupMethod<T> { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509_LOOKUP_METHOD { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509LookupMethodRef<T> { /* ... */ }
    ```

#### Struct `X509LookupMethodRef`

A reference to an [`X509LookupMethod`].

```rust
pub struct X509LookupMethodRef<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ForeignTypeRef**
- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509LookupMethodRef<T> { /* ... */ }
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

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509LookupMethodRef<T> { /* ... */ }
    ```

#### Struct `X509Store`

A certificate store to hold trusted `X509` certificates.

```rust
pub struct X509Store(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509StoreRef { /* ... */ }
    ```

- **Unpin**
- **Receiver**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509StoreRef { /* ... */ }
    ```

- **RefUnwindSafe**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509_STORE) -> X509Store { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509_STORE { /* ... */ }
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

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509StoreRef { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509StoreRef { /* ... */ }
    ```

#### Struct `X509StoreRef`

Reference to an `X509Store`.

```rust
pub struct X509StoreRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn objects(self: &Self) -> &StackRef<X509Object> { /* ... */ }
  ```
   Get a reference to the cache of certificates in this store.

- ```rust
  pub fn all_certificates(self: &Self) -> Stack<X509> { /* ... */ }
  ```
   Returns a stack of all the certificates in this store.

###### Trait Implementations

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **ForeignTypeRef**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Sync**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509StoreRef { /* ... */ }
    ```

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509StoreRef { /* ... */ }
    ```

### Types

#### Struct `X509StoreContext`

An `X509` certificate store context.

```rust
pub struct X509StoreContext(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn ssl_idx() -> Result<Index<X509StoreContext, SslRef>, ErrorStack> { /* ... */ }
  ```
   Returns the index which can be used to obtain a reference to the `Ssl` associated with a

- ```rust
  pub fn new() -> Result<X509StoreContext, ErrorStack> { /* ... */ }
  ```
   Creates a new `X509StoreContext` instance.

###### Trait Implementations

- **Receiver**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509StoreContextRef { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509StoreContextRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509StoreContextRef { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509StoreContextRef { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509_STORE_CTX) -> X509StoreContext { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509_STORE_CTX { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
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

#### Struct `X509StoreContextRef`

A reference to an [`X509StoreContext`].

```rust
pub struct X509StoreContextRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn ex_data<T>(self: &Self, index: Index<X509StoreContext, T>) -> Option<&T> { /* ... */ }
  ```
   Returns application data pertaining to an `X509` store context.

- ```rust
  pub fn error(self: &Self) -> X509VerifyResult { /* ... */ }
  ```
   Returns the error code of the context.

- ```rust
  pub fn init<F, T>(self: &mut Self, trust: &store::X509StoreRef, cert: &X509Ref, cert_chain: &StackRef<X509>, with_context: F) -> Result<T, ErrorStack>
where
    F: FnOnce(&mut X509StoreContextRef) -> Result<T, ErrorStack> { /* ... */ }
  ```
  Initializes this context with the given certificate, certificates chain and certificate

- ```rust
  pub fn verify_cert(self: &mut Self) -> Result<bool, ErrorStack> { /* ... */ }
  ```
   Verifies the stored certificate.

- ```rust
  pub fn set_error(self: &mut Self, result: X509VerifyResult) { /* ... */ }
  ```
   Set the error code of the context.

- ```rust
  pub fn current_cert(self: &Self) -> Option<&X509Ref> { /* ... */ }
  ```
   Returns a reference to the certificate which caused the error or None if

- ```rust
  pub fn error_depth(self: &Self) -> u32 { /* ... */ }
  ```
   Returns a non-negative integer representing the depth in the certificate

- ```rust
  pub fn chain(self: &Self) -> Option<&StackRef<X509>> { /* ... */ }
  ```
   Returns a reference to a complete valid `X509` certificate chain.

###### Trait Implementations

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

- **Unpin**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509StoreContextRef { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509StoreContextRef { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ForeignTypeRef**
- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `X509Builder`

A builder used to construct an `X509`.

```rust
pub struct X509Builder(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Result<X509Builder, ErrorStack> { /* ... */ }
  ```
   Creates a new builder.

- ```rust
  pub fn set_not_after(self: &mut Self, not_after: &Asn1TimeRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the notAfter constraint on the certificate.

- ```rust
  pub fn set_not_before(self: &mut Self, not_before: &Asn1TimeRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the notBefore constraint on the certificate.

- ```rust
  pub fn set_version(self: &mut Self, version: i32) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the version of the certificate.

- ```rust
  pub fn set_serial_number(self: &mut Self, serial_number: &Asn1IntegerRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the serial number of the certificate.

- ```rust
  pub fn set_issuer_name(self: &mut Self, issuer_name: &X509NameRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the issuer name of the certificate.

- ```rust
  pub fn set_subject_name(self: &mut Self, subject_name: &X509NameRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Sets the subject name of the certificate.

- ```rust
  pub fn set_pubkey<T>(self: &mut Self, key: &PKeyRef<T>) -> Result<(), ErrorStack>
where
    T: HasPublic { /* ... */ }
  ```
   Sets the public key associated with the certificate.

- ```rust
  pub fn x509v3_context<''a>(self: &''a Self, issuer: Option<&''a X509Ref>, conf: Option<&''a ConfRef>) -> X509v3Context<''a> { /* ... */ }
  ```
   Returns a context object which is needed to create certain X509 extension values.

- ```rust
  pub fn append_extension(self: &mut Self, extension: X509Extension) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Adds an X509 extension value to the certificate.

- ```rust
  pub fn append_extension2(self: &mut Self, extension: &X509ExtensionRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Adds an X509 extension value to the certificate.

- ```rust
  pub fn sign<T>(self: &mut Self, key: &PKeyRef<T>, hash: MessageDigest) -> Result<(), ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
   Signs the certificate with a private key.

- ```rust
  pub fn build(self: Self) -> X509 { /* ... */ }
  ```
  Consumes the builder, returning the certificate.

###### Trait Implementations

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
- **Freeze**
#### Struct `X509`

An `X509` public key certificate.

```rust
pub struct X509(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn builder() -> Result<X509Builder, ErrorStack> { /* ... */ }
  ```
  Returns a new builder.

- ```rust
  pub fn from_pem(pem: &[u8]) -> Result<X509, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a PEM-encoded X509 structure.

- ```rust
  pub fn from_der(der: &[u8]) -> Result<X509, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a DER-encoded X509 structure.

- ```rust
  pub fn stack_from_pem(pem: &[u8]) -> Result<Vec<X509>, ErrorStack> { /* ... */ }
  ```
   Deserializes a list of PEM-formatted certificates.

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &X509) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &X509Ref) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509Ref { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Receiver**
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

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509) -> X509 { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509 { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509Ref { /* ... */ }
    ```

- **Stackable**
- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509Ref { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> X509 { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509Ref { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Send**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &X509) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &X509Ref) -> Option<cmp::Ordering> { /* ... */ }
    ```

#### Struct `X509Ref`

Reference to `X509`.

```rust
pub struct X509Ref(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn subject_name(self: &Self) -> &X509NameRef { /* ... */ }
  ```
   Returns this certificate's subject name.

- ```rust
  pub fn subject_name_hash(self: &Self) -> u32 { /* ... */ }
  ```
   Returns the hash of the certificates subject

- ```rust
  pub fn issuer_name(self: &Self) -> &X509NameRef { /* ... */ }
  ```
   Returns this certificate's issuer name.

- ```rust
  pub fn issuer_name_hash(self: &Self) -> u32 { /* ... */ }
  ```
   Returns the hash of the certificates issuer

- ```rust
  pub fn subject_alt_names(self: &Self) -> Option<Stack<GeneralName>> { /* ... */ }
  ```
   Returns this certificate's subject alternative name entries, if they exist.

- ```rust
  pub fn crl_distribution_points(self: &Self) -> Option<Stack<DistPoint>> { /* ... */ }
  ```
   Returns this certificate's CRL distribution points, if they exist.

- ```rust
  pub fn issuer_alt_names(self: &Self) -> Option<Stack<GeneralName>> { /* ... */ }
  ```
   Returns this certificate's issuer alternative name entries, if they exist.

- ```rust
  pub fn authority_info(self: &Self) -> Option<Stack<AccessDescription>> { /* ... */ }
  ```
   Returns this certificate's [`authority information access`] entries, if they exist.

- ```rust
  pub fn pathlen(self: &Self) -> Option<u32> { /* ... */ }
  ```
   Retrieves the path length extension from a certificate, if it exists.

- ```rust
  pub fn subject_key_id(self: &Self) -> Option<&Asn1OctetStringRef> { /* ... */ }
  ```
   Returns this certificate's subject key id, if it exists.

- ```rust
  pub fn authority_key_id(self: &Self) -> Option<&Asn1OctetStringRef> { /* ... */ }
  ```
   Returns this certificate's authority key id, if it exists.

- ```rust
  pub fn authority_issuer(self: &Self) -> Option<&StackRef<GeneralName>> { /* ... */ }
  ```
   Returns this certificate's authority issuer name entries, if they exist.

- ```rust
  pub fn authority_serial(self: &Self) -> Option<&Asn1IntegerRef> { /* ... */ }
  ```
   Returns this certificate's authority serial number, if it exists.

- ```rust
  pub fn public_key(self: &Self) -> Result<PKey<Public>, ErrorStack> { /* ... */ }
  ```

- ```rust
  pub fn digest(self: &Self, hash_type: MessageDigest) -> Result<DigestBytes, ErrorStack> { /* ... */ }
  ```
   Returns a digest of the DER representation of the certificate.

- ```rust
  pub fn fingerprint(self: &Self, hash_type: MessageDigest) -> Result<Vec<u8>, ErrorStack> { /* ... */ }
  ```

- ```rust
  pub fn not_after(self: &Self) -> &Asn1TimeRef { /* ... */ }
  ```
   Returns the certificate's Not After validity period.

- ```rust
  pub fn not_before(self: &Self) -> &Asn1TimeRef { /* ... */ }
  ```
   Returns the certificate's Not Before validity period.

- ```rust
  pub fn signature(self: &Self) -> &Asn1BitStringRef { /* ... */ }
  ```
   Returns the certificate's signature

- ```rust
  pub fn signature_algorithm(self: &Self) -> &X509AlgorithmRef { /* ... */ }
  ```
   Returns the certificate's signature algorithm.

- ```rust
  pub fn ocsp_responders(self: &Self) -> Result<Stack<OpensslString>, ErrorStack> { /* ... */ }
  ```
   Returns the list of OCSP responder URLs specified in the certificate's Authority Information

- ```rust
  pub fn issued(self: &Self, subject: &X509Ref) -> X509VerifyResult { /* ... */ }
  ```
   Checks that this certificate issued `subject`.

- ```rust
  pub fn version(self: &Self) -> i32 { /* ... */ }
  ```
   Returns certificate version. If this certificate has no explicit version set, it defaults to

- ```rust
  pub fn verify<T>(self: &Self, key: &PKeyRef<T>) -> Result<bool, ErrorStack>
where
    T: HasPublic { /* ... */ }
  ```
   Check if the certificate is signed using the given public key.

- ```rust
  pub fn serial_number(self: &Self) -> &Asn1IntegerRef { /* ... */ }
  ```
   Returns this certificate's serial number.

- ```rust
  pub fn alias(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
   Returns this certificate's "alias". This field is populated by

- ```rust
  pub fn to_pem(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the certificate into a PEM-encoded X509 structure.

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the certificate into a DER-encoded X509 structure.

- ```rust
  pub fn to_text(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Converts the certificate to human readable text.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
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

- **UnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> X509 { /* ... */ }
    ```

- **ForeignTypeRef**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509Ref { /* ... */ }
    ```

- **Sync**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509Ref { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &X509Ref { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &X509) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &X509Ref) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &X509) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &X509Ref) -> bool { /* ... */ }
    ```

- **Freeze**
#### Struct `X509v3Context`

A context object required to construct certain `X509` extension values.

```rust
pub struct X509v3Context<''a>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn as_ptr(self: &Self) -> *mut ffi::X509V3_CTX { /* ... */ }
  ```

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Send**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `X509Extension`

Permit additional fields to be added to an `X509` v3 certificate.

```rust
pub struct X509Extension(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(conf: Option<&ConfRef>, context: Option<&X509v3Context<''_>>, name: &str, value: &str) -> Result<X509Extension, ErrorStack> { /* ... */ }
  ```
  Constructs an X509 extension value. See `man x509v3_config` for information on supported

- ```rust
  pub fn new_nid(conf: Option<&ConfRef>, context: Option<&X509v3Context<''_>>, name: Nid, value: &str) -> Result<X509Extension, ErrorStack> { /* ... */ }
  ```
  Constructs an X509 extension value. See `man x509v3_config` for information on supported

- ```rust
  pub fn new_from_der(oid: &Asn1ObjectRef, critical: bool, der_contents: &Asn1OctetStringRef) -> Result<X509Extension, ErrorStack> { /* ... */ }
  ```
  Constructs a new X509 extension value from its OID, whether it's

- ```rust
  pub unsafe fn add_alias(to: Nid, from: Nid) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Adds an alias for an extension

###### Trait Implementations

- **Stackable**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509ExtensionRef { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509ExtensionRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509ExtensionRef { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Receiver**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Freeze**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509_EXTENSION) -> X509Extension { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509_EXTENSION { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509ExtensionRef { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
#### Struct `X509ExtensionRef`

Reference to `X509Extension`.

```rust
pub struct X509ExtensionRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the Extension to its standard DER encoding.

###### Trait Implementations

- **ForeignTypeRef**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509ExtensionRef { /* ... */ }
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

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Sync**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509ExtensionRef { /* ... */ }
    ```

#### Struct `X509NameBuilder`

A builder used to construct an `X509Name`.

```rust
pub struct X509NameBuilder(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Result<X509NameBuilder, ErrorStack> { /* ... */ }
  ```
  Creates a new builder.

- ```rust
  pub fn append_entry(self: &mut Self, ne: &X509NameEntryRef) -> std::result::Result<(), ErrorStack> { /* ... */ }
  ```
   Add a name entry

- ```rust
  pub fn append_entry_by_text(self: &mut Self, field: &str, value: &str) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Add a field entry by str.

- ```rust
  pub fn append_entry_by_text_with_type(self: &mut Self, field: &str, value: &str, ty: Asn1Type) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Add a field entry by str with a specific type.

- ```rust
  pub fn append_entry_by_nid(self: &mut Self, field: Nid, value: &str) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Add a field entry by NID.

- ```rust
  pub fn append_entry_by_nid_with_type(self: &mut Self, field: Nid, value: &str, ty: Asn1Type) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Add a field entry by NID with a specific type.

- ```rust
  pub fn build(self: Self) -> X509Name { /* ... */ }
  ```
  Return an `X509Name`.

###### Trait Implementations

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
#### Struct `X509Name`

The names of an `X509` certificate.

```rust
pub struct X509Name(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn builder() -> Result<X509NameBuilder, ErrorStack> { /* ... */ }
  ```
  Returns a new builder.

- ```rust
  pub fn load_client_ca_file<P: AsRef<Path>>(file: P) -> Result<Stack<X509Name>, ErrorStack> { /* ... */ }
  ```
  Loads subject names from a file containing PEM-formatted certificates.

- ```rust
  pub fn from_der(der: &[u8]) -> Result<X509Name, crate::error::ErrorStack> { /* ... */ }
  ```
  Deserializes a DER-encoded X509 name structure.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509NameRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509NameRef { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509NameRef { /* ... */ }
    ```

- **Receiver**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509_NAME) -> X509Name { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509_NAME { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509NameRef { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Stackable**
#### Struct `X509NameRef`

Reference to `X509Name`.

```rust
pub struct X509NameRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn entries_by_nid(self: &Self, nid: Nid) -> X509NameEntries<''_> { /* ... */ }
  ```
  Returns the name entries by the nid.

- ```rust
  pub fn entries(self: &Self) -> X509NameEntries<''_> { /* ... */ }
  ```
  Returns an iterator over all `X509NameEntry` values

- ```rust
  pub fn try_cmp(self: &Self, other: &X509NameRef) -> Result<Ordering, ErrorStack> { /* ... */ }
  ```
   Compare two names, like [`Ord`] but it may fail.

- ```rust
  pub fn to_owned(self: &Self) -> Result<X509Name, ErrorStack> { /* ... */ }
  ```
   Copies the name to a new `X509Name`.

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
  Serializes the certificate into a DER-encoded X509 name structure.

###### Trait Implementations

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

- **Unpin**
- **Sync**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509NameRef { /* ... */ }
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

- **RefUnwindSafe**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509NameRef { /* ... */ }
    ```

- **ForeignTypeRef**
- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Struct `X509NameEntries`

A type to destructure and examine an `X509Name`.

```rust
pub struct X509NameEntries<''a> {
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a X509NameEntryRef> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `X509NameEntry`

A name entry associated with a `X509Name`.

```rust
pub struct X509NameEntry(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509NameEntryRef { /* ... */ }
    ```

- **UnwindSafe**
- **Receiver**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509NameEntryRef { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Sync**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509_NAME_ENTRY) -> X509NameEntry { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509_NAME_ENTRY { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509NameEntryRef { /* ... */ }
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

  - ```rust
    fn borrow(self: &Self) -> &X509NameEntryRef { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
#### Struct `X509NameEntryRef`

Reference to `X509NameEntry`.

```rust
pub struct X509NameEntryRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn data(self: &Self) -> &Asn1StringRef { /* ... */ }
  ```
   Returns the field value of an `X509NameEntry`.

- ```rust
  pub fn object(self: &Self) -> &Asn1ObjectRef { /* ... */ }
  ```
   Returns the `Asn1Object` value of an `X509NameEntry`.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509NameEntryRef { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Freeze**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509NameEntryRef { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ForeignTypeRef**
- **Sync**
#### Struct `X509ReqBuilder`

A builder used to construct an `X509Req`.

```rust
pub struct X509ReqBuilder(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Result<X509ReqBuilder, ErrorStack> { /* ... */ }
  ```
   Returns a builder for a certificate request.

- ```rust
  pub fn set_version(self: &mut Self, version: i32) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Set the numerical value of the version field.

- ```rust
  pub fn set_subject_name(self: &mut Self, subject_name: &X509NameRef) -> Result<(), ErrorStack> { /* ... */ }
  ```
   Set the issuer name.

- ```rust
  pub fn set_pubkey<T>(self: &mut Self, key: &PKeyRef<T>) -> Result<(), ErrorStack>
where
    T: HasPublic { /* ... */ }
  ```
   Set the public key.

- ```rust
  pub fn x509v3_context<''a>(self: &''a Self, conf: Option<&''a ConfRef>) -> X509v3Context<''a> { /* ... */ }
  ```
  Return an `X509v3Context`. This context object can be used to construct

- ```rust
  pub fn add_extensions(self: &mut Self, extensions: &StackRef<X509Extension>) -> Result<(), ErrorStack> { /* ... */ }
  ```
  Permits any number of extension fields to be added to the certificate.

- ```rust
  pub fn sign<T>(self: &mut Self, key: &PKeyRef<T>, hash: MessageDigest) -> Result<(), ErrorStack>
where
    T: HasPrivate { /* ... */ }
  ```
   Sign the request using a private key.

- ```rust
  pub fn build(self: Self) -> X509Req { /* ... */ }
  ```
  Returns the `X509Req`.

###### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Send**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `X509Req`

An `X509` certificate request.

```rust
pub struct X509Req(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn builder() -> Result<X509ReqBuilder, ErrorStack> { /* ... */ }
  ```
  A builder for `X509Req`.

- ```rust
  pub fn from_pem(pem: &[u8]) -> Result<X509Req, crate::error::ErrorStack> { /* ... */ }
  ```
  Deserializes a PEM-encoded PKCS#10 certificate request structure.

- ```rust
  pub fn from_der(der: &[u8]) -> Result<X509Req, crate::error::ErrorStack> { /* ... */ }
  ```
  Deserializes a DER-encoded PKCS#10 certificate request structure.

###### Trait Implementations

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509ReqRef { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509ReqRef { /* ... */ }
    ```

- **Freeze**
- **Receiver**
- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509_REQ) -> X509Req { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509_REQ { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509ReqRef { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509ReqRef { /* ... */ }
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

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `X509ReqRef`

Reference to `X509Req`.

```rust
pub struct X509ReqRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn to_pem(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
  Serializes the certificate request to a PEM-encoded PKCS#10 structure.

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
  Serializes the certificate request to a DER-encoded PKCS#10 structure.

- ```rust
  pub fn to_text(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Converts the request to human readable text.

- ```rust
  pub fn version(self: &Self) -> i32 { /* ... */ }
  ```
   Returns the numerical value of the version field of the certificate request.

- ```rust
  pub fn subject_name(self: &Self) -> &X509NameRef { /* ... */ }
  ```
   Returns the subject name of the certificate request.

- ```rust
  pub fn public_key(self: &Self) -> Result<PKey<Public>, ErrorStack> { /* ... */ }
  ```
   Returns the public key of the certificate request.

- ```rust
  pub fn verify<T>(self: &Self, key: &PKeyRef<T>) -> Result<bool, ErrorStack>
where
    T: HasPublic { /* ... */ }
  ```
   Check if the certificate request is signed using the given public key.

- ```rust
  pub fn extensions(self: &Self) -> Result<Stack<X509Extension>, ErrorStack> { /* ... */ }
  ```
   Returns the extensions of the certificate request.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **ForeignTypeRef**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509ReqRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509ReqRef { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **RefUnwindSafe**
#### Struct `CrlReason`

The reason that a certificate was revoked.

```rust
pub struct CrlReason(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_raw(value: c_int) -> Self { /* ... */ }
  ```
  Constructs an `CrlReason` from a raw OpenSSL value.

- ```rust
  pub const fn as_raw(self: &Self) -> c_int { /* ... */ }
  ```
  Returns the raw OpenSSL value represented by this type.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CrlReason) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CrlReason { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **Eq**
- **Unpin**
#### Struct `X509Revoked`

An `X509` certificate revocation status.

```rust
pub struct X509Revoked(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_der(der: &[u8]) -> Result<X509Revoked, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a DER-encoded certificate revocation status

###### Trait Implementations

- **UnwindSafe**
- **Send**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509RevokedRef { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509RevokedRef { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509RevokedRef { /* ... */ }
    ```

- **Stackable**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Receiver**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509RevokedRef { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509_REVOKED) -> X509Revoked { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509_REVOKED { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

#### Struct `X509RevokedRef`

Reference to `X509Revoked`.

```rust
pub struct X509RevokedRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the certificate request to a DER-encoded certificate revocation status

- ```rust
  pub fn to_owned(self: &Self) -> Result<X509Revoked, ErrorStack> { /* ... */ }
  ```
   Copies the entry to a new `X509Revoked`.

- ```rust
  pub fn revocation_date(self: &Self) -> &Asn1TimeRef { /* ... */ }
  ```
   Get the date that the certificate was revoked

- ```rust
  pub fn serial_number(self: &Self) -> &Asn1IntegerRef { /* ... */ }
  ```
   Get the serial number of the revoked certificate

- ```rust
  pub fn extension<T: ExtensionType>(self: &Self) -> Result<Option<(bool, <T as >::Output)>, ErrorStack> { /* ... */ }
  ```
   Get the criticality and value of an extension.

###### Trait Implementations

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509RevokedRef { /* ... */ }
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

- **ForeignTypeRef**
- **Sync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509RevokedRef { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
#### Enum `ReasonCode`

The CRL entry extension identifying the reason for revocation see [`CrlReason`],
this is as defined in RFC 5280 Section 5.3.1.

```rust
pub enum ReasonCode {
}
```

##### Variants

##### Implementations

###### Trait Implementations

- **Freeze**
- **ExtensionType**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Send**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Enum `CertificateIssuer`

The CRL entry extension identifying the issuer of a certificate used in
indirect CRLs, as defined in RFC 5280 Section 5.3.3.

```rust
pub enum CertificateIssuer {
}
```

##### Variants

##### Implementations

###### Trait Implementations

- **Unpin**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **ExtensionType**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
#### Enum `AuthorityInformationAccess`

The CRL extension identifying how to access information and services for the issuer of the CRL

```rust
pub enum AuthorityInformationAccess {
}
```

##### Variants

##### Implementations

###### Trait Implementations

- **Send**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ExtensionType**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
#### Struct `X509Crl`

An `X509` certificate revocation list.

```rust
pub struct X509Crl(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_pem(pem: &[u8]) -> Result<X509Crl, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a PEM-encoded Certificate Revocation List

- ```rust
  pub fn from_der(der: &[u8]) -> Result<X509Crl, crate::error::ErrorStack> { /* ... */ }
  ```
   Deserializes a DER-encoded Certificate Revocation List

###### Trait Implementations

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509CrlRef { /* ... */ }
    ```

- **Send**
- **Receiver**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509_CRL) -> X509Crl { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509_CRL { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509CrlRef { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509CrlRef { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509CrlRef { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `X509CrlRef`

Reference to `X509Crl`.

```rust
pub struct X509CrlRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn to_pem(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the certificate request to a PEM-encoded Certificate Revocation List.

- ```rust
  pub fn to_der(self: &Self) -> Result<Vec<u8>, crate::error::ErrorStack> { /* ... */ }
  ```
   Serializes the certificate request to a DER-encoded Certificate Revocation List.

- ```rust
  pub fn get_revoked(self: &Self) -> Option<&StackRef<X509Revoked>> { /* ... */ }
  ```
  Get the stack of revocation entries

- ```rust
  pub fn last_update(self: &Self) -> &Asn1TimeRef { /* ... */ }
  ```
   Returns the CRL's `lastUpdate` time.

- ```rust
  pub fn next_update(self: &Self) -> Option<&Asn1TimeRef> { /* ... */ }
  ```
   Returns the CRL's `nextUpdate` time.

- ```rust
  pub fn get_by_serial<''a>(self: &''a Self, serial: &Asn1IntegerRef) -> CrlStatus<''a> { /* ... */ }
  ```
   Get the revocation status of a certificate by its serial number

- ```rust
  pub fn get_by_cert<''a>(self: &''a Self, cert: &X509) -> CrlStatus<''a> { /* ... */ }
  ```
   Get the revocation status of a certificate

- ```rust
  pub fn issuer_name(self: &Self) -> &X509NameRef { /* ... */ }
  ```
   Get the issuer name from the revocation list.

- ```rust
  pub fn verify<T>(self: &Self, key: &PKeyRef<T>) -> Result<bool, ErrorStack>
where
    T: HasPublic { /* ... */ }
  ```
   Check if the CRL is signed using the given public key.

- ```rust
  pub fn extension<T: ExtensionType>(self: &Self) -> Result<Option<(bool, <T as >::Output)>, ErrorStack> { /* ... */ }
  ```
   Get the criticality and value of an extension.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **ForeignTypeRef**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **RefUnwindSafe**
- **Freeze**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509CrlRef { /* ... */ }
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

  - ```rust
    fn borrow(self: &Self) -> &X509CrlRef { /* ... */ }
    ```

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

#### Enum `CrlStatus`

The status of a certificate in a revoction list

Corresponds to the return value from the [`X509_CRL_get0_by_*`] methods.

[`X509_CRL_get0_by_*`]: https://www.openssl.org/docs/man1.1.0/man3/X509_CRL_get0_by_serial.html

```rust
pub enum CrlStatus<''a> {
    NotRevoked,
    Revoked(&''a X509RevokedRef),
    RemoveFromCrl(&''a X509RevokedRef),
}
```

##### Variants

###### `NotRevoked`

The certificate is not present in the list

###### `Revoked`

The certificate is in the list and is revoked

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a X509RevokedRef` |  |

###### `RemoveFromCrl`

The certificate is in the list, but has the "removeFromCrl" status.

This can occur if the certificate was revoked with the "CertificateHold"
reason, and has since been unrevoked.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a X509RevokedRef` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **UnwindSafe**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `X509VerifyResult`

The result of peer certificate verification.

```rust
pub struct X509VerifyResult(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub unsafe fn from_raw(err: c_int) -> X509VerifyResult { /* ... */ }
  ```
  Creates an `X509VerifyResult` from a raw error number.

- ```rust
  pub fn as_raw(self: &Self) -> c_int { /* ... */ }
  ```
  Return the integer representation of an `X509VerifyResult`.

- ```rust
  pub fn error_string(self: &Self) -> &''static str { /* ... */ }
  ```
   Return a human readable error string from the verification error.

###### Trait Implementations

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> X509VerifyResult { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **UnwindSafe**
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
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Sync**
- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &X509VerifyResult) -> bool { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Error**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `GeneralName`

An `X509` certificate alternative names.

```rust
pub struct GeneralName(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::GENERAL_NAME) -> GeneralName { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::GENERAL_NAME { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut GeneralNameRef { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Stackable**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &GeneralNameRef { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Receiver**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &GeneralNameRef { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &GeneralNameRef { /* ... */ }
    ```

#### Struct `GeneralNameRef`

Reference to `GeneralName`.

```rust
pub struct GeneralNameRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn email(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Returns the contents of this `GeneralName` if it is an `rfc822Name`.

- ```rust
  pub fn directory_name(self: &Self) -> Option<&X509NameRef> { /* ... */ }
  ```
  Returns the contents of this `GeneralName` if it is a `directoryName`.

- ```rust
  pub fn dnsname(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Returns the contents of this `GeneralName` if it is a `dNSName`.

- ```rust
  pub fn uri(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Returns the contents of this `GeneralName` if it is an `uniformResourceIdentifier`.

- ```rust
  pub fn ipaddress(self: &Self) -> Option<&[u8]> { /* ... */ }
  ```
  Returns the contents of this `GeneralName` if it is an `iPAddress`.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &GeneralNameRef { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &GeneralNameRef { /* ... */ }
    ```

- **ForeignTypeRef**
#### Struct `DistPoint`

A `X509` distribution point.

```rust
pub struct DistPoint(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &DistPointRef { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Receiver**
- **Send**
- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &DistPointRef { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::DIST_POINT) -> DistPoint { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::DIST_POINT { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &DistPointRef { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut DistPointRef { /* ... */ }
    ```

- **Stackable**
#### Struct `DistPointRef`

Reference to `DistPoint`.

```rust
pub struct DistPointRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn distpoint(self: &Self) -> Option<&DistPointNameRef> { /* ... */ }
  ```
  Returns the name of this distribution point if it exists

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Unpin**
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
- **ForeignTypeRef**
- **Sync**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &DistPointRef { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &DistPointRef { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `DistPointName`

A `X509` distribution point.

```rust
pub struct DistPointName(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Sync**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &DistPointNameRef { /* ... */ }
    ```

- **Receiver**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::DIST_POINT_NAME) -> DistPointName { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::DIST_POINT_NAME { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &DistPointNameRef { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut DistPointNameRef { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &DistPointNameRef { /* ... */ }
    ```

#### Struct `DistPointNameRef`

Reference to `DistPointName`.

```rust
pub struct DistPointNameRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn fullname(self: &Self) -> Option<&StackRef<GeneralName>> { /* ... */ }
  ```
  Returns the contents of this DistPointName if it is a fullname.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &DistPointNameRef { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &DistPointNameRef { /* ... */ }
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

- **UnwindSafe**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ForeignTypeRef**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Send**
#### Struct `AccessDescription`

`AccessDescription` of certificate authority information.

```rust
pub struct AccessDescription(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &AccessDescriptionRef { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &AccessDescriptionRef { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::ACCESS_DESCRIPTION) -> AccessDescription { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::ACCESS_DESCRIPTION { /* ... */ }
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

- **Freeze**
- **Stackable**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Unpin**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut AccessDescriptionRef { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Receiver**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &AccessDescriptionRef { /* ... */ }
    ```

#### Struct `AccessDescriptionRef`

Reference to `AccessDescription`.

```rust
pub struct AccessDescriptionRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn method(self: &Self) -> &Asn1ObjectRef { /* ... */ }
  ```
  Returns the access method OID.

- ```rust
  pub fn location(self: &Self) -> &GeneralNameRef { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &AccessDescriptionRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ForeignTypeRef**
- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &AccessDescriptionRef { /* ... */ }
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

#### Struct `X509Algorithm`

An `X509` certificate signature algorithm.

```rust
pub struct X509Algorithm(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509AlgorithmRef { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509AlgorithmRef { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Receiver**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509_ALGOR) -> X509Algorithm { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509_ALGOR { /* ... */ }
    ```

- **Unpin**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509AlgorithmRef { /* ... */ }
    ```

- **Send**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509AlgorithmRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Sync**
#### Struct `X509AlgorithmRef`

Reference to `X509Algorithm`.

```rust
pub struct X509AlgorithmRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn object(self: &Self) -> &Asn1ObjectRef { /* ... */ }
  ```
  Returns the ASN.1 OID of this algorithm.

###### Trait Implementations

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ForeignTypeRef**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509AlgorithmRef { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509AlgorithmRef { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Freeze**
- **Sync**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `X509Object`

An `X509` or an X509 certificate revocation list.

```rust
pub struct X509Object(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &X509ObjectRef { /* ... */ }
    ```

- **Sync**
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Receiver**
- **Stackable**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut X509ObjectRef { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509ObjectRef { /* ... */ }
    ```

- **ForeignType**
  - ```rust
    unsafe fn from_ptr(ptr: *mut ffi::X509_OBJECT) -> X509Object { /* ... */ }
    ```

  - ```rust
    fn as_ptr(self: &Self) -> *mut ffi::X509_OBJECT { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509ObjectRef { /* ... */ }
    ```

#### Struct `X509ObjectRef`

Reference to `X509Object`

```rust
pub struct X509ObjectRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn x509(self: &Self) -> Option<&X509Ref> { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &X509ObjectRef { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **ForeignTypeRef**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &X509ObjectRef { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `X509PurposeId`

```rust
pub struct X509PurposeId(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn from_raw(id: c_int) -> Self { /* ... */ }
  ```
  Constructs an `X509PurposeId` from a raw OpenSSL value.

- ```rust
  pub fn as_raw(self: &Self) -> c_int { /* ... */ }
  ```
  Returns the raw OpenSSL value represented by this type.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **StructuralPartialEq**
- **Eq**
- **Sync**
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

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> X509PurposeId { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &X509PurposeId) -> bool { /* ... */ }
    ```

- **Unpin**
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

#### Struct `X509PurposeRef`

A reference to an [`X509_PURPOSE`].

```rust
pub struct X509PurposeRef(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn get_by_sname(sname: &str) -> Result<c_int, ErrorStack> { /* ... */ }
  ```
  Get the internal table index of an X509_PURPOSE for a given short name. Valid short

- ```rust
  pub fn from_idx(idx: c_int) -> Result<&''static X509PurposeRef, ErrorStack> { /* ... */ }
  ```
   Get an `X509PurposeRef` for a given index value. The index can be obtained from e.g.

- ```rust
  pub fn purpose(self: &Self) -> X509PurposeId { /* ... */ }
  ```
  Get the purpose value from an X509Purpose structure. This value is one of

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **UnwindSafe**
- **Freeze**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **ForeignTypeRef**
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

### Traits

#### Trait `ExtensionType`

A type of X509 extension.

# Safety
The value of NID and Output must match those in OpenSSL so that
`Output::from_ptr_opt(*_get_ext_d2i(*, NID, ...))` is valid.

```rust
pub unsafe trait ExtensionType {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Output`

###### Associated Constants

- `NID`

##### Implementations

This trait is implemented for the following types:

- `ReasonCode`
- `CertificateIssuer`
- `AuthorityInformationAccess`

## Re-exports

### Re-export `init`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use ffi::init;
```

