# Crate Documentation

**Version:** 1.1.9

**Format Version:** 43

# Module `camino`

UTF-8 encoded paths.

`camino` is an extension of the `std::path` module that adds new [`Utf8PathBuf`] and [`Utf8Path`]
types. These are like the standard library's [`PathBuf`] and [`Path`] types, except they are
guaranteed to only contain UTF-8 encoded data. Therefore, they expose the ability to get their
contents as strings, they implement `Display`, etc.

The `std::path` types are not guaranteed to be valid UTF-8. This is the right decision for the standard library,
since it must be as general as possible. However, on all platforms, non-Unicode paths are vanishingly uncommon for a
number of reasons:
* Unicode won. There are still some legacy codebases that store paths in encodings like Shift-JIS, but most
  have been converted to Unicode at this point.
* Unicode is the common subset of supported paths across Windows and Unix platforms. (On Windows, Rust stores paths
  as [an extension to UTF-8](https://simonsapin.github.io/wtf-8/), and converts them to UTF-16 at Win32
  API boundaries.)
* There are already many systems, such as Cargo, that only support UTF-8 paths. If your own tool interacts with any such
  system, you can assume that paths are valid UTF-8 without creating any additional burdens on consumers.
* The ["makefile problem"](https://www.mercurial-scm.org/wiki/EncodingStrategy#The_.22makefile_problem.22)
  (which also applies to `Cargo.toml`, and any other metadata file that lists the names of other files) has *no general,
  cross-platform solution* in systems that support non-UTF-8 paths. However, restricting paths to UTF-8 eliminates
  this problem.

Therefore, many programs that want to manipulate paths *do* assume they contain UTF-8 data, and convert them to `str`s
as  necessary. However, because this invariant is not encoded in the `Path` type, conversions such as
`path.to_str().unwrap()` need to be repeated again and again, creating a frustrating experience.

Instead, `camino` allows you to check that your paths are UTF-8 *once*, and then manipulate them
as valid UTF-8 from there on, avoiding repeated lossy and confusing conversions.

## Types

### Struct `Utf8PathBuf`

**Attributes:**

- `#[<cfg_attr>(feature = "serde1",
derive(serde::Serialize, serde::Deserialize))]`
- `#[<cfg_attr>(feature = "serde1", serde(transparent))]`
- `#[serde(transparent)]`

An owned, mutable UTF-8 path (akin to [`String`]).

This type provides methods like [`push`] and [`set_extension`] that mutate
the path in place. It also implements [`Deref`] to [`Utf8Path`], meaning that
all methods on [`Utf8Path`] slices are available on `Utf8PathBuf` values as well.

[`push`]: Utf8PathBuf::push
[`set_extension`]: Utf8PathBuf::set_extension

# Examples

You can use [`push`] to build up a `Utf8PathBuf` from
components:

```
use camino::Utf8PathBuf;

let mut path = Utf8PathBuf::new();

path.push(r"C:\");
path.push("windows");
path.push("system32");

path.set_extension("dll");
```

However, [`push`] is best used for dynamic situations. This is a better way
to do this when you know all of the components ahead of time:

```
use camino::Utf8PathBuf;

let path: Utf8PathBuf = [r"C:\", "windows", "system32.dll"].iter().collect();
```

We can still do better than this! Since these are all strings, we can use
`From::from`:

```
use camino::Utf8PathBuf;

let path = Utf8PathBuf::from(r"C:\windows\system32.dll");
```

Which method works best depends on what kind of situation you're in.

```rust
pub struct Utf8PathBuf(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> Utf8PathBuf { /* ... */ }
  ```
  Allocates an empty `Utf8PathBuf`.

- ```rust
  pub fn from_path_buf(path: PathBuf) -> Result<Utf8PathBuf, PathBuf> { /* ... */ }
  ```
  Creates a new `Utf8PathBuf` from a `PathBuf` containing valid UTF-8 characters.

- ```rust
  pub fn into_std_path_buf(self: Self) -> PathBuf { /* ... */ }
  ```
  Converts a `Utf8PathBuf` to a [`PathBuf`].

- ```rust
  pub fn with_capacity(capacity: usize) -> Utf8PathBuf { /* ... */ }
  ```
  Creates a new `Utf8PathBuf` with a given capacity used to create the internal [`PathBuf`].

- ```rust
  pub fn as_path(self: &Self) -> &Utf8Path { /* ... */ }
  ```
  Coerces to a [`Utf8Path`] slice.

- ```rust
  pub fn push</* synthetic */ impl AsRef<Utf8Path>: AsRef<Utf8Path>>(self: &mut Self, path: impl AsRef<Utf8Path>) { /* ... */ }
  ```
  Extends `self` with `path`.

- ```rust
  pub fn pop(self: &mut Self) -> bool { /* ... */ }
  ```
  Truncates `self` to [`self.parent`].

- ```rust
  pub fn set_file_name</* synthetic */ impl AsRef<str>: AsRef<str>>(self: &mut Self, file_name: impl AsRef<str>) { /* ... */ }
  ```
  Updates [`self.file_name`] to `file_name`.

- ```rust
  pub fn set_extension</* synthetic */ impl AsRef<str>: AsRef<str>>(self: &mut Self, extension: impl AsRef<str>) -> bool { /* ... */ }
  ```
  Updates [`self.extension`] to `extension`.

- ```rust
  pub fn into_string(self: Self) -> String { /* ... */ }
  ```
  Consumes the `Utf8PathBuf`, yielding its internal [`String`] storage.

- ```rust
  pub fn into_os_string(self: Self) -> OsString { /* ... */ }
  ```
  Consumes the `Utf8PathBuf`, yielding its internal [`OsString`] storage.

- ```rust
  pub fn into_boxed_path(self: Self) -> Box<Utf8Path> { /* ... */ }
  ```
  Converts this `Utf8PathBuf` into a [boxed](Box) [`Utf8Path`].

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Invokes [`capacity`] on the underlying instance of [`PathBuf`].

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Invokes [`clear`] on the underlying instance of [`PathBuf`].

- ```rust
  pub fn reserve(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Invokes [`reserve`] on the underlying instance of [`PathBuf`].

- ```rust
  pub fn try_reserve(self: &mut Self, additional: usize) -> Result<(), std::collections::TryReserveError> { /* ... */ }
  ```
  Invokes [`try_reserve`] on the underlying instance of [`PathBuf`].

- ```rust
  pub fn reserve_exact(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Invokes [`reserve_exact`] on the underlying instance of [`PathBuf`].

- ```rust
  pub fn try_reserve_exact(self: &mut Self, additional: usize) -> Result<(), std::collections::TryReserveError> { /* ... */ }
  ```
  Invokes [`try_reserve_exact`] on the underlying instance of [`PathBuf`].

- ```rust
  pub fn shrink_to_fit(self: &mut Self) { /* ... */ }
  ```
  Invokes [`shrink_to_fit`] on the underlying instance of [`PathBuf`].

- ```rust
  pub fn shrink_to(self: &mut Self, min_capacity: usize) { /* ... */ }
  ```
  Invokes [`shrink_to`] on the underlying instance of [`PathBuf`].

##### Trait Implementations

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Utf8Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &OsStr { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Utf8PathBuf { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Utf8Path { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Utf8PathBuf) -> Ordering { /* ... */ }
    ```

- **Receiver**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> Iter<''a> { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> Utf8PathBuf { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Unpin**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Utf8Path) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Cow<''a, Utf8Path>) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Cow<''a, Path>) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &str) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a str) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Cow<''a, str>) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &String) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &OsStr) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a OsStr) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Cow<''a, OsStr>) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &OsString) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Cow<''a, Utf8Path>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Cow<''a, Path>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Cow<''a, str>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &String) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &OsStr) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a OsStr) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Cow<''a, OsStr>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &OsString) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Utf8PathBuf { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(path: PathBuf) -> Result<Utf8PathBuf, <Self as >::Error> { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(string: String) -> Utf8PathBuf { /* ... */ }
    ```

  - ```rust
    fn from(s: &T) -> Utf8PathBuf { /* ... */ }
    ```

  - ```rust
    fn from(path: Box<Utf8Path>) -> Utf8PathBuf { /* ... */ }
    ```

  - ```rust
    fn from(path: Utf8PathBuf) -> Box<Utf8Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: Cow<''a, Utf8Path>) -> Utf8PathBuf { /* ... */ }
    ```

  - ```rust
    fn from(path: Utf8PathBuf) -> String { /* ... */ }
    ```

  - ```rust
    fn from(path: Utf8PathBuf) -> OsString { /* ... */ }
    ```

  - ```rust
    fn from(path: Utf8PathBuf) -> Cow<''a, Utf8Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: Utf8PathBuf) -> Arc<Utf8Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: Utf8PathBuf) -> Rc<Utf8Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: Utf8PathBuf) -> PathBuf { /* ... */ }
    ```

  - ```rust
    fn from(path: Utf8PathBuf) -> Box<Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: Utf8PathBuf) -> Arc<Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: Utf8PathBuf) -> Rc<Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: Utf8PathBuf) -> Cow<''a, Path> { /* ... */ }
    ```

- **DeserializeOwned**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Extend**
  - ```rust
    fn extend<I: IntoIterator<Item = P>>(self: &mut Self, iter: I) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Utf8Path { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

### Struct `Utf8Path`

A slice of a UTF-8 path (akin to [`str`]).

This type supports a number of operations for inspecting a path, including
breaking the path into its components (separated by `/` on Unix and by either
`/` or `\` on Windows), extracting the file name, determining whether the path
is absolute, and so on.

This is an *unsized* type, meaning that it must always be used behind a
pointer like `&` or [`Box`]. For an owned version of this type,
see [`Utf8PathBuf`].

# Examples

```
use camino::Utf8Path;

// Note: this example does work on Windows
let path = Utf8Path::new("./foo/bar.txt");

let parent = path.parent();
assert_eq!(parent, Some(Utf8Path::new("./foo")));

let file_stem = path.file_stem();
assert_eq!(file_stem, Some("bar"));

let extension = path.extension();
assert_eq!(extension, Some("txt"));
```

```rust
pub struct Utf8Path(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn new</* synthetic */ impl AsRef<str> + ?Sized: AsRef<str> + ?Sized>(s: &impl AsRef<str> + ?Sized) -> &Utf8Path { /* ... */ }
  ```
  Directly wraps a string slice as a `Utf8Path` slice.

- ```rust
  pub fn from_path(path: &Path) -> Option<&Utf8Path> { /* ... */ }
  ```
  Converts a [`Path`] to a `Utf8Path`.

- ```rust
  pub fn as_std_path(self: &Self) -> &Path { /* ... */ }
  ```
  Converts a `Utf8Path` to a [`Path`].

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Yields the underlying [`str`] slice.

- ```rust
  pub fn as_os_str(self: &Self) -> &OsStr { /* ... */ }
  ```
  Yields the underlying [`OsStr`] slice.

- ```rust
  pub fn to_path_buf(self: &Self) -> Utf8PathBuf { /* ... */ }
  ```
  Converts a `Utf8Path` to an owned [`Utf8PathBuf`].

- ```rust
  pub fn is_absolute(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the `Utf8Path` is absolute, i.e., if it is independent of

- ```rust
  pub fn is_relative(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the `Utf8Path` is relative, i.e., not absolute.

- ```rust
  pub fn has_root(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the `Utf8Path` has a root.

- ```rust
  pub fn parent(self: &Self) -> Option<&Utf8Path> { /* ... */ }
  ```
  Returns the `Path` without its final component, if there is one.

- ```rust
  pub fn ancestors(self: &Self) -> Utf8Ancestors<''_> { /* ... */ }
  ```
  Produces an iterator over `Utf8Path` and its ancestors.

- ```rust
  pub fn file_name(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Returns the final component of the `Utf8Path`, if there is one.

- ```rust
  pub fn strip_prefix</* synthetic */ impl AsRef<Path>: AsRef<Path>>(self: &Self, base: impl AsRef<Path>) -> Result<&Utf8Path, StripPrefixError> { /* ... */ }
  ```
  Returns a path that, when joined onto `base`, yields `self`.

- ```rust
  pub fn starts_with</* synthetic */ impl AsRef<Path>: AsRef<Path>>(self: &Self, base: impl AsRef<Path>) -> bool { /* ... */ }
  ```
  Determines whether `base` is a prefix of `self`.

- ```rust
  pub fn ends_with</* synthetic */ impl AsRef<Path>: AsRef<Path>>(self: &Self, base: impl AsRef<Path>) -> bool { /* ... */ }
  ```
  Determines whether `child` is a suffix of `self`.

- ```rust
  pub fn file_stem(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Extracts the stem (non-extension) portion of [`self.file_name`].

- ```rust
  pub fn extension(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Extracts the extension of [`self.file_name`], if possible.

- ```rust
  pub fn join</* synthetic */ impl AsRef<Utf8Path>: AsRef<Utf8Path>>(self: &Self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf { /* ... */ }
  ```
  Creates an owned [`Utf8PathBuf`] with `path` adjoined to `self`.

- ```rust
  pub fn join_os</* synthetic */ impl AsRef<Path>: AsRef<Path>>(self: &Self, path: impl AsRef<Path>) -> PathBuf { /* ... */ }
  ```
  Creates an owned [`PathBuf`] with `path` adjoined to `self`.

- ```rust
  pub fn with_file_name</* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, file_name: impl AsRef<str>) -> Utf8PathBuf { /* ... */ }
  ```
  Creates an owned [`Utf8PathBuf`] like `self` but with the given file name.

- ```rust
  pub fn with_extension</* synthetic */ impl AsRef<str>: AsRef<str>>(self: &Self, extension: impl AsRef<str>) -> Utf8PathBuf { /* ... */ }
  ```
  Creates an owned [`Utf8PathBuf`] like `self` but with the given extension.

- ```rust
  pub fn components(self: &Self) -> Utf8Components<''_> { /* ... */ }
  ```
  Produces an iterator over the [`Utf8Component`]s of the path.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_> { /* ... */ }
  ```
  Produces an iterator over the path's components viewed as [`str`]

- ```rust
  pub fn metadata(self: &Self) -> io::Result<fs::Metadata> { /* ... */ }
  ```
  Queries the file system to get information about a file, directory, etc.

- ```rust
  pub fn symlink_metadata(self: &Self) -> io::Result<fs::Metadata> { /* ... */ }
  ```
  Queries the metadata about a file without following symlinks.

- ```rust
  pub fn canonicalize(self: &Self) -> io::Result<PathBuf> { /* ... */ }
  ```
  Returns the canonical, absolute form of the path with all intermediate

- ```rust
  pub fn canonicalize_utf8(self: &Self) -> io::Result<Utf8PathBuf> { /* ... */ }
  ```
  Returns the canonical, absolute form of the path with all intermediate

- ```rust
  pub fn read_link(self: &Self) -> io::Result<PathBuf> { /* ... */ }
  ```
  Reads a symbolic link, returning the file that the link points to.

- ```rust
  pub fn read_link_utf8(self: &Self) -> io::Result<Utf8PathBuf> { /* ... */ }
  ```
  Reads a symbolic link, returning the file that the link points to.

- ```rust
  pub fn read_dir(self: &Self) -> io::Result<fs::ReadDir> { /* ... */ }
  ```
  Returns an iterator over the entries within a directory.

- ```rust
  pub fn read_dir_utf8(self: &Self) -> io::Result<ReadDirUtf8> { /* ... */ }
  ```
  Returns an iterator over the entries within a directory.

- ```rust
  pub fn exists(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the path points at an existing entity.

- ```rust
  pub fn try_exists(self: &Self) -> io::Result<bool> { /* ... */ }
  ```
  Returns `Ok(true)` if the path points at an existing entity.

- ```rust
  pub fn is_file(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the path exists on disk and is pointing at a regular file.

- ```rust
  pub fn is_dir(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the path exists on disk and is pointing at a directory.

- ```rust
  pub fn is_symlink(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the path exists on disk and is pointing at a symbolic link.

- ```rust
  pub fn into_path_buf(self: Box<Utf8Path>) -> Utf8PathBuf { /* ... */ }
  ```
  Converts a `Box<Utf8Path>` into a [`Utf8PathBuf`] without copying or allocating.

##### Trait Implementations

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &Utf8Path { /* ... */ }
    ```

- **UnwindSafe**
- **Sized**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> Iter<''a> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Utf8Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &Utf8Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &Utf8Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &Utf8Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &Utf8Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &Utf8Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &Utf8Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &OsStr { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(path: &''a Path) -> Result<&''a Utf8Path, <Self as >::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> Utf8PathBuf { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Utf8Path) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PathBuf) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Cow<''a, Utf8Path>) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''b Utf8Path) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Cow<''a, Utf8Path>) -> Option<Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Cow<''a, Path>) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Cow<''b, Path>) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &PathBuf) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &str) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a str) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Cow<''a, str>) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &String) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &str) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Cow<''b, str>) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &String) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &OsStr) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a OsStr) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Cow<''a, OsStr>) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &OsString) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &OsStr) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Cow<''b, OsStr>) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &OsString) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a Utf8Path) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>
where
    D: Deserializer<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>
where
    D: Deserializer<''de> { /* ... */ }
    ```

- **Sync**
- **Serialize**
  - ```rust
    fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: Serializer { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Utf8Path) -> Ordering { /* ... */ }
    ```

- **Send**
- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Cow<''a, Utf8Path>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''b Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Cow<''a, Utf8Path>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Cow<''a, Path>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Cow<''b, Path>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &PathBuf) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Cow<''a, str>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &String) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Cow<''b, str>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &String) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &OsStr) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a OsStr) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Cow<''a, OsStr>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &OsString) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &OsStr) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Cow<''b, OsStr>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Utf8Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &OsString) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Utf8Path) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(s: &''a str) -> &''a Utf8Path { /* ... */ }
    ```

  - ```rust
    fn from(s: &T) -> Box<Utf8Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: &Utf8Path) -> Arc<Utf8Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: &Utf8Path) -> Rc<Utf8Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: &''a Utf8Path) -> Cow<''a, Utf8Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: &Utf8Path) -> Box<Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: &Utf8Path) -> Arc<Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: &Utf8Path) -> Rc<Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: &''a Utf8Path) -> Cow<''a, Path> { /* ... */ }
    ```

  - ```rust
    fn from(path: Utf8PathBuf) -> Box<Utf8Path> { /* ... */ }
    ```

### Struct `Utf8Ancestors`

**Attributes:**

- `#[must_use = "iterators are lazy and do nothing unless consumed"]`

An iterator over [`Utf8Path`] and its ancestors.

This `struct` is created by the [`ancestors`] method on [`Utf8Path`].
See its documentation for more.

# Examples

```
use camino::Utf8Path;

let path = Utf8Path::new("/foo/bar");

for ancestor in path.ancestors() {
    println!("{}", ancestor);
}
```

[`ancestors`]: Utf8Path::ancestors

```rust
pub struct Utf8Ancestors<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Trait Implementations

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Utf8Ancestors<''a> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **FusedIterator**
- **UnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Unpin**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Struct `Utf8Components`

**Attributes:**

- `#[must_use = "iterators are lazy and do nothing unless consumed"]`

An iterator over the [`Utf8Component`]s of a [`Utf8Path`].

This `struct` is created by the [`components`] method on [`Utf8Path`].
See its documentation for more.

# Examples

```
use camino::Utf8Path;

let path = Utf8Path::new("/tmp/foo/bar.txt");

for component in path.components() {
    println!("{:?}", component);
}
```

[`components`]: Utf8Path::components

```rust
pub struct Utf8Components<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn as_path(self: &Self) -> &''a Utf8Path { /* ... */ }
  ```
  Extracts a slice corresponding to the portion of the path remaining for iteration.

##### Trait Implementations

- **UnwindSafe**
- **Eq**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Utf8Components<''a>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **StructuralPartialEq**
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
    fn eq(self: &Self, other: &Utf8Components<''a>) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Components<''a>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **FusedIterator**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
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
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Utf8Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &OsStr { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Utf8Components<''a> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
### Struct `Iter`

**Attributes:**

- `#[must_use = "iterators are lazy and do nothing unless consumed"]`

An iterator over the [`Utf8Component`]s of a [`Utf8Path`], as [`str`] slices.

This `struct` is created by the [`iter`] method on [`Utf8Path`].
See its documentation for more.

[`iter`]: Utf8Path::iter

```rust
pub struct Iter<''a> {
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
  pub fn as_path(self: &Self) -> &''a Utf8Path { /* ... */ }
  ```
  Extracts a slice corresponding to the portion of the path remaining for iteration.

##### Trait Implementations

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<&''a str> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Utf8Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &OsStr { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Iter<''a> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **FusedIterator**
- **UnwindSafe**
- **Freeze**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a str> { /* ... */ }
    ```

### Enum `Utf8Component`

A single component of a path.

A `Utf8Component` roughly corresponds to a substring between path separators
(`/` or `\`).

This `enum` is created by iterating over [`Utf8Components`], which in turn is
created by the [`components`](Utf8Path::components) method on [`Utf8Path`].

# Examples

```rust
use camino::{Utf8Component, Utf8Path};

let path = Utf8Path::new("/tmp/foo/bar.txt");
let components = path.components().collect::<Vec<_>>();
assert_eq!(&components, &[
    Utf8Component::RootDir,
    Utf8Component::Normal("tmp"),
    Utf8Component::Normal("foo"),
    Utf8Component::Normal("bar.txt"),
]);
```

```rust
pub enum Utf8Component<''a> {
    Prefix(Utf8PrefixComponent<''a>),
    RootDir,
    CurDir,
    ParentDir,
    Normal(&''a str),
}
```

#### Variants

##### `Prefix`

A Windows path prefix, e.g., `C:` or `\\server\share`.

There is a large variety of prefix types, see [`Utf8Prefix`]'s documentation
for more.

Does not occur on Unix.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Utf8PrefixComponent<''a>` |  |

##### `RootDir`

The root directory component, appears after any prefix and before anything else.

It represents a separator that designates that a path starts from root.

##### `CurDir`

A reference to the current directory, i.e., `.`.

##### `ParentDir`

A reference to the parent directory, i.e., `..`.

##### `Normal`

A normal component, e.g., `a` and `b` in `a/b`.

This variant is the most common one, it represents references to files
or directories.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

#### Implementations

##### Methods

- ```rust
  pub fn as_str(self: &Self) -> &''a str { /* ... */ }
  ```
  Extracts the underlying [`str`] slice.

- ```rust
  pub fn as_os_str(self: &Self) -> &''a OsStr { /* ... */ }
  ```
  Extracts the underlying [`OsStr`] slice.

##### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Utf8Component<''a>) -> bool { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Utf8Component<''a>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Utf8Component<''a> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Component<''a>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Utf8Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &Path { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &OsStr { /* ... */ }
    ```

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

- **StructuralPartialEq**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
### Enum `Utf8Prefix`

Windows path prefixes, e.g., `C:` or `\\server\share`.

Windows uses a variety of path prefix styles, including references to drive
volumes (like `C:`), network shared folders (like `\\server\share`), and
others. In addition, some path prefixes are "verbatim" (i.e., prefixed with
`\\?\`), in which case `/` is *not* treated as a separator and essentially
no normalization is performed.

# Examples

```
use camino::{Utf8Component, Utf8Path, Utf8Prefix};
use camino::Utf8Prefix::*;

fn get_path_prefix(s: &str) -> Utf8Prefix {
    let path = Utf8Path::new(s);
    match path.components().next().unwrap() {
        Utf8Component::Prefix(prefix_component) => prefix_component.kind(),
        _ => panic!(),
    }
}

# if cfg!(windows) {
assert_eq!(Verbatim("pictures"), get_path_prefix(r"\\?\pictures\kittens"));
assert_eq!(VerbatimUNC("server", "share"), get_path_prefix(r"\\?\UNC\server\share"));
assert_eq!(VerbatimDisk(b'C'), get_path_prefix(r"\\?\c:\"));
assert_eq!(DeviceNS("BrainInterface"), get_path_prefix(r"\\.\BrainInterface"));
assert_eq!(UNC("server", "share"), get_path_prefix(r"\\server\share"));
assert_eq!(Disk(b'C'), get_path_prefix(r"C:\Users\Rust\Pictures\Ferris"));
# }
```

```rust
pub enum Utf8Prefix<''a> {
    Verbatim(&''a str),
    VerbatimUNC(&''a str, &''a str),
    VerbatimDisk(u8),
    DeviceNS(&''a str),
    UNC(&''a str, &''a str),
    Disk(u8),
}
```

#### Variants

##### `Verbatim`

Verbatim prefix, e.g., `\\?\cat_pics`.

Verbatim prefixes consist of `\\?\` immediately followed by the given
component.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

##### `VerbatimUNC`

Verbatim prefix using Windows' _**U**niform **N**aming **C**onvention_,
e.g., `\\?\UNC\server\share`.

Verbatim UNC prefixes consist of `\\?\UNC\` immediately followed by the
server's hostname and a share name.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |
| 1 | `&''a str` |  |

##### `VerbatimDisk`

Verbatim disk prefix, e.g., `\\?\C:`.

Verbatim disk prefixes consist of `\\?\` immediately followed by the
drive letter and `:`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### `DeviceNS`

Device namespace prefix, e.g., `\\.\COM42`.

Device namespace prefixes consist of `\\.\` immediately followed by the
device name.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

##### `UNC`

Prefix using Windows' _**U**niform **N**aming **C**onvention_, e.g.
`\\server\share`.

UNC prefixes consist of the server's hostname and a share name.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |
| 1 | `&''a str` |  |

##### `Disk`

Prefix `C:` for the given disk drive.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

#### Implementations

##### Methods

- ```rust
  pub fn is_verbatim(self: &Self) -> bool { /* ... */ }
  ```
  Determines if the prefix is verbatim, i.e., begins with `\\?\`.

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8Prefix<''a>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Utf8Prefix<''a>) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Freeze**
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

- **Copy**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Utf8Prefix<''a> { /* ... */ }
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

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Utf8Prefix<''a>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

### Struct `Utf8PrefixComponent`

A structure wrapping a Windows path prefix as well as its unparsed string
representation.

In addition to the parsed [`Utf8Prefix`] information returned by [`kind`],
`Utf8PrefixComponent` also holds the raw and unparsed [`str`] slice,
returned by [`as_str`].

Instances of this `struct` can be obtained by matching against the
[`Prefix` variant] on [`Utf8Component`].

Does not occur on Unix.

# Examples

```
# if cfg!(windows) {
use camino::{Utf8Component, Utf8Path, Utf8Prefix};
use std::ffi::OsStr;

let path = Utf8Path::new(r"c:\you\later\");
match path.components().next().unwrap() {
    Utf8Component::Prefix(prefix_component) => {
        assert_eq!(Utf8Prefix::Disk(b'C'), prefix_component.kind());
        assert_eq!("c:", prefix_component.as_str());
    }
    _ => unreachable!(),
}
# }
```

[`as_str`]: Utf8PrefixComponent::as_str
[`kind`]: Utf8PrefixComponent::kind
[`Prefix` variant]: Utf8Component::Prefix

```rust
pub struct Utf8PrefixComponent<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn kind(self: &Self) -> Utf8Prefix<''a> { /* ... */ }
  ```
  Returns the parsed prefix data.

- ```rust
  pub fn as_str(self: &Self) -> &''a str { /* ... */ }
  ```
  Returns the [`str`] slice for this prefix.

- ```rust
  pub fn as_os_str(self: &Self) -> &''a OsStr { /* ... */ }
  ```
  Returns the raw [`OsStr`] slice for this prefix.

##### Trait Implementations

- **Send**
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

- **Sync**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Utf8PrefixComponent<''a>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Utf8PrefixComponent<''a>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
- **RefUnwindSafe**
- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Utf8PrefixComponent<''a> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Utf8PrefixComponent<''a>) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

### Struct `ReadDirUtf8`

Iterator over the entries in a directory.

This iterator is returned from [`Utf8Path::read_dir_utf8`] and will yield instances of
<code>[io::Result]<[Utf8DirEntry]></code>. Through a [`Utf8DirEntry`] information like the entry's path
and possibly other metadata can be learned.

The order in which this iterator returns entries is platform and filesystem
dependent.

# Errors

This [`io::Result`] will be an [`Err`] if there's some sort of intermittent
IO error during iteration.

If a directory entry is not UTF-8, an [`io::Error`] is returned with the
[`ErrorKind`](io::ErrorKind) set to `InvalidData` and the payload set to a [`FromPathBufError`].

```rust
pub struct ReadDirUtf8 {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Sync**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<io::Result<Utf8DirEntry>> { /* ... */ }
    ```

### Struct `Utf8DirEntry`

Entries returned by the [`ReadDirUtf8`] iterator.

An instance of `Utf8DirEntry` represents an entry inside of a directory on the filesystem. Each
entry can be inspected via methods to learn about the full path or possibly other metadata.

```rust
pub struct Utf8DirEntry {
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
  pub fn path(self: &Self) -> &Utf8Path { /* ... */ }
  ```
  Returns the full path to the file that this entry represents.

- ```rust
  pub fn metadata(self: &Self) -> io::Result<Metadata> { /* ... */ }
  ```
  Returns the metadata for the file that this entry points at.

- ```rust
  pub fn file_type(self: &Self) -> io::Result<fs::FileType> { /* ... */ }
  ```
  Returns the file type for the file that this entry points at.

- ```rust
  pub fn file_name(self: &Self) -> &str { /* ... */ }
  ```
  Returns the bare file name of this directory entry without any other

- ```rust
  pub fn into_inner(self: Self) -> fs::DirEntry { /* ... */ }
  ```
  Returns the original [`fs::DirEntry`] within this [`Utf8DirEntry`].

- ```rust
  pub fn into_path(self: Self) -> Utf8PathBuf { /* ... */ }
  ```
  Returns the full path to the file that this entry represents.

##### Trait Implementations

- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `FromPathBufError`

A possible error value while converting a [`PathBuf`] to a [`Utf8PathBuf`].

Produced by the `TryFrom<PathBuf>` implementation for [`Utf8PathBuf`].

# Examples

```
use camino::{Utf8PathBuf, FromPathBufError};
use std::convert::{TryFrom, TryInto};
use std::ffi::OsStr;
# #[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

let unicode_path = PathBuf::from("/valid/unicode");
let utf8_path_buf: Utf8PathBuf = unicode_path.try_into().expect("valid Unicode path succeeded");

// Paths on Unix can be non-UTF-8.
# #[cfg(unix)]
let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
# #[cfg(unix)]
let non_unicode_path = PathBuf::from(non_unicode_str);
# #[cfg(unix)]
let err: FromPathBufError = Utf8PathBuf::try_from(non_unicode_path.clone())
    .expect_err("non-Unicode path failed");
# #[cfg(unix)]
assert_eq!(err.as_path(), &non_unicode_path);
# #[cfg(unix)]
assert_eq!(err.into_path_buf(), non_unicode_path);
```

```rust
pub struct FromPathBufError {
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
  pub fn as_path(self: &Self) -> &Path { /* ... */ }
  ```
  Returns the [`Path`] slice that was attempted to be converted to [`Utf8PathBuf`].

- ```rust
  pub fn into_path_buf(self: Self) -> PathBuf { /* ... */ }
  ```
  Returns the [`PathBuf`] that was attempted to be converted to [`Utf8PathBuf`].

- ```rust
  pub fn from_path_error(self: &Self) -> FromPathError { /* ... */ }
  ```
  Fetches a [`FromPathError`] for more about the conversion failure.

- ```rust
  pub fn into_io_error(self: Self) -> io::Error { /* ... */ }
  ```
  Converts self into a [`std::io::Error`] with kind

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FromPathBufError) -> bool { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FromPathBufError { /* ... */ }
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

- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn error::Error + ''static> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **StructuralPartialEq**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
### Struct `FromPathError`

A possible error value while converting a [`Path`] to a [`Utf8Path`].

Produced by the `TryFrom<&Path>` implementation for [`&Utf8Path`](Utf8Path).


# Examples

```
use camino::{Utf8Path, FromPathError};
use std::convert::{TryFrom, TryInto};
use std::ffi::OsStr;
# #[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

let unicode_path = Path::new("/valid/unicode");
let utf8_path: &Utf8Path = unicode_path.try_into().expect("valid Unicode path succeeded");

// Paths on Unix can be non-UTF-8.
# #[cfg(unix)]
let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
# #[cfg(unix)]
let non_unicode_path = Path::new(non_unicode_str);
# #[cfg(unix)]
let err: FromPathError = <&Utf8Path>::try_from(non_unicode_path)
    .expect_err("non-Unicode path failed");
```

```rust
pub struct FromPathError(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn into_io_error(self: Self) -> io::Error { /* ... */ }
  ```
  Converts self into a [`std::io::Error`] with kind

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FromPathError) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn error::Error + ''static> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FromPathError { /* ... */ }
    ```

- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
## Functions

### Function `absolute_utf8`

**Attributes:**

- `#[<cfg>(absolute_path)]`

Makes the path absolute without accessing the filesystem, converting it to a [`Utf8PathBuf`].

If the path is relative, the current directory is used as the base directory. All intermediate
components will be resolved according to platform-specific rules, but unlike
[`canonicalize`][Utf8Path::canonicalize] or [`canonicalize_utf8`](Utf8Path::canonicalize_utf8),
this does not resolve symlinks and may succeed even if the path does not exist.

*Requires Rust 1.79 or newer.*

# Errors

Errors if:

* The path is empty.
* The [current directory][std::env::current_dir] cannot be determined.
* The path is not valid UTF-8.

# Examples

## POSIX paths

```
# #[cfg(unix)]
fn main() -> std::io::Result<()> {
    use camino::Utf8Path;

    // Relative to absolute
    let absolute = camino::absolute_utf8("foo/./bar")?;
    assert!(absolute.ends_with("foo/bar"));

    // Absolute to absolute
    let absolute = camino::absolute_utf8("/foo//test/.././bar.rs")?;
    assert_eq!(absolute, Utf8Path::new("/foo/test/../bar.rs"));
    Ok(())
}
# #[cfg(not(unix))]
# fn main() {}
```

The path is resolved using [POSIX semantics][posix-semantics] except that it stops short of
resolving symlinks. This means it will keep `..` components and trailing slashes.

## Windows paths

```
# #[cfg(windows)]
fn main() -> std::io::Result<()> {
    use camino::Utf8Path;

    // Relative to absolute
    let absolute = camino::absolute_utf8("foo/./bar")?;
    assert!(absolute.ends_with(r"foo\bar"));

    // Absolute to absolute
    let absolute = camino::absolute_utf8(r"C:\foo//test\..\./bar.rs")?;

    assert_eq!(absolute, Utf8Path::new(r"C:\foo\bar.rs"));
    Ok(())
}
# #[cfg(not(windows))]
# fn main() {}
```

For verbatim paths this will simply return the path as given. For other paths this is currently
equivalent to calling [`GetFullPathNameW`][windows-path].

Note that this [may change in the future][changes].

[changes]: io#platform-specific-behavior
[posix-semantics]:
    https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap04.html#tag_04_13
[windows-path]:
    https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfullpathnamew

```rust
pub fn absolute_utf8<P: AsRef<Path>>(path: P) -> io::Result<Utf8PathBuf> { /* ... */ }
```

