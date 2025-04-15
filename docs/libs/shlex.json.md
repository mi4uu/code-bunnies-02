# Crate Documentation

**Version:** 1.3.0

**Format Version:** 43

# Module `shlex`

Parse strings like, and escape strings for, POSIX shells.

Same idea as (but implementation not directly based on) the Python shlex module.

Disabling the `std` feature (which is enabled by default) will allow the crate to work in
`no_std` environments, where the `alloc` crate, and a global allocator, are available.

## <span style="color:red">Warning</span>

The [`try_quote`]/[`try_join`] family of APIs does not quote control characters (because they
cannot be quoted portably).

This is fully safe in noninteractive contexts, like shell scripts and `sh -c` arguments (or
even scripts `source`d from interactive shells).

But if you are quoting for human consumption, you should keep in mind that ugly inputs produce
ugly outputs (which may not be copy-pastable).

And if by chance you are piping the output of [`try_quote`]/[`try_join`] directly to the stdin
of an interactive shell, you should stop, because control characters can lead to arbitrary
command injection.

For more information, and for information about more minor issues, please see [quoting_warning].

## Compatibility

This crate's quoting functionality tries to be compatible with **any POSIX-compatible shell**;
it's tested against `bash`, `zsh`, `dash`, Busybox `ash`, and `mksh`, plus `fish` (which is not
POSIX-compatible but close enough).

It also aims to be compatible with Python `shlex` and C `wordexp`.

## Modules

## Module `bytes`

[`Shlex`] and friends for byte strings.

This is used internally by the [outer module](crate), and may be more
convenient if you are working with byte slices (`[u8]`) or types that are
wrappers around bytes, such as [`OsStr`](std::ffi::OsStr):

```rust
#[cfg(unix)] {
    use shlex::bytes::quote;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    // `\x80` is invalid in UTF-8.
    let os_str = OsStr::from_bytes(b"a\x80b c");
    assert_eq!(quote(os_str.as_bytes()), &b"'a\x80b c'"[..]);
}
```

(On Windows, `OsStr` uses 16 bit wide characters so this will not work.)

```rust
pub mod bytes { /* ... */ }
```

### Types

#### Struct `Shlex`

An iterator that takes an input byte string and splits it into the words using the same syntax as
the POSIX shell.

```rust
pub struct Shlex<''a> {
    pub line_no: usize,
    pub had_error: bool,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `line_no` | `usize` | The number of newlines read so far, plus one. |
| `had_error` | `bool` | An input string is erroneous if it ends while inside a quotation or right after an<br>unescaped backslash.  Since Iterator does not have a mechanism to return an error, if that<br>happens, Shlex just throws out the last token, ends the iteration, and sets 'had_error' to<br>true; best to check it after you're done iterating. |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new(in_bytes: &''a [u8]) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **Send**
- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Unpin**
#### Struct `Quoter`

A more configurable interface to quote strings.  If you only want the default settings you can
use the convenience functions [`try_quote`] and [`try_join`].

The string equivalent is [`shlex::Quoter`].

```rust
pub struct Quoter {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new [`Quoter`] with default settings.

- ```rust
  pub fn allow_nul(self: Self, allow: bool) -> Self { /* ... */ }
  ```
  Set whether to allow [nul bytes](quoting_warning#nul-bytes).  By default they are not

- ```rust
  pub fn join<''a, I: IntoIterator<Item = &''a [u8]>>(self: &Self, words: I) -> Result<Vec<u8>, QuoteError> { /* ... */ }
  ```
  Convenience function that consumes an iterable of words and turns it into a single byte string,

- ```rust
  pub fn quote<''a>(self: &Self, in_bytes: &''a [u8]) -> Result<Cow<''a, [u8]>, QuoteError> { /* ... */ }
  ```
  Given a single word, return a byte string suitable to encode it as a shell argument.

###### Trait Implementations

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Quoter { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(inner: bytes::Quoter) -> Quoter { /* ... */ }
    ```

  - ```rust
    fn from(quoter: Quoter) -> bytes::Quoter { /* ... */ }
    ```

- **Send**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Quoter { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Functions

#### Function `split`

Convenience function that consumes the whole byte string at once.  Returns None if the input was
erroneous.

```rust
pub fn split(in_bytes: &[u8]) -> Option<alloc::vec::Vec<alloc::vec::Vec<u8>>> { /* ... */ }
```

#### Function `join`

**⚠️ Deprecated since 1.3.0**: replace with `try_join(words)?` to avoid nul byte danger

Convenience function that consumes an iterable of words and turns it into a single byte string,
quoting words when necessary. Consecutive words will be separated by a single space.

Uses default settings except that nul bytes are passed through, which [may be
dangerous](quoting_warning#nul-bytes), leading to this function being deprecated.

Equivalent to [`Quoter::new().allow_nul(true).join(words).unwrap()`](Quoter).

(That configuration never returns `Err`, so this function does not panic.)

The string equivalent is [shlex::join].

```rust
pub fn join<''a, I: IntoIterator<Item = &''a [u8]>>(words: I) -> alloc::vec::Vec<u8> { /* ... */ }
```

#### Function `try_join`

Convenience function that consumes an iterable of words and turns it into a single byte string,
quoting words when necessary. Consecutive words will be separated by a single space.

Uses default settings.  The only error that can be returned is [`QuoteError::Nul`].

Equivalent to [`Quoter::new().join(words)`](Quoter).

The string equivalent is [shlex::try_join].

```rust
pub fn try_join<''a, I: IntoIterator<Item = &''a [u8]>>(words: I) -> Result<alloc::vec::Vec<u8>, super::QuoteError> { /* ... */ }
```

#### Function `quote`

**⚠️ Deprecated since 1.3.0**: replace with `try_quote(str)?` to avoid nul byte danger

Given a single word, return a string suitable to encode it as a shell argument.

Uses default settings except that nul bytes are passed through, which [may be
dangerous](quoting_warning#nul-bytes), leading to this function being deprecated.

Equivalent to [`Quoter::new().allow_nul(true).quote(in_bytes).unwrap()`](Quoter).

(That configuration never returns `Err`, so this function does not panic.)

The string equivalent is [shlex::quote].

```rust
pub fn quote(in_bytes: &[u8]) -> alloc::borrow::Cow<''_, [u8]> { /* ... */ }
```

#### Function `try_quote`

Given a single word, return a string suitable to encode it as a shell argument.

Uses default settings.  The only error that can be returned is [`QuoteError::Nul`].

Equivalent to [`Quoter::new().quote(in_bytes)`](Quoter).

(That configuration never returns `Err`, so this function does not panic.)

The string equivalent is [shlex::try_quote].

```rust
pub fn try_quote(in_bytes: &[u8]) -> Result<alloc::borrow::Cow<''_, [u8]>, super::QuoteError> { /* ... */ }
```

## Module `quoting_warning`

**Attributes:**

- `#[<cfg>(all(doc, not(doctest)))]`
- `#[path = "quoting_warning.md"]`


A more detailed version of the [warning at the top level](super#warning) about the `quote`/`join`
family of APIs.

In general, passing the output of these APIs to a shell should recover the original string(s).
This page lists cases where it fails to do so.

In noninteractive contexts, there are only minor issues.  'Noninteractive' includes shell scripts
and `sh -c` arguments, or even scripts `source`d from interactive shells.  The issues are:

- [Nul bytes](#nul-bytes)

- [Overlong commands](#overlong-commands)

If you are writing directly to the stdin of an interactive (`-i`) shell (i.e., if you are
pretending to be a terminal), or if you are writing to a cooked-mode pty (even if the other end is
noninteractive), then there is a **severe** security issue:

- [Control characters](#control-characters-interactive-contexts-only)

Finally, there are some [solved issues](#solved-issues).

# List of issues

## Nul bytes

For non-interactive shells, the most problematic input is nul bytes (bytes with value 0).  The
non-deprecated functions all default to returning [`QuoteError::Nul`] when encountering them, but
the deprecated [`quote`] and [`join`] functions leave them as-is.

In Unix, nul bytes can't appear in command arguments, environment variables, or filenames.  It's
not a question of proper quoting; they just can't be used at all.  This is a consequence of Unix's
system calls all being designed around nul-terminated C strings.

Shells inherit that limitation.  Most of them do not accept nul bytes in strings even internally.
Even when they do, it's pretty much useless or even dangerous, since you can't pass them to
external commands.

In some cases, you might fail to pass the nul byte to the shell in the first place.  For example,
the following code uses [`join`] to tunnel a command over an SSH connection:

```rust
std::process::Command::new("ssh")
    .arg("myhost")
    .arg("--")
    .arg(join(my_cmd_args))
```

If any argument in `my_cmd_args` contains a nul byte, then `join(my_cmd_args)` will contain a nul
byte.  But `join(my_cmd_args)` is itself being passed as an argument to a command (the ssh
command), and command arguments can't contain nul bytes!  So this will simply result in the
`Command` failing to launch.

Still, there are other ways to smuggle nul bytes into a shell.  How the shell reacts depends on the
shell and the method of smuggling.  For example, here is Bash 5.2.21 exhibiting three different
behaviors:

- With ANSI-C quoting, the string is truncated at the first nul byte:
  ```bash
  $ echo $'foo\0bar' | hexdump -C
  00000000  66 6f 6f 0a                                       |foo.|
  ```

- With command substitution, nul bytes are removed with a warning:
  ```bash
  $ echo $(printf 'foo\0bar') | hexdump -C
  bash: warning: command substitution: ignored null byte in input
  00000000  66 6f 6f 62 61 72 0a                              |foobar.|
  ```

- When a nul byte appears directly in a shell script, it's removed with no warning:
  ```bash
  $ printf 'echo "foo\0bar"' | bash | hexdump -C
  00000000  66 6f 6f 62 61 72 0a                              |foobar.|
  ```

Zsh, in contrast, actually allows nul bytes internally, in shell variables and even arguments to
builtin commands.  But if a variable is exported to the environment, or if an argument is used for
an external command, then the child process will see it silently truncated at the first nul.  This
might actually be more dangerous, depending on the use case.

## Overlong commands

If you pass a long string into a shell, several things might happen:

- It might succeed, yet the shell might have trouble actually doing anything with it.  For example:

  ```bash
  x=$(printf '%010000000d' 0); /bin/echo $x
  bash: /bin/echo: Argument list too long
  ```

- If you're using certain shells (e.g. Busybox Ash) *and* using a pty for communication, then the
  shell will impose a line length limit, ignoring all input past the limit.

- If you're using a pty in cooked mode, then by default, if you write so many bytes as input that
  it fills the kernel's internal buffer, the kernel will simply drop those bytes, instead of
  blocking waiting for the shell to empty out the buffer.  In other words, random bits of input can
  be lost, which is obviously insecure.

Future versions of this crate may add an option to [`Quoter`] to check the length for you.

## Control characters (*interactive contexts only*)

Control characters are the bytes from `\x00` to `\x1f`, plus `\x7f`.  `\x00` (the nul byte) is
discussed [above](#nul-bytes), but what about the rest?  Well, many of them correspond to terminal
keyboard shortcuts.  For example, when you press Ctrl-A at a shell prompt, your terminal sends the
byte `\x01`.  The shell sees that byte and (if not configured differently) takes the standard
action for Ctrl-A, which is to move the cursor to the beginning of the line.

This means that it's quite dangerous to pipe bytes to an interactive shell.  For example, here is a
program that tries to tell Bash to echo an arbitrary string, 'safely':
```rust
use std::process::{Command, Stdio};
use std::io::Write;

let evil_string = "\x01do_something_evil; ";
let quoted = shlex::try_quote(evil_string).unwrap();
println!("quoted string is {:?}", quoted);

let mut bash = Command::new("bash")
    .arg("-i") // force interactive mode
    .stdin(Stdio::piped())
    .spawn()
    .unwrap();
let stdin = bash.stdin.as_mut().unwrap();
write!(stdin, "echo {}\n", quoted).unwrap();
```

Here's the output of the program (with irrelevant bits removed):

```text
quoted string is "'\u{1}do_something_evil; '"
/tmp comex$ do_something_evil; 'echo '
bash: do_something_evil: command not found
bash: echo : command not found
```

Even though we quoted it, Bash still ran an arbitrary command!

This is not because the quoting was insufficient, per se.  In single quotes, all input is supposed
to be treated as raw data until the closing single quote.  And in fact, this would work fine
without the `"-i"` argument.

But line input is a separate stage from shell syntax parsing.  After all, if you type a single
quote on the keyboard, you wouldn't expect it to disable all your keyboard shortcuts.  So a control
character always has its designated effect, no matter if it's quoted or backslash-escaped.

Also, some control characters are interpreted by the kernel tty layer instead, like CTRL-C to send
SIGINT.  These can be an issue even with noninteractive shells, but only if using a pty for
communication, as opposed to a pipe.

To be safe, you just have to avoid sending them.

### Why not just use hex escapes?

In any normal programming languages, this would be no big deal.

Any normal language has a way to escape arbitrary characters in strings by writing out their
numeric values.  For example, Rust lets you write them in hexadecimal, like `"\x4f"` (or
`"\u{1d546}"` for Unicode).  In this way, arbitrary strings can be represented using only 'nice'
simple characters.  Any remotely suspicious character can be replaced with a numeric escape
sequence, where the escape sequence itself consists only of alphanumeric characters and some
punctuation.  The result may not be the most readable[^choices], but it's quite safe from being
misinterpreted or corrupted in transit.

Shell is not normal.  It has no numeric escape sequences.

There are a few different ways to quote characters (unquoted, unquoted-with-backslash, single
quotes, double quotes), but all of them involve writing the character itself.  If the input
contains a control character, the output must contain that same character.

### Mitigation: terminal filters

In practice, automating interactive shells like in the above example is pretty uncommon these days.
In most cases, the only way for a programmatically generated string to make its way to the input of
an interactive shell is if a human copies and pastes it into their terminal.

And many terminals detect when you paste a string containing control characters.  iTerm2 strips
them out; gnome-terminal replaces them with alternate characters[^gr]; Kitty outright prompts for
confirmation.  This mitigates the risk.

But it's not perfect.  Some other terminals don't implement this check or implement it incorrectly.
Also, these checks tend to not filter the tab character, which could trigger tab completion.  In
most cases that's a non-issue, because most shells support paste bracketing, which disables tab and
some other control characters[^bracketing] within pasted text.  But in some cases paste bracketing
gets disabled.

### Future possibility: ANSI-C quoting

I said that shell syntax has no numeric escapes, but that only applies to *portable* shell syntax.
Bash and Zsh support an obscure alternate quoting style with the syntax `$'foo'`.  It's called
["ANSI-C quoting"][ansic], and inside it you can use all the escape sequences supported by C,
including hex escapes:

```bash
$ echo $'\x41\n\x42'
A
B
```

But other shells don't support it — including Dash, a popular choice for `/bin/sh`, and Busybox's
Ash, frequently seen on stripped-down embedded systems.  This crate's quoting functionality [tries
to be compatible](crate#compatibility) with those shells, plus all other POSIX-compatible shells.
That makes ANSI-C quoting a no-go.

Still, future versions of this crate may provide an option to enable ANSI-C quoting, at the cost of
reduced portability.

### Future possibility: printf

Another option would be to invoke the `printf` command, which is required by POSIX to support octal
escapes.  For example, you could 'escape' the Rust string `"\x01"` into the shell syntax `"$(printf
'\001')"`.  The shell will execute the command `printf` with the first argument being literally a
backslash followed by three digits; `printf` will output the actual byte with value 1; and the
shell will substitute that back into the original command.

The problem is that 'escaping' a string into a command substitution just feels too surprising.  If
nothing else, it only works with an actual shell; [other languages' shell parsing
routines](crate#compatibility) wouldn't understand it.  Neither would this crate's own parser,
though that could be fixed.

Future versions of this crate may provide an option to use `printf` for quoting.

### Special note: newlines

Did you know that `\r` and `\n` are control characters?  They aren't as dangerous as other control
characters (if quoted properly).  But there's still an issue with them in interactive contexts.

Namely, in some cases, interactive shells and/or the tty layer will 'helpfully' translate between
different line ending conventions.  The possibilities include replacing `\r` with `\n`, replacing
`\n` with `\r\n`, and others.  This can't result in command injection, but it's still a lossy
transformation which can result in a failure to round-trip (i.e. the shell sees a different string
from what was originally passed to `quote`).

Numeric escapes would solve this as well.

# Solved issues

## Solved: Past vulnerability (GHSA-r7qv-8r2h-pg27 / RUSTSEC-2024-XXX)

Versions of this crate before 1.3.0 did not quote `{`, `}`, and `\xa0`.

See:
- <https://github.com/advisories/GHSA-r7qv-8r2h-pg27>
- (TODO: Add Rustsec link)

## Solved: `!` and `^`

There are two non-control characters which have a special meaning in interactive contexts only: `!` and
`^`.  Luckily, these can be escaped adequately.

The `!` character triggers [history expansion][he]; the `^` character can trigger a variant of
history expansion known as [Quick Substitution][qs].  Both of these characters get expanded even
inside of double-quoted strings\!

If we're in a double-quoted string, then we can't just escape these characters with a backslash.
Only a specific set of characters can be backslash-escaped inside double quotes; the set of
supported characters depends on the shell, but it often doesn't include `!` and `^`.[^escbs]
Trying to backslash-escape an unsupported character produces a literal backslash:
```bash
$ echo "\!"
\!
```

However, these characters don't get expanded in single-quoted strings, so this crate just
single-quotes them.

But there's a Bash bug where `^` actually does get partially expanded in single-quoted strings:
```bash
$ echo '
> ^a^b
> '

!!:s^a^b
```

To work around that, this crate forces `^` to appear right after an opening single quote.  For
example, the string `"^` is quoted into `'"''^'` instead of `'"^'`.  This restriction is overkill,
since `^` is only meaningful right after a newline, but it's a sufficient restriction (after all, a
`^` character can't be preceded by a newline if it's forced to be preceded by a single quote), and
for now it simplifies things.

## Solved: `\xa0`

The byte `\xa0` may be treated as a shell word separator, specifically on Bash on macOS when using
the default UTF-8 locale, only when the input is invalid UTF-8.  This crate handles the issue by
always using quotes for arguments containing this byte.

In fact, this crate always uses quotes for arguments containing any non-ASCII bytes.  This may be
changed in the future, since it's a bit unfriendly to non-English users.  But for now it
minimizes risk, especially considering the large number of different legacy single-byte locales
someone might hypothetically be running their shell in.

### Demonstration

```bash
$ echo -e 'ls a\xa0b' | bash
ls: a: No such file or directory
ls: b: No such file or directory
```
The normal behavior would be to output a single line, e.g.:
```bash
$ echo -e 'ls a\xa0b' | bash
ls: cannot access 'a'$'\240''b': No such file or directory
```
(The specific quoting in the error doesn't matter.)

### Cause

Just for fun, here's why this behavior occurs:

Bash decides which bytes serve as word separators based on the libc function [`isblank`][isblank].
On macOS on UTF-8 locales, this passes for `\xa0`, corresponding to U+00A0 NO-BREAK SPACE.

This is doubly unique compared to the other systems I tested (Linux/glibc, Linux/musl, and
Windows/MSVC).  First, the other systems don't allow bytes in the range [0x80, 0xFF] to pass
<code>is<i>foo</i></code> functions in UTF-8 locales, even if the corresponding Unicode codepoint
does pass, as determined by the wide-character equivalent function, <code>isw<i>foo</i></code>.
Second, the other systems don't treat U+00A0 as blank (even using `iswblank`).

Meanwhile, Bash checks for multi-byte sequences and forbids them from being treated as special
characters, so the proper UTF-8 encoding of U+00A0, `b"\xc2\xa0"`, is not treated as a word
separator.  Treatment as a word separator only happens for `b"\xa0"` alone, which is illegal UTF-8.

[ansic]: https://www.gnu.org/software/bash/manual/html_node/ANSI_002dC-Quoting.html
[he]: https://www.gnu.org/software/bash/manual/html_node/History-Interaction.html
[qs]: https://www.gnu.org/software/bash/manual/html_node/Event-Designators.html
[isblank]: https://man7.org/linux/man-pages/man3/isblank.3p.html
[nul]: #nul-bytes

[^choices]: This can lead to tough choices over which
  characters to escape and which to leave as-is, especially when Unicode gets involved and you
  have to balance the risk of confusion with the benefit of properly supporting non-English
  languages.
  <br>
  <br>
  We don't have the luxury of those choices.

[^gr]: For example, backspace (in Unicode lingo, U+0008 BACKSPACE) turns into U+2408 SYMBOL FOR BACKSPACE.

[^bracketing]: It typically disables almost all handling of control characters by the shell proper,
    but one necessary exception is the end-of-paste sequence itself (which starts with the control
    character `\x1b`).  In addition, paste bracketing does not suppress handling of control
    characters by the kernel tty layer, such as `\x03` sending SIGINT (which typically clears the
    currently typed command, making it dangerous in a similar way to `\x01`).

[^escbs]: For example, Dash doesn't remove the backslash from `"\!"` because it simply doesn't know
    anything about `!` as a special character: it doesn't support history expansion.  On the other
    end of the spectrum, Zsh supports history expansion and does remove the backslash — though only
    in interactive mode.  Bash's behavior is weirder.  It supports history expansion, and if you
    write `"\!"`, the backslash does prevent history expansion from occurring — but it doesn't get
    removed!

```rust
pub mod quoting_warning { /* ... */ }
```

## Types

### Struct `Shlex`

An iterator that takes an input string and splits it into the words using the same syntax as
the POSIX shell.

See [`bytes::Shlex`].

```rust
pub struct Shlex<''a>(/* private field */);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

#### Implementations

##### Methods

- ```rust
  pub fn new(in_str: &''a str) -> Self { /* ... */ }
  ```

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Unpin**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<String> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Receiver**
### Enum `QuoteError`

**Attributes:**

- `#[non_exhaustive]`

Errors from [`Quoter::quote`], [`Quoter::join`], etc. (and their [`bytes`] counterparts).

By default, the only error that can be returned is [`QuoteError::Nul`].  If you call
`allow_nul(true)`, then no errors can be returned at all.  Any error variants added in the
future will not be enabled by default; they will be enabled through corresponding non-default
[`Quoter`] options.

...In theory.  In the unlikely event that additional classes of inputs are discovered that,
like nul bytes, are fundamentally unsafe to quote even for non-interactive shells, the risk
will be mitigated by adding corresponding [`QuoteError`] variants that *are* enabled by
default.

```rust
pub enum QuoteError {
    Nul,
}
```

#### Variants

##### `Nul`

The input contained a nul byte.  In most cases, shells fundamentally [cannot handle strings
containing nul bytes](quoting_warning#nul-bytes), no matter how they are quoted.  But if
you're sure you can handle nul bytes, you can call `allow_nul(true)` on the `Quoter` to let
them pass through.

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &QuoteError) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **Send**
- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &QuoteError) -> bool { /* ... */ }
    ```

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> QuoteError { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &QuoteError) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Error**
- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Struct `Quoter`

A more configurable interface to quote strings.  If you only want the default settings you can
use the convenience functions [`try_quote`] and [`try_join`].

The bytes equivalent is [`bytes::Quoter`].

```rust
pub struct Quoter {
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
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new [`Quoter`] with default settings.

- ```rust
  pub fn allow_nul(self: Self, allow: bool) -> Self { /* ... */ }
  ```
  Set whether to allow [nul bytes](quoting_warning#nul-bytes).  By default they are not

- ```rust
  pub fn join<''a, I: IntoIterator<Item = &''a str>>(self: &Self, words: I) -> Result<String, QuoteError> { /* ... */ }
  ```
  Convenience function that consumes an iterable of words and turns it into a single string,

- ```rust
  pub fn quote<''a>(self: &Self, in_str: &''a str) -> Result<Cow<''a, str>, QuoteError> { /* ... */ }
  ```
  Given a single word, return a string suitable to encode it as a shell argument.

##### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

  - ```rust
    fn from(inner: bytes::Quoter) -> Quoter { /* ... */ }
    ```

  - ```rust
    fn from(quoter: Quoter) -> bytes::Quoter { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Quoter { /* ... */ }
    ```

- **Unpin**
- **Default**
  - ```rust
    fn default() -> Quoter { /* ... */ }
    ```

## Functions

### Function `split`

Convenience function that consumes the whole string at once.  Returns None if the input was
erroneous.

```rust
pub fn split(in_str: &str) -> Option<alloc::vec::Vec<alloc::string::String>> { /* ... */ }
```

### Function `join`

**⚠️ Deprecated since 1.3.0**: replace with `try_join(words)?` to avoid nul byte danger

Convenience function that consumes an iterable of words and turns it into a single string,
quoting words when necessary. Consecutive words will be separated by a single space.

Uses default settings except that nul bytes are passed through, which [may be
dangerous](quoting_warning#nul-bytes), leading to this function being deprecated.

Equivalent to [`Quoter::new().allow_nul(true).join(words).unwrap()`](Quoter).

(That configuration never returns `Err`, so this function does not panic.)

The bytes equivalent is [bytes::join].

```rust
pub fn join<''a, I: IntoIterator<Item = &''a str>>(words: I) -> alloc::string::String { /* ... */ }
```

### Function `try_join`

Convenience function that consumes an iterable of words and turns it into a single string,
quoting words when necessary. Consecutive words will be separated by a single space.

Uses default settings.  The only error that can be returned is [`QuoteError::Nul`].

Equivalent to [`Quoter::new().join(words)`](Quoter).

The bytes equivalent is [bytes::try_join].

```rust
pub fn try_join<''a, I: IntoIterator<Item = &''a str>>(words: I) -> Result<alloc::string::String, QuoteError> { /* ... */ }
```

### Function `quote`

**⚠️ Deprecated since 1.3.0**: replace with `try_quote(str)?` to avoid nul byte danger

Given a single word, return a string suitable to encode it as a shell argument.

Uses default settings except that nul bytes are passed through, which [may be
dangerous](quoting_warning#nul-bytes), leading to this function being deprecated.

Equivalent to [`Quoter::new().allow_nul(true).quote(in_str).unwrap()`](Quoter).

(That configuration never returns `Err`, so this function does not panic.)

The bytes equivalent is [bytes::quote].

```rust
pub fn quote(in_str: &str) -> alloc::borrow::Cow<''_, str> { /* ... */ }
```

### Function `try_quote`

Given a single word, return a string suitable to encode it as a shell argument.

Uses default settings.  The only error that can be returned is [`QuoteError::Nul`].

Equivalent to [`Quoter::new().quote(in_str)`](Quoter).

(That configuration never returns `Err`, so this function does not panic.)

The bytes equivalent is [bytes::try_quote].

```rust
pub fn try_quote(in_str: &str) -> Result<alloc::borrow::Cow<''_, str>, QuoteError> { /* ... */ }
```

