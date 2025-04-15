# Crate Documentation

**Version:** 0.6.0

**Format Version:** 43

# Module `convert_case`

Converts to and from various cases.

# Command Line Utility `ccase`

This library was developed for the purposes of a command line utility for converting
the case of strings and filenames.  You can check out 
[`ccase` on Github](https://github.com/rutrum/convert-case/tree/master/ccase).

# Rust Library

Provides a [`Case`](enum.Case.html) enum which defines a variety of cases to convert into.
Strings have implemented the [`Casing`](trait.Casing.html) trait, which adds methods for 
case conversion.

You can convert strings into a case using the [`to_case`](Casing::to_case) method.
```
use convert_case::{Case, Casing};

assert_eq!("Ronnie James Dio", "ronnie james dio".to_case(Case::Title));
assert_eq!("ronnieJamesDio", "Ronnie_James_dio".to_case(Case::Camel));
assert_eq!("Ronnie-James-Dio", "RONNIE_JAMES_DIO".to_case(Case::Train));
```

By default, `to_case` will split along a set of default word boundaries, that is
* space characters ` `,
* underscores `_`,
* hyphens `-`,
* changes in capitalization from lowercase to uppercase `aA`,
* adjacent digits and letters `a1`, `1a`, `A1`, `1A`,
* and acroynms `AAa` (as in `HTTPRequest`).

For more accuracy, the `from_case` method splits based on the word boundaries
of a particular case.  For example, splitting from snake case will only use
underscores as word boundaries.
```
use convert_case::{Case, Casing};

assert_eq!(
    "2020 04 16 My Cat Cali",
    "2020-04-16_my_cat_cali".to_case(Case::Title)
);
assert_eq!(
    "2020-04-16 My Cat Cali",
    "2020-04-16_my_cat_cali".from_case(Case::Snake).to_case(Case::Title)
);
```

Case conversion can detect acronyms for camel-like strings.  It also ignores any leading, 
trailing, or duplicate delimiters.
```
use convert_case::{Case, Casing};

assert_eq!("io_stream", "IOStream".to_case(Case::Snake));
assert_eq!("my_json_parser", "myJSONParser".to_case(Case::Snake));

assert_eq!("weird_var_name", "__weird--var _name-".to_case(Case::Snake));
```

It also works non-ascii characters.  However, no inferences on the language itself is made.
For instance, the digraph `ij` in Dutch will not be capitalized, because it is represented
as two distinct Unicode characters.  However, `æ` would be capitalized.  Accuracy with unicode
characters is done using the `unicode-segmentation` crate, the sole dependency of this crate.
```
use convert_case::{Case, Casing};

assert_eq!("granat-äpfel", "GranatÄpfel".to_case(Case::Kebab));
assert_eq!("Перспектива 24", "ПЕРСПЕКТИВА24".to_case(Case::Title));

// The example from str::to_lowercase documentation
let odysseus = "ὈΔΥΣΣΕΎΣ";
assert_eq!("ὀδυσσεύς", odysseus.to_case(Case::Lower));
```

By default, characters followed by digits and vice-versa are
considered word boundaries.  In addition, any special ASCII characters (besides `_` and `-`)
are ignored.
```
use convert_case::{Case, Casing};

assert_eq!("e_5150", "E5150".to_case(Case::Snake));
assert_eq!("10,000_days", "10,000Days".to_case(Case::Snake));
assert_eq!("HELLO, WORLD!", "Hello, world!".to_case(Case::Upper));
assert_eq!("One\ntwo\nthree", "ONE\nTWO\nTHREE".to_case(Case::Title));
```

You can also test what case a string is in.
```
use convert_case::{Case, Casing};

assert!( "css-class-name".is_case(Case::Kebab));
assert!(!"css-class-name".is_case(Case::Snake));
assert!(!"UPPER_CASE_VAR".is_case(Case::Snake));
```

# Note on Accuracy

The `Casing` methods `from_case` and `to_case` do not fail.  Conversion to a case will always
succeed.  However, the results can still be unexpected.  Failure to detect any word boundaries
for a particular case means the entire string will be considered a single word.
```
use convert_case::{Case, Casing};

// Mistakenly parsing using Case::Snake
assert_eq!("My-kebab-var", "my-kebab-var".from_case(Case::Snake).to_case(Case::Title));

// Converts using an unexpected method
assert_eq!("my_kebab_like_variable", "myKebab-like-variable".to_case(Case::Snake));
```

# Boundary Specificity

It can be difficult to determine how to split a string into words.  That is why this case
provides the [`from_case`](Casing::from_case) functionality, but sometimes that isn't enough
to meet a specific use case.

Take an identifier has the word `2D`, such as `scale2D`.  No exclusive usage of `from_case` will
be enough to solve the problem.  In this case we can further specify which boundaries to split
the string on.  `convert_case` provides some patterns for achieving this specificity.
We can specify what boundaries we want to split on using the [`Boundary` enum](Boundary).
```
use convert_case::{Boundary, Case, Casing};

// Not quite what we want
assert_eq!(
    "scale_2_d",
    "scale2D"
        .from_case(Case::Camel)
        .to_case(Case::Snake)
);

// Remove boundary from Case::Camel
assert_eq!(
    "scale_2d",
    "scale2D"
        .from_case(Case::Camel)
        .without_boundaries(&[Boundary::DigitUpper, Boundary::DigitLower])
        .to_case(Case::Snake)
);

// Write boundaries explicitly
assert_eq!(
    "scale_2d",
    "scale2D"
        .with_boundaries(&[Boundary::LowerDigit])
        .to_case(Case::Snake)
);
```

The `Casing` trait provides initial methods, but any subsequent methods that do not resolve
the conversion return a [`StateConverter`] struct.  It contains similar methods as `Casing`.

# Custom Cases

Because `Case` is an enum, you can't create your own variant for your use case.  However
the parameters for case conversion have been encapsulated into the [`Converter`] struct
which can be used for specific use cases.

Suppose you wanted to format a word like camel case, where the first word is lower case and the
rest are capitalized.  But you want to include a delimeter like underscore.  This case isn't
available as a `Case` variant, but you can create it by constructing the parameters of the
`Converter`.
```
use convert_case::{Case, Casing, Converter, Pattern};

let conv = Converter::new()
    .set_pattern(Pattern::Camel)
    .set_delim("_");

assert_eq!(
    "my_Special_Case",
    conv.convert("My Special Case")
)
```
Just as with the `Casing` trait, you can also manually set the boundaries strings are split 
on.  You can use any of the [`Pattern`] variants available.  This even includes [`Pattern::Sentence`]
which isn't used in any `Case` variant.  You can also set no pattern at all, which will
maintain the casing of each letter in the input string.  You can also, of course, set any string as your
delimeter.

For more details on how strings are converted, see the docs for [`Converter`].

# Random Feature

To ensure this library had zero dependencies, randomness was moved to the _random_ feature,
which requires the `rand` crate. You can enable this feature by including the
following in your `Cargo.toml`.
```{toml}
[dependencies]
convert_case = { version = "^0.3.0", features = ["random"] }
```
This will add two additional cases: Random and PseudoRandom.  You can read about their
construction in the [Case enum](enum.Case.html).

## Types

### Struct `StateConverter`

Holds information about parsing before converting into a case.

This struct is used when invoking the `from_case` and `with_boundaries` methods on
`Casing`.  For a more fine grained approach to case conversion, consider using the [`Converter`]
struct.
```
use convert_case::{Case, Casing};

let title = "ninety-nine_problems".from_case(Case::Snake).to_case(Case::Title);
assert_eq!("Ninety-nine Problems", title);
```

```rust
pub struct StateConverter<''a, T: AsRef<str>> {
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
  pub fn from_case(self: Self, case: Case) -> Self { /* ... */ }
  ```
  Uses the boundaries associated with `case` for word segmentation.  This

- ```rust
  pub fn with_boundaries(self: Self, bs: &[Boundary]) -> Self { /* ... */ }
  ```
  Overwrites boundaries for word segmentation with those provided.  This will overwrite

- ```rust
  pub fn without_boundaries(self: Self, bs: &[Boundary]) -> Self { /* ... */ }
  ```
  Removes any boundaries that were already initialized.  This is particularly useful when a

- ```rust
  pub fn to_case(self: Self, case: Case) -> String { /* ... */ }
  ```
  Consumes the `StateConverter` and returns the converted string.

##### Trait Implementations

- **Freeze**
- **Unpin**
- **UnwindSafe**
- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Traits

### Trait `Casing`

Describes items that can be converted into a case.  This trait is used
in conjunction with the [`StateConverter`] struct which is returned from a couple
methods on `Casing`.

Implemented for strings `&str`, `String`, and `&String`.

```rust
pub trait Casing<T: AsRef<str>> {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `to_case`: Convert the string into the given case.  It will reference `self` and create a new
- `from_case`: Start the case conversion by storing the boundaries associated with the given case.
- `with_boundaries`: Creates a `StateConverter` struct initialized with the boundaries
- `is_case`: Determines if `self` is of the given case.  This is done simply by applying

#### Implementations

This trait is implemented for the following types:

- `T` with <T: AsRef<str>>

## Re-exports

### Re-export `Case`

```rust
pub use case::Case;
```

### Re-export `Converter`

```rust
pub use converter::Converter;
```

### Re-export `Pattern`

```rust
pub use pattern::Pattern;
```

### Re-export `Boundary`

```rust
pub use segmentation::Boundary;
```

