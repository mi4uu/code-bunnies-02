# Crate Documentation

**Version:** 1.11.0

**Format Version:** 43

# Module `portable_atomic`

<!-- Note: Document from sync-markdown-to-rustdoc:start through sync-markdown-to-rustdoc:end
     is synchronized from README.md. Any changes to that range are not preserved. -->
<!-- tidy:sync-markdown-to-rustdoc:start -->

Portable atomic types including support for 128-bit atomics, atomic float, etc.

- Provide all atomic integer types (`Atomic{I,U}{8,16,32,64}`) for all targets that can use atomic CAS. (i.e., all targets that can use `std`, and most no-std targets)
- Provide `AtomicI128` and `AtomicU128`.
- Provide `AtomicF32` and `AtomicF64`. ([optional, requires the `float` feature](#optional-features-float))
- Provide `AtomicF16` and `AtomicF128` for [unstable `f16` and `f128`](https://github.com/rust-lang/rust/issues/116909). ([optional, requires the `float` feature and unstable cfgs](#optional-features-float))
- Provide atomic load/store for targets where atomic is not available at all in the standard library. (RISC-V without A-extension, MSP430, AVR)
- Provide atomic CAS for targets where atomic CAS is not available in the standard library. (thumbv6m, pre-v6 Arm, RISC-V without A-extension, MSP430, AVR, Xtensa, etc.) (always enabled for MSP430 and AVR, [optional](#optional-features-critical-section) otherwise)
- Provide stable equivalents of the standard library's atomic types' unstable APIs, such as [`AtomicPtr::fetch_*`](https://github.com/rust-lang/rust/issues/99108).
- Make features that require newer compilers, such as [`fetch_{max,min}`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.fetch_max), [`fetch_update`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.fetch_update), [`as_ptr`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.as_ptr), [`from_ptr`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.from_ptr), [`AtomicBool::fetch_not`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html#method.fetch_not) and [stronger CAS failure ordering](https://github.com/rust-lang/rust/pull/98383) available on Rust 1.34+.
- Provide workaround for bugs in the standard library's atomic-related APIs, such as [rust-lang/rust#100650], `fence`/`compiler_fence` on MSP430 that cause LLVM error, etc.

<!-- TODO:
- mention Atomic{I,U}*::fetch_neg, Atomic{I*,U*,Ptr}::bit_*, etc.
- mention optimizations not available in the standard library's equivalents
-->

portable-atomic version of `std::sync::Arc` is provided by the [portable-atomic-util](https://github.com/taiki-e/portable-atomic/tree/HEAD/portable-atomic-util) crate.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
portable-atomic = "1"
```

The default features are mainly for users who use atomics larger than the pointer width.
If you don't need them, disabling the default features may reduce code size and compile time slightly.

```toml
[dependencies]
portable-atomic = { version = "1", default-features = false }
```

If your crate supports no-std environment and requires atomic CAS, enabling the `require-cas` feature will allow the `portable-atomic` to display a [helpful error message](https://github.com/taiki-e/portable-atomic/pull/100) to users on targets requiring additional action on the user side to provide atomic CAS.

```toml
[dependencies]
portable-atomic = { version = "1.3", default-features = false, features = ["require-cas"] }
```

## 128-bit atomics support

Native 128-bit atomic operations are available on x86_64 (Rust 1.59+), AArch64 (Rust 1.59+), riscv64 (Rust 1.59+), Arm64EC (Rust 1.84+), s390x (Rust 1.84+), and powerpc64 (nightly only), otherwise the fallback implementation is used.

On x86_64, even if `cmpxchg16b` is not available at compile-time (note: `cmpxchg16b` target feature is enabled by default only on Apple and Windows (except Windows 7) targets), run-time detection checks whether `cmpxchg16b` is available. If `cmpxchg16b` is not available at either compile-time or run-time detection, the fallback implementation is used. See also [`portable_atomic_no_outline_atomics`](#optional-cfg-no-outline-atomics) cfg.

They are usually implemented using inline assembly, and when using Miri or ThreadSanitizer that do not support inline assembly, core intrinsics are used instead of inline assembly if possible.

See the [`atomic128` module's readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/atomic128/README.md) for details.

## Optional features

- **`fallback`** *(enabled by default)*<br>
  Enable fallback implementations.

  Disabling this allows only atomic types for which the platform natively supports atomic operations.

- <a name="optional-features-float"></a>**`float`**<br>
  Provide `AtomicF{32,64}`.

  - When unstable `--cfg portable_atomic_unstable_f16` is also enabled, `AtomicF16` for [unstable `f16`](https://github.com/rust-lang/rust/issues/116909) is also provided.
  - When unstable `--cfg portable_atomic_unstable_f128` is also enabled, `AtomicF128` for [unstable `f128`](https://github.com/rust-lang/rust/issues/116909) is also provided.

  Note:
  - Most of `fetch_*` operations of atomic floats are implemented using CAS loops, which can be slower than equivalent operations of atomic integers. (AArch64 with FEAT_LSFE and GPU targets have atomic instructions for float, [so we plan to use these instructions for them in the future.](https://github.com/taiki-e/portable-atomic/issues/34))
  - Unstable cfgs are outside of the normal semver guarantees and minor or patch versions of portable-atomic may make breaking changes to them at any time.

- **`std`**<br>
  Use `std`.

- <a name="optional-features-require-cas"></a>**`require-cas`**<br>
  Emit compile error if atomic CAS is not available. See [Usage](#usage) section and [#100](https://github.com/taiki-e/portable-atomic/pull/100) for more.

- <a name="optional-features-serde"></a>**`serde`**<br>
  Implement `serde::{Serialize,Deserialize}` for atomic types.

  Note:
  - The MSRV when this feature is enabled depends on the MSRV of [serde].

- <a name="optional-features-critical-section"></a>**`critical-section`**<br>
  When this feature is enabled, this crate uses [critical-section] to provide atomic CAS for targets where
  it is not natively available. When enabling it, you should provide a suitable critical section implementation
  for the current target, see the [critical-section] documentation for details on how to do so.

  `critical-section` support is useful to get atomic CAS when the [`unsafe-assume-single-core` feature](#optional-features-unsafe-assume-single-core) can't be used,
  such as multi-core targets, unprivileged code running under some RTOS, or environments where disabling interrupts
  needs extra care due to e.g. real-time requirements.

  Note that with the `critical-section` feature, critical sections are taken for all atomic operations, while with
  [`unsafe-assume-single-core` feature](#optional-features-unsafe-assume-single-core) some operations don't require disabling interrupts (loads and stores, but
  additionally on MSP430 `add`, `sub`, `and`, `or`, `xor`, `not`). Therefore, for better performance, if
  all the `critical-section` implementation for your target does is disable interrupts, prefer using
  `unsafe-assume-single-core` feature instead.

  Note:
  - The MSRV when this feature is enabled depends on the MSRV of [critical-section].
  - It is usually *not* recommended to always enable this feature in dependencies of the library.

    Enabling this feature will prevent the end user from having the chance to take advantage of other (potentially) efficient implementations ([Implementations provided by `unsafe-assume-single-core` feature, default implementations on MSP430 and AVR](#optional-features-unsafe-assume-single-core), implementation proposed in [#60], etc. Other systems may also be supported in the future).

    The recommended approach for libraries is to leave it up to the end user whether or not to enable this feature. (However, it may make sense to enable this feature by default for libraries specific to a platform where other implementations are known not to work.)

    As an example, the end-user's `Cargo.toml` that uses a crate that provides a critical-section implementation and a crate that depends on portable-atomic as an option would be expected to look like this:

    ```toml
    [dependencies]
    portable-atomic = { version = "1", default-features = false, features = ["critical-section"] }
    crate-provides-critical-section-impl = "..."
    crate-uses-portable-atomic-as-feature = { version = "...", features = ["portable-atomic"] }
    ```

- <a name="optional-features-unsafe-assume-single-core"></a>**`unsafe-assume-single-core`**<br>
  Assume that the target is single-core.
  When this feature is enabled, this crate provides atomic CAS for targets where atomic CAS is not available in the standard library by disabling interrupts.

  This feature is `unsafe`, and note the following safety requirements:
  - Enabling this feature for multi-core systems is always **unsound**.
  - This uses privileged instructions to disable interrupts, so it usually doesn't work on unprivileged mode.
    Enabling this feature in an environment where privileged instructions are not available, or if the instructions used are not sufficient to disable interrupts in the system, it is also usually considered **unsound**, although the details are system-dependent.

    The following are known cases:
    - On pre-v6 Arm, this disables only IRQs by default. For many systems (e.g., GBA) this is enough. If the system need to disable both IRQs and FIQs, you need to enable the `disable-fiq` feature together.
    - On RISC-V without A-extension, this generates code for machine-mode (M-mode) by default. If you enable the `s-mode` together, this generates code for supervisor-mode (S-mode). In particular, `qemu-system-riscv*` uses [OpenSBI](https://github.com/riscv-software-src/opensbi) as the default firmware.

    See also the [`interrupt` module's readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/interrupt/README.md).

  Consider using the [`critical-section` feature](#optional-features-critical-section) for systems that cannot use this feature.

  It is **very strongly discouraged** to enable this feature in libraries that depend on `portable-atomic`. The recommended approach for libraries is to leave it up to the end user whether or not to enable this feature. (However, it may make sense to enable this feature by default for libraries specific to a platform where it is guaranteed to always be sound, for example in a hardware abstraction layer targeting a single-core chip.)

  Armv6-M (thumbv6m), pre-v6 Arm (e.g., thumbv4t, thumbv5te), RISC-V without A-extension, and Xtensa are currently supported.

  Since all MSP430 and AVR are single-core, we always provide atomic CAS for them without this feature.

  Enabling this feature for targets that have atomic CAS will result in a compile error.

  Feel free to submit an issue if your target is not supported yet.

## Optional cfg

One of the ways to enable cfg is to set [rustflags in the cargo config](https://doc.rust-lang.org/cargo/reference/config.html#targettriplerustflags):

```toml
# .cargo/config.toml
[target.<target>]
rustflags = ["--cfg", "portable_atomic_no_outline_atomics"]
```

Or set environment variable:

```sh
RUSTFLAGS="--cfg portable_atomic_no_outline_atomics" cargo ...
```

- <a name="optional-cfg-unsafe-assume-single-core"></a>**`--cfg portable_atomic_unsafe_assume_single_core`**<br>
  Since 1.4.0, this cfg is an alias of [`unsafe-assume-single-core` feature](#optional-features-unsafe-assume-single-core).

  Originally, we were providing these as cfgs instead of features, but based on a strong request from the embedded ecosystem, we have agreed to provide them as features as well. See [#94](https://github.com/taiki-e/portable-atomic/pull/94) for more.

- <a name="optional-cfg-no-outline-atomics"></a>**`--cfg portable_atomic_no_outline_atomics`**<br>
  Disable dynamic dispatching by run-time CPU feature detection.

  If dynamic dispatching by run-time CPU feature detection is enabled, it allows maintaining support for older CPUs while using features that are not supported on older CPUs, such as CMPXCHG16B (x86_64) and FEAT_LSE/FEAT_LSE2 (AArch64).

  Note:
  - Dynamic detection is currently only supported in x86_64, AArch64, Arm, RISC-V, Arm64EC, and powerpc64, otherwise it works the same as when this cfg is set.
  - If the required target features are enabled at compile-time, the atomic operations are inlined.
  - This is compatible with no-std (as with all features except `std`).
  - On some targets, run-time detection is disabled by default mainly for compatibility with incomplete build environments or support for it is experimental, and can be enabled by `--cfg portable_atomic_outline_atomics`. (When both cfg are enabled, `*_no_*` cfg is preferred.)
  - Some AArch64 targets enable LLVM's `outline-atomics` target feature by default, so if you set this cfg, you may want to disable that as well. (portable-atomic's outline-atomics does not depend on the compiler-rt symbols, so even if you need to disable LLVM's outline-atomics, you may not need to disable portable-atomic's outline-atomics.)

  See also the [`atomic128` module's readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/atomic128/README.md).

## Related Projects

- [atomic-maybe-uninit]: Atomic operations on potentially uninitialized integers.
- [atomic-memcpy]: Byte-wise atomic memcpy.

[#60]: https://github.com/taiki-e/portable-atomic/issues/60
[atomic-maybe-uninit]: https://github.com/taiki-e/atomic-maybe-uninit
[atomic-memcpy]: https://github.com/taiki-e/atomic-memcpy
[critical-section]: https://github.com/rust-embedded/critical-section
[rust-lang/rust#100650]: https://github.com/rust-lang/rust/issues/100650
[serde]: https://github.com/serde-rs/serde

<!-- tidy:sync-markdown-to-rustdoc:end -->

## Modules

## Module `hint`

Re-export of the [`core::hint`] module.

The only difference from the [`core::hint`] module is that [`spin_loop`]
is available in all rust versions that this crate supports.

```
use portable_atomic::hint;

hint::spin_loop();
```

```rust
pub mod hint { /* ... */ }
```

### Functions

#### Function `spin_loop`

**Attributes:**

- `#[inline]`

Emits a machine instruction to signal the processor that it is running in
a busy-wait spin-loop ("spin lock").

Upon receiving the spin-loop signal the processor can optimize its behavior by,
for example, saving power or switching hyper-threads.

This function is different from [`thread::yield_now`] which directly
yields to the system's scheduler, whereas `spin_loop` does not interact
with the operating system.

A common use case for `spin_loop` is implementing bounded optimistic
spinning in a CAS loop in synchronization primitives. To avoid problems
like priority inversion, it is strongly recommended that the spin loop is
terminated after a finite amount of iterations and an appropriate blocking
syscall is made.

**Note:** On platforms that do not support receiving spin-loop hints this
function does not do anything at all.

[`thread::yield_now`]: https://doc.rust-lang.org/std/thread/fn.yield_now.html

```rust
pub fn spin_loop() { /* ... */ }
```

### Re-exports

#### Re-export `core::hint::*`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use core::hint::*;
```

## Types

### Struct `AtomicBool`

**Attributes:**

- `#[repr(C, align(1))]`

A boolean type which can be safely shared between threads.

This type has the same in-memory representation as a [`bool`].

If the compiler and the platform support atomic loads and stores of `u8`,
this type is a wrapper for the standard library's
[`AtomicBool`](core::sync::atomic::AtomicBool). If the platform supports it
but the compiler does not, atomic operations are implemented using inline
assembly.

```rust
pub struct AtomicBool {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(v: bool) -> Self { /* ... */ }
  ```
  Creates a new `AtomicBool`.

- ```rust
  pub const unsafe fn from_ptr<''a>(ptr: *mut bool) -> &''a Self { /* ... */ }
  ```
  Creates a new `AtomicBool` from a pointer.

- ```rust
  pub fn is_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn is_always_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn get_mut(self: &mut Self) -> &mut bool { /* ... */ }
  ```
  Returns a mutable reference to the underlying [`bool`].

- ```rust
  pub const fn into_inner(self: Self) -> bool { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> bool { /* ... */ }
  ```
  Loads a value from the bool.

- ```rust
  pub fn store(self: &Self, val: bool, order: Ordering) { /* ... */ }
  ```
  Stores a value into the bool.

- ```rust
  pub fn swap(self: &Self, val: bool, order: Ordering) -> bool { /* ... */ }
  ```
  Stores a value into the bool, returning the previous value.

- ```rust
  pub fn compare_exchange(self: &Self, current: bool, new: bool, success: Ordering, failure: Ordering) -> Result<bool, bool> { /* ... */ }
  ```
  Stores a value into the [`bool`] if the current value is the same as the `current` value.

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: bool, new: bool, success: Ordering, failure: Ordering) -> Result<bool, bool> { /* ... */ }
  ```
  Stores a value into the [`bool`] if the current value is the same as the `current` value.

- ```rust
  pub fn fetch_and(self: &Self, val: bool, order: Ordering) -> bool { /* ... */ }
  ```
  Logical "and" with a boolean value.

- ```rust
  pub fn and(self: &Self, val: bool, order: Ordering) { /* ... */ }
  ```
  Logical "and" with a boolean value.

- ```rust
  pub fn fetch_nand(self: &Self, val: bool, order: Ordering) -> bool { /* ... */ }
  ```
  Logical "nand" with a boolean value.

- ```rust
  pub fn fetch_or(self: &Self, val: bool, order: Ordering) -> bool { /* ... */ }
  ```
  Logical "or" with a boolean value.

- ```rust
  pub fn or(self: &Self, val: bool, order: Ordering) { /* ... */ }
  ```
  Logical "or" with a boolean value.

- ```rust
  pub fn fetch_xor(self: &Self, val: bool, order: Ordering) -> bool { /* ... */ }
  ```
  Logical "xor" with a boolean value.

- ```rust
  pub fn xor(self: &Self, val: bool, order: Ordering) { /* ... */ }
  ```
  Logical "xor" with a boolean value.

- ```rust
  pub fn fetch_not(self: &Self, order: Ordering) -> bool { /* ... */ }
  ```
  Logical "not" with a boolean value.

- ```rust
  pub fn not(self: &Self, order: Ordering) { /* ... */ }
  ```
  Logical "not" with a boolean value.

- ```rust
  pub fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<bool, bool>
where
    F: FnMut(bool) -> Option<bool> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an optional

- ```rust
  pub const fn as_ptr(self: &Self) -> *mut bool { /* ... */ }
  ```
  Returns a mutable pointer to the underlying [`bool`].

##### Trait Implementations

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(b: bool) -> Self { /* ... */ }
    ```
    Converts a `bool` into an `AtomicBool`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```
    Creates an `AtomicBool` initialized to `false`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Struct `AtomicPtr`

**Attributes:**

- `#[<cfg_attr>(target_pointer_width = "16", repr(C, align(2)))]`
- `#[<cfg_attr>(target_pointer_width = "32", repr(C, align(4)))]`
- `#[<cfg_attr>(target_pointer_width = "64", repr(C, align(8)))]`
- `#[<cfg_attr>(target_pointer_width = "128", repr(C, align(16)))]`
- `#[repr(C, align(8))]`

A raw pointer type which can be safely shared between threads.

This type has the same in-memory representation as a `*mut T`.

If the compiler and the platform support atomic loads and stores of pointers,
this type is a wrapper for the standard library's
[`AtomicPtr`](core::sync::atomic::AtomicPtr). If the platform supports it
but the compiler does not, atomic operations are implemented using inline
assembly.

```rust
pub struct AtomicPtr<T> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(p: *mut T) -> Self { /* ... */ }
  ```
  Creates a new `AtomicPtr`.

- ```rust
  pub const unsafe fn from_ptr<''a>(ptr: *mut *mut T) -> &''a Self { /* ... */ }
  ```
  Creates a new `AtomicPtr` from a pointer.

- ```rust
  pub fn is_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn is_always_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn get_mut(self: &mut Self) -> &mut *mut T { /* ... */ }
  ```
  Returns a mutable reference to the underlying pointer.

- ```rust
  pub const fn into_inner(self: Self) -> *mut T { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> *mut T { /* ... */ }
  ```
  Loads a value from the pointer.

- ```rust
  pub fn store(self: &Self, ptr: *mut T, order: Ordering) { /* ... */ }
  ```
  Stores a value into the pointer.

- ```rust
  pub fn swap(self: &Self, ptr: *mut T, order: Ordering) -> *mut T { /* ... */ }
  ```
  Stores a value into the pointer, returning the previous value.

- ```rust
  pub fn compare_exchange(self: &Self, current: *mut T, new: *mut T, success: Ordering, failure: Ordering) -> Result<*mut T, *mut T> { /* ... */ }
  ```
  Stores a value into the pointer if the current value is the same as the `current` value.

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: *mut T, new: *mut T, success: Ordering, failure: Ordering) -> Result<*mut T, *mut T> { /* ... */ }
  ```
  Stores a value into the pointer if the current value is the same as the `current` value.

- ```rust
  pub fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<*mut T, *mut T>
where
    F: FnMut(*mut T) -> Option<*mut T> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an optional

- ```rust
  pub fn fetch_ptr_add(self: &Self, val: usize, order: Ordering) -> *mut T { /* ... */ }
  ```
  Offsets the pointer's address by adding `val` (in units of `T`),

- ```rust
  pub fn fetch_ptr_sub(self: &Self, val: usize, order: Ordering) -> *mut T { /* ... */ }
  ```
  Offsets the pointer's address by subtracting `val` (in units of `T`),

- ```rust
  pub fn fetch_byte_add(self: &Self, val: usize, order: Ordering) -> *mut T { /* ... */ }
  ```
  Offsets the pointer's address by adding `val` *bytes*, returning the

- ```rust
  pub fn fetch_byte_sub(self: &Self, val: usize, order: Ordering) -> *mut T { /* ... */ }
  ```
  Offsets the pointer's address by subtracting `val` *bytes*, returning the

- ```rust
  pub fn fetch_or(self: &Self, val: usize, order: Ordering) -> *mut T { /* ... */ }
  ```
  Performs a bitwise "or" operation on the address of the current pointer,

- ```rust
  pub fn fetch_and(self: &Self, val: usize, order: Ordering) -> *mut T { /* ... */ }
  ```
  Performs a bitwise "and" operation on the address of the current

- ```rust
  pub fn fetch_xor(self: &Self, val: usize, order: Ordering) -> *mut T { /* ... */ }
  ```
  Performs a bitwise "xor" operation on the address of the current

- ```rust
  pub fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Sets the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Clears the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Toggles the bit at the specified bit-position.

- ```rust
  pub const fn as_ptr(self: &Self) -> *mut *mut T { /* ... */ }
  ```
  Returns a mutable pointer to the underlying pointer.

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Pointer**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```
    Creates a null `AtomicPtr<T>`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(p: *mut T) -> Self { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Unpin**
### Struct `AtomicIsize`

**Attributes:**

- `#[repr(C, align(8))]`

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`isize`].

If the compiler and the platform support atomic loads and stores of [`isize`], this type is a wrapper for the standard library's `AtomicIsize`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call [`AtomicIsize::is_lock_free()`] to check whether
atomic instructions or locks will be used.

```rust
pub struct AtomicIsize {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(v: isize) -> Self { /* ... */ }
  ```
  Creates a new atomic integer.

- ```rust
  pub const unsafe fn from_ptr<''a>(ptr: *mut isize) -> &''a Self { /* ... */ }
  ```
  Creates a new reference to an atomic integer from a pointer.

- ```rust
  pub fn is_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn is_always_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn get_mut(self: &mut Self) -> &mut isize { /* ... */ }
  ```
  Returns a mutable reference to the underlying integer.

- ```rust
  pub const fn into_inner(self: Self) -> isize { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> isize { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: isize, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: isize, order: Ordering) -> isize { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous value.

- ```rust
  pub fn compare_exchange(self: &Self, current: isize, new: isize, success: Ordering, failure: Ordering) -> Result<isize, isize> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: isize, new: isize, success: Ordering, failure: Ordering) -> Result<isize, isize> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn fetch_add(self: &Self, val: isize, order: Ordering) -> isize { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn add(self: &Self, val: isize, order: Ordering) { /* ... */ }
  ```
  Adds to the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: isize, order: Ordering) -> isize { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn sub(self: &Self, val: isize, order: Ordering) { /* ... */ }
  ```
  Subtracts from the current value.

- ```rust
  pub fn fetch_and(self: &Self, val: isize, order: Ordering) -> isize { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn and(self: &Self, val: isize, order: Ordering) { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: isize, order: Ordering) -> isize { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: isize, order: Ordering) -> isize { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn or(self: &Self, val: isize, order: Ordering) { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_xor(self: &Self, val: isize, order: Ordering) -> isize { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn xor(self: &Self, val: isize, order: Ordering) { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<isize, isize>
where
    F: FnMut(isize) -> Option<isize> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an optional

- ```rust
  pub fn fetch_max(self: &Self, val: isize, order: Ordering) -> isize { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: isize, order: Ordering) -> isize { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Sets the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Clears the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Toggles the bit at the specified bit-position.

- ```rust
  pub fn fetch_not(self: &Self, order: Ordering) -> isize { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn not(self: &Self, order: Ordering) { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn fetch_neg(self: &Self, order: Ordering) -> isize { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub fn neg(self: &Self, order: Ordering) { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub const fn as_ptr(self: &Self) -> *mut isize { /* ... */ }
  ```
  Returns a mutable pointer to the underlying integer.

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(v: isize) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Unpin**
### Struct `AtomicUsize`

**Attributes:**

- `#[repr(C, align(8))]`

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`usize`].

If the compiler and the platform support atomic loads and stores of [`usize`], this type is a wrapper for the standard library's `AtomicUsize`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call [`AtomicUsize::is_lock_free()`] to check whether
atomic instructions or locks will be used.

```rust
pub struct AtomicUsize {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(v: usize) -> Self { /* ... */ }
  ```
  Creates a new atomic integer.

- ```rust
  pub const unsafe fn from_ptr<''a>(ptr: *mut usize) -> &''a Self { /* ... */ }
  ```
  Creates a new reference to an atomic integer from a pointer.

- ```rust
  pub fn is_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn is_always_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn get_mut(self: &mut Self) -> &mut usize { /* ... */ }
  ```
  Returns a mutable reference to the underlying integer.

- ```rust
  pub const fn into_inner(self: Self) -> usize { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> usize { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: usize, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: usize, order: Ordering) -> usize { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous value.

- ```rust
  pub fn compare_exchange(self: &Self, current: usize, new: usize, success: Ordering, failure: Ordering) -> Result<usize, usize> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: usize, new: usize, success: Ordering, failure: Ordering) -> Result<usize, usize> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn fetch_add(self: &Self, val: usize, order: Ordering) -> usize { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn add(self: &Self, val: usize, order: Ordering) { /* ... */ }
  ```
  Adds to the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: usize, order: Ordering) -> usize { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn sub(self: &Self, val: usize, order: Ordering) { /* ... */ }
  ```
  Subtracts from the current value.

- ```rust
  pub fn fetch_and(self: &Self, val: usize, order: Ordering) -> usize { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn and(self: &Self, val: usize, order: Ordering) { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: usize, order: Ordering) -> usize { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: usize, order: Ordering) -> usize { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn or(self: &Self, val: usize, order: Ordering) { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_xor(self: &Self, val: usize, order: Ordering) -> usize { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn xor(self: &Self, val: usize, order: Ordering) { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<usize, usize>
where
    F: FnMut(usize) -> Option<usize> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an optional

- ```rust
  pub fn fetch_max(self: &Self, val: usize, order: Ordering) -> usize { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: usize, order: Ordering) -> usize { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Sets the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Clears the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Toggles the bit at the specified bit-position.

- ```rust
  pub fn fetch_not(self: &Self, order: Ordering) -> usize { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn not(self: &Self, order: Ordering) { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn fetch_neg(self: &Self, order: Ordering) -> usize { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub fn neg(self: &Self, order: Ordering) { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub const fn as_ptr(self: &Self) -> *mut usize { /* ... */ }
  ```
  Returns a mutable pointer to the underlying integer.

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(v: usize) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
### Struct `AtomicI8`

**Attributes:**

- `#[repr(C, align(1))]`

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`i8`].

If the compiler and the platform support atomic loads and stores of [`i8`], this type is a wrapper for the standard library's `AtomicI8`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call [`AtomicI8::is_lock_free()`] to check whether
atomic instructions or locks will be used.

```rust
pub struct AtomicI8 {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(v: i8) -> Self { /* ... */ }
  ```
  Creates a new atomic integer.

- ```rust
  pub const unsafe fn from_ptr<''a>(ptr: *mut i8) -> &''a Self { /* ... */ }
  ```
  Creates a new reference to an atomic integer from a pointer.

- ```rust
  pub fn is_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn is_always_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn get_mut(self: &mut Self) -> &mut i8 { /* ... */ }
  ```
  Returns a mutable reference to the underlying integer.

- ```rust
  pub const fn into_inner(self: Self) -> i8 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> i8 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: i8, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: i8, order: Ordering) -> i8 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous value.

- ```rust
  pub fn compare_exchange(self: &Self, current: i8, new: i8, success: Ordering, failure: Ordering) -> Result<i8, i8> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: i8, new: i8, success: Ordering, failure: Ordering) -> Result<i8, i8> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn fetch_add(self: &Self, val: i8, order: Ordering) -> i8 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn add(self: &Self, val: i8, order: Ordering) { /* ... */ }
  ```
  Adds to the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: i8, order: Ordering) -> i8 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn sub(self: &Self, val: i8, order: Ordering) { /* ... */ }
  ```
  Subtracts from the current value.

- ```rust
  pub fn fetch_and(self: &Self, val: i8, order: Ordering) -> i8 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn and(self: &Self, val: i8, order: Ordering) { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: i8, order: Ordering) -> i8 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: i8, order: Ordering) -> i8 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn or(self: &Self, val: i8, order: Ordering) { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_xor(self: &Self, val: i8, order: Ordering) -> i8 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn xor(self: &Self, val: i8, order: Ordering) { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i8, i8>
where
    F: FnMut(i8) -> Option<i8> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an optional

- ```rust
  pub fn fetch_max(self: &Self, val: i8, order: Ordering) -> i8 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: i8, order: Ordering) -> i8 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Sets the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Clears the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Toggles the bit at the specified bit-position.

- ```rust
  pub fn fetch_not(self: &Self, order: Ordering) -> i8 { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn not(self: &Self, order: Ordering) { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn fetch_neg(self: &Self, order: Ordering) -> i8 { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub fn neg(self: &Self, order: Ordering) { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub const fn as_ptr(self: &Self) -> *mut i8 { /* ... */ }
  ```
  Returns a mutable pointer to the underlying integer.

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(v: i8) -> Self { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Send**
### Struct `AtomicU8`

**Attributes:**

- `#[repr(C, align(1))]`

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`u8`].

If the compiler and the platform support atomic loads and stores of [`u8`], this type is a wrapper for the standard library's `AtomicU8`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call [`AtomicU8::is_lock_free()`] to check whether
atomic instructions or locks will be used.

```rust
pub struct AtomicU8 {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(v: u8) -> Self { /* ... */ }
  ```
  Creates a new atomic integer.

- ```rust
  pub const unsafe fn from_ptr<''a>(ptr: *mut u8) -> &''a Self { /* ... */ }
  ```
  Creates a new reference to an atomic integer from a pointer.

- ```rust
  pub fn is_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn is_always_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn get_mut(self: &mut Self) -> &mut u8 { /* ... */ }
  ```
  Returns a mutable reference to the underlying integer.

- ```rust
  pub const fn into_inner(self: Self) -> u8 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> u8 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: u8, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: u8, order: Ordering) -> u8 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous value.

- ```rust
  pub fn compare_exchange(self: &Self, current: u8, new: u8, success: Ordering, failure: Ordering) -> Result<u8, u8> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: u8, new: u8, success: Ordering, failure: Ordering) -> Result<u8, u8> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn fetch_add(self: &Self, val: u8, order: Ordering) -> u8 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn add(self: &Self, val: u8, order: Ordering) { /* ... */ }
  ```
  Adds to the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: u8, order: Ordering) -> u8 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn sub(self: &Self, val: u8, order: Ordering) { /* ... */ }
  ```
  Subtracts from the current value.

- ```rust
  pub fn fetch_and(self: &Self, val: u8, order: Ordering) -> u8 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn and(self: &Self, val: u8, order: Ordering) { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: u8, order: Ordering) -> u8 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: u8, order: Ordering) -> u8 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn or(self: &Self, val: u8, order: Ordering) { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_xor(self: &Self, val: u8, order: Ordering) -> u8 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn xor(self: &Self, val: u8, order: Ordering) { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u8, u8>
where
    F: FnMut(u8) -> Option<u8> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an optional

- ```rust
  pub fn fetch_max(self: &Self, val: u8, order: Ordering) -> u8 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: u8, order: Ordering) -> u8 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Sets the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Clears the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Toggles the bit at the specified bit-position.

- ```rust
  pub fn fetch_not(self: &Self, order: Ordering) -> u8 { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn not(self: &Self, order: Ordering) { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn fetch_neg(self: &Self, order: Ordering) -> u8 { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub fn neg(self: &Self, order: Ordering) { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub const fn as_ptr(self: &Self) -> *mut u8 { /* ... */ }
  ```
  Returns a mutable pointer to the underlying integer.

##### Trait Implementations

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(v: u8) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

### Struct `AtomicI16`

**Attributes:**

- `#[repr(C, align(2))]`

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`i16`].

If the compiler and the platform support atomic loads and stores of [`i16`], this type is a wrapper for the standard library's `AtomicI16`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call [`AtomicI16::is_lock_free()`] to check whether
atomic instructions or locks will be used.

```rust
pub struct AtomicI16 {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(v: i16) -> Self { /* ... */ }
  ```
  Creates a new atomic integer.

- ```rust
  pub const unsafe fn from_ptr<''a>(ptr: *mut i16) -> &''a Self { /* ... */ }
  ```
  Creates a new reference to an atomic integer from a pointer.

- ```rust
  pub fn is_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn is_always_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn get_mut(self: &mut Self) -> &mut i16 { /* ... */ }
  ```
  Returns a mutable reference to the underlying integer.

- ```rust
  pub const fn into_inner(self: Self) -> i16 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> i16 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: i16, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous value.

- ```rust
  pub fn compare_exchange(self: &Self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn fetch_add(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn add(self: &Self, val: i16, order: Ordering) { /* ... */ }
  ```
  Adds to the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn sub(self: &Self, val: i16, order: Ordering) { /* ... */ }
  ```
  Subtracts from the current value.

- ```rust
  pub fn fetch_and(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn and(self: &Self, val: i16, order: Ordering) { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn or(self: &Self, val: i16, order: Ordering) { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_xor(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn xor(self: &Self, val: i16, order: Ordering) { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i16, i16>
where
    F: FnMut(i16) -> Option<i16> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an optional

- ```rust
  pub fn fetch_max(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: i16, order: Ordering) -> i16 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Sets the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Clears the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Toggles the bit at the specified bit-position.

- ```rust
  pub fn fetch_not(self: &Self, order: Ordering) -> i16 { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn not(self: &Self, order: Ordering) { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn fetch_neg(self: &Self, order: Ordering) -> i16 { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub fn neg(self: &Self, order: Ordering) { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub const fn as_ptr(self: &Self) -> *mut i16 { /* ... */ }
  ```
  Returns a mutable pointer to the underlying integer.

##### Trait Implementations

- **UnwindSafe**
- **RefUnwindSafe**
- **Freeze**
- **Sync**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(v: i16) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Struct `AtomicU16`

**Attributes:**

- `#[repr(C, align(2))]`

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`u16`].

If the compiler and the platform support atomic loads and stores of [`u16`], this type is a wrapper for the standard library's `AtomicU16`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call [`AtomicU16::is_lock_free()`] to check whether
atomic instructions or locks will be used.

```rust
pub struct AtomicU16 {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(v: u16) -> Self { /* ... */ }
  ```
  Creates a new atomic integer.

- ```rust
  pub const unsafe fn from_ptr<''a>(ptr: *mut u16) -> &''a Self { /* ... */ }
  ```
  Creates a new reference to an atomic integer from a pointer.

- ```rust
  pub fn is_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn is_always_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn get_mut(self: &mut Self) -> &mut u16 { /* ... */ }
  ```
  Returns a mutable reference to the underlying integer.

- ```rust
  pub const fn into_inner(self: Self) -> u16 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> u16 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: u16, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous value.

- ```rust
  pub fn compare_exchange(self: &Self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn fetch_add(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn add(self: &Self, val: u16, order: Ordering) { /* ... */ }
  ```
  Adds to the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn sub(self: &Self, val: u16, order: Ordering) { /* ... */ }
  ```
  Subtracts from the current value.

- ```rust
  pub fn fetch_and(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn and(self: &Self, val: u16, order: Ordering) { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn or(self: &Self, val: u16, order: Ordering) { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_xor(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn xor(self: &Self, val: u16, order: Ordering) { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u16, u16>
where
    F: FnMut(u16) -> Option<u16> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an optional

- ```rust
  pub fn fetch_max(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: u16, order: Ordering) -> u16 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Sets the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Clears the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Toggles the bit at the specified bit-position.

- ```rust
  pub fn fetch_not(self: &Self, order: Ordering) -> u16 { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn not(self: &Self, order: Ordering) { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn fetch_neg(self: &Self, order: Ordering) -> u16 { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub fn neg(self: &Self, order: Ordering) { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub const fn as_ptr(self: &Self) -> *mut u16 { /* ... */ }
  ```
  Returns a mutable pointer to the underlying integer.

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(v: u16) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Struct `AtomicI32`

**Attributes:**

- `#[repr(C, align(4))]`

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`i32`].

If the compiler and the platform support atomic loads and stores of [`i32`], this type is a wrapper for the standard library's `AtomicI32`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call [`AtomicI32::is_lock_free()`] to check whether
atomic instructions or locks will be used.

```rust
pub struct AtomicI32 {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(v: i32) -> Self { /* ... */ }
  ```
  Creates a new atomic integer.

- ```rust
  pub const unsafe fn from_ptr<''a>(ptr: *mut i32) -> &''a Self { /* ... */ }
  ```
  Creates a new reference to an atomic integer from a pointer.

- ```rust
  pub fn is_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn is_always_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn get_mut(self: &mut Self) -> &mut i32 { /* ... */ }
  ```
  Returns a mutable reference to the underlying integer.

- ```rust
  pub const fn into_inner(self: Self) -> i32 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> i32 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: i32, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous value.

- ```rust
  pub fn compare_exchange(self: &Self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn fetch_add(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn add(self: &Self, val: i32, order: Ordering) { /* ... */ }
  ```
  Adds to the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn sub(self: &Self, val: i32, order: Ordering) { /* ... */ }
  ```
  Subtracts from the current value.

- ```rust
  pub fn fetch_and(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn and(self: &Self, val: i32, order: Ordering) { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn or(self: &Self, val: i32, order: Ordering) { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_xor(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn xor(self: &Self, val: i32, order: Ordering) { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i32, i32>
where
    F: FnMut(i32) -> Option<i32> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an optional

- ```rust
  pub fn fetch_max(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: i32, order: Ordering) -> i32 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Sets the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Clears the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Toggles the bit at the specified bit-position.

- ```rust
  pub fn fetch_not(self: &Self, order: Ordering) -> i32 { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn not(self: &Self, order: Ordering) { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn fetch_neg(self: &Self, order: Ordering) -> i32 { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub fn neg(self: &Self, order: Ordering) { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub const fn as_ptr(self: &Self) -> *mut i32 { /* ... */ }
  ```
  Returns a mutable pointer to the underlying integer.

##### Trait Implementations

- **Send**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(v: i32) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `AtomicU32`

**Attributes:**

- `#[repr(C, align(4))]`

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`u32`].

If the compiler and the platform support atomic loads and stores of [`u32`], this type is a wrapper for the standard library's `AtomicU32`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call [`AtomicU32::is_lock_free()`] to check whether
atomic instructions or locks will be used.

```rust
pub struct AtomicU32 {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(v: u32) -> Self { /* ... */ }
  ```
  Creates a new atomic integer.

- ```rust
  pub const unsafe fn from_ptr<''a>(ptr: *mut u32) -> &''a Self { /* ... */ }
  ```
  Creates a new reference to an atomic integer from a pointer.

- ```rust
  pub fn is_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn is_always_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn get_mut(self: &mut Self) -> &mut u32 { /* ... */ }
  ```
  Returns a mutable reference to the underlying integer.

- ```rust
  pub const fn into_inner(self: Self) -> u32 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> u32 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: u32, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous value.

- ```rust
  pub fn compare_exchange(self: &Self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn fetch_add(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn add(self: &Self, val: u32, order: Ordering) { /* ... */ }
  ```
  Adds to the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn sub(self: &Self, val: u32, order: Ordering) { /* ... */ }
  ```
  Subtracts from the current value.

- ```rust
  pub fn fetch_and(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn and(self: &Self, val: u32, order: Ordering) { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn or(self: &Self, val: u32, order: Ordering) { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_xor(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn xor(self: &Self, val: u32, order: Ordering) { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u32, u32>
where
    F: FnMut(u32) -> Option<u32> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an optional

- ```rust
  pub fn fetch_max(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: u32, order: Ordering) -> u32 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Sets the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Clears the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Toggles the bit at the specified bit-position.

- ```rust
  pub fn fetch_not(self: &Self, order: Ordering) -> u32 { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn not(self: &Self, order: Ordering) { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn fetch_neg(self: &Self, order: Ordering) -> u32 { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub fn neg(self: &Self, order: Ordering) { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub const fn as_ptr(self: &Self) -> *mut u32 { /* ... */ }
  ```
  Returns a mutable pointer to the underlying integer.

##### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(v: u32) -> Self { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `AtomicI64`

**Attributes:**

- `#[repr(C, align(8))]`

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`i64`].

If the compiler and the platform support atomic loads and stores of [`i64`], this type is a wrapper for the standard library's `AtomicI64`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call [`AtomicI64::is_lock_free()`] to check whether
atomic instructions or locks will be used.

```rust
pub struct AtomicI64 {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(v: i64) -> Self { /* ... */ }
  ```
  Creates a new atomic integer.

- ```rust
  pub const unsafe fn from_ptr<''a>(ptr: *mut i64) -> &''a Self { /* ... */ }
  ```
  Creates a new reference to an atomic integer from a pointer.

- ```rust
  pub fn is_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn is_always_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn get_mut(self: &mut Self) -> &mut i64 { /* ... */ }
  ```
  Returns a mutable reference to the underlying integer.

- ```rust
  pub const fn into_inner(self: Self) -> i64 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> i64 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: i64, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous value.

- ```rust
  pub fn compare_exchange(self: &Self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn fetch_add(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn add(self: &Self, val: i64, order: Ordering) { /* ... */ }
  ```
  Adds to the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn sub(self: &Self, val: i64, order: Ordering) { /* ... */ }
  ```
  Subtracts from the current value.

- ```rust
  pub fn fetch_and(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn and(self: &Self, val: i64, order: Ordering) { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn or(self: &Self, val: i64, order: Ordering) { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_xor(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn xor(self: &Self, val: i64, order: Ordering) { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i64, i64>
where
    F: FnMut(i64) -> Option<i64> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an optional

- ```rust
  pub fn fetch_max(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: i64, order: Ordering) -> i64 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Sets the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Clears the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Toggles the bit at the specified bit-position.

- ```rust
  pub fn fetch_not(self: &Self, order: Ordering) -> i64 { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn not(self: &Self, order: Ordering) { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn fetch_neg(self: &Self, order: Ordering) -> i64 { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub fn neg(self: &Self, order: Ordering) { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub const fn as_ptr(self: &Self) -> *mut i64 { /* ... */ }
  ```
  Returns a mutable pointer to the underlying integer.

##### Trait Implementations

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **RefUnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(v: i64) -> Self { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

### Struct `AtomicU64`

**Attributes:**

- `#[repr(C, align(8))]`

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`u64`].

If the compiler and the platform support atomic loads and stores of [`u64`], this type is a wrapper for the standard library's `AtomicU64`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call [`AtomicU64::is_lock_free()`] to check whether
atomic instructions or locks will be used.

```rust
pub struct AtomicU64 {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(v: u64) -> Self { /* ... */ }
  ```
  Creates a new atomic integer.

- ```rust
  pub const unsafe fn from_ptr<''a>(ptr: *mut u64) -> &''a Self { /* ... */ }
  ```
  Creates a new reference to an atomic integer from a pointer.

- ```rust
  pub fn is_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn is_always_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn get_mut(self: &mut Self) -> &mut u64 { /* ... */ }
  ```
  Returns a mutable reference to the underlying integer.

- ```rust
  pub const fn into_inner(self: Self) -> u64 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> u64 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: u64, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous value.

- ```rust
  pub fn compare_exchange(self: &Self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn fetch_add(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn add(self: &Self, val: u64, order: Ordering) { /* ... */ }
  ```
  Adds to the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn sub(self: &Self, val: u64, order: Ordering) { /* ... */ }
  ```
  Subtracts from the current value.

- ```rust
  pub fn fetch_and(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn and(self: &Self, val: u64, order: Ordering) { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn or(self: &Self, val: u64, order: Ordering) { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_xor(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn xor(self: &Self, val: u64, order: Ordering) { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u64, u64>
where
    F: FnMut(u64) -> Option<u64> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an optional

- ```rust
  pub fn fetch_max(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: u64, order: Ordering) -> u64 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Sets the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Clears the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Toggles the bit at the specified bit-position.

- ```rust
  pub fn fetch_not(self: &Self, order: Ordering) -> u64 { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn not(self: &Self, order: Ordering) { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn fetch_neg(self: &Self, order: Ordering) -> u64 { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub fn neg(self: &Self, order: Ordering) { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub const fn as_ptr(self: &Self) -> *mut u64 { /* ... */ }
  ```
  Returns a mutable pointer to the underlying integer.

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(v: u64) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
### Struct `AtomicI128`

**Attributes:**

- `#[repr(C, align(16))]`

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`i128`].

If the compiler and the platform support atomic loads and stores of [`i128`], this type is a wrapper for the standard library's `AtomicI128`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call [`AtomicI128::is_lock_free()`] to check whether
atomic instructions or locks will be used.

```rust
pub struct AtomicI128 {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(v: i128) -> Self { /* ... */ }
  ```
  Creates a new atomic integer.

- ```rust
  pub const unsafe fn from_ptr<''a>(ptr: *mut i128) -> &''a Self { /* ... */ }
  ```
  Creates a new reference to an atomic integer from a pointer.

- ```rust
  pub fn is_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn is_always_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn get_mut(self: &mut Self) -> &mut i128 { /* ... */ }
  ```
  Returns a mutable reference to the underlying integer.

- ```rust
  pub const fn into_inner(self: Self) -> i128 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> i128 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: i128, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: i128, order: Ordering) -> i128 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous value.

- ```rust
  pub fn compare_exchange(self: &Self, current: i128, new: i128, success: Ordering, failure: Ordering) -> Result<i128, i128> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: i128, new: i128, success: Ordering, failure: Ordering) -> Result<i128, i128> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn fetch_add(self: &Self, val: i128, order: Ordering) -> i128 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn add(self: &Self, val: i128, order: Ordering) { /* ... */ }
  ```
  Adds to the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: i128, order: Ordering) -> i128 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn sub(self: &Self, val: i128, order: Ordering) { /* ... */ }
  ```
  Subtracts from the current value.

- ```rust
  pub fn fetch_and(self: &Self, val: i128, order: Ordering) -> i128 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn and(self: &Self, val: i128, order: Ordering) { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: i128, order: Ordering) -> i128 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: i128, order: Ordering) -> i128 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn or(self: &Self, val: i128, order: Ordering) { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_xor(self: &Self, val: i128, order: Ordering) -> i128 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn xor(self: &Self, val: i128, order: Ordering) { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i128, i128>
where
    F: FnMut(i128) -> Option<i128> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an optional

- ```rust
  pub fn fetch_max(self: &Self, val: i128, order: Ordering) -> i128 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: i128, order: Ordering) -> i128 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Sets the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Clears the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Toggles the bit at the specified bit-position.

- ```rust
  pub fn fetch_not(self: &Self, order: Ordering) -> i128 { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn not(self: &Self, order: Ordering) { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn fetch_neg(self: &Self, order: Ordering) -> i128 { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub fn neg(self: &Self, order: Ordering) { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub const fn as_ptr(self: &Self) -> *mut i128 { /* ... */ }
  ```
  Returns a mutable pointer to the underlying integer.

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(v: i128) -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **UnwindSafe**
- **Sync**
### Struct `AtomicU128`

**Attributes:**

- `#[repr(C, align(16))]`

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`u128`].

If the compiler and the platform support atomic loads and stores of [`u128`], this type is a wrapper for the standard library's `AtomicU128`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call [`AtomicU128::is_lock_free()`] to check whether
atomic instructions or locks will be used.

```rust
pub struct AtomicU128 {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub const fn new(v: u128) -> Self { /* ... */ }
  ```
  Creates a new atomic integer.

- ```rust
  pub const unsafe fn from_ptr<''a>(ptr: *mut u128) -> &''a Self { /* ... */ }
  ```
  Creates a new reference to an atomic integer from a pointer.

- ```rust
  pub fn is_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn is_always_lock_free() -> bool { /* ... */ }
  ```
  Returns `true` if operations on values of this type are lock-free.

- ```rust
  pub const fn get_mut(self: &mut Self) -> &mut u128 { /* ... */ }
  ```
  Returns a mutable reference to the underlying integer.

- ```rust
  pub const fn into_inner(self: Self) -> u128 { /* ... */ }
  ```
  Consumes the atomic and returns the contained value.

- ```rust
  pub fn load(self: &Self, order: Ordering) -> u128 { /* ... */ }
  ```
  Loads a value from the atomic integer.

- ```rust
  pub fn store(self: &Self, val: u128, order: Ordering) { /* ... */ }
  ```
  Stores a value into the atomic integer.

- ```rust
  pub fn swap(self: &Self, val: u128, order: Ordering) -> u128 { /* ... */ }
  ```
  Stores a value into the atomic integer, returning the previous value.

- ```rust
  pub fn compare_exchange(self: &Self, current: u128, new: u128, success: Ordering, failure: Ordering) -> Result<u128, u128> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn compare_exchange_weak(self: &Self, current: u128, new: u128, success: Ordering, failure: Ordering) -> Result<u128, u128> { /* ... */ }
  ```
  Stores a value into the atomic integer if the current value is the same as

- ```rust
  pub fn fetch_add(self: &Self, val: u128, order: Ordering) -> u128 { /* ... */ }
  ```
  Adds to the current value, returning the previous value.

- ```rust
  pub fn add(self: &Self, val: u128, order: Ordering) { /* ... */ }
  ```
  Adds to the current value.

- ```rust
  pub fn fetch_sub(self: &Self, val: u128, order: Ordering) -> u128 { /* ... */ }
  ```
  Subtracts from the current value, returning the previous value.

- ```rust
  pub fn sub(self: &Self, val: u128, order: Ordering) { /* ... */ }
  ```
  Subtracts from the current value.

- ```rust
  pub fn fetch_and(self: &Self, val: u128, order: Ordering) -> u128 { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn and(self: &Self, val: u128, order: Ordering) { /* ... */ }
  ```
  Bitwise "and" with the current value.

- ```rust
  pub fn fetch_nand(self: &Self, val: u128, order: Ordering) -> u128 { /* ... */ }
  ```
  Bitwise "nand" with the current value.

- ```rust
  pub fn fetch_or(self: &Self, val: u128, order: Ordering) -> u128 { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn or(self: &Self, val: u128, order: Ordering) { /* ... */ }
  ```
  Bitwise "or" with the current value.

- ```rust
  pub fn fetch_xor(self: &Self, val: u128, order: Ordering) -> u128 { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn xor(self: &Self, val: u128, order: Ordering) { /* ... */ }
  ```
  Bitwise "xor" with the current value.

- ```rust
  pub fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u128, u128>
where
    F: FnMut(u128) -> Option<u128> { /* ... */ }
  ```
  Fetches the value, and applies a function to it that returns an optional

- ```rust
  pub fn fetch_max(self: &Self, val: u128, order: Ordering) -> u128 { /* ... */ }
  ```
  Maximum with the current value.

- ```rust
  pub fn fetch_min(self: &Self, val: u128, order: Ordering) -> u128 { /* ... */ }
  ```
  Minimum with the current value.

- ```rust
  pub fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Sets the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Clears the bit at the specified bit-position to 1.

- ```rust
  pub fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool { /* ... */ }
  ```
  Toggles the bit at the specified bit-position.

- ```rust
  pub fn fetch_not(self: &Self, order: Ordering) -> u128 { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn not(self: &Self, order: Ordering) { /* ... */ }
  ```
  Logical negates the current value, and sets the new value to the result.

- ```rust
  pub fn fetch_neg(self: &Self, order: Ordering) -> u128 { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub fn neg(self: &Self, order: Ordering) { /* ... */ }
  ```
  Negates the current value, and sets the new value to the result.

- ```rust
  pub const fn as_ptr(self: &Self) -> *mut u128 { /* ... */ }
  ```
  Returns a mutable pointer to the underlying integer.

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(v: u128) -> Self { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
## Macros

### Macro `cfg_has_atomic_8`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! cfg_has_atomic_8 {
    /* macro_rules! cfg_has_atomic_8 {
    ($($tt:tt)*) => { ... };
} */
}
```

### Macro `cfg_no_atomic_8`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! cfg_no_atomic_8 {
    /* macro_rules! cfg_no_atomic_8 {
    ($($tt:tt)*) => { ... };
} */
}
```

### Macro `cfg_has_atomic_16`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! cfg_has_atomic_16 {
    /* macro_rules! cfg_has_atomic_16 {
    ($($tt:tt)*) => { ... };
} */
}
```

### Macro `cfg_no_atomic_16`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! cfg_no_atomic_16 {
    /* macro_rules! cfg_no_atomic_16 {
    ($($tt:tt)*) => { ... };
} */
}
```

### Macro `cfg_has_atomic_32`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! cfg_has_atomic_32 {
    /* macro_rules! cfg_has_atomic_32 {
    ($($tt:tt)*) => { ... };
} */
}
```

### Macro `cfg_no_atomic_32`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! cfg_no_atomic_32 {
    /* macro_rules! cfg_no_atomic_32 {
    ($($tt:tt)*) => { ... };
} */
}
```

### Macro `cfg_has_atomic_64`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! cfg_has_atomic_64 {
    /* macro_rules! cfg_has_atomic_64 {
    ($($tt:tt)*) => { ... };
} */
}
```

### Macro `cfg_no_atomic_64`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! cfg_no_atomic_64 {
    /* macro_rules! cfg_no_atomic_64 {
    ($($tt:tt)*) => { ... };
} */
}
```

### Macro `cfg_has_atomic_128`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! cfg_has_atomic_128 {
    /* macro_rules! cfg_has_atomic_128 {
    ($($tt:tt)*) => { ... };
} */
}
```

### Macro `cfg_no_atomic_128`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! cfg_no_atomic_128 {
    /* macro_rules! cfg_no_atomic_128 {
    ($($tt:tt)*) => { ... };
} */
}
```

### Macro `cfg_has_atomic_cas`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! cfg_has_atomic_cas {
    /* macro_rules! cfg_has_atomic_cas {
    ($($tt:tt)*) => { ... };
} */
}
```

### Macro `cfg_no_atomic_cas`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! cfg_no_atomic_cas {
    /* macro_rules! cfg_no_atomic_cas {
    ($($tt:tt)*) => { ... };
} */
}
```

## Re-exports

### Re-export `cfg_has_atomic_64`

**Attributes:**

- `#[<cfg>(target_pointer_width = "64")]`

```rust
pub use self::cfg_has_atomic_64 as cfg_has_atomic_ptr;
```

### Re-export `cfg_no_atomic_64`

**Attributes:**

- `#[<cfg>(target_pointer_width = "64")]`

```rust
pub use self::cfg_no_atomic_64 as cfg_no_atomic_ptr;
```

### Re-export `Ordering`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use core::sync::atomic::Ordering;
```

### Re-export `compiler_fence`

**Attributes:**

- `#[doc(no_inline)]`
- `#[<cfg>(not(target_arch = "msp430"))]`

```rust
pub use core::sync::atomic::compiler_fence;
```

### Re-export `fence`

**Attributes:**

- `#[doc(no_inline)]`
- `#[<cfg>(not(target_arch = "msp430"))]`

```rust
pub use core::sync::atomic::fence;
```

