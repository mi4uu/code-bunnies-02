# Crate Documentation

**Version:** 0.2.6

**Format Version:** 43

# Module `anstyle_parse`

Parser for implementing virtual terminal emulators

[`Parser`] is implemented according to [Paul Williams' ANSI parser
state machine]. The state machine doesn't assign meaning to the parsed data
and is thus not itself sufficient for writing a terminal emulator. Instead,
it is expected that an implementation of [`Perform`] is provided which does
something useful with the parsed data. The [`Parser`] handles the book
keeping, and the [`Perform`] gets to simply handle actions.

# Examples

For an example of using the [`Parser`] please see the examples folder. The example included
there simply logs all the actions [`Perform`] does. One quick thing to see it in action is to
pipe `vim` into it

```sh
cargo build --release --example parselog
vim | target/release/examples/parselog
```

Just type `:q` to exit.

# Differences from original state machine description

* UTF-8 Support for Input
* OSC Strings can be terminated by 0x07
* Only supports 7-bit codes. Some 8-bit codes are still supported, but they no longer work in
  all states.

[Paul Williams' ANSI parser state machine]: https://vt100.net/emu/dec_ansi_parser

## Modules

## Module `state`

ANSI escape code parsing state machine

```rust
pub mod state { /* ... */ }
```

### Functions

#### Function `state_change`

**Attributes:**

- `#[inline]`

Transition to next [`State`]

Note: This does not directly support UTF-8.
- If the data is validated as UTF-8 (e.g. `str`) or single-byte C1 control codes are
  unsupported, then treat [`Action::BeginUtf8`] and [`Action::Execute`] for UTF-8 continuations
  as [`Action::Print`].
- If the data is not validated, then a UTF-8 state machine will need to be implemented on top,
  starting with [`Action::BeginUtf8`].

Note: When [`State::Anywhere`] is returned, revert back to the prior state.

```rust
pub const fn state_change(state: State, byte: u8) -> (State, Action) { /* ... */ }
```

### Re-exports

#### Re-export `Action`

```rust
pub use definitions::Action;
```

#### Re-export `State`

```rust
pub use definitions::State;
```

## Types

### Struct `Parser`

**Attributes:**

- `#[allow(unused_qualifications)]`

Parser for raw _VTE_ protocol which delegates actions to a [`Perform`]

```rust
pub struct Parser<C = DefaultCharAccumulator> {
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
  pub fn new() -> Parser { /* ... */ }
  ```
  Create a new Parser

- ```rust
  pub fn advance<P: Perform>(self: &mut Self, performer: &mut P, byte: u8) { /* ... */ }
  ```
  Advance the parser state

##### Trait Implementations

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Parser<C>) -> bool { /* ... */ }
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

- **Sync**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Parser<C> { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **UnwindSafe**
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Parser<C> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

### Type Alias `DefaultCharAccumulator`

**Attributes:**

- `#[<cfg>(feature = "utf8")]`

Most flexible [`CharAccumulator`] for [`Parser`] based on active features

```rust
pub type DefaultCharAccumulator = Utf8Parser;
```

### Struct `AsciiParser`

**Attributes:**

- `#[allow(clippy::exhaustive_structs)]`

Only allow parsing 7-bit ASCII

```rust
pub struct AsciiParser;
```

#### Implementations

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> AsciiParser { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AsciiParser) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **Unpin**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> AsciiParser { /* ... */ }
    ```

- **StructuralPartialEq**
- **CharAccumulator**
  - ```rust
    fn add(self: &mut Self, _byte: u8) -> Option<char> { /* ... */ }
    ```

### Struct `Utf8Parser`

**Attributes:**

- `#[<cfg>(feature = "utf8")]`

Allow parsing UTF-8

```rust
pub struct Utf8Parser {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Utf8Parser { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **StructuralPartialEq**
- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Utf8Parser) -> bool { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CharAccumulator**
  - ```rust
    fn add(self: &mut Self, byte: u8) -> Option<char> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Utf8Parser { /* ... */ }
    ```

## Traits

### Trait `CharAccumulator`

Build a `char` out of bytes

```rust
pub trait CharAccumulator: Default {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `add`: Build a `char` out of bytes

#### Implementations

This trait is implemented for the following types:

- `AsciiParser`
- `Utf8Parser`

### Trait `Perform`

Performs actions requested by the [`Parser`]

Actions in this case mean, for example, handling a CSI escape sequence describing cursor
movement, or simply printing characters to the screen.

The methods on this type correspond to actions described in
<http://vt100.net/emu/dec_ansi_parser>. I've done my best to describe them in
a useful way in my own words for completeness, but the site should be
referenced if something isn't clear. If the site disappears at some point in
the future, consider checking archive.org.

```rust
pub trait Perform {
    /* Associated items */
}
```

#### Provided Methods

- ```rust
  fn print(self: &mut Self, _c: char) { /* ... */ }
  ```
  Draw a character to the screen and update states.

- ```rust
  fn execute(self: &mut Self, _byte: u8) { /* ... */ }
  ```
  Execute a C0 or C1 control function.

- ```rust
  fn hook(self: &mut Self, _params: &Params, _intermediates: &[u8], _ignore: bool, _action: u8) { /* ... */ }
  ```
  Invoked when a final character arrives in first part of device control string.

- ```rust
  fn put(self: &mut Self, _byte: u8) { /* ... */ }
  ```
  Pass bytes as part of a device control string to the handle chosen in `hook`. C0 controls

- ```rust
  fn unhook(self: &mut Self) { /* ... */ }
  ```
  Called when a device control string is terminated.

- ```rust
  fn osc_dispatch(self: &mut Self, _params: &[&[u8]], _bell_terminated: bool) { /* ... */ }
  ```
  Dispatch an operating system command.

- ```rust
  fn csi_dispatch(self: &mut Self, _params: &Params, _intermediates: &[u8], _ignore: bool, _action: u8) { /* ... */ }
  ```
  A final character has arrived for a CSI sequence

- ```rust
  fn esc_dispatch(self: &mut Self, _intermediates: &[u8], _ignore: bool, _byte: u8) { /* ... */ }
  ```
  The final character of an escape sequence has arrived.

## Re-exports

### Re-export `Params`

```rust
pub use params::Params;
```

### Re-export `ParamsIter`

```rust
pub use params::ParamsIter;
```

