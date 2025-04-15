# Crate Documentation

**Version:** 2.8.0

**Format Version:** 43

# Module `pest_meta`

# pest meta

This crate parses, validates, optimizes, and converts pest's own grammars to ASTs.

## Modules

## Module `ast`

Types for the pest's abstract syntax tree.

```rust
pub mod ast { /* ... */ }
```

### Types

#### Struct `Rule`

A grammar rule

```rust
pub struct Rule {
    pub name: String,
    pub ty: RuleType,
    pub expr: Expr,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | The name of the rule |
| `ty` | `RuleType` | The rule's type (silent, atomic, ...) |
| `expr` | `Expr` | The rule's expression |

##### Implementations

###### Trait Implementations

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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Rule) -> bool { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Rule { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Enum `RuleType`

All possible rule types

```rust
pub enum RuleType {
    Normal,
    Silent,
    Atomic,
    CompoundAtomic,
    NonAtomic,
}
```

##### Variants

###### `Normal`

The normal rule type

###### `Silent`

Silent rules are just like normal rules
— when run, they function the same way —
except they do not produce pairs or tokens.
If a rule is silent, it will never appear in a parse result.
(their syntax is `_{ ... }`)

###### `Atomic`

atomic rule prevent implicit whitespace: inside an atomic rule,
the tilde ~ means "immediately followed by",
and repetition operators (asterisk * and plus sign +)
have no implicit separation. In addition, all other rules
called from an atomic rule are also treated as atomic.
In an atomic rule, interior matching rules are silent.
(their syntax is `@{ ... }`)

###### `CompoundAtomic`

Compound atomic rules are similar to atomic rules,
but they produce inner tokens as normal.
(their syntax is `${ ... }`)

###### `NonAtomic`

Non-atomic rules cancel the effect of atomic rules.
(their syntax is `!{ ... }`)

##### Implementations

###### Trait Implementations

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Eq**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RuleType) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> RuleType { /* ... */ }
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

#### Enum `Expr`

All possible rule expressions

# Warning: Semantic Versioning
There may be non-breaking changes to the meta-grammar
between minor versions. Those non-breaking changes, however,
may translate into semver-breaking changes due to the additional variants
propaged from the `Rule` enum. This is a known issue and will be fixed in the
future (e.g. by increasing MSRV and non_exhaustive annotations).

```rust
pub enum Expr {
    Str(String),
    Insens(String),
    Range(String, String),
    Ident(String),
    PeekSlice(i32, Option<i32>),
    PosPred(Box<Expr>),
    NegPred(Box<Expr>),
    Seq(Box<Expr>, Box<Expr>),
    Choice(Box<Expr>, Box<Expr>),
    Opt(Box<Expr>),
    Rep(Box<Expr>),
    RepOnce(Box<Expr>),
    RepExact(Box<Expr>, u32),
    RepMin(Box<Expr>, u32),
    RepMax(Box<Expr>, u32),
    RepMinMax(Box<Expr>, u32, u32),
    Skip(Vec<String>),
    Push(Box<Expr>),
}
```

##### Variants

###### `Str`

Matches an exact string, e.g. `"a"`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Insens`

Matches an exact string, case insensitively (ASCII only), e.g. `^"a"`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Range`

Matches one character in the range, e.g. `'a'..'z'`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |
| 1 | `String` |  |

###### `Ident`

Matches the rule with the given name, e.g. `a`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `PeekSlice`

Matches a custom part of the stack, e.g. `PEEK[..]`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |
| 1 | `Option<i32>` |  |

###### `PosPred`

Positive lookahead; matches expression without making progress, e.g. `&e`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `NegPred`

Negative lookahead; matches if expression doesn't match, without making progress, e.g. `!e`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `Seq`

Matches a sequence of two expressions, e.g. `e1 ~ e2`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |
| 1 | `Box<Expr>` |  |

###### `Choice`

Matches either of two expressions, e.g. `e1 | e2`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |
| 1 | `Box<Expr>` |  |

###### `Opt`

Optionally matches an expression, e.g. `e?`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `Rep`

Matches an expression zero or more times, e.g. `e*`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `RepOnce`

Matches an expression one or more times, e.g. `e+`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `RepExact`

Matches an expression an exact number of times, e.g. `e{n}`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |
| 1 | `u32` |  |

###### `RepMin`

Matches an expression at least a number of times, e.g. `e{n,}`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |
| 1 | `u32` |  |

###### `RepMax`

Matches an expression at most a number of times, e.g. `e{,n}`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |
| 1 | `u32` |  |

###### `RepMinMax`

Matches an expression a number of times within a range, e.g. `e{m, n}`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |
| 1 | `u32` |  |
| 2 | `u32` |  |

###### `Skip`

Continues to match expressions until one of the strings in the `Vec` is found

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<String>` |  |

###### `Push`

Matches an expression and pushes it to the stack, e.g. `push(e)`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn iter_top_down(self: &Self) -> ExprTopDownIterator { /* ... */ }
  ```
  Returns the iterator that steps the expression from top to bottom.

- ```rust
  pub fn map_top_down<F>(self: Self, f: F) -> Expr
where
    F: FnMut(Expr) -> Expr { /* ... */ }
  ```
  Applies `f` to the expression and all its children (top to bottom).

- ```rust
  pub fn map_bottom_up<F>(self: Self, f: F) -> Expr
where
    F: FnMut(Expr) -> Expr { /* ... */ }
  ```
  Applies `f` to the expression and all its children (bottom up).

###### Trait Implementations

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Expr { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Expr) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `ExprTopDownIterator`

The top down iterator for an expression.

```rust
pub struct ExprTopDownIterator {
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
  pub fn new(expr: &Expr) -> Self { /* ... */ }
  ```
  Constructs a top-down iterator from the expression.

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
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

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
## Module `optimizer`

Different optimizations for pest's ASTs.

```rust
pub mod optimizer { /* ... */ }
```

### Types

#### Struct `OptimizedRule`

The optimized version of the pest AST's `Rule`.

```rust
pub struct OptimizedRule {
    pub name: String,
    pub ty: RuleType,
    pub expr: OptimizedExpr,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | The name of the rule. |
| `ty` | `RuleType` | The type of the rule. |
| `expr` | `OptimizedExpr` | The optimized expression of the rule. |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OptimizedRule { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OptimizedRule) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
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

#### Enum `OptimizedExpr`

The optimized version of the pest AST's `Expr`.

# Warning: Semantic Versioning
There may be non-breaking changes to the meta-grammar
between minor versions. Those non-breaking changes, however,
may translate into semver-breaking changes due to the additional variants
propaged from the `Rule` enum. This is a known issue and will be fixed in the
future (e.g. by increasing MSRV and non_exhaustive annotations).

```rust
pub enum OptimizedExpr {
    Str(String),
    Insens(String),
    Range(String, String),
    Ident(String),
    PeekSlice(i32, Option<i32>),
    PosPred(Box<OptimizedExpr>),
    NegPred(Box<OptimizedExpr>),
    Seq(Box<OptimizedExpr>, Box<OptimizedExpr>),
    Choice(Box<OptimizedExpr>, Box<OptimizedExpr>),
    Opt(Box<OptimizedExpr>),
    Rep(Box<OptimizedExpr>),
    Skip(Vec<String>),
    Push(Box<OptimizedExpr>),
    RestoreOnErr(Box<OptimizedExpr>),
}
```

##### Variants

###### `Str`

Matches an exact string, e.g. `"a"`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Insens`

Matches an exact string, case insensitively (ASCII only), e.g. `^"a"`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Range`

Matches one character in the range, e.g. `'a'..'z'`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |
| 1 | `String` |  |

###### `Ident`

Matches the rule with the given name, e.g. `a`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `PeekSlice`

Matches a custom part of the stack, e.g. `PEEK[..]`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |
| 1 | `Option<i32>` |  |

###### `PosPred`

Positive lookahead; matches expression without making progress, e.g. `&e`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<OptimizedExpr>` |  |

###### `NegPred`

Negative lookahead; matches if expression doesn't match, without making progress, e.g. `!e`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<OptimizedExpr>` |  |

###### `Seq`

Matches a sequence of two expressions, e.g. `e1 ~ e2`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<OptimizedExpr>` |  |
| 1 | `Box<OptimizedExpr>` |  |

###### `Choice`

Matches either of two expressions, e.g. `e1 | e2`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<OptimizedExpr>` |  |
| 1 | `Box<OptimizedExpr>` |  |

###### `Opt`

Optionally matches an expression, e.g. `e?`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<OptimizedExpr>` |  |

###### `Rep`

Matches an expression zero or more times, e.g. `e*`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<OptimizedExpr>` |  |

###### `Skip`

Continues to match expressions until one of the strings in the `Vec` is found

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<String>` |  |

###### `Push`

Matches an expression and pushes it to the stack, e.g. `push(e)`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<OptimizedExpr>` |  |

###### `RestoreOnErr`

Restores an expression's checkpoint

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<OptimizedExpr>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn iter_top_down(self: &Self) -> OptimizedExprTopDownIterator { /* ... */ }
  ```
  Returns a top-down iterator over the `OptimizedExpr`.

- ```rust
  pub fn map_top_down<F>(self: Self, f: F) -> OptimizedExpr
where
    F: FnMut(OptimizedExpr) -> OptimizedExpr { /* ... */ }
  ```
  Applies `f` to the `OptimizedExpr` top-down.

- ```rust
  pub fn map_bottom_up<F>(self: Self, f: F) -> OptimizedExpr
where
    F: FnMut(OptimizedExpr) -> OptimizedExpr { /* ... */ }
  ```
  Applies `f` to the `OptimizedExpr` bottom-up.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Unpin**
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
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OptimizedExpr) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OptimizedExpr { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
#### Struct `OptimizedExprTopDownIterator`

A top-down iterator over an `OptimizedExpr`.

```rust
pub struct OptimizedExprTopDownIterator {
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
  pub fn new(expr: &OptimizedExpr) -> Self { /* ... */ }
  ```
  Creates a new top down iterator from an `OptimizedExpr`.

###### Trait Implementations

- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Functions

#### Function `optimize`

Takes pest's ASTs and optimizes them

```rust
pub fn optimize(rules: Vec<Rule>) -> Vec<OptimizedRule> { /* ... */ }
```

## Module `parser`

Types and helpers for the pest's own grammar parser.

```rust
pub mod parser { /* ... */ }
```

### Types

#### Struct `ParserRule`

The pest grammar rule

```rust
pub struct ParserRule<''i> {
    pub name: String,
    pub span: pest::Span<''i>,
    pub ty: crate::ast::RuleType,
    pub node: ParserNode<''i>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` | The rule's name |
| `span` | `pest::Span<''i>` | The rule's span |
| `ty` | `crate::ast::RuleType` | The rule's type |
| `node` | `ParserNode<''i>` | The rule's parser node |

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParserRule<''i> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ParserRule<''i>) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **StructuralPartialEq**
#### Struct `ParserNode`

The pest grammar node

```rust
pub struct ParserNode<''i> {
    pub expr: ParserExpr<''i>,
    pub span: pest::Span<''i>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `ParserExpr<''i>` | The node's expression |
| `span` | `pest::Span<''i>` | The node's span |

##### Implementations

###### Methods

- ```rust
  pub fn filter_map_top_down<F, T>(self: Self, f: F) -> Vec<T>
where
    F: FnMut(ParserNode<''i>) -> Option<T> { /* ... */ }
  ```
  will remove nodes that do not match `f`

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParserNode<''i> { /* ... */ }
    ```

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ParserNode<''i>) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Enum `ParserExpr`

All possible parser expressions

```rust
pub enum ParserExpr<''i> {
    Str(String),
    Insens(String),
    Range(String, String),
    Ident(String),
    PeekSlice(i32, Option<i32>),
    PosPred(Box<ParserNode<''i>>),
    NegPred(Box<ParserNode<''i>>),
    Seq(Box<ParserNode<''i>>, Box<ParserNode<''i>>),
    Choice(Box<ParserNode<''i>>, Box<ParserNode<''i>>),
    Opt(Box<ParserNode<''i>>),
    Rep(Box<ParserNode<''i>>),
    RepOnce(Box<ParserNode<''i>>),
    RepExact(Box<ParserNode<''i>>, u32),
    RepMin(Box<ParserNode<''i>>, u32),
    RepMax(Box<ParserNode<''i>>, u32),
    RepMinMax(Box<ParserNode<''i>>, u32, u32),
    Push(Box<ParserNode<''i>>),
}
```

##### Variants

###### `Str`

Matches an exact string, e.g. `"a"`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Insens`

Matches an exact string, case insensitively (ASCII only), e.g. `^"a"`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Range`

Matches one character in the range, e.g. `'a'..'z'`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |
| 1 | `String` |  |

###### `Ident`

Matches the rule with the given name, e.g. `a`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `PeekSlice`

Matches a custom part of the stack, e.g. `PEEK[..]`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |
| 1 | `Option<i32>` |  |

###### `PosPred`

Positive lookahead; matches expression without making progress, e.g. `&e`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ParserNode<''i>>` |  |

###### `NegPred`

Negative lookahead; matches if expression doesn't match, without making progress, e.g. `!e`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ParserNode<''i>>` |  |

###### `Seq`

Matches a sequence of two expressions, e.g. `e1 ~ e2`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ParserNode<''i>>` |  |
| 1 | `Box<ParserNode<''i>>` |  |

###### `Choice`

Matches either of two expressions, e.g. `e1 | e2`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ParserNode<''i>>` |  |
| 1 | `Box<ParserNode<''i>>` |  |

###### `Opt`

Optionally matches an expression, e.g. `e?`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ParserNode<''i>>` |  |

###### `Rep`

Matches an expression zero or more times, e.g. `e*`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ParserNode<''i>>` |  |

###### `RepOnce`

Matches an expression one or more times, e.g. `e+`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ParserNode<''i>>` |  |

###### `RepExact`

Matches an expression an exact number of times, e.g. `e{n}`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ParserNode<''i>>` |  |
| 1 | `u32` |  |

###### `RepMin`

Matches an expression at least a number of times, e.g. `e{n,}`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ParserNode<''i>>` |  |
| 1 | `u32` |  |

###### `RepMax`

Matches an expression at most a number of times, e.g. `e{,n}`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ParserNode<''i>>` |  |
| 1 | `u32` |  |

###### `RepMinMax`

Matches an expression a number of times within a range, e.g. `e{m, n}`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ParserNode<''i>>` |  |
| 1 | `u32` |  |
| 2 | `u32` |  |

###### `Push`

Matches an expression and pushes it to the stack, e.g. `push(e)`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<ParserNode<''i>>` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Unpin**
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
    fn clone(self: &Self) -> ParserExpr<''i> { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ParserExpr<''i>) -> bool { /* ... */ }
    ```

### Functions

#### Function `parse`

**Attributes:**

- `#[allow(clippy::perf)]`

A helper that will parse using the pest grammar

```rust
pub fn parse(rule: Rule, data: &str) -> Result<pest::iterators::Pairs<''_, Rule>, pest::error::Error<Rule>> { /* ... */ }
```

#### Function `consume_rules`

Converts a parser's result (`Pairs`) to an AST

```rust
pub fn consume_rules(pairs: pest::iterators::Pairs<''_, Rule>) -> Result<Vec<crate::ast::Rule>, Vec<pest::error::Error<Rule>>> { /* ... */ }
```

#### Function `rename_meta_rule`

**Attributes:**

- `#[inline]`

A helper function to rename verbose rules
for the sake of better error messages

```rust
pub fn rename_meta_rule(rule: &Rule) -> String { /* ... */ }
```

### Re-exports

#### Re-export `self::grammar::*`

Import included grammar (`PestParser` class globally for current module).

```rust
pub use self::grammar::*;
```

## Module `validator`

Helpers for validating pest grammars that could help with debugging
and provide a more user-friendly error message.

```rust
pub mod validator { /* ... */ }
```

### Functions

#### Function `validate_pairs`

It checks the parsed grammar for common mistakes:
- using Pest keywords
- duplicate rules
- undefined rules

It returns a `Result` with a `Vec` of `Error`s if any of the above is found.
If no errors are found, it returns the vector of names of used builtin rules.

```rust
pub fn validate_pairs(pairs: pest::iterators::Pairs<''_, crate::parser::Rule>) -> Result<Vec<&str>, Vec<pest::error::Error<crate::parser::Rule>>> { /* ... */ }
```

#### Function `validate_rust_keywords`

**Attributes:**

- `#[allow(clippy::ptr_arg)]`

**⚠️ Deprecated**: Rust keywords are no longer restricted from the pest grammar

Validates that the given `definitions` do not contain any Rust keywords.

```rust
pub fn validate_rust_keywords(definitions: &Vec<pest::Span<''_>>) -> Vec<pest::error::Error<crate::parser::Rule>> { /* ... */ }
```

#### Function `validate_pest_keywords`

**Attributes:**

- `#[allow(clippy::ptr_arg)]`

Validates that the given `definitions` do not contain any Pest keywords.

```rust
pub fn validate_pest_keywords(definitions: &Vec<pest::Span<''_>>) -> Vec<pest::error::Error<crate::parser::Rule>> { /* ... */ }
```

#### Function `validate_already_defined`

**Attributes:**

- `#[allow(clippy::ptr_arg)]`

Validates that the given `definitions` do not contain any duplicate rules.

```rust
pub fn validate_already_defined(definitions: &Vec<pest::Span<''_>>) -> Vec<pest::error::Error<crate::parser::Rule>> { /* ... */ }
```

#### Function `validate_undefined`

**Attributes:**

- `#[allow(clippy::ptr_arg)]`

Validates that the given `definitions` do not contain any undefined rules.

```rust
pub fn validate_undefined<''i>(definitions: &Vec<pest::Span<''i>>, called_rules: &Vec<pest::Span<''i>>) -> Vec<pest::error::Error<crate::parser::Rule>> { /* ... */ }
```

#### Function `validate_ast`

**Attributes:**

- `#[allow(clippy::ptr_arg)]`

Validates the abstract syntax tree for common mistakes:
- infinite repetitions
- choices that cannot be reached
- left recursion

```rust
pub fn validate_ast<''a, ''i: ''a>(rules: &''a Vec<crate::parser::ParserRule<''i>>) -> Vec<pest::error::Error<crate::parser::Rule>> { /* ... */ }
```

## Functions

### Function `unwrap_or_report`

A helper that will unwrap the result or panic
with the nicely formatted error message.

```rust
pub fn unwrap_or_report<T, E>(result: Result<T, E>) -> T
where
    E: IntoIterator,
    <E as >::Item: Display { /* ... */ }
```

### Function `parse_and_optimize`

Parses, validates, processes and optimizes the provided grammar.

```rust
pub fn parse_and_optimize(grammar: &str) -> Result<(Vec<&''_ str>, Vec<optimizer::OptimizedRule>), Vec<pest::error::Error<parser::Rule>>> { /* ... */ }
```

