# Crate Documentation

**Version:** 1.1.3

**Format Version:** 43

# Module `aho_corasick`

A library for finding occurrences of many patterns at once. This library
provides multiple pattern search principally through an implementation of the
[Aho-Corasick algorithm](https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm),
which builds a fast finite state machine for executing searches in linear time.

Additionally, this library provides a number of configuration options for
building the automaton that permit controlling the space versus time trade
off. Other features include simple ASCII case insensitive matching, finding
overlapping matches, replacements, searching streams and even searching and
replacing text in streams.

Finally, unlike most other Aho-Corasick implementations, this one
supports enabling [leftmost-first](MatchKind::LeftmostFirst) or
[leftmost-longest](MatchKind::LeftmostLongest) match semantics, using a
(seemingly) novel alternative construction algorithm. For more details on what
match semantics means, see the [`MatchKind`] type.

# Overview

This section gives a brief overview of the primary types in this crate:

* [`AhoCorasick`] is the primary type and represents an Aho-Corasick automaton.
This is the type you use to execute searches.
* [`AhoCorasickBuilder`] can be used to build an Aho-Corasick automaton, and
supports configuring a number of options.
* [`Match`] represents a single match reported by an Aho-Corasick automaton.
Each match has two pieces of information: the pattern that matched and the
start and end byte offsets corresponding to the position in the haystack at
which it matched.

# Example: basic searching

This example shows how to search for occurrences of multiple patterns
simultaneously. Each match includes the pattern that matched along with the
byte offsets of the match.

```
use aho_corasick::{AhoCorasick, PatternID};

let patterns = &["apple", "maple", "Snapple"];
let haystack = "Nobody likes maple in their apple flavored Snapple.";

let ac = AhoCorasick::new(patterns).unwrap();
let mut matches = vec![];
for mat in ac.find_iter(haystack) {
    matches.push((mat.pattern(), mat.start(), mat.end()));
}
assert_eq!(matches, vec![
    (PatternID::must(1), 13, 18),
    (PatternID::must(0), 28, 33),
    (PatternID::must(2), 43, 50),
]);
```

# Example: case insensitivity

This is like the previous example, but matches `Snapple` case insensitively
using `AhoCorasickBuilder`:

```
use aho_corasick::{AhoCorasick, PatternID};

let patterns = &["apple", "maple", "snapple"];
let haystack = "Nobody likes maple in their apple flavored Snapple.";

let ac = AhoCorasick::builder()
    .ascii_case_insensitive(true)
    .build(patterns)
    .unwrap();
let mut matches = vec![];
for mat in ac.find_iter(haystack) {
    matches.push((mat.pattern(), mat.start(), mat.end()));
}
assert_eq!(matches, vec![
    (PatternID::must(1), 13, 18),
    (PatternID::must(0), 28, 33),
    (PatternID::must(2), 43, 50),
]);
```

# Example: replacing matches in a stream

This example shows how to execute a search and replace on a stream without
loading the entire stream into memory first.

```
# #[cfg(feature = "std")] {
use aho_corasick::AhoCorasick;

# fn example() -> Result<(), std::io::Error> {
let patterns = &["fox", "brown", "quick"];
let replace_with = &["sloth", "grey", "slow"];

// In a real example, these might be `std::fs::File`s instead. All you need to
// do is supply a pair of `std::io::Read` and `std::io::Write` implementations.
let rdr = "The quick brown fox.";
let mut wtr = vec![];

let ac = AhoCorasick::new(patterns).unwrap();
ac.try_stream_replace_all(rdr.as_bytes(), &mut wtr, replace_with)?;
assert_eq!(b"The slow grey sloth.".to_vec(), wtr);
# Ok(()) }; example().unwrap()
# }
```

# Example: finding the leftmost first match

In the textbook description of Aho-Corasick, its formulation is typically
structured such that it reports all possible matches, even when they overlap
with another. In many cases, overlapping matches may not be desired, such as
the case of finding all successive non-overlapping matches like you might with
a standard regular expression.

Unfortunately the "obvious" way to modify the Aho-Corasick algorithm to do
this doesn't always work in the expected way, since it will report matches as
soon as they are seen. For example, consider matching the regex `Samwise|Sam`
against the text `Samwise`. Most regex engines (that are Perl-like, or
non-POSIX) will report `Samwise` as a match, but the standard Aho-Corasick
algorithm modified for reporting non-overlapping matches will report `Sam`.

A novel contribution of this library is the ability to change the match
semantics of Aho-Corasick (without additional search time overhead) such that
`Samwise` is reported instead. For example, here's the standard approach:

```
use aho_corasick::AhoCorasick;

let patterns = &["Samwise", "Sam"];
let haystack = "Samwise";

let ac = AhoCorasick::new(patterns).unwrap();
let mat = ac.find(haystack).expect("should have a match");
assert_eq!("Sam", &haystack[mat.start()..mat.end()]);
```

And now here's the leftmost-first version, which matches how a Perl-like
regex will work:

```
use aho_corasick::{AhoCorasick, MatchKind};

let patterns = &["Samwise", "Sam"];
let haystack = "Samwise";

let ac = AhoCorasick::builder()
    .match_kind(MatchKind::LeftmostFirst)
    .build(patterns)
    .unwrap();
let mat = ac.find(haystack).expect("should have a match");
assert_eq!("Samwise", &haystack[mat.start()..mat.end()]);
```

In addition to leftmost-first semantics, this library also supports
leftmost-longest semantics, which match the POSIX behavior of a regular
expression alternation. See [`MatchKind`] for more details.

# Prefilters

While an Aho-Corasick automaton can perform admirably when compared to more
naive solutions, it is generally slower than more specialized algorithms that
are accelerated using vector instructions such as SIMD.

For that reason, this library will internally use a "prefilter" to attempt
to accelerate searches when possible. Currently, this library has several
different algorithms it might use depending on the patterns provided. Once the
number of patterns gets too big, prefilters are no longer used.

While a prefilter is generally good to have on by default since it works
well in the common case, it can lead to less predictable or even sub-optimal
performance in some cases. For that reason, prefilters can be explicitly
disabled via [`AhoCorasickBuilder::prefilter`].

# Lower level APIs

This crate also provides several sub-modules that collectively expose many of
the implementation details of the main [`AhoCorasick`] type. Most users of this
library can completely ignore the submodules and their contents, but if you
needed finer grained control, some parts of them may be useful to you. Here is
a brief overview of each and why you might want to use them:

* The [`packed`] sub-module contains a lower level API for using fast
vectorized routines for finding a small number of patterns in a haystack.
You might want to use this API when you want to completely side-step using
Aho-Corasick automata. Otherwise, the fast vectorized routines are used
automatically as prefilters for `AhoCorasick` searches whenever possible.
* The [`automaton`] sub-module provides a lower level finite state
machine interface that the various Aho-Corasick implementations in
this crate implement. This sub-module's main contribution is the
[`Automaton`](automaton::Automaton) trait, which permits manually walking the
state transitions of an Aho-Corasick automaton.
* The [`dfa`] and [`nfa`] sub-modules provide DFA and NFA implementations of
the aforementioned `Automaton` trait. The main reason one might want to use
these sub-modules is to get access to a type that implements the `Automaton`
trait. (The top-level `AhoCorasick` type does not implement the `Automaton`
trait.)

As mentioned above, if you aren't sure whether you need these sub-modules,
you should be able to safely ignore them and just focus on the [`AhoCorasick`]
type.

# Crate features

This crate exposes a few features for controlling dependency usage and whether
this crate can be used without the standard library.

* **std** -
  Enables support for the standard library. This feature is enabled by
  default. When disabled, only `core` and `alloc` are used. At an API
  level, enabling `std` enables `std::error::Error` trait impls for the
  various error types, and higher level stream search routines such as
  [`AhoCorasick::try_stream_find_iter`]. But the `std` feature is also required
  to enable vectorized prefilters. Prefilters can greatly accelerate searches,
  but generally only apply when the number of patterns is small (less than
  ~100).
* **perf-literal** -
  Enables support for literal prefilters that use vectorized routines from
  external crates. This feature is enabled by default. If you're only using
  Aho-Corasick for large numbers of patterns or otherwise can abide lower
  throughput when searching with a small number of patterns, then it is
  reasonable to disable this feature.
* **logging** -
  Enables a dependency on the `log` crate and emits messages to aide in
  diagnostics. This feature is disabled by default.

## Modules

## Module `automaton`

Provides [`Automaton`] trait for abstracting over Aho-Corasick automata.

The `Automaton` trait provides a way to write generic code over any
Aho-Corasick automaton. It also provides access to lower level APIs that
permit walking the state transitions of an Aho-Corasick automaton manually.

```rust
pub mod automaton { /* ... */ }
```

### Types

#### Struct `OverlappingState`

Represents the current state of an overlapping search.

This is used for overlapping searches since they need to know something
about the previous search. For example, when multiple patterns match at the
same position, this state tracks the last reported pattern so that the next
search knows whether to report another matching pattern or continue with
the search at the next position. Additionally, it also tracks which state
the last search call terminated in and the current offset of the search
in the haystack.

This type provides limited introspection capabilities. The only thing a
caller can do is construct it and pass it around to permit search routines
to use it to track state, and to ask whether a match has been found.

Callers should always provide a fresh state constructed via
[`OverlappingState::start`] when starting a new search. That same state
should be reused for subsequent searches on the same `Input`. The state
given will advance through the haystack itself. Callers can detect the end
of a search when neither an error nor a match is returned.

# Example

This example shows how to manually iterate over all overlapping matches. If
you need this, you might consider using
[`AhoCorasick::find_overlapping_iter`](crate::AhoCorasick::find_overlapping_iter)
instead, but this shows how to correctly use an `OverlappingState`.

```
use aho_corasick::{
    automaton::OverlappingState,
    AhoCorasick, Input, Match,
};

let patterns = &["append", "appendage", "app"];
let haystack = "append the app to the appendage";

let ac = AhoCorasick::new(patterns).unwrap();
let mut state = OverlappingState::start();
let mut matches = vec![];

loop {
    ac.find_overlapping(haystack, &mut state);
    let mat = match state.get_match() {
        None => break,
        Some(mat) => mat,
    };
    matches.push(mat);
}
let expected = vec![
    Match::must(2, 0..3),
    Match::must(0, 0..6),
    Match::must(2, 11..14),
    Match::must(2, 22..25),
    Match::must(0, 22..28),
    Match::must(1, 22..31),
];
assert_eq!(expected, matches);
```

```rust
pub struct OverlappingState {
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
  pub fn start() -> OverlappingState { /* ... */ }
  ```
  Create a new overlapping state that begins at the start state.

- ```rust
  pub fn get_match(self: &Self) -> Option<Match> { /* ... */ }
  ```
  Return the match result of the most recent search to execute with this

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Unpin**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OverlappingState { /* ... */ }
    ```

#### Struct `FindIter`

An iterator of non-overlapping matches in a particular haystack.

This iterator yields matches according to the [`MatchKind`] used by this
automaton.

This iterator is constructed via the [`Automaton::try_find_iter`] method.

The type variable `A` refers to the implementation of the [`Automaton`]
trait used to execute the search.

The lifetime `'a` refers to the lifetime of the [`Automaton`]
implementation.

The lifetime `'h` refers to the lifetime of the haystack being searched.

```rust
pub struct FindIter<''a, ''h, A> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Match> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `FindOverlappingIter`

An iterator of overlapping matches in a particular haystack.

This iterator will report all possible matches in a particular haystack,
even when the matches overlap.

This iterator is constructed via the
[`Automaton::try_find_overlapping_iter`] method.

The type variable `A` refers to the implementation of the [`Automaton`]
trait used to execute the search.

The lifetime `'a` refers to the lifetime of the [`Automaton`]
implementation.

The lifetime `'h` refers to the lifetime of the haystack being searched.

```rust
pub struct FindOverlappingIter<''a, ''h, A> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Match> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `StreamFindIter`

**Attributes:**

- `#[<cfg>(feature = "std")]`

An iterator that reports matches in a stream.

This iterator yields elements of type `io::Result<Match>`, where an error
is reported if there was a problem reading from the underlying stream.
The iterator terminates only when the underlying stream reaches `EOF`.

This iterator is constructed via the [`Automaton::try_stream_find_iter`]
method.

The type variable `A` refers to the implementation of the [`Automaton`]
trait used to execute the search.

The type variable `R` refers to the `io::Read` stream that is being read
from.

The lifetime `'a` refers to the lifetime of the [`Automaton`]
implementation.

```rust
pub struct StreamFindIter<''a, A, R> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<std::io::Result<Match>> { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

### Traits

#### Trait `Automaton`

A trait that abstracts over Aho-Corasick automata.

This trait primarily exists for niche use cases such as:

* Using an NFA or DFA directly, bypassing the top-level
[`AhoCorasick`](crate::AhoCorasick) searcher. Currently, these include
[`noncontiguous::NFA`](crate::nfa::noncontiguous::NFA),
[`contiguous::NFA`](crate::nfa::contiguous::NFA) and
[`dfa::DFA`](crate::dfa::DFA).
* Implementing your own custom search routine by walking the automaton
yourself. This might be useful for implementing search on non-contiguous
strings or streams.

For most use cases, it is not expected that users will need
to use or even know about this trait. Indeed, the top level
[`AhoCorasick`](crate::AhoCorasick) searcher does not expose any details
about this trait, nor does it implement it itself.

Note that this trait defines a number of default methods, such as
[`Automaton::try_find`] and [`Automaton::try_find_iter`], which implement
higher level search routines in terms of the lower level automata API.

# Sealed

Currently, this trait is sealed. That means users of this crate can write
generic routines over this trait but cannot implement it themselves. This
restriction may be lifted in the future, but sealing the trait permits
adding new required methods in a backwards compatible fashion.

# Special states

This trait encodes a notion of "special" states in an automaton. Namely,
a state is treated as special if it is a dead, match or start state:

* A dead state is a state that cannot be left once entered. All transitions
on a dead state lead back to itself. The dead state is meant to be treated
as a sentinel indicating that the search should stop and return a match if
one has been found, and nothing otherwise.
* A match state is a state that indicates one or more patterns have
matched. Depending on the [`MatchKind`] of the automaton, a search may
stop once a match is seen, or it may continue looking for matches until
it enters a dead state or sees the end of the haystack.
* A start state is a state that a search begins in. It is useful to know
when a search enters a start state because it may mean that a prefilter can
be used to skip ahead and quickly look for candidate matches. Unlike dead
and match states, it is never necessary to explicitly handle start states
for correctness. Indeed, in this crate, implementations of `Automaton`
will only treat start states as "special" when a prefilter is enabled and
active. Otherwise, treating it as special has no purpose and winds up
slowing down the overall search because it results in ping-ponging between
the main state transition and the "special" state logic.

Since checking whether a state is special by doing three different
checks would be too expensive inside a fast search loop, the
[`Automaton::is_special`] method is provided for quickly checking whether
the state is special. The `Automaton::is_dead`, `Automaton::is_match` and
`Automaton::is_start` predicates can then be used to determine which kind
of special state it is.

# Panics

Most of the APIs on this trait should panic or give incorrect results
if invalid inputs are given to it. For example, `Automaton::next_state`
has unspecified behavior if the state ID given to it is not a valid
state ID for the underlying automaton. Valid state IDs can only be
retrieved in one of two ways: calling `Automaton::start_state` or calling
`Automaton::next_state` with a valid state ID.

# Safety

This trait is not safe to implement so that code may rely on the
correctness of implementations of this trait to avoid undefined behavior.
The primary correctness guarantees are:

* `Automaton::start_state` always returns a valid state ID or an error or
panics.
* `Automaton::next_state`, when given a valid state ID, always returns
a valid state ID for all values of `anchored` and `byte`, or otherwise
panics.

In general, the rest of the methods on `Automaton` need to uphold their
contracts as well. For example, `Automaton::is_dead` should only returns
true if the given state ID is actually a dead state.

Note that currently this crate does not rely on the safety property defined
here to avoid undefined behavior. Instead, this was done to make it
_possible_ to do in the future.

# Example

This example shows how one might implement a basic but correct search
routine. We keep things simple by not using prefilters or worrying about
anchored searches, but do make sure our search is correct for all possible
[`MatchKind`] semantics. (The comments in the code below note the parts
that are needed to support certain `MatchKind` semantics.)

```
use aho_corasick::{
    automaton::Automaton,
    nfa::noncontiguous::NFA,
    Anchored, Match, MatchError, MatchKind,
};

// Run an unanchored search for 'aut' in 'haystack'. Return the first match
// seen according to the automaton's match semantics. This returns an error
// if the given automaton does not support unanchored searches.
fn find<A: Automaton>(
    aut: A,
    haystack: &[u8],
) -> Result<Option<Match>, MatchError> {
    let mut sid = aut.start_state(Anchored::No)?;
    let mut at = 0;
    let mut mat = None;
    let get_match = |sid, at| {
        let pid = aut.match_pattern(sid, 0);
        let len = aut.pattern_len(pid);
        Match::new(pid, (at - len)..at)
    };
    // Start states can be match states!
    if aut.is_match(sid) {
        mat = Some(get_match(sid, at));
        // Standard semantics require matches to be reported as soon as
        // they're seen. Otherwise, we continue until we see a dead state
        // or the end of the haystack.
        if matches!(aut.match_kind(), MatchKind::Standard) {
            return Ok(mat);
        }
    }
    while at < haystack.len() {
        sid = aut.next_state(Anchored::No, sid, haystack[at]);
        if aut.is_special(sid) {
            if aut.is_dead(sid) {
                return Ok(mat);
            } else if aut.is_match(sid) {
                mat = Some(get_match(sid, at + 1));
                // As above, standard semantics require that we return
                // immediately once a match is found.
                if matches!(aut.match_kind(), MatchKind::Standard) {
                    return Ok(mat);
                }
            }
        }
        at += 1;
    }
    Ok(mat)
}

// Show that it works for standard searches.
let nfa = NFA::new(&["samwise", "sam"]).unwrap();
assert_eq!(Some(Match::must(1, 0..3)), find(&nfa, b"samwise")?);

// But also works when using leftmost-first. Notice how the match result
// has changed!
let nfa = NFA::builder()
    .match_kind(MatchKind::LeftmostFirst)
    .build(&["samwise", "sam"])
    .unwrap();
assert_eq!(Some(Match::must(0, 0..7)), find(&nfa, b"samwise")?);

# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub unsafe trait Automaton: private::Sealed {
    /* Associated items */
}
```

> This trait is unsafe to implement.

##### Required Items

###### Required Methods

- `start_state`: Returns the starting state for the given anchor mode.
- `next_state`: Performs a state transition from `sid` for `byte` and returns the next
- `is_special`: Returns true if the given ID represents a "special" state. A special
- `is_dead`: Returns true if the given ID represents a dead state.
- `is_match`: Returns true if the given ID represents a match state.
- `is_start`: Returns true if the given ID represents a start state.
- `match_kind`: Returns the match semantics that this automaton was built with.
- `match_len`: Returns the total number of matches for the given state ID.
- `match_pattern`: Returns the pattern ID for the match state given by `sid` at the
- `patterns_len`: Returns the total number of patterns compiled into this automaton.
- `pattern_len`: Returns the length of the pattern for the given ID.
- `min_pattern_len`: Returns the length, in bytes, of the shortest pattern in this
- `max_pattern_len`: Returns the length, in bytes, of the longest pattern in this automaton.
- `memory_usage`: Returns the heap memory usage, in bytes, used by this automaton.
- `prefilter`: Returns a prefilter, if available, that can be used to accelerate

##### Provided Methods

- ```rust
  fn try_find(self: &Self, input: &Input<''_>) -> Result<Option<Match>, MatchError> { /* ... */ }
  ```
  Executes a non-overlapping search with this automaton using the given

- ```rust
  fn try_find_overlapping(self: &Self, input: &Input<''_>, state: &mut OverlappingState) -> Result<(), MatchError> { /* ... */ }
  ```
  Executes a overlapping search with this automaton using the given

- ```rust
  fn try_find_iter<''a, ''h>(self: &''a Self, input: Input<''h>) -> Result<FindIter<''a, ''h, Self>, MatchError>
where
    Self: Sized { /* ... */ }
  ```
  Returns an iterator of non-overlapping matches with this automaton

- ```rust
  fn try_find_overlapping_iter<''a, ''h>(self: &''a Self, input: Input<''h>) -> Result<FindOverlappingIter<''a, ''h, Self>, MatchError>
where
    Self: Sized { /* ... */ }
  ```
  Returns an iterator of overlapping matches with this automaton

- ```rust
  fn try_replace_all<B>(self: &Self, haystack: &str, replace_with: &[B]) -> Result<String, MatchError>
where
    Self: Sized,
    B: AsRef<str> { /* ... */ }
  ```
  Replaces all non-overlapping matches in `haystack` with

- ```rust
  fn try_replace_all_bytes<B>(self: &Self, haystack: &[u8], replace_with: &[B]) -> Result<Vec<u8>, MatchError>
where
    Self: Sized,
    B: AsRef<[u8]> { /* ... */ }
  ```
  Replaces all non-overlapping matches in `haystack` with

- ```rust
  fn try_replace_all_with<F>(self: &Self, haystack: &str, dst: &mut String, replace_with: F) -> Result<(), MatchError>
where
    Self: Sized,
    F: FnMut(&Match, &str, &mut String) -> bool { /* ... */ }
  ```
  Replaces all non-overlapping matches in `haystack` by calling the

- ```rust
  fn try_replace_all_with_bytes<F>(self: &Self, haystack: &[u8], dst: &mut Vec<u8>, replace_with: F) -> Result<(), MatchError>
where
    Self: Sized,
    F: FnMut(&Match, &[u8], &mut Vec<u8>) -> bool { /* ... */ }
  ```
  Replaces all non-overlapping matches in `haystack` by calling the

- ```rust
  fn try_stream_find_iter<''a, R: std::io::Read>(self: &''a Self, rdr: R) -> Result<StreamFindIter<''a, Self, R>, MatchError>
where
    Self: Sized { /* ... */ }
  ```
  Returns an iterator of non-overlapping matches with this automaton

- ```rust
  fn try_stream_replace_all<R, W, B>(self: &Self, rdr: R, wtr: W, replace_with: &[B]) -> std::io::Result<()>
where
    Self: Sized,
    R: std::io::Read,
    W: std::io::Write,
    B: AsRef<[u8]> { /* ... */ }
  ```
  Replaces all non-overlapping matches in `rdr` with strings from

- ```rust
  fn try_stream_replace_all_with<R, W, F>(self: &Self, rdr: R, wtr: W, replace_with: F) -> std::io::Result<()>
where
    Self: Sized,
    R: std::io::Read,
    W: std::io::Write,
    F: FnMut(&Match, &[u8], &mut W) -> std::io::Result<()> { /* ... */ }
  ```
  Replaces all non-overlapping matches in `rdr` by calling the

##### Implementations

This trait is implemented for the following types:

- `&''a A` with <''a, A: Automaton + ?Sized>
- `DFA`
- `NFA`
- `NFA`

### Re-exports

#### Re-export `Candidate`

```rust
pub use crate::util::prefilter::Candidate;
```

#### Re-export `Prefilter`

```rust
pub use crate::util::prefilter::Prefilter;
```

#### Re-export `StateID`

```rust
pub use crate::util::primitives::StateID;
```

#### Re-export `StateIDError`

```rust
pub use crate::util::primitives::StateIDError;
```

## Module `dfa`

Provides direct access to a DFA implementation of Aho-Corasick.

This is a low-level API that generally only needs to be used in niche
circumstances. When possible, prefer using [`AhoCorasick`](crate::AhoCorasick)
instead of a DFA directly. Using an `DFA` directly is typically only necessary
when one needs access to the [`Automaton`] trait implementation.

```rust
pub mod dfa { /* ... */ }
```

### Types

#### Struct `DFA`

A DFA implementation of Aho-Corasick.

When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
this type directly. Using a `DFA` directly is typically only necessary when
one needs access to the [`Automaton`] trait implementation.

This DFA can only be built by first constructing a [`noncontiguous::NFA`].
Both [`DFA::new`] and [`Builder::build`] do this for you automatically, but
[`Builder::build_from_noncontiguous`] permits doing it explicitly.

A DFA provides the best possible search performance (in this crate) via two
mechanisms:

* All states use a dense representation for their transitions.
* All failure transitions are pre-computed such that they are never
explicitly handled at search time.

These two facts combined mean that every state transition is performed
using a constant number of instructions. However, this comes at
great cost. The memory usage of a DFA can be quite exorbitant.
It is potentially multiple orders of magnitude greater than a
[`contiguous::NFA`](crate::nfa::contiguous::NFA) for example. In exchange,
a DFA will typically have better search speed than a `contiguous::NFA`, but
not by orders of magnitude.

Unless you have a small number of patterns or memory usage is not a concern
and search performance is critical, a DFA is usually not the best choice.

Moreover, unlike the NFAs in this crate, it is costly for a DFA to
support for anchored and unanchored search configurations. Namely,
since failure transitions are pre-computed, supporting both anchored
and unanchored searches requires a duplication of the transition table,
making the memory usage of such a DFA ever bigger. (The NFAs in this crate
unconditionally support both anchored and unanchored searches because there
is essentially no added cost for doing so.) It is for this reason that
a DFA's support for anchored and unanchored searches can be configured
via [`Builder::start_kind`]. By default, a DFA only supports unanchored
searches.

# Example

This example shows how to build an `DFA` directly and use it to execute
[`Automaton::try_find`]:

```
use aho_corasick::{
    automaton::Automaton,
    dfa::DFA,
    Input, Match,
};

let patterns = &["b", "abc", "abcd"];
let haystack = "abcd";

let nfa = DFA::new(patterns).unwrap();
assert_eq!(
    Some(Match::must(0, 1..2)),
    nfa.try_find(&Input::new(haystack))?,
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

It is also possible to implement your own version of `try_find`. See the
[`Automaton`] documentation for an example.

```rust
pub struct DFA {
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
  pub fn new<I, P>(patterns: I) -> Result<DFA, BuildError>
where
    I: IntoIterator<Item = P>,
    P: AsRef<[u8]> { /* ... */ }
  ```
  Create a new Aho-Corasick DFA using the default configuration.

- ```rust
  pub fn builder() -> Builder { /* ... */ }
  ```
  A convenience method for returning a new Aho-Corasick DFA builder.

###### Trait Implementations

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
    fn clone(self: &Self) -> DFA { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Automaton**
  - ```rust
    fn start_state(self: &Self, anchored: Anchored) -> Result<StateID, MatchError> { /* ... */ }
    ```

  - ```rust
    fn next_state(self: &Self, _anchored: Anchored, sid: StateID, byte: u8) -> StateID { /* ... */ }
    ```

  - ```rust
    fn is_special(self: &Self, sid: StateID) -> bool { /* ... */ }
    ```

  - ```rust
    fn is_dead(self: &Self, sid: StateID) -> bool { /* ... */ }
    ```

  - ```rust
    fn is_match(self: &Self, sid: StateID) -> bool { /* ... */ }
    ```

  - ```rust
    fn is_start(self: &Self, sid: StateID) -> bool { /* ... */ }
    ```

  - ```rust
    fn match_kind(self: &Self) -> MatchKind { /* ... */ }
    ```

  - ```rust
    fn patterns_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn pattern_len(self: &Self, pid: PatternID) -> usize { /* ... */ }
    ```

  - ```rust
    fn min_pattern_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn max_pattern_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn match_len(self: &Self, sid: StateID) -> usize { /* ... */ }
    ```

  - ```rust
    fn match_pattern(self: &Self, sid: StateID, index: usize) -> PatternID { /* ... */ }
    ```

  - ```rust
    fn memory_usage(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn prefilter(self: &Self) -> Option<&Prefilter> { /* ... */ }
    ```

- **Freeze**
#### Struct `Builder`

A builder for configuring an Aho-Corasick DFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

```rust
pub struct Builder {
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
  pub fn new() -> Builder { /* ... */ }
  ```
  Create a new builder for configuring an Aho-Corasick DFA.

- ```rust
  pub fn build<I, P>(self: &Self, patterns: I) -> Result<DFA, BuildError>
where
    I: IntoIterator<Item = P>,
    P: AsRef<[u8]> { /* ... */ }
  ```
  Build an Aho-Corasick DFA from the given iterator of patterns.

- ```rust
  pub fn build_from_noncontiguous(self: &Self, nnfa: &noncontiguous::NFA) -> Result<DFA, BuildError> { /* ... */ }
  ```
  Build an Aho-Corasick DFA from the given noncontiguous NFA.

- ```rust
  pub fn match_kind(self: &mut Self, kind: MatchKind) -> &mut Builder { /* ... */ }
  ```
  Set the desired match semantics.

- ```rust
  pub fn ascii_case_insensitive(self: &mut Self, yes: bool) -> &mut Builder { /* ... */ }
  ```
  Enable ASCII-aware case insensitive matching.

- ```rust
  pub fn prefilter(self: &mut Self, yes: bool) -> &mut Builder { /* ... */ }
  ```
  Enable heuristic prefilter optimizations.

- ```rust
  pub fn start_kind(self: &mut Self, kind: StartKind) -> &mut Builder { /* ... */ }
  ```
  Sets the starting state configuration for the automaton.

- ```rust
  pub fn byte_classes(self: &mut Self, yes: bool) -> &mut Builder { /* ... */ }
  ```
  A debug setting for whether to attempt to shrink the size of the

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Builder { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Builder { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
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

## Module `nfa`

Provides direct access to NFA implementations of Aho-Corasick.

The principle characteristic of an NFA in this crate is that it may
transition through multiple states per byte of haystack. In Aho-Corasick
parlance, NFAs follow failure transitions during a search. In contrast,
a [`DFA`](crate::dfa::DFA) pre-computes all failure transitions during
compilation at the expense of a much bigger memory footprint.

Currently, there are two NFA implementations provided: noncontiguous and
contiguous. The names reflect their internal representation, and consequently,
the trade offs associated with them:

* A [`noncontiguous::NFA`] uses a separate allocation for every NFA state to
represent its transitions in a sparse format. This is ideal for building an
NFA, since it cheaply permits different states to have a different number of
transitions. A noncontiguous NFA is where the main Aho-Corasick construction
algorithm is implemented. All other Aho-Corasick implementations are built by
first constructing a noncontiguous NFA.
* A [`contiguous::NFA`] is uses a single allocation to represent all states,
while still encoding most states as sparse states but permitting states near
the starting state to have a dense representation. The dense representation
uses more memory, but permits computing transitions during a search more
quickly. By only making the most active states dense (the states near the
starting state), a contiguous NFA better balances memory usage with search
speed. The single contiguous allocation also uses less overhead per state and
enables compression tricks where most states only use 8 bytes of heap memory.

When given the choice between these two, you almost always want to pick a
contiguous NFA. It takes only a little longer to build, but both its memory
usage and search speed are typically much better than a noncontiguous NFA. A
noncontiguous NFA is useful when prioritizing build times, or when there are
so many patterns that a contiguous NFA could not be built. (Currently, because
of both memory and search speed improvements, a contiguous NFA has a smaller
internal limit on the total number of NFA states it can represent. But you
would likely need to have hundreds of thousands or even millions of patterns
before you hit this limit.)

```rust
pub mod nfa { /* ... */ }
```

### Modules

## Module `contiguous`

Provides a contiguous NFA implementation of Aho-Corasick.

This is a low-level API that generally only needs to be used in niche
circumstances. When possible, prefer using [`AhoCorasick`](crate::AhoCorasick)
instead of a contiguous NFA directly. Using an `NFA` directly is typically only
necessary when one needs access to the [`Automaton`] trait implementation.

```rust
pub mod contiguous { /* ... */ }
```

### Types

#### Struct `NFA`

A contiguous NFA implementation of Aho-Corasick.

When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
this type directly. Using an `NFA` directly is typically only necessary
when one needs access to the [`Automaton`] trait implementation.

This NFA can only be built by first constructing a [`noncontiguous::NFA`].
Both [`NFA::new`] and [`Builder::build`] do this for you automatically, but
[`Builder::build_from_noncontiguous`] permits doing it explicitly.

The main difference between a noncontiguous NFA and a contiguous NFA is
that the latter represents all of its states and transitions in a single
allocation, where as the former uses a separate allocation for each state.
Doing this at construction time while keeping a low memory footprint isn't
feasible, which is primarily why there are two different NFA types: one
that does the least amount of work possible to build itself, and another
that does a little extra work to compact itself and make state transitions
faster by making some states use a dense representation.

Because a contiguous NFA uses a single allocation, there is a lot more
opportunity for compression tricks to reduce the heap memory used. Indeed,
it is not uncommon for a contiguous NFA to use an order of magnitude less
heap memory than a noncontiguous NFA. Since building a contiguous NFA
usually only takes a fraction of the time it takes to build a noncontiguous
NFA, the overall build time is not much slower. Thus, in most cases, a
contiguous NFA is the best choice.

Since a contiguous NFA uses various tricks for compression and to achieve
faster state transitions, currently, its limit on the number of states
is somewhat smaller than what a noncontiguous NFA can achieve. Generally
speaking, you shouldn't expect to run into this limit if the number of
patterns is under 1 million. It is plausible that this limit will be
increased in the future. If the limit is reached, building a contiguous NFA
will return an error. Often, since building a contiguous NFA is relatively
cheap, it can make sense to always try it even if you aren't sure if it
will fail or not. If it does, you can always fall back to a noncontiguous
NFA. (Indeed, the main [`AhoCorasick`](crate::AhoCorasick) type employs a
strategy similar to this at construction time.)

# Example

This example shows how to build an `NFA` directly and use it to execute
[`Automaton::try_find`]:

```
use aho_corasick::{
    automaton::Automaton,
    nfa::contiguous::NFA,
    Input, Match,
};

let patterns = &["b", "abc", "abcd"];
let haystack = "abcd";

let nfa = NFA::new(patterns).unwrap();
assert_eq!(
    Some(Match::must(0, 1..2)),
    nfa.try_find(&Input::new(haystack))?,
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

It is also possible to implement your own version of `try_find`. See the
[`Automaton`] documentation for an example.

```rust
pub struct NFA {
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
  pub fn new<I, P>(patterns: I) -> Result<NFA, BuildError>
where
    I: IntoIterator<Item = P>,
    P: AsRef<[u8]> { /* ... */ }
  ```
  Create a new Aho-Corasick contiguous NFA using the default

- ```rust
  pub fn builder() -> Builder { /* ... */ }
  ```
  A convenience method for returning a new Aho-Corasick contiguous NFA

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Automaton**
  - ```rust
    fn start_state(self: &Self, anchored: Anchored) -> Result<StateID, MatchError> { /* ... */ }
    ```

  - ```rust
    fn next_state(self: &Self, anchored: Anchored, sid: StateID, byte: u8) -> StateID { /* ... */ }
    ```

  - ```rust
    fn is_special(self: &Self, sid: StateID) -> bool { /* ... */ }
    ```

  - ```rust
    fn is_dead(self: &Self, sid: StateID) -> bool { /* ... */ }
    ```

  - ```rust
    fn is_match(self: &Self, sid: StateID) -> bool { /* ... */ }
    ```

  - ```rust
    fn is_start(self: &Self, sid: StateID) -> bool { /* ... */ }
    ```

  - ```rust
    fn match_kind(self: &Self) -> MatchKind { /* ... */ }
    ```

  - ```rust
    fn patterns_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn pattern_len(self: &Self, pid: PatternID) -> usize { /* ... */ }
    ```

  - ```rust
    fn min_pattern_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn max_pattern_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn match_len(self: &Self, sid: StateID) -> usize { /* ... */ }
    ```

  - ```rust
    fn match_pattern(self: &Self, sid: StateID, index: usize) -> PatternID { /* ... */ }
    ```

  - ```rust
    fn memory_usage(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn prefilter(self: &Self) -> Option<&Prefilter> { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> NFA { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Builder`

A builder for configuring an Aho-Corasick contiguous NFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

```rust
pub struct Builder {
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
  pub fn new() -> Builder { /* ... */ }
  ```
  Create a new builder for configuring an Aho-Corasick contiguous NFA.

- ```rust
  pub fn build<I, P>(self: &Self, patterns: I) -> Result<NFA, BuildError>
where
    I: IntoIterator<Item = P>,
    P: AsRef<[u8]> { /* ... */ }
  ```
  Build an Aho-Corasick contiguous NFA from the given iterator of

- ```rust
  pub fn build_from_noncontiguous(self: &Self, nnfa: &noncontiguous::NFA) -> Result<NFA, BuildError> { /* ... */ }
  ```
  Build an Aho-Corasick contiguous NFA from the given noncontiguous NFA.

- ```rust
  pub fn match_kind(self: &mut Self, kind: MatchKind) -> &mut Builder { /* ... */ }
  ```
  Set the desired match semantics.

- ```rust
  pub fn ascii_case_insensitive(self: &mut Self, yes: bool) -> &mut Builder { /* ... */ }
  ```
  Enable ASCII-aware case insensitive matching.

- ```rust
  pub fn prefilter(self: &mut Self, yes: bool) -> &mut Builder { /* ... */ }
  ```
  Enable heuristic prefilter optimizations.

- ```rust
  pub fn dense_depth(self: &mut Self, depth: usize) -> &mut Builder { /* ... */ }
  ```
  Set the limit on how many states use a dense representation for their

- ```rust
  pub fn byte_classes(self: &mut Self, yes: bool) -> &mut Builder { /* ... */ }
  ```
  A debug setting for whether to attempt to shrink the size of the

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> Builder { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Builder { /* ... */ }
    ```

## Module `noncontiguous`

Provides a noncontiguous NFA implementation of Aho-Corasick.

This is a low-level API that generally only needs to be used in niche
circumstances. When possible, prefer using [`AhoCorasick`](crate::AhoCorasick)
instead of a noncontiguous NFA directly. Using an `NFA` directly is typically
only necessary when one needs access to the [`Automaton`] trait implementation.

```rust
pub mod noncontiguous { /* ... */ }
```

### Types

#### Struct `NFA`

A noncontiguous NFA implementation of Aho-Corasick.

When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
this type directly. Using an `NFA` directly is typically only necessary
when one needs access to the [`Automaton`] trait implementation.

This NFA represents the "core" implementation of Aho-Corasick in this
crate. Namely, constructing this NFA involving building a trie and then
filling in the failure transitions between states, similar to what is
described in any standard textbook description of Aho-Corasick.

In order to minimize heap usage and to avoid additional construction costs,
this implementation represents the transitions of all states as distinct
sparse memory allocations. This is where it gets its name from. That is,
this NFA has no contiguous memory allocation for its transition table. Each
state gets its own allocation.

While the sparse representation keeps memory usage to somewhat reasonable
levels, it is still quite large and also results in somewhat mediocre
search performance. For this reason, it is almost always a good idea to
use a [`contiguous::NFA`](crate::nfa::contiguous::NFA) instead. It is
marginally slower to build, but has higher throughput and can sometimes use
an order of magnitude less memory. The main reason to use a noncontiguous
NFA is when you need the fastest possible construction time, or when a
contiguous NFA does not have the desired capacity. (The total number of NFA
states it can have is fewer than a noncontiguous NFA.)

# Example

This example shows how to build an `NFA` directly and use it to execute
[`Automaton::try_find`]:

```
use aho_corasick::{
    automaton::Automaton,
    nfa::noncontiguous::NFA,
    Input, Match,
};

let patterns = &["b", "abc", "abcd"];
let haystack = "abcd";

let nfa = NFA::new(patterns).unwrap();
assert_eq!(
    Some(Match::must(0, 1..2)),
    nfa.try_find(&Input::new(haystack))?,
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

It is also possible to implement your own version of `try_find`. See the
[`Automaton`] documentation for an example.

```rust
pub struct NFA {
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
  pub fn new<I, P>(patterns: I) -> Result<NFA, BuildError>
where
    I: IntoIterator<Item = P>,
    P: AsRef<[u8]> { /* ... */ }
  ```
  Create a new Aho-Corasick noncontiguous NFA using the default

- ```rust
  pub fn builder() -> Builder { /* ... */ }
  ```
  A convenience method for returning a new Aho-Corasick noncontiguous NFA

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

- **Unpin**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NFA { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Send**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Automaton**
  - ```rust
    fn start_state(self: &Self, anchored: Anchored) -> Result<StateID, MatchError> { /* ... */ }
    ```

  - ```rust
    fn next_state(self: &Self, anchored: Anchored, sid: StateID, byte: u8) -> StateID { /* ... */ }
    ```

  - ```rust
    fn is_special(self: &Self, sid: StateID) -> bool { /* ... */ }
    ```

  - ```rust
    fn is_dead(self: &Self, sid: StateID) -> bool { /* ... */ }
    ```

  - ```rust
    fn is_match(self: &Self, sid: StateID) -> bool { /* ... */ }
    ```

  - ```rust
    fn is_start(self: &Self, sid: StateID) -> bool { /* ... */ }
    ```

  - ```rust
    fn match_kind(self: &Self) -> MatchKind { /* ... */ }
    ```

  - ```rust
    fn patterns_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn pattern_len(self: &Self, pid: PatternID) -> usize { /* ... */ }
    ```

  - ```rust
    fn min_pattern_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn max_pattern_len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn match_len(self: &Self, sid: StateID) -> usize { /* ... */ }
    ```

  - ```rust
    fn match_pattern(self: &Self, sid: StateID, index: usize) -> PatternID { /* ... */ }
    ```

  - ```rust
    fn memory_usage(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn prefilter(self: &Self) -> Option<&Prefilter> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Builder`

A builder for configuring an Aho-Corasick noncontiguous NFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

```rust
pub struct Builder {
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
  pub fn new() -> Builder { /* ... */ }
  ```
  Create a new builder for configuring an Aho-Corasick noncontiguous NFA.

- ```rust
  pub fn build<I, P>(self: &Self, patterns: I) -> Result<NFA, BuildError>
where
    I: IntoIterator<Item = P>,
    P: AsRef<[u8]> { /* ... */ }
  ```
  Build an Aho-Corasick noncontiguous NFA from the given iterator of

- ```rust
  pub fn match_kind(self: &mut Self, kind: MatchKind) -> &mut Builder { /* ... */ }
  ```
  Set the desired match semantics.

- ```rust
  pub fn ascii_case_insensitive(self: &mut Self, yes: bool) -> &mut Builder { /* ... */ }
  ```
  Enable ASCII-aware case insensitive matching.

- ```rust
  pub fn dense_depth(self: &mut Self, depth: usize) -> &mut Builder { /* ... */ }
  ```
  Set the limit on how many states use a dense representation for their

- ```rust
  pub fn prefilter(self: &mut Self, yes: bool) -> &mut Builder { /* ... */ }
  ```
  Enable heuristic prefilter optimizations.

###### Trait Implementations

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Builder { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Builder { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `packed`

Provides packed multiple substring search, principally for a small number of
patterns.

This sub-module provides vectorized routines for quickly finding
matches of a small number of patterns. In general, users of this crate
shouldn't need to interface with this module directly, as the primary
[`AhoCorasick`](crate::AhoCorasick) searcher will use these routines
automatically as a prefilter when applicable. However, in some cases, callers
may want to bypass the Aho-Corasick machinery entirely and use this vectorized
searcher directly.

# Overview

The primary types in this sub-module are:

* [`Searcher`] executes the actual search algorithm to report matches in a
haystack.
* [`Builder`] accumulates patterns incrementally and can construct a
`Searcher`.
* [`Config`] permits tuning the searcher, and itself will produce a `Builder`
(which can then be used to build a `Searcher`). Currently, the only tuneable
knob are the match semantics, but this may be expanded in the future.

# Examples

This example shows how to create a searcher from an iterator of patterns.
By default, leftmost-first match semantics are used. (See the top-level
[`MatchKind`] type for more details about match semantics, which apply
similarly to packed substring search.)

```
use aho_corasick::{packed::{MatchKind, Searcher}, PatternID};

# fn example() -> Option<()> {
let searcher = Searcher::new(["foobar", "foo"].iter().cloned())?;
let matches: Vec<PatternID> = searcher
    .find_iter("foobar")
    .map(|mat| mat.pattern())
    .collect();
assert_eq!(vec![PatternID::ZERO], matches);
# Some(()) }
# if cfg!(all(feature = "std", any(
#     target_arch = "x86_64", target_arch = "aarch64",
# ))) {
#     example().unwrap()
# } else {
#     assert!(example().is_none());
# }
```

This example shows how to use [`Config`] to change the match semantics to
leftmost-longest:

```
use aho_corasick::{packed::{Config, MatchKind}, PatternID};

# fn example() -> Option<()> {
let searcher = Config::new()
    .match_kind(MatchKind::LeftmostLongest)
    .builder()
    .add("foo")
    .add("foobar")
    .build()?;
let matches: Vec<PatternID> = searcher
    .find_iter("foobar")
    .map(|mat| mat.pattern())
    .collect();
assert_eq!(vec![PatternID::must(1)], matches);
# Some(()) }
# if cfg!(all(feature = "std", any(
#     target_arch = "x86_64", target_arch = "aarch64",
# ))) {
#     example().unwrap()
# } else {
#     assert!(example().is_none());
# }
```

# Packed substring searching

Packed substring searching refers to the use of SIMD (Single Instruction,
Multiple Data) to accelerate the detection of matches in a haystack. Unlike
conventional algorithms, such as Aho-Corasick, SIMD algorithms for substring
search tend to do better with a small number of patterns, where as Aho-Corasick
generally maintains reasonably consistent performance regardless of the number
of patterns you give it. Because of this, the vectorized searcher in this
sub-module cannot be used as a general purpose searcher, since building the
searcher may fail even when given a small number of patterns. However, in
exchange, when searching for a small number of patterns, searching can be quite
a bit faster than Aho-Corasick (sometimes by an order of magnitude).

The key take away here is that constructing a searcher from a list of patterns
is a fallible operation with no clear rules for when it will fail. While the
precise conditions under which building a searcher can fail is specifically an
implementation detail, here are some common reasons:

* Too many patterns were given. Typically, the limit is on the order of 100 or
  so, but this limit may fluctuate based on available CPU features.
* The available packed algorithms require CPU features that aren't available.
  For example, currently, this crate only provides packed algorithms for
  `x86_64` and `aarch64`. Therefore, constructing a packed searcher on any
  other target will always fail.
* Zero patterns were given, or one of the patterns given was empty. Packed
  searchers require at least one pattern and that all patterns are non-empty.
* Something else about the nature of the patterns (typically based on
  heuristics) suggests that a packed searcher would perform very poorly, so
  no searcher is built.

```rust
pub mod packed { /* ... */ }
```

### Re-exports

#### Re-export `Builder`

```rust
pub use crate::packed::api::Builder;
```

#### Re-export `Config`

```rust
pub use crate::packed::api::Config;
```

#### Re-export `FindIter`

```rust
pub use crate::packed::api::FindIter;
```

#### Re-export `MatchKind`

```rust
pub use crate::packed::api::MatchKind;
```

#### Re-export `Searcher`

```rust
pub use crate::packed::api::Searcher;
```

## Re-exports

### Re-export `StreamFindIter`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::ahocorasick::StreamFindIter;
```

### Re-export `AhoCorasick`

```rust
pub use crate::ahocorasick::AhoCorasick;
```

### Re-export `AhoCorasickBuilder`

```rust
pub use crate::ahocorasick::AhoCorasickBuilder;
```

### Re-export `AhoCorasickKind`

```rust
pub use crate::ahocorasick::AhoCorasickKind;
```

### Re-export `FindIter`

```rust
pub use crate::ahocorasick::FindIter;
```

### Re-export `FindOverlappingIter`

```rust
pub use crate::ahocorasick::FindOverlappingIter;
```

### Re-export `BuildError`

```rust
pub use crate::util::error::BuildError;
```

### Re-export `MatchError`

```rust
pub use crate::util::error::MatchError;
```

### Re-export `MatchErrorKind`

```rust
pub use crate::util::error::MatchErrorKind;
```

### Re-export `PatternID`

```rust
pub use crate::util::primitives::PatternID;
```

### Re-export `PatternIDError`

```rust
pub use crate::util::primitives::PatternIDError;
```

### Re-export `Anchored`

```rust
pub use crate::util::search::Anchored;
```

### Re-export `Input`

```rust
pub use crate::util::search::Input;
```

### Re-export `Match`

```rust
pub use crate::util::search::Match;
```

### Re-export `MatchKind`

```rust
pub use crate::util::search::MatchKind;
```

### Re-export `Span`

```rust
pub use crate::util::search::Span;
```

### Re-export `StartKind`

```rust
pub use crate::util::search::StartKind;
```

