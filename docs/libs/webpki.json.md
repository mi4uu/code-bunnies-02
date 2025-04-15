# Crate Documentation

**Version:** 0.103.1

**Format Version:** 43

# Module `webpki`

webpki: Web PKI X.509 Certificate Validation.

See `EndEntityCert`'s documentation for a description of the certificate
processing steps necessary for a TLS connection.

# Features

| Feature | Description |
| ------- | ----------- |
| `alloc` | Enable features that require use of the heap. Currently all RSA signature algorithms require this feature. |
| `std` | Enable features that require libstd. Implies `alloc`. |
| `ring` | Enable use of the *ring* crate for cryptography. |
| `aws-lc-rs` | Enable use of the aws-lc-rs crate for cryptography. Previously this feature was named `aws_lc_rs`. |

## Modules

## Module `ring`

**Attributes:**

- `#[<cfg>(feature = "ring")]`

Signature verification algorithm implementations using the *ring* crypto library.

```rust
pub mod ring { /* ... */ }
```

### Re-exports

#### Re-export `ECDSA_P256_SHA256`

```rust
pub use super::ring_algs::ECDSA_P256_SHA256;
```

#### Re-export `ECDSA_P256_SHA384`

```rust
pub use super::ring_algs::ECDSA_P256_SHA384;
```

#### Re-export `ECDSA_P384_SHA256`

```rust
pub use super::ring_algs::ECDSA_P384_SHA256;
```

#### Re-export `ECDSA_P384_SHA384`

```rust
pub use super::ring_algs::ECDSA_P384_SHA384;
```

#### Re-export `ED25519`

```rust
pub use super::ring_algs::ED25519;
```

#### Re-export `RSA_PKCS1_2048_8192_SHA256`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use super::ring_algs::RSA_PKCS1_2048_8192_SHA256;
```

#### Re-export `RSA_PKCS1_2048_8192_SHA384`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use super::ring_algs::RSA_PKCS1_2048_8192_SHA384;
```

#### Re-export `RSA_PKCS1_2048_8192_SHA512`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use super::ring_algs::RSA_PKCS1_2048_8192_SHA512;
```

#### Re-export `RSA_PKCS1_3072_8192_SHA384`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use super::ring_algs::RSA_PKCS1_3072_8192_SHA384;
```

#### Re-export `RSA_PSS_2048_8192_SHA256_LEGACY_KEY`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use super::ring_algs::RSA_PSS_2048_8192_SHA256_LEGACY_KEY;
```

#### Re-export `RSA_PSS_2048_8192_SHA384_LEGACY_KEY`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use super::ring_algs::RSA_PSS_2048_8192_SHA384_LEGACY_KEY;
```

#### Re-export `RSA_PSS_2048_8192_SHA512_LEGACY_KEY`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use super::ring_algs::RSA_PSS_2048_8192_SHA512_LEGACY_KEY;
```

## Constants and Statics

### Static `ALL_VERIFICATION_ALGS`

An array of all the verification algorithms exported by this crate.

This will be empty if the crate is built without the `ring` and `aws-lc-rs` features.

```rust
pub static ALL_VERIFICATION_ALGS: &[&dyn pki_types::SignatureVerificationAlgorithm] = _;
```

## Re-exports

### Re-export `Cert`

```rust
pub use cert::Cert;
```

### Re-export `BorrowedCertRevocationList`

```rust
pub use crl::BorrowedCertRevocationList;
```

### Re-export `BorrowedRevokedCert`

```rust
pub use crl::BorrowedRevokedCert;
```

### Re-export `CertRevocationList`

```rust
pub use crl::CertRevocationList;
```

### Re-export `ExpirationPolicy`

```rust
pub use crl::ExpirationPolicy;
```

### Re-export `RevocationCheckDepth`

```rust
pub use crl::RevocationCheckDepth;
```

### Re-export `RevocationOptions`

```rust
pub use crl::RevocationOptions;
```

### Re-export `RevocationOptionsBuilder`

```rust
pub use crl::RevocationOptionsBuilder;
```

### Re-export `RevocationReason`

```rust
pub use crl::RevocationReason;
```

### Re-export `UnknownStatusPolicy`

```rust
pub use crl::UnknownStatusPolicy;
```

### Re-export `EndEntityCert`

```rust
pub use end_entity::EndEntityCert;
```

### Re-export `DerTypeId`

```rust
pub use error::DerTypeId;
```

### Re-export `Error`

```rust
pub use error::Error;
```

### Re-export `InvalidNameContext`

```rust
pub use error::InvalidNameContext;
```

### Re-export `RawPublicKeyEntity`

```rust
pub use rpk_entity::RawPublicKeyEntity;
```

### Re-export `anchor_from_trusted_cert`

```rust
pub use trust_anchor::anchor_from_trusted_cert;
```

### Re-export `KeyUsage`

```rust
pub use verify_cert::KeyUsage;
```

### Re-export `VerifiedPath`

```rust
pub use verify_cert::VerifiedPath;
```

### Re-export `OwnedCertRevocationList`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crl::OwnedCertRevocationList;
```

### Re-export `OwnedRevokedCert`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

```rust
pub use crl::OwnedRevokedCert;
```

