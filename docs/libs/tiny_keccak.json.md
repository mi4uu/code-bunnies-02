# Crate Documentation

**Version:** 2.0.2

**Format Version:** 43

# Module `tiny_keccak`

Keccak derived functions specified in [`FIPS-202`], [`SP800-185`] and [`KangarooTwelve`].

# Example

```
# use tiny_keccak::Hasher;
#
# fn foo<H: Hasher>(mut hasher: H) {
let input_a = b"hello world";
let input_b = b"!";
let mut output = [0u8; 32];
hasher.update(input_a);
hasher.update(input_b);
hasher.finalize(&mut output);
# }
```

# Credits

- [`coruus/keccak-tiny`] for C implementation of keccak function
- [`@quininer`] for `no-std` support and rust implementation [`SP800-185`]
- [`mimoo/GoKangarooTwelve`] for GO implementation of `KangarooTwelve`
- [`@Vurich`] for optimizations
- [`@oleganza`] for adding support for half-duplex use

# License

[`CC0`]. Attribution kindly requested. Blame taken too,
but not liability.

[`FIPS-202`]: https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
[`SP800-185`]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-185.pdf
[`KangarooTwelve`]: https://eprint.iacr.org/2016/770.pdf
[`coruus/keccak-tiny`]: https://github.com/coruus/keccak-tiny
[`mimoo/GoKangarooTwelve`]: https://github.com/mimoo/GoKangarooTwelve
[`@quininer`]: https://github.com/quininer
[`@Vurich`]: https://github.com/Vurich
[`@oleganza`]: https://github.com/oleganza
[`CC0`]: https://github.com/debris/tiny-keccak/blob/master/LICENSE

## Traits

### Trait `Hasher`

A trait for hashing an arbitrary stream of bytes.

# Example

```
# use tiny_keccak::Hasher;
#
# fn foo<H: Hasher>(mut hasher: H) {
let input_a = b"hello world";
let input_b = b"!";
let mut output = [0u8; 32];
hasher.update(input_a);
hasher.update(input_b);
hasher.finalize(&mut output);
# }
```

```rust
pub trait Hasher {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `update`: Absorb additional input. Can be called multiple times.
- `finalize`: Pad and squeeze the state to the output.

#### Implementations

This trait is implemented for the following types:

- `Shake`

### Trait `IntoXof`

A trait used to convert [`Hasher`] into it's [`Xof`] counterpart.

# Example

```
# use tiny_keccak::IntoXof;
#
# fn foo<H: IntoXof>(hasher: H) {
let xof = hasher.into_xof();
# }
```

[`Hasher`]: trait.Hasher.html
[`Xof`]: trait.Xof.html

```rust
pub trait IntoXof {
    /* Associated items */
}
```

#### Required Items

##### Associated Types

- `Xof`: A type implementing [`Xof`], eXtendable-output function interface.

##### Required Methods

- `into_xof`: A method used to convert type into [`Xof`].

### Trait `Xof`

Extendable-output function (`XOF`) is a function on bit strings in which the output can be
extended to any desired length.

# Example

```
# use tiny_keccak::Xof;
#
# fn foo<X: Xof>(mut xof: X) {
let mut output = [0u8; 64];
xof.squeeze(&mut output[0..32]);
xof.squeeze(&mut output[32..]);
# }
```

```rust
pub trait Xof {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `squeeze`: A method used to retrieve another part of hash function output.

#### Implementations

This trait is implemented for the following types:

- `Shake`

## Re-exports

### Re-export `keccakf`

**Attributes:**

- `#[<cfg>(any(feature = "keccak", feature = "shake", feature = "sha3", feature =
"cshake", feature = "kmac", feature = "tuple_hash", feature =
"parallel_hash"))]`

```rust
pub use keccakf::keccakf;
```

### Re-export `Shake`

**Attributes:**

- `#[<cfg>(feature = "shake")]`

```rust
pub use shake::Shake;
```

