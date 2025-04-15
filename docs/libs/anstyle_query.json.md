# Crate Documentation

**Version:** 1.1.2

**Format Version:** 43

# Module `anstyle_query`

Low level terminal capability lookups

## Modules

## Module `windows`

Windows-specific style queries

```rust
pub mod windows { /* ... */ }
```

### Functions

#### Function `enable_ansi_colors`

Enable ANSI escape codes ([`ENABLE_VIRTUAL_TERMINAL_PROCESSING`](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#output-sequences))

For non-windows systems, returns `None`

```rust
pub fn enable_ansi_colors() -> Option<bool> { /* ... */ }
```

## Functions

### Function `clicolor`

**Attributes:**

- `#[inline]`

Check [CLICOLOR] status

- When `true`, ANSI colors are supported and should be used when the program isn't piped,
  similar to [`term_supports_color`]
- When `false`, don’t output ANSI color escape codes, similar to [`no_color`]

See also:
- [terminfo](https://crates.io/crates/terminfo) or [term](https://crates.io/crates/term) for
  checking termcaps
- [termbg](https://crates.io/crates/termbg) for detecting background color

[CLICOLOR]: https://bixense.com/clicolors/

```rust
pub fn clicolor() -> Option<bool> { /* ... */ }
```

### Function `clicolor_force`

**Attributes:**

- `#[inline]`

Check [CLICOLOR_FORCE] status

ANSI colors should be enabled no matter what.

[CLICOLOR_FORCE]: https://bixense.com/clicolors/

```rust
pub fn clicolor_force() -> bool { /* ... */ }
```

### Function `no_color`

**Attributes:**

- `#[inline]`

Check [NO_COLOR] status

When `true`, should prevent the addition of ANSI color.

User-level configuration files and per-instance command-line arguments should override
[NO_COLOR]. A user should be able to export `$NO_COLOR` in their shell configuration file as a
default, but configure a specific program in its configuration file to specifically enable
color.

[NO_COLOR]: https://no-color.org/

```rust
pub fn no_color() -> bool { /* ... */ }
```

### Function `term_supports_color`

**Attributes:**

- `#[inline]`

Check `TERM` for color support

```rust
pub fn term_supports_color() -> bool { /* ... */ }
```

### Function `term_supports_ansi_color`

**Attributes:**

- `#[inline]`

Check `TERM` for ANSI color support

On Windows, you might need to also check [`windows::enable_ansi_colors`] as ANSI color support
is opt-in, rather than assumed.

```rust
pub fn term_supports_ansi_color() -> bool { /* ... */ }
```

### Function `truecolor`

**Attributes:**

- `#[inline]`

Check [COLORTERM] for truecolor support

[COLORTERM]: https://github.com/termstandard/colors

```rust
pub fn truecolor() -> bool { /* ... */ }
```

### Function `is_ci`

**Attributes:**

- `#[inline]`

Report whether this is running in CI

CI is a common environment where, despite being piped, ansi color codes are supported

This is not as exhaustive as you'd find in a crate like `is_ci` but it should work in enough
cases.

```rust
pub fn is_ci() -> bool { /* ... */ }
```

