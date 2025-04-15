# Crate Documentation

**Version:** 2.2.6

**Format Version:** 43

# Module `trauma`

Trauma is crate aiming at providing a simple way to download files
asynchronously via HTTP(S).

## Modules

## Module `download`

Represents a file to be downloaded.

```rust
pub mod download { /* ... */ }
```

### Types

#### Struct `Download`

Represents a file to be downloaded.

```rust
pub struct Download {
    pub url: reqwest::Url,
    pub filename: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `url` | `reqwest::Url` | URL of the file to download. |
| `filename` | `String` | File name used to save the file on disk. |

##### Implementations

###### Methods

- ```rust
  pub fn new(url: &Url, filename: &str) -> Self { /* ... */ }
  ```
  Creates a new [`Download`].

- ```rust
  pub async fn is_resumable(self: &Self, client: &ClientWithMiddleware) -> Result<bool, reqwest_middleware::Error> { /* ... */ }
  ```
  Check whether the download is resumable.

- ```rust
  pub async fn content_length(self: &Self, client: &ClientWithMiddleware) -> Result<Option<u64>, reqwest_middleware::Error> { /* ... */ }
  ```
  Retrieve the content_length of the download.

###### Trait Implementations

- **Instrument**
- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **FutureExt**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Download { /* ... */ }
    ```

- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: &Url) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: &str) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
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

- **ErasedDestructor**
- **WithSubscriber**
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

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Enum `Status`

```rust
pub enum Status {
    Fail(String),
    NotStarted,
    Skipped(String),
    Success,
}
```

##### Variants

###### `Fail`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `NotStarted`

###### `Skipped`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Success`

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **StructuralPartialEq**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Instrument**
- **WithSubscriber**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Status { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Status) -> bool { /* ... */ }
    ```

- **Sync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **FutureExt**
- **MaybeSendSync**
- **ErasedDestructor**
#### Struct `Summary`

Represents a [`Download`] summary.

```rust
pub struct Summary {
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
  pub fn new(download: Download, statuscode: StatusCode, size: u64, resumable: bool) -> Self { /* ... */ }
  ```
  Create a new [`Download`] [`Summary`].

- ```rust
  pub fn with_status(self: Self, status: Status) -> Self { /* ... */ }
  ```
  Attach a status to a [`Download`] [`Summary`].

- ```rust
  pub fn statuscode(self: &Self) -> StatusCode { /* ... */ }
  ```
  Get the summary's status.

- ```rust
  pub fn size(self: &Self) -> u64 { /* ... */ }
  ```
  Get the summary's size.

- ```rust
  pub fn download(self: &Self) -> &Download { /* ... */ }
  ```
  Get a reference to the summary's download.

- ```rust
  pub fn status(self: &Self) -> &Status { /* ... */ }
  ```
  Get a reference to the summary's status.

- ```rust
  pub fn fail</* synthetic */ impl std::fmt::Display: std::fmt::Display>(self: Self, msg: impl std::fmt::Display) -> Self { /* ... */ }
  ```

- ```rust
  pub fn set_resumable(self: &mut Self, resumable: bool) { /* ... */ }
  ```
  Set the summary's resumable.

- ```rust
  pub fn resumable(self: &Self) -> bool { /* ... */ }
  ```
  Get the summary's resumable.

###### Trait Implementations

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
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

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **FutureExt**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Summary { /* ... */ }
    ```

- **Instrument**
- **ErasedDestructor**
- **WithSubscriber**
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

## Module `downloader`

Represents the download controller.

```rust
pub mod downloader { /* ... */ }
```

### Types

#### Struct `TimeTrace`

```rust
pub struct TimeTrace;
```

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

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

- **RefUnwindSafe**
- **Instrument**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
- **Unpin**
- **Send**
- **WithSubscriber**
- **IntoEither**
- **FutureExt**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `Downloader`

Represents the download controller.

A downloader can be created via its builder:

```rust
# fn main()  {
use trauma::downloader::DownloaderBuilder;

let d = DownloaderBuilder::new().build();
# }
```

```rust
pub struct Downloader {
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
  pub async fn download(self: &Self, downloads: &[Download]) -> Vec<Summary> { /* ... */ }
  ```
  Starts the downloads.

- ```rust
  pub async fn download_with_proxy(self: &Self, downloads: &[Download], proxy: reqwest::Proxy) -> Vec<Summary> { /* ... */ }
  ```
  Starts the downloads with proxy.

- ```rust
  pub async fn download_inner(self: &Self, downloads: &[Download], proxy: Option<reqwest::Proxy>) -> Vec<Summary> { /* ... */ }
  ```
  Starts the downloads.

###### Trait Implementations

- **UnwindSafe**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Downloader { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FutureExt**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **WithSubscriber**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Instrument**
#### Struct `DownloaderBuilder`

A builder used to create a [`Downloader`].

```rust
# fn main()  {
use trauma::downloader::DownloaderBuilder;

let d = DownloaderBuilder::new().retries(5).directory("downloads".into()).build();
# }
```

```rust
pub struct DownloaderBuilder(/* private field */);
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
  Creates a builder with the default options.

- ```rust
  pub fn hidden() -> Self { /* ... */ }
  ```
  Convenience function to hide the progress bars.

- ```rust
  pub fn directory(self: Self, directory: PathBuf) -> Self { /* ... */ }
  ```
  Sets the directory where to store the [`Download`]s.

- ```rust
  pub fn retries(self: Self, retries: u32) -> Self { /* ... */ }
  ```
  Set the number of retries per [`Download`].

- ```rust
  pub fn concurrent_downloads(self: Self, concurrent_downloads: usize) -> Self { /* ... */ }
  ```
  Set the number of concurrent [`Download`]s.

- ```rust
  pub fn style_options(self: Self, style_options: StyleOptions) -> Self { /* ... */ }
  ```
  Set the downloader style options.

- ```rust
  pub fn headers(self: Self, headers: HeaderMap) -> Self { /* ... */ }
  ```
  Add the http headers.

- ```rust
  pub fn header<K: IntoHeaderName>(self: Self, name: K, value: HeaderValue) -> Self { /* ... */ }
  ```
  Add the http header

- ```rust
  pub fn build(self: Self) -> Downloader { /* ... */ }
  ```
  Create the [`Downloader`] with the specified options.

###### Trait Implementations

- **IntoEither**
- **Instrument**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **WithSubscriber**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FutureExt**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
#### Struct `StyleOptions`

Define the [`Downloader`] options.

By default, the main progress bar will stay on the screen upon completion,
but the child ones will be cleared once complete.

```rust
pub struct StyleOptions {
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
  pub fn new(main: ProgressBarOpts, child: ProgressBarOpts) -> Self { /* ... */ }
  ```
  Create new [`Downloader`] [`StyleOptions`].

- ```rust
  pub fn set_main(self: &mut Self, main: ProgressBarOpts) { /* ... */ }
  ```
  Set the options for the main progress bar.

- ```rust
  pub fn set_child(self: &mut Self, child: ProgressBarOpts) { /* ... */ }
  ```
  Set the options for the child progress bar.

- ```rust
  pub fn is_enabled(self: Self) -> bool { /* ... */ }
  ```
  Return `false` if neither the main nor the child bar is enabled.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StyleOptions { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **WithSubscriber**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Instrument**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **ErasedDestructor**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **FutureExt**
#### Struct `ProgressBarOpts`

Define the options for a progress bar.

```rust
pub struct ProgressBarOpts {
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
  pub fn new(template: Option<String>, progress_chars: Option<String>, enabled: bool, clear: bool) -> Self { /* ... */ }
  ```
  Create a new [`ProgressBarOpts`].

- ```rust
  pub fn to_progress_style(self: Self) -> ProgressStyle { /* ... */ }
  ```
  Create a [`ProgressStyle`] based on the provided options.

- ```rust
  pub fn to_progress_bar(self: Self, len: u64) -> ProgressBar { /* ... */ }
  ```
  Create a [`ProgressBar`] based on the provided options.

- ```rust
  pub fn with_pip_style() -> Self { /* ... */ }
  ```
  Create a new [`ProgressBarOpts`] which looks like Python pip.

- ```rust
  pub fn set_clear(self: &mut Self, clear: bool) { /* ... */ }
  ```
  Set to `true` to clear the progress bar upon completion.

- ```rust
  pub fn hidden() -> Self { /* ... */ }
  ```
  Create a new [`ProgressBarOpts`] which hides the progress bars.

###### Trait Implementations

- **RefUnwindSafe**
- **IntoEither**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ProgressBarOpts { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **WithSubscriber**
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
- **Unpin**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ErasedDestructor**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Instrument**
- **FutureExt**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
## Types

### Enum `Error`

Errors that can happen when using Trauma.

```rust
pub enum Error {
    Internal(String),
    InvalidUrl(String),
    IOError {
        source: io::Error,
    },
    Reqwest {
        source: reqwest::Error,
    },
}
```

#### Variants

##### `Internal`

Error from an underlying system.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### `InvalidUrl`

Error from the underlying URL parser or the expected URL format.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### `IOError`

I/O Error.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `source` | `io::Error` |  |

##### `Reqwest`

Error from the Reqwest library.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `source` | `reqwest::Error` |  |

#### Implementations

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Error**
  - ```rust
    fn source(self: &Self) -> ::core::option::Option<&dyn ::thiserror::__private::Error + ''static> { /* ... */ }
    ```

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **FutureExt**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Instrument**
- **Unpin**
- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: io::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(source: reqwest::Error) -> Self { /* ... */ }
    ```

- **WithSubscriber**
- **Freeze**
- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
