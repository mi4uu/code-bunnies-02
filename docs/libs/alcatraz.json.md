# Crate Documentation

**Version:** 0.1.0

**Format Version:** 43

# Module `alcatraz`

<span align="center">

<h1><pre><code>cli-sandbox</code></pre></h1>

<a href="https://crates.io/crates/cli-sandbox"><img src="https://img.shields.io/crates/d/cli-sandbox?style=for-the-badge&logo=rust"></img></a>
<a href="https://docs.rs/cli-sandbox"><img src="https://img.shields.io/docsrs/cli-sandbox?style=for-the-badge&logo=docsdotrs"></img></a>

</span>

`cli-sandbox` is a sandboxing environment and testing utility to help you test and debug your CLI applications, inspired by [Cargo's `cargo-test-support`](https://github.com/rust-lang/cargo/tree/master/crates/cargo-test-support).

All tests get their own temporary directories, where you can create files, check files, test your program against those files and check the output of your program in various ways.

For example, if you want to check that your Python to Rust transpiler works correctly:

```rust
use alcatraz::{project, WithStdout};
use std::error::Error;

#[test]
fn compiling() -> Result<(), Box<dyn Error>> {
    alcatraz::init(); // Initialize the sandbox
    let proj = project()?;                      // Create a project

    // Let's create a file, and put in there some Python.
    proj.new_file("my-program.py",
r#"def main():
    print("Hi! this is a test")

main()"#)?;

    let cmd = proj.command(["build"])?;         // Execute the command "<YOUR COMMAND> build". Cli-sandbox will automatically get pickup your command.

    // Now, let's check that the transpiler created the file correctly.
    proj.check_file("my-program.rs",
r#"fn main() {
    println!("Hi! this is a test");
}

main()"#)?;

    // And that the command stdout and stderr are correct.

    cmd.with_stdout("File transpiled correctly! (`my-program.py` -> `my-program.rs`)");

    // If the stderr isn't empty, we'll panic.
    if !cmd.empty_stderr() {
        panic!("Something went wrong! stderr isn't empty");
    };
}
```

You can also get the path of a project (it changes each time the tests are executed, they're temporary).

## Installation

```sh
cargo add cli-sandbox --dev
```

## Usage

The first step is to create a `Project`. You can use either `Project::new()` or `project()`. This will create a temporary directory for you to put all your testing files in there.

From a project, you can execute commands, do I/O operations or even operate over it manually by getting the project's path (`Project::path()`).

Check the [project's documentation](https://docs.rs/cli-sandbox) for more info.

## Features

* Regex support for checking `stdout` and `stderr`. (feature: `regex`)
* All output is beautiful thanks to [`pretty-assertions`](https://docs.rs/pretty_assertions/latest/pretty_assertions/) and [`better_panic`](https://docs.rs/better_panic). (feature: `pretty`, also can be enabled individually)
* Little fuzzing functionality (feature: `fuzz`)
* Testing either the `debug` or `release` profile (features: `dev` or `release`)


## Modules

## Module `panic`

**Attributes:**

- `#[<cfg>(feature = "better_panic")]`

```rust
pub mod panic { /* ... */ }
```

### Functions

#### Function `minimal`

**Attributes:**

- `#[inline]`

Shortcut to `better_panic::Settings::new().verbosity(better_panic::Verbosity::Minimal).install()`;

Meant to be used at the start of your tests.

```rust
pub fn minimal() { /* ... */ }
```

#### Function `medium`

**Attributes:**

- `#[inline]`

Shortcut to `better_panic::Settings::new().verbosity(better_panic::Verbosity::Medium).install()`;

Meant to be used at the start of your tests.

```rust
pub fn medium() { /* ... */ }
```

#### Function `full`

**Attributes:**

- `#[inline]`

Shortcut to `better_panic::Settings::new().verbosity(better_panic::Verbosity::Full).install()`;

Meant to be used at the start of your tests.

```rust
pub fn full() { /* ... */ }
```

## Types

### Struct `Project`

```rust
pub struct Project {
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
  pub fn new() -> Result<Self> { /* ... */ }
  ```
  Creates a new [`Project`]

- ```rust
  pub fn path(self: &Self) -> &Path { /* ... */ }
  ```
  Gets the [`std::path::Path`] for the [`Project`]'s temporary directory.

- ```rust
  pub fn new_file<P: AsRef<Path>>(self: &mut Self, path: P, contents: &str) -> Result<()> { /* ... */ }
  ```
  Creates a new file with a relative path to the project's directory.

- ```rust
  pub fn check_file<P: AsRef<Path>>(self: &Self, path: P, contents: &str) -> Result<()> { /* ... */ }
  ```
  Checks that the contents of a file are correct. It will panic if they aren't, and show the differences if the feature **`pretty_assertions`** is enabled

- ```rust
  pub fn command<I, S>(self: &Self, args: I) -> anyhow::Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr> { /* ... */ }
  ```
  Executes a command relative to the project's directory

- ```rust
  pub fn is_bin<P: AsRef<Path>>(self: &Self, path: P) -> bool { /* ... */ }
  ```
  Checks the [file signature](https://en.m.wikipedia.org/wiki/File_format#Magic_number) of a file and returns `true` if the file in that path is an executable.

- ```rust
  pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(self: &Self, src: P, dst: Q) { /* ... */ }
  ```
  Creates a [symbolic link](wikipedia.org/wiki/Symlinks), both paths are relative to the temporary project's path.

- ```rust
  pub fn clean_env(self: &Self, prefix: &str) { /* ... */ }
  ```
  Cleans your environment used in the working directory (i.e. removing all environment variables that start with a prefix).

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Send**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Paint**
  - ```rust
    fn fg(self: &Self, value: Color) -> Painted<&T> { /* ... */ }
    ```
    Returns a styled value derived from `self` with the foreground set to

  - ```rust
    fn primary(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn fixed(self: &Self, color: u8) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn rgb(self: &Self, r: u8, g: u8, b: u8) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn black(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn red(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn green(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn yellow(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn blue(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn magenta(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn cyan(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn white(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_black(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_red(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_green(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_yellow(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_blue(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_magenta(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_cyan(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright_white(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bg(self: &Self, value: Color) -> Painted<&T> { /* ... */ }
    ```
    Returns a styled value derived from `self` with the background set to

  - ```rust
    fn on_primary(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_fixed(self: &Self, color: u8) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_rgb(self: &Self, r: u8, g: u8, b: u8) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_black(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_red(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_green(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_yellow(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_blue(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_magenta(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_cyan(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_white(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_black(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_red(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_green(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_yellow(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_blue(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_magenta(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_cyan(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright_white(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn attr(self: &Self, value: Attribute) -> Painted<&T> { /* ... */ }
    ```
    Enables the styling [`Attribute`] `value`.

  - ```rust
    fn bold(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn dim(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn italic(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn underline(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn blink(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn rapid_blink(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn invert(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn conceal(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn strike(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn quirk(self: &Self, value: Quirk) -> Painted<&T> { /* ... */ }
    ```
    Enables the `yansi` [`Quirk`] `value`.

  - ```rust
    fn mask(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn wrap(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn linger(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn clear(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn resetting(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn bright(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn on_bright(self: &Self) -> Painted<&T> { /* ... */ }
    ```
    Returns `self` with the

  - ```rust
    fn whenever(self: &Self, value: Condition) -> Painted<&T> { /* ... */ }
    ```
    Conditionally enable styling based on whether the [`Condition`] `value`

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Traits

### Trait `WithStdout`

```rust
pub trait WithStdout {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `with_stdout`: Checks that the standard output of a command is what's expected. If they aren't the same, it will show the differences if the `pretty_asssertions` feature is enabled
- `with_stderr`: Checks that the standard error of a command is what's expected. If they aren't the same, it will show the differences if the `pretty_asssertions` feature is enabled
- `stdout_warns`: Returns how many times the program contains the word "warning:" in the `stderr`. Useful for checking compile-time warnings.
- `stderr_warns`: Returns how many times the program contains the word "warning:" in the `stderr`. Useful for checking compile-time warnings.
- `empty_stderr`: Checks that the stderr is empty. It's different from `.with_stderr("")` in that this won't print a whole diff. Useful for when ANY presence of a stderr would mean that there were errors, and the output is invalid.
- `empty_stdout`: Checks that the stdout is empty. It's different from `.with_stdout("")` in that this won't print a whole diff. Useful for when ANY presence of a stdout, would mean that there were errors, and the output is invalid.
- `with_stdout_file`: Checks that the stdout is corresponding with a file (usually "<my-test>.stdout");
- `with_stderr_file`: Checks that the stderr is corresponding with a file (usually "<my-test>.stderr");

#### Implementations

This trait is implemented for the following types:

- `std::process::Output`

## Functions

### Function `project`

**Attributes:**

- `#[inline(always)]`

Shortcut for [`Project::new()`].

```rust
pub fn project() -> anyhow::Result<Project> { /* ... */ }
```

### Function `init`

Initializes a new sandbox testing environment. Note that **this doesn't initialize a project**, just creates some
environment variables with metadata about your project.

# Panics

This function may panic if it cannot find the root package metadata (a.k.a your project's metadata).

```rust
pub fn init() { /* ... */ }
```

## Constants and Statics

### Constant `MANIFEST_DIR`

```rust
pub const MANIFEST_DIR: &str = "/app/crates/alcatraz";
```

## Re-exports

### Re-export `better_panic`

**Attributes:**

- `#[<cfg>(feature = "better_panic")]`

```rust
pub use better_panic;
```

