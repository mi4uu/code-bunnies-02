# Crate Documentation

**Version:** 2.0.0

**Format Version:** 43

# Module `divsufsort`

## Modules

## Module `crosscheck`

```rust
pub mod crosscheck { /* ... */ }
```

### Functions

#### Function `flush`

```rust
pub fn flush() { /* ... */ }
```

## Functions

### Function `sort_in_place`

Sort suffixes of `text` and store their lexographic order
in the given suffix array `sa`.
Will panic if `sa.len()` != `text.len()`

```rust
pub fn sort_in_place(text: &[u8], sa: &mut [i32]) { /* ... */ }
```

### Function `sort`

```rust
pub fn sort(text: &[u8]) -> sacabase::SuffixArray<''_, i32> { /* ... */ }
```

## Macros

### Macro `crosscheck`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! crosscheck {
    /* macro_rules! crosscheck {
    ($($arg: expr),*) => { ... };
} */
}
```

### Macro `SA_dump`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! SA_dump {
    /* macro_rules! SA_dump {
    ($SA: expr, $label: expr) => { ... };
} */
}
```

### Macro `A_dump`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! A_dump {
    /* macro_rules! A_dump {
    ($A: expr, $label: expr) => { ... };
} */
}
```

### Macro `BSTAR_dump`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! BSTAR_dump {
    /* macro_rules! BSTAR_dump {
    ($B: expr, $label: expr) => { ... };
} */
}
```

