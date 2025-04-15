# Crate Documentation

**Version:** 0.16.0

**Format Version:** 43

# Module `tabled`

An easy to use library for pretty print tables of Rust `struct`s and `enum`s.

The library supports different approaches of table building.
You can use [`Tabled`] trait if the data type is known.
Or you can use [`Builder`] to construct the table from scratch.

## Derive

If you want to build a table for your custom type.
A starting point is to a annotate your type with `#[derive(Tabled)]`.

Then to provide your collection to [`Table::new`] and you will be set to render table.

```
use tabled::{Tabled, Table};

#[derive(Tabled)]
struct Language {
    name: &'static str,
    designed_by: &'static str,
    invented_year: usize,
}

let languages = vec![
    Language{
        name: "C",
        designed_by: "Dennis Ritchie",
        invented_year: 1972
    },
    Language{
        name: "Rust",
        designed_by: "Graydon Hoare",
        invented_year: 2010
    },
    Language{
        name: "Go",
        designed_by: "Rob Pike",
        invented_year: 2009
    },
];

let table = Table::new(languages).to_string();

let expected = "+------+----------------+---------------+\n\
                | name | designed_by    | invented_year |\n\
                +------+----------------+---------------+\n\
                | C    | Dennis Ritchie | 1972          |\n\
                +------+----------------+---------------+\n\
                | Rust | Graydon Hoare  | 2010          |\n\
                +------+----------------+---------------+\n\
                | Go   | Rob Pike       | 2009          |\n\
                +------+----------------+---------------+";

assert_eq!(table, expected);
```

BEWARE not all types can derive [`Tabled`] trait.
The example below can't be compiled.

Because `tabled` must know what we're up to print as a field, so
each field must implement [`std::fmt::Display`].

```rust,compile_fail
  # use tabled::Tabled;
    #[derive(Tabled)]
    struct SomeType {
        field1: SomeOtherType,
    }

    struct SomeOtherType;
```

You can tweak it by derive options.

### Default implementations

[`Table`] can be build from vast majority of Rust's standard types.
This allows you to run the following code.

```
use tabled::{Tabled, Table};
let table = Table::new(&[1, 2, 3]);
# let expected = "+-----+\n\
#                 | i32 |\n\
#                 +-----+\n\
#                 | 1   |\n\
#                 +-----+\n\
#                 | 2   |\n\
#                 +-----+\n\
#                 | 3   |\n\
#                 +-----+";
# assert_eq!(table.to_string(), expected);
```

### Builder

When you data scheme is not known at compile time.
You most likely will not able to relay on [`Tabled`] trait.

So one option would be is to use [`Builder`].

```
use std::iter;

use tabled::{
    builder::Builder,
    settings::{Modify, object::Rows, Alignment, Style}
};

let (x, y) = (3, 10);

let mut builder = Builder::default();

let header = iter::once(String::from("i")).chain((0..y).map(|i| i.to_string()));
builder.push_record(header);

for i in 0..x {
    let row = iter::once(i).chain((0..y).map(|j| i * j)).map(|i| i.to_string());
    builder.push_record(row);
}

let table = builder.build()
    .with(Style::rounded())
    .modify(Rows::new(1..), Alignment::left())
    .to_string();

assert_eq!(
    table,
    concat!(
        "╭───┬───┬───┬───┬───┬───┬────┬────┬────┬────┬────╮\n",
        "│ i │ 0 │ 1 │ 2 │ 3 │ 4 │ 5  │ 6  │ 7  │ 8  │ 9  │\n",
        "├───┼───┼───┼───┼───┼───┼────┼────┼────┼────┼────┤\n",
        "│ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0  │ 0  │ 0  │ 0  │ 0  │\n",
        "│ 1 │ 0 │ 1 │ 2 │ 3 │ 4 │ 5  │ 6  │ 7  │ 8  │ 9  │\n",
        "│ 2 │ 0 │ 2 │ 4 │ 6 │ 8 │ 10 │ 12 │ 14 │ 16 │ 18 │\n",
        "╰───┴───┴───┴───┴───┴───┴────┴────┴────┴────┴────╯",
    )
);
```

### Build table using [`row!`] and [`col!`] macros.

```
use tabled::{row, col};

let table = row![
    col!["Hello", "World", "!"],
    col!["Hello"; 3],
    col!["World"; 3],
];

assert_eq!(
    table.to_string(),
    concat!(
        "+-----------+-----------+-----------+\n",
        "| +-------+ | +-------+ | +-------+ |\n",
        "| | Hello | | | Hello | | | World | |\n",
        "| +-------+ | +-------+ | +-------+ |\n",
        "| | World | | | Hello | | | World | |\n",
        "| +-------+ | +-------+ | +-------+ |\n",
        "| | !     | | | Hello | | | World | |\n",
        "| +-------+ | +-------+ | +-------+ |\n",
        "+-----------+-----------+-----------+",
    )
);
```

### Settings

You can use many settings which is found in [`tabled::settings`] module.

# Features

- `std`     - Used by default. If not its considered `no_std` with a limited set of functionality.
- `derive`  - Used by default. A support for `Tabled` derive macro.
- `ansi`    - A support for ANSI sequences.
- `macros`  - A support for `row!`, `col!` macro.

# Advanced

## Table types

[`Table`] keeps data buffered, which sometimes not ideal choice.
For such reason there is [`IterTable`] and [`CompactTable`].

### [`IterTable`]

[`IterTable`] stands on a middle ground between [`Table`] and [`CompactTable`].

It does allocate memory but in a much smaller chunks that a [`Table`] does.
The benefit is that it can be used interchangeably with [`Table`].

```
use tabled::tables::IterTable;

let iterator = (0..3).map(|row| (0..4).map(move |col| format!("{}-{}", row, col)));

let table = IterTable::new(iterator).to_string();

assert_eq!(
    table,
    "+-----+-----+-----+-----+\n\
     | 0-0 | 0-1 | 0-2 | 0-3 |\n\
     +-----+-----+-----+-----+\n\
     | 1-0 | 1-1 | 1-2 | 1-3 |\n\
     +-----+-----+-----+-----+\n\
     | 2-0 | 2-1 | 2-2 | 2-3 |\n\
     +-----+-----+-----+-----+",
);
```

### [`CompactTable`]

Alloc free can be configured ('1) to not make any allocations.
But the price is that the set of settings which can be applied to it is limited.  

It also can be printed directly to [`fmt::Write`] to not have any intermidiaries.

'1. It does not make any allocations in case you provide it with `width` and `count_rows`.

```
use tabled::{settings::Style, tables::CompactTable};
use core::fmt::{Write, Result};

struct StubWriter;

impl Write for StubWriter {
    fn write_str(&mut self, _: &str) -> Result {
        Ok(())
    }
}

let data = [
    ["FreeBSD", "1993", "William and Lynne Jolitz", "?"],
    ["OpenBSD", "1995", "Theo de Raadt", ""],
    ["HardenedBSD", "2014", "Oliver Pinter and Shawn Webb", ""],
];

let table = CompactTable::from(data).with(Style::psql());

table.fmt(StubWriter);
```

## `no_std`

[`CompactTable`] can be used in `no_std` context.

## More information

You can find more examples of settings and attributes in
[README.md](https://github.com/zhiburt/tabled/blob/master/README.md)

[`Builder`]: crate::builder::Builder
[`IterTable`]: crate::tables::IterTable
[`CompactTable`]: crate::tables::CompactTable
[`fmt::Write`]: core::fmt::Write
[`row!`]: crate::row
[`col!`]: crate::col
[`tabled::settings`]: crate::settings

## Modules

## Module `builder`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`
- `#[<cfg_attr>(feature = "derive", doc = "```")]`
- `#[<cfg_attr>(not(feature = "derive"), doc = "```ignore")]`
- `#[<cfg_attr>(feature = "derive", doc = "```")]`
- `#[<cfg_attr>(not(feature = "derive"), doc = "```ignore")]`

Builder module provides a [`Builder`] type which helps building
a [`Table`] dynamically.

It also contains [`IndexBuilder`] which can help to build a table with index.

# Examples

Here's an example of [`IndexBuilder`] usage

```
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct Mission {
    name: &'static str,
    #[tabled(inline)]
    status: Status,
}

#[derive(Tabled)]
enum Status {
    Complete,
    Started,
    Ready,
    Unknown,
}

let data = [
    Mission { name: "Algebra", status: Status::Unknown },
    Mission { name: "Apolo", status: Status::Complete },
];

let mut builder = Table::builder(&data)
    .index()
    .column(0)
    .name(None)
    .transpose();

let mut table = builder.build();
table.with(Style::modern());

println!("{}", table);

assert_eq!(
    table.to_string(),
    concat!(
        "┌──────────┬─────────┬───────┐\n",
        "│          │ Algebra │ Apolo │\n",
        "├──────────┼─────────┼───────┤\n",
        "│ Complete │         │ +     │\n",
        "├──────────┼─────────┼───────┤\n",
        "│ Started  │         │       │\n",
        "├──────────┼─────────┼───────┤\n",
        "│ Ready    │         │       │\n",
        "├──────────┼─────────┼───────┤\n",
        "│ Unknown  │ +       │       │\n",
        "└──────────┴─────────┴───────┘",
   ),
)
```

Example when we don't want to show empty data of enum where not all variants are used.

```
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
enum Status {
    #[tabled(inline)]
    Complete {
        started_timestamp: usize,
        finihsed_timestamp: usize,
    },
    #[tabled(inline)]
    Started {
        timestamp: usize,
    },
    Ready,
    Unknown,
}

let data = [
    Status::Unknown,
    Status::Complete { started_timestamp: 123, finihsed_timestamp: 234 },
];

let table = Table::new(data)
    .with(Style::modern())
    .to_string();

println!("{}", table);

assert_eq!(
    table,
    concat!(
        "┌───────────────────┬────────────────────┬───────────┬───────┬─────────┐\n",
        "│ started_timestamp │ finihsed_timestamp │ timestamp │ Ready │ Unknown │\n",
        "├───────────────────┼────────────────────┼───────────┼───────┼─────────┤\n",
        "│                   │                    │           │       │ +       │\n",
        "├───────────────────┼────────────────────┼───────────┼───────┼─────────┤\n",
        "│ 123               │ 234                │           │       │         │\n",
        "└───────────────────┴────────────────────┴───────────┴───────┴─────────┘",
   ),
)
```

[`Table`]: crate::Table

```rust
pub mod builder { /* ... */ }
```

### Re-exports

#### Re-export `IndexBuilder`

```rust
pub use index_builder::IndexBuilder;
```

#### Re-export `Builder`

```rust
pub use table_builder::Builder;
```

## Module `settings`

**Attributes:**

- `#[<cfg_attr>(feature = "std", doc = "```")]`
- `#[<cfg_attr>(not(feature = "std"), doc = "```ignore")]`

Module contains various table configuration settings.

There 2 types of settings;

- [`CellOption`] which can modify only a cell.
- [`TableOption`] which can modify table as a whole.

[`CellOption`] works on behave of [`Modify`] which is actually a [`TableOption`].

Notice that it's possible to combine settings together by the help of [`Settings`].

```
use tabled::{Table, settings::{Settings, Style, Padding}};

let table_config = Settings::default()
    .with(Padding::new(2, 2, 1, 1))
    .with(Style::rounded());

let data = [[2023;9]; 3];

let table = Table::new(data).with(table_config).to_string();

assert_eq!(
    table,
    "╭────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────╮\n\
     │        │        │        │        │        │        │        │        │        │\n\
     │  0     │  1     │  2     │  3     │  4     │  5     │  6     │  7     │  8     │\n\
     │        │        │        │        │        │        │        │        │        │\n\
     ├────────┼────────┼────────┼────────┼────────┼────────┼────────┼────────┼────────┤\n\
     │        │        │        │        │        │        │        │        │        │\n\
     │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │\n\
     │        │        │        │        │        │        │        │        │        │\n\
     │        │        │        │        │        │        │        │        │        │\n\
     │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │\n\
     │        │        │        │        │        │        │        │        │        │\n\
     │        │        │        │        │        │        │        │        │        │\n\
     │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │\n\
     │        │        │        │        │        │        │        │        │        │\n\
     ╰────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────╯"
)
```

```rust
pub mod settings { /* ... */ }
```

### Modules

## Module `style`

**Attributes:**

- `#[<cfg_attr>(feature = "std", doc = "```")]`
- `#[<cfg_attr>(not(feature = "std"), doc = "```ignore")]`
- `#[<cfg_attr>(feature = "std", doc = "```")]`
- `#[<cfg_attr>(not(feature = "std"), doc = "```ignore")]`
- `#[<cfg_attr>(feature = "std", doc = "```")]`
- `#[<cfg_attr>(not(feature = "std"), doc = "```ignore")]`

This module contains a list of primitives which can be applied to change [`Table`] style.

## [`Style`]

It is responsible for a table border style.
An individual cell border can be set by [`Border`].
  
### Example

```
use tabled::{Table, settings::Style};

let data = vec!["Hello", "2022"];
let mut table = Table::new(&data);
table.with(Style::psql());

assert_eq!(
    table.to_string(),
    concat!(
        " &str  \n",
        "-------\n",
        " Hello \n",
        " 2022  ",
    )
)
```

## [`LineText`]

It's used to override a border with a custom text.

### Example

```
use tabled::{
    Table, settings::{style::{LineText, Style}, object::Rows},
};

let data = vec!["Hello", "2022"];
let table = Table::new(&data)
    .with(Style::psql())
    .with(LineText::new("Santa", Rows::single(1)))
    .to_string();

assert_eq!(
    table,
    concat!(
        " &str  \n",
        "Santa--\n",
        " Hello \n",
        " 2022  ",
    )
)
```

## [`Border`]

[`Border`] can be used to modify cell's borders.

It's possible to set a collored border when `color` feature is on.

### Example

```
use tabled::{Table, settings::{Modify, Style, style::Border}};

let data = vec!["Hello", "2022"];
let table = Table::new(&data)
    .with(Style::psql())
    .modify((0, 0), Border::inherit(Style::modern()))
    .to_string();

assert_eq!(
    table,
    concat!(
        "┌───────┐\n",
        "│ &str  │\n",
        "└───────┘\n",
        "  Hello  \n",
        "  2022   ",
    )
)
```

## [`Theme`]

A different representation of [`Theme`].
With no checks in place.

It also contains a list of types to support colors.

[`Table`]: crate::Table
[`BorderText`]: crate::settings::style::BorderText
[`Theme`]: crate::settings::themes::Theme

```rust
pub mod style { /* ... */ }
```

### Re-exports

#### Re-export `BorderColor`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::border_color::BorderColor;
```

#### Re-export `LineText`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::border_text::LineText;
```

#### Re-export `LineChar`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::line_char::LineChar;
```

#### Re-export `BorderSpanCorrection`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::span_border_correction::BorderSpanCorrection;
```

#### Re-export `Border`

```rust
pub use self::border::Border;
```

#### Re-export `On`

```rust
pub use self::builder::On;
```

#### Re-export `Style`

```rust
pub use self::builder::Style;
```

#### Re-export `HorizontalLine`

```rust
pub use self::horizontal_line::HorizontalLine;
```

#### Re-export `Offset`

```rust
pub use self::offset::Offset;
```

#### Re-export `VerticalLine`

```rust
pub use self::vertical_line::VerticalLine;
```

## Module `object`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

This module contains a list of primitives that implement a [`Object`] trait.
They help to locate a necessary segment on a [`Table`].

[`Table`]: crate::Table

```rust
pub mod object { /* ... */ }
```

### Types

#### Struct `UnionCombination`

Combination struct used for chaining [`Object`]'s.

Combines 2 sets of cells into one.

Duplicates are removed from the output set.

```rust
pub struct UnionCombination<L, R, I> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Object**
  - ```rust
    fn cells(self: &Self, records: &I) -> <Self as >::Iter { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `DiffCombination`

Difference struct used for chaining [`Object`]'s.

Returns cells from 1st set with removed ones from the 2nd set.

```rust
pub struct DiffCombination<L, R, I> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Object**
  - ```rust
    fn cells(self: &Self, records: &I) -> <Self as >::Iter { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `IntersectionCombination`

Intersection struct used for chaining [`Object`]'s.

Returns cells which are present in 2 sets.
But not in one of them

```rust
pub struct IntersectionCombination<L, R, I> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Sync**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Object**
  - ```rust
    fn cells(self: &Self, records: &I) -> <Self as >::Iter { /* ... */ }
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
- **Send**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `InversionCombination`

Inversion struct used for chaining [`Object`]'s.

Returns cells which are present in 2 sets.
But not in one of them

```rust
pub struct InversionCombination<O, I> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Object**
  - ```rust
    fn cells(self: &Self, records: &I) -> <Self as >::Iter { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
#### Struct `UnionIter`

An [`Iterator`] which goes over a combination [`Object::Iter`].

```rust
pub struct UnionIter<L, R> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Send**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

#### Struct `DiffIter`

An [`Iterator`] which goes over only cells which are present in first [`Object::Iter`] but not second.

```rust
pub struct DiffIter<L> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **RefUnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `IntersectIter`

An [`Iterator`] which goes goes over cells which are present in both [`Object::Iter`]ators.

```rust
pub struct IntersectIter<L> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Unpin**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
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

#### Struct `InversionIter`

An [`Iterator`] which goes goes over cells which are not present an [`Object::Iter`]ator.

```rust
pub struct InversionIter {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **RefUnwindSafe**
- **Freeze**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `Object`

Object helps to locate a necessary part of a [`Table`].

[`Table`]: crate::Table

```rust
pub trait Object<R> {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Iter`: An [`Iterator`] which returns a list of cells.

###### Required Methods

- `cells`: Cells returns a set of coordinates of cells.

##### Provided Methods

- ```rust
  fn and<O>(self: Self, rhs: O) -> UnionCombination<Self, O, R>
where
    Self: Sized { /* ... */ }
  ```
  Combines cells.

- ```rust
  fn not<O>(self: Self, rhs: O) -> DiffCombination<Self, O, R>
where
    Self: Sized { /* ... */ }
  ```
  Excludes rhs cells from this cells.

- ```rust
  fn intersect<O>(self: Self, rhs: O) -> IntersectionCombination<Self, O, R>
where
    Self: Sized { /* ... */ }
  ```
  Returns cells which are present in both [`Object`]s only.

- ```rust
  fn inverse(self: Self) -> InversionCombination<Self, R>
where
    Self: Sized { /* ... */ }
  ```
  Returns cells which are not present in target [`Object`].

##### Implementations

This trait is implemented for the following types:

- `Cell` with <I>
- `crate::grid::config::Position` with <I>
- `Columns<R>` with <I, R>
- `FirstColumn` with <I>
- `LastColumn` with <I>
- `Column` with <I>
- `LastColumnOffset` with <I>
- `Frame` with <I>
- `Rows<R>` with <I, R>
- `Row` with <I>
- `FirstRow` with <I>
- `LastRow` with <I>
- `LastRowOffset` with <I>
- `Segment<C, R>` with <I, C, R>
- `SegmentAll` with <I>
- `UnionCombination<L, R, I>` with <I, L, R>
- `DiffCombination<L, R, I>` with <I, L, R>
- `IntersectionCombination<L, R, I>` with <I, L, R>
- `InversionCombination<O, I>` with <I, O>
- `ByColumnName<S>` with <S, R>
- `ByCondition<F>` with <F, R>
- `ByContent<S>` with <S, R>

### Re-exports

#### Re-export `Cell`

```rust
pub use cell::Cell;
```

#### Re-export `EntityOnce`

```rust
pub use cell::EntityOnce;
```

#### Re-export `Column`

```rust
pub use columns::Column;
```

#### Re-export `Columns`

```rust
pub use columns::Columns;
```

#### Re-export `ColumnsIter`

```rust
pub use columns::ColumnsIter;
```

#### Re-export `FirstColumn`

```rust
pub use columns::FirstColumn;
```

#### Re-export `LastColumn`

```rust
pub use columns::LastColumn;
```

#### Re-export `LastColumnOffset`

```rust
pub use columns::LastColumnOffset;
```

#### Re-export `Frame`

```rust
pub use frame::Frame;
```

#### Re-export `FrameIter`

```rust
pub use frame::FrameIter;
```

#### Re-export `FirstRow`

```rust
pub use rows::FirstRow;
```

#### Re-export `LastRow`

```rust
pub use rows::LastRow;
```

#### Re-export `LastRowOffset`

```rust
pub use rows::LastRowOffset;
```

#### Re-export `Row`

```rust
pub use rows::Row;
```

#### Re-export `Rows`

```rust
pub use rows::Rows;
```

#### Re-export `RowsIter`

```rust
pub use rows::RowsIter;
```

#### Re-export `SectorIter`

```rust
pub use segment::SectorIter;
```

#### Re-export `Segment`

```rust
pub use segment::Segment;
```

#### Re-export `SegmentAll`

```rust
pub use segment::SegmentAll;
```

## Module `disable`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

This module contains a [`Disable`] structure which helps to
remove an etheir column or row from a [`Table`].

# Example

```rust,no_run
# use tabled::{Table, settings::{Disable, object::Rows}};
# let data: Vec<&'static str> = Vec::new();
let table = Table::new(&data).with(Disable::row(Rows::first()));
```

[`Table`]: crate::Table

```rust
pub mod disable { /* ... */ }
```

### Types

#### Struct `Disable`

Disable removes particular rows/columns from a [`Table`].

It tries to keeps track of style changes which may occur.
But it's not guaranteed will be the way you would expect it to be.

Generally you should avoid use of [`Disable`] because it's a slow function and modifies the underlying records.
Providing correct data right away is better.

# Example

```
use tabled::{Table, settings::{Disable, object::Rows}};

let data = vec!["Hello", "World", "!!!"];

let table = Table::new(data).with(Disable::row(Rows::new(1..2))).to_string();

assert_eq!(
    table,
    "+-------+\n\
     | &str  |\n\
     +-------+\n\
     | World |\n\
     +-------+\n\
     | !!!   |\n\
     +-------+"
);

```
[`Table`]: crate::Table

```rust
pub struct Disable<L, Target> {
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
  pub fn column(locator: L) -> Self { /* ... */ }
  ```
  Disable columns.

- ```rust
  pub fn row(locator: L) -> Self { /* ... */ }
  ```
  Disable rows.

###### Trait Implementations

- **RefUnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TableOption**
  - ```rust
    fn change(self: Self, records: &mut R, _: &mut C, _: &mut D) { /* ... */ }
    ```

  - ```rust
    fn change(self: Self, records: &mut R, _: &mut C, _: &mut D) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Freeze**
#### Struct `TargetRow`

A marker struct for [`Disable`].

```rust
pub struct TargetRow;
```

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `TargetColumn`

A marker struct for [`Disable`].

```rust
pub struct TargetColumn;
```

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Module `format`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

This module contains a list of primitives to help to modify a [`Table`].

[`Table`]: crate::Table

```rust
pub mod format { /* ... */ }
```

### Types

#### Struct `Format`

A formatting function of particular cells on a [`Table`].

[`Table`]: crate::Table

```rust
pub struct Format;
```

##### Implementations

###### Methods

- ```rust
  pub fn content<F>(f: F) -> FormatContent<F>
where
    F: FnMut(&str) -> String { /* ... */ }
  ```
  This function creates a new [`Format`] instance, so

- ```rust
  pub fn positioned<F>(f: F) -> FormatContentPositioned<F>
where
    F: FnMut(&str, (usize, usize)) -> String { /* ... */ }
  ```
  This function creates a new [`FormatContentPositioned`], so

- ```rust
  pub fn config<F>(f: F) -> FormatConfig<F> { /* ... */ }
  ```
  This function creates [`FormatConfig`] function to modify a table config.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **RefUnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

### Re-exports

#### Re-export `FormatConfig`

```rust
pub use format_config::FormatConfig;
```

#### Re-export `FormatContent`

```rust
pub use format_content::FormatContent;
```

#### Re-export `FormatContentPositioned`

```rust
pub use format_positioned::FormatContentPositioned;
```

## Module `formatting`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

This module contains settings for render strategy of papergrid.

- [`TrimStrategy`] and [`AlignmentStrategy`] allows to set [`Alignment`] settings.
- [`TabSize`] sets a default tab size.
- [`Charset`] responsible for special char treatment.
- [`Justification`] responsible for justification space of content.

[`Alignment`]: crate::settings::Alignment

```rust
pub mod formatting { /* ... */ }
```

### Re-exports

#### Re-export `AlignmentStrategy`

```rust
pub use alignment_strategy::AlignmentStrategy;
```

#### Re-export `Charset`

```rust
pub use charset::Charset;
```

#### Re-export `CleanCharset`

```rust
pub use charset::CleanCharset;
```

#### Re-export `Justification`

```rust
pub use justification::Justification;
```

#### Re-export `TabSize`

```rust
pub use tab_size::TabSize;
```

#### Re-export `TrimStrategy`

```rust
pub use trim_strategy::TrimStrategy;
```

## Module `height`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

The module contains [`Height`] structure which is responsible for a table and cell height.

```rust
pub mod height { /* ... */ }
```

### Types

#### Struct `Height`

Height is a abstract factory for height settings.

# Example

```
use tabled::{Table, settings::{Height, Settings}};

let data = vec![
    ("Some data", "here", "and here"),
    ("Some data on a next", "line", "right here"),
];

let table = Table::new(data)
    .with(Settings::new(Height::limit(10), Height::increase(10)))
    .to_string();

assert_eq!(
    table,
    "+---------------------+------+------------+\n\
     | &str                | &str | &str       |\n\
     |                     |      |            |\n\
     +---------------------+------+------------+\n\
     | Some data           | here | and here   |\n\
     |                     |      |            |\n\
     +---------------------+------+------------+\n\
     | Some data on a next | line | right here |\n\
     |                     |      |            |\n\
     +---------------------+------+------------+",
)
```

```rust
pub struct Height;
```

##### Implementations

###### Methods

- ```rust
  pub fn increase<W: Measurement<Height>>(height: W) -> CellHeightIncrease<W> { /* ... */ }
  ```
  Create [`CellHeightIncrease`] to set a table/cell height.

- ```rust
  pub fn limit<W: Measurement<Height>>(height: W) -> CellHeightLimit<W> { /* ... */ }
  ```
  Create [`CellHeightLimit`] to set a table/cell height.

- ```rust
  pub fn list<I: IntoIterator<Item = usize>>(rows: I) -> HeightList { /* ... */ }
  ```
  Create [`HeightList`] to set a table height to a constant list of row heights.

###### Trait Implementations

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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **RefUnwindSafe**
- **Measurement**
  - ```rust
    fn measure<R: Records + ExactRecords + PeekableRecords>(self: &Self, records: R, _: &SpannedConfig) -> usize { /* ... */ }
    ```

  - ```rust
    fn measure<R: Records + ExactRecords + PeekableRecords>(self: &Self, records: R, _: &SpannedConfig) -> usize { /* ... */ }
    ```

  - ```rust
    fn measure<R>(self: &Self, records: R, cfg: &SpannedConfig) -> usize
where
    R: Records + ExactRecords,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Re-exports

#### Re-export `CellHeightIncrease`

```rust
pub use cell_height_increase::CellHeightIncrease;
```

#### Re-export `CellHeightLimit`

```rust
pub use cell_height_limit::CellHeightLimit;
```

#### Re-export `HeightList`

```rust
pub use height_list::HeightList;
```

#### Re-export `TableHeightIncrease`

```rust
pub use table_height_increase::TableHeightIncrease;
```

#### Re-export `TableHeightLimit`

```rust
pub use table_height_limit::TableHeightLimit;
```

## Module `highlight`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

This module contains a [`Highlight`] primitive, which helps
changing a [`Border`] style of any segment on a [`Table`].

[`Table`]: crate::Table

```rust
pub mod highlight { /* ... */ }
```

### Types

#### Struct `Highlight`

Highlight modifies a table style by changing a border of a target [`Table`] segment.

# Example

```
use tabled::{
    Table,
    settings::{Highlight, Border, Style, object::Segment}
};

let data = [
    ("ELF", "Extensible Linking Format", true),
    ("DWARF", "", true),
    ("PE", "Portable Executable", false),
];

let table = Table::new(data.iter().enumerate())
               .with(Style::markdown())
               .with(Highlight::border(Segment::all(), Border::new().set_top('^').set_bottom('v')))
               .to_string();

assert_eq!(
    table,
    concat!(
        " ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ \n",
        "| usize | &str  | &str                      | bool  |\n",
        "|-------|-------|---------------------------|-------|\n",
        "| 0     | ELF   | Extensible Linking Format | true  |\n",
        "| 1     | DWARF |                           | true  |\n",
        "| 2     | PE    | Portable Executable       | false |\n",
        " vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv ",
    ),
);
```

It's possible to use [`Highlight`] for many kinds of figures.

```
use tabled::{
    Table,
    settings::{
        Highlight, Border, Style,
        object::{Segment, Object}
    }
};

let data = [
    ("ELF", "Extensible Linking Format", true),
    ("DWARF", "", true),
    ("PE", "Portable Executable", false),
];

let table = Table::new(data.iter().enumerate())
               .with(Style::markdown())
               .with(Highlight::border(Segment::all().not((0,0).and((1, 0)).and((0, 1)).and((0, 3))), Border::filled('*')))
               .to_string();

println!("{}", table);

assert_eq!(
    table,
    concat!(
        "                *****************************        \n",
        "| usize | &str  * &str                      * bool  |\n",
        "|-------*********---------------------------*********\n",
        "| 0     * ELF   | Extensible Linking Format | true  *\n",
        "*********                                           *\n",
        "* 1     | DWARF |                           | true  *\n",
        "*                                                   *\n",
        "* 2     | PE    | Portable Executable       | false *\n",
        "*****************************************************",
    ),
);
```

[`Table`]: crate::Table

```rust
pub struct Highlight<O> {
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
  pub const fn border<T, B, L, R>(target: O, border: Border<T, B, L, R>) -> Self { /* ... */ }
  ```
  Build a new instance of [`Highlight`]

- ```rust
  pub fn color(target: O, border: BorderColor) -> Self { /* ... */ }
  ```
  Build a new instance of [`Highlight`]

- ```rust
  pub const fn outline(target: O, c: char) -> Self { /* ... */ }
  ```
  Build a new instance of [`Highlight`]

- ```rust
  pub fn colored_outline<T, B, L, R>(target: O, c: char, color: Color) -> Self { /* ... */ }
  ```
  Build a new instance of [`Highlight`]

- ```rust
  pub fn colored_border<T, B, L, R>(target: O, border: Border<T, B, L, R>, color: BorderColor) -> Self { /* ... */ }
  ```
  Build a new instance of [`Highlight`]

###### Trait Implementations

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **TableOption**
  - ```rust
    fn change(self: Self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) { /* ... */ }
    ```

  - ```rust
    fn hint_change(self: &Self) -> Option<Entity> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Module `location`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`
- `#[<cfg_attr>(feature = "derive", doc = "```")]`
- `#[<cfg_attr>(not(feature = "derive"), doc = "```ignore")]`

The module contains a [`Location`] trait and implementations for it.

# Example

```
use tabled::{
    settings::{
        location::Locator,
        object::{Columns, Object},
        Alignment, Modify, Padding,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct Reading {
    link: &'static str,
    comment: &'static str,
}

let data = [
    Reading { link: "https://www.gnu.org/software/grub/manual/multiboot/multiboot.html", comment: "todo" },
    Reading { link: "https://wiki.debian.org/initramfs", comment: "todo" },
    Reading { link: "http://jdebp.uk/FGA/efi-boot-process.html", comment: "todo,2" },
    Reading { link: "https://wiki.debian.org/UEFI", comment: "todo,2" },
];

let mut table = Table::new(data);
table.with(Padding::zero());
table.with(Modify::new(Locator::column("link")).with(Alignment::right()));
table.with(Modify::new(Locator::content("todo")).with("todo,1"));
table.with(
    Modify::new(Columns::single(1).intersect(Locator::by(|text| text.contains("todo"))))
        .with(Padding::new(4, 0, 0, 0)),
);

let output = table.to_string();

assert_eq!(
    output,
    concat!(
        "+-----------------------------------------------------------------+----------+\n",
        "|                                                             link|comment   |\n",
        "+-----------------------------------------------------------------+----------+\n",
        "|https://www.gnu.org/software/grub/manual/multiboot/multiboot.html|    todo,1|\n",
        "+-----------------------------------------------------------------+----------+\n",
        "|                                https://wiki.debian.org/initramfs|    todo,1|\n",
        "+-----------------------------------------------------------------+----------+\n",
        "|                        http://jdebp.uk/FGA/efi-boot-process.html|    todo,2|\n",
        "+-----------------------------------------------------------------+----------+\n",
        "|                                     https://wiki.debian.org/UEFI|    todo,2|\n",
        "+-----------------------------------------------------------------+----------+",
    ),
);
```

```rust
pub mod location { /* ... */ }
```

### Traits

#### Trait `Location`

Location is an interface which searches for a particular thing in the [`Records`],
and returns coordinate of the foundings if any.

```rust
pub trait Location<Records> {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Coordinate`: A coordinate of the finding.
- `IntoIter`: An iterator of the coordinates.

###### Required Methods

- `locate`: Search for the thing in [`Records`], returning a list of coordinates.

##### Implementations

This trait is implemented for the following types:

- `ByColumnName<S>` with <R, S>
- `ByCondition<F>` with <F, R>
- `ByContent<S>` with <R, S>
- `crate::settings::object::Columns<B>` with <B, R>
- `crate::settings::object::Column` with <R>
- `crate::settings::object::FirstColumn` with <R>
- `crate::settings::object::LastColumn` with <R>
- `crate::settings::object::Rows<B>` with <B, R>
- `crate::settings::object::Row` with <R>
- `crate::settings::object::FirstRow` with <R>
- `crate::settings::object::LastRow` with <R>

### Re-exports

#### Re-export `ByColumnName`

```rust
pub use by_column_name::ByColumnName;
```

#### Re-export `ByCondition`

```rust
pub use by_condition::ByCondition;
```

#### Re-export `ByContent`

```rust
pub use by_content::ByContent;
```

#### Re-export `Locator`

```rust
pub use locator::Locator;
```

## Module `measurement`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

The module contains [`Measurement`] trait and its implementations to be used in [`Height`] and [`Width`].;

```rust
pub mod measurement { /* ... */ }
```

### Types

#### Struct `Max`

Max width value.

```rust
pub struct Max;
```

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Measurement**
  - ```rust
    fn measure<R: Records + ExactRecords + PeekableRecords>(self: &Self, records: R, _: &SpannedConfig) -> usize { /* ... */ }
    ```

  - ```rust
    fn measure<R: Records + ExactRecords + PeekableRecords>(self: &Self, records: R, _: &SpannedConfig) -> usize { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
#### Struct `Min`

Min width value.

```rust
pub struct Min;
```

##### Implementations

###### Trait Implementations

- **Freeze**
- **Measurement**
  - ```rust
    fn measure<R: Records + ExactRecords + PeekableRecords>(self: &Self, records: R, _: &SpannedConfig) -> usize { /* ... */ }
    ```

  - ```rust
    fn measure<R: Records + ExactRecords + PeekableRecords>(self: &Self, records: R, _: &SpannedConfig) -> usize { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Send**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

#### Struct `Percent`

Percent from a total table width.

```rust
pub struct Percent(pub usize);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Measurement**
  - ```rust
    fn measure<R>(self: &Self, records: R, cfg: &SpannedConfig) -> usize
where
    R: Records,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str> { /* ... */ }
    ```

  - ```rust
    fn measure<R>(self: &Self, records: R, cfg: &SpannedConfig) -> usize
where
    R: Records + ExactRecords,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Traits

#### Trait `Measurement`

A width value which can be obtained on behalf of [`Table`].

[`Table`]: crate::Table

```rust
pub trait Measurement<Attribute> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `measure`: Returns a measurement value.

##### Implementations

This trait is implemented for the following types:

- `usize` with <T>
- `Max`
- `Max`
- `Min`
- `Min`
- `Percent`
- `Percent`

## Module `merge`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

The module contains a set of methods to merge cells together via [`Span`]s.

[`Span`]: crate::settings::span::Span

```rust
pub mod merge { /* ... */ }
```

### Types

#### Struct `Merge`

Merge to combine duplicates together, using [`Span`].

[`Span`]: crate::settings::span::Span

```rust
pub struct Merge;
```

##### Implementations

###### Methods

- ```rust
  pub fn vertical() -> MergeDuplicatesVertical { /* ... */ }
  ```
  Vertical merge.

- ```rust
  pub fn horizontal() -> MergeDuplicatesHorizontal { /* ... */ }
  ```
  Horizontal merge.

###### Trait Implementations

- **Unpin**
- **Sync**
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

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
#### Struct `MergeDuplicatesVertical`

A modificator for [`Table`] which looks up for duplicates in columns and
in case of duplicate merges the cells together using [`Span`].

[`Table`]: crate::Table
[`Span`]: crate::settings::span::Span

```rust
pub struct MergeDuplicatesVertical;
```

##### Implementations

###### Trait Implementations

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

- **TableOption**
  - ```rust
    fn change(self: Self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `MergeDuplicatesHorizontal`

A modificator for [`Table`] which looks up for duplicates in rows and
in case of duplicate merges the cells together using [`Span`].

[`Table`]: crate::Table
[`Span`]: crate::settings::span::Span

```rust
pub struct MergeDuplicatesHorizontal;
```

##### Implementations

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TableOption**
  - ```rust
    fn change(self: Self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `padding_expand`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

This module contains a [`PaddingExpand`] setting to expand cells padding to its limit a cell.

```rust
pub mod padding_expand { /* ... */ }
```

### Types

#### Enum `PaddingExpand`

**Attributes:**

- `#[<cfg_attr>(feature = "ansi", doc = "```")]`
- `#[<cfg_attr>(not(feature = "ansi"), doc = "```ignore")]`

PaddingExpand is able to expand padding to its limit a cell.
Maybe usefull in cases were its colorization is supposed to be used next.

# Example

```ignore
use std::iter::FromIterator;

use tabled::{
    settings::{Padding, Style, Color, PaddingColor},
    Table,
};

let char_split = |s: &str| s.chars().map(|c| c.to_string()).collect::<Vec<_>>();
let data = vec![
    char_split("2021"),
    char_split("2022"),
    char_split("2023"),
    char_split("2024"),
];

let table = Table::from_iter(&data)
    .with(Style::ascii())
    .with(PaddingColor::filled(Color::BG_BLUE))
    .modify((2, 1), Padding::new(2, 2, 3, 3))
    .with(Padding::expand(true))
    .with(Padding::expand(false))
    .to_string();

assert_eq!(
    table,
    concat!(
        "+---+-----+---+---+\n",
        "|2\u{1b}[44m  \u{1b}[49m|0\u{1b}[44m    \u{1b}[49m|2\u{1b}[44m  \u{1b}[49m|1\u{1b}[44m  \u{1b}[49m|\n",
        "+---+-----+---+---+\n",
        "|2\u{1b}[44m  \u{1b}[49m|0\u{1b}[44m    \u{1b}[49m|2\u{1b}[44m  \u{1b}[49m|2\u{1b}[44m  \u{1b}[49m|\n",
        "+---+-----+---+---+\n",
        "|2\u{1b}[44m  \u{1b}[49m|0\u{1b}[44m    \u{1b}[49m|2\u{1b}[44m  \u{1b}[49m|3\u{1b}[44m  \u{1b}[49m|\n",
        "|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m     \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\n",
        "|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m     \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\n",
        "|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m     \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\n",
        "|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m     \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\n",
        "|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m     \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\n",
        "|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m     \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\u{1b}[44m   \u{1b}[49m|\n",
        "+---+-----+---+---+\n",
        "|2\u{1b}[44m  \u{1b}[49m|0\u{1b}[44m    \u{1b}[49m|2\u{1b}[44m  \u{1b}[49m|4\u{1b}[44m  \u{1b}[49m|\n",
        "+---+-----+---+---+",
    ),
);
```

```rust
pub enum PaddingExpand {
    Horizontal,
    Vertical,
}
```

##### Variants

###### `Horizontal`

Horizontal expansion of padding (LEFT and RIGHT)

###### `Vertical`

Vertical expansion of padding (TOP and BOTTOM)

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PaddingExpand { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &PaddingExpand) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Eq**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PaddingExpand) -> bool { /* ... */ }
    ```

- **TableOption**
  - ```rust
    fn change(self: Self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) { /* ... */ }
    ```

- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &PaddingExpand) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **CellOption**
  - ```rust
    fn change(self: Self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) { /* ... */ }
    ```

- **Freeze**
## Module `panel`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

This module contains primitives to create a spread row.
Ultimately it is a cell with a span set to a number of columns on the [`Table`].

You can use a [`Span`] to set a custom span.

# Example

```
use tabled::{Table, settings::Panel};

let data = [[1, 2, 3], [4, 5, 6]];

let table = Table::new(data)
    .with(Panel::vertical(1, "S\np\nl\ni\nt"))
    .with(Panel::header("Numbers"))
    .to_string();

println!("{}", table);

assert_eq!(
    table,
    concat!(
        "+---+---+---+---+\n",
        "| Numbers       |\n",
        "+---+---+---+---+\n",
        "| 0 | S | 1 | 2 |\n",
        "+---+ p +---+---+\n",
        "| 1 | l | 2 | 3 |\n",
        "+---+ i +---+---+\n",
        "| 4 | t | 5 | 6 |\n",
        "+---+---+---+---+",
    )
)
```

[`Table`]: crate::Table
[`Span`]: crate::settings::span::Span

```rust
pub mod panel { /* ... */ }
```

### Types

#### Struct `Panel`

Panel allows to add a Row which has 1 continues Cell to a [`Table`].

See `examples/panel.rs`.

[`Table`]: crate::Table

```rust
pub struct Panel;
```

##### Implementations

###### Methods

- ```rust
  pub fn vertical<S: AsRef<str>>(column: usize, text: S) -> VerticalPanel<S> { /* ... */ }
  ```
  Creates an empty vertical row at given index.

- ```rust
  pub fn horizontal<S: AsRef<str>>(row: usize, text: S) -> HorizontalPanel<S> { /* ... */ }
  ```
  Creates an empty horizontal row at given index.

- ```rust
  pub fn header<S: AsRef<str>>(text: S) -> Header<S> { /* ... */ }
  ```
  Creates an horizontal row at first row.

- ```rust
  pub fn footer<S: AsRef<str>>(text: S) -> Footer<S> { /* ... */ }
  ```
  Creates an horizontal row at last row.

###### Trait Implementations

- **Sync**
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

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
### Re-exports

#### Re-export `Footer`

```rust
pub use footer::Footer;
```

#### Re-export `Header`

```rust
pub use header::Header;
```

#### Re-export `HorizontalPanel`

```rust
pub use horizontal_panel::HorizontalPanel;
```

#### Re-export `VerticalPanel`

```rust
pub use vertical_panel::VerticalPanel;
```

## Module `peaker`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

The module contains [`Peaker`] trait and its implementations to be used in [`Height`] and [`Width`].

[`Width`]: crate::settings::width::Width
[`Height`]: crate::settings::height::Height

```rust
pub mod peaker { /* ... */ }
```

### Traits

#### Trait `Peaker`

A strategy of width function.
It determines the order how the function is applied.

```rust
pub trait Peaker {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `peak`: This function returns a column index which will be changed.

##### Implementations

This trait is implemented for the following types:

- `PriorityLeft`
- `PriorityMax`
- `PriorityMin`
- `PriorityNone`
- `PriorityRight`

### Re-exports

#### Re-export `PriorityLeft`

```rust
pub use left::PriorityLeft;
```

#### Re-export `PriorityMax`

```rust
pub use max::PriorityMax;
```

#### Re-export `PriorityMin`

```rust
pub use min::PriorityMin;
```

#### Re-export `PriorityNone`

```rust
pub use none::PriorityNone;
```

#### Re-export `PriorityRight`

```rust
pub use right::PriorityRight;
```

## Module `span`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

This module contains a [`Span`] settings, it helps to
make a cell take more space then it generally takes.

# Example

```
use tabled::{settings::{Span, Modify}, Table};

let data = [[1, 2, 3], [4, 5, 6]];

let table = Table::new(data)
    .with(Modify::new((2, 0)).with(Span::column(2)))
    .with(Modify::new((0, 1)).with(Span::column(2)))
    .to_string();

assert_eq!(
    table,
    concat!(
        "+---+---+---+\n",
        "| 0 | 1     |\n",
        "+---+---+---+\n",
        "| 1 | 2 | 3 |\n",
        "+---+---+---+\n",
        "| 4     | 6 |\n",
        "+---+---+---+",
    )
)
```

```rust
pub mod span { /* ... */ }
```

### Types

#### Struct `Span`

Span represent a horizontal/column span setting for any cell on a [`Table`].

It will be ignored if:
 - cell position is out of scope
 - size is bigger then the total number of columns.
 - size is bigger then the total number of rows.

```rust,no_run
# use tabled::{Table, settings::{Style, Span, Modify, object::Columns}};
# let data: Vec<&'static str> = Vec::new();
let table = Table::new(&data)
    .with(Modify::new(Columns::single(0)).with(Span::column(2)));
```

[`Table`]: crate::Table

```rust
pub struct Span;
```

##### Implementations

###### Methods

- ```rust
  pub fn column(size: usize) -> ColumnSpan { /* ... */ }
  ```
  New constructs a horizontal/column [`Span`].

- ```rust
  pub fn row(size: usize) -> RowSpan { /* ... */ }
  ```
  New constructs a vertical/row [`Span`].

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
### Re-exports

#### Re-export `ColumnSpan`

```rust
pub use column::ColumnSpan;
```

#### Re-export `RowSpan`

```rust
pub use row::RowSpan;
```

## Module `split`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

This module contains a [`Split`] setting which is used to
format the cells of a [`Table`] by a provided index, direction, behavior, and display preference.

[`Table`]: crate::Table

```rust
pub mod split { /* ... */ }
```

### Types

#### Struct `Split`

Returns a new [`Table`] formatted with several optional parameters.

The required index parameter determines how many columns/rows a table will be redistributed into.

- index
- direction
- behavior
- display

#### Directions

Direction functions are the entry point for the `Split` setting.

There are two directions available: `column` and `row`.

```rust
use std::iter::FromIterator;
use tabled::{Table, settings::split::Split};

let mut table = Table::from_iter(['a'..='z']);
table.with(Split::column(12));
table.with(Split::row(2));
```

```text
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ a │ b │ c │ d │ e │ f │ g │ h │ i │ j │ k │ l │ m │ n │ o │ p │ q │ r │ s │ t │ u │ v │ w │ x │ y │ z │
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ a │ b │ c │ d │ e │ f │ g │ h │ i │ j │ k │ l │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│ m │ n │ o │ p │ q │ r │ s │ t │ u │ v │ w │ x │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│ y │ z │   │   │   │   │   │   │   │   │   │   │<- y and z act as anchors to new empty cells
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘   to conform to the new shape
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ a │ y │ b │ z │ c │ d │ e │ f │ g │ h │ i │ j │ k │ l │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤<- Display::Clean removes empty cells that would be anchors otherwise
│ m │   │ n │   │ o │ p │ q │ r │ s │ t │ u │ v │ w │ x │
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
      ^anchors^
```


#### Behaviors

Behaviors determine how cells attempt to conform to the new tables shape.

There are two behaviors available: `zip` and `concat`.

`zip` is the default behavior.

```rust
use std::iter::FromIterator;
use tabled::{Table, settings::split::Split};

let mut table = Table::from_iter(['a'..='z']);
table.with(Split::column(2).concat());
table.with(Split::column(2).zip());
```

```text
                                                +---+---+
                                                | a | b |
                                                +---+---+
+---+---+---+---+---+                           | f | g |
| a | b | c | d | e | Split::column(2).concat() +---+---+
+---+---+---+---+---+           =>              | c | d |
| f | g | h | i | j |                           +---+---+
+---+---+---+---+---+                           | h | i |
                                                +---+---+
                                                | e |   |
                                                +---+---+
                                                | j |   |
                                                +---+---+

                  sect 3                        +---+---+
 sect 1   sect 2 (anchors)                      | a | b |
  /   \   /   \   /   \                         +---+---+
+---+---+---+---+---+                           | c | d |
| a | b | c | d | e |  Split::column(2).zip()   +---+---+
+---+---+---+---+---+           =>              | e |   |
| f | g | h | i | j |                           +---+---+
+---+---+---+---+---+                           | f | g |
                                                +---+---+
                                                | h | i |
                                                +---+---+
                                                | j |   |
                                                +---+---+
```

#### Displays

Display functions give the user the choice to `retain` or `clean` empty sections in a `Split` table result.

- `retain` does not filter any existing or newly added cells when conforming to a new shape.

- `clean` filters out empty columns/rows from the output and prevents empty cells from acting as anchors to newly inserted cells.

`clean` is the default `Display`.

```rust
use std::iter::FromIterator;
use tabled::{
    settings::{split::Split, style::Style},
    Table,
};
let mut table = Table::from_iter(['a'..='z']);
table.with(Split::column(25)).with(Style::modern());
table.clone().with(Split::column(1).concat().retain());
table.clone().with(Split::column(1).concat()); // .clean() is not necessary as it is the default display property
```

```text
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ a │ b │ c │ d │ e │ f │ g │ h │ i │ j │ k │ l │ m │ n │ o │ p │ q │ r │ s │ t │ u │ v │ w │ x │ y │
├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
│ z │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │<- lots of extra cells generated
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┐
│ a │ b │ c │ d │ e │ f │ g │ h │ i │ j │ k │ l │ m │ n │ o │ p │ q │ r │ s │ t │ u │ v │ w │ x │ y │ z │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │  │
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┘
┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐ ^ cells retained during concatenation
│ a │ b │ c │ d │ e │ f │ g │ h │ i │ j │ k │ l │ m │ n │ o │ p │ q │ r │ s │ t │ u │ v │ w │ x │ y │ z │
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘<- cells removed during concatenation
```


# Example

```rust
use std::iter::FromIterator;
use tabled::{
    settings::split::Split,
    Table,
};

let mut table = Table::from_iter(['a'..='z']);
let table = table.with(Split::column(4)).to_string();

assert_eq!(table, "+---+---+---+---+\n\
                   | a | b | c | d |\n\
                   +---+---+---+---+\n\
                   | e | f | g | h |\n\
                   +---+---+---+---+\n\
                   | i | j | k | l |\n\
                   +---+---+---+---+\n\
                   | m | n | o | p |\n\
                   +---+---+---+---+\n\
                   | q | r | s | t |\n\
                   +---+---+---+---+\n\
                   | u | v | w | x |\n\
                   +---+---+---+---+\n\
                   | y | z |   |   |\n\
                   +---+---+---+---+")
```

[`Table`]: crate::Table

```rust
pub struct Split {
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
  pub fn column(index: usize) -> Self { /* ... */ }
  ```
  Returns a new [`Table`] split on the column at the provided index.

- ```rust
  pub fn row(index: usize) -> Self { /* ... */ }
  ```
  Returns a new [`Table`] split on the row at the provided index.

- ```rust
  pub fn concat(self: Self) -> Self { /* ... */ }
  ```
  Returns a split [`Table`] with the redistributed cells pushed to the back of the new shape.

- ```rust
  pub fn zip(self: Self) -> Self { /* ... */ }
  ```
  Returns a split [`Table`] with the redistributed cells inserted behind

- ```rust
  pub fn clean(self: Self) -> Self { /* ... */ }
  ```
  Returns a split [`Table`] with the empty columns/rows filtered out.

- ```rust
  pub fn retain(self: Self) -> Self { /* ... */ }
  ```
  Returns a split [`Table`] with all cells retained.

###### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Split { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TableOption**
  - ```rust
    fn change(self: Self, records: &mut R, _: &mut C, _: &mut D) { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `themes`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

The module contains a variety of configurations of table, which often
changes not a single setting.
As such they are making relatively big changes to the configuration.

```rust
pub mod themes { /* ... */ }
```

### Re-exports

#### Re-export `Colorization`

```rust
pub use colorization::Colorization;
```

#### Re-export `ExactColorization`

```rust
pub use colorization::ExactColorization;
```

#### Re-export `ColumnNames`

```rust
pub use column_names::ColumnNames;
```

#### Re-export `Layout`

```rust
pub use layout::Layout;
```

#### Re-export `Theme`

```rust
pub use theme::Theme;
```

## Module `width`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

This module contains object which can be used to limit a cell to a given width:

- [`Truncate`] cuts a cell content to limit width.
- [`Wrap`] split the content via new lines in order to fit max width.
- [`Justify`] sets columns width to the same value.

To set a a table width, a combination of [`Width::truncate`] or [`Width::wrap`] and [`Width::increase`] can be used.

## Example

```
use tabled::{Table, settings::Width};

let table = Table::new(&["Hello World!"])
    .with(Width::wrap(7))
    .with(Width::increase(7))
    .to_string();

assert_eq!(
    table,
    concat!(
        "+-----+\n",
        "| &st |\n",
        "| r   |\n",
        "+-----+\n",
        "| Hel |\n",
        "| lo  |\n",
        "| Wor |\n",
        "| ld! |\n",
        "+-----+",
    )
);
```

```rust
pub mod width { /* ... */ }
```

### Types

#### Struct `Width`

Width allows you to set a min and max width of an object on a [`Table`]
using different strategies.

It also allows you to set a min and max width for a whole table.

You can apply a min and max strategy at the same time with the same value,
the value will be a total table width.

It is an abstract factory.

Beware that borders are not removed when you set a size value to very small.
For example if you set size to 0 the table still be rendered but with all content removed.

Also be aware that it doesn't changes [`Padding`] settings nor it considers them.

The function is color aware if a `color` feature is on.

## Examples

### Cell change

```
use tabled::{Table, settings::{object::Segment, Width, Style, Modify}};

let data = ["Hello", "World", "!"];

let table = Table::new(&data)
    .with(Style::markdown())
    .with(Modify::new(Segment::all()).with(Width::truncate(3).suffix("...")));
```

### Table change

```
use tabled::{Table, settings::Width};

let table = Table::new(&["Hello World!"]).with(Width::wrap(5));
```

### Total width

```
use tabled::{Table, settings::Width};

let table = Table::new(&["Hello World!"])
    .with(Width::wrap(5))
    .with(Width::increase(5));
```

[`Padding`]: crate::settings::Padding
[`Table`]: crate::Table

```rust
pub struct Width;
```

##### Implementations

###### Methods

- ```rust
  pub fn wrap<W: Measurement<Width>>(width: W) -> Wrap<W> { /* ... */ }
  ```
  Returns a [`Wrap`] structure.

- ```rust
  pub fn truncate<W: Measurement<Width>>(width: W) -> Truncate<''static, W> { /* ... */ }
  ```
  Returns a [`Truncate`] structure.

- ```rust
  pub fn increase<W: Measurement<Width>>(width: W) -> MinWidth<W> { /* ... */ }
  ```
  Returns a [`MinWidth`] structure.

- ```rust
  pub fn justify<W: Measurement<Width>>(width: W) -> Justify<W> { /* ... */ }
  ```
  Returns a [`Justify`] structure.

- ```rust
  pub fn list<I: IntoIterator<Item = usize>>(rows: I) -> WidthList { /* ... */ }
  ```
  Create [`WidthList`] to set a table width to a constant list of column widths.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Measurement**
  - ```rust
    fn measure<R: Records + ExactRecords + PeekableRecords>(self: &Self, records: R, _: &SpannedConfig) -> usize { /* ... */ }
    ```

  - ```rust
    fn measure<R: Records + ExactRecords + PeekableRecords>(self: &Self, records: R, _: &SpannedConfig) -> usize { /* ... */ }
    ```

  - ```rust
    fn measure<R>(self: &Self, records: R, cfg: &SpannedConfig) -> usize
where
    R: Records,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
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

### Re-exports

#### Re-export `Justify`

```rust
pub use self::justify::Justify;
```

#### Re-export `MinWidth`

```rust
pub use self::min_width::MinWidth;
```

#### Re-export `SuffixLimit`

```rust
pub use self::truncate::SuffixLimit;
```

#### Re-export `Truncate`

```rust
pub use self::truncate::Truncate;
```

#### Re-export `WidthList`

```rust
pub use self::width_list::WidthList;
```

#### Re-export `Wrap`

```rust
pub use self::wrap::Wrap;
```

### Re-exports

#### Re-export `CellOption`

```rust
pub use cell_option::CellOption;
```

#### Re-export `EmptySettings`

```rust
pub use settings_list::EmptySettings;
```

#### Re-export `Settings`

```rust
pub use settings_list::Settings;
```

#### Re-export `TableOption`

```rust
pub use table_option::TableOption;
```

#### Re-export `Alignment`

```rust
pub use self::alignment::Alignment;
```

#### Re-export `Extract`

```rust
pub use self::extract::Extract;
```

#### Re-export `Margin`

```rust
pub use self::margin::Margin;
```

#### Re-export `MarginColor`

```rust
pub use self::margin_color::MarginColor;
```

#### Re-export `Padding`

```rust
pub use self::padding::Padding;
```

#### Re-export `PaddingColor`

```rust
pub use self::padding_color::PaddingColor;
```

#### Re-export `Reverse`

```rust
pub use self::reverse::Reverse;
```

#### Re-export `Rotate`

```rust
pub use self::rotate::Rotate;
```

#### Re-export `Border`

```rust
pub use self::style::Border;
```

#### Re-export `Style`

```rust
pub use self::style::Style;
```

#### Re-export `Color`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::color::Color;
```

#### Re-export `Concat`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::concat::Concat;
```

#### Re-export `Disable`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::disable::Disable;
```

#### Re-export `Dup`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::duplicate::Dup;
```

#### Re-export `Format`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::format::Format;
```

#### Re-export `Height`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::height::Height;
```

#### Re-export `Highlight`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::highlight::Highlight;
```

#### Re-export `Merge`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::merge::Merge;
```

#### Re-export `Modify`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::modify::Modify;
```

#### Re-export `ModifyList`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::modify::ModifyList;
```

#### Re-export `Panel`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::panel::Panel;
```

#### Re-export `Shadow`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::shadow::Shadow;
```

#### Re-export `Span`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::span::Span;
```

#### Re-export `Theme`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::themes::Theme;
```

#### Re-export `Width`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::width::Width;
```

## Module `tables`

Module contains a list of table representatives.

## [`Table`]

A default table implementation.

## [`IterTable`]

Just like [`Table`] but it's API is a bit different to serve better in context
where there is a memory limit.

## [`ExtendedTable`]

It's a table which is useful for large amount of data.

## [`PoolTable`]

A table with a greater control of a layout.

```rust
pub mod tables { /* ... */ }
```

### Re-exports

#### Re-export `Table`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use table::Table;
```

#### Re-export `IterTable`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use iter::IterTable;
```

#### Re-export `ExtendedTable`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use extended::ExtendedTable;
```

#### Re-export `PoolTable`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use table_pool::PoolTable;
```

#### Re-export `TableValue`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use table_pool::TableValue;
```

#### Re-export `CompactTable`

```rust
pub use compact::CompactTable;
```

## Module `macros`

**Attributes:**

- `#[<cfg>(feature = "macros")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "macros")))]`

This module contains macro functions for dynamic [`Table`] construction.

[`Table`]: crate::Table

```rust
pub mod macros { /* ... */ }
```

## Module `grid`

Module is responsible for tables underlyign grid.

It might be used when implementing your own [`TableOption`] and [`CellOption`].

[`TableOption`]: crate::settings::TableOption
[`CellOption`]: crate::settings::CellOption

```rust
pub mod grid { /* ... */ }
```

### Modules

## Module `dimension`

Module contains a list of implementations of [`Estimate`] and [`Dimension`].

```rust
pub mod dimension { /* ... */ }
```

### Re-exports

#### Re-export `CompleteDimension`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::complete_dimension::CompleteDimension;
```

#### Re-export `CompleteDimensionVecRecords`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::complete_dimension_vec_records::CompleteDimensionVecRecords;
```

#### Re-export `PeekableDimension`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::peekable_dimension::PeekableDimension;
```

#### Re-export `DimensionValue`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::static_dimension::DimensionValue;
```

#### Re-export `StaticDimension`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use self::static_dimension::StaticDimension;
```

#### Re-export `ConstDimension`

```rust
pub use const_dimension::ConstDimension;
```

#### Re-export `ConstSize`

```rust
pub use const_dimension::ConstSize;
```

#### Re-export `Dimension`

```rust
pub use papergrid::dimension::Dimension;
```

#### Re-export `Estimate`

```rust
pub use papergrid::dimension::Estimate;
```

#### Re-export `DimensionPriority`

```rust
pub use pool_table_dimension::DimensionPriority;
```

#### Re-export `PoolTableDimension`

```rust
pub use pool_table_dimension::PoolTableDimension;
```

#### Re-export `CompactGridDimension`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use papergrid::dimension::compact::CompactGridDimension;
```

#### Re-export `SpannedGridDimension`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use papergrid::dimension::spanned::SpannedGridDimension;
```

#### Re-export `SpannedVecRecordsDimension`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use papergrid::dimension::spanned_vec_records::SpannedVecRecordsDimension;
```

## Module `records`

The module contains [`Records`], [`ExactRecords`], [`RecordsMut`], [`Resizable`] traits
and its implementations.

Also it provides a list of helpers for a user built [`Records`] via [`into_records`].

```rust
pub mod records { /* ... */ }
```

### Modules

## Module `into_records`

The module contains a list of helpers for [`IntoRecords`]

[`IntoRecords`]: crate::grid::records::IntoRecords

```rust
pub mod into_records { /* ... */ }
```

### Modules

## Module `limit_column_records`

The module contains [`LimitColumns`] records iterator.

```rust
pub mod limit_column_records { /* ... */ }
```

### Types

#### Struct `LimitColumns`

An iterator which limits amount of columns.

```rust
pub struct LimitColumns<I> {
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
  pub fn new<I>(records: I, limit: usize) -> LimitColumns<I> { /* ... */ }
  ```
  Creates new [`LimitColumns`].

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **IntoRecords**
  - ```rust
    fn iter_rows(self: Self) -> <Self as >::IterRows { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `LimitColumnsIter`

An iterator over rows for [`LimitColumns`].

```rust
pub struct LimitColumnsIter<I> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **IntoRecords**
  - ```rust
    fn iter_rows(self: Self) -> <T as IntoRecords>::IterRows { /* ... */ }
    ```

#### Struct `LimitColumnsColumnsIter`

An iterator over columns for [`LimitColumns`].

```rust
pub struct LimitColumnsColumnsIter<I> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **IntoRecords**
  - ```rust
    fn iter_rows(self: Self) -> <T as IntoRecords>::IterRows { /* ... */ }
    ```

- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Unpin**
- **Send**
## Module `limit_row_records`

The module contains [`LimitRows`] records iterator.

```rust
pub mod limit_row_records { /* ... */ }
```

### Types

#### Struct `LimitRows`

[`LimitRows`] is an records iterator which limits amount of rows.

```rust
pub struct LimitRows<I> {
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
  pub fn new<I: IntoRecords>(records: I, limit: usize) -> LimitRows<I> { /* ... */ }
  ```
  Creates new [`LimitRows`] iterator.

###### Trait Implementations

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **IntoRecords**
  - ```rust
    fn iter_rows(self: Self) -> <Self as >::IterRows { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Send**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `LimitRowsIter`

A rows iterator for [`LimitRows`]

```rust
pub struct LimitRowsIter<I> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **IntoRecords**
  - ```rust
    fn iter_rows(self: Self) -> <T as IntoRecords>::IterRows { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
## Module `buf_records`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

A module contains [`BufRecords`] iterator.

```rust
pub mod buf_records { /* ... */ }
```

### Types

#### Struct `BufRecords`

BufRecords inspects [`IntoRecords`] iterator and keeps read data buffered.
So it can be checking before hand.

```rust
pub struct BufRecords<I, T> {
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
  pub fn new<I>(records: I, sniff: usize) -> BufRecords<<<I as >::IterRows as IntoIterator>::IntoIter, <I as >::Cell>
where
    I: IntoRecords { /* ... */ }
  ```
  Creates new [`BufRecords`] structure, filling the buffer.

- ```rust
  pub fn as_slice(self: &Self) -> &[Vec<T>] { /* ... */ }
  ```
  Returns a slice of a keeping buffer.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **IntoRecords**
  - ```rust
    fn iter_rows(self: Self) -> <Self as >::IterRows { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `BufRecordsIter`

A row iterator for [`BufRecords`]

```rust
pub struct BufRecordsIter<I, T> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **IntoRecords**
  - ```rust
    fn iter_rows(self: Self) -> <T as IntoRecords>::IterRows { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Enum `BufIterator`

An iterator over some iterator or allocated buffer.

```rust
pub enum BufIterator<I, T> {
    Buffered(std::vec::IntoIter<T>),
    Iterator(I),
}
```

##### Variants

###### `Buffered`

Allocated iterator.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::vec::IntoIter<T>` |  |

###### `Iterator`

Given iterator.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `I` |  |

##### Implementations

###### Trait Implementations

- **Sync**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Unpin**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **IntoRecords**
  - ```rust
    fn iter_rows(self: Self) -> <T as IntoRecords>::IterRows { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `either_string`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

A module with a utility enum [`EitherString`].

```rust
pub mod either_string { /* ... */ }
```

### Types

#### Enum `EitherString`

Either allocated string or some type which can be used as a string.

```rust
pub enum EitherString<T> {
    Owned(String),
    Some(T),
}
```

##### Variants

###### `Owned`

Allocated string.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Some`

Something which can be used as a string.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `truncate_records`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

The module contains [`TruncateContent`] records iterator.

```rust
pub mod truncate_records { /* ... */ }
```

### Types

#### Struct `TruncateContent`

A records iterator which truncates all cells to a given width.

```rust
pub struct TruncateContent<I, D> {
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
  pub fn new<I, D>(records: I, dimension: D) -> TruncateContent<I, D> { /* ... */ }
  ```
  Creates new [`TruncateContent`] object.

###### Trait Implementations

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoRecords**
  - ```rust
    fn iter_rows(self: Self) -> <Self as >::IterRows { /* ... */ }
    ```

#### Struct `TruncateContentIter`

A row iterator for [`TruncateContent`].

```rust
pub struct TruncateContentIter<I, D> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoRecords**
  - ```rust
    fn iter_rows(self: Self) -> <T as IntoRecords>::IterRows { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
#### Struct `TruncateContentColumnsIter`

A column iterator for [`TruncateContent`].

```rust
pub struct TruncateContentColumnsIter<I, D> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Send**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Sync**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Re-exports

#### Re-export `BufRecords`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use buf_records::BufRecords;
```

#### Re-export `TruncateContent`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use truncate_records::TruncateContent;
```

#### Re-export `LimitColumns`

```rust
pub use limit_column_records::LimitColumns;
```

#### Re-export `LimitRows`

```rust
pub use limit_row_records::LimitRows;
```

### Re-exports

#### Re-export `EmptyRecords`

```rust
pub use empty_records::EmptyRecords;
```

#### Re-export `ExactRecords`

```rust
pub use papergrid::records::ExactRecords;
```

#### Re-export `IntoRecords`

```rust
pub use papergrid::records::IntoRecords;
```

#### Re-export `IterRecords`

```rust
pub use papergrid::records::IterRecords;
```

#### Re-export `PeekableRecords`

```rust
pub use papergrid::records::PeekableRecords;
```

#### Re-export `Records`

```rust
pub use papergrid::records::Records;
```

#### Re-export `RecordsMut`

```rust
pub use records_mut::RecordsMut;
```

#### Re-export `Resizable`

```rust
pub use resizable::Resizable;
```

#### Re-export `vec_records`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use papergrid::records::vec_records;
```

## Module `config`

Module contains a list of configs for varios tables/grids.

```rust
pub mod config { /* ... */ }
```

### Re-exports

#### Re-export `CompactConfig`

```rust
pub use papergrid::config::compact::CompactConfig;
```

#### Re-export `AlignmentHorizontal`

```rust
pub use papergrid::config::AlignmentHorizontal;
```

#### Re-export `AlignmentVertical`

```rust
pub use papergrid::config::AlignmentVertical;
```

#### Re-export `Border`

```rust
pub use papergrid::config::Border;
```

#### Re-export `Borders`

```rust
pub use papergrid::config::Borders;
```

#### Re-export `Entity`

```rust
pub use papergrid::config::Entity;
```

#### Re-export `EntityIterator`

```rust
pub use papergrid::config::EntityIterator;
```

#### Re-export `Formatting`

```rust
pub use papergrid::config::Formatting;
```

#### Re-export `HorizontalLine`

```rust
pub use papergrid::config::HorizontalLine;
```

#### Re-export `Indent`

```rust
pub use papergrid::config::Indent;
```

#### Re-export `Position`

```rust
pub use papergrid::config::Position;
```

#### Re-export `Sides`

```rust
pub use papergrid::config::Sides;
```

#### Re-export `VerticalLine`

```rust
pub use papergrid::config::VerticalLine;
```

#### Re-export `EntityMap`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use papergrid::config::spanned::EntityMap;
```

#### Re-export `Offset`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use papergrid::config::spanned::Offset;
```

#### Re-export `SpannedConfig`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use papergrid::config::spanned::SpannedConfig;
```

#### Re-export `ColorMap`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use super::colored_config::ColorMap;
```

#### Re-export `ColoredConfig`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use super::colored_config::ColoredConfig;
```

#### Re-export `CompactMultilineConfig`

```rust
pub use super::compact_multiline_config::CompactMultilineConfig;
```

### Re-exports

#### Re-export `ansi`

```rust
pub use papergrid::ansi;
```

#### Re-export `colors`

```rust
pub use papergrid::colors;
```

#### Re-export `util`

```rust
pub use papergrid::util;
```

#### Re-export `CompactGrid`

```rust
pub use papergrid::grid::compact::CompactGrid;
```

#### Re-export `Grid`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use papergrid::grid::iterable::Grid;
```

#### Re-export `PeekableGrid`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use papergrid::grid::peekable::PeekableGrid;
```

## Macros

### Macro `col`

**Attributes:**

- `#[macro_export]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "macros")))]`

Creates a [`Table`] with [`Display`] arguments nested within.

The macros allows several tables to be displayed vertically.

Companion to [`row!`].

# Examples
```rust,no_run
# use tabled::{row, col, Table};
# let (table1, table2, table3) = (Table::new(&[String::new()]), Table::new(&[String::new()]), Table::new(&[String::new()]));
let new_table = col![table1, table2];
let new_table_of_clones = col![table1; 3];
let columns_and_rows = col![
    table1,
    row![table2, table3]
];
```

[`row!`]: crate::row
[`Table`]: crate::Table
[`Display`]: std::fmt::Display

```rust
pub macro_rules! col {
    /* macro_rules! col {
    ( $($table:expr), * $(,)? ) => { ... };
    ( $table:expr; $N:expr) => { ... };
} */
}
```

### Macro `row`

**Attributes:**

- `#[macro_export]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "macros")))]`

Creates a [`Table`] with [`Display`] arguments nested within.

The macros allows several tables to be displayed horizontally.

Companion to [`col!`].

# Examples
```rust,no_run
# use tabled::{row, col, Table};
# let (table1, table2, table3) = (Table::new(&[String::new()]), Table::new(&[String::new()]), Table::new(&[String::new()]));
let new_table = row![table1, table2];
let new_table_of_clones = row![table1; 3];
let rows_and_columns = row![
    table1,
    col![table2, table3]
];
```

[`col!`]: crate::col
[`Table`]: crate::Table
[`Display`]: std::fmt::Display

```rust
pub macro_rules! row {
    /* macro_rules! row {
    ( $($table:expr), * $(,)? ) => { ... };
    ( $table:expr; $N:expr) => { ... };
} */
}
```

## Re-exports

### Re-export `Tabled`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use crate::tabled::Tabled;
```

### Re-export `Table`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use crate::tables::Table;
```

### Re-export `Tabled`

**Attributes:**

- `#[<cfg>(feature = "derive")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "derive")))]`

A derive macro to implement a [`Tabled`] trait.

The macro available only when `derive` feature in turned on (and it is by default).

To be able to use the derive each field must implement `std::fmt::Display`.
The following example will cause an error because of that.

```rust,compile_fail
use tabled::Tabled;
#[derive(Tabled)]
struct SomeType {
    field1: SomeOtherType,
}

struct SomeOtherType;
```

Bellow you'll find available options for it.

### Override a column name

You can use a `#[tabled(rename = "")]` attribute to override a column name.

```rust,no_run
use tabled::Tabled;

#[derive(Tabled)]
struct Person {
    #[tabled(rename = "Name")]
    first_name: String,
    #[tabled(rename = "Surname")]
    last_name: String,
}
```

### Hide a column

You can mark fields as hidden in which case they fill be ignored and not be present on a sheet.

A similar affect could be achieved by the means of a `Disable` setting.

```rust,no_run
use tabled::Tabled;

#[derive(Tabled)]
struct Person {
   id: u8,
   #[tabled(skip)]
   number: String,
   name: String,
}
```

### Set column order

You can change the order in which they will be displayed in table.

```rust,no_run
use tabled::Tabled;

#[derive(Tabled)]
struct Person {
   id: u8,
   #[tabled(order = 0)]
   number: String,
   #[tabled(order = 1)]
   name: String,
}
```

### Format fields

As was said already, using `#[derive(Tabled)]` is possible only when all fields implement a `Display` trait.
However, this may be often not the case for example when a field uses the `Option` type. There's 2 common ways how to solve this:

- Implement `Tabled` trait manually for a type.
- Wrap `Option` to something like `DisplayedOption<T>(Option<T>)` and implement a Display trait for it.

Alternatively, you can use the `#[tabled(display_with = "func")]` attribute for the field to specify a display function.

```rust,no_run
use tabled::Tabled;

#[derive(Tabled)]
pub struct MyRecord {
    pub id: i64,
    #[tabled(display_with = "display_option")]
    pub valid: Option<bool>
}

fn display_option(o: &Option<bool>) -> String {
    match o {
        Some(s) => format!("is valid thing = {}", s),
        None => format!("is not valid"),
    }
}
```

It's also possible to change function argument to be `&self`,
using `#[tabled(display_with("some_function", self))]`

```rust,no_run
use tabled::Tabled;

#[derive(Tabled)]
pub struct MyRecord {
    pub id: i64,
    #[tabled(display_with("Self::display_valid", self))]
    pub valid: Option<bool>
}

impl MyRecord {
    fn display_valid(&self) -> String {
        match self.valid {
            Some(s) => format!("is valid thing = {}", s),
            None => format!("is not valid"),
        }
    }
}
```

There's also a probably more suitable way for formatting, if your format is constant.
Using `#[tabled(format = "{}")]` and `#[tabled(format("{}"))]` and proving a general formatting string.

```
use tabled::Tabled;

#[derive(Tabled)]
struct Record {
    #[tabled(skip)]
    id: u8,
    #[tabled(format("{}.{}", self.id, self.name))]
    name: String,
}
```

### Format headers

Beside `#[tabled(rename = "")]` you can change a format of a column name using
`#[tabled(rename_all = "UPPERCASE")]`.

```rust,no_run
use tabled::Tabled;

#[derive(Tabled)]
#[tabled(rename_all = "CamelCase")]
struct Person {
    id: u8,
    number: String,
    name: String,
    #[tabled(rename_all = "snake_case")]
    middle_name: String,
}
```

### Inline

It's possible to inline internal data if it implements the `Tabled` trait using `#[tabled(inline)]`.
You can also set a prefix which will be used for all inlined elements by `#[tabled(inline("prefix>>"))]`.

```rust,no_run
use tabled::Tabled;

#[derive(Tabled)]
struct Person {
    id: u8,
    name: String,
    #[tabled(inline)]
    ed: Education,
}

#[derive(Tabled)]
struct Education {
    uni: String,
    graduated: bool,
}
```

And it works for enums as well.

```rust,no_run
use tabled::Tabled;

#[derive(Tabled)]
enum Vehicle {
    #[tabled(inline("Auto::"))]
    Auto {
        model: String,
        engine: String,
    },
    #[tabled(inline)]
    Bikecycle(
        String,
        #[tabled(inline)] Bike,
    ),
}

#[derive(Tabled)]
struct Bike {
    brand: &'static str,
    price: f32,
}
```

```rust
pub use tabled_derive::Tabled;
```

