# Crate Documentation

**Version:** 0.15.11

**Format Version:** 43

# Module `console`

console is a library for Rust that provides access to various terminal
features so you can build nicer looking command line interfaces.  It
comes with various tools and utilities for working with Terminals and
formatting text.

Best paired with other libraries in the family:

* [dialoguer](https://docs.rs/dialoguer)
* [indicatif](https://docs.rs/indicatif)

# Terminal Access

The terminal is abstracted through the `console::Term` type.  It can
either directly provide access to the connected terminal or by buffering
up commands.  A buffered terminal will however not be completely buffered
on windows where cursor movements are currently directly passed through.

Example usage:

```
# fn test() -> Result<(), Box<dyn std::error::Error>> {
use std::thread;
use std::time::Duration;

use console::Term;

let term = Term::stdout();
term.write_line("Hello World!")?;
thread::sleep(Duration::from_millis(2000));
term.clear_line()?;
# Ok(()) } test().unwrap();
```

# Colors and Styles

`console` automatically detects when to use colors based on the tty flag.  It also
provides higher level wrappers for styling text and other things that can be
displayed with the `style` function and utility types.

Example usage:

```
use console::style;

println!("This is {} neat", style("quite").cyan());
```

You can also store styles and apply them to text later:

```
use console::Style;

let cyan = Style::new().cyan();
println!("This is {} neat", cyan.apply_to("quite"));
```

# Working with ANSI Codes

The crate provides the function `strip_ansi_codes` to remove ANSI codes
from a string as well as `measure_text_width` to calculate the width of a
string as it would be displayed by the terminal.  Both of those together
are useful for more complex formatting.

# Unicode Width Support

By default this crate depends on the `unicode-width` crate to calculate
the width of terminal characters.  If you do not need this you can disable
the `unicode-width` feature which will cut down on dependencies.

# Features

By default all features are enabled.  The following features exist:

* `unicode-width`: adds support for unicode width calculations
* `ansi-parsing`: adds support for parsing ansi codes (this adds support
  for stripping and taking ansi escape codes into account for length
  calculations).

## Re-exports

### Re-export `Key`

```rust
pub use crate::kb::Key;
```

### Re-export `user_attended`

```rust
pub use crate::term::user_attended;
```

### Re-export `user_attended_stderr`

```rust
pub use crate::term::user_attended_stderr;
```

### Re-export `Term`

```rust
pub use crate::term::Term;
```

### Re-export `TermFamily`

```rust
pub use crate::term::TermFamily;
```

### Re-export `TermFeatures`

```rust
pub use crate::term::TermFeatures;
```

### Re-export `TermTarget`

```rust
pub use crate::term::TermTarget;
```

### Re-export `colors_enabled`

```rust
pub use crate::utils::colors_enabled;
```

### Re-export `colors_enabled_stderr`

```rust
pub use crate::utils::colors_enabled_stderr;
```

### Re-export `measure_text_width`

```rust
pub use crate::utils::measure_text_width;
```

### Re-export `pad_str`

```rust
pub use crate::utils::pad_str;
```

### Re-export `pad_str_with`

```rust
pub use crate::utils::pad_str_with;
```

### Re-export `set_colors_enabled`

```rust
pub use crate::utils::set_colors_enabled;
```

### Re-export `set_colors_enabled_stderr`

```rust
pub use crate::utils::set_colors_enabled_stderr;
```

### Re-export `style`

```rust
pub use crate::utils::style;
```

### Re-export `truncate_str`

```rust
pub use crate::utils::truncate_str;
```

### Re-export `Alignment`

```rust
pub use crate::utils::Alignment;
```

### Re-export `Attribute`

```rust
pub use crate::utils::Attribute;
```

### Re-export `Color`

```rust
pub use crate::utils::Color;
```

### Re-export `Emoji`

```rust
pub use crate::utils::Emoji;
```

### Re-export `Style`

```rust
pub use crate::utils::Style;
```

### Re-export `StyledObject`

```rust
pub use crate::utils::StyledObject;
```

### Re-export `strip_ansi_codes`

**Attributes:**

- `#[<cfg>(feature = "ansi-parsing")]`

```rust
pub use crate::ansi::strip_ansi_codes;
```

### Re-export `AnsiCodeIterator`

**Attributes:**

- `#[<cfg>(feature = "ansi-parsing")]`

```rust
pub use crate::ansi::AnsiCodeIterator;
```

