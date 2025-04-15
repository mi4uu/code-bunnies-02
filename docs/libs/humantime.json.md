# Crate Documentation

**Version:** 2.2.0

**Format Version:** 43

# Module `humantime`

Human-friendly time parser and formatter

Features:

* Parses durations in free form like `15days 2min 2s`
* Formats durations in similar form `2years 2min 12us`
* Parses and formats timestamp in `rfc3339` format: `2018-01-01T12:53:00Z`
* Parses timestamps in a weaker format: `2018-01-01 12:53:00`

Timestamp parsing/formatting is super-fast because format is basically
fixed.

See [humantime-serde] for serde integration (previous crate [serde-humantime] looks unmaintained).

[serde-humantime]: https://docs.rs/serde-humantime/0.1.1/serde_humantime/
[humantime-serde]: https://docs.rs/humantime-serde

## Re-exports

### Re-export `Rfc3339Timestamp`

```rust
pub use self::date::Rfc3339Timestamp;
```

### Re-export `format_rfc3339`

```rust
pub use self::date::format_rfc3339;
```

### Re-export `format_rfc3339_micros`

```rust
pub use self::date::format_rfc3339_micros;
```

### Re-export `format_rfc3339_millis`

```rust
pub use self::date::format_rfc3339_millis;
```

### Re-export `format_rfc3339_nanos`

```rust
pub use self::date::format_rfc3339_nanos;
```

### Re-export `format_rfc3339_seconds`

```rust
pub use self::date::format_rfc3339_seconds;
```

### Re-export `parse_rfc3339`

```rust
pub use self::date::parse_rfc3339;
```

### Re-export `parse_rfc3339_weak`

```rust
pub use self::date::parse_rfc3339_weak;
```

### Re-export `Error`

```rust
pub use self::date::Error as TimestampError;
```

### Re-export `format_duration`

```rust
pub use self::duration::format_duration;
```

### Re-export `FormattedDuration`

```rust
pub use self::duration::FormattedDuration;
```

### Re-export `parse_duration`

```rust
pub use self::duration::parse_duration;
```

### Re-export `Error`

```rust
pub use self::duration::Error as DurationError;
```

### Re-export `Duration`

```rust
pub use self::wrapper::Duration;
```

### Re-export `Timestamp`

```rust
pub use self::wrapper::Timestamp;
```

