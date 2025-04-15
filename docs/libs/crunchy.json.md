# Crate Documentation

**Version:** 0.2.3

**Format Version:** 43

# Module `crunchy`

The crunchy unroller - deterministically unroll constant loops. For number "crunching".

The Rust optimizer will unroll constant loops that don't use the loop variable, like this:

```ignore
for _ in 0..100 {
    println!("Hello!");
}
```

However, using the loop variable will cause it to never unroll the loop. This is unfortunate because it means that you can't
constant-fold the loop variable, and if you end up stomping on the registers it will have to do a load for each iteration.
This crate ensures that your code is unrolled and const-folded. It only works on literals,
unfortunately, but there's a work-around:

```ignore
debug_assert_eq!(MY_CONSTANT, 100);
unroll! {
    for i in 0..100 {
        println!("Iteration {}", i);
    }
}
```
This means that your tests will catch if you redefine the constant.

To default maximum number of loops to unroll is `64`, but that can be easily increased using the cargo features:

* `limit_128`
* `limit_256`
* `limit_512`
* `limit_1024`
* `limit_2048`

## Macros

### Macro `unroll`

**Attributes:**

- `#[macro_export]`

Unroll the given for loop

Example:

```ignore
unroll! {
  for i in 0..5 {
    println!("Iteration {}", i);
  }
}
```

will expand into:

```ignore
{ println!("Iteration {}", 0); }
{ println!("Iteration {}", 1); }
{ println!("Iteration {}", 2); }
{ println!("Iteration {}", 3); }
{ println!("Iteration {}", 4); }
```

```rust
pub macro_rules! unroll {
    /* macro_rules! unroll {
    (for $v:ident in 0..0 $c:block) => { ... };
    (for $v:ident < $max:tt in ($start:tt..$end:tt).step_by($val:expr) {$($c:tt)*}) => { ... };
    (for $v:ident in ($start:tt..$end:tt).step_by($val:expr) {$($c:tt)*}) => { ... };
    (for $v:ident in ($start:tt..$end:tt) {$($c:tt)*}) => { ... };
    (for $v:ident in $start:tt..$end:tt {$($c:tt)*}) => { ... };
    (for $v:ident < $max:tt in $start:tt..$end:tt $c:block) => { ... };
    (for $v:ident in 0..$end:tt {$($statement:tt)*}) => { ... };
    (@$v:ident, $a:expr, 0, $c:block) => { ... };
    (@$v:ident, $a:expr, 1, $c:block) => { ... };
    (@$v:ident, $a:expr, 2, $c:block) => { ... };
    (@$v:ident, $a:expr, 3, $c:block) => { ... };
    (@$v:ident, $a:expr, 4, $c:block) => { ... };
    (@$v:ident, $a:expr, 5, $c:block) => { ... };
    (@$v:ident, $a:expr, 6, $c:block) => { ... };
    (@$v:ident, $a:expr, 7, $c:block) => { ... };
    (@$v:ident, $a:expr, 8, $c:block) => { ... };
    (@$v:ident, $a:expr, 9, $c:block) => { ... };
    (@$v:ident, $a:expr, 10, $c:block) => { ... };
    (@$v:ident, $a:expr, 11, $c:block) => { ... };
    (@$v:ident, $a:expr, 12, $c:block) => { ... };
    (@$v:ident, $a:expr, 13, $c:block) => { ... };
    (@$v:ident, $a:expr, 14, $c:block) => { ... };
    (@$v:ident, $a:expr, 15, $c:block) => { ... };
    (@$v:ident, $a:expr, 16, $c:block) => { ... };
    (@$v:ident, $a:expr, 17, $c:block) => { ... };
    (@$v:ident, $a:expr, 18, $c:block) => { ... };
    (@$v:ident, $a:expr, 19, $c:block) => { ... };
    (@$v:ident, $a:expr, 20, $c:block) => { ... };
    (@$v:ident, $a:expr, 21, $c:block) => { ... };
    (@$v:ident, $a:expr, 22, $c:block) => { ... };
    (@$v:ident, $a:expr, 23, $c:block) => { ... };
    (@$v:ident, $a:expr, 24, $c:block) => { ... };
    (@$v:ident, $a:expr, 25, $c:block) => { ... };
    (@$v:ident, $a:expr, 26, $c:block) => { ... };
    (@$v:ident, $a:expr, 27, $c:block) => { ... };
    (@$v:ident, $a:expr, 28, $c:block) => { ... };
    (@$v:ident, $a:expr, 29, $c:block) => { ... };
    (@$v:ident, $a:expr, 30, $c:block) => { ... };
    (@$v:ident, $a:expr, 31, $c:block) => { ... };
    (@$v:ident, $a:expr, 32, $c:block) => { ... };
    (@$v:ident, $a:expr, 33, $c:block) => { ... };
    (@$v:ident, $a:expr, 34, $c:block) => { ... };
    (@$v:ident, $a:expr, 35, $c:block) => { ... };
    (@$v:ident, $a:expr, 36, $c:block) => { ... };
    (@$v:ident, $a:expr, 37, $c:block) => { ... };
    (@$v:ident, $a:expr, 38, $c:block) => { ... };
    (@$v:ident, $a:expr, 39, $c:block) => { ... };
    (@$v:ident, $a:expr, 40, $c:block) => { ... };
    (@$v:ident, $a:expr, 41, $c:block) => { ... };
    (@$v:ident, $a:expr, 42, $c:block) => { ... };
    (@$v:ident, $a:expr, 43, $c:block) => { ... };
    (@$v:ident, $a:expr, 44, $c:block) => { ... };
    (@$v:ident, $a:expr, 45, $c:block) => { ... };
    (@$v:ident, $a:expr, 46, $c:block) => { ... };
    (@$v:ident, $a:expr, 47, $c:block) => { ... };
    (@$v:ident, $a:expr, 48, $c:block) => { ... };
    (@$v:ident, $a:expr, 49, $c:block) => { ... };
    (@$v:ident, $a:expr, 50, $c:block) => { ... };
    (@$v:ident, $a:expr, 51, $c:block) => { ... };
    (@$v:ident, $a:expr, 52, $c:block) => { ... };
    (@$v:ident, $a:expr, 53, $c:block) => { ... };
    (@$v:ident, $a:expr, 54, $c:block) => { ... };
    (@$v:ident, $a:expr, 55, $c:block) => { ... };
    (@$v:ident, $a:expr, 56, $c:block) => { ... };
    (@$v:ident, $a:expr, 57, $c:block) => { ... };
    (@$v:ident, $a:expr, 58, $c:block) => { ... };
    (@$v:ident, $a:expr, 59, $c:block) => { ... };
    (@$v:ident, $a:expr, 60, $c:block) => { ... };
    (@$v:ident, $a:expr, 61, $c:block) => { ... };
    (@$v:ident, $a:expr, 62, $c:block) => { ... };
    (@$v:ident, $a:expr, 63, $c:block) => { ... };
    (@$v:ident, $a:expr, 64, $c:block) => { ... };
    (@$v:ident, $a:expr, 65, $c:block) => { ... };
    (@$v:ident, $a:expr, 66, $c:block) => { ... };
    (@$v:ident, $a:expr, 67, $c:block) => { ... };
    (@$v:ident, $a:expr, 68, $c:block) => { ... };
    (@$v:ident, $a:expr, 69, $c:block) => { ... };
    (@$v:ident, $a:expr, 70, $c:block) => { ... };
    (@$v:ident, $a:expr, 71, $c:block) => { ... };
    (@$v:ident, $a:expr, 72, $c:block) => { ... };
    (@$v:ident, $a:expr, 73, $c:block) => { ... };
    (@$v:ident, $a:expr, 74, $c:block) => { ... };
    (@$v:ident, $a:expr, 75, $c:block) => { ... };
    (@$v:ident, $a:expr, 76, $c:block) => { ... };
    (@$v:ident, $a:expr, 77, $c:block) => { ... };
    (@$v:ident, $a:expr, 78, $c:block) => { ... };
    (@$v:ident, $a:expr, 79, $c:block) => { ... };
    (@$v:ident, $a:expr, 80, $c:block) => { ... };
    (@$v:ident, $a:expr, 81, $c:block) => { ... };
    (@$v:ident, $a:expr, 82, $c:block) => { ... };
    (@$v:ident, $a:expr, 83, $c:block) => { ... };
    (@$v:ident, $a:expr, 84, $c:block) => { ... };
    (@$v:ident, $a:expr, 85, $c:block) => { ... };
    (@$v:ident, $a:expr, 86, $c:block) => { ... };
    (@$v:ident, $a:expr, 87, $c:block) => { ... };
    (@$v:ident, $a:expr, 88, $c:block) => { ... };
    (@$v:ident, $a:expr, 89, $c:block) => { ... };
    (@$v:ident, $a:expr, 90, $c:block) => { ... };
    (@$v:ident, $a:expr, 91, $c:block) => { ... };
    (@$v:ident, $a:expr, 92, $c:block) => { ... };
    (@$v:ident, $a:expr, 93, $c:block) => { ... };
    (@$v:ident, $a:expr, 94, $c:block) => { ... };
    (@$v:ident, $a:expr, 95, $c:block) => { ... };
    (@$v:ident, $a:expr, 96, $c:block) => { ... };
    (@$v:ident, $a:expr, 97, $c:block) => { ... };
    (@$v:ident, $a:expr, 98, $c:block) => { ... };
    (@$v:ident, $a:expr, 99, $c:block) => { ... };
    (@$v:ident, $a:expr, 100, $c:block) => { ... };
    (@$v:ident, $a:expr, 101, $c:block) => { ... };
    (@$v:ident, $a:expr, 102, $c:block) => { ... };
    (@$v:ident, $a:expr, 103, $c:block) => { ... };
    (@$v:ident, $a:expr, 104, $c:block) => { ... };
    (@$v:ident, $a:expr, 105, $c:block) => { ... };
    (@$v:ident, $a:expr, 106, $c:block) => { ... };
    (@$v:ident, $a:expr, 107, $c:block) => { ... };
    (@$v:ident, $a:expr, 108, $c:block) => { ... };
    (@$v:ident, $a:expr, 109, $c:block) => { ... };
    (@$v:ident, $a:expr, 110, $c:block) => { ... };
    (@$v:ident, $a:expr, 111, $c:block) => { ... };
    (@$v:ident, $a:expr, 112, $c:block) => { ... };
    (@$v:ident, $a:expr, 113, $c:block) => { ... };
    (@$v:ident, $a:expr, 114, $c:block) => { ... };
    (@$v:ident, $a:expr, 115, $c:block) => { ... };
    (@$v:ident, $a:expr, 116, $c:block) => { ... };
    (@$v:ident, $a:expr, 117, $c:block) => { ... };
    (@$v:ident, $a:expr, 118, $c:block) => { ... };
    (@$v:ident, $a:expr, 119, $c:block) => { ... };
    (@$v:ident, $a:expr, 120, $c:block) => { ... };
    (@$v:ident, $a:expr, 121, $c:block) => { ... };
    (@$v:ident, $a:expr, 122, $c:block) => { ... };
    (@$v:ident, $a:expr, 123, $c:block) => { ... };
    (@$v:ident, $a:expr, 124, $c:block) => { ... };
    (@$v:ident, $a:expr, 125, $c:block) => { ... };
    (@$v:ident, $a:expr, 126, $c:block) => { ... };
    (@$v:ident, $a:expr, 127, $c:block) => { ... };
    (@$v:ident, $a:expr, 128, $c:block) => { ... };
} */
}
```

