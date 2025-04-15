# Crate Documentation

**Version:** 0.5.0

**Format Version:** 43

# Module `dirs_sys`

## Functions

### Function `is_absolute_path`

```rust
pub fn is_absolute_path(path: std::ffi::OsString) -> Option<std::path::PathBuf> { /* ... */ }
```

## Re-exports

### Re-export `home_dir`

**Attributes:**

- `#[<cfg>(all(unix, not(target_os = "redox")))]`

```rust
pub use self::target_unix_not_redox::home_dir;
```

### Re-export `user_dir`

**Attributes:**

- `#[<cfg>(all(unix, not(any(target_os = "macos", target_os = "ios"))))]`

```rust
pub use self::target_unix_not_mac::user_dir;
```

### Re-export `user_dirs`

**Attributes:**

- `#[<cfg>(all(unix, not(any(target_os = "macos", target_os = "ios"))))]`

```rust
pub use self::target_unix_not_mac::user_dirs;
```

