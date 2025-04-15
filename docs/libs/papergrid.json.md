# Crate Documentation

**Version:** 0.12.0

**Format Version:** 43

# Module `papergrid`

Papergrid is a library for generating text-based tables.

It has relatively low level API.
If you're interested in a more friendly one take a look at [`tabled`](https://github.com/zhiburt/tabled).

# Example

```
use papergrid::{
    records::IterRecords,
    dimension::{Estimate},
    config::Borders,
    colors::NoColors,
    grid::iterable::Grid,
    config::spanned::SpannedConfig,
    dimension::spanned::SpannedGridDimension,
};

// Creating a borders structure of a grid.
let borders = Borders {
    top: Some('-'),
    top_left: Some('+'),
    top_right: Some('+'),
    top_intersection: Some('+'),
    bottom: Some('-'),
    bottom_left: Some('+'),
    bottom_right: Some('+'),
    bottom_intersection: Some('+'),
    horizontal: Some('-'),
    vertical: Some('|'),
    left: Some('|'),
    right: Some('|'),
    intersection: Some('+'),
    left_intersection: Some('+'),
    right_intersection: Some('+'),
};

// Creating a grid config.
let mut cfg = SpannedConfig::default();
cfg.set_borders(borders);

// Creating an actual data for grid.
let records = vec![vec!["Hello", "World"], vec!["Hi", "World"]];
let records = IterRecords::new(records, 2, None);

// Estimate grid dimension.
let mut dimension = SpannedGridDimension::default();
dimension.estimate(&records, &cfg);

// Creating a grid.
let grid = Grid::new(&records, &dimension, &cfg, NoColors).to_string();

assert_eq!(
    grid,
    concat!(
        "+-----+-----+\n",
        "|Hello|World|\n",
        "+-----+-----+\n",
        "|Hi   |World|\n",
        "+-----+-----+",
    ),
);
```

## Modules

## Module `ansi`

A module which contains [`ANSIFmt`] trait and its implementation [`ANSIStr`]

```rust
pub mod ansi { /* ... */ }
```

### Traits

#### Trait `ANSIFmt`

A trait which prints an ANSI prefix and suffix.

```rust
pub trait ANSIFmt {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `fmt_ansi_prefix`: Print ANSI prefix.

##### Provided Methods

- ```rust
  fn fmt_ansi_suffix<W: Write>(self: &Self, f: &mut W) -> fmt::Result { /* ... */ }
  ```
  Print ANSI suffix.

##### Implementations

This trait is implemented for the following types:

- `ANSIBuf`
- `ANSIStr<''_>`
- `&C` with <C>
- `EmptyColor`

### Re-exports

#### Re-export `ANSIBuf`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use ansi_buf::ANSIBuf;
```

#### Re-export `ANSIStr`

```rust
pub use self::ansi_str::ANSIStr;
```

## Module `colors`

A module which contains [Colors] trait and its blanket implementations.

```rust
pub mod colors { /* ... */ }
```

### Types

#### Struct `NoColors`

The structure represents empty [`Colors`] map.

```rust
pub struct NoColors;
```

##### Implementations

###### Trait Implementations

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Default**
  - ```rust
    fn default() -> NoColors { /* ... */ }
    ```

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

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NoColors { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Colors**
  - ```rust
    fn get_color(self: &Self, _: Position) -> Option<&<Self as >::Color> { /* ... */ }
    ```

  - ```rust
    fn is_empty(self: &Self) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `EmptyColor`

A color which is actually has not value.

```rust
pub struct EmptyColor;
```

##### Implementations

###### Trait Implementations

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ANSIFmt**
  - ```rust
    fn fmt_ansi_prefix<W: core::fmt::Write>(self: &Self, _: &mut W) -> core::fmt::Result { /* ... */ }
    ```

  - ```rust
    fn fmt_ansi_suffix<W: core::fmt::Write>(self: &Self, _: &mut W) -> core::fmt::Result { /* ... */ }
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

- **UnwindSafe**
### Traits

#### Trait `Colors`

A trait which represents map of colors.

```rust
pub trait Colors {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Color`: Color implementation.

###### Required Methods

- `get_color`: Returns a color for a given position.
- `is_empty`: Verifies whether a map is empty or not.

##### Implementations

This trait is implemented for the following types:

- `&C` with <C>
- `std::collections::HashMap<crate::config::Position, C>` with <C>
- `std::collections::BTreeMap<crate::config::Position, C>` with <C>
- `crate::config::spanned::EntityMap<Option<C>>` with <C>
- `NoColors`

## Module `config`

A module which contains a general settings which might be used in other grid implementations.

```rust
pub mod config { /* ... */ }
```

### Modules

## Module `compact`

A module which contains configuration of a [`CompactGrid`] which is responsible for grid configuration.

[`CompactGrid`]: crate::grid::compact::CompactGrid

```rust
pub mod compact { /* ... */ }
```

### Types

#### Struct `CompactConfig`

This structure represents a settings of a grid.

grid: crate::Grid.

```rust
pub struct CompactConfig {
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
  Returns an standard config.

- ```rust
  pub const fn set_margin(self: Self, margin: Sides<Indent>) -> Self { /* ... */ }
  ```
  Set grid margin.

- ```rust
  pub const fn get_margin(self: &Self) -> &Sides<Indent> { /* ... */ }
  ```
  Returns a grid margin.

- ```rust
  pub const fn set_borders(self: Self, borders: Borders<char>) -> Self { /* ... */ }
  ```
  Set the [`Borders`] value as correct one.

- ```rust
  pub const fn get_borders(self: &Self) -> &Borders<char> { /* ... */ }
  ```
  Returns a current [`Borders`] structure.

- ```rust
  pub const fn get_borders_color(self: &Self) -> &Borders<ANSIStr<''static>> { /* ... */ }
  ```
  Returns a current [`Borders`] structure.

- ```rust
  pub const fn set_padding(self: Self, padding: Sides<Indent>) -> Self { /* ... */ }
  ```
  Set a padding to a given cells.

- ```rust
  pub const fn get_padding(self: &Self) -> &Sides<Indent> { /* ... */ }
  ```
  Get a padding for a given.

- ```rust
  pub const fn set_alignment_horizontal(self: Self, alignment: AlignmentHorizontal) -> Self { /* ... */ }
  ```
  Set a horizontal alignment.

- ```rust
  pub const fn get_alignment_horizontal(self: &Self) -> AlignmentHorizontal { /* ... */ }
  ```
  Get a alignment horizontal.

- ```rust
  pub const fn set_borders_color(self: Self, borders: Borders<ANSIStr<''static>>) -> Self { /* ... */ }
  ```
  Sets colors of border carcass on the grid.

- ```rust
  pub const fn set_margin_color(self: Self, color: Sides<ANSIStr<''static>>) -> Self { /* ... */ }
  ```
  Set colors for a margin.

- ```rust
  pub const fn get_margin_color(self: &Self) -> &Sides<ANSIStr<''static>> { /* ... */ }
  ```
  Returns a margin color.

- ```rust
  pub const fn set_padding_color(self: Self, color: Sides<ANSIStr<''static>>) -> Self { /* ... */ }
  ```
  Set a padding to a given cells.

- ```rust
  pub const fn get_padding_color(self: &Self) -> &Sides<ANSIStr<''static>> { /* ... */ }
  ```
  Set a padding to a given cells.

###### Trait Implementations

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CompactConfig { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CompactConfig) -> bool { /* ... */ }
    ```

- **Estimate**
  - ```rust
    fn estimate(self: &mut Self, records: R, cfg: &CompactConfig) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &CompactConfig) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &CompactConfig) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(compact: CompactConfig) -> Self { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Eq**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

## Module `spanned`

**Attributes:**

- `#[<cfg>(feature = "std")]`

A module which contains configuration options for a [`Grid`].

[`Grid`]: crate::grid::iterable::Grid

```rust
pub mod spanned { /* ... */ }
```

### Types

#### Struct `SpannedConfig`

This structure represents a settings of a grid.

grid: crate::Grid.

```rust
pub struct SpannedConfig {
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
  pub fn set_margin(self: &mut Self, margin: Sides<Indent>) { /* ... */ }
  ```
  Set a margin of a grid.

- ```rust
  pub fn set_margin_color(self: &mut Self, margin: Sides<Option<ANSIBuf>>) { /* ... */ }
  ```
  Set a color of margin of a grid.

- ```rust
  pub fn set_margin_offset(self: &mut Self, margin: Sides<Offset>) { /* ... */ }
  ```
  Set an offset of margin of a grid.

- ```rust
  pub fn get_margin(self: &Self) -> Sides<Indent> { /* ... */ }
  ```
  Returns a margin value currently set.

- ```rust
  pub fn get_margin_color(self: &Self) -> Sides<Option<ANSIBuf>> { /* ... */ }
  ```
  Returns a margin color value currently set.

- ```rust
  pub fn get_margin_offset(self: &Self) -> Sides<Offset> { /* ... */ }
  ```
  Returns a margin offset value currently set.

- ```rust
  pub fn remove_borders(self: &mut Self) { /* ... */ }
  ```
  Removes border changes.

- ```rust
  pub fn remove_borders_colors(self: &mut Self) { /* ... */ }
  ```
  Removes border changes.

- ```rust
  pub fn remove_color_line_horizontal(self: &mut Self) { /* ... */ }
  ```
  Removes border changes.

- ```rust
  pub fn remove_color_line_vertical(self: &mut Self) { /* ... */ }
  ```
  Removes border changes.

- ```rust
  pub fn remove_horizontal_chars(self: &mut Self) { /* ... */ }
  ```
  Removes border changes.

- ```rust
  pub fn remove_vertical_chars(self: &mut Self) { /* ... */ }
  ```
  Removes border changes.

- ```rust
  pub fn set_borders(self: &mut Self, borders: Borders<char>) { /* ... */ }
  ```
  Set the [`Borders`] value as correct one.

- ```rust
  pub fn get_border_default(self: &Self) -> Option<&char> { /* ... */ }
  ```
  Gets a global border value if set.

- ```rust
  pub fn set_border_default(self: &mut Self, c: char) { /* ... */ }
  ```
  Set the all [`Borders`] values to a char.

- ```rust
  pub fn get_borders(self: &Self) -> &Borders<char> { /* ... */ }
  ```
  Returns a current [`Borders`] structure.

- ```rust
  pub fn insert_horizontal_line(self: &mut Self, line: usize, val: super::HorizontalLine<char>) { /* ... */ }
  ```
  Set the border line by row index.

- ```rust
  pub fn remove_horizontal_line(self: &mut Self, line: usize, count_rows: usize) { /* ... */ }
  ```
  Sets off the border line by row index if any were set

- ```rust
  pub fn get_vertical_line(self: &Self, line: usize) -> Option<&super::VerticalLine<char>> { /* ... */ }
  ```
  Gets a overridden vertical line.

- ```rust
  pub fn get_vertical_lines(self: &Self) -> HashMap<usize, super::VerticalLine<char>> { /* ... */ }
  ```
  Gets all overridden vertical lines.

- ```rust
  pub fn insert_vertical_line(self: &mut Self, line: usize, val: super::VerticalLine<char>) { /* ... */ }
  ```
  Set the border line by column index.

- ```rust
  pub fn remove_vertical_line(self: &mut Self, line: usize, count_columns: usize) { /* ... */ }
  ```
  Sets off the border line by column index if any were set

- ```rust
  pub fn get_horizontal_line(self: &Self, line: usize) -> Option<&super::HorizontalLine<char>> { /* ... */ }
  ```
  Gets a overridden line.

- ```rust
  pub fn get_horizontal_lines(self: &Self) -> HashMap<usize, super::HorizontalLine<char>> { /* ... */ }
  ```
  Gets all overridden lines.

- ```rust
  pub fn set_horizontal_char(self: &mut Self, pos: Position, c: char, offset: Offset) { /* ... */ }
  ```
  Override a character on a horizontal line.

- ```rust
  pub fn lookup_horizontal_char(self: &Self, pos: Position, offset: usize, end: usize) -> Option<char> { /* ... */ }
  ```
  Get a list of overridden chars in a horizontal border.

- ```rust
  pub fn is_overridden_horizontal(self: &Self, pos: Position) -> bool { /* ... */ }
  ```
  Checks if there any char in a horizontal border being overridden.

- ```rust
  pub fn remove_overridden_horizontal(self: &mut Self, pos: Position) { /* ... */ }
  ```
  Removes a list of overridden chars in a horizontal border.

- ```rust
  pub fn set_vertical_char(self: &mut Self, pos: Position, c: char, offset: Offset) { /* ... */ }
  ```
  Override a vertical split line.

- ```rust
  pub fn lookup_vertical_char(self: &Self, pos: Position, offset: usize, end: usize) -> Option<char> { /* ... */ }
  ```
  Get a list of overridden chars in a horizontal border.

- ```rust
  pub fn is_overridden_vertical(self: &Self, pos: Position) -> bool { /* ... */ }
  ```
  Checks if there any char in a horizontal border being overridden.

- ```rust
  pub fn remove_overridden_vertical(self: &mut Self, pos: Position) { /* ... */ }
  ```
  Removes a list of overridden chars in a horizontal border.

- ```rust
  pub fn set_horizontal_color(self: &mut Self, pos: Position, c: ANSIBuf, offset: Offset) { /* ... */ }
  ```
  Override a character color on a horizontal line.

- ```rust
  pub fn lookup_horizontal_color(self: &Self, pos: Position, offset: usize, end: usize) -> Option<&ANSIBuf> { /* ... */ }
  ```
  Get a overridden color in a horizontal border.

- ```rust
  pub fn set_vertical_color(self: &mut Self, pos: Position, c: ANSIBuf, offset: Offset) { /* ... */ }
  ```
  Override a character color on a vertical line.

- ```rust
  pub fn lookup_vertical_color(self: &Self, pos: Position, offset: usize, end: usize) -> Option<&ANSIBuf> { /* ... */ }
  ```
  Get a overridden color in a vertical border.

- ```rust
  pub fn set_padding(self: &mut Self, entity: Entity, padding: Sides<Indent>) { /* ... */ }
  ```
  Set a padding to a given cells.

- ```rust
  pub fn set_padding_color(self: &mut Self, entity: Entity, padding: Sides<Option<ANSIBuf>>) { /* ... */ }
  ```
  Set a padding to a given cells.

- ```rust
  pub fn get_padding(self: &Self, entity: Entity) -> Sides<Indent> { /* ... */ }
  ```
  Get a padding for a given [Entity].

- ```rust
  pub fn get_padding_color(self: &Self, entity: Entity) -> Sides<Option<ANSIBuf>> { /* ... */ }
  ```
  Get a padding color for a given [Entity].

- ```rust
  pub fn set_formatting(self: &mut Self, entity: Entity, formatting: Formatting) { /* ... */ }
  ```
  Set a formatting to a given cells.

- ```rust
  pub fn get_formatting(self: &Self, entity: Entity) -> &Formatting { /* ... */ }
  ```
  Get a formatting settings for a given [Entity].

- ```rust
  pub fn set_alignment_vertical(self: &mut Self, entity: Entity, alignment: AlignmentVertical) { /* ... */ }
  ```
  Set a vertical alignment to a given cells.

- ```rust
  pub fn get_alignment_vertical(self: &Self, entity: Entity) -> &AlignmentVertical { /* ... */ }
  ```
  Get a vertical alignment for a given [Entity].

- ```rust
  pub fn set_alignment_horizontal(self: &mut Self, entity: Entity, alignment: AlignmentHorizontal) { /* ... */ }
  ```
  Set a horizontal alignment to a given cells.

- ```rust
  pub fn get_alignment_horizontal(self: &Self, entity: Entity) -> &AlignmentHorizontal { /* ... */ }
  ```
  Get a horizontal alignment for a given [Entity].

- ```rust
  pub fn set_border(self: &mut Self, pos: Position, border: Border<char>) { /* ... */ }
  ```
  Set border set a border value to all cells in [`Entity`].

- ```rust
  pub fn get_border(self: &Self, pos: Position, shape: (usize, usize)) -> Border<char> { /* ... */ }
  ```
  Returns a border of a cell.

- ```rust
  pub fn get_border_color(self: &Self, pos: Position, shape: (usize, usize)) -> Border<&ANSIBuf> { /* ... */ }
  ```
  Returns a border color of a cell.

- ```rust
  pub fn set_borders_missing(self: &mut Self, c: char) { /* ... */ }
  ```
  Set a character which will be used in case any misconfiguration of borders.

- ```rust
  pub fn get_borders_missing(self: &Self) -> char { /* ... */ }
  ```
  Get a character which will be used in case any misconfiguration of borders.

- ```rust
  pub fn get_border_color_default(self: &Self) -> Option<&ANSIBuf> { /* ... */ }
  ```
  Gets a color of all borders on the grid.

- ```rust
  pub fn set_border_color_default(self: &mut Self, clr: ANSIBuf) { /* ... */ }
  ```
  Sets a color of all borders on the grid.

- ```rust
  pub fn get_color_borders(self: &Self) -> &Borders<ANSIBuf> { /* ... */ }
  ```
  Gets colors of a borders carcass on the grid.

- ```rust
  pub fn set_borders_color(self: &mut Self, clrs: Borders<ANSIBuf>) { /* ... */ }
  ```
  Sets colors of border carcass on the grid.

- ```rust
  pub fn set_border_color(self: &mut Self, pos: Position, border: Border<ANSIBuf>) { /* ... */ }
  ```
  Sets a color of border of a cell on the grid.

- ```rust
  pub fn remove_border(self: &mut Self, pos: Position, shape: (usize, usize)) { /* ... */ }
  ```
  Sets off all borders possible on the [`Entity`].

- ```rust
  pub fn remove_border_color(self: &mut Self, pos: Position, shape: (usize, usize)) { /* ... */ }
  ```
  Gets a color of border of a cell on the grid.

- ```rust
  pub fn get_justification(self: &Self, entity: Entity) -> char { /* ... */ }
  ```
  Get a justification which will be used while expanding cells width/height.

- ```rust
  pub fn get_justification_color(self: &Self, entity: Entity) -> Option<&ANSIBuf> { /* ... */ }
  ```
  Get a justification color which will be used while expanding cells width/height.

- ```rust
  pub fn set_justification(self: &mut Self, entity: Entity, c: char) { /* ... */ }
  ```
  Set a justification which will be used while expanding cells width/height.

- ```rust
  pub fn set_justification_color(self: &mut Self, entity: Entity, color: Option<ANSIBuf>) { /* ... */ }
  ```
  Set a justification color which will be used while expanding cells width/height.

- ```rust
  pub fn get_column_spans(self: &Self) -> HashMap<Position, usize> { /* ... */ }
  ```
  Get a span value of the cell, if any is set.

- ```rust
  pub fn get_row_spans(self: &Self) -> HashMap<Position, usize> { /* ... */ }
  ```
  Get a span value of the cell, if any is set.

- ```rust
  pub fn get_column_span(self: &Self, pos: Position) -> Option<usize> { /* ... */ }
  ```
  Get a span value of the cell, if any is set.

- ```rust
  pub fn get_row_span(self: &Self, pos: Position) -> Option<usize> { /* ... */ }
  ```
  Get a span value of the cell, if any is set.

- ```rust
  pub fn remove_column_spans(self: &mut Self) { /* ... */ }
  ```
  Removes column spans.

- ```rust
  pub fn remove_row_spans(self: &mut Self) { /* ... */ }
  ```
  Removes row spans.

- ```rust
  pub fn set_column_span(self: &mut Self, pos: Position, span: usize) { /* ... */ }
  ```
  Set a column span to a given cells.

- ```rust
  pub fn has_column_spans(self: &Self) -> bool { /* ... */ }
  ```
  Verifies if there's any spans set.

- ```rust
  pub fn set_row_span(self: &mut Self, pos: Position, span: usize) { /* ... */ }
  ```
  Set a column span to a given cells.

- ```rust
  pub fn has_row_spans(self: &Self) -> bool { /* ... */ }
  ```
  Verifies if there's any spans set.

- ```rust
  pub fn has_border_colors(self: &Self) -> bool { /* ... */ }
  ```
  Verifies if there's any colors set for a borders.

- ```rust
  pub fn has_offset_chars(self: &Self) -> bool { /* ... */ }
  ```
  Verifies if there's any colors set for a borders.

- ```rust
  pub fn has_justification(self: &Self) -> bool { /* ... */ }
  ```
  Verifies if there's any colors set for a borders.

- ```rust
  pub fn has_padding(self: &Self) -> bool { /* ... */ }
  ```
  Verifies if there's any custom padding set.

- ```rust
  pub fn has_padding_color(self: &Self) -> bool { /* ... */ }
  ```
  Verifies if there's any custom padding set.

- ```rust
  pub fn has_formatting(self: &Self) -> bool { /* ... */ }
  ```
  Verifies if there's any custom formatting set.

- ```rust
  pub fn has_alignment_vertical(self: &Self) -> bool { /* ... */ }
  ```
  Verifies if there's any custom alignment vertical set.

- ```rust
  pub fn has_alignment_horizontal(self: &Self) -> bool { /* ... */ }
  ```
  Verifies if there's any custom alignment horizontal set.

- ```rust
  pub fn get_intersection(self: &Self, pos: Position, shape: (usize, usize)) -> Option<char> { /* ... */ }
  ```
  Gets an intersection character which would be rendered on the grid.

- ```rust
  pub fn get_horizontal(self: &Self, pos: Position, count_rows: usize) -> Option<char> { /* ... */ }
  ```
  Gets a horizontal character which would be rendered on the grid.

- ```rust
  pub fn get_vertical(self: &Self, pos: Position, count_columns: usize) -> Option<char> { /* ... */ }
  ```
  Gets a vertical character which would be rendered on the grid.

- ```rust
  pub fn get_horizontal_color(self: &Self, pos: Position, count_rows: usize) -> Option<&ANSIBuf> { /* ... */ }
  ```
  Gets a color of a cell horizontal.

- ```rust
  pub fn get_vertical_color(self: &Self, pos: Position, count_columns: usize) -> Option<&ANSIBuf> { /* ... */ }
  ```
  Gets a color of a cell vertical.

- ```rust
  pub fn get_intersection_color(self: &Self, pos: Position, shape: (usize, usize)) -> Option<&ANSIBuf> { /* ... */ }
  ```
  Gets a color of a cell vertical.

- ```rust
  pub fn has_horizontal(self: &Self, row: usize, count_rows: usize) -> bool { /* ... */ }
  ```
  Checks if grid would have a horizontal border with the current configuration.

- ```rust
  pub fn has_vertical(self: &Self, col: usize, count_columns: usize) -> bool { /* ... */ }
  ```
  Checks if grid would have a vertical border with the current configuration.

- ```rust
  pub fn count_horizontal(self: &Self, count_rows: usize) -> usize { /* ... */ }
  ```
  Calculates an amount of horizontal lines would present on the grid.

- ```rust
  pub fn count_vertical(self: &Self, count_columns: usize) -> usize { /* ... */ }
  ```
  Calculates an amount of vertical lines would present on the grid.

- ```rust
  pub fn is_cell_visible(self: &Self, pos: Position) -> bool { /* ... */ }
  ```
  The function returns whether the cells will be rendered or it will be hidden because of a span.

- ```rust
  pub fn is_cell_covered_by_row_span(self: &Self, pos: Position) -> bool { /* ... */ }
  ```
  The function checks if a cell is hidden because of a row span.

- ```rust
  pub fn is_cell_covered_by_column_span(self: &Self, pos: Position) -> bool { /* ... */ }
  ```
  The function checks if a cell is hidden because of a column span.

- ```rust
  pub fn is_cell_covered_by_both_spans(self: &Self, pos: Position) -> bool { /* ... */ }
  ```
  The function checks if a cell is hidden indirectly because of a row and column span combination.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(compact: CompactConfig) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SpannedConfig) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SpannedConfig { /* ... */ }
    ```

- **Freeze**
- **Estimate**
  - ```rust
    fn estimate(self: &mut Self, records: R, cfg: &SpannedConfig) { /* ... */ }
    ```

  - ```rust
    fn estimate(self: &mut Self, records: &VecRecords<T>, cfg: &SpannedConfig) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Re-exports

#### Re-export `EntityMap`

```rust
pub use self::entity_map::EntityMap;
```

#### Re-export `Offset`

```rust
pub use self::offset::Offset;
```

### Re-exports

#### Re-export `AlignmentHorizontal`

```rust
pub use alignment::AlignmentHorizontal;
```

#### Re-export `AlignmentVertical`

```rust
pub use alignment::AlignmentVertical;
```

#### Re-export `Border`

```rust
pub use border::Border;
```

#### Re-export `Borders`

```rust
pub use borders::Borders;
```

#### Re-export `Entity`

```rust
pub use entity::Entity;
```

#### Re-export `EntityIterator`

```rust
pub use entity::EntityIterator;
```

#### Re-export `Formatting`

```rust
pub use formatting::Formatting;
```

#### Re-export `HorizontalLine`

```rust
pub use horizontal_line::HorizontalLine;
```

#### Re-export `Indent`

```rust
pub use indent::Indent;
```

#### Re-export `Position`

```rust
pub use position::Position;
```

#### Re-export `Sides`

```rust
pub use sides::Sides;
```

#### Re-export `VerticalLine`

```rust
pub use vertical_line::VerticalLine;
```

## Module `dimension`

The module contains an [`Dimension`] trait and its implementations.

```rust
pub mod dimension { /* ... */ }
```

### Modules

## Module `compact`

**Attributes:**

- `#[<cfg>(feature = "std")]`

The module contains a [`CompactGridDimension`] for [`CompactGrid`] height/width estimation.

[`CompactGrid`]: crate::grid::compact::CompactGrid

```rust
pub mod compact { /* ... */ }
```

### Types

#### Struct `CompactGridDimension`

A [`Dimension`] implementation which calculates exact column/row width/height.

[`Grid`]: crate::grid::iterable::Grid

```rust
pub struct CompactGridDimension {
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
  pub fn height<R>(records: R, cfg: &CompactConfig) -> Vec<usize>
where
    R: Records,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str> { /* ... */ }
  ```
  Calculates height of rows.

- ```rust
  pub fn width<R>(records: R, cfg: &CompactConfig) -> Vec<usize>
where
    R: Records,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str> { /* ... */ }
  ```
  Calculates width of columns.

- ```rust
  pub fn dimension<R>(records: R, cfg: &CompactConfig) -> (Vec<usize>, Vec<usize>)
where
    R: Records,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str> { /* ... */ }
  ```
  Calculates dimensions of columns.

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> CompactGridDimension { /* ... */ }
    ```

- **Estimate**
  - ```rust
    fn estimate(self: &mut Self, records: R, cfg: &CompactConfig) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
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

- **Sync**
- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CompactGridDimension) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CompactGridDimension { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Dimension**
  - ```rust
    fn get_width(self: &Self, column: usize) -> usize { /* ... */ }
    ```

  - ```rust
    fn get_height(self: &Self, _: usize) -> usize { /* ... */ }
    ```

- **Eq**
- **UnwindSafe**
## Module `spanned`

**Attributes:**

- `#[<cfg>(feature = "std")]`

The module contains a [`SpannedGridDimension`] for [`Grid`] height/width estimation.

[`Grid`]: crate::grid::iterable::Grid

```rust
pub mod spanned { /* ... */ }
```

### Types

#### Struct `SpannedGridDimension`

A [`Dimension`] implementation which calculates exact column/row width/height.

[`Grid`]: crate::grid::iterable::Grid

```rust
pub struct SpannedGridDimension {
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
  pub fn height<R>(records: R, cfg: &SpannedConfig) -> Vec<usize>
where
    R: Records,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str> { /* ... */ }
  ```
  Calculates height of rows.

- ```rust
  pub fn width<R>(records: R, cfg: &SpannedConfig) -> Vec<usize>
where
    R: Records,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str> { /* ... */ }
  ```
  Calculates width of columns.

- ```rust
  pub fn width_total<R>(records: R, cfg: &SpannedConfig) -> usize
where
    R: Records,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str> { /* ... */ }
  ```
  Calculates width of columns.

- ```rust
  pub fn height_total<R>(records: R, cfg: &SpannedConfig) -> usize
where
    R: Records,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str> { /* ... */ }
  ```
  Calculates height of rows.

- ```rust
  pub fn get_values(self: Self) -> (Vec<usize>, Vec<usize>) { /* ... */ }
  ```
  Return width and height lists.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SpannedGridDimension { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> SpannedGridDimension { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **Estimate**
  - ```rust
    fn estimate(self: &mut Self, records: R, cfg: &SpannedConfig) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SpannedGridDimension) -> bool { /* ... */ }
    ```

- **Dimension**
  - ```rust
    fn get_width(self: &Self, column: usize) -> usize { /* ... */ }
    ```

  - ```rust
    fn get_height(self: &Self, row: usize) -> usize { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

## Module `spanned_vec_records`

**Attributes:**

- `#[<cfg>(feature = "std")]`

The module contains a [`SpannedVecRecordsDimension`] for [`Grid`] height/width estimation.

[`Grid`]: crate::grid::iterable::Grid

```rust
pub mod spanned_vec_records { /* ... */ }
```

### Types

#### Struct `SpannedVecRecordsDimension`

A [`Dimension`] implementation which calculates exact column/row width/height for [`VecRecords`].

It is a specialization of [`SpannedGridDimension`] for [`VecRecords`].

[`SpannedGridDimension`]: crate::dimension::spanned::SpannedGridDimension

```rust
pub struct SpannedVecRecordsDimension {
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
  pub fn height<T: Cell + AsRef<str>>(records: &VecRecords<T>, cfg: &SpannedConfig) -> Vec<usize> { /* ... */ }
  ```
  Calculates height of rows.

- ```rust
  pub fn width<T: Cell + AsRef<str>>(records: &VecRecords<T>, cfg: &SpannedConfig) -> Vec<usize> { /* ... */ }
  ```
  Calculates width of columns.

- ```rust
  pub fn get_values(self: Self) -> (Vec<usize>, Vec<usize>) { /* ... */ }
  ```
  Return width and height lists.

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SpannedVecRecordsDimension) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> SpannedVecRecordsDimension { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **Dimension**
  - ```rust
    fn get_width(self: &Self, column: usize) -> usize { /* ... */ }
    ```

  - ```rust
    fn get_height(self: &Self, row: usize) -> usize { /* ... */ }
    ```

- **Unpin**
- **Estimate**
  - ```rust
    fn estimate(self: &mut Self, records: &VecRecords<T>, cfg: &SpannedConfig) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SpannedVecRecordsDimension { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Traits

#### Trait `Dimension`

Dimension of a [`Grid`]

It's a friend trait of [`Estimate`].

[`Grid`]: crate::grid::iterable::Grid

```rust
pub trait Dimension {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `get_width`: Get a column width by index.
- `get_height`: Get a row height by index.

##### Implementations

This trait is implemented for the following types:

- `CompactGridDimension`
- `SpannedGridDimension`
- `SpannedVecRecordsDimension`
- `&T` with <T>

#### Trait `Estimate`

Dimension estimation of a [`Grid`]

It's a friend trait of [`Dimension`].

[`Grid`]: crate::grid::iterable::Grid

```rust
pub trait Estimate<R, C> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `estimate`: Estimates a metric.

##### Implementations

This trait is implemented for the following types:

- `CompactGridDimension` with <R>
- `SpannedGridDimension` with <R>
- `SpannedVecRecordsDimension` with <T>

## Module `grid`

Module contains a list of backends for pretty print tables.

```rust
pub mod grid { /* ... */ }
```

### Modules

## Module `compact`

The module contains a [`CompactGrid`] structure,
which is a relatively strict grid.

```rust
pub mod compact { /* ... */ }
```

### Types

#### Struct `CompactGrid`

Grid provides a set of methods for building a text-based table.

```rust
pub struct CompactGrid<R, D, G, C> {
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
  pub fn new(records: R, dimension: D, config: G) -> Self { /* ... */ }
  ```
  The new method creates a grid instance with default styles.

- ```rust
  pub fn with_colors<Colors>(self: Self, colors: Colors) -> CompactGrid<R, D, G, Colors> { /* ... */ }
  ```
  Sets colors map.

- ```rust
  pub fn build<F>(self: Self, f: F) -> fmt::Result
where
    R: Records,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str>,
    D: Dimension,
    C: Colors,
    G: Borrow<CompactConfig>,
    F: Write { /* ... */ }
  ```
  Builds a table.

- ```rust
  pub fn to_string(self: Self) -> String
where
    R: Records,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str>,
    D: Dimension,
    G: Borrow<CompactConfig>,
    C: Colors { /* ... */ }
  ```
  Builds a table into string.

###### Trait Implementations

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Sync**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CompactGrid<R, D, G, C> { /* ... */ }
    ```

- **Unpin**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `iterable`

**Attributes:**

- `#[<cfg>(feature = "std")]`

The module contains a [`Grid`] structure.

```rust
pub mod iterable { /* ... */ }
```

### Types

#### Struct `Grid`

Grid provides a set of methods for building a text-based table.

```rust
pub struct Grid<R, D, G, C> {
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
  pub fn new(records: R, dimension: D, config: G, colors: C) -> Self { /* ... */ }
  ```
  The new method creates a grid instance with default styles.

- ```rust
  pub fn build<F>(self: Self, f: F) -> fmt::Result
where
    R: Records,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str>,
    D: Dimension,
    C: Colors,
    G: Borrow<SpannedConfig>,
    F: Write { /* ... */ }
  ```
  Builds a table.

- ```rust
  pub fn to_string(self: Self) -> String
where
    R: Records,
    <<R as >::Iter as IntoRecords>::Cell: AsRef<str>,
    D: Dimension,
    G: Borrow<SpannedConfig>,
    C: Colors { /* ... */ }
  ```
  Builds a table into string.

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Grid<R, D, G, C> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `peekable`

**Attributes:**

- `#[<cfg>(feature = "std")]`

The module contains a [`PeekableGrid`] structure.

```rust
pub mod peekable { /* ... */ }
```

### Types

#### Struct `PeekableGrid`

Grid provides a set of methods for building a text-based table.

```rust
pub struct PeekableGrid<R, G, D, C> {
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
  pub fn new(records: R, config: G, dimension: D, colors: C) -> Self { /* ... */ }
  ```
  The new method creates a grid instance with default styles.

- ```rust
  pub fn build<F>(self: Self, f: F) -> fmt::Result
where
    R: Records + PeekableRecords + ExactRecords,
    D: Dimension,
    C: Colors,
    G: Borrow<SpannedConfig>,
    F: Write { /* ... */ }
  ```
  Builds a table.

- ```rust
  pub fn to_string(self: Self) -> String
where
    R: Records + PeekableRecords + ExactRecords,
    D: Dimension,
    G: Borrow<SpannedConfig>,
    C: Colors { /* ... */ }
  ```
  Builds a table into string.

###### Trait Implementations

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

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PeekableGrid<R, G, D, C> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `records`

The module contains a [Records] abstraction of a [`Grid`] trait and its implementers.

[`Grid`]: crate::grid::iterable::Grid

```rust
pub mod records { /* ... */ }
```

### Modules

## Module `vec_records`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Module contains [`VecRecords`].

```rust
pub mod vec_records { /* ... */ }
```

### Types

#### Struct `VecRecords`

A [Records] implementation based on allocated buffers.

```rust
pub struct VecRecords<T> {
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
  pub fn new(data: Vec<Vec<T>>) -> Self { /* ... */ }
  ```
  Creates new [`VecRecords`] structure.

###### Trait Implementations

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(records: VecRecords<T>) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **PeekableRecords**
  - ```rust
    fn get_text(self: &Self, (row, col): Position) -> &str { /* ... */ }
    ```

  - ```rust
    fn count_lines(self: &Self, (row, col): Position) -> usize { /* ... */ }
    ```

  - ```rust
    fn get_line(self: &Self, (row, col): Position, line: usize) -> &str { /* ... */ }
    ```

  - ```rust
    fn get_line_width(self: &Self, (row, col): Position, line: usize) -> usize { /* ... */ }
    ```

  - ```rust
    fn get_width(self: &Self, (row, col): Position) -> usize { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &VecRecords<T>) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> VecRecords<T> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> VecRecords<T> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &VecRecords<T>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Freeze**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &VecRecords<T>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Receiver**
- **Records**
  - ```rust
    fn iter_rows(self: Self) -> <<Self as >::Iter as IntoRecords>::IterRows { /* ... */ }
    ```

  - ```rust
    fn count_columns(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn hint_count_rows(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn iter_rows(self: Self) -> <<Self as >::Iter as IntoRecords>::IterRows { /* ... */ }
    ```

  - ```rust
    fn count_columns(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn hint_count_rows(self: &Self) -> Option<usize> { /* ... */ }
    ```

- **ExactRecords**
  - ```rust
    fn count_rows(self: &Self) -> usize { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Estimate**
  - ```rust
    fn estimate(self: &mut Self, records: &VecRecords<T>, cfg: &SpannedConfig) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Re-exports

#### Re-export `Cell`

```rust
pub use cell::Cell;
```

#### Re-export `StrWithWidth`

```rust
pub use cell_info::StrWithWidth;
```

#### Re-export `Text`

```rust
pub use cell_info::Text;
```

### Traits

#### Trait `Records`

Records represents table data.

```rust
pub trait Records {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Iter`: Iterator which goes over rows.

###### Required Methods

- `iter_rows`: Returns a iterator over rows.
- `count_columns`: Returns count of columns in the records.
- `hint_count_rows`: Hint amount of rows in the records.

##### Implementations

This trait is implemented for the following types:

- `IterRecords<I>` with <I>
- `&''a IterRecords<I>` with <''a, I>
- `VecRecords<T>` with <T>
- `&''a VecRecords<T>` with <''a, T>

### Re-exports

#### Re-export `ExactRecords`

```rust
pub use exact_records::ExactRecords;
```

#### Re-export `IntoRecords`

```rust
pub use into_records::IntoRecords;
```

#### Re-export `IterRecords`

```rust
pub use iter_records::IterRecords;
```

#### Re-export `PeekableRecords`

```rust
pub use peekable_records::PeekableRecords;
```

## Module `util`

A module contains utility functions which grid relay on.

```rust
pub mod util { /* ... */ }
```

### Modules

## Module `string`

This module contains a different functions which are used by the [`Grid`].

You should use it if you want to comply with how [`Grid`].

[`Grid`]: crate::grid::iterable::Grid

```rust
pub mod string { /* ... */ }
```

### Types

#### Struct `Lines`

**Attributes:**

- `#[allow(missing_debug_implementations)]`
- `#[<cfg>(feature = "std")]`

Iterator over lines.

In comparison to `std::str::Lines`, it treats trailing '\n' as a new line.

```rust
pub struct Lines<''a> {
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

- **Send**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Functions

#### Function `get_text_dimension`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Returns string width and count lines of a string. It's a combination of [`string_width_multiline`] and [`count_lines`].

```rust
pub fn get_text_dimension(text: &str) -> (usize, usize) { /* ... */ }
```

#### Function `get_line_width`

Returns a string width.

```rust
pub fn get_line_width(text: &str) -> usize { /* ... */ }
```

#### Function `get_text_width`

Returns a max string width of a line.

```rust
pub fn get_text_width(text: &str) -> usize { /* ... */ }
```

#### Function `get_char_width`

Returns a char width.

```rust
pub fn get_char_width(c: char) -> usize { /* ... */ }
```

#### Function `get_string_width`

Returns a string width (accouting all characters).

```rust
pub fn get_string_width(text: &str) -> usize { /* ... */ }
```

#### Function `count_lines`

Calculates a number of lines.

```rust
pub fn count_lines(s: &str) -> usize { /* ... */ }
```

#### Function `count_tabs`

Returns a list of tabs (`\t`) in a string..

```rust
pub fn count_tabs(s: &str) -> usize { /* ... */ }
```

#### Function `get_lines`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Splits the string by lines.

```rust
pub fn get_lines(text: &str) -> Lines<''_> { /* ... */ }
```

#### Function `replace_tab`

**Attributes:**

- `#[<cfg>(feature = "std")]`

Replaces tabs in a string with a given width of spaces.

```rust
pub fn replace_tab(text: &str, n: usize) -> std::borrow::Cow<''_, str> { /* ... */ }
```

