# Crate Documentation

**Version:** 0.1.45

**Format Version:** 43

# Module `time`

Simple time handling.

# Usage

This crate is [on crates.io](https://crates.io/crates/time) and can be
used by adding `time` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
time = "0.1"
```

And this in your crate root:

```rust
extern crate time;
```

This crate uses the same syntax for format strings as the
[`strftime()`](http://man7.org/linux/man-pages/man3/strftime.3.html)
function from the C standard library.

## Types

### Struct `Timespec`

**Attributes:**

- `#[<cfg_attr>(feature = "rustc-serialize",
derive(RustcEncodable, RustcDecodable))]`

A record specifying a time value in seconds and nanoseconds, where
nanoseconds represent the offset from the given second.

For example a timespec of 1.2 seconds after the beginning of the epoch would
be represented as {sec: 1, nsec: 200000000}.

```rust
pub struct Timespec {
    pub sec: i64,
    pub nsec: i32,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `sec` | `i64` |  |
| `nsec` | `i32` |  |

#### Implementations

##### Methods

- ```rust
  pub fn new(sec: i64, nsec: i32) -> Timespec { /* ... */ }
  ```

##### Trait Implementations

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Send**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Timespec) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Timespec) -> bool { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Timespec) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: Duration) -> Timespec { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Timespec { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: Duration) -> Timespec { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: Timespec) -> Duration { /* ... */ }
    ```

### Struct `PreciseTime`

An opaque structure representing a moment in time.

The only operation that can be performed on a `PreciseTime` is the
calculation of the `Duration` of time that lies between them.

# Examples

Repeatedly call a function for 1 second:

```rust
use time::{Duration, PreciseTime};
# fn do_some_work() {}

let start = PreciseTime::now();

while start.to(PreciseTime::now()) < Duration::seconds(1) {
    do_some_work();
}
```

```rust
pub struct PreciseTime(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn now() -> PreciseTime { /* ... */ }
  ```
  Returns a `PreciseTime` representing the current moment in time.

- ```rust
  pub fn to(self: &Self, later: PreciseTime) -> Duration { /* ... */ }
  ```
  Returns a `Duration` representing the span of time from the value of

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PreciseTime { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Struct `SteadyTime`

A structure representing a moment in time.

`SteadyTime`s are generated by a "steady" clock, that is, a clock which
never experiences discontinuous jumps and for which time always flows at
the same rate.

# Examples

Repeatedly call a function for 1 second:

```rust
# use time::{Duration, SteadyTime};
# fn do_some_work() {}
let start = SteadyTime::now();

while SteadyTime::now() - start < Duration::seconds(1) {
    do_some_work();
}
```

```rust
pub struct SteadyTime(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn now() -> SteadyTime { /* ... */ }
  ```
  Returns a `SteadyTime` representing the current moment in time.

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SteadyTime) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Copy**
- **Sync**
- **StructuralPartialEq**
- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: Duration) -> SteadyTime { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SteadyTime) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SteadyTime) -> bool { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, other: SteadyTime) -> Duration { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, other: Duration) -> SteadyTime { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SteadyTime { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Struct `Tm`

**Attributes:**

- `#[<cfg_attr>(feature = "rustc-serialize",
derive(RustcEncodable, RustcDecodable))]`
- `#[repr(C)]`

Holds a calendar date and time broken down into its components (year, month,
day, and so on), also called a broken-down time value.

```rust
pub struct Tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_utcoff: i32,
    pub tm_nsec: i32,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `tm_sec` | `i32` | Seconds after the minute - [0, 60] |
| `tm_min` | `i32` | Minutes after the hour - [0, 59] |
| `tm_hour` | `i32` | Hours after midnight - [0, 23] |
| `tm_mday` | `i32` | Day of the month - [1, 31] |
| `tm_mon` | `i32` | Months since January - [0, 11] |
| `tm_year` | `i32` | Years since 1900 |
| `tm_wday` | `i32` | Days since Sunday - [0, 6]. 0 = Sunday, 1 = Monday, ..., 6 = Saturday. |
| `tm_yday` | `i32` | Days since January 1 - [0, 365] |
| `tm_isdst` | `i32` | Daylight Saving Time flag.<br><br>This value is positive if Daylight Saving Time is in effect, zero if<br>Daylight Saving Time is not in effect, and negative if this information<br>is not available. |
| `tm_utcoff` | `i32` | Identifies the time zone that was used to compute this broken-down time<br>value, including any adjustment for Daylight Saving Time. This is the<br>number of seconds east of UTC. For example, for U.S. Pacific Daylight<br>Time, the value is `-7*60*60 = -25200`. |
| `tm_nsec` | `i32` | Nanoseconds after the second - [0, 10<sup>9</sup> - 1] |

#### Implementations

##### Methods

- ```rust
  pub fn to_timespec(self: &Self) -> Timespec { /* ... */ }
  ```
  Convert time to the seconds from January 1, 1970

- ```rust
  pub fn to_local(self: &Self) -> Tm { /* ... */ }
  ```
  Convert time to the local timezone

- ```rust
  pub fn to_utc(self: &Self) -> Tm { /* ... */ }
  ```
  Convert time to the UTC

- ```rust
  pub fn ctime(self: &Self) -> TmFmt<''_> { /* ... */ }
  ```
  Returns a TmFmt that outputs according to the `asctime` format in ISO

- ```rust
  pub fn asctime(self: &Self) -> TmFmt<''_> { /* ... */ }
  ```
  Returns a TmFmt that outputs according to the `asctime` format in ISO

- ```rust
  pub fn strftime<''a>(self: &''a Self, format: &''a str) -> Result<TmFmt<''a>, ParseError> { /* ... */ }
  ```
  Formats the time according to the format string.

- ```rust
  pub fn rfc822(self: &Self) -> TmFmt<''_> { /* ... */ }
  ```
  Returns a TmFmt that outputs according to RFC 822.

- ```rust
  pub fn rfc822z(self: &Self) -> TmFmt<''_> { /* ... */ }
  ```
  Returns a TmFmt that outputs according to RFC 822 with Zulu time.

- ```rust
  pub fn rfc3339<''a>(self: &''a Self) -> TmFmt<''_> { /* ... */ }
  ```
  Returns a TmFmt that outputs according to RFC 3339. RFC 3339 is

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, other: Duration) -> Tm { /* ... */ }
    ```
    The resulting Tm is in UTC.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Tm { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

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

- **Sub**
  - ```rust
    fn sub(self: Self, other: Duration) -> Tm { /* ... */ }
    ```
    The resulting Tm is in UTC.

  - ```rust
    fn sub(self: Self, other: Tm) -> Duration { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Tm) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Tm) -> Option<Ordering> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Tm) -> Ordering { /* ... */ }
    ```

- **UnwindSafe**
### Enum `ParseError`

```rust
pub enum ParseError {
    InvalidSecond,
    InvalidMinute,
    InvalidHour,
    InvalidDay,
    InvalidMonth,
    InvalidYear,
    InvalidDayOfWeek,
    InvalidDayOfMonth,
    InvalidDayOfYear,
    InvalidZoneOffset,
    InvalidTime,
    InvalidSecondsSinceEpoch,
    MissingFormatConverter,
    InvalidFormatSpecifier(char),
    UnexpectedCharacter(char, char),
}
```

#### Variants

##### `InvalidSecond`

##### `InvalidMinute`

##### `InvalidHour`

##### `InvalidDay`

##### `InvalidMonth`

##### `InvalidYear`

##### `InvalidDayOfWeek`

##### `InvalidDayOfMonth`

##### `InvalidDayOfYear`

##### `InvalidZoneOffset`

##### `InvalidTime`

##### `InvalidSecondsSinceEpoch`

##### `MissingFormatConverter`

##### `InvalidFormatSpecifier`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `char` |  |

##### `UnexpectedCharacter`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `char` |  |
| 1 | `char` |  |

#### Implementations

##### Trait Implementations

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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Unpin**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ParseError) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParseError { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Struct `TmFmt`

A wrapper around a `Tm` and format string that implements Display.

```rust
pub struct TmFmt<''a> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

## Functions

### Function `get_time`

Returns the current time as a `timespec` containing the seconds and
nanoseconds since 1970-01-01T00:00:00Z.

```rust
pub fn get_time() -> Timespec { /* ... */ }
```

### Function `precise_time_ns`

**Attributes:**

- `#[inline]`

Returns the current value of a high-resolution performance counter
in nanoseconds since an unspecified epoch.

```rust
pub fn precise_time_ns() -> u64 { /* ... */ }
```

### Function `precise_time_s`

Returns the current value of a high-resolution performance counter
in seconds since an unspecified epoch.

```rust
pub fn precise_time_s() -> f64 { /* ... */ }
```

### Function `tzset`

**Attributes:**

- `#[<cfg>(not(any(windows, target_env = "sgx")))]`

```rust
pub fn tzset() { /* ... */ }
```

### Function `empty_tm`

```rust
pub fn empty_tm() -> Tm { /* ... */ }
```

### Function `at_utc`

Returns the specified time in UTC

```rust
pub fn at_utc(clock: Timespec) -> Tm { /* ... */ }
```

### Function `now_utc`

Returns the current time in UTC

```rust
pub fn now_utc() -> Tm { /* ... */ }
```

### Function `at`

Returns the specified time in the local timezone

```rust
pub fn at(clock: Timespec) -> Tm { /* ... */ }
```

### Function `now`

Returns the current time in the local timezone

```rust
pub fn now() -> Tm { /* ... */ }
```

### Function `strftime`

Formats the time according to the format string.

```rust
pub fn strftime(format: &str, tm: &Tm) -> Result<String, ParseError> { /* ... */ }
```

## Re-exports

### Re-export `Duration`

```rust
pub use duration::Duration;
```

### Re-export `OutOfRangeError`

```rust
pub use duration::OutOfRangeError;
```

### Re-export `strptime`

```rust
pub use parse::strptime;
```

