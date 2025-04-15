# Crate Documentation

**Version:** 0.3.3

**Format Version:** 43

# Module `indenter`

A few wrappers for the `fmt::Write` objects that efficiently appends and remove
common indentation after every newline

# Setup

Add this to your `Cargo.toml`:

```toml
[dependencies]
indenter = "0.2"
```

# Examples

## Indentation only

This type is intended primarily for writing error reporters that gracefully
format error messages that span multiple lines.

```rust
use std::error::Error;
use core::fmt::{self, Write};
use indenter::indented;

struct ErrorReporter<'a>(&'a dyn Error);

impl fmt::Debug for ErrorReporter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut source = Some(self.0);
        let mut i = 0;

        while let Some(error) = source {
            writeln!(f)?;
            write!(indented(f).ind(i), "{}", error)?;

            source = error.source();
            i += 1;
        }

        Ok(())
    }
}
```

## "Dedenting" (removing common leading indendation)

This type is intended primarily for formatting source code. For example, when
generating code.

This type requires the feature `std`.

```rust
# #[cfg(feature = "std")]
# fn main() {
use std::error::Error;
use core::fmt::{self, Write};
use indenter::CodeFormatter;

let mut output = String::new();
let mut f = CodeFormatter::new(&mut output, "    ");

write!(
    f,
    r#"
    Hello
        World
    "#,
);

assert_eq!(output, "Hello\n    World\n");

let mut output = String::new();
let mut f = CodeFormatter::new(&mut output, "    ");

// it can also indent...
f.indent(2);

write!(
    f,
    r#"
    Hello
        World
    "#,
);

assert_eq!(output, "        Hello\n            World\n");
# }
# #[cfg(not(feature = "std"))]
# fn main() {
# }
```

## Types

### Enum `Format`

**Attributes:**

- `#[allow(missing_debug_implementations)]`

The set of supported formats for indentation

```rust
pub enum Format<''a> {
    Uniform {
        indentation: &''static str,
    },
    Numbered {
        ind: usize,
    },
    Custom {
        inserter: &''a mut Inserter,
    },
}
```

#### Variants

##### `Uniform`

Insert uniform indentation before every line

This format takes a static string as input and inserts it after every newline

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `indentation` | `&''static str` | The string to insert as indentation |

##### `Numbered`

Inserts a number before the first line

This format hard codes the indentation level to match the indentation from
`core::backtrace::Backtrace`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `ind` | `usize` | The index to insert before the first line of output |

##### `Custom`

A custom indenter which is executed after every newline

Custom indenters are passed the current line number and the buffer to be written to as args

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `inserter` | `&''a mut Inserter` | The custom indenter |

#### Implementations

##### Trait Implementations

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Struct `Indented`

**Attributes:**

- `#[allow(missing_debug_implementations)]`

Helper struct for efficiently indenting multi line display implementations

# Explanation

This type will never allocate a string to handle inserting indentation. It instead leverages
the `write_str` function that serves as the foundation of the `core::fmt::Write` trait. This
lets it intercept each piece of output as its being written to the output buffer. It then
splits on newlines giving slices into the original string. Finally we alternate writing these
lines and the specified indentation to the output buffer.

```rust
pub struct Indented<''a, D: ?Sized> {
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
  pub fn ind(self: Self, ind: usize) -> Self { /* ... */ }
  ```
  Sets the format to `Format::Numbered` with the provided index

- ```rust
  pub fn with_str(self: Self, indentation: &''static str) -> Self { /* ... */ }
  ```
  Sets the format to `Format::Uniform` with the provided static string

- ```rust
  pub fn with_format(self: Self, format: Format<''a>) -> Self { /* ... */ }
  ```
  Construct an indenter with a user defined format

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write_str(self: &mut Self, s: &str) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
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

- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Type Alias `Inserter`

A callback for `Format::Custom` used to insert indenation after a new line

The first argument is the line number within the output, starting from 0

```rust
pub type Inserter = dyn FnMut(usize, &mut dyn fmt::Write) -> fmt::Result;
```

## Functions

### Function `indented`

Helper function for creating a default indenter

```rust
pub fn indented<D: ?Sized>(f: &mut D) -> Indented<''_, D> { /* ... */ }
```

