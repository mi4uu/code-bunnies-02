# Crate Documentation

**Version:** 0.3.9

**Format Version:** 43

# Module `arrayref`

This package contains just four macros, which enable the creation
of array references to portions of arrays or slices (or things
that can be sliced).

# Examples

Here is a simple example of slicing and dicing a slice into array
references with these macros.  Here we implement a simple
little-endian conversion from bytes to `u16`, and demonstrate code
that uses `array_ref!` to extract an array reference from a larger
array.  Note that the documentation for each macro also has an
example of its use.

```
#[macro_use]
extern crate arrayref;

fn read_u16(bytes: &[u8; 2]) -> u16 {
     bytes[0] as u16 + ((bytes[1] as u16) << 8)
}
// ...
# fn main() {
let data = [0,1,2,3,4,0,6,7,8,9];
assert_eq!(256, read_u16(array_ref![data,0,2]));
assert_eq!(4, read_u16(array_ref![data,4,2]));
# }
```

## Macros

### Macro `array_ref`

**Attributes:**

- `#[macro_export]`

You can use `array_ref` to generate an array reference to a subset
of a sliceable bit of data (which could be an array, or a slice,
or a Vec).

**Panics** if the slice is out of bounds.

```
#[macro_use]
extern crate arrayref;

fn read_u16(bytes: &[u8; 2]) -> u16 {
     bytes[0] as u16 + ((bytes[1] as u16) << 8)
}
// ...
# fn main() {
let data = [0,1,2,3,4,0,6,7,8,9];
assert_eq!(256, read_u16(array_ref![data,0,2]));
assert_eq!(4, read_u16(array_ref![data,4,2]));
# }
```

```rust
pub macro_rules! array_ref {
    /* macro_rules! array_ref {
    ($arr:expr, $offset:expr, $len:expr) => { ... };
} */
}
```

### Macro `array_refs`

**Attributes:**

- `#[macro_export]`

You can use `array_refs` to generate a series of array references
to an input array reference.  The idea is if you want to break an
array into a series of contiguous and non-overlapping arrays.
`array_refs` is a bit funny in that it insists on slicing up the
*entire* array.  This is intentional, as I find it handy to make
me ensure that my sub-arrays add up to the entire array.  This
macro will *never* panic, since the sizes are all checked at
compile time.

Note that unlike `array_ref!`, `array_refs` *requires* that the
first argument be an array reference.  The following arguments are
the lengths of each subarray you wish a reference to.  The total
of these arguments *must* equal the size of the array itself.

```
#[macro_use]
extern crate arrayref;

fn read_u16(bytes: &[u8; 2]) -> u16 {
     bytes[0] as u16 + ((bytes[1] as u16) << 8)
}
// ...
# fn main() {
let data = [0,1,2,3,4,0,6,7];
let (a,b,c) = array_refs![&data,2,2,4];
assert_eq!(read_u16(a), 256);
assert_eq!(read_u16(b), 3*256+2);
assert_eq!(*c, [4,0,6,7]);
# }
```

```rust
pub macro_rules! array_refs {
    /* macro_rules! array_refs {
    ( $arr:expr, $( $pre:expr ),* ; .. ;  $( $post:expr ),* ) => { ... };
    ( $arr:expr, $( $len:expr ),* ) => { ... };
} */
}
```

### Macro `mut_array_refs`

**Attributes:**

- `#[macro_export]`

You can use `mut_array_refs` to generate a series of mutable array
references to an input mutable array reference.  The idea is if
you want to break an array into a series of contiguous and
non-overlapping mutable array references.  Like `array_refs!`,
`mut_array_refs!` is a bit funny in that it insists on slicing up
the *entire* array.  This is intentional, as I find it handy to
make me ensure that my sub-arrays add up to the entire array.
This macro will *never* panic, since the sizes are all checked at
compile time.

Note that unlike `array_mut_ref!`, `mut_array_refs` *requires*
that the first argument be a mutable array reference.  The
following arguments are the lengths of each subarray you wish a
reference to.  The total of these arguments *must* equal the size
of the array itself.  Also note that this macro allows you to take
out multiple mutable references to a single object, which is both
weird and powerful.

```
#[macro_use]
extern crate arrayref;

fn write_u16(bytes: &mut [u8; 2], num: u16) {
     bytes[0] = num as u8;
     bytes[1] = (num >> 8) as u8;
}
fn write_u32(bytes: &mut [u8; 4], num: u32) {
     bytes[0] = num as u8;
     bytes[1] = (num >> 8) as u8; // this is buggy to save space...
}
// ...
# fn main() {
let mut data = [0,1,2,3,4,0,6,7];
let (a,b,c) = mut_array_refs![&mut data,2,2,4];
// let's write out some nice prime numbers!
write_u16(a, 37);
write_u16(b, 73);
write_u32(c, 137); // approximate inverse of the fine structure constant!
# }
```

```rust
pub macro_rules! mut_array_refs {
    /* macro_rules! mut_array_refs {
    ( $arr:expr, $( $pre:expr ),* ; .. ;  $( $post:expr ),* ) => { ... };
    ( $arr:expr, $( $len:expr ),* ) => { ... };
} */
}
```

### Macro `array_mut_ref`

**Attributes:**

- `#[macro_export]`

You can use `array_mut_ref` to generate a mutable array reference
to a subset of a sliceable bit of data (which could be an array,
or a slice, or a Vec).

**Panics** if the slice is out of bounds.

```
#[macro_use]
extern crate arrayref;

fn write_u16(bytes: &mut [u8; 2], num: u16) {
     bytes[0] = num as u8;
     bytes[1] = (num >> 8) as u8;
}
// ...
# fn main() {
let mut data = [0,1,2,3,4,0,6,7,8,9];
write_u16(array_mut_ref![data,0,2], 1);
write_u16(array_mut_ref![data,2,2], 5);
assert_eq!(*array_ref![data,0,4], [1,0,5,0]);
*array_mut_ref![data,4,5] = [4,3,2,1,0];
assert_eq!(data, [1,0,5,0,4,3,2,1,0,9]);
# }
```

```rust
pub macro_rules! array_mut_ref {
    /* macro_rules! array_mut_ref {
    ($arr:expr, $offset:expr, $len:expr) => { ... };
} */
}
```

