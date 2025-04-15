# Crate Documentation

**Version:** 0.10.6

**Format Version:** 43

# Module `sha1`

Pure Rust implementation of the [SHA-1][1] cryptographic hash algorithm
with optional hardware-specific optimizations.

# 🚨 Warning: Cryptographically Broken! 🚨

The SHA-1 hash function should be considered cryptographically broken and
unsuitable for further use in any security critical capacity, as it is
[practically vulnerable to chosen-prefix collisions][2].

We provide this crate for legacy interoperability purposes only.

# Usage

```rust
use hex_literal::hex;
use sha1::{Sha1, Digest};

// create a Sha1 object
let mut hasher = Sha1::new();

// process input message
hasher.update(b"hello world");

// acquire hash digest in the form of GenericArray,
// which in this case is equivalent to [u8; 20]
let result = hasher.finalize();
assert_eq!(result[..], hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));
```

Also see [RustCrypto/hashes][3] readme.

# Note for users of `sha1 v0.6`

This crate has been transferred to the RustCrypto organization and uses
implementation previously published as the `sha-1` crate. The previous
zero dependencies version is now published as the [`sha1_smol`] crate.

[1]: https://en.wikipedia.org/wiki/SHA-1
[2]: https://sha-mbles.github.io/
[3]: https://github.com/RustCrypto/hashes
[`sha1_smol`]: https://github.com/mitsuhiko/sha1-smol/

## Types

### Struct `Sha1Core`

Core SHA-1 hasher state.

```rust
pub struct Sha1Core {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **UpdateCore**
  - ```rust
    fn update_blocks(self: &mut Self, blocks: &[Block<Self>]) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Sha1Core { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Same**
- **Freeze**
- **BlockSizeUser**
- **Reset**
  - ```rust
    fn reset(self: &mut Self) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **AlgorithmName**
  - ```rust
    fn write_alg_name(f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **HashMarker**
- **Unpin**
- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **OutputSizeUser**
- **FixedOutputCore**
  - ```rust
    fn finalize_fixed_core(self: &mut Self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) { /* ... */ }
    ```

- **BufferKindUser**
- **RefUnwindSafe**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Type Alias `Sha1`

SHA-1 hasher state.

```rust
pub type Sha1 = digest::core_api::CoreWrapper<Sha1Core>;
```

## Re-exports

### Re-export `digest`

```rust
pub use digest;
```

### Re-export `Digest`

```rust
pub use digest::Digest;
```

