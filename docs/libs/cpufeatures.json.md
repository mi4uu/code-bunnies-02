# Crate Documentation

**Version:** 0.2.17

**Format Version:** 43

# Module `cpufeatures`

This crate provides macros for runtime CPU feature detection. It's intended
as a stopgap until Rust [RFC 2725] adding first-class target feature detection
macros to `libcore` is implemented.

# Supported target architectures

*NOTE: target features with an asterisk are unstable (nightly-only) and
subject to change to match upstream name changes in the Rust standard
library.

## `aarch64`

Linux, iOS, and macOS/ARM only (ARM64 does not support OS-independent feature detection)

Target features:

- `aes`*
- `sha2`*
- `sha3`*

Linux only

- `sm4`*

## `loongarch64`

Linux only (LoongArch64 does not support OS-independent feature detection)

Target features:

- `lam`*
- `ual`*
- `fpu`*
- `lsx`*
- `lasx`*
- `crc32`*
- `complex`*
- `crypto`*
- `lvz`*
- `lbt.x86`*
- `lbt.arm`*
- `lbt.mips`*
- `ptw`*

## `x86`/`x86_64`

OS independent and `no_std`-friendly

Target features:

- `adx`
- `aes`
- `avx`
- `avx2`
- `avx512bw`*
- `avx512cd`*
- `avx512dq`*
- `avx512er`*
- `avx512f`*
- `avx512ifma`*
- `avx512pf`*
- `avx512vl`*
- `bmi1`
- `bmi2`
- `fma`,
- `mmx`
- `pclmulqdq`
- `popcnt`
- `rdrand`
- `rdseed`
- `sgx`
- `sha`
- `sse`
- `sse2`
- `sse3`
- `sse4.1`
- `sse4.2`
- `ssse3`

If you would like detection support for a target feature which is not on
this list, please [open a GitHub issue][gh].

# Example
```
# #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
# {
// This macro creates `cpuid_aes_sha` module
cpufeatures::new!(cpuid_aes_sha, "aes", "sha");

// `token` is a Zero Sized Type (ZST) value, which guarantees
// that underlying static storage got properly initialized,
// which allows to omit initialization branch
let token: cpuid_aes_sha::InitToken = cpuid_aes_sha::init();

if token.get() {
    println!("CPU supports both SHA and AES extensions");
} else {
    println!("SHA and AES extensions are not supported");
}

// If stored value needed only once you can get stored value
// omitting the token
let val = cpuid_aes_sha::get();
assert_eq!(val, token.get());

// Additionally you can get both token and value
let (token, val) = cpuid_aes_sha::init_get();
assert_eq!(val, token.get());
# }
```

Note that if all tested target features are enabled via compiler options
(e.g. by using `RUSTFLAGS`), the `get` method will always return `true`
and `init` will not use CPUID instruction. Such behavior allows
compiler to completely eliminate fallback code.

After first call macro caches result and returns it in subsequent
calls, thus runtime overhead for them is minimal.

[RFC 2725]: https://github.com/rust-lang/rfcs/pull/2725
[gh]: https://github.com/RustCrypto/utils/issues/new?title=cpufeatures:%20requesting%20support%20for%20CHANGEME%20target%20feature

## Macros

### Macro `new`

**Attributes:**

- `#[macro_export]`

Create module with CPU feature detection code.

```rust
pub macro_rules! new {
    /* macro_rules! new {
    ($mod_name:ident, $($tf:tt),+ $(,)?) => { ... };
} */
}
```

