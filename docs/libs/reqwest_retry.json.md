# Crate Documentation

**Version:** 0.7.0

**Format Version:** 43

# Module `reqwest_retry`

Middleware to retry failed HTTP requests built on [`reqwest_middleware`].

Use [`RetryTransientMiddleware`] to retry failed HTTP requests. Retry control flow is managed
by a [`RetryPolicy`].

## Example

```
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

async fn run_retries() {
    // Retry up to 3 times with increasing intervals between attempts.
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    client
        .get("https://truelayer.com")
        .header("foo", "bar")
        .send()
        .await
        .unwrap();
}
```

## Types

### Enum `RetryError`

Custom error type to attach the number of retries to the error message.

```rust
pub enum RetryError {
    WithRetries {
        retries: u32,
        err: reqwest_middleware::Error,
    },
    Error(reqwest_middleware::Error),
}
```

#### Variants

##### `WithRetries`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `retries` | `u32` |  |
| `err` | `reqwest_middleware::Error` |  |

##### `Error`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `reqwest_middleware::Error` |  |

#### Implementations

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Instrument**
- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
- **Sync**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **WithSubscriber**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Error**
  - ```rust
    fn source(self: &Self) -> ::core::option::Option<&dyn std::error::Error + ''static> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

## Re-exports

### Re-export `policies`

```rust
pub use retry_policies::policies;
```

### Re-export `Jitter`

```rust
pub use retry_policies::Jitter;
```

### Re-export `RetryDecision`

```rust
pub use retry_policies::RetryDecision;
```

### Re-export `RetryPolicy`

```rust
pub use retry_policies::RetryPolicy;
```

### Re-export `RetryTransientMiddleware`

```rust
pub use middleware::RetryTransientMiddleware;
```

### Re-export `Retryable`

```rust
pub use retryable::Retryable;
```

### Re-export `default_on_request_failure`

```rust
pub use retryable_strategy::default_on_request_failure;
```

### Re-export `default_on_request_success`

```rust
pub use retryable_strategy::default_on_request_success;
```

### Re-export `DefaultRetryableStrategy`

```rust
pub use retryable_strategy::DefaultRetryableStrategy;
```

### Re-export `RetryableStrategy`

```rust
pub use retryable_strategy::RetryableStrategy;
```

