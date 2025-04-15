# Crate Documentation

**Version:** 1.5.0

**Format Version:** 43

# Module `icu_provider`

`icu_provider` is one of the [`ICU4X`] components.

Unicode's experience with ICU4X's parent projects, ICU4C and ICU4J, led the team to realize
that data management is the most critical aspect of deploying internationalization, and that it requires
a high level of customization for the needs of the platform it is embedded in. As a result
ICU4X comes with a selection of providers that should allow for ICU4X to naturally fit into
different business and technological needs of customers.

`icu_provider` defines traits and structs for transmitting data through the ICU4X locale
data pipeline. The primary trait is [`DataProvider`]. It is parameterized by a
[`KeyedDataMarker`], which contains the data type and a [`DataKey`]. It has one method,
[`DataProvider::load`], which transforms a [`DataRequest`]
into a [`DataResponse`].

- [`DataKey`] is a fixed identifier for the data type, such as `"plurals/cardinal@1"`.
- [`DataRequest`] contains additional annotations to choose a specific variant of the key,
  such as a locale.
- [`DataResponse`] contains the data if the request was successful.

In addition, there are three other traits which are widely implemented:

- [`AnyProvider`] returns data as `dyn Any` trait objects.
- [`BufferProvider`] returns data as `[u8]` buffers.
- [`DynamicDataProvider`] returns structured data but is not specific to a key.

The most common types required for this crate are included via the prelude:

```
use icu_provider::prelude::*;
```

## Types of Data Providers

All nontrivial data providers can fit into one of two classes.

1. [`AnyProvider`]: Those whose data originates as structured Rust objects
2. [`BufferProvider`]: Those whose data originates as unstructured `[u8]` buffers

**✨ Key Insight:** A given data provider is generally *either* an [`AnyProvider`] *or* a
[`BufferProvider`]. Which type depends on the data source, and it is not generally possible
to convert one to the other.

See also [crate::constructors].

### AnyProvider

These providers are able to return structured data cast into `dyn Any` trait objects. Users
can call [`as_downcasting()`] to get an object implementing [`DataProvider`] by downcasting
the trait objects.

Examples of AnyProviders:

- [`DatagenProvider`] reads structured data from CLDR source files and returns ICU4X data structs.
- [`AnyPayloadProvider`] wraps a specific data struct and returns it.
- The `BakedDataProvider` which encodes structured data directly in Rust source

### BufferProvider

These providers are able to return unstructured data typically represented as
[`serde`]-serialized buffers. Users can call [`as_deserializing()`] to get an object
implementing [`DataProvider`] by invoking Serde Deserialize.

Examples of BufferProviders:

- [`FsDataProvider`] reads individual buffers from the filesystem.
- [`BlobDataProvider`] reads buffers from a large in-memory blob.

## Provider Adapters

ICU4X offers several built-in modules to combine providers in interesting ways.
These can be found in the [`icu_provider_adapters`] crate.

## Testing Provider

This crate also contains a concrete provider for demonstration purposes:

- [`HelloWorldProvider`] returns "hello world" strings in several languages.

## Types and Lifetimes

Types compatible with [`Yokeable`] can be passed through the data provider, so long as they are
associated with a marker type implementing [`DataMarker`].

Data structs should generally have one lifetime argument: `'data`. This lifetime allows data
structs to borrow zero-copy data.

## Data generation API

*This functionality is enabled with the "datagen" Cargo feature*

The [`datagen`] module contains several APIs for data generation. See [`icu_datagen`] for the reference
data generation implementation.

[`ICU4X`]: ../icu/index.html
[`DataProvider`]: data_provider::DataProvider
[`DataKey`]: key::DataKey
[`DataLocale`]: request::DataLocale
[`IterableDynamicDataProvider`]: datagen::IterableDynamicDataProvider
[`IterableDataProvider`]: datagen::IterableDataProvider
[`AnyPayloadProvider`]: ../icu_provider_adapters/any_payload/struct.AnyPayloadProvider.html
[`HelloWorldProvider`]: hello_world::HelloWorldProvider
[`AnyProvider`]: any::AnyProvider
[`Yokeable`]: yoke::Yokeable
[`impl_dynamic_data_provider!`]: impl_dynamic_data_provider
[`icu_provider_adapters`]: ../icu_provider_adapters/index.html
[`DatagenProvider`]: ../icu_datagen/struct.DatagenProvider.html
[`as_downcasting()`]: AsDowncastingAnyProvider::as_downcasting
[`as_deserializing()`]: AsDeserializingBufferProvider::as_deserializing
[`CldrJsonDataProvider`]: ../icu_datagen/cldr/struct.CldrJsonDataProvider.html
[`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
[`BlobDataProvider`]: ../icu_provider_blob/struct.BlobDataProvider.html
[`icu_datagen`]: ../icu_datagen/index.html

## Modules

## Module `any`

Traits for data providers that produce `Any` objects.

```rust
pub mod any { /* ... */ }
```

### Types

#### Struct `AnyPayload`

A type-erased data payload.

The only useful method on this type is [`AnyPayload::downcast()`], which transforms this into
a normal `DataPayload` which you can subsequently access or mutate.

As with `DataPayload`, cloning is designed to be cheap.

```rust
pub struct AnyPayload {
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
  pub fn downcast<M>(self: Self) -> Result<DataPayload<M>, DataError>
where
    M: DataMarker,
    <M as >::Yokeable: ZeroFrom<''static, <M as >::Yokeable> + MaybeSendSync,
    for<''a> YokeTraitHack<<<M as >::Yokeable as Yokeable<''a>>::Output>: Clone { /* ... */ }
  ```
  Transforms a type-erased `AnyPayload` into a concrete `DataPayload<M>`.

- ```rust
  pub fn downcast_cloned<M>(self: &Self) -> Result<DataPayload<M>, DataError>
where
    M: DataMarker,
    <M as >::Yokeable: ZeroFrom<''static, <M as >::Yokeable> + MaybeSendSync,
    for<''a> YokeTraitHack<<<M as >::Yokeable as Yokeable<''a>>::Output>: Clone { /* ... */ }
  ```
  Clones and then transforms a type-erased `AnyPayload` into a concrete `DataPayload<M>`.

- ```rust
  pub fn from_static_ref<Y>(static_ref: &''static Y) -> Self
where
    Y: for<''a> Yokeable<''a> { /* ... */ }
  ```
  Creates an `AnyPayload` from a static reference to a data struct.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> AnyPayload { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Yokeable**
  - ```rust
    fn transform(self: &Self) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn transform_owned(self: Self) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    unsafe fn make(this: <Self as >::Output) -> Self { /* ... */ }
    ```

  - ```rust
    fn transform_mut<F>(self: &''a mut Self, f: F)
where
    F: ''static + for<''b> FnOnce(&''b mut <Self as >::Output) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **UnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `AnyMarker`

**Attributes:**

- `#[allow(clippy::exhaustive_structs)]`

The [`DataMarker`] marker type for [`AnyPayload`].

```rust
pub struct AnyMarker;
```

##### Implementations

###### Trait Implementations

- **Sync**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DataMarker**
- **UpcastDataPayload**
  - ```rust
    fn upcast(other: DataPayload<M>) -> DataPayload<AnyMarker> { /* ... */ }
    ```

- **ErasedDestructor**
- **MaybeSendSync**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DynamicDataProvider**
  - ```rust
    fn load_data(self: &Self, key: $crate::DataKey, req: $crate::DataRequest<''_>) -> Result<$crate::DataResponse<AnyMarker>, $crate::DataError> { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `AnyResponse`

**Attributes:**

- `#[allow(clippy::exhaustive_structs)]`

A [`DataResponse`] for type-erased values.

Convertible to and from `DataResponse<AnyMarker>`.

```rust
pub struct AnyResponse {
    pub metadata: DataResponseMetadata,
    pub payload: Option<AnyPayload>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `metadata` | `DataResponseMetadata` | Metadata about the returned object. |
| `payload` | `Option<AnyPayload>` | The object itself; `None` if it was not loaded. |

##### Implementations

###### Methods

- ```rust
  pub fn downcast<M>(self: Self) -> Result<DataResponse<M>, DataError>
where
    M: DataMarker,
    for<''a> YokeTraitHack<<<M as >::Yokeable as Yokeable<''a>>::Output>: Clone,
    <M as >::Yokeable: ZeroFrom<''static, <M as >::Yokeable> + MaybeSendSync { /* ... */ }
  ```
  Transforms a type-erased `AnyResponse` into a concrete `DataResponse<M>`.

- ```rust
  pub fn downcast_cloned<M>(self: &Self) -> Result<DataResponse<M>, DataError>
where
    M: DataMarker,
    <M as >::Yokeable: ZeroFrom<''static, <M as >::Yokeable> + MaybeSendSync,
    for<''a> YokeTraitHack<<<M as >::Yokeable as Yokeable<''a>>::Output>: Clone { /* ... */ }
  ```
  Clones and then transforms a type-erased `AnyResponse` into a concrete `DataResponse<M>`.

###### Trait Implementations

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(other: DataResponse<AnyMarker>) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(other: AnyResponse) -> Self { /* ... */ }
    ```

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
#### Struct `DynamicDataProviderAnyMarkerWrap`

**Attributes:**

- `#[allow(clippy::exhaustive_structs)]`

A wrapper over `DynamicDataProvider<AnyMarker>` that implements `AnyProvider`

```rust
pub struct DynamicDataProviderAnyMarkerWrap<''a, P: ?Sized>(pub &''a P);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a P` |  |

##### Implementations

###### Trait Implementations

- **AsDowncastingAnyProvider**
  - ```rust
    fn as_downcasting(self: &Self) -> DowncastingAnyProvider<''_, P> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Sync**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **RefUnwindSafe**
- **AnyProvider**
  - ```rust
    fn load_any(self: &Self, key: DataKey, req: DataRequest<''_>) -> Result<AnyResponse, DataError> { /* ... */ }
    ```

#### Struct `DowncastingAnyProvider`

**Attributes:**

- `#[allow(clippy::exhaustive_structs)]`

A wrapper over `AnyProvider` that implements `DynamicDataProvider<M>` via downcasting

```rust
pub struct DowncastingAnyProvider<''a, P: ?Sized>(pub &''a P);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a P` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: DataRequest<''_>) -> Result<DataResponse<M>, DataError> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Sync**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DynamicDataProvider**
  - ```rust
    fn load_data(self: &Self, key: DataKey, req: DataRequest<''_>) -> Result<DataResponse<M>, DataError> { /* ... */ }
    ```

- **ErasedDestructor**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Traits

#### Trait `MaybeSendSync`

**Attributes:**

- `#[allow(missing_docs)]`
- `#[<cfg>(not(feature = "sync"))]`

```rust
pub trait MaybeSendSync {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `T` with <T>

#### Trait `AnyProvider`

An object-safe data provider that returns data structs cast to `dyn Any` trait objects.

# Examples

```
use icu_provider::hello_world::*;
use icu_provider::prelude::*;
use std::borrow::Cow;

let any_provider = HelloWorldProvider.as_any_provider();

let req = DataRequest {
    locale: &icu_locid::langid!("de").into(),
    metadata: Default::default(),
};

// Downcasting manually
assert_eq!(
    any_provider
        .load_any(HelloWorldV1Marker::KEY, req)
        .expect("load should succeed")
        .downcast::<HelloWorldV1Marker>()
        .expect("types should match")
        .take_payload()
        .unwrap()
        .get(),
    &HelloWorldV1 {
        message: Cow::Borrowed("Hallo Welt"),
    },
);

// Downcasting automatically
let downcasting_provider: &dyn DataProvider<HelloWorldV1Marker> =
    &any_provider.as_downcasting();

assert_eq!(
    downcasting_provider
        .load(req)
        .expect("load should succeed")
        .take_payload()
        .unwrap()
        .get(),
    &HelloWorldV1 {
        message: Cow::Borrowed("Hallo Welt"),
    },
);
```

```rust
pub trait AnyProvider {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `load_any`: Loads an [`AnyPayload`] according to the key and request.

##### Implementations

This trait is implemented for the following types:

- `&''a T` with <''a, T: AnyProvider + ?Sized>
- `alloc::boxed::Box<T>` with <T: AnyProvider + ?Sized>
- `alloc::rc::Rc<T>` with <T: AnyProvider + ?Sized>
- `alloc::sync::Arc<T>` with <T: AnyProvider + ?Sized>
- `DynamicDataProviderAnyMarkerWrap<''_, P>` with <P>

#### Trait `AsDynamicDataProviderAnyMarkerWrap`

Blanket-implemented trait adding the [`Self::as_any_provider()`] function.

```rust
pub trait AsDynamicDataProviderAnyMarkerWrap {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `as_any_provider`: Returns an object implementing `AnyProvider` when called on `DynamicDataProvider<AnyMarker>`

##### Implementations

This trait is implemented for the following types:

- `P` with <P>

#### Trait `AsDowncastingAnyProvider`

Blanket-implemented trait adding the [`Self::as_downcasting()`] function.

```rust
pub trait AsDowncastingAnyProvider {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `as_downcasting`: Returns an object implementing `DynamicDataProvider<M>` when called on `AnyProvider`

##### Implementations

This trait is implemented for the following types:

- `P` with <P>

## Module `buf`

Traits for data providers that produce opaque buffers.

```rust
pub mod buf { /* ... */ }
```

### Types

#### Struct `BufferMarker`

**Attributes:**

- `#[allow(clippy::exhaustive_structs)]`

[`DataMarker`] for raw buffers. Returned by [`BufferProvider`].

The data is expected to be deserialized before it can be used; see
[`DataPayload::into_deserialized`].

```rust
pub struct BufferMarker;
```

##### Implementations

###### Trait Implementations

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
- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **DataMarker**
- **RefUnwindSafe**
- **Sync**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Enum `BufferFormat`

**Attributes:**

- `#[<cfg_attr>(feature = "serde", derive(serde::Serialize, serde::Deserialize))]`
- `#[non_exhaustive]`

An enum expressing all Serde formats known to ICU4X.

```rust
pub enum BufferFormat {
    Json,
    Bincode1,
    Postcard1,
}
```

##### Variants

###### `Json`

Serialize using JavaScript Object Notation (JSON).

###### `Bincode1`

Serialize using Bincode version 1.

###### `Postcard1`

Serialize using Postcard version 1.

##### Implementations

###### Methods

- ```rust
  pub fn check_available(self: &Self) -> Result<(), DataError> { /* ... */ }
  ```
  Returns an error if the buffer format is not enabled.

###### Trait Implementations

- **Unpin**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BufferFormat) -> bool { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> BufferFormat { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Copy**
- **MaybeSendSync**
- **Send**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `BufferProvider`

A data provider that returns opaque bytes.

Generally, these bytes are expected to be deserializable with Serde. To get an object
implementing [`DataProvider`] via Serde, use [`as_deserializing()`].

Passing a  `BufferProvider` to a `*_with_buffer_provider` constructor requires enabling
the deserialization Cargo feature for the expected format(s):
- `deserialize_json`
- `deserialize_postcard_1`
- `deserialize_bincode_1`

Along with [`DataProvider`], this is one of the two foundational traits in this crate.

[`BufferProvider`] can be made into a trait object. It is used over FFI.

# Examples

```
# #[cfg(feature = "deserialize_json")] {
use icu_locid::langid;
use icu_provider::hello_world::*;
use icu_provider::prelude::*;
use std::borrow::Cow;

let buffer_provider = HelloWorldProvider.into_json_provider();

let req = DataRequest {
    locale: &langid!("de").into(),
    metadata: Default::default(),
};

// Deserializing manually
assert_eq!(
    serde_json::from_slice::<HelloWorldV1>(
        buffer_provider
            .load_buffer(HelloWorldV1Marker::KEY, req)
            .expect("load should succeed")
            .take_payload()
            .unwrap()
            .get()
    )
    .expect("should deserialize"),
    HelloWorldV1 {
        message: Cow::Borrowed("Hallo Welt"),
    },
);

// Deserialize automatically
let deserializing_provider: &dyn DataProvider<HelloWorldV1Marker> =
    &buffer_provider.as_deserializing();

assert_eq!(
    deserializing_provider
        .load(req)
        .expect("load should succeed")
        .take_payload()
        .unwrap()
        .get(),
    &HelloWorldV1 {
        message: Cow::Borrowed("Hallo Welt"),
    },
);
# }
```

[`as_deserializing()`]: AsDeserializingBufferProvider::as_deserializing

```rust
pub trait BufferProvider {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `load_buffer`: Loads a [`DataPayload`]`<`[`BufferMarker`]`>` according to the key and request.

##### Implementations

This trait is implemented for the following types:

- `&''a T` with <''a, T: BufferProvider + ?Sized>
- `alloc::boxed::Box<T>` with <T: BufferProvider + ?Sized>
- `alloc::rc::Rc<T>` with <T: BufferProvider + ?Sized>
- `alloc::sync::Arc<T>` with <T: BufferProvider + ?Sized>

## Module `constructors`

📚 *This module documents ICU4X constructor signatures.*

One of the key differences between ICU4X and its parent projects, ICU4C and ICU4J, is in how
it deals with locale data.

In ICU4X, data can always be explicitly passed to any function that requires data.
This enables ICU4X to achieve the following value propositions:

1. Configurable data sources (machine-readable data file, baked into code, JSON, etc).
2. Dynamic data loading at runtime (load data on demand).
3. Reduced overhead and code size (data is resolved locally at each call site).
4. Explicit support for multiple ICU4X instances sharing data.

However, as manual data management can be tedious, ICU4X also has a `compiled_data`
default Cargo feature that includes data and makes ICU4X work out-of-the box.

Subsequently, there are 4 versions of all Rust ICU4X functions that use data:

1. `*`
2. `*_unstable`
3. `*_with_any_provider`
4. `*_with_buffer_provider`

# Which constructor should I use?

## When to use `*`

If you don't want to customize data at runtime (i.e. if you don't care about code size,
updating your data, etc.) you can use the `compiled_data` Cargo feature and don't have to think
about where your data comes from.

These constructors are sometimes `const` functions, this way Rust can most effectively optimize
your usage of ICU4X.

## When to use `*_unstable`

Use this constructor if your data provider implements the [`DataProvider`] trait for all
data structs in *current and future* ICU4X versions. Examples:

1. `BakedDataProvider` generated for the specific ICU4X minor version
2. Anything with a _blanket_ [`DataProvider`] impl

Since the exact set of bounds may change at any time, including in minor SemVer releases,
it is the client's responsibility to guarantee that the requirement is upheld.

## When to use `*_with_any_provider`

Use this constructor if you need to use a provider that implements [`AnyProvider`] but not
[`DataProvider`]. Examples:

1. [`AnyPayloadProvider`]
2. [`ForkByKeyProvider`] between two providers implementing [`AnyProvider`]
3. Providers that cache or override certain keys but not others and therefore
   can't implement [`DataProvider`]

## When to use `*_with_buffer_provider`

Use this constructor if your data originates as byte buffers that need to be deserialized.
All such providers should implement [`BufferProvider`]. Examples:

1. [`BlobDataProvider`]
2. [`FsDataProvider`]
3. [`ForkByKeyProvider`] between two providers implementing [`BufferProvider`]

Please note that you must enable the `serde` Cargo feature on each crate in which you use the
`*_with_buffer_provider` constructor.

# Data Versioning Policy

The `*_with_any_provider` and `*_with_buffer_provider` functions will succeed to compile and
run if given a data provider supporting all of the keys required for the object being
constructed, either the current or any previous version within the same SemVer major release.
For example, if a data file is built to support FooFormatter version 1.1, then FooFormatter
version 1.2 will be able to read the same data file. Likewise, backwards-compatible keys can
always be included by `icu_datagen` to support older library versions.

The `*_unstable` functions are only guaranteed to work on data built for the exact same minor version
of ICU4X. The advantage of the `*_unstable` functions is that they result in the smallest code
size and allow for automatic data slicing when `BakedDataProvider` is used. However, the type
bounds of this function may change over time, breaking SemVer guarantees. These functions
should therefore only be used when you have full control over your data lifecycle at compile
time.

# Data Providers Over FFI

Over FFI, there is only one data provider type: [`ICU4XDataProvider`]. Internally, it is an
`enum` between`dyn `[`BufferProvider`] and a unit compiled data variant.

To control for code size, there are two Cargo features, `compiled_data` and `buffer_provider`,
that enable the corresponding items in the enum.

In Rust ICU4X, a similar enum approach was not taken because:

1. Feature-gating the enum branches gets complex across crates.
2. Without feature gating, users need to carry Serde code even if they're not using it,
   violating one of the core value propositions of ICU4X.
3. We could reduce the number of constructors from 4 to 2 but not to 1, so the educational
   benefit is limited.

[`DataProvider`]: crate::DataProvider
[`BufferProvider`]: crate::BufferProvider
[`AnyProvider`]: crate::AnyProvider
[`AnyPayloadProvider`]: ../../icu_provider_adapters/any_payload/struct.AnyPayloadProvider.html
[`ForkByKeyProvider`]: ../../icu_provider_adapters/fork/struct.ForkByKeyProvider.html
[`BlobDataProvider`]: ../../icu_provider_blob/struct.BlobDataProvider.html
[`StaticDataProvider`]: ../../icu_provider_blob/struct.StaticDataProvider.html
[`FsDataProvider`]: ../../icu_provider_blob/struct.FsDataProvider.html
[`ICU4XDataProvider`]: ../../icu_capi/provider/ffi/struct.ICU4XDataProvider.html

```rust
pub mod constructors { /* ... */ }
```

## Module `dynutil`

Utilities for using trait objects with `DataPayload`.

```rust
pub mod dynutil { /* ... */ }
```

### Traits

#### Trait `UpcastDataPayload`

Trait to allow conversion from `DataPayload<T>` to `DataPayload<S>`.

This trait can be manually implemented in order to enable [`impl_dynamic_data_provider`](crate::impl_dynamic_data_provider).

[`DataPayload::downcast`]: crate::DataPayload::downcast

```rust
pub trait UpcastDataPayload<M>
where
    M: crate::DataMarker,
    Self: Sized + crate::DataMarker {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `upcast`: Upcast a `DataPayload<T>` to a `DataPayload<S>` where `T` implements trait `S`.

##### Implementations

This trait is implemented for the following types:

- `AnyMarker` with <M>

## Module `hello_world`

**Attributes:**

- `#[allow(clippy::exhaustive_structs)]`

Data provider returning multilingual "Hello World" strings for testing.

```rust
pub mod hello_world { /* ... */ }
```

### Types

#### Struct `HelloWorldV1`

**Attributes:**

- `#[<cfg_attr>(feature = "serde", derive(serde::Deserialize))]`
- `#[<cfg_attr>(any(feature = "deserialize_json", feature = "datagen"),
derive(serde::Serialize))]`
- `#[<cfg_attr>(feature = "datagen", derive(databake::Bake))]`
- `#[<cfg_attr>(feature = "datagen", databake(path = icu_provider::hello_world))]`

A struct containing "Hello World" in the requested language.

```rust
pub struct HelloWorldV1<''data> {
    pub message: alloc::borrow::Cow<''data, str>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `message` | `alloc::borrow::Cow<''data, str>` | The translation of "Hello World". |

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &HelloWorldV1<''data>) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> HelloWorldV1<''data> { /* ... */ }
    ```

- **Yokeable**
  - ```rust
    fn transform(self: &''a Self) -> &''a <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn transform_owned(self: Self) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    unsafe fn make(this: <Self as >::Output) -> Self { /* ... */ }
    ```

  - ```rust
    fn transform_mut<F>(self: &''a mut Self, f: F)
where
    F: ''static + for<''b> FnOnce(&''b mut <Self as >::Output) { /* ... */ }
    ```

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

- **Freeze**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
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

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **ZeroFrom**
  - ```rust
    fn zero_from(this: &''zf HelloWorldV1<''zf_inner>) -> Self { /* ... */ }
    ```

- **Send**
- **MaybeSendSync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
#### Struct `HelloWorldV1Marker`

**Attributes:**

- `#[<cfg_attr>(feature = "datagen", derive(Default, databake::Bake))]`
- `#[<cfg_attr>(feature = "datagen", databake(path = icu_provider::hello_world))]`

Marker type for [`HelloWorldV1`].

```rust
pub struct HelloWorldV1Marker;
```

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DataMarker**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Send**
- **Unpin**
- **KeyedDataMarker**
- **MaybeSendSync**
- **DataProvider**
  - ```rust
    fn load(self: &Self, req: DataRequest<''_>) -> Result<DataResponse<HelloWorldV1Marker>, DataError> { /* ... */ }
    ```

- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `HelloWorldProvider`

A data provider returning Hello World strings in different languages.

Mostly useful for testing.

# Examples

```
use icu_locid::langid;
use icu_provider::hello_world::*;
use icu_provider::prelude::*;

let german_hello_world: DataPayload<HelloWorldV1Marker> =
    HelloWorldProvider
        .load(DataRequest {
            locale: &langid!("de").into(),
            metadata: Default::default(),
        })
        .expect("Loading should succeed")
        .take_payload()
        .expect("Data should be present");

assert_eq!("Hallo Welt", german_hello_world.get().message);
```

Load the reverse string using an auxiliary key:

```
use icu_provider::hello_world::*;
use icu_provider::prelude::*;

let reverse_hello_world: DataPayload<HelloWorldV1Marker> =
    HelloWorldProvider
        .load(DataRequest {
            locale: &"en-x-reverse".parse().unwrap(),
            metadata: Default::default(),
        })
        .expect("Loading should succeed")
        .take_payload()
        .expect("Data should be present");

assert_eq!("Olleh Dlrow", reverse_hello_world.get().message);
```

```rust
pub struct HelloWorldProvider;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DataProvider**
  - ```rust
    fn load(self: &Self, req: DataRequest<''_>) -> Result<DataResponse<HelloWorldV1Marker>, DataError> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> HelloWorldProvider { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &HelloWorldProvider) -> bool { /* ... */ }
    ```

- **DynamicDataProvider**
  - ```rust
    fn load_data(self: &Self, key: $crate::DataKey, req: $crate::DataRequest<''_>) -> Result<$crate::DataResponse<AnyMarker>, $crate::DataError> { /* ... */ }
    ```

- **RefUnwindSafe**
- **AsDynamicDataProviderAnyMarkerWrap**
  - ```rust
    fn as_any_provider(self: &Self) -> DynamicDataProviderAnyMarkerWrap<''_, P> { /* ... */ }
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

- **Sync**
#### Struct `HelloWorldFormatter`

A type that formats localized "hello world" strings.

This type is intended to take the shape of a typical ICU4X formatter API.

# Examples

```
use icu_locid::locale;
use icu_provider::hello_world::{HelloWorldFormatter, HelloWorldProvider};
use writeable::assert_writeable_eq;

let fmt = HelloWorldFormatter::try_new_unstable(
    &HelloWorldProvider,
    &locale!("eo").into(),
)
.expect("locale exists");

assert_writeable_eq!(fmt.format(), "Saluton, Mondo");
```

```rust
pub struct HelloWorldFormatter {
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
  pub fn try_new(locale: &DataLocale) -> Result<Self, DataError> { /* ... */ }
  ```
  Creates a new [`HelloWorldFormatter`] for the specified locale.

- ```rust
  pub fn try_new_with_any_provider</* synthetic */ impl crate::AnyProvider + ?Sized: $crate::AnyProvider + ?Sized>(provider: &impl $crate::AnyProvider + ?Sized, locale: &$crate::DataLocale) -> Result<Self, DataError> { /* ... */ }
  ```
  A version of [`Self :: try_new`] that uses custom data provided by an [`AnyProvider`](icu_provider::AnyProvider).

- ```rust
  pub fn try_new_unstable<P>(provider: &P, locale: &DataLocale) -> Result<Self, DataError>
where
    P: DataProvider<HelloWorldV1Marker> { /* ... */ }
  ```
  A version of [`Self::try_new`] that uses custom data provided by a [`DataProvider`](icu_provider::DataProvider).

- ```rust
  pub fn format<''l>(self: &''l Self) -> FormattedHelloWorld<''l> { /* ... */ }
  ```
  Formats a hello world message, returning a [`FormattedHelloWorld`].

- ```rust
  pub fn format_to_string(self: &Self) -> String { /* ... */ }
  ```
  Formats a hello world message, returning a [`String`].

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
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

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
- **Sync**
#### Struct `FormattedHelloWorld`

A formatted hello world message. Implements [`Writeable`].

For an example, see [`HelloWorldFormatter`].

```rust
pub struct FormattedHelloWorld<''l> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Writeable**
  - ```rust
    fn write_to<W: core::fmt::Write + ?Sized>(self: &Self, sink: &mut W) -> core::fmt::Result { /* ... */ }
    ```

  - ```rust
    fn write_to_string(self: &Self) -> Cow<''_, str> { /* ... */ }
    ```

  - ```rust
    fn writeable_length_hint(self: &Self) -> writeable::LengthHint { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Send**
## Module `marker`

Marker types and traits for DataProvider.

```rust
pub mod marker { /* ... */ }
```

### Types

#### Struct `NeverMarker`

A [`DataMarker`] that never returns data.

All types that have non-blanket impls of `DataProvider<M>` are expected to explicitly
implement `DataProvider<NeverMarker<Y>>`, returning [`DataErrorKind::MissingDataKey`].
See [`impl_data_provider_never_marker!`].

[`DataErrorKind::MissingDataKey`]: crate::DataErrorKind::MissingDataKey
[`impl_data_provider_never_marker!`]: crate::impl_data_provider_never_marker

# Examples

```
use icu_locid::langid;
use icu_provider::hello_world::*;
use icu_provider::prelude::*;
use icu_provider::NeverMarker;

let buffer_provider = HelloWorldProvider.into_json_provider();

let result = DataProvider::<NeverMarker<HelloWorldV1<'static>>>::load(
    &buffer_provider.as_deserializing(),
    DataRequest {
        locale: &langid!("en").into(),
        metadata: Default::default(),
    },
);

assert!(matches!(
    result,
    Err(DataError {
        kind: DataErrorKind::MissingDataKey,
        ..
    })
));
```

```rust
pub struct NeverMarker<Y>(/* private field */);
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

- **ErasedDestructor**
- **Sync**
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

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **DataMarker**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NeverMarker<Y> { /* ... */ }
    ```

- **UnwindSafe**
- **KeyedDataMarker**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Traits

#### Trait `DataMarker`

Trait marker for data structs. All types delivered by the data provider must be associated with
something implementing this trait.

Structs implementing this trait are normally generated with the [`data_struct`] macro.

By convention, the non-standard `Marker` suffix is used by types implementing DataMarker.

In addition to a marker type implementing DataMarker, the following impls must also be present
for the data struct:

- `impl<'a> Yokeable<'a>` (required)
- `impl ZeroFrom<Self>`

Also see [`KeyedDataMarker`].

Note: `DataMarker`s are quasi-const-generic compile-time objects, and as such are expected
to be unit structs. As this is not something that can be enforced by the type system, we
currently only have a `'static` bound on them (which is needed by a lot of our code).

# Examples

Manually implementing DataMarker for a custom type:

```
use icu_provider::prelude::*;
use std::borrow::Cow;

#[derive(yoke::Yokeable, zerofrom::ZeroFrom)]
struct MyDataStruct<'data> {
    message: Cow<'data, str>,
}

struct MyDataStructMarker;

impl DataMarker for MyDataStructMarker {
    type Yokeable = MyDataStruct<'static>;
}

// We can now use MyDataStruct with DataProvider:
let s = MyDataStruct {
    message: Cow::Owned("Hello World".into()),
};
let payload = DataPayload::<MyDataStructMarker>::from_owned(s);
assert_eq!(payload.get().message, "Hello World");
```

[`data_struct`]: crate::data_struct

```rust
pub trait DataMarker: ''static {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Yokeable`: A type that implements [`Yokeable`]. This should typically be the `'static` version of a

##### Implementations

This trait is implemented for the following types:

- `AnyMarker`
- `BufferMarker`
- `HelloWorldV1Marker`
- `NeverMarker<Y>` with <Y>

#### Trait `KeyedDataMarker`

A [`DataMarker`] with a [`DataKey`] attached.

Structs implementing this trait are normally generated with the [`data_struct!`] macro.

Implementing this trait enables this marker to be used with the main [`DataProvider`] trait.
Most markers should be associated with a specific key and should therefore implement this
trait.

[`BufferMarker`] and [`AnyMarker`] are examples of markers that do _not_ implement this trait
because they are not specific to a single key.

Note: `KeyedDataMarker`s are quasi-const-generic compile-time objects, and as such are expected
to be unit structs. As this is not something that can be enforced by the type system, we
currently only have a `'static` bound on them (which is needed by a lot of our code).

[`data_struct!`]: crate::data_struct
[`DataProvider`]: crate::DataProvider
[`BufferMarker`]: crate::BufferMarker
[`AnyMarker`]: crate::AnyMarker

```rust
pub trait KeyedDataMarker: DataMarker {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Constants

- `KEY`: The single [`DataKey`] associated with this marker.

##### Provided Methods

- ```rust
  fn bind<P>(provider: P) -> DataProviderWithKey<Self, P>
where
    P: DataProvider<Self>,
    Self: Sized { /* ... */ }
  ```
  Binds this [`KeyedDataMarker`] to a provider supporting it.

##### Implementations

This trait is implemented for the following types:

- `HelloWorldV1Marker`
- `NeverMarker<Y>` with <Y>

## Module `prelude`

Core selection of APIs and structures for the ICU4X data provider.

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `data_key`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::data_key;
```

#### Re-export `AnyMarker`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::AnyMarker;
```

#### Re-export `AnyPayload`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::AnyPayload;
```

#### Re-export `AnyProvider`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::AnyProvider;
```

#### Re-export `AnyResponse`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::AnyResponse;
```

#### Re-export `AsDowncastingAnyProvider`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::AsDowncastingAnyProvider;
```

#### Re-export `AsDynamicDataProviderAnyMarkerWrap`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::AsDynamicDataProviderAnyMarkerWrap;
```

#### Re-export `BoundDataProvider`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::BoundDataProvider;
```

#### Re-export `BufferMarker`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::BufferMarker;
```

#### Re-export `BufferProvider`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::BufferProvider;
```

#### Re-export `DataError`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::DataError;
```

#### Re-export `DataErrorKind`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::DataErrorKind;
```

#### Re-export `DataKey`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::DataKey;
```

#### Re-export `DataKeyHash`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::DataKeyHash;
```

#### Re-export `DataLocale`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::DataLocale;
```

#### Re-export `DataMarker`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::DataMarker;
```

#### Re-export `DataPayload`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::DataPayload;
```

#### Re-export `DataProvider`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::DataProvider;
```

#### Re-export `DataRequest`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::DataRequest;
```

#### Re-export `DataRequestMetadata`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::DataRequestMetadata;
```

#### Re-export `DataResponse`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::DataResponse;
```

#### Re-export `DataResponseMetadata`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::DataResponseMetadata;
```

#### Re-export `DynamicDataProvider`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::DynamicDataProvider;
```

#### Re-export `KeyedDataMarker`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::KeyedDataMarker;
```

## Macros

### Macro `data_key`

**Attributes:**

- `#[macro_export]`

See [`DataKey`].

```rust
pub macro_rules! data_key {
    /* macro_rules! data_key {
    ($path:expr) => { ... };
    ($path:expr, $metadata:expr) => { ... };
} */
}
```

### Macro `impl_casting_upcast`

**Attributes:**

- `#[macro_export]`

Implements [`UpcastDataPayload`] from several data markers to a single data marker
that all share the same [`DataMarker::Yokeable`].

# Examples

```
use icu_provider::prelude::*;
use std::borrow::Cow;

#[icu_provider::data_struct(
    FooV1Marker,
    BarV1Marker = "demo/bar@1",
    BazV1Marker = "demo/baz@1"
)]
pub struct FooV1<'data> {
    message: Cow<'data, str>,
};

icu_provider::impl_casting_upcast!(
    FooV1Marker,
    [BarV1Marker, BazV1Marker,]
);
```

[`DataMarker::Yokeable`]: crate::DataMarker::Yokeable

```rust
pub macro_rules! impl_casting_upcast {
    /* macro_rules! impl_casting_upcast {
    ($dyn_m:path, [ $($struct_m:ident),+, ]) => { ... };
} */
}
```

### Macro `impl_dynamic_data_provider`

**Attributes:**

- `#[macro_export]`

Implements [`DynamicDataProvider`] for a marker type `S` on a type that already implements
[`DynamicDataProvider`] or [`DataProvider`] for one or more `M`, where `M` is a concrete type
that is convertible to `S` via [`UpcastDataPayload`].

Use this macro to add support to your data provider for:

- [`AnyPayload`] if your provider can return typed objects as [`Any`](core::any::Any).

## Wrapping DataProvider

If your type implements [`DataProvider`], pass a list of markers as the second argument.
This results in a `DynamicDataProvider` that delegates to a specific marker if the key
matches or else returns [`DataErrorKind::MissingDataKey`].

```
use icu_provider::prelude::*;
use icu_provider::hello_world::*;
#
# // Duplicating HelloWorldProvider because the real one already implements DynamicDataProvider<AnyMarker>
# struct HelloWorldProvider;
# impl DataProvider<HelloWorldV1Marker> for HelloWorldProvider {
#     fn load(
#         &self,
#         req: DataRequest,
#     ) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
#         icu_provider::hello_world::HelloWorldProvider.load(req)
#     }
# }

// Implement DynamicDataProvider<AnyMarker> on HelloWorldProvider: DataProvider<HelloWorldV1Marker>
icu_provider::impl_dynamic_data_provider!(HelloWorldProvider, [HelloWorldV1Marker,], AnyMarker);

let req = DataRequest {
    locale: &icu_locid::langid!("de").into(),
    metadata: Default::default(),
};

// Successful because the key matches:
HelloWorldProvider.load_data(HelloWorldV1Marker::KEY, req).unwrap();

// MissingDataKey error as the key does not match:
assert_eq!(
    HelloWorldProvider.load_data(icu_provider::data_key!("dummy@1"), req).unwrap_err().kind,
    DataErrorKind::MissingDataKey,
);
```

## Wrapping DynamicDataProvider

It is also possible to wrap a [`DynamicDataProvider`] to create another [`DynamicDataProvider`]. To do this,
pass a match-like statement for keys as the second argument:

```
use icu_provider::prelude::*;
use icu_provider::hello_world::*;
#
# struct HelloWorldProvider;
# impl DynamicDataProvider<HelloWorldV1Marker> for HelloWorldProvider {
#     fn load_data(&self, key: DataKey, req: DataRequest)
#             -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
#         icu_provider::hello_world::HelloWorldProvider.load(req)
#     }
# }

// Implement DataProvider<AnyMarker> on HelloWorldProvider: DynamicDataProvider<HelloWorldV1Marker>
icu_provider::impl_dynamic_data_provider!(HelloWorldProvider, {
    // Match HelloWorldV1Marker::KEY and delegate to DynamicDataProvider<HelloWorldV1Marker>.
    HW = HelloWorldV1Marker::KEY => HelloWorldV1Marker,
    // Send the wildcard match also to DynamicDataProvider<HelloWorldV1Marker>.
    _ => HelloWorldV1Marker,
}, AnyMarker);

let req = DataRequest {
    locale: &icu_locid::langid!("de").into(),
    metadata: Default::default(),
};

// Successful because the key matches:
HelloWorldProvider.as_any_provider().load_any(HelloWorldV1Marker::KEY, req).unwrap();

// Because of the wildcard, any key actually works:
HelloWorldProvider.as_any_provider().load_any(icu_provider::data_key!("dummy@1"), req).unwrap();
```

[`DynamicDataProvider`]: crate::DynamicDataProvider
[`DataProvider`]: crate::DataProvider
[`AnyPayload`]: (crate::any::AnyPayload)
[`DataErrorKind::MissingDataKey`]: (crate::DataErrorKind::MissingDataKey)
[`SerializeMarker`]: (crate::serde::SerializeMarker)

```rust
pub macro_rules! impl_dynamic_data_provider {
    /* macro_rules! impl_dynamic_data_provider {
    ($provider:ty, $arms:tt, $one:path, $($rest:path),+) => { ... };
    ($provider:ty, { $($ident:ident = $key:path => $struct_m:ty),+, $(_ => $struct_d:ty,)?}, $dyn_m:ty) => { ... };
    ($provider:ty, [ $($(#[$cfg:meta])? $struct_m:ty),+, ], $dyn_m:path) => { ... };
} */
}
```

### Macro `impl_data_provider_never_marker`

**Attributes:**

- `#[macro_export]`

Implements `DataProvider<NeverMarker<Y>>` on a struct.

For more information, see [`NeverMarker`].

# Examples

```
use icu_locid::langid;
use icu_provider::hello_world::*;
use icu_provider::prelude::*;
use icu_provider::NeverMarker;

struct MyProvider;

icu_provider::impl_data_provider_never_marker!(MyProvider);

let result = DataProvider::<NeverMarker<HelloWorldV1<'static>>>::load(
    &MyProvider,
    DataRequest {
        locale: &langid!("und").into(),
        metadata: Default::default(),
    },
);

assert!(matches!(
    result,
    Err(DataError {
        kind: DataErrorKind::MissingDataKey,
        ..
    })
));
```

```rust
pub macro_rules! impl_data_provider_never_marker {
    /* macro_rules! impl_data_provider_never_marker {
    ($ty:path) => { ... };
} */
}
```

### Macro `_internal_noop_log`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! _internal_noop_log {
    /* macro_rules! _internal_noop_log {
    ($($t:expr),*) => { ... };
} */
}
```

## Re-exports

### Re-export `BoundDataProvider`

```rust
pub use crate::data_provider::BoundDataProvider;
```

### Re-export `DataProvider`

```rust
pub use crate::data_provider::DataProvider;
```

### Re-export `DataProviderWithKey`

```rust
pub use crate::data_provider::DataProviderWithKey;
```

### Re-export `DynamicDataProvider`

```rust
pub use crate::data_provider::DynamicDataProvider;
```

### Re-export `DataError`

```rust
pub use crate::error::DataError;
```

### Re-export `DataErrorKind`

```rust
pub use crate::error::DataErrorKind;
```

### Re-export `DataKey`

```rust
pub use crate::key::DataKey;
```

### Re-export `DataKeyHash`

```rust
pub use crate::key::DataKeyHash;
```

### Re-export `DataKeyMetadata`

```rust
pub use crate::key::DataKeyMetadata;
```

### Re-export `DataKeyPath`

```rust
pub use crate::key::DataKeyPath;
```

### Re-export `DataLocale`

```rust
pub use crate::request::DataLocale;
```

### Re-export `DataRequest`

```rust
pub use crate::request::DataRequest;
```

### Re-export `DataRequestMetadata`

```rust
pub use crate::request::DataRequestMetadata;
```

### Re-export `Cart`

```rust
pub use crate::response::Cart;
```

### Re-export `DataPayload`

```rust
pub use crate::response::DataPayload;
```

### Re-export `DataResponse`

```rust
pub use crate::response::DataResponse;
```

### Re-export `DataResponseMetadata`

```rust
pub use crate::response::DataResponseMetadata;
```

### Re-export `data_struct`

**Attributes:**

- `#[<cfg>(feature = "macros")]`

```rust
pub use icu_provider_macros::data_struct;
```

### Re-export `AnyMarker`

```rust
pub use crate::any::AnyMarker;
```

### Re-export `AnyPayload`

```rust
pub use crate::any::AnyPayload;
```

### Re-export `AnyProvider`

```rust
pub use crate::any::AnyProvider;
```

### Re-export `AnyResponse`

```rust
pub use crate::any::AnyResponse;
```

### Re-export `AsDowncastingAnyProvider`

```rust
pub use crate::any::AsDowncastingAnyProvider;
```

### Re-export `AsDynamicDataProviderAnyMarkerWrap`

```rust
pub use crate::any::AsDynamicDataProviderAnyMarkerWrap;
```

### Re-export `MaybeSendSync`

```rust
pub use crate::any::MaybeSendSync;
```

### Re-export `BufferMarker`

```rust
pub use crate::buf::BufferMarker;
```

### Re-export `BufferProvider`

```rust
pub use crate::buf::BufferProvider;
```

### Re-export `DataMarker`

```rust
pub use crate::marker::DataMarker;
```

### Re-export `KeyedDataMarker`

```rust
pub use crate::marker::KeyedDataMarker;
```

### Re-export `NeverMarker`

```rust
pub use crate::marker::NeverMarker;
```

