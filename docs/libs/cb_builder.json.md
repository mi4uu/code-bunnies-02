# Crate Documentation

**Version:** 0.1.0

**Format Version:** 43

# Module `cb_builder`

## Modules

## Module `project`

```rust
pub mod project { /* ... */ }
```

### Types

#### Struct `Project`

```rust
pub struct Project {
    pub name: String,
    pub dir: std::path::PathBuf,
    pub cwd: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` |  |
| `dir` | `std::path::PathBuf` |  |
| `cwd` | `String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn path_is_allowed(self: &Self, path: &PathBuf) -> bool { /* ... */ }
  ```

- ```rust
  pub fn add_success(self: &Self, action: &str, result: &str) -> ActionResult { /* ... */ }
  ```

- ```rust
  pub fn add_error(self: &Self, action: &str, result: &str) -> ActionResult { /* ... */ }
  ```

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **WithSubscriber**
- **Same**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **MaybeSendSync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DeserializeOwned**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Instrument**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Project { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Unpin**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
#### Struct `ActionResult`

```rust
pub struct ActionResult {
    pub project: Project,
    pub action: String,
    pub result: String,
    pub is_error: bool,
    pub is_success: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `project` | `Project` |  |
| `action` | `String` |  |
| `result` | `String` |  |
| `is_error` | `bool` |  |
| `is_success` | `bool` |  |

##### Implementations

###### Trait Implementations

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **ErasedDestructor**
- **Same**
- **MaybeSendSync**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ActionResult { /* ... */ }
    ```

- **Sync**
- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **WithSubscriber**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DeserializeOwned**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Instrument**
## Module `providers`

```rust
pub mod providers { /* ... */ }
```

### Modules

## Module `config`

```rust
pub mod config { /* ... */ }
```

### Types

#### Enum `ProviderError`

```rust
pub enum ProviderError {
    Configuration(String),
    RequestPreparation(String),
    ApiCall(String),
    ResponseParsing(String),
}
```

##### Variants

###### `Configuration`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `RequestPreparation`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `ApiCall`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `ResponseParsing`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ProviderError { /* ... */ }
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

- **Same**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **ErasedDestructor**
- **WithSubscriber**
- **Instrument**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Error**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Send**
#### Struct `ProviderConfig`

```rust
pub struct ProviderConfig {
    pub api_type: super::provider::ApiType,
    pub api_key: Option<String>,
    pub api_base_url: Option<String>,
    pub model: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u16>,
    pub schema: Option<String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `api_type` | `super::provider::ApiType` |  |
| `api_key` | `Option<String>` |  |
| `api_base_url` | `Option<String>` |  |
| `model` | `String` |  |
| `temperature` | `Option<f32>` |  |
| `max_tokens` | `Option<u16>` |  |
| `schema` | `Option<String>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(api_type: ApiType, model: String) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_api_key(self: Self, api_key: String) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_api_base_url(self: Self, api_base_url: String) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_temperature(self: Self, temperature: f32) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_max_tokens(self: Self, max_tokens: u16) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_schema(self: Self, schema: String) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **WithSubscriber**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **MaybeSendSync**
- **Unpin**
- **RefUnwindSafe**
- **Sync**
- **UnwindSafe**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Freeze**
- **Instrument**
- **Same**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ProviderConfig { /* ... */ }
    ```

## Module `models`

```rust
pub mod models { /* ... */ }
```

### Types

#### Struct `Models`

```rust
pub struct Models {
    pub name: String,
    pub model: String,
    pub params: Option<ModelParams>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` |  |
| `model` | `String` |  |
| `params` | `Option<ModelParams>` |  |

##### Implementations

###### Trait Implementations

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
    fn eq(self: &Self, other: &Models) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **Same**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Models { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(model: &str) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(model: String) -> Self { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Instrument**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Models { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **StructuralPartialEq**
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

- **IntoEither**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DeserializeOwned**
- **WithSubscriber**
#### Struct `ModelParams`

```rust
pub struct ModelParams {
    pub ctx: Option<i32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
    pub max_tokens: Option<i32>,
    pub stop: Option<String>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub n: Option<i32>,
    pub stream: Option<bool>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ctx` | `Option<i32>` |  |
| `temperature` | `Option<f32>` |  |
| `top_p` | `Option<f32>` |  |
| `top_k` | `Option<i32>` |  |
| `max_tokens` | `Option<i32>` |  |
| `stop` | `Option<String>` |  |
| `presence_penalty` | `Option<f32>` |  |
| `frequency_penalty` | `Option<f32>` |  |
| `n` | `Option<i32>` |  |
| `stream` | `Option<bool>` |  |

##### Implementations

###### Trait Implementations

- **WithSubscriber**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **Instrument**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ModelParams { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DeserializeOwned**
- **StructuralPartialEq**
- **Same**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ModelParams) -> bool { /* ... */ }
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **RefUnwindSafe**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ModelParams { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
## Module `openai`

```rust
pub mod openai { /* ... */ }
```

### Types

#### Struct `OpenAiProvider`

```rust
pub struct OpenAiProvider {
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
  pub fn new(config: Arc<ProviderConfig>) -> Result<Self, ProviderError> { /* ... */ }
  ```

- ```rust
  pub async fn chat_completion(self: &Self, messages: Vec<ChatMessage>, model_override: Option<String>, temperature_override: Option<f32>, max_tokens_override: Option<u32>, response_schema: Option<serde_json::Value>) -> Result<String, ProviderError> { /* ... */ }
  ```

###### Trait Implementations

- **Instrument**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **WithSubscriber**
- **UnwindSafe**
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

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **IntoEither**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Same**
- **Unpin**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OpenAiProvider { /* ... */ }
    ```

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

#### Struct `ChatMessage`

```rust
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `role` | `String` |  |
| `content` | `String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn system</* synthetic */ impl Into<String>: Into<String>>(content: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn user</* synthetic */ impl Into<String>: Into<String>>(content: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn assistant</* synthetic */ impl Into<String>: Into<String>>(content: impl Into<String>) -> Self { /* ... */ }
  ```

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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **DeserializeOwned**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Same**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
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

- **WithSubscriber**
- **Freeze**
- **UnwindSafe**
- **Sync**
- **ErasedDestructor**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Instrument**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ChatMessage { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

## Module `provider`

```rust
pub mod provider { /* ... */ }
```

### Types

#### Struct `Provider`

```rust
pub struct Provider {
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    pub provider_type: ProviderType,
    pub api_type: ApiType,
    pub models: Vec<super::models::Models>,
    pub default_model: super::models::Models,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` |  |
| `base_url` | `String` |  |
| `api_key` | `String` |  |
| `provider_type` | `ProviderType` |  |
| `api_type` | `ApiType` |  |
| `models` | `Vec<super::models::Models>` |  |
| `default_model` | `super::models::Models` |  |

##### Implementations

###### Methods

- ```rust
  pub fn provider(provider_type: ProviderType) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_model(self: &Self, model_name: &str) -> Option<ModelBuilder> { /* ... */ }
  ```

- ```rust
  pub fn with_default_model(self: &Self) -> ModelBuilder { /* ... */ }
  ```

###### Trait Implementations

- **WithSubscriber**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Provider { /* ... */ }
    ```

- **Send**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **StructuralPartialEq**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Sync**
- **Unpin**
- **DeserializeOwned**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Provider) -> bool { /* ... */ }
    ```

- **ErasedDestructor**
- **MaybeSendSync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Instrument**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Same**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Enum `ProviderType`

```rust
pub enum ProviderType {
    Anthropic,
    Cerebras,
    DeepSeek,
    Groq,
    OpenAI,
    Llamafile,
    Ollama,
    LmStudio,
    XAI,
}
```

##### Variants

###### `Anthropic`

###### `Cerebras`

###### `DeepSeek`

###### `Groq`

###### `OpenAI`

###### `Llamafile`

###### `Ollama`

###### `LmStudio`

###### `XAI`

##### Implementations

###### Trait Implementations

- **Sync**
- **Instrument**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(x: ProviderType) -> &''static str { /* ... */ }
    ```

  - ```rust
    fn from(x: &''_derivative_strum ProviderType) -> &''static str { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Same**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ProviderType) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::result::Result<(), ::core::fmt::Error> { /* ... */ }
    ```

- **Send**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ProviderType { /* ... */ }
    ```

- **MaybeSendSync**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> ::core::result::Result<ProviderType, <Self as ::core::str::FromStr>::Err> { /* ... */ }
    ```

- **DeserializeOwned**
- **RefUnwindSafe**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(s: &str) -> ::core::result::Result<ProviderType, <Self as ::core::convert::TryFrom<&str>>::Error> { /* ... */ }
    ```

- **WithSubscriber**
- **StructuralPartialEq**
- **IntoEither**
- **Default**
  - ```rust
    fn default() -> ProviderType { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Enum `ApiType`

```rust
pub enum ApiType {
    Anthropic,
    OpenAI,
}
```

##### Variants

###### `Anthropic`

###### `OpenAI`

##### Implementations

###### Trait Implementations

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(x: ApiType) -> &''static str { /* ... */ }
    ```

  - ```rust
    fn from(x: &''_derivative_strum ApiType) -> &''static str { /* ... */ }
    ```

- **StructuralPartialEq**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ApiType) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ApiType { /* ... */ }
    ```

- **DeserializeOwned**
- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::result::Result<(), ::core::fmt::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Instrument**
- **Default**
  - ```rust
    fn default() -> ApiType { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(s: &str) -> ::core::result::Result<ApiType, <Self as ::core::convert::TryFrom<&str>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Same**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> ::core::result::Result<ApiType, <Self as ::core::str::FromStr>::Err> { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **WithSubscriber**
- **ErasedDestructor**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `ModelBuilder`

```rust
pub struct ModelBuilder {
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
  pub fn with_temperature(self: Self, temperature: f32) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_max_tokens(self: Self, max_tokens: u16) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_ctx(self: Self, ctx_size: u16) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_top_p(self: Self, top_p: f32) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_top_k(self: Self, top_k: u16) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_schema(self: Self, schema: String) -> Self { /* ... */ }
  ```

- ```rust
  pub fn build(self: &Self) -> Result<OpenAiProvider, ProviderError> { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **Send**
- **RefUnwindSafe**
- **Freeze**
- **Sync**
- **Unpin**
- **ErasedDestructor**
- **Same**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Instrument**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **WithSubscriber**
## Module `providers`

```rust
pub mod providers { /* ... */ }
```

### Types

#### Struct `Providers`

```rust
pub struct Providers {
    pub providers: Vec<super::provider::Provider>,
    pub default_provider: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `providers` | `Vec<super::provider::Provider>` |  |
| `default_provider` | `String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn get_config_path() -> PathBuf { /* ... */ }
  ```

- ```rust
  pub fn save(self: &Self) { /* ... */ }
  ```

- ```rust
  pub fn load() -> Self { /* ... */ }
  ```

- ```rust
  pub fn get_by_name(self: &Self, name: &str) -> Option<&Provider> { /* ... */ }
  ```

- ```rust
  pub fn get_by_name_mut(self: &mut Self, name: &str) -> Option<&mut Provider> { /* ... */ }
  ```

- ```rust
  pub fn get_default(self: &Self) -> Option<&Provider> { /* ... */ }
  ```

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **WithSubscriber**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
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
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Providers) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Instrument**
- **IntoEither**
- **Same**
- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Send**
- **Default**
  - ```rust
    fn default() -> Providers { /* ... */ }
    ```

- **DeserializeOwned**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Providers { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `tools`

```rust
pub mod tools { /* ... */ }
```

### Modules

## Module `agent_response`

```rust
pub mod agent_response { /* ... */ }
```

### Types

#### Struct `SearchWeb`

```rust
pub struct SearchWeb {
    pub query: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `query` | `String` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Instrument**
- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SearchWeb { /* ... */ }
    ```

- **Freeze**
- **WithSubscriber**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DeserializeOwned**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Same**
- **Unpin**
- **IntoEither**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Sync**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
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

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `UserAssistanceNeeded`

```rust
pub struct UserAssistanceNeeded {
    pub message: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `message` | `String` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **DeserializeOwned**
- **Send**
- **RefUnwindSafe**
- **WithSubscriber**
- **Instrument**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Same**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

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

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **UnwindSafe**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> UserAssistanceNeeded { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

#### Enum `AgentActions`

**Attributes:**

- `#[serde(tag = "action_type", rename_all = "snake_case")]`

```rust
pub enum AgentActions {
    Fs(super::fs::FsActions),
    Memory(super::memory::MemoryAction),
    SearchWeb(SearchWeb),
    UserAssistanceNeeded(UserAssistanceNeeded),
}
```

##### Variants

###### `Fs`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `super::fs::FsActions` |  |

###### `Memory`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `super::memory::MemoryAction` |  |

###### `SearchWeb`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `SearchWeb` |  |

###### `UserAssistanceNeeded`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `UserAssistanceNeeded` |  |

##### Implementations

###### Methods

- ```rust
  pub const fn is_fs(self: &Self) -> bool { /* ... */ }
  ```
  Returns [true] if the enum is [AgentActions::Fs] otherwise [false]

- ```rust
  pub const fn is_memory(self: &Self) -> bool { /* ... */ }
  ```
  Returns [true] if the enum is [AgentActions::Memory] otherwise [false]

- ```rust
  pub const fn is_search_web(self: &Self) -> bool { /* ... */ }
  ```
  Returns [true] if the enum is [AgentActions::SearchWeb] otherwise [false]

- ```rust
  pub const fn is_user_assistance_needed(self: &Self) -> bool { /* ... */ }
  ```
  Returns [true] if the enum is [AgentActions::UserAssistanceNeeded] otherwise [false]

###### Trait Implementations

- **Unpin**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Same**
- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **WithSubscriber**
- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DeserializeOwned**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AgentActions { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Instrument**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

#### Struct `AgentResponse`

```rust
pub struct AgentResponse {
    pub reasoning: Vec<String>,
    pub actions: Vec<AgentActions>,
    pub summary: Option<String>,
    pub expected: String,
    pub next: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `reasoning` | `Vec<String>` |  |
| `actions` | `Vec<AgentActions>` |  |
| `summary` | `Option<String>` |  |
| `expected` | `String` |  |
| `next` | `String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn schema() -> serde_json::Value { /* ... */ }
  ```

- ```rust
  pub fn format() -> String { /* ... */ }
  ```

###### Trait Implementations

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AgentResponse { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Instrument**
- **Same**
- **WithSubscriber**
- **IntoEither**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **DeserializeOwned**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **MaybeSendSync**
- **UnwindSafe**
- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **ErasedDestructor**
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

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
## Module `create_file`

```rust
pub mod create_file { /* ... */ }
```

### Functions

#### Function `get_props`

```rust
pub fn get_props() -> std::collections::HashMap<std::string::String, Box<openai_api_rs::v1::types::JSONSchemaDefine>> { /* ... */ }
```

#### Function `get_tool`

```rust
pub fn get_tool() -> chat_completion::Tool { /* ... */ }
```

## Module `fs`

```rust
pub mod fs { /* ... */ }
```

### Types

#### Struct `ReadFile`

```rust
pub struct ReadFile {
    pub file_path: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `file_path` | `String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn signature(self: &Self) -> String { /* ... */ }
  ```

- ```rust
  pub fn exec(self: &Self, project: Project) -> ActionResult { /* ... */ }
  ```

###### Trait Implementations

- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ReadFile { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Instrument**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Same**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **MaybeSendSync**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **WithSubscriber**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **DeserializeOwned**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
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

#### Struct `WriteFile`

```rust
pub struct WriteFile {
    pub file_path: String,
    pub content: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `file_path` | `String` |  |
| `content` | `String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn signature(self: &Self) -> String { /* ... */ }
  ```

- ```rust
  pub fn exec(self: &Self, project: Project) -> ActionResult { /* ... */ }
  ```

###### Trait Implementations

- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **Instrument**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> WriteFile { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **WithSubscriber**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DeserializeOwned**
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

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Same**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
#### Struct `ApplyPatchToFile`

```rust
pub struct ApplyPatchToFile {
    pub file_path: String,
    pub patch: String,
    pub start_line: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `file_path` | `String` |  |
| `patch` | `String` |  |
| `start_line` | `usize` |  |

##### Implementations

###### Trait Implementations

- **Same**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **Instrument**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **MaybeSendSync**
- **WithSubscriber**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ApplyPatchToFile { /* ... */ }
    ```

- **IntoEither**
- **UnwindSafe**
- **DeserializeOwned**
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

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `DiffFiles`

```rust
pub struct DiffFiles {
    pub file_0_path: String,
    pub file_1_path: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `file_0_path` | `String` |  |
| `file_1_path` | `String` |  |

##### Implementations

###### Trait Implementations

- **Instrument**
- **Sync**
- **UnwindSafe**
- **ErasedDestructor**
- **MaybeSendSync**
- **RefUnwindSafe**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Same**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **DeserializeOwned**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **WithSubscriber**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DiffFiles { /* ... */ }
    ```

#### Struct `DirLs`

```rust
pub struct DirLs {
    pub dir_path: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `dir_path` | `String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn signature(self: &Self) -> String { /* ... */ }
  ```

- ```rust
  pub fn exec(self: &Self, project: Project) -> ActionResult { /* ... */ }
  ```

###### Trait Implementations

- **DeserializeOwned**
- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Same**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **WithSubscriber**
- **MaybeSendSync**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **Instrument**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DirLs { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Sync**
- **ErasedDestructor**
- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

#### Struct `CD`

```rust
pub struct CD {
    pub dir_path: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `dir_path` | `String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn signature(self: &Self) -> String { /* ... */ }
  ```

- ```rust
  pub fn exec(self: &Self, project: Project) -> ActionResult { /* ... */ }
  ```

###### Trait Implementations

- **Send**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CD { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DeserializeOwned**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Freeze**
- **Instrument**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **WithSubscriber**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **Same**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
#### Struct `Pwd`

```rust
pub struct Pwd {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn signature(self: &Self) -> String { /* ... */ }
  ```

- ```rust
  pub fn exec(self: &Self, project: Project) -> ActionResult { /* ... */ }
  ```

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **WithSubscriber**
- **Instrument**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **DeserializeOwned**
- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Same**
- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Pwd { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Enum `FsActions`

**Attributes:**

- `#[serde(tag = "action_type", rename_all = "snake_case")]`

```rust
pub enum FsActions {
    ReadFile(ReadFile),
    WriteFile(WriteFile),
    DirLs(DirLs),
    DiffFiles(DiffFiles),
    ApplyPatchToFile(ApplyPatchToFile),
    CD(CD),
    Pwd(Pwd),
}
```

##### Variants

###### `ReadFile`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ReadFile` |  |

###### `WriteFile`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WriteFile` |  |

###### `DirLs`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `DirLs` |  |

###### `DiffFiles`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `DiffFiles` |  |

###### `ApplyPatchToFile`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ApplyPatchToFile` |  |

###### `CD`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `CD` |  |

###### `Pwd`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Pwd` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FsActions { /* ... */ }
    ```

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **IntoEither**
- **WithSubscriber**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Same**
- **Instrument**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **RefUnwindSafe**
- **DeserializeOwned**
- **ErasedDestructor**
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

## Module `get_price`

```rust
pub mod get_price { /* ... */ }
```

### Functions

#### Function `get_props`

```rust
pub fn get_props() -> std::collections::HashMap<std::string::String, Box<openai_api_rs::v1::types::JSONSchemaDefine>> { /* ... */ }
```

#### Function `get_tool`

```rust
pub fn get_tool() -> chat_completion::Tool { /* ... */ }
```

## Module `memory`

```rust
pub mod memory { /* ... */ }
```

### Types

#### Struct `Memory`

```rust
pub struct Memory {
    pub action: MemoryAction,
    pub params: MemoryParams,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `action` | `MemoryAction` |  |
| `params` | `MemoryParams` |  |

##### Implementations

###### Trait Implementations

- **DeserializeOwned**
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
- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **Same**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **WithSubscriber**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Instrument**
- **IntoEither**
- **Unpin**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Freeze**
#### Enum `MemoryAction`

**Attributes:**

- `#[serde(rename_all = "snake_case")]`

```rust
pub enum MemoryAction {
    List,
    Find,
    Forget,
    Store,
}
```

##### Variants

###### `List`

###### `Find`

###### `Forget`

###### `Store`

##### Implementations

###### Trait Implementations

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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
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

- **Freeze**
- **Instrument**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **WithSubscriber**
- **Same**
- **DeserializeOwned**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> MemoryAction { /* ... */ }
    ```

#### Enum `MemoryParams`

**Attributes:**

- `#[serde(untagged)]`

```rust
pub enum MemoryParams {
    ListParams {
    },
    FindParams {
        query: String,
    },
    ForgetParams {
        id: String,
    },
    StoreParams {
        content: String,
        id: String,
    },
}
```

##### Variants

###### `ListParams`

Fields:

| Name | Type | Documentation |
|------|------|---------------|

###### `FindParams`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `query` | `String` |  |

###### `ForgetParams`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `id` | `String` |  |

###### `StoreParams`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `content` | `String` |  |
| `id` | `String` |  |

##### Implementations

###### Trait Implementations

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MemoryParams { /* ... */ }
    ```

- **WithSubscriber**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Instrument**
- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
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

- **Same**
- **DeserializeOwned**
- **Send**
