# Crate Documentation

**Version:** 0.4.2

**Format Version:** 43

# Module `hf_hub`

This crates aims to emulate and be compatible with the
[huggingface_hub](https://github.com/huggingface/huggingface_hub/) python package.

compatible means the Api should reuse the same files skipping downloads if
they are already present and whenever this crate downloads or modifies this cache
it should be consistent with [huggingface_hub](https://github.com/huggingface/huggingface_hub/)

At this time only a limited subset of the functionality is present, the goal is to add new
features over time. We are currently treating this as an internel/external tool, meaning
we will are currently modifying everything at will for out internal needs. This will eventually
stabilize as it matures to accomodate most of our needs.

If you're interested in using this, you're welcome to do it but be warned about potential changing grounds.

If you want to contribute, you are more than welcome.

However allowing new features or creating new features might be denied by lack of maintainability
time. We're focusing on what we currently internally need. Hopefully that subset is already interesting
to more users.


# How to use 

Add the dependency

```bash
cargo add hf-hub  # --features tokio
```
`tokio` feature will enable an async (and potentially faster) API.

Use the crate:

```rust
use hf_hub::api::sync::Api;

let api = Api::new().unwrap();

let repo = api.model("bert-base-uncased".to_string());
let _filename = repo.get("config.json").unwrap();

// filename  is now the local location within hf cache of the config.json file
```

# SSL/TLS

This library uses tokio default TLS implementations which is `native-tls` (openssl) for `tokio`.

If you want control over the TLS backend you can remove the default features and only add the backend you are intending to use. 

```bash
cargo add hf-hub --no-default-features --features ureq,rustls-tls
cargo add hf-hub --no-default-features --features ureq,native-tls
cargo add hf-hub --no-default-features --features tokio,rustls-tls
cargo add hf-hub --no-default-features --features tokio,native-tls
```


When using the [`ureq`](https://github.com/algesten/ureq) feature, you will always use its default TLS backend which is [rustls](https://github.com/rustls/rustls).

When using [`tokio`](https://github.com/tokio-rs/tokio), by default `default-tls` will be enabled, which means OpenSSL. If you want/need to use rustls, disable the default features and use `rustls-tls` in conjunction with `tokio`.

## Modules

## Module `api`

**Attributes:**

- `#[<cfg>(any(feature = "tokio", feature = "ureq"))]`

The actual Api to interact with the hub.

```rust
pub mod api { /* ... */ }
```

### Modules

## Module `tokio`

**Attributes:**

- `#[<cfg>(feature = "tokio")]`

The asynchronous version of the API

```rust
pub mod tokio { /* ... */ }
```

### Types

#### Enum `ApiError`

All errors the API can throw

```rust
pub enum ApiError {
    MissingHeader(reqwest::header::HeaderName),
    InvalidHeader(reqwest::header::HeaderName),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    ToStr(reqwest::header::ToStrError),
    RequestError(reqwest::Error),
    ParseIntError(std::num::ParseIntError),
    IoError(std::io::Error),
    TooManyRetries(Box<ApiError>),
    TryAcquireError(tokio::sync::TryAcquireError),
    AcquireError(tokio::sync::AcquireError),
    Join(tokio::task::JoinError),
    LockAcquisition(std::path::PathBuf),
}
```

##### Variants

###### `MissingHeader`

Api expects certain header to be present in the results to derive some information

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `reqwest::header::HeaderName` |  |

###### `InvalidHeader`

The header exists, but the value is not conform to what the Api expects.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `reqwest::header::HeaderName` |  |

###### `InvalidHeaderValue`

The value cannot be used as a header during request header construction

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `reqwest::header::InvalidHeaderValue` |  |

###### `ToStr`

The header value is not valid utf-8

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `reqwest::header::ToStrError` |  |

###### `RequestError`

Error in the request

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `reqwest::Error` |  |

###### `ParseIntError`

Error parsing some range value

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::num::ParseIntError` |  |

###### `IoError`

I/O Error

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::io::Error` |  |

###### `TooManyRetries`

We tried to download chunk too many times

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ApiError>` |  |

###### `TryAcquireError`

Semaphore cannot be acquired

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `tokio::sync::TryAcquireError` |  |

###### `AcquireError`

Semaphore cannot be acquired

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `tokio::sync::AcquireError` |  |

###### `Join`

Join failed

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `tokio::task::JoinError` |  |

###### `LockAcquisition`

We failed to acquire lock for file `f`. Meaning
Someone else is writing/downloading said file

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::path::PathBuf` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Sync**
- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **Instrument**
- **RefUnwindSafe**
- **Error**
  - ```rust
    fn source(self: &Self) -> ::core::option::Option<&dyn ::thiserror::__private::Error + ''static> { /* ... */ }
    ```

- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: InvalidHeaderValue) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: ToStrError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: ReqwestError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: ParseIntError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: std::io::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: TryAcquireError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: AcquireError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: JoinError) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **WithSubscriber**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
#### Struct `ApiBuilder`

Helper to create [`Api`] with all the options.

```rust
pub struct ApiBuilder {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Default api builder

- ```rust
  pub fn from_env() -> Self { /* ... */ }
  ```
  Creates API with values potentially from environment variables.

- ```rust
  pub fn high(self: Self) -> Self { /* ... */ }
  ```
  High CPU download

- ```rust
  pub fn from_cache(cache: Cache) -> Self { /* ... */ }
  ```
  From a given cache

- ```rust
  pub fn with_progress(self: Self, progress: bool) -> Self { /* ... */ }
  ```
  Wether to show a progressbar

- ```rust
  pub fn with_endpoint(self: Self, endpoint: String) -> Self { /* ... */ }
  ```
  Changes the endpoint of the API. Default is `https://huggingface.co`.

- ```rust
  pub fn with_cache_dir(self: Self, cache_dir: PathBuf) -> Self { /* ... */ }
  ```
  Changes the location of the cache directory. Defaults is `~/.cache/huggingface/`.

- ```rust
  pub fn with_token(self: Self, token: Option<String>) -> Self { /* ... */ }
  ```
  Sets the token to be used in the API

- ```rust
  pub fn with_max_files(self: Self, max_files: usize) -> Self { /* ... */ }
  ```
  Sets the number of open files

- ```rust
  pub fn with_chunk_size(self: Self, chunk_size: Option<usize>) -> Self { /* ... */ }
  ```
  Sets the size of each chunk

- ```rust
  pub fn with_user_agent(self: Self, key: &str, value: &str) -> Self { /* ... */ }
  ```
  Adds custom fields to headers user-agent

- ```rust
  pub fn build(self: Self) -> Result<Api, ApiError> { /* ... */ }
  ```
  Consumes the builder and builds the final [`Api`]

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Send**
- **UnwindSafe**
- **ErasedDestructor**
- **Instrument**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **WithSubscriber**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **RefUnwindSafe**
#### Struct `Api`

The actual Api used to interact with the hub.
Use any repo with [`Api::repo`]

```rust
pub struct Api {
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
  pub fn new() -> Result<Self, ApiError> { /* ... */ }
  ```
  Creates a default Api, for Api options See [`ApiBuilder`]

- ```rust
  pub fn client(self: &Self) -> &Client { /* ... */ }
  ```
  Get the underlying api client

- ```rust
  pub fn repo(self: &Self, repo: Repo) -> ApiRepo { /* ... */ }
  ```
  Creates a new handle [`ApiRepo`] which contains operations

- ```rust
  pub fn model(self: &Self, model_id: String) -> ApiRepo { /* ... */ }
  ```
  Simple wrapper over

- ```rust
  pub fn dataset(self: &Self, model_id: String) -> ApiRepo { /* ... */ }
  ```
  Simple wrapper over

- ```rust
  pub fn space(self: &Self, model_id: String) -> ApiRepo { /* ... */ }
  ```
  Simple wrapper over

###### Trait Implementations

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
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

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Instrument**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Api { /* ... */ }
    ```

- **ErasedDestructor**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **WithSubscriber**
- **Send**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **MaybeSendSync**
#### Struct `ApiRepo`

Shorthand for accessing things within a particular repo
You can inspect repos with [`ApiRepo::info`]
or download files with [`ApiRepo::download`]

```rust
pub struct ApiRepo {
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
  pub fn url(self: &Self, filename: &str) -> String { /* ... */ }
  ```
  Get the fully qualified URL of the remote filename

- ```rust
  pub async fn get(self: &Self, filename: &str) -> Result<PathBuf, ApiError> { /* ... */ }
  ```
  This will attempt the fetch the file locally first, then [`Api.download`]

- ```rust
  pub async fn download(self: &Self, filename: &str) -> Result<PathBuf, ApiError> { /* ... */ }
  ```
  Downloads a remote file (if not already present) into the cache directory

- ```rust
  pub async fn download_with_progress<P: Progress + Clone + Send + Sync + ''static>(self: &Self, filename: &str, progress: P) -> Result<PathBuf, ApiError> { /* ... */ }
  ```
  This function is used to download a file with a custom progress function.

- ```rust
  pub async fn info(self: &Self) -> Result<RepoInfo, ApiError> { /* ... */ }
  ```
  Get information about the Repo

- ```rust
  pub fn info_request(self: &Self) -> RequestBuilder { /* ... */ }
  ```
  Get the raw [`reqwest::RequestBuilder`] with the url and method already set

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Instrument**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **WithSubscriber**
### Traits

#### Trait `Progress`

This trait is used by users of the lib
to implement custom behavior during file downloads

```rust
pub trait Progress {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `init`: At the start of the download
- `update`: This function is called whenever `size` bytes have been
- `finish`: This is called at the end of the download

##### Implementations

This trait is implemented for the following types:

- `indicatif::ProgressBar`
- `()`

## Module `sync`

**Attributes:**

- `#[<cfg>(feature = "ureq")]`

The synchronous version of the API

```rust
pub mod sync { /* ... */ }
```

### Types

#### Struct `HeaderAgent`

Simple wrapper over [`ureq::Agent`] to include default headers

```rust
pub struct HeaderAgent {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **WithSubscriber**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **Instrument**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> HeaderAgent { /* ... */ }
    ```

#### Enum `ApiError`

All errors the API can throw

```rust
pub enum ApiError {
    MissingHeader(&''static str),
    InvalidHeader(&''static str),
    RequestError(Box<ureq::Error>),
    ParseIntError(std::num::ParseIntError),
    IoError(std::io::Error),
    TooManyRetries(Box<ApiError>),
    Native(native_tls::Error),
    InvalidResume,
    LockAcquisition(std::path::PathBuf),
}
```

##### Variants

###### `MissingHeader`

Api expects certain header to be present in the results to derive some information

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''static str` |  |

###### `InvalidHeader`

The header exists, but the value is not conform to what the Api expects.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''static str` |  |

###### `RequestError`

Error in the request

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ureq::Error>` |  |

###### `ParseIntError`

Error parsing some range value

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::num::ParseIntError` |  |

###### `IoError`

I/O Error

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::io::Error` |  |

###### `TooManyRetries`

We tried to download chunk too many times

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ApiError>` |  |

###### `Native`

Native tls error

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `native_tls::Error` |  |

###### `InvalidResume`

The part file is corrupted

###### `LockAcquisition`

We failed to acquire lock for file `f`. Meaning
Someone else is writing/downloading said file

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::path::PathBuf` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **WithSubscriber**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: Box<ureq::Error>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: ParseIntError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: std::io::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: native_tls::Error) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Instrument**
- **Freeze**
- **Error**
  - ```rust
    fn source(self: &Self) -> ::core::option::Option<&dyn ::thiserror::__private::Error + ''static> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **Sync**
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

- **RefUnwindSafe**
#### Struct `ApiBuilder`

Helper to create [`Api`] with all the options.

```rust
pub struct ApiBuilder {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Default api builder

- ```rust
  pub fn from_env() -> Self { /* ... */ }
  ```
  Creates API with values potentially from environment variables.

- ```rust
  pub fn from_cache(cache: Cache) -> Self { /* ... */ }
  ```
  From a given cache

- ```rust
  pub fn with_progress(self: Self, progress: bool) -> Self { /* ... */ }
  ```
  Wether to show a progressbar

- ```rust
  pub fn with_endpoint(self: Self, endpoint: String) -> Self { /* ... */ }
  ```
  Changes the endpoint of the API. Default is `https://huggingface.co`.

- ```rust
  pub fn with_cache_dir(self: Self, cache_dir: PathBuf) -> Self { /* ... */ }
  ```
  Changes the location of the cache directory. Defaults is `~/.cache/huggingface/`.

- ```rust
  pub fn with_token(self: Self, token: Option<String>) -> Self { /* ... */ }
  ```
  Sets the token to be used in the API

- ```rust
  pub fn with_retries(self: Self, max_retries: usize) -> Self { /* ... */ }
  ```
  Sets the number of times the API will retry to download a file

- ```rust
  pub fn with_user_agent(self: Self, key: &str, value: &str) -> Self { /* ... */ }
  ```
  Adds custom fields to headers user-agent

- ```rust
  pub fn build(self: Self) -> Result<Api, ApiError> { /* ... */ }
  ```
  Consumes the builder and buids the final [`Api`]

###### Trait Implementations

- **Unpin**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **Instrument**
- **Send**
- **MaybeSendSync**
- **UnwindSafe**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **WithSubscriber**
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

#### Struct `Api`

The actual Api used to interacto with the hub.
Use any repo with [`Api::repo`]

```rust
pub struct Api {
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
  pub fn new() -> Result<Self, ApiError> { /* ... */ }
  ```
  Creates a default Api, for Api options See [`ApiBuilder`]

- ```rust
  pub fn client(self: &Self) -> &HeaderAgent { /* ... */ }
  ```
  Get the underlying api client

- ```rust
  pub fn repo(self: &Self, repo: Repo) -> ApiRepo { /* ... */ }
  ```
  Creates a new handle [`ApiRepo`] which contains operations

- ```rust
  pub fn model(self: &Self, model_id: String) -> ApiRepo { /* ... */ }
  ```
  Simple wrapper over

- ```rust
  pub fn dataset(self: &Self, model_id: String) -> ApiRepo { /* ... */ }
  ```
  Simple wrapper over

- ```rust
  pub fn space(self: &Self, model_id: String) -> ApiRepo { /* ... */ }
  ```
  Simple wrapper over

###### Trait Implementations

- **MaybeSendSync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Send**
- **Freeze**
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

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **Instrument**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **WithSubscriber**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Api { /* ... */ }
    ```

#### Struct `ApiRepo`

Shorthand for accessing things within a particular repo
You can inspect repos with [`ApiRepo::info`]
or download files with [`ApiRepo::download`]

```rust
pub struct ApiRepo {
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
  pub fn url(self: &Self, filename: &str) -> String { /* ... */ }
  ```
  Get the fully qualified URL of the remote filename

- ```rust
  pub fn get(self: &Self, filename: &str) -> Result<PathBuf, ApiError> { /* ... */ }
  ```
  This will attempt the fetch the file locally first, then [`Api.download`]

- ```rust
  pub fn download_with_progress<P: Progress>(self: &Self, filename: &str, progress: P) -> Result<PathBuf, ApiError> { /* ... */ }
  ```
  This function is used to download a file with a custom progress function.

- ```rust
  pub fn download(self: &Self, filename: &str) -> Result<PathBuf, ApiError> { /* ... */ }
  ```
  Downloads a remote file (if not already present) into the cache directory

- ```rust
  pub fn info(self: &Self) -> Result<RepoInfo, ApiError> { /* ... */ }
  ```
  Get information about the Repo

- ```rust
  pub fn info_request(self: &Self) -> Request { /* ... */ }
  ```
  Get the raw [`ureq::Request`] with the url and method already set

###### Trait Implementations

- **IntoEither**
- **RefUnwindSafe**
- **UnwindSafe**
- **Freeze**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **WithSubscriber**
- **ErasedDestructor**
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

- **Instrument**
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

- **MaybeSendSync**
### Types

#### Struct `Siblings`

Siblings are simplified file descriptions of remote files on the hub

```rust
pub struct Siblings {
    pub rfilename: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `rfilename` | `String` | The path within the repo. |

##### Implementations

###### Trait Implementations

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **MaybeSendSync**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Siblings { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Siblings) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **WithSubscriber**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **DeserializeOwned**
- **Send**
- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Instrument**
- **RefUnwindSafe**
- **ErasedDestructor**
#### Struct `RepoInfo`

The description of the repo given by the hub

```rust
pub struct RepoInfo {
    pub siblings: Vec<Siblings>,
    pub sha: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `siblings` | `Vec<Siblings>` | See [`Siblings`] |
| `sha` | `String` | The commit sha of the repo. |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RepoInfo { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
    fn eq(self: &Self, other: &RepoInfo) -> bool { /* ... */ }
    ```

- **Instrument**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **IntoEither**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **Sync**
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

- **DeserializeOwned**
- **WithSubscriber**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Traits

#### Trait `Progress`

This trait is used by users of the lib
to implement custom behavior during file downloads

```rust
pub trait Progress {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `init`: At the start of the download
- `update`: This function is called whenever `size` bytes have been
- `finish`: This is called at the end of the download

##### Implementations

This trait is implemented for the following types:

- `()`
- `indicatif::ProgressBar`

## Types

### Enum `RepoType`

The type of repo to interact with

```rust
pub enum RepoType {
    Model,
    Dataset,
    Space,
}
```

#### Variants

##### `Model`

This is a model, usually it consists of weight files and some configuration
files

##### `Dataset`

This is a dataset, usually contains data within parquet files

##### `Space`

This is a space, usually a demo showcashing a given model or dataset

#### Implementations

##### Trait Implementations

- **IntoEither**
- **Freeze**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
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
- **WithSubscriber**
- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> RepoType { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Instrument**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
### Struct `Cache`

A local struct used to fetch information from the cache folder.

```rust
pub struct Cache {
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
  pub fn new(path: PathBuf) -> Self { /* ... */ }
  ```
  Creates a new cache object location

- ```rust
  pub fn from_env() -> Self { /* ... */ }
  ```
  Creates cache from environment variable HF_HOME (if defined) otherwise

- ```rust
  pub fn path(self: &Self) -> &PathBuf { /* ... */ }
  ```
  Creates a new cache object location

- ```rust
  pub fn token_path(self: &Self) -> PathBuf { /* ... */ }
  ```
  Returns the location of the token file

- ```rust
  pub fn token(self: &Self) -> Option<String> { /* ... */ }
  ```
  Returns the token value if it exists in the cache

- ```rust
  pub fn repo(self: &Self, repo: Repo) -> CacheRepo { /* ... */ }
  ```
  Creates a new handle [`CacheRepo`] which contains operations

- ```rust
  pub fn model(self: &Self, model_id: String) -> CacheRepo { /* ... */ }
  ```
  Simple wrapper over

- ```rust
  pub fn dataset(self: &Self, model_id: String) -> CacheRepo { /* ... */ }
  ```
  Simple wrapper over

- ```rust
  pub fn space(self: &Self, model_id: String) -> CacheRepo { /* ... */ }
  ```
  Simple wrapper over

##### Trait Implementations

- **WithSubscriber**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Cache { /* ... */ }
    ```

- **Send**
- **IntoEither**
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

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
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

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Instrument**
### Struct `CacheRepo`

Shorthand for accessing things within a particular repo

```rust
pub struct CacheRepo {
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
  pub fn get(self: &Self, filename: &str) -> Option<PathBuf> { /* ... */ }
  ```
  This will get the location of the file within the cache for the remote

- ```rust
  pub fn create_ref(self: &Self, commit_hash: &str) -> Result<(), std::io::Error> { /* ... */ }
  ```
  Creates a reference in the cache directory that points branches to the correct

##### Trait Implementations

- **Sync**
- **Instrument**
- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **WithSubscriber**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **MaybeSendSync**
- **ErasedDestructor**
### Struct `Repo`

The representation of a repo on the hub.

```rust
pub struct Repo {
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
  pub fn new(repo_id: String, repo_type: RepoType) -> Self { /* ... */ }
  ```
  Repo with the default branch ("main").

- ```rust
  pub fn with_revision(repo_id: String, repo_type: RepoType, revision: String) -> Self { /* ... */ }
  ```
  fully qualified Repo

- ```rust
  pub fn model(repo_id: String) -> Self { /* ... */ }
  ```
  Shortcut for [`Repo::new`] with [`RepoType::Model`]

- ```rust
  pub fn dataset(repo_id: String) -> Self { /* ... */ }
  ```
  Shortcut for [`Repo::new`] with [`RepoType::Dataset`]

- ```rust
  pub fn space(repo_id: String) -> Self { /* ... */ }
  ```
  Shortcut for [`Repo::new`] with [`RepoType::Space`]

- ```rust
  pub fn folder_name(self: &Self) -> String { /* ... */ }
  ```
  The normalized folder nameof the repo within the cache directory

- ```rust
  pub fn revision(self: &Self) -> &str { /* ... */ }
  ```
  The revision

- ```rust
  pub fn url(self: &Self) -> String { /* ... */ }
  ```
  The actual URL part of the repo

- ```rust
  pub fn url_revision(self: &Self) -> String { /* ... */ }
  ```
  Revision needs to be url escaped before being used in a URL

- ```rust
  pub fn api_url(self: &Self) -> String { /* ... */ }
  ```
  Used to compute the repo's url part when accessing the metadata of the repo

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
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

- **WithSubscriber**
- **Instrument**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Repo { /* ... */ }
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

