# Crate Documentation

**Version:** 0.2.172

**Format Version:** 43

# Module `libc`

libc - Raw FFI bindings to platforms' system libraries

## Re-exports

### Re-export `c_void`

```rust
pub use core::ffi::c_void;
```

### Re-export `crate::primitives::*`

**Attributes:**

- `#[<cfg>(all(unix,
not(any(windows, target_os = "fuchsia", target_os = "switch",
target_os = "psp", target_os = "vxworks", target_os = "solid_asp3"))))]`

```rust
pub use crate::primitives::*;
```

### Re-export `crate::unix::*`

**Attributes:**

- `#[<cfg>(all(unix,
not(any(windows, target_os = "fuchsia", target_os = "switch",
target_os = "psp", target_os = "vxworks", target_os = "solid_asp3"))))]`

```rust
pub use crate::unix::*;
```

