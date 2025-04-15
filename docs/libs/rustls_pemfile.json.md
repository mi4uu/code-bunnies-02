# Crate Documentation

**Version:** 2.2.0

**Format Version:** 43

# Module `rustls_pemfile`

# rustls-pemfile
A basic parser for .pem files containing cryptographic keys and certificates.

The input to this crate is a .pem file containing potentially many sections,
and the output is those sections as alleged DER-encodings.  This crate does
not decode the actual DER-encoded keys/certificates.

## Quick start
Starting with an `io::BufRead` containing the file to be read:
- Use `read_all()` to ingest the whole file, then work through the contents in-memory, or,
- Use `read_one()` to stream through the file, processing the items as found, or,
- Use `certs()` to extract just the certificates (silently discarding other sections), and
  similarly for `rsa_private_keys()` and `pkcs8_private_keys()`.

# no-std support

The opt-out "std" Cargo feature can be disabled to put this crate in no-std mode.

In no-std mode, the `read_one_from_slice` API can be used to parse a .pem file that has already
been loaded into memory.

## Example code
```
use std::iter;
use rustls_pemfile::{Item, read_one};
# let mut reader = std::io::BufReader::new(&b"junk\n-----BEGIN RSA PRIVATE KEY-----\nqw\n-----END RSA PRIVATE KEY-----\n"[..]);
// Assume `reader` is any std::io::BufRead implementor
for item in iter::from_fn(|| read_one(&mut reader).transpose()) {
    match item.unwrap() {
        Item::X509Certificate(cert) => println!("certificate {:?}", cert),
        Item::Crl(crl) => println!("certificate revocation list: {:?}", crl),
        Item::Csr(csr) => println!("certificate signing request: {:?}", csr),
        Item::Pkcs1Key(key) => println!("rsa pkcs1 key {:?}", key),
        Item::Pkcs8Key(key) => println!("pkcs8 key {:?}", key),
        Item::Sec1Key(key) => println!("sec1 ec key {:?}", key),
        _ => println!("unhandled item"),
    }
}
```

## Functions

### Function `certs`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Return an iterator over certificates from `rd`.

Filters out any PEM sections that are not certificates and yields errors if a problem
occurs while trying to extract a certificate.

```rust
pub fn certs(rd: &mut dyn io::BufRead) -> impl Iterator<Item = Result<pki_types::CertificateDer<''static>, io::Error>> + ''_ { /* ... */ }
```

### Function `private_key`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Return the first private key found in `rd`.

Yields the first PEM section describing a private key (of any type), or an error if a
problem occurs while trying to read PEM sections.

```rust
pub fn private_key(rd: &mut dyn io::BufRead) -> Result<Option<pki_types::PrivateKeyDer<''static>>, io::Error> { /* ... */ }
```

### Function `csr`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Return the first certificate signing request (CSR) found in `rd`.

Yields the first PEM section describing a certificate signing request, or an error if a
problem occurs while trying to read PEM sections.

```rust
pub fn csr(rd: &mut dyn io::BufRead) -> Result<Option<pki_types::CertificateSigningRequestDer<''static>>, io::Error> { /* ... */ }
```

### Function `crls`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Return an iterator certificate revocation lists (CRLs) from `rd`.

Filters out any PEM sections that are not CRLs and yields errors if a problem occurs
while trying to extract a CRL.

```rust
pub fn crls(rd: &mut dyn io::BufRead) -> impl Iterator<Item = Result<pki_types::CertificateRevocationListDer<''static>, io::Error>> + ''_ { /* ... */ }
```

### Function `rsa_private_keys`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Return an iterator over RSA private keys from `rd`.

Filters out any PEM sections that are not RSA private keys and yields errors if a problem
occurs while trying to extract an RSA private key.

```rust
pub fn rsa_private_keys(rd: &mut dyn io::BufRead) -> impl Iterator<Item = Result<pki_types::PrivatePkcs1KeyDer<''static>, io::Error>> + ''_ { /* ... */ }
```

### Function `pkcs8_private_keys`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Return an iterator over PKCS8-encoded private keys from `rd`.

Filters out any PEM sections that are not PKCS8-encoded private keys and yields errors if a
problem occurs while trying to extract an RSA private key.

```rust
pub fn pkcs8_private_keys(rd: &mut dyn io::BufRead) -> impl Iterator<Item = Result<pki_types::PrivatePkcs8KeyDer<''static>, io::Error>> + ''_ { /* ... */ }
```

### Function `ec_private_keys`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Return an iterator over SEC1-encoded EC private keys from `rd`.

Filters out any PEM sections that are not SEC1-encoded EC private keys and yields errors if a
problem occurs while trying to extract a SEC1-encoded EC private key.

```rust
pub fn ec_private_keys(rd: &mut dyn io::BufRead) -> impl Iterator<Item = Result<pki_types::PrivateSec1KeyDer<''static>, io::Error>> + ''_ { /* ... */ }
```

### Function `public_keys`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Return an iterator over SPKI-encoded keys from `rd`.

Filters out any PEM sections that are not SPKI-encoded public keys and yields errors if a
problem occurs while trying to extract a SPKI-encoded public key.

```rust
pub fn public_keys(rd: &mut dyn io::BufRead) -> impl Iterator<Item = Result<pki_types::SubjectPublicKeyInfoDer<''static>, io::Error>> + ''_ { /* ... */ }
```

## Re-exports

### Re-export `read_all`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use pemfile::read_all;
```

### Re-export `read_one`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use pemfile::read_one;
```

### Re-export `read_one_from_slice`

```rust
pub use pemfile::read_one_from_slice;
```

### Re-export `Error`

```rust
pub use pemfile::Error;
```

### Re-export `Item`

```rust
pub use pemfile::Item;
```

