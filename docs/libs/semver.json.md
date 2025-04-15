# Crate Documentation

**Version:** 1.0.26

**Format Version:** 43

# Module `semver`

[![github]](https://github.com/dtolnay/semver)&ensp;[![crates-io]](https://crates.io/crates/semver)&ensp;[![docs-rs]](https://docs.rs/semver)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

A parser and evaluator for Cargo's flavor of Semantic Versioning.

Semantic Versioning (see <https://semver.org>) is a guideline for how
version numbers are assigned and incremented. It is widely followed within
the Cargo/crates.io ecosystem for Rust.

<br>

# Example

```
use semver::{BuildMetadata, Prerelease, Version, VersionReq};

fn main() {
    let req = VersionReq::parse(">=1.2.3, <1.8.0").unwrap();

    // Check whether this requirement matches version 1.2.3-alpha.1 (no)
    let version = Version {
        major: 1,
        minor: 2,
        patch: 3,
        pre: Prerelease::new("alpha.1").unwrap(),
        build: BuildMetadata::EMPTY,
    };
    assert!(!req.matches(&version));

    // Check whether it matches 1.3.0 (yes it does)
    let version = Version::parse("1.3.0").unwrap();
    assert!(req.matches(&version));
}
```

<br><br>

# Scope of this crate

Besides Cargo, several other package ecosystems and package managers for
other languages also use SemVer:&ensp;RubyGems/Bundler for Ruby, npm for
JavaScript, Composer for PHP, CocoaPods for Objective-C...

The `semver` crate is specifically intended to implement Cargo's
interpretation of Semantic Versioning.

Where the various tools differ in their interpretation or implementation of
the spec, this crate follows the implementation choices made by Cargo. If
you are operating on version numbers from some other package ecosystem, you
will want to use a different semver library which is appropriate to that
ecosystem.

The extent of Cargo's SemVer support is documented in the *[Specifying
Dependencies]* chapter of the Cargo reference.

[Specifying Dependencies]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html

## Types

### Struct `Version`

**SemVer version** as defined by <https://semver.org>.

# Syntax

- The major, minor, and patch numbers may be any integer 0 through u64::MAX.
  When representing a SemVer version as a string, each number is written as
  a base 10 integer. For example, `1.0.119`.

- Leading zeros are forbidden in those positions. For example `1.01.00` is
  invalid as a SemVer version.

- The pre-release identifier, if present, must conform to the syntax
  documented for [`Prerelease`].

- The build metadata, if present, must conform to the syntax documented for
  [`BuildMetadata`].

- Whitespace is not allowed anywhere in the version.

# Total ordering

Given any two SemVer versions, one is less than, greater than, or equal to
the other. Versions may be compared against one another using Rust's usual
comparison operators.

- The major, minor, and patch number are compared numerically from left to
  right, lexicographically ordered as a 3-tuple of integers. So for example
  version `1.5.0` is less than version `1.19.0`, despite the fact that
  "1.19.0" &lt; "1.5.0" as ASCIIbetically compared strings and 1.19 &lt; 1.5
  as real numbers.

- When major, minor, and patch are equal, a pre-release version is
  considered less than the ordinary release:&ensp;version `1.0.0-alpha.1` is
  less than version `1.0.0`.

- Two pre-releases of the same major, minor, patch are compared by
  lexicographic ordering of dot-separated components of the pre-release
  string.

  - Identifiers consisting of only digits are compared
    numerically:&ensp;`1.0.0-pre.8` is less than `1.0.0-pre.12`.

  - Identifiers that contain a letter or hyphen are compared in ASCII sort
    order:&ensp;`1.0.0-pre12` is less than `1.0.0-pre8`.

  - Any numeric identifier is always less than any non-numeric
    identifier:&ensp;`1.0.0-pre.1` is less than `1.0.0-pre.x`.

Example:&ensp;`1.0.0-alpha`&ensp;&lt;&ensp;`1.0.0-alpha.1`&ensp;&lt;&ensp;`1.0.0-alpha.beta`&ensp;&lt;&ensp;`1.0.0-beta`&ensp;&lt;&ensp;`1.0.0-beta.2`&ensp;&lt;&ensp;`1.0.0-beta.11`&ensp;&lt;&ensp;`1.0.0-rc.1`&ensp;&lt;&ensp;`1.0.0`

```rust
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Prerelease,
    pub build: BuildMetadata,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `major` | `u64` |  |
| `minor` | `u64` |  |
| `patch` | `u64` |  |
| `pre` | `Prerelease` |  |
| `build` | `BuildMetadata` |  |

#### Implementations

##### Methods

- ```rust
  pub const fn new(major: u64, minor: u64, patch: u64) -> Self { /* ... */ }
  ```
  Create `Version` with an empty pre-release and build metadata.

- ```rust
  pub fn parse(text: &str) -> Result<Self, Error> { /* ... */ }
  ```
  Create `Version` by parsing from string representation.

- ```rust
  pub fn cmp_precedence(self: &Self, other: &Self) -> Ordering { /* ... */ }
  ```
  Compare the major, minor, patch, and pre-release value of two versions,

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Version) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Unpin**
- **DeserializeOwned**
- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Version) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Version { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **FromStr**
  - ```rust
    fn from_str(text: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>
where
    D: Deserializer<''de> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **Serialize**
  - ```rust
    fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: Serializer { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Version) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Struct `VersionReq`

**Attributes:**

- `#[<cfg_attr>(no_const_vec_new, derive(Default))]`

**SemVer version requirement** describing the intersection of some version
comparators, such as `>=1.2.3, <1.8`.

# Syntax

- Either `*` (meaning "any"), or one or more comma-separated comparators.

- A [`Comparator`] is an operator ([`Op`]) and a partial version, separated
  by optional whitespace. For example `>=1.0.0` or `>=1.0`.

- Build metadata is syntactically permitted on the partial versions, but is
  completely ignored, as it's never relevant to whether any comparator
  matches a particular version.

- Whitespace is permitted around commas and around operators. Whitespace is
  not permitted within a partial version, i.e. anywhere between the major
  version number and its minor, patch, pre-release, or build metadata.

```rust
pub struct VersionReq {
    pub comparators: crate::alloc::vec::Vec<Comparator>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `comparators` | `crate::alloc::vec::Vec<Comparator>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn parse(text: &str) -> Result<Self, Error> { /* ... */ }
  ```
  Create `VersionReq` by parsing from string representation.

- ```rust
  pub fn matches(self: &Self, version: &Version) -> bool { /* ... */ }
  ```
  Evaluate whether the given `Version` satisfies the version requirement

##### Trait Implementations

- **Unpin**
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

- **StructuralPartialEq**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Serialize**
  - ```rust
    fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: Serializer { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>
where
    D: Deserializer<''de> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **FromIterator**
  - ```rust
    fn from_iter<I>(iter: I) -> Self
where
    I: IntoIterator<Item = Comparator> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &VersionReq) -> bool { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(text: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> VersionReq { /* ... */ }
    ```

- **Freeze**
- **DeserializeOwned**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
### Struct `Comparator`

A pair of comparison operator and partial version, such as `>=1.2`. Forms
one piece of a VersionReq.

```rust
pub struct Comparator {
    pub op: Op,
    pub major: u64,
    pub minor: Option<u64>,
    pub patch: Option<u64>,
    pub pre: Prerelease,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `op` | `Op` |  |
| `major` | `u64` |  |
| `minor` | `Option<u64>` |  |
| `patch` | `Option<u64>` | Patch is only allowed if minor is Some. |
| `pre` | `Prerelease` | Non-empty pre-release is only allowed if patch is Some. |

#### Implementations

##### Methods

- ```rust
  pub fn parse(text: &str) -> Result<Self, Error> { /* ... */ }
  ```

- ```rust
  pub fn matches(self: &Self, version: &Version) -> bool { /* ... */ }
  ```

##### Trait Implementations

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DeserializeOwned**
- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I>(iter: I) -> Self
where
    I: IntoIterator<Item = Comparator> { /* ... */ }
    ```

- **Sync**
- **FromStr**
  - ```rust
    fn from_str(text: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Comparator { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Serialize**
  - ```rust
    fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: Serializer { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>
where
    D: Deserializer<''de> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Comparator) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Enum `Op`

**Attributes:**

- `#[<cfg_attr>(not(no_non_exhaustive), non_exhaustive)]`
- `#[non_exhaustive]`

SemVer comparison operator: `=`, `>`, `>=`, `<`, `<=`, `~`, `^`, `*`.

# Op::Exact
- &ensp;**`=I.J.K`**&emsp;&mdash;&emsp;exactly the version I.J.K
- &ensp;**`=I.J`**&emsp;&mdash;&emsp;equivalent to `>=I.J.0, <I.(J+1).0`
- &ensp;**`=I`**&emsp;&mdash;&emsp;equivalent to `>=I.0.0, <(I+1).0.0`

# Op::Greater
- &ensp;**`>I.J.K`**
- &ensp;**`>I.J`**&emsp;&mdash;&emsp;equivalent to `>=I.(J+1).0`
- &ensp;**`>I`**&emsp;&mdash;&emsp;equivalent to `>=(I+1).0.0`

# Op::GreaterEq
- &ensp;**`>=I.J.K`**
- &ensp;**`>=I.J`**&emsp;&mdash;&emsp;equivalent to `>=I.J.0`
- &ensp;**`>=I`**&emsp;&mdash;&emsp;equivalent to `>=I.0.0`

# Op::Less
- &ensp;**`<I.J.K`**
- &ensp;**`<I.J`**&emsp;&mdash;&emsp;equivalent to `<I.J.0`
- &ensp;**`<I`**&emsp;&mdash;&emsp;equivalent to `<I.0.0`

# Op::LessEq
- &ensp;**`<=I.J.K`**
- &ensp;**`<=I.J`**&emsp;&mdash;&emsp;equivalent to `<I.(J+1).0`
- &ensp;**`<=I`**&emsp;&mdash;&emsp;equivalent to `<(I+1).0.0`

# Op::Tilde&emsp;("patch" updates)
*Tilde requirements allow the **patch** part of the semver version (the third number) to increase.*
- &ensp;**`~I.J.K`**&emsp;&mdash;&emsp;equivalent to `>=I.J.K, <I.(J+1).0`
- &ensp;**`~I.J`**&emsp;&mdash;&emsp;equivalent to `=I.J`
- &ensp;**`~I`**&emsp;&mdash;&emsp;equivalent to `=I`

# Op::Caret&emsp;("compatible" updates)
*Caret requirements allow parts that are **right of the first nonzero** part of the semver version to increase.*
- &ensp;**`^I.J.K`**&ensp;(for I\>0)&emsp;&mdash;&emsp;equivalent to `>=I.J.K, <(I+1).0.0`
- &ensp;**`^0.J.K`**&ensp;(for J\>0)&emsp;&mdash;&emsp;equivalent to `>=0.J.K, <0.(J+1).0`
- &ensp;**`^0.0.K`**&emsp;&mdash;&emsp;equivalent to `=0.0.K`
- &ensp;**`^I.J`**&ensp;(for I\>0 or J\>0)&emsp;&mdash;&emsp;equivalent to `^I.J.0`
- &ensp;**`^0.0`**&emsp;&mdash;&emsp;equivalent to `=0.0`
- &ensp;**`^I`**&emsp;&mdash;&emsp;equivalent to `=I`

# Op::Wildcard
- &ensp;**`I.J.*`**&emsp;&mdash;&emsp;equivalent to `=I.J`
- &ensp;**`I.*`**&ensp;or&ensp;**`I.*.*`**&emsp;&mdash;&emsp;equivalent to `=I`

```rust
pub enum Op {
    Exact,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Tilde,
    Caret,
    Wildcard,
}
```

#### Variants

##### `Exact`

##### `Greater`

##### `GreaterEq`

##### `Less`

##### `LessEq`

##### `Tilde`

##### `Caret`

##### `Wildcard`

#### Implementations

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Op) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Op { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Struct `Prerelease`

Optional pre-release identifier on a version string. This comes after `-` in
a SemVer version, like `1.0.0-alpha.1`

# Examples

Some real world pre-release idioms drawn from crates.io:

- **[mio]** <code>0.7.0-<b>alpha.1</b></code> &mdash; the most common style
  for numbering pre-releases.

- **[pest]** <code>1.0.0-<b>beta.8</b></code>,&ensp;<code>1.0.0-<b>rc.0</b></code>
  &mdash; this crate makes a distinction between betas and release
  candidates.

- **[sassers]** <code>0.11.0-<b>shitshow</b></code> &mdash; ???.

- **[atomic-utils]** <code>0.0.0-<b>reserved</b></code> &mdash; a squatted
  crate name.

[mio]: https://crates.io/crates/mio
[pest]: https://crates.io/crates/pest
[atomic-utils]: https://crates.io/crates/atomic-utils
[sassers]: https://crates.io/crates/sassers

*Tip:* Be aware that if you are planning to number your own pre-releases,
you should prefer to separate the numeric part from any non-numeric
identifiers by using a dot in between. That is, prefer pre-releases
`alpha.1`, `alpha.2`, etc rather than `alpha1`, `alpha2` etc. The SemVer
spec's rule for pre-release precedence has special treatment of numeric
components in the pre-release string, but only if there are no non-digit
characters in the same dot-separated component. So you'd have `alpha.2` &lt;
`alpha.11` as intended, but `alpha11` &lt; `alpha2`.

# Syntax

Pre-release strings are a series of dot separated identifiers immediately
following the patch version. Identifiers must comprise only ASCII
alphanumerics and hyphens: `0-9`, `A-Z`, `a-z`, `-`. Identifiers must not be
empty. Numeric identifiers must not include leading zeros.

# Total ordering

Pre-releases have a total order defined by the SemVer spec. It uses
lexicographic ordering of dot-separated components. Identifiers consisting
of only digits are compared numerically. Otherwise, identifiers are compared
in ASCII sort order. Any numeric identifier is always less than any
non-numeric identifier.

Example:&ensp;`alpha`&ensp;&lt;&ensp;`alpha.85`&ensp;&lt;&ensp;`alpha.90`&ensp;&lt;&ensp;`alpha.200`&ensp;&lt;&ensp;`alpha.0a`&ensp;&lt;&ensp;`alpha.1a0`&ensp;&lt;&ensp;`alpha.a`&ensp;&lt;&ensp;`beta`

```rust
pub struct Prerelease {
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
  pub fn new(text: &str) -> Result<Self, Error> { /* ... */ }
  ```

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Eq**
- **FromStr**
  - ```rust
    fn from_str(text: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
    ```

- **Receiver**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Prerelease { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Prerelease { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **Freeze**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, rhs: &Self) -> Ordering { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, rhs: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Prerelease) -> bool { /* ... */ }
    ```

### Struct `BuildMetadata`

Optional build metadata identifier. This comes after `+` in a SemVer
version, as in `0.8.1+zstd.1.5.0`.

# Examples

Some real world build metadata idioms drawn from crates.io:

- **[libgit2-sys]** <code>0.12.20+<b>1.1.0</b></code> &mdash; for this
  crate, the build metadata indicates the version of the C libgit2 library
  that the Rust crate is built against.

- **[mashup]** <code>0.1.13+<b>deprecated</b></code> &mdash; just the word
  "deprecated" for a crate that has been superseded by another. Eventually
  people will take notice of this in Cargo's build output where it lists the
  crates being compiled.

- **[google-bigquery2]** <code>2.0.4+<b>20210327</b></code> &mdash; this
  library is automatically generated from an official API schema, and the
  build metadata indicates the date on which that schema was last captured.

- **[fbthrift-git]** <code>0.0.6+<b>c7fcc0e</b></code> &mdash; this crate is
  published from snapshots of a big company monorepo. In monorepo
  development, there is no concept of versions, and all downstream code is
  just updated atomically in the same commit that breaking changes to a
  library are landed. Therefore for crates.io purposes, every published
  version must be assumed to be incompatible with the previous. The build
  metadata provides the source control hash of the snapshotted code.

[libgit2-sys]: https://crates.io/crates/libgit2-sys
[mashup]: https://crates.io/crates/mashup
[google-bigquery2]: https://crates.io/crates/google-bigquery2
[fbthrift-git]: https://crates.io/crates/fbthrift-git

# Syntax

Build metadata is a series of dot separated identifiers immediately
following the patch or pre-release version. Identifiers must comprise only
ASCII alphanumerics and hyphens: `0-9`, `A-Z`, `a-z`, `-`. Identifiers must
not be empty. Leading zeros *are* allowed, unlike any other place in the
SemVer grammar.

# Total ordering

Build metadata is ignored in evaluating `VersionReq`; it plays no role in
whether a `Version` matches any one of the comparison operators.

However for comparing build metadatas among one another, they do have a
total order which is determined by lexicographic ordering of dot-separated
components. Identifiers consisting of only digits are compared numerically.
Otherwise, identifiers are compared in ASCII sort order. Any numeric
identifier is always less than any non-numeric identifier.

Example:&ensp;`demo`&ensp;&lt;&ensp;`demo.85`&ensp;&lt;&ensp;`demo.90`&ensp;&lt;&ensp;`demo.090`&ensp;&lt;&ensp;`demo.200`&ensp;&lt;&ensp;`demo.1a0`&ensp;&lt;&ensp;`demo.a`&ensp;&lt;&ensp;`memo`

```rust
pub struct BuildMetadata {
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
  pub fn new(text: &str) -> Result<Self, Error> { /* ... */ }
  ```

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> BuildMetadata { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

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

- **Eq**
- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Sync**
- **Display**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BuildMetadata) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, rhs: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Receiver**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> BuildMetadata { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, rhs: &Self) -> Ordering { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(text: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
    ```

- **UnwindSafe**
## Re-exports

### Re-export `Error`

```rust
pub use crate::parse::Error;
```

