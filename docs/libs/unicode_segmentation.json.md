# Crate Documentation

**Version:** 1.12.0

**Format Version:** 43

# Module `unicode_segmentation`

Iterators which split strings on Grapheme Cluster, Word or Sentence boundaries, according
to the [Unicode Standard Annex #29](http://www.unicode.org/reports/tr29/) rules.

```rust
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s = "a̐éö̲\r\n";
    let g = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
    let b: &[_] = &["a̐", "é", "ö̲", "\r\n"];
    assert_eq!(g, b);

    let s = "The quick (\"brown\") fox can't jump 32.3 feet, right?";
    let w = s.unicode_words().collect::<Vec<&str>>();
    let b: &[_] = &["The", "quick", "brown", "fox", "can't", "jump", "32.3", "feet", "right"];
    assert_eq!(w, b);

    let s = "The quick (\"brown\")  fox";
    let w = s.split_word_bounds().collect::<Vec<&str>>();
    let b: &[_] = &["The", " ", "quick", " ", "(", "\"", "brown", "\"", ")", "  ", "fox"];
    assert_eq!(w, b);
}
```

# no_std

unicode-segmentation does not depend on libstd, so it can be used in crates
with the `#![no_std]` attribute.

# crates.io

You can use this package in your project by adding the following
to your `Cargo.toml`:

```toml
[dependencies]
unicode-segmentation = "1.9.0"
```

## Traits

### Trait `UnicodeSegmentation`

Methods for segmenting strings according to
[Unicode Standard Annex #29](http://www.unicode.org/reports/tr29/).

```rust
pub trait UnicodeSegmentation {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `graphemes`: Returns an iterator over the [grapheme clusters][graphemes] of `self`.
- `grapheme_indices`: Returns an iterator over the grapheme clusters of `self` and their
- `unicode_words`: Returns an iterator over the words of `self`, separated on
- `unicode_word_indices`: Returns an iterator over the words of `self`, separated on
- `split_word_bounds`: Returns an iterator over substrings of `self` separated on
- `split_word_bound_indices`: Returns an iterator over substrings of `self`, split on UAX#29 word boundaries,
- `unicode_sentences`: Returns an iterator over substrings of `self` separated on
- `split_sentence_bounds`: Returns an iterator over substrings of `self` separated on
- `split_sentence_bound_indices`: Returns an iterator over substrings of `self`, split on UAX#29 sentence boundaries,

#### Implementations

This trait is implemented for the following types:

- `str`

## Re-exports

### Re-export `GraphemeCursor`

```rust
pub use grapheme::GraphemeCursor;
```

### Re-export `GraphemeIncomplete`

```rust
pub use grapheme::GraphemeIncomplete;
```

### Re-export `GraphemeIndices`

```rust
pub use grapheme::GraphemeIndices;
```

### Re-export `Graphemes`

```rust
pub use grapheme::Graphemes;
```

### Re-export `USentenceBoundIndices`

```rust
pub use sentence::USentenceBoundIndices;
```

### Re-export `USentenceBounds`

```rust
pub use sentence::USentenceBounds;
```

### Re-export `UnicodeSentences`

```rust
pub use sentence::UnicodeSentences;
```

### Re-export `UNICODE_VERSION`

```rust
pub use tables::UNICODE_VERSION;
```

### Re-export `UWordBoundIndices`

```rust
pub use word::UWordBoundIndices;
```

### Re-export `UWordBounds`

```rust
pub use word::UWordBounds;
```

### Re-export `UnicodeWordIndices`

```rust
pub use word::UnicodeWordIndices;
```

### Re-export `UnicodeWords`

```rust
pub use word::UnicodeWords;
```

