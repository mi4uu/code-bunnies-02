# Crate Documentation

**Version:** 0.1.1

**Format Version:** 43

# Module `file_retriever`

Asyncronous download with (optional) progress bar and limited amount of workers.

Retriever is based on tokio and reqwest crates dancing together in a beautiful tango.

## Types

### Struct `RetrieverBuilder`

Factory which is used to configure the properties of a new Retriever.

# Examples

```
use reqwest::Client;
use file_retriever::RetrieverBuilder;
use tokio::fs::OpenOptions;

#[tokio::main]
async fn main() {
    // build a retriever
    let retriever = RetrieverBuilder::new()
        .show_progress(true)
        .workers(42)
        .build();

    // open a file to write to
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("index.html"))
        .await
        .expect("should return file");

    // setup a request to retrieve the file
    let req = Client::new().get("https://example.com").build().unwrap();

    // download a file
    let _  = retriever.download_file(req, file).await;
}
```

```rust
pub struct RetrieverBuilder {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a new Retriever builder.

- ```rust
  pub fn show_progress(self: Self, show_progress_bar: bool) -> Self { /* ... */ }
  ```
  Sets if progress bar will be shown.

- ```rust
  pub fn progress_style(self: Self, pb_style: ProgressStyle) -> Self { /* ... */ }
  ```
  Sets progress bar style.

- ```rust
  pub fn workers(self: Self, workers: usize) -> Self { /* ... */ }
  ```
  Sets the number of workers.

- ```rust
  pub fn build(self: Self) -> Retriever { /* ... */ }
  ```
  Creates a Retriever with the configured options.

##### Trait Implementations

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Instrument**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **ErasedDestructor**
- **Sync**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```
    Creates a new Retriever builder.

- **WithSubscriber**
- **UnwindSafe**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `Retriever`

Provies an easy interface for parallel downloads with limited workers and progress bar

# Examples

```
use reqwest::Client;
use file_retriever::Retriever;
use tokio::fs::OpenOptions;

#[tokio::main]
async fn main() {
    // create a retriever
    let retriever = Retriever::with_progress_bar();

    // open a file to write to
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("index.html"))
        .await
        .expect("should return file");

    // setup a request to retrieve the file
    let req = Client::new().get("https://example.com").build().unwrap();

    // download a file
    let _  = retriever.download_file(req, file).await;
}
```

```rust
pub struct Retriever {
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
  pub fn with_progress_bar() -> Self { /* ... */ }
  ```
  Same as default retriever but showing progress bar

- ```rust
  pub async fn download_file<W>(self: &Self, request: Request, writer: W) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    W: AsyncWriteExt + Unpin + Send + Sync + ''static { /* ... */ }
  ```
  Makes a request using a request and writes output into writer

##### Trait Implementations

- **MaybeSendSync**
- **Instrument**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Freeze**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **WithSubscriber**
- **IntoEither**
- **UnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```
    Create a default retriever with 10 workers

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

