# Crate Documentation

**Version:** 0.10.4

**Format Version:** 43

# Module `zerovec`

Zero-copy vector abstractions for arbitrary types, backed by byte slices.

`zerovec` enables a far wider range of types — beyond just `&[u8]` and `&str` — to participate in
zero-copy deserialization from byte slices. It is `serde` compatible and comes equipped with
proc macros

Clients upgrading to `zerovec` benefit from zero heap allocations when deserializing
read-only data.

This crate has four main types:

- [`ZeroVec<'a, T>`] (and [`ZeroSlice<T>`](ZeroSlice)) for fixed-width types like `u32`
- [`VarZeroVec<'a, T>`] (and [`VarZeroSlice<T>`](ZeroSlice)) for variable-width types like `str`
- [`ZeroMap<'a, K, V>`] to map from `K` to `V`
- [`ZeroMap2d<'a, K0, K1, V>`] to map from the pair `(K0, K1)` to `V`

The first two are intended as close-to-drop-in replacements for `Vec<T>` in Serde structs. The third and fourth are
intended as a replacement for `HashMap` or [`LiteMap`](docs.rs/litemap). When used with Serde derives, **be sure to apply
`#[serde(borrow)]` to these types**, same as one would for [`Cow<'a, T>`].

[`ZeroVec<'a, T>`], [`VarZeroVec<'a, T>`], [`ZeroMap<'a, K, V>`], and [`ZeroMap2d<'a, K0, K1, V>`] all behave like
[`Cow<'a, T>`] in that they abstract over either borrowed or owned data. When performing deserialization
from human-readable formats (like `json` and `xml`), typically these types will allocate and fully own their data, whereas if deserializing
from binary formats like `bincode` and `postcard`, these types will borrow data directly from the buffer being deserialized from,
avoiding allocations and only performing validity checks. As such, this crate can be pretty fast (see [below](#Performance) for more information)
on deserialization.

See [the design doc](https://github.com/unicode-org/icu4x/blob/main/utils/zerovec/design_doc.md) for details on how this crate
works under the hood.

# Cargo features

This crate has several optional Cargo features:
 -  `serde`: Allows serializing and deserializing `zerovec`'s abstractions via [`serde`](https://docs.rs/serde)
 -   `yoke`: Enables implementations of `Yokeable` from the [`yoke`](https://docs.rs/yoke/) crate, which is also useful
             in situations involving a lot of zero-copy deserialization.
 - `derive`: Makes it easier to use custom types in these collections by providing the [`#[make_ule]`](crate::make_ule) and
    [`#[make_varule]`](crate::make_varule) proc macros, which generate appropriate [`ULE`](crate::ule::ULE) and
    [`VarULE`](crate::ule::VarULE)-conformant types for a given "normal" type.
 - `std`: Enabled `std::Error` implementations for error types. This crate is by default `no_std` with a dependency on `alloc`.

[`ZeroVec<'a, T>`]: ZeroVec
[`VarZeroVec<'a, T>`]: VarZeroVec
[`ZeroMap<'a, K, V>`]: ZeroMap
[`ZeroMap2d<'a, K0, K1, V>`]: ZeroMap2d
[`Cow<'a, T>`]: alloc::borrow::Cow

# Examples

Serialize and deserialize a struct with ZeroVec and VarZeroVec with Bincode:

```
# #[cfg(feature = "serde")] {
use zerovec::{VarZeroVec, ZeroVec};

// This example requires the "serde" feature
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DataStruct<'data> {
    #[serde(borrow)]
    nums: ZeroVec<'data, u32>,
    #[serde(borrow)]
    chars: ZeroVec<'data, char>,
    #[serde(borrow)]
    strs: VarZeroVec<'data, str>,
}

let data = DataStruct {
    nums: ZeroVec::from_slice_or_alloc(&[211, 281, 421, 461]),
    chars: ZeroVec::alloc_from_slice(&['ö', '冇', 'म']),
    strs: VarZeroVec::from(&["hello", "world"]),
};
let bincode_bytes =
    bincode::serialize(&data).expect("Serialization should be successful");
assert_eq!(bincode_bytes.len(), 67);

let deserialized: DataStruct = bincode::deserialize(&bincode_bytes)
    .expect("Deserialization should be successful");
assert_eq!(deserialized.nums.first(), Some(211));
assert_eq!(deserialized.chars.get(1), Some('冇'));
assert_eq!(deserialized.strs.get(1), Some("world"));
// The deserialization will not have allocated anything
assert!(!deserialized.nums.is_owned());
# } // feature = "serde"
```

Use custom types inside of ZeroVec:

```rust
# #[cfg(all(feature = "serde", feature = "derive"))] {
use zerovec::{ZeroVec, VarZeroVec, ZeroMap};
use std::borrow::Cow;
use zerovec::ule::encode_varule_to_box;

// custom fixed-size ULE type for ZeroVec
#[zerovec::make_ule(DateULE)]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
struct Date {
    y: u64,
    m: u8,
    d: u8
}

// custom variable sized VarULE type for VarZeroVec
#[zerovec::make_varule(PersonULE)]
#[zerovec::derive(Serialize, Deserialize)] // add Serde impls to PersonULE
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
struct Person<'a> {
    birthday: Date,
    favorite_character: char,
    #[serde(borrow)]
    name: Cow<'a, str>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Data<'a> {
    #[serde(borrow)]
    important_dates: ZeroVec<'a, Date>,
    // note: VarZeroVec always must reference the ULE type directly
    #[serde(borrow)]
    important_people: VarZeroVec<'a, PersonULE>,
    #[serde(borrow)]
    birthdays_to_people: ZeroMap<'a, Date, PersonULE>
}


let person1 = Person {
    birthday: Date { y: 1990, m: 9, d: 7},
    favorite_character: 'π',
    name: Cow::from("Kate")
};
let person2 = Person {
    birthday: Date { y: 1960, m: 5, d: 25},
    favorite_character: '冇',
    name: Cow::from("Jesse")
};

let important_dates = ZeroVec::alloc_from_slice(&[Date { y: 1943, m: 3, d: 20}, Date { y: 1976, m: 8, d: 2}, Date { y: 1998, m: 2, d: 15}]);
let important_people = VarZeroVec::from(&[&person1, &person2]);
let mut birthdays_to_people: ZeroMap<Date, PersonULE> = ZeroMap::new();
// `.insert_var_v()` is slightly more convenient over `.insert()` for custom ULE types
birthdays_to_people.insert_var_v(&person1.birthday, &person1);
birthdays_to_people.insert_var_v(&person2.birthday, &person2);

let data = Data { important_dates, important_people, birthdays_to_people };

let bincode_bytes = bincode::serialize(&data)
    .expect("Serialization should be successful");
assert_eq!(bincode_bytes.len(), 168);

let deserialized: Data = bincode::deserialize(&bincode_bytes)
    .expect("Deserialization should be successful");

assert_eq!(deserialized.important_dates.get(0).unwrap().y, 1943);
assert_eq!(&deserialized.important_people.get(1).unwrap().name, "Jesse");
assert_eq!(&deserialized.important_people.get(0).unwrap().name, "Kate");
assert_eq!(&deserialized.birthdays_to_people.get(&person1.birthday).unwrap().name, "Kate");

} // feature = serde and derive
```

# Performance

`zerovec` is designed for fast deserialization from byte buffers with zero memory allocations
while minimizing performance regressions for common vector operations.

Benchmark results on x86_64:

| Operation | `Vec<T>` | `zerovec` |
|---|---|---|
| Deserialize vec of 100 `u32` | 233.18 ns | 14.120 ns |
| Compute sum of vec of 100 `u32` (read every element) | 8.7472 ns | 10.775 ns |
| Binary search vec of 1000 `u32` 50 times | 442.80 ns | 472.51 ns |
| Deserialize vec of 100 strings | 7.3740 μs\* | 1.4495 μs |
| Count chars in vec of 100 strings (read every element) | 747.50 ns | 955.28 ns |
| Binary search vec of 500 strings 10 times | 466.09 ns | 790.33 ns |

\* *This result is reported for `Vec<String>`. However, Serde also supports deserializing to the partially-zero-copy `Vec<&str>`; this gives 1.8420 μs, much faster than `Vec<String>` but a bit slower than `zerovec`.*

| Operation | `HashMap<K,V>`  | `LiteMap<K,V>` | `ZeroMap<K,V>` |
|---|---|---|---|
| Deserialize a small map | 2.72 μs | 1.28 μs | 480 ns |
| Deserialize a large map | 50.5 ms | 18.3 ms | 3.74 ms |
| Look up from a small deserialized map | 49 ns | 42 ns | 54 ns |
| Look up from a large deserialized map | 51 ns | 155 ns | 213 ns |

Small = 16 elements, large = 131,072 elements. Maps contain `<String, String>`.

The benches used to generate the above table can be found in the `benches` directory in the project repository.
`zeromap` benches are named by convention, e.g. `zeromap/deserialize/small`, `zeromap/lookup/large`. The type
is appended for baseline comparisons, e.g. `zeromap/lookup/small/hashmap`.

## Modules

## Module `ule`

**Attributes:**

- `#[allow(clippy::upper_case_acronyms)]`

Traits over unaligned little-endian data (ULE, pronounced "yule").

The main traits for this module are [`ULE`], [`AsULE`] and, [`VarULE`].

See [the design doc](https://github.com/unicode-org/icu4x/blob/main/utils/zerovec/design_doc.md) for details on how these traits
works under the hood.

```rust
pub mod ule { /* ... */ }
```

### Modules

## Module `custom`

**Attributes:**

- `#[<cfg>(doc)]`

Documentation on implementing custom VarULE types.

This module contains documentation for defining custom VarULE types,
especially those using complex custom dynamically sized types.

In *most cases* you should be able to create custom VarULE types using
[`#[make_varule]`](crate::make_ule).

# Example

For example, if your regular stack type is:

```rust
use zerofrom::ZeroFrom;
use zerovec::ule::*;
use zerovec::ZeroVec;

#[derive(serde::Serialize, serde::Deserialize)]
struct Foo<'a> {
    field1: char,
    field2: u32,
    #[serde(borrow)]
    field3: ZeroVec<'a, u32>,
}
```

then the ULE type will be implemented as follows. Ideally, you should have
`EncodeAsVarULE` and `ZeroFrom` implementations on `Foo` pertaining to `FooULE`,
as well as a `Serialize` impl on `FooULE` and a `Deserialize` impl on `Box<FooULE>`
to enable human-readable serialization and deserialization.

```rust
use zerovec::{ZeroVec, VarZeroVec, ZeroSlice};
use zerovec::ule::*;
use zerofrom::ZeroFrom;
use core::mem;

# #[derive(serde::Serialize, serde::Deserialize)]
# struct Foo<'a> {
#    field1: char,
#    field2: u32,
#    #[serde(borrow)]
#    field3: ZeroVec<'a, u32>   
# }

// Must be repr(C, packed) for safety of VarULE!
// Must also only contain ULE types
#[repr(C, packed)]
struct FooULE {
    field1: <char as AsULE>::ULE,   
    field2: <u32 as AsULE>::ULE,
    field3: ZeroSlice<u32>,
}

// Safety (based on the safety checklist on the VarULE trait):
//  1. FooULE does not include any uninitialized or padding bytes. (achieved by `#[repr(C, packed)]` on
//     a struct with only ULE fields)
//  2. FooULE is aligned to 1 byte. (achieved by `#[repr(C, packed)]` on
//     a struct with only ULE fields)
//  3. The impl of `validate_byte_slice()` returns an error if any byte is not valid.
//  4. The impl of `validate_byte_slice()` returns an error if the slice cannot be used in its entirety
//  5. The impl of `from_byte_slice_unchecked()` returns a reference to the same data.
//  6. The other VarULE methods use the default impl.
//  7. FooULE byte equality is semantic equality
unsafe impl VarULE for FooULE {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        // validate each field
        <char as AsULE>::ULE::validate_byte_slice(&bytes[0..3]).map_err(|_| ZeroVecError::parse::<Self>())?;
        <u32 as AsULE>::ULE::validate_byte_slice(&bytes[3..7]).map_err(|_| ZeroVecError::parse::<Self>())?;
        let _ = ZeroVec::<u32>::parse_byte_slice(&bytes[7..]).map_err(|_| ZeroVecError::parse::<Self>())?;
        Ok(())
    }
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        let ptr = bytes.as_ptr();
        let len = bytes.len();
        // subtract the length of the char and u32 to get the length of the array
        let len_new = (len - 7) / 4;
        // it's hard constructing custom DSTs, we fake a pointer/length construction
        // eventually we can use the Pointer::Metadata APIs when they stabilize
        let fake_slice = core::ptr::slice_from_raw_parts(ptr as *const <u32 as AsULE>::ULE, len_new);
        &*(fake_slice as *const Self)
    }
}

unsafe impl EncodeAsVarULE<FooULE> for Foo<'_> {
   fn encode_var_ule_as_slices<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
       // take each field, convert to ULE byte slices, and pass them through
       cb(&[<char as AsULE>::ULE::as_byte_slice(&[self.field1.to_unaligned()]),
            <u32 as AsULE>::ULE::as_byte_slice(&[self.field2.to_unaligned()]),
            // the ZeroVec is already in the correct slice format
            self.field3.as_bytes()])
   }
}

impl<'a> ZeroFrom<'a, FooULE> for Foo<'a> {
    fn zero_from(other: &'a FooULE) -> Self {
        Self {
            field1: AsULE::from_unaligned(other.field1),
            field2: AsULE::from_unaligned(other.field2),
            field3: ZeroFrom::zero_from(&other.field3),
        }
    }
}


impl serde::Serialize for FooULE
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Foo::zero_from(self).serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Box<FooULE>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut foo = Foo::deserialize(deserializer)?;
        Ok(encode_varule_to_box(&foo))
    }
}

fn main() {
    let mut foos = [Foo {field1: 'u', field2: 983, field3: ZeroVec::alloc_from_slice(&[1212,2309,500,7000])},
                    Foo {field1: 'l', field2: 1010, field3: ZeroVec::alloc_from_slice(&[1932, 0, 8888, 91237])}];

    let vzv = VarZeroVec::<_>::from(&foos);

    assert_eq!(char::from_unaligned(vzv.get(0).unwrap().field1), 'u');
    assert_eq!(u32::from_unaligned(vzv.get(0).unwrap().field2), 983);
    assert_eq!(&vzv.get(0).unwrap().field3, &[1212,2309,500,7000][..]);

    assert_eq!(char::from_unaligned(vzv.get(1).unwrap().field1), 'l');
    assert_eq!(u32::from_unaligned(vzv.get(1).unwrap().field2), 1010);
    assert_eq!(&vzv.get(1).unwrap().field3, &[1932, 0, 8888, 91237][..]);
}
```

```rust
pub mod custom { /* ... */ }
```

## Module `tuple`

ULE impls for tuples.

Rust does not guarantee the layout of tuples, so ZeroVec defines its own tuple ULE types.

Impls are defined for tuples of up to 6 elements. For longer tuples, use a custom struct
with [`#[make_ule]`](crate::make_ule).

# Examples

```
use zerovec::ZeroVec;

// ZeroVec of tuples!
let zerovec: ZeroVec<(u32, char)> = [(1, 'a'), (1234901, '啊'), (100, 'अ')]
    .iter()
    .copied()
    .collect();

assert_eq!(zerovec.get(1), Some((1234901, '啊')));
```

```rust
pub mod tuple { /* ... */ }
```

### Types

#### Struct `Tuple2ULE`

**Attributes:**

- `#[allow(clippy :: exhaustive_structs)]`
- `#[repr(C, packed(1))]`

ULE type for tuples with 2 elements.

```rust
pub struct Tuple2ULE<A, B>(pub A, pub B);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `A` |  |
| 1 | `B` |  |

##### Implementations

###### Trait Implementations

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ULE**
  - ```rust
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Copy**
- **Send**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

#### Struct `Tuple3ULE`

**Attributes:**

- `#[allow(clippy :: exhaustive_structs)]`
- `#[repr(C, packed(1))]`

ULE type for tuples with 3 elements.

```rust
pub struct Tuple3ULE<A, B, C>(pub A, pub B, pub C);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `A` |  |
| 1 | `B` |  |
| 2 | `C` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering { /* ... */ }
    ```

- **ULE**
  - ```rust
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering> { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

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

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

#### Struct `Tuple4ULE`

**Attributes:**

- `#[allow(clippy :: exhaustive_structs)]`
- `#[repr(C, packed(1))]`

ULE type for tuples with 4 elements.

```rust
pub struct Tuple4ULE<A, B, C, D>(pub A, pub B, pub C, pub D);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `A` |  |
| 1 | `B` |  |
| 2 | `C` |  |
| 3 | `D` |  |

##### Implementations

###### Trait Implementations

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **ULE**
  - ```rust
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **ErasedDestructor**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Tuple5ULE`

**Attributes:**

- `#[allow(clippy :: exhaustive_structs)]`
- `#[repr(C, packed(1))]`

ULE type for tuples with 5 elements.

```rust
pub struct Tuple5ULE<A, B, C, D, E>(pub A, pub B, pub C, pub D, pub E);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `A` |  |
| 1 | `B` |  |
| 2 | `C` |  |
| 3 | `D` |  |
| 4 | `E` |  |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering> { /* ... */ }
    ```

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **ErasedDestructor**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Copy**
- **ULE**
  - ```rust
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> { /* ... */ }
    ```

#### Struct `Tuple6ULE`

**Attributes:**

- `#[allow(clippy :: exhaustive_structs)]`
- `#[repr(C, packed(1))]`

ULE type for tuples with 6 elements.

```rust
pub struct Tuple6ULE<A, B, C, D, E, F>(pub A, pub B, pub C, pub D, pub E, pub F);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `A` |  |
| 1 | `B` |  |
| 2 | `C` |  |
| 3 | `D` |  |
| 4 | `E` |  |
| 5 | `F` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
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
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ULE**
  - ```rust
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
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

- **Copy**
### Traits

#### Trait `ULE`

Fixed-width, byte-aligned data that can be cast to and from a little-endian byte slice.

If you need to implement this trait, consider using [`#[make_ule]`](crate::make_ule) or
 [`#[derive(ULE)]`](macro@ULE) instead.

Types that are not fixed-width can implement [`VarULE`] instead.

"ULE" stands for "Unaligned little-endian"

# Safety

Safety checklist for `ULE`:

1. The type *must not* include any uninitialized or padding bytes.
2. The type must have an alignment of 1 byte.
3. The impl of [`ULE::validate_byte_slice()`] *must* return an error if the given byte slice
   would not represent a valid slice of this type.
4. The impl of [`ULE::validate_byte_slice()`] *must* return an error if the given byte slice
   cannot be used in its entirety (if its length is not a multiple of `size_of::<Self>()`).
5. All other methods *must* be left with their default impl, or else implemented according to
   their respective safety guidelines.
6. Acknowledge the following note about the equality invariant.

If the ULE type is a struct only containing other ULE types (or other types which satisfy invariants 1 and 2,
like `[u8; N]`), invariants 1 and 2 can be achieved via `#[repr(C, packed)]` or `#[repr(transparent)]`.

# Equality invariant

A non-safety invariant is that if `Self` implements `PartialEq`, the it *must* be logically
equivalent to byte equality on [`Self::as_byte_slice()`].

It may be necessary to introduce a "canonical form" of the ULE if logical equality does not
equal byte equality. In such a case, [`Self::validate_byte_slice()`] should return an error
for any values that are not in canonical form. For example, the decimal strings "1.23e4" and
"12.3e3" are logically equal, but not byte-for-byte equal, so we could define a canonical form
where only a single digit is allowed before `.`.

Failure to follow this invariant will cause surprising behavior in `PartialEq`, which may
result in unpredictable operations on `ZeroVec`, `VarZeroVec`, and `ZeroMap`.

```rust
pub unsafe trait ULE
where
    Self: Sized + Copy + ''static {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `validate_byte_slice`: Validates a byte slice, `&[u8]`.

##### Provided Methods

- ```rust
  fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], ZeroVecError> { /* ... */ }
  ```
  Parses a byte slice, `&[u8]`, and return it as `&[Self]` with the same lifetime.

- ```rust
  unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &[Self] { /* ... */ }
  ```
  Takes a byte slice, `&[u8]`, and return it as `&[Self]` with the same lifetime, assuming

- ```rust
  fn as_byte_slice(slice: &[Self]) -> &[u8] { /* ... */ }
  ```
  Given `&[Self]`, returns a `&[u8]` with the same lifetime.

##### Implementations

This trait is implemented for the following types:

- `CharULE`
- `NichedOptionULE<U, N>` with <U: NicheBytes<N> + ULE, const N: usize>
- `OptionULE<U>` with <U: ULE>
- `RawBytesULE<N>` with <const N: usize>
- `u8`
- `core::num::NonZeroU8`
- `i8`
- `bool`
- `[T; N]` with <T: ULE, const N: usize>
- `Tuple2ULE<A, B>` with <A: ULE, B: ULE>
- `Tuple3ULE<A, B, C>` with <A: ULE, B: ULE, C: ULE>
- `Tuple4ULE<A, B, C, D>` with <A: ULE, B: ULE, C: ULE, D: ULE>
- `Tuple5ULE<A, B, C, D, E>` with <A: ULE, B: ULE, C: ULE, D: ULE, E: ULE>
- `Tuple6ULE<A, B, C, D, E, F>` with <A: ULE, B: ULE, C: ULE, D: ULE, E: ULE, F: ULE>

#### Trait `AsULE`

A trait for any type that has a 1:1 mapping with an unaligned little-endian (ULE) type.

If you need to implement this trait, consider using [`#[make_ule]`](crate::make_ule) instead.

```rust
pub trait AsULE: Copy {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `ULE`: The ULE type corresponding to `Self`.

###### Required Methods

- `to_unaligned`: Converts from `Self` to `Self::ULE`.
- `from_unaligned`: Converts from `Self::ULE` to `Self`.

##### Implementations

This trait is implemented for the following types:

- `char`
- `NichedOption<U, N>` with <U: AsULE, const N: usize>
- `Option<T>` with <T: AsULE>
- `u16`
- `u32`
- `u64`
- `u128`
- `i16`
- `i32`
- `i64`
- `i128`
- `u8`
- `core::num::NonZeroU8`
- `i8`
- `core::num::NonZeroI8`
- `f32`
- `f64`
- `bool`
- `[T; N]` with <T: AsULE, const N: usize>
- `UnvalidatedChar`
- `(A, B)` with <A: AsULE, B: AsULE>
- `(A, B, C)` with <A: AsULE, B: AsULE, C: AsULE>
- `(A, B, C, D)` with <A: AsULE, B: AsULE, C: AsULE, D: AsULE>
- `(A, B, C, D, E)` with <A: AsULE, B: AsULE, C: AsULE, D: AsULE, E: AsULE>
- `(A, B, C, D, E, F)` with <A: AsULE, B: AsULE, C: AsULE, D: AsULE, E: AsULE, F: AsULE>

#### Trait `EqULE`

An [`EqULE`] type is one whose byte sequence equals the byte sequence of its ULE type on
little-endian platforms. This enables certain performance optimizations, such as
[`ZeroVec::try_from_slice`](crate::ZeroVec::try_from_slice).

# Implementation safety

This trait is safe to implement if the type's ULE (as defined by `impl `[`AsULE`]` for T`)
has an equal byte sequence as the type itself on little-endian platforms; i.e., one where
`*const T` can be cast to a valid `*const T::ULE`.

```rust
pub unsafe trait EqULE: AsULE {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `u16`
- `u32`
- `u64`
- `u128`
- `i16`
- `i32`
- `i64`
- `i128`
- `u8`
- `core::num::NonZeroU8`
- `i8`
- `f32`
- `f64`
- `bool`
- `[T; N]` with <T: EqULE, const N: usize>
- `UnvalidatedChar`

#### Trait `SliceAsULE`

A trait for a type where aligned slices can be cast to unaligned slices.

Auto-implemented on all types implementing [`EqULE`].

```rust
pub trait SliceAsULE
where
    Self: AsULE + Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `slice_to_unaligned`: Converts from `&[Self]` to `&[Self::ULE]` if possible.

##### Implementations

This trait is implemented for the following types:

- `T` with <T>

#### Trait `VarULE`

Variable-width, byte-aligned data that can be cast to and from a little-endian byte slice.

If you need to implement this trait, consider using [`#[make_varule]`](crate::make_varule) or
 [`#[derive(VarULE)]`](macro@VarULE) instead.

This trait is mostly for unsized types like `str` and `[T]`. It can be implemented on sized types;
however, it is much more preferable to use [`ULE`] for that purpose. The [`custom`] module contains
additional documentation on how this type can be implemented on custom types.

If deserialization with `VarZeroVec` is desired is recommended to implement `Deserialize` for
`Box<T>` (serde does not do this automatically for unsized `T`).

For convenience it is typically desired to implement [`EncodeAsVarULE`] and [`ZeroFrom`](zerofrom::ZeroFrom)
on some stack type to convert to and from the ULE type efficiently when necessary.

# Safety

Safety checklist for `VarULE`:

1. The type *must not* include any uninitialized or padding bytes.
2. The type must have an alignment of 1 byte.
3. The impl of [`VarULE::validate_byte_slice()`] *must* return an error if the given byte slice
   would not represent a valid slice of this type.
4. The impl of [`VarULE::validate_byte_slice()`] *must* return an error if the given byte slice
   cannot be used in its entirety.
5. The impl of [`VarULE::from_byte_slice_unchecked()`] must produce a reference to the same
   underlying data assuming that the given bytes previously passed validation.
6. All other methods *must* be left with their default impl, or else implemented according to
   their respective safety guidelines.
7. Acknowledge the following note about the equality invariant.

If the ULE type is a struct only containing other ULE/VarULE types (or other types which satisfy invariants 1 and 2,
like `[u8; N]`), invariants 1 and 2 can be achieved via `#[repr(C, packed)]` or `#[repr(transparent)]`.

# Equality invariant

A non-safety invariant is that if `Self` implements `PartialEq`, the it *must* be logically
equivalent to byte equality on [`Self::as_byte_slice()`].

It may be necessary to introduce a "canonical form" of the ULE if logical equality does not
equal byte equality. In such a case, [`Self::validate_byte_slice()`] should return an error
for any values that are not in canonical form. For example, the decimal strings "1.23e4" and
"12.3e3" are logically equal, but not byte-for-byte equal, so we could define a canonical form
where only a single digit is allowed before `.`.

There may also be cases where a `VarULE` has muiltiple canonical forms, such as a faster
version and a smaller version. The cleanest way to handle this case would be separate types.
However, if this is not feasible, then the application should ensure that the data it is
deserializing is in the expected form. For example, if the data is being loaded from an
external source, then requests could carry information about the expected form of the data.

Failure to follow this invariant will cause surprising behavior in `PartialEq`, which may
result in unpredictable operations on `ZeroVec`, `VarZeroVec`, and `ZeroMap`.

```rust
pub unsafe trait VarULE: ''static {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `validate_byte_slice`: Validates a byte slice, `&[u8]`.
- `from_byte_slice_unchecked`: Takes a byte slice, `&[u8]`, and return it as `&Self` with the same lifetime, assuming

##### Provided Methods

- ```rust
  fn parse_byte_slice(bytes: &[u8]) -> Result<&Self, ZeroVecError> { /* ... */ }
  ```
  Parses a byte slice, `&[u8]`, and return it as `&Self` with the same lifetime.

- ```rust
  fn as_byte_slice(self: &Self) -> &[u8] { /* ... */ }
  ```
  Given `&Self`, returns a `&[u8]` with the same lifetime.

- ```rust
  fn to_boxed(self: &Self) -> Box<Self> { /* ... */ }
  ```
  Allocate on the heap as a `Box<T>`

##### Implementations

This trait is implemented for the following types:

- `VarZeroSlice<T, F>` with <T: VarULE + ?Sized + ''static, F: VarZeroVecFormat>
- `ZeroSlice<T>` with <T: AsULE + ''static>
- `MultiFieldsULE`
- `OptionVarULE<U>` with <U: VarULE + ?Sized>
- `str`
- `[T]` with <T>
- `UnvalidatedStr`

### Re-exports

#### Re-export `ZeroVecError`

```rust
pub use super::ZeroVecError;
```

#### Re-export `CharULE`

```rust
pub use chars::CharULE;
```

#### Re-export `encode_varule_to_box`

```rust
pub use encode::encode_varule_to_box;
```

#### Re-export `EncodeAsVarULE`

```rust
pub use encode::EncodeAsVarULE;
```

#### Re-export `MultiFieldsULE`

```rust
pub use multi::MultiFieldsULE;
```

#### Re-export `NicheBytes`

```rust
pub use niche::NicheBytes;
```

#### Re-export `NichedOption`

```rust
pub use niche::NichedOption;
```

#### Re-export `NichedOptionULE`

```rust
pub use niche::NichedOptionULE;
```

#### Re-export `OptionULE`

```rust
pub use option::OptionULE;
```

#### Re-export `OptionVarULE`

```rust
pub use option::OptionVarULE;
```

#### Re-export `RawBytesULE`

```rust
pub use plain::RawBytesULE;
```

#### Re-export `UnvalidatedChar`

```rust
pub use unvalidated::UnvalidatedChar;
```

#### Re-export `UnvalidatedStr`

```rust
pub use unvalidated::UnvalidatedStr;
```

#### Re-export `ULE`

**Attributes:**

- `#[<cfg>(feature = "derive")]`

Custom derive for [`ULE`].

This can be attached to [`Copy`] structs containing only [`ULE`] types.

Most of the time, it is recommended one use [`#[make_ule]`](crate::make_ule) instead of defining
a custom ULE type.

```rust
pub use zerovec_derive::ULE;
```

#### Re-export `VarULE`

**Attributes:**

- `#[<cfg>(feature = "derive")]`

Custom derive for [`VarULE`]

This can be attached to structs containing only [`ULE`] types with one [`VarULE`] type at the end.

Most of the time, it is recommended one use [`#[make_varule]`](crate::make_varule) instead of defining
a custom [`VarULE`] type.

```rust
pub use zerovec_derive::VarULE;
```

## Module `maps`

This module contains additional utility types and traits for working with
[`ZeroMap`] and [`ZeroMap2d`]. See their docs for more details on the general purpose
of these types.

[`ZeroMapBorrowed`] and [`ZeroMap2dBorrowed`] are versions of [`ZeroMap`] and [`ZeroMap2d`]
that can be used when you wish to guarantee that the map data is always borrowed, leading to
relaxed lifetime constraints.

The [`ZeroMapKV`] trait is required to be implemented on any type that needs to be used
within a map type. [`ZeroVecLike`] and [`MutableZeroVecLike`] are traits used in the
internal workings of the map types, and should typically not be used or implemented by
users of this crate.

```rust
pub mod maps { /* ... */ }
```

### Re-exports

#### Re-export `ZeroMap`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::map::ZeroMap;
```

#### Re-export `ZeroMapBorrowed`

```rust
pub use crate::map::ZeroMapBorrowed;
```

#### Re-export `ZeroMap2d`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::map2d::ZeroMap2d;
```

#### Re-export `ZeroMap2dBorrowed`

```rust
pub use crate::map2d::ZeroMap2dBorrowed;
```

#### Re-export `MutableZeroVecLike`

```rust
pub use crate::map::MutableZeroVecLike;
```

#### Re-export `ZeroMapKV`

```rust
pub use crate::map::ZeroMapKV;
```

#### Re-export `ZeroVecLike`

```rust
pub use crate::map::ZeroVecLike;
```

#### Re-export `ZeroMap2dCursor`

```rust
pub use crate::map2d::ZeroMap2dCursor;
```

## Module `vecs`

This module contains additional utility types for working with
[`ZeroVec`] and  [`VarZeroVec`]. See their docs for more details on the general purpose
of these types.

[`ZeroSlice`] and [`VarZeroSlice`] provide slice-like versions of the vector types
for use behind references and in custom ULE types.

[`VarZeroVecOwned`] is a special owned/mutable version of [`VarZeroVec`], allowing
direct manipulation of the backing buffer.

```rust
pub mod vecs { /* ... */ }
```

### Re-exports

#### Re-export `ZeroSlice`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::zerovec::ZeroSlice;
```

#### Re-export `ZeroVec`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::zerovec::ZeroVec;
```

#### Re-export `VarZeroSlice`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::varzerovec::VarZeroSlice;
```

#### Re-export `VarZeroVec`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use crate::varzerovec::VarZeroVec;
```

#### Re-export `Index16`

```rust
pub use crate::varzerovec::Index16;
```

#### Re-export `Index32`

```rust
pub use crate::varzerovec::Index32;
```

#### Re-export `VarZeroVecFormat`

```rust
pub use crate::varzerovec::VarZeroVecFormat;
```

#### Re-export `VarZeroVecOwned`

```rust
pub use crate::varzerovec::VarZeroVecOwned;
```

#### Re-export `FlexZeroSlice`

```rust
pub use crate::flexzerovec::FlexZeroSlice;
```

#### Re-export `FlexZeroVec`

```rust
pub use crate::flexzerovec::FlexZeroVec;
```

#### Re-export `FlexZeroVecOwned`

```rust
pub use crate::flexzerovec::FlexZeroVecOwned;
```

## Macros

### Macro `zeroslice`

**Attributes:**

- `#[macro_export]`

Convenience wrapper for [`ZeroSlice::from_ule_slice`]. The value will be created at compile-time,
meaning that all arguments must also be constant.

# Arguments

* `$aligned` - The type of an element in its canonical, aligned form, e.g., `char`.
* `$convert` - A const function that converts an `$aligned` into its unaligned equivalent, e.g.,
                `const fn from_aligned(a: CanonicalType) -> CanonicalType::ULE`.
* `$x` - The elements that the `ZeroSlice` will hold.

# Examples

Using array-conversion functions provided by this crate:

```
use zerovec::{ZeroSlice, zeroslice, ule::AsULE};
use zerovec::ule::UnvalidatedChar;

const SIGNATURE: &ZeroSlice<char> = zeroslice!(char; <char as AsULE>::ULE::from_aligned; ['b', 'y', 'e', '✌']);
const EMPTY: &ZeroSlice<u32> = zeroslice![];
const UC: &ZeroSlice<UnvalidatedChar> =
    zeroslice!(
        UnvalidatedChar;
        <UnvalidatedChar as AsULE>::ULE::from_unvalidated_char;
        [UnvalidatedChar::from_char('a')]
    );
let empty: &ZeroSlice<u32> = zeroslice![];
let nums = zeroslice!(u32; <u32 as AsULE>::ULE::from_unsigned; [1, 2, 3, 4, 5]);
assert_eq!(nums.last().unwrap(), 5);
```

Using a custom array-conversion function:

```
use zerovec::{ule::AsULE, ule::RawBytesULE, zeroslice, ZeroSlice};

const fn be_convert(num: i16) -> <i16 as AsULE>::ULE {
    RawBytesULE(num.to_be_bytes())
}

const NUMBERS_BE: &ZeroSlice<i16> =
    zeroslice!(i16; be_convert; [1, -2, 3, -4, 5]);
```

```rust
pub macro_rules! zeroslice {
    /* macro_rules! zeroslice {
    () => { ... };
    ($aligned:ty; $convert:expr; [$($x:expr),+ $(,)?]) => { ... };
} */
}
```

### Macro `zerovec`

**Attributes:**

- `#[macro_export]`

Creates a borrowed `ZeroVec`. Convenience wrapper for `zeroslice!(...).as_zerovec()`. The value
will be created at compile-time, meaning that all arguments must also be constant.

See [`zeroslice!`](crate::zeroslice) for more information.

# Examples

```
use zerovec::{ZeroVec, zerovec, ule::AsULE};

const SIGNATURE: ZeroVec<char> = zerovec!(char; <char as AsULE>::ULE::from_aligned; ['a', 'y', 'e', '✌']);
assert!(!SIGNATURE.is_owned());

const EMPTY: ZeroVec<u32> = zerovec![];
assert!(!EMPTY.is_owned());
```

```rust
pub macro_rules! zerovec {
    /* macro_rules! zerovec {
    () => { ... };
    ($aligned:ty; $convert:expr; [$($x:expr),+ $(,)?]) => { ... };
} */
}
```

### Macro `impl_ule_from_array`

**Attributes:**

- `#[macro_export]`

Given `Self` (`$aligned`), `Self::ULE` (`$unaligned`), and a conversion function (`$single` or
`Self::from_aligned`), implement `from_array` for arrays of `$aligned` to `$unaligned`.

The `$default` argument is due to current compiler limitations.
Pass any (cheap to construct) value.

```rust
pub macro_rules! impl_ule_from_array {
    /* macro_rules! impl_ule_from_array {
    ($aligned:ty, $unaligned:ty, $default:expr, $single:path) => { ... };
    ($aligned:ty, $unaligned:ty, $default:expr) => { ... };
} */
}
```

## Re-exports

### Re-export `ZeroVecError`

```rust
pub use crate::error::ZeroVecError;
```

### Re-export `ZeroMap`

```rust
pub use crate::map::map::ZeroMap;
```

### Re-export `ZeroMap2d`

```rust
pub use crate::map2d::map::ZeroMap2d;
```

### Re-export `VarZeroSlice`

```rust
pub use crate::varzerovec::slice::VarZeroSlice;
```

### Re-export `VarZeroVec`

```rust
pub use crate::varzerovec::vec::VarZeroVec;
```

### Re-export `ZeroSlice`

```rust
pub use crate::zerovec::ZeroSlice;
```

### Re-export `ZeroVec`

```rust
pub use crate::zerovec::ZeroVec;
```

### Re-export `make_ule`

**Attributes:**

- `#[<cfg>(feature = "derive")]`

Generate a corresponding [`ULE`] type and the relevant [`AsULE`] implementations for this type

This can be attached to structs containing only [`AsULE`] types, or C-like enums that have `#[repr(u8)]`
and all explicit discriminants.

The type must be [`Copy`], [`PartialEq`], and [`Eq`].

`#[make_ule]` will automatically derive the following traits on the [`ULE`] type:

- [`Ord`] and [`PartialOrd`]
- [`ZeroMapKV`]

To disable one of the automatic derives, use `#[zerovec::skip_derive(...)]` like so: `#[zerovec::skip_derive(ZeroMapKV)]`.
`Ord` and `PartialOrd` are implemented as a unit and can only be disabled as a group with `#[zerovec::skip_derive(Ord)]`.

The following traits are available to derive, but not automatic:

- [`Debug`]

To enable one of these additional derives, use `#[zerovec::derive(...)]` like so: `#[zerovec::derive(Debug)]`.

In most cases these derives will defer to the impl of the same trait on the current type, so such impls must exist.

For enums, this attribute will generate a crate-public `fn new_from_u8(value: u8) -> Option<Self>`
method on the main type that allows one to construct the value from a u8. If this method is desired
to be more public, it should be wrapped.

[`ULE`]: ule::ULE
[`AsULE`]: ule::AsULE
[`ZeroMapKV`]: maps::ZeroMapKV

# Example

```rust
use zerovec::ZeroVec;

#[zerovec::make_ule(DateULE)]
#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
struct Date {
    y: u64,
    m: u8,
    d: u8,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Dates<'a> {
    #[serde(borrow)]
    dates: ZeroVec<'a, Date>,
}

let dates = Dates {
    dates: ZeroVec::alloc_from_slice(&[
        Date {
            y: 1985,
            m: 9,
            d: 3,
        },
        Date {
            y: 1970,
            m: 2,
            d: 20,
        },
        Date {
            y: 1990,
            m: 6,
            d: 13,
        },
    ]),
};

let bincode_bytes =
    bincode::serialize(&dates).expect("Serialization should be successful");

// Will deserialize without allocations
let deserialized: Dates = bincode::deserialize(&bincode_bytes)
    .expect("Deserialization should be successful");

assert_eq!(deserialized.dates.get(1).unwrap().y, 1970);
assert_eq!(deserialized.dates.get(2).unwrap().d, 13);
```

```rust
pub use zerovec_derive::make_ule;
```

### Re-export `make_varule`

**Attributes:**

- `#[<cfg>(feature = "derive")]`

Generate a corresponding [`VarULE`] type and the relevant [`EncodeAsVarULE`]/[`zerofrom::ZeroFrom`]
implementations for this type

This can be attached to structs containing only [`AsULE`] types with the last fields being
[`Cow<'a, str>`](alloc::borrow::Cow), [`ZeroSlice`], or [`VarZeroSlice`]. If there is more than one such field, it will be represented
using [`MultiFieldsULE`](crate::ule::MultiFieldsULE) and getters will be generated. Other VarULE fields will be detected if they are
tagged with `#[zerovec::varule(NameOfVarULETy)]`.

The type must be [`PartialEq`] and [`Eq`].

[`EncodeAsVarULE`] and [`zerofrom::ZeroFrom`] are useful for avoiding the need to deal with
the [`VarULE`] type directly. In particular, it is recommended to use [`zerofrom::ZeroFrom`]
to convert the [`VarULE`] type back to this type in a cheap, zero-copy way (see the example below
for more details).

`#[make_varule]` will automatically derive the following traits on the [`VarULE`] type:

- [`Ord`] and [`PartialOrd`]
- [`ZeroMapKV`]

To disable one of the automatic derives, use `#[zerovec::skip_derive(...)]` like so: `#[zerovec::skip_derive(ZeroMapKV)]`.
`Ord` and `PartialOrd` are implemented as a unit and can only be disabled as a group with `#[zerovec::skip_derive(Ord)]`.

The following traits are available to derive, but not automatic:

- [`Debug`]
- [`Serialize`](serde::Serialize)
- [`Deserialize`](serde::Deserialize)

To enable one of these additional derives, use `#[zerovec::derive(...)]` like so: `#[zerovec::derive(Debug)]`.

In most cases these derives will defer to the impl of the same trait on the current type, so such impls must exist.

This implementation will also by default autogenerate [`Ord`] and [`PartialOrd`] on the [`VarULE`] type based on
the implementation on `Self`. You can opt out of this with `#[zerovec::skip_derive(Ord)]`

Note that this implementation will autogenerate [`EncodeAsVarULE`] impls for _both_ `Self` and `&Self`
for convenience. This allows for a little more flexibility encoding slices.

[`EncodeAsVarULE`]: ule::EncodeAsVarULE
[`VarULE`]: ule::VarULE
[`ULE`]: ule::ULE
[`AsULE`]: ule::AsULE
[`ZeroMapKV`]: maps::ZeroMapKV

# Example

```rust
use std::borrow::Cow;
use zerofrom::ZeroFrom;
use zerovec::ule::encode_varule_to_box;
use zerovec::{VarZeroVec, ZeroMap, ZeroVec};

// custom fixed-size ULE type for ZeroVec
#[zerovec::make_ule(DateULE)]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
struct Date {
    y: u64,
    m: u8,
    d: u8,
}

// custom variable sized VarULE type for VarZeroVec
#[zerovec::make_varule(PersonULE)]
#[zerovec::derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
struct Person<'a> {
    birthday: Date,
    favorite_character: char,
    #[serde(borrow)]
    name: Cow<'a, str>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Data<'a> {
    // note: VarZeroVec always must reference the ULE type directly
    #[serde(borrow)]
    important_people: VarZeroVec<'a, PersonULE>,
}

let person1 = Person {
    birthday: Date {
        y: 1990,
        m: 9,
        d: 7,
    },
    favorite_character: 'π',
    name: Cow::from("Kate"),
};
let person2 = Person {
    birthday: Date {
        y: 1960,
        m: 5,
        d: 25,
    },
    favorite_character: '冇',
    name: Cow::from("Jesse"),
};

let important_people = VarZeroVec::from(&[person1, person2]);
let data = Data { important_people };

let bincode_bytes = bincode::serialize(&data).expect("Serialization should be successful");

// Will deserialize without allocations
let deserialized: Data =
    bincode::deserialize(&bincode_bytes).expect("Deserialization should be successful");

assert_eq!(&deserialized.important_people.get(1).unwrap().name, "Jesse");
assert_eq!(&deserialized.important_people.get(0).unwrap().name, "Kate");

// Since VarZeroVec produces PersonULE types, it's convenient to use ZeroFrom
// to recoup Person values in a zero-copy way
let person_converted: Person =
    ZeroFrom::zero_from(deserialized.important_people.get(1).unwrap());
assert_eq!(person_converted.name, "Jesse");
assert_eq!(person_converted.birthday.y, 1960);
```

```rust
pub use zerovec_derive::make_varule;
```

