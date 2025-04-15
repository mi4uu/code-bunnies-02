# Crate Documentation

**Version:** 0.4.9

**Format Version:** 43

# Module `regex_automata`

This crate exposes a variety of regex engines used by the `regex` crate.
It provides a vast, sprawling and "expert" level API to each regex engine.
The regex engines provided by this crate focus heavily on finite automata
implementations and specifically guarantee worst case `O(m * n)` time
complexity for all searches. (Where `m ~ len(regex)` and `n ~ len(haystack)`.)

The primary goal of this crate is to serve as an implementation detail for the
`regex` crate. A secondary goal is to make its internals available for use by
others.

# Table of contents

* [Should I be using this crate?](#should-i-be-using-this-crate) gives some
reasons for and against using this crate.
* [Examples](#examples) provides a small selection of things you can do with
this crate.
* [Available regex engines](#available-regex-engines) provides a hyperlinked
list of all regex engines in this crate.
* [API themes](#api-themes) discusses common elements used throughout this
crate.
* [Crate features](#crate-features) documents the extensive list of Cargo
features available.

# Should I be using this crate?

If you find yourself here because you just want to use regexes, then you should
first check out whether the [`regex` crate](https://docs.rs/regex) meets
your needs. It provides a streamlined and difficult-to-misuse API for regex
searching.

If you're here because there is something specific you want to do that can't
be easily done with `regex` crate, then you are perhaps in the right place.
It's most likely that the first stop you'll want to make is to explore the
[`meta` regex APIs](meta). Namely, the `regex` crate is just a light wrapper
over a [`meta::Regex`], so its API will probably be the easiest to transition
to. In contrast to the `regex` crate, the `meta::Regex` API supports more
search parameters and does multi-pattern searches. However, it isn't quite as
ergonomic.

Otherwise, the following is an inexhaustive list of reasons to use this crate:

* You want to analyze or use a [Thompson `NFA`](nfa::thompson::NFA) directly.
* You want more powerful multi-pattern search than what is provided by
`RegexSet` in the `regex` crate. All regex engines in this crate support
multi-pattern searches.
* You want to use one of the `regex` crate's internal engines directly because
of some interesting configuration that isn't possible via the `regex` crate.
For example, a [lazy DFA's configuration](hybrid::dfa::Config) exposes a
dizzying number of options for controlling its execution.
* You want to use the lower level search APIs. For example, both the [lazy
DFA](hybrid::dfa) and [fully compiled DFAs](dfa) support searching by exploring
the automaton one state at a time. This might be useful, for example, for
stream searches or searches of strings stored in non-contiguous in memory.
* You want to build a fully compiled DFA and then [use zero-copy
deserialization](dfa::dense::DFA::from_bytes) to load it into memory and use
it for searching. This use case is supported in core-only no-std/no-alloc
environments.
* You want to run [anchored searches](Input::anchored) without using the `^`
anchor in your regex pattern.
* You need to work-around contention issues with
sharing a regex across multiple threads. The
[`meta::Regex::search_with`](meta::Regex::search_with) API permits bypassing
any kind of synchronization at all by requiring the caller to provide the
mutable scratch spaced needed during a search.
* You want to build your own regex engine on top of the `regex` crate's
infrastructure.

# Examples

This section tries to identify a few interesting things you can do with this
crate and demonstrates them.

### Multi-pattern searches with capture groups

One of the more frustrating limitations of `RegexSet` in the `regex` crate
(at the time of writing) is that it doesn't report match positions. With this
crate, multi-pattern support was intentionally designed in from the beginning,
which means it works in all regex engines and even for capture groups as well.

This example shows how to search for matches of multiple regexes, where each
regex uses the same capture group names to parse different key-value formats.

```
use regex_automata::{meta::Regex, PatternID};

let re = Regex::new_many(&[
    r#"(?m)^(?<key>[[:word:]]+)=(?<val>[[:word:]]+)$"#,
    r#"(?m)^(?<key>[[:word:]]+)="(?<val>[^"]+)"$"#,
    r#"(?m)^(?<key>[[:word:]]+)='(?<val>[^']+)'$"#,
    r#"(?m)^(?<key>[[:word:]]+):\s*(?<val>[[:word:]]+)$"#,
])?;
let hay = r#"
best_album="Blow Your Face Out"
best_quote='"then as it was, then again it will be"'
best_year=1973
best_simpsons_episode: HOMR
"#;
let mut kvs = vec![];
for caps in re.captures_iter(hay) {
    // N.B. One could use capture indices '1' and '2' here
    // as well. Capture indices are local to each pattern.
    // (Just like names are.)
    let key = &hay[caps.get_group_by_name("key").unwrap()];
    let val = &hay[caps.get_group_by_name("val").unwrap()];
    kvs.push((key, val));
}
assert_eq!(kvs, vec![
    ("best_album", "Blow Your Face Out"),
    ("best_quote", "\"then as it was, then again it will be\""),
    ("best_year", "1973"),
    ("best_simpsons_episode", "HOMR"),
]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

### Build a full DFA and walk it manually

One of the regex engines in this crate is a fully compiled DFA. It takes worst
case exponential time to build, but once built, it can be easily explored and
used for searches. Here's a simple example that uses its lower level APIs to
implement a simple anchored search by hand.

```
use regex_automata::{dfa::{Automaton, dense}, Input};

let dfa = dense::DFA::new(r"(?-u)\b[A-Z]\w+z\b")?;
let haystack = "Quartz";

// The start state is determined by inspecting the position and the
// initial bytes of the haystack.
let mut state = dfa.start_state_forward(&Input::new(haystack))?;
// Walk all the bytes in the haystack.
for &b in haystack.as_bytes().iter() {
    state = dfa.next_state(state, b);
}
// DFAs in this crate require an explicit
// end-of-input transition if a search reaches
// the end of a haystack.
state = dfa.next_eoi_state(state);
assert!(dfa.is_match_state(state));

# Ok::<(), Box<dyn std::error::Error>>(())
```

Or do the same with a lazy DFA that avoids exponential worst case compile time,
but requires mutable scratch space to lazily build the DFA during the search.

```
use regex_automata::{hybrid::dfa::DFA, Input};

let dfa = DFA::new(r"(?-u)\b[A-Z]\w+z\b")?;
let mut cache = dfa.create_cache();
let hay = "Quartz";

// The start state is determined by inspecting the position and the
// initial bytes of the haystack.
let mut state = dfa.start_state_forward(&mut cache, &Input::new(hay))?;
// Walk all the bytes in the haystack.
for &b in hay.as_bytes().iter() {
    state = dfa.next_state(&mut cache, state, b)?;
}
// DFAs in this crate require an explicit
// end-of-input transition if a search reaches
// the end of a haystack.
state = dfa.next_eoi_state(&mut cache, state)?;
assert!(state.is_match());

# Ok::<(), Box<dyn std::error::Error>>(())
```

### Find all overlapping matches

This example shows how to build a DFA and use it to find all possible matches,
including overlapping matches. A similar example will work with a lazy DFA as
well. This also works with multiple patterns and will report all matches at the
same position where multiple patterns match.

```
use regex_automata::{
    dfa::{dense, Automaton, OverlappingState},
    Input, MatchKind,
};

let dfa = dense::DFA::builder()
    .configure(dense::DFA::config().match_kind(MatchKind::All))
    .build(r"(?-u)\w{3,}")?;
let input = Input::new("homer marge bart lisa maggie");
let mut state = OverlappingState::start();

let mut matches = vec![];
while let Some(hm) = {
    dfa.try_search_overlapping_fwd(&input, &mut state)?;
    state.get_match()
} {
    matches.push(hm.offset());
}
assert_eq!(matches, vec![
    3, 4, 5,        // hom, home, homer
    9, 10, 11,      // mar, marg, marge
    15, 16,         // bar, bart
    20, 21,         // lis, lisa
    25, 26, 27, 28, // mag, magg, maggi, maggie
]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Available regex engines

The following is a complete list of all regex engines provided by this crate,
along with a very brief description of it and why you might want to use it.

* [`dfa::regex::Regex`] is a regex engine that works on top of either
[dense](dfa::dense) or [sparse](dfa::sparse) fully compiled DFAs. You might
use a DFA if you need the fastest possible regex engine in this crate and can
afford the exorbitant memory usage usually required by DFAs. Low level APIs on
fully compiled DFAs are provided by the [`Automaton` trait](dfa::Automaton).
Fully compiled dense DFAs can handle all regexes except for searching a regex
with a Unicode word boundary on non-ASCII haystacks. A fully compiled DFA based
regex can only report the start and end of each match.
* [`hybrid::regex::Regex`] is a regex engine that works on top of a lazily
built DFA. Its performance profile is very similar to that of fully compiled
DFAs, but can be slower in some pathological cases. Fully compiled DFAs are
also amenable to more optimizations, such as state acceleration, that aren't
available in a lazy DFA. You might use this lazy DFA if you can't abide the
worst case exponential compile time of a full DFA, but still want the DFA
search performance in the vast majority of cases. A lazy DFA based regex can
only report the start and end of each match.
* [`dfa::onepass::DFA`] is a regex engine that is implemented as a DFA, but
can report the matches of each capture group in addition to the start and end
of each match. The catch is that it only works on a somewhat small subset of
regexes known as "one-pass." You'll want to use this for cases when you need
capture group matches and the regex is one-pass since it is likely to be faster
than any alternative. A one-pass DFA can handle all types of regexes, but does
have some reasonable limits on the number of capture groups it can handle.
* [`nfa::thompson::backtrack::BoundedBacktracker`] is a regex engine that uses
backtracking, but keeps track of the work it has done to avoid catastrophic
backtracking. Like the one-pass DFA, it provides the matches of each capture
group. It retains the `O(m * n)` worst case time bound. This tends to be slower
than the one-pass DFA regex engine, but faster than the PikeVM. It can handle
all types of regexes, but usually only works well with small haystacks and
small regexes due to the memory required to avoid redoing work.
* [`nfa::thompson::pikevm::PikeVM`] is a regex engine that can handle all
regexes, of all sizes and provides capture group matches. It tends to be a tool
of last resort because it is also usually the slowest regex engine.
* [`meta::Regex`] is the meta regex engine that combines *all* of the above
engines into one. The reason for this is that each of the engines above have
their own caveats such as, "only handles a subset of regexes" or "is generally
slow." The meta regex engine accounts for all of these caveats and composes
the engines in a way that attempts to mitigate each engine's weaknesses while
emphasizing its strengths. For example, it will attempt to run a lazy DFA even
if it might fail. In which case, it will restart the search with a likely
slower but more capable regex engine. The meta regex engine is what you should
default to. Use one of the above engines directly only if you have a specific
reason to.

# API themes

While each regex engine has its own APIs and configuration options, there are
some general themes followed by all of them.

### The `Input` abstraction

Most search routines in this crate accept anything that implements
`Into<Input>`. Both `&str` and `&[u8]` haystacks satisfy this constraint, which
means that things like `engine.search("foo")` will work as you would expect.

By virtue of accepting an `Into<Input>` though, callers can provide more than
just a haystack. Indeed, the [`Input`] type has more details, but briefly,
callers can use it to configure various aspects of the search:

* The span of the haystack to search via [`Input::span`] or [`Input::range`],
which might be a substring of the haystack.
* Whether to run an anchored search or not via [`Input::anchored`]. This
permits one to require matches to start at the same offset that the search
started.
* Whether to ask the regex engine to stop as soon as a match is seen via
[`Input::earliest`]. This can be used to find the offset of a match as soon
as it is known without waiting for the full leftmost-first match to be found.
This can also be used to avoid the worst case `O(m * n^2)` time complexity
of iteration.

Some lower level search routines accept an `&Input` for performance reasons.
In which case, `&Input::new("haystack")` can be used for a simple search.

### Error reporting

Most, but not all, regex engines in this crate can fail to execute a search.
When a search fails, callers cannot determine whether or not a match exists.
That is, the result is indeterminate.

Search failure, in all cases in this crate, is represented by a [`MatchError`].
Routines that can fail start with the `try_` prefix in their name. For example,
[`hybrid::regex::Regex::try_search`] can fail for a number of reasons.
Conversely, routines that either can't fail or can panic on failure lack the
`try_` prefix. For example, [`hybrid::regex::Regex::find`] will panic in
cases where [`hybrid::regex::Regex::try_search`] would return an error, and
[`meta::Regex::find`] will never panic. Therefore, callers need to pay close
attention to the panicking conditions in the documentation.

In most cases, the reasons that a search fails are either predictable or
configurable, albeit at some additional cost.

An example of predictable failure is
[`BoundedBacktracker::try_search`](nfa::thompson::backtrack::BoundedBacktracker::try_search).
Namely, it fails whenever the multiplication of the haystack, the regex and some
constant exceeds the
[configured visited capacity](nfa::thompson::backtrack::Config::visited_capacity).
Callers can predict the failure in terms of haystack length via the
[`BoundedBacktracker::max_haystack_len`](nfa::thompson::backtrack::BoundedBacktracker::max_haystack_len)
method. While this form of failure is technically avoidable by increasing the
visited capacity, it isn't practical to do so for all inputs because the
memory usage required for larger haystacks becomes impractically large. So in
practice, if one is using the bounded backtracker, you really do have to deal
with the failure.

An example of configurable failure happens when one enables heuristic support
for Unicode word boundaries in a DFA. Namely, since the DFAs in this crate
(except for the one-pass DFA) do not support Unicode word boundaries on
non-ASCII haystacks, building a DFA from an NFA that contains a Unicode word
boundary will itself fail. However, one can configure DFAs to still be built in
this case by
[configuring heuristic support for Unicode word boundaries](hybrid::dfa::Config::unicode_word_boundary).
If the NFA the DFA is built from contains a Unicode word boundary, then the
DFA will still be built, but special transitions will be added to every state
that cause the DFA to fail if any non-ASCII byte is seen. This failure happens
at search time and it requires the caller to opt into this.

There are other ways for regex engines to fail in this crate, but the above
two should represent the general theme of failures one can find. Dealing
with these failures is, in part, one the responsibilities of the [meta regex
engine](meta). Notice, for example, that the meta regex engine exposes an API
that never returns an error nor panics. It carefully manages all of the ways
in which the regex engines can fail and either avoids the predictable ones
entirely (e.g., the bounded backtracker) or reacts to configured failures by
falling back to a different engine (e.g., the lazy DFA quitting because it saw
a non-ASCII byte).

### Configuration and Builders

Most of the regex engines in this crate come with two types to facilitate
building the regex engine: a `Config` and a `Builder`. A `Config` is usually
specific to that particular regex engine, but other objects such as parsing and
NFA compilation have `Config` types too. A `Builder` is the thing responsible
for taking inputs (either pattern strings or already-parsed patterns or even
NFAs directly) and turning them into an actual regex engine that can be used
for searching.

The main reason why building a regex engine is a bit complicated is because
of the desire to permit composition with de-coupled components. For example,
you might want to [manually construct a Thompson NFA](nfa::thompson::Builder)
and then build a regex engine from it without ever using a regex parser
at all. On the other hand, you might also want to build a regex engine directly
from the concrete syntax. This demonstrates why regex engine construction is
so flexible: it needs to support not just convenient construction, but also
construction from parts built elsewhere.

This is also in turn why there are many different `Config` structs in this
crate. Let's look more closely at an example: [`hybrid::regex::Builder`]. It
accepts three different `Config` types for configuring construction of a lazy
DFA regex:

* [`hybrid::regex::Builder::syntax`] accepts a
[`util::syntax::Config`] for configuring the options found in the
[`regex-syntax`](regex_syntax) crate. For example, whether to match
case insensitively.
* [`hybrid::regex::Builder::thompson`] accepts a [`nfa::thompson::Config`] for
configuring construction of a [Thompson NFA](nfa::thompson::NFA). For example,
whether to build an NFA that matches the reverse language described by the
regex.
* [`hybrid::regex::Builder::dfa`] accept a [`hybrid::dfa::Config`] for
configuring construction of the pair of underlying lazy DFAs that make up the
lazy DFA regex engine. For example, changing the capacity of the cache used to
store the transition table.

The lazy DFA regex engine uses all three of those configuration objects for
methods like [`hybrid::regex::Builder::build`], which accepts a pattern
string containing the concrete syntax of your regex. It uses the syntax
configuration to parse it into an AST and translate it into an HIR. Then the
NFA configuration when compiling the HIR into an NFA. And then finally the DFA
configuration when lazily determinizing the NFA into a DFA.

Notice though that the builder also has a
[`hybrid::regex::Builder::build_from_dfas`] constructor. This permits callers
to build the underlying pair of lazy DFAs themselves (one for the forward
searching to find the end of a match and one for the reverse searching to find
the start of a match), and then build the regex engine from them. The lazy
DFAs, in turn, have their own builder that permits [construction directly from
a Thompson NFA](hybrid::dfa::Builder::build_from_nfa). Continuing down the
rabbit hole, a Thompson NFA has its own compiler that permits [construction
directly from an HIR](nfa::thompson::Compiler::build_from_hir). The lazy DFA
regex engine builder lets you follow this rabbit hole all the way down, but
also provides convenience routines that do it for you when you don't need
precise control over every component.

The [meta regex engine](meta) is a good example of something that utilizes the
full flexibility of these builders. It often needs not only precise control
over each component, but also shares them across multiple regex engines.
(Most sharing is done by internal reference accounting. For example, an
[`NFA`](nfa::thompson::NFA) is reference counted internally which makes cloning
cheap.)

### Size limits

Unlike the `regex` crate, the `regex-automata` crate specifically does not
enable any size limits by default. That means users of this crate need to
be quite careful when using untrusted patterns. Namely, because bounded
repetitions can grow exponentially by stacking them, it is possible to build a
very large internal regex object from just a small pattern string. For example,
the NFA built from the pattern `a{10}{10}{10}{10}{10}{10}{10}` is over 240MB.

There are multiple size limit options in this crate. If one or more size limits
are relevant for the object you're building, they will be configurable via
methods on a corresponding `Config` type.

# Crate features

This crate has a dizzying number of features. The main idea is to be able to
control how much stuff you pull in for your specific use case, since the full
crate is quite large and can dramatically increase compile times and binary
size.

The most barebones but useful configuration is to disable all default features
and enable only `dfa-search`. This will bring in just the DFA deserialization
and search routines without any dependency on `std` or `alloc`. This does
require generating and serializing a DFA, and then storing it somewhere, but
it permits regex searches in freestanding or embedded environments.

Because there are so many features, they are split into a few groups.

The default set of features is: `std`, `syntax`, `perf`, `unicode`, `meta`,
`nfa`, `dfa` and `hybrid`. Basically, the default is to enable everything
except for development related features like `logging`.

### Ecosystem features

* **std** - Enables use of the standard library. In terms of APIs, this usually
just means that error types implement the `std::error::Error` trait. Otherwise,
`std` sometimes enables the code to be faster, for example, using a `HashMap`
instead of a `BTreeMap`. (The `std` feature matters more for dependencies like
`aho-corasick` and `memchr`, where `std` is required to enable certain classes
of SIMD optimizations.) Enabling `std` automatically enables `alloc`.
* **alloc** - Enables use of the `alloc` library. This is required for most
APIs in this crate. The main exception is deserializing and searching with
fully compiled DFAs.
* **logging** - Adds a dependency on the `log` crate and makes this crate emit
log messages of varying degrees of utility. The log messages are especially
useful in trying to understand what the meta regex engine is doing.

### Performance features

* **perf** - Enables all of the below features.
* **perf-inline** - When enabled, `inline(always)` is used in (many) strategic
locations to help performance at the expense of longer compile times and
increased binary size.
* **perf-literal** - Enables all literal related optimizations.
    * **perf-literal-substring** - Enables all single substring literal
    optimizations. This includes adding a dependency on the `memchr` crate.
    * **perf-literal-multisubstring** - Enables all multiple substring literal
    optimizations. This includes adding a dependency on the `aho-corasick`
    crate.

### Unicode features

* **unicode** -
  Enables all Unicode features. This feature is enabled by default, and will
  always cover all Unicode features, even if more are added in the future.
* **unicode-age** -
  Provide the data for the
  [Unicode `Age` property](https://www.unicode.org/reports/tr44/tr44-24.html#Character_Age).
  This makes it possible to use classes like `\p{Age:6.0}` to refer to all
  codepoints first introduced in Unicode 6.0
* **unicode-bool** -
  Provide the data for numerous Unicode boolean properties. The full list
  is not included here, but contains properties like `Alphabetic`, `Emoji`,
  `Lowercase`, `Math`, `Uppercase` and `White_Space`.
* **unicode-case** -
  Provide the data for case insensitive matching using
  [Unicode's "simple loose matches" specification](https://www.unicode.org/reports/tr18/#Simple_Loose_Matches).
* **unicode-gencat** -
  Provide the data for
  [Unicode general categories](https://www.unicode.org/reports/tr44/tr44-24.html#General_Category_Values).
  This includes, but is not limited to, `Decimal_Number`, `Letter`,
  `Math_Symbol`, `Number` and `Punctuation`.
* **unicode-perl** -
  Provide the data for supporting the Unicode-aware Perl character classes,
  corresponding to `\w`, `\s` and `\d`. This is also necessary for using
  Unicode-aware word boundary assertions. Note that if this feature is
  disabled, the `\s` and `\d` character classes are still available if the
  `unicode-bool` and `unicode-gencat` features are enabled, respectively.
* **unicode-script** -
  Provide the data for
  [Unicode scripts and script extensions](https://www.unicode.org/reports/tr24/).
  This includes, but is not limited to, `Arabic`, `Cyrillic`, `Hebrew`,
  `Latin` and `Thai`.
* **unicode-segment** -
  Provide the data necessary to provide the properties used to implement the
  [Unicode text segmentation algorithms](https://www.unicode.org/reports/tr29/).
  This enables using classes like `\p{gcb=Extend}`, `\p{wb=Katakana}` and
  `\p{sb=ATerm}`.
* **unicode-word-boundary** -
  Enables support for Unicode word boundaries, i.e., `\b`, in regexes. When
  this and `unicode-perl` are enabled, then data tables from `regex-syntax` are
  used to implement Unicode word boundaries. However, if `regex-syntax` isn't
  enabled as a dependency then one can still enable this feature. It will
  cause `regex-automata` to bundle its own data table that would otherwise be
  redundant with `regex-syntax`'s table.

### Regex engine features

* **syntax** - Enables a dependency on `regex-syntax`. This makes APIs
for building regex engines from pattern strings available. Without the
`regex-syntax` dependency, the only way to build a regex engine is generally
to deserialize a previously built DFA or to hand assemble an NFA using its
[builder API](nfa::thompson::Builder). Once you have an NFA, you can build any
of the regex engines in this crate. The `syntax` feature also enables `alloc`.
* **meta** - Enables the meta regex engine. This also enables the `syntax` and
`nfa-pikevm` features, as both are the minimal requirements needed. The meta
regex engine benefits from enabling any of the other regex engines and will
use them automatically when appropriate.
* **nfa** - Enables all NFA related features below.
    * **nfa-thompson** - Enables the Thompson NFA APIs. This enables `alloc`.
    * **nfa-pikevm** - Enables the PikeVM regex engine. This enables
    `nfa-thompson`.
    * **nfa-backtrack** - Enables the bounded backtracker regex engine. This
    enables `nfa-thompson`.
* **dfa** - Enables all DFA related features below.
    * **dfa-build** - Enables APIs for determinizing DFAs from NFAs. This
    enables `nfa-thompson` and `dfa-search`.
    * **dfa-search** - Enables APIs for searching with DFAs.
    * **dfa-onepass** - Enables the one-pass DFA API. This enables
    `nfa-thompson`.
* **hybrid** - Enables the hybrid NFA/DFA or "lazy DFA" regex engine. This
enables `alloc` and `nfa-thompson`.

## Modules

## Module `dfa`

**Attributes:**

- `#[<cfg>(any(feature = "dfa-search", feature = "dfa-onepass"))]`

A module for building and searching with deterministic finite automata (DFAs).

Like other modules in this crate, DFAs support a rich regex syntax with Unicode
features. DFAs also have extensive options for configuring the best space vs
time trade off for your use case and provides support for cheap deserialization
of automata for use in `no_std` environments.

If you're looking for lazy DFAs that build themselves incrementally during
search, then please see the top-level [`hybrid` module](crate::hybrid).

# Overview

This section gives a brief overview of the primary types in this module:

* A [`regex::Regex`] provides a way to search for matches of a regular
expression using DFAs. This includes iterating over matches with both the start
and end positions of each match.
* A [`dense::DFA`] provides low level access to a DFA that uses a dense
representation (uses lots of space, but fast searching).
* A [`sparse::DFA`] provides the same API as a `dense::DFA`, but uses a sparse
representation (uses less space, but slower searching).
* An [`Automaton`] trait that defines an interface that both dense and sparse
DFAs implement. (A `regex::Regex` is generic over this trait.)
* Both dense DFAs and sparse DFAs support serialization to raw bytes (e.g.,
[`dense::DFA::to_bytes_little_endian`]) and cheap deserialization (e.g.,
[`dense::DFA::from_bytes`]).

There is also a [`onepass`] module that provides a [one-pass
DFA](onepass::DFA). The unique advantage of this DFA is that, for the class
of regexes it can be built with, it supports reporting the spans of matching
capturing groups. It is the only DFA in this crate capable of such a thing.

# Example: basic regex searching

This example shows how to compile a regex using the default configuration
and then use it to find matches in a byte string:

```
use regex_automata::{Match, dfa::regex::Regex};

let re = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}")?;
let text = b"2018-12-24 2016-10-08";
let matches: Vec<Match> = re.find_iter(text).collect();
assert_eq!(matches, vec![
    Match::must(0, 0..10),
    Match::must(0, 11..21),
]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: searching with regex sets

The DFAs in this module all fully support searching with multiple regexes
simultaneously. You can use this support with standard leftmost-first style
searching to find non-overlapping matches:

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{Match, dfa::regex::Regex};

let re = Regex::new_many(&[r"\w+", r"\S+"])?;
let text = b"@foo bar";
let matches: Vec<Match> = re.find_iter(text).collect();
assert_eq!(matches, vec![
    Match::must(1, 0..4),
    Match::must(0, 5..8),
]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: use sparse DFAs

By default, compiling a regex will use dense DFAs internally. This uses more
memory, but executes searches more quickly. If you can abide slower searches
(somewhere around 3-5x), then sparse DFAs might make more sense since they can
use significantly less space.

Using sparse DFAs is as easy as using `Regex::new_sparse` instead of
`Regex::new`:

```
use regex_automata::{Match, dfa::regex::Regex};

let re = Regex::new_sparse(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
let text = b"2018-12-24 2016-10-08";
let matches: Vec<Match> = re.find_iter(text).collect();
assert_eq!(matches, vec![
    Match::must(0, 0..10),
    Match::must(0, 11..21),
]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

If you already have dense DFAs for some reason, they can be converted to sparse
DFAs and used to build a new `Regex`. For example:

```
use regex_automata::{Match, dfa::regex::Regex};

let dense_re = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
let sparse_re = Regex::builder().build_from_dfas(
    dense_re.forward().to_sparse()?,
    dense_re.reverse().to_sparse()?,
);
let text = b"2018-12-24 2016-10-08";
let matches: Vec<Match> = sparse_re.find_iter(text).collect();
assert_eq!(matches, vec![
    Match::must(0, 0..10),
    Match::must(0, 11..21),
]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: deserialize a DFA

This shows how to first serialize a DFA into raw bytes, and then deserialize
those raw bytes back into a DFA. While this particular example is a
bit contrived, this same technique can be used in your program to
deserialize a DFA at start up time or by memory mapping a file.

```
use regex_automata::{Match, dfa::{dense, regex::Regex}};

let re1 = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
// serialize both the forward and reverse DFAs, see note below
let (fwd_bytes, fwd_pad) = re1.forward().to_bytes_native_endian();
let (rev_bytes, rev_pad) = re1.reverse().to_bytes_native_endian();
// now deserialize both---we need to specify the correct type!
let fwd: dense::DFA<&[u32]> = dense::DFA::from_bytes(&fwd_bytes[fwd_pad..])?.0;
let rev: dense::DFA<&[u32]> = dense::DFA::from_bytes(&rev_bytes[rev_pad..])?.0;
// finally, reconstruct our regex
let re2 = Regex::builder().build_from_dfas(fwd, rev);

// we can use it like normal
let text = b"2018-12-24 2016-10-08";
let matches: Vec<Match> = re2.find_iter(text).collect();
assert_eq!(matches, vec![
    Match::must(0, 0..10),
    Match::must(0, 11..21),
]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

There are a few points worth noting here:

* We need to extract the raw DFAs used by the regex and serialize those. You
can build the DFAs manually yourself using [`dense::Builder`], but using
the DFAs from a `Regex` guarantees that the DFAs are built correctly. (In
particular, a `Regex` constructs a reverse DFA for finding the starting
location of matches.)
* To convert the DFA to raw bytes, we use the `to_bytes_native_endian` method.
In practice, you'll want to use either [`dense::DFA::to_bytes_little_endian`]
or [`dense::DFA::to_bytes_big_endian`], depending on which platform you're
deserializing your DFA from. If you intend to deserialize on either platform,
then you'll need to serialize both and deserialize the right one depending on
your target's endianness.
* Safely deserializing a DFA requires verifying the raw bytes, particularly if
they are untrusted, since an invalid DFA could cause logical errors, panics
or even undefined behavior. This verification step requires visiting all of
the transitions in the DFA, which can be costly. If cheaper verification is
desired, then [`dense::DFA::from_bytes_unchecked`] is available that only does
verification that can be performed in constant time. However, one can only use
this routine if the caller can guarantee that the bytes provided encoded a
valid DFA.

The same process can be achieved with sparse DFAs as well:

```
use regex_automata::{Match, dfa::{sparse, regex::Regex}};

let re1 = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
// serialize both
let fwd_bytes = re1.forward().to_sparse()?.to_bytes_native_endian();
let rev_bytes = re1.reverse().to_sparse()?.to_bytes_native_endian();
// now deserialize both---we need to specify the correct type!
let fwd: sparse::DFA<&[u8]> = sparse::DFA::from_bytes(&fwd_bytes)?.0;
let rev: sparse::DFA<&[u8]> = sparse::DFA::from_bytes(&rev_bytes)?.0;
// finally, reconstruct our regex
let re2 = Regex::builder().build_from_dfas(fwd, rev);

// we can use it like normal
let text = b"2018-12-24 2016-10-08";
let matches: Vec<Match> = re2.find_iter(text).collect();
assert_eq!(matches, vec![
    Match::must(0, 0..10),
    Match::must(0, 11..21),
]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Note that unlike dense DFAs, sparse DFAs have no alignment requirements.
Conversely, dense DFAs must be aligned to the same alignment as a
[`StateID`](crate::util::primitives::StateID).

# Support for `no_std` and `alloc`-only

This crate comes with `alloc` and `std` features that are enabled by default.
When the `alloc` or `std` features are enabled, the API of this module will
include the facilities necessary for compiling, serializing, deserializing
and searching with DFAs. When only the `alloc` feature is enabled, then
implementations of the `std::error::Error` trait are dropped, but everything
else generally remains the same. When both the `alloc` and `std` features are
disabled, the API of this module will shrink such that it only includes the
facilities necessary for deserializing and searching with DFAs.

The intended workflow for `no_std` environments is thus as follows:

* Write a program with the `alloc` or `std` features that compiles and
serializes a regular expression. You may need to serialize both little and big
endian versions of each DFA. (So that's 4 DFAs in total for each regex.)
* In your `no_std` environment, follow the examples above for deserializing
your previously serialized DFAs into regexes. You can then search with them as
you would any regex.

Deserialization can happen anywhere. For example, with bytes embedded into a
binary or with a file memory mapped at runtime.

The `regex-cli` command (found in the same repository as this crate) can be
used to serialize DFAs to files and generate Rust code to read them.

# Syntax

This module supports the same syntax as the `regex` crate, since they share the
same parser. You can find an exhaustive list of supported syntax in the
[documentation for the `regex` crate](https://docs.rs/regex/1/regex/#syntax).

There are two things that are not supported by the DFAs in this module:

* Capturing groups. The DFAs (and [`Regex`](regex::Regex)es built on top
of them) can only find the offsets of an entire match, but cannot resolve
the offsets of each capturing group. This is because DFAs do not have the
expressive power necessary.
* Unicode word boundaries. These present particularly difficult challenges for
DFA construction and would result in an explosion in the number of states.
One can enable [`dense::Config::unicode_word_boundary`] though, which provides
heuristic support for Unicode word boundaries that only works on ASCII text.
Otherwise, one can use `(?-u:\b)` for an ASCII word boundary, which will work
on any input.

There are no plans to lift either of these limitations.

Note that these restrictions are identical to the restrictions on lazy DFAs.

# Differences with general purpose regexes

The main goal of the [`regex`](https://docs.rs/regex) crate is to serve as a
general purpose regular expression engine. It aims to automatically balance low
compile times, fast search times and low memory usage, while also providing
a convenient API for users. In contrast, this module provides a lower level
regular expression interface based exclusively on DFAs that is a bit less
convenient while providing more explicit control over memory usage and search
times.

Here are some specific negative differences:

* **Compilation can take an exponential amount of time and space** in the size
of the regex pattern. While most patterns do not exhibit worst case exponential
time, such patterns do exist. For example, `[01]*1[01]{N}` will build a DFA
with approximately `2^(N+2)` states. For this reason, untrusted patterns should
not be compiled with this module. (In the future, the API may expose an option
to return an error if the DFA gets too big.)
* This module does not support sub-match extraction via capturing groups, which
can be achieved with the regex crate's "captures" API.
* While the regex crate doesn't necessarily sport fast compilation times,
the regexes in this module are almost universally slow to compile, especially
when they contain large Unicode character classes. For example, on my system,
compiling `\w{50}` takes about 1 second and almost 15MB of memory! (Compiling
a sparse regex takes about the same time but only uses about 1.2MB of
memory.) Conversely, compiling the same regex without Unicode support, e.g.,
`(?-u)\w{50}`, takes under 1 millisecond and about 15KB of memory. For this
reason, you should only use Unicode character classes if you absolutely need
them! (They are enabled by default though.)
* This module does not support Unicode word boundaries. ASCII word bondaries
may be used though by disabling Unicode or selectively doing so in the syntax,
e.g., `(?-u:\b)`. There is also an option to
[heuristically enable Unicode word boundaries](crate::dfa::dense::Config::unicode_word_boundary),
where the corresponding DFA will give up if any non-ASCII byte is seen.
* As a lower level API, this module does not do literal optimizations
automatically. Although it does provide hooks in its API to make use of the
[`Prefilter`](crate::util::prefilter::Prefilter) trait. Missing literal
optimizations means that searches may run much slower than what you're
accustomed to, although, it does provide more predictable and consistent
performance.
* There is no `&str` API like in the regex crate. In this module, all APIs
operate on `&[u8]`. By default, match indices are
guaranteed to fall on UTF-8 boundaries, unless either of
[`syntax::Config::utf8`](crate::util::syntax::Config::utf8) or
[`thompson::Config::utf8`](crate::nfa::thompson::Config::utf8) are disabled.

With some of the downsides out of the way, here are some positive differences:

* Both dense and sparse DFAs can be serialized to raw bytes, and then cheaply
deserialized. Deserialization can be done in constant time with the unchecked
APIs, since searching can be performed directly on the raw serialized bytes of
a DFA.
* This module was specifically designed so that the searching phase of a
DFA has minimal runtime requirements, and can therefore be used in `no_std`
environments. While `no_std` environments cannot compile regexes, they can
deserialize pre-compiled regexes.
* Since this module builds DFAs ahead of time, it will generally out-perform
the `regex` crate on equivalent tasks. The performance difference is likely
not large. However, because of a complex set of optimizations in the regex
crate (like literal optimizations), an accurate performance comparison may be
difficult to do.
* Sparse DFAs provide a way to build a DFA ahead of time that sacrifices search
performance a small amount, but uses much less storage space. Potentially even
less than what the regex crate uses.
* This module exposes DFAs directly, such as [`dense::DFA`] and
[`sparse::DFA`], which enables one to do less work in some cases. For example,
if you only need the end of a match and not the start of a match, then you can
use a DFA directly without building a `Regex`, which always requires a second
DFA to find the start of a match.
* This module provides more control over memory usage. Aside from choosing
between dense and sparse DFAs, one can also choose a smaller state identifier
representation to use less space. Also, one can enable DFA minimization
via [`dense::Config::minimize`], but it can increase compilation times
dramatically.

```rust
pub mod dfa { /* ... */ }
```

### Modules

## Module `onepass`

**Attributes:**

- `#[<cfg>(feature = "dfa-onepass")]`

A DFA that can return spans for matching capturing groups.

This module is the home of a [one-pass DFA](DFA).

This module also contains a [`Builder`] and a [`Config`] for building and
configuring a one-pass DFA.

```rust
pub mod onepass { /* ... */ }
```

### Types

#### Struct `Config`

The configuration used for building a [one-pass DFA](DFA).

A one-pass DFA configuration is a simple data object that is typically used
with [`Builder::configure`]. It can be cheaply cloned.

A default configuration can be created either with `Config::new`, or
perhaps more conveniently, with [`DFA::config`].

```rust
pub struct Config {
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
  pub fn new() -> Config { /* ... */ }
  ```
  Return a new default one-pass DFA configuration.

- ```rust
  pub fn match_kind(self: Self, kind: MatchKind) -> Config { /* ... */ }
  ```
  Set the desired match semantics.

- ```rust
  pub fn starts_for_each_pattern(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Whether to compile a separate start state for each pattern in the

- ```rust
  pub fn byte_classes(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Whether to attempt to shrink the size of the DFA's alphabet or not.

- ```rust
  pub fn size_limit(self: Self, limit: Option<usize>) -> Config { /* ... */ }
  ```
  Set a size limit on the total heap used by a one-pass DFA.

- ```rust
  pub fn get_match_kind(self: &Self) -> MatchKind { /* ... */ }
  ```
  Returns the match semantics set in this configuration.

- ```rust
  pub fn get_starts_for_each_pattern(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether this configuration has enabled anchored starting states

- ```rust
  pub fn get_byte_classes(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether this configuration has enabled byte classes or not.

- ```rust
  pub fn get_size_limit(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns the DFA size limit of this configuration if one was set.

###### Trait Implementations

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Config { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Config { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `Builder`

A builder for a [one-pass DFA](DFA).

This builder permits configuring options for the syntax of a pattern, the
NFA construction and the DFA construction. This builder is different from a
general purpose regex builder in that it permits fine grain configuration
of the construction process. The trade off for this is complexity, and
the possibility of setting a configuration that might not make sense. For
example, there are two different UTF-8 modes:

* [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) controls
whether the pattern itself can contain sub-expressions that match invalid
UTF-8.
* [`thompson::Config::utf8`] controls whether empty matches that split a
Unicode codepoint are reported or not.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the NFA.
This is generally what you want for matching on arbitrary bytes.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    dfa::onepass::DFA,
    nfa::thompson,
    util::syntax,
    Match,
};

let re = DFA::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

let haystack = b"foo\xFFarzz\xE2\x98\xFF\n";
re.captures(&mut cache, haystack, &mut caps);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
//
// N.B. This example does not show the impact of
// disabling UTF-8 mode on a one-pass DFA Config,
//  since that only impacts regexes that can
// produce matches of length 0.
assert_eq!(Some(Match::must(0, 0..8)), caps.get_match());

# Ok::<(), Box<dyn std::error::Error>>(())
```

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
  Create a new one-pass DFA builder with the default configuration.

- ```rust
  pub fn build(self: &Self, pattern: &str) -> Result<DFA, BuildError> { /* ... */ }
  ```
  Build a one-pass DFA from the given pattern.

- ```rust
  pub fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<DFA, BuildError> { /* ... */ }
  ```
  Build a one-pass DFA from the given patterns.

- ```rust
  pub fn build_from_nfa(self: &Self, nfa: NFA) -> Result<DFA, BuildError> { /* ... */ }
  ```
  Build a DFA from the given NFA.

- ```rust
  pub fn configure(self: &mut Self, config: Config) -> &mut Builder { /* ... */ }
  ```
  Apply the given one-pass DFA configuration options to this builder.

- ```rust
  pub fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder { /* ... */ }
  ```
  Set the syntax configuration for this builder using

- ```rust
  pub fn thompson(self: &mut Self, config: thompson::Config) -> &mut Builder { /* ... */ }
  ```
  Set the Thompson NFA configuration for this builder using

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Builder { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
#### Struct `DFA`

A one-pass DFA for executing a subset of anchored regex searches while
resolving capturing groups.

A one-pass DFA can be built from an NFA that is one-pass. An NFA is
one-pass when there is never any ambiguity about how to continue a search.
For example, `a*a` is not one-pass becuase during a search, it's not
possible to know whether to continue matching the `a*` or to move on to
the single `a`. However, `a*b` is one-pass, because for every byte in the
input, it's always clear when to move on from `a*` to `b`.

# Only anchored searches are supported

In this crate, especially for DFAs, unanchored searches are implemented by
treating the pattern as if it had a `(?s-u:.)*?` prefix. While the prefix
is one-pass on its own, adding anything after it, e.g., `(?s-u:.)*?a` will
make the overall pattern not one-pass. Why? Because the `(?s-u:.)` matches
any byte, and there is therefore ambiguity as to when the prefix should
stop matching and something else should start matching.

Therefore, one-pass DFAs do not support unanchored searches. In addition
to many regexes simply not being one-pass, it implies that one-pass DFAs
have limited utility. With that said, when a one-pass DFA can be used, it
can potentially provide a dramatic speed up over alternatives like the
[`BoundedBacktracker`](crate::nfa::thompson::backtrack::BoundedBacktracker)
and the [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM). In particular,
a one-pass DFA is the only DFA capable of reporting the spans of matching
capturing groups.

To clarify, when we say that unanchored searches are not supported, what
that actually means is:

* The high level routines, [`DFA::is_match`] and [`DFA::captures`], always
do anchored searches.
* Since iterators are most useful in the context of unanchored searches,
there is no `DFA::captures_iter` method.
* For lower level routines like [`DFA::try_search`], an error will be
returned if the given [`Input`] is configured to do an unanchored search or
search for an invalid pattern ID. (Note that an [`Input`] is configured to
do an unanchored search by default, so just giving a `Input::new` is
guaranteed to return an error.)

# Other limitations

In addition to the [configurable heap limit](Config::size_limit) and
the requirement that a regex pattern be one-pass, there are some other
limitations:

* There is an internal limit on the total number of explicit capturing
groups that appear across all patterns. It is somewhat small and there is
no way to configure it. If your pattern(s) exceed this limit, then building
a one-pass DFA will fail.
* If the number of patterns exceeds an internal unconfigurable limit, then
building a one-pass DFA will fail. This limit is quite large and you're
unlikely to hit it.
* If the total number of states exceeds an internal unconfigurable limit,
then building a one-pass DFA will fail. This limit is quite large and
you're unlikely to hit it.

# Other examples of regexes that aren't one-pass

One particularly unfortunate example is that enabling Unicode can cause
regexes that were one-pass to no longer be one-pass. Consider the regex
`(?-u)\w*\s` for example. It is one-pass because there is exactly no
overlap between the ASCII definitions of `\w` and `\s`. But `\w*\s`
(i.e., with Unicode enabled) is *not* one-pass because `\w` and `\s` get
translated to UTF-8 automatons. And while the *codepoints* in `\w` and `\s`
do not overlap, the underlying UTF-8 encodings do. Indeed, because of the
overlap between UTF-8 automata, the use of Unicode character classes will
tend to vastly increase the likelihood of a regex not being one-pass.

# How does one know if a regex is one-pass or not?

At the time of writing, the only way to know is to try and build a one-pass
DFA. The one-pass property is checked while constructing the DFA.

This does mean that you might potentially waste some CPU cycles and memory
by optimistically trying to build a one-pass DFA. But this is currently the
only way. In the future, building a one-pass DFA might be able to use some
heuristics to detect common violations of the one-pass property and bail
more quickly.

# Resource usage

Unlike a general DFA, a one-pass DFA has stricter bounds on its resource
usage. Namely, construction of a one-pass DFA has a time and space
complexity of `O(n)`, where `n ~ nfa.states().len()`. (A general DFA's time
and space complexity is `O(2^n)`.) This smaller time bound is achieved
because there is at most one DFA state created for each NFA state. If
additional DFA states would be required, then the pattern is not one-pass
and construction will fail.

Note though that currently, this DFA uses a fully dense representation.
This means that while its space complexity is no worse than an NFA, it may
in practice use more memory because of higher constant factors. The reason
for this trade off is two-fold. Firstly, a dense representation makes the
search faster. Secondly, the bigger an NFA, the more unlikely it is to be
one-pass. Therefore, most one-pass DFAs are usually pretty small.

# Example

This example shows that the one-pass DFA implements Unicode word boundaries
correctly while simultaneously reporting spans for capturing groups that
participate in a match. (This is the only DFA that implements full support
for Unicode word boundaries.)

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{dfa::onepass::DFA, Match, Span};

let re = DFA::new(r"\b(?P<first>\w+)[[:space:]]+(?P<last>\w+)\b")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

re.captures(&mut cache, "Шерлок Холмс", &mut caps);
assert_eq!(Some(Match::must(0, 0..23)), caps.get_match());
assert_eq!(Some(Span::from(0..12)), caps.get_group_by_name("first"));
assert_eq!(Some(Span::from(13..23)), caps.get_group_by_name("last"));
# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: iteration

Unlike other regex engines in this crate, this one does not provide
iterator search functions. This is because a one-pass DFA only supports
anchored searches, and so iterator functions are generally not applicable.

However, if you know that all of your matches are
directly adjacent, then an iterator can be used. The
[`util::iter::Searcher`](crate::util::iter::Searcher) type can be used for
this purpose:

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    dfa::onepass::DFA,
    util::iter::Searcher,
    Anchored, Input, Span,
};

let re = DFA::new(r"\w(\d)\w")?;
let (mut cache, caps) = (re.create_cache(), re.create_captures());
let input = Input::new("a1zb2yc3x").anchored(Anchored::Yes);

let mut it = Searcher::new(input).into_captures_iter(caps, |input, caps| {
    Ok(re.try_search(&mut cache, input, caps)?)
}).infallible();
let caps0 = it.next().unwrap();
assert_eq!(Some(Span::from(1..2)), caps0.get_group(1));

# Ok::<(), Box<dyn std::error::Error>>(())
```

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
  pub fn new(pattern: &str) -> Result<DFA, BuildError> { /* ... */ }
  ```
  Parse the given regular expression using the default configuration and

- ```rust
  pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<DFA, BuildError> { /* ... */ }
  ```
  Like `new`, but parses multiple patterns into a single "multi regex."

- ```rust
  pub fn new_from_nfa(nfa: NFA) -> Result<DFA, BuildError> { /* ... */ }
  ```
  Like `new`, but builds a one-pass DFA directly from an NFA. This is

- ```rust
  pub fn always_match() -> Result<DFA, BuildError> { /* ... */ }
  ```
  Create a new one-pass DFA that matches every input.

- ```rust
  pub fn never_match() -> Result<DFA, BuildError> { /* ... */ }
  ```
  Create a new one-pass DFA that never matches any input.

- ```rust
  pub fn config() -> Config { /* ... */ }
  ```
  Return a default configuration for a DFA.

- ```rust
  pub fn builder() -> Builder { /* ... */ }
  ```
  Return a builder for configuring the construction of a DFA.

- ```rust
  pub fn create_captures(self: &Self) -> Captures { /* ... */ }
  ```
  Create a new empty set of capturing groups that is guaranteed to be

- ```rust
  pub fn create_cache(self: &Self) -> Cache { /* ... */ }
  ```
  Create a new cache for this DFA.

- ```rust
  pub fn reset_cache(self: &Self, cache: &mut Cache) { /* ... */ }
  ```
  Reset the given cache such that it can be used for searching with the

- ```rust
  pub fn get_config(self: &Self) -> &Config { /* ... */ }
  ```
  Return the config for this one-pass DFA.

- ```rust
  pub fn get_nfa(self: &Self) -> &NFA { /* ... */ }
  ```
  Returns a reference to the underlying NFA.

- ```rust
  pub fn pattern_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of patterns compiled into this DFA.

- ```rust
  pub fn state_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of states in this one-pass DFA.

- ```rust
  pub fn alphabet_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of elements in the alphabet for this DFA.

- ```rust
  pub fn stride2(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total stride for every state in this DFA, expressed as the

- ```rust
  pub fn stride(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total stride for every state in this DFA. This corresponds

- ```rust
  pub fn memory_usage(self: &Self) -> usize { /* ... */ }
  ```
  Returns the memory usage, in bytes, of this DFA.

- ```rust
  pub fn is_match<''h, I: Into<Input<''h>>>(self: &Self, cache: &mut Cache, input: I) -> bool { /* ... */ }
  ```
  Executes an anchored leftmost forward search, and returns true if and

- ```rust
  pub fn find<''h, I: Into<Input<''h>>>(self: &Self, cache: &mut Cache, input: I) -> Option<Match> { /* ... */ }
  ```
  Executes an anchored leftmost forward search, and returns a `Match` if

- ```rust
  pub fn captures<''h, I: Into<Input<''h>>>(self: &Self, cache: &mut Cache, input: I, caps: &mut Captures) { /* ... */ }
  ```
  Executes an anchored leftmost forward search and writes the spans

- ```rust
  pub fn try_search(self: &Self, cache: &mut Cache, input: &Input<''_>, caps: &mut Captures) -> Result<(), MatchError> { /* ... */ }
  ```
  Executes an anchored leftmost forward search and writes the spans

- ```rust
  pub fn try_search_slots(self: &Self, cache: &mut Cache, input: &Input<''_>, slots: &mut [Option<NonMaxUsize>]) -> Result<Option<PatternID>, MatchError> { /* ... */ }
  ```
  Executes an anchored leftmost forward search and writes the spans

###### Trait Implementations

- **Sync**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DFA { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Cache`

A cache represents mutable state that a one-pass [`DFA`] requires during a
search.

For a given one-pass DFA, its corresponding cache may be created either via
[`DFA::create_cache`], or via [`Cache::new`]. They are equivalent in every
way, except the former does not require explicitly importing `Cache`.

A particular `Cache` is coupled with the one-pass DFA from which it was
created. It may only be used with that one-pass DFA. A cache and its
allocations may be re-purposed via [`Cache::reset`], in which case, it can
only be used with the new one-pass DFA (and not the old one).

```rust
pub struct Cache {
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
  pub fn new(re: &DFA) -> Cache { /* ... */ }
  ```
  Create a new [`onepass::DFA`](DFA) cache.

- ```rust
  pub fn reset(self: &mut Self, re: &DFA) { /* ... */ }
  ```
  Reset this cache such that it can be used for searching with a

- ```rust
  pub fn memory_usage(self: &Self) -> usize { /* ... */ }
  ```
  Returns the heap memory usage, in bytes, of this cache.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Cache { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `BuildError`

An error that occurred during the construction of a one-pass DFA.

This error does not provide many introspection capabilities. There are
generally only two things you can do with it:

* Obtain a human readable message via its `std::fmt::Display` impl.
* Access an underlying [`thompson::BuildError`] type from its `source`
method via the `std::error::Error` trait. This error only occurs when using
convenience routines for building a one-pass DFA directly from a pattern
string.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

```rust
pub struct BuildError {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> BuildError { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn std::error::Error + ''static> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Module `hybrid`

**Attributes:**

- `#[<cfg>(feature = "hybrid")]`

A module for building and searching with lazy deterministic finite automata
(DFAs).

Like other modules in this crate, lazy DFAs support a rich regex syntax with
Unicode features. The key feature of a lazy DFA is that it builds itself
incrementally during search, and never uses more than a configured capacity of
memory. Thus, when searching with a lazy DFA, one must supply a mutable "cache"
in which the actual DFA's transition table is stored.

If you're looking for fully compiled DFAs, then please see the top-level
[`dfa` module](crate::dfa).

# Overview

This section gives a brief overview of the primary types in this module:

* A [`regex::Regex`] provides a way to search for matches of a regular
expression using lazy DFAs. This includes iterating over matches with both the
start and end positions of each match.
* A [`dfa::DFA`] provides direct low level access to a lazy DFA.

# Example: basic regex searching

This example shows how to compile a regex using the default configuration
and then use it to find matches in a byte string:

```
use regex_automata::{hybrid::regex::Regex, Match};

let re = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}")?;
let mut cache = re.create_cache();

let haystack = "2018-12-24 2016-10-08";
let matches: Vec<Match> = re.find_iter(&mut cache, haystack).collect();
assert_eq!(matches, vec![
    Match::must(0, 0..10),
    Match::must(0, 11..21),
]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: searching with multiple regexes

The lazy DFAs in this module all fully support searching with multiple regexes
simultaneously. You can use this support with standard leftmost-first style
searching to find non-overlapping matches:

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{hybrid::regex::Regex, Match};

let re = Regex::new_many(&[r"\w+", r"\S+"])?;
let mut cache = re.create_cache();

let haystack = "@foo bar";
let matches: Vec<Match> = re.find_iter(&mut cache, haystack).collect();
assert_eq!(matches, vec![
    Match::must(1, 0..4),
    Match::must(0, 5..8),
]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

# When should I use this?

Generally speaking, if you can abide the use of mutable state during search,
and you don't need things like capturing groups or Unicode word boundary
support in non-ASCII text, then a lazy DFA is likely a robust choice with
respect to both search speed and memory usage. Note however that its speed
may be worse than a general purpose regex engine if you don't select a good
[prefilter](crate::util::prefilter).

If you know ahead of time that your pattern would result in a very large DFA
if it was fully compiled, it may be better to use an NFA simulation instead
of a lazy DFA. Either that, or increase the cache capacity of your lazy DFA
to something that is big enough to hold the state machine (likely through
experimentation). The issue here is that if the cache is too small, then it
could wind up being reset too frequently and this might decrease searching
speed significantly.

# Differences with fully compiled DFAs

A [`hybrid::regex::Regex`](crate::hybrid::regex::Regex) and a
[`dfa::regex::Regex`](crate::dfa::regex::Regex) both have the same capabilities
(and similarly for their underlying DFAs), but they achieve them through
different means. The main difference is that a hybrid or "lazy" regex builds
its DFA lazily during search, where as a fully compiled regex will build its
DFA at construction time. While building a DFA at search time might sound like
it's slow, it tends to work out where most bytes seen during a search will
reuse pre-built parts of the DFA and thus can be almost as fast as a fully
compiled DFA. The main downside is that searching requires mutable space to
store the DFA, and, in the worst case, a search can result in a new state being
created for each byte seen, which would make searching quite a bit slower.

A fully compiled DFA never has to worry about searches being slower once
it's built. (Aside from, say, the transition table being so large that it
is subject to harsh CPU cache effects.) However, of course, building a full
DFA can be quite time consuming and memory hungry. Particularly when large
Unicode character classes are used, which tend to translate into very large
DFAs.

A lazy DFA strikes a nice balance _in practice_, particularly in the
presence of Unicode mode, by only building what is needed. It avoids the
worst case exponential time complexity of DFA compilation by guaranteeing that
it will only build at most one state per byte searched. While the worst
case here can lead to a very high constant, it will never be exponential.

# Syntax

This module supports the same syntax as the `regex` crate, since they share the
same parser. You can find an exhaustive list of supported syntax in the
[documentation for the `regex` crate](https://docs.rs/regex/1/regex/#syntax).

There are two things that are not supported by the lazy DFAs in this module:

* Capturing groups. The DFAs (and [`Regex`](regex::Regex)es built on top
of them) can only find the offsets of an entire match, but cannot resolve
the offsets of each capturing group. This is because DFAs do not have the
expressive power necessary. Note that it is okay to build a lazy DFA from an
NFA that contains capture groups. The capture groups will simply be ignored.
* Unicode word boundaries. These present particularly difficult challenges for
DFA construction and would result in an explosion in the number of states.
One can enable [`dfa::Config::unicode_word_boundary`] though, which provides
heuristic support for Unicode word boundaries that only works on ASCII text.
Otherwise, one can use `(?-u:\b)` for an ASCII word boundary, which will work
on any input.

There are no plans to lift either of these limitations.

Note that these restrictions are identical to the restrictions on fully
compiled DFAs.

```rust
pub mod hybrid { /* ... */ }
```

### Modules

## Module `dfa`

Types and routines specific to lazy DFAs.

This module is the home of [`hybrid::dfa::DFA`](DFA).

This module also contains a [`hybrid::dfa::Builder`](Builder) and a
[`hybrid::dfa::Config`](Config) for configuring and building a lazy DFA.

```rust
pub mod dfa { /* ... */ }
```

### Types

#### Struct `DFA`

A hybrid NFA/DFA (also called a "lazy DFA") for regex searching.

A lazy DFA is a DFA that builds itself at search time. It otherwise has
very similar characteristics as a [`dense::DFA`](crate::dfa::dense::DFA).
Indeed, both support precisely the same regex features with precisely the
same semantics.

Where as a `dense::DFA` must be completely built to handle any input before
it may be used for search, a lazy DFA starts off effectively empty. During
a search, a lazy DFA will build itself depending on whether it has already
computed the next transition or not. If it has, then it looks a lot like
a `dense::DFA` internally: it does a very fast table based access to find
the next transition. Otherwise, if the state hasn't been computed, then it
does determinization _for that specific transition_ to compute the next DFA
state.

The main selling point of a lazy DFA is that, in practice, it has
the performance profile of a `dense::DFA` without the weakness of it
taking worst case exponential time to build. Indeed, for each byte of
input, the lazy DFA will construct as most one new DFA state. Thus, a
lazy DFA achieves worst case `O(mn)` time for regex search (where `m ~
pattern.len()` and `n ~ haystack.len()`).

The main downsides of a lazy DFA are:

1. It requires mutable "cache" space during search. This is where the
transition table, among other things, is stored.
2. In pathological cases (e.g., if the cache is too small), it will run
out of room and either require a bigger cache capacity or will repeatedly
clear the cache and thus repeatedly regenerate DFA states. Overall, this
will tend to be slower than a typical NFA simulation.

# Capabilities

Like a `dense::DFA`, a single lazy DFA fundamentally supports the following
operations:

1. Detection of a match.
2. Location of the end of a match.
3. In the case of a lazy DFA with multiple patterns, which pattern matched
is reported as well.

A notable absence from the above list of capabilities is the location of
the *start* of a match. In order to provide both the start and end of
a match, *two* lazy DFAs are required. This functionality is provided by a
[`Regex`](crate::hybrid::regex::Regex).

# Example

This shows how to build a lazy DFA with the default configuration and
execute a search. Notice how, in contrast to a `dense::DFA`, we must create
a cache and pass it to our search routine.

```
use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};

let dfa = DFA::new("foo[0-9]+")?;
let mut cache = dfa.create_cache();

let expected = Some(HalfMatch::must(0, 8));
assert_eq!(expected, dfa.try_search_fwd(
    &mut cache, &Input::new("foo12345"))?,
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

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
  pub fn new(pattern: &str) -> Result<DFA, BuildError> { /* ... */ }
  ```
  Parse the given regular expression using a default configuration and

- ```rust
  pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<DFA, BuildError> { /* ... */ }
  ```
  Parse the given regular expressions using a default configuration and

- ```rust
  pub fn always_match() -> Result<DFA, BuildError> { /* ... */ }
  ```
  Create a new lazy DFA that matches every input.

- ```rust
  pub fn never_match() -> Result<DFA, BuildError> { /* ... */ }
  ```
  Create a new lazy DFA that never matches any input.

- ```rust
  pub fn config() -> Config { /* ... */ }
  ```
  Return a default configuration for a `DFA`.

- ```rust
  pub fn builder() -> Builder { /* ... */ }
  ```
  Return a builder for configuring the construction of a `Regex`.

- ```rust
  pub fn create_cache(self: &Self) -> Cache { /* ... */ }
  ```
  Create a new cache for this lazy DFA.

- ```rust
  pub fn reset_cache(self: &Self, cache: &mut Cache) { /* ... */ }
  ```
  Reset the given cache such that it can be used for searching with the

- ```rust
  pub fn pattern_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of patterns compiled into this lazy DFA.

- ```rust
  pub fn byte_classes(self: &Self) -> &ByteClasses { /* ... */ }
  ```
  Returns the equivalence classes that make up the alphabet for this DFA.

- ```rust
  pub fn get_config(self: &Self) -> &Config { /* ... */ }
  ```
  Returns this lazy DFA's configuration.

- ```rust
  pub fn get_nfa(self: &Self) -> &thompson::NFA { /* ... */ }
  ```
  Returns a reference to the underlying NFA.

- ```rust
  pub fn memory_usage(self: &Self) -> usize { /* ... */ }
  ```
  Returns the memory usage, in bytes, of this lazy DFA.

- ```rust
  pub fn try_search_fwd(self: &Self, cache: &mut Cache, input: &Input<''_>) -> Result<Option<HalfMatch>, MatchError> { /* ... */ }
  ```
  Executes a forward search and returns the end position of the leftmost

- ```rust
  pub fn try_search_rev(self: &Self, cache: &mut Cache, input: &Input<''_>) -> Result<Option<HalfMatch>, MatchError> { /* ... */ }
  ```
  Executes a reverse search and returns the start of the position of the

- ```rust
  pub fn try_search_overlapping_fwd(self: &Self, cache: &mut Cache, input: &Input<''_>, state: &mut OverlappingState) -> Result<(), MatchError> { /* ... */ }
  ```
  Executes an overlapping forward search and returns the end position of

- ```rust
  pub fn try_search_overlapping_rev(self: &Self, cache: &mut Cache, input: &Input<''_>, state: &mut OverlappingState) -> Result<(), MatchError> { /* ... */ }
  ```
  Executes a reverse overlapping search and returns the start of the

- ```rust
  pub fn try_which_overlapping_matches(self: &Self, cache: &mut Cache, input: &Input<''_>, patset: &mut PatternSet) -> Result<(), MatchError> { /* ... */ }
  ```
  Writes the set of patterns that match anywhere in the given search

- ```rust
  pub fn next_state(self: &Self, cache: &mut Cache, current: LazyStateID, input: u8) -> Result<LazyStateID, CacheError> { /* ... */ }
  ```
  Transitions from the current state to the next state, given the next

- ```rust
  pub fn next_state_untagged(self: &Self, cache: &Cache, current: LazyStateID, input: u8) -> LazyStateID { /* ... */ }
  ```
  Transitions from the current state to the next state, given the next

- ```rust
  pub unsafe fn next_state_untagged_unchecked(self: &Self, cache: &Cache, current: LazyStateID, input: u8) -> LazyStateID { /* ... */ }
  ```
  Transitions from the current state to the next state, eliding bounds

- ```rust
  pub fn next_eoi_state(self: &Self, cache: &mut Cache, current: LazyStateID) -> Result<LazyStateID, CacheError> { /* ... */ }
  ```
  Transitions from the current state to the next state for the special

- ```rust
  pub fn start_state(self: &Self, cache: &mut Cache, config: &start::Config) -> Result<LazyStateID, StartError> { /* ... */ }
  ```
  Return the ID of the start state for this lazy DFA for the given

- ```rust
  pub fn start_state_forward(self: &Self, cache: &mut Cache, input: &Input<''_>) -> Result<LazyStateID, MatchError> { /* ... */ }
  ```
  Return the ID of the start state for this lazy DFA when executing a

- ```rust
  pub fn start_state_reverse(self: &Self, cache: &mut Cache, input: &Input<''_>) -> Result<LazyStateID, MatchError> { /* ... */ }
  ```
  Return the ID of the start state for this lazy DFA when executing a

- ```rust
  pub fn match_len(self: &Self, cache: &Cache, id: LazyStateID) -> usize { /* ... */ }
  ```
  Returns the total number of patterns that match in this state.

- ```rust
  pub fn match_pattern(self: &Self, cache: &Cache, id: LazyStateID, match_index: usize) -> PatternID { /* ... */ }
  ```
  Returns the pattern ID corresponding to the given match index in the

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DFA { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

#### Struct `Cache`

A cache represents a partially computed DFA.

A cache is the key component that differentiates a classical DFA and a
hybrid NFA/DFA (also called a "lazy DFA"). Where a classical DFA builds a
complete transition table that can handle all possible inputs, a hybrid
NFA/DFA starts with an empty transition table and builds only the parts
required during search. The parts that are built are stored in a cache. For
this reason, a cache is a required parameter for nearly every operation on
a [`DFA`].

Caches can be created from their corresponding DFA via
[`DFA::create_cache`]. A cache can only be used with either the DFA that
created it, or the DFA that was most recently used to reset it with
[`Cache::reset`]. Using a cache with any other DFA may result in panics
or incorrect results.

```rust
pub struct Cache {
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
  pub fn new(dfa: &DFA) -> Cache { /* ... */ }
  ```
  Create a new cache for the given lazy DFA.

- ```rust
  pub fn reset(self: &mut Self, dfa: &DFA) { /* ... */ }
  ```
  Reset this cache such that it can be used for searching with the given

- ```rust
  pub fn search_start(self: &mut Self, at: usize) { /* ... */ }
  ```
  Initializes a new search starting at the given position.

- ```rust
  pub fn search_update(self: &mut Self, at: usize) { /* ... */ }
  ```
  Updates the current search to indicate that it has search to the

- ```rust
  pub fn search_finish(self: &mut Self, at: usize) { /* ... */ }
  ```
  Indicates that a search has finished at the given position.

- ```rust
  pub fn search_total_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of bytes that have been searched since this

- ```rust
  pub fn clear_count(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of times this cache has been cleared since it

- ```rust
  pub fn memory_usage(self: &Self) -> usize { /* ... */ }
  ```
  Returns the heap memory usage, in bytes, of this cache.

###### Trait Implementations

- **UnwindSafe**
- **Freeze**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Unpin**
- **Send**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Cache { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Config`

The configuration used for building a lazy DFA.

As a convenience, [`DFA::config`] is an alias for [`Config::new`]. The
advantage of the former is that it often lets you avoid importing the
`Config` type directly.

A lazy DFA configuration is a simple data object that is typically used
with [`Builder::configure`].

The default configuration guarantees that a search will never return a
"gave up" or "quit" error, although it is possible for a search to fail
if [`Config::starts_for_each_pattern`] wasn't enabled (which it is not by
default) and an [`Anchored::Pattern`] mode is requested via [`Input`].

```rust
pub struct Config {
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
  pub fn new() -> Config { /* ... */ }
  ```
  Return a new default lazy DFA builder configuration.

- ```rust
  pub fn match_kind(self: Self, kind: MatchKind) -> Config { /* ... */ }
  ```
  Set the desired match semantics.

- ```rust
  pub fn prefilter(self: Self, pre: Option<Prefilter>) -> Config { /* ... */ }
  ```
  Set a prefilter to be used whenever a start state is entered.

- ```rust
  pub fn starts_for_each_pattern(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Whether to compile a separate start state for each pattern in the

- ```rust
  pub fn byte_classes(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Whether to attempt to shrink the size of the lazy DFA's alphabet or

- ```rust
  pub fn unicode_word_boundary(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Heuristically enable Unicode word boundaries.

- ```rust
  pub fn quit(self: Self, byte: u8, yes: bool) -> Config { /* ... */ }
  ```
  Add a "quit" byte to the lazy DFA.

- ```rust
  pub fn specialize_start_states(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Enable specializing start states in the lazy DFA.

- ```rust
  pub fn cache_capacity(self: Self, bytes: usize) -> Config { /* ... */ }
  ```
  Sets the maximum amount of heap memory, in bytes, to allocate to the

- ```rust
  pub fn skip_cache_capacity_check(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Configures construction of a lazy DFA to use the minimum cache capacity

- ```rust
  pub fn minimum_cache_clear_count(self: Self, min: Option<usize>) -> Config { /* ... */ }
  ```
  Configure a lazy DFA search to quit after a certain number of cache

- ```rust
  pub fn minimum_bytes_per_state(self: Self, min: Option<usize>) -> Config { /* ... */ }
  ```
  Configure a lazy DFA search to quit only when its efficiency drops

- ```rust
  pub fn get_match_kind(self: &Self) -> MatchKind { /* ... */ }
  ```
  Returns the match semantics set in this configuration.

- ```rust
  pub fn get_prefilter(self: &Self) -> Option<&Prefilter> { /* ... */ }
  ```
  Returns the prefilter set in this configuration, if one at all.

- ```rust
  pub fn get_starts_for_each_pattern(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether this configuration has enabled anchored starting states

- ```rust
  pub fn get_byte_classes(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether this configuration has enabled byte classes or not.

- ```rust
  pub fn get_unicode_word_boundary(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether this configuration has enabled heuristic Unicode word

- ```rust
  pub fn get_quit(self: &Self, byte: u8) -> bool { /* ... */ }
  ```
  Returns whether this configuration will instruct the lazy DFA to enter

- ```rust
  pub fn get_specialize_start_states(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether this configuration will instruct the lazy DFA to

- ```rust
  pub fn get_cache_capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the cache capacity set on this configuration.

- ```rust
  pub fn get_skip_cache_capacity_check(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the cache capacity check should be skipped.

- ```rust
  pub fn get_minimum_cache_clear_count(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns, if set, the minimum number of times the cache must be cleared

- ```rust
  pub fn get_minimum_bytes_per_state(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Returns, if set, the minimum number of bytes per state that need to be

- ```rust
  pub fn get_minimum_cache_capacity(self: &Self, nfa: &thompson::NFA) -> Result<usize, BuildError> { /* ... */ }
  ```
  Returns the minimum lazy DFA cache capacity required for the given NFA.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **Sync**
- **Freeze**
- **Send**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Config { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Config { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `Builder`

A builder for constructing a lazy deterministic finite automaton from
regular expressions.

As a convenience, [`DFA::builder`] is an alias for [`Builder::new`]. The
advantage of the former is that it often lets you avoid importing the
`Builder` type directly.

This builder provides two main things:

1. It provides a few different `build` routines for actually constructing
a DFA from different kinds of inputs. The most convenient is
[`Builder::build`], which builds a DFA directly from a pattern string. The
most flexible is [`Builder::build_from_nfa`], which builds a DFA straight
from an NFA.
2. The builder permits configuring a number of things.
[`Builder::configure`] is used with [`Config`] to configure aspects of
the DFA and the construction process itself. [`Builder::syntax`] and
[`Builder::thompson`] permit configuring the regex parser and Thompson NFA
construction, respectively. The syntax and thompson configurations only
apply when building from a pattern string.

This builder always constructs a *single* lazy DFA. As such, this builder
can only be used to construct regexes that either detect the presence
of a match or find the end location of a match. A single DFA cannot
produce both the start and end of a match. For that information, use a
[`Regex`](crate::hybrid::regex::Regex), which can be similarly configured
using [`regex::Builder`](crate::hybrid::regex::Builder). The main reason
to use a DFA directly is if the end location of a match is enough for your
use case. Namely, a `Regex` will construct two lazy DFAs instead of one,
since a second reverse DFA is needed to find the start of a match.

# Example

This example shows how to build a lazy DFA that uses a tiny cache capacity
and completely disables Unicode. That is:

* Things such as `\w`, `.` and `\b` are no longer Unicode-aware. `\w`
  and `\b` are ASCII-only while `.` matches any byte except for `\n`
  (instead of any UTF-8 encoding of a Unicode scalar value except for
  `\n`). Things that are Unicode only, such as `\pL`, are not allowed.
* The pattern itself is permitted to match invalid UTF-8. For example,
  things like `[^a]` that match any byte except for `a` are permitted.

```
use regex_automata::{
    hybrid::dfa::DFA,
    nfa::thompson,
    util::syntax,
    HalfMatch, Input,
};

let dfa = DFA::builder()
    .configure(DFA::config().cache_capacity(5_000))
    .thompson(thompson::Config::new().utf8(false))
    .syntax(syntax::Config::new().unicode(false).utf8(false))
    .build(r"foo[^b]ar.*")?;
let mut cache = dfa.create_cache();

let haystack = b"\xFEfoo\xFFar\xE2\x98\xFF\n";
let expected = Some(HalfMatch::must(0, 10));
let got = dfa.try_search_fwd(&mut cache, &Input::new(haystack))?;
assert_eq!(expected, got);

# Ok::<(), Box<dyn std::error::Error>>(())
```

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
  Create a new lazy DFA builder with the default configuration.

- ```rust
  pub fn build(self: &Self, pattern: &str) -> Result<DFA, BuildError> { /* ... */ }
  ```
  Build a lazy DFA from the given pattern.

- ```rust
  pub fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<DFA, BuildError> { /* ... */ }
  ```
  Build a lazy DFA from the given patterns.

- ```rust
  pub fn build_from_nfa(self: &Self, nfa: thompson::NFA) -> Result<DFA, BuildError> { /* ... */ }
  ```
  Build a DFA from the given NFA.

- ```rust
  pub fn configure(self: &mut Self, config: Config) -> &mut Builder { /* ... */ }
  ```
  Apply the given lazy DFA configuration options to this builder.

- ```rust
  pub fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder { /* ... */ }
  ```
  Set the syntax configuration for this builder using

- ```rust
  pub fn thompson(self: &mut Self, config: thompson::Config) -> &mut Builder { /* ... */ }
  ```
  Set the Thompson NFA configuration for this builder using

###### Trait Implementations

- **Freeze**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **UnwindSafe**
- **Send**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Builder { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `OverlappingState`

Represents the current state of an overlapping search.

This is used for overlapping searches since they need to know something
about the previous search. For example, when multiple patterns match at the
same position, this state tracks the last reported pattern so that the next
search knows whether to report another matching pattern or continue with
the search at the next position. Additionally, it also tracks which state
the last search call terminated in.

This type provides little introspection capabilities. The only thing a
caller can do is construct it and pass it around to permit search routines
to use it to track state, and also ask whether a match has been found.

Callers should always provide a fresh state constructed via
[`OverlappingState::start`] when starting a new search. Reusing state from
a previous search may result in incorrect results.

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
  Create a new overlapping state that begins at the start state of any

- ```rust
  pub fn get_match(self: &Self) -> Option<HalfMatch> { /* ... */ }
  ```
  Return the match result of the most recent search to execute with this

###### Trait Implementations

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OverlappingState { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OverlappingState) -> bool { /* ... */ }
    ```

- **Freeze**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **UnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
## Module `regex`

A lazy DFA backed `Regex`.

This module provides a [`Regex`] backed by a lazy DFA. A `Regex` implements
convenience routines you might have come to expect, such as finding a match
and iterating over all non-overlapping matches. This `Regex` type is limited
in its capabilities to what a lazy DFA can provide. Therefore, APIs involving
capturing groups, for example, are not provided.

Internally, a `Regex` is composed of two DFAs. One is a "forward" DFA that
finds the end offset of a match, where as the other is a "reverse" DFA that
find the start offset of a match.

See the [parent module](crate::hybrid) for examples.

```rust
pub mod regex { /* ... */ }
```

### Types

#### Struct `Regex`

A regular expression that uses hybrid NFA/DFAs (also called "lazy DFAs")
for searching.

A regular expression is comprised of two lazy DFAs, a "forward" DFA and a
"reverse" DFA. The forward DFA is responsible for detecting the end of
a match while the reverse DFA is responsible for detecting the start
of a match. Thus, in order to find the bounds of any given match, a
forward search must first be run followed by a reverse search. A match
found by the forward DFA guarantees that the reverse DFA will also find
a match.

# Fallibility

Most of the search routines defined on this type will _panic_ when the
underlying search fails. This might be because the DFA gave up because it
saw a quit byte, whether configured explicitly or via heuristic Unicode
word boundary support, although neither are enabled by default. It might
also fail if the underlying DFA determines it isn't making effective use of
the cache (which also never happens by default). Or it might fail because
an invalid `Input` configuration is given, for example, with an unsupported
[`Anchored`] mode.

If you need to handle these error cases instead of allowing them to trigger
a panic, then the lower level [`Regex::try_search`] provides a fallible API
that never panics.

# Example

This example shows how to cause a search to terminate if it sees a
`\n` byte, and handle the error returned. This could be useful if, for
example, you wanted to prevent a user supplied pattern from matching
across a line boundary.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{hybrid::{dfa, regex::Regex}, Input, MatchError};

let re = Regex::builder()
    .dfa(dfa::Config::new().quit(b'\n', true))
    .build(r"foo\p{any}+bar")?;
let mut cache = re.create_cache();

let input = Input::new("foo\nbar");
// Normally this would produce a match, since \p{any} contains '\n'.
// But since we instructed the automaton to enter a quit state if a
// '\n' is observed, this produces a match error instead.
let expected = MatchError::quit(b'\n', 3);
let got = re.try_search(&mut cache, &input).unwrap_err();
assert_eq!(expected, got);

# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub struct Regex {
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
  pub fn new(pattern: &str) -> Result<Regex, BuildError> { /* ... */ }
  ```
  Parse the given regular expression using the default configuration and

- ```rust
  pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<Regex, BuildError> { /* ... */ }
  ```
  Like `new`, but parses multiple patterns into a single "multi regex."

- ```rust
  pub fn builder() -> Builder { /* ... */ }
  ```
  Return a builder for configuring the construction of a `Regex`.

- ```rust
  pub fn create_cache(self: &Self) -> Cache { /* ... */ }
  ```
  Create a new cache for this `Regex`.

- ```rust
  pub fn reset_cache(self: &Self, cache: &mut Cache) { /* ... */ }
  ```
  Reset the given cache such that it can be used for searching with the

- ```rust
  pub fn is_match<''h, I: Into<Input<''h>>>(self: &Self, cache: &mut Cache, input: I) -> bool { /* ... */ }
  ```
  Returns true if and only if this regex matches the given haystack.

- ```rust
  pub fn find<''h, I: Into<Input<''h>>>(self: &Self, cache: &mut Cache, input: I) -> Option<Match> { /* ... */ }
  ```
  Returns the start and end offset of the leftmost match. If no match

- ```rust
  pub fn find_iter<''r, ''c, ''h, I: Into<Input<''h>>>(self: &''r Self, cache: &''c mut Cache, input: I) -> FindMatches<''r, ''c, ''h> { /* ... */ }
  ```
  Returns an iterator over all non-overlapping leftmost matches in the

- ```rust
  pub fn try_search(self: &Self, cache: &mut Cache, input: &Input<''_>) -> Result<Option<Match>, MatchError> { /* ... */ }
  ```
  Returns the start and end offset of the leftmost match. If no match

- ```rust
  pub fn forward(self: &Self) -> &DFA { /* ... */ }
  ```
  Return the underlying lazy DFA responsible for forward matching.

- ```rust
  pub fn reverse(self: &Self) -> &DFA { /* ... */ }
  ```
  Return the underlying lazy DFA responsible for reverse matching.

- ```rust
  pub fn pattern_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of patterns matched by this regex.

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
- **Unpin**
- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
#### Struct `FindMatches`

An iterator over all non-overlapping matches for an infallible search.

The iterator yields a [`Match`] value until no more matches could be found.
If the underlying regex engine returns an error, then a panic occurs.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the regex object.
* `'h` represents the lifetime of the haystack being searched.
* `'c` represents the lifetime of the regex cache.

This iterator can be created with the [`Regex::find_iter`] method.

```rust
pub struct FindMatches<''r, ''c, ''h> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Match> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
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

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `Cache`

A cache represents a partially computed forward and reverse DFA.

A cache is the key component that differentiates a classical DFA and a
hybrid NFA/DFA (also called a "lazy DFA"). Where a classical DFA builds a
complete transition table that can handle all possible inputs, a hybrid
NFA/DFA starts with an empty transition table and builds only the parts
required during search. The parts that are built are stored in a cache. For
this reason, a cache is a required parameter for nearly every operation on
a [`Regex`].

Caches can be created from their corresponding `Regex` via
[`Regex::create_cache`]. A cache can only be used with either the `Regex`
that created it, or the `Regex` that was most recently used to reset it
with [`Cache::reset`]. Using a cache with any other `Regex` may result in
panics or incorrect results.

```rust
pub struct Cache {
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
  pub fn new(re: &Regex) -> Cache { /* ... */ }
  ```
  Create a new cache for the given `Regex`.

- ```rust
  pub fn reset(self: &mut Self, re: &Regex) { /* ... */ }
  ```
  Reset this cache such that it can be used for searching with the given

- ```rust
  pub fn forward(self: &mut Self) -> &dfa::Cache { /* ... */ }
  ```
  Return a reference to the forward cache.

- ```rust
  pub fn reverse(self: &mut Self) -> &dfa::Cache { /* ... */ }
  ```
  Return a reference to the reverse cache.

- ```rust
  pub fn forward_mut(self: &mut Self) -> &mut dfa::Cache { /* ... */ }
  ```
  Return a mutable reference to the forward cache.

- ```rust
  pub fn reverse_mut(self: &mut Self) -> &mut dfa::Cache { /* ... */ }
  ```
  Return a mutable reference to the reverse cache.

- ```rust
  pub fn as_parts(self: &Self) -> (&dfa::Cache, &dfa::Cache) { /* ... */ }
  ```
  Return references to the forward and reverse caches, respectively.

- ```rust
  pub fn as_parts_mut(self: &mut Self) -> (&mut dfa::Cache, &mut dfa::Cache) { /* ... */ }
  ```
  Return mutable references to the forward and reverse caches,

- ```rust
  pub fn memory_usage(self: &Self) -> usize { /* ... */ }
  ```
  Returns the heap memory usage, in bytes, as a sum of the forward and

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Cache { /* ... */ }
    ```

- **Send**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Freeze**
#### Struct `Builder`

A builder for a regex based on a hybrid NFA/DFA.

This builder permits configuring options for the syntax of a pattern, the
NFA construction, the lazy DFA construction and finally the regex searching
itself. This builder is different from a general purpose regex builder
in that it permits fine grain configuration of the construction process.
The trade off for this is complexity, and the possibility of setting a
configuration that might not make sense. For example, there are two
different UTF-8 modes:

* [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) controls
whether the pattern itself can contain sub-expressions that match invalid
UTF-8.
* [`thompson::Config::utf8`] controls how the regex iterators themselves
advance the starting position of the next search when a match with zero
length is found.

Generally speaking, callers will want to either enable all of these or
disable all of these.

Internally, building a regex requires building two hybrid NFA/DFAs,
where one is responsible for finding the end of a match and the other is
responsible for finding the start of a match. If you only need to detect
whether something matched, or only the end of a match, then you should use
a [`dfa::Builder`] to construct a single hybrid NFA/DFA, which is cheaper
than building two of them.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    hybrid::regex::Regex, nfa::thompson, util::syntax, Match,
};

let re = Regex::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let mut cache = re.create_cache();

let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
let expected = Some(Match::must(0, 1..9));
let got = re.find(&mut cache, haystack);
assert_eq!(expected, got);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
assert_eq!(b"foo\xFFarzz", &haystack[got.unwrap().range()]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

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
  Create a new regex builder with the default configuration.

- ```rust
  pub fn build(self: &Self, pattern: &str) -> Result<Regex, BuildError> { /* ... */ }
  ```
  Build a regex from the given pattern.

- ```rust
  pub fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<Regex, BuildError> { /* ... */ }
  ```
  Build a regex from the given patterns.

- ```rust
  pub fn build_from_dfas(self: &Self, forward: DFA, reverse: DFA) -> Regex { /* ... */ }
  ```
  Build a regex from its component forward and reverse hybrid NFA/DFAs.

- ```rust
  pub fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder { /* ... */ }
  ```
  Set the syntax configuration for this builder using

- ```rust
  pub fn thompson(self: &mut Self, config: thompson::Config) -> &mut Builder { /* ... */ }
  ```
  Set the Thompson NFA configuration for this builder using

- ```rust
  pub fn dfa(self: &mut Self, config: dfa::Config) -> &mut Builder { /* ... */ }
  ```
  Set the lazy DFA compilation configuration for this builder using

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Default**
  - ```rust
    fn default() -> Builder { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
### Re-exports

#### Re-export `BuildError`

```rust
pub use self::error::BuildError;
```

#### Re-export `CacheError`

```rust
pub use self::error::CacheError;
```

#### Re-export `StartError`

```rust
pub use self::error::StartError;
```

#### Re-export `LazyStateID`

```rust
pub use self::id::LazyStateID;
```

## Module `meta`

**Attributes:**

- `#[<cfg>(feature = "meta")]`

Provides a regex matcher that composes several other regex matchers
automatically.

This module is home to a meta [`Regex`], which provides a convenient high
level API for executing regular expressions in linear time.

# Comparison with the `regex` crate

A meta `Regex` is the implementation used directly by the `regex` crate.
Indeed, the `regex` crate API is essentially just a light wrapper over a meta
`Regex`. This means that if you need the full flexibility offered by this
API, then you should be able to switch to using this API directly without
any changes in match semantics or syntax. However, there are some API level
differences:

* The `regex` crate API returns match objects that include references to the
haystack itself, which in turn makes it easy to access the matching strings
without having to slice the haystack yourself. In contrast, a meta `Regex`
returns match objects that only have offsets in them.
* At time of writing, a meta `Regex` doesn't have some of the convenience
routines that the `regex` crate has, such as replacements. Note though that
[`Captures::interpolate_string`](crate::util::captures::Captures::interpolate_string)
will handle the replacement string interpolation for you.
* A meta `Regex` supports the [`Input`](crate::Input) abstraction, which
provides a way to configure a search in more ways than is supported by the
`regex` crate. For example, [`Input::anchored`](crate::Input::anchored) can
be used to run an anchored search, regardless of whether the pattern is itself
anchored with a `^`.
* A meta `Regex` supports multi-pattern searching everywhere.
Indeed, every [`Match`](crate::Match) returned by the search APIs
include a [`PatternID`](crate::PatternID) indicating which pattern
matched. In the single pattern case, all matches correspond to
[`PatternID::ZERO`](crate::PatternID::ZERO). In contrast, the `regex` crate
has distinct `Regex` and a `RegexSet` APIs. The former only supports a single
pattern, while the latter supports multiple patterns but cannot report the
offsets of a match.
* A meta `Regex` provides the explicit capability of bypassing its internal
memory pool for automatically acquiring mutable scratch space required by its
internal regex engines. Namely, a [`Cache`] can be explicitly provided to lower
level routines such as [`Regex::search_with`].

```rust
pub mod meta { /* ... */ }
```

### Re-exports

#### Re-export `BuildError`

```rust
pub use self::error::BuildError;
```

#### Re-export `Builder`

```rust
pub use self::regex::Builder;
```

#### Re-export `Cache`

```rust
pub use self::regex::Cache;
```

#### Re-export `CapturesMatches`

```rust
pub use self::regex::CapturesMatches;
```

#### Re-export `Config`

```rust
pub use self::regex::Config;
```

#### Re-export `FindMatches`

```rust
pub use self::regex::FindMatches;
```

#### Re-export `Regex`

```rust
pub use self::regex::Regex;
```

#### Re-export `Split`

```rust
pub use self::regex::Split;
```

#### Re-export `SplitN`

```rust
pub use self::regex::SplitN;
```

## Module `nfa`

**Attributes:**

- `#[<cfg>(feature = "nfa-thompson")]`

Provides non-deterministic finite automata (NFA) and regex engines that use
them.

While NFAs and DFAs (deterministic finite automata) have equivalent *theoretical*
power, their usage in practice tends to result in different engineering trade
offs. While this isn't meant to be a comprehensive treatment of the topic, here
are a few key trade offs that are, at minimum, true for this crate:

* NFAs tend to be represented sparsely where as DFAs are represented densely.
Sparse representations use less memory, but are slower to traverse. Conversely,
dense representations use more memory, but are faster to traverse. (Sometimes
these lines are blurred. For example, an `NFA` might choose to represent a
particular state in a dense fashion, and a DFA can be built using a sparse
representation via [`sparse::DFA`](crate::dfa::sparse::DFA).
* NFAs have espilon transitions and DFAs don't. In practice, this means that
handling a single byte in a haystack with an NFA at search time may require
visiting multiple NFA states. In a DFA, each byte only requires visiting
a single state. Stated differently, NFAs require a variable number of CPU
instructions to process one byte in a haystack where as a DFA uses a constant
number of CPU instructions to process one byte.
* NFAs are generally easier to amend with secondary storage. For example, the
[`thompson::pikevm::PikeVM`] uses an NFA to match, but also uses additional
memory beyond the model of a finite state machine to track offsets for matching
capturing groups. Conversely, the most a DFA can do is report the offset (and
pattern ID) at which a match occurred. This is generally why we also compile
DFAs in reverse, so that we can run them after finding the end of a match to
also find the start of a match.
* NFAs take worst case linear time to build, but DFAs take worst case
exponential time to build. The [hybrid NFA/DFA](crate::hybrid) mitigates this
challenge for DFAs in many practical cases.

There are likely other differences, but the bottom line is that NFAs tend to be
more memory efficient and give easier opportunities for increasing expressive
power, where as DFAs are faster to search with.

# Why only a Thompson NFA?

Currently, the only kind of NFA we support in this crate is a [Thompson
NFA](https://en.wikipedia.org/wiki/Thompson%27s_construction). This refers
to a specific construction algorithm that takes the syntax of a regex
pattern and converts it to an NFA. Specifically, it makes gratuitous use of
epsilon transitions in order to keep its structure simple. In exchange, its
construction time is linear in the size of the regex. A Thompson NFA also makes
the guarantee that given any state and a character in a haystack, there is at
most one transition defined for it. (Although there may be many epsilon
transitions.)

It possible that other types of NFAs will be added in the future, such as a
[Glushkov NFA](https://en.wikipedia.org/wiki/Glushkov%27s_construction_algorithm).
But currently, this crate only provides a Thompson NFA.

```rust
pub mod nfa { /* ... */ }
```

### Modules

## Module `thompson`

**Attributes:**

- `#[<cfg>(feature = "nfa-thompson")]`

Defines a Thompson NFA and provides the [`PikeVM`](pikevm::PikeVM) and
[`BoundedBacktracker`](backtrack::BoundedBacktracker) regex engines.

A Thompson NFA (non-deterministic finite automaton) is arguably _the_ central
data type in this library. It is the result of what is commonly referred to as
"regex compilation." That is, turning a regex pattern from its concrete syntax
string into something that can run a search looks roughly like this:

* A `&str` is parsed into a [`regex-syntax::ast::Ast`](regex_syntax::ast::Ast).
* An `Ast` is translated into a [`regex-syntax::hir::Hir`](regex_syntax::hir::Hir).
* An `Hir` is compiled into a [`NFA`].
* The `NFA` is then used to build one of a few different regex engines:
  * An `NFA` is used directly in the `PikeVM` and `BoundedBacktracker` engines.
  * An `NFA` is used by a [hybrid NFA/DFA](crate::hybrid) to build out a DFA's
  transition table at search time.
  * An `NFA`, assuming it is one-pass, is used to build a full
  [one-pass DFA](crate::dfa::onepass) ahead of time.
  * An `NFA` is used to build a [full DFA](crate::dfa) ahead of time.

The [`meta`](crate::meta) regex engine makes all of these choices for you based
on various criteria. However, if you have a lower level use case, _you_ can
build any of the above regex engines and use them directly. But you must start
here by building an `NFA`.

# Details

It is perhaps worth expanding a bit more on what it means to go through the
`&str`->`Ast`->`Hir`->`NFA` process.

* Parsing a string into an `Ast` gives it a structured representation.
Crucially, the size and amount of work done in this step is proportional to the
size of the original string. No optimization or Unicode handling is done at
this point. This means that parsing into an `Ast` has very predictable costs.
Moreover, an `Ast` can be roundtripped back to its original pattern string as
written.
* Translating an `Ast` into an `Hir` is a process by which the structured
representation is simplified down to its most fundamental components.
Translation deals with flags such as case insensitivity by converting things
like `(?i:a)` to `[Aa]`. Translation is also where Unicode tables are consulted
to resolve things like `\p{Emoji}` and `\p{Greek}`. It also flattens each
character class, regardless of how deeply nested it is, into a single sequence
of non-overlapping ranges. All the various literal forms are thrown out in
favor of one common representation. Overall, the `Hir` is small enough to fit
into your head and makes analysis and other tasks much simpler.
* Compiling an `Hir` into an `NFA` formulates the regex into a finite state
machine whose transitions are defined over bytes. For example, an `Hir` might
have a Unicode character class corresponding to a sequence of ranges defined
in terms of `char`. Compilation is then responsible for turning those ranges
into a UTF-8 automaton. That is, an automaton that matches the UTF-8 encoding
of just the codepoints specified by those ranges. Otherwise, the main job of
an `NFA` is to serve as a byte-code of sorts for a virtual machine. It can be
seen as a sequence of instructions for how to match a regex.

```rust
pub mod thompson { /* ... */ }
```

### Modules

## Module `backtrack`

**Attributes:**

- `#[<cfg>(feature = "nfa-backtrack")]`

An NFA backed bounded backtracker for executing regex searches with capturing
groups.

This module provides a [`BoundedBacktracker`] that works by simulating an NFA
using the classical backtracking algorithm with a twist: it avoids redoing
work that it has done before and thereby avoids worst case exponential time.
In exchange, it can only be used on "short" haystacks. Its advantage is that
is can be faster than the [`PikeVM`](thompson::pikevm::PikeVM) in many cases
because it does less book-keeping.

```rust
pub mod backtrack { /* ... */ }
```

### Types

#### Struct `Config`

The configuration used for building a bounded backtracker.

A bounded backtracker configuration is a simple data object that is
typically used with [`Builder::configure`].

```rust
pub struct Config {
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
  pub fn new() -> Config { /* ... */ }
  ```
  Return a new default regex configuration.

- ```rust
  pub fn prefilter(self: Self, pre: Option<Prefilter>) -> Config { /* ... */ }
  ```
  Set a prefilter to be used whenever a start state is entered.

- ```rust
  pub fn visited_capacity(self: Self, capacity: usize) -> Config { /* ... */ }
  ```
  Set the visited capacity used to bound backtracking.

- ```rust
  pub fn get_prefilter(self: &Self) -> Option<&Prefilter> { /* ... */ }
  ```
  Returns the prefilter set in this configuration, if one at all.

- ```rust
  pub fn get_visited_capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the configured visited capacity.

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
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

- **Freeze**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Config { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Config { /* ... */ }
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

- **Unpin**
#### Struct `Builder`

A builder for a bounded backtracker.

This builder permits configuring options for the syntax of a pattern, the
NFA construction and the `BoundedBacktracker` construction. This builder
is different from a general purpose regex builder in that it permits fine
grain configuration of the construction process. The trade off for this is
complexity, and the possibility of setting a configuration that might not
make sense. For example, there are two different UTF-8 modes:

* [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) controls
whether the pattern itself can contain sub-expressions that match invalid
UTF-8.
* [`thompson::Config::utf8`] controls how the regex iterators themselves
advance the starting position of the next search when a match with zero
length is found.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```
use regex_automata::{
    nfa::thompson::{self, backtrack::BoundedBacktracker},
    util::syntax,
    Match,
};

let re = BoundedBacktracker::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let mut cache = re.create_cache();

let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
let expected = Some(Ok(Match::must(0, 1..9)));
let got = re.try_find_iter(&mut cache, haystack).next();
assert_eq!(expected, got);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
//
// N.B. This example does not show the impact of
// disabling UTF-8 mode on a BoundedBacktracker Config, since that
// only impacts regexes that can produce matches of
// length 0.
assert_eq!(b"foo\xFFarzz", &haystack[got.unwrap()?.range()]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

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
  Create a new BoundedBacktracker builder with its default configuration.

- ```rust
  pub fn build(self: &Self, pattern: &str) -> Result<BoundedBacktracker, BuildError> { /* ... */ }
  ```
  Build a `BoundedBacktracker` from the given pattern.

- ```rust
  pub fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<BoundedBacktracker, BuildError> { /* ... */ }
  ```
  Build a `BoundedBacktracker` from the given patterns.

- ```rust
  pub fn build_from_nfa(self: &Self, nfa: NFA) -> Result<BoundedBacktracker, BuildError> { /* ... */ }
  ```
  Build a `BoundedBacktracker` directly from its NFA.

- ```rust
  pub fn configure(self: &mut Self, config: Config) -> &mut Builder { /* ... */ }
  ```
  Apply the given `BoundedBacktracker` configuration options to this

- ```rust
  pub fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder { /* ... */ }
  ```
  Set the syntax configuration for this builder using

- ```rust
  pub fn thompson(self: &mut Self, config: thompson::Config) -> &mut Builder { /* ... */ }
  ```
  Set the Thompson NFA configuration for this builder using

###### Trait Implementations

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

- **UnwindSafe**
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
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Builder { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `BoundedBacktracker`

A backtracking regex engine that bounds its execution to avoid exponential
blow-up.

This regex engine only implements leftmost-first match semantics and
only supports leftmost searches. It effectively does the same thing as a
[`PikeVM`](thompson::pikevm::PikeVM), but typically does it faster because
it doesn't have to worry about copying capturing group spans for most NFA
states. Instead, the backtracker can maintain one set of captures (provided
by the caller) and never needs to copy them. In exchange, the backtracker
bounds itself to ensure it doesn't exhibit worst case exponential time.
This results in the backtracker only being able to handle short haystacks
given reasonable memory usage.

# Searches may return an error!

By design, this backtracking regex engine is bounded. This bound is
implemented by not visiting any combination of NFA state ID and position
in a haystack more than once. Thus, the total memory required to bound
backtracking is proportional to `haystack.len() * nfa.states().len()`.
This can obviously get quite large, since large haystacks aren't terribly
uncommon. To avoid using exorbitant memory, the capacity is bounded by
a fixed limit set via [`Config::visited_capacity`]. Thus, if the total
capacity required for a particular regex and a haystack exceeds this
capacity, then the search routine will return an error.

Unlike other regex engines that may return an error at search time (like
the DFA or the hybrid NFA/DFA), there is no way to guarantee that a bounded
backtracker will work for every haystack. Therefore, this regex engine
_only_ exposes fallible search routines to avoid the footgun of panicking
when running a search on a haystack that is too big.

If one wants to use the fallible search APIs without handling the
error, the only way to guarantee an error won't occur from the
haystack length is to ensure the haystack length does not exceed
[`BoundedBacktracker::max_haystack_len`].

# Example: Unicode word boundaries

This example shows that the bounded backtracker implements Unicode word
boundaries correctly by default.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{nfa::thompson::backtrack::BoundedBacktracker, Match};

let re = BoundedBacktracker::new(r"\b\w+\b")?;
let mut cache = re.create_cache();

let mut it = re.try_find_iter(&mut cache, "Шерлок Холмс");
assert_eq!(Some(Ok(Match::must(0, 0..12))), it.next());
assert_eq!(Some(Ok(Match::must(0, 13..23))), it.next());
assert_eq!(None, it.next());
# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: multiple regex patterns

The bounded backtracker supports searching for multiple patterns
simultaneously, just like other regex engines. Note though that because it
uses a backtracking strategy, this regex engine is unlikely to scale well
as more patterns are added. But then again, as more patterns are added, the
maximum haystack length allowed will also shorten (assuming the visited
capacity remains invariant).

```
use regex_automata::{nfa::thompson::backtrack::BoundedBacktracker, Match};

let re = BoundedBacktracker::new_many(&["[a-z]+", "[0-9]+"])?;
let mut cache = re.create_cache();

let mut it = re.try_find_iter(&mut cache, "abc 1 foo 4567 0 quux");
assert_eq!(Some(Ok(Match::must(0, 0..3))), it.next());
assert_eq!(Some(Ok(Match::must(1, 4..5))), it.next());
assert_eq!(Some(Ok(Match::must(0, 6..9))), it.next());
assert_eq!(Some(Ok(Match::must(1, 10..14))), it.next());
assert_eq!(Some(Ok(Match::must(1, 15..16))), it.next());
assert_eq!(Some(Ok(Match::must(0, 17..21))), it.next());
assert_eq!(None, it.next());
# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub struct BoundedBacktracker {
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
  pub fn new(pattern: &str) -> Result<BoundedBacktracker, BuildError> { /* ... */ }
  ```
  Parse the given regular expression using the default configuration and

- ```rust
  pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<BoundedBacktracker, BuildError> { /* ... */ }
  ```
  Like `new`, but parses multiple patterns into a single "multi regex."

- ```rust
  pub fn new_from_nfa(nfa: NFA) -> Result<BoundedBacktracker, BuildError> { /* ... */ }
  ```
  # Example

- ```rust
  pub fn always_match() -> Result<BoundedBacktracker, BuildError> { /* ... */ }
  ```
  Create a new `BoundedBacktracker` that matches every input.

- ```rust
  pub fn never_match() -> Result<BoundedBacktracker, BuildError> { /* ... */ }
  ```
  Create a new `BoundedBacktracker` that never matches any input.

- ```rust
  pub fn config() -> Config { /* ... */ }
  ```
  Return a default configuration for a `BoundedBacktracker`.

- ```rust
  pub fn builder() -> Builder { /* ... */ }
  ```
  Return a builder for configuring the construction of a

- ```rust
  pub fn create_cache(self: &Self) -> Cache { /* ... */ }
  ```
  Create a new cache for this regex.

- ```rust
  pub fn create_captures(self: &Self) -> Captures { /* ... */ }
  ```
  Create a new empty set of capturing groups that is guaranteed to be

- ```rust
  pub fn reset_cache(self: &Self, cache: &mut Cache) { /* ... */ }
  ```
  Reset the given cache such that it can be used for searching with the

- ```rust
  pub fn pattern_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of patterns compiled into this

- ```rust
  pub fn get_config(self: &Self) -> &Config { /* ... */ }
  ```
  Return the config for this `BoundedBacktracker`.

- ```rust
  pub fn get_nfa(self: &Self) -> &NFA { /* ... */ }
  ```
  Returns a reference to the underlying NFA.

- ```rust
  pub fn max_haystack_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the maximum haystack length supported by this backtracker.

- ```rust
  pub fn try_is_match<''h, I: Into<Input<''h>>>(self: &Self, cache: &mut Cache, input: I) -> Result<bool, MatchError> { /* ... */ }
  ```
  Returns true if and only if this regex matches the given haystack.

- ```rust
  pub fn try_find<''h, I: Into<Input<''h>>>(self: &Self, cache: &mut Cache, input: I) -> Result<Option<Match>, MatchError> { /* ... */ }
  ```
  Executes a leftmost forward search and returns a `Match` if one exists.

- ```rust
  pub fn try_captures<''h, I: Into<Input<''h>>>(self: &Self, cache: &mut Cache, input: I, caps: &mut Captures) -> Result<(), MatchError> { /* ... */ }
  ```
  Executes a leftmost forward search and writes the spans of capturing

- ```rust
  pub fn try_find_iter<''r, ''c, ''h, I: Into<Input<''h>>>(self: &''r Self, cache: &''c mut Cache, input: I) -> TryFindMatches<''r, ''c, ''h> { /* ... */ }
  ```
  Returns an iterator over all non-overlapping leftmost matches in the

- ```rust
  pub fn try_captures_iter<''r, ''c, ''h, I: Into<Input<''h>>>(self: &''r Self, cache: &''c mut Cache, input: I) -> TryCapturesMatches<''r, ''c, ''h> { /* ... */ }
  ```
  Returns an iterator over all non-overlapping `Captures` values. If no

- ```rust
  pub fn try_search(self: &Self, cache: &mut Cache, input: &Input<''_>, caps: &mut Captures) -> Result<(), MatchError> { /* ... */ }
  ```
  Executes a leftmost forward search and writes the spans of capturing

- ```rust
  pub fn try_search_slots(self: &Self, cache: &mut Cache, input: &Input<''_>, slots: &mut [Option<NonMaxUsize>]) -> Result<Option<PatternID>, MatchError> { /* ... */ }
  ```
  Executes a leftmost forward search and writes the spans of capturing

###### Trait Implementations

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

- **Unpin**
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

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> BoundedBacktracker { /* ... */ }
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

- **Sync**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
#### Struct `TryFindMatches`

An iterator over all non-overlapping matches for a fallible search.

The iterator yields a `Result<Match, MatchError` value until no more
matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the BoundedBacktracker.
* `'c` represents the lifetime of the BoundedBacktracker's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the [`BoundedBacktracker::try_find_iter`]
method.

```rust
pub struct TryFindMatches<''r, ''c, ''h> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Send**
- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Result<Match, MatchError>> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `TryCapturesMatches`

An iterator over all non-overlapping leftmost matches, with their capturing
groups, for a fallible search.

The iterator yields a `Result<Captures, MatchError>` value until no more
matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the BoundedBacktracker.
* `'c` represents the lifetime of the BoundedBacktracker's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the
[`BoundedBacktracker::try_captures_iter`] method.

```rust
pub struct TryCapturesMatches<''r, ''c, ''h> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Result<Captures, MatchError>> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
#### Struct `Cache`

A cache represents mutable state that a [`BoundedBacktracker`] requires
during a search.

For a given [`BoundedBacktracker`], its corresponding cache may be created
either via [`BoundedBacktracker::create_cache`], or via [`Cache::new`].
They are equivalent in every way, except the former does not require
explicitly importing `Cache`.

A particular `Cache` is coupled with the [`BoundedBacktracker`] from which
it was created. It may only be used with that `BoundedBacktracker`. A cache
and its allocations may be re-purposed via [`Cache::reset`], in which case,
it can only be used with the new `BoundedBacktracker` (and not the old
one).

```rust
pub struct Cache {
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
  pub fn new(re: &BoundedBacktracker) -> Cache { /* ... */ }
  ```
  Create a new [`BoundedBacktracker`] cache.

- ```rust
  pub fn reset(self: &mut Self, re: &BoundedBacktracker) { /* ... */ }
  ```
  Reset this cache such that it can be used for searching with different

- ```rust
  pub fn memory_usage(self: &Self) -> usize { /* ... */ }
  ```
  Returns the heap memory usage, in bytes, of this cache.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Cache { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
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
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Functions

#### Function `min_visited_capacity`

Returns the minimum visited capacity for the given haystack.

This function can be used as the argument to [`Config::visited_capacity`]
in order to guarantee that a backtracking search for the given `input`
won't return an error when using a [`BoundedBacktracker`] built from the
given `NFA`.

This routine exists primarily as a way to test that the bounded backtracker
works correctly when its capacity is set to the smallest possible amount.
Still, it may be useful in cases where you know you want to use the bounded
backtracker for a specific input, and just need to know what visited
capacity to provide to make it work.

Be warned that this number could be quite large as it is multiplicative in
the size the given NFA and haystack.

```rust
pub fn min_visited_capacity(nfa: &crate::nfa::thompson::NFA, input: &crate::util::search::Input<''_>) -> usize { /* ... */ }
```

## Module `pikevm`

**Attributes:**

- `#[<cfg>(feature = "nfa-pikevm")]`

An NFA backed Pike VM for executing regex searches with capturing groups.

This module provides a [`PikeVM`] that works by simulating an NFA and
resolving all spans of capturing groups that participate in a match.

```rust
pub mod pikevm { /* ... */ }
```

### Types

#### Struct `Config`

The configuration used for building a [`PikeVM`].

A PikeVM configuration is a simple data object that is typically used with
[`Builder::configure`]. It can be cheaply cloned.

A default configuration can be created either with `Config::new`, or
perhaps more conveniently, with [`PikeVM::config`].

```rust
pub struct Config {
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
  pub fn new() -> Config { /* ... */ }
  ```
  Return a new default PikeVM configuration.

- ```rust
  pub fn match_kind(self: Self, kind: MatchKind) -> Config { /* ... */ }
  ```
  Set the desired match semantics.

- ```rust
  pub fn prefilter(self: Self, pre: Option<Prefilter>) -> Config { /* ... */ }
  ```
  Set a prefilter to be used whenever a start state is entered.

- ```rust
  pub fn get_match_kind(self: &Self) -> MatchKind { /* ... */ }
  ```
  Returns the match semantics set in this configuration.

- ```rust
  pub fn get_prefilter(self: &Self) -> Option<&Prefilter> { /* ... */ }
  ```
  Returns the prefilter set in this configuration, if one at all.

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> Config { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Config { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Unpin**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `Builder`

A builder for a `PikeVM`.

This builder permits configuring options for the syntax of a pattern,
the NFA construction and the `PikeVM` construction. This builder is
different from a general purpose regex builder in that it permits fine
grain configuration of the construction process. The trade off for this is
complexity, and the possibility of setting a configuration that might not
make sense. For example, there are two different UTF-8 modes:

* [`util::syntax::Config::utf8`](crate::util::syntax::Config::utf8)
controls whether the pattern itself can contain sub-expressions that match
invalid UTF-8.
* [`thompson::Config::utf8`] controls whether empty matches that split a
Unicode codepoint are reported or not.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```
use regex_automata::{
    nfa::thompson::{self, pikevm::PikeVM},
    util::syntax,
    Match,
};

let re = PikeVM::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let mut cache = re.create_cache();

let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
let expected = Some(Match::must(0, 1..9));
let got = re.find_iter(&mut cache, haystack).next();
assert_eq!(expected, got);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
//
// N.B. This example does not show the impact of
// disabling UTF-8 mode on a PikeVM Config, since that
// only impacts regexes that can produce matches of
// length 0.
assert_eq!(b"foo\xFFarzz", &haystack[got.unwrap().range()]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

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
  Create a new PikeVM builder with its default configuration.

- ```rust
  pub fn build(self: &Self, pattern: &str) -> Result<PikeVM, BuildError> { /* ... */ }
  ```
  Build a `PikeVM` from the given pattern.

- ```rust
  pub fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<PikeVM, BuildError> { /* ... */ }
  ```
  Build a `PikeVM` from the given patterns.

- ```rust
  pub fn build_from_nfa(self: &Self, nfa: NFA) -> Result<PikeVM, BuildError> { /* ... */ }
  ```
  Build a `PikeVM` directly from its NFA.

- ```rust
  pub fn configure(self: &mut Self, config: Config) -> &mut Builder { /* ... */ }
  ```
  Apply the given `PikeVM` configuration options to this builder.

- ```rust
  pub fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder { /* ... */ }
  ```
  Set the syntax configuration for this builder using

- ```rust
  pub fn thompson(self: &mut Self, config: thompson::Config) -> &mut Builder { /* ... */ }
  ```
  Set the Thompson NFA configuration for this builder using

###### Trait Implementations

- **Send**
- **Freeze**
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Builder { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `PikeVM`

A virtual machine for executing regex searches with capturing groups.

# Infallible APIs

Unlike most other regex engines in this crate, a `PikeVM` never returns an
error at search time. It supports all [`Anchored`] configurations, never
quits and works on haystacks of arbitrary length.

There are two caveats to mention though:

* If an invalid pattern ID is given to a search via [`Anchored::Pattern`],
then the PikeVM will report "no match." This is consistent with all other
regex engines in this crate.
* When using [`PikeVM::which_overlapping_matches`] with a [`PatternSet`]
that has insufficient capacity to store all valid pattern IDs, then if a
match occurs for a `PatternID` that cannot be inserted, it is silently
dropped as if it did not match.

# Advice

The `PikeVM` is generally the most "powerful" regex engine in this crate.
"Powerful" in this context means that it can handle any regular expression
that is parseable by `regex-syntax` and any size haystack. Regretably,
the `PikeVM` is also simultaneously often the _slowest_ regex engine in
practice. This results in an annoying situation where one generally tries
to pick any other regex engine (or perhaps none at all) before being
forced to fall back to a `PikeVM`.

For example, a common strategy for dealing with capturing groups is to
actually look for the overall match of the regex using a faster regex
engine, like a [lazy DFA](crate::hybrid::regex::Regex). Once the overall
match is found, one can then run the `PikeVM` on just the match span to
find the spans of the capturing groups. In this way, the faster regex
engine does the majority of the work, while the `PikeVM` only lends its
power in a more limited role.

Unfortunately, this isn't always possible because the faster regex engines
don't support all of the regex features in `regex-syntax`. This notably
includes (and is currently limited to) Unicode word boundaries. So if
your pattern has Unicode word boundaries, you typically can't use a
DFA-based regex engine at all (unless you [enable heuristic support for
it](crate::hybrid::dfa::Config::unicode_word_boundary)). (The [one-pass
DFA](crate::dfa::onepass::DFA) can handle Unicode word boundaries for
anchored searches only, but in a cruel sort of joke, many Unicode features
tend to result in making the regex _not_ one-pass.)

# Example

This example shows that the `PikeVM` implements Unicode word boundaries
correctly by default.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

let re = PikeVM::new(r"\b\w+\b")?;
let mut cache = re.create_cache();

let mut it = re.find_iter(&mut cache, "Шерлок Холмс");
assert_eq!(Some(Match::must(0, 0..12)), it.next());
assert_eq!(Some(Match::must(0, 13..23)), it.next());
assert_eq!(None, it.next());
# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub struct PikeVM {
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
  pub fn new(pattern: &str) -> Result<PikeVM, BuildError> { /* ... */ }
  ```
  Parse the given regular expression using the default configuration and

- ```rust
  pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<PikeVM, BuildError> { /* ... */ }
  ```
  Like `new`, but parses multiple patterns into a single "multi regex."

- ```rust
  pub fn new_from_nfa(nfa: NFA) -> Result<PikeVM, BuildError> { /* ... */ }
  ```
  Like `new`, but builds a PikeVM directly from an NFA. This is useful

- ```rust
  pub fn always_match() -> Result<PikeVM, BuildError> { /* ... */ }
  ```
  Create a new `PikeVM` that matches every input.

- ```rust
  pub fn never_match() -> Result<PikeVM, BuildError> { /* ... */ }
  ```
  Create a new `PikeVM` that never matches any input.

- ```rust
  pub fn config() -> Config { /* ... */ }
  ```
  Return a default configuration for a `PikeVM`.

- ```rust
  pub fn builder() -> Builder { /* ... */ }
  ```
  Return a builder for configuring the construction of a `PikeVM`.

- ```rust
  pub fn create_captures(self: &Self) -> Captures { /* ... */ }
  ```
  Create a new empty set of capturing groups that is guaranteed to be

- ```rust
  pub fn create_cache(self: &Self) -> Cache { /* ... */ }
  ```
  Create a new cache for this `PikeVM`.

- ```rust
  pub fn reset_cache(self: &Self, cache: &mut Cache) { /* ... */ }
  ```
  Reset the given cache such that it can be used for searching with the

- ```rust
  pub fn pattern_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of patterns compiled into this `PikeVM`.

- ```rust
  pub fn get_config(self: &Self) -> &Config { /* ... */ }
  ```
  Return the config for this `PikeVM`.

- ```rust
  pub fn get_nfa(self: &Self) -> &NFA { /* ... */ }
  ```
  Returns a reference to the underlying NFA.

- ```rust
  pub fn is_match<''h, I: Into<Input<''h>>>(self: &Self, cache: &mut Cache, input: I) -> bool { /* ... */ }
  ```
  Returns true if and only if this `PikeVM` matches the given haystack.

- ```rust
  pub fn find<''h, I: Into<Input<''h>>>(self: &Self, cache: &mut Cache, input: I) -> Option<Match> { /* ... */ }
  ```
  Executes a leftmost forward search and returns a `Match` if one exists.

- ```rust
  pub fn captures<''h, I: Into<Input<''h>>>(self: &Self, cache: &mut Cache, input: I, caps: &mut Captures) { /* ... */ }
  ```
  Executes a leftmost forward search and writes the spans of capturing

- ```rust
  pub fn find_iter<''r, ''c, ''h, I: Into<Input<''h>>>(self: &''r Self, cache: &''c mut Cache, input: I) -> FindMatches<''r, ''c, ''h> { /* ... */ }
  ```
  Returns an iterator over all non-overlapping leftmost matches in the

- ```rust
  pub fn captures_iter<''r, ''c, ''h, I: Into<Input<''h>>>(self: &''r Self, cache: &''c mut Cache, input: I) -> CapturesMatches<''r, ''c, ''h> { /* ... */ }
  ```
  Returns an iterator over all non-overlapping `Captures` values. If no

- ```rust
  pub fn search(self: &Self, cache: &mut Cache, input: &Input<''_>, caps: &mut Captures) { /* ... */ }
  ```
  Executes a leftmost forward search and writes the spans of capturing

- ```rust
  pub fn search_slots(self: &Self, cache: &mut Cache, input: &Input<''_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID> { /* ... */ }
  ```
  Executes a leftmost forward search and writes the spans of capturing

- ```rust
  pub fn which_overlapping_matches(self: &Self, cache: &mut Cache, input: &Input<''_>, patset: &mut PatternSet) { /* ... */ }
  ```
  Writes the set of patterns that match anywhere in the given search

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **Freeze**
- **Sync**
- **Send**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PikeVM { /* ... */ }
    ```

#### Struct `FindMatches`

An iterator over all non-overlapping matches for a particular search.

The iterator yields a [`Match`] value until no more matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the PikeVM.
* `'c` represents the lifetime of the PikeVM's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the [`PikeVM::find_iter`] method.

```rust
pub struct FindMatches<''r, ''c, ''h> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Match> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **UnwindSafe**
- **Send**
#### Struct `CapturesMatches`

An iterator over all non-overlapping leftmost matches, with their capturing
groups, for a particular search.

The iterator yields a [`Captures`] value until no more matches could be
found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the PikeVM.
* `'c` represents the lifetime of the PikeVM's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the [`PikeVM::captures_iter`] method.

```rust
pub struct CapturesMatches<''r, ''c, ''h> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Captures> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
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

- **RefUnwindSafe**
- **Unpin**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `Cache`

A cache represents mutable state that a [`PikeVM`] requires during a
search.

For a given [`PikeVM`], its corresponding cache may be created either via
[`PikeVM::create_cache`], or via [`Cache::new`]. They are equivalent in
every way, except the former does not require explicitly importing `Cache`.

A particular `Cache` is coupled with the [`PikeVM`] from which it
was created. It may only be used with that `PikeVM`. A cache and its
allocations may be re-purposed via [`Cache::reset`], in which case, it can
only be used with the new `PikeVM` (and not the old one).

```rust
pub struct Cache {
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
  pub fn new(re: &PikeVM) -> Cache { /* ... */ }
  ```
  Create a new [`PikeVM`] cache.

- ```rust
  pub fn reset(self: &mut Self, re: &PikeVM) { /* ... */ }
  ```
  Reset this cache such that it can be used for searching with a

- ```rust
  pub fn memory_usage(self: &Self) -> usize { /* ... */ }
  ```
  Returns the heap memory usage, in bytes, of this cache.

###### Trait Implementations

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Cache { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Re-exports

#### Re-export `Builder`

```rust
pub use self::builder::Builder;
```

#### Re-export `BuildError`

```rust
pub use self::error::BuildError;
```

#### Re-export `DenseTransitions`

```rust
pub use self::nfa::DenseTransitions;
```

#### Re-export `PatternIter`

```rust
pub use self::nfa::PatternIter;
```

#### Re-export `SparseTransitions`

```rust
pub use self::nfa::SparseTransitions;
```

#### Re-export `State`

```rust
pub use self::nfa::State;
```

#### Re-export `Transition`

```rust
pub use self::nfa::Transition;
```

#### Re-export `NFA`

```rust
pub use self::nfa::NFA;
```

#### Re-export `Compiler`

**Attributes:**

- `#[<cfg>(feature = "syntax")]`

```rust
pub use compiler::Compiler;
```

#### Re-export `Config`

**Attributes:**

- `#[<cfg>(feature = "syntax")]`

```rust
pub use compiler::Config;
```

#### Re-export `WhichCaptures`

**Attributes:**

- `#[<cfg>(feature = "syntax")]`

```rust
pub use compiler::WhichCaptures;
```

## Module `util`

A collection of modules that provide APIs that are useful across many regex
engines.

While one should explore the sub-modules directly to get a sense of what's
there, here are some highlights that tie the sub-modules to higher level
use cases:

* `alphabet` contains APIs that are useful if you're doing low level things
with the DFAs in this crate. For example, implementing determinization or
walking its state graph directly.
* `captures` contains APIs for dealing with capture group matches and their
mapping to "slots" used inside an NFA graph. This is also where you can find
iterators over capture group names.
* `escape` contains types for pretty-printing raw byte slices as strings.
* `iter` contains API helpers for writing regex iterators.
* `lazy` contains a no-std and no-alloc variant of `lazy_static!` and
`once_cell`.
* `look` contains APIs for matching and configuring look-around assertions.
* `pool` provides a way to reuse mutable memory allocated in a thread safe
manner.
* `prefilter` provides APIs for building prefilters and using them in searches.
* `primitives` are what you might use if you're doing lower level work on
automata, such as walking an NFA state graph.
* `syntax` provides some higher level convenience functions for interacting
with the `regex-syntax` crate.
* `wire` is useful if you're working with DFA serialization.

```rust
pub mod util { /* ... */ }
```

### Modules

## Module `alphabet`

This module provides APIs for dealing with the alphabets of finite state
machines.

There are two principal types in this module, [`ByteClasses`] and [`Unit`].
The former defines the alphabet of a finite state machine while the latter
represents an element of that alphabet.

To a first approximation, the alphabet of all automata in this crate is just
a `u8`. Namely, every distinct byte value. All 256 of them. In practice, this
can be quite wasteful when building a transition table for a DFA, since it
requires storing a state identifier for each element in the alphabet. Instead,
we collapse the alphabet of an automaton down into equivalence classes, where
every byte in the same equivalence class never discriminates between a match or
a non-match from any other byte in the same class. For example, in the regex
`[a-z]+`, then you could consider it having an alphabet consisting of two
equivalence classes: `a-z` and everything else. In terms of the transitions on
an automaton, it doesn't actually require representing every distinct byte.
Just the equivalence classes.

The downside of equivalence classes is that, of course, searching a haystack
deals with individual byte values. Those byte values need to be mapped to
their corresponding equivalence class. This is what `ByteClasses` does. In
practice, doing this for every state transition has negligible impact on modern
CPUs. Moreover, it helps make more efficient use of the CPU cache by (possibly
considerably) shrinking the size of the transition table.

One last hiccup concerns `Unit`. Namely, because of look-around and how the
DFAs in this crate work, we need to add a sentinel value to our alphabet
of equivalence classes that represents the "end" of a search. We call that
sentinel [`Unit::eoi`] or "end of input." Thus, a `Unit` is either an
equivalence class corresponding to a set of bytes, or it is a special "end of
input" sentinel.

In general, you should not expect to need either of these types unless you're
doing lower level shenanigans with DFAs, or even building your own DFAs.
(Although, you don't have to use these types to build your own DFAs of course.)
For example, if you're walking a DFA's state graph, it's probably useful to
make use of [`ByteClasses`] to visit each element in the DFA's alphabet instead
of just visiting every distinct `u8` value. The latter isn't necessarily wrong,
but it could be potentially very wasteful.

```rust
pub mod alphabet { /* ... */ }
```

### Types

#### Struct `Unit`

Unit represents a single unit of haystack for DFA based regex engines.

It is not expected for consumers of this crate to need to use this type
unless they are implementing their own DFA. And even then, it's not
required: implementors may use other techniques to handle haystack units.

Typically, a single unit of haystack for a DFA would be a single byte.
However, for the DFAs in this crate, matches are delayed by a single byte
in order to handle look-ahead assertions (`\b`, `$` and `\z`). Thus, once
we have consumed the haystack, we must run the DFA through one additional
transition using a unit that indicates the haystack has ended.

There is no way to represent a sentinel with a `u8` since all possible
values *may* be valid haystack units to a DFA, therefore this type
explicitly adds room for a sentinel value.

The sentinel EOI value is always its own equivalence class and is
ultimately represented by adding 1 to the maximum equivalence class value.
So for example, the regex `^[a-z]+$` might be split into the following
equivalence classes:

```text
0 => [\x00-`]
1 => [a-z]
2 => [{-\xFF]
3 => [EOI]
```

Where EOI is the special sentinel value that is always in its own
singleton equivalence class.

```rust
pub struct Unit(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn u8(byte: u8) -> Unit { /* ... */ }
  ```
  Create a new haystack unit from a byte value.

- ```rust
  pub fn eoi(num_byte_equiv_classes: usize) -> Unit { /* ... */ }
  ```
  Create a new "end of input" haystack unit.

- ```rust
  pub fn as_u8(self: Self) -> Option<u8> { /* ... */ }
  ```
  If this unit is not an "end of input" sentinel, then returns its

- ```rust
  pub fn as_eoi(self: Self) -> Option<u16> { /* ... */ }
  ```
  If this unit is an "end of input" sentinel, then return the underlying

- ```rust
  pub fn as_usize(self: Self) -> usize { /* ... */ }
  ```
  Return this unit as a `usize`, regardless of whether it is a byte value

- ```rust
  pub fn is_byte(self: Self, byte: u8) -> bool { /* ... */ }
  ```
  Returns true if and only of this unit is a byte value equivalent to the

- ```rust
  pub fn is_eoi(self: Self) -> bool { /* ... */ }
  ```
  Returns true when this unit represents an "end of input" sentinel.

- ```rust
  pub fn is_word_byte(self: Self) -> bool { /* ... */ }
  ```
  Returns true when this unit corresponds to an ASCII word byte.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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
    fn clone(self: &Self) -> Unit { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Unit) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Unit) -> bool { /* ... */ }
    ```

- **Freeze**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Unit) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **StructuralPartialEq**
- **Send**
#### Struct `ByteClasses`

A representation of byte oriented equivalence classes.

This is used in a DFA to reduce the size of the transition table. This can
have a particularly large impact not only on the total size of a dense DFA,
but also on compile times.

The essential idea here is that the alphabet of a DFA is shrunk from the
usual 256 distinct byte values down to a set of equivalence classes. The
guarantee you get is that any byte belonging to the same equivalence class
can be treated as if it were any other byte in the same class, and the
result of a search wouldn't change.

# Example

This example shows how to get byte classes from an
[`NFA`](crate::nfa::thompson::NFA) and ask for the class of various bytes.

```
use regex_automata::nfa::thompson::NFA;

let nfa = NFA::new("[a-z]+")?;
let classes = nfa.byte_classes();
// 'a' and 'z' are in the same class for this regex.
assert_eq!(classes.get(b'a'), classes.get(b'z'));
// But 'a' and 'A' are not.
assert_ne!(classes.get(b'a'), classes.get(b'A'));

# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub struct ByteClasses(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn empty() -> ByteClasses { /* ... */ }
  ```
  Creates a new set of equivalence classes where all bytes are mapped to

- ```rust
  pub fn singletons() -> ByteClasses { /* ... */ }
  ```
  Creates a new set of equivalence classes where each byte belongs to

- ```rust
  pub fn set(self: &mut Self, byte: u8, class: u8) { /* ... */ }
  ```
  Set the equivalence class for the given byte.

- ```rust
  pub fn get(self: &Self, byte: u8) -> u8 { /* ... */ }
  ```
  Get the equivalence class for the given byte.

- ```rust
  pub fn get_by_unit(self: &Self, unit: Unit) -> usize { /* ... */ }
  ```
  Get the equivalence class for the given haystack unit and return the

- ```rust
  pub fn eoi(self: &Self) -> Unit { /* ... */ }
  ```
  Create a unit that represents the "end of input" sentinel based on the

- ```rust
  pub fn alphabet_len(self: &Self) -> usize { /* ... */ }
  ```
  Return the total number of elements in the alphabet represented by

- ```rust
  pub fn stride2(self: &Self) -> usize { /* ... */ }
  ```
  Returns the stride, as a base-2 exponent, required for these

- ```rust
  pub fn is_singleton(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if every byte in this class maps to its own

- ```rust
  pub fn iter(self: &Self) -> ByteClassIter<''_> { /* ... */ }
  ```
  Returns an iterator over all equivalence classes in this set.

- ```rust
  pub fn representatives<R: core::ops::RangeBounds<u8>>(self: &Self, range: R) -> ByteClassRepresentatives<''_> { /* ... */ }
  ```
  Returns an iterator over a sequence of representative bytes from each

- ```rust
  pub fn elements(self: &Self, class: Unit) -> ByteClassElements<''_> { /* ... */ }
  ```
  Returns an iterator of the bytes in the given equivalence class.

###### Trait Implementations

- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ByteClasses { /* ... */ }
    ```

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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

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

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ByteClasses { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `ByteClassIter`

An iterator over each equivalence class.

The last element in this iterator always corresponds to [`Unit::eoi`].

This is created by the [`ByteClasses::iter`] method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

```rust
pub struct ByteClassIter<''a> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Unit> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **RefUnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
#### Struct `ByteClassRepresentatives`

An iterator over representative bytes from each equivalence class.

This is created by the [`ByteClasses::representatives`] method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

```rust
pub struct ByteClassRepresentatives<''a> {
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

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Unit> { /* ... */ }
    ```

- **Sync**
#### Struct `ByteClassElements`

An iterator over all elements in an equivalence class.

This is created by the [`ByteClasses::elements`] method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

```rust
pub struct ByteClassElements<''a> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Unit> { /* ... */ }
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

- **Unpin**
## Module `captures`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

Provides types for dealing with capturing groups.

Capturing groups refer to sub-patterns of regexes that some regex engines can
report matching offsets for. For example, matching `[a-z]([0-9]+)` against
`a789` would give `a789` as the overall match (for the implicit capturing group
at index `0`) and `789` as the match for the capturing group `([0-9]+)` (an
explicit capturing group at index `1`).

Not all regex engines can report match offsets for capturing groups. Indeed,
to a first approximation, regex engines that can report capturing group offsets
tend to be quite a bit slower than regex engines that can't. This is because
tracking capturing groups at search time usually requires more "power" that
in turn adds overhead.

Other regex implementations might call capturing groups "submatches."

# Overview

The main types in this module are:

* [`Captures`] records the capturing group offsets found during a search. It
provides convenience routines for looking up capturing group offsets by either
index or name.
* [`GroupInfo`] records the mapping between capturing groups and "slots,"
where the latter are how capturing groups are recorded during a regex search.
This also keeps a mapping from capturing group name to index, and capture
group index to name. A `GroupInfo` is used by `Captures` internally to
provide a convenient API. It is unlikely that you'll use a `GroupInfo`
directly, but for example, if you've compiled an Thompson NFA, then you can use
[`thompson::NFA::group_info`](crate::nfa::thompson::NFA::group_info) to get its
underlying `GroupInfo`.

```rust
pub mod captures { /* ... */ }
```

### Types

#### Struct `Captures`

The span offsets of capturing groups after a match has been found.

This type represents the output of regex engines that can report the
offsets at which capturing groups matches or "submatches" occur. For
example, the [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM). When a match
occurs, it will at minimum contain the [`PatternID`] of the pattern that
matched. Depending upon how it was constructed, it may also contain the
start/end offsets of the entire match of the pattern and the start/end
offsets of each capturing group that participated in the match.

Values of this type are always created for a specific [`GroupInfo`]. It is
unspecified behavior to use a `Captures` value in a search with any regex
engine that has a different `GroupInfo` than the one the `Captures` were
created with.

# Constructors

There are three constructors for this type that control what kind of
information is available upon a match:

* [`Captures::all`]: Will store overall pattern match offsets in addition
to the offsets of capturing groups that participated in the match.
* [`Captures::matches`]: Will store only the overall pattern
match offsets. The offsets of capturing groups (even ones that participated
in the match) are not available.
* [`Captures::empty`]: Will only store the pattern ID that matched. No
match offsets are available at all.

If you aren't sure which to choose, then pick the first one. The first one
is what convenience routines like,
[`PikeVM::create_captures`](crate::nfa::thompson::pikevm::PikeVM::create_captures),
will use automatically.

The main difference between these choices is performance. Namely, if you
ask for _less_ information, then the execution of regex search may be able
to run more quickly.

# Notes

It is worth pointing out that this type is not coupled to any one specific
regex engine. Instead, its coupling is with [`GroupInfo`], which is the
thing that is responsible for mapping capturing groups to "slot" offsets.
Slot offsets are indices into a single sequence of memory at which matching
haystack offsets for the corresponding group are written by regex engines.

# Example

This example shows how to parse a simple date and extract the components of
the date via capturing groups:

```
use regex_automata::{nfa::thompson::pikevm::PikeVM, Span};

let re = PikeVM::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

re.captures(&mut cache, "2010-03-14", &mut caps);
assert!(caps.is_match());
assert_eq!(Some(Span::from(0..4)), caps.get_group(1));
assert_eq!(Some(Span::from(5..7)), caps.get_group(2));
assert_eq!(Some(Span::from(8..10)), caps.get_group(3));

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: named capturing groups

This example is like the one above, but leverages the ability to name
capturing groups in order to make the code a bit clearer:

```
use regex_automata::{nfa::thompson::pikevm::PikeVM, Span};

let re = PikeVM::new(r"^(?P<y>[0-9]{4})-(?P<m>[0-9]{2})-(?P<d>[0-9]{2})$")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

re.captures(&mut cache, "2010-03-14", &mut caps);
assert!(caps.is_match());
assert_eq!(Some(Span::from(0..4)), caps.get_group_by_name("y"));
assert_eq!(Some(Span::from(5..7)), caps.get_group_by_name("m"));
assert_eq!(Some(Span::from(8..10)), caps.get_group_by_name("d"));

# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub struct Captures {
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
  pub fn all(group_info: GroupInfo) -> Captures { /* ... */ }
  ```
  Create new storage for the offsets of all matching capturing groups.

- ```rust
  pub fn matches(group_info: GroupInfo) -> Captures { /* ... */ }
  ```
  Create new storage for only the full match spans of a pattern. This

- ```rust
  pub fn empty(group_info: GroupInfo) -> Captures { /* ... */ }
  ```
  Create new storage for only tracking which pattern matched. No offsets

- ```rust
  pub fn is_match(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this capturing group represents a match.

- ```rust
  pub fn pattern(self: &Self) -> Option<PatternID> { /* ... */ }
  ```
  Returns the identifier of the pattern that matched when this

- ```rust
  pub fn get_match(self: &Self) -> Option<Match> { /* ... */ }
  ```
  Returns the pattern ID and the span of the match, if one occurred.

- ```rust
  pub fn get_group(self: &Self, index: usize) -> Option<Span> { /* ... */ }
  ```
  Returns the span of a capturing group match corresponding to the group

- ```rust
  pub fn get_group_by_name(self: &Self, name: &str) -> Option<Span> { /* ... */ }
  ```
  Returns the span of a capturing group match corresponding to the group

- ```rust
  pub fn iter(self: &Self) -> CapturesPatternIter<''_> { /* ... */ }
  ```
  Returns an iterator of possible spans for every capturing group in the

- ```rust
  pub fn group_len(self: &Self) -> usize { /* ... */ }
  ```
  Return the total number of capturing groups for the matching pattern.

- ```rust
  pub fn group_info(self: &Self) -> &GroupInfo { /* ... */ }
  ```
  Returns a reference to the underlying group info on which these

- ```rust
  pub fn interpolate_string(self: &Self, haystack: &str, replacement: &str) -> String { /* ... */ }
  ```
  Interpolates the capture references in `replacement` with the

- ```rust
  pub fn interpolate_string_into(self: &Self, haystack: &str, replacement: &str, dst: &mut String) { /* ... */ }
  ```
  Interpolates the capture references in `replacement` with the

- ```rust
  pub fn interpolate_bytes(self: &Self, haystack: &[u8], replacement: &[u8]) -> Vec<u8> { /* ... */ }
  ```
  Interpolates the capture references in `replacement` with the

- ```rust
  pub fn interpolate_bytes_into(self: &Self, haystack: &[u8], replacement: &[u8], dst: &mut Vec<u8>) { /* ... */ }
  ```
  Interpolates the capture references in `replacement` with the

- ```rust
  pub fn extract<''h, const N: usize>(self: &Self, haystack: &''h str) -> (&''h str, [&''h str; N]) { /* ... */ }
  ```
  This is a convenience routine for extracting the substrings

- ```rust
  pub fn extract_bytes<''h, const N: usize>(self: &Self, haystack: &''h [u8]) -> (&''h [u8], [&''h [u8]; N]) { /* ... */ }
  ```
  This is a convenience routine for extracting the substrings

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clear this `Captures` value.

- ```rust
  pub fn set_pattern(self: &mut Self, pid: Option<PatternID>) { /* ... */ }
  ```
  Set the pattern on this `Captures` value.

- ```rust
  pub fn slots(self: &Self) -> &[Option<NonMaxUsize>] { /* ... */ }
  ```
  Returns the underlying slots, where each slot stores a single offset.

- ```rust
  pub fn slots_mut(self: &mut Self) -> &mut [Option<NonMaxUsize>] { /* ... */ }
  ```
  Returns the underlying slots as a mutable slice, where each slot stores

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Captures { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Sync**
#### Struct `CapturesPatternIter`

An iterator over all capturing groups in a `Captures` value.

This iterator includes capturing groups that did not participate in a
match. See the [`Captures::iter`] method documentation for more details
and examples.

The lifetime parameter `'a` refers to the lifetime of the underlying
`Captures` value.

```rust
pub struct CapturesPatternIter<''a> {
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Option<Span>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

  - ```rust
    fn count(self: Self) -> usize { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CapturesPatternIter<''a> { /* ... */ }
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
- **ExactSizeIterator**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **FusedIterator**
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

#### Struct `GroupInfo`

Represents information about capturing groups in a compiled regex.

The information encapsulated by this type consists of the following. For
each pattern:

* A map from every capture group name to its corresponding capture group
index.
* A map from every capture group index to its corresponding capture group
name.
* A map from capture group index to its corresponding slot index. A slot
refers to one half of a capturing group. That is, a capture slot is either
the start or end of a capturing group. A slot is usually the mechanism
by which a regex engine records offsets for each capturing group during a
search.

A `GroupInfo` uses reference counting internally and is thus cheap to
clone.

# Mapping from capture groups to slots

One of the main responsibilities of a `GroupInfo` is to build a mapping
from `(PatternID, u32)` (where the `u32` is a capture index) to something
called a "slot." As mentioned above, a slot refers to one half of a
capturing group. Both combined provide the start and end offsets of
a capturing group that participated in a match.

**The mapping between group indices and slots is an API guarantee.** That
is, the mapping won't change within a semver compatible release.

Slots exist primarily because this is a convenient mechanism by which
regex engines report group offsets at search time. For example, the
[`nfa::thompson::State::Capture`](crate::nfa::thompson::State::Capture)
NFA state includes the slot index. When a regex engine transitions through
this state, it will likely use the slot index to write the current haystack
offset to some region of memory. When a match is found, those slots are
then reported to the caller, typically via a convenient abstraction like a
[`Captures`] value.

Because this crate provides first class support for multi-pattern regexes,
and because of some performance related reasons, the mapping between
capturing groups and slots is a little complex. However, in the case of a
single pattern, the mapping can be described very simply: for all capture
group indices `i`, its corresponding slots are at `i * 2` and `i * 2 + 1`.
Notice that the pattern ID isn't involved at all here, because it only
applies to a single-pattern regex, it is therefore always `0`.

In the multi-pattern case, the mapping is a bit more complicated. To talk
about it, we must define what we mean by "implicit" vs "explicit"
capturing groups:

* An **implicit** capturing group refers to the capturing group that is
present for every pattern automatically, and corresponds to the overall
match of a pattern. Every pattern has precisely one implicit capturing
group. It is always unnamed and it always corresponds to the capture group
index `0`.
* An **explicit** capturing group refers to any capturing group that
appears in the concrete syntax of the pattern. (Or, if an NFA was hand
built without any concrete syntax, it refers to any capturing group with an
index greater than `0`.)

Some examples:

* `\w+` has one implicit capturing group and zero explicit capturing
groups.
* `(\w+)` has one implicit group and one explicit group.
* `foo(\d+)(?:\pL+)(\d+)` has one implicit group and two explicit groups.

Turning back to the slot mapping, we can now state it as follows:

* Given a pattern ID `pid`, the slots for its implicit group are always
at `pid * 2` and `pid * 2 + 1`.
* Given a pattern ID `0`, the slots for its explicit groups start
at `group_info.pattern_len() * 2`.
* Given a pattern ID `pid > 0`, the slots for its explicit groups start
immediately following where the slots for the explicit groups of `pid - 1`
end.

In particular, while there is a concrete formula one can use to determine
where the slots for the implicit group of any pattern are, there is no
general formula for determining where the slots for explicit capturing
groups are. This is because each pattern can contain a different number
of groups.

The intended way of getting the slots for a particular capturing group
(whether implicit or explicit) is via the [`GroupInfo::slot`] or
[`GroupInfo::slots`] method.

See below for a concrete example of how capturing groups get mapped to
slots.

# Example

This example shows how to build a new `GroupInfo` and query it for
information.

```
use regex_automata::util::{captures::GroupInfo, primitives::PatternID};

let info = GroupInfo::new(vec![
    vec![None, Some("foo")],
    vec![None],
    vec![None, None, None, Some("bar"), None],
    vec![None, None, Some("foo")],
])?;
// The number of patterns being tracked.
assert_eq!(4, info.pattern_len());
// We can query the number of groups for any pattern.
assert_eq!(2, info.group_len(PatternID::must(0)));
assert_eq!(1, info.group_len(PatternID::must(1)));
assert_eq!(5, info.group_len(PatternID::must(2)));
assert_eq!(3, info.group_len(PatternID::must(3)));
// An invalid pattern always has zero groups.
assert_eq!(0, info.group_len(PatternID::must(999)));
// 2 slots per group
assert_eq!(22, info.slot_len());

// We can map a group index for a particular pattern to its name, if
// one exists.
assert_eq!(Some("foo"), info.to_name(PatternID::must(3), 2));
assert_eq!(None, info.to_name(PatternID::must(2), 4));
// Or map a name to its group index.
assert_eq!(Some(1), info.to_index(PatternID::must(0), "foo"));
assert_eq!(Some(2), info.to_index(PatternID::must(3), "foo"));

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: mapping from capture groups to slots

This example shows the specific mapping from capture group indices for
each pattern to their corresponding slots. The slot values shown in this
example are considered an API guarantee.

```
use regex_automata::util::{captures::GroupInfo, primitives::PatternID};

let info = GroupInfo::new(vec![
    vec![None, Some("foo")],
    vec![None],
    vec![None, None, None, Some("bar"), None],
    vec![None, None, Some("foo")],
])?;

// We first show the slots for each pattern's implicit group.
assert_eq!(Some((0, 1)), info.slots(PatternID::must(0), 0));
assert_eq!(Some((2, 3)), info.slots(PatternID::must(1), 0));
assert_eq!(Some((4, 5)), info.slots(PatternID::must(2), 0));
assert_eq!(Some((6, 7)), info.slots(PatternID::must(3), 0));

// And now we show the slots for each pattern's explicit group.
assert_eq!(Some((8, 9)), info.slots(PatternID::must(0), 1));
assert_eq!(Some((10, 11)), info.slots(PatternID::must(2), 1));
assert_eq!(Some((12, 13)), info.slots(PatternID::must(2), 2));
assert_eq!(Some((14, 15)), info.slots(PatternID::must(2), 3));
assert_eq!(Some((16, 17)), info.slots(PatternID::must(2), 4));
assert_eq!(Some((18, 19)), info.slots(PatternID::must(3), 1));
assert_eq!(Some((20, 21)), info.slots(PatternID::must(3), 2));

// Asking for the slots for an invalid pattern ID or even for an invalid
// group index for a specific pattern will return None. So for example,
// you're guaranteed to not get the slots for a different pattern than the
// one requested.
assert_eq!(None, info.slots(PatternID::must(5), 0));
assert_eq!(None, info.slots(PatternID::must(1), 1));

# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub struct GroupInfo(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new<P, G, N>(pattern_groups: P) -> Result<GroupInfo, GroupInfoError>
where
    P: IntoIterator<Item = G>,
    G: IntoIterator<Item = Option<N>>,
    N: AsRef<str> { /* ... */ }
  ```
  Creates a new group info from a sequence of patterns, where each

- ```rust
  pub fn empty() -> GroupInfo { /* ... */ }
  ```
  This creates an empty `GroupInfo`.

- ```rust
  pub fn to_index(self: &Self, pid: PatternID, name: &str) -> Option<usize> { /* ... */ }
  ```
  Return the capture group index corresponding to the given name in the

- ```rust
  pub fn to_name(self: &Self, pid: PatternID, group_index: usize) -> Option<&str> { /* ... */ }
  ```
  Return the capture name for the given index and given pattern. If the

- ```rust
  pub fn pattern_names(self: &Self, pid: PatternID) -> GroupInfoPatternNames<''_> { /* ... */ }
  ```
  Return an iterator of all capture groups and their names (if present)

- ```rust
  pub fn all_names(self: &Self) -> GroupInfoAllNames<''_> { /* ... */ }
  ```
  Return an iterator of all capture groups for all patterns supported by

- ```rust
  pub fn slots(self: &Self, pid: PatternID, group_index: usize) -> Option<(usize, usize)> { /* ... */ }
  ```
  Returns the starting and ending slot corresponding to the given

- ```rust
  pub fn slot(self: &Self, pid: PatternID, group_index: usize) -> Option<usize> { /* ... */ }
  ```
  Returns the starting slot corresponding to the given capturing group

- ```rust
  pub fn pattern_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of patterns in this `GroupInfo`.

- ```rust
  pub fn group_len(self: &Self, pid: PatternID) -> usize { /* ... */ }
  ```
  Return the number of capture groups in a pattern.

- ```rust
  pub fn all_group_len(self: &Self) -> usize { /* ... */ }
  ```
  Return the total number of capture groups across all patterns.

- ```rust
  pub fn slot_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of slots in this `GroupInfo` across all

- ```rust
  pub fn implicit_slot_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of slots for implicit capturing groups.

- ```rust
  pub fn explicit_slot_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of slots for explicit capturing groups.

- ```rust
  pub fn memory_usage(self: &Self) -> usize { /* ... */ }
  ```
  Returns the memory usage, in bytes, of this `GroupInfo`.

###### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Freeze**
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
    fn clone(self: &Self) -> GroupInfo { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
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

- **Default**
  - ```rust
    fn default() -> GroupInfo { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `GroupInfoError`

An error that may occur when building a `GroupInfo`.

Building a `GroupInfo` does a variety of checks to make sure the
capturing groups satisfy a number of invariants. This includes, but is not
limited to, ensuring that the first capturing group is unnamed and that
there are no duplicate capture groups for a specific pattern.

```rust
pub struct GroupInfoError {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn std::error::Error + ''static> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
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
    fn clone(self: &Self) -> GroupInfoError { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

#### Struct `GroupInfoPatternNames`

An iterator over capturing groups and their names for a specific pattern.

This iterator is created by [`GroupInfo::pattern_names`].

The lifetime parameter `'a` refers to the lifetime of the `GroupInfo`
from which this iterator was created.

```rust
pub struct GroupInfoPatternNames<''a> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Option<&''a str>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

  - ```rust
    fn count(self: Self) -> usize { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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
    fn clone(self: &Self) -> GroupInfoPatternNames<''a> { /* ... */ }
    ```

- **ExactSizeIterator**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Send**
- **FusedIterator**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
#### Struct `GroupInfoAllNames`

An iterator over capturing groups and their names for a `GroupInfo`.

This iterator is created by [`GroupInfo::all_names`].

The lifetime parameter `'a` refers to the lifetime of the `GroupInfo`
from which this iterator was created.

```rust
pub struct GroupInfoAllNames<''a> {
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
    fn next(self: &mut Self) -> Option<(PatternID, usize, Option<&''a str>)> { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **RefUnwindSafe**
## Module `escape`

Provides convenience routines for escaping raw bytes.

Since this crate tends to deal with `&[u8]` everywhere and the default
`Debug` implementation just shows decimal integers, it makes debugging those
representations quite difficult. This module provides types that show `&[u8]`
as if it were a string, with invalid UTF-8 escaped into its byte-by-byte hex
representation.

```rust
pub mod escape { /* ... */ }
```

### Types

#### Struct `DebugByte`

Provides a convenient `Debug` implementation for a `u8`.

The `Debug` impl treats the byte as an ASCII, and emits a human readable
representation of it. If the byte isn't ASCII, then it's emitted as a hex
escape sequence.

```rust
pub struct DebugByte(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DebugByte { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **RefUnwindSafe**
#### Struct `DebugHaystack`

Provides a convenient `Debug` implementation for `&[u8]`.

This generally works best when the bytes are presumed to be mostly UTF-8,
but will work for anything. For any bytes that aren't UTF-8, they are
emitted as hex escape sequences.

```rust
pub struct DebugHaystack<''a>(pub &''a [u8]);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a [u8]` |  |

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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Send**
- **Unpin**
- **UnwindSafe**
## Module `interpolate`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

Provides routines for interpolating capture group references.

That is, if a replacement string contains references like `$foo` or `${foo1}`,
then they are replaced with the corresponding capture values for the groups
named `foo` and `foo1`, respectively. Similarly, syntax like `$1` and `${1}`
is supported as well, with `1` corresponding to a capture group index and not
a name.

This module provides the free functions [`string`] and [`bytes`], which
interpolate Rust Unicode strings and byte strings, respectively.

# Format

These routines support two different kinds of capture references: unbraced and
braced.

For the unbraced format, the format supported is `$ref` where `name` can be
any character in the class `[0-9A-Za-z_]`. `ref` is always the longest
possible parse. So for example, `$1a` corresponds to the capture group named
`1a` and not the capture group at index `1`. If `ref` matches `^[0-9]+$`, then
it is treated as a capture group index itself and not a name.

For the braced format, the format supported is `${ref}` where `ref` can be any
sequence of bytes except for `}`. If no closing brace occurs, then it is not
considered a capture reference. As with the unbraced format, if `ref` matches
`^[0-9]+$`, then it is treated as a capture group index and not a name.

The braced format is useful for exerting precise control over the name of the
capture reference. For example, `${1}a` corresponds to the capture group
reference `1` followed by the letter `a`, where as `$1a` (as mentioned above)
corresponds to the capture group reference `1a`. The braced format is also
useful for expressing capture group names that use characters not supported by
the unbraced format. For example, `${foo[bar].baz}` refers to the capture group
named `foo[bar].baz`.

If a capture group reference is found and it does not refer to a valid capture
group, then it will be replaced with the empty string.

To write a literal `$`, use `$$`.

To be clear, and as exhibited via the type signatures in the routines in this
module, it is impossible for a replacement string to be invalid. A replacement
string may not have the intended semantics, but the interpolation procedure
itself can never fail.

```rust
pub mod interpolate { /* ... */ }
```

### Functions

#### Function `string`

Accepts a replacement string and interpolates capture references with their
corresponding values.

`append` should be a function that appends the string value of a capture
group at a particular index to the string given. If the capture group
index is invalid, then nothing should be appended.

`name_to_index` should be a function that maps a capture group name to a
capture group index. If the given name doesn't exist, then `None` should
be returned.

Finally, `dst` is where the final interpolated contents should be written.
If `replacement` contains no capture group references, then `dst` will be
equivalent to `replacement`.

See the [module documentation](self) for details about the format
supported.

# Example

```
use regex_automata::util::interpolate;

let mut dst = String::new();
interpolate::string(
    "foo $bar baz",
    |index, dst| {
        if index == 0 {
            dst.push_str("BAR");
        }
    },
    |name| {
        if name == "bar" {
            Some(0)
        } else {
            None
        }
    },
    &mut dst,
);
assert_eq!("foo BAR baz", dst);
```

```rust
pub fn string</* synthetic */ impl FnMut(usize, &mut String): FnMut(usize, &mut alloc::string::String), /* synthetic */ impl FnMut(&str) -> Option<usize>: FnMut(&str) -> Option<usize>>(replacement: &str, append: impl FnMut(usize, &mut alloc::string::String), name_to_index: impl FnMut(&str) -> Option<usize>, dst: &mut alloc::string::String) { /* ... */ }
```

#### Function `bytes`

Accepts a replacement byte string and interpolates capture references with
their corresponding values.

`append` should be a function that appends the byte string value of a
capture group at a particular index to the byte string given. If the
capture group index is invalid, then nothing should be appended.

`name_to_index` should be a function that maps a capture group name to a
capture group index. If the given name doesn't exist, then `None` should
be returned.

Finally, `dst` is where the final interpolated contents should be written.
If `replacement` contains no capture group references, then `dst` will be
equivalent to `replacement`.

See the [module documentation](self) for details about the format
supported.

# Example

```
use regex_automata::util::interpolate;

let mut dst = vec![];
interpolate::bytes(
    b"foo $bar baz",
    |index, dst| {
        if index == 0 {
            dst.extend_from_slice(b"BAR");
        }
    },
    |name| {
        if name == "bar" {
            Some(0)
        } else {
            None
        }
    },
    &mut dst,
);
assert_eq!(&b"foo BAR baz"[..], dst);
```

```rust
pub fn bytes</* synthetic */ impl FnMut(usize, &mut Vec<u8>): FnMut(usize, &mut alloc::vec::Vec<u8>), /* synthetic */ impl FnMut(&str) -> Option<usize>: FnMut(&str) -> Option<usize>>(replacement: &[u8], append: impl FnMut(usize, &mut alloc::vec::Vec<u8>), name_to_index: impl FnMut(&str) -> Option<usize>, dst: &mut alloc::vec::Vec<u8>) { /* ... */ }
```

## Module `iter`

Generic helpers for iteration of matches from a regex engine in a haystack.

The principle type in this module is a [`Searcher`]. A `Searcher` provides
its own lower level iterator-like API in addition to methods for constructing
types that implement `Iterator`. The documentation for `Searcher` explains a
bit more about why these different APIs exist.

Currently, this module supports iteration over any regex engine that works
with the [`HalfMatch`], [`Match`] or [`Captures`] types.

```rust
pub mod iter { /* ... */ }
```

### Types

#### Struct `Searcher`

A searcher for creating iterators and performing lower level iteration.

This searcher encapsulates the logic required for finding all successive
non-overlapping matches in a haystack. In theory, iteration would look
something like this:

1. Setting the start position to `0`.
2. Execute a regex search. If no match, end iteration.
3. Report the match and set the start position to the end of the match.
4. Go back to (2).

And if this were indeed the case, it's likely that `Searcher` wouldn't
exist. Unfortunately, because a regex may match the empty string, the above
logic won't work for all possible regexes. Namely, if an empty match is
found, then step (3) would set the start position of the search to the
position it was at. Thus, iteration would never end.

Instead, a `Searcher` knows how to detect these cases and forcefully
advance iteration in the case of an empty match that overlaps with a
previous match.

If you know that your regex cannot match any empty string, then the simple
algorithm described above will work correctly.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

In particular, a `Searcher` is not itself an iterator. Instead, it provides
`advance` routines that permit moving the search along explicitly. It also
provides various routines, like [`Searcher::into_matches_iter`], that
accept a closure (representing how a regex engine executes a search) and
returns a conventional iterator.

The lifetime parameters come from the [`Input`] type passed to
[`Searcher::new`]:

* `'h` is the lifetime of the underlying haystack.

# Searcher vs Iterator

Why does a search type with "advance" APIs exist at all when we also have
iterators? Unfortunately, the reasoning behind this split is a complex
combination of the following things:

1. While many of the regex engines expose their own iterators, it is also
nice to expose this lower level iteration helper because it permits callers
to provide their own `Input` configuration. Moreover, a `Searcher` can work
with _any_ regex engine instead of only the ones defined in this crate.
This way, everyone benefits from a shared iteration implementation.
2. There are many different regex engines that, while they have the same
match semantics, they have slightly different APIs. Iteration is just
complex enough to want to share code, and so we need a way of abstracting
over those different regex engines. While we could define a new trait that
describes any regex engine search API, it would wind up looking very close
to a closure. While there may still be reasons for the more generic trait
to exist, for now and for the purposes of iteration, we use a closure.
Closures also provide a lot of easy flexibility at the call site, in that
they permit the caller to borrow any kind of state they want for use during
each search call.
3. As a result of using closures, and because closures are anonymous types
that cannot be named, it is difficult to encapsulate them without both
costs to speed and added complexity to the public API. For example, in
defining an iterator type like
[`dfa::regex::FindMatches`](crate::dfa::regex::FindMatches),
if we use a closure internally, it's not possible to name this type in the
return type of the iterator constructor. Thus, the only way around it is
to erase the type by boxing it and turning it into a `Box<dyn FnMut ...>`.
This boxed closure is unlikely to be inlined _and_ it infects the public
API in subtle ways. Namely, unless you declare the closure as implementing
`Send` and `Sync`, then the resulting iterator type won't implement it
either. But there are practical issues with requiring the closure to
implement `Send` and `Sync` that result in other API complexities that
are beyond the scope of this already long exposition.
4. Some regex engines expose more complex match information than just
"which pattern matched" and "at what offsets." For example, the PikeVM
exposes match spans for each capturing group that participated in the
match. In such cases, it can be quite beneficial to reuse the capturing
group allocation on subsequent searches. A proper iterator doesn't permit
this API due to its interface, so it's useful to have something a bit lower
level that permits callers to amortize allocations while also reusing a
shared implementation of iteration. (See the documentation for
[`Searcher::advance`] for an example of using the "advance" API with the
PikeVM.)

What this boils down to is that there are "advance" APIs which require
handing a closure to it for every call, and there are also APIs to create
iterators from a closure. The former are useful for _implementing_
iterators or when you need more flexibility, while the latter are useful
for conveniently writing custom iterators on-the-fly.

# Example: iterating with captures

Several regex engines in this crate over convenient iterator APIs over
[`Captures`] values. To do so, this requires allocating a new `Captures`
value for each iteration step. This can perhaps be more costly than you
might want. Instead of implementing your own iterator to avoid that
cost (which can be a little subtle if you want to handle empty matches
correctly), you can use this `Searcher` to do it for you:

```
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    util::iter::Searcher,
    Input, Span,
};

let re = PikeVM::new("foo(?P<numbers>[0-9]+)")?;
let haystack = "foo1 foo12 foo123";

let mut caps = re.create_captures();
let mut cache = re.create_cache();
let mut matches = vec![];
let mut searcher = Searcher::new(Input::new(haystack));
while let Some(_) = searcher.advance(|input| {
    re.search(&mut cache, input, &mut caps);
    Ok(caps.get_match())
}) {
    // The unwrap is OK since 'numbers' matches if the pattern matches.
    matches.push(caps.get_group_by_name("numbers").unwrap());
}
assert_eq!(matches, vec![
    Span::from(3..4),
    Span::from(8..10),
    Span::from(14..17),
]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub struct Searcher<''h> {
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
  pub fn new(input: Input<''h>) -> Searcher<''h> { /* ... */ }
  ```
  Create a new fallible non-overlapping matches iterator.

- ```rust
  pub fn input<''s>(self: &''s Self) -> &''s Input<''h> { /* ... */ }
  ```
  Returns the current `Input` used by this searcher.

- ```rust
  pub fn advance_half<F>(self: &mut Self, finder: F) -> Option<HalfMatch>
where
    F: FnMut(&Input<''_>) -> Result<Option<HalfMatch>, MatchError> { /* ... */ }
  ```
  Return the next half match for an infallible search if one exists, and

- ```rust
  pub fn advance<F>(self: &mut Self, finder: F) -> Option<Match>
where
    F: FnMut(&Input<''_>) -> Result<Option<Match>, MatchError> { /* ... */ }
  ```
  Return the next match for an infallible search if one exists, and

- ```rust
  pub fn try_advance_half<F>(self: &mut Self, finder: F) -> Result<Option<HalfMatch>, MatchError>
where
    F: FnMut(&Input<''_>) -> Result<Option<HalfMatch>, MatchError> { /* ... */ }
  ```
  Return the next half match for a fallible search if one exists, and

- ```rust
  pub fn try_advance<F>(self: &mut Self, finder: F) -> Result<Option<Match>, MatchError>
where
    F: FnMut(&Input<''_>) -> Result<Option<Match>, MatchError> { /* ... */ }
  ```
  Return the next match for a fallible search if one exists, and advance

- ```rust
  pub fn into_half_matches_iter<F>(self: Self, finder: F) -> TryHalfMatchesIter<''h, F>
where
    F: FnMut(&Input<''_>) -> Result<Option<HalfMatch>, MatchError> { /* ... */ }
  ```
  Given a closure that executes a single search, return an iterator over

- ```rust
  pub fn into_matches_iter<F>(self: Self, finder: F) -> TryMatchesIter<''h, F>
where
    F: FnMut(&Input<''_>) -> Result<Option<Match>, MatchError> { /* ... */ }
  ```
  Given a closure that executes a single search, return an iterator over

- ```rust
  pub fn into_captures_iter<F>(self: Self, caps: Captures, finder: F) -> TryCapturesIter<''h, F>
where
    F: FnMut(&Input<''_>, &mut Captures) -> Result<(), MatchError> { /* ... */ }
  ```
  Given a closure that executes a single search, return an iterator over

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Searcher<''h> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Sync**
- **Freeze**
- **RefUnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
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

#### Struct `TryHalfMatchesIter`

An iterator over all non-overlapping half matches for a fallible search.

The iterator yields a `Result<HalfMatch, MatchError>` value until no more
matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`] type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by [`Searcher::into_half_matches_iter`].

```rust
pub struct TryHalfMatchesIter<''h, F> {
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
  pub fn infallible(self: Self) -> HalfMatchesIter<''h, F> { /* ... */ }
  ```
  Return an infallible version of this iterator.

- ```rust
  pub fn input<''i>(self: &''i Self) -> &''i Input<''h> { /* ... */ }
  ```
  Returns the current `Input` used by this iterator.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Result<HalfMatch, MatchError>> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Send**
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

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `HalfMatchesIter`

An iterator over all non-overlapping half matches for an infallible search.

The iterator yields a [`HalfMatch`] value until no more matches could be
found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`] type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by [`Searcher::into_half_matches_iter`] and
then calling [`TryHalfMatchesIter::infallible`].

```rust
pub struct HalfMatchesIter<''h, F>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn input<''i>(self: &''i Self) -> &''i Input<''h> { /* ... */ }
  ```
  Returns the current `Input` used by this iterator.

###### Trait Implementations

- **Sync**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<HalfMatch> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `TryMatchesIter`

An iterator over all non-overlapping matches for a fallible search.

The iterator yields a `Result<Match, MatchError>` value until no more
matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`] type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by [`Searcher::into_matches_iter`].

```rust
pub struct TryMatchesIter<''h, F> {
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
  pub fn infallible(self: Self) -> MatchesIter<''h, F> { /* ... */ }
  ```
  Return an infallible version of this iterator.

- ```rust
  pub fn input<''i>(self: &''i Self) -> &''i Input<''h> { /* ... */ }
  ```
  Returns the current `Input` used by this iterator.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Unpin**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Result<Match, MatchError>> { /* ... */ }
    ```

#### Struct `MatchesIter`

An iterator over all non-overlapping matches for an infallible search.

The iterator yields a [`Match`] value until no more matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`] type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by [`Searcher::into_matches_iter`] and
then calling [`TryMatchesIter::infallible`].

```rust
pub struct MatchesIter<''h, F>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn input<''i>(self: &''i Self) -> &''i Input<''h> { /* ... */ }
  ```
  Returns the current `Input` used by this iterator.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Match> { /* ... */ }
    ```

- **Send**
- **Sync**
- **RefUnwindSafe**
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

#### Struct `TryCapturesIter`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

An iterator over all non-overlapping captures for a fallible search.

The iterator yields a `Result<Captures, MatchError>` value until no more
matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`] type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by [`Searcher::into_captures_iter`].

```rust
pub struct TryCapturesIter<''h, F> {
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
  pub fn infallible(self: Self) -> CapturesIter<''h, F> { /* ... */ }
  ```
  Return an infallible version of this iterator.

###### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
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
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Result<Captures, MatchError>> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
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

- **RefUnwindSafe**
#### Struct `CapturesIter`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

An iterator over all non-overlapping captures for an infallible search.

The iterator yields a [`Captures`] value until no more matches could be
found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`] type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by [`Searcher::into_captures_iter`] and then
calling [`TryCapturesIter::infallible`].

```rust
pub struct CapturesIter<''h, F>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

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
- **Sync**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Captures> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **RefUnwindSafe**
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
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

## Module `lazy`

A lazily initialized value for safe sharing between threads.

The principal type in this module is `Lazy`, which makes it easy to construct
values that are shared safely across multiple threads simultaneously.

```rust
pub mod lazy { /* ... */ }
```

### Types

#### Struct `Lazy`

A lazily initialized value that implements `Deref` for `T`.

A `Lazy` takes an initialization function and permits callers from any
thread to access the result of that initialization function in a safe
manner. In effect, this permits one-time initialization of global resources
in a (possibly) multi-threaded program.

This type and its functionality are available even when neither the `alloc`
nor the `std` features are enabled. In exchange, a `Lazy` does **not**
guarantee that the given `create` function is called at most once. It
might be called multiple times. Moreover, a call to `Lazy::get` (either
explicitly or implicitly via `Lazy`'s `Deref` impl) may block until a `T`
is available.

This is very similar to `lazy_static` or `once_cell`, except it doesn't
guarantee that the initialization function will be run once and it works
in no-alloc no-std environments. With that said, if you need stronger
guarantees or a more flexible API, then it is recommended to use either
`lazy_static` or `once_cell`.

# Warning: may use a spin lock

When this crate is compiled _without_ the `alloc` feature, then this type
may used a spin lock internally. This can have subtle effects that may
be undesirable. See [Spinlocks Considered Harmful][spinharm] for a more
thorough treatment of this topic.

[spinharm]: https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html

# Example

This type is useful for creating regexes once, and then using them from
multiple threads simultaneously without worrying about synchronization.

```
use regex_automata::{dfa::regex::Regex, util::lazy::Lazy, Match};

static RE: Lazy<Regex> = Lazy::new(|| Regex::new("foo[0-9]+bar").unwrap());

let expected = Some(Match::must(0, 3..14));
assert_eq!(expected, RE.find(b"zzzfoo12345barzzz"));
```

```rust
pub struct Lazy<T, F = fn() -> T>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(create: F) -> Lazy<T, F> { /* ... */ }
  ```
  Create a new `Lazy` value that is initialized via the given function.

- ```rust
  pub fn get(this: &Lazy<T, F>) -> &T { /* ... */ }
  ```
  Return a reference to the lazily initialized value.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Receiver**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `look`

Types and routines for working with look-around assertions.

This module principally defines two types:

* [`Look`] enumerates all of the assertions supported by this crate.
* [`LookSet`] provides a way to efficiently store a set of [`Look`] values.
* [`LookMatcher`] provides routines for checking whether a `Look` or a
`LookSet` matches at a particular position in a haystack.

```rust
pub mod look { /* ... */ }
```

### Types

#### Enum `Look`

A look-around assertion.

An assertion matches at a position between characters in a haystack.
Namely, it does not actually "consume" any input as most parts of a regular
expression do. Assertions are a way of stating that some property must be
true at a particular point during matching.

For example, `(?m)^[a-z]+$` is a pattern that:

* Scans the haystack for a position at which `(?m:^)` is satisfied. That
occurs at either the beginning of the haystack, or immediately following
a `\n` character.
* Looks for one or more occurrences of `[a-z]`.
* Once `[a-z]+` has matched as much as it can, an overall match is only
reported when `[a-z]+` stops just before a `\n`.

So in this case, `abc` and `\nabc\n` match, but `\nabc1\n` does not.

Assertions are also called "look-around," "look-behind" and "look-ahead."
Specifically, some assertions are look-behind (like `^`), other assertions
are look-ahead (like `$`) and yet other assertions are both look-ahead and
look-behind (like `\b`).

# Assertions in an NFA

An assertion in a [`thompson::NFA`](crate::nfa::thompson::NFA) can be
thought of as a conditional epsilon transition. That is, a matching engine
like the [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM) only permits
moving through conditional epsilon transitions when their condition
is satisfied at whatever position the `PikeVM` is currently at in the
haystack.

How assertions are handled in a `DFA` is trickier, since a DFA does not
have epsilon transitions at all. In this case, they are compiled into the
automaton itself, at the expense of more states than what would be required
without an assertion.

```rust
pub enum Look {
    Start = { _ },
    End = { _ },
    StartLF = { _ },
    EndLF = { _ },
    StartCRLF = { _ },
    EndCRLF = { _ },
    WordAscii = { _ },
    WordAsciiNegate = { _ },
    WordUnicode = { _ },
    WordUnicodeNegate = { _ },
    WordStartAscii = { _ },
    WordEndAscii = { _ },
    WordStartUnicode = { _ },
    WordEndUnicode = { _ },
    WordStartHalfAscii = { _ },
    WordEndHalfAscii = { _ },
    WordStartHalfUnicode = { _ },
    WordEndHalfUnicode = { _ },
}
```

##### Variants

###### `Start`

Match the beginning of text. Specifically, this matches at the starting
position of the input.

Discriminant: `{ _ }`

Discriminant value: `1`

###### `End`

Match the end of text. Specifically, this matches at the ending
position of the input.

Discriminant: `{ _ }`

Discriminant value: `2`

###### `StartLF`

Match the beginning of a line or the beginning of text. Specifically,
this matches at the starting position of the input, or at the position
immediately following a `\n` character.

Discriminant: `{ _ }`

Discriminant value: `4`

###### `EndLF`

Match the end of a line or the end of text. Specifically, this matches
at the end position of the input, or at the position immediately
preceding a `\n` character.

Discriminant: `{ _ }`

Discriminant value: `8`

###### `StartCRLF`

Match the beginning of a line or the beginning of text. Specifically,
this matches at the starting position of the input, or at the position
immediately following either a `\r` or `\n` character, but never after
a `\r` when a `\n` follows.

Discriminant: `{ _ }`

Discriminant value: `16`

###### `EndCRLF`

Match the end of a line or the end of text. Specifically, this matches
at the end position of the input, or at the position immediately
preceding a `\r` or `\n` character, but never before a `\n` when a `\r`
precedes it.

Discriminant: `{ _ }`

Discriminant value: `32`

###### `WordAscii`

Match an ASCII-only word boundary. That is, this matches a position
where the left adjacent character and right adjacent character
correspond to a word and non-word or a non-word and word character.

Discriminant: `{ _ }`

Discriminant value: `64`

###### `WordAsciiNegate`

Match an ASCII-only negation of a word boundary.

Discriminant: `{ _ }`

Discriminant value: `128`

###### `WordUnicode`

Match a Unicode-aware word boundary. That is, this matches a position
where the left adjacent character and right adjacent character
correspond to a word and non-word or a non-word and word character.

Discriminant: `{ _ }`

Discriminant value: `256`

###### `WordUnicodeNegate`

Match a Unicode-aware negation of a word boundary.

Discriminant: `{ _ }`

Discriminant value: `512`

###### `WordStartAscii`

Match the start of an ASCII-only word boundary. That is, this matches a
position at either the beginning of the haystack or where the previous
character is not a word character and the following character is a word
character.

Discriminant: `{ _ }`

Discriminant value: `1024`

###### `WordEndAscii`

Match the end of an ASCII-only word boundary. That is, this matches
a position at either the end of the haystack or where the previous
character is a word character and the following character is not a word
character.

Discriminant: `{ _ }`

Discriminant value: `2048`

###### `WordStartUnicode`

Match the start of a Unicode word boundary. That is, this matches a
position at either the beginning of the haystack or where the previous
character is not a word character and the following character is a word
character.

Discriminant: `{ _ }`

Discriminant value: `4096`

###### `WordEndUnicode`

Match the end of a Unicode word boundary. That is, this matches a
position at either the end of the haystack or where the previous
character is a word character and the following character is not a word
character.

Discriminant: `{ _ }`

Discriminant value: `8192`

###### `WordStartHalfAscii`

Match the start half of an ASCII-only word boundary. That is, this
matches a position at either the beginning of the haystack or where the
previous character is not a word character.

Discriminant: `{ _ }`

Discriminant value: `16384`

###### `WordEndHalfAscii`

Match the end half of an ASCII-only word boundary. That is, this
matches a position at either the end of the haystack or where the
following character is not a word character.

Discriminant: `{ _ }`

Discriminant value: `32768`

###### `WordStartHalfUnicode`

Match the start half of a Unicode word boundary. That is, this matches
a position at either the beginning of the haystack or where the
previous character is not a word character.

Discriminant: `{ _ }`

Discriminant value: `65536`

###### `WordEndHalfUnicode`

Match the end half of a Unicode word boundary. That is, this matches
a position at either the end of the haystack or where the following
character is not a word character.

Discriminant: `{ _ }`

Discriminant value: `131072`

##### Implementations

###### Methods

- ```rust
  pub const fn reversed(self: Self) -> Look { /* ... */ }
  ```
  Flip the look-around assertion to its equivalent for reverse searches.

- ```rust
  pub const fn as_repr(self: Self) -> u32 { /* ... */ }
  ```
  Return the underlying representation of this look-around enumeration

- ```rust
  pub const fn from_repr(repr: u32) -> Option<Look> { /* ... */ }
  ```
  Given the underlying representation of a `Look` value, return the

- ```rust
  pub const fn as_char(self: Self) -> char { /* ... */ }
  ```
  Returns a convenient single codepoint representation of this

###### Trait Implementations

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Copy**
- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Look { /* ... */ }
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

- **Freeze**
- **UnwindSafe**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Look) -> bool { /* ... */ }
    ```

#### Struct `LookSet`

LookSet is a memory-efficient set of look-around assertions.

This is useful for efficiently tracking look-around assertions. For
example, a [`thompson::NFA`](crate::nfa::thompson::NFA) provides properties
that return `LookSet`s.

```rust
pub struct LookSet {
    pub bits: u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `bits` | `u32` | The underlying representation this set is exposed to make it possible<br>to store it somewhere efficiently. The representation is that<br>of a bitset, where each assertion occupies bit `i` where<br>`i = Look::as_repr()`.<br><br>Note that users of this internal representation must permit the full<br>range of `u16` values to be represented. For example, even if the<br>current implementation only makes use of the 10 least significant bits,<br>it may use more bits in a future semver compatible release. |

##### Implementations

###### Methods

- ```rust
  pub fn empty() -> LookSet { /* ... */ }
  ```
  Create an empty set of look-around assertions.

- ```rust
  pub fn full() -> LookSet { /* ... */ }
  ```
  Create a full set of look-around assertions.

- ```rust
  pub fn singleton(look: Look) -> LookSet { /* ... */ }
  ```
  Create a look-around set containing the look-around assertion given.

- ```rust
  pub fn len(self: Self) -> usize { /* ... */ }
  ```
  Returns the total number of look-around assertions in this set.

- ```rust
  pub fn is_empty(self: Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set is empty.

- ```rust
  pub fn contains(self: Self, look: Look) -> bool { /* ... */ }
  ```
  Returns true if and only if the given look-around assertion is in this

- ```rust
  pub fn contains_anchor(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any anchor assertions.

- ```rust
  pub fn contains_anchor_haystack(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any "start/end of

- ```rust
  pub fn contains_anchor_line(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any "start/end of line"

- ```rust
  pub fn contains_anchor_lf(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any "start/end of line"

- ```rust
  pub fn contains_anchor_crlf(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any "start/end of line"

- ```rust
  pub fn contains_word(self: Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any word boundary or

- ```rust
  pub fn contains_word_unicode(self: Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any Unicode word boundary

- ```rust
  pub fn contains_word_ascii(self: Self) -> bool { /* ... */ }
  ```
  Returns true if and only if this set contains any ASCII word boundary

- ```rust
  pub fn iter(self: Self) -> LookSetIter { /* ... */ }
  ```
  Returns an iterator over all of the look-around assertions in this set.

- ```rust
  pub fn insert(self: Self, look: Look) -> LookSet { /* ... */ }
  ```
  Return a new set that is equivalent to the original, but with the given

- ```rust
  pub fn set_insert(self: &mut Self, look: Look) { /* ... */ }
  ```
  Updates this set in place with the result of inserting the given

- ```rust
  pub fn remove(self: Self, look: Look) -> LookSet { /* ... */ }
  ```
  Return a new set that is equivalent to the original, but with the given

- ```rust
  pub fn set_remove(self: &mut Self, look: Look) { /* ... */ }
  ```
  Updates this set in place with the result of removing the given

- ```rust
  pub fn subtract(self: Self, other: LookSet) -> LookSet { /* ... */ }
  ```
  Returns a new set that is the result of subtracting the given set from

- ```rust
  pub fn set_subtract(self: &mut Self, other: LookSet) { /* ... */ }
  ```
  Updates this set in place with the result of subtracting the given set

- ```rust
  pub fn union(self: Self, other: LookSet) -> LookSet { /* ... */ }
  ```
  Returns a new set that is the union of this and the one given.

- ```rust
  pub fn set_union(self: &mut Self, other: LookSet) { /* ... */ }
  ```
  Updates this set in place with the result of unioning it with the one

- ```rust
  pub fn intersect(self: Self, other: LookSet) -> LookSet { /* ... */ }
  ```
  Returns a new set that is the intersection of this and the one given.

- ```rust
  pub fn set_intersect(self: &mut Self, other: LookSet) { /* ... */ }
  ```
  Updates this set in place with the result of intersecting it with the

- ```rust
  pub fn read_repr(slice: &[u8]) -> LookSet { /* ... */ }
  ```
  Return a `LookSet` from the slice given as a native endian 32-bit

- ```rust
  pub fn write_repr(self: Self, slice: &mut [u8]) { /* ... */ }
  ```
  Write a `LookSet` as a native endian 32-bit integer to the beginning

- ```rust
  pub fn available(self: Self) -> Result<(), UnicodeWordBoundaryError> { /* ... */ }
  ```
  Checks that all assertions in this set can be matched.

###### Trait Implementations

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

- **Default**
  - ```rust
    fn default() -> LookSet { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LookSet) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LookSet { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Freeze**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
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

#### Struct `LookSetIter`

An iterator over all look-around assertions in a [`LookSet`].

This iterator is created by [`LookSet::iter`].

```rust
pub struct LookSetIter {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Look> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LookSetIter { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **UnwindSafe**
#### Struct `LookMatcher`

A matcher for look-around assertions.

This matcher permits configuring aspects of how look-around assertions are
matched.

# Example

A `LookMatcher` can change the line terminator used for matching multi-line
anchors such as `(?m:^)` and `(?m:$)`.

```
use regex_automata::{
    nfa::thompson::{self, pikevm::PikeVM},
    util::look::LookMatcher,
    Match, Input,
};

let mut lookm = LookMatcher::new();
lookm.set_line_terminator(b'\x00');

let re = PikeVM::builder()
    .thompson(thompson::Config::new().look_matcher(lookm))
    .build(r"(?m)^[a-z]+$")?;
let mut cache = re.create_cache();

// Multi-line assertions now use NUL as a terminator.
assert_eq!(
    Some(Match::must(0, 1..4)),
    re.find(&mut cache, b"\x00abc\x00"),
);
// ... and \n is no longer recognized as a terminator.
assert_eq!(
    None,
    re.find(&mut cache, b"\nabc\n"),
);

# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub struct LookMatcher {
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
  pub fn new() -> LookMatcher { /* ... */ }
  ```
  Creates a new default matcher for look-around assertions.

- ```rust
  pub fn set_line_terminator(self: &mut Self, byte: u8) -> &mut LookMatcher { /* ... */ }
  ```
  Sets the line terminator for use with `(?m:^)` and `(?m:$)`.

- ```rust
  pub fn get_line_terminator(self: &Self) -> u8 { /* ... */ }
  ```
  Returns the line terminator that was configured for this matcher.

- ```rust
  pub fn matches(self: &Self, look: Look, haystack: &[u8], at: usize) -> bool { /* ... */ }
  ```
  Returns true when the position `at` in `haystack` satisfies the given

- ```rust
  pub fn matches_set(self: &Self, set: LookSet, haystack: &[u8], at: usize) -> bool { /* ... */ }
  ```
  Returns true when _all_ of the assertions in the given set match at the

- ```rust
  pub fn is_start(self: &Self, _haystack: &[u8], at: usize) -> bool { /* ... */ }
  ```
  Returns true when [`Look::Start`] is satisfied `at` the given position

- ```rust
  pub fn is_end(self: &Self, haystack: &[u8], at: usize) -> bool { /* ... */ }
  ```
  Returns true when [`Look::End`] is satisfied `at` the given position in

- ```rust
  pub fn is_start_lf(self: &Self, haystack: &[u8], at: usize) -> bool { /* ... */ }
  ```
  Returns true when [`Look::StartLF`] is satisfied `at` the given

- ```rust
  pub fn is_end_lf(self: &Self, haystack: &[u8], at: usize) -> bool { /* ... */ }
  ```
  Returns true when [`Look::EndLF`] is satisfied `at` the given position

- ```rust
  pub fn is_start_crlf(self: &Self, haystack: &[u8], at: usize) -> bool { /* ... */ }
  ```
  Returns true when [`Look::StartCRLF`] is satisfied `at` the given

- ```rust
  pub fn is_end_crlf(self: &Self, haystack: &[u8], at: usize) -> bool { /* ... */ }
  ```
  Returns true when [`Look::EndCRLF`] is satisfied `at` the given

- ```rust
  pub fn is_word_ascii(self: &Self, haystack: &[u8], at: usize) -> bool { /* ... */ }
  ```
  Returns true when [`Look::WordAscii`] is satisfied `at` the given

- ```rust
  pub fn is_word_ascii_negate(self: &Self, haystack: &[u8], at: usize) -> bool { /* ... */ }
  ```
  Returns true when [`Look::WordAsciiNegate`] is satisfied `at` the given

- ```rust
  pub fn is_word_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError> { /* ... */ }
  ```
  Returns true when [`Look::WordUnicode`] is satisfied `at` the given

- ```rust
  pub fn is_word_unicode_negate(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError> { /* ... */ }
  ```
  Returns true when [`Look::WordUnicodeNegate`] is satisfied `at` the

- ```rust
  pub fn is_word_start_ascii(self: &Self, haystack: &[u8], at: usize) -> bool { /* ... */ }
  ```
  Returns true when [`Look::WordStartAscii`] is satisfied `at` the given

- ```rust
  pub fn is_word_end_ascii(self: &Self, haystack: &[u8], at: usize) -> bool { /* ... */ }
  ```
  Returns true when [`Look::WordEndAscii`] is satisfied `at` the given

- ```rust
  pub fn is_word_start_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError> { /* ... */ }
  ```
  Returns true when [`Look::WordStartUnicode`] is satisfied `at` the

- ```rust
  pub fn is_word_end_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError> { /* ... */ }
  ```
  Returns true when [`Look::WordEndUnicode`] is satisfied `at` the

- ```rust
  pub fn is_word_start_half_ascii(self: &Self, haystack: &[u8], at: usize) -> bool { /* ... */ }
  ```
  Returns true when [`Look::WordStartHalfAscii`] is satisfied `at` the

- ```rust
  pub fn is_word_end_half_ascii(self: &Self, haystack: &[u8], at: usize) -> bool { /* ... */ }
  ```
  Returns true when [`Look::WordEndHalfAscii`] is satisfied `at` the

- ```rust
  pub fn is_word_start_half_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError> { /* ... */ }
  ```
  Returns true when [`Look::WordStartHalfUnicode`] is satisfied `at` the

- ```rust
  pub fn is_word_end_half_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError> { /* ... */ }
  ```
  Returns true when [`Look::WordEndHalfUnicode`] is satisfied `at` the

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> LookMatcher { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LookMatcher { /* ... */ }
    ```

- **Sync**
- **Freeze**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
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

#### Struct `UnicodeWordBoundaryError`

An error that occurs when the Unicode-aware `\w` class is unavailable.

This error can occur when the data tables necessary for the Unicode aware
Perl character class `\w` are unavailable. The `\w` class is used to
determine whether a codepoint is considered a word character or not when
determining whether a Unicode aware `\b` (or `\B`) matches at a particular
position.

This error can only occur when the `unicode-word-boundary` feature is
disabled.

```rust
pub struct UnicodeWordBoundaryError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn check() -> Result<(), UnicodeWordBoundaryError> { /* ... */ }
  ```
  Returns an error if and only if Unicode word boundary data is

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Error**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> UnicodeWordBoundaryError { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

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

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `pool`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

A thread safe memory pool.

The principal type in this module is a [`Pool`]. It main use case is for
holding a thread safe collection of mutable scratch spaces (usually called
`Cache` in this crate) that regex engines need to execute a search. This then
permits sharing the same read-only regex object across multiple threads while
having a quick way of reusing scratch space in a thread safe way. This avoids
needing to re-create the scratch space for every search, which could wind up
being quite expensive.

```rust
pub mod pool { /* ... */ }
```

### Types

#### Struct `Pool`

A thread safe pool that works in an `alloc`-only context.

Getting a value out comes with a guard. When that guard is dropped, the
value is automatically put back in the pool. The guard provides both a
`Deref` and a `DerefMut` implementation for easy access to an underlying
`T`.

A `Pool` impls `Sync` when `T` is `Send` (even if `T` is not `Sync`). This
is possible because a pool is guaranteed to provide a value to exactly one
thread at any time.

Currently, a pool never contracts in size. Its size is proportional to the
maximum number of simultaneous uses. This may change in the future.

A `Pool` is a particularly useful data structure for this crate because
many of the regex engines require a mutable "cache" in order to execute
a search. Since regexes themselves tend to be global, the problem is then:
how do you get a mutable cache to execute a search? You could:

1. Use a `thread_local!`, which requires the standard library and requires
that the regex pattern be statically known.
2. Use a `Pool`.
3. Make the cache an explicit dependency in your code and pass it around.
4. Put the cache state in a `Mutex`, but this means only one search can
execute at a time.
5. Create a new cache for every search.

A `thread_local!` is perhaps the best choice if it works for your use case.
Putting the cache in a mutex or creating a new cache for every search are
perhaps the worst choices. Of the remaining two choices, whether you use
this `Pool` or thread through a cache explicitly in your code is a matter
of taste and depends on your code architecture.

# Warning: may use a spin lock

When this crate is compiled _without_ the `std` feature, then this type
may used a spin lock internally. This can have subtle effects that may
be undesirable. See [Spinlocks Considered Harmful][spinharm] for a more
thorough treatment of this topic.

[spinharm]: https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html

# Example

This example shows how to share a single hybrid regex among multiple
threads, while also safely getting exclusive access to a hybrid's
[`Cache`](crate::hybrid::regex::Cache) without preventing other searches
from running while your thread uses the `Cache`.

```
use regex_automata::{
    hybrid::regex::{Cache, Regex},
    util::{lazy::Lazy, pool::Pool},
    Match,
};

static RE: Lazy<Regex> =
    Lazy::new(|| Regex::new("foo[0-9]+bar").unwrap());
static CACHE: Lazy<Pool<Cache>> =
    Lazy::new(|| Pool::new(|| RE.create_cache()));

let expected = Some(Match::must(0, 3..14));
assert_eq!(expected, RE.find(&mut CACHE.get(), b"zzzfoo12345barzzz"));
```

```rust
pub struct Pool<T, F = fn() -> T>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(create: F) -> Pool<T, F> { /* ... */ }
  ```
  Create a new pool. The given closure is used to create values in

- ```rust
  pub fn get(self: &Self) -> PoolGuard<''_, T, F> { /* ... */ }
  ```
  Get a value from the pool. The caller is guaranteed to have

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Sync**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
#### Struct `PoolGuard`

A guard that is returned when a caller requests a value from the pool.

The purpose of the guard is to use RAII to automatically put the value
back in the pool once it's dropped.

```rust
pub struct PoolGuard<''a, T: Send, F: Fn() -> T>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn put(this: PoolGuard<''_, T, F>) { /* ... */ }
  ```
  Consumes this guard and puts it back into the pool.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
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

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Receiver**
- **Unpin**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Sync**
## Module `prefilter`

Defines a prefilter for accelerating regex searches.

A prefilter can be created by building a [`Prefilter`] value.

A prefilter represents one of the most important optimizations available for
accelerating regex searches. The idea of a prefilter is to very quickly find
candidate locations in a haystack where a regex _could_ match. Once a candidate
is found, it is then intended for the regex engine to run at that position to
determine whether the candidate is a match or a false positive.

In the aforementioned description of the prefilter optimization also lay its
demise. Namely, if a prefilter has a high false positive rate and it produces
lots of candidates, then a prefilter can overall make a regex search slower.
It can run more slowly because more time is spent ping-ponging between the
prefilter search and the regex engine attempting to confirm each candidate as
a match. This ping-ponging has overhead that adds up, and is exacerbated by
a high false positive rate.

Nevertheless, the optimization is still generally worth performing in most
cases. Particularly given just how much throughput can be improved. (It is not
uncommon for prefilter optimizations to improve throughput by one or two orders
of magnitude.)

Typically a prefilter is used to find occurrences of literal prefixes from a
regex pattern, but this isn't required. A prefilter can be used to look for
suffixes or even inner literals.

Note that as of now, prefilters throw away information about which pattern
each literal comes from. In other words, when a prefilter finds a match,
there's no way to know which pattern (or patterns) it came from. Therefore,
in order to confirm a match, you'll have to check all of the patterns by
running the full regex engine.

```rust
pub mod prefilter { /* ... */ }
```

### Types

#### Struct `Prefilter`

A prefilter for accelerating regex searches.

If you already have your literals that you want to search with,
then the vanilla [`Prefilter::new`] constructor is for you. But
if you have an [`Hir`] value from the `regex-syntax` crate, then
[`Prefilter::from_hir_prefix`] might be more convenient. Namely, it uses
the [`regex-syntax::hir::literal`](regex_syntax::hir::literal) module to
extract literal prefixes for you, optimize them and then select and build a
prefilter matcher.

A prefilter must have **zero false negatives**. However, by its very
nature, it may produce false positives. That is, a prefilter will never
skip over a position in the haystack that corresponds to a match of the
original regex pattern, but it *may* produce a match for a position
in the haystack that does *not* correspond to a match of the original
regex pattern. If you use either the [`Prefilter::from_hir_prefix`] or
[`Prefilter::from_hirs_prefix`] constructors, then this guarantee is
upheld for you automatically. This guarantee is not preserved if you use
[`Prefilter::new`] though, since it is up to the caller to provide correct
literal strings with respect to the original regex pattern.

# Cloning

It is an API guarantee that cloning a prefilter is cheap. That is, cloning
it will not duplicate whatever heap memory is used to represent the
underlying matcher.

# Example

This example shows how to attach a `Prefilter` to the
[`PikeVM`](crate::nfa::thompson::pikevm::PikeVM) in order to accelerate
searches.

```
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    util::prefilter::Prefilter,
    Match, MatchKind,
};

let pre = Prefilter::new(MatchKind::LeftmostFirst, &["Bruce "])
    .expect("a prefilter");
let re = PikeVM::builder()
    .configure(PikeVM::config().prefilter(Some(pre)))
    .build(r"Bruce \w+")?;
let mut cache = re.create_cache();
assert_eq!(
    Some(Match::must(0, 6..23)),
    re.find(&mut cache, "Hello Bruce Springsteen!"),
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

But note that if you get your prefilter incorrect, it could lead to an
incorrect result!

```
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    util::prefilter::Prefilter,
    Match, MatchKind,
};

// This prefilter is wrong!
let pre = Prefilter::new(MatchKind::LeftmostFirst, &["Patti "])
    .expect("a prefilter");
let re = PikeVM::builder()
    .configure(PikeVM::config().prefilter(Some(pre)))
    .build(r"Bruce \w+")?;
let mut cache = re.create_cache();
// We find no match even though the regex does match.
assert_eq!(
    None,
    re.find(&mut cache, "Hello Bruce Springsteen!"),
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub struct Prefilter {
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
  pub fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Prefilter> { /* ... */ }
  ```
  Create a new prefilter from a sequence of needles and a corresponding

- ```rust
  pub fn from_hir_prefix(kind: MatchKind, hir: &Hir) -> Option<Prefilter> { /* ... */ }
  ```
  This attempts to extract prefixes from the given `Hir` expression for

- ```rust
  pub fn from_hirs_prefix<H: Borrow<Hir>>(kind: MatchKind, hirs: &[H]) -> Option<Prefilter> { /* ... */ }
  ```
  This attempts to extract prefixes from the given `Hir` expressions for

- ```rust
  pub fn find(self: &Self, haystack: &[u8], span: Span) -> Option<Span> { /* ... */ }
  ```
  Run this prefilter on `haystack[span.start..end]` and return a matching

- ```rust
  pub fn prefix(self: &Self, haystack: &[u8], span: Span) -> Option<Span> { /* ... */ }
  ```
  Returns the span of a prefix of `haystack[span.start..span.end]` if

- ```rust
  pub fn memory_usage(self: &Self) -> usize { /* ... */ }
  ```
  Returns the heap memory, in bytes, used by the underlying prefilter.

- ```rust
  pub fn max_needle_len(self: &Self) -> usize { /* ... */ }
  ```
  Return the length of the longest needle

- ```rust
  pub fn is_fast(self: &Self) -> bool { /* ... */ }
  ```
  Implementations might return true here if they believe themselves to

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Prefilter { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `primitives`

Lower level primitive types that are useful in a variety of circumstances.

# Overview

This list represents the principle types in this module and briefly describes
when you might want to use them.

* [`PatternID`] - A type that represents the identifier of a regex pattern.
This is probably the most widely used type in this module (which is why it's
also re-exported in the crate root).
* [`StateID`] - A type the represents the identifier of a finite automaton
state. This is used for both NFAs and DFAs, with the notable exception of
the hybrid NFA/DFA. (The hybrid NFA/DFA uses a special purpose "lazy" state
identifier.)
* [`SmallIndex`] - The internal representation of both a `PatternID` and a
`StateID`. Its purpose is to serve as a type that can index memory without
being as big as a `usize` on 64-bit targets. The main idea behind this type
is that there are many things in regex engines that will, in practice, never
overflow a 32-bit integer. (For example, like the number of patterns in a regex
or the number of states in an NFA.) Thus, a `SmallIndex` can be used to index
memory without peppering `as` casts everywhere. Moreover, it forces callers
to handle errors in the case where, somehow, the value would otherwise overflow
either a 32-bit integer or a `usize` (e.g., on 16-bit targets).
* [`NonMaxUsize`] - Represents a `usize` that cannot be `usize::MAX`. As a
result, `Option<NonMaxUsize>` has the same size in memory as a `usize`. This
useful, for example, when representing the offsets of submatches since it
reduces memory usage by a factor of 2. It is a legal optimization since Rust
guarantees that slices never have a length that exceeds `isize::MAX`.

```rust
pub mod primitives { /* ... */ }
```

### Types

#### Struct `NonMaxUsize`

A `usize` that can never be `usize::MAX`.

This is similar to `core::num::NonZeroUsize`, but instead of not permitting
a zero value, this does not permit a max value.

This is useful in certain contexts where one wants to optimize the memory
usage of things that contain match offsets. Namely, since Rust slices
are guaranteed to never have a length exceeding `isize::MAX`, we can use
`usize::MAX` as a sentinel to indicate that no match was found. Indeed,
types like `Option<NonMaxUsize>` have exactly the same size in memory as a
`usize`.

This type is defined to be `repr(transparent)` for
`core::num::NonZeroUsize`, which is in turn defined to be
`repr(transparent)` for `usize`.

```rust
pub struct NonMaxUsize(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(value: usize) -> Option<NonMaxUsize> { /* ... */ }
  ```
  Create a new `NonMaxUsize` from the given value.

- ```rust
  pub fn get(self: Self) -> usize { /* ... */ }
  ```
  Return the underlying `usize` value. The returned value is guaranteed

###### Trait Implementations

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &NonMaxUsize) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &NonMaxUsize) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &NonMaxUsize) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NonMaxUsize { /* ... */ }
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

- **Freeze**
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
- **Unpin**
#### Struct `SmallIndex`

A type that represents a "small" index.

The main idea of this type is to provide something that can index memory,
but uses less memory than `usize` on 64-bit systems. Specifically, its
representation is always a `u32` and has `repr(transparent)` enabled. (So
it is safe to transmute between a `u32` and a `SmallIndex`.)

A small index is typically useful in cases where there is no practical way
that the index will overflow a 32-bit integer. A good example of this is
an NFA state. If you could somehow build an NFA with `2^30` states, its
memory usage would be exorbitant and its runtime execution would be so
slow as to be completely worthless. Therefore, this crate generally deems
it acceptable to return an error if it would otherwise build an NFA that
requires a slice longer than what a 32-bit integer can index. In exchange,
we can use 32-bit indices instead of 64-bit indices in various places.

This type ensures this by providing a constructor that will return an error
if its argument cannot fit into the type. This makes it much easier to
handle these sorts of boundary cases that are otherwise extremely subtle.

On all targets, this type guarantees that its value will fit in a `u32`,
`i32`, `usize` and an `isize`. This means that on 16-bit targets, for
example, this type's maximum value will never overflow an `isize`,
which means it will never overflow a `i16` even though its internal
representation is still a `u32`.

The purpose for making the type fit into even signed integer types like
`isize` is to guarantee that the difference between any two small indices
is itself also a small index. This is useful in certain contexts, e.g.,
for delta encoding.

# Other types

The following types wrap `SmallIndex` to provide a more focused use case:

* [`PatternID`] is for representing the identifiers of patterns.
* [`StateID`] is for representing the identifiers of states in finite
automata. It is used for both NFAs and DFAs.

# Representation

This type is always represented internally by a `u32` and is marked as
`repr(transparent)`. Thus, this type always has the same representation as
a `u32`. It is thus safe to transmute between a `u32` and a `SmallIndex`.

# Indexing

For convenience, callers may use a `SmallIndex` to index slices.

# Safety

While a `SmallIndex` is meant to guarantee that its value fits into `usize`
without using as much space as a `usize` on all targets, callers must
not rely on this property for safety. Callers may choose to rely on this
property for correctness however. For example, creating a `SmallIndex` with
an invalid value can be done in entirely safe code. This may in turn result
in panics or silent logical errors.

```rust
pub struct SmallIndex(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(index: usize) -> Result<SmallIndex, SmallIndexError> { /* ... */ }
  ```
  Create a new small index.

- ```rust
  pub const fn new_unchecked(index: usize) -> SmallIndex { /* ... */ }
  ```
  Create a new small index without checking whether the given value

- ```rust
  pub fn must(index: usize) -> SmallIndex { /* ... */ }
  ```
  Like [`SmallIndex::new`], but panics if the given index is not valid.

- ```rust
  pub const fn as_usize(self: &Self) -> usize { /* ... */ }
  ```
  Return this small index as a `usize`. This is guaranteed to never

- ```rust
  pub const fn as_u64(self: &Self) -> u64 { /* ... */ }
  ```
  Return this small index as a `u64`. This is guaranteed to never

- ```rust
  pub const fn as_u32(self: &Self) -> u32 { /* ... */ }
  ```
  Return the internal `u32` of this small index. This is guaranteed to

- ```rust
  pub const fn as_i32(self: &Self) -> i32 { /* ... */ }
  ```
  Return the internal `u32` of this small index represented as an `i32`.

- ```rust
  pub fn one_more(self: &Self) -> usize { /* ... */ }
  ```
  Returns one more than this small index as a usize.

- ```rust
  pub fn from_ne_bytes(bytes: [u8; 4]) -> Result<SmallIndex, SmallIndexError> { /* ... */ }
  ```
  Decode this small index from the bytes given using the native endian

- ```rust
  pub fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> SmallIndex { /* ... */ }
  ```
  Decode this small index from the bytes given using the native endian

- ```rust
  pub fn to_ne_bytes(self: &Self) -> [u8; 4] { /* ... */ }
  ```
  Return the underlying small index integer as raw bytes in native endian

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(index: u8) -> SmallIndex { /* ... */ }
    ```

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: SmallIndex) -> &mut T { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, index: SmallIndex) -> &mut T { /* ... */ }
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
- **Freeze**
- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(index: u16) -> Result<SmallIndex, SmallIndexError> { /* ... */ }
    ```

  - ```rust
    fn try_from(index: u32) -> Result<SmallIndex, SmallIndexError> { /* ... */ }
    ```

  - ```rust
    fn try_from(index: u64) -> Result<SmallIndex, SmallIndexError> { /* ... */ }
    ```

  - ```rust
    fn try_from(index: usize) -> Result<SmallIndex, SmallIndexError> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SmallIndex) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SmallIndex) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **UnwindSafe**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> SmallIndex { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SmallIndex) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: SmallIndex) -> &T { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: SmallIndex) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SmallIndex { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `SmallIndexError`

This error occurs when a small index could not be constructed.

This occurs when given an integer exceeding the maximum small index value.

When the `std` feature is enabled, this implements the `Error` trait.

```rust
pub struct SmallIndexError {
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
  pub fn attempted(self: &Self) -> u64 { /* ... */ }
  ```
  Returns the value that could not be converted to a small index.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SmallIndexError { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **Error**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SmallIndexError) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Eq**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `PatternID`

The identifier of a regex pattern, represented by a [`SmallIndex`].

The identifier for a pattern corresponds to its relative position among
other patterns in a single finite state machine. Namely, when building
a multi-pattern regex engine, one must supply a sequence of patterns to
match. The position (starting at 0) of each pattern in that sequence
represents its identifier. This identifier is in turn used to identify and
report matches of that pattern in various APIs.

See the [`SmallIndex`] type for more information about what it means for
a pattern ID to be a "small index."

Note that this type is defined in the
[`util::primitives`](crate::util::primitives) module, but it is also
re-exported at the crate root due to how common it is.

```rust
pub struct PatternID(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(value: usize) -> Result<PatternID, PatternIDError> { /* ... */ }
  ```
  Create a new value that is represented by a "small index."

- ```rust
  pub const fn new_unchecked(value: usize) -> PatternID { /* ... */ }
  ```
  Create a new value without checking whether the given argument

- ```rust
  pub fn must(value: usize) -> PatternID { /* ... */ }
  ```
  Like `new`, but panics if the given value is not valid.

- ```rust
  pub const fn as_usize(self: &Self) -> usize { /* ... */ }
  ```
  Return the internal value as a `usize`. This is guaranteed to

- ```rust
  pub const fn as_u64(self: &Self) -> u64 { /* ... */ }
  ```
  Return the internal value as a `u64`. This is guaranteed to

- ```rust
  pub const fn as_u32(self: &Self) -> u32 { /* ... */ }
  ```
  Return the internal value as a `u32`. This is guaranteed to

- ```rust
  pub const fn as_i32(self: &Self) -> i32 { /* ... */ }
  ```
  Return the internal value as a i32`. This is guaranteed to

- ```rust
  pub fn one_more(self: &Self) -> usize { /* ... */ }
  ```
  Returns one more than this value as a usize.

- ```rust
  pub fn from_ne_bytes(bytes: [u8; 4]) -> Result<PatternID, PatternIDError> { /* ... */ }
  ```
  Decode this value from the bytes given using the native endian

- ```rust
  pub fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> PatternID { /* ... */ }
  ```
  Decode this value from the bytes given using the native endian

- ```rust
  pub fn to_ne_bytes(self: &Self) -> [u8; 4] { /* ... */ }
  ```
  Return the underlying integer as raw bytes in native endian

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Send**
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

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PatternID { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u16) -> Result<PatternID, PatternIDError> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u32) -> Result<PatternID, PatternIDError> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u64) -> Result<PatternID, PatternIDError> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<PatternID, PatternIDError> { /* ... */ }
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

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PatternID) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &PatternID) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> PatternID { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u8) -> PatternID { /* ... */ }
    ```

- **StructuralPartialEq**
- **Index**
  - ```rust
    fn index(self: &Self, index: PatternID) -> &T { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: PatternID) -> &T { /* ... */ }
    ```

- **Copy**
- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: PatternID) -> &mut T { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, index: PatternID) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &PatternID) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

#### Struct `StateID`

The identifier of a finite automaton state, represented by a
[`SmallIndex`].

Most regex engines in this crate are built on top of finite automata. Each
state in a finite automaton defines transitions from its state to another.
Those transitions point to other states via their identifiers, i.e., a
`StateID`. Since finite automata tend to contain many transitions, it is
much more memory efficient to define state IDs as small indices.

See the [`SmallIndex`] type for more information about what it means for
a state ID to be a "small index."

```rust
pub struct StateID(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(value: usize) -> Result<StateID, StateIDError> { /* ... */ }
  ```
  Create a new value that is represented by a "small index."

- ```rust
  pub const fn new_unchecked(value: usize) -> StateID { /* ... */ }
  ```
  Create a new value without checking whether the given argument

- ```rust
  pub fn must(value: usize) -> StateID { /* ... */ }
  ```
  Like `new`, but panics if the given value is not valid.

- ```rust
  pub const fn as_usize(self: &Self) -> usize { /* ... */ }
  ```
  Return the internal value as a `usize`. This is guaranteed to

- ```rust
  pub const fn as_u64(self: &Self) -> u64 { /* ... */ }
  ```
  Return the internal value as a `u64`. This is guaranteed to

- ```rust
  pub const fn as_u32(self: &Self) -> u32 { /* ... */ }
  ```
  Return the internal value as a `u32`. This is guaranteed to

- ```rust
  pub const fn as_i32(self: &Self) -> i32 { /* ... */ }
  ```
  Return the internal value as a i32`. This is guaranteed to

- ```rust
  pub fn one_more(self: &Self) -> usize { /* ... */ }
  ```
  Returns one more than this value as a usize.

- ```rust
  pub fn from_ne_bytes(bytes: [u8; 4]) -> Result<StateID, StateIDError> { /* ... */ }
  ```
  Decode this value from the bytes given using the native endian

- ```rust
  pub fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> StateID { /* ... */ }
  ```
  Decode this value from the bytes given using the native endian

- ```rust
  pub fn to_ne_bytes(self: &Self) -> [u8; 4] { /* ... */ }
  ```
  Return the underlying integer as raw bytes in native endian

###### Trait Implementations

- **Copy**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u16) -> Result<StateID, StateIDError> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u32) -> Result<StateID, StateIDError> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: u64) -> Result<StateID, StateIDError> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: usize) -> Result<StateID, StateIDError> { /* ... */ }
    ```

- **Unpin**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &StateID) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: u8) -> StateID { /* ... */ }
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
- **Default**
  - ```rust
    fn default() -> StateID { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: StateID) -> &T { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: StateID) -> &T { /* ... */ }
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

- **Send**
- **UnwindSafe**
- **Sync**
- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StateID { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StateID) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &StateID) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: StateID) -> &mut T { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, index: StateID) -> &mut T { /* ... */ }
    ```

- **Eq**
#### Struct `PatternIDError`

This error occurs when a value could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

```rust
pub struct PatternIDError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn attempted(self: &Self) -> u64 { /* ... */ }
  ```
  Returns the value that could not be converted to an ID.

###### Trait Implementations

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Error**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PatternIDError { /* ... */ }
    ```

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PatternIDError) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `StateIDError`

This error occurs when a value could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

```rust
pub struct StateIDError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn attempted(self: &Self) -> u64 { /* ... */ }
  ```
  Returns the value that could not be converted to an ID.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StateIDError) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> StateIDError { /* ... */ }
    ```

- **Error**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
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

- **StructuralPartialEq**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

## Module `start`

Provides helpers for dealing with start state configurations in DFAs.

```rust
pub mod start { /* ... */ }
```

### Types

#### Struct `Config`

The configuration used to determine a DFA's start state for a search.

A DFA has a single starting state in the typical textbook description. That
is, it corresponds to the set of all starting states for the NFA that built
it, along with their espsilon closures. In this crate, however, DFAs have
many possible start states due to a few factors:

* DFAs support the ability to run either anchored or unanchored searches.
Each type of search needs its own start state. For example, an unanchored
search requires starting at a state corresponding to a regex with a
`(?s-u:.)*?` prefix, which will match through anything.
* DFAs also optionally support starting an anchored search for any one
specific pattern. Each such pattern requires its own start state.
* If a look-behind assertion like `^` or `\b` is used in the regex, then
the DFA will need to inspect a single byte immediately before the start of
the search to choose the correct start state.

Indeed, this configuration precisely encapsulates all of the above factors.
The [`Config::anchored`] method sets which kind of anchored search to
perform while the [`Config::look_behind`] method provides a way to set
the byte that occurs immediately before the start of the search.

Generally speaking, this type is only useful when you want to run searches
without using an [`Input`]. In particular, an `Input` wants a haystack
slice, but callers may not have a contiguous sequence of bytes as a
haystack in all cases. This type provides a lower level of control such
that callers can provide their own anchored configuration and look-behind
byte explicitly.

# Example

This shows basic usage that permits running a search with a DFA without
using the `Input` abstraction.

```
use regex_automata::{
    dfa::{Automaton, dense},
    util::start,
    Anchored,
};

let dfa = dense::DFA::new(r"(?-u)\b\w+\b")?;
let haystack = "quartz";

let config = start::Config::new().anchored(Anchored::Yes);
let mut state = dfa.start_state(&config)?;
for &b in haystack.as_bytes().iter() {
    state = dfa.next_state(state, b);
}
state = dfa.next_eoi_state(state);
assert!(dfa.is_match_state(state));

# Ok::<(), Box<dyn std::error::Error>>(())
```

This example shows how to correctly run a search that doesn't begin at
the start of a haystack. Notice how we set the look-behind byte, and as
a result, the `\b` assertion does not match.

```
use regex_automata::{
    dfa::{Automaton, dense},
    util::start,
    Anchored,
};

let dfa = dense::DFA::new(r"(?-u)\b\w+\b")?;
let haystack = "quartz";

let config = start::Config::new()
    .anchored(Anchored::Yes)
    .look_behind(Some(b'q'));
let mut state = dfa.start_state(&config)?;
for &b in haystack.as_bytes().iter().skip(1) {
    state = dfa.next_state(state, b);
}
state = dfa.next_eoi_state(state);
// No match!
assert!(!dfa.is_match_state(state));

# Ok::<(), Box<dyn std::error::Error>>(())
```

If we had instead not set a look-behind byte, then the DFA would assume
that it was starting at the beginning of the haystack, and thus `\b` should
match. This in turn would result in erroneously reporting a match:

```
use regex_automata::{
    dfa::{Automaton, dense},
    util::start,
    Anchored,
};

let dfa = dense::DFA::new(r"(?-u)\b\w+\b")?;
let haystack = "quartz";

// Whoops, forgot the look-behind byte...
let config = start::Config::new().anchored(Anchored::Yes);
let mut state = dfa.start_state(&config)?;
for &b in haystack.as_bytes().iter().skip(1) {
    state = dfa.next_state(state, b);
}
state = dfa.next_eoi_state(state);
// And now we get a match unexpectedly.
assert!(dfa.is_match_state(state));

# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub struct Config {
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
  pub fn new() -> Config { /* ... */ }
  ```
  Create a new default start configuration.

- ```rust
  pub fn from_input_forward(input: &Input<''_>) -> Config { /* ... */ }
  ```
  A convenience routine for building a start configuration from an

- ```rust
  pub fn from_input_reverse(input: &Input<''_>) -> Config { /* ... */ }
  ```
  A convenience routine for building a start configuration from an

- ```rust
  pub fn look_behind(self: Self, byte: Option<u8>) -> Config { /* ... */ }
  ```
  Set the look-behind byte at the start of a search.

- ```rust
  pub fn anchored(self: Self, mode: Anchored) -> Config { /* ... */ }
  ```
  Set the anchored mode of a search.

- ```rust
  pub fn get_look_behind(self: &Self) -> Option<u8> { /* ... */ }
  ```
  Return the look-behind byte in this configuration, if one exists.

- ```rust
  pub fn get_anchored(self: &Self) -> Anchored { /* ... */ }
  ```
  Return the anchored mode in this configuration.

###### Trait Implementations

- **UnwindSafe**
- **Sync**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Config { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `syntax`

**Attributes:**

- `#[<cfg>(feature = "syntax")]`

Utilities for dealing with the syntax of a regular expression.

This module currently only exposes a [`Config`] type that
itself represents a wrapper around the configuration for a
[`regex-syntax::ParserBuilder`](regex_syntax::ParserBuilder). The purpose of
this wrapper is to make configuring syntax options very similar to how other
configuration is done throughout this crate. Namely, instead of duplicating
syntax options across every builder (of which there are many), we instead
create small config objects like this one that can be passed around and
composed.

```rust
pub mod syntax { /* ... */ }
```

### Types

#### Struct `Config`

A common set of configuration options that apply to the syntax of a regex.

This represents a group of configuration options that specifically apply
to how the concrete syntax of a regular expression is interpreted. In
particular, they are generally forwarded to the
[`ParserBuilder`](https://docs.rs/regex-syntax/*/regex_syntax/struct.ParserBuilder.html)
in the
[`regex-syntax`](https://docs.rs/regex-syntax)
crate when building a regex from its concrete syntax directly.

These options are defined as a group since they apply to every regex engine
in this crate. Instead of re-defining them on every engine's builder, they
are instead provided here as one cohesive unit.

```rust
pub struct Config {
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
  pub fn new() -> Config { /* ... */ }
  ```
  Return a new default syntax configuration.

- ```rust
  pub fn case_insensitive(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Enable or disable the case insensitive flag by default.

- ```rust
  pub fn multi_line(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Enable or disable the multi-line matching flag by default.

- ```rust
  pub fn dot_matches_new_line(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Enable or disable the "dot matches any character" flag by default.

- ```rust
  pub fn crlf(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Enable or disable the "CRLF mode" flag by default.

- ```rust
  pub fn line_terminator(self: Self, byte: u8) -> Config { /* ... */ }
  ```
  Sets the line terminator for use with `(?u-s:.)` and `(?-us:.)`.

- ```rust
  pub fn swap_greed(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Enable or disable the "swap greed" flag by default.

- ```rust
  pub fn ignore_whitespace(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Enable verbose mode in the regular expression.

- ```rust
  pub fn unicode(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Enable or disable the Unicode flag (`u`) by default.

- ```rust
  pub fn utf8(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  When disabled, the builder will permit the construction of a regular

- ```rust
  pub fn nest_limit(self: Self, limit: u32) -> Config { /* ... */ }
  ```
  Set the nesting limit used for the regular expression parser.

- ```rust
  pub fn octal(self: Self, yes: bool) -> Config { /* ... */ }
  ```
  Whether to support octal syntax or not.

- ```rust
  pub fn get_unicode(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether "unicode" mode is enabled.

- ```rust
  pub fn get_case_insensitive(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether "case insensitive" mode is enabled.

- ```rust
  pub fn get_multi_line(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether "multi line" mode is enabled.

- ```rust
  pub fn get_dot_matches_new_line(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether "dot matches new line" mode is enabled.

- ```rust
  pub fn get_crlf(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether "CRLF" mode is enabled.

- ```rust
  pub fn get_line_terminator(self: &Self) -> u8 { /* ... */ }
  ```
  Returns the line terminator in this syntax configuration.

- ```rust
  pub fn get_swap_greed(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether "swap greed" mode is enabled.

- ```rust
  pub fn get_ignore_whitespace(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether "ignore whitespace" mode is enabled.

- ```rust
  pub fn get_utf8(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether UTF-8 mode is enabled.

- ```rust
  pub fn get_nest_limit(self: &Self) -> u32 { /* ... */ }
  ```
  Returns the "nest limit" setting.

- ```rust
  pub fn get_octal(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether "octal" mode is enabled.

###### Trait Implementations

- **RefUnwindSafe**
- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Config { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Config { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Functions

#### Function `parse`

A convenience routine for parsing a pattern into an HIR value with the
default configuration.

# Example

This shows how to parse a pattern into an HIR value:

```
use regex_automata::util::syntax;

let hir = syntax::parse(r"([a-z]+)|([0-9]+)")?;
assert_eq!(Some(1), hir.properties().static_explicit_captures_len());

# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub fn parse(pattern: &str) -> Result<regex_syntax::hir::Hir, regex_syntax::Error> { /* ... */ }
```

#### Function `parse_many`

A convenience routine for parsing many patterns into HIR value with the
default configuration.

# Example

This shows how to parse many patterns into an corresponding HIR values:

```
use {
    regex_automata::util::syntax,
    regex_syntax::hir::Properties,
};

let hirs = syntax::parse_many(&[
    r"([a-z]+)|([0-9]+)",
    r"foo(A-Z]+)bar",
])?;
let props = Properties::union(hirs.iter().map(|h| h.properties()));
assert_eq!(Some(1), props.static_explicit_captures_len());

# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub fn parse_many<P: AsRef<str>>(patterns: &[P]) -> Result<alloc::vec::Vec<regex_syntax::hir::Hir>, regex_syntax::Error> { /* ... */ }
```

#### Function `parse_with`

A convenience routine for parsing a pattern into an HIR value using a
`Config`.

# Example

This shows how to parse a pattern into an HIR value with a non-default
configuration:

```
use regex_automata::util::syntax;

let hir = syntax::parse_with(
    r"^[a-z]+$",
    &syntax::Config::new().multi_line(true).crlf(true),
)?;
assert!(hir.properties().look_set().contains_anchor_crlf());

# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub fn parse_with(pattern: &str, config: &Config) -> Result<regex_syntax::hir::Hir, regex_syntax::Error> { /* ... */ }
```

#### Function `parse_many_with`

A convenience routine for parsing many patterns into HIR values using a
`Config`.

# Example

This shows how to parse many patterns into an corresponding HIR values
with a non-default configuration:

```
use {
    regex_automata::util::syntax,
    regex_syntax::hir::Properties,
};

let patterns = &[
    r"([a-z]+)|([0-9]+)",
    r"\W",
    r"foo(A-Z]+)bar",
];
let config = syntax::Config::new().unicode(false).utf8(false);
let hirs = syntax::parse_many_with(patterns, &config)?;
let props = Properties::union(hirs.iter().map(|h| h.properties()));
assert!(!props.is_utf8());

# Ok::<(), Box<dyn std::error::Error>>(())
```

```rust
pub fn parse_many_with<P: AsRef<str>>(patterns: &[P], config: &Config) -> Result<alloc::vec::Vec<regex_syntax::hir::Hir>, regex_syntax::Error> { /* ... */ }
```

## Module `wire`

Types and routines that support the wire format of finite automata.

Currently, this module just exports a few error types and some small helpers
for deserializing [dense DFAs](crate::dfa::dense::DFA) using correct alignment.

```rust
pub mod wire { /* ... */ }
```

### Types

#### Struct `AlignAs`

**Attributes:**

- `#[repr(C)]`

A hack to align a smaller type `B` with a bigger type `T`.

The usual use of this is with `B = [u8]` and `T = u32`. That is,
it permits aligning a sequence of bytes on a 4-byte boundary. This
is useful in contexts where one wants to embed a serialized [dense
DFA](crate::dfa::dense::DFA) into a Rust a program while guaranteeing the
alignment required for the DFA.

See [`dense::DFA::from_bytes`](crate::dfa::dense::DFA::from_bytes) for an
example of how to use this type.

```rust
pub struct AlignAs<B: ?Sized, T> {
    pub _align: [T; 0],
    pub bytes: B,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_align` | `[T; 0]` | A zero-sized field indicating the alignment we want. |
| `bytes` | `B` | A possibly non-sized field containing a sequence of bytes. |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `SerializeError`

An error that occurs when serializing an object from this crate.

Serialization, as used in this crate, universally refers to the process
of transforming a structure (like a DFA) into a custom binary format
represented by `&[u8]`. To this end, serialization is generally infallible.
However, it can fail when caller provided buffer sizes are too small. When
that occurs, a serialization error is reported.

A `SerializeError` provides no introspection capabilities. Its only
supported operation is conversion to a human readable error message.

This error type implements the `std::error::Error` trait only when the
`std` feature is enabled. Otherwise, this type is defined in all
configurations.

```rust
pub struct SerializeError {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Error**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Sync**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
#### Struct `DeserializeError`

An error that occurs when deserializing an object defined in this crate.

Serialization, as used in this crate, universally refers to the process
of transforming a structure (like a DFA) into a custom binary format
represented by `&[u8]`. Deserialization, then, refers to the process of
cheaply converting this binary format back to the object's in-memory
representation as defined in this crate. To the extent possible,
deserialization will report this error whenever this process fails.

A `DeserializeError` provides no introspection capabilities. Its only
supported operation is conversion to a human readable error message.

This error type implements the `std::error::Error` trait only when the
`std` feature is enabled. Otherwise, this type is defined in all
configurations.

```rust
pub struct DeserializeError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Error**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Unpin**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Re-exports

### Re-export `PatternID`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::util::primitives::PatternID;
```

### Re-export `crate::util::search::*`

```rust
pub use crate::util::search::*;
```

