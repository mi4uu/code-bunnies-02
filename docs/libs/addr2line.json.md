# Crate Documentation

**Version:** 0.24.2

**Format Version:** 43

# Module `addr2line`

`addr2line` provides a cross-platform library for retrieving per-address debug information
from files with DWARF debug information. Given an address, it can return the file name,
line number, and function name associated with that address, as well as the inline call
stack leading to that address.

At the lowest level, the library uses a [`Context`] to cache parsed information so that
multiple lookups are efficient. To create a `Context`, you first need to open and parse the
file using an object file parser such as [`object`](https://github.com/gimli-rs/object),
create a [`gimli::Dwarf`], and finally call [`Context::from_dwarf`].

Location information is obtained with [`Context::find_location`] or
[`Context::find_location_range`]. Function information is obtained with
[`Context::find_frames`], which returns a frame for each inline function. Each frame
contains both name and location.

The library also provides a [`Loader`] which internally memory maps the files,
uses the `object` crate to do the parsing, and creates a `Context`.
The `Context` is not exposed, but the `Loader` provides the same functionality
via [`Loader::find_location`], [`Loader::find_location_range`], and
[`Loader::find_frames`]. The `Loader` also provides [`Loader::find_symbol`]
to use the symbol table instead of DWARF debugging information.
The `Loader` will load Mach-O dSYM files and split DWARF files as needed.

The crate has a CLI wrapper around the library which provides some of
the functionality of the `addr2line` command line tool distributed with
[GNU binutils](https://sourceware.org/binutils/docs/binutils/addr2line.html).

## Types

### Struct `Context`

The state necessary to perform address to line translation.

Constructing a `Context` is somewhat costly, so users should aim to reuse `Context`s
when performing lookups for many addresses in the same executable.

```rust
pub struct Context<R: gimli::Reader> {
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
  pub fn from_sections(debug_abbrev: gimli::DebugAbbrev<R>, debug_addr: gimli::DebugAddr<R>, debug_aranges: gimli::DebugAranges<R>, debug_info: gimli::DebugInfo<R>, debug_line: gimli::DebugLine<R>, debug_line_str: gimli::DebugLineStr<R>, debug_ranges: gimli::DebugRanges<R>, debug_rnglists: gimli::DebugRngLists<R>, debug_str: gimli::DebugStr<R>, debug_str_offsets: gimli::DebugStrOffsets<R>, default_section: R) -> Result<Self, gimli::Error> { /* ... */ }
  ```
  Construct a new `Context` from DWARF sections.

- ```rust
  pub fn from_dwarf(sections: gimli::Dwarf<R>) -> Result<Context<R>, gimli::Error> { /* ... */ }
  ```
  Construct a new `Context` from an existing [`gimli::Dwarf`] object.

- ```rust
  pub fn from_arc_dwarf(sections: Arc<gimli::Dwarf<R>>) -> Result<Context<R>, gimli::Error> { /* ... */ }
  ```
  Construct a new `Context` from an existing [`gimli::Dwarf`] object.

- ```rust
  pub fn find_dwarf_and_unit(self: &Self, probe: u64) -> LookupResult<impl LookupContinuation<Output = Option<gimli::UnitRef<''_, R>>, Buf = R>> { /* ... */ }
  ```
  Find the DWARF unit corresponding to the given virtual memory address.

- ```rust
  pub fn find_location(self: &Self, probe: u64) -> Result<Option<Location<''_>>, gimli::Error> { /* ... */ }
  ```
  Find the source file and line corresponding to the given virtual memory address.

- ```rust
  pub fn find_location_range(self: &Self, probe_low: u64, probe_high: u64) -> Result<LocationRangeIter<''_, R>, gimli::Error> { /* ... */ }
  ```
  Return source file and lines for a range of addresses. For each location it also

- ```rust
  pub fn find_frames(self: &Self, probe: u64) -> LookupResult<impl LookupContinuation<Output = Result<FrameIter<''_, R>, gimli::Error>, Buf = R>> { /* ... */ }
  ```
  Return an iterator for the function frames corresponding to the given virtual

- ```rust
  pub fn preload_units(self: &Self, probe: u64) -> impl Iterator<Item = (SplitDwarfLoad<R>, impl FnOnce(Option<Arc<gimli::Dwarf<R>>>) -> Result<(), gimli::Error> + ''_)> { /* ... */ }
  ```
  Preload units for `probe`.

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Send**
- **Unpin**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
## Re-exports

### Re-export `demangle`

```rust
pub use frame::demangle;
```

### Re-export `demangle_auto`

```rust
pub use frame::demangle_auto;
```

### Re-export `Frame`

```rust
pub use frame::Frame;
```

### Re-export `FrameIter`

```rust
pub use frame::FrameIter;
```

### Re-export `FunctionName`

```rust
pub use frame::FunctionName;
```

### Re-export `Location`

```rust
pub use frame::Location;
```

### Re-export `LookupContinuation`

```rust
pub use lookup::LookupContinuation;
```

### Re-export `LookupResult`

```rust
pub use lookup::LookupResult;
```

### Re-export `SplitDwarfLoad`

```rust
pub use lookup::SplitDwarfLoad;
```

### Re-export `LocationRangeIter`

```rust
pub use unit::LocationRangeIter;
```

## Other Items

### Extern Crate `gimli`

```rust
pub extern crate gimli;
```

