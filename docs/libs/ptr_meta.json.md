# Crate Documentation

**Version:** 0.3.0

**Format Version:** 43

# Module `ptr_meta`

A radioactive stabilization of the [`ptr_meta` RFC][rfc].

This crate provides the [`Pointee`] trait, [`from_raw_parts`] and
[`to_raw_parts`] functions, and proc macros for deriving `Pointee` for
structs and implementing `Pointee` for trait objects.

[rfc]: https://rust-lang.github.io/rfcs/2580-ptr-meta.html

# Usage

Raw pointers can be decomposed into the data address and metadata components
with [`to_raw_parts`] or [`to_raw_parts_mut`].

Alternatively, metadata alone can be extracted with the [`metadata`]
function. Although [`metadata`] accepts pointers, references can be passed
and will be implicitly coerced.

A pointer can be created from its address and metadata with
[`from_raw_parts`] or [`from_raw_parts_mut`].

## Provided impls

`ptr_meta` provides inherent implementations for many builtin types:

- All [`Sized`] types implement [`Pointee`] via a blanket implementation.
- `slice`, `str`, and `CStr`
- `OsStr` (requires `std`)
- `dyn Any`, optionally with `+ Send` and/or `+ Sync`
- `dyn Error`, optionally with `+ Send` and/or `+ Sync`

## Structs with trailing DSTs

You can derive [`Pointee`] for structs with trailing DSTs:

```
use ptr_meta::Pointee;

#[derive(Pointee)]
struct Block<H, T> {
    header: H,
    elements: [T],
}
```

Note that the last field is required to be a DST. Structs with a generic
type as the last field may have conflicting blanket implementations, as the
generic type may be `Sized`. A collection of specific implementations may be
required in these cases, with the generic parameter set (for example) a
slice, `str`, or specific trait object.

## Trait objects

You can generate [`Pointee`] implementations for trait objects:

```
use ptr_meta::pointee;

// Generates Pointee for dyn Stringy
#[ptr_meta::pointee]
trait Stringy {
    fn as_string(&self) -> String;
}
```

Note that this will not produce implementations for `Trait + Send + Sync`.

## Features

- `derive`: Re-exports the macros from `ptr_meta_derive`. Enabled by
  default.
- `std`: Enables additional impls for `std` types. Enabled by default.

## Example
```rust
// Get the associated metadata for pointers
let str = "hello world";
assert_eq!(ptr_meta::metadata(str), str.len());

let slice = &[1, 2, 3, 4, 5] as &[i32];
assert_eq!(ptr_meta::metadata(slice), slice.len());

// Make your own wide pointers from data pointers and metadata
let bytes = [b'h', b'e', b'l', b'l', b'o'];
let ptr = ptr_meta::from_raw_parts::<str>(bytes.as_ptr().cast(), 5);
println!("{} world!", unsafe { &*ptr }); // prints "hello world!"

// Derive Pointee on your own types
#[derive(ptr_meta::Pointee)]
#[repr(transparent)]
struct CoolStr {
    inner: str,
}

impl CoolStr {
    fn print_cool(&self) {
        println!("😎 {} 😎", &self.inner);
    }
}

let ptr = ptr_meta::from_raw_parts::<CoolStr>(bytes.as_ptr().cast(), 5);
let cool = unsafe { &*ptr };
cool.print_cool(); // prints "😎 hello 😎"

// Implement Pointee for trait objects
#[ptr_meta::pointee]
trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("i32: {self}");
    }
}

let i32_vtable = ptr_meta::metadata(&0i32 as &dyn Printable);
let one_hundred = 100i32;
let printable = ptr_meta::from_raw_parts::<dyn Printable>(
    (&one_hundred as *const i32).cast(),
    i32_vtable,
);
unsafe {
    (*printable).print(); // prints "i32: 100"
}
```

## Types

### Struct `DynMetadata`

The metadata for a trait object.

This struct wraps a pointer to a vtable (virtual method table) which
contains all of the necessary information to manipulate the concrete type
stored inside of the trait object:

* The size and alignment of the concrete type
* A function pointer to the type's `drop_in_place` impl
* Function pointers for each method in the concrete type's trait
  implementation

Providing a type argument that is not a `dyn` trait object is possible, but
does not correspond with a meaningful type.

```rust
pub struct DynMetadata<Dyn: ?Sized> {
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
  pub fn size_of(self: Self) -> usize { /* ... */ }
  ```
  Returns the size of the type associated with this metadata.

- ```rust
  pub fn align_of(self: Self) -> usize { /* ... */ }
  ```
  Returns the alignment of the type associated with this metadata.

- ```rust
  pub fn layout(self: Self) -> core::alloc::Layout { /* ... */ }
  ```
  Returns the layout of the type associated with this metadata.

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Freeze**
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

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Pointee**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Unpin**
- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, hasher: &mut H) { /* ... */ }
    ```

## Traits

### Trait `Pointee`

A trait which associates pointer metadata with a pointee type.

# Pointer metadata

Pointers and references can be thought of as having two parts: a data
address and some extra "pointer metadata".

Pointers to [statically-sized types](`Sized`) and `extern` types are
"narrow": their pointer metadata is `()`.

Pointers to [dynamically-sized types][dst] are "wide": they have pointer
metadata with a non-zero size. There are four classes of dynamically-sized
types currently available:

* `str`s have `usize` pointer metadata equal to the length of the string
  slice in bytes.
* Slices like `[i32]` have `usize` pointer metadata equal to the length of
  the slice in items.
* Trait objects like `dyn SomeTrait` have [`DynMetadata`] pointer metadata,
  which point to the trait objects' virtual method tables.
* Structs with a trailing DST have the same metadata as the trailing DST.

In the future, Rust may add new kinds of types which have different pointer
metadata.

[dst]: https://doc.rust-lang.org/reference/dynamically-sized-types.html

# Safety

The associated `Metadata` type must be the pointer metadata type for the
implementing type.

```rust
pub unsafe trait Pointee {
    /* Associated items */
}
```

> This trait is unsafe to implement.

#### Required Items

##### Associated Types

- `Metadata`: The metadata type for pointers and references to this type.

#### Implementations

This trait is implemented for the following types:

- `dyn Any`
- `dyn Any + Send`
- `dyn Any + Sync`
- `dyn Any + Send + Sync`
- `dyn Error`
- `dyn Error + Send`
- `dyn Error + Sync`
- `dyn Error + Send + Sync`
- `T` with <T>
- `[T]` with <T>
- `str`
- `core::ffi::CStr`
- `std::ffi::OsStr`

## Functions

### Function `metadata`

**Attributes:**

- `#[inline]`

Returns the metadata of the given pointer.

`*mut T`, `&T`, and `&mut T` can all be passed directly to this function as
they implicitly coerce to `*const T`.

# Example

```
// String slices have pointer metadata equal to their size in bytes
assert_eq!(ptr_meta::metadata("foo"), 3_usize);
```

```rust
pub const fn metadata<T: Pointee + ?Sized>(ptr: *const T) -> <T as Pointee>::Metadata { /* ... */ }
```

### Function `to_raw_parts`

**Attributes:**

- `#[inline]`

Returns the data address and metadata of the given pointer.

`*mut T`, `&T`, and `&mut T` can all be passed directly to this function as
they implicitly coerce to `*const T`.

# Example

```
let (data_address, metadata) = ptr_meta::to_raw_parts("foo");
assert_ne!(data_address, core::ptr::null());
assert_eq!(metadata, 3);
```

```rust
pub const fn to_raw_parts<T: Pointee + ?Sized>(ptr: *const T) -> (*const (), <T as Pointee>::Metadata) { /* ... */ }
```

### Function `to_raw_parts_mut`

**Attributes:**

- `#[inline]`

Returns the mutable data address and metadata of the given pointer.

See [`to_raw_parts`] for more details.

```rust
pub const fn to_raw_parts_mut<T: Pointee + ?Sized>(ptr: *mut T) -> (*mut (), <T as Pointee>::Metadata) { /* ... */ }
```

### Function `from_raw_parts`

**Attributes:**

- `#[inline]`

Returns a raw pointer with the given data address and metadata.

This function is safe, but the returned pointer is not necessarily safe to
dereference. For slices, see the documentation of [`slice::from_raw_parts`]
for safety requirements. For trait objects, the metadata must come from a
a trait object with the same underlying type.

[`slice::from_raw_parts`]: core::slice::from_raw_parts

```rust
pub const fn from_raw_parts<T: Pointee + ?Sized>(data_address: *const (), metadata: <T as Pointee>::Metadata) -> *const T { /* ... */ }
```

### Function `from_raw_parts_mut`

**Attributes:**

- `#[inline]`

Returns a mutable raw pointer with the given data address and metadata.

See [`from_raw_parts`] for more details.

```rust
pub const fn from_raw_parts_mut<T: Pointee + ?Sized>(data_address: *mut (), metadata: <T as Pointee>::Metadata) -> *mut T { /* ... */ }
```

## Re-exports

### Re-export `pointee`

**Attributes:**

- `#[<cfg>(feature = "derive")]`

```rust
pub use ptr_meta_derive::pointee;
```

### Re-export `Pointee`

**Attributes:**

- `#[<cfg>(feature = "derive")]`

```rust
pub use ptr_meta_derive::Pointee;
```

