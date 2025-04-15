# Crate Documentation

**Version:** 0.9.4

**Format Version:** 43

# Module `linux_raw_sys`

## Modules

## Module `ctypes`

**Attributes:**

- `#[<cfg>(all(not(feature = "std"), feature = "no_std"))]`

```rust
pub mod ctypes { /* ... */ }
```

### Types

#### Type Alias `c_char`

**Attributes:**

- `#[<cfg>(any(target_arch = "aarch64", target_arch = "arm", target_arch =
"msp430", target_arch = "powerpc", target_arch = "powerpc64", target_arch =
"riscv32", target_arch = "riscv64", target_arch = "s390x",))]`

```rust
pub type c_char = c_uchar;
```

#### Type Alias `c_schar`

```rust
pub type c_schar = i8;
```

#### Type Alias `c_uchar`

```rust
pub type c_uchar = u8;
```

#### Type Alias `c_short`

```rust
pub type c_short = i16;
```

#### Type Alias `c_ushort`

```rust
pub type c_ushort = u16;
```

#### Type Alias `c_int`

```rust
pub type c_int = i32;
```

#### Type Alias `c_uint`

```rust
pub type c_uint = u32;
```

#### Type Alias `c_long`

**Attributes:**

- `#[<cfg>(target_pointer_width = "64")]`

```rust
pub type c_long = i64;
```

#### Type Alias `c_ulong`

**Attributes:**

- `#[<cfg>(target_pointer_width = "64")]`

```rust
pub type c_ulong = u64;
```

#### Type Alias `c_longlong`

```rust
pub type c_longlong = i64;
```

#### Type Alias `c_ulonglong`

```rust
pub type c_ulonglong = u64;
```

#### Type Alias `c_float`

```rust
pub type c_float = f32;
```

#### Type Alias `c_double`

```rust
pub type c_double = f64;
```

### Re-exports

#### Re-export `c_void`

```rust
pub use core::ffi::c_void;
```

## Module `select_macros`

**Attributes:**

- `#[<cfg>(feature = "general")]`

```rust
pub mod select_macros { /* ... */ }
```

### Functions

#### Function `FD_CLR`

```rust
pub unsafe fn FD_CLR(fd: crate::ctypes::c_int, set: *mut crate::general::__kernel_fd_set) { /* ... */ }
```

#### Function `FD_SET`

```rust
pub unsafe fn FD_SET(fd: crate::ctypes::c_int, set: *mut crate::general::__kernel_fd_set) { /* ... */ }
```

#### Function `FD_ISSET`

```rust
pub unsafe fn FD_ISSET(fd: crate::ctypes::c_int, set: *const crate::general::__kernel_fd_set) -> bool { /* ... */ }
```

#### Function `FD_ZERO`

```rust
pub unsafe fn FD_ZERO(set: *mut crate::general::__kernel_fd_set) { /* ... */ }
```

## Module `signal_macros`

**Attributes:**

- `#[<cfg>(feature = "general")]`

```rust
pub mod signal_macros { /* ... */ }
```

### Functions

#### Function `sig_ign`

**Attributes:**

- `#[inline]`

Rust doesn't currently permit us to use `transmute` to convert the
`SIG_IGN` value into a function pointer in a `const` initializer, so
we make it a function instead.


```rust
pub const fn sig_ign() -> super::general::__kernel_sighandler_t { /* ... */ }
```

### Constants and Statics

#### Constant `SIG_DFL`

```rust
pub const SIG_DFL: super::general::__kernel_sighandler_t = None;
```

## Module `elf`

**Attributes:**

- `#[<cfg>(feature = "elf")]`

The ELF ABI. 🧝

This module is not as comprehensive as bindgened [`elf_uapi`] and provides only types for target
pointer width: instead of [`elf32_phdr`] and [`elf64_phdr`] there's only [`Elf_Phdr`].

[`elf_uapi`]: super::elf_uapi
[`elf32_phdr`]: super::elf_uapi::elf32_phdr
[`elf64_phdr`]: super::elf_uapi::elf64_phdr

```rust
pub mod elf { /* ... */ }
```

### Types

#### Struct `Elf_Ehdr`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct Elf_Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: usize,
    pub e_phoff: usize,
    pub e_shoff: usize,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `e_ident` | `[u8; 16]` |  |
| `e_type` | `u16` |  |
| `e_machine` | `u16` |  |
| `e_version` | `u32` |  |
| `e_entry` | `usize` |  |
| `e_phoff` | `usize` |  |
| `e_shoff` | `usize` |  |
| `e_flags` | `u32` |  |
| `e_ehsize` | `u16` |  |
| `e_phentsize` | `u16` |  |
| `e_phnum` | `u16` |  |
| `e_shentsize` | `u16` |  |
| `e_shnum` | `u16` |  |
| `e_shstrndx` | `u16` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
#### Struct `Elf_Phdr`

**Attributes:**

- `#[<cfg>(target_pointer_width = "64")]`
- `#[repr(C)]`

```rust
pub struct Elf_Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: usize,
    pub p_vaddr: usize,
    pub p_paddr: usize,
    pub p_filesz: usize,
    pub p_memsz: usize,
    pub p_align: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `p_type` | `u32` |  |
| `p_flags` | `u32` |  |
| `p_offset` | `usize` |  |
| `p_vaddr` | `usize` |  |
| `p_paddr` | `usize` |  |
| `p_filesz` | `usize` |  |
| `p_memsz` | `usize` |  |
| `p_align` | `usize` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
#### Struct `Elf_Sym`

**Attributes:**

- `#[<cfg>(target_pointer_width = "64")]`
- `#[repr(C)]`

```rust
pub struct Elf_Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: usize,
    pub st_size: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `st_name` | `u32` |  |
| `st_info` | `u8` |  |
| `st_other` | `u8` |  |
| `st_shndx` | `u16` |  |
| `st_value` | `usize` |  |
| `st_size` | `usize` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `Elf_Verdef`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct Elf_Verdef {
    pub vd_version: u16,
    pub vd_flags: u16,
    pub vd_ndx: u16,
    pub vd_cnt: u16,
    pub vd_hash: u32,
    pub vd_aux: u32,
    pub vd_next: u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `vd_version` | `u16` |  |
| `vd_flags` | `u16` |  |
| `vd_ndx` | `u16` |  |
| `vd_cnt` | `u16` |  |
| `vd_hash` | `u32` |  |
| `vd_aux` | `u32` |  |
| `vd_next` | `u32` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
- **Unpin**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `Elf_Verdaux`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct Elf_Verdaux {
    pub vda_name: u32,
    pub _vda_next: u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `vda_name` | `u32` |  |
| `_vda_next` | `u32` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Elf_Dyn`

**Attributes:**

- `#[<cfg>(target_pointer_width = "64")]`
- `#[repr(C)]`

```rust
pub struct Elf_Dyn {
    pub d_tag: usize,
    pub d_un: Elf_Dyn_Union,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `d_tag` | `usize` |  |
| `d_un` | `Elf_Dyn_Union` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Freeze**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Elf_Dyn { /* ... */ }
    ```

- **UnwindSafe**
#### Union `Elf_Dyn_Union`

**Attributes:**

- `#[<cfg>(target_pointer_width = "64")]`
- `#[repr(C)]`

```rust
pub union Elf_Dyn_Union {
    pub d_val: u64,
    pub d_ptr: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `d_val` | `u64` |  |
| `d_ptr` | `usize` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - `Error`: 
  - `try_into`: 
- **Send**
- **BorrowMut**
  - `borrow_mut`: 
- **TryFrom**
  - `Error`: 
  - `try_from`: 
- **Borrow**
  - `borrow`: 
- **Into**
  - `into`: Calls `U::from(self)`.
- **From**
  - `from`: Returns the argument unchanged.
- **UnwindSafe**
- **Unpin**
- **RefUnwindSafe**
- **Any**
  - `type_id`: 
- **Sync**
- **CloneToUninit**
  - `clone_to_uninit`: 
- **Copy**
- **Freeze**
- **Clone**
  - `clone`: 

#### Struct `Elf_Rela`

**Attributes:**

- `#[<cfg>(target_pointer_width = "64")]`
- `#[repr(C)]`

```rust
pub struct Elf_Rela {
    pub r_offset: usize,
    pub r_info: u64,
    pub r_addend: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `r_offset` | `usize` |  |
| `r_info` | `u64` |  |
| `r_addend` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn type_(self: &Self) -> u32 { /* ... */ }
  ```

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Sync**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Elf_Rel`

**Attributes:**

- `#[<cfg>(target_pointer_width = "64")]`
- `#[repr(C)]`

```rust
pub struct Elf_Rel {
    pub r_offset: usize,
    pub r_info: u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `r_offset` | `usize` |  |
| `r_info` | `u64` |  |

##### Implementations

###### Methods

- ```rust
  pub fn type_(self: &Self) -> u32 { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Elf_auxv_t`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct Elf_auxv_t {
    pub a_type: usize,
    pub a_val: *mut crate::ctypes::c_void,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `a_type` | `usize` |  |
| `a_val` | `*mut crate::ctypes::c_void` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Elf_auxv_t { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
### Functions

#### Function `ELF_ST_VISIBILITY`

**Attributes:**

- `#[inline]`

```rust
pub const fn ELF_ST_VISIBILITY(o: u8) -> u8 { /* ... */ }
```

#### Function `ELF_ST_BIND`

**Attributes:**

- `#[inline]`

```rust
pub const fn ELF_ST_BIND(val: u8) -> u8 { /* ... */ }
```

#### Function `ELF_ST_TYPE`

**Attributes:**

- `#[inline]`

```rust
pub const fn ELF_ST_TYPE(val: u8) -> u8 { /* ... */ }
```

### Constants and Statics

#### Constant `SELFMAG`

```rust
pub const SELFMAG: usize = 4;
```

#### Constant `ELFMAG`

```rust
pub const ELFMAG: [u8; 4] = _;
```

#### Constant `EI_CLASS`

```rust
pub const EI_CLASS: usize = 4;
```

#### Constant `EI_DATA`

```rust
pub const EI_DATA: usize = 5;
```

#### Constant `EI_VERSION`

```rust
pub const EI_VERSION: usize = 6;
```

#### Constant `EI_OSABI`

```rust
pub const EI_OSABI: usize = 7;
```

#### Constant `EI_ABIVERSION`

```rust
pub const EI_ABIVERSION: usize = 8;
```

#### Constant `EV_CURRENT`

```rust
pub const EV_CURRENT: u8 = 1;
```

#### Constant `ELFCLASS`

**Attributes:**

- `#[<cfg>(target_pointer_width = "64")]`

```rust
pub const ELFCLASS: u8 = 2;
```

#### Constant `ELFDATA`

**Attributes:**

- `#[<cfg>(target_endian = "little")]`

```rust
pub const ELFDATA: u8 = 1;
```

#### Constant `ELFOSABI_SYSV`

```rust
pub const ELFOSABI_SYSV: u8 = 0;
```

#### Constant `ELFOSABI_LINUX`

```rust
pub const ELFOSABI_LINUX: u8 = 3;
```

#### Constant `ELFABIVERSION`

```rust
pub const ELFABIVERSION: u8 = 0;
```

#### Constant `ET_DYN`

```rust
pub const ET_DYN: u16 = 3;
```

#### Constant `EI_NIDENT`

```rust
pub const EI_NIDENT: usize = 16;
```

#### Constant `SHN_UNDEF`

```rust
pub const SHN_UNDEF: u16 = 0;
```

#### Constant `SHN_ABS`

```rust
pub const SHN_ABS: u16 = 0xfff1;
```

#### Constant `PN_XNUM`

```rust
pub const PN_XNUM: u16 = 0xffff;
```

#### Constant `PT_LOAD`

```rust
pub const PT_LOAD: u32 = 1;
```

#### Constant `PT_DYNAMIC`

```rust
pub const PT_DYNAMIC: u32 = 2;
```

#### Constant `PT_INTERP`

```rust
pub const PT_INTERP: u32 = 3;
```

#### Constant `PT_PHDR`

```rust
pub const PT_PHDR: u32 = 6;
```

#### Constant `PT_TLS`

```rust
pub const PT_TLS: u32 = 7;
```

#### Constant `PT_GNU_STACK`

```rust
pub const PT_GNU_STACK: u32 = 0x6474_e551;
```

#### Constant `PT_GNU_RELRO`

```rust
pub const PT_GNU_RELRO: u32 = 0x6474_e552;
```

#### Constant `PF_X`

```rust
pub const PF_X: u32 = 1;
```

#### Constant `PF_W`

```rust
pub const PF_W: u32 = 2;
```

#### Constant `PF_R`

```rust
pub const PF_R: u32 = 4;
```

#### Constant `DT_NULL`

```rust
pub const DT_NULL: usize = 0;
```

#### Constant `DT_HASH`

```rust
pub const DT_HASH: usize = 4;
```

#### Constant `DT_STRTAB`

```rust
pub const DT_STRTAB: usize = 5;
```

#### Constant `DT_SYMTAB`

```rust
pub const DT_SYMTAB: usize = 6;
```

#### Constant `DT_RELA`

```rust
pub const DT_RELA: usize = 7;
```

#### Constant `DT_RELASZ`

```rust
pub const DT_RELASZ: usize = 8;
```

#### Constant `DT_RELAENT`

```rust
pub const DT_RELAENT: usize = 9;
```

#### Constant `DT_REL`

```rust
pub const DT_REL: usize = 17;
```

#### Constant `DT_RELSZ`

```rust
pub const DT_RELSZ: usize = 18;
```

#### Constant `DT_RELENT`

```rust
pub const DT_RELENT: usize = 19;
```

#### Constant `DT_SYMENT`

```rust
pub const DT_SYMENT: usize = 11;
```

#### Constant `DT_GNU_HASH`

```rust
pub const DT_GNU_HASH: usize = 0x6fff_fef5;
```

#### Constant `DT_VERSYM`

```rust
pub const DT_VERSYM: usize = 0x6fff_fff0;
```

#### Constant `DT_VERDEF`

```rust
pub const DT_VERDEF: usize = 0x6fff_fffc;
```

#### Constant `STB_WEAK`

```rust
pub const STB_WEAK: u8 = 2;
```

#### Constant `STB_GLOBAL`

```rust
pub const STB_GLOBAL: u8 = 1;
```

#### Constant `STT_NOTYPE`

```rust
pub const STT_NOTYPE: u8 = 0;
```

#### Constant `STT_FUNC`

```rust
pub const STT_FUNC: u8 = 2;
```

#### Constant `STN_UNDEF`

```rust
pub const STN_UNDEF: u32 = 0;
```

#### Constant `VER_FLG_BASE`

```rust
pub const VER_FLG_BASE: u16 = 0x1;
```

#### Constant `VER_DEF_CURRENT`

```rust
pub const VER_DEF_CURRENT: u16 = 1;
```

#### Constant `STV_DEFAULT`

```rust
pub const STV_DEFAULT: u8 = 0;
```

#### Constant `EM_CURRENT`

**Attributes:**

- `#[<cfg>(target_arch = "aarch64")]`

```rust
pub const EM_CURRENT: u16 = 183;
```

#### Constant `R_RELATIVE`

**Attributes:**

- `#[<cfg>(target_arch = "aarch64")]`

```rust
pub const R_RELATIVE: u32 = 1027;
```

## Module `errno`

**Attributes:**

- `#[<cfg>(feature = "errno")]`
- `#[<cfg>(target_arch = "aarch64")]`
- `#[path = "aarch64/errno.rs"]`

```rust
pub mod errno { /* ... */ }
```

### Constants and Statics

#### Constant `EPERM`

```rust
pub const EPERM: u32 = 1;
```

#### Constant `ENOENT`

```rust
pub const ENOENT: u32 = 2;
```

#### Constant `ESRCH`

```rust
pub const ESRCH: u32 = 3;
```

#### Constant `EINTR`

```rust
pub const EINTR: u32 = 4;
```

#### Constant `EIO`

```rust
pub const EIO: u32 = 5;
```

#### Constant `ENXIO`

```rust
pub const ENXIO: u32 = 6;
```

#### Constant `E2BIG`

```rust
pub const E2BIG: u32 = 7;
```

#### Constant `ENOEXEC`

```rust
pub const ENOEXEC: u32 = 8;
```

#### Constant `EBADF`

```rust
pub const EBADF: u32 = 9;
```

#### Constant `ECHILD`

```rust
pub const ECHILD: u32 = 10;
```

#### Constant `EAGAIN`

```rust
pub const EAGAIN: u32 = 11;
```

#### Constant `ENOMEM`

```rust
pub const ENOMEM: u32 = 12;
```

#### Constant `EACCES`

```rust
pub const EACCES: u32 = 13;
```

#### Constant `EFAULT`

```rust
pub const EFAULT: u32 = 14;
```

#### Constant `ENOTBLK`

```rust
pub const ENOTBLK: u32 = 15;
```

#### Constant `EBUSY`

```rust
pub const EBUSY: u32 = 16;
```

#### Constant `EEXIST`

```rust
pub const EEXIST: u32 = 17;
```

#### Constant `EXDEV`

```rust
pub const EXDEV: u32 = 18;
```

#### Constant `ENODEV`

```rust
pub const ENODEV: u32 = 19;
```

#### Constant `ENOTDIR`

```rust
pub const ENOTDIR: u32 = 20;
```

#### Constant `EISDIR`

```rust
pub const EISDIR: u32 = 21;
```

#### Constant `EINVAL`

```rust
pub const EINVAL: u32 = 22;
```

#### Constant `ENFILE`

```rust
pub const ENFILE: u32 = 23;
```

#### Constant `EMFILE`

```rust
pub const EMFILE: u32 = 24;
```

#### Constant `ENOTTY`

```rust
pub const ENOTTY: u32 = 25;
```

#### Constant `ETXTBSY`

```rust
pub const ETXTBSY: u32 = 26;
```

#### Constant `EFBIG`

```rust
pub const EFBIG: u32 = 27;
```

#### Constant `ENOSPC`

```rust
pub const ENOSPC: u32 = 28;
```

#### Constant `ESPIPE`

```rust
pub const ESPIPE: u32 = 29;
```

#### Constant `EROFS`

```rust
pub const EROFS: u32 = 30;
```

#### Constant `EMLINK`

```rust
pub const EMLINK: u32 = 31;
```

#### Constant `EPIPE`

```rust
pub const EPIPE: u32 = 32;
```

#### Constant `EDOM`

```rust
pub const EDOM: u32 = 33;
```

#### Constant `ERANGE`

```rust
pub const ERANGE: u32 = 34;
```

#### Constant `EDEADLK`

```rust
pub const EDEADLK: u32 = 35;
```

#### Constant `ENAMETOOLONG`

```rust
pub const ENAMETOOLONG: u32 = 36;
```

#### Constant `ENOLCK`

```rust
pub const ENOLCK: u32 = 37;
```

#### Constant `ENOSYS`

```rust
pub const ENOSYS: u32 = 38;
```

#### Constant `ENOTEMPTY`

```rust
pub const ENOTEMPTY: u32 = 39;
```

#### Constant `ELOOP`

```rust
pub const ELOOP: u32 = 40;
```

#### Constant `EWOULDBLOCK`

```rust
pub const EWOULDBLOCK: u32 = 11;
```

#### Constant `ENOMSG`

```rust
pub const ENOMSG: u32 = 42;
```

#### Constant `EIDRM`

```rust
pub const EIDRM: u32 = 43;
```

#### Constant `ECHRNG`

```rust
pub const ECHRNG: u32 = 44;
```

#### Constant `EL2NSYNC`

```rust
pub const EL2NSYNC: u32 = 45;
```

#### Constant `EL3HLT`

```rust
pub const EL3HLT: u32 = 46;
```

#### Constant `EL3RST`

```rust
pub const EL3RST: u32 = 47;
```

#### Constant `ELNRNG`

```rust
pub const ELNRNG: u32 = 48;
```

#### Constant `EUNATCH`

```rust
pub const EUNATCH: u32 = 49;
```

#### Constant `ENOCSI`

```rust
pub const ENOCSI: u32 = 50;
```

#### Constant `EL2HLT`

```rust
pub const EL2HLT: u32 = 51;
```

#### Constant `EBADE`

```rust
pub const EBADE: u32 = 52;
```

#### Constant `EBADR`

```rust
pub const EBADR: u32 = 53;
```

#### Constant `EXFULL`

```rust
pub const EXFULL: u32 = 54;
```

#### Constant `ENOANO`

```rust
pub const ENOANO: u32 = 55;
```

#### Constant `EBADRQC`

```rust
pub const EBADRQC: u32 = 56;
```

#### Constant `EBADSLT`

```rust
pub const EBADSLT: u32 = 57;
```

#### Constant `EDEADLOCK`

```rust
pub const EDEADLOCK: u32 = 35;
```

#### Constant `EBFONT`

```rust
pub const EBFONT: u32 = 59;
```

#### Constant `ENOSTR`

```rust
pub const ENOSTR: u32 = 60;
```

#### Constant `ENODATA`

```rust
pub const ENODATA: u32 = 61;
```

#### Constant `ETIME`

```rust
pub const ETIME: u32 = 62;
```

#### Constant `ENOSR`

```rust
pub const ENOSR: u32 = 63;
```

#### Constant `ENONET`

```rust
pub const ENONET: u32 = 64;
```

#### Constant `ENOPKG`

```rust
pub const ENOPKG: u32 = 65;
```

#### Constant `EREMOTE`

```rust
pub const EREMOTE: u32 = 66;
```

#### Constant `ENOLINK`

```rust
pub const ENOLINK: u32 = 67;
```

#### Constant `EADV`

```rust
pub const EADV: u32 = 68;
```

#### Constant `ESRMNT`

```rust
pub const ESRMNT: u32 = 69;
```

#### Constant `ECOMM`

```rust
pub const ECOMM: u32 = 70;
```

#### Constant `EPROTO`

```rust
pub const EPROTO: u32 = 71;
```

#### Constant `EMULTIHOP`

```rust
pub const EMULTIHOP: u32 = 72;
```

#### Constant `EDOTDOT`

```rust
pub const EDOTDOT: u32 = 73;
```

#### Constant `EBADMSG`

```rust
pub const EBADMSG: u32 = 74;
```

#### Constant `EOVERFLOW`

```rust
pub const EOVERFLOW: u32 = 75;
```

#### Constant `ENOTUNIQ`

```rust
pub const ENOTUNIQ: u32 = 76;
```

#### Constant `EBADFD`

```rust
pub const EBADFD: u32 = 77;
```

#### Constant `EREMCHG`

```rust
pub const EREMCHG: u32 = 78;
```

#### Constant `ELIBACC`

```rust
pub const ELIBACC: u32 = 79;
```

#### Constant `ELIBBAD`

```rust
pub const ELIBBAD: u32 = 80;
```

#### Constant `ELIBSCN`

```rust
pub const ELIBSCN: u32 = 81;
```

#### Constant `ELIBMAX`

```rust
pub const ELIBMAX: u32 = 82;
```

#### Constant `ELIBEXEC`

```rust
pub const ELIBEXEC: u32 = 83;
```

#### Constant `EILSEQ`

```rust
pub const EILSEQ: u32 = 84;
```

#### Constant `ERESTART`

```rust
pub const ERESTART: u32 = 85;
```

#### Constant `ESTRPIPE`

```rust
pub const ESTRPIPE: u32 = 86;
```

#### Constant `EUSERS`

```rust
pub const EUSERS: u32 = 87;
```

#### Constant `ENOTSOCK`

```rust
pub const ENOTSOCK: u32 = 88;
```

#### Constant `EDESTADDRREQ`

```rust
pub const EDESTADDRREQ: u32 = 89;
```

#### Constant `EMSGSIZE`

```rust
pub const EMSGSIZE: u32 = 90;
```

#### Constant `EPROTOTYPE`

```rust
pub const EPROTOTYPE: u32 = 91;
```

#### Constant `ENOPROTOOPT`

```rust
pub const ENOPROTOOPT: u32 = 92;
```

#### Constant `EPROTONOSUPPORT`

```rust
pub const EPROTONOSUPPORT: u32 = 93;
```

#### Constant `ESOCKTNOSUPPORT`

```rust
pub const ESOCKTNOSUPPORT: u32 = 94;
```

#### Constant `EOPNOTSUPP`

```rust
pub const EOPNOTSUPP: u32 = 95;
```

#### Constant `EPFNOSUPPORT`

```rust
pub const EPFNOSUPPORT: u32 = 96;
```

#### Constant `EAFNOSUPPORT`

```rust
pub const EAFNOSUPPORT: u32 = 97;
```

#### Constant `EADDRINUSE`

```rust
pub const EADDRINUSE: u32 = 98;
```

#### Constant `EADDRNOTAVAIL`

```rust
pub const EADDRNOTAVAIL: u32 = 99;
```

#### Constant `ENETDOWN`

```rust
pub const ENETDOWN: u32 = 100;
```

#### Constant `ENETUNREACH`

```rust
pub const ENETUNREACH: u32 = 101;
```

#### Constant `ENETRESET`

```rust
pub const ENETRESET: u32 = 102;
```

#### Constant `ECONNABORTED`

```rust
pub const ECONNABORTED: u32 = 103;
```

#### Constant `ECONNRESET`

```rust
pub const ECONNRESET: u32 = 104;
```

#### Constant `ENOBUFS`

```rust
pub const ENOBUFS: u32 = 105;
```

#### Constant `EISCONN`

```rust
pub const EISCONN: u32 = 106;
```

#### Constant `ENOTCONN`

```rust
pub const ENOTCONN: u32 = 107;
```

#### Constant `ESHUTDOWN`

```rust
pub const ESHUTDOWN: u32 = 108;
```

#### Constant `ETOOMANYREFS`

```rust
pub const ETOOMANYREFS: u32 = 109;
```

#### Constant `ETIMEDOUT`

```rust
pub const ETIMEDOUT: u32 = 110;
```

#### Constant `ECONNREFUSED`

```rust
pub const ECONNREFUSED: u32 = 111;
```

#### Constant `EHOSTDOWN`

```rust
pub const EHOSTDOWN: u32 = 112;
```

#### Constant `EHOSTUNREACH`

```rust
pub const EHOSTUNREACH: u32 = 113;
```

#### Constant `EALREADY`

```rust
pub const EALREADY: u32 = 114;
```

#### Constant `EINPROGRESS`

```rust
pub const EINPROGRESS: u32 = 115;
```

#### Constant `ESTALE`

```rust
pub const ESTALE: u32 = 116;
```

#### Constant `EUCLEAN`

```rust
pub const EUCLEAN: u32 = 117;
```

#### Constant `ENOTNAM`

```rust
pub const ENOTNAM: u32 = 118;
```

#### Constant `ENAVAIL`

```rust
pub const ENAVAIL: u32 = 119;
```

#### Constant `EISNAM`

```rust
pub const EISNAM: u32 = 120;
```

#### Constant `EREMOTEIO`

```rust
pub const EREMOTEIO: u32 = 121;
```

#### Constant `EDQUOT`

```rust
pub const EDQUOT: u32 = 122;
```

#### Constant `ENOMEDIUM`

```rust
pub const ENOMEDIUM: u32 = 123;
```

#### Constant `EMEDIUMTYPE`

```rust
pub const EMEDIUMTYPE: u32 = 124;
```

#### Constant `ECANCELED`

```rust
pub const ECANCELED: u32 = 125;
```

#### Constant `ENOKEY`

```rust
pub const ENOKEY: u32 = 126;
```

#### Constant `EKEYEXPIRED`

```rust
pub const EKEYEXPIRED: u32 = 127;
```

#### Constant `EKEYREVOKED`

```rust
pub const EKEYREVOKED: u32 = 128;
```

#### Constant `EKEYREJECTED`

```rust
pub const EKEYREJECTED: u32 = 129;
```

#### Constant `EOWNERDEAD`

```rust
pub const EOWNERDEAD: u32 = 130;
```

#### Constant `ENOTRECOVERABLE`

```rust
pub const ENOTRECOVERABLE: u32 = 131;
```

#### Constant `ERFKILL`

```rust
pub const ERFKILL: u32 = 132;
```

#### Constant `EHWPOISON`

```rust
pub const EHWPOISON: u32 = 133;
```

## Module `general`

**Attributes:**

- `#[<cfg>(feature = "general")]`
- `#[<cfg>(target_arch = "aarch64")]`
- `#[path = "aarch64/general.rs"]`

```rust
pub mod general { /* ... */ }
```

### Types

#### Type Alias `__s8`

```rust
pub type __s8 = crate::ctypes::c_schar;
```

#### Type Alias `__u8`

```rust
pub type __u8 = crate::ctypes::c_uchar;
```

#### Type Alias `__s16`

```rust
pub type __s16 = crate::ctypes::c_short;
```

#### Type Alias `__u16`

```rust
pub type __u16 = crate::ctypes::c_ushort;
```

#### Type Alias `__s32`

```rust
pub type __s32 = crate::ctypes::c_int;
```

#### Type Alias `__u32`

```rust
pub type __u32 = crate::ctypes::c_uint;
```

#### Type Alias `__s64`

```rust
pub type __s64 = crate::ctypes::c_longlong;
```

#### Type Alias `__u64`

```rust
pub type __u64 = crate::ctypes::c_ulonglong;
```

#### Type Alias `__kernel_sighandler_t`

```rust
pub type __kernel_sighandler_t = ::core::option::Option<unsafe extern "C" fn(crate::ctypes::c_int)>;
```

#### Type Alias `__kernel_key_t`

```rust
pub type __kernel_key_t = crate::ctypes::c_int;
```

#### Type Alias `__kernel_mqd_t`

```rust
pub type __kernel_mqd_t = crate::ctypes::c_int;
```

#### Type Alias `__kernel_old_uid_t`

```rust
pub type __kernel_old_uid_t = crate::ctypes::c_ushort;
```

#### Type Alias `__kernel_old_gid_t`

```rust
pub type __kernel_old_gid_t = crate::ctypes::c_ushort;
```

#### Type Alias `__kernel_long_t`

```rust
pub type __kernel_long_t = crate::ctypes::c_long;
```

#### Type Alias `__kernel_ulong_t`

```rust
pub type __kernel_ulong_t = crate::ctypes::c_ulong;
```

#### Type Alias `__kernel_ino_t`

```rust
pub type __kernel_ino_t = __kernel_ulong_t;
```

#### Type Alias `__kernel_mode_t`

```rust
pub type __kernel_mode_t = crate::ctypes::c_uint;
```

#### Type Alias `__kernel_pid_t`

```rust
pub type __kernel_pid_t = crate::ctypes::c_int;
```

#### Type Alias `__kernel_ipc_pid_t`

```rust
pub type __kernel_ipc_pid_t = crate::ctypes::c_int;
```

#### Type Alias `__kernel_uid_t`

```rust
pub type __kernel_uid_t = crate::ctypes::c_uint;
```

#### Type Alias `__kernel_gid_t`

```rust
pub type __kernel_gid_t = crate::ctypes::c_uint;
```

#### Type Alias `__kernel_suseconds_t`

```rust
pub type __kernel_suseconds_t = __kernel_long_t;
```

#### Type Alias `__kernel_daddr_t`

```rust
pub type __kernel_daddr_t = crate::ctypes::c_int;
```

#### Type Alias `__kernel_uid32_t`

```rust
pub type __kernel_uid32_t = crate::ctypes::c_uint;
```

#### Type Alias `__kernel_gid32_t`

```rust
pub type __kernel_gid32_t = crate::ctypes::c_uint;
```

#### Type Alias `__kernel_old_dev_t`

```rust
pub type __kernel_old_dev_t = crate::ctypes::c_uint;
```

#### Type Alias `__kernel_size_t`

```rust
pub type __kernel_size_t = __kernel_ulong_t;
```

#### Type Alias `__kernel_ssize_t`

```rust
pub type __kernel_ssize_t = __kernel_long_t;
```

#### Type Alias `__kernel_ptrdiff_t`

```rust
pub type __kernel_ptrdiff_t = __kernel_long_t;
```

#### Type Alias `__kernel_off_t`

```rust
pub type __kernel_off_t = __kernel_long_t;
```

#### Type Alias `__kernel_loff_t`

```rust
pub type __kernel_loff_t = crate::ctypes::c_longlong;
```

#### Type Alias `__kernel_old_time_t`

```rust
pub type __kernel_old_time_t = __kernel_long_t;
```

#### Type Alias `__kernel_time_t`

```rust
pub type __kernel_time_t = __kernel_long_t;
```

#### Type Alias `__kernel_time64_t`

```rust
pub type __kernel_time64_t = crate::ctypes::c_longlong;
```

#### Type Alias `__kernel_clock_t`

```rust
pub type __kernel_clock_t = __kernel_long_t;
```

#### Type Alias `__kernel_timer_t`

```rust
pub type __kernel_timer_t = crate::ctypes::c_int;
```

#### Type Alias `__kernel_clockid_t`

```rust
pub type __kernel_clockid_t = crate::ctypes::c_int;
```

#### Type Alias `__kernel_caddr_t`

```rust
pub type __kernel_caddr_t = *mut crate::ctypes::c_char;
```

#### Type Alias `__kernel_uid16_t`

```rust
pub type __kernel_uid16_t = crate::ctypes::c_ushort;
```

#### Type Alias `__kernel_gid16_t`

```rust
pub type __kernel_gid16_t = crate::ctypes::c_ushort;
```

#### Type Alias `__s128`

```rust
pub type __s128 = i128;
```

#### Type Alias `__u128`

```rust
pub type __u128 = u128;
```

#### Type Alias `__le16`

```rust
pub type __le16 = __u16;
```

#### Type Alias `__be16`

```rust
pub type __be16 = __u16;
```

#### Type Alias `__le32`

```rust
pub type __le32 = __u32;
```

#### Type Alias `__be32`

```rust
pub type __be32 = __u32;
```

#### Type Alias `__le64`

```rust
pub type __le64 = __u64;
```

#### Type Alias `__be64`

```rust
pub type __be64 = __u64;
```

#### Type Alias `__sum16`

```rust
pub type __sum16 = __u16;
```

#### Type Alias `__wsum`

```rust
pub type __wsum = __u32;
```

#### Type Alias `__poll_t`

```rust
pub type __poll_t = crate::ctypes::c_uint;
```

#### Type Alias `cap_user_header_t`

```rust
pub type cap_user_header_t = *mut __user_cap_header_struct;
```

#### Type Alias `cap_user_data_t`

```rust
pub type cap_user_data_t = *mut __user_cap_data_struct;
```

#### Type Alias `__kernel_rwf_t`

```rust
pub type __kernel_rwf_t = crate::ctypes::c_int;
```

#### Type Alias `old_sigset_t`

```rust
pub type old_sigset_t = crate::ctypes::c_ulong;
```

#### Type Alias `__signalfn_t`

```rust
pub type __signalfn_t = ::core::option::Option<unsafe extern "C" fn(crate::ctypes::c_int)>;
```

#### Type Alias `__sighandler_t`

```rust
pub type __sighandler_t = __signalfn_t;
```

#### Type Alias `__restorefn_t`

```rust
pub type __restorefn_t = ::core::option::Option<unsafe extern "C" fn()>;
```

#### Type Alias `__sigrestore_t`

```rust
pub type __sigrestore_t = __restorefn_t;
```

#### Type Alias `stack_t`

```rust
pub type stack_t = sigaltstack;
```

#### Type Alias `sigval_t`

```rust
pub type sigval_t = sigval;
```

#### Type Alias `siginfo_t`

```rust
pub type siginfo_t = siginfo;
```

#### Type Alias `sigevent_t`

```rust
pub type sigevent_t = sigevent;
```

#### Type Alias `cc_t`

```rust
pub type cc_t = crate::ctypes::c_uchar;
```

#### Type Alias `speed_t`

```rust
pub type speed_t = crate::ctypes::c_uint;
```

#### Type Alias `tcflag_t`

```rust
pub type tcflag_t = crate::ctypes::c_uint;
```

#### Type Alias `__fsword_t`

```rust
pub type __fsword_t = __kernel_long_t;
```

#### Struct `__BindgenBitfieldUnit`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __BindgenBitfieldUnit<Storage> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(storage: Storage) -> Self { /* ... */ }
  ```

- ```rust
  pub fn get_bit(self: &Self, index: usize) -> bool { /* ... */ }
  ```

- ```rust
  pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool { /* ... */ }
  ```

- ```rust
  pub fn set_bit(self: &mut Self, index: usize, val: bool) { /* ... */ }
  ```

- ```rust
  pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) { /* ... */ }
  ```

- ```rust
  pub fn get(self: &Self, bit_offset: usize, bit_width: u8) -> u64 { /* ... */ }
  ```

- ```rust
  pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 { /* ... */ }
  ```

- ```rust
  pub fn set(self: &mut Self, bit_offset: usize, bit_width: u8, val: u64) { /* ... */ }
  ```

- ```rust
  pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) { /* ... */ }
  ```

###### Trait Implementations

- **Copy**
- **Sync**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Eq**
- **Default**
  - ```rust
    fn default() -> __BindgenBitfieldUnit<Storage> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &__BindgenBitfieldUnit<Storage>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &__BindgenBitfieldUnit<Storage>) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &__BindgenBitfieldUnit<Storage>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> __BindgenBitfieldUnit<Storage> { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `__IncompleteArrayField`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __IncompleteArrayField<T>(/* private field */, /* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |
| 1 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn as_ptr(self: &Self) -> *const T { /* ... */ }
  ```

- ```rust
  pub fn as_mut_ptr(self: &mut Self) -> *mut T { /* ... */ }
  ```

- ```rust
  pub unsafe fn as_slice(self: &Self, len: usize) -> &[T] { /* ... */ }
  ```

- ```rust
  pub unsafe fn as_mut_slice(self: &mut Self, len: usize) -> &mut [T] { /* ... */ }
  ```

###### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> __IncompleteArrayField<T> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `__kernel_fd_set`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __kernel_fd_set {
    pub fds_bits: [crate::ctypes::c_ulong; 16],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `fds_bits` | `[crate::ctypes::c_ulong; 16]` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Copy**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> __kernel_fd_set { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `__kernel_fsid_t`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __kernel_fsid_t {
    pub val: [crate::ctypes::c_int; 2],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `val` | `[crate::ctypes::c_int; 2]` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> __kernel_fsid_t { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `__user_cap_header_struct`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __user_cap_header_struct {
    pub version: __u32,
    pub pid: crate::ctypes::c_int,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `version` | `__u32` |  |
| `pid` | `crate::ctypes::c_int` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> __user_cap_header_struct { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `__user_cap_data_struct`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __user_cap_data_struct {
    pub effective: __u32,
    pub permitted: __u32,
    pub inheritable: __u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `effective` | `__u32` |  |
| `permitted` | `__u32` |  |
| `inheritable` | `__u32` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Copy**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> __user_cap_data_struct { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `vfs_cap_data`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct vfs_cap_data {
    pub magic_etc: __le32,
    pub data: [vfs_cap_data__bindgen_ty_1; 2],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `magic_etc` | `__le32` |  |
| `data` | `[vfs_cap_data__bindgen_ty_1; 2]` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> vfs_cap_data { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
#### Struct `vfs_cap_data__bindgen_ty_1`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct vfs_cap_data__bindgen_ty_1 {
    pub permitted: __le32,
    pub inheritable: __le32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `permitted` | `__le32` |  |
| `inheritable` | `__le32` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> vfs_cap_data__bindgen_ty_1 { /* ... */ }
    ```

- **Sync**
- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
#### Struct `vfs_ns_cap_data`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct vfs_ns_cap_data {
    pub magic_etc: __le32,
    pub data: [vfs_ns_cap_data__bindgen_ty_1; 2],
    pub rootid: __le32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `magic_etc` | `__le32` |  |
| `data` | `[vfs_ns_cap_data__bindgen_ty_1; 2]` |  |
| `rootid` | `__le32` |  |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Freeze**
- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> vfs_ns_cap_data { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `vfs_ns_cap_data__bindgen_ty_1`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct vfs_ns_cap_data__bindgen_ty_1 {
    pub permitted: __le32,
    pub inheritable: __le32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `permitted` | `__le32` |  |
| `inheritable` | `__le32` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> vfs_ns_cap_data__bindgen_ty_1 { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `f_owner_ex`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct f_owner_ex {
    pub type_: crate::ctypes::c_int,
    pub pid: __kernel_pid_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `type_` | `crate::ctypes::c_int` |  |
| `pid` | `__kernel_pid_t` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> f_owner_ex { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Copy**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
#### Struct `flock`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct flock {
    pub l_type: crate::ctypes::c_short,
    pub l_whence: crate::ctypes::c_short,
    pub l_start: __kernel_off_t,
    pub l_len: __kernel_off_t,
    pub l_pid: __kernel_pid_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `l_type` | `crate::ctypes::c_short` |  |
| `l_whence` | `crate::ctypes::c_short` |  |
| `l_start` | `__kernel_off_t` |  |
| `l_len` | `__kernel_off_t` |  |
| `l_pid` | `__kernel_pid_t` |  |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> flock { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Sync**
- **Send**
- **Unpin**
#### Struct `flock64`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct flock64 {
    pub l_type: crate::ctypes::c_short,
    pub l_whence: crate::ctypes::c_short,
    pub l_start: __kernel_loff_t,
    pub l_len: __kernel_loff_t,
    pub l_pid: __kernel_pid_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `l_type` | `crate::ctypes::c_short` |  |
| `l_whence` | `crate::ctypes::c_short` |  |
| `l_start` | `__kernel_loff_t` |  |
| `l_len` | `__kernel_loff_t` |  |
| `l_pid` | `__kernel_pid_t` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> flock64 { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `open_how`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct open_how {
    pub flags: __u64,
    pub mode: __u64,
    pub resolve: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `flags` | `__u64` |  |
| `mode` | `__u64` |  |
| `resolve` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Sync**
- **UnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> open_how { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `epoll_event`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct epoll_event {
    pub events: __poll_t,
    pub data: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `events` | `__poll_t` |  |
| `data` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> epoll_event { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **UnwindSafe**
- **Copy**
- **Freeze**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `epoll_params`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct epoll_params {
    pub busy_poll_usecs: __u32,
    pub busy_poll_budget: __u16,
    pub prefer_busy_poll: __u8,
    pub __pad: __u8,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `busy_poll_usecs` | `__u32` |  |
| `busy_poll_budget` | `__u16` |  |
| `prefer_busy_poll` | `__u8` |  |
| `__pad` | `__u8` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> epoll_params { /* ... */ }
    ```

- **Send**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
#### Struct `fscrypt_policy_v1`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct fscrypt_policy_v1 {
    pub version: __u8,
    pub contents_encryption_mode: __u8,
    pub filenames_encryption_mode: __u8,
    pub flags: __u8,
    pub master_key_descriptor: [__u8; 8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `version` | `__u8` |  |
| `contents_encryption_mode` | `__u8` |  |
| `filenames_encryption_mode` | `__u8` |  |
| `flags` | `__u8` |  |
| `master_key_descriptor` | `[__u8; 8]` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> fscrypt_policy_v1 { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Copy**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
#### Struct `fscrypt_key`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct fscrypt_key {
    pub mode: __u32,
    pub raw: [__u8; 64],
    pub size: __u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `mode` | `__u32` |  |
| `raw` | `[__u8; 64]` |  |
| `size` | `__u32` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Send**
- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> fscrypt_key { /* ... */ }
    ```

#### Struct `fscrypt_policy_v2`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct fscrypt_policy_v2 {
    pub version: __u8,
    pub contents_encryption_mode: __u8,
    pub filenames_encryption_mode: __u8,
    pub flags: __u8,
    pub log2_data_unit_size: __u8,
    pub __reserved: [__u8; 3],
    pub master_key_identifier: [__u8; 16],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `version` | `__u8` |  |
| `contents_encryption_mode` | `__u8` |  |
| `filenames_encryption_mode` | `__u8` |  |
| `flags` | `__u8` |  |
| `log2_data_unit_size` | `__u8` |  |
| `__reserved` | `[__u8; 3]` |  |
| `master_key_identifier` | `[__u8; 16]` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Copy**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> fscrypt_policy_v2 { /* ... */ }
    ```

- **Unpin**
#### Struct `fscrypt_get_policy_ex_arg`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct fscrypt_get_policy_ex_arg {
    pub policy_size: __u64,
    pub policy: fscrypt_get_policy_ex_arg__bindgen_ty_1,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `policy_size` | `__u64` |  |
| `policy` | `fscrypt_get_policy_ex_arg__bindgen_ty_1` |  |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> fscrypt_get_policy_ex_arg { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
#### Struct `fscrypt_key_specifier`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct fscrypt_key_specifier {
    pub type_: __u32,
    pub __reserved: __u32,
    pub u: fscrypt_key_specifier__bindgen_ty_1,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `type_` | `__u32` |  |
| `__reserved` | `__u32` |  |
| `u` | `fscrypt_key_specifier__bindgen_ty_1` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> fscrypt_key_specifier { /* ... */ }
    ```

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

- **Sync**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `fscrypt_provisioning_key_payload`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct fscrypt_provisioning_key_payload {
    pub type_: __u32,
    pub __reserved: __u32,
    pub raw: __IncompleteArrayField<__u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `type_` | `__u32` |  |
| `__reserved` | `__u32` |  |
| `raw` | `__IncompleteArrayField<__u8>` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Send**
#### Struct `fscrypt_add_key_arg`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct fscrypt_add_key_arg {
    pub key_spec: fscrypt_key_specifier,
    pub raw_size: __u32,
    pub key_id: __u32,
    pub __reserved: [__u32; 8],
    pub raw: __IncompleteArrayField<__u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `key_spec` | `fscrypt_key_specifier` |  |
| `raw_size` | `__u32` |  |
| `key_id` | `__u32` |  |
| `__reserved` | `[__u32; 8]` |  |
| `raw` | `__IncompleteArrayField<__u8>` |  |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Freeze**
#### Struct `fscrypt_remove_key_arg`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct fscrypt_remove_key_arg {
    pub key_spec: fscrypt_key_specifier,
    pub removal_status_flags: __u32,
    pub __reserved: [__u32; 5],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `key_spec` | `fscrypt_key_specifier` |  |
| `removal_status_flags` | `__u32` |  |
| `__reserved` | `[__u32; 5]` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Copy**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> fscrypt_remove_key_arg { /* ... */ }
    ```

#### Struct `fscrypt_get_key_status_arg`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct fscrypt_get_key_status_arg {
    pub key_spec: fscrypt_key_specifier,
    pub __reserved: [__u32; 6],
    pub status: __u32,
    pub status_flags: __u32,
    pub user_count: __u32,
    pub __out_reserved: [__u32; 13],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `key_spec` | `fscrypt_key_specifier` |  |
| `__reserved` | `[__u32; 6]` |  |
| `status` | `__u32` |  |
| `status_flags` | `__u32` |  |
| `user_count` | `__u32` |  |
| `__out_reserved` | `[__u32; 13]` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Send**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> fscrypt_get_key_status_arg { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
#### Struct `mount_attr`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct mount_attr {
    pub attr_set: __u64,
    pub attr_clr: __u64,
    pub propagation: __u64,
    pub userns_fd: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `attr_set` | `__u64` |  |
| `attr_clr` | `__u64` |  |
| `propagation` | `__u64` |  |
| `userns_fd` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> mount_attr { /* ... */ }
    ```

- **Send**
- **Unpin**
- **UnwindSafe**
- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `statmount`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct statmount {
    pub size: __u32,
    pub mnt_opts: __u32,
    pub mask: __u64,
    pub sb_dev_major: __u32,
    pub sb_dev_minor: __u32,
    pub sb_magic: __u64,
    pub sb_flags: __u32,
    pub fs_type: __u32,
    pub mnt_id: __u64,
    pub mnt_parent_id: __u64,
    pub mnt_id_old: __u32,
    pub mnt_parent_id_old: __u32,
    pub mnt_attr: __u64,
    pub mnt_propagation: __u64,
    pub mnt_peer_group: __u64,
    pub mnt_master: __u64,
    pub propagate_from: __u64,
    pub mnt_root: __u32,
    pub mnt_point: __u32,
    pub mnt_ns_id: __u64,
    pub fs_subtype: __u32,
    pub sb_source: __u32,
    pub opt_num: __u32,
    pub opt_array: __u32,
    pub opt_sec_num: __u32,
    pub opt_sec_array: __u32,
    pub __spare2: [__u64; 46],
    pub str_: __IncompleteArrayField<crate::ctypes::c_char>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `size` | `__u32` |  |
| `mnt_opts` | `__u32` |  |
| `mask` | `__u64` |  |
| `sb_dev_major` | `__u32` |  |
| `sb_dev_minor` | `__u32` |  |
| `sb_magic` | `__u64` |  |
| `sb_flags` | `__u32` |  |
| `fs_type` | `__u32` |  |
| `mnt_id` | `__u64` |  |
| `mnt_parent_id` | `__u64` |  |
| `mnt_id_old` | `__u32` |  |
| `mnt_parent_id_old` | `__u32` |  |
| `mnt_attr` | `__u64` |  |
| `mnt_propagation` | `__u64` |  |
| `mnt_peer_group` | `__u64` |  |
| `mnt_master` | `__u64` |  |
| `propagate_from` | `__u64` |  |
| `mnt_root` | `__u32` |  |
| `mnt_point` | `__u32` |  |
| `mnt_ns_id` | `__u64` |  |
| `fs_subtype` | `__u32` |  |
| `sb_source` | `__u32` |  |
| `opt_num` | `__u32` |  |
| `opt_array` | `__u32` |  |
| `opt_sec_num` | `__u32` |  |
| `opt_sec_array` | `__u32` |  |
| `__spare2` | `[__u64; 46]` |  |
| `str_` | `__IncompleteArrayField<crate::ctypes::c_char>` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **RefUnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `mnt_id_req`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct mnt_id_req {
    pub size: __u32,
    pub spare: __u32,
    pub mnt_id: __u64,
    pub param: __u64,
    pub mnt_ns_id: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `size` | `__u32` |  |
| `spare` | `__u32` |  |
| `mnt_id` | `__u64` |  |
| `param` | `__u64` |  |
| `mnt_ns_id` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> mnt_id_req { /* ... */ }
    ```

- **Copy**
- **Unpin**
#### Struct `file_clone_range`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct file_clone_range {
    pub src_fd: __s64,
    pub src_offset: __u64,
    pub src_length: __u64,
    pub dest_offset: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `src_fd` | `__s64` |  |
| `src_offset` | `__u64` |  |
| `src_length` | `__u64` |  |
| `dest_offset` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> file_clone_range { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
#### Struct `fstrim_range`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct fstrim_range {
    pub start: __u64,
    pub len: __u64,
    pub minlen: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `start` | `__u64` |  |
| `len` | `__u64` |  |
| `minlen` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Send**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> fstrim_range { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `fsuuid2`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct fsuuid2 {
    pub len: __u8,
    pub uuid: [__u8; 16],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `len` | `__u8` |  |
| `uuid` | `[__u8; 16]` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> fsuuid2 { /* ... */ }
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

- **Send**
#### Struct `fs_sysfs_path`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct fs_sysfs_path {
    pub len: __u8,
    pub name: [__u8; 128],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `len` | `__u8` |  |
| `name` | `[__u8; 128]` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> fs_sysfs_path { /* ... */ }
    ```

- **Sync**
- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
#### Struct `file_dedupe_range_info`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct file_dedupe_range_info {
    pub dest_fd: __s64,
    pub dest_offset: __u64,
    pub bytes_deduped: __u64,
    pub status: __s32,
    pub reserved: __u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `dest_fd` | `__s64` |  |
| `dest_offset` | `__u64` |  |
| `bytes_deduped` | `__u64` |  |
| `status` | `__s32` |  |
| `reserved` | `__u32` |  |

##### Implementations

###### Trait Implementations

- **Copy**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> file_dedupe_range_info { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
#### Struct `file_dedupe_range`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct file_dedupe_range {
    pub src_offset: __u64,
    pub src_length: __u64,
    pub dest_count: __u16,
    pub reserved1: __u16,
    pub reserved2: __u32,
    pub info: __IncompleteArrayField<file_dedupe_range_info>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `src_offset` | `__u64` |  |
| `src_length` | `__u64` |  |
| `dest_count` | `__u16` |  |
| `reserved1` | `__u16` |  |
| `reserved2` | `__u32` |  |
| `info` | `__IncompleteArrayField<file_dedupe_range_info>` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
#### Struct `files_stat_struct`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct files_stat_struct {
    pub nr_files: crate::ctypes::c_ulong,
    pub nr_free_files: crate::ctypes::c_ulong,
    pub max_files: crate::ctypes::c_ulong,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `nr_files` | `crate::ctypes::c_ulong` |  |
| `nr_free_files` | `crate::ctypes::c_ulong` |  |
| `max_files` | `crate::ctypes::c_ulong` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Send**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> files_stat_struct { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `inodes_stat_t`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct inodes_stat_t {
    pub nr_inodes: crate::ctypes::c_long,
    pub nr_unused: crate::ctypes::c_long,
    pub dummy: [crate::ctypes::c_long; 5],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `nr_inodes` | `crate::ctypes::c_long` |  |
| `nr_unused` | `crate::ctypes::c_long` |  |
| `dummy` | `[crate::ctypes::c_long; 5]` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> inodes_stat_t { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `fsxattr`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct fsxattr {
    pub fsx_xflags: __u32,
    pub fsx_extsize: __u32,
    pub fsx_nextents: __u32,
    pub fsx_projid: __u32,
    pub fsx_cowextsize: __u32,
    pub fsx_pad: [crate::ctypes::c_uchar; 8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `fsx_xflags` | `__u32` |  |
| `fsx_extsize` | `__u32` |  |
| `fsx_nextents` | `__u32` |  |
| `fsx_projid` | `__u32` |  |
| `fsx_cowextsize` | `__u32` |  |
| `fsx_pad` | `[crate::ctypes::c_uchar; 8]` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> fsxattr { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `page_region`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct page_region {
    pub start: __u64,
    pub end: __u64,
    pub categories: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `start` | `__u64` |  |
| `end` | `__u64` |  |
| `categories` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> page_region { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Copy**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `pm_scan_arg`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct pm_scan_arg {
    pub size: __u64,
    pub flags: __u64,
    pub start: __u64,
    pub end: __u64,
    pub walk_end: __u64,
    pub vec: __u64,
    pub vec_len: __u64,
    pub max_pages: __u64,
    pub category_inverted: __u64,
    pub category_mask: __u64,
    pub category_anyof_mask: __u64,
    pub return_mask: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `size` | `__u64` |  |
| `flags` | `__u64` |  |
| `start` | `__u64` |  |
| `end` | `__u64` |  |
| `walk_end` | `__u64` |  |
| `vec` | `__u64` |  |
| `vec_len` | `__u64` |  |
| `max_pages` | `__u64` |  |
| `category_inverted` | `__u64` |  |
| `category_mask` | `__u64` |  |
| `category_anyof_mask` | `__u64` |  |
| `return_mask` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> pm_scan_arg { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
#### Struct `procmap_query`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct procmap_query {
    pub size: __u64,
    pub query_flags: __u64,
    pub query_addr: __u64,
    pub vma_start: __u64,
    pub vma_end: __u64,
    pub vma_flags: __u64,
    pub vma_page_size: __u64,
    pub vma_offset: __u64,
    pub inode: __u64,
    pub dev_major: __u32,
    pub dev_minor: __u32,
    pub vma_name_size: __u32,
    pub build_id_size: __u32,
    pub vma_name_addr: __u64,
    pub build_id_addr: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `size` | `__u64` |  |
| `query_flags` | `__u64` |  |
| `query_addr` | `__u64` |  |
| `vma_start` | `__u64` |  |
| `vma_end` | `__u64` |  |
| `vma_flags` | `__u64` |  |
| `vma_page_size` | `__u64` |  |
| `vma_offset` | `__u64` |  |
| `inode` | `__u64` |  |
| `dev_major` | `__u32` |  |
| `dev_minor` | `__u32` |  |
| `vma_name_size` | `__u32` |  |
| `build_id_size` | `__u32` |  |
| `vma_name_addr` | `__u64` |  |
| `build_id_addr` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Copy**
- **UnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> procmap_query { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `futex_waitv`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct futex_waitv {
    pub val: __u64,
    pub uaddr: __u64,
    pub flags: __u32,
    pub __reserved: __u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `val` | `__u64` |  |
| `uaddr` | `__u64` |  |
| `flags` | `__u32` |  |
| `__reserved` | `__u32` |  |

##### Implementations

###### Trait Implementations

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> futex_waitv { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
#### Struct `robust_list`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct robust_list {
    pub next: *mut robust_list,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `next` | `*mut robust_list` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> robust_list { /* ... */ }
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

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `robust_list_head`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct robust_list_head {
    pub list: robust_list,
    pub futex_offset: crate::ctypes::c_long,
    pub list_op_pending: *mut robust_list,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `list` | `robust_list` |  |
| `futex_offset` | `crate::ctypes::c_long` |  |
| `list_op_pending` | `*mut robust_list` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> robust_list_head { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Copy**
#### Struct `inotify_event`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct inotify_event {
    pub wd: __s32,
    pub mask: __u32,
    pub cookie: __u32,
    pub len: __u32,
    pub name: __IncompleteArrayField<crate::ctypes::c_char>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `wd` | `__s32` |  |
| `mask` | `__u32` |  |
| `cookie` | `__u32` |  |
| `len` | `__u32` |  |
| `name` | `__IncompleteArrayField<crate::ctypes::c_char>` |  |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **UnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `cachestat_range`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct cachestat_range {
    pub off: __u64,
    pub len: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `off` | `__u64` |  |
| `len` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> cachestat_range { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
#### Struct `cachestat`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct cachestat {
    pub nr_cache: __u64,
    pub nr_dirty: __u64,
    pub nr_writeback: __u64,
    pub nr_evicted: __u64,
    pub nr_recently_evicted: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `nr_cache` | `__u64` |  |
| `nr_dirty` | `__u64` |  |
| `nr_writeback` | `__u64` |  |
| `nr_evicted` | `__u64` |  |
| `nr_recently_evicted` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Sync**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> cachestat { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
#### Struct `pollfd`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct pollfd {
    pub fd: crate::ctypes::c_int,
    pub events: crate::ctypes::c_short,
    pub revents: crate::ctypes::c_short,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `fd` | `crate::ctypes::c_int` |  |
| `events` | `crate::ctypes::c_short` |  |
| `revents` | `crate::ctypes::c_short` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Copy**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> pollfd { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
#### Struct `rand_pool_info`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct rand_pool_info {
    pub entropy_count: crate::ctypes::c_int,
    pub buf_size: crate::ctypes::c_int,
    pub buf: __IncompleteArrayField<__u32>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `entropy_count` | `crate::ctypes::c_int` |  |
| `buf_size` | `crate::ctypes::c_int` |  |
| `buf` | `__IncompleteArrayField<__u32>` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Freeze**
#### Struct `vgetrandom_opaque_params`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct vgetrandom_opaque_params {
    pub size_of_opaque_state: __u32,
    pub mmap_prot: __u32,
    pub mmap_flags: __u32,
    pub reserved: [__u32; 13],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `size_of_opaque_state` | `__u32` |  |
| `mmap_prot` | `__u32` |  |
| `mmap_flags` | `__u32` |  |
| `reserved` | `[__u32; 13]` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> vgetrandom_opaque_params { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `__kernel_timespec`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __kernel_timespec {
    pub tv_sec: __kernel_time64_t,
    pub tv_nsec: crate::ctypes::c_longlong,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `tv_sec` | `__kernel_time64_t` |  |
| `tv_nsec` | `crate::ctypes::c_longlong` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> __kernel_timespec { /* ... */ }
    ```

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `__kernel_itimerspec`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __kernel_itimerspec {
    pub it_interval: __kernel_timespec,
    pub it_value: __kernel_timespec,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `it_interval` | `__kernel_timespec` |  |
| `it_value` | `__kernel_timespec` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> __kernel_itimerspec { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Sync**
- **Unpin**
#### Struct `__kernel_old_timeval`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __kernel_old_timeval {
    pub tv_sec: __kernel_long_t,
    pub tv_usec: __kernel_long_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `tv_sec` | `__kernel_long_t` |  |
| `tv_usec` | `__kernel_long_t` |  |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> __kernel_old_timeval { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
#### Struct `__kernel_old_timespec`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __kernel_old_timespec {
    pub tv_sec: __kernel_old_time_t,
    pub tv_nsec: crate::ctypes::c_long,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `tv_sec` | `__kernel_old_time_t` |  |
| `tv_nsec` | `crate::ctypes::c_long` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> __kernel_old_timespec { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Send**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
#### Struct `__kernel_old_itimerval`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __kernel_old_itimerval {
    pub it_interval: __kernel_old_timeval,
    pub it_value: __kernel_old_timeval,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `it_interval` | `__kernel_old_timeval` |  |
| `it_value` | `__kernel_old_timeval` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> __kernel_old_itimerval { /* ... */ }
    ```

#### Struct `__kernel_sock_timeval`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __kernel_sock_timeval {
    pub tv_sec: __s64,
    pub tv_usec: __s64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `tv_sec` | `__s64` |  |
| `tv_usec` | `__s64` |  |

##### Implementations

###### Trait Implementations

- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Freeze**
- **Sync**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> __kernel_sock_timeval { /* ... */ }
    ```

#### Struct `rusage`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct rusage {
    pub ru_utime: __kernel_old_timeval,
    pub ru_stime: __kernel_old_timeval,
    pub ru_maxrss: __kernel_long_t,
    pub ru_ixrss: __kernel_long_t,
    pub ru_idrss: __kernel_long_t,
    pub ru_isrss: __kernel_long_t,
    pub ru_minflt: __kernel_long_t,
    pub ru_majflt: __kernel_long_t,
    pub ru_nswap: __kernel_long_t,
    pub ru_inblock: __kernel_long_t,
    pub ru_oublock: __kernel_long_t,
    pub ru_msgsnd: __kernel_long_t,
    pub ru_msgrcv: __kernel_long_t,
    pub ru_nsignals: __kernel_long_t,
    pub ru_nvcsw: __kernel_long_t,
    pub ru_nivcsw: __kernel_long_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ru_utime` | `__kernel_old_timeval` |  |
| `ru_stime` | `__kernel_old_timeval` |  |
| `ru_maxrss` | `__kernel_long_t` |  |
| `ru_ixrss` | `__kernel_long_t` |  |
| `ru_idrss` | `__kernel_long_t` |  |
| `ru_isrss` | `__kernel_long_t` |  |
| `ru_minflt` | `__kernel_long_t` |  |
| `ru_majflt` | `__kernel_long_t` |  |
| `ru_nswap` | `__kernel_long_t` |  |
| `ru_inblock` | `__kernel_long_t` |  |
| `ru_oublock` | `__kernel_long_t` |  |
| `ru_msgsnd` | `__kernel_long_t` |  |
| `ru_msgrcv` | `__kernel_long_t` |  |
| `ru_nsignals` | `__kernel_long_t` |  |
| `ru_nvcsw` | `__kernel_long_t` |  |
| `ru_nivcsw` | `__kernel_long_t` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Copy**
- **UnwindSafe**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> rusage { /* ... */ }
    ```

- **Sync**
#### Struct `rlimit`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct rlimit {
    pub rlim_cur: __kernel_ulong_t,
    pub rlim_max: __kernel_ulong_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `rlim_cur` | `__kernel_ulong_t` |  |
| `rlim_max` | `__kernel_ulong_t` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> rlimit { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `rlimit64`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct rlimit64 {
    pub rlim_cur: __u64,
    pub rlim_max: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `rlim_cur` | `__u64` |  |
| `rlim_max` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> rlimit64 { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `clone_args`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct clone_args {
    pub flags: __u64,
    pub pidfd: __u64,
    pub child_tid: __u64,
    pub parent_tid: __u64,
    pub exit_signal: __u64,
    pub stack: __u64,
    pub stack_size: __u64,
    pub tls: __u64,
    pub set_tid: __u64,
    pub set_tid_size: __u64,
    pub cgroup: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `flags` | `__u64` |  |
| `pidfd` | `__u64` |  |
| `child_tid` | `__u64` |  |
| `parent_tid` | `__u64` |  |
| `exit_signal` | `__u64` |  |
| `stack` | `__u64` |  |
| `stack_size` | `__u64` |  |
| `tls` | `__u64` |  |
| `set_tid` | `__u64` |  |
| `set_tid_size` | `__u64` |  |
| `cgroup` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> clone_args { /* ... */ }
    ```

- **Sync**
- **Send**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `sigset_t`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct sigset_t {
    pub sig: [crate::ctypes::c_ulong; 1],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `sig` | `[crate::ctypes::c_ulong; 1]` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> sigset_t { /* ... */ }
    ```

- **Sync**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `sigaction`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_flags: crate::ctypes::c_ulong,
    pub sa_restorer: __sigrestore_t,
    pub sa_mask: sigset_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `sa_handler` | `__sighandler_t` |  |
| `sa_flags` | `crate::ctypes::c_ulong` |  |
| `sa_restorer` | `__sigrestore_t` |  |
| `sa_mask` | `sigset_t` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> sigaction { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `sigaltstack`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct sigaltstack {
    pub ss_sp: *mut crate::ctypes::c_void,
    pub ss_flags: crate::ctypes::c_int,
    pub ss_size: __kernel_size_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ss_sp` | `*mut crate::ctypes::c_void` |  |
| `ss_flags` | `crate::ctypes::c_int` |  |
| `ss_size` | `__kernel_size_t` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> sigaltstack { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `__sifields__bindgen_ty_1`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __sifields__bindgen_ty_1 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_pid` | `__kernel_pid_t` |  |
| `_uid` | `__kernel_uid32_t` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> __sifields__bindgen_ty_1 { /* ... */ }
    ```

#### Struct `__sifields__bindgen_ty_2`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __sifields__bindgen_ty_2 {
    pub _tid: __kernel_timer_t,
    pub _overrun: crate::ctypes::c_int,
    pub _sigval: sigval_t,
    pub _sys_private: crate::ctypes::c_int,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_tid` | `__kernel_timer_t` |  |
| `_overrun` | `crate::ctypes::c_int` |  |
| `_sigval` | `sigval_t` |  |
| `_sys_private` | `crate::ctypes::c_int` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> __sifields__bindgen_ty_2 { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Copy**
- **Unpin**
- **Freeze**
#### Struct `__sifields__bindgen_ty_3`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __sifields__bindgen_ty_3 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _sigval: sigval_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_pid` | `__kernel_pid_t` |  |
| `_uid` | `__kernel_uid32_t` |  |
| `_sigval` | `sigval_t` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> __sifields__bindgen_ty_3 { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Copy**
- **Send**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `__sifields__bindgen_ty_4`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __sifields__bindgen_ty_4 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _status: crate::ctypes::c_int,
    pub _utime: __kernel_clock_t,
    pub _stime: __kernel_clock_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_pid` | `__kernel_pid_t` |  |
| `_uid` | `__kernel_uid32_t` |  |
| `_status` | `crate::ctypes::c_int` |  |
| `_utime` | `__kernel_clock_t` |  |
| `_stime` | `__kernel_clock_t` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> __sifields__bindgen_ty_4 { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
#### Struct `__sifields__bindgen_ty_5`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __sifields__bindgen_ty_5 {
    pub _addr: *mut crate::ctypes::c_void,
    pub __bindgen_anon_1: __sifields__bindgen_ty_5__bindgen_ty_1,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_addr` | `*mut crate::ctypes::c_void` |  |
| `__bindgen_anon_1` | `__sifields__bindgen_ty_5__bindgen_ty_1` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
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

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> __sifields__bindgen_ty_5 { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 {
    pub _dummy_bnd: [crate::ctypes::c_char; 8],
    pub _lower: *mut crate::ctypes::c_void,
    pub _upper: *mut crate::ctypes::c_void,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_dummy_bnd` | `[crate::ctypes::c_char; 8]` |  |
| `_lower` | `*mut crate::ctypes::c_void` |  |
| `_upper` | `*mut crate::ctypes::c_void` |  |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2 {
    pub _dummy_pkey: [crate::ctypes::c_char; 8],
    pub _pkey: __u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_dummy_pkey` | `[crate::ctypes::c_char; 8]` |  |
| `_pkey` | `__u32` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2 { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3 {
    pub _data: crate::ctypes::c_ulong,
    pub _type: __u32,
    pub _flags: __u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_data` | `crate::ctypes::c_ulong` |  |
| `_type` | `__u32` |  |
| `_flags` | `__u32` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3 { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Copy**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `__sifields__bindgen_ty_6`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __sifields__bindgen_ty_6 {
    pub _band: crate::ctypes::c_long,
    pub _fd: crate::ctypes::c_int,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_band` | `crate::ctypes::c_long` |  |
| `_fd` | `crate::ctypes::c_int` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> __sifields__bindgen_ty_6 { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
#### Struct `__sifields__bindgen_ty_7`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct __sifields__bindgen_ty_7 {
    pub _call_addr: *mut crate::ctypes::c_void,
    pub _syscall: crate::ctypes::c_int,
    pub _arch: crate::ctypes::c_uint,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_call_addr` | `*mut crate::ctypes::c_void` |  |
| `_syscall` | `crate::ctypes::c_int` |  |
| `_arch` | `crate::ctypes::c_uint` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> __sifields__bindgen_ty_7 { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **UnwindSafe**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `siginfo`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct siginfo {
    pub __bindgen_anon_1: siginfo__bindgen_ty_1,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `__bindgen_anon_1` | `siginfo__bindgen_ty_1` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> siginfo { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `siginfo__bindgen_ty_1__bindgen_ty_1`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct siginfo__bindgen_ty_1__bindgen_ty_1 {
    pub si_signo: crate::ctypes::c_int,
    pub si_errno: crate::ctypes::c_int,
    pub si_code: crate::ctypes::c_int,
    pub _sifields: __sifields,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `si_signo` | `crate::ctypes::c_int` |  |
| `si_errno` | `crate::ctypes::c_int` |  |
| `si_code` | `crate::ctypes::c_int` |  |
| `_sifields` | `__sifields` |  |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> siginfo__bindgen_ty_1__bindgen_ty_1 { /* ... */ }
    ```

- **Sync**
#### Struct `sigevent`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct sigevent {
    pub sigev_value: sigval_t,
    pub sigev_signo: crate::ctypes::c_int,
    pub sigev_notify: crate::ctypes::c_int,
    pub _sigev_un: sigevent__bindgen_ty_1,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `sigev_value` | `sigval_t` |  |
| `sigev_signo` | `crate::ctypes::c_int` |  |
| `sigev_notify` | `crate::ctypes::c_int` |  |
| `_sigev_un` | `sigevent__bindgen_ty_1` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Copy**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> sigevent { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `sigevent__bindgen_ty_1__bindgen_ty_1`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct sigevent__bindgen_ty_1__bindgen_ty_1 {
    pub _function: ::core::option::Option<unsafe extern "C" fn(sigval_t)>,
    pub _attribute: *mut crate::ctypes::c_void,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_function` | `::core::option::Option<unsafe extern "C" fn(sigval_t)>` |  |
| `_attribute` | `*mut crate::ctypes::c_void` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> sigevent__bindgen_ty_1__bindgen_ty_1 { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `statx_timestamp`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct statx_timestamp {
    pub tv_sec: __s64,
    pub tv_nsec: __u32,
    pub __reserved: __s32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `tv_sec` | `__s64` |  |
| `tv_nsec` | `__u32` |  |
| `__reserved` | `__s32` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> statx_timestamp { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `statx`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct statx {
    pub stx_mask: __u32,
    pub stx_blksize: __u32,
    pub stx_attributes: __u64,
    pub stx_nlink: __u32,
    pub stx_uid: __u32,
    pub stx_gid: __u32,
    pub stx_mode: __u16,
    pub __spare0: [__u16; 1],
    pub stx_ino: __u64,
    pub stx_size: __u64,
    pub stx_blocks: __u64,
    pub stx_attributes_mask: __u64,
    pub stx_atime: statx_timestamp,
    pub stx_btime: statx_timestamp,
    pub stx_ctime: statx_timestamp,
    pub stx_mtime: statx_timestamp,
    pub stx_rdev_major: __u32,
    pub stx_rdev_minor: __u32,
    pub stx_dev_major: __u32,
    pub stx_dev_minor: __u32,
    pub stx_mnt_id: __u64,
    pub stx_dio_mem_align: __u32,
    pub stx_dio_offset_align: __u32,
    pub stx_subvol: __u64,
    pub stx_atomic_write_unit_min: __u32,
    pub stx_atomic_write_unit_max: __u32,
    pub stx_atomic_write_segments_max: __u32,
    pub __spare1: [__u32; 1],
    pub __spare3: [__u64; 9],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `stx_mask` | `__u32` |  |
| `stx_blksize` | `__u32` |  |
| `stx_attributes` | `__u64` |  |
| `stx_nlink` | `__u32` |  |
| `stx_uid` | `__u32` |  |
| `stx_gid` | `__u32` |  |
| `stx_mode` | `__u16` |  |
| `__spare0` | `[__u16; 1]` |  |
| `stx_ino` | `__u64` |  |
| `stx_size` | `__u64` |  |
| `stx_blocks` | `__u64` |  |
| `stx_attributes_mask` | `__u64` |  |
| `stx_atime` | `statx_timestamp` |  |
| `stx_btime` | `statx_timestamp` |  |
| `stx_ctime` | `statx_timestamp` |  |
| `stx_mtime` | `statx_timestamp` |  |
| `stx_rdev_major` | `__u32` |  |
| `stx_rdev_minor` | `__u32` |  |
| `stx_dev_major` | `__u32` |  |
| `stx_dev_minor` | `__u32` |  |
| `stx_mnt_id` | `__u64` |  |
| `stx_dio_mem_align` | `__u32` |  |
| `stx_dio_offset_align` | `__u32` |  |
| `stx_subvol` | `__u64` |  |
| `stx_atomic_write_unit_min` | `__u32` |  |
| `stx_atomic_write_unit_max` | `__u32` |  |
| `stx_atomic_write_segments_max` | `__u32` |  |
| `__spare1` | `[__u32; 1]` |  |
| `__spare3` | `[__u64; 9]` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> statx { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `termios`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 19],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `c_iflag` | `tcflag_t` |  |
| `c_oflag` | `tcflag_t` |  |
| `c_cflag` | `tcflag_t` |  |
| `c_lflag` | `tcflag_t` |  |
| `c_line` | `cc_t` |  |
| `c_cc` | `[cc_t; 19]` |  |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Copy**
- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> termios { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Send**
#### Struct `termios2`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct termios2 {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 19],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `c_iflag` | `tcflag_t` |  |
| `c_oflag` | `tcflag_t` |  |
| `c_cflag` | `tcflag_t` |  |
| `c_lflag` | `tcflag_t` |  |
| `c_line` | `cc_t` |  |
| `c_cc` | `[cc_t; 19]` |  |
| `c_ispeed` | `speed_t` |  |
| `c_ospeed` | `speed_t` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> termios2 { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **UnwindSafe**
- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `ktermios`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct ktermios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 19],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `c_iflag` | `tcflag_t` |  |
| `c_oflag` | `tcflag_t` |  |
| `c_cflag` | `tcflag_t` |  |
| `c_lflag` | `tcflag_t` |  |
| `c_line` | `cc_t` |  |
| `c_cc` | `[cc_t; 19]` |  |
| `c_ispeed` | `speed_t` |  |
| `c_ospeed` | `speed_t` |  |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ktermios { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
#### Struct `winsize`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct winsize {
    pub ws_row: crate::ctypes::c_ushort,
    pub ws_col: crate::ctypes::c_ushort,
    pub ws_xpixel: crate::ctypes::c_ushort,
    pub ws_ypixel: crate::ctypes::c_ushort,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ws_row` | `crate::ctypes::c_ushort` |  |
| `ws_col` | `crate::ctypes::c_ushort` |  |
| `ws_xpixel` | `crate::ctypes::c_ushort` |  |
| `ws_ypixel` | `crate::ctypes::c_ushort` |  |

##### Implementations

###### Trait Implementations

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> winsize { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `termio`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct termio {
    pub c_iflag: crate::ctypes::c_ushort,
    pub c_oflag: crate::ctypes::c_ushort,
    pub c_cflag: crate::ctypes::c_ushort,
    pub c_lflag: crate::ctypes::c_ushort,
    pub c_line: crate::ctypes::c_uchar,
    pub c_cc: [crate::ctypes::c_uchar; 8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `c_iflag` | `crate::ctypes::c_ushort` |  |
| `c_oflag` | `crate::ctypes::c_ushort` |  |
| `c_cflag` | `crate::ctypes::c_ushort` |  |
| `c_lflag` | `crate::ctypes::c_ushort` |  |
| `c_line` | `crate::ctypes::c_uchar` |  |
| `c_cc` | `[crate::ctypes::c_uchar; 8]` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> termio { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
#### Struct `timespec`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct timespec {
    pub tv_sec: __kernel_old_time_t,
    pub tv_nsec: crate::ctypes::c_long,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `tv_sec` | `__kernel_old_time_t` |  |
| `tv_nsec` | `crate::ctypes::c_long` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> timespec { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `timeval`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct timeval {
    pub tv_sec: __kernel_old_time_t,
    pub tv_usec: __kernel_suseconds_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `tv_sec` | `__kernel_old_time_t` |  |
| `tv_usec` | `__kernel_suseconds_t` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> timeval { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `itimerspec`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `it_interval` | `timespec` |  |
| `it_value` | `timespec` |  |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> itimerspec { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Copy**
#### Struct `itimerval`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `it_interval` | `timeval` |  |
| `it_value` | `timeval` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Send**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> itimerval { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Struct `timezone`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct timezone {
    pub tz_minuteswest: crate::ctypes::c_int,
    pub tz_dsttime: crate::ctypes::c_int,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `tz_minuteswest` | `crate::ctypes::c_int` |  |
| `tz_dsttime` | `crate::ctypes::c_int` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Unpin**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> timezone { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `iovec`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct iovec {
    pub iov_base: *mut crate::ctypes::c_void,
    pub iov_len: __kernel_size_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `iov_base` | `*mut crate::ctypes::c_void` |  |
| `iov_len` | `__kernel_size_t` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> iovec { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `dmabuf_cmsg`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct dmabuf_cmsg {
    pub frag_offset: __u64,
    pub frag_size: __u32,
    pub frag_token: __u32,
    pub dmabuf_id: __u32,
    pub flags: __u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `frag_offset` | `__u64` |  |
| `frag_size` | `__u32` |  |
| `frag_token` | `__u32` |  |
| `dmabuf_id` | `__u32` |  |
| `flags` | `__u32` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Sync**
- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> dmabuf_cmsg { /* ... */ }
    ```

#### Struct `dmabuf_token`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct dmabuf_token {
    pub token_start: __u32,
    pub token_count: __u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `token_start` | `__u32` |  |
| `token_count` | `__u32` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **Unpin**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> dmabuf_token { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `xattr_args`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct xattr_args {
    pub value: __u64,
    pub size: __u32,
    pub flags: __u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `value` | `__u64` |  |
| `size` | `__u32` |  |
| `flags` | `__u32` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> xattr_args { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `uffd_msg`

**Attributes:**

- `#[repr(C, packed(1))]`

```rust
pub struct uffd_msg {
    pub event: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
    pub reserved3: __u32,
    pub arg: uffd_msg__bindgen_ty_1,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `event` | `__u8` |  |
| `reserved1` | `__u8` |  |
| `reserved2` | `__u16` |  |
| `reserved3` | `__u32` |  |
| `arg` | `uffd_msg__bindgen_ty_1` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffd_msg { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Unpin**
#### Struct `uffd_msg__bindgen_ty_1__bindgen_ty_1`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct uffd_msg__bindgen_ty_1__bindgen_ty_1 {
    pub flags: __u64,
    pub address: __u64,
    pub feat: uffd_msg__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `flags` | `__u64` |  |
| `address` | `__u64` |  |
| `feat` | `uffd_msg__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffd_msg__bindgen_ty_1__bindgen_ty_1 { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **UnwindSafe**
- **Freeze**
#### Struct `uffd_msg__bindgen_ty_1__bindgen_ty_2`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct uffd_msg__bindgen_ty_1__bindgen_ty_2 {
    pub ufd: __u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ufd` | `__u32` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffd_msg__bindgen_ty_1__bindgen_ty_2 { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `uffd_msg__bindgen_ty_1__bindgen_ty_3`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct uffd_msg__bindgen_ty_1__bindgen_ty_3 {
    pub from: __u64,
    pub to: __u64,
    pub len: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `from` | `__u64` |  |
| `to` | `__u64` |  |
| `len` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Copy**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffd_msg__bindgen_ty_1__bindgen_ty_3 { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `uffd_msg__bindgen_ty_1__bindgen_ty_4`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct uffd_msg__bindgen_ty_1__bindgen_ty_4 {
    pub start: __u64,
    pub end: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `start` | `__u64` |  |
| `end` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
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
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffd_msg__bindgen_ty_1__bindgen_ty_4 { /* ... */ }
    ```

- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
#### Struct `uffd_msg__bindgen_ty_1__bindgen_ty_5`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct uffd_msg__bindgen_ty_1__bindgen_ty_5 {
    pub reserved1: __u64,
    pub reserved2: __u64,
    pub reserved3: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `reserved1` | `__u64` |  |
| `reserved2` | `__u64` |  |
| `reserved3` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffd_msg__bindgen_ty_1__bindgen_ty_5 { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `uffdio_api`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct uffdio_api {
    pub api: __u64,
    pub features: __u64,
    pub ioctls: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `api` | `__u64` |  |
| `features` | `__u64` |  |
| `ioctls` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffdio_api { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `uffdio_range`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct uffdio_range {
    pub start: __u64,
    pub len: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `start` | `__u64` |  |
| `len` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffdio_range { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `uffdio_register`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct uffdio_register {
    pub range: uffdio_range,
    pub mode: __u64,
    pub ioctls: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `range` | `uffdio_range` |  |
| `mode` | `__u64` |  |
| `ioctls` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffdio_register { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
#### Struct `uffdio_copy`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct uffdio_copy {
    pub dst: __u64,
    pub src: __u64,
    pub len: __u64,
    pub mode: __u64,
    pub copy: __s64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `dst` | `__u64` |  |
| `src` | `__u64` |  |
| `len` | `__u64` |  |
| `mode` | `__u64` |  |
| `copy` | `__s64` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffdio_copy { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `uffdio_zeropage`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct uffdio_zeropage {
    pub range: uffdio_range,
    pub mode: __u64,
    pub zeropage: __s64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `range` | `uffdio_range` |  |
| `mode` | `__u64` |  |
| `zeropage` | `__s64` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffdio_zeropage { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Send**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
#### Struct `uffdio_writeprotect`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct uffdio_writeprotect {
    pub range: uffdio_range,
    pub mode: __u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `range` | `uffdio_range` |  |
| `mode` | `__u64` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffdio_writeprotect { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `uffdio_continue`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct uffdio_continue {
    pub range: uffdio_range,
    pub mode: __u64,
    pub mapped: __s64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `range` | `uffdio_range` |  |
| `mode` | `__u64` |  |
| `mapped` | `__s64` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffdio_continue { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `uffdio_poison`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct uffdio_poison {
    pub range: uffdio_range,
    pub mode: __u64,
    pub updated: __s64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `range` | `uffdio_range` |  |
| `mode` | `__u64` |  |
| `updated` | `__s64` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffdio_poison { /* ... */ }
    ```

#### Struct `uffdio_move`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct uffdio_move {
    pub dst: __u64,
    pub src: __u64,
    pub len: __u64,
    pub mode: __u64,
    pub move_: __s64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `dst` | `__u64` |  |
| `src` | `__u64` |  |
| `len` | `__u64` |  |
| `mode` | `__u64` |  |
| `move_` | `__s64` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **RefUnwindSafe**
- **Unpin**
- **Copy**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> uffdio_move { /* ... */ }
    ```

#### Struct `linux_dirent64`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct linux_dirent64 {
    pub d_ino: crate::ctypes::c_ulong,
    pub d_off: crate::ctypes::c_long,
    pub d_reclen: __u16,
    pub d_type: __u8,
    pub d_name: __IncompleteArrayField<crate::ctypes::c_char>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `d_ino` | `crate::ctypes::c_ulong` |  |
| `d_off` | `crate::ctypes::c_long` |  |
| `d_reclen` | `__u16` |  |
| `d_type` | `__u8` |  |
| `d_name` | `__IncompleteArrayField<crate::ctypes::c_char>` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `stat`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct stat {
    pub st_dev: crate::ctypes::c_ulong,
    pub st_ino: crate::ctypes::c_ulong,
    pub st_mode: crate::ctypes::c_uint,
    pub st_nlink: crate::ctypes::c_uint,
    pub st_uid: crate::ctypes::c_uint,
    pub st_gid: crate::ctypes::c_uint,
    pub st_rdev: crate::ctypes::c_ulong,
    pub __pad1: crate::ctypes::c_ulong,
    pub st_size: crate::ctypes::c_long,
    pub st_blksize: crate::ctypes::c_int,
    pub __pad2: crate::ctypes::c_int,
    pub st_blocks: crate::ctypes::c_long,
    pub st_atime: crate::ctypes::c_long,
    pub st_atime_nsec: crate::ctypes::c_ulong,
    pub st_mtime: crate::ctypes::c_long,
    pub st_mtime_nsec: crate::ctypes::c_ulong,
    pub st_ctime: crate::ctypes::c_long,
    pub st_ctime_nsec: crate::ctypes::c_ulong,
    pub __unused4: crate::ctypes::c_uint,
    pub __unused5: crate::ctypes::c_uint,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `st_dev` | `crate::ctypes::c_ulong` |  |
| `st_ino` | `crate::ctypes::c_ulong` |  |
| `st_mode` | `crate::ctypes::c_uint` |  |
| `st_nlink` | `crate::ctypes::c_uint` |  |
| `st_uid` | `crate::ctypes::c_uint` |  |
| `st_gid` | `crate::ctypes::c_uint` |  |
| `st_rdev` | `crate::ctypes::c_ulong` |  |
| `__pad1` | `crate::ctypes::c_ulong` |  |
| `st_size` | `crate::ctypes::c_long` |  |
| `st_blksize` | `crate::ctypes::c_int` |  |
| `__pad2` | `crate::ctypes::c_int` |  |
| `st_blocks` | `crate::ctypes::c_long` |  |
| `st_atime` | `crate::ctypes::c_long` |  |
| `st_atime_nsec` | `crate::ctypes::c_ulong` |  |
| `st_mtime` | `crate::ctypes::c_long` |  |
| `st_mtime_nsec` | `crate::ctypes::c_ulong` |  |
| `st_ctime` | `crate::ctypes::c_long` |  |
| `st_ctime_nsec` | `crate::ctypes::c_ulong` |  |
| `__unused4` | `crate::ctypes::c_uint` |  |
| `__unused5` | `crate::ctypes::c_uint` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> stat { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Freeze**
- **Unpin**
- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `statfs`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct statfs {
    pub f_type: __kernel_long_t,
    pub f_bsize: __kernel_long_t,
    pub f_blocks: __kernel_long_t,
    pub f_bfree: __kernel_long_t,
    pub f_bavail: __kernel_long_t,
    pub f_files: __kernel_long_t,
    pub f_ffree: __kernel_long_t,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __kernel_long_t,
    pub f_frsize: __kernel_long_t,
    pub f_flags: __kernel_long_t,
    pub f_spare: [__kernel_long_t; 4],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `f_type` | `__kernel_long_t` |  |
| `f_bsize` | `__kernel_long_t` |  |
| `f_blocks` | `__kernel_long_t` |  |
| `f_bfree` | `__kernel_long_t` |  |
| `f_bavail` | `__kernel_long_t` |  |
| `f_files` | `__kernel_long_t` |  |
| `f_ffree` | `__kernel_long_t` |  |
| `f_fsid` | `__kernel_fsid_t` |  |
| `f_namelen` | `__kernel_long_t` |  |
| `f_frsize` | `__kernel_long_t` |  |
| `f_flags` | `__kernel_long_t` |  |
| `f_spare` | `[__kernel_long_t; 4]` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> statfs { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **RefUnwindSafe**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `statfs64`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct statfs64 {
    pub f_type: __kernel_long_t,
    pub f_bsize: __kernel_long_t,
    pub f_blocks: __u64,
    pub f_bfree: __u64,
    pub f_bavail: __u64,
    pub f_files: __u64,
    pub f_ffree: __u64,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __kernel_long_t,
    pub f_frsize: __kernel_long_t,
    pub f_flags: __kernel_long_t,
    pub f_spare: [__kernel_long_t; 4],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `f_type` | `__kernel_long_t` |  |
| `f_bsize` | `__kernel_long_t` |  |
| `f_blocks` | `__u64` |  |
| `f_bfree` | `__u64` |  |
| `f_bavail` | `__u64` |  |
| `f_files` | `__u64` |  |
| `f_ffree` | `__u64` |  |
| `f_fsid` | `__kernel_fsid_t` |  |
| `f_namelen` | `__kernel_long_t` |  |
| `f_frsize` | `__kernel_long_t` |  |
| `f_flags` | `__kernel_long_t` |  |
| `f_spare` | `[__kernel_long_t; 4]` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
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

- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> statfs64 { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Copy**
#### Struct `compat_statfs64`

**Attributes:**

- `#[repr(C, packed(4))]`

```rust
pub struct compat_statfs64 {
    pub f_type: __u32,
    pub f_bsize: __u32,
    pub f_blocks: __u64,
    pub f_bfree: __u64,
    pub f_bavail: __u64,
    pub f_files: __u64,
    pub f_ffree: __u64,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __u32,
    pub f_frsize: __u32,
    pub f_flags: __u32,
    pub f_spare: [__u32; 4],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `f_type` | `__u32` |  |
| `f_bsize` | `__u32` |  |
| `f_blocks` | `__u64` |  |
| `f_bfree` | `__u64` |  |
| `f_bavail` | `__u64` |  |
| `f_files` | `__u64` |  |
| `f_ffree` | `__u64` |  |
| `f_fsid` | `__kernel_fsid_t` |  |
| `f_namelen` | `__u32` |  |
| `f_frsize` | `__u32` |  |
| `f_flags` | `__u32` |  |
| `f_spare` | `[__u32; 4]` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> compat_statfs64 { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **RefUnwindSafe**
- **Copy**
- **Unpin**
#### Struct `user_desc`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct user_desc {
    pub entry_number: crate::ctypes::c_uint,
    pub base_addr: crate::ctypes::c_uint,
    pub limit: crate::ctypes::c_uint,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1]>,
    pub __bindgen_padding_0: [u8; 3],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `entry_number` | `crate::ctypes::c_uint` |  |
| `base_addr` | `crate::ctypes::c_uint` |  |
| `limit` | `crate::ctypes::c_uint` |  |
| `_bitfield_align_1` | `[u8; 0]` |  |
| `_bitfield_1` | `__BindgenBitfieldUnit<[u8; 1]>` |  |
| `__bindgen_padding_0` | `[u8; 3]` |  |

##### Implementations

###### Methods

- ```rust
  pub fn seg_32bit(self: &Self) -> crate::ctypes::c_uint { /* ... */ }
  ```

- ```rust
  pub fn set_seg_32bit(self: &mut Self, val: crate::ctypes::c_uint) { /* ... */ }
  ```

- ```rust
  pub unsafe fn seg_32bit_raw(this: *const Self) -> crate::ctypes::c_uint { /* ... */ }
  ```

- ```rust
  pub unsafe fn set_seg_32bit_raw(this: *mut Self, val: crate::ctypes::c_uint) { /* ... */ }
  ```

- ```rust
  pub fn contents(self: &Self) -> crate::ctypes::c_uint { /* ... */ }
  ```

- ```rust
  pub fn set_contents(self: &mut Self, val: crate::ctypes::c_uint) { /* ... */ }
  ```

- ```rust
  pub unsafe fn contents_raw(this: *const Self) -> crate::ctypes::c_uint { /* ... */ }
  ```

- ```rust
  pub unsafe fn set_contents_raw(this: *mut Self, val: crate::ctypes::c_uint) { /* ... */ }
  ```

- ```rust
  pub fn read_exec_only(self: &Self) -> crate::ctypes::c_uint { /* ... */ }
  ```

- ```rust
  pub fn set_read_exec_only(self: &mut Self, val: crate::ctypes::c_uint) { /* ... */ }
  ```

- ```rust
  pub unsafe fn read_exec_only_raw(this: *const Self) -> crate::ctypes::c_uint { /* ... */ }
  ```

- ```rust
  pub unsafe fn set_read_exec_only_raw(this: *mut Self, val: crate::ctypes::c_uint) { /* ... */ }
  ```

- ```rust
  pub fn limit_in_pages(self: &Self) -> crate::ctypes::c_uint { /* ... */ }
  ```

- ```rust
  pub fn set_limit_in_pages(self: &mut Self, val: crate::ctypes::c_uint) { /* ... */ }
  ```

- ```rust
  pub unsafe fn limit_in_pages_raw(this: *const Self) -> crate::ctypes::c_uint { /* ... */ }
  ```

- ```rust
  pub unsafe fn set_limit_in_pages_raw(this: *mut Self, val: crate::ctypes::c_uint) { /* ... */ }
  ```

- ```rust
  pub fn seg_not_present(self: &Self) -> crate::ctypes::c_uint { /* ... */ }
  ```

- ```rust
  pub fn set_seg_not_present(self: &mut Self, val: crate::ctypes::c_uint) { /* ... */ }
  ```

- ```rust
  pub unsafe fn seg_not_present_raw(this: *const Self) -> crate::ctypes::c_uint { /* ... */ }
  ```

- ```rust
  pub unsafe fn set_seg_not_present_raw(this: *mut Self, val: crate::ctypes::c_uint) { /* ... */ }
  ```

- ```rust
  pub fn useable(self: &Self) -> crate::ctypes::c_uint { /* ... */ }
  ```

- ```rust
  pub fn set_useable(self: &mut Self, val: crate::ctypes::c_uint) { /* ... */ }
  ```

- ```rust
  pub unsafe fn useable_raw(this: *const Self) -> crate::ctypes::c_uint { /* ... */ }
  ```

- ```rust
  pub unsafe fn set_useable_raw(this: *mut Self, val: crate::ctypes::c_uint) { /* ... */ }
  ```

- ```rust
  pub fn new_bitfield_1(seg_32bit: crate::ctypes::c_uint, contents: crate::ctypes::c_uint, read_exec_only: crate::ctypes::c_uint, limit_in_pages: crate::ctypes::c_uint, seg_not_present: crate::ctypes::c_uint, useable: crate::ctypes::c_uint) -> __BindgenBitfieldUnit<[u8; 1]> { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> user_desc { /* ... */ }
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

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `kernel_sigset_t`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct kernel_sigset_t {
    pub sig: [crate::ctypes::c_ulong; 1],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `sig` | `[crate::ctypes::c_ulong; 1]` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> kernel_sigset_t { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
#### Struct `kernel_sigaction`

**Attributes:**

- `#[repr(C)]`

```rust
pub struct kernel_sigaction {
    pub sa_handler_kernel: __kernel_sighandler_t,
    pub sa_flags: crate::ctypes::c_ulong,
    pub sa_restorer: __sigrestore_t,
    pub sa_mask: kernel_sigset_t,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `sa_handler_kernel` | `__kernel_sighandler_t` |  |
| `sa_flags` | `crate::ctypes::c_ulong` |  |
| `sa_restorer` | `__sigrestore_t` |  |
| `sa_mask` | `kernel_sigset_t` |  |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> kernel_sigaction { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Enum `fsconfig_command`

**Attributes:**

- `#[non_exhaustive]`
- `#[repr(u32)]`

```rust
pub enum fsconfig_command {
    FSCONFIG_SET_FLAG = 0,
    FSCONFIG_SET_STRING = 1,
    FSCONFIG_SET_BINARY = 2,
    FSCONFIG_SET_PATH = 3,
    FSCONFIG_SET_PATH_EMPTY = 4,
    FSCONFIG_SET_FD = 5,
    FSCONFIG_CMD_CREATE = 6,
    FSCONFIG_CMD_RECONFIGURE = 7,
    FSCONFIG_CMD_CREATE_EXCL = 8,
}
```

##### Variants

###### `FSCONFIG_SET_FLAG`

Discriminant: `0`

Discriminant value: `0`

###### `FSCONFIG_SET_STRING`

Discriminant: `1`

Discriminant value: `1`

###### `FSCONFIG_SET_BINARY`

Discriminant: `2`

Discriminant value: `2`

###### `FSCONFIG_SET_PATH`

Discriminant: `3`

Discriminant value: `3`

###### `FSCONFIG_SET_PATH_EMPTY`

Discriminant: `4`

Discriminant value: `4`

###### `FSCONFIG_SET_FD`

Discriminant: `5`

Discriminant value: `5`

###### `FSCONFIG_CMD_CREATE`

Discriminant: `6`

Discriminant value: `6`

###### `FSCONFIG_CMD_RECONFIGURE`

Discriminant: `7`

Discriminant value: `7`

###### `FSCONFIG_CMD_CREATE_EXCL`

Discriminant: `8`

Discriminant value: `8`

##### Implementations

###### Trait Implementations

- **Send**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &fsconfig_command) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> fsconfig_command { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
#### Enum `procmap_query_flags`

**Attributes:**

- `#[non_exhaustive]`
- `#[repr(u32)]`

```rust
pub enum procmap_query_flags {
    PROCMAP_QUERY_VMA_READABLE = 1,
    PROCMAP_QUERY_VMA_WRITABLE = 2,
    PROCMAP_QUERY_VMA_EXECUTABLE = 4,
    PROCMAP_QUERY_VMA_SHARED = 8,
    PROCMAP_QUERY_COVERING_OR_NEXT_VMA = 16,
    PROCMAP_QUERY_FILE_BACKED_VMA = 32,
}
```

##### Variants

###### `PROCMAP_QUERY_VMA_READABLE`

Discriminant: `1`

Discriminant value: `1`

###### `PROCMAP_QUERY_VMA_WRITABLE`

Discriminant: `2`

Discriminant value: `2`

###### `PROCMAP_QUERY_VMA_EXECUTABLE`

Discriminant: `4`

Discriminant value: `4`

###### `PROCMAP_QUERY_VMA_SHARED`

Discriminant: `8`

Discriminant value: `8`

###### `PROCMAP_QUERY_COVERING_OR_NEXT_VMA`

Discriminant: `16`

Discriminant value: `16`

###### `PROCMAP_QUERY_FILE_BACKED_VMA`

Discriminant: `32`

Discriminant value: `32`

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **StructuralPartialEq**
- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> procmap_query_flags { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &procmap_query_flags) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Enum `membarrier_cmd`

**Attributes:**

- `#[non_exhaustive]`
- `#[repr(u32)]`

```rust
pub enum membarrier_cmd {
    MEMBARRIER_CMD_QUERY = 0,
    MEMBARRIER_CMD_GLOBAL = 1,
    MEMBARRIER_CMD_GLOBAL_EXPEDITED = 2,
    MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED = 4,
    MEMBARRIER_CMD_PRIVATE_EXPEDITED = 8,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED = 16,
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE = 32,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE = 64,
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ = 128,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ = 256,
    MEMBARRIER_CMD_GET_REGISTRATIONS = 512,
}
```

##### Variants

###### `MEMBARRIER_CMD_QUERY`

Discriminant: `0`

Discriminant value: `0`

###### `MEMBARRIER_CMD_GLOBAL`

Discriminant: `1`

Discriminant value: `1`

###### `MEMBARRIER_CMD_GLOBAL_EXPEDITED`

Discriminant: `2`

Discriminant value: `2`

###### `MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED`

Discriminant: `4`

Discriminant value: `4`

###### `MEMBARRIER_CMD_PRIVATE_EXPEDITED`

Discriminant: `8`

Discriminant value: `8`

###### `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED`

Discriminant: `16`

Discriminant value: `16`

###### `MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE`

Discriminant: `32`

Discriminant value: `32`

###### `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE`

Discriminant: `64`

Discriminant value: `64`

###### `MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ`

Discriminant: `128`

Discriminant value: `128`

###### `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ`

Discriminant: `256`

Discriminant value: `256`

###### `MEMBARRIER_CMD_GET_REGISTRATIONS`

Discriminant: `512`

Discriminant value: `512`

##### Implementations

###### Methods

###### Trait Implementations

- **Freeze**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &membarrier_cmd) -> bool { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> membarrier_cmd { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Enum `membarrier_cmd_flag`

**Attributes:**

- `#[non_exhaustive]`
- `#[repr(u32)]`

```rust
pub enum membarrier_cmd_flag {
    MEMBARRIER_CMD_FLAG_CPU = 1,
}
```

##### Variants

###### `MEMBARRIER_CMD_FLAG_CPU`

Discriminant: `1`

Discriminant value: `1`

##### Implementations

###### Trait Implementations

- **Eq**
- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **StructuralPartialEq**
- **Send**
- **Unpin**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &membarrier_cmd_flag) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> membarrier_cmd_flag { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Union `fscrypt_get_policy_ex_arg__bindgen_ty_1`

**Attributes:**

- `#[repr(C)]`

```rust
pub union fscrypt_get_policy_ex_arg__bindgen_ty_1 {
    pub version: __u8,
    pub v1: fscrypt_policy_v1,
    pub v2: fscrypt_policy_v2,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `version` | `__u8` |  |
| `v1` | `fscrypt_policy_v1` |  |
| `v2` | `fscrypt_policy_v2` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **Sync**
- **UnwindSafe**
- **TryFrom**
  - `Error`: 
  - `try_from`: 
- **Unpin**
- **Borrow**
  - `borrow`: 
- **Freeze**
- **From**
  - `from`: Returns the argument unchanged.
- **RefUnwindSafe**
- **Clone**
  - `clone`: 
- **Copy**
- **BorrowMut**
  - `borrow_mut`: 
- **Any**
  - `type_id`: 
- **CloneToUninit**
  - `clone_to_uninit`: 
- **TryInto**
  - `Error`: 
  - `try_into`: 
- **Into**
  - `into`: Calls `U::from(self)`.

#### Union `fscrypt_key_specifier__bindgen_ty_1`

**Attributes:**

- `#[repr(C)]`

```rust
pub union fscrypt_key_specifier__bindgen_ty_1 {
    pub __reserved: [__u8; 32],
    pub descriptor: [__u8; 8],
    pub identifier: [__u8; 16],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `__reserved` | `[__u8; 32]` |  |
| `descriptor` | `[__u8; 8]` |  |
| `identifier` | `[__u8; 16]` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - `into`: Calls `U::from(self)`.
- **UnwindSafe**
- **TryFrom**
  - `Error`: 
  - `try_from`: 
- **Sync**
- **CloneToUninit**
  - `clone_to_uninit`: 
- **Unpin**
- **Send**
- **TryInto**
  - `Error`: 
  - `try_into`: 
- **Borrow**
  - `borrow`: 
- **Any**
  - `type_id`: 
- **From**
  - `from`: Returns the argument unchanged.
- **BorrowMut**
  - `borrow_mut`: 
- **RefUnwindSafe**
- **Freeze**
- **Clone**
  - `clone`: 
- **Copy**

#### Union `sigval`

**Attributes:**

- `#[repr(C)]`

```rust
pub union sigval {
    pub sival_int: crate::ctypes::c_int,
    pub sival_ptr: *mut crate::ctypes::c_void,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `sival_int` | `crate::ctypes::c_int` |  |
| `sival_ptr` | `*mut crate::ctypes::c_void` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - `clone_to_uninit`: 
- **From**
  - `from`: Returns the argument unchanged.
- **Copy**
- **Sync**
- **TryInto**
  - `Error`: 
  - `try_into`: 
- **Send**
- **Freeze**
- **Clone**
  - `clone`: 
- **RefUnwindSafe**
- **Any**
  - `type_id`: 
- **UnwindSafe**
- **Borrow**
  - `borrow`: 
- **TryFrom**
  - `Error`: 
  - `try_from`: 
- **BorrowMut**
  - `borrow_mut`: 
- **Unpin**
- **Into**
  - `into`: Calls `U::from(self)`.

#### Union `__sifields`

**Attributes:**

- `#[repr(C)]`

```rust
pub union __sifields {
    pub _kill: __sifields__bindgen_ty_1,
    pub _timer: __sifields__bindgen_ty_2,
    pub _rt: __sifields__bindgen_ty_3,
    pub _sigchld: __sifields__bindgen_ty_4,
    pub _sigfault: __sifields__bindgen_ty_5,
    pub _sigpoll: __sifields__bindgen_ty_6,
    pub _sigsys: __sifields__bindgen_ty_7,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_kill` | `__sifields__bindgen_ty_1` |  |
| `_timer` | `__sifields__bindgen_ty_2` |  |
| `_rt` | `__sifields__bindgen_ty_3` |  |
| `_sigchld` | `__sifields__bindgen_ty_4` |  |
| `_sigfault` | `__sifields__bindgen_ty_5` |  |
| `_sigpoll` | `__sifields__bindgen_ty_6` |  |
| `_sigsys` | `__sifields__bindgen_ty_7` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **CloneToUninit**
  - `clone_to_uninit`: 
- **Clone**
  - `clone`: 
- **From**
  - `from`: Returns the argument unchanged.
- **Freeze**
- **Borrow**
  - `borrow`: 
- **Send**
- **TryInto**
  - `Error`: 
  - `try_into`: 
- **Copy**
- **RefUnwindSafe**
- **Into**
  - `into`: Calls `U::from(self)`.
- **BorrowMut**
  - `borrow_mut`: 
- **Unpin**
- **Any**
  - `type_id`: 
- **Sync**
- **TryFrom**
  - `Error`: 
  - `try_from`: 

#### Union `__sifields__bindgen_ty_5__bindgen_ty_1`

**Attributes:**

- `#[repr(C)]`

```rust
pub union __sifields__bindgen_ty_5__bindgen_ty_1 {
    pub _trapno: crate::ctypes::c_int,
    pub _addr_lsb: crate::ctypes::c_short,
    pub _addr_bnd: __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1,
    pub _addr_pkey: __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2,
    pub _perf: __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_trapno` | `crate::ctypes::c_int` |  |
| `_addr_lsb` | `crate::ctypes::c_short` |  |
| `_addr_bnd` | `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1` |  |
| `_addr_pkey` | `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2` |  |
| `_perf` | `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - `type_id`: 
- **From**
  - `from`: Returns the argument unchanged.
- **Copy**
- **Into**
  - `into`: Calls `U::from(self)`.
- **TryInto**
  - `Error`: 
  - `try_into`: 
- **UnwindSafe**
- **Sync**
- **Send**
- **RefUnwindSafe**
- **BorrowMut**
  - `borrow_mut`: 
- **Freeze**
- **Clone**
  - `clone`: 
- **Unpin**
- **TryFrom**
  - `Error`: 
  - `try_from`: 
- **CloneToUninit**
  - `clone_to_uninit`: 
- **Borrow**
  - `borrow`: 

#### Union `siginfo__bindgen_ty_1`

**Attributes:**

- `#[repr(C)]`

```rust
pub union siginfo__bindgen_ty_1 {
    pub __bindgen_anon_1: siginfo__bindgen_ty_1__bindgen_ty_1,
    pub _si_pad: [crate::ctypes::c_int; 32],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `__bindgen_anon_1` | `siginfo__bindgen_ty_1__bindgen_ty_1` |  |
| `_si_pad` | `[crate::ctypes::c_int; 32]` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - `type_id`: 
- **From**
  - `from`: Returns the argument unchanged.
- **Sync**
- **TryInto**
  - `Error`: 
  - `try_into`: 
- **Into**
  - `into`: Calls `U::from(self)`.
- **BorrowMut**
  - `borrow_mut`: 
- **CloneToUninit**
  - `clone_to_uninit`: 
- **Borrow**
  - `borrow`: 
- **UnwindSafe**
- **Unpin**
- **Copy**
- **TryFrom**
  - `Error`: 
  - `try_from`: 
- **Send**
- **Freeze**
- **RefUnwindSafe**
- **Clone**
  - `clone`: 

#### Union `sigevent__bindgen_ty_1`

**Attributes:**

- `#[repr(C)]`

```rust
pub union sigevent__bindgen_ty_1 {
    pub _pad: [crate::ctypes::c_int; 12],
    pub _tid: crate::ctypes::c_int,
    pub _sigev_thread: sigevent__bindgen_ty_1__bindgen_ty_1,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_pad` | `[crate::ctypes::c_int; 12]` |  |
| `_tid` | `crate::ctypes::c_int` |  |
| `_sigev_thread` | `sigevent__bindgen_ty_1__bindgen_ty_1` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **Borrow**
  - `borrow`: 
- **Sync**
- **UnwindSafe**
- **CloneToUninit**
  - `clone_to_uninit`: 
- **Unpin**
- **Clone**
  - `clone`: 
- **Freeze**
- **TryInto**
  - `Error`: 
  - `try_into`: 
- **BorrowMut**
  - `borrow_mut`: 
- **Into**
  - `into`: Calls `U::from(self)`.
- **Copy**
- **Any**
  - `type_id`: 
- **From**
  - `from`: Returns the argument unchanged.
- **RefUnwindSafe**
- **TryFrom**
  - `Error`: 
  - `try_from`: 

#### Union `uffd_msg__bindgen_ty_1`

**Attributes:**

- `#[repr(C)]`

```rust
pub union uffd_msg__bindgen_ty_1 {
    pub pagefault: uffd_msg__bindgen_ty_1__bindgen_ty_1,
    pub fork: uffd_msg__bindgen_ty_1__bindgen_ty_2,
    pub remap: uffd_msg__bindgen_ty_1__bindgen_ty_3,
    pub remove: uffd_msg__bindgen_ty_1__bindgen_ty_4,
    pub reserved: uffd_msg__bindgen_ty_1__bindgen_ty_5,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `pagefault` | `uffd_msg__bindgen_ty_1__bindgen_ty_1` |  |
| `fork` | `uffd_msg__bindgen_ty_1__bindgen_ty_2` |  |
| `remap` | `uffd_msg__bindgen_ty_1__bindgen_ty_3` |  |
| `remove` | `uffd_msg__bindgen_ty_1__bindgen_ty_4` |  |
| `reserved` | `uffd_msg__bindgen_ty_1__bindgen_ty_5` |  |

##### Implementations

###### Trait Implementations

- **Clone**
  - `clone`: 
- **Sync**
- **Copy**
- **Any**
  - `type_id`: 
- **Send**
- **Borrow**
  - `borrow`: 
- **From**
  - `from`: Returns the argument unchanged.
- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **Unpin**
- **BorrowMut**
  - `borrow_mut`: 
- **Into**
  - `into`: Calls `U::from(self)`.
- **TryFrom**
  - `Error`: 
  - `try_from`: 
- **TryInto**
  - `Error`: 
  - `try_into`: 
- **CloneToUninit**
  - `clone_to_uninit`: 

#### Union `uffd_msg__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1`

**Attributes:**

- `#[repr(C)]`

```rust
pub union uffd_msg__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
    pub ptid: __u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ptid` | `__u32` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **CloneToUninit**
  - `clone_to_uninit`: 
- **Borrow**
  - `borrow`: 
- **Any**
  - `type_id`: 
- **UnwindSafe**
- **BorrowMut**
  - `borrow_mut`: 
- **Sync**
- **RefUnwindSafe**
- **TryInto**
  - `Error`: 
  - `try_into`: 
- **Freeze**
- **Into**
  - `into`: Calls `U::from(self)`.
- **From**
  - `from`: Returns the argument unchanged.
- **Clone**
  - `clone`: 
- **Copy**
- **TryFrom**
  - `Error`: 
  - `try_from`: 
- **Send**

### Constants and Statics

#### Constant `LINUX_VERSION_CODE`

```rust
pub const LINUX_VERSION_CODE: u32 = 396544;
```

#### Constant `LINUX_VERSION_MAJOR`

```rust
pub const LINUX_VERSION_MAJOR: u32 = 6;
```

#### Constant `LINUX_VERSION_PATCHLEVEL`

```rust
pub const LINUX_VERSION_PATCHLEVEL: u32 = 13;
```

#### Constant `LINUX_VERSION_SUBLEVEL`

```rust
pub const LINUX_VERSION_SUBLEVEL: u32 = 0;
```

#### Constant `AT_SYSINFO_EHDR`

```rust
pub const AT_SYSINFO_EHDR: u32 = 33;
```

#### Constant `AT_MINSIGSTKSZ`

```rust
pub const AT_MINSIGSTKSZ: u32 = 51;
```

#### Constant `AT_VECTOR_SIZE_ARCH`

```rust
pub const AT_VECTOR_SIZE_ARCH: u32 = 2;
```

#### Constant `AT_NULL`

```rust
pub const AT_NULL: u32 = 0;
```

#### Constant `AT_IGNORE`

```rust
pub const AT_IGNORE: u32 = 1;
```

#### Constant `AT_EXECFD`

```rust
pub const AT_EXECFD: u32 = 2;
```

#### Constant `AT_PHDR`

```rust
pub const AT_PHDR: u32 = 3;
```

#### Constant `AT_PHENT`

```rust
pub const AT_PHENT: u32 = 4;
```

#### Constant `AT_PHNUM`

```rust
pub const AT_PHNUM: u32 = 5;
```

#### Constant `AT_PAGESZ`

```rust
pub const AT_PAGESZ: u32 = 6;
```

#### Constant `AT_BASE`

```rust
pub const AT_BASE: u32 = 7;
```

#### Constant `AT_FLAGS`

```rust
pub const AT_FLAGS: u32 = 8;
```

#### Constant `AT_ENTRY`

```rust
pub const AT_ENTRY: u32 = 9;
```

#### Constant `AT_NOTELF`

```rust
pub const AT_NOTELF: u32 = 10;
```

#### Constant `AT_UID`

```rust
pub const AT_UID: u32 = 11;
```

#### Constant `AT_EUID`

```rust
pub const AT_EUID: u32 = 12;
```

#### Constant `AT_GID`

```rust
pub const AT_GID: u32 = 13;
```

#### Constant `AT_EGID`

```rust
pub const AT_EGID: u32 = 14;
```

#### Constant `AT_PLATFORM`

```rust
pub const AT_PLATFORM: u32 = 15;
```

#### Constant `AT_HWCAP`

```rust
pub const AT_HWCAP: u32 = 16;
```

#### Constant `AT_CLKTCK`

```rust
pub const AT_CLKTCK: u32 = 17;
```

#### Constant `AT_SECURE`

```rust
pub const AT_SECURE: u32 = 23;
```

#### Constant `AT_BASE_PLATFORM`

```rust
pub const AT_BASE_PLATFORM: u32 = 24;
```

#### Constant `AT_RANDOM`

```rust
pub const AT_RANDOM: u32 = 25;
```

#### Constant `AT_HWCAP2`

```rust
pub const AT_HWCAP2: u32 = 26;
```

#### Constant `AT_RSEQ_FEATURE_SIZE`

```rust
pub const AT_RSEQ_FEATURE_SIZE: u32 = 27;
```

#### Constant `AT_RSEQ_ALIGN`

```rust
pub const AT_RSEQ_ALIGN: u32 = 28;
```

#### Constant `AT_HWCAP3`

```rust
pub const AT_HWCAP3: u32 = 29;
```

#### Constant `AT_HWCAP4`

```rust
pub const AT_HWCAP4: u32 = 30;
```

#### Constant `AT_EXECFN`

```rust
pub const AT_EXECFN: u32 = 31;
```

#### Constant `__BITS_PER_LONG_LONG`

```rust
pub const __BITS_PER_LONG_LONG: u32 = 64;
```

#### Constant `__FD_SETSIZE`

```rust
pub const __FD_SETSIZE: u32 = 1024;
```

#### Constant `_LINUX_CAPABILITY_VERSION_1`

```rust
pub const _LINUX_CAPABILITY_VERSION_1: u32 = 429392688;
```

#### Constant `_LINUX_CAPABILITY_U32S_1`

```rust
pub const _LINUX_CAPABILITY_U32S_1: u32 = 1;
```

#### Constant `_LINUX_CAPABILITY_VERSION_2`

```rust
pub const _LINUX_CAPABILITY_VERSION_2: u32 = 537333798;
```

#### Constant `_LINUX_CAPABILITY_U32S_2`

```rust
pub const _LINUX_CAPABILITY_U32S_2: u32 = 2;
```

#### Constant `_LINUX_CAPABILITY_VERSION_3`

```rust
pub const _LINUX_CAPABILITY_VERSION_3: u32 = 537396514;
```

#### Constant `_LINUX_CAPABILITY_U32S_3`

```rust
pub const _LINUX_CAPABILITY_U32S_3: u32 = 2;
```

#### Constant `VFS_CAP_REVISION_MASK`

```rust
pub const VFS_CAP_REVISION_MASK: u32 = 4278190080;
```

#### Constant `VFS_CAP_REVISION_SHIFT`

```rust
pub const VFS_CAP_REVISION_SHIFT: u32 = 24;
```

#### Constant `VFS_CAP_FLAGS_MASK`

```rust
pub const VFS_CAP_FLAGS_MASK: i64 = -4278190081;
```

#### Constant `VFS_CAP_FLAGS_EFFECTIVE`

```rust
pub const VFS_CAP_FLAGS_EFFECTIVE: u32 = 1;
```

#### Constant `VFS_CAP_REVISION_1`

```rust
pub const VFS_CAP_REVISION_1: u32 = 16777216;
```

#### Constant `VFS_CAP_U32_1`

```rust
pub const VFS_CAP_U32_1: u32 = 1;
```

#### Constant `VFS_CAP_REVISION_2`

```rust
pub const VFS_CAP_REVISION_2: u32 = 33554432;
```

#### Constant `VFS_CAP_U32_2`

```rust
pub const VFS_CAP_U32_2: u32 = 2;
```

#### Constant `VFS_CAP_REVISION_3`

```rust
pub const VFS_CAP_REVISION_3: u32 = 50331648;
```

#### Constant `VFS_CAP_U32_3`

```rust
pub const VFS_CAP_U32_3: u32 = 2;
```

#### Constant `VFS_CAP_U32`

```rust
pub const VFS_CAP_U32: u32 = 2;
```

#### Constant `VFS_CAP_REVISION`

```rust
pub const VFS_CAP_REVISION: u32 = 50331648;
```

#### Constant `_LINUX_CAPABILITY_VERSION`

```rust
pub const _LINUX_CAPABILITY_VERSION: u32 = 429392688;
```

#### Constant `_LINUX_CAPABILITY_U32S`

```rust
pub const _LINUX_CAPABILITY_U32S: u32 = 1;
```

#### Constant `CAP_CHOWN`

```rust
pub const CAP_CHOWN: u32 = 0;
```

#### Constant `CAP_DAC_OVERRIDE`

```rust
pub const CAP_DAC_OVERRIDE: u32 = 1;
```

#### Constant `CAP_DAC_READ_SEARCH`

```rust
pub const CAP_DAC_READ_SEARCH: u32 = 2;
```

#### Constant `CAP_FOWNER`

```rust
pub const CAP_FOWNER: u32 = 3;
```

#### Constant `CAP_FSETID`

```rust
pub const CAP_FSETID: u32 = 4;
```

#### Constant `CAP_KILL`

```rust
pub const CAP_KILL: u32 = 5;
```

#### Constant `CAP_SETGID`

```rust
pub const CAP_SETGID: u32 = 6;
```

#### Constant `CAP_SETUID`

```rust
pub const CAP_SETUID: u32 = 7;
```

#### Constant `CAP_SETPCAP`

```rust
pub const CAP_SETPCAP: u32 = 8;
```

#### Constant `CAP_LINUX_IMMUTABLE`

```rust
pub const CAP_LINUX_IMMUTABLE: u32 = 9;
```

#### Constant `CAP_NET_BIND_SERVICE`

```rust
pub const CAP_NET_BIND_SERVICE: u32 = 10;
```

#### Constant `CAP_NET_BROADCAST`

```rust
pub const CAP_NET_BROADCAST: u32 = 11;
```

#### Constant `CAP_NET_ADMIN`

```rust
pub const CAP_NET_ADMIN: u32 = 12;
```

#### Constant `CAP_NET_RAW`

```rust
pub const CAP_NET_RAW: u32 = 13;
```

#### Constant `CAP_IPC_LOCK`

```rust
pub const CAP_IPC_LOCK: u32 = 14;
```

#### Constant `CAP_IPC_OWNER`

```rust
pub const CAP_IPC_OWNER: u32 = 15;
```

#### Constant `CAP_SYS_MODULE`

```rust
pub const CAP_SYS_MODULE: u32 = 16;
```

#### Constant `CAP_SYS_RAWIO`

```rust
pub const CAP_SYS_RAWIO: u32 = 17;
```

#### Constant `CAP_SYS_CHROOT`

```rust
pub const CAP_SYS_CHROOT: u32 = 18;
```

#### Constant `CAP_SYS_PTRACE`

```rust
pub const CAP_SYS_PTRACE: u32 = 19;
```

#### Constant `CAP_SYS_PACCT`

```rust
pub const CAP_SYS_PACCT: u32 = 20;
```

#### Constant `CAP_SYS_ADMIN`

```rust
pub const CAP_SYS_ADMIN: u32 = 21;
```

#### Constant `CAP_SYS_BOOT`

```rust
pub const CAP_SYS_BOOT: u32 = 22;
```

#### Constant `CAP_SYS_NICE`

```rust
pub const CAP_SYS_NICE: u32 = 23;
```

#### Constant `CAP_SYS_RESOURCE`

```rust
pub const CAP_SYS_RESOURCE: u32 = 24;
```

#### Constant `CAP_SYS_TIME`

```rust
pub const CAP_SYS_TIME: u32 = 25;
```

#### Constant `CAP_SYS_TTY_CONFIG`

```rust
pub const CAP_SYS_TTY_CONFIG: u32 = 26;
```

#### Constant `CAP_MKNOD`

```rust
pub const CAP_MKNOD: u32 = 27;
```

#### Constant `CAP_LEASE`

```rust
pub const CAP_LEASE: u32 = 28;
```

#### Constant `CAP_AUDIT_WRITE`

```rust
pub const CAP_AUDIT_WRITE: u32 = 29;
```

#### Constant `CAP_AUDIT_CONTROL`

```rust
pub const CAP_AUDIT_CONTROL: u32 = 30;
```

#### Constant `CAP_SETFCAP`

```rust
pub const CAP_SETFCAP: u32 = 31;
```

#### Constant `CAP_MAC_OVERRIDE`

```rust
pub const CAP_MAC_OVERRIDE: u32 = 32;
```

#### Constant `CAP_MAC_ADMIN`

```rust
pub const CAP_MAC_ADMIN: u32 = 33;
```

#### Constant `CAP_SYSLOG`

```rust
pub const CAP_SYSLOG: u32 = 34;
```

#### Constant `CAP_WAKE_ALARM`

```rust
pub const CAP_WAKE_ALARM: u32 = 35;
```

#### Constant `CAP_BLOCK_SUSPEND`

```rust
pub const CAP_BLOCK_SUSPEND: u32 = 36;
```

#### Constant `CAP_AUDIT_READ`

```rust
pub const CAP_AUDIT_READ: u32 = 37;
```

#### Constant `CAP_PERFMON`

```rust
pub const CAP_PERFMON: u32 = 38;
```

#### Constant `CAP_BPF`

```rust
pub const CAP_BPF: u32 = 39;
```

#### Constant `CAP_CHECKPOINT_RESTORE`

```rust
pub const CAP_CHECKPOINT_RESTORE: u32 = 40;
```

#### Constant `CAP_LAST_CAP`

```rust
pub const CAP_LAST_CAP: u32 = 40;
```

#### Constant `O_DIRECTORY`

```rust
pub const O_DIRECTORY: u32 = 16384;
```

#### Constant `O_NOFOLLOW`

```rust
pub const O_NOFOLLOW: u32 = 32768;
```

#### Constant `O_DIRECT`

```rust
pub const O_DIRECT: u32 = 65536;
```

#### Constant `O_LARGEFILE`

```rust
pub const O_LARGEFILE: u32 = 131072;
```

#### Constant `O_ACCMODE`

```rust
pub const O_ACCMODE: u32 = 3;
```

#### Constant `O_RDONLY`

```rust
pub const O_RDONLY: u32 = 0;
```

#### Constant `O_WRONLY`

```rust
pub const O_WRONLY: u32 = 1;
```

#### Constant `O_RDWR`

```rust
pub const O_RDWR: u32 = 2;
```

#### Constant `O_CREAT`

```rust
pub const O_CREAT: u32 = 64;
```

#### Constant `O_EXCL`

```rust
pub const O_EXCL: u32 = 128;
```

#### Constant `O_NOCTTY`

```rust
pub const O_NOCTTY: u32 = 256;
```

#### Constant `O_TRUNC`

```rust
pub const O_TRUNC: u32 = 512;
```

#### Constant `O_APPEND`

```rust
pub const O_APPEND: u32 = 1024;
```

#### Constant `O_NONBLOCK`

```rust
pub const O_NONBLOCK: u32 = 2048;
```

#### Constant `O_DSYNC`

```rust
pub const O_DSYNC: u32 = 4096;
```

#### Constant `FASYNC`

```rust
pub const FASYNC: u32 = 8192;
```

#### Constant `O_NOATIME`

```rust
pub const O_NOATIME: u32 = 262144;
```

#### Constant `O_CLOEXEC`

```rust
pub const O_CLOEXEC: u32 = 524288;
```

#### Constant `__O_SYNC`

```rust
pub const __O_SYNC: u32 = 1048576;
```

#### Constant `O_SYNC`

```rust
pub const O_SYNC: u32 = 1052672;
```

#### Constant `O_PATH`

```rust
pub const O_PATH: u32 = 2097152;
```

#### Constant `__O_TMPFILE`

```rust
pub const __O_TMPFILE: u32 = 4194304;
```

#### Constant `O_TMPFILE`

```rust
pub const O_TMPFILE: u32 = 4210688;
```

#### Constant `O_NDELAY`

```rust
pub const O_NDELAY: u32 = 2048;
```

#### Constant `F_DUPFD`

```rust
pub const F_DUPFD: u32 = 0;
```

#### Constant `F_GETFD`

```rust
pub const F_GETFD: u32 = 1;
```

#### Constant `F_SETFD`

```rust
pub const F_SETFD: u32 = 2;
```

#### Constant `F_GETFL`

```rust
pub const F_GETFL: u32 = 3;
```

#### Constant `F_SETFL`

```rust
pub const F_SETFL: u32 = 4;
```

#### Constant `F_GETLK`

```rust
pub const F_GETLK: u32 = 5;
```

#### Constant `F_SETLK`

```rust
pub const F_SETLK: u32 = 6;
```

#### Constant `F_SETLKW`

```rust
pub const F_SETLKW: u32 = 7;
```

#### Constant `F_SETOWN`

```rust
pub const F_SETOWN: u32 = 8;
```

#### Constant `F_GETOWN`

```rust
pub const F_GETOWN: u32 = 9;
```

#### Constant `F_SETSIG`

```rust
pub const F_SETSIG: u32 = 10;
```

#### Constant `F_GETSIG`

```rust
pub const F_GETSIG: u32 = 11;
```

#### Constant `F_SETOWN_EX`

```rust
pub const F_SETOWN_EX: u32 = 15;
```

#### Constant `F_GETOWN_EX`

```rust
pub const F_GETOWN_EX: u32 = 16;
```

#### Constant `F_GETOWNER_UIDS`

```rust
pub const F_GETOWNER_UIDS: u32 = 17;
```

#### Constant `F_OFD_GETLK`

```rust
pub const F_OFD_GETLK: u32 = 36;
```

#### Constant `F_OFD_SETLK`

```rust
pub const F_OFD_SETLK: u32 = 37;
```

#### Constant `F_OFD_SETLKW`

```rust
pub const F_OFD_SETLKW: u32 = 38;
```

#### Constant `F_OWNER_TID`

```rust
pub const F_OWNER_TID: u32 = 0;
```

#### Constant `F_OWNER_PID`

```rust
pub const F_OWNER_PID: u32 = 1;
```

#### Constant `F_OWNER_PGRP`

```rust
pub const F_OWNER_PGRP: u32 = 2;
```

#### Constant `FD_CLOEXEC`

```rust
pub const FD_CLOEXEC: u32 = 1;
```

#### Constant `F_RDLCK`

```rust
pub const F_RDLCK: u32 = 0;
```

#### Constant `F_WRLCK`

```rust
pub const F_WRLCK: u32 = 1;
```

#### Constant `F_UNLCK`

```rust
pub const F_UNLCK: u32 = 2;
```

#### Constant `F_EXLCK`

```rust
pub const F_EXLCK: u32 = 4;
```

#### Constant `F_SHLCK`

```rust
pub const F_SHLCK: u32 = 8;
```

#### Constant `LOCK_SH`

```rust
pub const LOCK_SH: u32 = 1;
```

#### Constant `LOCK_EX`

```rust
pub const LOCK_EX: u32 = 2;
```

#### Constant `LOCK_NB`

```rust
pub const LOCK_NB: u32 = 4;
```

#### Constant `LOCK_UN`

```rust
pub const LOCK_UN: u32 = 8;
```

#### Constant `LOCK_MAND`

```rust
pub const LOCK_MAND: u32 = 32;
```

#### Constant `LOCK_READ`

```rust
pub const LOCK_READ: u32 = 64;
```

#### Constant `LOCK_WRITE`

```rust
pub const LOCK_WRITE: u32 = 128;
```

#### Constant `LOCK_RW`

```rust
pub const LOCK_RW: u32 = 192;
```

#### Constant `F_LINUX_SPECIFIC_BASE`

```rust
pub const F_LINUX_SPECIFIC_BASE: u32 = 1024;
```

#### Constant `RESOLVE_NO_XDEV`

```rust
pub const RESOLVE_NO_XDEV: u32 = 1;
```

#### Constant `RESOLVE_NO_MAGICLINKS`

```rust
pub const RESOLVE_NO_MAGICLINKS: u32 = 2;
```

#### Constant `RESOLVE_NO_SYMLINKS`

```rust
pub const RESOLVE_NO_SYMLINKS: u32 = 4;
```

#### Constant `RESOLVE_BENEATH`

```rust
pub const RESOLVE_BENEATH: u32 = 8;
```

#### Constant `RESOLVE_IN_ROOT`

```rust
pub const RESOLVE_IN_ROOT: u32 = 16;
```

#### Constant `RESOLVE_CACHED`

```rust
pub const RESOLVE_CACHED: u32 = 32;
```

#### Constant `F_SETLEASE`

```rust
pub const F_SETLEASE: u32 = 1024;
```

#### Constant `F_GETLEASE`

```rust
pub const F_GETLEASE: u32 = 1025;
```

#### Constant `F_NOTIFY`

```rust
pub const F_NOTIFY: u32 = 1026;
```

#### Constant `F_DUPFD_QUERY`

```rust
pub const F_DUPFD_QUERY: u32 = 1027;
```

#### Constant `F_CREATED_QUERY`

```rust
pub const F_CREATED_QUERY: u32 = 1028;
```

#### Constant `F_CANCELLK`

```rust
pub const F_CANCELLK: u32 = 1029;
```

#### Constant `F_DUPFD_CLOEXEC`

```rust
pub const F_DUPFD_CLOEXEC: u32 = 1030;
```

#### Constant `F_SETPIPE_SZ`

```rust
pub const F_SETPIPE_SZ: u32 = 1031;
```

#### Constant `F_GETPIPE_SZ`

```rust
pub const F_GETPIPE_SZ: u32 = 1032;
```

#### Constant `F_ADD_SEALS`

```rust
pub const F_ADD_SEALS: u32 = 1033;
```

#### Constant `F_GET_SEALS`

```rust
pub const F_GET_SEALS: u32 = 1034;
```

#### Constant `F_SEAL_SEAL`

```rust
pub const F_SEAL_SEAL: u32 = 1;
```

#### Constant `F_SEAL_SHRINK`

```rust
pub const F_SEAL_SHRINK: u32 = 2;
```

#### Constant `F_SEAL_GROW`

```rust
pub const F_SEAL_GROW: u32 = 4;
```

#### Constant `F_SEAL_WRITE`

```rust
pub const F_SEAL_WRITE: u32 = 8;
```

#### Constant `F_SEAL_FUTURE_WRITE`

```rust
pub const F_SEAL_FUTURE_WRITE: u32 = 16;
```

#### Constant `F_SEAL_EXEC`

```rust
pub const F_SEAL_EXEC: u32 = 32;
```

#### Constant `F_GET_RW_HINT`

```rust
pub const F_GET_RW_HINT: u32 = 1035;
```

#### Constant `F_SET_RW_HINT`

```rust
pub const F_SET_RW_HINT: u32 = 1036;
```

#### Constant `F_GET_FILE_RW_HINT`

```rust
pub const F_GET_FILE_RW_HINT: u32 = 1037;
```

#### Constant `F_SET_FILE_RW_HINT`

```rust
pub const F_SET_FILE_RW_HINT: u32 = 1038;
```

#### Constant `RWH_WRITE_LIFE_NOT_SET`

```rust
pub const RWH_WRITE_LIFE_NOT_SET: u32 = 0;
```

#### Constant `RWH_WRITE_LIFE_NONE`

```rust
pub const RWH_WRITE_LIFE_NONE: u32 = 1;
```

#### Constant `RWH_WRITE_LIFE_SHORT`

```rust
pub const RWH_WRITE_LIFE_SHORT: u32 = 2;
```

#### Constant `RWH_WRITE_LIFE_MEDIUM`

```rust
pub const RWH_WRITE_LIFE_MEDIUM: u32 = 3;
```

#### Constant `RWH_WRITE_LIFE_LONG`

```rust
pub const RWH_WRITE_LIFE_LONG: u32 = 4;
```

#### Constant `RWH_WRITE_LIFE_EXTREME`

```rust
pub const RWH_WRITE_LIFE_EXTREME: u32 = 5;
```

#### Constant `RWF_WRITE_LIFE_NOT_SET`

```rust
pub const RWF_WRITE_LIFE_NOT_SET: u32 = 0;
```

#### Constant `DN_ACCESS`

```rust
pub const DN_ACCESS: u32 = 1;
```

#### Constant `DN_MODIFY`

```rust
pub const DN_MODIFY: u32 = 2;
```

#### Constant `DN_CREATE`

```rust
pub const DN_CREATE: u32 = 4;
```

#### Constant `DN_DELETE`

```rust
pub const DN_DELETE: u32 = 8;
```

#### Constant `DN_RENAME`

```rust
pub const DN_RENAME: u32 = 16;
```

#### Constant `DN_ATTRIB`

```rust
pub const DN_ATTRIB: u32 = 32;
```

#### Constant `DN_MULTISHOT`

```rust
pub const DN_MULTISHOT: u32 = 2147483648;
```

#### Constant `AT_FDCWD`

```rust
pub const AT_FDCWD: i32 = -100;
```

#### Constant `AT_SYMLINK_NOFOLLOW`

```rust
pub const AT_SYMLINK_NOFOLLOW: u32 = 256;
```

#### Constant `AT_SYMLINK_FOLLOW`

```rust
pub const AT_SYMLINK_FOLLOW: u32 = 1024;
```

#### Constant `AT_NO_AUTOMOUNT`

```rust
pub const AT_NO_AUTOMOUNT: u32 = 2048;
```

#### Constant `AT_EMPTY_PATH`

```rust
pub const AT_EMPTY_PATH: u32 = 4096;
```

#### Constant `AT_STATX_SYNC_TYPE`

```rust
pub const AT_STATX_SYNC_TYPE: u32 = 24576;
```

#### Constant `AT_STATX_SYNC_AS_STAT`

```rust
pub const AT_STATX_SYNC_AS_STAT: u32 = 0;
```

#### Constant `AT_STATX_FORCE_SYNC`

```rust
pub const AT_STATX_FORCE_SYNC: u32 = 8192;
```

#### Constant `AT_STATX_DONT_SYNC`

```rust
pub const AT_STATX_DONT_SYNC: u32 = 16384;
```

#### Constant `AT_RECURSIVE`

```rust
pub const AT_RECURSIVE: u32 = 32768;
```

#### Constant `AT_RENAME_NOREPLACE`

```rust
pub const AT_RENAME_NOREPLACE: u32 = 1;
```

#### Constant `AT_RENAME_EXCHANGE`

```rust
pub const AT_RENAME_EXCHANGE: u32 = 2;
```

#### Constant `AT_RENAME_WHITEOUT`

```rust
pub const AT_RENAME_WHITEOUT: u32 = 4;
```

#### Constant `AT_EACCESS`

```rust
pub const AT_EACCESS: u32 = 512;
```

#### Constant `AT_REMOVEDIR`

```rust
pub const AT_REMOVEDIR: u32 = 512;
```

#### Constant `AT_HANDLE_FID`

```rust
pub const AT_HANDLE_FID: u32 = 512;
```

#### Constant `AT_HANDLE_MNT_ID_UNIQUE`

```rust
pub const AT_HANDLE_MNT_ID_UNIQUE: u32 = 1;
```

#### Constant `AT_HANDLE_CONNECTABLE`

```rust
pub const AT_HANDLE_CONNECTABLE: u32 = 2;
```

#### Constant `EPOLL_CLOEXEC`

```rust
pub const EPOLL_CLOEXEC: u32 = 524288;
```

#### Constant `EPOLL_CTL_ADD`

```rust
pub const EPOLL_CTL_ADD: u32 = 1;
```

#### Constant `EPOLL_CTL_DEL`

```rust
pub const EPOLL_CTL_DEL: u32 = 2;
```

#### Constant `EPOLL_CTL_MOD`

```rust
pub const EPOLL_CTL_MOD: u32 = 3;
```

#### Constant `EPOLL_IOC_TYPE`

```rust
pub const EPOLL_IOC_TYPE: u32 = 138;
```

#### Constant `POSIX_FADV_NORMAL`

```rust
pub const POSIX_FADV_NORMAL: u32 = 0;
```

#### Constant `POSIX_FADV_RANDOM`

```rust
pub const POSIX_FADV_RANDOM: u32 = 1;
```

#### Constant `POSIX_FADV_SEQUENTIAL`

```rust
pub const POSIX_FADV_SEQUENTIAL: u32 = 2;
```

#### Constant `POSIX_FADV_WILLNEED`

```rust
pub const POSIX_FADV_WILLNEED: u32 = 3;
```

#### Constant `POSIX_FADV_DONTNEED`

```rust
pub const POSIX_FADV_DONTNEED: u32 = 4;
```

#### Constant `POSIX_FADV_NOREUSE`

```rust
pub const POSIX_FADV_NOREUSE: u32 = 5;
```

#### Constant `FALLOC_FL_ALLOCATE_RANGE`

```rust
pub const FALLOC_FL_ALLOCATE_RANGE: u32 = 0;
```

#### Constant `FALLOC_FL_KEEP_SIZE`

```rust
pub const FALLOC_FL_KEEP_SIZE: u32 = 1;
```

#### Constant `FALLOC_FL_PUNCH_HOLE`

```rust
pub const FALLOC_FL_PUNCH_HOLE: u32 = 2;
```

#### Constant `FALLOC_FL_NO_HIDE_STALE`

```rust
pub const FALLOC_FL_NO_HIDE_STALE: u32 = 4;
```

#### Constant `FALLOC_FL_COLLAPSE_RANGE`

```rust
pub const FALLOC_FL_COLLAPSE_RANGE: u32 = 8;
```

#### Constant `FALLOC_FL_ZERO_RANGE`

```rust
pub const FALLOC_FL_ZERO_RANGE: u32 = 16;
```

#### Constant `FALLOC_FL_INSERT_RANGE`

```rust
pub const FALLOC_FL_INSERT_RANGE: u32 = 32;
```

#### Constant `FALLOC_FL_UNSHARE_RANGE`

```rust
pub const FALLOC_FL_UNSHARE_RANGE: u32 = 64;
```

#### Constant `NR_OPEN`

```rust
pub const NR_OPEN: u32 = 1024;
```

#### Constant `NGROUPS_MAX`

```rust
pub const NGROUPS_MAX: u32 = 65536;
```

#### Constant `ARG_MAX`

```rust
pub const ARG_MAX: u32 = 131072;
```

#### Constant `LINK_MAX`

```rust
pub const LINK_MAX: u32 = 127;
```

#### Constant `MAX_CANON`

```rust
pub const MAX_CANON: u32 = 255;
```

#### Constant `MAX_INPUT`

```rust
pub const MAX_INPUT: u32 = 255;
```

#### Constant `NAME_MAX`

```rust
pub const NAME_MAX: u32 = 255;
```

#### Constant `PATH_MAX`

```rust
pub const PATH_MAX: u32 = 4096;
```

#### Constant `PIPE_BUF`

```rust
pub const PIPE_BUF: u32 = 4096;
```

#### Constant `XATTR_NAME_MAX`

```rust
pub const XATTR_NAME_MAX: u32 = 255;
```

#### Constant `XATTR_SIZE_MAX`

```rust
pub const XATTR_SIZE_MAX: u32 = 65536;
```

#### Constant `XATTR_LIST_MAX`

```rust
pub const XATTR_LIST_MAX: u32 = 65536;
```

#### Constant `RTSIG_MAX`

```rust
pub const RTSIG_MAX: u32 = 32;
```

#### Constant `_IOC_NRBITS`

```rust
pub const _IOC_NRBITS: u32 = 8;
```

#### Constant `_IOC_TYPEBITS`

```rust
pub const _IOC_TYPEBITS: u32 = 8;
```

#### Constant `_IOC_SIZEBITS`

```rust
pub const _IOC_SIZEBITS: u32 = 14;
```

#### Constant `_IOC_DIRBITS`

```rust
pub const _IOC_DIRBITS: u32 = 2;
```

#### Constant `_IOC_NRMASK`

```rust
pub const _IOC_NRMASK: u32 = 255;
```

#### Constant `_IOC_TYPEMASK`

```rust
pub const _IOC_TYPEMASK: u32 = 255;
```

#### Constant `_IOC_SIZEMASK`

```rust
pub const _IOC_SIZEMASK: u32 = 16383;
```

#### Constant `_IOC_DIRMASK`

```rust
pub const _IOC_DIRMASK: u32 = 3;
```

#### Constant `_IOC_NRSHIFT`

```rust
pub const _IOC_NRSHIFT: u32 = 0;
```

#### Constant `_IOC_TYPESHIFT`

```rust
pub const _IOC_TYPESHIFT: u32 = 8;
```

#### Constant `_IOC_SIZESHIFT`

```rust
pub const _IOC_SIZESHIFT: u32 = 16;
```

#### Constant `_IOC_DIRSHIFT`

```rust
pub const _IOC_DIRSHIFT: u32 = 30;
```

#### Constant `_IOC_NONE`

```rust
pub const _IOC_NONE: u32 = 0;
```

#### Constant `_IOC_WRITE`

```rust
pub const _IOC_WRITE: u32 = 1;
```

#### Constant `_IOC_READ`

```rust
pub const _IOC_READ: u32 = 2;
```

#### Constant `IOC_IN`

```rust
pub const IOC_IN: u32 = 1073741824;
```

#### Constant `IOC_OUT`

```rust
pub const IOC_OUT: u32 = 2147483648;
```

#### Constant `IOC_INOUT`

```rust
pub const IOC_INOUT: u32 = 3221225472;
```

#### Constant `IOCSIZE_MASK`

```rust
pub const IOCSIZE_MASK: u32 = 1073676288;
```

#### Constant `IOCSIZE_SHIFT`

```rust
pub const IOCSIZE_SHIFT: u32 = 16;
```

#### Constant `FSCRYPT_POLICY_FLAGS_PAD_4`

```rust
pub const FSCRYPT_POLICY_FLAGS_PAD_4: u32 = 0;
```

#### Constant `FSCRYPT_POLICY_FLAGS_PAD_8`

```rust
pub const FSCRYPT_POLICY_FLAGS_PAD_8: u32 = 1;
```

#### Constant `FSCRYPT_POLICY_FLAGS_PAD_16`

```rust
pub const FSCRYPT_POLICY_FLAGS_PAD_16: u32 = 2;
```

#### Constant `FSCRYPT_POLICY_FLAGS_PAD_32`

```rust
pub const FSCRYPT_POLICY_FLAGS_PAD_32: u32 = 3;
```

#### Constant `FSCRYPT_POLICY_FLAGS_PAD_MASK`

```rust
pub const FSCRYPT_POLICY_FLAGS_PAD_MASK: u32 = 3;
```

#### Constant `FSCRYPT_POLICY_FLAG_DIRECT_KEY`

```rust
pub const FSCRYPT_POLICY_FLAG_DIRECT_KEY: u32 = 4;
```

#### Constant `FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64`

```rust
pub const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64: u32 = 8;
```

#### Constant `FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32`

```rust
pub const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32: u32 = 16;
```

#### Constant `FSCRYPT_MODE_AES_256_XTS`

```rust
pub const FSCRYPT_MODE_AES_256_XTS: u32 = 1;
```

#### Constant `FSCRYPT_MODE_AES_256_CTS`

```rust
pub const FSCRYPT_MODE_AES_256_CTS: u32 = 4;
```

#### Constant `FSCRYPT_MODE_AES_128_CBC`

```rust
pub const FSCRYPT_MODE_AES_128_CBC: u32 = 5;
```

#### Constant `FSCRYPT_MODE_AES_128_CTS`

```rust
pub const FSCRYPT_MODE_AES_128_CTS: u32 = 6;
```

#### Constant `FSCRYPT_MODE_SM4_XTS`

```rust
pub const FSCRYPT_MODE_SM4_XTS: u32 = 7;
```

#### Constant `FSCRYPT_MODE_SM4_CTS`

```rust
pub const FSCRYPT_MODE_SM4_CTS: u32 = 8;
```

#### Constant `FSCRYPT_MODE_ADIANTUM`

```rust
pub const FSCRYPT_MODE_ADIANTUM: u32 = 9;
```

#### Constant `FSCRYPT_MODE_AES_256_HCTR2`

```rust
pub const FSCRYPT_MODE_AES_256_HCTR2: u32 = 10;
```

#### Constant `FSCRYPT_POLICY_V1`

```rust
pub const FSCRYPT_POLICY_V1: u32 = 0;
```

#### Constant `FSCRYPT_KEY_DESCRIPTOR_SIZE`

```rust
pub const FSCRYPT_KEY_DESCRIPTOR_SIZE: u32 = 8;
```

#### Constant `FSCRYPT_KEY_DESC_PREFIX`

```rust
pub const FSCRYPT_KEY_DESC_PREFIX: &[u8; 9] = b"fscrypt:\0";
```

#### Constant `FSCRYPT_KEY_DESC_PREFIX_SIZE`

```rust
pub const FSCRYPT_KEY_DESC_PREFIX_SIZE: u32 = 8;
```

#### Constant `FSCRYPT_MAX_KEY_SIZE`

```rust
pub const FSCRYPT_MAX_KEY_SIZE: u32 = 64;
```

#### Constant `FSCRYPT_POLICY_V2`

```rust
pub const FSCRYPT_POLICY_V2: u32 = 2;
```

#### Constant `FSCRYPT_KEY_IDENTIFIER_SIZE`

```rust
pub const FSCRYPT_KEY_IDENTIFIER_SIZE: u32 = 16;
```

#### Constant `FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR`

```rust
pub const FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR: u32 = 1;
```

#### Constant `FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER`

```rust
pub const FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER: u32 = 2;
```

#### Constant `FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY`

```rust
pub const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY: u32 = 1;
```

#### Constant `FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS`

```rust
pub const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS: u32 = 2;
```

#### Constant `FSCRYPT_KEY_STATUS_ABSENT`

```rust
pub const FSCRYPT_KEY_STATUS_ABSENT: u32 = 1;
```

#### Constant `FSCRYPT_KEY_STATUS_PRESENT`

```rust
pub const FSCRYPT_KEY_STATUS_PRESENT: u32 = 2;
```

#### Constant `FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED`

```rust
pub const FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED: u32 = 3;
```

#### Constant `FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF`

```rust
pub const FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF: u32 = 1;
```

#### Constant `FS_KEY_DESCRIPTOR_SIZE`

```rust
pub const FS_KEY_DESCRIPTOR_SIZE: u32 = 8;
```

#### Constant `FS_POLICY_FLAGS_PAD_4`

```rust
pub const FS_POLICY_FLAGS_PAD_4: u32 = 0;
```

#### Constant `FS_POLICY_FLAGS_PAD_8`

```rust
pub const FS_POLICY_FLAGS_PAD_8: u32 = 1;
```

#### Constant `FS_POLICY_FLAGS_PAD_16`

```rust
pub const FS_POLICY_FLAGS_PAD_16: u32 = 2;
```

#### Constant `FS_POLICY_FLAGS_PAD_32`

```rust
pub const FS_POLICY_FLAGS_PAD_32: u32 = 3;
```

#### Constant `FS_POLICY_FLAGS_PAD_MASK`

```rust
pub const FS_POLICY_FLAGS_PAD_MASK: u32 = 3;
```

#### Constant `FS_POLICY_FLAG_DIRECT_KEY`

```rust
pub const FS_POLICY_FLAG_DIRECT_KEY: u32 = 4;
```

#### Constant `FS_POLICY_FLAGS_VALID`

```rust
pub const FS_POLICY_FLAGS_VALID: u32 = 7;
```

#### Constant `FS_ENCRYPTION_MODE_INVALID`

```rust
pub const FS_ENCRYPTION_MODE_INVALID: u32 = 0;
```

#### Constant `FS_ENCRYPTION_MODE_AES_256_XTS`

```rust
pub const FS_ENCRYPTION_MODE_AES_256_XTS: u32 = 1;
```

#### Constant `FS_ENCRYPTION_MODE_AES_256_GCM`

```rust
pub const FS_ENCRYPTION_MODE_AES_256_GCM: u32 = 2;
```

#### Constant `FS_ENCRYPTION_MODE_AES_256_CBC`

```rust
pub const FS_ENCRYPTION_MODE_AES_256_CBC: u32 = 3;
```

#### Constant `FS_ENCRYPTION_MODE_AES_256_CTS`

```rust
pub const FS_ENCRYPTION_MODE_AES_256_CTS: u32 = 4;
```

#### Constant `FS_ENCRYPTION_MODE_AES_128_CBC`

```rust
pub const FS_ENCRYPTION_MODE_AES_128_CBC: u32 = 5;
```

#### Constant `FS_ENCRYPTION_MODE_AES_128_CTS`

```rust
pub const FS_ENCRYPTION_MODE_AES_128_CTS: u32 = 6;
```

#### Constant `FS_ENCRYPTION_MODE_ADIANTUM`

```rust
pub const FS_ENCRYPTION_MODE_ADIANTUM: u32 = 9;
```

#### Constant `FS_KEY_DESC_PREFIX`

```rust
pub const FS_KEY_DESC_PREFIX: &[u8; 9] = b"fscrypt:\0";
```

#### Constant `FS_KEY_DESC_PREFIX_SIZE`

```rust
pub const FS_KEY_DESC_PREFIX_SIZE: u32 = 8;
```

#### Constant `FS_MAX_KEY_SIZE`

```rust
pub const FS_MAX_KEY_SIZE: u32 = 64;
```

#### Constant `MS_RDONLY`

```rust
pub const MS_RDONLY: u32 = 1;
```

#### Constant `MS_NOSUID`

```rust
pub const MS_NOSUID: u32 = 2;
```

#### Constant `MS_NODEV`

```rust
pub const MS_NODEV: u32 = 4;
```

#### Constant `MS_NOEXEC`

```rust
pub const MS_NOEXEC: u32 = 8;
```

#### Constant `MS_SYNCHRONOUS`

```rust
pub const MS_SYNCHRONOUS: u32 = 16;
```

#### Constant `MS_REMOUNT`

```rust
pub const MS_REMOUNT: u32 = 32;
```

#### Constant `MS_MANDLOCK`

```rust
pub const MS_MANDLOCK: u32 = 64;
```

#### Constant `MS_DIRSYNC`

```rust
pub const MS_DIRSYNC: u32 = 128;
```

#### Constant `MS_NOSYMFOLLOW`

```rust
pub const MS_NOSYMFOLLOW: u32 = 256;
```

#### Constant `MS_NOATIME`

```rust
pub const MS_NOATIME: u32 = 1024;
```

#### Constant `MS_NODIRATIME`

```rust
pub const MS_NODIRATIME: u32 = 2048;
```

#### Constant `MS_BIND`

```rust
pub const MS_BIND: u32 = 4096;
```

#### Constant `MS_MOVE`

```rust
pub const MS_MOVE: u32 = 8192;
```

#### Constant `MS_REC`

```rust
pub const MS_REC: u32 = 16384;
```

#### Constant `MS_VERBOSE`

```rust
pub const MS_VERBOSE: u32 = 32768;
```

#### Constant `MS_SILENT`

```rust
pub const MS_SILENT: u32 = 32768;
```

#### Constant `MS_POSIXACL`

```rust
pub const MS_POSIXACL: u32 = 65536;
```

#### Constant `MS_UNBINDABLE`

```rust
pub const MS_UNBINDABLE: u32 = 131072;
```

#### Constant `MS_PRIVATE`

```rust
pub const MS_PRIVATE: u32 = 262144;
```

#### Constant `MS_SLAVE`

```rust
pub const MS_SLAVE: u32 = 524288;
```

#### Constant `MS_SHARED`

```rust
pub const MS_SHARED: u32 = 1048576;
```

#### Constant `MS_RELATIME`

```rust
pub const MS_RELATIME: u32 = 2097152;
```

#### Constant `MS_KERNMOUNT`

```rust
pub const MS_KERNMOUNT: u32 = 4194304;
```

#### Constant `MS_I_VERSION`

```rust
pub const MS_I_VERSION: u32 = 8388608;
```

#### Constant `MS_STRICTATIME`

```rust
pub const MS_STRICTATIME: u32 = 16777216;
```

#### Constant `MS_LAZYTIME`

```rust
pub const MS_LAZYTIME: u32 = 33554432;
```

#### Constant `MS_SUBMOUNT`

```rust
pub const MS_SUBMOUNT: u32 = 67108864;
```

#### Constant `MS_NOREMOTELOCK`

```rust
pub const MS_NOREMOTELOCK: u32 = 134217728;
```

#### Constant `MS_NOSEC`

```rust
pub const MS_NOSEC: u32 = 268435456;
```

#### Constant `MS_BORN`

```rust
pub const MS_BORN: u32 = 536870912;
```

#### Constant `MS_ACTIVE`

```rust
pub const MS_ACTIVE: u32 = 1073741824;
```

#### Constant `MS_NOUSER`

```rust
pub const MS_NOUSER: u32 = 2147483648;
```

#### Constant `MS_RMT_MASK`

```rust
pub const MS_RMT_MASK: u32 = 41943121;
```

#### Constant `MS_MGC_VAL`

```rust
pub const MS_MGC_VAL: u32 = 3236757504;
```

#### Constant `MS_MGC_MSK`

```rust
pub const MS_MGC_MSK: u32 = 4294901760;
```

#### Constant `OPEN_TREE_CLONE`

```rust
pub const OPEN_TREE_CLONE: u32 = 1;
```

#### Constant `OPEN_TREE_CLOEXEC`

```rust
pub const OPEN_TREE_CLOEXEC: u32 = 524288;
```

#### Constant `MOVE_MOUNT_F_SYMLINKS`

```rust
pub const MOVE_MOUNT_F_SYMLINKS: u32 = 1;
```

#### Constant `MOVE_MOUNT_F_AUTOMOUNTS`

```rust
pub const MOVE_MOUNT_F_AUTOMOUNTS: u32 = 2;
```

#### Constant `MOVE_MOUNT_F_EMPTY_PATH`

```rust
pub const MOVE_MOUNT_F_EMPTY_PATH: u32 = 4;
```

#### Constant `MOVE_MOUNT_T_SYMLINKS`

```rust
pub const MOVE_MOUNT_T_SYMLINKS: u32 = 16;
```

#### Constant `MOVE_MOUNT_T_AUTOMOUNTS`

```rust
pub const MOVE_MOUNT_T_AUTOMOUNTS: u32 = 32;
```

#### Constant `MOVE_MOUNT_T_EMPTY_PATH`

```rust
pub const MOVE_MOUNT_T_EMPTY_PATH: u32 = 64;
```

#### Constant `MOVE_MOUNT_SET_GROUP`

```rust
pub const MOVE_MOUNT_SET_GROUP: u32 = 256;
```

#### Constant `MOVE_MOUNT_BENEATH`

```rust
pub const MOVE_MOUNT_BENEATH: u32 = 512;
```

#### Constant `MOVE_MOUNT__MASK`

```rust
pub const MOVE_MOUNT__MASK: u32 = 887;
```

#### Constant `FSOPEN_CLOEXEC`

```rust
pub const FSOPEN_CLOEXEC: u32 = 1;
```

#### Constant `FSPICK_CLOEXEC`

```rust
pub const FSPICK_CLOEXEC: u32 = 1;
```

#### Constant `FSPICK_SYMLINK_NOFOLLOW`

```rust
pub const FSPICK_SYMLINK_NOFOLLOW: u32 = 2;
```

#### Constant `FSPICK_NO_AUTOMOUNT`

```rust
pub const FSPICK_NO_AUTOMOUNT: u32 = 4;
```

#### Constant `FSPICK_EMPTY_PATH`

```rust
pub const FSPICK_EMPTY_PATH: u32 = 8;
```

#### Constant `FSMOUNT_CLOEXEC`

```rust
pub const FSMOUNT_CLOEXEC: u32 = 1;
```

#### Constant `MOUNT_ATTR_RDONLY`

```rust
pub const MOUNT_ATTR_RDONLY: u32 = 1;
```

#### Constant `MOUNT_ATTR_NOSUID`

```rust
pub const MOUNT_ATTR_NOSUID: u32 = 2;
```

#### Constant `MOUNT_ATTR_NODEV`

```rust
pub const MOUNT_ATTR_NODEV: u32 = 4;
```

#### Constant `MOUNT_ATTR_NOEXEC`

```rust
pub const MOUNT_ATTR_NOEXEC: u32 = 8;
```

#### Constant `MOUNT_ATTR__ATIME`

```rust
pub const MOUNT_ATTR__ATIME: u32 = 112;
```

#### Constant `MOUNT_ATTR_RELATIME`

```rust
pub const MOUNT_ATTR_RELATIME: u32 = 0;
```

#### Constant `MOUNT_ATTR_NOATIME`

```rust
pub const MOUNT_ATTR_NOATIME: u32 = 16;
```

#### Constant `MOUNT_ATTR_STRICTATIME`

```rust
pub const MOUNT_ATTR_STRICTATIME: u32 = 32;
```

#### Constant `MOUNT_ATTR_NODIRATIME`

```rust
pub const MOUNT_ATTR_NODIRATIME: u32 = 128;
```

#### Constant `MOUNT_ATTR_IDMAP`

```rust
pub const MOUNT_ATTR_IDMAP: u32 = 1048576;
```

#### Constant `MOUNT_ATTR_NOSYMFOLLOW`

```rust
pub const MOUNT_ATTR_NOSYMFOLLOW: u32 = 2097152;
```

#### Constant `MOUNT_ATTR_SIZE_VER0`

```rust
pub const MOUNT_ATTR_SIZE_VER0: u32 = 32;
```

#### Constant `MNT_ID_REQ_SIZE_VER0`

```rust
pub const MNT_ID_REQ_SIZE_VER0: u32 = 24;
```

#### Constant `MNT_ID_REQ_SIZE_VER1`

```rust
pub const MNT_ID_REQ_SIZE_VER1: u32 = 32;
```

#### Constant `STATMOUNT_SB_BASIC`

```rust
pub const STATMOUNT_SB_BASIC: u32 = 1;
```

#### Constant `STATMOUNT_MNT_BASIC`

```rust
pub const STATMOUNT_MNT_BASIC: u32 = 2;
```

#### Constant `STATMOUNT_PROPAGATE_FROM`

```rust
pub const STATMOUNT_PROPAGATE_FROM: u32 = 4;
```

#### Constant `STATMOUNT_MNT_ROOT`

```rust
pub const STATMOUNT_MNT_ROOT: u32 = 8;
```

#### Constant `STATMOUNT_MNT_POINT`

```rust
pub const STATMOUNT_MNT_POINT: u32 = 16;
```

#### Constant `STATMOUNT_FS_TYPE`

```rust
pub const STATMOUNT_FS_TYPE: u32 = 32;
```

#### Constant `STATMOUNT_MNT_NS_ID`

```rust
pub const STATMOUNT_MNT_NS_ID: u32 = 64;
```

#### Constant `STATMOUNT_MNT_OPTS`

```rust
pub const STATMOUNT_MNT_OPTS: u32 = 128;
```

#### Constant `STATMOUNT_FS_SUBTYPE`

```rust
pub const STATMOUNT_FS_SUBTYPE: u32 = 256;
```

#### Constant `STATMOUNT_SB_SOURCE`

```rust
pub const STATMOUNT_SB_SOURCE: u32 = 512;
```

#### Constant `STATMOUNT_OPT_ARRAY`

```rust
pub const STATMOUNT_OPT_ARRAY: u32 = 1024;
```

#### Constant `STATMOUNT_OPT_SEC_ARRAY`

```rust
pub const STATMOUNT_OPT_SEC_ARRAY: u32 = 2048;
```

#### Constant `LSMT_ROOT`

```rust
pub const LSMT_ROOT: i32 = -1;
```

#### Constant `LISTMOUNT_REVERSE`

```rust
pub const LISTMOUNT_REVERSE: u32 = 1;
```

#### Constant `INR_OPEN_CUR`

```rust
pub const INR_OPEN_CUR: u32 = 1024;
```

#### Constant `INR_OPEN_MAX`

```rust
pub const INR_OPEN_MAX: u32 = 4096;
```

#### Constant `BLOCK_SIZE_BITS`

```rust
pub const BLOCK_SIZE_BITS: u32 = 10;
```

#### Constant `BLOCK_SIZE`

```rust
pub const BLOCK_SIZE: u32 = 1024;
```

#### Constant `SEEK_SET`

```rust
pub const SEEK_SET: u32 = 0;
```

#### Constant `SEEK_CUR`

```rust
pub const SEEK_CUR: u32 = 1;
```

#### Constant `SEEK_END`

```rust
pub const SEEK_END: u32 = 2;
```

#### Constant `SEEK_DATA`

```rust
pub const SEEK_DATA: u32 = 3;
```

#### Constant `SEEK_HOLE`

```rust
pub const SEEK_HOLE: u32 = 4;
```

#### Constant `SEEK_MAX`

```rust
pub const SEEK_MAX: u32 = 4;
```

#### Constant `RENAME_NOREPLACE`

```rust
pub const RENAME_NOREPLACE: u32 = 1;
```

#### Constant `RENAME_EXCHANGE`

```rust
pub const RENAME_EXCHANGE: u32 = 2;
```

#### Constant `RENAME_WHITEOUT`

```rust
pub const RENAME_WHITEOUT: u32 = 4;
```

#### Constant `FILE_DEDUPE_RANGE_SAME`

```rust
pub const FILE_DEDUPE_RANGE_SAME: u32 = 0;
```

#### Constant `FILE_DEDUPE_RANGE_DIFFERS`

```rust
pub const FILE_DEDUPE_RANGE_DIFFERS: u32 = 1;
```

#### Constant `NR_FILE`

```rust
pub const NR_FILE: u32 = 8192;
```

#### Constant `FS_XFLAG_REALTIME`

```rust
pub const FS_XFLAG_REALTIME: u32 = 1;
```

#### Constant `FS_XFLAG_PREALLOC`

```rust
pub const FS_XFLAG_PREALLOC: u32 = 2;
```

#### Constant `FS_XFLAG_IMMUTABLE`

```rust
pub const FS_XFLAG_IMMUTABLE: u32 = 8;
```

#### Constant `FS_XFLAG_APPEND`

```rust
pub const FS_XFLAG_APPEND: u32 = 16;
```

#### Constant `FS_XFLAG_SYNC`

```rust
pub const FS_XFLAG_SYNC: u32 = 32;
```

#### Constant `FS_XFLAG_NOATIME`

```rust
pub const FS_XFLAG_NOATIME: u32 = 64;
```

#### Constant `FS_XFLAG_NODUMP`

```rust
pub const FS_XFLAG_NODUMP: u32 = 128;
```

#### Constant `FS_XFLAG_RTINHERIT`

```rust
pub const FS_XFLAG_RTINHERIT: u32 = 256;
```

#### Constant `FS_XFLAG_PROJINHERIT`

```rust
pub const FS_XFLAG_PROJINHERIT: u32 = 512;
```

#### Constant `FS_XFLAG_NOSYMLINKS`

```rust
pub const FS_XFLAG_NOSYMLINKS: u32 = 1024;
```

#### Constant `FS_XFLAG_EXTSIZE`

```rust
pub const FS_XFLAG_EXTSIZE: u32 = 2048;
```

#### Constant `FS_XFLAG_EXTSZINHERIT`

```rust
pub const FS_XFLAG_EXTSZINHERIT: u32 = 4096;
```

#### Constant `FS_XFLAG_NODEFRAG`

```rust
pub const FS_XFLAG_NODEFRAG: u32 = 8192;
```

#### Constant `FS_XFLAG_FILESTREAM`

```rust
pub const FS_XFLAG_FILESTREAM: u32 = 16384;
```

#### Constant `FS_XFLAG_DAX`

```rust
pub const FS_XFLAG_DAX: u32 = 32768;
```

#### Constant `FS_XFLAG_COWEXTSIZE`

```rust
pub const FS_XFLAG_COWEXTSIZE: u32 = 65536;
```

#### Constant `FS_XFLAG_HASATTR`

```rust
pub const FS_XFLAG_HASATTR: u32 = 2147483648;
```

#### Constant `BMAP_IOCTL`

```rust
pub const BMAP_IOCTL: u32 = 1;
```

#### Constant `FSLABEL_MAX`

```rust
pub const FSLABEL_MAX: u32 = 256;
```

#### Constant `FS_SECRM_FL`

```rust
pub const FS_SECRM_FL: u32 = 1;
```

#### Constant `FS_UNRM_FL`

```rust
pub const FS_UNRM_FL: u32 = 2;
```

#### Constant `FS_COMPR_FL`

```rust
pub const FS_COMPR_FL: u32 = 4;
```

#### Constant `FS_SYNC_FL`

```rust
pub const FS_SYNC_FL: u32 = 8;
```

#### Constant `FS_IMMUTABLE_FL`

```rust
pub const FS_IMMUTABLE_FL: u32 = 16;
```

#### Constant `FS_APPEND_FL`

```rust
pub const FS_APPEND_FL: u32 = 32;
```

#### Constant `FS_NODUMP_FL`

```rust
pub const FS_NODUMP_FL: u32 = 64;
```

#### Constant `FS_NOATIME_FL`

```rust
pub const FS_NOATIME_FL: u32 = 128;
```

#### Constant `FS_DIRTY_FL`

```rust
pub const FS_DIRTY_FL: u32 = 256;
```

#### Constant `FS_COMPRBLK_FL`

```rust
pub const FS_COMPRBLK_FL: u32 = 512;
```

#### Constant `FS_NOCOMP_FL`

```rust
pub const FS_NOCOMP_FL: u32 = 1024;
```

#### Constant `FS_ENCRYPT_FL`

```rust
pub const FS_ENCRYPT_FL: u32 = 2048;
```

#### Constant `FS_BTREE_FL`

```rust
pub const FS_BTREE_FL: u32 = 4096;
```

#### Constant `FS_INDEX_FL`

```rust
pub const FS_INDEX_FL: u32 = 4096;
```

#### Constant `FS_IMAGIC_FL`

```rust
pub const FS_IMAGIC_FL: u32 = 8192;
```

#### Constant `FS_JOURNAL_DATA_FL`

```rust
pub const FS_JOURNAL_DATA_FL: u32 = 16384;
```

#### Constant `FS_NOTAIL_FL`

```rust
pub const FS_NOTAIL_FL: u32 = 32768;
```

#### Constant `FS_DIRSYNC_FL`

```rust
pub const FS_DIRSYNC_FL: u32 = 65536;
```

#### Constant `FS_TOPDIR_FL`

```rust
pub const FS_TOPDIR_FL: u32 = 131072;
```

#### Constant `FS_HUGE_FILE_FL`

```rust
pub const FS_HUGE_FILE_FL: u32 = 262144;
```

#### Constant `FS_EXTENT_FL`

```rust
pub const FS_EXTENT_FL: u32 = 524288;
```

#### Constant `FS_VERITY_FL`

```rust
pub const FS_VERITY_FL: u32 = 1048576;
```

#### Constant `FS_EA_INODE_FL`

```rust
pub const FS_EA_INODE_FL: u32 = 2097152;
```

#### Constant `FS_EOFBLOCKS_FL`

```rust
pub const FS_EOFBLOCKS_FL: u32 = 4194304;
```

#### Constant `FS_NOCOW_FL`

```rust
pub const FS_NOCOW_FL: u32 = 8388608;
```

#### Constant `FS_DAX_FL`

```rust
pub const FS_DAX_FL: u32 = 33554432;
```

#### Constant `FS_INLINE_DATA_FL`

```rust
pub const FS_INLINE_DATA_FL: u32 = 268435456;
```

#### Constant `FS_PROJINHERIT_FL`

```rust
pub const FS_PROJINHERIT_FL: u32 = 536870912;
```

#### Constant `FS_CASEFOLD_FL`

```rust
pub const FS_CASEFOLD_FL: u32 = 1073741824;
```

#### Constant `FS_RESERVED_FL`

```rust
pub const FS_RESERVED_FL: u32 = 2147483648;
```

#### Constant `FS_FL_USER_VISIBLE`

```rust
pub const FS_FL_USER_VISIBLE: u32 = 253951;
```

#### Constant `FS_FL_USER_MODIFIABLE`

```rust
pub const FS_FL_USER_MODIFIABLE: u32 = 229631;
```

#### Constant `SYNC_FILE_RANGE_WAIT_BEFORE`

```rust
pub const SYNC_FILE_RANGE_WAIT_BEFORE: u32 = 1;
```

#### Constant `SYNC_FILE_RANGE_WRITE`

```rust
pub const SYNC_FILE_RANGE_WRITE: u32 = 2;
```

#### Constant `SYNC_FILE_RANGE_WAIT_AFTER`

```rust
pub const SYNC_FILE_RANGE_WAIT_AFTER: u32 = 4;
```

#### Constant `SYNC_FILE_RANGE_WRITE_AND_WAIT`

```rust
pub const SYNC_FILE_RANGE_WRITE_AND_WAIT: u32 = 7;
```

#### Constant `PROCFS_IOCTL_MAGIC`

```rust
pub const PROCFS_IOCTL_MAGIC: u8 = 102u8;
```

#### Constant `PAGE_IS_WPALLOWED`

```rust
pub const PAGE_IS_WPALLOWED: u32 = 1;
```

#### Constant `PAGE_IS_WRITTEN`

```rust
pub const PAGE_IS_WRITTEN: u32 = 2;
```

#### Constant `PAGE_IS_FILE`

```rust
pub const PAGE_IS_FILE: u32 = 4;
```

#### Constant `PAGE_IS_PRESENT`

```rust
pub const PAGE_IS_PRESENT: u32 = 8;
```

#### Constant `PAGE_IS_SWAPPED`

```rust
pub const PAGE_IS_SWAPPED: u32 = 16;
```

#### Constant `PAGE_IS_PFNZERO`

```rust
pub const PAGE_IS_PFNZERO: u32 = 32;
```

#### Constant `PAGE_IS_HUGE`

```rust
pub const PAGE_IS_HUGE: u32 = 64;
```

#### Constant `PAGE_IS_SOFT_DIRTY`

```rust
pub const PAGE_IS_SOFT_DIRTY: u32 = 128;
```

#### Constant `PM_SCAN_WP_MATCHING`

```rust
pub const PM_SCAN_WP_MATCHING: u32 = 1;
```

#### Constant `PM_SCAN_CHECK_WPASYNC`

```rust
pub const PM_SCAN_CHECK_WPASYNC: u32 = 2;
```

#### Constant `FUTEX_WAIT`

```rust
pub const FUTEX_WAIT: u32 = 0;
```

#### Constant `FUTEX_WAKE`

```rust
pub const FUTEX_WAKE: u32 = 1;
```

#### Constant `FUTEX_FD`

```rust
pub const FUTEX_FD: u32 = 2;
```

#### Constant `FUTEX_REQUEUE`

```rust
pub const FUTEX_REQUEUE: u32 = 3;
```

#### Constant `FUTEX_CMP_REQUEUE`

```rust
pub const FUTEX_CMP_REQUEUE: u32 = 4;
```

#### Constant `FUTEX_WAKE_OP`

```rust
pub const FUTEX_WAKE_OP: u32 = 5;
```

#### Constant `FUTEX_LOCK_PI`

```rust
pub const FUTEX_LOCK_PI: u32 = 6;
```

#### Constant `FUTEX_UNLOCK_PI`

```rust
pub const FUTEX_UNLOCK_PI: u32 = 7;
```

#### Constant `FUTEX_TRYLOCK_PI`

```rust
pub const FUTEX_TRYLOCK_PI: u32 = 8;
```

#### Constant `FUTEX_WAIT_BITSET`

```rust
pub const FUTEX_WAIT_BITSET: u32 = 9;
```

#### Constant `FUTEX_WAKE_BITSET`

```rust
pub const FUTEX_WAKE_BITSET: u32 = 10;
```

#### Constant `FUTEX_WAIT_REQUEUE_PI`

```rust
pub const FUTEX_WAIT_REQUEUE_PI: u32 = 11;
```

#### Constant `FUTEX_CMP_REQUEUE_PI`

```rust
pub const FUTEX_CMP_REQUEUE_PI: u32 = 12;
```

#### Constant `FUTEX_LOCK_PI2`

```rust
pub const FUTEX_LOCK_PI2: u32 = 13;
```

#### Constant `FUTEX_PRIVATE_FLAG`

```rust
pub const FUTEX_PRIVATE_FLAG: u32 = 128;
```

#### Constant `FUTEX_CLOCK_REALTIME`

```rust
pub const FUTEX_CLOCK_REALTIME: u32 = 256;
```

#### Constant `FUTEX_CMD_MASK`

```rust
pub const FUTEX_CMD_MASK: i32 = -385;
```

#### Constant `FUTEX_WAIT_PRIVATE`

```rust
pub const FUTEX_WAIT_PRIVATE: u32 = 128;
```

#### Constant `FUTEX_WAKE_PRIVATE`

```rust
pub const FUTEX_WAKE_PRIVATE: u32 = 129;
```

#### Constant `FUTEX_REQUEUE_PRIVATE`

```rust
pub const FUTEX_REQUEUE_PRIVATE: u32 = 131;
```

#### Constant `FUTEX_CMP_REQUEUE_PRIVATE`

```rust
pub const FUTEX_CMP_REQUEUE_PRIVATE: u32 = 132;
```

#### Constant `FUTEX_WAKE_OP_PRIVATE`

```rust
pub const FUTEX_WAKE_OP_PRIVATE: u32 = 133;
```

#### Constant `FUTEX_LOCK_PI_PRIVATE`

```rust
pub const FUTEX_LOCK_PI_PRIVATE: u32 = 134;
```

#### Constant `FUTEX_LOCK_PI2_PRIVATE`

```rust
pub const FUTEX_LOCK_PI2_PRIVATE: u32 = 141;
```

#### Constant `FUTEX_UNLOCK_PI_PRIVATE`

```rust
pub const FUTEX_UNLOCK_PI_PRIVATE: u32 = 135;
```

#### Constant `FUTEX_TRYLOCK_PI_PRIVATE`

```rust
pub const FUTEX_TRYLOCK_PI_PRIVATE: u32 = 136;
```

#### Constant `FUTEX_WAIT_BITSET_PRIVATE`

```rust
pub const FUTEX_WAIT_BITSET_PRIVATE: u32 = 137;
```

#### Constant `FUTEX_WAKE_BITSET_PRIVATE`

```rust
pub const FUTEX_WAKE_BITSET_PRIVATE: u32 = 138;
```

#### Constant `FUTEX_WAIT_REQUEUE_PI_PRIVATE`

```rust
pub const FUTEX_WAIT_REQUEUE_PI_PRIVATE: u32 = 139;
```

#### Constant `FUTEX_CMP_REQUEUE_PI_PRIVATE`

```rust
pub const FUTEX_CMP_REQUEUE_PI_PRIVATE: u32 = 140;
```

#### Constant `FUTEX2_SIZE_U8`

```rust
pub const FUTEX2_SIZE_U8: u32 = 0;
```

#### Constant `FUTEX2_SIZE_U16`

```rust
pub const FUTEX2_SIZE_U16: u32 = 1;
```

#### Constant `FUTEX2_SIZE_U32`

```rust
pub const FUTEX2_SIZE_U32: u32 = 2;
```

#### Constant `FUTEX2_SIZE_U64`

```rust
pub const FUTEX2_SIZE_U64: u32 = 3;
```

#### Constant `FUTEX2_NUMA`

```rust
pub const FUTEX2_NUMA: u32 = 4;
```

#### Constant `FUTEX2_PRIVATE`

```rust
pub const FUTEX2_PRIVATE: u32 = 128;
```

#### Constant `FUTEX2_SIZE_MASK`

```rust
pub const FUTEX2_SIZE_MASK: u32 = 3;
```

#### Constant `FUTEX_32`

```rust
pub const FUTEX_32: u32 = 2;
```

#### Constant `FUTEX_WAITV_MAX`

```rust
pub const FUTEX_WAITV_MAX: u32 = 128;
```

#### Constant `FUTEX_WAITERS`

```rust
pub const FUTEX_WAITERS: u32 = 2147483648;
```

#### Constant `FUTEX_OWNER_DIED`

```rust
pub const FUTEX_OWNER_DIED: u32 = 1073741824;
```

#### Constant `FUTEX_TID_MASK`

```rust
pub const FUTEX_TID_MASK: u32 = 1073741823;
```

#### Constant `ROBUST_LIST_LIMIT`

```rust
pub const ROBUST_LIST_LIMIT: u32 = 2048;
```

#### Constant `FUTEX_BITSET_MATCH_ANY`

```rust
pub const FUTEX_BITSET_MATCH_ANY: u32 = 4294967295;
```

#### Constant `FUTEX_OP_SET`

```rust
pub const FUTEX_OP_SET: u32 = 0;
```

#### Constant `FUTEX_OP_ADD`

```rust
pub const FUTEX_OP_ADD: u32 = 1;
```

#### Constant `FUTEX_OP_OR`

```rust
pub const FUTEX_OP_OR: u32 = 2;
```

#### Constant `FUTEX_OP_ANDN`

```rust
pub const FUTEX_OP_ANDN: u32 = 3;
```

#### Constant `FUTEX_OP_XOR`

```rust
pub const FUTEX_OP_XOR: u32 = 4;
```

#### Constant `FUTEX_OP_OPARG_SHIFT`

```rust
pub const FUTEX_OP_OPARG_SHIFT: u32 = 8;
```

#### Constant `FUTEX_OP_CMP_EQ`

```rust
pub const FUTEX_OP_CMP_EQ: u32 = 0;
```

#### Constant `FUTEX_OP_CMP_NE`

```rust
pub const FUTEX_OP_CMP_NE: u32 = 1;
```

#### Constant `FUTEX_OP_CMP_LT`

```rust
pub const FUTEX_OP_CMP_LT: u32 = 2;
```

#### Constant `FUTEX_OP_CMP_LE`

```rust
pub const FUTEX_OP_CMP_LE: u32 = 3;
```

#### Constant `FUTEX_OP_CMP_GT`

```rust
pub const FUTEX_OP_CMP_GT: u32 = 4;
```

#### Constant `FUTEX_OP_CMP_GE`

```rust
pub const FUTEX_OP_CMP_GE: u32 = 5;
```

#### Constant `IN_ACCESS`

```rust
pub const IN_ACCESS: u32 = 1;
```

#### Constant `IN_MODIFY`

```rust
pub const IN_MODIFY: u32 = 2;
```

#### Constant `IN_ATTRIB`

```rust
pub const IN_ATTRIB: u32 = 4;
```

#### Constant `IN_CLOSE_WRITE`

```rust
pub const IN_CLOSE_WRITE: u32 = 8;
```

#### Constant `IN_CLOSE_NOWRITE`

```rust
pub const IN_CLOSE_NOWRITE: u32 = 16;
```

#### Constant `IN_OPEN`

```rust
pub const IN_OPEN: u32 = 32;
```

#### Constant `IN_MOVED_FROM`

```rust
pub const IN_MOVED_FROM: u32 = 64;
```

#### Constant `IN_MOVED_TO`

```rust
pub const IN_MOVED_TO: u32 = 128;
```

#### Constant `IN_CREATE`

```rust
pub const IN_CREATE: u32 = 256;
```

#### Constant `IN_DELETE`

```rust
pub const IN_DELETE: u32 = 512;
```

#### Constant `IN_DELETE_SELF`

```rust
pub const IN_DELETE_SELF: u32 = 1024;
```

#### Constant `IN_MOVE_SELF`

```rust
pub const IN_MOVE_SELF: u32 = 2048;
```

#### Constant `IN_UNMOUNT`

```rust
pub const IN_UNMOUNT: u32 = 8192;
```

#### Constant `IN_Q_OVERFLOW`

```rust
pub const IN_Q_OVERFLOW: u32 = 16384;
```

#### Constant `IN_IGNORED`

```rust
pub const IN_IGNORED: u32 = 32768;
```

#### Constant `IN_CLOSE`

```rust
pub const IN_CLOSE: u32 = 24;
```

#### Constant `IN_MOVE`

```rust
pub const IN_MOVE: u32 = 192;
```

#### Constant `IN_ONLYDIR`

```rust
pub const IN_ONLYDIR: u32 = 16777216;
```

#### Constant `IN_DONT_FOLLOW`

```rust
pub const IN_DONT_FOLLOW: u32 = 33554432;
```

#### Constant `IN_EXCL_UNLINK`

```rust
pub const IN_EXCL_UNLINK: u32 = 67108864;
```

#### Constant `IN_MASK_CREATE`

```rust
pub const IN_MASK_CREATE: u32 = 268435456;
```

#### Constant `IN_MASK_ADD`

```rust
pub const IN_MASK_ADD: u32 = 536870912;
```

#### Constant `IN_ISDIR`

```rust
pub const IN_ISDIR: u32 = 1073741824;
```

#### Constant `IN_ONESHOT`

```rust
pub const IN_ONESHOT: u32 = 2147483648;
```

#### Constant `IN_ALL_EVENTS`

```rust
pub const IN_ALL_EVENTS: u32 = 4095;
```

#### Constant `IN_CLOEXEC`

```rust
pub const IN_CLOEXEC: u32 = 524288;
```

#### Constant `IN_NONBLOCK`

```rust
pub const IN_NONBLOCK: u32 = 2048;
```

#### Constant `ADFS_SUPER_MAGIC`

```rust
pub const ADFS_SUPER_MAGIC: u32 = 44533;
```

#### Constant `AFFS_SUPER_MAGIC`

```rust
pub const AFFS_SUPER_MAGIC: u32 = 44543;
```

#### Constant `AFS_SUPER_MAGIC`

```rust
pub const AFS_SUPER_MAGIC: u32 = 1397113167;
```

#### Constant `AUTOFS_SUPER_MAGIC`

```rust
pub const AUTOFS_SUPER_MAGIC: u32 = 391;
```

#### Constant `CEPH_SUPER_MAGIC`

```rust
pub const CEPH_SUPER_MAGIC: u32 = 12805120;
```

#### Constant `CODA_SUPER_MAGIC`

```rust
pub const CODA_SUPER_MAGIC: u32 = 1937076805;
```

#### Constant `CRAMFS_MAGIC`

```rust
pub const CRAMFS_MAGIC: u32 = 684539205;
```

#### Constant `CRAMFS_MAGIC_WEND`

```rust
pub const CRAMFS_MAGIC_WEND: u32 = 1161678120;
```

#### Constant `DEBUGFS_MAGIC`

```rust
pub const DEBUGFS_MAGIC: u32 = 1684170528;
```

#### Constant `SECURITYFS_MAGIC`

```rust
pub const SECURITYFS_MAGIC: u32 = 1935894131;
```

#### Constant `SELINUX_MAGIC`

```rust
pub const SELINUX_MAGIC: u32 = 4185718668;
```

#### Constant `SMACK_MAGIC`

```rust
pub const SMACK_MAGIC: u32 = 1128357203;
```

#### Constant `RAMFS_MAGIC`

```rust
pub const RAMFS_MAGIC: u32 = 2240043254;
```

#### Constant `TMPFS_MAGIC`

```rust
pub const TMPFS_MAGIC: u32 = 16914836;
```

#### Constant `HUGETLBFS_MAGIC`

```rust
pub const HUGETLBFS_MAGIC: u32 = 2508478710;
```

#### Constant `SQUASHFS_MAGIC`

```rust
pub const SQUASHFS_MAGIC: u32 = 1936814952;
```

#### Constant `ECRYPTFS_SUPER_MAGIC`

```rust
pub const ECRYPTFS_SUPER_MAGIC: u32 = 61791;
```

#### Constant `EFS_SUPER_MAGIC`

```rust
pub const EFS_SUPER_MAGIC: u32 = 4278867;
```

#### Constant `EROFS_SUPER_MAGIC_V1`

```rust
pub const EROFS_SUPER_MAGIC_V1: u32 = 3774210530;
```

#### Constant `EXT2_SUPER_MAGIC`

```rust
pub const EXT2_SUPER_MAGIC: u32 = 61267;
```

#### Constant `EXT3_SUPER_MAGIC`

```rust
pub const EXT3_SUPER_MAGIC: u32 = 61267;
```

#### Constant `XENFS_SUPER_MAGIC`

```rust
pub const XENFS_SUPER_MAGIC: u32 = 2881100148;
```

#### Constant `EXT4_SUPER_MAGIC`

```rust
pub const EXT4_SUPER_MAGIC: u32 = 61267;
```

#### Constant `BTRFS_SUPER_MAGIC`

```rust
pub const BTRFS_SUPER_MAGIC: u32 = 2435016766;
```

#### Constant `NILFS_SUPER_MAGIC`

```rust
pub const NILFS_SUPER_MAGIC: u32 = 13364;
```

#### Constant `F2FS_SUPER_MAGIC`

```rust
pub const F2FS_SUPER_MAGIC: u32 = 4076150800;
```

#### Constant `HPFS_SUPER_MAGIC`

```rust
pub const HPFS_SUPER_MAGIC: u32 = 4187351113;
```

#### Constant `ISOFS_SUPER_MAGIC`

```rust
pub const ISOFS_SUPER_MAGIC: u32 = 38496;
```

#### Constant `JFFS2_SUPER_MAGIC`

```rust
pub const JFFS2_SUPER_MAGIC: u32 = 29366;
```

#### Constant `XFS_SUPER_MAGIC`

```rust
pub const XFS_SUPER_MAGIC: u32 = 1481003842;
```

#### Constant `PSTOREFS_MAGIC`

```rust
pub const PSTOREFS_MAGIC: u32 = 1634035564;
```

#### Constant `EFIVARFS_MAGIC`

```rust
pub const EFIVARFS_MAGIC: u32 = 3730735588;
```

#### Constant `HOSTFS_SUPER_MAGIC`

```rust
pub const HOSTFS_SUPER_MAGIC: u32 = 12648430;
```

#### Constant `OVERLAYFS_SUPER_MAGIC`

```rust
pub const OVERLAYFS_SUPER_MAGIC: u32 = 2035054128;
```

#### Constant `FUSE_SUPER_MAGIC`

```rust
pub const FUSE_SUPER_MAGIC: u32 = 1702057286;
```

#### Constant `BCACHEFS_SUPER_MAGIC`

```rust
pub const BCACHEFS_SUPER_MAGIC: u32 = 3393526350;
```

#### Constant `MINIX_SUPER_MAGIC`

```rust
pub const MINIX_SUPER_MAGIC: u32 = 4991;
```

#### Constant `MINIX_SUPER_MAGIC2`

```rust
pub const MINIX_SUPER_MAGIC2: u32 = 5007;
```

#### Constant `MINIX2_SUPER_MAGIC`

```rust
pub const MINIX2_SUPER_MAGIC: u32 = 9320;
```

#### Constant `MINIX2_SUPER_MAGIC2`

```rust
pub const MINIX2_SUPER_MAGIC2: u32 = 9336;
```

#### Constant `MINIX3_SUPER_MAGIC`

```rust
pub const MINIX3_SUPER_MAGIC: u32 = 19802;
```

#### Constant `MSDOS_SUPER_MAGIC`

```rust
pub const MSDOS_SUPER_MAGIC: u32 = 19780;
```

#### Constant `EXFAT_SUPER_MAGIC`

```rust
pub const EXFAT_SUPER_MAGIC: u32 = 538032816;
```

#### Constant `NCP_SUPER_MAGIC`

```rust
pub const NCP_SUPER_MAGIC: u32 = 22092;
```

#### Constant `NFS_SUPER_MAGIC`

```rust
pub const NFS_SUPER_MAGIC: u32 = 26985;
```

#### Constant `OCFS2_SUPER_MAGIC`

```rust
pub const OCFS2_SUPER_MAGIC: u32 = 1952539503;
```

#### Constant `OPENPROM_SUPER_MAGIC`

```rust
pub const OPENPROM_SUPER_MAGIC: u32 = 40865;
```

#### Constant `QNX4_SUPER_MAGIC`

```rust
pub const QNX4_SUPER_MAGIC: u32 = 47;
```

#### Constant `QNX6_SUPER_MAGIC`

```rust
pub const QNX6_SUPER_MAGIC: u32 = 1746473250;
```

#### Constant `AFS_FS_MAGIC`

```rust
pub const AFS_FS_MAGIC: u32 = 1799439955;
```

#### Constant `REISERFS_SUPER_MAGIC`

```rust
pub const REISERFS_SUPER_MAGIC: u32 = 1382369651;
```

#### Constant `REISERFS_SUPER_MAGIC_STRING`

```rust
pub const REISERFS_SUPER_MAGIC_STRING: &[u8; 9] = b"ReIsErFs\0";
```

#### Constant `REISER2FS_SUPER_MAGIC_STRING`

```rust
pub const REISER2FS_SUPER_MAGIC_STRING: &[u8; 10] = b"ReIsEr2Fs\0";
```

#### Constant `REISER2FS_JR_SUPER_MAGIC_STRING`

```rust
pub const REISER2FS_JR_SUPER_MAGIC_STRING: &[u8; 10] = b"ReIsEr3Fs\0";
```

#### Constant `SMB_SUPER_MAGIC`

```rust
pub const SMB_SUPER_MAGIC: u32 = 20859;
```

#### Constant `CIFS_SUPER_MAGIC`

```rust
pub const CIFS_SUPER_MAGIC: u32 = 4283649346;
```

#### Constant `SMB2_SUPER_MAGIC`

```rust
pub const SMB2_SUPER_MAGIC: u32 = 4266872130;
```

#### Constant `CGROUP_SUPER_MAGIC`

```rust
pub const CGROUP_SUPER_MAGIC: u32 = 2613483;
```

#### Constant `CGROUP2_SUPER_MAGIC`

```rust
pub const CGROUP2_SUPER_MAGIC: u32 = 1667723888;
```

#### Constant `RDTGROUP_SUPER_MAGIC`

```rust
pub const RDTGROUP_SUPER_MAGIC: u32 = 124082209;
```

#### Constant `STACK_END_MAGIC`

```rust
pub const STACK_END_MAGIC: u32 = 1470918301;
```

#### Constant `TRACEFS_MAGIC`

```rust
pub const TRACEFS_MAGIC: u32 = 1953653091;
```

#### Constant `V9FS_MAGIC`

```rust
pub const V9FS_MAGIC: u32 = 16914839;
```

#### Constant `BDEVFS_MAGIC`

```rust
pub const BDEVFS_MAGIC: u32 = 1650746742;
```

#### Constant `DAXFS_MAGIC`

```rust
pub const DAXFS_MAGIC: u32 = 1684300152;
```

#### Constant `BINFMTFS_MAGIC`

```rust
pub const BINFMTFS_MAGIC: u32 = 1112100429;
```

#### Constant `DEVPTS_SUPER_MAGIC`

```rust
pub const DEVPTS_SUPER_MAGIC: u32 = 7377;
```

#### Constant `BINDERFS_SUPER_MAGIC`

```rust
pub const BINDERFS_SUPER_MAGIC: u32 = 1819242352;
```

#### Constant `FUTEXFS_SUPER_MAGIC`

```rust
pub const FUTEXFS_SUPER_MAGIC: u32 = 195894762;
```

#### Constant `PIPEFS_MAGIC`

```rust
pub const PIPEFS_MAGIC: u32 = 1346981957;
```

#### Constant `PROC_SUPER_MAGIC`

```rust
pub const PROC_SUPER_MAGIC: u32 = 40864;
```

#### Constant `SOCKFS_MAGIC`

```rust
pub const SOCKFS_MAGIC: u32 = 1397703499;
```

#### Constant `SYSFS_MAGIC`

```rust
pub const SYSFS_MAGIC: u32 = 1650812274;
```

#### Constant `USBDEVICE_SUPER_MAGIC`

```rust
pub const USBDEVICE_SUPER_MAGIC: u32 = 40866;
```

#### Constant `MTD_INODE_FS_MAGIC`

```rust
pub const MTD_INODE_FS_MAGIC: u32 = 288389204;
```

#### Constant `ANON_INODE_FS_MAGIC`

```rust
pub const ANON_INODE_FS_MAGIC: u32 = 151263540;
```

#### Constant `BTRFS_TEST_MAGIC`

```rust
pub const BTRFS_TEST_MAGIC: u32 = 1936880249;
```

#### Constant `NSFS_MAGIC`

```rust
pub const NSFS_MAGIC: u32 = 1853056627;
```

#### Constant `BPF_FS_MAGIC`

```rust
pub const BPF_FS_MAGIC: u32 = 3405662737;
```

#### Constant `AAFS_MAGIC`

```rust
pub const AAFS_MAGIC: u32 = 1513908720;
```

#### Constant `ZONEFS_MAGIC`

```rust
pub const ZONEFS_MAGIC: u32 = 1515144787;
```

#### Constant `UDF_SUPER_MAGIC`

```rust
pub const UDF_SUPER_MAGIC: u32 = 352400198;
```

#### Constant `DMA_BUF_MAGIC`

```rust
pub const DMA_BUF_MAGIC: u32 = 1145913666;
```

#### Constant `DEVMEM_MAGIC`

```rust
pub const DEVMEM_MAGIC: u32 = 1162691661;
```

#### Constant `SECRETMEM_MAGIC`

```rust
pub const SECRETMEM_MAGIC: u32 = 1397048141;
```

#### Constant `PID_FS_MAGIC`

```rust
pub const PID_FS_MAGIC: u32 = 1346978886;
```

#### Constant `PROT_READ`

```rust
pub const PROT_READ: u32 = 1;
```

#### Constant `PROT_WRITE`

```rust
pub const PROT_WRITE: u32 = 2;
```

#### Constant `PROT_EXEC`

```rust
pub const PROT_EXEC: u32 = 4;
```

#### Constant `PROT_SEM`

```rust
pub const PROT_SEM: u32 = 8;
```

#### Constant `PROT_NONE`

```rust
pub const PROT_NONE: u32 = 0;
```

#### Constant `PROT_GROWSDOWN`

```rust
pub const PROT_GROWSDOWN: u32 = 16777216;
```

#### Constant `PROT_GROWSUP`

```rust
pub const PROT_GROWSUP: u32 = 33554432;
```

#### Constant `MAP_TYPE`

```rust
pub const MAP_TYPE: u32 = 15;
```

#### Constant `MAP_FIXED`

```rust
pub const MAP_FIXED: u32 = 16;
```

#### Constant `MAP_ANONYMOUS`

```rust
pub const MAP_ANONYMOUS: u32 = 32;
```

#### Constant `MAP_POPULATE`

```rust
pub const MAP_POPULATE: u32 = 32768;
```

#### Constant `MAP_NONBLOCK`

```rust
pub const MAP_NONBLOCK: u32 = 65536;
```

#### Constant `MAP_STACK`

```rust
pub const MAP_STACK: u32 = 131072;
```

#### Constant `MAP_HUGETLB`

```rust
pub const MAP_HUGETLB: u32 = 262144;
```

#### Constant `MAP_SYNC`

```rust
pub const MAP_SYNC: u32 = 524288;
```

#### Constant `MAP_FIXED_NOREPLACE`

```rust
pub const MAP_FIXED_NOREPLACE: u32 = 1048576;
```

#### Constant `MAP_UNINITIALIZED`

```rust
pub const MAP_UNINITIALIZED: u32 = 67108864;
```

#### Constant `MLOCK_ONFAULT`

```rust
pub const MLOCK_ONFAULT: u32 = 1;
```

#### Constant `MS_ASYNC`

```rust
pub const MS_ASYNC: u32 = 1;
```

#### Constant `MS_INVALIDATE`

```rust
pub const MS_INVALIDATE: u32 = 2;
```

#### Constant `MS_SYNC`

```rust
pub const MS_SYNC: u32 = 4;
```

#### Constant `MADV_NORMAL`

```rust
pub const MADV_NORMAL: u32 = 0;
```

#### Constant `MADV_RANDOM`

```rust
pub const MADV_RANDOM: u32 = 1;
```

#### Constant `MADV_SEQUENTIAL`

```rust
pub const MADV_SEQUENTIAL: u32 = 2;
```

#### Constant `MADV_WILLNEED`

```rust
pub const MADV_WILLNEED: u32 = 3;
```

#### Constant `MADV_DONTNEED`

```rust
pub const MADV_DONTNEED: u32 = 4;
```

#### Constant `MADV_FREE`

```rust
pub const MADV_FREE: u32 = 8;
```

#### Constant `MADV_REMOVE`

```rust
pub const MADV_REMOVE: u32 = 9;
```

#### Constant `MADV_DONTFORK`

```rust
pub const MADV_DONTFORK: u32 = 10;
```

#### Constant `MADV_DOFORK`

```rust
pub const MADV_DOFORK: u32 = 11;
```

#### Constant `MADV_HWPOISON`

```rust
pub const MADV_HWPOISON: u32 = 100;
```

#### Constant `MADV_SOFT_OFFLINE`

```rust
pub const MADV_SOFT_OFFLINE: u32 = 101;
```

#### Constant `MADV_MERGEABLE`

```rust
pub const MADV_MERGEABLE: u32 = 12;
```

#### Constant `MADV_UNMERGEABLE`

```rust
pub const MADV_UNMERGEABLE: u32 = 13;
```

#### Constant `MADV_HUGEPAGE`

```rust
pub const MADV_HUGEPAGE: u32 = 14;
```

#### Constant `MADV_NOHUGEPAGE`

```rust
pub const MADV_NOHUGEPAGE: u32 = 15;
```

#### Constant `MADV_DONTDUMP`

```rust
pub const MADV_DONTDUMP: u32 = 16;
```

#### Constant `MADV_DODUMP`

```rust
pub const MADV_DODUMP: u32 = 17;
```

#### Constant `MADV_WIPEONFORK`

```rust
pub const MADV_WIPEONFORK: u32 = 18;
```

#### Constant `MADV_KEEPONFORK`

```rust
pub const MADV_KEEPONFORK: u32 = 19;
```

#### Constant `MADV_COLD`

```rust
pub const MADV_COLD: u32 = 20;
```

#### Constant `MADV_PAGEOUT`

```rust
pub const MADV_PAGEOUT: u32 = 21;
```

#### Constant `MADV_POPULATE_READ`

```rust
pub const MADV_POPULATE_READ: u32 = 22;
```

#### Constant `MADV_POPULATE_WRITE`

```rust
pub const MADV_POPULATE_WRITE: u32 = 23;
```

#### Constant `MADV_DONTNEED_LOCKED`

```rust
pub const MADV_DONTNEED_LOCKED: u32 = 24;
```

#### Constant `MADV_COLLAPSE`

```rust
pub const MADV_COLLAPSE: u32 = 25;
```

#### Constant `MADV_GUARD_INSTALL`

```rust
pub const MADV_GUARD_INSTALL: u32 = 102;
```

#### Constant `MADV_GUARD_REMOVE`

```rust
pub const MADV_GUARD_REMOVE: u32 = 103;
```

#### Constant `MAP_FILE`

```rust
pub const MAP_FILE: u32 = 0;
```

#### Constant `PKEY_DISABLE_ACCESS`

```rust
pub const PKEY_DISABLE_ACCESS: u32 = 1;
```

#### Constant `PKEY_DISABLE_WRITE`

```rust
pub const PKEY_DISABLE_WRITE: u32 = 2;
```

#### Constant `PKEY_ACCESS_MASK`

```rust
pub const PKEY_ACCESS_MASK: u32 = 3;
```

#### Constant `MAP_GROWSDOWN`

```rust
pub const MAP_GROWSDOWN: u32 = 256;
```

#### Constant `MAP_DENYWRITE`

```rust
pub const MAP_DENYWRITE: u32 = 2048;
```

#### Constant `MAP_EXECUTABLE`

```rust
pub const MAP_EXECUTABLE: u32 = 4096;
```

#### Constant `MAP_LOCKED`

```rust
pub const MAP_LOCKED: u32 = 8192;
```

#### Constant `MAP_NORESERVE`

```rust
pub const MAP_NORESERVE: u32 = 16384;
```

#### Constant `MCL_CURRENT`

```rust
pub const MCL_CURRENT: u32 = 1;
```

#### Constant `MCL_FUTURE`

```rust
pub const MCL_FUTURE: u32 = 2;
```

#### Constant `MCL_ONFAULT`

```rust
pub const MCL_ONFAULT: u32 = 4;
```

#### Constant `SHADOW_STACK_SET_TOKEN`

```rust
pub const SHADOW_STACK_SET_TOKEN: u32 = 1;
```

#### Constant `SHADOW_STACK_SET_MARKER`

```rust
pub const SHADOW_STACK_SET_MARKER: u32 = 2;
```

#### Constant `PROT_BTI`

```rust
pub const PROT_BTI: u32 = 16;
```

#### Constant `PROT_MTE`

```rust
pub const PROT_MTE: u32 = 32;
```

#### Constant `PKEY_DISABLE_EXECUTE`

```rust
pub const PKEY_DISABLE_EXECUTE: u32 = 4;
```

#### Constant `PKEY_DISABLE_READ`

```rust
pub const PKEY_DISABLE_READ: u32 = 8;
```

#### Constant `HUGETLB_FLAG_ENCODE_SHIFT`

```rust
pub const HUGETLB_FLAG_ENCODE_SHIFT: u32 = 26;
```

#### Constant `HUGETLB_FLAG_ENCODE_MASK`

```rust
pub const HUGETLB_FLAG_ENCODE_MASK: u32 = 63;
```

#### Constant `HUGETLB_FLAG_ENCODE_16KB`

```rust
pub const HUGETLB_FLAG_ENCODE_16KB: u32 = 939524096;
```

#### Constant `HUGETLB_FLAG_ENCODE_64KB`

```rust
pub const HUGETLB_FLAG_ENCODE_64KB: u32 = 1073741824;
```

#### Constant `HUGETLB_FLAG_ENCODE_512KB`

```rust
pub const HUGETLB_FLAG_ENCODE_512KB: u32 = 1275068416;
```

#### Constant `HUGETLB_FLAG_ENCODE_1MB`

```rust
pub const HUGETLB_FLAG_ENCODE_1MB: u32 = 1342177280;
```

#### Constant `HUGETLB_FLAG_ENCODE_2MB`

```rust
pub const HUGETLB_FLAG_ENCODE_2MB: u32 = 1409286144;
```

#### Constant `HUGETLB_FLAG_ENCODE_8MB`

```rust
pub const HUGETLB_FLAG_ENCODE_8MB: u32 = 1543503872;
```

#### Constant `HUGETLB_FLAG_ENCODE_16MB`

```rust
pub const HUGETLB_FLAG_ENCODE_16MB: u32 = 1610612736;
```

#### Constant `HUGETLB_FLAG_ENCODE_32MB`

```rust
pub const HUGETLB_FLAG_ENCODE_32MB: u32 = 1677721600;
```

#### Constant `HUGETLB_FLAG_ENCODE_256MB`

```rust
pub const HUGETLB_FLAG_ENCODE_256MB: u32 = 1879048192;
```

#### Constant `HUGETLB_FLAG_ENCODE_512MB`

```rust
pub const HUGETLB_FLAG_ENCODE_512MB: u32 = 1946157056;
```

#### Constant `HUGETLB_FLAG_ENCODE_1GB`

```rust
pub const HUGETLB_FLAG_ENCODE_1GB: u32 = 2013265920;
```

#### Constant `HUGETLB_FLAG_ENCODE_2GB`

```rust
pub const HUGETLB_FLAG_ENCODE_2GB: u32 = 2080374784;
```

#### Constant `HUGETLB_FLAG_ENCODE_16GB`

```rust
pub const HUGETLB_FLAG_ENCODE_16GB: u32 = 2281701376;
```

#### Constant `MREMAP_MAYMOVE`

```rust
pub const MREMAP_MAYMOVE: u32 = 1;
```

#### Constant `MREMAP_FIXED`

```rust
pub const MREMAP_FIXED: u32 = 2;
```

#### Constant `MREMAP_DONTUNMAP`

```rust
pub const MREMAP_DONTUNMAP: u32 = 4;
```

#### Constant `OVERCOMMIT_GUESS`

```rust
pub const OVERCOMMIT_GUESS: u32 = 0;
```

#### Constant `OVERCOMMIT_ALWAYS`

```rust
pub const OVERCOMMIT_ALWAYS: u32 = 1;
```

#### Constant `OVERCOMMIT_NEVER`

```rust
pub const OVERCOMMIT_NEVER: u32 = 2;
```

#### Constant `MAP_SHARED`

```rust
pub const MAP_SHARED: u32 = 1;
```

#### Constant `MAP_PRIVATE`

```rust
pub const MAP_PRIVATE: u32 = 2;
```

#### Constant `MAP_SHARED_VALIDATE`

```rust
pub const MAP_SHARED_VALIDATE: u32 = 3;
```

#### Constant `MAP_DROPPABLE`

```rust
pub const MAP_DROPPABLE: u32 = 8;
```

#### Constant `MAP_HUGE_SHIFT`

```rust
pub const MAP_HUGE_SHIFT: u32 = 26;
```

#### Constant `MAP_HUGE_MASK`

```rust
pub const MAP_HUGE_MASK: u32 = 63;
```

#### Constant `MAP_HUGE_16KB`

```rust
pub const MAP_HUGE_16KB: u32 = 939524096;
```

#### Constant `MAP_HUGE_64KB`

```rust
pub const MAP_HUGE_64KB: u32 = 1073741824;
```

#### Constant `MAP_HUGE_512KB`

```rust
pub const MAP_HUGE_512KB: u32 = 1275068416;
```

#### Constant `MAP_HUGE_1MB`

```rust
pub const MAP_HUGE_1MB: u32 = 1342177280;
```

#### Constant `MAP_HUGE_2MB`

```rust
pub const MAP_HUGE_2MB: u32 = 1409286144;
```

#### Constant `MAP_HUGE_8MB`

```rust
pub const MAP_HUGE_8MB: u32 = 1543503872;
```

#### Constant `MAP_HUGE_16MB`

```rust
pub const MAP_HUGE_16MB: u32 = 1610612736;
```

#### Constant `MAP_HUGE_32MB`

```rust
pub const MAP_HUGE_32MB: u32 = 1677721600;
```

#### Constant `MAP_HUGE_256MB`

```rust
pub const MAP_HUGE_256MB: u32 = 1879048192;
```

#### Constant `MAP_HUGE_512MB`

```rust
pub const MAP_HUGE_512MB: u32 = 1946157056;
```

#### Constant `MAP_HUGE_1GB`

```rust
pub const MAP_HUGE_1GB: u32 = 2013265920;
```

#### Constant `MAP_HUGE_2GB`

```rust
pub const MAP_HUGE_2GB: u32 = 2080374784;
```

#### Constant `MAP_HUGE_16GB`

```rust
pub const MAP_HUGE_16GB: u32 = 2281701376;
```

#### Constant `POLLIN`

```rust
pub const POLLIN: u32 = 1;
```

#### Constant `POLLPRI`

```rust
pub const POLLPRI: u32 = 2;
```

#### Constant `POLLOUT`

```rust
pub const POLLOUT: u32 = 4;
```

#### Constant `POLLERR`

```rust
pub const POLLERR: u32 = 8;
```

#### Constant `POLLHUP`

```rust
pub const POLLHUP: u32 = 16;
```

#### Constant `POLLNVAL`

```rust
pub const POLLNVAL: u32 = 32;
```

#### Constant `POLLRDNORM`

```rust
pub const POLLRDNORM: u32 = 64;
```

#### Constant `POLLRDBAND`

```rust
pub const POLLRDBAND: u32 = 128;
```

#### Constant `POLLWRNORM`

```rust
pub const POLLWRNORM: u32 = 256;
```

#### Constant `POLLWRBAND`

```rust
pub const POLLWRBAND: u32 = 512;
```

#### Constant `POLLMSG`

```rust
pub const POLLMSG: u32 = 1024;
```

#### Constant `POLLREMOVE`

```rust
pub const POLLREMOVE: u32 = 4096;
```

#### Constant `POLLRDHUP`

```rust
pub const POLLRDHUP: u32 = 8192;
```

#### Constant `GRND_NONBLOCK`

```rust
pub const GRND_NONBLOCK: u32 = 1;
```

#### Constant `GRND_RANDOM`

```rust
pub const GRND_RANDOM: u32 = 2;
```

#### Constant `GRND_INSECURE`

```rust
pub const GRND_INSECURE: u32 = 4;
```

#### Constant `LINUX_REBOOT_MAGIC1`

```rust
pub const LINUX_REBOOT_MAGIC1: u32 = 4276215469;
```

#### Constant `LINUX_REBOOT_MAGIC2`

```rust
pub const LINUX_REBOOT_MAGIC2: u32 = 672274793;
```

#### Constant `LINUX_REBOOT_MAGIC2A`

```rust
pub const LINUX_REBOOT_MAGIC2A: u32 = 85072278;
```

#### Constant `LINUX_REBOOT_MAGIC2B`

```rust
pub const LINUX_REBOOT_MAGIC2B: u32 = 369367448;
```

#### Constant `LINUX_REBOOT_MAGIC2C`

```rust
pub const LINUX_REBOOT_MAGIC2C: u32 = 537993216;
```

#### Constant `LINUX_REBOOT_CMD_RESTART`

```rust
pub const LINUX_REBOOT_CMD_RESTART: u32 = 19088743;
```

#### Constant `LINUX_REBOOT_CMD_HALT`

```rust
pub const LINUX_REBOOT_CMD_HALT: u32 = 3454992675;
```

#### Constant `LINUX_REBOOT_CMD_CAD_ON`

```rust
pub const LINUX_REBOOT_CMD_CAD_ON: u32 = 2309737967;
```

#### Constant `LINUX_REBOOT_CMD_CAD_OFF`

```rust
pub const LINUX_REBOOT_CMD_CAD_OFF: u32 = 0;
```

#### Constant `LINUX_REBOOT_CMD_POWER_OFF`

```rust
pub const LINUX_REBOOT_CMD_POWER_OFF: u32 = 1126301404;
```

#### Constant `LINUX_REBOOT_CMD_RESTART2`

```rust
pub const LINUX_REBOOT_CMD_RESTART2: u32 = 2712847316;
```

#### Constant `LINUX_REBOOT_CMD_SW_SUSPEND`

```rust
pub const LINUX_REBOOT_CMD_SW_SUSPEND: u32 = 3489725666;
```

#### Constant `LINUX_REBOOT_CMD_KEXEC`

```rust
pub const LINUX_REBOOT_CMD_KEXEC: u32 = 1163412803;
```

#### Constant `RUSAGE_SELF`

```rust
pub const RUSAGE_SELF: u32 = 0;
```

#### Constant `RUSAGE_CHILDREN`

```rust
pub const RUSAGE_CHILDREN: i32 = -1;
```

#### Constant `RUSAGE_BOTH`

```rust
pub const RUSAGE_BOTH: i32 = -2;
```

#### Constant `RUSAGE_THREAD`

```rust
pub const RUSAGE_THREAD: u32 = 1;
```

#### Constant `RLIM64_INFINITY`

```rust
pub const RLIM64_INFINITY: i32 = -1;
```

#### Constant `PRIO_MIN`

```rust
pub const PRIO_MIN: i32 = -20;
```

#### Constant `PRIO_MAX`

```rust
pub const PRIO_MAX: u32 = 20;
```

#### Constant `PRIO_PROCESS`

```rust
pub const PRIO_PROCESS: u32 = 0;
```

#### Constant `PRIO_PGRP`

```rust
pub const PRIO_PGRP: u32 = 1;
```

#### Constant `PRIO_USER`

```rust
pub const PRIO_USER: u32 = 2;
```

#### Constant `_STK_LIM`

```rust
pub const _STK_LIM: u32 = 8388608;
```

#### Constant `MLOCK_LIMIT`

```rust
pub const MLOCK_LIMIT: u32 = 8388608;
```

#### Constant `RLIMIT_CPU`

```rust
pub const RLIMIT_CPU: u32 = 0;
```

#### Constant `RLIMIT_FSIZE`

```rust
pub const RLIMIT_FSIZE: u32 = 1;
```

#### Constant `RLIMIT_DATA`

```rust
pub const RLIMIT_DATA: u32 = 2;
```

#### Constant `RLIMIT_STACK`

```rust
pub const RLIMIT_STACK: u32 = 3;
```

#### Constant `RLIMIT_CORE`

```rust
pub const RLIMIT_CORE: u32 = 4;
```

#### Constant `RLIMIT_RSS`

```rust
pub const RLIMIT_RSS: u32 = 5;
```

#### Constant `RLIMIT_NPROC`

```rust
pub const RLIMIT_NPROC: u32 = 6;
```

#### Constant `RLIMIT_NOFILE`

```rust
pub const RLIMIT_NOFILE: u32 = 7;
```

#### Constant `RLIMIT_MEMLOCK`

```rust
pub const RLIMIT_MEMLOCK: u32 = 8;
```

#### Constant `RLIMIT_AS`

```rust
pub const RLIMIT_AS: u32 = 9;
```

#### Constant `RLIMIT_LOCKS`

```rust
pub const RLIMIT_LOCKS: u32 = 10;
```

#### Constant `RLIMIT_SIGPENDING`

```rust
pub const RLIMIT_SIGPENDING: u32 = 11;
```

#### Constant `RLIMIT_MSGQUEUE`

```rust
pub const RLIMIT_MSGQUEUE: u32 = 12;
```

#### Constant `RLIMIT_NICE`

```rust
pub const RLIMIT_NICE: u32 = 13;
```

#### Constant `RLIMIT_RTPRIO`

```rust
pub const RLIMIT_RTPRIO: u32 = 14;
```

#### Constant `RLIMIT_RTTIME`

```rust
pub const RLIMIT_RTTIME: u32 = 15;
```

#### Constant `RLIM_NLIMITS`

```rust
pub const RLIM_NLIMITS: u32 = 16;
```

#### Constant `RLIM_INFINITY`

```rust
pub const RLIM_INFINITY: i32 = -1;
```

#### Constant `CSIGNAL`

```rust
pub const CSIGNAL: u32 = 255;
```

#### Constant `CLONE_VM`

```rust
pub const CLONE_VM: u32 = 256;
```

#### Constant `CLONE_FS`

```rust
pub const CLONE_FS: u32 = 512;
```

#### Constant `CLONE_FILES`

```rust
pub const CLONE_FILES: u32 = 1024;
```

#### Constant `CLONE_SIGHAND`

```rust
pub const CLONE_SIGHAND: u32 = 2048;
```

#### Constant `CLONE_PIDFD`

```rust
pub const CLONE_PIDFD: u32 = 4096;
```

#### Constant `CLONE_PTRACE`

```rust
pub const CLONE_PTRACE: u32 = 8192;
```

#### Constant `CLONE_VFORK`

```rust
pub const CLONE_VFORK: u32 = 16384;
```

#### Constant `CLONE_PARENT`

```rust
pub const CLONE_PARENT: u32 = 32768;
```

#### Constant `CLONE_THREAD`

```rust
pub const CLONE_THREAD: u32 = 65536;
```

#### Constant `CLONE_NEWNS`

```rust
pub const CLONE_NEWNS: u32 = 131072;
```

#### Constant `CLONE_SYSVSEM`

```rust
pub const CLONE_SYSVSEM: u32 = 262144;
```

#### Constant `CLONE_SETTLS`

```rust
pub const CLONE_SETTLS: u32 = 524288;
```

#### Constant `CLONE_PARENT_SETTID`

```rust
pub const CLONE_PARENT_SETTID: u32 = 1048576;
```

#### Constant `CLONE_CHILD_CLEARTID`

```rust
pub const CLONE_CHILD_CLEARTID: u32 = 2097152;
```

#### Constant `CLONE_DETACHED`

```rust
pub const CLONE_DETACHED: u32 = 4194304;
```

#### Constant `CLONE_UNTRACED`

```rust
pub const CLONE_UNTRACED: u32 = 8388608;
```

#### Constant `CLONE_CHILD_SETTID`

```rust
pub const CLONE_CHILD_SETTID: u32 = 16777216;
```

#### Constant `CLONE_NEWCGROUP`

```rust
pub const CLONE_NEWCGROUP: u32 = 33554432;
```

#### Constant `CLONE_NEWUTS`

```rust
pub const CLONE_NEWUTS: u32 = 67108864;
```

#### Constant `CLONE_NEWIPC`

```rust
pub const CLONE_NEWIPC: u32 = 134217728;
```

#### Constant `CLONE_NEWUSER`

```rust
pub const CLONE_NEWUSER: u32 = 268435456;
```

#### Constant `CLONE_NEWPID`

```rust
pub const CLONE_NEWPID: u32 = 536870912;
```

#### Constant `CLONE_NEWNET`

```rust
pub const CLONE_NEWNET: u32 = 1073741824;
```

#### Constant `CLONE_IO`

```rust
pub const CLONE_IO: u32 = 2147483648;
```

#### Constant `CLONE_CLEAR_SIGHAND`

```rust
pub const CLONE_CLEAR_SIGHAND: u64 = 4294967296;
```

#### Constant `CLONE_INTO_CGROUP`

```rust
pub const CLONE_INTO_CGROUP: u64 = 8589934592;
```

#### Constant `CLONE_NEWTIME`

```rust
pub const CLONE_NEWTIME: u32 = 128;
```

#### Constant `CLONE_ARGS_SIZE_VER0`

```rust
pub const CLONE_ARGS_SIZE_VER0: u32 = 64;
```

#### Constant `CLONE_ARGS_SIZE_VER1`

```rust
pub const CLONE_ARGS_SIZE_VER1: u32 = 80;
```

#### Constant `CLONE_ARGS_SIZE_VER2`

```rust
pub const CLONE_ARGS_SIZE_VER2: u32 = 88;
```

#### Constant `SCHED_NORMAL`

```rust
pub const SCHED_NORMAL: u32 = 0;
```

#### Constant `SCHED_FIFO`

```rust
pub const SCHED_FIFO: u32 = 1;
```

#### Constant `SCHED_RR`

```rust
pub const SCHED_RR: u32 = 2;
```

#### Constant `SCHED_BATCH`

```rust
pub const SCHED_BATCH: u32 = 3;
```

#### Constant `SCHED_IDLE`

```rust
pub const SCHED_IDLE: u32 = 5;
```

#### Constant `SCHED_DEADLINE`

```rust
pub const SCHED_DEADLINE: u32 = 6;
```

#### Constant `SCHED_EXT`

```rust
pub const SCHED_EXT: u32 = 7;
```

#### Constant `SCHED_RESET_ON_FORK`

```rust
pub const SCHED_RESET_ON_FORK: u32 = 1073741824;
```

#### Constant `SCHED_FLAG_RESET_ON_FORK`

```rust
pub const SCHED_FLAG_RESET_ON_FORK: u32 = 1;
```

#### Constant `SCHED_FLAG_RECLAIM`

```rust
pub const SCHED_FLAG_RECLAIM: u32 = 2;
```

#### Constant `SCHED_FLAG_DL_OVERRUN`

```rust
pub const SCHED_FLAG_DL_OVERRUN: u32 = 4;
```

#### Constant `SCHED_FLAG_KEEP_POLICY`

```rust
pub const SCHED_FLAG_KEEP_POLICY: u32 = 8;
```

#### Constant `SCHED_FLAG_KEEP_PARAMS`

```rust
pub const SCHED_FLAG_KEEP_PARAMS: u32 = 16;
```

#### Constant `SCHED_FLAG_UTIL_CLAMP_MIN`

```rust
pub const SCHED_FLAG_UTIL_CLAMP_MIN: u32 = 32;
```

#### Constant `SCHED_FLAG_UTIL_CLAMP_MAX`

```rust
pub const SCHED_FLAG_UTIL_CLAMP_MAX: u32 = 64;
```

#### Constant `SCHED_FLAG_KEEP_ALL`

```rust
pub const SCHED_FLAG_KEEP_ALL: u32 = 24;
```

#### Constant `SCHED_FLAG_UTIL_CLAMP`

```rust
pub const SCHED_FLAG_UTIL_CLAMP: u32 = 96;
```

#### Constant `SCHED_FLAG_ALL`

```rust
pub const SCHED_FLAG_ALL: u32 = 127;
```

#### Constant `SA_RESTORER`

```rust
pub const SA_RESTORER: u32 = 67108864;
```

#### Constant `MINSIGSTKSZ`

```rust
pub const MINSIGSTKSZ: u32 = 5120;
```

#### Constant `SIGSTKSZ`

```rust
pub const SIGSTKSZ: u32 = 16384;
```

#### Constant `_NSIG`

```rust
pub const _NSIG: u32 = 64;
```

#### Constant `_NSIG_BPW`

```rust
pub const _NSIG_BPW: u32 = 64;
```

#### Constant `_NSIG_WORDS`

```rust
pub const _NSIG_WORDS: u32 = 1;
```

#### Constant `SIGHUP`

```rust
pub const SIGHUP: u32 = 1;
```

#### Constant `SIGINT`

```rust
pub const SIGINT: u32 = 2;
```

#### Constant `SIGQUIT`

```rust
pub const SIGQUIT: u32 = 3;
```

#### Constant `SIGILL`

```rust
pub const SIGILL: u32 = 4;
```

#### Constant `SIGTRAP`

```rust
pub const SIGTRAP: u32 = 5;
```

#### Constant `SIGABRT`

```rust
pub const SIGABRT: u32 = 6;
```

#### Constant `SIGIOT`

```rust
pub const SIGIOT: u32 = 6;
```

#### Constant `SIGBUS`

```rust
pub const SIGBUS: u32 = 7;
```

#### Constant `SIGFPE`

```rust
pub const SIGFPE: u32 = 8;
```

#### Constant `SIGKILL`

```rust
pub const SIGKILL: u32 = 9;
```

#### Constant `SIGUSR1`

```rust
pub const SIGUSR1: u32 = 10;
```

#### Constant `SIGSEGV`

```rust
pub const SIGSEGV: u32 = 11;
```

#### Constant `SIGUSR2`

```rust
pub const SIGUSR2: u32 = 12;
```

#### Constant `SIGPIPE`

```rust
pub const SIGPIPE: u32 = 13;
```

#### Constant `SIGALRM`

```rust
pub const SIGALRM: u32 = 14;
```

#### Constant `SIGTERM`

```rust
pub const SIGTERM: u32 = 15;
```

#### Constant `SIGSTKFLT`

```rust
pub const SIGSTKFLT: u32 = 16;
```

#### Constant `SIGCHLD`

```rust
pub const SIGCHLD: u32 = 17;
```

#### Constant `SIGCONT`

```rust
pub const SIGCONT: u32 = 18;
```

#### Constant `SIGSTOP`

```rust
pub const SIGSTOP: u32 = 19;
```

#### Constant `SIGTSTP`

```rust
pub const SIGTSTP: u32 = 20;
```

#### Constant `SIGTTIN`

```rust
pub const SIGTTIN: u32 = 21;
```

#### Constant `SIGTTOU`

```rust
pub const SIGTTOU: u32 = 22;
```

#### Constant `SIGURG`

```rust
pub const SIGURG: u32 = 23;
```

#### Constant `SIGXCPU`

```rust
pub const SIGXCPU: u32 = 24;
```

#### Constant `SIGXFSZ`

```rust
pub const SIGXFSZ: u32 = 25;
```

#### Constant `SIGVTALRM`

```rust
pub const SIGVTALRM: u32 = 26;
```

#### Constant `SIGPROF`

```rust
pub const SIGPROF: u32 = 27;
```

#### Constant `SIGWINCH`

```rust
pub const SIGWINCH: u32 = 28;
```

#### Constant `SIGIO`

```rust
pub const SIGIO: u32 = 29;
```

#### Constant `SIGPOLL`

```rust
pub const SIGPOLL: u32 = 29;
```

#### Constant `SIGPWR`

```rust
pub const SIGPWR: u32 = 30;
```

#### Constant `SIGSYS`

```rust
pub const SIGSYS: u32 = 31;
```

#### Constant `SIGUNUSED`

```rust
pub const SIGUNUSED: u32 = 31;
```

#### Constant `SIGRTMIN`

```rust
pub const SIGRTMIN: u32 = 32;
```

#### Constant `SIGRTMAX`

```rust
pub const SIGRTMAX: u32 = 64;
```

#### Constant `SA_NOCLDSTOP`

```rust
pub const SA_NOCLDSTOP: u32 = 1;
```

#### Constant `SA_NOCLDWAIT`

```rust
pub const SA_NOCLDWAIT: u32 = 2;
```

#### Constant `SA_SIGINFO`

```rust
pub const SA_SIGINFO: u32 = 4;
```

#### Constant `SA_UNSUPPORTED`

```rust
pub const SA_UNSUPPORTED: u32 = 1024;
```

#### Constant `SA_EXPOSE_TAGBITS`

```rust
pub const SA_EXPOSE_TAGBITS: u32 = 2048;
```

#### Constant `SA_ONSTACK`

```rust
pub const SA_ONSTACK: u32 = 134217728;
```

#### Constant `SA_RESTART`

```rust
pub const SA_RESTART: u32 = 268435456;
```

#### Constant `SA_NODEFER`

```rust
pub const SA_NODEFER: u32 = 1073741824;
```

#### Constant `SA_RESETHAND`

```rust
pub const SA_RESETHAND: u32 = 2147483648;
```

#### Constant `SA_NOMASK`

```rust
pub const SA_NOMASK: u32 = 1073741824;
```

#### Constant `SA_ONESHOT`

```rust
pub const SA_ONESHOT: u32 = 2147483648;
```

#### Constant `SIG_BLOCK`

```rust
pub const SIG_BLOCK: u32 = 0;
```

#### Constant `SIG_UNBLOCK`

```rust
pub const SIG_UNBLOCK: u32 = 1;
```

#### Constant `SIG_SETMASK`

```rust
pub const SIG_SETMASK: u32 = 2;
```

#### Constant `SI_MAX_SIZE`

```rust
pub const SI_MAX_SIZE: u32 = 128;
```

#### Constant `SI_USER`

```rust
pub const SI_USER: u32 = 0;
```

#### Constant `SI_KERNEL`

```rust
pub const SI_KERNEL: u32 = 128;
```

#### Constant `SI_QUEUE`

```rust
pub const SI_QUEUE: i32 = -1;
```

#### Constant `SI_TIMER`

```rust
pub const SI_TIMER: i32 = -2;
```

#### Constant `SI_MESGQ`

```rust
pub const SI_MESGQ: i32 = -3;
```

#### Constant `SI_ASYNCIO`

```rust
pub const SI_ASYNCIO: i32 = -4;
```

#### Constant `SI_SIGIO`

```rust
pub const SI_SIGIO: i32 = -5;
```

#### Constant `SI_TKILL`

```rust
pub const SI_TKILL: i32 = -6;
```

#### Constant `SI_DETHREAD`

```rust
pub const SI_DETHREAD: i32 = -7;
```

#### Constant `SI_ASYNCNL`

```rust
pub const SI_ASYNCNL: i32 = -60;
```

#### Constant `ILL_ILLOPC`

```rust
pub const ILL_ILLOPC: u32 = 1;
```

#### Constant `ILL_ILLOPN`

```rust
pub const ILL_ILLOPN: u32 = 2;
```

#### Constant `ILL_ILLADR`

```rust
pub const ILL_ILLADR: u32 = 3;
```

#### Constant `ILL_ILLTRP`

```rust
pub const ILL_ILLTRP: u32 = 4;
```

#### Constant `ILL_PRVOPC`

```rust
pub const ILL_PRVOPC: u32 = 5;
```

#### Constant `ILL_PRVREG`

```rust
pub const ILL_PRVREG: u32 = 6;
```

#### Constant `ILL_COPROC`

```rust
pub const ILL_COPROC: u32 = 7;
```

#### Constant `ILL_BADSTK`

```rust
pub const ILL_BADSTK: u32 = 8;
```

#### Constant `ILL_BADIADDR`

```rust
pub const ILL_BADIADDR: u32 = 9;
```

#### Constant `__ILL_BREAK`

```rust
pub const __ILL_BREAK: u32 = 10;
```

#### Constant `__ILL_BNDMOD`

```rust
pub const __ILL_BNDMOD: u32 = 11;
```

#### Constant `NSIGILL`

```rust
pub const NSIGILL: u32 = 11;
```

#### Constant `FPE_INTDIV`

```rust
pub const FPE_INTDIV: u32 = 1;
```

#### Constant `FPE_INTOVF`

```rust
pub const FPE_INTOVF: u32 = 2;
```

#### Constant `FPE_FLTDIV`

```rust
pub const FPE_FLTDIV: u32 = 3;
```

#### Constant `FPE_FLTOVF`

```rust
pub const FPE_FLTOVF: u32 = 4;
```

#### Constant `FPE_FLTUND`

```rust
pub const FPE_FLTUND: u32 = 5;
```

#### Constant `FPE_FLTRES`

```rust
pub const FPE_FLTRES: u32 = 6;
```

#### Constant `FPE_FLTINV`

```rust
pub const FPE_FLTINV: u32 = 7;
```

#### Constant `FPE_FLTSUB`

```rust
pub const FPE_FLTSUB: u32 = 8;
```

#### Constant `__FPE_DECOVF`

```rust
pub const __FPE_DECOVF: u32 = 9;
```

#### Constant `__FPE_DECDIV`

```rust
pub const __FPE_DECDIV: u32 = 10;
```

#### Constant `__FPE_DECERR`

```rust
pub const __FPE_DECERR: u32 = 11;
```

#### Constant `__FPE_INVASC`

```rust
pub const __FPE_INVASC: u32 = 12;
```

#### Constant `__FPE_INVDEC`

```rust
pub const __FPE_INVDEC: u32 = 13;
```

#### Constant `FPE_FLTUNK`

```rust
pub const FPE_FLTUNK: u32 = 14;
```

#### Constant `FPE_CONDTRAP`

```rust
pub const FPE_CONDTRAP: u32 = 15;
```

#### Constant `NSIGFPE`

```rust
pub const NSIGFPE: u32 = 15;
```

#### Constant `SEGV_MAPERR`

```rust
pub const SEGV_MAPERR: u32 = 1;
```

#### Constant `SEGV_ACCERR`

```rust
pub const SEGV_ACCERR: u32 = 2;
```

#### Constant `SEGV_BNDERR`

```rust
pub const SEGV_BNDERR: u32 = 3;
```

#### Constant `SEGV_PKUERR`

```rust
pub const SEGV_PKUERR: u32 = 4;
```

#### Constant `SEGV_ACCADI`

```rust
pub const SEGV_ACCADI: u32 = 5;
```

#### Constant `SEGV_ADIDERR`

```rust
pub const SEGV_ADIDERR: u32 = 6;
```

#### Constant `SEGV_ADIPERR`

```rust
pub const SEGV_ADIPERR: u32 = 7;
```

#### Constant `SEGV_MTEAERR`

```rust
pub const SEGV_MTEAERR: u32 = 8;
```

#### Constant `SEGV_MTESERR`

```rust
pub const SEGV_MTESERR: u32 = 9;
```

#### Constant `SEGV_CPERR`

```rust
pub const SEGV_CPERR: u32 = 10;
```

#### Constant `NSIGSEGV`

```rust
pub const NSIGSEGV: u32 = 10;
```

#### Constant `BUS_ADRALN`

```rust
pub const BUS_ADRALN: u32 = 1;
```

#### Constant `BUS_ADRERR`

```rust
pub const BUS_ADRERR: u32 = 2;
```

#### Constant `BUS_OBJERR`

```rust
pub const BUS_OBJERR: u32 = 3;
```

#### Constant `BUS_MCEERR_AR`

```rust
pub const BUS_MCEERR_AR: u32 = 4;
```

#### Constant `BUS_MCEERR_AO`

```rust
pub const BUS_MCEERR_AO: u32 = 5;
```

#### Constant `NSIGBUS`

```rust
pub const NSIGBUS: u32 = 5;
```

#### Constant `TRAP_BRKPT`

```rust
pub const TRAP_BRKPT: u32 = 1;
```

#### Constant `TRAP_TRACE`

```rust
pub const TRAP_TRACE: u32 = 2;
```

#### Constant `TRAP_BRANCH`

```rust
pub const TRAP_BRANCH: u32 = 3;
```

#### Constant `TRAP_HWBKPT`

```rust
pub const TRAP_HWBKPT: u32 = 4;
```

#### Constant `TRAP_UNK`

```rust
pub const TRAP_UNK: u32 = 5;
```

#### Constant `TRAP_PERF`

```rust
pub const TRAP_PERF: u32 = 6;
```

#### Constant `NSIGTRAP`

```rust
pub const NSIGTRAP: u32 = 6;
```

#### Constant `TRAP_PERF_FLAG_ASYNC`

```rust
pub const TRAP_PERF_FLAG_ASYNC: u32 = 1;
```

#### Constant `CLD_EXITED`

```rust
pub const CLD_EXITED: u32 = 1;
```

#### Constant `CLD_KILLED`

```rust
pub const CLD_KILLED: u32 = 2;
```

#### Constant `CLD_DUMPED`

```rust
pub const CLD_DUMPED: u32 = 3;
```

#### Constant `CLD_TRAPPED`

```rust
pub const CLD_TRAPPED: u32 = 4;
```

#### Constant `CLD_STOPPED`

```rust
pub const CLD_STOPPED: u32 = 5;
```

#### Constant `CLD_CONTINUED`

```rust
pub const CLD_CONTINUED: u32 = 6;
```

#### Constant `NSIGCHLD`

```rust
pub const NSIGCHLD: u32 = 6;
```

#### Constant `POLL_IN`

```rust
pub const POLL_IN: u32 = 1;
```

#### Constant `POLL_OUT`

```rust
pub const POLL_OUT: u32 = 2;
```

#### Constant `POLL_MSG`

```rust
pub const POLL_MSG: u32 = 3;
```

#### Constant `POLL_ERR`

```rust
pub const POLL_ERR: u32 = 4;
```

#### Constant `POLL_PRI`

```rust
pub const POLL_PRI: u32 = 5;
```

#### Constant `POLL_HUP`

```rust
pub const POLL_HUP: u32 = 6;
```

#### Constant `NSIGPOLL`

```rust
pub const NSIGPOLL: u32 = 6;
```

#### Constant `SYS_SECCOMP`

```rust
pub const SYS_SECCOMP: u32 = 1;
```

#### Constant `SYS_USER_DISPATCH`

```rust
pub const SYS_USER_DISPATCH: u32 = 2;
```

#### Constant `NSIGSYS`

```rust
pub const NSIGSYS: u32 = 2;
```

#### Constant `EMT_TAGOVF`

```rust
pub const EMT_TAGOVF: u32 = 1;
```

#### Constant `NSIGEMT`

```rust
pub const NSIGEMT: u32 = 1;
```

#### Constant `SIGEV_SIGNAL`

```rust
pub const SIGEV_SIGNAL: u32 = 0;
```

#### Constant `SIGEV_NONE`

```rust
pub const SIGEV_NONE: u32 = 1;
```

#### Constant `SIGEV_THREAD`

```rust
pub const SIGEV_THREAD: u32 = 2;
```

#### Constant `SIGEV_THREAD_ID`

```rust
pub const SIGEV_THREAD_ID: u32 = 4;
```

#### Constant `SIGEV_MAX_SIZE`

```rust
pub const SIGEV_MAX_SIZE: u32 = 64;
```

#### Constant `SS_ONSTACK`

```rust
pub const SS_ONSTACK: u32 = 1;
```

#### Constant `SS_DISABLE`

```rust
pub const SS_DISABLE: u32 = 2;
```

#### Constant `SS_AUTODISARM`

```rust
pub const SS_AUTODISARM: u32 = 2147483648;
```

#### Constant `SS_FLAG_BITS`

```rust
pub const SS_FLAG_BITS: u32 = 2147483648;
```

#### Constant `S_IFMT`

```rust
pub const S_IFMT: u32 = 61440;
```

#### Constant `S_IFSOCK`

```rust
pub const S_IFSOCK: u32 = 49152;
```

#### Constant `S_IFLNK`

```rust
pub const S_IFLNK: u32 = 40960;
```

#### Constant `S_IFREG`

```rust
pub const S_IFREG: u32 = 32768;
```

#### Constant `S_IFBLK`

```rust
pub const S_IFBLK: u32 = 24576;
```

#### Constant `S_IFDIR`

```rust
pub const S_IFDIR: u32 = 16384;
```

#### Constant `S_IFCHR`

```rust
pub const S_IFCHR: u32 = 8192;
```

#### Constant `S_IFIFO`

```rust
pub const S_IFIFO: u32 = 4096;
```

#### Constant `S_ISUID`

```rust
pub const S_ISUID: u32 = 2048;
```

#### Constant `S_ISGID`

```rust
pub const S_ISGID: u32 = 1024;
```

#### Constant `S_ISVTX`

```rust
pub const S_ISVTX: u32 = 512;
```

#### Constant `S_IRWXU`

```rust
pub const S_IRWXU: u32 = 448;
```

#### Constant `S_IRUSR`

```rust
pub const S_IRUSR: u32 = 256;
```

#### Constant `S_IWUSR`

```rust
pub const S_IWUSR: u32 = 128;
```

#### Constant `S_IXUSR`

```rust
pub const S_IXUSR: u32 = 64;
```

#### Constant `S_IRWXG`

```rust
pub const S_IRWXG: u32 = 56;
```

#### Constant `S_IRGRP`

```rust
pub const S_IRGRP: u32 = 32;
```

#### Constant `S_IWGRP`

```rust
pub const S_IWGRP: u32 = 16;
```

#### Constant `S_IXGRP`

```rust
pub const S_IXGRP: u32 = 8;
```

#### Constant `S_IRWXO`

```rust
pub const S_IRWXO: u32 = 7;
```

#### Constant `S_IROTH`

```rust
pub const S_IROTH: u32 = 4;
```

#### Constant `S_IWOTH`

```rust
pub const S_IWOTH: u32 = 2;
```

#### Constant `S_IXOTH`

```rust
pub const S_IXOTH: u32 = 1;
```

#### Constant `STATX_TYPE`

```rust
pub const STATX_TYPE: u32 = 1;
```

#### Constant `STATX_MODE`

```rust
pub const STATX_MODE: u32 = 2;
```

#### Constant `STATX_NLINK`

```rust
pub const STATX_NLINK: u32 = 4;
```

#### Constant `STATX_UID`

```rust
pub const STATX_UID: u32 = 8;
```

#### Constant `STATX_GID`

```rust
pub const STATX_GID: u32 = 16;
```

#### Constant `STATX_ATIME`

```rust
pub const STATX_ATIME: u32 = 32;
```

#### Constant `STATX_MTIME`

```rust
pub const STATX_MTIME: u32 = 64;
```

#### Constant `STATX_CTIME`

```rust
pub const STATX_CTIME: u32 = 128;
```

#### Constant `STATX_INO`

```rust
pub const STATX_INO: u32 = 256;
```

#### Constant `STATX_SIZE`

```rust
pub const STATX_SIZE: u32 = 512;
```

#### Constant `STATX_BLOCKS`

```rust
pub const STATX_BLOCKS: u32 = 1024;
```

#### Constant `STATX_BASIC_STATS`

```rust
pub const STATX_BASIC_STATS: u32 = 2047;
```

#### Constant `STATX_BTIME`

```rust
pub const STATX_BTIME: u32 = 2048;
```

#### Constant `STATX_MNT_ID`

```rust
pub const STATX_MNT_ID: u32 = 4096;
```

#### Constant `STATX_DIOALIGN`

```rust
pub const STATX_DIOALIGN: u32 = 8192;
```

#### Constant `STATX_MNT_ID_UNIQUE`

```rust
pub const STATX_MNT_ID_UNIQUE: u32 = 16384;
```

#### Constant `STATX_SUBVOL`

```rust
pub const STATX_SUBVOL: u32 = 32768;
```

#### Constant `STATX_WRITE_ATOMIC`

```rust
pub const STATX_WRITE_ATOMIC: u32 = 65536;
```

#### Constant `STATX__RESERVED`

```rust
pub const STATX__RESERVED: u32 = 2147483648;
```

#### Constant `STATX_ALL`

```rust
pub const STATX_ALL: u32 = 4095;
```

#### Constant `STATX_ATTR_COMPRESSED`

```rust
pub const STATX_ATTR_COMPRESSED: u32 = 4;
```

#### Constant `STATX_ATTR_IMMUTABLE`

```rust
pub const STATX_ATTR_IMMUTABLE: u32 = 16;
```

#### Constant `STATX_ATTR_APPEND`

```rust
pub const STATX_ATTR_APPEND: u32 = 32;
```

#### Constant `STATX_ATTR_NODUMP`

```rust
pub const STATX_ATTR_NODUMP: u32 = 64;
```

#### Constant `STATX_ATTR_ENCRYPTED`

```rust
pub const STATX_ATTR_ENCRYPTED: u32 = 2048;
```

#### Constant `STATX_ATTR_AUTOMOUNT`

```rust
pub const STATX_ATTR_AUTOMOUNT: u32 = 4096;
```

#### Constant `STATX_ATTR_MOUNT_ROOT`

```rust
pub const STATX_ATTR_MOUNT_ROOT: u32 = 8192;
```

#### Constant `STATX_ATTR_VERITY`

```rust
pub const STATX_ATTR_VERITY: u32 = 1048576;
```

#### Constant `STATX_ATTR_DAX`

```rust
pub const STATX_ATTR_DAX: u32 = 2097152;
```

#### Constant `STATX_ATTR_WRITE_ATOMIC`

```rust
pub const STATX_ATTR_WRITE_ATOMIC: u32 = 4194304;
```

#### Constant `IGNBRK`

```rust
pub const IGNBRK: u32 = 1;
```

#### Constant `BRKINT`

```rust
pub const BRKINT: u32 = 2;
```

#### Constant `IGNPAR`

```rust
pub const IGNPAR: u32 = 4;
```

#### Constant `PARMRK`

```rust
pub const PARMRK: u32 = 8;
```

#### Constant `INPCK`

```rust
pub const INPCK: u32 = 16;
```

#### Constant `ISTRIP`

```rust
pub const ISTRIP: u32 = 32;
```

#### Constant `INLCR`

```rust
pub const INLCR: u32 = 64;
```

#### Constant `IGNCR`

```rust
pub const IGNCR: u32 = 128;
```

#### Constant `ICRNL`

```rust
pub const ICRNL: u32 = 256;
```

#### Constant `IXANY`

```rust
pub const IXANY: u32 = 2048;
```

#### Constant `OPOST`

```rust
pub const OPOST: u32 = 1;
```

#### Constant `OCRNL`

```rust
pub const OCRNL: u32 = 8;
```

#### Constant `ONOCR`

```rust
pub const ONOCR: u32 = 16;
```

#### Constant `ONLRET`

```rust
pub const ONLRET: u32 = 32;
```

#### Constant `OFILL`

```rust
pub const OFILL: u32 = 64;
```

#### Constant `OFDEL`

```rust
pub const OFDEL: u32 = 128;
```

#### Constant `B0`

```rust
pub const B0: u32 = 0;
```

#### Constant `B50`

```rust
pub const B50: u32 = 1;
```

#### Constant `B75`

```rust
pub const B75: u32 = 2;
```

#### Constant `B110`

```rust
pub const B110: u32 = 3;
```

#### Constant `B134`

```rust
pub const B134: u32 = 4;
```

#### Constant `B150`

```rust
pub const B150: u32 = 5;
```

#### Constant `B200`

```rust
pub const B200: u32 = 6;
```

#### Constant `B300`

```rust
pub const B300: u32 = 7;
```

#### Constant `B600`

```rust
pub const B600: u32 = 8;
```

#### Constant `B1200`

```rust
pub const B1200: u32 = 9;
```

#### Constant `B1800`

```rust
pub const B1800: u32 = 10;
```

#### Constant `B2400`

```rust
pub const B2400: u32 = 11;
```

#### Constant `B4800`

```rust
pub const B4800: u32 = 12;
```

#### Constant `B9600`

```rust
pub const B9600: u32 = 13;
```

#### Constant `B19200`

```rust
pub const B19200: u32 = 14;
```

#### Constant `B38400`

```rust
pub const B38400: u32 = 15;
```

#### Constant `EXTA`

```rust
pub const EXTA: u32 = 14;
```

#### Constant `EXTB`

```rust
pub const EXTB: u32 = 15;
```

#### Constant `ADDRB`

```rust
pub const ADDRB: u32 = 536870912;
```

#### Constant `CMSPAR`

```rust
pub const CMSPAR: u32 = 1073741824;
```

#### Constant `CRTSCTS`

```rust
pub const CRTSCTS: u32 = 2147483648;
```

#### Constant `IBSHIFT`

```rust
pub const IBSHIFT: u32 = 16;
```

#### Constant `TCOOFF`

```rust
pub const TCOOFF: u32 = 0;
```

#### Constant `TCOON`

```rust
pub const TCOON: u32 = 1;
```

#### Constant `TCIOFF`

```rust
pub const TCIOFF: u32 = 2;
```

#### Constant `TCION`

```rust
pub const TCION: u32 = 3;
```

#### Constant `TCIFLUSH`

```rust
pub const TCIFLUSH: u32 = 0;
```

#### Constant `TCOFLUSH`

```rust
pub const TCOFLUSH: u32 = 1;
```

#### Constant `TCIOFLUSH`

```rust
pub const TCIOFLUSH: u32 = 2;
```

#### Constant `NCCS`

```rust
pub const NCCS: u32 = 19;
```

#### Constant `VINTR`

```rust
pub const VINTR: u32 = 0;
```

#### Constant `VQUIT`

```rust
pub const VQUIT: u32 = 1;
```

#### Constant `VERASE`

```rust
pub const VERASE: u32 = 2;
```

#### Constant `VKILL`

```rust
pub const VKILL: u32 = 3;
```

#### Constant `VEOF`

```rust
pub const VEOF: u32 = 4;
```

#### Constant `VTIME`

```rust
pub const VTIME: u32 = 5;
```

#### Constant `VMIN`

```rust
pub const VMIN: u32 = 6;
```

#### Constant `VSWTC`

```rust
pub const VSWTC: u32 = 7;
```

#### Constant `VSTART`

```rust
pub const VSTART: u32 = 8;
```

#### Constant `VSTOP`

```rust
pub const VSTOP: u32 = 9;
```

#### Constant `VSUSP`

```rust
pub const VSUSP: u32 = 10;
```

#### Constant `VEOL`

```rust
pub const VEOL: u32 = 11;
```

#### Constant `VREPRINT`

```rust
pub const VREPRINT: u32 = 12;
```

#### Constant `VDISCARD`

```rust
pub const VDISCARD: u32 = 13;
```

#### Constant `VWERASE`

```rust
pub const VWERASE: u32 = 14;
```

#### Constant `VLNEXT`

```rust
pub const VLNEXT: u32 = 15;
```

#### Constant `VEOL2`

```rust
pub const VEOL2: u32 = 16;
```

#### Constant `IUCLC`

```rust
pub const IUCLC: u32 = 512;
```

#### Constant `IXON`

```rust
pub const IXON: u32 = 1024;
```

#### Constant `IXOFF`

```rust
pub const IXOFF: u32 = 4096;
```

#### Constant `IMAXBEL`

```rust
pub const IMAXBEL: u32 = 8192;
```

#### Constant `IUTF8`

```rust
pub const IUTF8: u32 = 16384;
```

#### Constant `OLCUC`

```rust
pub const OLCUC: u32 = 2;
```

#### Constant `ONLCR`

```rust
pub const ONLCR: u32 = 4;
```

#### Constant `NLDLY`

```rust
pub const NLDLY: u32 = 256;
```

#### Constant `NL0`

```rust
pub const NL0: u32 = 0;
```

#### Constant `NL1`

```rust
pub const NL1: u32 = 256;
```

#### Constant `CRDLY`

```rust
pub const CRDLY: u32 = 1536;
```

#### Constant `CR0`

```rust
pub const CR0: u32 = 0;
```

#### Constant `CR1`

```rust
pub const CR1: u32 = 512;
```

#### Constant `CR2`

```rust
pub const CR2: u32 = 1024;
```

#### Constant `CR3`

```rust
pub const CR3: u32 = 1536;
```

#### Constant `TABDLY`

```rust
pub const TABDLY: u32 = 6144;
```

#### Constant `TAB0`

```rust
pub const TAB0: u32 = 0;
```

#### Constant `TAB1`

```rust
pub const TAB1: u32 = 2048;
```

#### Constant `TAB2`

```rust
pub const TAB2: u32 = 4096;
```

#### Constant `TAB3`

```rust
pub const TAB3: u32 = 6144;
```

#### Constant `XTABS`

```rust
pub const XTABS: u32 = 6144;
```

#### Constant `BSDLY`

```rust
pub const BSDLY: u32 = 8192;
```

#### Constant `BS0`

```rust
pub const BS0: u32 = 0;
```

#### Constant `BS1`

```rust
pub const BS1: u32 = 8192;
```

#### Constant `VTDLY`

```rust
pub const VTDLY: u32 = 16384;
```

#### Constant `VT0`

```rust
pub const VT0: u32 = 0;
```

#### Constant `VT1`

```rust
pub const VT1: u32 = 16384;
```

#### Constant `FFDLY`

```rust
pub const FFDLY: u32 = 32768;
```

#### Constant `FF0`

```rust
pub const FF0: u32 = 0;
```

#### Constant `FF1`

```rust
pub const FF1: u32 = 32768;
```

#### Constant `CBAUD`

```rust
pub const CBAUD: u32 = 4111;
```

#### Constant `CSIZE`

```rust
pub const CSIZE: u32 = 48;
```

#### Constant `CS5`

```rust
pub const CS5: u32 = 0;
```

#### Constant `CS6`

```rust
pub const CS6: u32 = 16;
```

#### Constant `CS7`

```rust
pub const CS7: u32 = 32;
```

#### Constant `CS8`

```rust
pub const CS8: u32 = 48;
```

#### Constant `CSTOPB`

```rust
pub const CSTOPB: u32 = 64;
```

#### Constant `CREAD`

```rust
pub const CREAD: u32 = 128;
```

#### Constant `PARENB`

```rust
pub const PARENB: u32 = 256;
```

#### Constant `PARODD`

```rust
pub const PARODD: u32 = 512;
```

#### Constant `HUPCL`

```rust
pub const HUPCL: u32 = 1024;
```

#### Constant `CLOCAL`

```rust
pub const CLOCAL: u32 = 2048;
```

#### Constant `CBAUDEX`

```rust
pub const CBAUDEX: u32 = 4096;
```

#### Constant `BOTHER`

```rust
pub const BOTHER: u32 = 4096;
```

#### Constant `B57600`

```rust
pub const B57600: u32 = 4097;
```

#### Constant `B115200`

```rust
pub const B115200: u32 = 4098;
```

#### Constant `B230400`

```rust
pub const B230400: u32 = 4099;
```

#### Constant `B460800`

```rust
pub const B460800: u32 = 4100;
```

#### Constant `B500000`

```rust
pub const B500000: u32 = 4101;
```

#### Constant `B576000`

```rust
pub const B576000: u32 = 4102;
```

#### Constant `B921600`

```rust
pub const B921600: u32 = 4103;
```

#### Constant `B1000000`

```rust
pub const B1000000: u32 = 4104;
```

#### Constant `B1152000`

```rust
pub const B1152000: u32 = 4105;
```

#### Constant `B1500000`

```rust
pub const B1500000: u32 = 4106;
```

#### Constant `B2000000`

```rust
pub const B2000000: u32 = 4107;
```

#### Constant `B2500000`

```rust
pub const B2500000: u32 = 4108;
```

#### Constant `B3000000`

```rust
pub const B3000000: u32 = 4109;
```

#### Constant `B3500000`

```rust
pub const B3500000: u32 = 4110;
```

#### Constant `B4000000`

```rust
pub const B4000000: u32 = 4111;
```

#### Constant `CIBAUD`

```rust
pub const CIBAUD: u32 = 269418496;
```

#### Constant `ISIG`

```rust
pub const ISIG: u32 = 1;
```

#### Constant `ICANON`

```rust
pub const ICANON: u32 = 2;
```

#### Constant `XCASE`

```rust
pub const XCASE: u32 = 4;
```

#### Constant `ECHO`

```rust
pub const ECHO: u32 = 8;
```

#### Constant `ECHOE`

```rust
pub const ECHOE: u32 = 16;
```

#### Constant `ECHOK`

```rust
pub const ECHOK: u32 = 32;
```

#### Constant `ECHONL`

```rust
pub const ECHONL: u32 = 64;
```

#### Constant `NOFLSH`

```rust
pub const NOFLSH: u32 = 128;
```

#### Constant `TOSTOP`

```rust
pub const TOSTOP: u32 = 256;
```

#### Constant `ECHOCTL`

```rust
pub const ECHOCTL: u32 = 512;
```

#### Constant `ECHOPRT`

```rust
pub const ECHOPRT: u32 = 1024;
```

#### Constant `ECHOKE`

```rust
pub const ECHOKE: u32 = 2048;
```

#### Constant `FLUSHO`

```rust
pub const FLUSHO: u32 = 4096;
```

#### Constant `PENDIN`

```rust
pub const PENDIN: u32 = 16384;
```

#### Constant `IEXTEN`

```rust
pub const IEXTEN: u32 = 32768;
```

#### Constant `EXTPROC`

```rust
pub const EXTPROC: u32 = 65536;
```

#### Constant `TCSANOW`

```rust
pub const TCSANOW: u32 = 0;
```

#### Constant `TCSADRAIN`

```rust
pub const TCSADRAIN: u32 = 1;
```

#### Constant `TCSAFLUSH`

```rust
pub const TCSAFLUSH: u32 = 2;
```

#### Constant `TIOCPKT_DATA`

```rust
pub const TIOCPKT_DATA: u32 = 0;
```

#### Constant `TIOCPKT_FLUSHREAD`

```rust
pub const TIOCPKT_FLUSHREAD: u32 = 1;
```

#### Constant `TIOCPKT_FLUSHWRITE`

```rust
pub const TIOCPKT_FLUSHWRITE: u32 = 2;
```

#### Constant `TIOCPKT_STOP`

```rust
pub const TIOCPKT_STOP: u32 = 4;
```

#### Constant `TIOCPKT_START`

```rust
pub const TIOCPKT_START: u32 = 8;
```

#### Constant `TIOCPKT_NOSTOP`

```rust
pub const TIOCPKT_NOSTOP: u32 = 16;
```

#### Constant `TIOCPKT_DOSTOP`

```rust
pub const TIOCPKT_DOSTOP: u32 = 32;
```

#### Constant `TIOCPKT_IOCTL`

```rust
pub const TIOCPKT_IOCTL: u32 = 64;
```

#### Constant `TIOCSER_TEMT`

```rust
pub const TIOCSER_TEMT: u32 = 1;
```

#### Constant `NCC`

```rust
pub const NCC: u32 = 8;
```

#### Constant `TIOCM_LE`

```rust
pub const TIOCM_LE: u32 = 1;
```

#### Constant `TIOCM_DTR`

```rust
pub const TIOCM_DTR: u32 = 2;
```

#### Constant `TIOCM_RTS`

```rust
pub const TIOCM_RTS: u32 = 4;
```

#### Constant `TIOCM_ST`

```rust
pub const TIOCM_ST: u32 = 8;
```

#### Constant `TIOCM_SR`

```rust
pub const TIOCM_SR: u32 = 16;
```

#### Constant `TIOCM_CTS`

```rust
pub const TIOCM_CTS: u32 = 32;
```

#### Constant `TIOCM_CAR`

```rust
pub const TIOCM_CAR: u32 = 64;
```

#### Constant `TIOCM_RNG`

```rust
pub const TIOCM_RNG: u32 = 128;
```

#### Constant `TIOCM_DSR`

```rust
pub const TIOCM_DSR: u32 = 256;
```

#### Constant `TIOCM_CD`

```rust
pub const TIOCM_CD: u32 = 64;
```

#### Constant `TIOCM_RI`

```rust
pub const TIOCM_RI: u32 = 128;
```

#### Constant `TIOCM_OUT1`

```rust
pub const TIOCM_OUT1: u32 = 8192;
```

#### Constant `TIOCM_OUT2`

```rust
pub const TIOCM_OUT2: u32 = 16384;
```

#### Constant `TIOCM_LOOP`

```rust
pub const TIOCM_LOOP: u32 = 32768;
```

#### Constant `ITIMER_REAL`

```rust
pub const ITIMER_REAL: u32 = 0;
```

#### Constant `ITIMER_VIRTUAL`

```rust
pub const ITIMER_VIRTUAL: u32 = 1;
```

#### Constant `ITIMER_PROF`

```rust
pub const ITIMER_PROF: u32 = 2;
```

#### Constant `CLOCK_REALTIME`

```rust
pub const CLOCK_REALTIME: u32 = 0;
```

#### Constant `CLOCK_MONOTONIC`

```rust
pub const CLOCK_MONOTONIC: u32 = 1;
```

#### Constant `CLOCK_PROCESS_CPUTIME_ID`

```rust
pub const CLOCK_PROCESS_CPUTIME_ID: u32 = 2;
```

#### Constant `CLOCK_THREAD_CPUTIME_ID`

```rust
pub const CLOCK_THREAD_CPUTIME_ID: u32 = 3;
```

#### Constant `CLOCK_MONOTONIC_RAW`

```rust
pub const CLOCK_MONOTONIC_RAW: u32 = 4;
```

#### Constant `CLOCK_REALTIME_COARSE`

```rust
pub const CLOCK_REALTIME_COARSE: u32 = 5;
```

#### Constant `CLOCK_MONOTONIC_COARSE`

```rust
pub const CLOCK_MONOTONIC_COARSE: u32 = 6;
```

#### Constant `CLOCK_BOOTTIME`

```rust
pub const CLOCK_BOOTTIME: u32 = 7;
```

#### Constant `CLOCK_REALTIME_ALARM`

```rust
pub const CLOCK_REALTIME_ALARM: u32 = 8;
```

#### Constant `CLOCK_BOOTTIME_ALARM`

```rust
pub const CLOCK_BOOTTIME_ALARM: u32 = 9;
```

#### Constant `CLOCK_SGI_CYCLE`

```rust
pub const CLOCK_SGI_CYCLE: u32 = 10;
```

#### Constant `CLOCK_TAI`

```rust
pub const CLOCK_TAI: u32 = 11;
```

#### Constant `MAX_CLOCKS`

```rust
pub const MAX_CLOCKS: u32 = 16;
```

#### Constant `CLOCKS_MASK`

```rust
pub const CLOCKS_MASK: u32 = 1;
```

#### Constant `CLOCKS_MONO`

```rust
pub const CLOCKS_MONO: u32 = 1;
```

#### Constant `TIMER_ABSTIME`

```rust
pub const TIMER_ABSTIME: u32 = 1;
```

#### Constant `UIO_FASTIOV`

```rust
pub const UIO_FASTIOV: u32 = 8;
```

#### Constant `UIO_MAXIOV`

```rust
pub const UIO_MAXIOV: u32 = 1024;
```

#### Constant `__NR_io_setup`

```rust
pub const __NR_io_setup: u32 = 0;
```

#### Constant `__NR_io_destroy`

```rust
pub const __NR_io_destroy: u32 = 1;
```

#### Constant `__NR_io_submit`

```rust
pub const __NR_io_submit: u32 = 2;
```

#### Constant `__NR_io_cancel`

```rust
pub const __NR_io_cancel: u32 = 3;
```

#### Constant `__NR_io_getevents`

```rust
pub const __NR_io_getevents: u32 = 4;
```

#### Constant `__NR_setxattr`

```rust
pub const __NR_setxattr: u32 = 5;
```

#### Constant `__NR_lsetxattr`

```rust
pub const __NR_lsetxattr: u32 = 6;
```

#### Constant `__NR_fsetxattr`

```rust
pub const __NR_fsetxattr: u32 = 7;
```

#### Constant `__NR_getxattr`

```rust
pub const __NR_getxattr: u32 = 8;
```

#### Constant `__NR_lgetxattr`

```rust
pub const __NR_lgetxattr: u32 = 9;
```

#### Constant `__NR_fgetxattr`

```rust
pub const __NR_fgetxattr: u32 = 10;
```

#### Constant `__NR_listxattr`

```rust
pub const __NR_listxattr: u32 = 11;
```

#### Constant `__NR_llistxattr`

```rust
pub const __NR_llistxattr: u32 = 12;
```

#### Constant `__NR_flistxattr`

```rust
pub const __NR_flistxattr: u32 = 13;
```

#### Constant `__NR_removexattr`

```rust
pub const __NR_removexattr: u32 = 14;
```

#### Constant `__NR_lremovexattr`

```rust
pub const __NR_lremovexattr: u32 = 15;
```

#### Constant `__NR_fremovexattr`

```rust
pub const __NR_fremovexattr: u32 = 16;
```

#### Constant `__NR_getcwd`

```rust
pub const __NR_getcwd: u32 = 17;
```

#### Constant `__NR_lookup_dcookie`

```rust
pub const __NR_lookup_dcookie: u32 = 18;
```

#### Constant `__NR_eventfd2`

```rust
pub const __NR_eventfd2: u32 = 19;
```

#### Constant `__NR_epoll_create1`

```rust
pub const __NR_epoll_create1: u32 = 20;
```

#### Constant `__NR_epoll_ctl`

```rust
pub const __NR_epoll_ctl: u32 = 21;
```

#### Constant `__NR_epoll_pwait`

```rust
pub const __NR_epoll_pwait: u32 = 22;
```

#### Constant `__NR_dup`

```rust
pub const __NR_dup: u32 = 23;
```

#### Constant `__NR_dup3`

```rust
pub const __NR_dup3: u32 = 24;
```

#### Constant `__NR_fcntl`

```rust
pub const __NR_fcntl: u32 = 25;
```

#### Constant `__NR_inotify_init1`

```rust
pub const __NR_inotify_init1: u32 = 26;
```

#### Constant `__NR_inotify_add_watch`

```rust
pub const __NR_inotify_add_watch: u32 = 27;
```

#### Constant `__NR_inotify_rm_watch`

```rust
pub const __NR_inotify_rm_watch: u32 = 28;
```

#### Constant `__NR_ioctl`

```rust
pub const __NR_ioctl: u32 = 29;
```

#### Constant `__NR_ioprio_set`

```rust
pub const __NR_ioprio_set: u32 = 30;
```

#### Constant `__NR_ioprio_get`

```rust
pub const __NR_ioprio_get: u32 = 31;
```

#### Constant `__NR_flock`

```rust
pub const __NR_flock: u32 = 32;
```

#### Constant `__NR_mknodat`

```rust
pub const __NR_mknodat: u32 = 33;
```

#### Constant `__NR_mkdirat`

```rust
pub const __NR_mkdirat: u32 = 34;
```

#### Constant `__NR_unlinkat`

```rust
pub const __NR_unlinkat: u32 = 35;
```

#### Constant `__NR_symlinkat`

```rust
pub const __NR_symlinkat: u32 = 36;
```

#### Constant `__NR_linkat`

```rust
pub const __NR_linkat: u32 = 37;
```

#### Constant `__NR_renameat`

```rust
pub const __NR_renameat: u32 = 38;
```

#### Constant `__NR_umount2`

```rust
pub const __NR_umount2: u32 = 39;
```

#### Constant `__NR_mount`

```rust
pub const __NR_mount: u32 = 40;
```

#### Constant `__NR_pivot_root`

```rust
pub const __NR_pivot_root: u32 = 41;
```

#### Constant `__NR_nfsservctl`

```rust
pub const __NR_nfsservctl: u32 = 42;
```

#### Constant `__NR_statfs`

```rust
pub const __NR_statfs: u32 = 43;
```

#### Constant `__NR_fstatfs`

```rust
pub const __NR_fstatfs: u32 = 44;
```

#### Constant `__NR_truncate`

```rust
pub const __NR_truncate: u32 = 45;
```

#### Constant `__NR_ftruncate`

```rust
pub const __NR_ftruncate: u32 = 46;
```

#### Constant `__NR_fallocate`

```rust
pub const __NR_fallocate: u32 = 47;
```

#### Constant `__NR_faccessat`

```rust
pub const __NR_faccessat: u32 = 48;
```

#### Constant `__NR_chdir`

```rust
pub const __NR_chdir: u32 = 49;
```

#### Constant `__NR_fchdir`

```rust
pub const __NR_fchdir: u32 = 50;
```

#### Constant `__NR_chroot`

```rust
pub const __NR_chroot: u32 = 51;
```

#### Constant `__NR_fchmod`

```rust
pub const __NR_fchmod: u32 = 52;
```

#### Constant `__NR_fchmodat`

```rust
pub const __NR_fchmodat: u32 = 53;
```

#### Constant `__NR_fchownat`

```rust
pub const __NR_fchownat: u32 = 54;
```

#### Constant `__NR_fchown`

```rust
pub const __NR_fchown: u32 = 55;
```

#### Constant `__NR_openat`

```rust
pub const __NR_openat: u32 = 56;
```

#### Constant `__NR_close`

```rust
pub const __NR_close: u32 = 57;
```

#### Constant `__NR_vhangup`

```rust
pub const __NR_vhangup: u32 = 58;
```

#### Constant `__NR_pipe2`

```rust
pub const __NR_pipe2: u32 = 59;
```

#### Constant `__NR_quotactl`

```rust
pub const __NR_quotactl: u32 = 60;
```

#### Constant `__NR_getdents64`

```rust
pub const __NR_getdents64: u32 = 61;
```

#### Constant `__NR_lseek`

```rust
pub const __NR_lseek: u32 = 62;
```

#### Constant `__NR_read`

```rust
pub const __NR_read: u32 = 63;
```

#### Constant `__NR_write`

```rust
pub const __NR_write: u32 = 64;
```

#### Constant `__NR_readv`

```rust
pub const __NR_readv: u32 = 65;
```

#### Constant `__NR_writev`

```rust
pub const __NR_writev: u32 = 66;
```

#### Constant `__NR_pread64`

```rust
pub const __NR_pread64: u32 = 67;
```

#### Constant `__NR_pwrite64`

```rust
pub const __NR_pwrite64: u32 = 68;
```

#### Constant `__NR_preadv`

```rust
pub const __NR_preadv: u32 = 69;
```

#### Constant `__NR_pwritev`

```rust
pub const __NR_pwritev: u32 = 70;
```

#### Constant `__NR_sendfile`

```rust
pub const __NR_sendfile: u32 = 71;
```

#### Constant `__NR_pselect6`

```rust
pub const __NR_pselect6: u32 = 72;
```

#### Constant `__NR_ppoll`

```rust
pub const __NR_ppoll: u32 = 73;
```

#### Constant `__NR_signalfd4`

```rust
pub const __NR_signalfd4: u32 = 74;
```

#### Constant `__NR_vmsplice`

```rust
pub const __NR_vmsplice: u32 = 75;
```

#### Constant `__NR_splice`

```rust
pub const __NR_splice: u32 = 76;
```

#### Constant `__NR_tee`

```rust
pub const __NR_tee: u32 = 77;
```

#### Constant `__NR_readlinkat`

```rust
pub const __NR_readlinkat: u32 = 78;
```

#### Constant `__NR_newfstatat`

```rust
pub const __NR_newfstatat: u32 = 79;
```

#### Constant `__NR_fstat`

```rust
pub const __NR_fstat: u32 = 80;
```

#### Constant `__NR_sync`

```rust
pub const __NR_sync: u32 = 81;
```

#### Constant `__NR_fsync`

```rust
pub const __NR_fsync: u32 = 82;
```

#### Constant `__NR_fdatasync`

```rust
pub const __NR_fdatasync: u32 = 83;
```

#### Constant `__NR_sync_file_range`

```rust
pub const __NR_sync_file_range: u32 = 84;
```

#### Constant `__NR_timerfd_create`

```rust
pub const __NR_timerfd_create: u32 = 85;
```

#### Constant `__NR_timerfd_settime`

```rust
pub const __NR_timerfd_settime: u32 = 86;
```

#### Constant `__NR_timerfd_gettime`

```rust
pub const __NR_timerfd_gettime: u32 = 87;
```

#### Constant `__NR_utimensat`

```rust
pub const __NR_utimensat: u32 = 88;
```

#### Constant `__NR_acct`

```rust
pub const __NR_acct: u32 = 89;
```

#### Constant `__NR_capget`

```rust
pub const __NR_capget: u32 = 90;
```

#### Constant `__NR_capset`

```rust
pub const __NR_capset: u32 = 91;
```

#### Constant `__NR_personality`

```rust
pub const __NR_personality: u32 = 92;
```

#### Constant `__NR_exit`

```rust
pub const __NR_exit: u32 = 93;
```

#### Constant `__NR_exit_group`

```rust
pub const __NR_exit_group: u32 = 94;
```

#### Constant `__NR_waitid`

```rust
pub const __NR_waitid: u32 = 95;
```

#### Constant `__NR_set_tid_address`

```rust
pub const __NR_set_tid_address: u32 = 96;
```

#### Constant `__NR_unshare`

```rust
pub const __NR_unshare: u32 = 97;
```

#### Constant `__NR_futex`

```rust
pub const __NR_futex: u32 = 98;
```

#### Constant `__NR_set_robust_list`

```rust
pub const __NR_set_robust_list: u32 = 99;
```

#### Constant `__NR_get_robust_list`

```rust
pub const __NR_get_robust_list: u32 = 100;
```

#### Constant `__NR_nanosleep`

```rust
pub const __NR_nanosleep: u32 = 101;
```

#### Constant `__NR_getitimer`

```rust
pub const __NR_getitimer: u32 = 102;
```

#### Constant `__NR_setitimer`

```rust
pub const __NR_setitimer: u32 = 103;
```

#### Constant `__NR_kexec_load`

```rust
pub const __NR_kexec_load: u32 = 104;
```

#### Constant `__NR_init_module`

```rust
pub const __NR_init_module: u32 = 105;
```

#### Constant `__NR_delete_module`

```rust
pub const __NR_delete_module: u32 = 106;
```

#### Constant `__NR_timer_create`

```rust
pub const __NR_timer_create: u32 = 107;
```

#### Constant `__NR_timer_gettime`

```rust
pub const __NR_timer_gettime: u32 = 108;
```

#### Constant `__NR_timer_getoverrun`

```rust
pub const __NR_timer_getoverrun: u32 = 109;
```

#### Constant `__NR_timer_settime`

```rust
pub const __NR_timer_settime: u32 = 110;
```

#### Constant `__NR_timer_delete`

```rust
pub const __NR_timer_delete: u32 = 111;
```

#### Constant `__NR_clock_settime`

```rust
pub const __NR_clock_settime: u32 = 112;
```

#### Constant `__NR_clock_gettime`

```rust
pub const __NR_clock_gettime: u32 = 113;
```

#### Constant `__NR_clock_getres`

```rust
pub const __NR_clock_getres: u32 = 114;
```

#### Constant `__NR_clock_nanosleep`

```rust
pub const __NR_clock_nanosleep: u32 = 115;
```

#### Constant `__NR_syslog`

```rust
pub const __NR_syslog: u32 = 116;
```

#### Constant `__NR_ptrace`

```rust
pub const __NR_ptrace: u32 = 117;
```

#### Constant `__NR_sched_setparam`

```rust
pub const __NR_sched_setparam: u32 = 118;
```

#### Constant `__NR_sched_setscheduler`

```rust
pub const __NR_sched_setscheduler: u32 = 119;
```

#### Constant `__NR_sched_getscheduler`

```rust
pub const __NR_sched_getscheduler: u32 = 120;
```

#### Constant `__NR_sched_getparam`

```rust
pub const __NR_sched_getparam: u32 = 121;
```

#### Constant `__NR_sched_setaffinity`

```rust
pub const __NR_sched_setaffinity: u32 = 122;
```

#### Constant `__NR_sched_getaffinity`

```rust
pub const __NR_sched_getaffinity: u32 = 123;
```

#### Constant `__NR_sched_yield`

```rust
pub const __NR_sched_yield: u32 = 124;
```

#### Constant `__NR_sched_get_priority_max`

```rust
pub const __NR_sched_get_priority_max: u32 = 125;
```

#### Constant `__NR_sched_get_priority_min`

```rust
pub const __NR_sched_get_priority_min: u32 = 126;
```

#### Constant `__NR_sched_rr_get_interval`

```rust
pub const __NR_sched_rr_get_interval: u32 = 127;
```

#### Constant `__NR_restart_syscall`

```rust
pub const __NR_restart_syscall: u32 = 128;
```

#### Constant `__NR_kill`

```rust
pub const __NR_kill: u32 = 129;
```

#### Constant `__NR_tkill`

```rust
pub const __NR_tkill: u32 = 130;
```

#### Constant `__NR_tgkill`

```rust
pub const __NR_tgkill: u32 = 131;
```

#### Constant `__NR_sigaltstack`

```rust
pub const __NR_sigaltstack: u32 = 132;
```

#### Constant `__NR_rt_sigsuspend`

```rust
pub const __NR_rt_sigsuspend: u32 = 133;
```

#### Constant `__NR_rt_sigaction`

```rust
pub const __NR_rt_sigaction: u32 = 134;
```

#### Constant `__NR_rt_sigprocmask`

```rust
pub const __NR_rt_sigprocmask: u32 = 135;
```

#### Constant `__NR_rt_sigpending`

```rust
pub const __NR_rt_sigpending: u32 = 136;
```

#### Constant `__NR_rt_sigtimedwait`

```rust
pub const __NR_rt_sigtimedwait: u32 = 137;
```

#### Constant `__NR_rt_sigqueueinfo`

```rust
pub const __NR_rt_sigqueueinfo: u32 = 138;
```

#### Constant `__NR_rt_sigreturn`

```rust
pub const __NR_rt_sigreturn: u32 = 139;
```

#### Constant `__NR_setpriority`

```rust
pub const __NR_setpriority: u32 = 140;
```

#### Constant `__NR_getpriority`

```rust
pub const __NR_getpriority: u32 = 141;
```

#### Constant `__NR_reboot`

```rust
pub const __NR_reboot: u32 = 142;
```

#### Constant `__NR_setregid`

```rust
pub const __NR_setregid: u32 = 143;
```

#### Constant `__NR_setgid`

```rust
pub const __NR_setgid: u32 = 144;
```

#### Constant `__NR_setreuid`

```rust
pub const __NR_setreuid: u32 = 145;
```

#### Constant `__NR_setuid`

```rust
pub const __NR_setuid: u32 = 146;
```

#### Constant `__NR_setresuid`

```rust
pub const __NR_setresuid: u32 = 147;
```

#### Constant `__NR_getresuid`

```rust
pub const __NR_getresuid: u32 = 148;
```

#### Constant `__NR_setresgid`

```rust
pub const __NR_setresgid: u32 = 149;
```

#### Constant `__NR_getresgid`

```rust
pub const __NR_getresgid: u32 = 150;
```

#### Constant `__NR_setfsuid`

```rust
pub const __NR_setfsuid: u32 = 151;
```

#### Constant `__NR_setfsgid`

```rust
pub const __NR_setfsgid: u32 = 152;
```

#### Constant `__NR_times`

```rust
pub const __NR_times: u32 = 153;
```

#### Constant `__NR_setpgid`

```rust
pub const __NR_setpgid: u32 = 154;
```

#### Constant `__NR_getpgid`

```rust
pub const __NR_getpgid: u32 = 155;
```

#### Constant `__NR_getsid`

```rust
pub const __NR_getsid: u32 = 156;
```

#### Constant `__NR_setsid`

```rust
pub const __NR_setsid: u32 = 157;
```

#### Constant `__NR_getgroups`

```rust
pub const __NR_getgroups: u32 = 158;
```

#### Constant `__NR_setgroups`

```rust
pub const __NR_setgroups: u32 = 159;
```

#### Constant `__NR_uname`

```rust
pub const __NR_uname: u32 = 160;
```

#### Constant `__NR_sethostname`

```rust
pub const __NR_sethostname: u32 = 161;
```

#### Constant `__NR_setdomainname`

```rust
pub const __NR_setdomainname: u32 = 162;
```

#### Constant `__NR_getrlimit`

```rust
pub const __NR_getrlimit: u32 = 163;
```

#### Constant `__NR_setrlimit`

```rust
pub const __NR_setrlimit: u32 = 164;
```

#### Constant `__NR_getrusage`

```rust
pub const __NR_getrusage: u32 = 165;
```

#### Constant `__NR_umask`

```rust
pub const __NR_umask: u32 = 166;
```

#### Constant `__NR_prctl`

```rust
pub const __NR_prctl: u32 = 167;
```

#### Constant `__NR_getcpu`

```rust
pub const __NR_getcpu: u32 = 168;
```

#### Constant `__NR_gettimeofday`

```rust
pub const __NR_gettimeofday: u32 = 169;
```

#### Constant `__NR_settimeofday`

```rust
pub const __NR_settimeofday: u32 = 170;
```

#### Constant `__NR_adjtimex`

```rust
pub const __NR_adjtimex: u32 = 171;
```

#### Constant `__NR_getpid`

```rust
pub const __NR_getpid: u32 = 172;
```

#### Constant `__NR_getppid`

```rust
pub const __NR_getppid: u32 = 173;
```

#### Constant `__NR_getuid`

```rust
pub const __NR_getuid: u32 = 174;
```

#### Constant `__NR_geteuid`

```rust
pub const __NR_geteuid: u32 = 175;
```

#### Constant `__NR_getgid`

```rust
pub const __NR_getgid: u32 = 176;
```

#### Constant `__NR_getegid`

```rust
pub const __NR_getegid: u32 = 177;
```

#### Constant `__NR_gettid`

```rust
pub const __NR_gettid: u32 = 178;
```

#### Constant `__NR_sysinfo`

```rust
pub const __NR_sysinfo: u32 = 179;
```

#### Constant `__NR_mq_open`

```rust
pub const __NR_mq_open: u32 = 180;
```

#### Constant `__NR_mq_unlink`

```rust
pub const __NR_mq_unlink: u32 = 181;
```

#### Constant `__NR_mq_timedsend`

```rust
pub const __NR_mq_timedsend: u32 = 182;
```

#### Constant `__NR_mq_timedreceive`

```rust
pub const __NR_mq_timedreceive: u32 = 183;
```

#### Constant `__NR_mq_notify`

```rust
pub const __NR_mq_notify: u32 = 184;
```

#### Constant `__NR_mq_getsetattr`

```rust
pub const __NR_mq_getsetattr: u32 = 185;
```

#### Constant `__NR_msgget`

```rust
pub const __NR_msgget: u32 = 186;
```

#### Constant `__NR_msgctl`

```rust
pub const __NR_msgctl: u32 = 187;
```

#### Constant `__NR_msgrcv`

```rust
pub const __NR_msgrcv: u32 = 188;
```

#### Constant `__NR_msgsnd`

```rust
pub const __NR_msgsnd: u32 = 189;
```

#### Constant `__NR_semget`

```rust
pub const __NR_semget: u32 = 190;
```

#### Constant `__NR_semctl`

```rust
pub const __NR_semctl: u32 = 191;
```

#### Constant `__NR_semtimedop`

```rust
pub const __NR_semtimedop: u32 = 192;
```

#### Constant `__NR_semop`

```rust
pub const __NR_semop: u32 = 193;
```

#### Constant `__NR_shmget`

```rust
pub const __NR_shmget: u32 = 194;
```

#### Constant `__NR_shmctl`

```rust
pub const __NR_shmctl: u32 = 195;
```

#### Constant `__NR_shmat`

```rust
pub const __NR_shmat: u32 = 196;
```

#### Constant `__NR_shmdt`

```rust
pub const __NR_shmdt: u32 = 197;
```

#### Constant `__NR_socket`

```rust
pub const __NR_socket: u32 = 198;
```

#### Constant `__NR_socketpair`

```rust
pub const __NR_socketpair: u32 = 199;
```

#### Constant `__NR_bind`

```rust
pub const __NR_bind: u32 = 200;
```

#### Constant `__NR_listen`

```rust
pub const __NR_listen: u32 = 201;
```

#### Constant `__NR_accept`

```rust
pub const __NR_accept: u32 = 202;
```

#### Constant `__NR_connect`

```rust
pub const __NR_connect: u32 = 203;
```

#### Constant `__NR_getsockname`

```rust
pub const __NR_getsockname: u32 = 204;
```

#### Constant `__NR_getpeername`

```rust
pub const __NR_getpeername: u32 = 205;
```

#### Constant `__NR_sendto`

```rust
pub const __NR_sendto: u32 = 206;
```

#### Constant `__NR_recvfrom`

```rust
pub const __NR_recvfrom: u32 = 207;
```

#### Constant `__NR_setsockopt`

```rust
pub const __NR_setsockopt: u32 = 208;
```

#### Constant `__NR_getsockopt`

```rust
pub const __NR_getsockopt: u32 = 209;
```

#### Constant `__NR_shutdown`

```rust
pub const __NR_shutdown: u32 = 210;
```

#### Constant `__NR_sendmsg`

```rust
pub const __NR_sendmsg: u32 = 211;
```

#### Constant `__NR_recvmsg`

```rust
pub const __NR_recvmsg: u32 = 212;
```

#### Constant `__NR_readahead`

```rust
pub const __NR_readahead: u32 = 213;
```

#### Constant `__NR_brk`

```rust
pub const __NR_brk: u32 = 214;
```

#### Constant `__NR_munmap`

```rust
pub const __NR_munmap: u32 = 215;
```

#### Constant `__NR_mremap`

```rust
pub const __NR_mremap: u32 = 216;
```

#### Constant `__NR_add_key`

```rust
pub const __NR_add_key: u32 = 217;
```

#### Constant `__NR_request_key`

```rust
pub const __NR_request_key: u32 = 218;
```

#### Constant `__NR_keyctl`

```rust
pub const __NR_keyctl: u32 = 219;
```

#### Constant `__NR_clone`

```rust
pub const __NR_clone: u32 = 220;
```

#### Constant `__NR_execve`

```rust
pub const __NR_execve: u32 = 221;
```

#### Constant `__NR_mmap`

```rust
pub const __NR_mmap: u32 = 222;
```

#### Constant `__NR_fadvise64`

```rust
pub const __NR_fadvise64: u32 = 223;
```

#### Constant `__NR_swapon`

```rust
pub const __NR_swapon: u32 = 224;
```

#### Constant `__NR_swapoff`

```rust
pub const __NR_swapoff: u32 = 225;
```

#### Constant `__NR_mprotect`

```rust
pub const __NR_mprotect: u32 = 226;
```

#### Constant `__NR_msync`

```rust
pub const __NR_msync: u32 = 227;
```

#### Constant `__NR_mlock`

```rust
pub const __NR_mlock: u32 = 228;
```

#### Constant `__NR_munlock`

```rust
pub const __NR_munlock: u32 = 229;
```

#### Constant `__NR_mlockall`

```rust
pub const __NR_mlockall: u32 = 230;
```

#### Constant `__NR_munlockall`

```rust
pub const __NR_munlockall: u32 = 231;
```

#### Constant `__NR_mincore`

```rust
pub const __NR_mincore: u32 = 232;
```

#### Constant `__NR_madvise`

```rust
pub const __NR_madvise: u32 = 233;
```

#### Constant `__NR_remap_file_pages`

```rust
pub const __NR_remap_file_pages: u32 = 234;
```

#### Constant `__NR_mbind`

```rust
pub const __NR_mbind: u32 = 235;
```

#### Constant `__NR_get_mempolicy`

```rust
pub const __NR_get_mempolicy: u32 = 236;
```

#### Constant `__NR_set_mempolicy`

```rust
pub const __NR_set_mempolicy: u32 = 237;
```

#### Constant `__NR_migrate_pages`

```rust
pub const __NR_migrate_pages: u32 = 238;
```

#### Constant `__NR_move_pages`

```rust
pub const __NR_move_pages: u32 = 239;
```

#### Constant `__NR_rt_tgsigqueueinfo`

```rust
pub const __NR_rt_tgsigqueueinfo: u32 = 240;
```

#### Constant `__NR_perf_event_open`

```rust
pub const __NR_perf_event_open: u32 = 241;
```

#### Constant `__NR_accept4`

```rust
pub const __NR_accept4: u32 = 242;
```

#### Constant `__NR_recvmmsg`

```rust
pub const __NR_recvmmsg: u32 = 243;
```

#### Constant `__NR_wait4`

```rust
pub const __NR_wait4: u32 = 260;
```

#### Constant `__NR_prlimit64`

```rust
pub const __NR_prlimit64: u32 = 261;
```

#### Constant `__NR_fanotify_init`

```rust
pub const __NR_fanotify_init: u32 = 262;
```

#### Constant `__NR_fanotify_mark`

```rust
pub const __NR_fanotify_mark: u32 = 263;
```

#### Constant `__NR_name_to_handle_at`

```rust
pub const __NR_name_to_handle_at: u32 = 264;
```

#### Constant `__NR_open_by_handle_at`

```rust
pub const __NR_open_by_handle_at: u32 = 265;
```

#### Constant `__NR_clock_adjtime`

```rust
pub const __NR_clock_adjtime: u32 = 266;
```

#### Constant `__NR_syncfs`

```rust
pub const __NR_syncfs: u32 = 267;
```

#### Constant `__NR_setns`

```rust
pub const __NR_setns: u32 = 268;
```

#### Constant `__NR_sendmmsg`

```rust
pub const __NR_sendmmsg: u32 = 269;
```

#### Constant `__NR_process_vm_readv`

```rust
pub const __NR_process_vm_readv: u32 = 270;
```

#### Constant `__NR_process_vm_writev`

```rust
pub const __NR_process_vm_writev: u32 = 271;
```

#### Constant `__NR_kcmp`

```rust
pub const __NR_kcmp: u32 = 272;
```

#### Constant `__NR_finit_module`

```rust
pub const __NR_finit_module: u32 = 273;
```

#### Constant `__NR_sched_setattr`

```rust
pub const __NR_sched_setattr: u32 = 274;
```

#### Constant `__NR_sched_getattr`

```rust
pub const __NR_sched_getattr: u32 = 275;
```

#### Constant `__NR_renameat2`

```rust
pub const __NR_renameat2: u32 = 276;
```

#### Constant `__NR_seccomp`

```rust
pub const __NR_seccomp: u32 = 277;
```

#### Constant `__NR_getrandom`

```rust
pub const __NR_getrandom: u32 = 278;
```

#### Constant `__NR_memfd_create`

```rust
pub const __NR_memfd_create: u32 = 279;
```

#### Constant `__NR_bpf`

```rust
pub const __NR_bpf: u32 = 280;
```

#### Constant `__NR_execveat`

```rust
pub const __NR_execveat: u32 = 281;
```

#### Constant `__NR_userfaultfd`

```rust
pub const __NR_userfaultfd: u32 = 282;
```

#### Constant `__NR_membarrier`

```rust
pub const __NR_membarrier: u32 = 283;
```

#### Constant `__NR_mlock2`

```rust
pub const __NR_mlock2: u32 = 284;
```

#### Constant `__NR_copy_file_range`

```rust
pub const __NR_copy_file_range: u32 = 285;
```

#### Constant `__NR_preadv2`

```rust
pub const __NR_preadv2: u32 = 286;
```

#### Constant `__NR_pwritev2`

```rust
pub const __NR_pwritev2: u32 = 287;
```

#### Constant `__NR_pkey_mprotect`

```rust
pub const __NR_pkey_mprotect: u32 = 288;
```

#### Constant `__NR_pkey_alloc`

```rust
pub const __NR_pkey_alloc: u32 = 289;
```

#### Constant `__NR_pkey_free`

```rust
pub const __NR_pkey_free: u32 = 290;
```

#### Constant `__NR_statx`

```rust
pub const __NR_statx: u32 = 291;
```

#### Constant `__NR_io_pgetevents`

```rust
pub const __NR_io_pgetevents: u32 = 292;
```

#### Constant `__NR_rseq`

```rust
pub const __NR_rseq: u32 = 293;
```

#### Constant `__NR_kexec_file_load`

```rust
pub const __NR_kexec_file_load: u32 = 294;
```

#### Constant `__NR_pidfd_send_signal`

```rust
pub const __NR_pidfd_send_signal: u32 = 424;
```

#### Constant `__NR_io_uring_setup`

```rust
pub const __NR_io_uring_setup: u32 = 425;
```

#### Constant `__NR_io_uring_enter`

```rust
pub const __NR_io_uring_enter: u32 = 426;
```

#### Constant `__NR_io_uring_register`

```rust
pub const __NR_io_uring_register: u32 = 427;
```

#### Constant `__NR_open_tree`

```rust
pub const __NR_open_tree: u32 = 428;
```

#### Constant `__NR_move_mount`

```rust
pub const __NR_move_mount: u32 = 429;
```

#### Constant `__NR_fsopen`

```rust
pub const __NR_fsopen: u32 = 430;
```

#### Constant `__NR_fsconfig`

```rust
pub const __NR_fsconfig: u32 = 431;
```

#### Constant `__NR_fsmount`

```rust
pub const __NR_fsmount: u32 = 432;
```

#### Constant `__NR_fspick`

```rust
pub const __NR_fspick: u32 = 433;
```

#### Constant `__NR_pidfd_open`

```rust
pub const __NR_pidfd_open: u32 = 434;
```

#### Constant `__NR_clone3`

```rust
pub const __NR_clone3: u32 = 435;
```

#### Constant `__NR_close_range`

```rust
pub const __NR_close_range: u32 = 436;
```

#### Constant `__NR_openat2`

```rust
pub const __NR_openat2: u32 = 437;
```

#### Constant `__NR_pidfd_getfd`

```rust
pub const __NR_pidfd_getfd: u32 = 438;
```

#### Constant `__NR_faccessat2`

```rust
pub const __NR_faccessat2: u32 = 439;
```

#### Constant `__NR_process_madvise`

```rust
pub const __NR_process_madvise: u32 = 440;
```

#### Constant `__NR_epoll_pwait2`

```rust
pub const __NR_epoll_pwait2: u32 = 441;
```

#### Constant `__NR_mount_setattr`

```rust
pub const __NR_mount_setattr: u32 = 442;
```

#### Constant `__NR_quotactl_fd`

```rust
pub const __NR_quotactl_fd: u32 = 443;
```

#### Constant `__NR_landlock_create_ruleset`

```rust
pub const __NR_landlock_create_ruleset: u32 = 444;
```

#### Constant `__NR_landlock_add_rule`

```rust
pub const __NR_landlock_add_rule: u32 = 445;
```

#### Constant `__NR_landlock_restrict_self`

```rust
pub const __NR_landlock_restrict_self: u32 = 446;
```

#### Constant `__NR_memfd_secret`

```rust
pub const __NR_memfd_secret: u32 = 447;
```

#### Constant `__NR_process_mrelease`

```rust
pub const __NR_process_mrelease: u32 = 448;
```

#### Constant `__NR_futex_waitv`

```rust
pub const __NR_futex_waitv: u32 = 449;
```

#### Constant `__NR_set_mempolicy_home_node`

```rust
pub const __NR_set_mempolicy_home_node: u32 = 450;
```

#### Constant `__NR_cachestat`

```rust
pub const __NR_cachestat: u32 = 451;
```

#### Constant `__NR_fchmodat2`

```rust
pub const __NR_fchmodat2: u32 = 452;
```

#### Constant `__NR_map_shadow_stack`

```rust
pub const __NR_map_shadow_stack: u32 = 453;
```

#### Constant `__NR_futex_wake`

```rust
pub const __NR_futex_wake: u32 = 454;
```

#### Constant `__NR_futex_wait`

```rust
pub const __NR_futex_wait: u32 = 455;
```

#### Constant `__NR_futex_requeue`

```rust
pub const __NR_futex_requeue: u32 = 456;
```

#### Constant `__NR_statmount`

```rust
pub const __NR_statmount: u32 = 457;
```

#### Constant `__NR_listmount`

```rust
pub const __NR_listmount: u32 = 458;
```

#### Constant `__NR_lsm_get_self_attr`

```rust
pub const __NR_lsm_get_self_attr: u32 = 459;
```

#### Constant `__NR_lsm_set_self_attr`

```rust
pub const __NR_lsm_set_self_attr: u32 = 460;
```

#### Constant `__NR_lsm_list_modules`

```rust
pub const __NR_lsm_list_modules: u32 = 461;
```

#### Constant `__NR_mseal`

```rust
pub const __NR_mseal: u32 = 462;
```

#### Constant `__NR_setxattrat`

```rust
pub const __NR_setxattrat: u32 = 463;
```

#### Constant `__NR_getxattrat`

```rust
pub const __NR_getxattrat: u32 = 464;
```

#### Constant `__NR_listxattrat`

```rust
pub const __NR_listxattrat: u32 = 465;
```

#### Constant `__NR_removexattrat`

```rust
pub const __NR_removexattrat: u32 = 466;
```

#### Constant `WNOHANG`

```rust
pub const WNOHANG: u32 = 1;
```

#### Constant `WUNTRACED`

```rust
pub const WUNTRACED: u32 = 2;
```

#### Constant `WSTOPPED`

```rust
pub const WSTOPPED: u32 = 2;
```

#### Constant `WEXITED`

```rust
pub const WEXITED: u32 = 4;
```

#### Constant `WCONTINUED`

```rust
pub const WCONTINUED: u32 = 8;
```

#### Constant `WNOWAIT`

```rust
pub const WNOWAIT: u32 = 16777216;
```

#### Constant `__WNOTHREAD`

```rust
pub const __WNOTHREAD: u32 = 536870912;
```

#### Constant `__WALL`

```rust
pub const __WALL: u32 = 1073741824;
```

#### Constant `__WCLONE`

```rust
pub const __WCLONE: u32 = 2147483648;
```

#### Constant `P_ALL`

```rust
pub const P_ALL: u32 = 0;
```

#### Constant `P_PID`

```rust
pub const P_PID: u32 = 1;
```

#### Constant `P_PGID`

```rust
pub const P_PGID: u32 = 2;
```

#### Constant `P_PIDFD`

```rust
pub const P_PIDFD: u32 = 3;
```

#### Constant `XATTR_CREATE`

```rust
pub const XATTR_CREATE: u32 = 1;
```

#### Constant `XATTR_REPLACE`

```rust
pub const XATTR_REPLACE: u32 = 2;
```

#### Constant `XATTR_OS2_PREFIX`

```rust
pub const XATTR_OS2_PREFIX: &[u8; 5] = b"os2.\0";
```

#### Constant `XATTR_MAC_OSX_PREFIX`

```rust
pub const XATTR_MAC_OSX_PREFIX: &[u8; 5] = b"osx.\0";
```

#### Constant `XATTR_BTRFS_PREFIX`

```rust
pub const XATTR_BTRFS_PREFIX: &[u8; 7] = b"btrfs.\0";
```

#### Constant `XATTR_HURD_PREFIX`

```rust
pub const XATTR_HURD_PREFIX: &[u8; 5] = b"gnu.\0";
```

#### Constant `XATTR_SECURITY_PREFIX`

```rust
pub const XATTR_SECURITY_PREFIX: &[u8; 10] = b"security.\0";
```

#### Constant `XATTR_SYSTEM_PREFIX`

```rust
pub const XATTR_SYSTEM_PREFIX: &[u8; 8] = b"system.\0";
```

#### Constant `XATTR_TRUSTED_PREFIX`

```rust
pub const XATTR_TRUSTED_PREFIX: &[u8; 9] = b"trusted.\0";
```

#### Constant `XATTR_USER_PREFIX`

```rust
pub const XATTR_USER_PREFIX: &[u8; 6] = b"user.\0";
```

#### Constant `XATTR_EVM_SUFFIX`

```rust
pub const XATTR_EVM_SUFFIX: &[u8; 4] = b"evm\0";
```

#### Constant `XATTR_NAME_EVM`

```rust
pub const XATTR_NAME_EVM: &[u8; 13] = b"security.evm\0";
```

#### Constant `XATTR_IMA_SUFFIX`

```rust
pub const XATTR_IMA_SUFFIX: &[u8; 4] = b"ima\0";
```

#### Constant `XATTR_NAME_IMA`

```rust
pub const XATTR_NAME_IMA: &[u8; 13] = b"security.ima\0";
```

#### Constant `XATTR_SELINUX_SUFFIX`

```rust
pub const XATTR_SELINUX_SUFFIX: &[u8; 8] = b"selinux\0";
```

#### Constant `XATTR_NAME_SELINUX`

```rust
pub const XATTR_NAME_SELINUX: &[u8; 17] = b"security.selinux\0";
```

#### Constant `XATTR_SMACK_SUFFIX`

```rust
pub const XATTR_SMACK_SUFFIX: &[u8; 8] = b"SMACK64\0";
```

#### Constant `XATTR_SMACK_IPIN`

```rust
pub const XATTR_SMACK_IPIN: &[u8; 12] = b"SMACK64IPIN\0";
```

#### Constant `XATTR_SMACK_IPOUT`

```rust
pub const XATTR_SMACK_IPOUT: &[u8; 13] = b"SMACK64IPOUT\0";
```

#### Constant `XATTR_SMACK_EXEC`

```rust
pub const XATTR_SMACK_EXEC: &[u8; 12] = b"SMACK64EXEC\0";
```

#### Constant `XATTR_SMACK_TRANSMUTE`

```rust
pub const XATTR_SMACK_TRANSMUTE: &[u8; 17] = b"SMACK64TRANSMUTE\0";
```

#### Constant `XATTR_SMACK_MMAP`

```rust
pub const XATTR_SMACK_MMAP: &[u8; 12] = b"SMACK64MMAP\0";
```

#### Constant `XATTR_NAME_SMACK`

```rust
pub const XATTR_NAME_SMACK: &[u8; 17] = b"security.SMACK64\0";
```

#### Constant `XATTR_NAME_SMACKIPIN`

```rust
pub const XATTR_NAME_SMACKIPIN: &[u8; 21] = b"security.SMACK64IPIN\0";
```

#### Constant `XATTR_NAME_SMACKIPOUT`

```rust
pub const XATTR_NAME_SMACKIPOUT: &[u8; 22] = b"security.SMACK64IPOUT\0";
```

#### Constant `XATTR_NAME_SMACKEXEC`

```rust
pub const XATTR_NAME_SMACKEXEC: &[u8; 21] = b"security.SMACK64EXEC\0";
```

#### Constant `XATTR_NAME_SMACKTRANSMUTE`

```rust
pub const XATTR_NAME_SMACKTRANSMUTE: &[u8; 26] = b"security.SMACK64TRANSMUTE\0";
```

#### Constant `XATTR_NAME_SMACKMMAP`

```rust
pub const XATTR_NAME_SMACKMMAP: &[u8; 21] = b"security.SMACK64MMAP\0";
```

#### Constant `XATTR_APPARMOR_SUFFIX`

```rust
pub const XATTR_APPARMOR_SUFFIX: &[u8; 9] = b"apparmor\0";
```

#### Constant `XATTR_NAME_APPARMOR`

```rust
pub const XATTR_NAME_APPARMOR: &[u8; 18] = b"security.apparmor\0";
```

#### Constant `XATTR_CAPS_SUFFIX`

```rust
pub const XATTR_CAPS_SUFFIX: &[u8; 11] = b"capability\0";
```

#### Constant `XATTR_NAME_CAPS`

```rust
pub const XATTR_NAME_CAPS: &[u8; 20] = b"security.capability\0";
```

#### Constant `XATTR_POSIX_ACL_ACCESS`

```rust
pub const XATTR_POSIX_ACL_ACCESS: &[u8; 17] = b"posix_acl_access\0";
```

#### Constant `XATTR_NAME_POSIX_ACL_ACCESS`

```rust
pub const XATTR_NAME_POSIX_ACL_ACCESS: &[u8; 24] = b"system.posix_acl_access\0";
```

#### Constant `XATTR_POSIX_ACL_DEFAULT`

```rust
pub const XATTR_POSIX_ACL_DEFAULT: &[u8; 18] = b"posix_acl_default\0";
```

#### Constant `XATTR_NAME_POSIX_ACL_DEFAULT`

```rust
pub const XATTR_NAME_POSIX_ACL_DEFAULT: &[u8; 25] = b"system.posix_acl_default\0";
```

#### Constant `MFD_CLOEXEC`

```rust
pub const MFD_CLOEXEC: u32 = 1;
```

#### Constant `MFD_ALLOW_SEALING`

```rust
pub const MFD_ALLOW_SEALING: u32 = 2;
```

#### Constant `MFD_HUGETLB`

```rust
pub const MFD_HUGETLB: u32 = 4;
```

#### Constant `MFD_NOEXEC_SEAL`

```rust
pub const MFD_NOEXEC_SEAL: u32 = 8;
```

#### Constant `MFD_EXEC`

```rust
pub const MFD_EXEC: u32 = 16;
```

#### Constant `MFD_HUGE_SHIFT`

```rust
pub const MFD_HUGE_SHIFT: u32 = 26;
```

#### Constant `MFD_HUGE_MASK`

```rust
pub const MFD_HUGE_MASK: u32 = 63;
```

#### Constant `MFD_HUGE_64KB`

```rust
pub const MFD_HUGE_64KB: u32 = 1073741824;
```

#### Constant `MFD_HUGE_512KB`

```rust
pub const MFD_HUGE_512KB: u32 = 1275068416;
```

#### Constant `MFD_HUGE_1MB`

```rust
pub const MFD_HUGE_1MB: u32 = 1342177280;
```

#### Constant `MFD_HUGE_2MB`

```rust
pub const MFD_HUGE_2MB: u32 = 1409286144;
```

#### Constant `MFD_HUGE_8MB`

```rust
pub const MFD_HUGE_8MB: u32 = 1543503872;
```

#### Constant `MFD_HUGE_16MB`

```rust
pub const MFD_HUGE_16MB: u32 = 1610612736;
```

#### Constant `MFD_HUGE_32MB`

```rust
pub const MFD_HUGE_32MB: u32 = 1677721600;
```

#### Constant `MFD_HUGE_256MB`

```rust
pub const MFD_HUGE_256MB: u32 = 1879048192;
```

#### Constant `MFD_HUGE_512MB`

```rust
pub const MFD_HUGE_512MB: u32 = 1946157056;
```

#### Constant `MFD_HUGE_1GB`

```rust
pub const MFD_HUGE_1GB: u32 = 2013265920;
```

#### Constant `MFD_HUGE_2GB`

```rust
pub const MFD_HUGE_2GB: u32 = 2080374784;
```

#### Constant `MFD_HUGE_16GB`

```rust
pub const MFD_HUGE_16GB: u32 = 2281701376;
```

#### Constant `TFD_TIMER_ABSTIME`

```rust
pub const TFD_TIMER_ABSTIME: u32 = 1;
```

#### Constant `TFD_TIMER_CANCEL_ON_SET`

```rust
pub const TFD_TIMER_CANCEL_ON_SET: u32 = 2;
```

#### Constant `TFD_CLOEXEC`

```rust
pub const TFD_CLOEXEC: u32 = 524288;
```

#### Constant `TFD_NONBLOCK`

```rust
pub const TFD_NONBLOCK: u32 = 2048;
```

#### Constant `USERFAULTFD_IOC`

```rust
pub const USERFAULTFD_IOC: u32 = 170;
```

#### Constant `_UFFDIO_REGISTER`

```rust
pub const _UFFDIO_REGISTER: u32 = 0;
```

#### Constant `_UFFDIO_UNREGISTER`

```rust
pub const _UFFDIO_UNREGISTER: u32 = 1;
```

#### Constant `_UFFDIO_WAKE`

```rust
pub const _UFFDIO_WAKE: u32 = 2;
```

#### Constant `_UFFDIO_COPY`

```rust
pub const _UFFDIO_COPY: u32 = 3;
```

#### Constant `_UFFDIO_ZEROPAGE`

```rust
pub const _UFFDIO_ZEROPAGE: u32 = 4;
```

#### Constant `_UFFDIO_MOVE`

```rust
pub const _UFFDIO_MOVE: u32 = 5;
```

#### Constant `_UFFDIO_WRITEPROTECT`

```rust
pub const _UFFDIO_WRITEPROTECT: u32 = 6;
```

#### Constant `_UFFDIO_CONTINUE`

```rust
pub const _UFFDIO_CONTINUE: u32 = 7;
```

#### Constant `_UFFDIO_POISON`

```rust
pub const _UFFDIO_POISON: u32 = 8;
```

#### Constant `_UFFDIO_API`

```rust
pub const _UFFDIO_API: u32 = 63;
```

#### Constant `UFFDIO`

```rust
pub const UFFDIO: u32 = 170;
```

#### Constant `UFFD_EVENT_PAGEFAULT`

```rust
pub const UFFD_EVENT_PAGEFAULT: u32 = 18;
```

#### Constant `UFFD_EVENT_FORK`

```rust
pub const UFFD_EVENT_FORK: u32 = 19;
```

#### Constant `UFFD_EVENT_REMAP`

```rust
pub const UFFD_EVENT_REMAP: u32 = 20;
```

#### Constant `UFFD_EVENT_REMOVE`

```rust
pub const UFFD_EVENT_REMOVE: u32 = 21;
```

#### Constant `UFFD_EVENT_UNMAP`

```rust
pub const UFFD_EVENT_UNMAP: u32 = 22;
```

#### Constant `UFFD_PAGEFAULT_FLAG_WRITE`

```rust
pub const UFFD_PAGEFAULT_FLAG_WRITE: u32 = 1;
```

#### Constant `UFFD_PAGEFAULT_FLAG_WP`

```rust
pub const UFFD_PAGEFAULT_FLAG_WP: u32 = 2;
```

#### Constant `UFFD_PAGEFAULT_FLAG_MINOR`

```rust
pub const UFFD_PAGEFAULT_FLAG_MINOR: u32 = 4;
```

#### Constant `UFFD_FEATURE_PAGEFAULT_FLAG_WP`

```rust
pub const UFFD_FEATURE_PAGEFAULT_FLAG_WP: u32 = 1;
```

#### Constant `UFFD_FEATURE_EVENT_FORK`

```rust
pub const UFFD_FEATURE_EVENT_FORK: u32 = 2;
```

#### Constant `UFFD_FEATURE_EVENT_REMAP`

```rust
pub const UFFD_FEATURE_EVENT_REMAP: u32 = 4;
```

#### Constant `UFFD_FEATURE_EVENT_REMOVE`

```rust
pub const UFFD_FEATURE_EVENT_REMOVE: u32 = 8;
```

#### Constant `UFFD_FEATURE_MISSING_HUGETLBFS`

```rust
pub const UFFD_FEATURE_MISSING_HUGETLBFS: u32 = 16;
```

#### Constant `UFFD_FEATURE_MISSING_SHMEM`

```rust
pub const UFFD_FEATURE_MISSING_SHMEM: u32 = 32;
```

#### Constant `UFFD_FEATURE_EVENT_UNMAP`

```rust
pub const UFFD_FEATURE_EVENT_UNMAP: u32 = 64;
```

#### Constant `UFFD_FEATURE_SIGBUS`

```rust
pub const UFFD_FEATURE_SIGBUS: u32 = 128;
```

#### Constant `UFFD_FEATURE_THREAD_ID`

```rust
pub const UFFD_FEATURE_THREAD_ID: u32 = 256;
```

#### Constant `UFFD_FEATURE_MINOR_HUGETLBFS`

```rust
pub const UFFD_FEATURE_MINOR_HUGETLBFS: u32 = 512;
```

#### Constant `UFFD_FEATURE_MINOR_SHMEM`

```rust
pub const UFFD_FEATURE_MINOR_SHMEM: u32 = 1024;
```

#### Constant `UFFD_FEATURE_EXACT_ADDRESS`

```rust
pub const UFFD_FEATURE_EXACT_ADDRESS: u32 = 2048;
```

#### Constant `UFFD_FEATURE_WP_HUGETLBFS_SHMEM`

```rust
pub const UFFD_FEATURE_WP_HUGETLBFS_SHMEM: u32 = 4096;
```

#### Constant `UFFD_FEATURE_WP_UNPOPULATED`

```rust
pub const UFFD_FEATURE_WP_UNPOPULATED: u32 = 8192;
```

#### Constant `UFFD_FEATURE_POISON`

```rust
pub const UFFD_FEATURE_POISON: u32 = 16384;
```

#### Constant `UFFD_FEATURE_WP_ASYNC`

```rust
pub const UFFD_FEATURE_WP_ASYNC: u32 = 32768;
```

#### Constant `UFFD_FEATURE_MOVE`

```rust
pub const UFFD_FEATURE_MOVE: u32 = 65536;
```

#### Constant `UFFD_USER_MODE_ONLY`

```rust
pub const UFFD_USER_MODE_ONLY: u32 = 1;
```

#### Constant `DT_UNKNOWN`

```rust
pub const DT_UNKNOWN: u32 = 0;
```

#### Constant `DT_FIFO`

```rust
pub const DT_FIFO: u32 = 1;
```

#### Constant `DT_CHR`

```rust
pub const DT_CHR: u32 = 2;
```

#### Constant `DT_DIR`

```rust
pub const DT_DIR: u32 = 4;
```

#### Constant `DT_BLK`

```rust
pub const DT_BLK: u32 = 6;
```

#### Constant `DT_REG`

```rust
pub const DT_REG: u32 = 8;
```

#### Constant `DT_LNK`

```rust
pub const DT_LNK: u32 = 10;
```

#### Constant `DT_SOCK`

```rust
pub const DT_SOCK: u32 = 12;
```

#### Constant `STAT_HAVE_NSEC`

```rust
pub const STAT_HAVE_NSEC: u32 = 1;
```

#### Constant `F_OK`

```rust
pub const F_OK: u32 = 0;
```

#### Constant `R_OK`

```rust
pub const R_OK: u32 = 4;
```

#### Constant `W_OK`

```rust
pub const W_OK: u32 = 2;
```

#### Constant `X_OK`

```rust
pub const X_OK: u32 = 1;
```

#### Constant `UTIME_NOW`

```rust
pub const UTIME_NOW: u32 = 1073741823;
```

#### Constant `UTIME_OMIT`

```rust
pub const UTIME_OMIT: u32 = 1073741822;
```

#### Constant `MNT_FORCE`

```rust
pub const MNT_FORCE: u32 = 1;
```

#### Constant `MNT_DETACH`

```rust
pub const MNT_DETACH: u32 = 2;
```

#### Constant `MNT_EXPIRE`

```rust
pub const MNT_EXPIRE: u32 = 4;
```

#### Constant `UMOUNT_NOFOLLOW`

```rust
pub const UMOUNT_NOFOLLOW: u32 = 8;
```

#### Constant `UMOUNT_UNUSED`

```rust
pub const UMOUNT_UNUSED: u32 = 2147483648;
```

#### Constant `STDIN_FILENO`

```rust
pub const STDIN_FILENO: u32 = 0;
```

#### Constant `STDOUT_FILENO`

```rust
pub const STDOUT_FILENO: u32 = 1;
```

#### Constant `STDERR_FILENO`

```rust
pub const STDERR_FILENO: u32 = 2;
```

#### Constant `RWF_HIPRI`

```rust
pub const RWF_HIPRI: u32 = 1;
```

#### Constant `RWF_DSYNC`

```rust
pub const RWF_DSYNC: u32 = 2;
```

#### Constant `RWF_SYNC`

```rust
pub const RWF_SYNC: u32 = 4;
```

#### Constant `RWF_NOWAIT`

```rust
pub const RWF_NOWAIT: u32 = 8;
```

#### Constant `RWF_APPEND`

```rust
pub const RWF_APPEND: u32 = 16;
```

#### Constant `EFD_SEMAPHORE`

```rust
pub const EFD_SEMAPHORE: u32 = 1;
```

#### Constant `EFD_CLOEXEC`

```rust
pub const EFD_CLOEXEC: u32 = 524288;
```

#### Constant `EFD_NONBLOCK`

```rust
pub const EFD_NONBLOCK: u32 = 2048;
```

#### Constant `EPOLLIN`

```rust
pub const EPOLLIN: u32 = 1;
```

#### Constant `EPOLLPRI`

```rust
pub const EPOLLPRI: u32 = 2;
```

#### Constant `EPOLLOUT`

```rust
pub const EPOLLOUT: u32 = 4;
```

#### Constant `EPOLLERR`

```rust
pub const EPOLLERR: u32 = 8;
```

#### Constant `EPOLLHUP`

```rust
pub const EPOLLHUP: u32 = 16;
```

#### Constant `EPOLLNVAL`

```rust
pub const EPOLLNVAL: u32 = 32;
```

#### Constant `EPOLLRDNORM`

```rust
pub const EPOLLRDNORM: u32 = 64;
```

#### Constant `EPOLLRDBAND`

```rust
pub const EPOLLRDBAND: u32 = 128;
```

#### Constant `EPOLLWRNORM`

```rust
pub const EPOLLWRNORM: u32 = 256;
```

#### Constant `EPOLLWRBAND`

```rust
pub const EPOLLWRBAND: u32 = 512;
```

#### Constant `EPOLLMSG`

```rust
pub const EPOLLMSG: u32 = 1024;
```

#### Constant `EPOLLRDHUP`

```rust
pub const EPOLLRDHUP: u32 = 8192;
```

#### Constant `EPOLLEXCLUSIVE`

```rust
pub const EPOLLEXCLUSIVE: u32 = 268435456;
```

#### Constant `EPOLLWAKEUP`

```rust
pub const EPOLLWAKEUP: u32 = 536870912;
```

#### Constant `EPOLLONESHOT`

```rust
pub const EPOLLONESHOT: u32 = 1073741824;
```

#### Constant `EPOLLET`

```rust
pub const EPOLLET: u32 = 2147483648;
```

#### Constant `TFD_SHARED_FCNTL_FLAGS`

```rust
pub const TFD_SHARED_FCNTL_FLAGS: u32 = 526336;
```

#### Constant `TFD_CREATE_FLAGS`

```rust
pub const TFD_CREATE_FLAGS: u32 = 526336;
```

#### Constant `TFD_SETTIME_FLAGS`

```rust
pub const TFD_SETTIME_FLAGS: u32 = 1;
```

#### Constant `UFFD_API`

```rust
pub const UFFD_API: u32 = 170;
```

#### Constant `UFFDIO_REGISTER_MODE_MISSING`

```rust
pub const UFFDIO_REGISTER_MODE_MISSING: u32 = 1;
```

#### Constant `UFFDIO_REGISTER_MODE_WP`

```rust
pub const UFFDIO_REGISTER_MODE_WP: u32 = 2;
```

#### Constant `UFFDIO_REGISTER_MODE_MINOR`

```rust
pub const UFFDIO_REGISTER_MODE_MINOR: u32 = 4;
```

#### Constant `UFFDIO_COPY_MODE_DONTWAKE`

```rust
pub const UFFDIO_COPY_MODE_DONTWAKE: u32 = 1;
```

#### Constant `UFFDIO_COPY_MODE_WP`

```rust
pub const UFFDIO_COPY_MODE_WP: u32 = 2;
```

#### Constant `UFFDIO_ZEROPAGE_MODE_DONTWAKE`

```rust
pub const UFFDIO_ZEROPAGE_MODE_DONTWAKE: u32 = 1;
```

#### Constant `SPLICE_F_MOVE`

```rust
pub const SPLICE_F_MOVE: u32 = 1;
```

#### Constant `SPLICE_F_NONBLOCK`

```rust
pub const SPLICE_F_NONBLOCK: u32 = 2;
```

#### Constant `SPLICE_F_MORE`

```rust
pub const SPLICE_F_MORE: u32 = 4;
```

#### Constant `SPLICE_F_GIFT`

```rust
pub const SPLICE_F_GIFT: u32 = 8;
```

## Module `ioctl`

**Attributes:**

- `#[<cfg>(feature = "ioctl")]`
- `#[<cfg>(target_arch = "aarch64")]`
- `#[path = "aarch64/ioctl.rs"]`

```rust
pub mod ioctl { /* ... */ }
```

### Constants and Statics

#### Constant `FIONREAD`

```rust
pub const FIONREAD: u32 = 21531;
```

#### Constant `FIONBIO`

```rust
pub const FIONBIO: u32 = 21537;
```

#### Constant `FIOCLEX`

```rust
pub const FIOCLEX: u32 = 21585;
```

#### Constant `FIONCLEX`

```rust
pub const FIONCLEX: u32 = 21584;
```

#### Constant `FIOASYNC`

```rust
pub const FIOASYNC: u32 = 21586;
```

#### Constant `FIOQSIZE`

```rust
pub const FIOQSIZE: u32 = 21600;
```

#### Constant `TCXONC`

```rust
pub const TCXONC: u32 = 21514;
```

#### Constant `TCFLSH`

```rust
pub const TCFLSH: u32 = 21515;
```

#### Constant `TIOCSCTTY`

```rust
pub const TIOCSCTTY: u32 = 21518;
```

#### Constant `TIOCSPGRP`

```rust
pub const TIOCSPGRP: u32 = 21520;
```

#### Constant `TIOCOUTQ`

```rust
pub const TIOCOUTQ: u32 = 21521;
```

#### Constant `TIOCSTI`

```rust
pub const TIOCSTI: u32 = 21522;
```

#### Constant `TIOCSWINSZ`

```rust
pub const TIOCSWINSZ: u32 = 21524;
```

#### Constant `TIOCMGET`

```rust
pub const TIOCMGET: u32 = 21525;
```

#### Constant `TIOCMBIS`

```rust
pub const TIOCMBIS: u32 = 21526;
```

#### Constant `TIOCMBIC`

```rust
pub const TIOCMBIC: u32 = 21527;
```

#### Constant `TIOCMSET`

```rust
pub const TIOCMSET: u32 = 21528;
```

#### Constant `TIOCSSOFTCAR`

```rust
pub const TIOCSSOFTCAR: u32 = 21530;
```

#### Constant `TIOCLINUX`

```rust
pub const TIOCLINUX: u32 = 21532;
```

#### Constant `TIOCCONS`

```rust
pub const TIOCCONS: u32 = 21533;
```

#### Constant `TIOCSSERIAL`

```rust
pub const TIOCSSERIAL: u32 = 21535;
```

#### Constant `TIOCPKT`

```rust
pub const TIOCPKT: u32 = 21536;
```

#### Constant `TIOCNOTTY`

```rust
pub const TIOCNOTTY: u32 = 21538;
```

#### Constant `TIOCSETD`

```rust
pub const TIOCSETD: u32 = 21539;
```

#### Constant `TIOCSBRK`

```rust
pub const TIOCSBRK: u32 = 21543;
```

#### Constant `TIOCCBRK`

```rust
pub const TIOCCBRK: u32 = 21544;
```

#### Constant `TIOCSRS485`

```rust
pub const TIOCSRS485: u32 = 21551;
```

#### Constant `TIOCSPTLCK`

```rust
pub const TIOCSPTLCK: u32 = 1074025521;
```

#### Constant `TIOCSIG`

```rust
pub const TIOCSIG: u32 = 1074025526;
```

#### Constant `TIOCVHANGUP`

```rust
pub const TIOCVHANGUP: u32 = 21559;
```

#### Constant `TIOCSERCONFIG`

```rust
pub const TIOCSERCONFIG: u32 = 21587;
```

#### Constant `TIOCSERGWILD`

```rust
pub const TIOCSERGWILD: u32 = 21588;
```

#### Constant `TIOCSERSWILD`

```rust
pub const TIOCSERSWILD: u32 = 21589;
```

#### Constant `TIOCSLCKTRMIOS`

```rust
pub const TIOCSLCKTRMIOS: u32 = 21591;
```

#### Constant `TIOCSERGSTRUCT`

```rust
pub const TIOCSERGSTRUCT: u32 = 21592;
```

#### Constant `TIOCSERGETLSR`

```rust
pub const TIOCSERGETLSR: u32 = 21593;
```

#### Constant `TIOCSERGETMULTI`

```rust
pub const TIOCSERGETMULTI: u32 = 21594;
```

#### Constant `TIOCSERSETMULTI`

```rust
pub const TIOCSERSETMULTI: u32 = 21595;
```

#### Constant `TIOCMIWAIT`

```rust
pub const TIOCMIWAIT: u32 = 21596;
```

#### Constant `TCGETS`

```rust
pub const TCGETS: u32 = 21505;
```

#### Constant `TCGETA`

```rust
pub const TCGETA: u32 = 21509;
```

#### Constant `TCSBRK`

```rust
pub const TCSBRK: u32 = 21513;
```

#### Constant `TCSBRKP`

```rust
pub const TCSBRKP: u32 = 21541;
```

#### Constant `TCSETA`

```rust
pub const TCSETA: u32 = 21510;
```

#### Constant `TCSETAF`

```rust
pub const TCSETAF: u32 = 21512;
```

#### Constant `TCSETAW`

```rust
pub const TCSETAW: u32 = 21511;
```

#### Constant `TIOCEXCL`

```rust
pub const TIOCEXCL: u32 = 21516;
```

#### Constant `TIOCNXCL`

```rust
pub const TIOCNXCL: u32 = 21517;
```

#### Constant `TIOCGDEV`

```rust
pub const TIOCGDEV: u32 = 2147767346;
```

#### Constant `TIOCGEXCL`

```rust
pub const TIOCGEXCL: u32 = 2147767360;
```

#### Constant `TIOCGICOUNT`

```rust
pub const TIOCGICOUNT: u32 = 21597;
```

#### Constant `TIOCGLCKTRMIOS`

```rust
pub const TIOCGLCKTRMIOS: u32 = 21590;
```

#### Constant `TIOCGPGRP`

```rust
pub const TIOCGPGRP: u32 = 21519;
```

#### Constant `TIOCGPKT`

```rust
pub const TIOCGPKT: u32 = 2147767352;
```

#### Constant `TIOCGPTLCK`

```rust
pub const TIOCGPTLCK: u32 = 2147767353;
```

#### Constant `TIOCGPTN`

```rust
pub const TIOCGPTN: u32 = 2147767344;
```

#### Constant `TIOCGPTPEER`

```rust
pub const TIOCGPTPEER: u32 = 21569;
```

#### Constant `TIOCGRS485`

```rust
pub const TIOCGRS485: u32 = 21550;
```

#### Constant `TIOCGSERIAL`

```rust
pub const TIOCGSERIAL: u32 = 21534;
```

#### Constant `TIOCGSID`

```rust
pub const TIOCGSID: u32 = 21545;
```

#### Constant `TIOCGSOFTCAR`

```rust
pub const TIOCGSOFTCAR: u32 = 21529;
```

#### Constant `TIOCGWINSZ`

```rust
pub const TIOCGWINSZ: u32 = 21523;
```

#### Constant `TCGETS2`

```rust
pub const TCGETS2: u32 = 2150388778;
```

#### Constant `TCGETX`

```rust
pub const TCGETX: u32 = 21554;
```

#### Constant `TCSETS`

```rust
pub const TCSETS: u32 = 21506;
```

#### Constant `TCSETS2`

```rust
pub const TCSETS2: u32 = 1076646955;
```

#### Constant `TCSETSF`

```rust
pub const TCSETSF: u32 = 21508;
```

#### Constant `TCSETSF2`

```rust
pub const TCSETSF2: u32 = 1076646957;
```

#### Constant `TCSETSW`

```rust
pub const TCSETSW: u32 = 21507;
```

#### Constant `TCSETSW2`

```rust
pub const TCSETSW2: u32 = 1076646956;
```

#### Constant `TCSETX`

```rust
pub const TCSETX: u32 = 21555;
```

#### Constant `TCSETXF`

```rust
pub const TCSETXF: u32 = 21556;
```

#### Constant `TCSETXW`

```rust
pub const TCSETXW: u32 = 21557;
```

#### Constant `TIOCGETD`

```rust
pub const TIOCGETD: u32 = 21540;
```

#### Constant `MTIOCGET`

```rust
pub const MTIOCGET: u32 = 2150657282;
```

#### Constant `BLKSSZGET`

```rust
pub const BLKSSZGET: u32 = 4712;
```

#### Constant `BLKPBSZGET`

```rust
pub const BLKPBSZGET: u32 = 4731;
```

#### Constant `BLKROSET`

```rust
pub const BLKROSET: u32 = 4701;
```

#### Constant `BLKROGET`

```rust
pub const BLKROGET: u32 = 4702;
```

#### Constant `BLKRRPART`

```rust
pub const BLKRRPART: u32 = 4703;
```

#### Constant `BLKGETSIZE`

```rust
pub const BLKGETSIZE: u32 = 4704;
```

#### Constant `BLKFLSBUF`

```rust
pub const BLKFLSBUF: u32 = 4705;
```

#### Constant `BLKRASET`

```rust
pub const BLKRASET: u32 = 4706;
```

#### Constant `BLKRAGET`

```rust
pub const BLKRAGET: u32 = 4707;
```

#### Constant `BLKFRASET`

```rust
pub const BLKFRASET: u32 = 4708;
```

#### Constant `BLKFRAGET`

```rust
pub const BLKFRAGET: u32 = 4709;
```

#### Constant `BLKSECTSET`

```rust
pub const BLKSECTSET: u32 = 4710;
```

#### Constant `BLKSECTGET`

```rust
pub const BLKSECTGET: u32 = 4711;
```

#### Constant `BLKPG`

```rust
pub const BLKPG: u32 = 4713;
```

#### Constant `BLKBSZGET`

```rust
pub const BLKBSZGET: u32 = 2148012656;
```

#### Constant `BLKBSZSET`

```rust
pub const BLKBSZSET: u32 = 1074270833;
```

#### Constant `BLKGETSIZE64`

```rust
pub const BLKGETSIZE64: u32 = 2148012658;
```

#### Constant `BLKTRACESETUP`

```rust
pub const BLKTRACESETUP: u32 = 3225948787;
```

#### Constant `BLKTRACESTART`

```rust
pub const BLKTRACESTART: u32 = 4724;
```

#### Constant `BLKTRACESTOP`

```rust
pub const BLKTRACESTOP: u32 = 4725;
```

#### Constant `BLKTRACETEARDOWN`

```rust
pub const BLKTRACETEARDOWN: u32 = 4726;
```

#### Constant `BLKDISCARD`

```rust
pub const BLKDISCARD: u32 = 4727;
```

#### Constant `BLKIOMIN`

```rust
pub const BLKIOMIN: u32 = 4728;
```

#### Constant `BLKIOOPT`

```rust
pub const BLKIOOPT: u32 = 4729;
```

#### Constant `BLKALIGNOFF`

```rust
pub const BLKALIGNOFF: u32 = 4730;
```

#### Constant `BLKDISCARDZEROES`

```rust
pub const BLKDISCARDZEROES: u32 = 4732;
```

#### Constant `BLKSECDISCARD`

```rust
pub const BLKSECDISCARD: u32 = 4733;
```

#### Constant `BLKROTATIONAL`

```rust
pub const BLKROTATIONAL: u32 = 4734;
```

#### Constant `BLKZEROOUT`

```rust
pub const BLKZEROOUT: u32 = 4735;
```

#### Constant `FIEMAP_MAX_OFFSET`

```rust
pub const FIEMAP_MAX_OFFSET: i32 = -1;
```

#### Constant `FIEMAP_FLAG_SYNC`

```rust
pub const FIEMAP_FLAG_SYNC: u32 = 1;
```

#### Constant `FIEMAP_FLAG_XATTR`

```rust
pub const FIEMAP_FLAG_XATTR: u32 = 2;
```

#### Constant `FIEMAP_FLAG_CACHE`

```rust
pub const FIEMAP_FLAG_CACHE: u32 = 4;
```

#### Constant `FIEMAP_FLAGS_COMPAT`

```rust
pub const FIEMAP_FLAGS_COMPAT: u32 = 3;
```

#### Constant `FIEMAP_EXTENT_LAST`

```rust
pub const FIEMAP_EXTENT_LAST: u32 = 1;
```

#### Constant `FIEMAP_EXTENT_UNKNOWN`

```rust
pub const FIEMAP_EXTENT_UNKNOWN: u32 = 2;
```

#### Constant `FIEMAP_EXTENT_DELALLOC`

```rust
pub const FIEMAP_EXTENT_DELALLOC: u32 = 4;
```

#### Constant `FIEMAP_EXTENT_ENCODED`

```rust
pub const FIEMAP_EXTENT_ENCODED: u32 = 8;
```

#### Constant `FIEMAP_EXTENT_DATA_ENCRYPTED`

```rust
pub const FIEMAP_EXTENT_DATA_ENCRYPTED: u32 = 128;
```

#### Constant `FIEMAP_EXTENT_NOT_ALIGNED`

```rust
pub const FIEMAP_EXTENT_NOT_ALIGNED: u32 = 256;
```

#### Constant `FIEMAP_EXTENT_DATA_INLINE`

```rust
pub const FIEMAP_EXTENT_DATA_INLINE: u32 = 512;
```

#### Constant `FIEMAP_EXTENT_DATA_TAIL`

```rust
pub const FIEMAP_EXTENT_DATA_TAIL: u32 = 1024;
```

#### Constant `FIEMAP_EXTENT_UNWRITTEN`

```rust
pub const FIEMAP_EXTENT_UNWRITTEN: u32 = 2048;
```

#### Constant `FIEMAP_EXTENT_MERGED`

```rust
pub const FIEMAP_EXTENT_MERGED: u32 = 4096;
```

#### Constant `FIEMAP_EXTENT_SHARED`

```rust
pub const FIEMAP_EXTENT_SHARED: u32 = 8192;
```

#### Constant `UFFDIO_REGISTER`

```rust
pub const UFFDIO_REGISTER: u32 = 3223366144;
```

#### Constant `UFFDIO_UNREGISTER`

```rust
pub const UFFDIO_UNREGISTER: u32 = 2148575745;
```

#### Constant `UFFDIO_WAKE`

```rust
pub const UFFDIO_WAKE: u32 = 2148575746;
```

#### Constant `UFFDIO_COPY`

```rust
pub const UFFDIO_COPY: u32 = 3223890435;
```

#### Constant `UFFDIO_ZEROPAGE`

```rust
pub const UFFDIO_ZEROPAGE: u32 = 3223366148;
```

#### Constant `UFFDIO_WRITEPROTECT`

```rust
pub const UFFDIO_WRITEPROTECT: u32 = 3222841862;
```

#### Constant `UFFDIO_API`

```rust
pub const UFFDIO_API: u32 = 3222841919;
```

#### Constant `NS_GET_USERNS`

```rust
pub const NS_GET_USERNS: u32 = 46849;
```

#### Constant `NS_GET_PARENT`

```rust
pub const NS_GET_PARENT: u32 = 46850;
```

#### Constant `NS_GET_NSTYPE`

```rust
pub const NS_GET_NSTYPE: u32 = 46851;
```

#### Constant `KDGETLED`

```rust
pub const KDGETLED: u32 = 19249;
```

#### Constant `KDSETLED`

```rust
pub const KDSETLED: u32 = 19250;
```

#### Constant `KDGKBLED`

```rust
pub const KDGKBLED: u32 = 19300;
```

#### Constant `KDSKBLED`

```rust
pub const KDSKBLED: u32 = 19301;
```

#### Constant `KDGKBTYPE`

```rust
pub const KDGKBTYPE: u32 = 19251;
```

#### Constant `KDADDIO`

```rust
pub const KDADDIO: u32 = 19252;
```

#### Constant `KDDELIO`

```rust
pub const KDDELIO: u32 = 19253;
```

#### Constant `KDENABIO`

```rust
pub const KDENABIO: u32 = 19254;
```

#### Constant `KDDISABIO`

```rust
pub const KDDISABIO: u32 = 19255;
```

#### Constant `KDSETMODE`

```rust
pub const KDSETMODE: u32 = 19258;
```

#### Constant `KDGETMODE`

```rust
pub const KDGETMODE: u32 = 19259;
```

#### Constant `KDMKTONE`

```rust
pub const KDMKTONE: u32 = 19248;
```

#### Constant `KIOCSOUND`

```rust
pub const KIOCSOUND: u32 = 19247;
```

#### Constant `GIO_CMAP`

```rust
pub const GIO_CMAP: u32 = 19312;
```

#### Constant `PIO_CMAP`

```rust
pub const PIO_CMAP: u32 = 19313;
```

#### Constant `GIO_FONT`

```rust
pub const GIO_FONT: u32 = 19296;
```

#### Constant `GIO_FONTX`

```rust
pub const GIO_FONTX: u32 = 19307;
```

#### Constant `PIO_FONT`

```rust
pub const PIO_FONT: u32 = 19297;
```

#### Constant `PIO_FONTX`

```rust
pub const PIO_FONTX: u32 = 19308;
```

#### Constant `PIO_FONTRESET`

```rust
pub const PIO_FONTRESET: u32 = 19309;
```

#### Constant `GIO_SCRNMAP`

```rust
pub const GIO_SCRNMAP: u32 = 19264;
```

#### Constant `GIO_UNISCRNMAP`

```rust
pub const GIO_UNISCRNMAP: u32 = 19305;
```

#### Constant `PIO_SCRNMAP`

```rust
pub const PIO_SCRNMAP: u32 = 19265;
```

#### Constant `PIO_UNISCRNMAP`

```rust
pub const PIO_UNISCRNMAP: u32 = 19306;
```

#### Constant `GIO_UNIMAP`

```rust
pub const GIO_UNIMAP: u32 = 19302;
```

#### Constant `PIO_UNIMAP`

```rust
pub const PIO_UNIMAP: u32 = 19303;
```

#### Constant `PIO_UNIMAPCLR`

```rust
pub const PIO_UNIMAPCLR: u32 = 19304;
```

#### Constant `KDGKBMODE`

```rust
pub const KDGKBMODE: u32 = 19268;
```

#### Constant `KDSKBMODE`

```rust
pub const KDSKBMODE: u32 = 19269;
```

#### Constant `KDGKBMETA`

```rust
pub const KDGKBMETA: u32 = 19298;
```

#### Constant `KDSKBMETA`

```rust
pub const KDSKBMETA: u32 = 19299;
```

#### Constant `KDGKBENT`

```rust
pub const KDGKBENT: u32 = 19270;
```

#### Constant `KDSKBENT`

```rust
pub const KDSKBENT: u32 = 19271;
```

#### Constant `KDGKBSENT`

```rust
pub const KDGKBSENT: u32 = 19272;
```

#### Constant `KDSKBSENT`

```rust
pub const KDSKBSENT: u32 = 19273;
```

#### Constant `KDGKBDIACR`

```rust
pub const KDGKBDIACR: u32 = 19274;
```

#### Constant `KDGETKEYCODE`

```rust
pub const KDGETKEYCODE: u32 = 19276;
```

#### Constant `KDSETKEYCODE`

```rust
pub const KDSETKEYCODE: u32 = 19277;
```

#### Constant `KDSIGACCEPT`

```rust
pub const KDSIGACCEPT: u32 = 19278;
```

#### Constant `VT_OPENQRY`

```rust
pub const VT_OPENQRY: u32 = 22016;
```

#### Constant `VT_GETMODE`

```rust
pub const VT_GETMODE: u32 = 22017;
```

#### Constant `VT_SETMODE`

```rust
pub const VT_SETMODE: u32 = 22018;
```

#### Constant `VT_GETSTATE`

```rust
pub const VT_GETSTATE: u32 = 22019;
```

#### Constant `VT_RELDISP`

```rust
pub const VT_RELDISP: u32 = 22021;
```

#### Constant `VT_ACTIVATE`

```rust
pub const VT_ACTIVATE: u32 = 22022;
```

#### Constant `VT_WAITACTIVE`

```rust
pub const VT_WAITACTIVE: u32 = 22023;
```

#### Constant `VT_DISALLOCATE`

```rust
pub const VT_DISALLOCATE: u32 = 22024;
```

#### Constant `VT_RESIZE`

```rust
pub const VT_RESIZE: u32 = 22025;
```

#### Constant `VT_RESIZEX`

```rust
pub const VT_RESIZEX: u32 = 22026;
```

#### Constant `FIOSETOWN`

```rust
pub const FIOSETOWN: u32 = 35073;
```

#### Constant `SIOCSPGRP`

```rust
pub const SIOCSPGRP: u32 = 35074;
```

#### Constant `FIOGETOWN`

```rust
pub const FIOGETOWN: u32 = 35075;
```

#### Constant `SIOCGPGRP`

```rust
pub const SIOCGPGRP: u32 = 35076;
```

#### Constant `SIOCATMARK`

```rust
pub const SIOCATMARK: u32 = 35077;
```

#### Constant `SIOCGSTAMP`

```rust
pub const SIOCGSTAMP: u32 = 35078;
```

#### Constant `TIOCINQ`

```rust
pub const TIOCINQ: u32 = 21531;
```

#### Constant `SIOCADDRT`

```rust
pub const SIOCADDRT: u32 = 35083;
```

#### Constant `SIOCDELRT`

```rust
pub const SIOCDELRT: u32 = 35084;
```

#### Constant `SIOCGIFNAME`

```rust
pub const SIOCGIFNAME: u32 = 35088;
```

#### Constant `SIOCSIFLINK`

```rust
pub const SIOCSIFLINK: u32 = 35089;
```

#### Constant `SIOCGIFCONF`

```rust
pub const SIOCGIFCONF: u32 = 35090;
```

#### Constant `SIOCGIFFLAGS`

```rust
pub const SIOCGIFFLAGS: u32 = 35091;
```

#### Constant `SIOCSIFFLAGS`

```rust
pub const SIOCSIFFLAGS: u32 = 35092;
```

#### Constant `SIOCGIFADDR`

```rust
pub const SIOCGIFADDR: u32 = 35093;
```

#### Constant `SIOCSIFADDR`

```rust
pub const SIOCSIFADDR: u32 = 35094;
```

#### Constant `SIOCGIFDSTADDR`

```rust
pub const SIOCGIFDSTADDR: u32 = 35095;
```

#### Constant `SIOCSIFDSTADDR`

```rust
pub const SIOCSIFDSTADDR: u32 = 35096;
```

#### Constant `SIOCGIFBRDADDR`

```rust
pub const SIOCGIFBRDADDR: u32 = 35097;
```

#### Constant `SIOCSIFBRDADDR`

```rust
pub const SIOCSIFBRDADDR: u32 = 35098;
```

#### Constant `SIOCGIFNETMASK`

```rust
pub const SIOCGIFNETMASK: u32 = 35099;
```

#### Constant `SIOCSIFNETMASK`

```rust
pub const SIOCSIFNETMASK: u32 = 35100;
```

#### Constant `SIOCGIFMETRIC`

```rust
pub const SIOCGIFMETRIC: u32 = 35101;
```

#### Constant `SIOCSIFMETRIC`

```rust
pub const SIOCSIFMETRIC: u32 = 35102;
```

#### Constant `SIOCGIFMEM`

```rust
pub const SIOCGIFMEM: u32 = 35103;
```

#### Constant `SIOCSIFMEM`

```rust
pub const SIOCSIFMEM: u32 = 35104;
```

#### Constant `SIOCGIFMTU`

```rust
pub const SIOCGIFMTU: u32 = 35105;
```

#### Constant `SIOCSIFMTU`

```rust
pub const SIOCSIFMTU: u32 = 35106;
```

#### Constant `SIOCSIFHWADDR`

```rust
pub const SIOCSIFHWADDR: u32 = 35108;
```

#### Constant `SIOCGIFENCAP`

```rust
pub const SIOCGIFENCAP: u32 = 35109;
```

#### Constant `SIOCSIFENCAP`

```rust
pub const SIOCSIFENCAP: u32 = 35110;
```

#### Constant `SIOCGIFHWADDR`

```rust
pub const SIOCGIFHWADDR: u32 = 35111;
```

#### Constant `SIOCGIFSLAVE`

```rust
pub const SIOCGIFSLAVE: u32 = 35113;
```

#### Constant `SIOCSIFSLAVE`

```rust
pub const SIOCSIFSLAVE: u32 = 35120;
```

#### Constant `SIOCADDMULTI`

```rust
pub const SIOCADDMULTI: u32 = 35121;
```

#### Constant `SIOCDELMULTI`

```rust
pub const SIOCDELMULTI: u32 = 35122;
```

#### Constant `SIOCDARP`

```rust
pub const SIOCDARP: u32 = 35155;
```

#### Constant `SIOCGARP`

```rust
pub const SIOCGARP: u32 = 35156;
```

#### Constant `SIOCSARP`

```rust
pub const SIOCSARP: u32 = 35157;
```

#### Constant `SIOCDRARP`

```rust
pub const SIOCDRARP: u32 = 35168;
```

#### Constant `SIOCGRARP`

```rust
pub const SIOCGRARP: u32 = 35169;
```

#### Constant `SIOCSRARP`

```rust
pub const SIOCSRARP: u32 = 35170;
```

#### Constant `SIOCGIFMAP`

```rust
pub const SIOCGIFMAP: u32 = 35184;
```

#### Constant `SIOCSIFMAP`

```rust
pub const SIOCSIFMAP: u32 = 35185;
```

#### Constant `SIOCRTMSG`

```rust
pub const SIOCRTMSG: u32 = 35085;
```

#### Constant `SIOCSIFNAME`

```rust
pub const SIOCSIFNAME: u32 = 35107;
```

#### Constant `SIOCGIFINDEX`

```rust
pub const SIOCGIFINDEX: u32 = 35123;
```

#### Constant `SIOGIFINDEX`

```rust
pub const SIOGIFINDEX: u32 = 35123;
```

#### Constant `SIOCSIFPFLAGS`

```rust
pub const SIOCSIFPFLAGS: u32 = 35124;
```

#### Constant `SIOCGIFPFLAGS`

```rust
pub const SIOCGIFPFLAGS: u32 = 35125;
```

#### Constant `SIOCDIFADDR`

```rust
pub const SIOCDIFADDR: u32 = 35126;
```

#### Constant `SIOCSIFHWBROADCAST`

```rust
pub const SIOCSIFHWBROADCAST: u32 = 35127;
```

#### Constant `SIOCGIFCOUNT`

```rust
pub const SIOCGIFCOUNT: u32 = 35128;
```

#### Constant `SIOCGIFBR`

```rust
pub const SIOCGIFBR: u32 = 35136;
```

#### Constant `SIOCSIFBR`

```rust
pub const SIOCSIFBR: u32 = 35137;
```

#### Constant `SIOCGIFTXQLEN`

```rust
pub const SIOCGIFTXQLEN: u32 = 35138;
```

#### Constant `SIOCSIFTXQLEN`

```rust
pub const SIOCSIFTXQLEN: u32 = 35139;
```

#### Constant `SIOCADDDLCI`

```rust
pub const SIOCADDDLCI: u32 = 35200;
```

#### Constant `SIOCDELDLCI`

```rust
pub const SIOCDELDLCI: u32 = 35201;
```

#### Constant `SIOCDEVPRIVATE`

```rust
pub const SIOCDEVPRIVATE: u32 = 35312;
```

#### Constant `SIOCPROTOPRIVATE`

```rust
pub const SIOCPROTOPRIVATE: u32 = 35296;
```

#### Constant `FIBMAP`

```rust
pub const FIBMAP: u32 = 1;
```

#### Constant `FIGETBSZ`

```rust
pub const FIGETBSZ: u32 = 2;
```

#### Constant `FIFREEZE`

```rust
pub const FIFREEZE: u32 = 3221510263;
```

#### Constant `FITHAW`

```rust
pub const FITHAW: u32 = 3221510264;
```

#### Constant `FITRIM`

```rust
pub const FITRIM: u32 = 3222820985;
```

#### Constant `FICLONE`

```rust
pub const FICLONE: u32 = 1074041865;
```

#### Constant `FICLONERANGE`

```rust
pub const FICLONERANGE: u32 = 1075876877;
```

#### Constant `FIDEDUPERANGE`

```rust
pub const FIDEDUPERANGE: u32 = 3222836278;
```

#### Constant `FS_IOC_GETFLAGS`

```rust
pub const FS_IOC_GETFLAGS: u32 = 2148034049;
```

#### Constant `FS_IOC_SETFLAGS`

```rust
pub const FS_IOC_SETFLAGS: u32 = 1074292226;
```

#### Constant `FS_IOC_GETVERSION`

```rust
pub const FS_IOC_GETVERSION: u32 = 2148038145;
```

#### Constant `FS_IOC_SETVERSION`

```rust
pub const FS_IOC_SETVERSION: u32 = 1074296322;
```

#### Constant `FS_IOC_FIEMAP`

```rust
pub const FS_IOC_FIEMAP: u32 = 3223348747;
```

#### Constant `FS_IOC32_GETFLAGS`

```rust
pub const FS_IOC32_GETFLAGS: u32 = 2147771905;
```

#### Constant `FS_IOC32_SETFLAGS`

```rust
pub const FS_IOC32_SETFLAGS: u32 = 1074030082;
```

#### Constant `FS_IOC32_GETVERSION`

```rust
pub const FS_IOC32_GETVERSION: u32 = 2147776001;
```

#### Constant `FS_IOC32_SETVERSION`

```rust
pub const FS_IOC32_SETVERSION: u32 = 1074034178;
```

#### Constant `FS_IOC_FSGETXATTR`

```rust
pub const FS_IOC_FSGETXATTR: u32 = 2149341215;
```

#### Constant `FS_IOC_FSSETXATTR`

```rust
pub const FS_IOC_FSSETXATTR: u32 = 1075599392;
```

#### Constant `FS_IOC_GETFSLABEL`

```rust
pub const FS_IOC_GETFSLABEL: u32 = 2164298801;
```

#### Constant `FS_IOC_SETFSLABEL`

```rust
pub const FS_IOC_SETFSLABEL: u32 = 1090556978;
```

#### Constant `EXT4_IOC_GETVERSION`

```rust
pub const EXT4_IOC_GETVERSION: u32 = 2148034051;
```

#### Constant `EXT4_IOC_SETVERSION`

```rust
pub const EXT4_IOC_SETVERSION: u32 = 1074292228;
```

#### Constant `EXT4_IOC_GETVERSION_OLD`

```rust
pub const EXT4_IOC_GETVERSION_OLD: u32 = 2148038145;
```

#### Constant `EXT4_IOC_SETVERSION_OLD`

```rust
pub const EXT4_IOC_SETVERSION_OLD: u32 = 1074296322;
```

#### Constant `EXT4_IOC_GETRSVSZ`

```rust
pub const EXT4_IOC_GETRSVSZ: u32 = 2148034053;
```

#### Constant `EXT4_IOC_SETRSVSZ`

```rust
pub const EXT4_IOC_SETRSVSZ: u32 = 1074292230;
```

#### Constant `EXT4_IOC_GROUP_EXTEND`

```rust
pub const EXT4_IOC_GROUP_EXTEND: u32 = 1074292231;
```

#### Constant `EXT4_IOC_MIGRATE`

```rust
pub const EXT4_IOC_MIGRATE: u32 = 26121;
```

#### Constant `EXT4_IOC_ALLOC_DA_BLKS`

```rust
pub const EXT4_IOC_ALLOC_DA_BLKS: u32 = 26124;
```

#### Constant `EXT4_IOC_RESIZE_FS`

```rust
pub const EXT4_IOC_RESIZE_FS: u32 = 1074292240;
```

#### Constant `EXT4_IOC_SWAP_BOOT`

```rust
pub const EXT4_IOC_SWAP_BOOT: u32 = 26129;
```

#### Constant `EXT4_IOC_PRECACHE_EXTENTS`

```rust
pub const EXT4_IOC_PRECACHE_EXTENTS: u32 = 26130;
```

#### Constant `EXT4_IOC_CLEAR_ES_CACHE`

```rust
pub const EXT4_IOC_CLEAR_ES_CACHE: u32 = 26152;
```

#### Constant `EXT4_IOC_GETSTATE`

```rust
pub const EXT4_IOC_GETSTATE: u32 = 1074030121;
```

#### Constant `EXT4_IOC_GET_ES_CACHE`

```rust
pub const EXT4_IOC_GET_ES_CACHE: u32 = 3223348778;
```

#### Constant `EXT4_IOC_CHECKPOINT`

```rust
pub const EXT4_IOC_CHECKPOINT: u32 = 1074030123;
```

#### Constant `EXT4_IOC_SHUTDOWN`

```rust
pub const EXT4_IOC_SHUTDOWN: u32 = 2147768445;
```

#### Constant `EXT4_IOC32_GETVERSION`

```rust
pub const EXT4_IOC32_GETVERSION: u32 = 2147771907;
```

#### Constant `EXT4_IOC32_SETVERSION`

```rust
pub const EXT4_IOC32_SETVERSION: u32 = 1074030084;
```

#### Constant `EXT4_IOC32_GETRSVSZ`

```rust
pub const EXT4_IOC32_GETRSVSZ: u32 = 2147771909;
```

#### Constant `EXT4_IOC32_SETRSVSZ`

```rust
pub const EXT4_IOC32_SETRSVSZ: u32 = 1074030086;
```

#### Constant `EXT4_IOC32_GROUP_EXTEND`

```rust
pub const EXT4_IOC32_GROUP_EXTEND: u32 = 1074030087;
```

#### Constant `EXT4_IOC32_GETVERSION_OLD`

```rust
pub const EXT4_IOC32_GETVERSION_OLD: u32 = 2147776001;
```

#### Constant `EXT4_IOC32_SETVERSION_OLD`

```rust
pub const EXT4_IOC32_SETVERSION_OLD: u32 = 1074034178;
```

#### Constant `VIDIOC_SUBDEV_QUERYSTD`

```rust
pub const VIDIOC_SUBDEV_QUERYSTD: u32 = 2148030015;
```

#### Constant `AUTOFS_DEV_IOCTL_CLOSEMOUNT`

```rust
pub const AUTOFS_DEV_IOCTL_CLOSEMOUNT: u32 = 3222836085;
```

#### Constant `LIRC_SET_SEND_CARRIER`

```rust
pub const LIRC_SET_SEND_CARRIER: u32 = 1074030867;
```

#### Constant `AUTOFS_IOC_PROTOSUBVER`

```rust
pub const AUTOFS_IOC_PROTOSUBVER: u32 = 2147783527;
```

#### Constant `PTP_SYS_OFFSET_PRECISE`

```rust
pub const PTP_SYS_OFFSET_PRECISE: u32 = 3225435400;
```

#### Constant `FSI_SCOM_WRITE`

```rust
pub const FSI_SCOM_WRITE: u32 = 3223352066;
```

#### Constant `ATM_GETCIRANGE`

```rust
pub const ATM_GETCIRANGE: u32 = 1074815370;
```

#### Constant `DMA_BUF_SET_NAME_B`

```rust
pub const DMA_BUF_SET_NAME_B: u32 = 1074291201;
```

#### Constant `RIO_CM_EP_GET_LIST_SIZE`

```rust
pub const RIO_CM_EP_GET_LIST_SIZE: u32 = 3221512961;
```

#### Constant `TUNSETPERSIST`

```rust
pub const TUNSETPERSIST: u32 = 1074025675;
```

#### Constant `FS_IOC_GET_ENCRYPTION_POLICY`

```rust
pub const FS_IOC_GET_ENCRYPTION_POLICY: u32 = 1074554389;
```

#### Constant `CEC_RECEIVE`

```rust
pub const CEC_RECEIVE: u32 = 3224920326;
```

#### Constant `MGSL_IOCGPARAMS`

```rust
pub const MGSL_IOCGPARAMS: u32 = 2150657281;
```

#### Constant `ENI_SETMULT`

```rust
pub const ENI_SETMULT: u32 = 1074815335;
```

#### Constant `RIO_GET_EVENT_MASK`

```rust
pub const RIO_GET_EVENT_MASK: u32 = 2147773710;
```

#### Constant `LIRC_GET_MAX_TIMEOUT`

```rust
pub const LIRC_GET_MAX_TIMEOUT: u32 = 2147772681;
```

#### Constant `USBDEVFS_CLAIMINTERFACE`

```rust
pub const USBDEVFS_CLAIMINTERFACE: u32 = 2147767567;
```

#### Constant `CHIOMOVE`

```rust
pub const CHIOMOVE: u32 = 1075077889;
```

#### Constant `SONYPI_IOCGBATFLAGS`

```rust
pub const SONYPI_IOCGBATFLAGS: u32 = 2147579399;
```

#### Constant `BTRFS_IOC_SYNC`

```rust
pub const BTRFS_IOC_SYNC: u32 = 37896;
```

#### Constant `VIDIOC_TRY_FMT`

```rust
pub const VIDIOC_TRY_FMT: u32 = 3234879040;
```

#### Constant `LIRC_SET_REC_MODE`

```rust
pub const LIRC_SET_REC_MODE: u32 = 1074030866;
```

#### Constant `VIDIOC_DQEVENT`

```rust
pub const VIDIOC_DQEVENT: u32 = 2156418649;
```

#### Constant `RPMSG_DESTROY_EPT_IOCTL`

```rust
pub const RPMSG_DESTROY_EPT_IOCTL: u32 = 46338;
```

#### Constant `UVCIOC_CTRL_MAP`

```rust
pub const UVCIOC_CTRL_MAP: u32 = 3227546912;
```

#### Constant `VHOST_SET_BACKEND_FEATURES`

```rust
pub const VHOST_SET_BACKEND_FEATURES: u32 = 1074310949;
```

#### Constant `VHOST_VSOCK_SET_GUEST_CID`

```rust
pub const VHOST_VSOCK_SET_GUEST_CID: u32 = 1074311008;
```

#### Constant `UI_SET_KEYBIT`

```rust
pub const UI_SET_KEYBIT: u32 = 1074025829;
```

#### Constant `LIRC_SET_REC_TIMEOUT`

```rust
pub const LIRC_SET_REC_TIMEOUT: u32 = 1074030872;
```

#### Constant `FS_IOC_GET_ENCRYPTION_KEY_STATUS`

```rust
pub const FS_IOC_GET_ENCRYPTION_KEY_STATUS: u32 = 3229640218;
```

#### Constant `BTRFS_IOC_TREE_SEARCH_V2`

```rust
pub const BTRFS_IOC_TREE_SEARCH_V2: u32 = 3228603409;
```

#### Constant `VHOST_SET_VRING_BASE`

```rust
pub const VHOST_SET_VRING_BASE: u32 = 1074310930;
```

#### Constant `RIO_ENABLE_DOORBELL_RANGE`

```rust
pub const RIO_ENABLE_DOORBELL_RANGE: u32 = 1074294025;
```

#### Constant `VIDIOC_TRY_EXT_CTRLS`

```rust
pub const VIDIOC_TRY_EXT_CTRLS: u32 = 3223344713;
```

#### Constant `LIRC_GET_REC_MODE`

```rust
pub const LIRC_GET_REC_MODE: u32 = 2147772674;
```

#### Constant `PPGETTIME`

```rust
pub const PPGETTIME: u32 = 2148561045;
```

#### Constant `BTRFS_IOC_RM_DEV`

```rust
pub const BTRFS_IOC_RM_DEV: u32 = 1342215179;
```

#### Constant `ATM_SETBACKEND`

```rust
pub const ATM_SETBACKEND: u32 = 1073897970;
```

#### Constant `FSL_HV_IOCTL_PARTITION_START`

```rust
pub const FSL_HV_IOCTL_PARTITION_START: u32 = 3222318851;
```

#### Constant `FBIO_WAITEVENT`

```rust
pub const FBIO_WAITEVENT: u32 = 18056;
```

#### Constant `SWITCHTEC_IOCTL_PORT_TO_PFF`

```rust
pub const SWITCHTEC_IOCTL_PORT_TO_PFF: u32 = 3222034245;
```

#### Constant `NVME_IOCTL_IO_CMD`

```rust
pub const NVME_IOCTL_IO_CMD: u32 = 3225964099;
```

#### Constant `IPMICTL_RECEIVE_MSG_TRUNC`

```rust
pub const IPMICTL_RECEIVE_MSG_TRUNC: u32 = 3224398091;
```

#### Constant `FDTWADDLE`

```rust
pub const FDTWADDLE: u32 = 601;
```

#### Constant `NVME_IOCTL_SUBMIT_IO`

```rust
pub const NVME_IOCTL_SUBMIT_IO: u32 = 1076907586;
```

#### Constant `NILFS_IOCTL_SYNC`

```rust
pub const NILFS_IOCTL_SYNC: u32 = 2148036234;
```

#### Constant `VIDIOC_SUBDEV_S_DV_TIMINGS`

```rust
pub const VIDIOC_SUBDEV_S_DV_TIMINGS: u32 = 3229898327;
```

#### Constant `ASPEED_LPC_CTRL_IOCTL_GET_SIZE`

```rust
pub const ASPEED_LPC_CTRL_IOCTL_GET_SIZE: u32 = 3222319616;
```

#### Constant `DM_DEV_STATUS`

```rust
pub const DM_DEV_STATUS: u32 = 3241737479;
```

#### Constant `TEE_IOC_CLOSE_SESSION`

```rust
pub const TEE_IOC_CLOSE_SESSION: u32 = 2147787781;
```

#### Constant `NS_GETPSTAT`

```rust
pub const NS_GETPSTAT: u32 = 3222298977;
```

#### Constant `UI_SET_PROPBIT`

```rust
pub const UI_SET_PROPBIT: u32 = 1074025838;
```

#### Constant `TUNSETFILTEREBPF`

```rust
pub const TUNSETFILTEREBPF: u32 = 2147767521;
```

#### Constant `RIO_MPORT_MAINT_COMPTAG_SET`

```rust
pub const RIO_MPORT_MAINT_COMPTAG_SET: u32 = 1074031874;
```

#### Constant `AUTOFS_DEV_IOCTL_VERSION`

```rust
pub const AUTOFS_DEV_IOCTL_VERSION: u32 = 3222836081;
```

#### Constant `WDIOC_SETOPTIONS`

```rust
pub const WDIOC_SETOPTIONS: u32 = 2147768068;
```

#### Constant `VHOST_SCSI_SET_ENDPOINT`

```rust
pub const VHOST_SCSI_SET_ENDPOINT: u32 = 1088991040;
```

#### Constant `MGSL_IOCGTXIDLE`

```rust
pub const MGSL_IOCGTXIDLE: u32 = 27907;
```

#### Constant `ATM_ADDLECSADDR`

```rust
pub const ATM_ADDLECSADDR: u32 = 1074815374;
```

#### Constant `FSL_HV_IOCTL_GETPROP`

```rust
pub const FSL_HV_IOCTL_GETPROP: u32 = 3223891719;
```

#### Constant `FDGETPRM`

```rust
pub const FDGETPRM: u32 = 2149581316;
```

#### Constant `HIDIOCAPPLICATION`

```rust
pub const HIDIOCAPPLICATION: u32 = 18434;
```

#### Constant `ENI_MEMDUMP`

```rust
pub const ENI_MEMDUMP: u32 = 1074815328;
```

#### Constant `PTP_SYS_OFFSET2`

```rust
pub const PTP_SYS_OFFSET2: u32 = 1128283406;
```

#### Constant `VIDIOC_SUBDEV_G_DV_TIMINGS`

```rust
pub const VIDIOC_SUBDEV_G_DV_TIMINGS: u32 = 3229898328;
```

#### Constant `DMA_BUF_SET_NAME_A`

```rust
pub const DMA_BUF_SET_NAME_A: u32 = 1074029057;
```

#### Constant `PTP_PIN_GETFUNC`

```rust
pub const PTP_PIN_GETFUNC: u32 = 3227532550;
```

#### Constant `PTP_SYS_OFFSET_EXTENDED`

```rust
pub const PTP_SYS_OFFSET_EXTENDED: u32 = 3300932873;
```

#### Constant `DFL_FPGA_PORT_UINT_SET_IRQ`

```rust
pub const DFL_FPGA_PORT_UINT_SET_IRQ: u32 = 1074312776;
```

#### Constant `RTC_EPOCH_READ`

```rust
pub const RTC_EPOCH_READ: u32 = 2148036621;
```

#### Constant `VIDIOC_SUBDEV_S_SELECTION`

```rust
pub const VIDIOC_SUBDEV_S_SELECTION: u32 = 3225441854;
```

#### Constant `VIDIOC_QUERY_EXT_CTRL`

```rust
pub const VIDIOC_QUERY_EXT_CTRL: u32 = 3236451943;
```

#### Constant `ATM_GETLECSADDR`

```rust
pub const ATM_GETLECSADDR: u32 = 1074815376;
```

#### Constant `FSL_HV_IOCTL_PARTITION_STOP`

```rust
pub const FSL_HV_IOCTL_PARTITION_STOP: u32 = 3221794564;
```

#### Constant `SONET_GETDIAG`

```rust
pub const SONET_GETDIAG: u32 = 2147770644;
```

#### Constant `ATMMPC_DATA`

```rust
pub const ATMMPC_DATA: u32 = 25049;
```

#### Constant `IPMICTL_UNREGISTER_FOR_CMD_CHANS`

```rust
pub const IPMICTL_UNREGISTER_FOR_CMD_CHANS: u32 = 2148296989;
```

#### Constant `HIDIOCGCOLLECTIONINDEX`

```rust
pub const HIDIOCGCOLLECTIONINDEX: u32 = 1075333136;
```

#### Constant `RPMSG_CREATE_EPT_IOCTL`

```rust
pub const RPMSG_CREATE_EPT_IOCTL: u32 = 1076409601;
```

#### Constant `GPIOHANDLE_GET_LINE_VALUES_IOCTL`

```rust
pub const GPIOHANDLE_GET_LINE_VALUES_IOCTL: u32 = 3225465864;
```

#### Constant `UI_DEV_SETUP`

```rust
pub const UI_DEV_SETUP: u32 = 1079792899;
```

#### Constant `ISST_IF_IO_CMD`

```rust
pub const ISST_IF_IO_CMD: u32 = 1074331138;
```

#### Constant `RIO_MPORT_MAINT_READ_REMOTE`

```rust
pub const RIO_MPORT_MAINT_READ_REMOTE: u32 = 2149084423;
```

#### Constant `VIDIOC_OMAP3ISP_HIST_CFG`

```rust
pub const VIDIOC_OMAP3ISP_HIST_CFG: u32 = 3224393412;
```

#### Constant `BLKGETNRZONES`

```rust
pub const BLKGETNRZONES: u32 = 2147750533;
```

#### Constant `VIDIOC_G_MODULATOR`

```rust
pub const VIDIOC_G_MODULATOR: u32 = 3225703990;
```

#### Constant `VBG_IOCTL_WRITE_CORE_DUMP`

```rust
pub const VBG_IOCTL_WRITE_CORE_DUMP: u32 = 3223082515;
```

#### Constant `USBDEVFS_SETINTERFACE`

```rust
pub const USBDEVFS_SETINTERFACE: u32 = 2148029700;
```

#### Constant `PPPIOCGCHAN`

```rust
pub const PPPIOCGCHAN: u32 = 2147775543;
```

#### Constant `EVIOCGVERSION`

```rust
pub const EVIOCGVERSION: u32 = 2147763457;
```

#### Constant `VHOST_NET_SET_BACKEND`

```rust
pub const VHOST_NET_SET_BACKEND: u32 = 1074310960;
```

#### Constant `USBDEVFS_REAPURBNDELAY`

```rust
pub const USBDEVFS_REAPURBNDELAY: u32 = 1074287885;
```

#### Constant `RNDZAPENTCNT`

```rust
pub const RNDZAPENTCNT: u32 = 20996;
```

#### Constant `VIDIOC_G_PARM`

```rust
pub const VIDIOC_G_PARM: u32 = 3234616853;
```

#### Constant `TUNGETDEVNETNS`

```rust
pub const TUNGETDEVNETNS: u32 = 21731;
```

#### Constant `LIRC_SET_MEASURE_CARRIER_MODE`

```rust
pub const LIRC_SET_MEASURE_CARRIER_MODE: u32 = 1074030877;
```

#### Constant `VHOST_SET_VRING_ERR`

```rust
pub const VHOST_SET_VRING_ERR: u32 = 1074310946;
```

#### Constant `VDUSE_VQ_SETUP`

```rust
pub const VDUSE_VQ_SETUP: u32 = 1075872020;
```

#### Constant `AUTOFS_IOC_SETTIMEOUT`

```rust
pub const AUTOFS_IOC_SETTIMEOUT: u32 = 3221787492;
```

#### Constant `VIDIOC_S_FREQUENCY`

```rust
pub const VIDIOC_S_FREQUENCY: u32 = 1076647481;
```

#### Constant `F2FS_IOC_SEC_TRIM_FILE`

```rust
pub const F2FS_IOC_SEC_TRIM_FILE: u32 = 1075377428;
```

#### Constant `FS_IOC_REMOVE_ENCRYPTION_KEY`

```rust
pub const FS_IOC_REMOVE_ENCRYPTION_KEY: u32 = 3225445912;
```

#### Constant `WDIOC_GETPRETIMEOUT`

```rust
pub const WDIOC_GETPRETIMEOUT: u32 = 2147768073;
```

#### Constant `USBDEVFS_DROP_PRIVILEGES`

```rust
pub const USBDEVFS_DROP_PRIVILEGES: u32 = 1074025758;
```

#### Constant `BTRFS_IOC_SNAP_CREATE_V2`

```rust
pub const BTRFS_IOC_SNAP_CREATE_V2: u32 = 1342215191;
```

#### Constant `VHOST_VSOCK_SET_RUNNING`

```rust
pub const VHOST_VSOCK_SET_RUNNING: u32 = 1074048865;
```

#### Constant `STP_SET_OPTIONS`

```rust
pub const STP_SET_OPTIONS: u32 = 1074275586;
```

#### Constant `FBIO_RADEON_GET_MIRROR`

```rust
pub const FBIO_RADEON_GET_MIRROR: u32 = 2148024323;
```

#### Constant `IVTVFB_IOC_DMA_FRAME`

```rust
pub const IVTVFB_IOC_DMA_FRAME: u32 = 1075336896;
```

#### Constant `IPMICTL_SEND_COMMAND`

```rust
pub const IPMICTL_SEND_COMMAND: u32 = 2150131981;
```

#### Constant `VIDIOC_G_ENC_INDEX`

```rust
pub const VIDIOC_G_ENC_INDEX: u32 = 2283296332;
```

#### Constant `DFL_FPGA_FME_PORT_PR`

```rust
pub const DFL_FPGA_FME_PORT_PR: u32 = 46720;
```

#### Constant `CHIOSVOLTAG`

```rust
pub const CHIOSVOLTAG: u32 = 1076912914;
```

#### Constant `ATM_SETESIF`

```rust
pub const ATM_SETESIF: u32 = 1074815373;
```

#### Constant `FW_CDEV_IOC_SEND_RESPONSE`

```rust
pub const FW_CDEV_IOC_SEND_RESPONSE: u32 = 1075323652;
```

#### Constant `PMU_IOC_GET_MODEL`

```rust
pub const PMU_IOC_GET_MODEL: u32 = 2148024835;
```

#### Constant `JSIOCGBTNMAP`

```rust
pub const JSIOCGBTNMAP: u32 = 2214619700;
```

#### Constant `USBDEVFS_HUB_PORTINFO`

```rust
pub const USBDEVFS_HUB_PORTINFO: u32 = 2155894035;
```

#### Constant `VBG_IOCTL_INTERRUPT_ALL_WAIT_FOR_EVENTS`

```rust
pub const VBG_IOCTL_INTERRUPT_ALL_WAIT_FOR_EVENTS: u32 = 3222820363;
```

#### Constant `FDCLRPRM`

```rust
pub const FDCLRPRM: u32 = 577;
```

#### Constant `BTRFS_IOC_SCRUB`

```rust
pub const BTRFS_IOC_SCRUB: u32 = 3288372251;
```

#### Constant `USBDEVFS_DISCONNECT`

```rust
pub const USBDEVFS_DISCONNECT: u32 = 21782;
```

#### Constant `TUNSETVNETBE`

```rust
pub const TUNSETVNETBE: u32 = 1074025694;
```

#### Constant `ATMTCP_REMOVE`

```rust
pub const ATMTCP_REMOVE: u32 = 24975;
```

#### Constant `VHOST_VDPA_GET_CONFIG`

```rust
pub const VHOST_VDPA_GET_CONFIG: u32 = 2148052851;
```

#### Constant `PPPIOCGNPMODE`

```rust
pub const PPPIOCGNPMODE: u32 = 3221779532;
```

#### Constant `FDGETDRVPRM`

```rust
pub const FDGETDRVPRM: u32 = 2155872785;
```

#### Constant `TUNSETVNETLE`

```rust
pub const TUNSETVNETLE: u32 = 1074025692;
```

#### Constant `PHN_SETREG`

```rust
pub const PHN_SETREG: u32 = 1074294790;
```

#### Constant `PPPIOCDETACH`

```rust
pub const PPPIOCDETACH: u32 = 1074033724;
```

#### Constant `MMTIMER_GETRES`

```rust
pub const MMTIMER_GETRES: u32 = 2148035841;
```

#### Constant `VIDIOC_SUBDEV_ENUMSTD`

```rust
pub const VIDIOC_SUBDEV_ENUMSTD: u32 = 3225966105;
```

#### Constant `PPGETFLAGS`

```rust
pub const PPGETFLAGS: u32 = 2147774618;
```

#### Constant `VDUSE_DEV_GET_FEATURES`

```rust
pub const VDUSE_DEV_GET_FEATURES: u32 = 2148040977;
```

#### Constant `CAPI_MANUFACTURER_CMD`

```rust
pub const CAPI_MANUFACTURER_CMD: u32 = 3222291232;
```

#### Constant `VIDIOC_G_TUNER`

```rust
pub const VIDIOC_G_TUNER: u32 = 3226752541;
```

#### Constant `DM_TABLE_STATUS`

```rust
pub const DM_TABLE_STATUS: u32 = 3241737484;
```

#### Constant `DM_DEV_ARM_POLL`

```rust
pub const DM_DEV_ARM_POLL: u32 = 3241737488;
```

#### Constant `NE_CREATE_VM`

```rust
pub const NE_CREATE_VM: u32 = 2148052512;
```

#### Constant `MEDIA_IOC_ENUM_LINKS`

```rust
pub const MEDIA_IOC_ENUM_LINKS: u32 = 3223878658;
```

#### Constant `F2FS_IOC_PRECACHE_EXTENTS`

```rust
pub const F2FS_IOC_PRECACHE_EXTENTS: u32 = 62735;
```

#### Constant `DFL_FPGA_PORT_DMA_MAP`

```rust
pub const DFL_FPGA_PORT_DMA_MAP: u32 = 46659;
```

#### Constant `MGSL_IOCGXCTRL`

```rust
pub const MGSL_IOCGXCTRL: u32 = 27926;
```

#### Constant `FW_CDEV_IOC_SEND_REQUEST`

```rust
pub const FW_CDEV_IOC_SEND_REQUEST: u32 = 1076372225;
```

#### Constant `SONYPI_IOCGBLUE`

```rust
pub const SONYPI_IOCGBLUE: u32 = 2147579400;
```

#### Constant `F2FS_IOC_DECOMPRESS_FILE`

```rust
pub const F2FS_IOC_DECOMPRESS_FILE: u32 = 62743;
```

#### Constant `I2OHTML`

```rust
pub const I2OHTML: u32 = 3224398089;
```

#### Constant `VFIO_GET_API_VERSION`

```rust
pub const VFIO_GET_API_VERSION: u32 = 15204;
```

#### Constant `IDT77105_GETSTATZ`

```rust
pub const IDT77105_GETSTATZ: u32 = 1074815283;
```

#### Constant `I2OPARMSET`

```rust
pub const I2OPARMSET: u32 = 3223873795;
```

#### Constant `TEE_IOC_CANCEL`

```rust
pub const TEE_IOC_CANCEL: u32 = 2148049924;
```

#### Constant `PTP_SYS_OFFSET_PRECISE2`

```rust
pub const PTP_SYS_OFFSET_PRECISE2: u32 = 3225435409;
```

#### Constant `DFL_FPGA_PORT_RESET`

```rust
pub const DFL_FPGA_PORT_RESET: u32 = 46656;
```

#### Constant `PPPIOCGASYNCMAP`

```rust
pub const PPPIOCGASYNCMAP: u32 = 2147775576;
```

#### Constant `EVIOCGKEYCODE_V2`

```rust
pub const EVIOCGKEYCODE_V2: u32 = 2150122756;
```

#### Constant `DM_DEV_SET_GEOMETRY`

```rust
pub const DM_DEV_SET_GEOMETRY: u32 = 3241737487;
```

#### Constant `HIDIOCSUSAGE`

```rust
pub const HIDIOCSUSAGE: u32 = 1075333132;
```

#### Constant `FW_CDEV_IOC_DEALLOCATE_ISO_RESOURCE_ONCE`

```rust
pub const FW_CDEV_IOC_DEALLOCATE_ISO_RESOURCE_ONCE: u32 = 1075323664;
```

#### Constant `PTP_EXTTS_REQUEST`

```rust
pub const PTP_EXTTS_REQUEST: u32 = 1074806018;
```

#### Constant `SWITCHTEC_IOCTL_EVENT_CTL`

```rust
pub const SWITCHTEC_IOCTL_EVENT_CTL: u32 = 3223869251;
```

#### Constant `WDIOC_SETPRETIMEOUT`

```rust
pub const WDIOC_SETPRETIMEOUT: u32 = 3221509896;
```

#### Constant `VHOST_SCSI_CLEAR_ENDPOINT`

```rust
pub const VHOST_SCSI_CLEAR_ENDPOINT: u32 = 1088991041;
```

#### Constant `JSIOCGAXES`

```rust
pub const JSIOCGAXES: u32 = 2147576337;
```

#### Constant `HIDIOCSFLAG`

```rust
pub const HIDIOCSFLAG: u32 = 1074022415;
```

#### Constant `PTP_PEROUT_REQUEST2`

```rust
pub const PTP_PEROUT_REQUEST2: u32 = 1077427468;
```

#### Constant `PPWDATA`

```rust
pub const PPWDATA: u32 = 1073836166;
```

#### Constant `PTP_CLOCK_GETCAPS`

```rust
pub const PTP_CLOCK_GETCAPS: u32 = 2152742145;
```

#### Constant `FDGETMAXERRS`

```rust
pub const FDGETMAXERRS: u32 = 2148794894;
```

#### Constant `TUNSETQUEUE`

```rust
pub const TUNSETQUEUE: u32 = 1074025689;
```

#### Constant `PTP_ENABLE_PPS`

```rust
pub const PTP_ENABLE_PPS: u32 = 1074019588;
```

#### Constant `SIOCSIFATMTCP`

```rust
pub const SIOCSIFATMTCP: u32 = 24960;
```

#### Constant `CEC_ADAP_G_LOG_ADDRS`

```rust
pub const CEC_ADAP_G_LOG_ADDRS: u32 = 2153537795;
```

#### Constant `ND_IOCTL_ARS_CAP`

```rust
pub const ND_IOCTL_ARS_CAP: u32 = 3223342593;
```

#### Constant `NBD_SET_BLKSIZE`

```rust
pub const NBD_SET_BLKSIZE: u32 = 43777;
```

#### Constant `NBD_SET_TIMEOUT`

```rust
pub const NBD_SET_TIMEOUT: u32 = 43785;
```

#### Constant `VHOST_SCSI_GET_ABI_VERSION`

```rust
pub const VHOST_SCSI_GET_ABI_VERSION: u32 = 1074048834;
```

#### Constant `RIO_UNMAP_INBOUND`

```rust
pub const RIO_UNMAP_INBOUND: u32 = 1074294034;
```

#### Constant `ATM_QUERYLOOP`

```rust
pub const ATM_QUERYLOOP: u32 = 1074815316;
```

#### Constant `DFL_FPGA_GET_API_VERSION`

```rust
pub const DFL_FPGA_GET_API_VERSION: u32 = 46592;
```

#### Constant `USBDEVFS_WAIT_FOR_RESUME`

```rust
pub const USBDEVFS_WAIT_FOR_RESUME: u32 = 21795;
```

#### Constant `FBIO_CURSOR`

```rust
pub const FBIO_CURSOR: u32 = 3228059144;
```

#### Constant `RNDCLEARPOOL`

```rust
pub const RNDCLEARPOOL: u32 = 20998;
```

#### Constant `VIDIOC_QUERYSTD`

```rust
pub const VIDIOC_QUERYSTD: u32 = 2148030015;
```

#### Constant `DMA_BUF_IOCTL_SYNC`

```rust
pub const DMA_BUF_IOCTL_SYNC: u32 = 1074291200;
```

#### Constant `SCIF_RECV`

```rust
pub const SCIF_RECV: u32 = 3222827783;
```

#### Constant `PTP_PIN_GETFUNC2`

```rust
pub const PTP_PIN_GETFUNC2: u32 = 3227532559;
```

#### Constant `FW_CDEV_IOC_ALLOCATE`

```rust
pub const FW_CDEV_IOC_ALLOCATE: u32 = 3223331586;
```

#### Constant `CEC_ADAP_G_CAPS`

```rust
pub const CEC_ADAP_G_CAPS: u32 = 3226231040;
```

#### Constant `VIDIOC_G_FBUF`

```rust
pub const VIDIOC_G_FBUF: u32 = 2150651402;
```

#### Constant `PTP_ENABLE_PPS2`

```rust
pub const PTP_ENABLE_PPS2: u32 = 1074019597;
```

#### Constant `PCITEST_CLEAR_IRQ`

```rust
pub const PCITEST_CLEAR_IRQ: u32 = 20496;
```

#### Constant `IPMICTL_SET_GETS_EVENTS_CMD`

```rust
pub const IPMICTL_SET_GETS_EVENTS_CMD: u32 = 2147772688;
```

#### Constant `BTRFS_IOC_DEVICES_READY`

```rust
pub const BTRFS_IOC_DEVICES_READY: u32 = 2415957031;
```

#### Constant `JSIOCGAXMAP`

```rust
pub const JSIOCGAXMAP: u32 = 2151705138;
```

#### Constant `FW_CDEV_IOC_GET_CYCLE_TIMER`

```rust
pub const FW_CDEV_IOC_GET_CYCLE_TIMER: u32 = 2148541196;
```

#### Constant `FW_CDEV_IOC_SET_ISO_CHANNELS`

```rust
pub const FW_CDEV_IOC_SET_ISO_CHANNELS: u32 = 1074799383;
```

#### Constant `RTC_WIE_OFF`

```rust
pub const RTC_WIE_OFF: u32 = 28688;
```

#### Constant `PPGETMODE`

```rust
pub const PPGETMODE: u32 = 2147774616;
```

#### Constant `VIDIOC_DBG_G_REGISTER`

```rust
pub const VIDIOC_DBG_G_REGISTER: u32 = 3224917584;
```

#### Constant `PTP_SYS_OFFSET`

```rust
pub const PTP_SYS_OFFSET: u32 = 1128283397;
```

#### Constant `BTRFS_IOC_SPACE_INFO`

```rust
pub const BTRFS_IOC_SPACE_INFO: u32 = 3222311956;
```

#### Constant `VIDIOC_SUBDEV_ENUM_FRAME_SIZE`

```rust
pub const VIDIOC_SUBDEV_ENUM_FRAME_SIZE: u32 = 3225441866;
```

#### Constant `ND_IOCTL_VENDOR`

```rust
pub const ND_IOCTL_VENDOR: u32 = 3221769737;
```

#### Constant `SCIF_VREADFROM`

```rust
pub const SCIF_VREADFROM: u32 = 3223876364;
```

#### Constant `BTRFS_IOC_TRANS_START`

```rust
pub const BTRFS_IOC_TRANS_START: u32 = 37894;
```

#### Constant `INOTIFY_IOC_SETNEXTWD`

```rust
pub const INOTIFY_IOC_SETNEXTWD: u32 = 1074022656;
```

#### Constant `SNAPSHOT_GET_IMAGE_SIZE`

```rust
pub const SNAPSHOT_GET_IMAGE_SIZE: u32 = 2148021006;
```

#### Constant `TUNDETACHFILTER`

```rust
pub const TUNDETACHFILTER: u32 = 1074812118;
```

#### Constant `ND_IOCTL_CLEAR_ERROR`

```rust
pub const ND_IOCTL_CLEAR_ERROR: u32 = 3223342596;
```

#### Constant `IOC_PR_CLEAR`

```rust
pub const IOC_PR_CLEAR: u32 = 1074819277;
```

#### Constant `SCIF_READFROM`

```rust
pub const SCIF_READFROM: u32 = 3223876362;
```

#### Constant `PPPIOCGDEBUG`

```rust
pub const PPPIOCGDEBUG: u32 = 2147775553;
```

#### Constant `BLKGETZONESZ`

```rust
pub const BLKGETZONESZ: u32 = 2147750532;
```

#### Constant `HIDIOCGUSAGES`

```rust
pub const HIDIOCGUSAGES: u32 = 3491514387;
```

#### Constant `SONYPI_IOCGTEMP`

```rust
pub const SONYPI_IOCGTEMP: u32 = 2147579404;
```

#### Constant `UI_SET_MSCBIT`

```rust
pub const UI_SET_MSCBIT: u32 = 1074025832;
```

#### Constant `APM_IOC_SUSPEND`

```rust
pub const APM_IOC_SUSPEND: u32 = 16642;
```

#### Constant `BTRFS_IOC_TREE_SEARCH`

```rust
pub const BTRFS_IOC_TREE_SEARCH: u32 = 3489698833;
```

#### Constant `RTC_PLL_GET`

```rust
pub const RTC_PLL_GET: u32 = 2149609489;
```

#### Constant `RIO_CM_EP_GET_LIST`

```rust
pub const RIO_CM_EP_GET_LIST: u32 = 3221512962;
```

#### Constant `USBDEVFS_DISCSIGNAL`

```rust
pub const USBDEVFS_DISCSIGNAL: u32 = 2148553998;
```

#### Constant `LIRC_GET_MIN_TIMEOUT`

```rust
pub const LIRC_GET_MIN_TIMEOUT: u32 = 2147772680;
```

#### Constant `SWITCHTEC_IOCTL_EVENT_SUMMARY_LEGACY`

```rust
pub const SWITCHTEC_IOCTL_EVENT_SUMMARY_LEGACY: u32 = 2174244674;
```

#### Constant `DM_TARGET_MSG`

```rust
pub const DM_TARGET_MSG: u32 = 3241737486;
```

#### Constant `SONYPI_IOCGBAT1REM`

```rust
pub const SONYPI_IOCGBAT1REM: u32 = 2147644931;
```

#### Constant `EVIOCSFF`

```rust
pub const EVIOCSFF: u32 = 1076905344;
```

#### Constant `TUNSETGROUP`

```rust
pub const TUNSETGROUP: u32 = 1074025678;
```

#### Constant `EVIOCGKEYCODE`

```rust
pub const EVIOCGKEYCODE: u32 = 2148025604;
```

#### Constant `KCOV_REMOTE_ENABLE`

```rust
pub const KCOV_REMOTE_ENABLE: u32 = 1075340134;
```

#### Constant `ND_IOCTL_GET_CONFIG_SIZE`

```rust
pub const ND_IOCTL_GET_CONFIG_SIZE: u32 = 3222031876;
```

#### Constant `FDEJECT`

```rust
pub const FDEJECT: u32 = 602;
```

#### Constant `TUNSETOFFLOAD`

```rust
pub const TUNSETOFFLOAD: u32 = 1074025680;
```

#### Constant `PPPIOCCONNECT`

```rust
pub const PPPIOCCONNECT: u32 = 1074033722;
```

#### Constant `ATM_ADDADDR`

```rust
pub const ATM_ADDADDR: u32 = 1074815368;
```

#### Constant `VDUSE_DEV_INJECT_CONFIG_IRQ`

```rust
pub const VDUSE_DEV_INJECT_CONFIG_IRQ: u32 = 33043;
```

#### Constant `AUTOFS_DEV_IOCTL_ASKUMOUNT`

```rust
pub const AUTOFS_DEV_IOCTL_ASKUMOUNT: u32 = 3222836093;
```

#### Constant `VHOST_VDPA_GET_STATUS`

```rust
pub const VHOST_VDPA_GET_STATUS: u32 = 2147594097;
```

#### Constant `CCISS_PASSTHRU`

```rust
pub const CCISS_PASSTHRU: u32 = 3227009547;
```

#### Constant `MGSL_IOCCLRMODCOUNT`

```rust
pub const MGSL_IOCCLRMODCOUNT: u32 = 27919;
```

#### Constant `TEE_IOC_SUPPL_SEND`

```rust
pub const TEE_IOC_SUPPL_SEND: u32 = 2148574215;
```

#### Constant `ATMARPD_CTRL`

```rust
pub const ATMARPD_CTRL: u32 = 25057;
```

#### Constant `UI_ABS_SETUP`

```rust
pub const UI_ABS_SETUP: u32 = 1075598596;
```

#### Constant `UI_DEV_DESTROY`

```rust
pub const UI_DEV_DESTROY: u32 = 21762;
```

#### Constant `BTRFS_IOC_QUOTA_CTL`

```rust
pub const BTRFS_IOC_QUOTA_CTL: u32 = 3222311976;
```

#### Constant `RTC_AIE_ON`

```rust
pub const RTC_AIE_ON: u32 = 28673;
```

#### Constant `AUTOFS_IOC_EXPIRE`

```rust
pub const AUTOFS_IOC_EXPIRE: u32 = 2165085029;
```

#### Constant `PPPIOCSDEBUG`

```rust
pub const PPPIOCSDEBUG: u32 = 1074033728;
```

#### Constant `GPIO_V2_LINE_SET_VALUES_IOCTL`

```rust
pub const GPIO_V2_LINE_SET_VALUES_IOCTL: u32 = 3222320143;
```

#### Constant `PPPIOCSMRU`

```rust
pub const PPPIOCSMRU: u32 = 1074033746;
```

#### Constant `CCISS_DEREGDISK`

```rust
pub const CCISS_DEREGDISK: u32 = 16908;
```

#### Constant `UI_DEV_CREATE`

```rust
pub const UI_DEV_CREATE: u32 = 21761;
```

#### Constant `FUSE_DEV_IOC_CLONE`

```rust
pub const FUSE_DEV_IOC_CLONE: u32 = 2147804416;
```

#### Constant `BTRFS_IOC_START_SYNC`

```rust
pub const BTRFS_IOC_START_SYNC: u32 = 2148045848;
```

#### Constant `NILFS_IOCTL_DELETE_CHECKPOINT`

```rust
pub const NILFS_IOCTL_DELETE_CHECKPOINT: u32 = 1074294401;
```

#### Constant `SNAPSHOT_AVAIL_SWAP_SIZE`

```rust
pub const SNAPSHOT_AVAIL_SWAP_SIZE: u32 = 2148021011;
```

#### Constant `DM_TABLE_CLEAR`

```rust
pub const DM_TABLE_CLEAR: u32 = 3241737482;
```

#### Constant `CCISS_GETINTINFO`

```rust
pub const CCISS_GETINTINFO: u32 = 2148024834;
```

#### Constant `PPPIOCSASYNCMAP`

```rust
pub const PPPIOCSASYNCMAP: u32 = 1074033751;
```

#### Constant `I2OEVTGET`

```rust
pub const I2OEVTGET: u32 = 2154326283;
```

#### Constant `NVME_IOCTL_RESET`

```rust
pub const NVME_IOCTL_RESET: u32 = 20036;
```

#### Constant `PPYIELD`

```rust
pub const PPYIELD: u32 = 28813;
```

#### Constant `NVME_IOCTL_IO64_CMD`

```rust
pub const NVME_IOCTL_IO64_CMD: u32 = 3226488392;
```

#### Constant `TUNSETCARRIER`

```rust
pub const TUNSETCARRIER: u32 = 1074025698;
```

#### Constant `DM_DEV_WAIT`

```rust
pub const DM_DEV_WAIT: u32 = 3241737480;
```

#### Constant `RTC_WIE_ON`

```rust
pub const RTC_WIE_ON: u32 = 28687;
```

#### Constant `MEDIA_IOC_DEVICE_INFO`

```rust
pub const MEDIA_IOC_DEVICE_INFO: u32 = 3238034432;
```

#### Constant `RIO_CM_CHAN_CREATE`

```rust
pub const RIO_CM_CHAN_CREATE: u32 = 3221381891;
```

#### Constant `MGSL_IOCSPARAMS`

```rust
pub const MGSL_IOCSPARAMS: u32 = 1076915456;
```

#### Constant `RTC_SET_TIME`

```rust
pub const RTC_SET_TIME: u32 = 1076129802;
```

#### Constant `VHOST_RESET_OWNER`

```rust
pub const VHOST_RESET_OWNER: u32 = 44802;
```

#### Constant `IOC_OPAL_PSID_REVERT_TPR`

```rust
pub const IOC_OPAL_PSID_REVERT_TPR: u32 = 1091072232;
```

#### Constant `AUTOFS_DEV_IOCTL_OPENMOUNT`

```rust
pub const AUTOFS_DEV_IOCTL_OPENMOUNT: u32 = 3222836084;
```

#### Constant `UDF_GETEABLOCK`

```rust
pub const UDF_GETEABLOCK: u32 = 2148035649;
```

#### Constant `VFIO_IOMMU_MAP_DMA`

```rust
pub const VFIO_IOMMU_MAP_DMA: u32 = 15217;
```

#### Constant `VIDIOC_SUBSCRIBE_EVENT`

```rust
pub const VIDIOC_SUBSCRIBE_EVENT: u32 = 1075861082;
```

#### Constant `HIDIOCGFLAG`

```rust
pub const HIDIOCGFLAG: u32 = 2147764238;
```

#### Constant `HIDIOCGUCODE`

```rust
pub const HIDIOCGUCODE: u32 = 3222816781;
```

#### Constant `VIDIOC_OMAP3ISP_AF_CFG`

```rust
pub const VIDIOC_OMAP3ISP_AF_CFG: u32 = 3226228421;
```

#### Constant `DM_REMOVE_ALL`

```rust
pub const DM_REMOVE_ALL: u32 = 3241737473;
```

#### Constant `ASPEED_LPC_CTRL_IOCTL_MAP`

```rust
pub const ASPEED_LPC_CTRL_IOCTL_MAP: u32 = 1074835969;
```

#### Constant `CCISS_GETFIRMVER`

```rust
pub const CCISS_GETFIRMVER: u32 = 2147762696;
```

#### Constant `ND_IOCTL_ARS_START`

```rust
pub const ND_IOCTL_ARS_START: u32 = 3223342594;
```

#### Constant `PPPIOCSMRRU`

```rust
pub const PPPIOCSMRRU: u32 = 1074033723;
```

#### Constant `CEC_ADAP_S_LOG_ADDRS`

```rust
pub const CEC_ADAP_S_LOG_ADDRS: u32 = 3227279620;
```

#### Constant `RPROC_GET_SHUTDOWN_ON_RELEASE`

```rust
pub const RPROC_GET_SHUTDOWN_ON_RELEASE: u32 = 2147792642;
```

#### Constant `DMA_HEAP_IOCTL_ALLOC`

```rust
pub const DMA_HEAP_IOCTL_ALLOC: u32 = 3222816768;
```

#### Constant `PPSETTIME`

```rust
pub const PPSETTIME: u32 = 1074819222;
```

#### Constant `RTC_ALM_READ`

```rust
pub const RTC_ALM_READ: u32 = 2149871624;
```

#### Constant `VDUSE_SET_API_VERSION`

```rust
pub const VDUSE_SET_API_VERSION: u32 = 1074299137;
```

#### Constant `RIO_MPORT_MAINT_WRITE_REMOTE`

```rust
pub const RIO_MPORT_MAINT_WRITE_REMOTE: u32 = 1075342600;
```

#### Constant `VIDIOC_SUBDEV_S_CROP`

```rust
pub const VIDIOC_SUBDEV_S_CROP: u32 = 3224917564;
```

#### Constant `USBDEVFS_CONNECT`

```rust
pub const USBDEVFS_CONNECT: u32 = 21783;
```

#### Constant `SYNC_IOC_FILE_INFO`

```rust
pub const SYNC_IOC_FILE_INFO: u32 = 3224911364;
```

#### Constant `ATMARP_MKIP`

```rust
pub const ATMARP_MKIP: u32 = 25058;
```

#### Constant `VFIO_IOMMU_SPAPR_TCE_GET_INFO`

```rust
pub const VFIO_IOMMU_SPAPR_TCE_GET_INFO: u32 = 15216;
```

#### Constant `CCISS_GETHEARTBEAT`

```rust
pub const CCISS_GETHEARTBEAT: u32 = 2147762694;
```

#### Constant `ATM_RSTADDR`

```rust
pub const ATM_RSTADDR: u32 = 1074815367;
```

#### Constant `NBD_SET_SIZE`

```rust
pub const NBD_SET_SIZE: u32 = 43778;
```

#### Constant `UDF_GETVOLIDENT`

```rust
pub const UDF_GETVOLIDENT: u32 = 2148035650;
```

#### Constant `GPIO_V2_LINE_GET_VALUES_IOCTL`

```rust
pub const GPIO_V2_LINE_GET_VALUES_IOCTL: u32 = 3222320142;
```

#### Constant `MGSL_IOCSTXIDLE`

```rust
pub const MGSL_IOCSTXIDLE: u32 = 27906;
```

#### Constant `FSL_HV_IOCTL_SETPROP`

```rust
pub const FSL_HV_IOCTL_SETPROP: u32 = 3223891720;
```

#### Constant `BTRFS_IOC_GET_DEV_STATS`

```rust
pub const BTRFS_IOC_GET_DEV_STATS: u32 = 3288896564;
```

#### Constant `PPRSTATUS`

```rust
pub const PPRSTATUS: u32 = 2147577985;
```

#### Constant `MGSL_IOCTXENABLE`

```rust
pub const MGSL_IOCTXENABLE: u32 = 27908;
```

#### Constant `UDF_GETEASIZE`

```rust
pub const UDF_GETEASIZE: u32 = 2147773504;
```

#### Constant `NVME_IOCTL_ADMIN64_CMD`

```rust
pub const NVME_IOCTL_ADMIN64_CMD: u32 = 3226488391;
```

#### Constant `VHOST_SET_OWNER`

```rust
pub const VHOST_SET_OWNER: u32 = 44801;
```

#### Constant `RIO_ALLOC_DMA`

```rust
pub const RIO_ALLOC_DMA: u32 = 3222826259;
```

#### Constant `RIO_CM_CHAN_ACCEPT`

```rust
pub const RIO_CM_CHAN_ACCEPT: u32 = 3221775111;
```

#### Constant `I2OHRTGET`

```rust
pub const I2OHRTGET: u32 = 3222825217;
```

#### Constant `ATM_SETCIRANGE`

```rust
pub const ATM_SETCIRANGE: u32 = 1074815371;
```

#### Constant `HPET_IE_ON`

```rust
pub const HPET_IE_ON: u32 = 26625;
```

#### Constant `PERF_EVENT_IOC_ID`

```rust
pub const PERF_EVENT_IOC_ID: u32 = 2148017159;
```

#### Constant `TUNSETSNDBUF`

```rust
pub const TUNSETSNDBUF: u32 = 1074025684;
```

#### Constant `PTP_PIN_SETFUNC`

```rust
pub const PTP_PIN_SETFUNC: u32 = 1080048903;
```

#### Constant `PPPIOCDISCONN`

```rust
pub const PPPIOCDISCONN: u32 = 29753;
```

#### Constant `VIDIOC_QUERYCTRL`

```rust
pub const VIDIOC_QUERYCTRL: u32 = 3225703972;
```

#### Constant `PPEXCL`

```rust
pub const PPEXCL: u32 = 28815;
```

#### Constant `PCITEST_MSI`

```rust
pub const PCITEST_MSI: u32 = 1074024451;
```

#### Constant `FDWERRORCLR`

```rust
pub const FDWERRORCLR: u32 = 598;
```

#### Constant `AUTOFS_IOC_FAIL`

```rust
pub const AUTOFS_IOC_FAIL: u32 = 37729;
```

#### Constant `USBDEVFS_IOCTL`

```rust
pub const USBDEVFS_IOCTL: u32 = 3222295826;
```

#### Constant `VIDIOC_S_STD`

```rust
pub const VIDIOC_S_STD: u32 = 1074288152;
```

#### Constant `F2FS_IOC_RESIZE_FS`

```rust
pub const F2FS_IOC_RESIZE_FS: u32 = 1074328848;
```

#### Constant `SONET_SETDIAG`

```rust
pub const SONET_SETDIAG: u32 = 3221512466;
```

#### Constant `BTRFS_IOC_DEFRAG`

```rust
pub const BTRFS_IOC_DEFRAG: u32 = 1342215170;
```

#### Constant `CCISS_GETDRIVVER`

```rust
pub const CCISS_GETDRIVVER: u32 = 2147762697;
```

#### Constant `IPMICTL_GET_TIMING_PARMS_CMD`

```rust
pub const IPMICTL_GET_TIMING_PARMS_CMD: u32 = 2148034839;
```

#### Constant `HPET_IRQFREQ`

```rust
pub const HPET_IRQFREQ: u32 = 1074292742;
```

#### Constant `ATM_GETESI`

```rust
pub const ATM_GETESI: u32 = 1074815365;
```

#### Constant `CCISS_GETLUNINFO`

```rust
pub const CCISS_GETLUNINFO: u32 = 2148286993;
```

#### Constant `AUTOFS_DEV_IOCTL_ISMOUNTPOINT`

```rust
pub const AUTOFS_DEV_IOCTL_ISMOUNTPOINT: u32 = 3222836094;
```

#### Constant `TEE_IOC_SHM_ALLOC`

```rust
pub const TEE_IOC_SHM_ALLOC: u32 = 3222316033;
```

#### Constant `PERF_EVENT_IOC_SET_BPF`

```rust
pub const PERF_EVENT_IOC_SET_BPF: u32 = 1074013192;
```

#### Constant `UDMABUF_CREATE_LIST`

```rust
pub const UDMABUF_CREATE_LIST: u32 = 1074296131;
```

#### Constant `VHOST_SET_LOG_BASE`

```rust
pub const VHOST_SET_LOG_BASE: u32 = 1074310916;
```

#### Constant `ZATM_GETPOOL`

```rust
pub const ZATM_GETPOOL: u32 = 1074815329;
```

#### Constant `BR2684_SETFILT`

```rust
pub const BR2684_SETFILT: u32 = 1075601808;
```

#### Constant `RNDGETPOOL`

```rust
pub const RNDGETPOOL: u32 = 2148028930;
```

#### Constant `PPS_GETPARAMS`

```rust
pub const PPS_GETPARAMS: u32 = 2148036769;
```

#### Constant `IOC_PR_RESERVE`

```rust
pub const IOC_PR_RESERVE: u32 = 1074819273;
```

#### Constant `VIDIOC_TRY_DECODER_CMD`

```rust
pub const VIDIOC_TRY_DECODER_CMD: u32 = 3225966177;
```

#### Constant `RIO_CM_CHAN_CLOSE`

```rust
pub const RIO_CM_CHAN_CLOSE: u32 = 1073898244;
```

#### Constant `VIDIOC_DV_TIMINGS_CAP`

```rust
pub const VIDIOC_DV_TIMINGS_CAP: u32 = 3230684772;
```

#### Constant `IOCTL_MEI_CONNECT_CLIENT_VTAG`

```rust
pub const IOCTL_MEI_CONNECT_CLIENT_VTAG: u32 = 3222554628;
```

#### Constant `PMU_IOC_GET_BACKLIGHT`

```rust
pub const PMU_IOC_GET_BACKLIGHT: u32 = 2148024833;
```

#### Constant `USBDEVFS_GET_CAPABILITIES`

```rust
pub const USBDEVFS_GET_CAPABILITIES: u32 = 2147767578;
```

#### Constant `SCIF_WRITETO`

```rust
pub const SCIF_WRITETO: u32 = 3223876363;
```

#### Constant `UDF_RELOCATE_BLOCKS`

```rust
pub const UDF_RELOCATE_BLOCKS: u32 = 3221777475;
```

#### Constant `FSL_HV_IOCTL_PARTITION_RESTART`

```rust
pub const FSL_HV_IOCTL_PARTITION_RESTART: u32 = 3221794561;
```

#### Constant `CCISS_REGNEWD`

```rust
pub const CCISS_REGNEWD: u32 = 16910;
```

#### Constant `FAT_IOCTL_SET_ATTRIBUTES`

```rust
pub const FAT_IOCTL_SET_ATTRIBUTES: u32 = 1074033169;
```

#### Constant `VIDIOC_CREATE_BUFS`

```rust
pub const VIDIOC_CREATE_BUFS: u32 = 3238024796;
```

#### Constant `CAPI_GET_VERSION`

```rust
pub const CAPI_GET_VERSION: u32 = 3222291207;
```

#### Constant `SWITCHTEC_IOCTL_EVENT_SUMMARY`

```rust
pub const SWITCHTEC_IOCTL_EVENT_SUMMARY: u32 = 2228770626;
```

#### Constant `VFIO_EEH_PE_OP`

```rust
pub const VFIO_EEH_PE_OP: u32 = 15225;
```

#### Constant `FW_CDEV_IOC_CREATE_ISO_CONTEXT`

```rust
pub const FW_CDEV_IOC_CREATE_ISO_CONTEXT: u32 = 3223331592;
```

#### Constant `F2FS_IOC_RELEASE_COMPRESS_BLOCKS`

```rust
pub const F2FS_IOC_RELEASE_COMPRESS_BLOCKS: u32 = 2148070674;
```

#### Constant `NBD_SET_SIZE_BLOCKS`

```rust
pub const NBD_SET_SIZE_BLOCKS: u32 = 43783;
```

#### Constant `IPMI_BMC_IOCTL_SET_SMS_ATN`

```rust
pub const IPMI_BMC_IOCTL_SET_SMS_ATN: u32 = 45312;
```

#### Constant `ASPEED_P2A_CTRL_IOCTL_GET_MEMORY_CONFIG`

```rust
pub const ASPEED_P2A_CTRL_IOCTL_GET_MEMORY_CONFIG: u32 = 3222319873;
```

#### Constant `VIDIOC_S_AUDOUT`

```rust
pub const VIDIOC_S_AUDOUT: u32 = 1077171762;
```

#### Constant `VIDIOC_S_FMT`

```rust
pub const VIDIOC_S_FMT: u32 = 3234878981;
```

#### Constant `PPPIOCATTACH`

```rust
pub const PPPIOCATTACH: u32 = 1074033725;
```

#### Constant `VHOST_GET_VRING_BUSYLOOP_TIMEOUT`

```rust
pub const VHOST_GET_VRING_BUSYLOOP_TIMEOUT: u32 = 1074310948;
```

#### Constant `FS_IOC_MEASURE_VERITY`

```rust
pub const FS_IOC_MEASURE_VERITY: u32 = 3221513862;
```

#### Constant `CCISS_BIG_PASSTHRU`

```rust
pub const CCISS_BIG_PASSTHRU: u32 = 3227533842;
```

#### Constant `IPMICTL_SET_MY_LUN_CMD`

```rust
pub const IPMICTL_SET_MY_LUN_CMD: u32 = 2147772691;
```

#### Constant `PCITEST_LEGACY_IRQ`

```rust
pub const PCITEST_LEGACY_IRQ: u32 = 20482;
```

#### Constant `USBDEVFS_SUBMITURB`

```rust
pub const USBDEVFS_SUBMITURB: u32 = 2151175434;
```

#### Constant `AUTOFS_IOC_READY`

```rust
pub const AUTOFS_IOC_READY: u32 = 37728;
```

#### Constant `BTRFS_IOC_SEND`

```rust
pub const BTRFS_IOC_SEND: u32 = 1078498342;
```

#### Constant `VIDIOC_G_EXT_CTRLS`

```rust
pub const VIDIOC_G_EXT_CTRLS: u32 = 3223344711;
```

#### Constant `JSIOCSBTNMAP`

```rust
pub const JSIOCSBTNMAP: u32 = 1140877875;
```

#### Constant `PPPIOCSFLAGS`

```rust
pub const PPPIOCSFLAGS: u32 = 1074033753;
```

#### Constant `NVRAM_INIT`

```rust
pub const NVRAM_INIT: u32 = 28736;
```

#### Constant `RFKILL_IOCTL_NOINPUT`

```rust
pub const RFKILL_IOCTL_NOINPUT: u32 = 20993;
```

#### Constant `BTRFS_IOC_BALANCE`

```rust
pub const BTRFS_IOC_BALANCE: u32 = 1342215180;
```

#### Constant `FS_IOC_GETFSMAP`

```rust
pub const FS_IOC_GETFSMAP: u32 = 3233830971;
```

#### Constant `IPMICTL_GET_MY_CHANNEL_LUN_CMD`

```rust
pub const IPMICTL_GET_MY_CHANNEL_LUN_CMD: u32 = 2147772699;
```

#### Constant `STP_POLICY_ID_GET`

```rust
pub const STP_POLICY_ID_GET: u32 = 2148541697;
```

#### Constant `PPSETFLAGS`

```rust
pub const PPSETFLAGS: u32 = 1074032795;
```

#### Constant `CEC_ADAP_S_PHYS_ADDR`

```rust
pub const CEC_ADAP_S_PHYS_ADDR: u32 = 1073897730;
```

#### Constant `ATMTCP_CREATE`

```rust
pub const ATMTCP_CREATE: u32 = 24974;
```

#### Constant `IPMI_BMC_IOCTL_FORCE_ABORT`

```rust
pub const IPMI_BMC_IOCTL_FORCE_ABORT: u32 = 45314;
```

#### Constant `PPPIOCGXASYNCMAP`

```rust
pub const PPPIOCGXASYNCMAP: u32 = 2149610576;
```

#### Constant `VHOST_SET_VRING_CALL`

```rust
pub const VHOST_SET_VRING_CALL: u32 = 1074310945;
```

#### Constant `LIRC_GET_FEATURES`

```rust
pub const LIRC_GET_FEATURES: u32 = 2147772672;
```

#### Constant `GSMIOC_DISABLE_NET`

```rust
pub const GSMIOC_DISABLE_NET: u32 = 18179;
```

#### Constant `AUTOFS_IOC_CATATONIC`

```rust
pub const AUTOFS_IOC_CATATONIC: u32 = 37730;
```

#### Constant `NBD_DO_IT`

```rust
pub const NBD_DO_IT: u32 = 43779;
```

#### Constant `LIRC_SET_REC_CARRIER_RANGE`

```rust
pub const LIRC_SET_REC_CARRIER_RANGE: u32 = 1074030879;
```

#### Constant `IPMICTL_GET_MY_CHANNEL_ADDRESS_CMD`

```rust
pub const IPMICTL_GET_MY_CHANNEL_ADDRESS_CMD: u32 = 2147772697;
```

#### Constant `EVIOCSCLOCKID`

```rust
pub const EVIOCSCLOCKID: u32 = 1074021792;
```

#### Constant `USBDEVFS_FREE_STREAMS`

```rust
pub const USBDEVFS_FREE_STREAMS: u32 = 2148029725;
```

#### Constant `FSI_SCOM_RESET`

```rust
pub const FSI_SCOM_RESET: u32 = 1074033411;
```

#### Constant `PMU_IOC_GRAB_BACKLIGHT`

```rust
pub const PMU_IOC_GRAB_BACKLIGHT: u32 = 2148024838;
```

#### Constant `VIDIOC_SUBDEV_S_FMT`

```rust
pub const VIDIOC_SUBDEV_S_FMT: u32 = 3227014661;
```

#### Constant `FDDEFPRM`

```rust
pub const FDDEFPRM: u32 = 1075839555;
```

#### Constant `TEE_IOC_INVOKE`

```rust
pub const TEE_IOC_INVOKE: u32 = 2148574211;
```

#### Constant `USBDEVFS_BULK`

```rust
pub const USBDEVFS_BULK: u32 = 3222820098;
```

#### Constant `SCIF_VWRITETO`

```rust
pub const SCIF_VWRITETO: u32 = 3223876365;
```

#### Constant `SONYPI_IOCSBRT`

```rust
pub const SONYPI_IOCSBRT: u32 = 1073837568;
```

#### Constant `BTRFS_IOC_FILE_EXTENT_SAME`

```rust
pub const BTRFS_IOC_FILE_EXTENT_SAME: u32 = 3222836278;
```

#### Constant `RTC_PIE_ON`

```rust
pub const RTC_PIE_ON: u32 = 28677;
```

#### Constant `BTRFS_IOC_SCAN_DEV`

```rust
pub const BTRFS_IOC_SCAN_DEV: u32 = 1342215172;
```

#### Constant `PPPIOCXFERUNIT`

```rust
pub const PPPIOCXFERUNIT: u32 = 29774;
```

#### Constant `WDIOC_GETTIMEOUT`

```rust
pub const WDIOC_GETTIMEOUT: u32 = 2147768071;
```

#### Constant `BTRFS_IOC_SET_RECEIVED_SUBVOL`

```rust
pub const BTRFS_IOC_SET_RECEIVED_SUBVOL: u32 = 3234370597;
```

#### Constant `DFL_FPGA_PORT_ERR_SET_IRQ`

```rust
pub const DFL_FPGA_PORT_ERR_SET_IRQ: u32 = 1074312774;
```

#### Constant `FBIO_WAITFORVSYNC`

```rust
pub const FBIO_WAITFORVSYNC: u32 = 1074021920;
```

#### Constant `RTC_PIE_OFF`

```rust
pub const RTC_PIE_OFF: u32 = 28678;
```

#### Constant `EVIOCGRAB`

```rust
pub const EVIOCGRAB: u32 = 1074021776;
```

#### Constant `PMU_IOC_SET_BACKLIGHT`

```rust
pub const PMU_IOC_SET_BACKLIGHT: u32 = 1074283010;
```

#### Constant `EVIOCGREP`

```rust
pub const EVIOCGREP: u32 = 2148025603;
```

#### Constant `PERF_EVENT_IOC_MODIFY_ATTRIBUTES`

```rust
pub const PERF_EVENT_IOC_MODIFY_ATTRIBUTES: u32 = 1074275339;
```

#### Constant `UFFDIO_CONTINUE`

```rust
pub const UFFDIO_CONTINUE: u32 = 3223366151;
```

#### Constant `VDUSE_GET_API_VERSION`

```rust
pub const VDUSE_GET_API_VERSION: u32 = 2148040960;
```

#### Constant `RTC_RD_TIME`

```rust
pub const RTC_RD_TIME: u32 = 2149871625;
```

#### Constant `FDMSGOFF`

```rust
pub const FDMSGOFF: u32 = 582;
```

#### Constant `IPMICTL_REGISTER_FOR_CMD_CHANS`

```rust
pub const IPMICTL_REGISTER_FOR_CMD_CHANS: u32 = 2148296988;
```

#### Constant `CAPI_GET_ERRCODE`

```rust
pub const CAPI_GET_ERRCODE: u32 = 2147631905;
```

#### Constant `PCITEST_SET_IRQTYPE`

```rust
pub const PCITEST_SET_IRQTYPE: u32 = 1074024456;
```

#### Constant `VIDIOC_SUBDEV_S_EDID`

```rust
pub const VIDIOC_SUBDEV_S_EDID: u32 = 3223868969;
```

#### Constant `MATROXFB_SET_OUTPUT_MODE`

```rust
pub const MATROXFB_SET_OUTPUT_MODE: u32 = 1074294522;
```

#### Constant `RIO_DEV_ADD`

```rust
pub const RIO_DEV_ADD: u32 = 1075866903;
```

#### Constant `VIDIOC_ENUM_FREQ_BANDS`

```rust
pub const VIDIOC_ENUM_FREQ_BANDS: u32 = 3225441893;
```

#### Constant `FBIO_RADEON_SET_MIRROR`

```rust
pub const FBIO_RADEON_SET_MIRROR: u32 = 1074282500;
```

#### Constant `PCITEST_GET_IRQTYPE`

```rust
pub const PCITEST_GET_IRQTYPE: u32 = 20489;
```

#### Constant `JSIOCGVERSION`

```rust
pub const JSIOCGVERSION: u32 = 2147772929;
```

#### Constant `SONYPI_IOCSBLUE`

```rust
pub const SONYPI_IOCSBLUE: u32 = 1073837577;
```

#### Constant `SNAPSHOT_PREF_IMAGE_SIZE`

```rust
pub const SNAPSHOT_PREF_IMAGE_SIZE: u32 = 13074;
```

#### Constant `F2FS_IOC_GET_FEATURES`

```rust
pub const F2FS_IOC_GET_FEATURES: u32 = 2147808524;
```

#### Constant `SCIF_REG`

```rust
pub const SCIF_REG: u32 = 3223876360;
```

#### Constant `NILFS_IOCTL_CLEAN_SEGMENTS`

```rust
pub const NILFS_IOCTL_CLEAN_SEGMENTS: u32 = 1081634440;
```

#### Constant `FW_CDEV_IOC_INITIATE_BUS_RESET`

```rust
pub const FW_CDEV_IOC_INITIATE_BUS_RESET: u32 = 1074012933;
```

#### Constant `RIO_WAIT_FOR_ASYNC`

```rust
pub const RIO_WAIT_FOR_ASYNC: u32 = 1074294038;
```

#### Constant `VHOST_SET_VRING_NUM`

```rust
pub const VHOST_SET_VRING_NUM: u32 = 1074310928;
```

#### Constant `AUTOFS_DEV_IOCTL_PROTOVER`

```rust
pub const AUTOFS_DEV_IOCTL_PROTOVER: u32 = 3222836082;
```

#### Constant `RIO_FREE_DMA`

```rust
pub const RIO_FREE_DMA: u32 = 1074294036;
```

#### Constant `MGSL_IOCRXENABLE`

```rust
pub const MGSL_IOCRXENABLE: u32 = 27909;
```

#### Constant `IOCTL_VM_SOCKETS_GET_LOCAL_CID`

```rust
pub const IOCTL_VM_SOCKETS_GET_LOCAL_CID: u32 = 1977;
```

#### Constant `IPMICTL_SET_TIMING_PARMS_CMD`

```rust
pub const IPMICTL_SET_TIMING_PARMS_CMD: u32 = 2148034838;
```

#### Constant `PPPIOCGL2TPSTATS`

```rust
pub const PPPIOCGL2TPSTATS: u32 = 2152231990;
```

#### Constant `PERF_EVENT_IOC_PERIOD`

```rust
pub const PERF_EVENT_IOC_PERIOD: u32 = 1074275332;
```

#### Constant `PTP_PIN_SETFUNC2`

```rust
pub const PTP_PIN_SETFUNC2: u32 = 1080048912;
```

#### Constant `CHIOEXCHANGE`

```rust
pub const CHIOEXCHANGE: u32 = 1075602178;
```

#### Constant `NILFS_IOCTL_GET_SUINFO`

```rust
pub const NILFS_IOCTL_GET_SUINFO: u32 = 2149084804;
```

#### Constant `CEC_DQEVENT`

```rust
pub const CEC_DQEVENT: u32 = 3226493191;
```

#### Constant `UI_SET_SWBIT`

```rust
pub const UI_SET_SWBIT: u32 = 1074025837;
```

#### Constant `VHOST_VDPA_SET_CONFIG`

```rust
pub const VHOST_VDPA_SET_CONFIG: u32 = 1074311028;
```

#### Constant `TUNSETIFF`

```rust
pub const TUNSETIFF: u32 = 1074025674;
```

#### Constant `CHIOPOSITION`

```rust
pub const CHIOPOSITION: u32 = 1074553603;
```

#### Constant `IPMICTL_SET_MAINTENANCE_MODE_CMD`

```rust
pub const IPMICTL_SET_MAINTENANCE_MODE_CMD: u32 = 1074030879;
```

#### Constant `BTRFS_IOC_DEFAULT_SUBVOL`

```rust
pub const BTRFS_IOC_DEFAULT_SUBVOL: u32 = 1074304019;
```

#### Constant `RIO_UNMAP_OUTBOUND`

```rust
pub const RIO_UNMAP_OUTBOUND: u32 = 1076391184;
```

#### Constant `CAPI_CLR_FLAGS`

```rust
pub const CAPI_CLR_FLAGS: u32 = 2147762981;
```

#### Constant `FW_CDEV_IOC_ALLOCATE_ISO_RESOURCE_ONCE`

```rust
pub const FW_CDEV_IOC_ALLOCATE_ISO_RESOURCE_ONCE: u32 = 1075323663;
```

#### Constant `MATROXFB_GET_OUTPUT_CONNECTION`

```rust
pub const MATROXFB_GET_OUTPUT_CONNECTION: u32 = 2148036344;
```

#### Constant `EVIOCSMASK`

```rust
pub const EVIOCSMASK: u32 = 1074808211;
```

#### Constant `BTRFS_IOC_FORGET_DEV`

```rust
pub const BTRFS_IOC_FORGET_DEV: u32 = 1342215173;
```

#### Constant `CXL_MEM_QUERY_COMMANDS`

```rust
pub const CXL_MEM_QUERY_COMMANDS: u32 = 2148060673;
```

#### Constant `CEC_S_MODE`

```rust
pub const CEC_S_MODE: u32 = 1074028809;
```

#### Constant `MGSL_IOCSIF`

```rust
pub const MGSL_IOCSIF: u32 = 27914;
```

#### Constant `SWITCHTEC_IOCTL_PFF_TO_PORT`

```rust
pub const SWITCHTEC_IOCTL_PFF_TO_PORT: u32 = 3222034244;
```

#### Constant `PPSETMODE`

```rust
pub const PPSETMODE: u32 = 1074032768;
```

#### Constant `VFIO_DEVICE_SET_IRQS`

```rust
pub const VFIO_DEVICE_SET_IRQS: u32 = 15214;
```

#### Constant `VIDIOC_PREPARE_BUF`

```rust
pub const VIDIOC_PREPARE_BUF: u32 = 3227014749;
```

#### Constant `CEC_ADAP_G_CONNECTOR_INFO`

```rust
pub const CEC_ADAP_G_CONNECTOR_INFO: u32 = 2151964938;
```

#### Constant `IOC_OPAL_WRITE_SHADOW_MBR`

```rust
pub const IOC_OPAL_WRITE_SHADOW_MBR: u32 = 1092645098;
```

#### Constant `VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL`

```rust
pub const VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL: u32 = 3225441867;
```

#### Constant `UDMABUF_CREATE`

```rust
pub const UDMABUF_CREATE: u32 = 1075344706;
```

#### Constant `SONET_CLRDIAG`

```rust
pub const SONET_CLRDIAG: u32 = 3221512467;
```

#### Constant `PHN_SET_REG`

```rust
pub const PHN_SET_REG: u32 = 1074294785;
```

#### Constant `RNDADDTOENTCNT`

```rust
pub const RNDADDTOENTCNT: u32 = 1074024961;
```

#### Constant `VBG_IOCTL_CHECK_BALLOON`

```rust
pub const VBG_IOCTL_CHECK_BALLOON: u32 = 3223344657;
```

#### Constant `VIDIOC_OMAP3ISP_STAT_REQ`

```rust
pub const VIDIOC_OMAP3ISP_STAT_REQ: u32 = 3223869126;
```

#### Constant `PPS_FETCH`

```rust
pub const PPS_FETCH: u32 = 3221778596;
```

#### Constant `RTC_AIE_OFF`

```rust
pub const RTC_AIE_OFF: u32 = 28674;
```

#### Constant `VFIO_GROUP_SET_CONTAINER`

```rust
pub const VFIO_GROUP_SET_CONTAINER: u32 = 15208;
```

#### Constant `FW_CDEV_IOC_RECEIVE_PHY_PACKETS`

```rust
pub const FW_CDEV_IOC_RECEIVE_PHY_PACKETS: u32 = 1074275094;
```

#### Constant `VFIO_IOMMU_SPAPR_TCE_REMOVE`

```rust
pub const VFIO_IOMMU_SPAPR_TCE_REMOVE: u32 = 15224;
```

#### Constant `VFIO_IOMMU_GET_INFO`

```rust
pub const VFIO_IOMMU_GET_INFO: u32 = 15216;
```

#### Constant `DM_DEV_SUSPEND`

```rust
pub const DM_DEV_SUSPEND: u32 = 3241737478;
```

#### Constant `F2FS_IOC_GET_COMPRESS_OPTION`

```rust
pub const F2FS_IOC_GET_COMPRESS_OPTION: u32 = 2147677461;
```

#### Constant `FW_CDEV_IOC_STOP_ISO`

```rust
pub const FW_CDEV_IOC_STOP_ISO: u32 = 1074012939;
```

#### Constant `GPIO_V2_GET_LINEINFO_IOCTL`

```rust
pub const GPIO_V2_GET_LINEINFO_IOCTL: u32 = 3238048773;
```

#### Constant `ATMMPC_CTRL`

```rust
pub const ATMMPC_CTRL: u32 = 25048;
```

#### Constant `PPPIOCSXASYNCMAP`

```rust
pub const PPPIOCSXASYNCMAP: u32 = 1075868751;
```

#### Constant `CHIOGSTATUS`

```rust
pub const CHIOGSTATUS: u32 = 1074815752;
```

#### Constant `FW_CDEV_IOC_ALLOCATE_ISO_RESOURCE`

```rust
pub const FW_CDEV_IOC_ALLOCATE_ISO_RESOURCE: u32 = 3222807309;
```

#### Constant `RIO_MPORT_MAINT_PORT_IDX_GET`

```rust
pub const RIO_MPORT_MAINT_PORT_IDX_GET: u32 = 2147773699;
```

#### Constant `CAPI_SET_FLAGS`

```rust
pub const CAPI_SET_FLAGS: u32 = 2147762980;
```

#### Constant `VFIO_GROUP_GET_DEVICE_FD`

```rust
pub const VFIO_GROUP_GET_DEVICE_FD: u32 = 15210;
```

#### Constant `VHOST_SET_MEM_TABLE`

```rust
pub const VHOST_SET_MEM_TABLE: u32 = 1074310915;
```

#### Constant `MATROXFB_SET_OUTPUT_CONNECTION`

```rust
pub const MATROXFB_SET_OUTPUT_CONNECTION: u32 = 1074294520;
```

#### Constant `DFL_FPGA_PORT_GET_REGION_INFO`

```rust
pub const DFL_FPGA_PORT_GET_REGION_INFO: u32 = 46658;
```

#### Constant `VHOST_GET_FEATURES`

```rust
pub const VHOST_GET_FEATURES: u32 = 2148052736;
```

#### Constant `LIRC_GET_REC_RESOLUTION`

```rust
pub const LIRC_GET_REC_RESOLUTION: u32 = 2147772679;
```

#### Constant `PACKET_CTRL_CMD`

```rust
pub const PACKET_CTRL_CMD: u32 = 3222820865;
```

#### Constant `LIRC_SET_TRANSMITTER_MASK`

```rust
pub const LIRC_SET_TRANSMITTER_MASK: u32 = 1074030871;
```

#### Constant `BTRFS_IOC_ADD_DEV`

```rust
pub const BTRFS_IOC_ADD_DEV: u32 = 1342215178;
```

#### Constant `JSIOCGCORR`

```rust
pub const JSIOCGCORR: u32 = 2149870114;
```

#### Constant `VIDIOC_G_FMT`

```rust
pub const VIDIOC_G_FMT: u32 = 3234878980;
```

#### Constant `RTC_EPOCH_SET`

```rust
pub const RTC_EPOCH_SET: u32 = 1074294798;
```

#### Constant `CAPI_GET_PROFILE`

```rust
pub const CAPI_GET_PROFILE: u32 = 3225436937;
```

#### Constant `ATM_GETLOOP`

```rust
pub const ATM_GETLOOP: u32 = 1074815314;
```

#### Constant `SCIF_LISTEN`

```rust
pub const SCIF_LISTEN: u32 = 1074033410;
```

#### Constant `NBD_CLEAR_QUE`

```rust
pub const NBD_CLEAR_QUE: u32 = 43781;
```

#### Constant `F2FS_IOC_MOVE_RANGE`

```rust
pub const F2FS_IOC_MOVE_RANGE: u32 = 3223385353;
```

#### Constant `LIRC_GET_LENGTH`

```rust
pub const LIRC_GET_LENGTH: u32 = 2147772687;
```

#### Constant `I8K_SET_FAN`

```rust
pub const I8K_SET_FAN: u32 = 3221776775;
```

#### Constant `FDSETMAXERRS`

```rust
pub const FDSETMAXERRS: u32 = 1075053132;
```

#### Constant `VIDIOC_SUBDEV_QUERYCAP`

```rust
pub const VIDIOC_SUBDEV_QUERYCAP: u32 = 2151699968;
```

#### Constant `SNAPSHOT_SET_SWAP_AREA`

```rust
pub const SNAPSHOT_SET_SWAP_AREA: u32 = 1074541325;
```

#### Constant `LIRC_GET_REC_TIMEOUT`

```rust
pub const LIRC_GET_REC_TIMEOUT: u32 = 2147772708;
```

#### Constant `EVIOCRMFF`

```rust
pub const EVIOCRMFF: u32 = 1074021761;
```

#### Constant `GPIO_GET_LINEEVENT_IOCTL`

```rust
pub const GPIO_GET_LINEEVENT_IOCTL: u32 = 3224417284;
```

#### Constant `PPRDATA`

```rust
pub const PPRDATA: u32 = 2147577989;
```

#### Constant `RIO_MPORT_GET_PROPERTIES`

```rust
pub const RIO_MPORT_GET_PROPERTIES: u32 = 2150657284;
```

#### Constant `TUNSETVNETHDRSZ`

```rust
pub const TUNSETVNETHDRSZ: u32 = 1074025688;
```

#### Constant `GPIO_GET_LINEINFO_IOCTL`

```rust
pub const GPIO_GET_LINEINFO_IOCTL: u32 = 3225990146;
```

#### Constant `GSMIOC_GETCONF`

```rust
pub const GSMIOC_GETCONF: u32 = 2152482560;
```

#### Constant `LIRC_GET_SEND_MODE`

```rust
pub const LIRC_GET_SEND_MODE: u32 = 2147772673;
```

#### Constant `PPPIOCSACTIVE`

```rust
pub const PPPIOCSACTIVE: u32 = 1074820166;
```

#### Constant `SIOCGSTAMPNS_NEW`

```rust
pub const SIOCGSTAMPNS_NEW: u32 = 2148567303;
```

#### Constant `IPMICTL_RECEIVE_MSG`

```rust
pub const IPMICTL_RECEIVE_MSG: u32 = 3224398092;
```

#### Constant `LIRC_SET_SEND_DUTY_CYCLE`

```rust
pub const LIRC_SET_SEND_DUTY_CYCLE: u32 = 1074030869;
```

#### Constant `UI_END_FF_ERASE`

```rust
pub const UI_END_FF_ERASE: u32 = 1074550219;
```

#### Constant `SWITCHTEC_IOCTL_FLASH_PART_INFO`

```rust
pub const SWITCHTEC_IOCTL_FLASH_PART_INFO: u32 = 3222296385;
```

#### Constant `FW_CDEV_IOC_SEND_PHY_PACKET`

```rust
pub const FW_CDEV_IOC_SEND_PHY_PACKET: u32 = 3222807317;
```

#### Constant `NBD_SET_FLAGS`

```rust
pub const NBD_SET_FLAGS: u32 = 43786;
```

#### Constant `VFIO_DEVICE_GET_REGION_INFO`

```rust
pub const VFIO_DEVICE_GET_REGION_INFO: u32 = 15212;
```

#### Constant `REISERFS_IOC_UNPACK`

```rust
pub const REISERFS_IOC_UNPACK: u32 = 1074318593;
```

#### Constant `FW_CDEV_IOC_REMOVE_DESCRIPTOR`

```rust
pub const FW_CDEV_IOC_REMOVE_DESCRIPTOR: u32 = 1074012935;
```

#### Constant `RIO_SET_EVENT_MASK`

```rust
pub const RIO_SET_EVENT_MASK: u32 = 1074031885;
```

#### Constant `SNAPSHOT_ALLOC_SWAP_PAGE`

```rust
pub const SNAPSHOT_ALLOC_SWAP_PAGE: u32 = 2148021012;
```

#### Constant `VDUSE_VQ_INJECT_IRQ`

```rust
pub const VDUSE_VQ_INJECT_IRQ: u32 = 1074037015;
```

#### Constant `I2OPASSTHRU`

```rust
pub const I2OPASSTHRU: u32 = 2148559116;
```

#### Constant `IOC_OPAL_SET_PW`

```rust
pub const IOC_OPAL_SET_PW: u32 = 1109422304;
```

#### Constant `FSI_SCOM_READ`

```rust
pub const FSI_SCOM_READ: u32 = 3223352065;
```

#### Constant `VHOST_VDPA_GET_DEVICE_ID`

```rust
pub const VHOST_VDPA_GET_DEVICE_ID: u32 = 2147790704;
```

#### Constant `VIDIOC_QBUF`

```rust
pub const VIDIOC_QBUF: u32 = 3227014671;
```

#### Constant `VIDIOC_S_TUNER`

```rust
pub const VIDIOC_S_TUNER: u32 = 1079268894;
```

#### Constant `TUNGETVNETHDRSZ`

```rust
pub const TUNGETVNETHDRSZ: u32 = 2147767511;
```

#### Constant `CAPI_NCCI_GETUNIT`

```rust
pub const CAPI_NCCI_GETUNIT: u32 = 2147762983;
```

#### Constant `DFL_FPGA_PORT_UINT_GET_IRQ_NUM`

```rust
pub const DFL_FPGA_PORT_UINT_GET_IRQ_NUM: u32 = 2147792455;
```

#### Constant `VIDIOC_OMAP3ISP_STAT_EN`

```rust
pub const VIDIOC_OMAP3ISP_STAT_EN: u32 = 3221771975;
```

#### Constant `GPIO_V2_LINE_SET_CONFIG_IOCTL`

```rust
pub const GPIO_V2_LINE_SET_CONFIG_IOCTL: u32 = 3239097357;
```

#### Constant `TEE_IOC_VERSION`

```rust
pub const TEE_IOC_VERSION: u32 = 2148312064;
```

#### Constant `VIDIOC_LOG_STATUS`

```rust
pub const VIDIOC_LOG_STATUS: u32 = 22086;
```

#### Constant `IPMICTL_SEND_COMMAND_SETTIME`

```rust
pub const IPMICTL_SEND_COMMAND_SETTIME: u32 = 2150656277;
```

#### Constant `VHOST_SET_LOG_FD`

```rust
pub const VHOST_SET_LOG_FD: u32 = 1074048775;
```

#### Constant `SCIF_SEND`

```rust
pub const SCIF_SEND: u32 = 3222827782;
```

#### Constant `VIDIOC_SUBDEV_G_FMT`

```rust
pub const VIDIOC_SUBDEV_G_FMT: u32 = 3227014660;
```

#### Constant `NS_ADJBUFLEV`

```rust
pub const NS_ADJBUFLEV: u32 = 24931;
```

#### Constant `VIDIOC_DBG_S_REGISTER`

```rust
pub const VIDIOC_DBG_S_REGISTER: u32 = 1077433935;
```

#### Constant `NILFS_IOCTL_RESIZE`

```rust
pub const NILFS_IOCTL_RESIZE: u32 = 1074294411;
```

#### Constant `PHN_GETREG`

```rust
pub const PHN_GETREG: u32 = 3221778437;
```

#### Constant `I2OSWDL`

```rust
pub const I2OSWDL: u32 = 3224398085;
```

#### Constant `VBG_IOCTL_VMMDEV_REQUEST_BIG`

```rust
pub const VBG_IOCTL_VMMDEV_REQUEST_BIG: u32 = 22019;
```

#### Constant `JSIOCGBUTTONS`

```rust
pub const JSIOCGBUTTONS: u32 = 2147576338;
```

#### Constant `VFIO_IOMMU_ENABLE`

```rust
pub const VFIO_IOMMU_ENABLE: u32 = 15219;
```

#### Constant `DM_DEV_RENAME`

```rust
pub const DM_DEV_RENAME: u32 = 3241737477;
```

#### Constant `MEDIA_IOC_SETUP_LINK`

```rust
pub const MEDIA_IOC_SETUP_LINK: u32 = 3224665091;
```

#### Constant `VIDIOC_ENUMOUTPUT`

```rust
pub const VIDIOC_ENUMOUTPUT: u32 = 3225966128;
```

#### Constant `STP_POLICY_ID_SET`

```rust
pub const STP_POLICY_ID_SET: u32 = 3222283520;
```

#### Constant `VHOST_VDPA_SET_CONFIG_CALL`

```rust
pub const VHOST_VDPA_SET_CONFIG_CALL: u32 = 1074048887;
```

#### Constant `VIDIOC_SUBDEV_G_CROP`

```rust
pub const VIDIOC_SUBDEV_G_CROP: u32 = 3224917563;
```

#### Constant `VIDIOC_S_CROP`

```rust
pub const VIDIOC_S_CROP: u32 = 1075074620;
```

#### Constant `WDIOC_GETTEMP`

```rust
pub const WDIOC_GETTEMP: u32 = 2147768067;
```

#### Constant `IOC_OPAL_ADD_USR_TO_LR`

```rust
pub const IOC_OPAL_ADD_USR_TO_LR: u32 = 1092120804;
```

#### Constant `UI_SET_LEDBIT`

```rust
pub const UI_SET_LEDBIT: u32 = 1074025833;
```

#### Constant `NBD_SET_SOCK`

```rust
pub const NBD_SET_SOCK: u32 = 43776;
```

#### Constant `BTRFS_IOC_SNAP_DESTROY_V2`

```rust
pub const BTRFS_IOC_SNAP_DESTROY_V2: u32 = 1342215231;
```

#### Constant `HIDIOCGCOLLECTIONINFO`

```rust
pub const HIDIOCGCOLLECTIONINFO: u32 = 3222292497;
```

#### Constant `I2OSWUL`

```rust
pub const I2OSWUL: u32 = 3224398086;
```

#### Constant `IOCTL_MEI_NOTIFY_GET`

```rust
pub const IOCTL_MEI_NOTIFY_GET: u32 = 2147764227;
```

#### Constant `FDFMTTRK`

```rust
pub const FDFMTTRK: u32 = 1074528840;
```

#### Constant `MMTIMER_GETBITS`

```rust
pub const MMTIMER_GETBITS: u32 = 27908;
```

#### Constant `VIDIOC_ENUMSTD`

```rust
pub const VIDIOC_ENUMSTD: u32 = 3225966105;
```

#### Constant `VHOST_GET_VRING_BASE`

```rust
pub const VHOST_GET_VRING_BASE: u32 = 3221794578;
```

#### Constant `VFIO_DEVICE_IOEVENTFD`

```rust
pub const VFIO_DEVICE_IOEVENTFD: u32 = 15220;
```

#### Constant `ATMARP_SETENTRY`

```rust
pub const ATMARP_SETENTRY: u32 = 25059;
```

#### Constant `CCISS_REVALIDVOLS`

```rust
pub const CCISS_REVALIDVOLS: u32 = 16906;
```

#### Constant `MGSL_IOCLOOPTXDONE`

```rust
pub const MGSL_IOCLOOPTXDONE: u32 = 27913;
```

#### Constant `RTC_VL_READ`

```rust
pub const RTC_VL_READ: u32 = 2147774483;
```

#### Constant `ND_IOCTL_ARS_STATUS`

```rust
pub const ND_IOCTL_ARS_STATUS: u32 = 3224391171;
```

#### Constant `RIO_DEV_DEL`

```rust
pub const RIO_DEV_DEL: u32 = 1075866904;
```

#### Constant `VBG_IOCTL_ACQUIRE_GUEST_CAPABILITIES`

```rust
pub const VBG_IOCTL_ACQUIRE_GUEST_CAPABILITIES: u32 = 3223606797;
```

#### Constant `VIDIOC_SUBDEV_DV_TIMINGS_CAP`

```rust
pub const VIDIOC_SUBDEV_DV_TIMINGS_CAP: u32 = 3230684772;
```

#### Constant `SONYPI_IOCSFAN`

```rust
pub const SONYPI_IOCSFAN: u32 = 1073837579;
```

#### Constant `SPIOCSTYPE`

```rust
pub const SPIOCSTYPE: u32 = 1074295041;
```

#### Constant `IPMICTL_REGISTER_FOR_CMD`

```rust
pub const IPMICTL_REGISTER_FOR_CMD: u32 = 2147641614;
```

#### Constant `I8K_GET_FAN`

```rust
pub const I8K_GET_FAN: u32 = 3221776774;
```

#### Constant `TUNGETVNETBE`

```rust
pub const TUNGETVNETBE: u32 = 2147767519;
```

#### Constant `AUTOFS_DEV_IOCTL_FAIL`

```rust
pub const AUTOFS_DEV_IOCTL_FAIL: u32 = 3222836087;
```

#### Constant `UI_END_FF_UPLOAD`

```rust
pub const UI_END_FF_UPLOAD: u32 = 1080579529;
```

#### Constant `TOSH_SMM`

```rust
pub const TOSH_SMM: u32 = 3222828176;
```

#### Constant `SONYPI_IOCGBAT2REM`

```rust
pub const SONYPI_IOCGBAT2REM: u32 = 2147644933;
```

#### Constant `F2FS_IOC_GET_COMPRESS_BLOCKS`

```rust
pub const F2FS_IOC_GET_COMPRESS_BLOCKS: u32 = 2148070673;
```

#### Constant `PPPIOCSNPMODE`

```rust
pub const PPPIOCSNPMODE: u32 = 1074295883;
```

#### Constant `USBDEVFS_CONTROL`

```rust
pub const USBDEVFS_CONTROL: u32 = 3222820096;
```

#### Constant `HIDIOCGUSAGE`

```rust
pub const HIDIOCGUSAGE: u32 = 3222816779;
```

#### Constant `TUNSETTXFILTER`

```rust
pub const TUNSETTXFILTER: u32 = 1074025681;
```

#### Constant `TUNGETVNETLE`

```rust
pub const TUNGETVNETLE: u32 = 2147767517;
```

#### Constant `VIDIOC_ENUM_DV_TIMINGS`

```rust
pub const VIDIOC_ENUM_DV_TIMINGS: u32 = 3230946914;
```

#### Constant `BTRFS_IOC_INO_PATHS`

```rust
pub const BTRFS_IOC_INO_PATHS: u32 = 3224933411;
```

#### Constant `MGSL_IOCGXSYNC`

```rust
pub const MGSL_IOCGXSYNC: u32 = 27924;
```

#### Constant `HIDIOCGFIELDINFO`

```rust
pub const HIDIOCGFIELDINFO: u32 = 3224913930;
```

#### Constant `VIDIOC_SUBDEV_G_STD`

```rust
pub const VIDIOC_SUBDEV_G_STD: u32 = 2148029975;
```

#### Constant `I2OVALIDATE`

```rust
pub const I2OVALIDATE: u32 = 2147772680;
```

#### Constant `VIDIOC_TRY_ENCODER_CMD`

```rust
pub const VIDIOC_TRY_ENCODER_CMD: u32 = 3223869006;
```

#### Constant `NILFS_IOCTL_GET_CPINFO`

```rust
pub const NILFS_IOCTL_GET_CPINFO: u32 = 2149084802;
```

#### Constant `VIDIOC_G_FREQUENCY`

```rust
pub const VIDIOC_G_FREQUENCY: u32 = 3224131128;
```

#### Constant `VFAT_IOCTL_READDIR_SHORT`

```rust
pub const VFAT_IOCTL_READDIR_SHORT: u32 = 2184212994;
```

#### Constant `ND_IOCTL_GET_CONFIG_DATA`

```rust
pub const ND_IOCTL_GET_CONFIG_DATA: u32 = 3222031877;
```

#### Constant `F2FS_IOC_RESERVE_COMPRESS_BLOCKS`

```rust
pub const F2FS_IOC_RESERVE_COMPRESS_BLOCKS: u32 = 2148070675;
```

#### Constant `FDGETDRVSTAT`

```rust
pub const FDGETDRVSTAT: u32 = 2152727058;
```

#### Constant `SYNC_IOC_MERGE`

```rust
pub const SYNC_IOC_MERGE: u32 = 3224387075;
```

#### Constant `VIDIOC_S_DV_TIMINGS`

```rust
pub const VIDIOC_S_DV_TIMINGS: u32 = 3229898327;
```

#### Constant `PPPIOCBRIDGECHAN`

```rust
pub const PPPIOCBRIDGECHAN: u32 = 1074033717;
```

#### Constant `LIRC_SET_SEND_MODE`

```rust
pub const LIRC_SET_SEND_MODE: u32 = 1074030865;
```

#### Constant `RIO_ENABLE_PORTWRITE_RANGE`

```rust
pub const RIO_ENABLE_PORTWRITE_RANGE: u32 = 1074818315;
```

#### Constant `ATM_GETTYPE`

```rust
pub const ATM_GETTYPE: u32 = 1074815364;
```

#### Constant `PHN_GETREGS`

```rust
pub const PHN_GETREGS: u32 = 3223875591;
```

#### Constant `FDSETEMSGTRESH`

```rust
pub const FDSETEMSGTRESH: u32 = 586;
```

#### Constant `NILFS_IOCTL_GET_VINFO`

```rust
pub const NILFS_IOCTL_GET_VINFO: u32 = 3222826630;
```

#### Constant `MGSL_IOCWAITEVENT`

```rust
pub const MGSL_IOCWAITEVENT: u32 = 3221515528;
```

#### Constant `CAPI_INSTALLED`

```rust
pub const CAPI_INSTALLED: u32 = 2147631906;
```

#### Constant `EVIOCGMASK`

```rust
pub const EVIOCGMASK: u32 = 2148550034;
```

#### Constant `BTRFS_IOC_SUBVOL_GETFLAGS`

```rust
pub const BTRFS_IOC_SUBVOL_GETFLAGS: u32 = 2148045849;
```

#### Constant `FSL_HV_IOCTL_PARTITION_GET_STATUS`

```rust
pub const FSL_HV_IOCTL_PARTITION_GET_STATUS: u32 = 3222056706;
```

#### Constant `MEDIA_IOC_ENUM_ENTITIES`

```rust
pub const MEDIA_IOC_ENUM_ENTITIES: u32 = 3238034433;
```

#### Constant `GSMIOC_GETFIRST`

```rust
pub const GSMIOC_GETFIRST: u32 = 2147763972;
```

#### Constant `FW_CDEV_IOC_FLUSH_ISO`

```rust
pub const FW_CDEV_IOC_FLUSH_ISO: u32 = 1074012952;
```

#### Constant `VIDIOC_DBG_G_CHIP_INFO`

```rust
pub const VIDIOC_DBG_G_CHIP_INFO: u32 = 3234354790;
```

#### Constant `F2FS_IOC_RELEASE_VOLATILE_WRITE`

```rust
pub const F2FS_IOC_RELEASE_VOLATILE_WRITE: u32 = 62724;
```

#### Constant `CAPI_GET_SERIAL`

```rust
pub const CAPI_GET_SERIAL: u32 = 3221504776;
```

#### Constant `FDSETDRVPRM`

```rust
pub const FDSETDRVPRM: u32 = 1082131088;
```

#### Constant `IOC_OPAL_SAVE`

```rust
pub const IOC_OPAL_SAVE: u32 = 1092120796;
```

#### Constant `VIDIOC_G_DV_TIMINGS`

```rust
pub const VIDIOC_G_DV_TIMINGS: u32 = 3229898328;
```

#### Constant `TUNSETIFINDEX`

```rust
pub const TUNSETIFINDEX: u32 = 1074025690;
```

#### Constant `CCISS_SETINTINFO`

```rust
pub const CCISS_SETINTINFO: u32 = 1074283011;
```

#### Constant `RTC_VL_CLR`

```rust
pub const RTC_VL_CLR: u32 = 28692;
```

#### Constant `VIDIOC_REQBUFS`

```rust
pub const VIDIOC_REQBUFS: u32 = 3222558216;
```

#### Constant `USBDEVFS_REAPURBNDELAY32`

```rust
pub const USBDEVFS_REAPURBNDELAY32: u32 = 1074025741;
```

#### Constant `TEE_IOC_SHM_REGISTER`

```rust
pub const TEE_IOC_SHM_REGISTER: u32 = 3222840329;
```

#### Constant `USBDEVFS_SETCONFIGURATION`

```rust
pub const USBDEVFS_SETCONFIGURATION: u32 = 2147767557;
```

#### Constant `CCISS_GETNODENAME`

```rust
pub const CCISS_GETNODENAME: u32 = 2148549124;
```

#### Constant `VIDIOC_SUBDEV_S_FRAME_INTERVAL`

```rust
pub const VIDIOC_SUBDEV_S_FRAME_INTERVAL: u32 = 3224393238;
```

#### Constant `VIDIOC_ENUM_FRAMESIZES`

```rust
pub const VIDIOC_ENUM_FRAMESIZES: u32 = 3224131146;
```

#### Constant `VFIO_DEVICE_PCI_HOT_RESET`

```rust
pub const VFIO_DEVICE_PCI_HOT_RESET: u32 = 15217;
```

#### Constant `FW_CDEV_IOC_SEND_BROADCAST_REQUEST`

```rust
pub const FW_CDEV_IOC_SEND_BROADCAST_REQUEST: u32 = 1076372242;
```

#### Constant `LPSETTIMEOUT_NEW`

```rust
pub const LPSETTIMEOUT_NEW: u32 = 1074791951;
```

#### Constant `RIO_CM_MPORT_GET_LIST`

```rust
pub const RIO_CM_MPORT_GET_LIST: u32 = 3221512971;
```

#### Constant `FW_CDEV_IOC_QUEUE_ISO`

```rust
pub const FW_CDEV_IOC_QUEUE_ISO: u32 = 3222807305;
```

#### Constant `FDRAWCMD`

```rust
pub const FDRAWCMD: u32 = 600;
```

#### Constant `SCIF_UNREG`

```rust
pub const SCIF_UNREG: u32 = 3222303497;
```

#### Constant `PPPIOCGIDLE64`

```rust
pub const PPPIOCGIDLE64: u32 = 2148561983;
```

#### Constant `USBDEVFS_RELEASEINTERFACE`

```rust
pub const USBDEVFS_RELEASEINTERFACE: u32 = 2147767568;
```

#### Constant `VIDIOC_CROPCAP`

```rust
pub const VIDIOC_CROPCAP: u32 = 3224131130;
```

#### Constant `DFL_FPGA_PORT_GET_INFO`

```rust
pub const DFL_FPGA_PORT_GET_INFO: u32 = 46657;
```

#### Constant `PHN_SET_REGS`

```rust
pub const PHN_SET_REGS: u32 = 1074294787;
```

#### Constant `ATMLEC_DATA`

```rust
pub const ATMLEC_DATA: u32 = 25041;
```

#### Constant `PPPOEIOCDFWD`

```rust
pub const PPPOEIOCDFWD: u32 = 45313;
```

#### Constant `VIDIOC_S_SELECTION`

```rust
pub const VIDIOC_S_SELECTION: u32 = 3225441887;
```

#### Constant `SNAPSHOT_FREE_SWAP_PAGES`

```rust
pub const SNAPSHOT_FREE_SWAP_PAGES: u32 = 13065;
```

#### Constant `BTRFS_IOC_LOGICAL_INO`

```rust
pub const BTRFS_IOC_LOGICAL_INO: u32 = 3224933412;
```

#### Constant `VIDIOC_S_CTRL`

```rust
pub const VIDIOC_S_CTRL: u32 = 3221771804;
```

#### Constant `ZATM_SETPOOL`

```rust
pub const ZATM_SETPOOL: u32 = 1074815331;
```

#### Constant `MTIOCPOS`

```rust
pub const MTIOCPOS: u32 = 2148035843;
```

#### Constant `PMU_IOC_SLEEP`

```rust
pub const PMU_IOC_SLEEP: u32 = 16896;
```

#### Constant `AUTOFS_DEV_IOCTL_PROTOSUBVER`

```rust
pub const AUTOFS_DEV_IOCTL_PROTOSUBVER: u32 = 3222836083;
```

#### Constant `VBG_IOCTL_CHANGE_FILTER_MASK`

```rust
pub const VBG_IOCTL_CHANGE_FILTER_MASK: u32 = 3223344652;
```

#### Constant `NILFS_IOCTL_GET_SUSTAT`

```rust
pub const NILFS_IOCTL_GET_SUSTAT: u32 = 2150657669;
```

#### Constant `VIDIOC_QUERYCAP`

```rust
pub const VIDIOC_QUERYCAP: u32 = 2154321408;
```

#### Constant `HPET_INFO`

```rust
pub const HPET_INFO: u32 = 2149083139;
```

#### Constant `VIDIOC_AM437X_CCDC_CFG`

```rust
pub const VIDIOC_AM437X_CCDC_CFG: u32 = 1074288321;
```

#### Constant `DM_LIST_DEVICES`

```rust
pub const DM_LIST_DEVICES: u32 = 3241737474;
```

#### Constant `TUNSETOWNER`

```rust
pub const TUNSETOWNER: u32 = 1074025676;
```

#### Constant `VBG_IOCTL_CHANGE_GUEST_CAPABILITIES`

```rust
pub const VBG_IOCTL_CHANGE_GUEST_CAPABILITIES: u32 = 3223344654;
```

#### Constant `RNDADDENTROPY`

```rust
pub const RNDADDENTROPY: u32 = 1074287107;
```

#### Constant `USBDEVFS_RESET`

```rust
pub const USBDEVFS_RESET: u32 = 21780;
```

#### Constant `BTRFS_IOC_SUBVOL_CREATE`

```rust
pub const BTRFS_IOC_SUBVOL_CREATE: u32 = 1342215182;
```

#### Constant `USBDEVFS_FORBID_SUSPEND`

```rust
pub const USBDEVFS_FORBID_SUSPEND: u32 = 21793;
```

#### Constant `FDGETDRVTYP`

```rust
pub const FDGETDRVTYP: u32 = 2148532751;
```

#### Constant `PPWCONTROL`

```rust
pub const PPWCONTROL: u32 = 1073836164;
```

#### Constant `VIDIOC_ENUM_FRAMEINTERVALS`

```rust
pub const VIDIOC_ENUM_FRAMEINTERVALS: u32 = 3224655435;
```

#### Constant `KCOV_DISABLE`

```rust
pub const KCOV_DISABLE: u32 = 25445;
```

#### Constant `IOC_OPAL_ACTIVATE_LSP`

```rust
pub const IOC_OPAL_ACTIVATE_LSP: u32 = 1092120799;
```

#### Constant `VHOST_VDPA_GET_IOVA_RANGE`

```rust
pub const VHOST_VDPA_GET_IOVA_RANGE: u32 = 2148577144;
```

#### Constant `PPPIOCSPASS`

```rust
pub const PPPIOCSPASS: u32 = 1074820167;
```

#### Constant `RIO_CM_CHAN_CONNECT`

```rust
pub const RIO_CM_CHAN_CONNECT: u32 = 1074291464;
```

#### Constant `I2OSWDEL`

```rust
pub const I2OSWDEL: u32 = 3224398087;
```

#### Constant `FS_IOC_SET_ENCRYPTION_POLICY`

```rust
pub const FS_IOC_SET_ENCRYPTION_POLICY: u32 = 2148296211;
```

#### Constant `IOC_OPAL_MBR_DONE`

```rust
pub const IOC_OPAL_MBR_DONE: u32 = 1091596521;
```

#### Constant `PPPIOCSMAXCID`

```rust
pub const PPPIOCSMAXCID: u32 = 1074033745;
```

#### Constant `PPSETPHASE`

```rust
pub const PPSETPHASE: u32 = 1074032788;
```

#### Constant `VHOST_VDPA_SET_VRING_ENABLE`

```rust
pub const VHOST_VDPA_SET_VRING_ENABLE: u32 = 1074311029;
```

#### Constant `USBDEVFS_GET_SPEED`

```rust
pub const USBDEVFS_GET_SPEED: u32 = 21791;
```

#### Constant `SONET_GETFRAMING`

```rust
pub const SONET_GETFRAMING: u32 = 2147770646;
```

#### Constant `VIDIOC_QUERYBUF`

```rust
pub const VIDIOC_QUERYBUF: u32 = 3227014665;
```

#### Constant `VIDIOC_S_EDID`

```rust
pub const VIDIOC_S_EDID: u32 = 3223868969;
```

#### Constant `BTRFS_IOC_QGROUP_ASSIGN`

```rust
pub const BTRFS_IOC_QGROUP_ASSIGN: u32 = 1075352617;
```

#### Constant `PPS_GETCAP`

```rust
pub const PPS_GETCAP: u32 = 2148036771;
```

#### Constant `SNAPSHOT_PLATFORM_SUPPORT`

```rust
pub const SNAPSHOT_PLATFORM_SUPPORT: u32 = 13071;
```

#### Constant `LIRC_SET_REC_TIMEOUT_REPORTS`

```rust
pub const LIRC_SET_REC_TIMEOUT_REPORTS: u32 = 1074030873;
```

#### Constant `SCIF_GET_NODEIDS`

```rust
pub const SCIF_GET_NODEIDS: u32 = 3222827790;
```

#### Constant `NBD_DISCONNECT`

```rust
pub const NBD_DISCONNECT: u32 = 43784;
```

#### Constant `VIDIOC_SUBDEV_G_FRAME_INTERVAL`

```rust
pub const VIDIOC_SUBDEV_G_FRAME_INTERVAL: u32 = 3224393237;
```

#### Constant `VFIO_IOMMU_DISABLE`

```rust
pub const VFIO_IOMMU_DISABLE: u32 = 15220;
```

#### Constant `SNAPSHOT_CREATE_IMAGE`

```rust
pub const SNAPSHOT_CREATE_IMAGE: u32 = 1074017041;
```

#### Constant `SNAPSHOT_POWER_OFF`

```rust
pub const SNAPSHOT_POWER_OFF: u32 = 13072;
```

#### Constant `APM_IOC_STANDBY`

```rust
pub const APM_IOC_STANDBY: u32 = 16641;
```

#### Constant `PPPIOCGUNIT`

```rust
pub const PPPIOCGUNIT: u32 = 2147775574;
```

#### Constant `AUTOFS_IOC_EXPIRE_MULTI`

```rust
pub const AUTOFS_IOC_EXPIRE_MULTI: u32 = 1074041702;
```

#### Constant `SCIF_BIND`

```rust
pub const SCIF_BIND: u32 = 3221779201;
```

#### Constant `IOC_WATCH_QUEUE_SET_SIZE`

```rust
pub const IOC_WATCH_QUEUE_SET_SIZE: u32 = 22368;
```

#### Constant `NILFS_IOCTL_CHANGE_CPMODE`

```rust
pub const NILFS_IOCTL_CHANGE_CPMODE: u32 = 1074818688;
```

#### Constant `IOC_OPAL_LOCK_UNLOCK`

```rust
pub const IOC_OPAL_LOCK_UNLOCK: u32 = 1092120797;
```

#### Constant `F2FS_IOC_SET_PIN_FILE`

```rust
pub const F2FS_IOC_SET_PIN_FILE: u32 = 1074066701;
```

#### Constant `PPPIOCGRASYNCMAP`

```rust
pub const PPPIOCGRASYNCMAP: u32 = 2147775573;
```

#### Constant `MMTIMER_MMAPAVAIL`

```rust
pub const MMTIMER_MMAPAVAIL: u32 = 27910;
```

#### Constant `I2OPASSTHRU32`

```rust
pub const I2OPASSTHRU32: u32 = 2148034828;
```

#### Constant `DFL_FPGA_FME_PORT_RELEASE`

```rust
pub const DFL_FPGA_FME_PORT_RELEASE: u32 = 1074050689;
```

#### Constant `VIDIOC_SUBDEV_QUERY_DV_TIMINGS`

```rust
pub const VIDIOC_SUBDEV_QUERY_DV_TIMINGS: u32 = 2156156515;
```

#### Constant `UI_SET_SNDBIT`

```rust
pub const UI_SET_SNDBIT: u32 = 1074025834;
```

#### Constant `VIDIOC_G_AUDOUT`

```rust
pub const VIDIOC_G_AUDOUT: u32 = 2150913585;
```

#### Constant `RTC_PLL_SET`

```rust
pub const RTC_PLL_SET: u32 = 1075867666;
```

#### Constant `VIDIOC_ENUMAUDIO`

```rust
pub const VIDIOC_ENUMAUDIO: u32 = 3224655425;
```

#### Constant `AUTOFS_DEV_IOCTL_TIMEOUT`

```rust
pub const AUTOFS_DEV_IOCTL_TIMEOUT: u32 = 3222836090;
```

#### Constant `VBG_IOCTL_DRIVER_VERSION_INFO`

```rust
pub const VBG_IOCTL_DRIVER_VERSION_INFO: u32 = 3224131072;
```

#### Constant `VHOST_SCSI_GET_EVENTS_MISSED`

```rust
pub const VHOST_SCSI_GET_EVENTS_MISSED: u32 = 1074048836;
```

#### Constant `VHOST_SET_VRING_ADDR`

```rust
pub const VHOST_SET_VRING_ADDR: u32 = 1076408081;
```

#### Constant `VDUSE_CREATE_DEV`

```rust
pub const VDUSE_CREATE_DEV: u32 = 1095794946;
```

#### Constant `FDFLUSH`

```rust
pub const FDFLUSH: u32 = 587;
```

#### Constant `VBG_IOCTL_WAIT_FOR_EVENTS`

```rust
pub const VBG_IOCTL_WAIT_FOR_EVENTS: u32 = 3223344650;
```

#### Constant `DFL_FPGA_FME_ERR_SET_IRQ`

```rust
pub const DFL_FPGA_FME_ERR_SET_IRQ: u32 = 1074312836;
```

#### Constant `F2FS_IOC_GET_PIN_FILE`

```rust
pub const F2FS_IOC_GET_PIN_FILE: u32 = 2147808526;
```

#### Constant `SCIF_CONNECT`

```rust
pub const SCIF_CONNECT: u32 = 3221779203;
```

#### Constant `BLKREPORTZONE`

```rust
pub const BLKREPORTZONE: u32 = 3222278786;
```

#### Constant `AUTOFS_IOC_ASKUMOUNT`

```rust
pub const AUTOFS_IOC_ASKUMOUNT: u32 = 2147783536;
```

#### Constant `ATM_ADDPARTY`

```rust
pub const ATM_ADDPARTY: u32 = 1074815476;
```

#### Constant `FDSETPRM`

```rust
pub const FDSETPRM: u32 = 1075839554;
```

#### Constant `ATM_GETSTATZ`

```rust
pub const ATM_GETSTATZ: u32 = 1074815313;
```

#### Constant `ISST_IF_MSR_COMMAND`

```rust
pub const ISST_IF_MSR_COMMAND: u32 = 3221814788;
```

#### Constant `BTRFS_IOC_GET_SUBVOL_INFO`

```rust
pub const BTRFS_IOC_GET_SUBVOL_INFO: u32 = 2180551740;
```

#### Constant `VIDIOC_UNSUBSCRIBE_EVENT`

```rust
pub const VIDIOC_UNSUBSCRIBE_EVENT: u32 = 1075861083;
```

#### Constant `SEV_ISSUE_CMD`

```rust
pub const SEV_ISSUE_CMD: u32 = 3222295296;
```

#### Constant `GPIOHANDLE_SET_LINE_VALUES_IOCTL`

```rust
pub const GPIOHANDLE_SET_LINE_VALUES_IOCTL: u32 = 3225465865;
```

#### Constant `PCITEST_COPY`

```rust
pub const PCITEST_COPY: u32 = 1074286598;
```

#### Constant `IPMICTL_GET_MY_ADDRESS_CMD`

```rust
pub const IPMICTL_GET_MY_ADDRESS_CMD: u32 = 2147772690;
```

#### Constant `CHIOGPICKER`

```rust
pub const CHIOGPICKER: u32 = 2147771140;
```

#### Constant `CAPI_NCCI_OPENCOUNT`

```rust
pub const CAPI_NCCI_OPENCOUNT: u32 = 2147762982;
```

#### Constant `CXL_MEM_SEND_COMMAND`

```rust
pub const CXL_MEM_SEND_COMMAND: u32 = 3224423938;
```

#### Constant `PERF_EVENT_IOC_SET_FILTER`

```rust
pub const PERF_EVENT_IOC_SET_FILTER: u32 = 1074275334;
```

#### Constant `IOC_OPAL_REVERT_TPR`

```rust
pub const IOC_OPAL_REVERT_TPR: u32 = 1091072226;
```

#### Constant `CHIOGVPARAMS`

```rust
pub const CHIOGVPARAMS: u32 = 2154849043;
```

#### Constant `PTP_PEROUT_REQUEST`

```rust
pub const PTP_PEROUT_REQUEST: u32 = 1077427459;
```

#### Constant `FSI_SCOM_CHECK`

```rust
pub const FSI_SCOM_CHECK: u32 = 2147775232;
```

#### Constant `RTC_IRQP_READ`

```rust
pub const RTC_IRQP_READ: u32 = 2148036619;
```

#### Constant `RIO_MPORT_MAINT_READ_LOCAL`

```rust
pub const RIO_MPORT_MAINT_READ_LOCAL: u32 = 2149084421;
```

#### Constant `HIDIOCGRDESCSIZE`

```rust
pub const HIDIOCGRDESCSIZE: u32 = 2147764225;
```

#### Constant `UI_GET_VERSION`

```rust
pub const UI_GET_VERSION: u32 = 2147767597;
```

#### Constant `NILFS_IOCTL_GET_CPSTAT`

```rust
pub const NILFS_IOCTL_GET_CPSTAT: u32 = 2149084803;
```

#### Constant `CCISS_GETBUSTYPES`

```rust
pub const CCISS_GETBUSTYPES: u32 = 2147762695;
```

#### Constant `VFIO_IOMMU_SPAPR_TCE_CREATE`

```rust
pub const VFIO_IOMMU_SPAPR_TCE_CREATE: u32 = 15223;
```

#### Constant `VIDIOC_EXPBUF`

```rust
pub const VIDIOC_EXPBUF: u32 = 3225441808;
```

#### Constant `UI_SET_RELBIT`

```rust
pub const UI_SET_RELBIT: u32 = 1074025830;
```

#### Constant `VFIO_SET_IOMMU`

```rust
pub const VFIO_SET_IOMMU: u32 = 15206;
```

#### Constant `VIDIOC_S_MODULATOR`

```rust
pub const VIDIOC_S_MODULATOR: u32 = 1078220343;
```

#### Constant `TUNGETFILTER`

```rust
pub const TUNGETFILTER: u32 = 2148553947;
```

#### Constant `CCISS_SETNODENAME`

```rust
pub const CCISS_SETNODENAME: u32 = 1074807301;
```

#### Constant `FBIO_GETCONTROL2`

```rust
pub const FBIO_GETCONTROL2: u32 = 2148025993;
```

#### Constant `TUNSETDEBUG`

```rust
pub const TUNSETDEBUG: u32 = 1074025673;
```

#### Constant `DM_DEV_REMOVE`

```rust
pub const DM_DEV_REMOVE: u32 = 3241737476;
```

#### Constant `HIDIOCSUSAGES`

```rust
pub const HIDIOCSUSAGES: u32 = 1344030740;
```

#### Constant `FS_IOC_ADD_ENCRYPTION_KEY`

```rust
pub const FS_IOC_ADD_ENCRYPTION_KEY: u32 = 3226494487;
```

#### Constant `FBIOGET_VBLANK`

```rust
pub const FBIOGET_VBLANK: u32 = 2149598738;
```

#### Constant `ATM_GETSTAT`

```rust
pub const ATM_GETSTAT: u32 = 1074815312;
```

#### Constant `VIDIOC_G_JPEGCOMP`

```rust
pub const VIDIOC_G_JPEGCOMP: u32 = 2156680765;
```

#### Constant `TUNATTACHFILTER`

```rust
pub const TUNATTACHFILTER: u32 = 1074812117;
```

#### Constant `UI_SET_ABSBIT`

```rust
pub const UI_SET_ABSBIT: u32 = 1074025831;
```

#### Constant `DFL_FPGA_PORT_ERR_GET_IRQ_NUM`

```rust
pub const DFL_FPGA_PORT_ERR_GET_IRQ_NUM: u32 = 2147792453;
```

#### Constant `USBDEVFS_REAPURB32`

```rust
pub const USBDEVFS_REAPURB32: u32 = 1074025740;
```

#### Constant `BTRFS_IOC_TRANS_END`

```rust
pub const BTRFS_IOC_TRANS_END: u32 = 37895;
```

#### Constant `CAPI_REGISTER`

```rust
pub const CAPI_REGISTER: u32 = 1074545409;
```

#### Constant `F2FS_IOC_COMPRESS_FILE`

```rust
pub const F2FS_IOC_COMPRESS_FILE: u32 = 62744;
```

#### Constant `USBDEVFS_DISCARDURB`

```rust
pub const USBDEVFS_DISCARDURB: u32 = 21771;
```

#### Constant `HE_GET_REG`

```rust
pub const HE_GET_REG: u32 = 1074815328;
```

#### Constant `ATM_SETLOOP`

```rust
pub const ATM_SETLOOP: u32 = 1074815315;
```

#### Constant `ATMSIGD_CTRL`

```rust
pub const ATMSIGD_CTRL: u32 = 25072;
```

#### Constant `CIOC_KERNEL_VERSION`

```rust
pub const CIOC_KERNEL_VERSION: u32 = 3221775114;
```

#### Constant `BTRFS_IOC_CLONE_RANGE`

```rust
pub const BTRFS_IOC_CLONE_RANGE: u32 = 1075876877;
```

#### Constant `SNAPSHOT_UNFREEZE`

```rust
pub const SNAPSHOT_UNFREEZE: u32 = 13058;
```

#### Constant `F2FS_IOC_START_VOLATILE_WRITE`

```rust
pub const F2FS_IOC_START_VOLATILE_WRITE: u32 = 62723;
```

#### Constant `PMU_IOC_HAS_ADB`

```rust
pub const PMU_IOC_HAS_ADB: u32 = 2148024836;
```

#### Constant `I2OGETIOPS`

```rust
pub const I2OGETIOPS: u32 = 2149607680;
```

#### Constant `VIDIOC_S_FBUF`

```rust
pub const VIDIOC_S_FBUF: u32 = 1076909579;
```

#### Constant `PPRCONTROL`

```rust
pub const PPRCONTROL: u32 = 2147577987;
```

#### Constant `CHIOSPICKER`

```rust
pub const CHIOSPICKER: u32 = 1074029317;
```

#### Constant `VFIO_IOMMU_SPAPR_REGISTER_MEMORY`

```rust
pub const VFIO_IOMMU_SPAPR_REGISTER_MEMORY: u32 = 15221;
```

#### Constant `TUNGETSNDBUF`

```rust
pub const TUNGETSNDBUF: u32 = 2147767507;
```

#### Constant `GSMIOC_SETCONF`

```rust
pub const GSMIOC_SETCONF: u32 = 1078740737;
```

#### Constant `IOC_PR_PREEMPT`

```rust
pub const IOC_PR_PREEMPT: u32 = 1075343563;
```

#### Constant `KCOV_INIT_TRACE`

```rust
pub const KCOV_INIT_TRACE: u32 = 2148033281;
```

#### Constant `SONYPI_IOCGBAT1CAP`

```rust
pub const SONYPI_IOCGBAT1CAP: u32 = 2147644930;
```

#### Constant `SWITCHTEC_IOCTL_FLASH_INFO`

```rust
pub const SWITCHTEC_IOCTL_FLASH_INFO: u32 = 2148554560;
```

#### Constant `MTIOCTOP`

```rust
pub const MTIOCTOP: u32 = 1074294017;
```

#### Constant `VHOST_VDPA_SET_STATUS`

```rust
pub const VHOST_VDPA_SET_STATUS: u32 = 1073852274;
```

#### Constant `VHOST_SCSI_SET_EVENTS_MISSED`

```rust
pub const VHOST_SCSI_SET_EVENTS_MISSED: u32 = 1074048835;
```

#### Constant `VFIO_IOMMU_DIRTY_PAGES`

```rust
pub const VFIO_IOMMU_DIRTY_PAGES: u32 = 15221;
```

#### Constant `BTRFS_IOC_SCRUB_PROGRESS`

```rust
pub const BTRFS_IOC_SCRUB_PROGRESS: u32 = 3288372253;
```

#### Constant `PPPIOCGMRU`

```rust
pub const PPPIOCGMRU: u32 = 2147775571;
```

#### Constant `BTRFS_IOC_DEV_REPLACE`

```rust
pub const BTRFS_IOC_DEV_REPLACE: u32 = 3391657013;
```

#### Constant `PPPIOCGFLAGS`

```rust
pub const PPPIOCGFLAGS: u32 = 2147775578;
```

#### Constant `NILFS_IOCTL_SET_SUINFO`

```rust
pub const NILFS_IOCTL_SET_SUINFO: u32 = 1075342989;
```

#### Constant `FW_CDEV_IOC_GET_CYCLE_TIMER2`

```rust
pub const FW_CDEV_IOC_GET_CYCLE_TIMER2: u32 = 3222807316;
```

#### Constant `ATM_DELLECSADDR`

```rust
pub const ATM_DELLECSADDR: u32 = 1074815375;
```

#### Constant `FW_CDEV_IOC_GET_SPEED`

```rust
pub const FW_CDEV_IOC_GET_SPEED: u32 = 8977;
```

#### Constant `PPPIOCGIDLE32`

```rust
pub const PPPIOCGIDLE32: u32 = 2148037695;
```

#### Constant `VFIO_DEVICE_RESET`

```rust
pub const VFIO_DEVICE_RESET: u32 = 15215;
```

#### Constant `GPIO_GET_LINEINFO_UNWATCH_IOCTL`

```rust
pub const GPIO_GET_LINEINFO_UNWATCH_IOCTL: u32 = 3221533708;
```

#### Constant `WDIOC_GETSTATUS`

```rust
pub const WDIOC_GETSTATUS: u32 = 2147768065;
```

#### Constant `BTRFS_IOC_SET_FEATURES`

```rust
pub const BTRFS_IOC_SET_FEATURES: u32 = 1076925497;
```

#### Constant `IOCTL_MEI_CONNECT_CLIENT`

```rust
pub const IOCTL_MEI_CONNECT_CLIENT: u32 = 3222292481;
```

#### Constant `VIDIOC_OMAP3ISP_AEWB_CFG`

```rust
pub const VIDIOC_OMAP3ISP_AEWB_CFG: u32 = 3223344835;
```

#### Constant `PCITEST_READ`

```rust
pub const PCITEST_READ: u32 = 1074286597;
```

#### Constant `VFIO_GROUP_GET_STATUS`

```rust
pub const VFIO_GROUP_GET_STATUS: u32 = 15207;
```

#### Constant `MATROXFB_GET_ALL_OUTPUTS`

```rust
pub const MATROXFB_GET_ALL_OUTPUTS: u32 = 2148036347;
```

#### Constant `USBDEVFS_CLEAR_HALT`

```rust
pub const USBDEVFS_CLEAR_HALT: u32 = 2147767573;
```

#### Constant `VIDIOC_DECODER_CMD`

```rust
pub const VIDIOC_DECODER_CMD: u32 = 3225966176;
```

#### Constant `VIDIOC_G_AUDIO`

```rust
pub const VIDIOC_G_AUDIO: u32 = 2150913569;
```

#### Constant `CCISS_RESCANDISK`

```rust
pub const CCISS_RESCANDISK: u32 = 16912;
```

#### Constant `RIO_DISABLE_PORTWRITE_RANGE`

```rust
pub const RIO_DISABLE_PORTWRITE_RANGE: u32 = 1074818316;
```

#### Constant `IOC_OPAL_SECURE_ERASE_LR`

```rust
pub const IOC_OPAL_SECURE_ERASE_LR: u32 = 1091596519;
```

#### Constant `USBDEVFS_REAPURB`

```rust
pub const USBDEVFS_REAPURB: u32 = 1074287884;
```

#### Constant `DFL_FPGA_CHECK_EXTENSION`

```rust
pub const DFL_FPGA_CHECK_EXTENSION: u32 = 46593;
```

#### Constant `AUTOFS_IOC_PROTOVER`

```rust
pub const AUTOFS_IOC_PROTOVER: u32 = 2147783523;
```

#### Constant `FSL_HV_IOCTL_MEMCPY`

```rust
pub const FSL_HV_IOCTL_MEMCPY: u32 = 3223891717;
```

#### Constant `BTRFS_IOC_GET_FEATURES`

```rust
pub const BTRFS_IOC_GET_FEATURES: u32 = 2149094457;
```

#### Constant `PCITEST_MSIX`

```rust
pub const PCITEST_MSIX: u32 = 1074024455;
```

#### Constant `BTRFS_IOC_DEFRAG_RANGE`

```rust
pub const BTRFS_IOC_DEFRAG_RANGE: u32 = 1076925456;
```

#### Constant `UI_BEGIN_FF_ERASE`

```rust
pub const UI_BEGIN_FF_ERASE: u32 = 3222033866;
```

#### Constant `DM_GET_TARGET_VERSION`

```rust
pub const DM_GET_TARGET_VERSION: u32 = 3241737489;
```

#### Constant `PPPIOCGIDLE`

```rust
pub const PPPIOCGIDLE: u32 = 2148561983;
```

#### Constant `NVRAM_SETCKS`

```rust
pub const NVRAM_SETCKS: u32 = 28737;
```

#### Constant `WDIOC_GETSUPPORT`

```rust
pub const WDIOC_GETSUPPORT: u32 = 2150127360;
```

#### Constant `GSMIOC_ENABLE_NET`

```rust
pub const GSMIOC_ENABLE_NET: u32 = 1077167874;
```

#### Constant `GPIO_GET_CHIPINFO_IOCTL`

```rust
pub const GPIO_GET_CHIPINFO_IOCTL: u32 = 2151986177;
```

#### Constant `NE_ADD_VCPU`

```rust
pub const NE_ADD_VCPU: u32 = 3221532193;
```

#### Constant `EVIOCSKEYCODE_V2`

```rust
pub const EVIOCSKEYCODE_V2: u32 = 1076380932;
```

#### Constant `PTP_SYS_OFFSET_EXTENDED2`

```rust
pub const PTP_SYS_OFFSET_EXTENDED2: u32 = 3300932882;
```

#### Constant `SCIF_FENCE_WAIT`

```rust
pub const SCIF_FENCE_WAIT: u32 = 3221517072;
```

#### Constant `RIO_TRANSFER`

```rust
pub const RIO_TRANSFER: u32 = 3222826261;
```

#### Constant `FSL_HV_IOCTL_DOORBELL`

```rust
pub const FSL_HV_IOCTL_DOORBELL: u32 = 3221794566;
```

#### Constant `RIO_MPORT_MAINT_WRITE_LOCAL`

```rust
pub const RIO_MPORT_MAINT_WRITE_LOCAL: u32 = 1075342598;
```

#### Constant `I2OEVTREG`

```rust
pub const I2OEVTREG: u32 = 1074555146;
```

#### Constant `I2OPARMGET`

```rust
pub const I2OPARMGET: u32 = 3223873796;
```

#### Constant `EVIOCGID`

```rust
pub const EVIOCGID: u32 = 2148025602;
```

#### Constant `BTRFS_IOC_QGROUP_CREATE`

```rust
pub const BTRFS_IOC_QGROUP_CREATE: u32 = 1074828330;
```

#### Constant `AUTOFS_DEV_IOCTL_SETPIPEFD`

```rust
pub const AUTOFS_DEV_IOCTL_SETPIPEFD: u32 = 3222836088;
```

#### Constant `VIDIOC_S_PARM`

```rust
pub const VIDIOC_S_PARM: u32 = 3234616854;
```

#### Constant `TUNSETSTEERINGEBPF`

```rust
pub const TUNSETSTEERINGEBPF: u32 = 2147767520;
```

#### Constant `ATM_GETNAMES`

```rust
pub const ATM_GETNAMES: u32 = 1074815363;
```

#### Constant `VIDIOC_QUERYMENU`

```rust
pub const VIDIOC_QUERYMENU: u32 = 3224131109;
```

#### Constant `DFL_FPGA_PORT_DMA_UNMAP`

```rust
pub const DFL_FPGA_PORT_DMA_UNMAP: u32 = 46660;
```

#### Constant `I2OLCTGET`

```rust
pub const I2OLCTGET: u32 = 3222825218;
```

#### Constant `FS_IOC_GET_ENCRYPTION_PWSALT`

```rust
pub const FS_IOC_GET_ENCRYPTION_PWSALT: u32 = 1074816532;
```

#### Constant `NS_SETBUFLEV`

```rust
pub const NS_SETBUFLEV: u32 = 1074815330;
```

#### Constant `BLKCLOSEZONE`

```rust
pub const BLKCLOSEZONE: u32 = 1074795143;
```

#### Constant `SONET_GETFRSENSE`

```rust
pub const SONET_GETFRSENSE: u32 = 2147901719;
```

#### Constant `UI_SET_EVBIT`

```rust
pub const UI_SET_EVBIT: u32 = 1074025828;
```

#### Constant `DM_LIST_VERSIONS`

```rust
pub const DM_LIST_VERSIONS: u32 = 3241737485;
```

#### Constant `HIDIOCGSTRING`

```rust
pub const HIDIOCGSTRING: u32 = 2164541444;
```

#### Constant `PPPIOCATTCHAN`

```rust
pub const PPPIOCATTCHAN: u32 = 1074033720;
```

#### Constant `VDUSE_DEV_SET_CONFIG`

```rust
pub const VDUSE_DEV_SET_CONFIG: u32 = 1074299154;
```

#### Constant `TUNGETFEATURES`

```rust
pub const TUNGETFEATURES: u32 = 2147767503;
```

#### Constant `VFIO_GROUP_UNSET_CONTAINER`

```rust
pub const VFIO_GROUP_UNSET_CONTAINER: u32 = 15209;
```

#### Constant `IPMICTL_SET_MY_ADDRESS_CMD`

```rust
pub const IPMICTL_SET_MY_ADDRESS_CMD: u32 = 2147772689;
```

#### Constant `CCISS_REGNEWDISK`

```rust
pub const CCISS_REGNEWDISK: u32 = 1074020877;
```

#### Constant `VIDIOC_QUERY_DV_TIMINGS`

```rust
pub const VIDIOC_QUERY_DV_TIMINGS: u32 = 2156156515;
```

#### Constant `PHN_SETREGS`

```rust
pub const PHN_SETREGS: u32 = 1076391944;
```

#### Constant `FAT_IOCTL_GET_ATTRIBUTES`

```rust
pub const FAT_IOCTL_GET_ATTRIBUTES: u32 = 2147774992;
```

#### Constant `FSL_MC_SEND_MC_COMMAND`

```rust
pub const FSL_MC_SEND_MC_COMMAND: u32 = 3225440992;
```

#### Constant `TUNGETIFF`

```rust
pub const TUNGETIFF: u32 = 2147767506;
```

#### Constant `PTP_CLOCK_GETCAPS2`

```rust
pub const PTP_CLOCK_GETCAPS2: u32 = 2152742154;
```

#### Constant `BTRFS_IOC_RESIZE`

```rust
pub const BTRFS_IOC_RESIZE: u32 = 1342215171;
```

#### Constant `VHOST_SET_VRING_ENDIAN`

```rust
pub const VHOST_SET_VRING_ENDIAN: u32 = 1074310931;
```

#### Constant `PPS_KC_BIND`

```rust
pub const PPS_KC_BIND: u32 = 1074294949;
```

#### Constant `F2FS_IOC_WRITE_CHECKPOINT`

```rust
pub const F2FS_IOC_WRITE_CHECKPOINT: u32 = 62727;
```

#### Constant `UI_SET_FFBIT`

```rust
pub const UI_SET_FFBIT: u32 = 1074025835;
```

#### Constant `IPMICTL_GET_MY_LUN_CMD`

```rust
pub const IPMICTL_GET_MY_LUN_CMD: u32 = 2147772692;
```

#### Constant `CEC_ADAP_G_PHYS_ADDR`

```rust
pub const CEC_ADAP_G_PHYS_ADDR: u32 = 2147639553;
```

#### Constant `CEC_G_MODE`

```rust
pub const CEC_G_MODE: u32 = 2147770632;
```

#### Constant `USBDEVFS_RESETEP`

```rust
pub const USBDEVFS_RESETEP: u32 = 2147767555;
```

#### Constant `MEDIA_REQUEST_IOC_QUEUE`

```rust
pub const MEDIA_REQUEST_IOC_QUEUE: u32 = 31872;
```

#### Constant `USBDEVFS_ALLOC_STREAMS`

```rust
pub const USBDEVFS_ALLOC_STREAMS: u32 = 2148029724;
```

#### Constant `MGSL_IOCSXCTRL`

```rust
pub const MGSL_IOCSXCTRL: u32 = 27925;
```

#### Constant `MEDIA_IOC_G_TOPOLOGY`

```rust
pub const MEDIA_IOC_G_TOPOLOGY: u32 = 3225975812;
```

#### Constant `PPPIOCUNBRIDGECHAN`

```rust
pub const PPPIOCUNBRIDGECHAN: u32 = 29748;
```

#### Constant `F2FS_IOC_COMMIT_ATOMIC_WRITE`

```rust
pub const F2FS_IOC_COMMIT_ATOMIC_WRITE: u32 = 62722;
```

#### Constant `ISST_IF_GET_PLATFORM_INFO`

```rust
pub const ISST_IF_GET_PLATFORM_INFO: u32 = 2148072960;
```

#### Constant `SCIF_FENCE_MARK`

```rust
pub const SCIF_FENCE_MARK: u32 = 3222303503;
```

#### Constant `USBDEVFS_RELEASE_PORT`

```rust
pub const USBDEVFS_RELEASE_PORT: u32 = 2147767577;
```

#### Constant `VFIO_CHECK_EXTENSION`

```rust
pub const VFIO_CHECK_EXTENSION: u32 = 15205;
```

#### Constant `BTRFS_IOC_QGROUP_LIMIT`

```rust
pub const BTRFS_IOC_QGROUP_LIMIT: u32 = 2150667307;
```

#### Constant `FAT_IOCTL_GET_VOLUME_ID`

```rust
pub const FAT_IOCTL_GET_VOLUME_ID: u32 = 2147774995;
```

#### Constant `UI_SET_PHYS`

```rust
pub const UI_SET_PHYS: u32 = 1074287980;
```

#### Constant `FDWERRORGET`

```rust
pub const FDWERRORGET: u32 = 2150105623;
```

#### Constant `VIDIOC_SUBDEV_G_EDID`

```rust
pub const VIDIOC_SUBDEV_G_EDID: u32 = 3223868968;
```

#### Constant `MGSL_IOCGSTATS`

```rust
pub const MGSL_IOCGSTATS: u32 = 27911;
```

#### Constant `RPROC_SET_SHUTDOWN_ON_RELEASE`

```rust
pub const RPROC_SET_SHUTDOWN_ON_RELEASE: u32 = 1074050817;
```

#### Constant `SIOCGSTAMP_NEW`

```rust
pub const SIOCGSTAMP_NEW: u32 = 2148567302;
```

#### Constant `RTC_WKALM_RD`

```rust
pub const RTC_WKALM_RD: u32 = 2150133776;
```

#### Constant `PHN_GET_REG`

```rust
pub const PHN_GET_REG: u32 = 3221778432;
```

#### Constant `DELL_WMI_SMBIOS_CMD`

```rust
pub const DELL_WMI_SMBIOS_CMD: u32 = 3224655616;
```

#### Constant `PHN_NOT_OH`

```rust
pub const PHN_NOT_OH: u32 = 28676;
```

#### Constant `PPGETMODES`

```rust
pub const PPGETMODES: u32 = 2147774615;
```

#### Constant `CHIOGPARAMS`

```rust
pub const CHIOGPARAMS: u32 = 2148819718;
```

#### Constant `VFIO_DEVICE_GET_GFX_DMABUF`

```rust
pub const VFIO_DEVICE_GET_GFX_DMABUF: u32 = 15219;
```

#### Constant `VHOST_SET_VRING_BUSYLOOP_TIMEOUT`

```rust
pub const VHOST_SET_VRING_BUSYLOOP_TIMEOUT: u32 = 1074310947;
```

#### Constant `VIDIOC_SUBDEV_G_SELECTION`

```rust
pub const VIDIOC_SUBDEV_G_SELECTION: u32 = 3225441853;
```

#### Constant `BTRFS_IOC_RM_DEV_V2`

```rust
pub const BTRFS_IOC_RM_DEV_V2: u32 = 1342215226;
```

#### Constant `MGSL_IOCWAITGPIO`

```rust
pub const MGSL_IOCWAITGPIO: u32 = 3222301970;
```

#### Constant `PMU_IOC_CAN_SLEEP`

```rust
pub const PMU_IOC_CAN_SLEEP: u32 = 2148024837;
```

#### Constant `KCOV_ENABLE`

```rust
pub const KCOV_ENABLE: u32 = 25444;
```

#### Constant `BTRFS_IOC_CLONE`

```rust
pub const BTRFS_IOC_CLONE: u32 = 1074041865;
```

#### Constant `F2FS_IOC_DEFRAGMENT`

```rust
pub const F2FS_IOC_DEFRAGMENT: u32 = 3222336776;
```

#### Constant `FW_CDEV_IOC_DEALLOCATE_ISO_RESOURCE`

```rust
pub const FW_CDEV_IOC_DEALLOCATE_ISO_RESOURCE: u32 = 1074012942;
```

#### Constant `AGPIOC_ALLOCATE`

```rust
pub const AGPIOC_ALLOCATE: u32 = 3221766406;
```

#### Constant `NE_SET_USER_MEMORY_REGION`

```rust
pub const NE_SET_USER_MEMORY_REGION: u32 = 1075359267;
```

#### Constant `MGSL_IOCTXABORT`

```rust
pub const MGSL_IOCTXABORT: u32 = 27910;
```

#### Constant `MGSL_IOCSGPIO`

```rust
pub const MGSL_IOCSGPIO: u32 = 1074818320;
```

#### Constant `LIRC_SET_REC_CARRIER`

```rust
pub const LIRC_SET_REC_CARRIER: u32 = 1074030868;
```

#### Constant `F2FS_IOC_FLUSH_DEVICE`

```rust
pub const F2FS_IOC_FLUSH_DEVICE: u32 = 1074328842;
```

#### Constant `SNAPSHOT_ATOMIC_RESTORE`

```rust
pub const SNAPSHOT_ATOMIC_RESTORE: u32 = 13060;
```

#### Constant `RTC_UIE_OFF`

```rust
pub const RTC_UIE_OFF: u32 = 28676;
```

#### Constant `BT_BMC_IOCTL_SMS_ATN`

```rust
pub const BT_BMC_IOCTL_SMS_ATN: u32 = 45312;
```

#### Constant `NVME_IOCTL_ID`

```rust
pub const NVME_IOCTL_ID: u32 = 20032;
```

#### Constant `NE_START_ENCLAVE`

```rust
pub const NE_START_ENCLAVE: u32 = 3222318628;
```

#### Constant `VIDIOC_STREAMON`

```rust
pub const VIDIOC_STREAMON: u32 = 1074026002;
```

#### Constant `FDPOLLDRVSTAT`

```rust
pub const FDPOLLDRVSTAT: u32 = 2152727059;
```

#### Constant `AUTOFS_DEV_IOCTL_READY`

```rust
pub const AUTOFS_DEV_IOCTL_READY: u32 = 3222836086;
```

#### Constant `VIDIOC_ENUMAUDOUT`

```rust
pub const VIDIOC_ENUMAUDOUT: u32 = 3224655426;
```

#### Constant `VIDIOC_SUBDEV_S_STD`

```rust
pub const VIDIOC_SUBDEV_S_STD: u32 = 1074288152;
```

#### Constant `WDIOC_GETTIMELEFT`

```rust
pub const WDIOC_GETTIMELEFT: u32 = 2147768074;
```

#### Constant `ATM_GETLINKRATE`

```rust
pub const ATM_GETLINKRATE: u32 = 1074815361;
```

#### Constant `RTC_WKALM_SET`

```rust
pub const RTC_WKALM_SET: u32 = 1076391951;
```

#### Constant `VHOST_GET_BACKEND_FEATURES`

```rust
pub const VHOST_GET_BACKEND_FEATURES: u32 = 2148052774;
```

#### Constant `ATMARP_ENCAP`

```rust
pub const ATMARP_ENCAP: u32 = 25061;
```

#### Constant `CAPI_GET_FLAGS`

```rust
pub const CAPI_GET_FLAGS: u32 = 2147762979;
```

#### Constant `IPMICTL_SET_MY_CHANNEL_ADDRESS_CMD`

```rust
pub const IPMICTL_SET_MY_CHANNEL_ADDRESS_CMD: u32 = 2147772696;
```

#### Constant `DFL_FPGA_FME_PORT_ASSIGN`

```rust
pub const DFL_FPGA_FME_PORT_ASSIGN: u32 = 1074050690;
```

#### Constant `NS_GET_OWNER_UID`

```rust
pub const NS_GET_OWNER_UID: u32 = 46852;
```

#### Constant `VIDIOC_OVERLAY`

```rust
pub const VIDIOC_OVERLAY: u32 = 1074025998;
```

#### Constant `BTRFS_IOC_WAIT_SYNC`

```rust
pub const BTRFS_IOC_WAIT_SYNC: u32 = 1074304022;
```

#### Constant `GPIOHANDLE_SET_CONFIG_IOCTL`

```rust
pub const GPIOHANDLE_SET_CONFIG_IOCTL: u32 = 3226776586;
```

#### Constant `VHOST_GET_VRING_ENDIAN`

```rust
pub const VHOST_GET_VRING_ENDIAN: u32 = 1074310932;
```

#### Constant `ATM_GETADDR`

```rust
pub const ATM_GETADDR: u32 = 1074815366;
```

#### Constant `PHN_GET_REGS`

```rust
pub const PHN_GET_REGS: u32 = 3221778434;
```

#### Constant `AUTOFS_DEV_IOCTL_REQUESTER`

```rust
pub const AUTOFS_DEV_IOCTL_REQUESTER: u32 = 3222836091;
```

#### Constant `AUTOFS_DEV_IOCTL_EXPIRE`

```rust
pub const AUTOFS_DEV_IOCTL_EXPIRE: u32 = 3222836092;
```

#### Constant `SNAPSHOT_S2RAM`

```rust
pub const SNAPSHOT_S2RAM: u32 = 13067;
```

#### Constant `JSIOCSAXMAP`

```rust
pub const JSIOCSAXMAP: u32 = 1077963313;
```

#### Constant `F2FS_IOC_SET_COMPRESS_OPTION`

```rust
pub const F2FS_IOC_SET_COMPRESS_OPTION: u32 = 1073935638;
```

#### Constant `VBG_IOCTL_HGCM_DISCONNECT`

```rust
pub const VBG_IOCTL_HGCM_DISCONNECT: u32 = 3223082501;
```

#### Constant `SCIF_FENCE_SIGNAL`

```rust
pub const SCIF_FENCE_SIGNAL: u32 = 3223876369;
```

#### Constant `VFIO_DEVICE_GET_PCI_HOT_RESET_INFO`

```rust
pub const VFIO_DEVICE_GET_PCI_HOT_RESET_INFO: u32 = 15216;
```

#### Constant `VIDIOC_SUBDEV_ENUM_MBUS_CODE`

```rust
pub const VIDIOC_SUBDEV_ENUM_MBUS_CODE: u32 = 3224393218;
```

#### Constant `MMTIMER_GETOFFSET`

```rust
pub const MMTIMER_GETOFFSET: u32 = 27904;
```

#### Constant `RIO_CM_CHAN_LISTEN`

```rust
pub const RIO_CM_CHAN_LISTEN: u32 = 1073898246;
```

#### Constant `ATM_SETSC`

```rust
pub const ATM_SETSC: u32 = 1074029041;
```

#### Constant `F2FS_IOC_SHUTDOWN`

```rust
pub const F2FS_IOC_SHUTDOWN: u32 = 2147768445;
```

#### Constant `NVME_IOCTL_RESCAN`

```rust
pub const NVME_IOCTL_RESCAN: u32 = 20038;
```

#### Constant `BLKOPENZONE`

```rust
pub const BLKOPENZONE: u32 = 1074795142;
```

#### Constant `DM_VERSION`

```rust
pub const DM_VERSION: u32 = 3241737472;
```

#### Constant `CEC_TRANSMIT`

```rust
pub const CEC_TRANSMIT: u32 = 3224920325;
```

#### Constant `FS_IOC_GET_ENCRYPTION_POLICY_EX`

```rust
pub const FS_IOC_GET_ENCRYPTION_POLICY_EX: u32 = 3221841430;
```

#### Constant `SIOCMKCLIP`

```rust
pub const SIOCMKCLIP: u32 = 25056;
```

#### Constant `IPMI_BMC_IOCTL_CLEAR_SMS_ATN`

```rust
pub const IPMI_BMC_IOCTL_CLEAR_SMS_ATN: u32 = 45313;
```

#### Constant `HIDIOCGVERSION`

```rust
pub const HIDIOCGVERSION: u32 = 2147764225;
```

#### Constant `VIDIOC_S_INPUT`

```rust
pub const VIDIOC_S_INPUT: u32 = 3221509671;
```

#### Constant `VIDIOC_G_CROP`

```rust
pub const VIDIOC_G_CROP: u32 = 3222558267;
```

#### Constant `LIRC_SET_WIDEBAND_RECEIVER`

```rust
pub const LIRC_SET_WIDEBAND_RECEIVER: u32 = 1074030883;
```

#### Constant `EVIOCGEFFECTS`

```rust
pub const EVIOCGEFFECTS: u32 = 2147763588;
```

#### Constant `UVCIOC_CTRL_QUERY`

```rust
pub const UVCIOC_CTRL_QUERY: u32 = 3222304033;
```

#### Constant `IOC_OPAL_GENERIC_TABLE_RW`

```rust
pub const IOC_OPAL_GENERIC_TABLE_RW: u32 = 1094217963;
```

#### Constant `FS_IOC_READ_VERITY_METADATA`

```rust
pub const FS_IOC_READ_VERITY_METADATA: u32 = 3223873159;
```

#### Constant `ND_IOCTL_SET_CONFIG_DATA`

```rust
pub const ND_IOCTL_SET_CONFIG_DATA: u32 = 3221769734;
```

#### Constant `USBDEVFS_GETDRIVER`

```rust
pub const USBDEVFS_GETDRIVER: u32 = 1090802952;
```

#### Constant `IDT77105_GETSTAT`

```rust
pub const IDT77105_GETSTAT: u32 = 1074815282;
```

#### Constant `HIDIOCINITREPORT`

```rust
pub const HIDIOCINITREPORT: u32 = 18437;
```

#### Constant `VFIO_DEVICE_GET_INFO`

```rust
pub const VFIO_DEVICE_GET_INFO: u32 = 15211;
```

#### Constant `RIO_CM_CHAN_RECEIVE`

```rust
pub const RIO_CM_CHAN_RECEIVE: u32 = 3222299402;
```

#### Constant `RNDGETENTCNT`

```rust
pub const RNDGETENTCNT: u32 = 2147766784;
```

#### Constant `PPPIOCNEWUNIT`

```rust
pub const PPPIOCNEWUNIT: u32 = 3221517374;
```

#### Constant `BTRFS_IOC_INO_LOOKUP`

```rust
pub const BTRFS_IOC_INO_LOOKUP: u32 = 3489698834;
```

#### Constant `FDRESET`

```rust
pub const FDRESET: u32 = 596;
```

#### Constant `IOC_PR_REGISTER`

```rust
pub const IOC_PR_REGISTER: u32 = 1075343560;
```

#### Constant `HIDIOCSREPORT`

```rust
pub const HIDIOCSREPORT: u32 = 1074546696;
```

#### Constant `TEE_IOC_OPEN_SESSION`

```rust
pub const TEE_IOC_OPEN_SESSION: u32 = 2148574210;
```

#### Constant `TEE_IOC_SUPPL_RECV`

```rust
pub const TEE_IOC_SUPPL_RECV: u32 = 2148574214;
```

#### Constant `BTRFS_IOC_BALANCE_CTL`

```rust
pub const BTRFS_IOC_BALANCE_CTL: u32 = 1074041889;
```

#### Constant `GPIO_GET_LINEINFO_WATCH_IOCTL`

```rust
pub const GPIO_GET_LINEINFO_WATCH_IOCTL: u32 = 3225990155;
```

#### Constant `HIDIOCGRAWINFO`

```rust
pub const HIDIOCGRAWINFO: u32 = 2148026371;
```

#### Constant `PPPIOCSCOMPRESS`

```rust
pub const PPPIOCSCOMPRESS: u32 = 1074820173;
```

#### Constant `USBDEVFS_CONNECTINFO`

```rust
pub const USBDEVFS_CONNECTINFO: u32 = 1074287889;
```

#### Constant `BLKRESETZONE`

```rust
pub const BLKRESETZONE: u32 = 1074795139;
```

#### Constant `CHIOINITELEM`

```rust
pub const CHIOINITELEM: u32 = 25361;
```

#### Constant `NILFS_IOCTL_SET_ALLOC_RANGE`

```rust
pub const NILFS_IOCTL_SET_ALLOC_RANGE: u32 = 1074818700;
```

#### Constant `AUTOFS_DEV_IOCTL_CATATONIC`

```rust
pub const AUTOFS_DEV_IOCTL_CATATONIC: u32 = 3222836089;
```

#### Constant `RIO_MPORT_MAINT_HDID_SET`

```rust
pub const RIO_MPORT_MAINT_HDID_SET: u32 = 1073900801;
```

#### Constant `PPGETPHASE`

```rust
pub const PPGETPHASE: u32 = 2147774617;
```

#### Constant `USBDEVFS_DISCONNECT_CLAIM`

```rust
pub const USBDEVFS_DISCONNECT_CLAIM: u32 = 2164806939;
```

#### Constant `FDMSGON`

```rust
pub const FDMSGON: u32 = 581;
```

#### Constant `VIDIOC_G_SLICED_VBI_CAP`

```rust
pub const VIDIOC_G_SLICED_VBI_CAP: u32 = 3228849733;
```

#### Constant `BTRFS_IOC_BALANCE_V2`

```rust
pub const BTRFS_IOC_BALANCE_V2: u32 = 3288372256;
```

#### Constant `MEDIA_REQUEST_IOC_REINIT`

```rust
pub const MEDIA_REQUEST_IOC_REINIT: u32 = 31873;
```

#### Constant `IOC_OPAL_ERASE_LR`

```rust
pub const IOC_OPAL_ERASE_LR: u32 = 1091596518;
```

#### Constant `FDFMTBEG`

```rust
pub const FDFMTBEG: u32 = 583;
```

#### Constant `RNDRESEEDCRNG`

```rust
pub const RNDRESEEDCRNG: u32 = 20999;
```

#### Constant `ISST_IF_GET_PHY_ID`

```rust
pub const ISST_IF_GET_PHY_ID: u32 = 3221814785;
```

#### Constant `TUNSETNOCSUM`

```rust
pub const TUNSETNOCSUM: u32 = 1074025672;
```

#### Constant `SONET_GETSTAT`

```rust
pub const SONET_GETSTAT: u32 = 2149867792;
```

#### Constant `TFD_IOC_SET_TICKS`

```rust
pub const TFD_IOC_SET_TICKS: u32 = 1074287616;
```

#### Constant `PPDATADIR`

```rust
pub const PPDATADIR: u32 = 1074032784;
```

#### Constant `IOC_OPAL_ENABLE_DISABLE_MBR`

```rust
pub const IOC_OPAL_ENABLE_DISABLE_MBR: u32 = 1091596517;
```

#### Constant `GPIO_V2_GET_LINE_IOCTL`

```rust
pub const GPIO_V2_GET_LINE_IOCTL: u32 = 3260068871;
```

#### Constant `RIO_CM_CHAN_SEND`

```rust
pub const RIO_CM_CHAN_SEND: u32 = 1074815753;
```

#### Constant `PPWCTLONIRQ`

```rust
pub const PPWCTLONIRQ: u32 = 1073836178;
```

#### Constant `SONYPI_IOCGBRT`

```rust
pub const SONYPI_IOCGBRT: u32 = 2147579392;
```

#### Constant `IOC_PR_RELEASE`

```rust
pub const IOC_PR_RELEASE: u32 = 1074819274;
```

#### Constant `PPCLRIRQ`

```rust
pub const PPCLRIRQ: u32 = 2147774611;
```

#### Constant `IPMICTL_SET_MY_CHANNEL_LUN_CMD`

```rust
pub const IPMICTL_SET_MY_CHANNEL_LUN_CMD: u32 = 2147772698;
```

#### Constant `MGSL_IOCSXSYNC`

```rust
pub const MGSL_IOCSXSYNC: u32 = 27923;
```

#### Constant `HPET_IE_OFF`

```rust
pub const HPET_IE_OFF: u32 = 26626;
```

#### Constant `IOC_OPAL_ACTIVATE_USR`

```rust
pub const IOC_OPAL_ACTIVATE_USR: u32 = 1091596513;
```

#### Constant `SONET_SETFRAMING`

```rust
pub const SONET_SETFRAMING: u32 = 1074028821;
```

#### Constant `PERF_EVENT_IOC_PAUSE_OUTPUT`

```rust
pub const PERF_EVENT_IOC_PAUSE_OUTPUT: u32 = 1074013193;
```

#### Constant `BTRFS_IOC_LOGICAL_INO_V2`

```rust
pub const BTRFS_IOC_LOGICAL_INO_V2: u32 = 3224933435;
```

#### Constant `VBG_IOCTL_HGCM_CONNECT`

```rust
pub const VBG_IOCTL_HGCM_CONNECT: u32 = 3231471108;
```

#### Constant `BLKFINISHZONE`

```rust
pub const BLKFINISHZONE: u32 = 1074795144;
```

#### Constant `EVIOCREVOKE`

```rust
pub const EVIOCREVOKE: u32 = 1074021777;
```

#### Constant `VFIO_DEVICE_FEATURE`

```rust
pub const VFIO_DEVICE_FEATURE: u32 = 15221;
```

#### Constant `CCISS_GETPCIINFO`

```rust
pub const CCISS_GETPCIINFO: u32 = 2148024833;
```

#### Constant `ISST_IF_MBOX_COMMAND`

```rust
pub const ISST_IF_MBOX_COMMAND: u32 = 3221814787;
```

#### Constant `SCIF_ACCEPTREQ`

```rust
pub const SCIF_ACCEPTREQ: u32 = 3222303492;
```

#### Constant `PERF_EVENT_IOC_QUERY_BPF`

```rust
pub const PERF_EVENT_IOC_QUERY_BPF: u32 = 3221758986;
```

#### Constant `VIDIOC_STREAMOFF`

```rust
pub const VIDIOC_STREAMOFF: u32 = 1074026003;
```

#### Constant `VDUSE_DESTROY_DEV`

```rust
pub const VDUSE_DESTROY_DEV: u32 = 1090552067;
```

#### Constant `FDGETFDCSTAT`

```rust
pub const FDGETFDCSTAT: u32 = 2150105621;
```

#### Constant `VIDIOC_S_PRIORITY`

```rust
pub const VIDIOC_S_PRIORITY: u32 = 1074026052;
```

#### Constant `SNAPSHOT_FREEZE`

```rust
pub const SNAPSHOT_FREEZE: u32 = 13057;
```

#### Constant `VIDIOC_ENUMINPUT`

```rust
pub const VIDIOC_ENUMINPUT: u32 = 3226490394;
```

#### Constant `ZATM_GETPOOLZ`

```rust
pub const ZATM_GETPOOLZ: u32 = 1074815330;
```

#### Constant `RIO_DISABLE_DOORBELL_RANGE`

```rust
pub const RIO_DISABLE_DOORBELL_RANGE: u32 = 1074294026;
```

#### Constant `GPIO_V2_GET_LINEINFO_WATCH_IOCTL`

```rust
pub const GPIO_V2_GET_LINEINFO_WATCH_IOCTL: u32 = 3238048774;
```

#### Constant `VIDIOC_G_STD`

```rust
pub const VIDIOC_G_STD: u32 = 2148029975;
```

#### Constant `USBDEVFS_ALLOW_SUSPEND`

```rust
pub const USBDEVFS_ALLOW_SUSPEND: u32 = 21794;
```

#### Constant `SONET_GETSTATZ`

```rust
pub const SONET_GETSTATZ: u32 = 2149867793;
```

#### Constant `SCIF_ACCEPTREG`

```rust
pub const SCIF_ACCEPTREG: u32 = 3221779205;
```

#### Constant `VIDIOC_ENCODER_CMD`

```rust
pub const VIDIOC_ENCODER_CMD: u32 = 3223869005;
```

#### Constant `PPPIOCSRASYNCMAP`

```rust
pub const PPPIOCSRASYNCMAP: u32 = 1074033748;
```

#### Constant `IOCTL_MEI_NOTIFY_SET`

```rust
pub const IOCTL_MEI_NOTIFY_SET: u32 = 1074022402;
```

#### Constant `BTRFS_IOC_QUOTA_RESCAN_STATUS`

```rust
pub const BTRFS_IOC_QUOTA_RESCAN_STATUS: u32 = 2151715885;
```

#### Constant `F2FS_IOC_GARBAGE_COLLECT`

```rust
pub const F2FS_IOC_GARBAGE_COLLECT: u32 = 1074066694;
```

#### Constant `ATMLEC_CTRL`

```rust
pub const ATMLEC_CTRL: u32 = 25040;
```

#### Constant `MATROXFB_GET_AVAILABLE_OUTPUTS`

```rust
pub const MATROXFB_GET_AVAILABLE_OUTPUTS: u32 = 2148036345;
```

#### Constant `DM_DEV_CREATE`

```rust
pub const DM_DEV_CREATE: u32 = 3241737475;
```

#### Constant `VHOST_VDPA_GET_VRING_NUM`

```rust
pub const VHOST_VDPA_GET_VRING_NUM: u32 = 2147659638;
```

#### Constant `VIDIOC_G_CTRL`

```rust
pub const VIDIOC_G_CTRL: u32 = 3221771803;
```

#### Constant `NBD_CLEAR_SOCK`

```rust
pub const NBD_CLEAR_SOCK: u32 = 43780;
```

#### Constant `VFIO_DEVICE_QUERY_GFX_PLANE`

```rust
pub const VFIO_DEVICE_QUERY_GFX_PLANE: u32 = 15218;
```

#### Constant `WDIOC_KEEPALIVE`

```rust
pub const WDIOC_KEEPALIVE: u32 = 2147768069;
```

#### Constant `NVME_IOCTL_SUBSYS_RESET`

```rust
pub const NVME_IOCTL_SUBSYS_RESET: u32 = 20037;
```

#### Constant `PTP_EXTTS_REQUEST2`

```rust
pub const PTP_EXTTS_REQUEST2: u32 = 1074806027;
```

#### Constant `PCITEST_BAR`

```rust
pub const PCITEST_BAR: u32 = 20481;
```

#### Constant `MGSL_IOCGGPIO`

```rust
pub const MGSL_IOCGGPIO: u32 = 2148560145;
```

#### Constant `EVIOCSREP`

```rust
pub const EVIOCSREP: u32 = 1074283779;
```

#### Constant `VFIO_DEVICE_GET_IRQ_INFO`

```rust
pub const VFIO_DEVICE_GET_IRQ_INFO: u32 = 15213;
```

#### Constant `HPET_DPI`

```rust
pub const HPET_DPI: u32 = 26629;
```

#### Constant `VDUSE_VQ_SETUP_KICKFD`

```rust
pub const VDUSE_VQ_SETUP_KICKFD: u32 = 1074299158;
```

#### Constant `ND_IOCTL_CALL`

```rust
pub const ND_IOCTL_CALL: u32 = 3225439754;
```

#### Constant `HIDIOCGDEVINFO`

```rust
pub const HIDIOCGDEVINFO: u32 = 2149337091;
```

#### Constant `DM_TABLE_DEPS`

```rust
pub const DM_TABLE_DEPS: u32 = 3241737483;
```

#### Constant `BTRFS_IOC_DEV_INFO`

```rust
pub const BTRFS_IOC_DEV_INFO: u32 = 3489698846;
```

#### Constant `VDUSE_IOTLB_GET_FD`

```rust
pub const VDUSE_IOTLB_GET_FD: u32 = 3223355664;
```

#### Constant `FW_CDEV_IOC_GET_INFO`

```rust
pub const FW_CDEV_IOC_GET_INFO: u32 = 3223855872;
```

#### Constant `VIDIOC_G_PRIORITY`

```rust
pub const VIDIOC_G_PRIORITY: u32 = 2147767875;
```

#### Constant `ATM_NEWBACKENDIF`

```rust
pub const ATM_NEWBACKENDIF: u32 = 1073897971;
```

#### Constant `VIDIOC_S_EXT_CTRLS`

```rust
pub const VIDIOC_S_EXT_CTRLS: u32 = 3223344712;
```

#### Constant `VIDIOC_SUBDEV_ENUM_DV_TIMINGS`

```rust
pub const VIDIOC_SUBDEV_ENUM_DV_TIMINGS: u32 = 3230946914;
```

#### Constant `VIDIOC_OMAP3ISP_CCDC_CFG`

```rust
pub const VIDIOC_OMAP3ISP_CCDC_CFG: u32 = 3224917697;
```

#### Constant `VIDIOC_S_HW_FREQ_SEEK`

```rust
pub const VIDIOC_S_HW_FREQ_SEEK: u32 = 1076909650;
```

#### Constant `DM_TABLE_LOAD`

```rust
pub const DM_TABLE_LOAD: u32 = 3241737481;
```

#### Constant `F2FS_IOC_START_ATOMIC_WRITE`

```rust
pub const F2FS_IOC_START_ATOMIC_WRITE: u32 = 62721;
```

#### Constant `VIDIOC_G_OUTPUT`

```rust
pub const VIDIOC_G_OUTPUT: u32 = 2147767854;
```

#### Constant `ATM_DROPPARTY`

```rust
pub const ATM_DROPPARTY: u32 = 1074029045;
```

#### Constant `CHIOGELEM`

```rust
pub const CHIOGELEM: u32 = 1080845072;
```

#### Constant `BTRFS_IOC_GET_SUPPORTED_FEATURES`

```rust
pub const BTRFS_IOC_GET_SUPPORTED_FEATURES: u32 = 2152240185;
```

#### Constant `EVIOCSKEYCODE`

```rust
pub const EVIOCSKEYCODE: u32 = 1074283780;
```

#### Constant `NE_GET_IMAGE_LOAD_INFO`

```rust
pub const NE_GET_IMAGE_LOAD_INFO: u32 = 3222318626;
```

#### Constant `TUNSETLINK`

```rust
pub const TUNSETLINK: u32 = 1074025677;
```

#### Constant `FW_CDEV_IOC_ADD_DESCRIPTOR`

```rust
pub const FW_CDEV_IOC_ADD_DESCRIPTOR: u32 = 3222807302;
```

#### Constant `BTRFS_IOC_SCRUB_CANCEL`

```rust
pub const BTRFS_IOC_SCRUB_CANCEL: u32 = 37916;
```

#### Constant `PPS_SETPARAMS`

```rust
pub const PPS_SETPARAMS: u32 = 1074294946;
```

#### Constant `IOC_OPAL_LR_SETUP`

```rust
pub const IOC_OPAL_LR_SETUP: u32 = 1093169379;
```

#### Constant `FW_CDEV_IOC_DEALLOCATE`

```rust
pub const FW_CDEV_IOC_DEALLOCATE: u32 = 1074012931;
```

#### Constant `WDIOC_SETTIMEOUT`

```rust
pub const WDIOC_SETTIMEOUT: u32 = 3221509894;
```

#### Constant `IOC_WATCH_QUEUE_SET_FILTER`

```rust
pub const IOC_WATCH_QUEUE_SET_FILTER: u32 = 22369;
```

#### Constant `CAPI_GET_MANUFACTURER`

```rust
pub const CAPI_GET_MANUFACTURER: u32 = 3221504774;
```

#### Constant `VFIO_IOMMU_SPAPR_UNREGISTER_MEMORY`

```rust
pub const VFIO_IOMMU_SPAPR_UNREGISTER_MEMORY: u32 = 15222;
```

#### Constant `ASPEED_P2A_CTRL_IOCTL_SET_WINDOW`

```rust
pub const ASPEED_P2A_CTRL_IOCTL_SET_WINDOW: u32 = 1074836224;
```

#### Constant `VIDIOC_G_EDID`

```rust
pub const VIDIOC_G_EDID: u32 = 3223868968;
```

#### Constant `F2FS_IOC_GARBAGE_COLLECT_RANGE`

```rust
pub const F2FS_IOC_GARBAGE_COLLECT_RANGE: u32 = 1075377419;
```

#### Constant `RIO_MAP_INBOUND`

```rust
pub const RIO_MAP_INBOUND: u32 = 3223874833;
```

#### Constant `IOC_OPAL_TAKE_OWNERSHIP`

```rust
pub const IOC_OPAL_TAKE_OWNERSHIP: u32 = 1091072222;
```

#### Constant `USBDEVFS_CLAIM_PORT`

```rust
pub const USBDEVFS_CLAIM_PORT: u32 = 2147767576;
```

#### Constant `VIDIOC_S_AUDIO`

```rust
pub const VIDIOC_S_AUDIO: u32 = 1077171746;
```

#### Constant `FS_IOC_GET_ENCRYPTION_NONCE`

```rust
pub const FS_IOC_GET_ENCRYPTION_NONCE: u32 = 2148558363;
```

#### Constant `FW_CDEV_IOC_SEND_STREAM_PACKET`

```rust
pub const FW_CDEV_IOC_SEND_STREAM_PACKET: u32 = 1076372243;
```

#### Constant `BTRFS_IOC_SNAP_DESTROY`

```rust
pub const BTRFS_IOC_SNAP_DESTROY: u32 = 1342215183;
```

#### Constant `SNAPSHOT_FREE`

```rust
pub const SNAPSHOT_FREE: u32 = 13061;
```

#### Constant `I8K_GET_SPEED`

```rust
pub const I8K_GET_SPEED: u32 = 3221776773;
```

#### Constant `HIDIOCGREPORT`

```rust
pub const HIDIOCGREPORT: u32 = 1074546695;
```

#### Constant `HPET_EPI`

```rust
pub const HPET_EPI: u32 = 26628;
```

#### Constant `JSIOCSCORR`

```rust
pub const JSIOCSCORR: u32 = 1076128289;
```

#### Constant `IOC_PR_PREEMPT_ABORT`

```rust
pub const IOC_PR_PREEMPT_ABORT: u32 = 1075343564;
```

#### Constant `RIO_MAP_OUTBOUND`

```rust
pub const RIO_MAP_OUTBOUND: u32 = 3223874831;
```

#### Constant `ATM_SETESI`

```rust
pub const ATM_SETESI: u32 = 1074815372;
```

#### Constant `FW_CDEV_IOC_START_ISO`

```rust
pub const FW_CDEV_IOC_START_ISO: u32 = 1074799370;
```

#### Constant `ATM_DELADDR`

```rust
pub const ATM_DELADDR: u32 = 1074815369;
```

#### Constant `PPFCONTROL`

```rust
pub const PPFCONTROL: u32 = 1073901710;
```

#### Constant `SONYPI_IOCGFAN`

```rust
pub const SONYPI_IOCGFAN: u32 = 2147579402;
```

#### Constant `RTC_IRQP_SET`

```rust
pub const RTC_IRQP_SET: u32 = 1074294796;
```

#### Constant `PCITEST_WRITE`

```rust
pub const PCITEST_WRITE: u32 = 1074286596;
```

#### Constant `PPCLAIM`

```rust
pub const PPCLAIM: u32 = 28811;
```

#### Constant `VIDIOC_S_JPEGCOMP`

```rust
pub const VIDIOC_S_JPEGCOMP: u32 = 1082938942;
```

#### Constant `IPMICTL_UNREGISTER_FOR_CMD`

```rust
pub const IPMICTL_UNREGISTER_FOR_CMD: u32 = 2147641615;
```

#### Constant `VHOST_SET_FEATURES`

```rust
pub const VHOST_SET_FEATURES: u32 = 1074310912;
```

#### Constant `TOSHIBA_ACPI_SCI`

```rust
pub const TOSHIBA_ACPI_SCI: u32 = 3222828177;
```

#### Constant `VIDIOC_DQBUF`

```rust
pub const VIDIOC_DQBUF: u32 = 3227014673;
```

#### Constant `BTRFS_IOC_BALANCE_PROGRESS`

```rust
pub const BTRFS_IOC_BALANCE_PROGRESS: u32 = 2214630434;
```

#### Constant `BTRFS_IOC_SUBVOL_SETFLAGS`

```rust
pub const BTRFS_IOC_SUBVOL_SETFLAGS: u32 = 1074304026;
```

#### Constant `ATMLEC_MCAST`

```rust
pub const ATMLEC_MCAST: u32 = 25042;
```

#### Constant `MMTIMER_GETFREQ`

```rust
pub const MMTIMER_GETFREQ: u32 = 2148035842;
```

#### Constant `VIDIOC_G_SELECTION`

```rust
pub const VIDIOC_G_SELECTION: u32 = 3225441886;
```

#### Constant `RTC_ALM_SET`

```rust
pub const RTC_ALM_SET: u32 = 1076129799;
```

#### Constant `PPPOEIOCSFWD`

```rust
pub const PPPOEIOCSFWD: u32 = 1074311424;
```

#### Constant `IPMICTL_GET_MAINTENANCE_MODE_CMD`

```rust
pub const IPMICTL_GET_MAINTENANCE_MODE_CMD: u32 = 2147772702;
```

#### Constant `FS_IOC_ENABLE_VERITY`

```rust
pub const FS_IOC_ENABLE_VERITY: u32 = 1082156677;
```

#### Constant `NILFS_IOCTL_GET_BDESCS`

```rust
pub const NILFS_IOCTL_GET_BDESCS: u32 = 3222826631;
```

#### Constant `FDFMTEND`

```rust
pub const FDFMTEND: u32 = 585;
```

#### Constant `DMA_BUF_SET_NAME`

```rust
pub const DMA_BUF_SET_NAME: u32 = 1074291201;
```

#### Constant `UI_BEGIN_FF_UPLOAD`

```rust
pub const UI_BEGIN_FF_UPLOAD: u32 = 3228063176;
```

#### Constant `RTC_UIE_ON`

```rust
pub const RTC_UIE_ON: u32 = 28675;
```

#### Constant `PPRELEASE`

```rust
pub const PPRELEASE: u32 = 28812;
```

#### Constant `VFIO_IOMMU_UNMAP_DMA`

```rust
pub const VFIO_IOMMU_UNMAP_DMA: u32 = 15218;
```

#### Constant `VIDIOC_OMAP3ISP_PRV_CFG`

```rust
pub const VIDIOC_OMAP3ISP_PRV_CFG: u32 = 3228587714;
```

#### Constant `GPIO_GET_LINEHANDLE_IOCTL`

```rust
pub const GPIO_GET_LINEHANDLE_IOCTL: u32 = 3245126659;
```

#### Constant `VFAT_IOCTL_READDIR_BOTH`

```rust
pub const VFAT_IOCTL_READDIR_BOTH: u32 = 2184212993;
```

#### Constant `NVME_IOCTL_ADMIN_CMD`

```rust
pub const NVME_IOCTL_ADMIN_CMD: u32 = 3225964097;
```

#### Constant `VHOST_SET_VRING_KICK`

```rust
pub const VHOST_SET_VRING_KICK: u32 = 1074310944;
```

#### Constant `BTRFS_IOC_SUBVOL_CREATE_V2`

```rust
pub const BTRFS_IOC_SUBVOL_CREATE_V2: u32 = 1342215192;
```

#### Constant `BTRFS_IOC_SNAP_CREATE`

```rust
pub const BTRFS_IOC_SNAP_CREATE: u32 = 1342215169;
```

#### Constant `SONYPI_IOCGBAT2CAP`

```rust
pub const SONYPI_IOCGBAT2CAP: u32 = 2147644932;
```

#### Constant `PPNEGOT`

```rust
pub const PPNEGOT: u32 = 1074032785;
```

#### Constant `NBD_PRINT_DEBUG`

```rust
pub const NBD_PRINT_DEBUG: u32 = 43782;
```

#### Constant `BTRFS_IOC_INO_LOOKUP_USER`

```rust
pub const BTRFS_IOC_INO_LOOKUP_USER: u32 = 3489698878;
```

#### Constant `BTRFS_IOC_GET_SUBVOL_ROOTREF`

```rust
pub const BTRFS_IOC_GET_SUBVOL_ROOTREF: u32 = 3489698877;
```

#### Constant `FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS`

```rust
pub const FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS: u32 = 3225445913;
```

#### Constant `BTRFS_IOC_FS_INFO`

```rust
pub const BTRFS_IOC_FS_INFO: u32 = 2214630431;
```

#### Constant `VIDIOC_ENUM_FMT`

```rust
pub const VIDIOC_ENUM_FMT: u32 = 3225441794;
```

#### Constant `VIDIOC_G_INPUT`

```rust
pub const VIDIOC_G_INPUT: u32 = 2147767846;
```

#### Constant `VTPM_PROXY_IOC_NEW_DEV`

```rust
pub const VTPM_PROXY_IOC_NEW_DEV: u32 = 3222577408;
```

#### Constant `DFL_FPGA_FME_ERR_GET_IRQ_NUM`

```rust
pub const DFL_FPGA_FME_ERR_GET_IRQ_NUM: u32 = 2147792515;
```

#### Constant `ND_IOCTL_DIMM_FLAGS`

```rust
pub const ND_IOCTL_DIMM_FLAGS: u32 = 3221769731;
```

#### Constant `BTRFS_IOC_QUOTA_RESCAN`

```rust
pub const BTRFS_IOC_QUOTA_RESCAN: u32 = 1077974060;
```

#### Constant `MMTIMER_GETCOUNTER`

```rust
pub const MMTIMER_GETCOUNTER: u32 = 2148035849;
```

#### Constant `MATROXFB_GET_OUTPUT_MODE`

```rust
pub const MATROXFB_GET_OUTPUT_MODE: u32 = 3221778170;
```

#### Constant `BTRFS_IOC_QUOTA_RESCAN_WAIT`

```rust
pub const BTRFS_IOC_QUOTA_RESCAN_WAIT: u32 = 37934;
```

#### Constant `RIO_CM_CHAN_BIND`

```rust
pub const RIO_CM_CHAN_BIND: u32 = 1074291461;
```

#### Constant `HIDIOCGRDESC`

```rust
pub const HIDIOCGRDESC: u32 = 2416199682;
```

#### Constant `MGSL_IOCGIF`

```rust
pub const MGSL_IOCGIF: u32 = 27915;
```

#### Constant `VIDIOC_S_OUTPUT`

```rust
pub const VIDIOC_S_OUTPUT: u32 = 3221509679;
```

#### Constant `HIDIOCGREPORTINFO`

```rust
pub const HIDIOCGREPORTINFO: u32 = 3222030345;
```

#### Constant `WDIOC_GETBOOTSTATUS`

```rust
pub const WDIOC_GETBOOTSTATUS: u32 = 2147768066;
```

#### Constant `VDUSE_VQ_GET_INFO`

```rust
pub const VDUSE_VQ_GET_INFO: u32 = 3224404245;
```

#### Constant `ACRN_IOCTL_ASSIGN_PCIDEV`

```rust
pub const ACRN_IOCTL_ASSIGN_PCIDEV: u32 = 1076142677;
```

#### Constant `BLKGETDISKSEQ`

```rust
pub const BLKGETDISKSEQ: u32 = 2148012672;
```

#### Constant `ACRN_IOCTL_PM_GET_CPU_STATE`

```rust
pub const ACRN_IOCTL_PM_GET_CPU_STATE: u32 = 3221791328;
```

#### Constant `ACRN_IOCTL_DESTROY_VM`

```rust
pub const ACRN_IOCTL_DESTROY_VM: u32 = 41489;
```

#### Constant `ACRN_IOCTL_SET_PTDEV_INTR`

```rust
pub const ACRN_IOCTL_SET_PTDEV_INTR: u32 = 1075094099;
```

#### Constant `ACRN_IOCTL_CREATE_IOREQ_CLIENT`

```rust
pub const ACRN_IOCTL_CREATE_IOREQ_CLIENT: u32 = 41522;
```

#### Constant `ACRN_IOCTL_IRQFD`

```rust
pub const ACRN_IOCTL_IRQFD: u32 = 1075356273;
```

#### Constant `ACRN_IOCTL_CREATE_VM`

```rust
pub const ACRN_IOCTL_CREATE_VM: u32 = 3224412688;
```

#### Constant `ACRN_IOCTL_INJECT_MSI`

```rust
pub const ACRN_IOCTL_INJECT_MSI: u32 = 1074831907;
```

#### Constant `ACRN_IOCTL_ATTACH_IOREQ_CLIENT`

```rust
pub const ACRN_IOCTL_ATTACH_IOREQ_CLIENT: u32 = 41523;
```

#### Constant `ACRN_IOCTL_RESET_PTDEV_INTR`

```rust
pub const ACRN_IOCTL_RESET_PTDEV_INTR: u32 = 1075094100;
```

#### Constant `ACRN_IOCTL_NOTIFY_REQUEST_FINISH`

```rust
pub const ACRN_IOCTL_NOTIFY_REQUEST_FINISH: u32 = 1074307633;
```

#### Constant `ACRN_IOCTL_SET_IRQLINE`

```rust
pub const ACRN_IOCTL_SET_IRQLINE: u32 = 1074307621;
```

#### Constant `ACRN_IOCTL_START_VM`

```rust
pub const ACRN_IOCTL_START_VM: u32 = 41490;
```

#### Constant `ACRN_IOCTL_SET_VCPU_REGS`

```rust
pub const ACRN_IOCTL_SET_VCPU_REGS: u32 = 1093181974;
```

#### Constant `ACRN_IOCTL_SET_MEMSEG`

```rust
pub const ACRN_IOCTL_SET_MEMSEG: u32 = 1075880513;
```

#### Constant `ACRN_IOCTL_PAUSE_VM`

```rust
pub const ACRN_IOCTL_PAUSE_VM: u32 = 41491;
```

#### Constant `ACRN_IOCTL_CLEAR_VM_IOREQ`

```rust
pub const ACRN_IOCTL_CLEAR_VM_IOREQ: u32 = 41525;
```

#### Constant `ACRN_IOCTL_UNSET_MEMSEG`

```rust
pub const ACRN_IOCTL_UNSET_MEMSEG: u32 = 1075880514;
```

#### Constant `ACRN_IOCTL_IOEVENTFD`

```rust
pub const ACRN_IOCTL_IOEVENTFD: u32 = 1075880560;
```

#### Constant `ACRN_IOCTL_DEASSIGN_PCIDEV`

```rust
pub const ACRN_IOCTL_DEASSIGN_PCIDEV: u32 = 1076142678;
```

#### Constant `ACRN_IOCTL_RESET_VM`

```rust
pub const ACRN_IOCTL_RESET_VM: u32 = 41493;
```

#### Constant `ACRN_IOCTL_DESTROY_IOREQ_CLIENT`

```rust
pub const ACRN_IOCTL_DESTROY_IOREQ_CLIENT: u32 = 41524;
```

#### Constant `ACRN_IOCTL_VM_INTR_MONITOR`

```rust
pub const ACRN_IOCTL_VM_INTR_MONITOR: u32 = 1074307620;
```

