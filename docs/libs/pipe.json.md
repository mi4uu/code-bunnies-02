# Crate Documentation

**Version:** 0.4.0

**Format Version:** 43

# Module `pipe`

Synchronous in-memory pipe

## Example

```
use std::thread::spawn;
use std::io::{Read, Write};

let (mut read, mut write) = pipe::pipe();

let message = "Hello, world!";
spawn(move || write.write_all(message.as_bytes()).unwrap());

let mut s = String::new();
read.read_to_string(&mut s).unwrap();

assert_eq!(&s, message);
```

## Types

### Struct `PipeReader`

The `Read` end of a pipe (see `pipe()`)

```rust
pub struct PipeReader {
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
  pub fn into_inner(self: Self) -> (Receiver<Vec<u8>>, Vec<u8>) { /* ... */ }
  ```
  Extracts the inner `Receiver` from the writer, and any pending buffered data

- ```rust
  pub fn buffer(self: &Self) -> &[u8] { /* ... */ }
  ```
  Returns a reference to the internally buffered data.

##### Trait Implementations

- **BufRead**
  - ```rust
    fn fill_buf(self: &mut Self) -> io::Result<&[u8]> { /* ... */ }
    ```

  - ```rust
    fn consume(self: &mut Self, amt: usize) { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Read**
  - ```rust
    fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Unpin**
### Struct `PipeWriter`

The `Write` end of a pipe (see `pipe()`)

```rust
pub struct PipeWriter {
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
  pub fn into_inner(self: Self) -> Sender<Vec<u8>> { /* ... */ }
  ```
  Extracts the inner `Sender` from the writer

- ```rust
  pub fn sender(self: &Self) -> &Sender<Vec<u8>> { /* ... */ }
  ```
  Gets a reference to the underlying `Sender`

- ```rust
  pub fn send<B: Into<Vec<u8>>>(self: &Self, bytes: B) -> io::Result<()> { /* ... */ }
  ```
  Write data to the associated `PipeReader`

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PipeWriter { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **RefUnwindSafe**
- **Send**
### Struct `PipeBufWriter`

The `Write` end of a pipe (see `pipe()`) that will buffer small writes before sending
to the reader end.

```rust
pub struct PipeBufWriter {
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
  pub fn into_inner(self: Self) -> (Sender<Vec<u8>>, Vec<u8>) { /* ... */ }
  ```
  Extracts the inner `Sender` from the writer, and any pending buffered data

- ```rust
  pub fn sender(self: &Self) -> &Sender<Vec<u8>> { /* ... */ }
  ```
  Gets a reference to the underlying `Sender`

- ```rust
  pub fn buffer(self: &Self) -> &[u8] { /* ... */ }
  ```
  Returns a reference to the internally buffered data.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of bytes the internal buffer can hold without flushing.

##### Trait Implementations

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

## Functions

### Function `pipe`

Creates a synchronous memory pipe

```rust
pub fn pipe() -> (PipeReader, PipeWriter) { /* ... */ }
```

### Function `pipe_buffered`

Creates a synchronous memory pipe with buffered writer

```rust
pub fn pipe_buffered() -> (PipeReader, PipeBufWriter) { /* ... */ }
```

