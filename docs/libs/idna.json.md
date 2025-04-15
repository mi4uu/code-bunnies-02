# Crate Documentation

**Version:** 1.0.3

**Format Version:** 43

# Module `idna`

This Rust crate implements IDNA
[per the WHATWG URL Standard](https://url.spec.whatwg.org/#idna).

It also exposes the underlying algorithms from [*Unicode IDNA Compatibility Processing*
(Unicode Technical Standard #46)](http://www.unicode.org/reports/tr46/)
and [Punycode (RFC 3492)](https://tools.ietf.org/html/rfc3492).

Quoting from [UTS #46’s introduction](http://www.unicode.org/reports/tr46/#Introduction):

> Initially, domain names were restricted to ASCII characters.
> A system was introduced in 2003 for internationalized domain names (IDN).
> This system is called Internationalizing Domain Names for Applications,
> or IDNA2003 for short.
> This mechanism supports IDNs by means of a client software transformation
> into a format known as Punycode.
> A revision of IDNA was approved in 2010 (IDNA2008).
> This revision has a number of incompatibilities with IDNA2003.
>
> The incompatibilities force implementers of client software,
> such as browsers and emailers,
> to face difficult choices during the transition period
> as registries shift from IDNA2003 to IDNA2008.
> This document specifies a mechanism
> that minimizes the impact of this transition for client software,
> allowing client software to access domains that are valid under either system.

## Modules

## Module `punycode`

Punycode ([RFC 3492](http://tools.ietf.org/html/rfc3492)) implementation.

Since Punycode fundamentally works on unicode code points,
`encode` and `decode` take and return slices and vectors of `char`.
`encode_str` and `decode_to_string` provide convenience wrappers
that convert from and to Rust’s UTF-8 based `str` and `String` types.

```rust
pub mod punycode { /* ... */ }
```

### Functions

#### Function `decode_to_string`

**Attributes:**

- `#[inline]`

Convert Punycode to an Unicode `String`.

Return None on malformed input or overflow.
Overflow can only happen on inputs that take more than
63 encoded bytes, the DNS limit on domain name labels.

```rust
pub fn decode_to_string(input: &str) -> Option<alloc::string::String> { /* ... */ }
```

#### Function `decode`

Convert Punycode to Unicode.

Return None on malformed input or overflow.
Overflow can only happen on inputs that take more than
63 encoded bytes, the DNS limit on domain name labels.

```rust
pub fn decode(input: &str) -> Option<alloc::vec::Vec<char>> { /* ... */ }
```

#### Function `encode_str`

**Attributes:**

- `#[inline]`

Convert an Unicode `str` to Punycode.

This is a convenience wrapper around `encode`.

```rust
pub fn encode_str(input: &str) -> Option<alloc::string::String> { /* ... */ }
```

#### Function `encode`

Convert Unicode to Punycode.

Return None on overflow, which can only happen on inputs that would take more than
63 encoded bytes, the DNS limit on domain name labels.

```rust
pub fn encode(input: &[char]) -> Option<alloc::string::String> { /* ... */ }
```

## Module `uts46`

This module provides the lower-level API for UTS 46.

[`Uts46::process`] is the core that the other convenience
methods build on.

UTS 46 flags map to this API as follows:

* _CheckHyphens_ - _true_: [`Hyphens::Check`], _false_: [`Hyphens::Allow`]; the WHATWG URL Standard sets this to _false_ for normal (non-conformance-checker) user agents.
* _CheckBidi_ - Always _true_; cannot be configured, since this flag is _true_ even when WHATWG URL Standard _beStrict_ is _false_.
* _CheckJoiners_ - Always _true_; cannot be configured, since this flag is _true_ even when WHATWG URL Standard _beStrict_ is _false_.
* _UseSTD3ASCIIRules_ - _true_: [`AsciiDenyList::STD3`], _false_: [`AsciiDenyList::EMPTY`]; however, the check the WHATWG URL Standard performs right after the UTS 46 invocation corresponds to [`AsciiDenyList::URL`].
* _Transitional_Processing_ - Always _false_ but could be implemented as a preprocessing step. This flag is deprecated and for Web purposes the transition is over in the sense that all of Firefox, Safari, or Chrome set this flag to _false_.
* _VerifyDnsLength_ - _true_: [`DnsLength::Verify`], _false_: [`DnsLength::Ignore`]; the WHATWG URL Standard sets this to _false_ for normal (non-conformance-checker) user agents.
* _IgnoreInvalidPunycode_ - Always _false_; cannot be configured. (Not yet covered by the WHATWG URL Standard, but 2 out of 3 major browser clearly behave as if this was _false_).

```rust
pub mod uts46 { /* ... */ }
```

### Types

#### Struct `AsciiDenyList`

The ASCII deny list to be applied.

```rust
pub struct AsciiDenyList {
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
  pub const fn new(deny_glyphless: bool, deny_list: &str) -> Self { /* ... */ }
  ```
  Computes (preferably at compile time) an ASCII deny list.

###### Trait Implementations

- **Freeze**
- **Copy**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AsciiDenyList) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AsciiDenyList { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **StructuralPartialEq**
#### Enum `Hyphens`

**Attributes:**

- `#[non_exhaustive]`

The _CheckHyphens_ mode.

```rust
pub enum Hyphens {
    Allow,
    CheckFirstLast,
    Check,
}
```

##### Variants

###### `Allow`

_CheckHyphens=false_: Do not place positional restrictions on hyphens.

This mode is used by the WHATWG URL Standard for normal User Agent processing
(i.e. not conformance checking).

###### `CheckFirstLast`

Prohibit hyphens in the first and last position in the label but allow in
the third and fourth position.

Note that this mode rejects real-world names, including some GitHub user pages.

###### `Check`

_CheckHyphens=true_: Prohibit hyphens in the first, third, fourth,
and last position in the label.

Note that this mode rejects real-world names, including YouTube CDN nodes
and some GitHub user pages.

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Hyphens) -> bool { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Hyphens { /* ... */ }
    ```

- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Enum `DnsLength`

**Attributes:**

- `#[non_exhaustive]`

The UTS 46 _VerifyDNSLength_ flag.

```rust
pub enum DnsLength {
    Ignore,
    VerifyAllowRootDot,
    Verify,
}
```

##### Variants

###### `Ignore`

_VerifyDNSLength=false_. (Possibly relevant for allowing non-DNS naming systems.)

###### `VerifyAllowRootDot`

_VerifyDNSLength=true_ with the exception that the trailing root label dot is
allowed.

###### `Verify`

_VerifyDNSLength=true_. (The trailing root label dot is not allowed.)

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DnsLength { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DnsLength) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
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
- **ErasedDestructor**
- **MaybeSendSync**
- **UnwindSafe**
- **Copy**
#### Enum `ErrorPolicy`

**Attributes:**

- `#[non_exhaustive]`

Policy for customizing behavior in case of an error.

```rust
pub enum ErrorPolicy {
    FailFast,
    MarkErrors,
}
```

##### Variants

###### `FailFast`

Return as early as possible without producing output in case of error.

###### `MarkErrors`

In case of error, mark errors with the REPLACEMENT CHARACTER. (The output
containing REPLACEMENT CHARACTERs may be show to the user to illustrate
what was wrong but must not be used for naming in a network protocol.)

##### Implementations

###### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **MaybeSendSync**
- **ErasedDestructor**
- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ErrorPolicy) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
- **Unpin**
- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ErrorPolicy { /* ... */ }
    ```

#### Enum `ProcessingSuccess`

The success outcome of [`Uts46::process`]

```rust
pub enum ProcessingSuccess {
    Passthrough,
    WroteToSink,
}
```

##### Variants

###### `Passthrough`

There were no errors. The caller must consider the input to be the output.

This asserts that the input can be safely passed to [`core::str::from_utf8_unchecked`].

(Distinct from `WroteToSink` in order to allow `Cow` behavior to be implemented on top of
[`Uts46::process`].)

###### `WroteToSink`

There were no errors. The caller must consider what was written to the sink to be the output.

(Distinct from `Passthrough` in order to allow `Cow` behavior to be implemented on top of
[`Uts46::process`].)

##### Implementations

###### Trait Implementations

- **Freeze**
- **ErasedDestructor**
- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ProcessingSuccess { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ProcessingSuccess) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Enum `ProcessingError`

The failure outcome of [`Uts46::process`]

```rust
pub enum ProcessingError {
    ValidityError,
    SinkError,
}
```

##### Variants

###### `ValidityError`

There was a validity error according to the chosen options.

In case of `Operation::ToAscii`, there is no output. Otherwise, output was written to the
sink and the output contains at least one U+FFFD REPLACEMENT CHARACTER to denote an error.

###### `SinkError`

The sink emitted [`core::fmt::Error`]. The partial output written to the sink must not
be used.

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(_: core::fmt::Error) -> Self { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **UnwindSafe**
- **Sync**
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ProcessingError) -> bool { /* ... */ }
    ```

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ProcessingError { /* ... */ }
    ```

- **StructuralPartialEq**
- **Copy**
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

- **ErasedDestructor**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Uts46`

An implementation of UTS #46.

```rust
pub struct Uts46 {
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
  pub const fn new() -> Self { /* ... */ }
  ```
  Constructor using data compiled into the binary.

- ```rust
  pub fn to_ascii<''a>(self: &Self, domain_name: &''a [u8], ascii_deny_list: AsciiDenyList, hyphens: Hyphens, dns_length: DnsLength) -> Result<Cow<''a, str>, crate::Errors> { /* ... */ }
  ```
  Performs the [ToASCII](https://www.unicode.org/reports/tr46/#ToASCII) operation

- ```rust
  pub fn to_unicode<''a>(self: &Self, domain_name: &''a [u8], ascii_deny_list: AsciiDenyList, hyphens: Hyphens) -> (Cow<''a, str>, Result<(), crate::Errors>) { /* ... */ }
  ```
  Performs the [ToUnicode](https://www.unicode.org/reports/tr46/#ToUnicode) operation

- ```rust
  pub fn to_user_interface<''a, OutputUnicode: FnMut(&[char], &[char], bool) -> bool>(self: &Self, domain_name: &''a [u8], ascii_deny_list: AsciiDenyList, hyphens: Hyphens, output_as_unicode: OutputUnicode) -> (Cow<''a, str>, Result<(), crate::Errors>) { /* ... */ }
  ```
  Performs the [ToUnicode](https://www.unicode.org/reports/tr46/#ToUnicode) operation

- ```rust
  pub fn process<W: Write + ?Sized, OutputUnicode: FnMut(&[char], &[char], bool) -> bool>(self: &Self, domain_name: &[u8], ascii_deny_list: AsciiDenyList, hyphens: Hyphens, error_policy: ErrorPolicy, output_as_unicode: OutputUnicode, sink: &mut W, ascii_sink: Option<&mut W>) -> Result<ProcessingSuccess, ProcessingError> { /* ... */ }
  ```
  The lower-level function that [`Uts46::to_ascii`], [`Uts46::to_unicode`], and

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
### Functions

#### Function `verify_dns_length`

Performs the _VerifyDNSLength_ check on the output of the _ToASCII_ operation.

If the second argument is `false`, the trailing root label dot is allowed.

# Panics

Panics in debug mode if the argument isn't ASCII.

```rust
pub fn verify_dns_length(domain_name: &str, allow_trailing_dot: bool) -> bool { /* ... */ }
```

## Types

### Struct `Errors`

**Attributes:**

- `#[non_exhaustive]`

Type indicating that there were errors during UTS #46 processing.

```rust
pub struct Errors {
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Error**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(e: Errors) -> Result<(), Errors> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Errors { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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
- **Sync**
- **Send**
- **UnwindSafe**
- **MaybeSendSync**
- **ErasedDestructor**
## Functions

### Function `domain_to_ascii_cow`

The [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm;
version returning a `Cow`.

Most applications should be using this function rather than the sibling functions,
and most applications should pass [`AsciiDenyList::URL`] as the second argument.
Passing [`AsciiDenyList::URL`] as the second argument makes this function also
perform the [forbidden domain code point](https://url.spec.whatwg.org/#forbidden-domain-code-point)
check in addition to the [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii)
algorithm.

Returns the ASCII representation a domain name,
normalizing characters (upper-case to lower-case and other kinds of equivalence)
and using Punycode as necessary.

This process may fail.

If you have a `&str` instead of `&[u8]`, just call `.to_bytes()` on it before
passing it to this function. It's still preferable to use this function over
the sibling functions that take `&str`.

```rust
pub fn domain_to_ascii_cow(domain: &[u8], ascii_deny_list: AsciiDenyList) -> Result<alloc::borrow::Cow<''_, str>, Errors> { /* ... */ }
```

### Function `domain_to_ascii`

The [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm;
version returning `String` and no ASCII deny list (i.e. _UseSTD3ASCIIRules=false_).

This function exists for backward-compatibility. Consider using [`domain_to_ascii_cow`]
instead.

Return the ASCII representation a domain name,
normalizing characters (upper-case to lower-case and other kinds of equivalence)
and using Punycode as necessary.

This process may fail.

```rust
pub fn domain_to_ascii(domain: &str) -> Result<alloc::string::String, Errors> { /* ... */ }
```

### Function `domain_to_ascii_strict`

The [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm,
with the `beStrict` flag set.

Note that this rejects various real-world names including:
* YouTube CDN nodes
* Some GitHub user pages
* Pseudo-hosts used by various TXT record-based protocols.

```rust
pub fn domain_to_ascii_strict(domain: &str) -> Result<alloc::string::String, Errors> { /* ... */ }
```

### Function `domain_to_unicode`

The [domain to Unicode](https://url.spec.whatwg.org/#concept-domain-to-unicode) algorithm;
version returning `String` and no ASCII deny list (i.e. _UseSTD3ASCIIRules=false_).

This function exists for backward-compatibility. Consider using [`Uts46::to_user_interface`]
or [`Uts46::to_unicode`].

Return the Unicode representation of a domain name,
normalizing characters (upper-case to lower-case and other kinds of equivalence)
and decoding Punycode as necessary.

If the second item of the tuple indicates an error, the first item of the tuple
denotes errors using the REPLACEMENT CHARACTERs in order to be able to illustrate
errors to the user. When the second item of the return tuple signals an error,
the first item of the tuple must not be used in a network protocol.

```rust
pub fn domain_to_unicode(domain: &str) -> (alloc::string::String, Result<(), Errors>) { /* ... */ }
```

## Re-exports

### Re-export `AsciiDenyList`

```rust
pub use uts46::AsciiDenyList;
```

### Re-export `Config`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use crate::deprecated::Config;
```

### Re-export `Idna`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use crate::deprecated::Idna;
```

