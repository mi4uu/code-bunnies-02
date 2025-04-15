# Crate Documentation

**Version:** 0.1.6

**Format Version:** 43

# Module `openssl_probe`

## Types

### Struct `ProbeResult`

```rust
pub struct ProbeResult {
    pub cert_file: Option<std::path::PathBuf>,
    pub cert_dir: Option<std::path::PathBuf>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `cert_file` | `Option<std::path::PathBuf>` |  |
| `cert_dir` | `Option<std::path::PathBuf>` |  |

#### Implementations

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

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
## Functions

### Function `candidate_cert_dirs`

Probe the system for the directory in which CA certificates should likely be
found.

This will only search known system locations.

```rust
pub fn candidate_cert_dirs() -> impl Iterator<Item = &''static std::path::Path> { /* ... */ }
```

### Function `init_openssl_env_vars`

Probe for SSL certificates on the system, then configure the SSL certificate `SSL_CERT_FILE`
and `SSL_CERT_DIR` environment variables in this process for OpenSSL to use.

Preconfigured values in the environment variables will not be overwritten if the paths they
point to exist and are accessible.

# Safety

This function is not safe because it mutates the process's environment
variables which is generally not safe. See the [documentation in libstd][doc]
for information about why setting environment variables is not safe.

If possible use the [`probe`] function and directly configure OpenSSL
methods instead of relying on environment variables.

[doc]: https://doc.rust-lang.org/stable/std/env/fn.set_var.html#safety

```rust
pub unsafe fn init_openssl_env_vars() { /* ... */ }
```

### Function `try_init_openssl_env_vars`

Probe for SSL certificates on the system, then configure the SSL certificate `SSL_CERT_FILE`
and `SSL_CERT_DIR` environment variables in this process for OpenSSL to use.

Preconfigured values in the environment variables will not be overwritten if the paths they
point to exist and are accessible.

Returns `true` if any certificate file or directory was found while probing.
Combine this with `has_ssl_cert_env_vars()` to check whether previously configured environment
variables are valid.

# Safety

This function is not safe because it mutates the process's environment
variables which is generally not safe. See the [documentation in libstd][doc]
for information about why setting environment variables is not safe.

If possible use the [`probe`] function and directly configure OpenSSL
methods instead of relying on environment variables.

[doc]: https://doc.rust-lang.org/stable/std/env/fn.set_var.html#safety

```rust
pub unsafe fn try_init_openssl_env_vars() -> bool { /* ... */ }
```

### Function `has_ssl_cert_env_vars`

Check whether the OpenSSL `SSL_CERT_FILE` and/or `SSL_CERT_DIR` environment variable is
configured in this process with an existing file or directory.

That being the case would indicate that certificates will be found successfully by OpenSSL.

Returns `true` if either variable is set to an existing file or directory.

```rust
pub fn has_ssl_cert_env_vars() -> bool { /* ... */ }
```

### Function `probe`

Probe the current system for the "cert file" and "cert dir" variables that
OpenSSL typically requires.

The probe result is returned as a [`ProbeResult`] structure here.

```rust
pub fn probe() -> ProbeResult { /* ... */ }
```

## Constants and Statics

### Constant `ENV_CERT_FILE`

The OpenSSL environment variable to configure what certificate file to use.

```rust
pub const ENV_CERT_FILE: &''static str = "SSL_CERT_FILE";
```

### Constant `ENV_CERT_DIR`

The OpenSSL environment variable to configure what certificates directory to use.

```rust
pub const ENV_CERT_DIR: &''static str = "SSL_CERT_DIR";
```

