# Crate Documentation

**Version:** 2.1.3

**Format Version:** 43

# Module `integer_encoding`

Fast serialization of integers.

This crate implements encoding and decoding of integer types to and from `FixedInt` (i.e. a
representation of integers similar or equal to how they are stored in memory) as well as
`VarInt` (encoding integers so that they only use as much memory as needed to represent their
magnitude).

This is useful when (de)serializing data from and to binary representations. For example,
Protocol Buffers (by Google) use these kinds of encoding.

```
use integer_encoding::*;

fn main() {
    let a: u32 = 344;
    let encoded_byte_slice = a.encode_fixed_light();
    assert_eq!(a, u32::decode_fixed(encoded_byte_slice));
    assert_eq!(4, encoded_byte_slice.len());

    let b: i32 = -111;
    let encoded_byte_vec = b.encode_var_vec();
    assert_eq!(Some((b, 2)), i32::decode_var(&encoded_byte_vec));
}
```

## Re-exports

### Re-export `FixedInt`

```rust
pub use fixed::FixedInt;
```

### Re-export `VarInt`

```rust
pub use varint::VarInt;
```

### Re-export `FixedIntReader`

```rust
pub use reader::FixedIntReader;
```

### Re-export `VarIntReader`

```rust
pub use reader::VarIntReader;
```

### Re-export `FixedIntWriter`

```rust
pub use writer::FixedIntWriter;
```

### Re-export `VarIntWriter`

```rust
pub use writer::VarIntWriter;
```

