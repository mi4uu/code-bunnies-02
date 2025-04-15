# Crate Documentation

**Version:** 0.14.0

**Format Version:** 43

# Module `names`

This crate provides a generate that constructs random name strings suitable
for use in container instances, project names, application instances, etc.

The name `Generator` implements the `Iterator` trait so it can be used with
adapters, consumers, and in loops.

## Usage

This crate is [on crates.io](https://crates.io/crates/names) and can be
used by adding `names` to your dependencies in your project's `Cargo.toml`
file:

```toml
[dependencies]
names = { version = "0.14.0", default-features = false }
```
## Examples

### Example: painless defaults

The easiest way to get started is to use the default `Generator` to return
a name:

```
use names::Generator;

let mut generator = Generator::default();
println!("Your project is: {}", generator.next().unwrap());
// #=> "Your project is: rusty-nail"
```

If more randomness is required, you can generate a name with a trailing
4-digit number:

```
use names::{Generator, Name};

let mut generator = Generator::with_naming(Name::Numbered);
println!("Your project is: {}", generator.next().unwrap());
// #=> "Your project is: pushy-pencil-5602"
```

### Example: with custom dictionaries

If you would rather supply your own custom adjective and noun word lists,
you can provide your own by supplying 2 string slices. For example,
this returns only one result:

```
use names::{Generator, Name};

let adjectives = &["imaginary"];
let nouns = &["roll"];
let mut generator = Generator::new(adjectives, nouns, Name::default());

assert_eq!("imaginary-roll", generator.next().unwrap());
```

## Types

### Enum `Name`

A naming strategy for the `Generator`

```rust
pub enum Name {
    Plain,
    Numbered,
}
```

#### Variants

##### `Plain`

This represents a plain naming strategy of the form `"ADJECTIVE-NOUN"`

##### `Numbered`

This represents a naming strategy with a random number appended to the
end, of the form `"ADJECTIVE-NOUN-NUMBER"`

#### Implementations

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **Sync**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **RefUnwindSafe**
- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

### Struct `Generator`

A random name generator which combines an adjective, a noun, and an
optional number

A `Generator` takes a slice of adjective and noun words strings and has
a naming strategy (with or without a number appended).

```rust
pub struct Generator<''a> {
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
  pub fn new(adjectives: &''a [&''a str], nouns: &''a [&''a str], naming: Name) -> Self { /* ... */ }
  ```
  Constructs a new `Generator<'a>`

- ```rust
  pub fn with_naming(naming: Name) -> Self { /* ... */ }
  ```
  Construct and returns a default `Generator<'a>` containing a large

##### Trait Implementations

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IteratorRandom**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<String> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Constants and Statics

### Constant `ADJECTIVES`

List of English adjective words

```rust
pub const ADJECTIVES: &[&str] = _;
```

### Constant `NOUNS`

List of English noun words

```rust
pub const NOUNS: &[&str] = _;
```

