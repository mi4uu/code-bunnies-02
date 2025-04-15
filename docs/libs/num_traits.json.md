# Crate Documentation

**Version:** 0.2.19

**Format Version:** 43

# Module `num_traits`

Numeric traits for generic mathematics

## Compatibility

The `num-traits` crate is tested for rustc 1.60 and greater.

## Modules

## Module `bounds`

```rust
pub mod bounds { /* ... */ }
```

### Traits

#### Trait `Bounded`

Numbers which have upper and lower bounds

```rust
pub trait Bounded {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `min_value`: Returns the smallest finite number this type can represent
- `max_value`: Returns the largest finite number this type can represent

##### Implementations

This trait is implemented for the following types:

- `usize`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `isize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `core::num::Wrapping<T>` with <T: Bounded>
- `f32`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)` with <A: Bounded, B: Bounded, C: Bounded, D: Bounded, E: Bounded, F: Bounded, G: Bounded, H: Bounded, I: Bounded, J: Bounded, K: Bounded, L: Bounded, M: Bounded, N: Bounded, O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)` with <B: Bounded, C: Bounded, D: Bounded, E: Bounded, F: Bounded, G: Bounded, H: Bounded, I: Bounded, J: Bounded, K: Bounded, L: Bounded, M: Bounded, N: Bounded, O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)` with <C: Bounded, D: Bounded, E: Bounded, F: Bounded, G: Bounded, H: Bounded, I: Bounded, J: Bounded, K: Bounded, L: Bounded, M: Bounded, N: Bounded, O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)` with <D: Bounded, E: Bounded, F: Bounded, G: Bounded, H: Bounded, I: Bounded, J: Bounded, K: Bounded, L: Bounded, M: Bounded, N: Bounded, O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)` with <E: Bounded, F: Bounded, G: Bounded, H: Bounded, I: Bounded, J: Bounded, K: Bounded, L: Bounded, M: Bounded, N: Bounded, O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)` with <F: Bounded, G: Bounded, H: Bounded, I: Bounded, J: Bounded, K: Bounded, L: Bounded, M: Bounded, N: Bounded, O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(G, H, I, J, K, L, M, N, O, P, Q, R, S, T)` with <G: Bounded, H: Bounded, I: Bounded, J: Bounded, K: Bounded, L: Bounded, M: Bounded, N: Bounded, O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(H, I, J, K, L, M, N, O, P, Q, R, S, T)` with <H: Bounded, I: Bounded, J: Bounded, K: Bounded, L: Bounded, M: Bounded, N: Bounded, O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(I, J, K, L, M, N, O, P, Q, R, S, T)` with <I: Bounded, J: Bounded, K: Bounded, L: Bounded, M: Bounded, N: Bounded, O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(J, K, L, M, N, O, P, Q, R, S, T)` with <J: Bounded, K: Bounded, L: Bounded, M: Bounded, N: Bounded, O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(K, L, M, N, O, P, Q, R, S, T)` with <K: Bounded, L: Bounded, M: Bounded, N: Bounded, O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(L, M, N, O, P, Q, R, S, T)` with <L: Bounded, M: Bounded, N: Bounded, O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(M, N, O, P, Q, R, S, T)` with <M: Bounded, N: Bounded, O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(N, O, P, Q, R, S, T)` with <N: Bounded, O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(O, P, Q, R, S, T)` with <O: Bounded, P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(P, Q, R, S, T)` with <P: Bounded, Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(Q, R, S, T)` with <Q: Bounded, R: Bounded, S: Bounded, T: Bounded>
- `(R, S, T)` with <R: Bounded, S: Bounded, T: Bounded>
- `(S, T)` with <S: Bounded, T: Bounded>
- `(T)` with <T: Bounded>
- `()`
- `f64`

#### Trait `LowerBounded`

Numbers which have lower bounds

```rust
pub trait LowerBounded {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `min_value`: Returns the smallest finite number this type can represent

##### Implementations

This trait is implemented for the following types:

- `T` with <T: Bounded>

#### Trait `UpperBounded`

Numbers which have upper bounds

```rust
pub trait UpperBounded {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `max_value`: Returns the largest finite number this type can represent

##### Implementations

This trait is implemented for the following types:

- `T` with <T: Bounded>

## Module `cast`

```rust
pub mod cast { /* ... */ }
```

### Traits

#### Trait `ToPrimitive`

A generic trait for converting a value to a number.

A value can be represented by the target type when it lies within
the range of scalars supported by the target type.
For example, a negative integer cannot be represented by an unsigned
integer type, and an `i64` with a very high magnitude might not be
convertible to an `i32`.
On the other hand, conversions with possible precision loss or truncation
are admitted, like an `f32` with a decimal part to an integer type, or
even a large `f64` saturating to `f32` infinity.

```rust
pub trait ToPrimitive {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `to_i64`: Converts the value of `self` to an `i64`. If the value cannot be
- `to_u64`: Converts the value of `self` to a `u64`. If the value cannot be

##### Provided Methods

- ```rust
  fn to_isize(self: &Self) -> Option<isize> { /* ... */ }
  ```
  Converts the value of `self` to an `isize`. If the value cannot be

- ```rust
  fn to_i8(self: &Self) -> Option<i8> { /* ... */ }
  ```
  Converts the value of `self` to an `i8`. If the value cannot be

- ```rust
  fn to_i16(self: &Self) -> Option<i16> { /* ... */ }
  ```
  Converts the value of `self` to an `i16`. If the value cannot be

- ```rust
  fn to_i32(self: &Self) -> Option<i32> { /* ... */ }
  ```
  Converts the value of `self` to an `i32`. If the value cannot be

- ```rust
  fn to_i128(self: &Self) -> Option<i128> { /* ... */ }
  ```
  Converts the value of `self` to an `i128`. If the value cannot be

- ```rust
  fn to_usize(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Converts the value of `self` to a `usize`. If the value cannot be

- ```rust
  fn to_u8(self: &Self) -> Option<u8> { /* ... */ }
  ```
  Converts the value of `self` to a `u8`. If the value cannot be

- ```rust
  fn to_u16(self: &Self) -> Option<u16> { /* ... */ }
  ```
  Converts the value of `self` to a `u16`. If the value cannot be

- ```rust
  fn to_u32(self: &Self) -> Option<u32> { /* ... */ }
  ```
  Converts the value of `self` to a `u32`. If the value cannot be

- ```rust
  fn to_u128(self: &Self) -> Option<u128> { /* ... */ }
  ```
  Converts the value of `self` to a `u128`. If the value cannot be

- ```rust
  fn to_f32(self: &Self) -> Option<f32> { /* ... */ }
  ```
  Converts the value of `self` to an `f32`. Overflows may map to positive

- ```rust
  fn to_f64(self: &Self) -> Option<f64> { /* ... */ }
  ```
  Converts the value of `self` to an `f64`. Overflows may map to positive

##### Implementations

This trait is implemented for the following types:

- `isize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `usize`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `f32`
- `f64`
- `core::num::Wrapping<T>` with <T: ToPrimitive>

#### Trait `FromPrimitive`

A generic trait for converting a number to a value.

A value can be represented by the target type when it lies within
the range of scalars supported by the target type.
For example, a negative integer cannot be represented by an unsigned
integer type, and an `i64` with a very high magnitude might not be
convertible to an `i32`.
On the other hand, conversions with possible precision loss or truncation
are admitted, like an `f32` with a decimal part to an integer type, or
even a large `f64` saturating to `f32` infinity.

```rust
pub trait FromPrimitive: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `from_i64`: Converts an `i64` to return an optional value of this type. If the
- `from_u64`: Converts an `u64` to return an optional value of this type. If the

##### Provided Methods

- ```rust
  fn from_isize(n: isize) -> Option<Self> { /* ... */ }
  ```
  Converts an `isize` to return an optional value of this type. If the

- ```rust
  fn from_i8(n: i8) -> Option<Self> { /* ... */ }
  ```
  Converts an `i8` to return an optional value of this type. If the

- ```rust
  fn from_i16(n: i16) -> Option<Self> { /* ... */ }
  ```
  Converts an `i16` to return an optional value of this type. If the

- ```rust
  fn from_i32(n: i32) -> Option<Self> { /* ... */ }
  ```
  Converts an `i32` to return an optional value of this type. If the

- ```rust
  fn from_i128(n: i128) -> Option<Self> { /* ... */ }
  ```
  Converts an `i128` to return an optional value of this type. If the

- ```rust
  fn from_usize(n: usize) -> Option<Self> { /* ... */ }
  ```
  Converts a `usize` to return an optional value of this type. If the

- ```rust
  fn from_u8(n: u8) -> Option<Self> { /* ... */ }
  ```
  Converts an `u8` to return an optional value of this type. If the

- ```rust
  fn from_u16(n: u16) -> Option<Self> { /* ... */ }
  ```
  Converts an `u16` to return an optional value of this type. If the

- ```rust
  fn from_u32(n: u32) -> Option<Self> { /* ... */ }
  ```
  Converts an `u32` to return an optional value of this type. If the

- ```rust
  fn from_u128(n: u128) -> Option<Self> { /* ... */ }
  ```
  Converts an `u128` to return an optional value of this type. If the

- ```rust
  fn from_f32(n: f32) -> Option<Self> { /* ... */ }
  ```
  Converts a `f32` to return an optional value of this type. If the

- ```rust
  fn from_f64(n: f64) -> Option<Self> { /* ... */ }
  ```
  Converts a `f64` to return an optional value of this type. If the

##### Implementations

This trait is implemented for the following types:

- `isize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `usize`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `f32`
- `f64`
- `core::num::Wrapping<T>` with <T: FromPrimitive>

#### Trait `NumCast`

An interface for casting between machine scalars.

```rust
pub trait NumCast: Sized + ToPrimitive {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `from`: Creates a number from another value that can be converted into

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `usize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `isize`
- `f32`
- `f64`
- `core::num::Wrapping<T>` with <T: NumCast>

#### Trait `AsPrimitive`

A generic interface for casting between machine scalars with the
`as` operator, which admits narrowing and precision loss.
Implementers of this trait `AsPrimitive` should behave like a primitive
numeric type (e.g. a newtype around another primitive), and the
intended conversion must never fail.

# Examples

```
# use num_traits::AsPrimitive;
let three: i32 = (3.14159265f32).as_();
assert_eq!(three, 3);
```

# Safety

**In Rust versions before 1.45.0**, some uses of the `as` operator were not entirely safe.
In particular, it was undefined behavior if
a truncated floating point value could not fit in the target integer
type ([#10184](https://github.com/rust-lang/rust/issues/10184)).

```ignore
# use num_traits::AsPrimitive;
let x: u8 = (1.04E+17).as_(); // UB
```


```rust
pub trait AsPrimitive<T>: ''static + Copy
where
    T: ''static + Copy {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `as_`: Convert a value to another, using the `as` operator.

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u8`
- `u8`
- `u8`
- `u8`
- `u8`
- `u8`
- `u8`
- `u8`
- `u8`
- `u8`
- `u8`
- `u8`
- `u8`
- `u8`
- `i8`
- `i8`
- `i8`
- `i8`
- `i8`
- `i8`
- `i8`
- `i8`
- `i8`
- `i8`
- `i8`
- `i8`
- `i8`
- `i8`
- `u16`
- `u16`
- `u16`
- `u16`
- `u16`
- `u16`
- `u16`
- `u16`
- `u16`
- `u16`
- `u16`
- `u16`
- `u16`
- `u16`
- `i16`
- `i16`
- `i16`
- `i16`
- `i16`
- `i16`
- `i16`
- `i16`
- `i16`
- `i16`
- `i16`
- `i16`
- `i16`
- `i16`
- `u32`
- `u32`
- `u32`
- `u32`
- `u32`
- `u32`
- `u32`
- `u32`
- `u32`
- `u32`
- `u32`
- `u32`
- `u32`
- `u32`
- `i32`
- `i32`
- `i32`
- `i32`
- `i32`
- `i32`
- `i32`
- `i32`
- `i32`
- `i32`
- `i32`
- `i32`
- `i32`
- `i32`
- `u64`
- `u64`
- `u64`
- `u64`
- `u64`
- `u64`
- `u64`
- `u64`
- `u64`
- `u64`
- `u64`
- `u64`
- `u64`
- `u64`
- `i64`
- `i64`
- `i64`
- `i64`
- `i64`
- `i64`
- `i64`
- `i64`
- `i64`
- `i64`
- `i64`
- `i64`
- `i64`
- `i64`
- `u128`
- `u128`
- `u128`
- `u128`
- `u128`
- `u128`
- `u128`
- `u128`
- `u128`
- `u128`
- `u128`
- `u128`
- `u128`
- `u128`
- `i128`
- `i128`
- `i128`
- `i128`
- `i128`
- `i128`
- `i128`
- `i128`
- `i128`
- `i128`
- `i128`
- `i128`
- `i128`
- `i128`
- `usize`
- `usize`
- `usize`
- `usize`
- `usize`
- `usize`
- `usize`
- `usize`
- `usize`
- `usize`
- `usize`
- `usize`
- `usize`
- `usize`
- `isize`
- `isize`
- `isize`
- `isize`
- `isize`
- `isize`
- `isize`
- `isize`
- `isize`
- `isize`
- `isize`
- `isize`
- `isize`
- `isize`
- `f32`
- `f32`
- `f32`
- `f32`
- `f32`
- `f32`
- `f32`
- `f32`
- `f32`
- `f32`
- `f32`
- `f32`
- `f32`
- `f32`
- `f64`
- `f64`
- `f64`
- `f64`
- `f64`
- `f64`
- `f64`
- `f64`
- `f64`
- `f64`
- `f64`
- `f64`
- `f64`
- `f64`
- `char`
- `char`
- `char`
- `char`
- `char`
- `char`
- `char`
- `char`
- `char`
- `char`
- `char`
- `char`
- `char`
- `bool`
- `bool`
- `bool`
- `bool`
- `bool`
- `bool`
- `bool`
- `bool`
- `bool`
- `bool`
- `bool`
- `bool`

### Functions

#### Function `cast`

**Attributes:**

- `#[inline]`

Cast from one machine scalar to another.

# Examples

```
# use num_traits as num;
let twenty: f32 = num::cast(0x14).unwrap();
assert_eq!(twenty, 20f32);
```


```rust
pub fn cast<T: NumCast, U: NumCast>(n: T) -> Option<U> { /* ... */ }
```

## Module `float`

```rust
pub mod float { /* ... */ }
```

### Traits

#### Trait `FloatCore`

Generic trait for floating point numbers that works with `no_std`.

This trait implements a subset of the `Float` trait.

```rust
pub trait FloatCore: Num + NumCast + Neg<Output = Self> + PartialOrd + Copy {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `infinity`: Returns positive infinity.
- `neg_infinity`: Returns negative infinity.
- `nan`: Returns NaN.
- `neg_zero`: Returns `-0.0`.
- `min_value`: Returns the smallest finite value that this type can represent.
- `min_positive_value`: Returns the smallest positive, normalized value that this type can represent.
- `epsilon`: Returns epsilon, a small positive value.
- `max_value`: Returns the largest finite value that this type can represent.
- `classify`: Returns the floating point category of the number. If only one property
- `to_degrees`: Converts to degrees, assuming the number is in radians.
- `to_radians`: Converts to radians, assuming the number is in degrees.
- `integer_decode`: Returns the mantissa, base 2 exponent, and sign as integers, respectively.

##### Provided Methods

- ```rust
  fn is_nan(self: Self) -> bool { /* ... */ }
  ```
  Returns `true` if the number is NaN.

- ```rust
  fn is_infinite(self: Self) -> bool { /* ... */ }
  ```
  Returns `true` if the number is infinite.

- ```rust
  fn is_finite(self: Self) -> bool { /* ... */ }
  ```
  Returns `true` if the number is neither infinite or NaN.

- ```rust
  fn is_normal(self: Self) -> bool { /* ... */ }
  ```
  Returns `true` if the number is neither zero, infinite, subnormal or NaN.

- ```rust
  fn is_subnormal(self: Self) -> bool { /* ... */ }
  ```
  Returns `true` if the number is [subnormal].

- ```rust
  fn floor(self: Self) -> Self { /* ... */ }
  ```
  Returns the largest integer less than or equal to a number.

- ```rust
  fn ceil(self: Self) -> Self { /* ... */ }
  ```
  Returns the smallest integer greater than or equal to a number.

- ```rust
  fn round(self: Self) -> Self { /* ... */ }
  ```
  Returns the nearest integer to a number. Round half-way cases away from `0.0`.

- ```rust
  fn trunc(self: Self) -> Self { /* ... */ }
  ```
  Return the integer part of a number.

- ```rust
  fn fract(self: Self) -> Self { /* ... */ }
  ```
  Returns the fractional part of a number.

- ```rust
  fn abs(self: Self) -> Self { /* ... */ }
  ```
  Computes the absolute value of `self`. Returns `FloatCore::nan()` if the

- ```rust
  fn signum(self: Self) -> Self { /* ... */ }
  ```
  Returns a number that represents the sign of `self`.

- ```rust
  fn is_sign_positive(self: Self) -> bool { /* ... */ }
  ```
  Returns `true` if `self` is positive, including `+0.0` and

- ```rust
  fn is_sign_negative(self: Self) -> bool { /* ... */ }
  ```
  Returns `true` if `self` is negative, including `-0.0` and

- ```rust
  fn min(self: Self, other: Self) -> Self { /* ... */ }
  ```
  Returns the minimum of the two numbers.

- ```rust
  fn max(self: Self, other: Self) -> Self { /* ... */ }
  ```
  Returns the maximum of the two numbers.

- ```rust
  fn clamp(self: Self, min: Self, max: Self) -> Self { /* ... */ }
  ```
  A value bounded by a minimum and a maximum

- ```rust
  fn recip(self: Self) -> Self { /* ... */ }
  ```
  Returns the reciprocal (multiplicative inverse) of the number.

- ```rust
  fn powi(self: Self, exp: i32) -> Self { /* ... */ }
  ```
  Raise a number to an integer power.

##### Implementations

This trait is implemented for the following types:

- `f32`
- `f64`

#### Trait `Float`

**Attributes:**

- `#[<cfg>(any(feature = "std", feature = "libm"))]`

Generic trait for floating point numbers

This trait is only available with the `std` feature, or with the `libm` feature otherwise.

```rust
pub trait Float: Num + Copy + NumCast + PartialOrd + Neg<Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `nan`: Returns the `NaN` value.
- `infinity`: Returns the infinite value.
- `neg_infinity`: Returns the negative infinite value.
- `neg_zero`: Returns `-0.0`.
- `min_value`: Returns the smallest finite value that this type can represent.
- `min_positive_value`: Returns the smallest positive, normalized value that this type can represent.
- `max_value`: Returns the largest finite value that this type can represent.
- `is_nan`: Returns `true` if this value is `NaN` and false otherwise.
- `is_infinite`: Returns `true` if this value is positive infinity or negative infinity and
- `is_finite`: Returns `true` if this number is neither infinite nor `NaN`.
- `is_normal`: Returns `true` if the number is neither zero, infinite,
- `classify`: Returns the floating point category of the number. If only one property
- `floor`: Returns the largest integer less than or equal to a number.
- `ceil`: Returns the smallest integer greater than or equal to a number.
- `round`: Returns the nearest integer to a number. Round half-way cases away from
- `trunc`: Return the integer part of a number.
- `fract`: Returns the fractional part of a number.
- `abs`: Computes the absolute value of `self`. Returns `Float::nan()` if the
- `signum`: Returns a number that represents the sign of `self`.
- `is_sign_positive`: Returns `true` if `self` is positive, including `+0.0`,
- `is_sign_negative`: Returns `true` if `self` is negative, including `-0.0`,
- `mul_add`: Fused multiply-add. Computes `(self * a) + b` with only one rounding
- `recip`: Take the reciprocal (inverse) of a number, `1/x`.
- `powi`: Raise a number to an integer power.
- `powf`: Raise a number to a floating point power.
- `sqrt`: Take the square root of a number.
- `exp`: Returns `e^(self)`, (the exponential function).
- `exp2`: Returns `2^(self)`.
- `ln`: Returns the natural logarithm of the number.
- `log`: Returns the logarithm of the number with respect to an arbitrary base.
- `log2`: Returns the base 2 logarithm of the number.
- `log10`: Returns the base 10 logarithm of the number.
- `max`: Returns the maximum of the two numbers.
- `min`: Returns the minimum of the two numbers.
- `abs_sub`: The positive difference of two numbers.
- `cbrt`: Take the cubic root of a number.
- `hypot`: Calculate the length of the hypotenuse of a right-angle triangle given
- `sin`: Computes the sine of a number (in radians).
- `cos`: Computes the cosine of a number (in radians).
- `tan`: Computes the tangent of a number (in radians).
- `asin`: Computes the arcsine of a number. Return value is in radians in
- `acos`: Computes the arccosine of a number. Return value is in radians in
- `atan`: Computes the arctangent of a number. Return value is in radians in the
- `atan2`: Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`).
- `sin_cos`: Simultaneously computes the sine and cosine of the number, `x`. Returns
- `exp_m1`: Returns `e^(self) - 1` in a way that is accurate even if the
- `ln_1p`: Returns `ln(1+n)` (natural logarithm) more accurately than if
- `sinh`: Hyperbolic sine function.
- `cosh`: Hyperbolic cosine function.
- `tanh`: Hyperbolic tangent function.
- `asinh`: Inverse hyperbolic sine function.
- `acosh`: Inverse hyperbolic cosine function.
- `atanh`: Inverse hyperbolic tangent function.
- `integer_decode`: Returns the mantissa, base 2 exponent, and sign as integers, respectively.

##### Provided Methods

- ```rust
  fn epsilon() -> Self { /* ... */ }
  ```
  Returns epsilon, a small positive value.

- ```rust
  fn is_subnormal(self: Self) -> bool { /* ... */ }
  ```
  Returns `true` if the number is [subnormal].

- ```rust
  fn to_degrees(self: Self) -> Self { /* ... */ }
  ```
  Converts radians to degrees.

- ```rust
  fn to_radians(self: Self) -> Self { /* ... */ }
  ```
  Converts degrees to radians.

- ```rust
  fn clamp(self: Self, min: Self, max: Self) -> Self { /* ... */ }
  ```
  Clamps a value between a min and max.

- ```rust
  fn copysign(self: Self, sign: Self) -> Self { /* ... */ }
  ```
  Returns a number composed of the magnitude of `self` and the sign of

##### Implementations

This trait is implemented for the following types:

- `f32`
- `f64`

#### Trait `FloatConst`

**Attributes:**

- `#[allow(non_snake_case)]`

```rust
pub trait FloatConst {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `E`: Return Euler’s number.
- `FRAC_1_PI`: Return `1.0 / π`.
- `FRAC_1_SQRT_2`: Return `1.0 / sqrt(2.0)`.
- `FRAC_2_PI`: Return `2.0 / π`.
- `FRAC_2_SQRT_PI`: Return `2.0 / sqrt(π)`.
- `FRAC_PI_2`: Return `π / 2.0`.
- `FRAC_PI_3`: Return `π / 3.0`.
- `FRAC_PI_4`: Return `π / 4.0`.
- `FRAC_PI_6`: Return `π / 6.0`.
- `FRAC_PI_8`: Return `π / 8.0`.
- `LN_10`: Return `ln(10.0)`.
- `LN_2`: Return `ln(2.0)`.
- `LOG10_E`: Return `log10(e)`.
- `LOG2_E`: Return `log2(e)`.
- `PI`: Return Archimedes’ constant `π`.
- `SQRT_2`: Return `sqrt(2.0)`.

##### Provided Methods

- ```rust
  fn TAU() -> Self
where
    Self: Sized + Add<Self, Output = Self> { /* ... */ }
  ```
  Return the full circle constant `τ`.

- ```rust
  fn LOG10_2() -> Self
where
    Self: Sized + Div<Self, Output = Self> { /* ... */ }
  ```
  Return `log10(2.0)`.

- ```rust
  fn LOG2_10() -> Self
where
    Self: Sized + Div<Self, Output = Self> { /* ... */ }
  ```
  Return `log2(10.0)`.

##### Implementations

This trait is implemented for the following types:

- `f32`
- `f64`

#### Trait `TotalOrder`

Trait for floating point numbers that provide an implementation
of the `totalOrder` predicate as defined in the IEEE 754 (2008 revision)
floating point standard.

```rust
pub trait TotalOrder {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `total_cmp`: Return the ordering between `self` and `other`.

##### Implementations

This trait is implemented for the following types:

- `f64`
- `f32`

## Module `identities`

```rust
pub mod identities { /* ... */ }
```

### Traits

#### Trait `Zero`

Defines an additive identity element for `Self`.

# Laws

```text
a + 0 = a       ∀ a ∈ Self
0 + a = a       ∀ a ∈ Self
```

```rust
pub trait Zero: Sized + Add<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `zero`: Returns the additive identity element of `Self`, `0`.
- `is_zero`: Returns `true` if `self` is equal to the additive identity.

##### Provided Methods

- ```rust
  fn set_zero(self: &mut Self) { /* ... */ }
  ```
  Sets `self` to the additive identity element of `Self`, `0`.

##### Implementations

This trait is implemented for the following types:

- `usize`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `isize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `f32`
- `f64`
- `core::num::Wrapping<T>` with <T: Zero>

#### Trait `ConstZero`

Defines an associated constant representing the additive identity element
for `Self`.

```rust
pub trait ConstZero: Zero {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Constants

- `ZERO`: The additive identity element of `Self`, `0`.

##### Implementations

This trait is implemented for the following types:

- `usize`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `isize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `f32`
- `f64`
- `core::num::Wrapping<T>` with <T: ConstZero>

#### Trait `One`

Defines a multiplicative identity element for `Self`.

# Laws

```text
a * 1 = a       ∀ a ∈ Self
1 * a = a       ∀ a ∈ Self
```

```rust
pub trait One: Sized + Mul<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `one`: Returns the multiplicative identity element of `Self`, `1`.

##### Provided Methods

- ```rust
  fn set_one(self: &mut Self) { /* ... */ }
  ```
  Sets `self` to the multiplicative identity element of `Self`, `1`.

- ```rust
  fn is_one(self: &Self) -> bool
where
    Self: PartialEq { /* ... */ }
  ```
  Returns `true` if `self` is equal to the multiplicative identity.

##### Implementations

This trait is implemented for the following types:

- `usize`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `isize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `f32`
- `f64`
- `core::num::Wrapping<T>` with <T: One>

#### Trait `ConstOne`

Defines an associated constant representing the multiplicative identity
element for `Self`.

```rust
pub trait ConstOne: One {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Constants

- `ONE`: The multiplicative identity element of `Self`, `1`.

##### Implementations

This trait is implemented for the following types:

- `usize`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `isize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `f32`
- `f64`
- `core::num::Wrapping<T>` with <T: ConstOne>

### Functions

#### Function `zero`

**Attributes:**

- `#[inline(always)]`

Returns the additive identity, `0`.

```rust
pub fn zero<T: Zero>() -> T { /* ... */ }
```

#### Function `one`

**Attributes:**

- `#[inline(always)]`

Returns the multiplicative identity, `1`.

```rust
pub fn one<T: One>() -> T { /* ... */ }
```

## Module `int`

```rust
pub mod int { /* ... */ }
```

### Traits

#### Trait `PrimInt`

Generic trait for primitive integers.

The `PrimInt` trait is an abstraction over the builtin primitive integer types (e.g., `u8`,
`u32`, `isize`, `i128`, ...). It inherits the basic numeric traits and extends them with
bitwise operators and non-wrapping arithmetic.

The trait explicitly inherits `Copy`, `Eq`, `Ord`, and `Sized`. The intention is that all
types implementing this trait behave like primitive types that are passed by value by default
and behave like builtin integers. Furthermore, the types are expected to expose the integer
value in binary representation and support bitwise operators. The standard bitwise operations
(e.g., bitwise-and, bitwise-or, right-shift, left-shift) are inherited and the trait extends
these with introspective queries (e.g., `PrimInt::count_ones()`, `PrimInt::leading_zeros()`),
bitwise combinators (e.g., `PrimInt::rotate_left()`), and endianness converters (e.g.,
`PrimInt::to_be()`).

All `PrimInt` types are expected to be fixed-width binary integers. The width can be queried
via `T::zero().count_zeros()`. The trait currently lacks a way to query the width at
compile-time.

While a default implementation for all builtin primitive integers is provided, the trait is in
no way restricted to these. Other integer types that fulfil the requirements are free to
implement the trait was well.

This trait and many of the method names originate in the unstable `core::num::Int` trait from
the rust standard library. The original trait was never stabilized and thus removed from the
standard library.

```rust
pub trait PrimInt: Sized + Copy + Num + NumCast + Bounded + PartialOrd + Ord + Eq + Not<Output = Self> + BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> + Shl<usize, Output = Self> + Shr<usize, Output = Self> + CheckedAdd<Output = Self> + CheckedSub<Output = Self> + CheckedMul<Output = Self> + CheckedDiv<Output = Self> + Saturating {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `count_ones`: Returns the number of ones in the binary representation of `self`.
- `count_zeros`: Returns the number of zeros in the binary representation of `self`.
- `leading_zeros`: Returns the number of leading zeros in the binary representation
- `trailing_zeros`: Returns the number of trailing zeros in the binary representation
- `rotate_left`: Shifts the bits to the left by a specified amount, `n`, wrapping
- `rotate_right`: Shifts the bits to the right by a specified amount, `n`, wrapping
- `signed_shl`: Shifts the bits to the left by a specified amount, `n`, filling
- `signed_shr`: Shifts the bits to the right by a specified amount, `n`, copying
- `unsigned_shl`: Shifts the bits to the left by a specified amount, `n`, filling
- `unsigned_shr`: Shifts the bits to the right by a specified amount, `n`, filling
- `swap_bytes`: Reverses the byte order of the integer.
- `from_be`: Convert an integer from big endian to the target's endianness.
- `from_le`: Convert an integer from little endian to the target's endianness.
- `to_be`: Convert `self` to big endian from the target's endianness.
- `to_le`: Convert `self` to little endian from the target's endianness.
- `pow`: Raises self to the power of `exp`, using exponentiation by squaring.

##### Provided Methods

- ```rust
  fn leading_ones(self: Self) -> u32 { /* ... */ }
  ```
  Returns the number of leading ones in the binary representation

- ```rust
  fn trailing_ones(self: Self) -> u32 { /* ... */ }
  ```
  Returns the number of trailing ones in the binary representation

- ```rust
  fn reverse_bits(self: Self) -> Self { /* ... */ }
  ```
  Reverses the order of bits in the integer.

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `usize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `isize`

## Module `ops`

```rust
pub mod ops { /* ... */ }
```

### Modules

## Module `bytes`

```rust
pub mod bytes { /* ... */ }
```

### Traits

#### Trait `NumBytes`

```rust
pub trait NumBytes: Debug + AsRef<[u8]> + AsMut<[u8]> + PartialEq + Eq + PartialOrd + Ord + Hash + Borrow<[u8]> + BorrowMut<[u8]> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `T` with <T>

#### Trait `ToBytes`

```rust
pub trait ToBytes {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Bytes`

###### Required Methods

- `to_be_bytes`: Return the memory representation of this number as a byte array in big-endian byte order.
- `to_le_bytes`: Return the memory representation of this number as a byte array in little-endian byte order.

##### Provided Methods

- ```rust
  fn to_ne_bytes(self: &Self) -> <Self as >::Bytes { /* ... */ }
  ```
  Return the memory representation of this number as a byte array in native byte order.

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `usize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `isize`
- `f32`
- `f64`

#### Trait `FromBytes`

```rust
pub trait FromBytes: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Bytes`

###### Required Methods

- `from_be_bytes`: Create a number from its representation as a byte array in big endian.
- `from_le_bytes`: Create a number from its representation as a byte array in little endian.

##### Provided Methods

- ```rust
  fn from_ne_bytes(bytes: &<Self as >::Bytes) -> Self { /* ... */ }
  ```
  Create a number from its memory representation as a byte array in native endianness.

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `usize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `isize`
- `f32`
- `f64`

## Module `checked`

```rust
pub mod checked { /* ... */ }
```

### Traits

#### Trait `CheckedAdd`

Performs addition that returns `None` instead of wrapping around on
overflow.

```rust
pub trait CheckedAdd: Sized + Add<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `checked_add`: Adds two numbers, checking for overflow. If overflow happens, `None` is

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

#### Trait `CheckedSub`

Performs subtraction that returns `None` instead of wrapping around on underflow.

```rust
pub trait CheckedSub: Sized + Sub<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `checked_sub`: Subtracts two numbers, checking for underflow. If underflow happens,

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

#### Trait `CheckedMul`

Performs multiplication that returns `None` instead of wrapping around on underflow or
overflow.

```rust
pub trait CheckedMul: Sized + Mul<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `checked_mul`: Multiplies two numbers, checking for underflow or overflow. If underflow

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

#### Trait `CheckedDiv`

Performs division that returns `None` instead of panicking on division by zero and instead of
wrapping around on underflow and overflow.

```rust
pub trait CheckedDiv: Sized + Div<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `checked_div`: Divides two numbers, checking for underflow, overflow and division by

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

#### Trait `CheckedRem`

Performs an integral remainder that returns `None` instead of panicking on division by zero and
instead of wrapping around on underflow and overflow.

```rust
pub trait CheckedRem: Sized + Rem<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `checked_rem`: Finds the remainder of dividing two numbers, checking for underflow, overflow and division

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

#### Trait `CheckedNeg`

Performs negation that returns `None` if the result can't be represented.

```rust
pub trait CheckedNeg: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `checked_neg`: Negates a number, returning `None` for results that can't be represented, like signed `MIN`

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

#### Trait `CheckedShl`

Performs a left shift that returns `None` on shifts larger than
or equal to the type width.

```rust
pub trait CheckedShl: Sized + Shl<u32, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `checked_shl`: Checked shift left. Computes `self << rhs`, returning `None`

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

#### Trait `CheckedShr`

Performs a right shift that returns `None` on shifts larger than
or equal to the type width.

```rust
pub trait CheckedShr: Sized + Shr<u32, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `checked_shr`: Checked shift right. Computes `self >> rhs`, returning `None`

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

## Module `euclid`

```rust
pub mod euclid { /* ... */ }
```

### Traits

#### Trait `Euclid`

```rust
pub trait Euclid: Sized + Div<Self, Output = Self> + Rem<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `div_euclid`: Calculates Euclidean division, the matching method for `rem_euclid`.
- `rem_euclid`: Calculates the least nonnegative remainder of `self (mod v)`.

##### Provided Methods

- ```rust
  fn div_rem_euclid(self: &Self, v: &Self) -> (Self, Self) { /* ... */ }
  ```
  Returns both the quotient and remainder from Euclidean division.

##### Implementations

This trait is implemented for the following types:

- `isize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `usize`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `f32`
- `f64`

#### Trait `CheckedEuclid`

```rust
pub trait CheckedEuclid: Euclid {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `checked_div_euclid`: Performs euclid division that returns `None` instead of panicking on division by zero
- `checked_rem_euclid`: Finds the euclid remainder of dividing two numbers, checking for underflow, overflow and

##### Provided Methods

- ```rust
  fn checked_div_rem_euclid(self: &Self, v: &Self) -> Option<(Self, Self)> { /* ... */ }
  ```
  Returns both the quotient and remainder from checked Euclidean division.

##### Implementations

This trait is implemented for the following types:

- `isize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `usize`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`

## Module `inv`

```rust
pub mod inv { /* ... */ }
```

### Traits

#### Trait `Inv`

Unary operator for retrieving the multiplicative inverse, or reciprocal, of a value.

```rust
pub trait Inv {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Output`: The result after applying the operator.

###### Required Methods

- `inv`: Returns the multiplicative inverse of `self`.

##### Implementations

This trait is implemented for the following types:

- `f32`
- `f64`
- `&''a f32` with <''a>
- `&''a f64` with <''a>

## Module `mul_add`

```rust
pub mod mul_add { /* ... */ }
```

### Traits

#### Trait `MulAdd`

Fused multiply-add. Computes `(self * a) + b` with only one rounding
error, yielding a more accurate result than an unfused multiply-add.

Using `mul_add` can be more performant than an unfused multiply-add if
the target architecture has a dedicated `fma` CPU instruction.

Note that `A` and `B` are `Self` by default, but this is not mandatory.

# Example

```
use std::f32;

let m = 10.0_f32;
let x = 4.0_f32;
let b = 60.0_f32;

// 100.0
let abs_difference = (m.mul_add(x, b) - (m*x + b)).abs();

assert!(abs_difference <= 100.0 * f32::EPSILON);
```

```rust
pub trait MulAdd<A = Self, B = Self> {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Output`: The resulting type after applying the fused multiply-add.

###### Required Methods

- `mul_add`: Performs the fused multiply-add operation `(self * a) + b`

##### Implementations

This trait is implemented for the following types:

- `f32`
- `f64`
- `isize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `usize`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`

#### Trait `MulAddAssign`

The fused multiply-add assignment operation `*self = (*self * a) + b`

```rust
pub trait MulAddAssign<A = Self, B = Self> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `mul_add_assign`: Performs the fused multiply-add assignment operation `*self = (*self * a) + b`

##### Implementations

This trait is implemented for the following types:

- `f32`
- `f64`
- `isize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `usize`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`

## Module `overflowing`

```rust
pub mod overflowing { /* ... */ }
```

### Traits

#### Trait `OverflowingAdd`

Performs addition with a flag for overflow.

```rust
pub trait OverflowingAdd: Sized + Add<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `overflowing_add`: Returns a tuple of the sum along with a boolean indicating whether an arithmetic overflow would occur.

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

#### Trait `OverflowingSub`

Performs substraction with a flag for overflow.

```rust
pub trait OverflowingSub: Sized + Sub<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `overflowing_sub`: Returns a tuple of the difference along with a boolean indicating whether an arithmetic overflow would occur.

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

#### Trait `OverflowingMul`

Performs multiplication with a flag for overflow.

```rust
pub trait OverflowingMul: Sized + Mul<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `overflowing_mul`: Returns a tuple of the product along with a boolean indicating whether an arithmetic overflow would occur.

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

## Module `saturating`

```rust
pub mod saturating { /* ... */ }
```

### Traits

#### Trait `Saturating`

Saturating math operations. Deprecated, use `SaturatingAdd`, `SaturatingSub` and
`SaturatingMul` instead.

```rust
pub trait Saturating {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `saturating_add`: Saturating addition operator.
- `saturating_sub`: Saturating subtraction operator.

##### Implementations

This trait is implemented for the following types:

- `isize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `usize`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`

#### Trait `SaturatingAdd`

Performs addition that saturates at the numeric bounds instead of overflowing.

```rust
pub trait SaturatingAdd: Sized + Add<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `saturating_add`: Saturating addition. Computes `self + other`, saturating at the relevant high or low boundary of

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

#### Trait `SaturatingSub`

Performs subtraction that saturates at the numeric bounds instead of overflowing.

```rust
pub trait SaturatingSub: Sized + Sub<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `saturating_sub`: Saturating subtraction. Computes `self - other`, saturating at the relevant high or low boundary of

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

#### Trait `SaturatingMul`

Performs multiplication that saturates at the numeric bounds instead of overflowing.

```rust
pub trait SaturatingMul: Sized + Mul<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `saturating_mul`: Saturating multiplication. Computes `self * other`, saturating at the relevant high or low boundary of

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

## Module `wrapping`

```rust
pub mod wrapping { /* ... */ }
```

### Traits

#### Trait `WrappingAdd`

Performs addition that wraps around on overflow.

```rust
pub trait WrappingAdd: Sized + Add<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `wrapping_add`: Wrapping (modular) addition. Computes `self + other`, wrapping around at the boundary of

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`
- `core::num::Wrapping<T>` with <T: WrappingAdd>

#### Trait `WrappingSub`

Performs subtraction that wraps around on overflow.

```rust
pub trait WrappingSub: Sized + Sub<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `wrapping_sub`: Wrapping (modular) subtraction. Computes `self - other`, wrapping around at the boundary

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`
- `core::num::Wrapping<T>` with <T: WrappingSub>

#### Trait `WrappingMul`

Performs multiplication that wraps around on overflow.

```rust
pub trait WrappingMul: Sized + Mul<Self, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `wrapping_mul`: Wrapping (modular) multiplication. Computes `self * other`, wrapping around at the boundary

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`
- `core::num::Wrapping<T>` with <T: WrappingMul>

#### Trait `WrappingNeg`

Performs a negation that does not panic.

```rust
pub trait WrappingNeg: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `wrapping_neg`: Wrapping (modular) negation. Computes `-self`,

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`
- `core::num::Wrapping<T>` with <T: WrappingNeg>

#### Trait `WrappingShl`

Performs a left shift that does not panic.

```rust
pub trait WrappingShl: Sized + Shl<usize, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `wrapping_shl`: Panic-free bitwise shift-left; yields `self << mask(rhs)`,

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`
- `core::num::Wrapping<T>` with <T: WrappingShl>

#### Trait `WrappingShr`

Performs a right shift that does not panic.

```rust
pub trait WrappingShr: Sized + Shr<usize, Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `wrapping_shr`: Panic-free bitwise shift-right; yields `self >> mask(rhs)`,

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`
- `core::num::Wrapping<T>` with <T: WrappingShr>

## Module `pow`

```rust
pub mod pow { /* ... */ }
```

### Traits

#### Trait `Pow`

Binary operator for raising a value to a power.

```rust
pub trait Pow<RHS> {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Output`: The result after applying the operator.

###### Required Methods

- `pow`: Returns `self` to the power `rhs`.

##### Implementations

This trait is implemented for the following types:

- `u8`
- `u8` with <''a>
- `&''a u8` with <''a>
- `&''b u8` with <''a, ''b>
- `u8`
- `u8` with <''a>
- `&''a u8` with <''a>
- `&''b u8` with <''a, ''b>
- `u8`
- `u8` with <''a>
- `&''a u8` with <''a>
- `&''b u8` with <''a, ''b>
- `u8`
- `u8` with <''a>
- `&''a u8` with <''a>
- `&''b u8` with <''a, ''b>
- `i8`
- `i8` with <''a>
- `&''a i8` with <''a>
- `&''b i8` with <''a, ''b>
- `i8`
- `i8` with <''a>
- `&''a i8` with <''a>
- `&''b i8` with <''a, ''b>
- `i8`
- `i8` with <''a>
- `&''a i8` with <''a>
- `&''b i8` with <''a, ''b>
- `i8`
- `i8` with <''a>
- `&''a i8` with <''a>
- `&''b i8` with <''a, ''b>
- `u16`
- `u16` with <''a>
- `&''a u16` with <''a>
- `&''b u16` with <''a, ''b>
- `u16`
- `u16` with <''a>
- `&''a u16` with <''a>
- `&''b u16` with <''a, ''b>
- `u16`
- `u16` with <''a>
- `&''a u16` with <''a>
- `&''b u16` with <''a, ''b>
- `u16`
- `u16` with <''a>
- `&''a u16` with <''a>
- `&''b u16` with <''a, ''b>
- `i16`
- `i16` with <''a>
- `&''a i16` with <''a>
- `&''b i16` with <''a, ''b>
- `i16`
- `i16` with <''a>
- `&''a i16` with <''a>
- `&''b i16` with <''a, ''b>
- `i16`
- `i16` with <''a>
- `&''a i16` with <''a>
- `&''b i16` with <''a, ''b>
- `i16`
- `i16` with <''a>
- `&''a i16` with <''a>
- `&''b i16` with <''a, ''b>
- `u32`
- `u32` with <''a>
- `&''a u32` with <''a>
- `&''b u32` with <''a, ''b>
- `u32`
- `u32` with <''a>
- `&''a u32` with <''a>
- `&''b u32` with <''a, ''b>
- `u32`
- `u32` with <''a>
- `&''a u32` with <''a>
- `&''b u32` with <''a, ''b>
- `u32`
- `u32` with <''a>
- `&''a u32` with <''a>
- `&''b u32` with <''a, ''b>
- `i32`
- `i32` with <''a>
- `&''a i32` with <''a>
- `&''b i32` with <''a, ''b>
- `i32`
- `i32` with <''a>
- `&''a i32` with <''a>
- `&''b i32` with <''a, ''b>
- `i32`
- `i32` with <''a>
- `&''a i32` with <''a>
- `&''b i32` with <''a, ''b>
- `i32`
- `i32` with <''a>
- `&''a i32` with <''a>
- `&''b i32` with <''a, ''b>
- `u64`
- `u64` with <''a>
- `&''a u64` with <''a>
- `&''b u64` with <''a, ''b>
- `u64`
- `u64` with <''a>
- `&''a u64` with <''a>
- `&''b u64` with <''a, ''b>
- `u64`
- `u64` with <''a>
- `&''a u64` with <''a>
- `&''b u64` with <''a, ''b>
- `u64`
- `u64` with <''a>
- `&''a u64` with <''a>
- `&''b u64` with <''a, ''b>
- `i64`
- `i64` with <''a>
- `&''a i64` with <''a>
- `&''b i64` with <''a, ''b>
- `i64`
- `i64` with <''a>
- `&''a i64` with <''a>
- `&''b i64` with <''a, ''b>
- `i64`
- `i64` with <''a>
- `&''a i64` with <''a>
- `&''b i64` with <''a, ''b>
- `i64`
- `i64` with <''a>
- `&''a i64` with <''a>
- `&''b i64` with <''a, ''b>
- `u128`
- `u128` with <''a>
- `&''a u128` with <''a>
- `&''b u128` with <''a, ''b>
- `u128`
- `u128` with <''a>
- `&''a u128` with <''a>
- `&''b u128` with <''a, ''b>
- `u128`
- `u128` with <''a>
- `&''a u128` with <''a>
- `&''b u128` with <''a, ''b>
- `u128`
- `u128` with <''a>
- `&''a u128` with <''a>
- `&''b u128` with <''a, ''b>
- `i128`
- `i128` with <''a>
- `&''a i128` with <''a>
- `&''b i128` with <''a, ''b>
- `i128`
- `i128` with <''a>
- `&''a i128` with <''a>
- `&''b i128` with <''a, ''b>
- `i128`
- `i128` with <''a>
- `&''a i128` with <''a>
- `&''b i128` with <''a, ''b>
- `i128`
- `i128` with <''a>
- `&''a i128` with <''a>
- `&''b i128` with <''a, ''b>
- `usize`
- `usize` with <''a>
- `&''a usize` with <''a>
- `&''b usize` with <''a, ''b>
- `usize`
- `usize` with <''a>
- `&''a usize` with <''a>
- `&''b usize` with <''a, ''b>
- `usize`
- `usize` with <''a>
- `&''a usize` with <''a>
- `&''b usize` with <''a, ''b>
- `usize`
- `usize` with <''a>
- `&''a usize` with <''a>
- `&''b usize` with <''a, ''b>
- `isize`
- `isize` with <''a>
- `&''a isize` with <''a>
- `&''b isize` with <''a, ''b>
- `isize`
- `isize` with <''a>
- `&''a isize` with <''a>
- `&''b isize` with <''a, ''b>
- `isize`
- `isize` with <''a>
- `&''a isize` with <''a>
- `&''b isize` with <''a, ''b>
- `isize`
- `isize` with <''a>
- `&''a isize` with <''a>
- `&''b isize` with <''a, ''b>
- `core::num::Wrapping<u8>`
- `core::num::Wrapping<u8>` with <''a>
- `&''a core::num::Wrapping<u8>` with <''a>
- `&''b core::num::Wrapping<u8>` with <''a, ''b>
- `core::num::Wrapping<u8>`
- `core::num::Wrapping<u8>` with <''a>
- `&''a core::num::Wrapping<u8>` with <''a>
- `&''b core::num::Wrapping<u8>` with <''a, ''b>
- `core::num::Wrapping<i8>`
- `core::num::Wrapping<i8>` with <''a>
- `&''a core::num::Wrapping<i8>` with <''a>
- `&''b core::num::Wrapping<i8>` with <''a, ''b>
- `core::num::Wrapping<i8>`
- `core::num::Wrapping<i8>` with <''a>
- `&''a core::num::Wrapping<i8>` with <''a>
- `&''b core::num::Wrapping<i8>` with <''a, ''b>
- `core::num::Wrapping<u16>`
- `core::num::Wrapping<u16>` with <''a>
- `&''a core::num::Wrapping<u16>` with <''a>
- `&''b core::num::Wrapping<u16>` with <''a, ''b>
- `core::num::Wrapping<u16>`
- `core::num::Wrapping<u16>` with <''a>
- `&''a core::num::Wrapping<u16>` with <''a>
- `&''b core::num::Wrapping<u16>` with <''a, ''b>
- `core::num::Wrapping<i16>`
- `core::num::Wrapping<i16>` with <''a>
- `&''a core::num::Wrapping<i16>` with <''a>
- `&''b core::num::Wrapping<i16>` with <''a, ''b>
- `core::num::Wrapping<i16>`
- `core::num::Wrapping<i16>` with <''a>
- `&''a core::num::Wrapping<i16>` with <''a>
- `&''b core::num::Wrapping<i16>` with <''a, ''b>
- `core::num::Wrapping<u32>`
- `core::num::Wrapping<u32>` with <''a>
- `&''a core::num::Wrapping<u32>` with <''a>
- `&''b core::num::Wrapping<u32>` with <''a, ''b>
- `core::num::Wrapping<u32>`
- `core::num::Wrapping<u32>` with <''a>
- `&''a core::num::Wrapping<u32>` with <''a>
- `&''b core::num::Wrapping<u32>` with <''a, ''b>
- `core::num::Wrapping<i32>`
- `core::num::Wrapping<i32>` with <''a>
- `&''a core::num::Wrapping<i32>` with <''a>
- `&''b core::num::Wrapping<i32>` with <''a, ''b>
- `core::num::Wrapping<i32>`
- `core::num::Wrapping<i32>` with <''a>
- `&''a core::num::Wrapping<i32>` with <''a>
- `&''b core::num::Wrapping<i32>` with <''a, ''b>
- `core::num::Wrapping<u64>`
- `core::num::Wrapping<u64>` with <''a>
- `&''a core::num::Wrapping<u64>` with <''a>
- `&''b core::num::Wrapping<u64>` with <''a, ''b>
- `core::num::Wrapping<u64>`
- `core::num::Wrapping<u64>` with <''a>
- `&''a core::num::Wrapping<u64>` with <''a>
- `&''b core::num::Wrapping<u64>` with <''a, ''b>
- `core::num::Wrapping<i64>`
- `core::num::Wrapping<i64>` with <''a>
- `&''a core::num::Wrapping<i64>` with <''a>
- `&''b core::num::Wrapping<i64>` with <''a, ''b>
- `core::num::Wrapping<i64>`
- `core::num::Wrapping<i64>` with <''a>
- `&''a core::num::Wrapping<i64>` with <''a>
- `&''b core::num::Wrapping<i64>` with <''a, ''b>
- `core::num::Wrapping<u128>`
- `core::num::Wrapping<u128>` with <''a>
- `&''a core::num::Wrapping<u128>` with <''a>
- `&''b core::num::Wrapping<u128>` with <''a, ''b>
- `core::num::Wrapping<u128>`
- `core::num::Wrapping<u128>` with <''a>
- `&''a core::num::Wrapping<u128>` with <''a>
- `&''b core::num::Wrapping<u128>` with <''a, ''b>
- `core::num::Wrapping<i128>`
- `core::num::Wrapping<i128>` with <''a>
- `&''a core::num::Wrapping<i128>` with <''a>
- `&''b core::num::Wrapping<i128>` with <''a, ''b>
- `core::num::Wrapping<i128>`
- `core::num::Wrapping<i128>` with <''a>
- `&''a core::num::Wrapping<i128>` with <''a>
- `&''b core::num::Wrapping<i128>` with <''a, ''b>
- `core::num::Wrapping<usize>`
- `core::num::Wrapping<usize>` with <''a>
- `&''a core::num::Wrapping<usize>` with <''a>
- `&''b core::num::Wrapping<usize>` with <''a, ''b>
- `core::num::Wrapping<usize>`
- `core::num::Wrapping<usize>` with <''a>
- `&''a core::num::Wrapping<usize>` with <''a>
- `&''b core::num::Wrapping<usize>` with <''a, ''b>
- `core::num::Wrapping<isize>`
- `core::num::Wrapping<isize>` with <''a>
- `&''a core::num::Wrapping<isize>` with <''a>
- `&''b core::num::Wrapping<isize>` with <''a, ''b>
- `core::num::Wrapping<isize>`
- `core::num::Wrapping<isize>` with <''a>
- `&''a core::num::Wrapping<isize>` with <''a>
- `&''b core::num::Wrapping<isize>` with <''a, ''b>
- `f32`
- `f32` with <''a>
- `&''a f32` with <''a>
- `&''b f32` with <''a, ''b>
- `f32`
- `f32` with <''a>
- `&''a f32` with <''a>
- `&''b f32` with <''a, ''b>
- `f32`
- `f32` with <''a>
- `&''a f32` with <''a>
- `&''b f32` with <''a, ''b>
- `f32`
- `f32` with <''a>
- `&''a f32` with <''a>
- `&''b f32` with <''a, ''b>
- `f32`
- `f32` with <''a>
- `&''a f32` with <''a>
- `&''b f32` with <''a, ''b>
- `f64`
- `f64` with <''a>
- `&''a f64` with <''a>
- `&''b f64` with <''a, ''b>
- `f64`
- `f64` with <''a>
- `&''a f64` with <''a>
- `&''b f64` with <''a, ''b>
- `f64`
- `f64` with <''a>
- `&''a f64` with <''a>
- `&''b f64` with <''a, ''b>
- `f64`
- `f64` with <''a>
- `&''a f64` with <''a>
- `&''b f64` with <''a, ''b>
- `f64`
- `f64` with <''a>
- `&''a f64` with <''a>
- `&''b f64` with <''a, ''b>
- `f32`
- `f32` with <''a>
- `&''a f32` with <''a>
- `&''b f32` with <''a, ''b>
- `f64`
- `f64` with <''a>
- `&''a f64` with <''a>
- `&''b f64` with <''a, ''b>
- `f64`
- `f64` with <''a>
- `&''a f64` with <''a>
- `&''b f64` with <''a, ''b>

### Functions

#### Function `pow`

**Attributes:**

- `#[inline]`

Raises a value to the power of exp, using exponentiation by squaring.

Note that `0⁰` (`pow(0, 0)`) returns `1`. Mathematically this is undefined.

# Example

```rust
use num_traits::pow;

assert_eq!(pow(2i8, 4), 16);
assert_eq!(pow(6u8, 3), 216);
assert_eq!(pow(0u8, 0), 1); // Be aware if this case affects you
```

```rust
pub fn pow<T: Clone + One + Mul<T, Output = T>>(base: T, exp: usize) -> T { /* ... */ }
```

#### Function `checked_pow`

**Attributes:**

- `#[inline]`

Raises a value to the power of exp, returning `None` if an overflow occurred.

Note that `0⁰` (`checked_pow(0, 0)`) returns `Some(1)`. Mathematically this is undefined.

Otherwise same as the `pow` function.

# Example

```rust
use num_traits::checked_pow;

assert_eq!(checked_pow(2i8, 4), Some(16));
assert_eq!(checked_pow(7i8, 8), None);
assert_eq!(checked_pow(7u32, 8), Some(5_764_801));
assert_eq!(checked_pow(0u32, 0), Some(1)); // Be aware if this case affect you
```

```rust
pub fn checked_pow<T: Clone + One + CheckedMul>(base: T, exp: usize) -> Option<T> { /* ... */ }
```

## Module `real`

**Attributes:**

- `#[<cfg>(any(feature = "std", feature = "libm"))]`

```rust
pub mod real { /* ... */ }
```

### Traits

#### Trait `Real`

A trait for real number types that do not necessarily have
floating-point-specific characteristics such as NaN and infinity.

See [this Wikipedia article](https://en.wikipedia.org/wiki/Real_data_type)
for a list of data types that could meaningfully implement this trait.

This trait is only available with the `std` feature, or with the `libm` feature otherwise.

```rust
pub trait Real: Num + Copy + NumCast + PartialOrd + Neg<Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `min_value`: Returns the smallest finite value that this type can represent.
- `min_positive_value`: Returns the smallest positive, normalized value that this type can represent.
- `epsilon`: Returns epsilon, a small positive value.
- `max_value`: Returns the largest finite value that this type can represent.
- `floor`: Returns the largest integer less than or equal to a number.
- `ceil`: Returns the smallest integer greater than or equal to a number.
- `round`: Returns the nearest integer to a number. Round half-way cases away from
- `trunc`: Return the integer part of a number.
- `fract`: Returns the fractional part of a number.
- `abs`: Computes the absolute value of `self`. Returns `Float::nan()` if the
- `signum`: Returns a number that represents the sign of `self`.
- `is_sign_positive`: Returns `true` if `self` is positive, including `+0.0`,
- `is_sign_negative`: Returns `true` if `self` is negative, including `-0.0`,
- `mul_add`: Fused multiply-add. Computes `(self * a) + b` with only one rounding
- `recip`: Take the reciprocal (inverse) of a number, `1/x`.
- `powi`: Raise a number to an integer power.
- `powf`: Raise a number to a real number power.
- `sqrt`: Take the square root of a number.
- `exp`: Returns `e^(self)`, (the exponential function).
- `exp2`: Returns `2^(self)`.
- `ln`: Returns the natural logarithm of the number.
- `log`: Returns the logarithm of the number with respect to an arbitrary base.
- `log2`: Returns the base 2 logarithm of the number.
- `log10`: Returns the base 10 logarithm of the number.
- `to_degrees`: Converts radians to degrees.
- `to_radians`: Converts degrees to radians.
- `max`: Returns the maximum of the two numbers.
- `min`: Returns the minimum of the two numbers.
- `abs_sub`: The positive difference of two numbers.
- `cbrt`: Take the cubic root of a number.
- `hypot`: Calculate the length of the hypotenuse of a right-angle triangle given
- `sin`: Computes the sine of a number (in radians).
- `cos`: Computes the cosine of a number (in radians).
- `tan`: Computes the tangent of a number (in radians).
- `asin`: Computes the arcsine of a number. Return value is in radians in
- `acos`: Computes the arccosine of a number. Return value is in radians in
- `atan`: Computes the arctangent of a number. Return value is in radians in the
- `atan2`: Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`).
- `sin_cos`: Simultaneously computes the sine and cosine of the number, `x`. Returns
- `exp_m1`: Returns `e^(self) - 1` in a way that is accurate even if the
- `ln_1p`: Returns `ln(1+n)` (natural logarithm) more accurately than if
- `sinh`: Hyperbolic sine function.
- `cosh`: Hyperbolic cosine function.
- `tanh`: Hyperbolic tangent function.
- `asinh`: Inverse hyperbolic sine function.
- `acosh`: Inverse hyperbolic cosine function.
- `atanh`: Inverse hyperbolic tangent function.

##### Implementations

This trait is implemented for the following types:

- `T` with <T: Float>

## Module `sign`

```rust
pub mod sign { /* ... */ }
```

### Traits

#### Trait `Signed`

Useful functions for signed numbers (i.e. numbers that can be negative).

```rust
pub trait Signed: Sized + Num + Neg<Output = Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `abs`: Computes the absolute value.
- `abs_sub`: The positive difference of two numbers.
- `signum`: Returns the sign of the number.
- `is_positive`: Returns true if the number is positive and false if the number is zero or negative.
- `is_negative`: Returns true if the number is negative and false if the number is zero or positive.

##### Implementations

This trait is implemented for the following types:

- `isize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `core::num::Wrapping<T>` with <T: Signed>
- `f32`
- `f64`

#### Trait `Unsigned`

A trait for values which cannot be negative

```rust
pub trait Unsigned: Num {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `usize`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `core::num::Wrapping<T>` with <T: Unsigned>

### Functions

#### Function `abs`

**Attributes:**

- `#[inline(always)]`

Computes the absolute value.

For `f32` and `f64`, `NaN` will be returned if the number is `NaN`

For signed integers, `::MIN` will be returned if the number is `::MIN`.

```rust
pub fn abs<T: Signed>(value: T) -> T { /* ... */ }
```

#### Function `abs_sub`

**Attributes:**

- `#[inline(always)]`

The positive difference of two numbers.

Returns zero if `x` is less than or equal to `y`, otherwise the difference
between `x` and `y` is returned.

```rust
pub fn abs_sub<T: Signed>(x: T, y: T) -> T { /* ... */ }
```

#### Function `signum`

**Attributes:**

- `#[inline(always)]`

Returns the sign of the number.

For `f32` and `f64`:

* `1.0` if the number is positive, `+0.0` or `INFINITY`
* `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
* `NaN` if the number is `NaN`

For signed integers:

* `0` if the number is zero
* `1` if the number is positive
* `-1` if the number is negative

```rust
pub fn signum<T: Signed>(value: T) -> T { /* ... */ }
```

## Types

### Enum `FloatErrorKind`

```rust
pub enum FloatErrorKind {
    Empty,
    Invalid,
}
```

#### Variants

##### `Empty`

##### `Invalid`

#### Implementations

##### Trait Implementations

- **UnwindSafe**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Struct `ParseFloatError`

```rust
pub struct ParseFloatError {
    pub kind: FloatErrorKind,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `kind` | `FloatErrorKind` |  |

#### Implementations

##### Trait Implementations

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Traits

### Trait `Num`

The base trait for numeric types, covering `0` and `1` values,
comparisons, basic numeric operations, and string conversion.

```rust
pub trait Num: PartialEq + Zero + One + NumOps {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `FromStrRadixErr`

##### Required Methods

- `from_str_radix`: Convert from a string and radix (typically `2..=36`).

#### Implementations

This trait is implemented for the following types:

- `usize`
- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `isize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `core::num::Wrapping<T>` with <T: Num>
- `f32`
- `f64`

### Trait `NumOps`

Generic trait for types implementing basic numeric operations

This is automatically implemented for types which implement the operators.

```rust
pub trait NumOps<Rhs = Self, Output = Self>: Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + Mul<Rhs, Output = Output> + Div<Rhs, Output = Output> + Rem<Rhs, Output = Output> {
    /* Associated items */
}
```

#### Implementations

This trait is implemented for the following types:

- `T` with <T, Rhs, Output>

### Trait `NumRef`

The trait for `Num` types which also implement numeric operations taking
the second operand by reference.

This is automatically implemented for types which implement the operators.

```rust
pub trait NumRef: Num + for<''r> NumOps<&''r Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Implementations

This trait is implemented for the following types:

- `T` with <T>

### Trait `RefNum`

The trait for `Num` references which implement numeric operations, taking the
second operand either by value or by reference.

This is automatically implemented for all types which implement the operators. It covers
every type implementing the operations though, regardless of it being a reference or
related to `Num`.

```rust
pub trait RefNum<Base>: NumOps<Base, Base> + for<''r> NumOps<&''r Base, Base> {
    /* Associated items */
}
```

#### Implementations

This trait is implemented for the following types:

- `T` with <T, Base>

### Trait `NumAssignOps`

Generic trait for types implementing numeric assignment operators (like `+=`).

This is automatically implemented for types which implement the operators.

```rust
pub trait NumAssignOps<Rhs = Self>: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs> {
    /* Associated items */
}
```

#### Implementations

This trait is implemented for the following types:

- `T` with <T, Rhs>

### Trait `NumAssign`

The trait for `Num` types which also implement assignment operators.

This is automatically implemented for types which implement the operators.

```rust
pub trait NumAssign: Num + NumAssignOps {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Implementations

This trait is implemented for the following types:

- `T` with <T>

### Trait `NumAssignRef`

The trait for `NumAssign` types which also implement assignment operations
taking the second operand by reference.

This is automatically implemented for types which implement the operators.

```rust
pub trait NumAssignRef: NumAssign + for<''r> NumAssignOps<&''r Self> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Implementations

This trait is implemented for the following types:

- `T` with <T>

## Functions

### Function `clamp`

**Attributes:**

- `#[inline]`

A value bounded by a minimum and a maximum

 If input is less than min then this returns min.
 If input is greater than max then this returns max.
 Otherwise this returns input.

**Panics** in debug mode if `!(min <= max)`.

```rust
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T { /* ... */ }
```

### Function `clamp_min`

**Attributes:**

- `#[inline]`
- `#[allow(clippy::eq_op)]`

A value bounded by a minimum value

 If input is less than min then this returns min.
 Otherwise this returns input.
 `clamp_min(std::f32::NAN, 1.0)` preserves `NAN` different from `f32::min(std::f32::NAN, 1.0)`.

**Panics** in debug mode if `!(min == min)`. (This occurs if `min` is `NAN`.)

```rust
pub fn clamp_min<T: PartialOrd>(input: T, min: T) -> T { /* ... */ }
```

### Function `clamp_max`

**Attributes:**

- `#[inline]`
- `#[allow(clippy::eq_op)]`

A value bounded by a maximum value

 If input is greater than max then this returns max.
 Otherwise this returns input.
 `clamp_max(std::f32::NAN, 1.0)` preserves `NAN` different from `f32::max(std::f32::NAN, 1.0)`.

**Panics** in debug mode if `!(max == max)`. (This occurs if `max` is `NAN`.)

```rust
pub fn clamp_max<T: PartialOrd>(input: T, max: T) -> T { /* ... */ }
```

## Re-exports

### Re-export `Bounded`

```rust
pub use crate::bounds::Bounded;
```

### Re-export `Float`

**Attributes:**

- `#[<cfg>(any(feature = "std", feature = "libm"))]`

```rust
pub use crate::float::Float;
```

### Re-export `FloatConst`

```rust
pub use crate::float::FloatConst;
```

### Re-export `cast`

```rust
pub use crate::cast::cast;
```

### Re-export `AsPrimitive`

```rust
pub use crate::cast::AsPrimitive;
```

### Re-export `FromPrimitive`

```rust
pub use crate::cast::FromPrimitive;
```

### Re-export `NumCast`

```rust
pub use crate::cast::NumCast;
```

### Re-export `ToPrimitive`

```rust
pub use crate::cast::ToPrimitive;
```

### Re-export `one`

```rust
pub use crate::identities::one;
```

### Re-export `zero`

```rust
pub use crate::identities::zero;
```

### Re-export `ConstOne`

```rust
pub use crate::identities::ConstOne;
```

### Re-export `ConstZero`

```rust
pub use crate::identities::ConstZero;
```

### Re-export `One`

```rust
pub use crate::identities::One;
```

### Re-export `Zero`

```rust
pub use crate::identities::Zero;
```

### Re-export `PrimInt`

```rust
pub use crate::int::PrimInt;
```

### Re-export `FromBytes`

```rust
pub use crate::ops::bytes::FromBytes;
```

### Re-export `ToBytes`

```rust
pub use crate::ops::bytes::ToBytes;
```

### Re-export `CheckedAdd`

```rust
pub use crate::ops::checked::CheckedAdd;
```

### Re-export `CheckedDiv`

```rust
pub use crate::ops::checked::CheckedDiv;
```

### Re-export `CheckedMul`

```rust
pub use crate::ops::checked::CheckedMul;
```

### Re-export `CheckedNeg`

```rust
pub use crate::ops::checked::CheckedNeg;
```

### Re-export `CheckedRem`

```rust
pub use crate::ops::checked::CheckedRem;
```

### Re-export `CheckedShl`

```rust
pub use crate::ops::checked::CheckedShl;
```

### Re-export `CheckedShr`

```rust
pub use crate::ops::checked::CheckedShr;
```

### Re-export `CheckedSub`

```rust
pub use crate::ops::checked::CheckedSub;
```

### Re-export `CheckedEuclid`

```rust
pub use crate::ops::euclid::CheckedEuclid;
```

### Re-export `Euclid`

```rust
pub use crate::ops::euclid::Euclid;
```

### Re-export `Inv`

```rust
pub use crate::ops::inv::Inv;
```

### Re-export `MulAdd`

```rust
pub use crate::ops::mul_add::MulAdd;
```

### Re-export `MulAddAssign`

```rust
pub use crate::ops::mul_add::MulAddAssign;
```

### Re-export `Saturating`

```rust
pub use crate::ops::saturating::Saturating;
```

### Re-export `SaturatingAdd`

```rust
pub use crate::ops::saturating::SaturatingAdd;
```

### Re-export `SaturatingMul`

```rust
pub use crate::ops::saturating::SaturatingMul;
```

### Re-export `SaturatingSub`

```rust
pub use crate::ops::saturating::SaturatingSub;
```

### Re-export `WrappingAdd`

```rust
pub use crate::ops::wrapping::WrappingAdd;
```

### Re-export `WrappingMul`

```rust
pub use crate::ops::wrapping::WrappingMul;
```

### Re-export `WrappingNeg`

```rust
pub use crate::ops::wrapping::WrappingNeg;
```

### Re-export `WrappingShl`

```rust
pub use crate::ops::wrapping::WrappingShl;
```

### Re-export `WrappingShr`

```rust
pub use crate::ops::wrapping::WrappingShr;
```

### Re-export `WrappingSub`

```rust
pub use crate::ops::wrapping::WrappingSub;
```

### Re-export `checked_pow`

```rust
pub use crate::pow::checked_pow;
```

### Re-export `pow`

```rust
pub use crate::pow::pow;
```

### Re-export `Pow`

```rust
pub use crate::pow::Pow;
```

### Re-export `abs`

```rust
pub use crate::sign::abs;
```

### Re-export `abs_sub`

```rust
pub use crate::sign::abs_sub;
```

### Re-export `signum`

```rust
pub use crate::sign::signum;
```

### Re-export `Signed`

```rust
pub use crate::sign::Signed;
```

### Re-export `Unsigned`

```rust
pub use crate::sign::Unsigned;
```

