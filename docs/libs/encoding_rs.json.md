# Crate Documentation

**Version:** 0.8.35

**Format Version:** 43

# Module `encoding_rs`

encoding_rs is a Gecko-oriented Free Software / Open Source implementation
of the [Encoding Standard](https://encoding.spec.whatwg.org/) in Rust.
Gecko-oriented means that converting to and from UTF-16 is supported in
addition to converting to and from UTF-8, that the performance and
streamability goals are browser-oriented, and that FFI-friendliness is a
goal.

Additionally, the `mem` module provides functions that are useful for
applications that need to be able to deal with legacy in-memory
representations of Unicode.

For expectation setting, please be sure to read the sections
[_UTF-16LE, UTF-16BE and Unicode Encoding Schemes_](#utf-16le-utf-16be-and-unicode-encoding-schemes),
[_ISO-8859-1_](#iso-8859-1) and [_Web / Browser Focus_](#web--browser-focus) below.

There is a [long-form write-up](https://hsivonen.fi/encoding_rs/) about the
design and internals of the crate.

# Availability

The code is available under the
[Apache license, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
or the [MIT license](https://opensource.org/licenses/MIT), at your option.
See the
[`COPYRIGHT`](https://github.com/hsivonen/encoding_rs/blob/master/COPYRIGHT)
file for details.
The [repository is on GitHub](https://github.com/hsivonen/encoding_rs). The
[crate is available on crates.io](https://crates.io/crates/encoding_rs).

# Integration with `std::io`

This crate doesn't implement traits from `std::io`. However, for the case of
wrapping a `std::io::Read` in a decoder that implements `std::io::Read` and
presents the data from the wrapped `std::io::Read` as UTF-8 is addressed by
the [`encoding_rs_io`](https://docs.rs/encoding_rs_io/) crate.

# Examples

Example programs:

* [Rust](https://github.com/hsivonen/recode_rs)
* [C](https://github.com/hsivonen/recode_c)
* [C++](https://github.com/hsivonen/recode_cpp)

Decode using the non-streaming API:

```
#[cfg(feature = "alloc")] {
use encoding_rs::*;

let expectation = "\u{30CF}\u{30ED}\u{30FC}\u{30FB}\u{30EF}\u{30FC}\u{30EB}\u{30C9}";
let bytes = b"\x83n\x83\x8D\x81[\x81E\x83\x8F\x81[\x83\x8B\x83h";

let (cow, encoding_used, had_errors) = SHIFT_JIS.decode(bytes);
assert_eq!(&cow[..], expectation);
assert_eq!(encoding_used, SHIFT_JIS);
assert!(!had_errors);
}
```

Decode using the streaming API with minimal `unsafe`:

```
use encoding_rs::*;

let expectation = "\u{30CF}\u{30ED}\u{30FC}\u{30FB}\u{30EF}\u{30FC}\u{30EB}\u{30C9}";

// Use an array of byte slices to demonstrate content arriving piece by
// piece from the network.
let bytes: [&'static [u8]; 4] = [b"\x83",
                                 b"n\x83\x8D\x81",
                                 b"[\x81E\x83\x8F\x81[\x83",
                                 b"\x8B\x83h"];

// Very short output buffer to demonstrate the output buffer getting full.
// Normally, you'd use something like `[0u8; 2048]`.
let mut buffer_bytes = [0u8; 8];
let mut buffer: &mut str = std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap();

// How many bytes in the buffer currently hold significant data.
let mut bytes_in_buffer = 0usize;

// Collect the output to a string for demonstration purposes.
let mut output = String::new();

// The `Decoder`
let mut decoder = SHIFT_JIS.new_decoder();

// Track whether we see errors.
let mut total_had_errors = false;

// Decode using a fixed-size intermediate buffer (for demonstrating the
// use of a fixed-size buffer; normally when the output of an incremental
// decode goes to a `String` one would use `Decoder.decode_to_string()` to
// avoid the intermediate buffer).
for input in &bytes[..] {
    // The number of bytes already read from current `input` in total.
    let mut total_read_from_current_input = 0usize;

    loop {
        let (result, read, written, had_errors) =
            decoder.decode_to_str(&input[total_read_from_current_input..],
                                  &mut buffer[bytes_in_buffer..],
                                  false);
        total_read_from_current_input += read;
        bytes_in_buffer += written;
        total_had_errors |= had_errors;
        match result {
            CoderResult::InputEmpty => {
                // We have consumed the current input buffer. Break out of
                // the inner loop to get the next input buffer from the
                // outer loop.
                break;
            },
            CoderResult::OutputFull => {
                // Write the current buffer out and consider the buffer
                // empty.
                output.push_str(&buffer[..bytes_in_buffer]);
                bytes_in_buffer = 0usize;
                continue;
            }
        }
    }
}

// Process EOF
loop {
    let (result, _, written, had_errors) =
        decoder.decode_to_str(b"",
                              &mut buffer[bytes_in_buffer..],
                              true);
    bytes_in_buffer += written;
    total_had_errors |= had_errors;
    // Write the current buffer out and consider the buffer empty.
    // Need to do this here for both `match` arms, because we exit the
    // loop on `CoderResult::InputEmpty`.
    output.push_str(&buffer[..bytes_in_buffer]);
    bytes_in_buffer = 0usize;
    match result {
        CoderResult::InputEmpty => {
            // Done!
            break;
        },
        CoderResult::OutputFull => {
            continue;
        }
    }
}

assert_eq!(&output[..], expectation);
assert!(!total_had_errors);
```

## UTF-16LE, UTF-16BE and Unicode Encoding Schemes

The Encoding Standard doesn't specify encoders for UTF-16LE and UTF-16BE,
__so this crate does not provide encoders for those encodings__!
Along with the replacement encoding, their _output encoding_ (i.e. the
encoding used for form submission and error handling in the query string
of URLs) is UTF-8, so you get an UTF-8 encoder if you request an encoder
for them.

Additionally, the Encoding Standard factors BOM handling into wrapper
algorithms so that BOM handling isn't part of the definition of the
encodings themselves. The Unicode _encoding schemes_ in the Unicode
Standard define BOM handling or lack thereof as part of the encoding
scheme.

When used with the `_without_bom_handling` entry points, the UTF-16LE
and UTF-16BE _encodings_ match the same-named _encoding schemes_ from
the Unicode Standard.

When used with the `_with_bom_removal` entry points, the UTF-8
_encoding_ matches the UTF-8 _encoding scheme_ from the Unicode
Standard.

This crate does not provide a mode that matches the UTF-16 _encoding
scheme_ from the Unicode Stardard. The UTF-16BE encoding used with
the entry points without `_bom_` qualifiers is the closest match,
but in that case, the UTF-8 BOM triggers UTF-8 decoding, which is
not part of the behavior of the UTF-16 _encoding scheme_ per the
Unicode Standard.

The UTF-32 family of Unicode encoding schemes is not supported
by this crate. The Encoding Standard doesn't define any UTF-32
family encodings, since they aren't necessary for consuming Web
content.

While gb18030 is capable of representing U+FEFF, the Encoding
Standard does not treat the gb18030 byte representation of U+FEFF
as a BOM, so neither does this crate.

## ISO-8859-1

ISO-8859-1 does not exist as a distinct encoding from windows-1252 in
the Encoding Standard. Therefore, an encoding that maps the unsigned
byte value to the same Unicode scalar value is not available via
`Encoding` in this crate.

However, the functions whose name starts with `convert` and contains
`latin1` in the `mem` module support such conversions, which are known as
[_isomorphic decode_](https://infra.spec.whatwg.org/#isomorphic-decode)
and [_isomorphic encode_](https://infra.spec.whatwg.org/#isomorphic-encode)
in the [Infra Standard](https://infra.spec.whatwg.org/).

## Web / Browser Focus

Both in terms of scope and performance, the focus is on the Web. For scope,
this means that encoding_rs implements the Encoding Standard fully and
doesn't implement encodings that are not specified in the Encoding
Standard. For performance, this means that decoding performance is
important as well as performance for encoding into UTF-8 or encoding the
Basic Latin range (ASCII) into legacy encodings. Non-Basic Latin needs to
be encoded into legacy encodings in only two places in the Web platform: in
the query part of URLs, in which case it's a matter of relatively rare
error handling, and in form submission, in which case the user action and
networking tend to hide the performance of the encoder.

Deemphasizing performance of encoding non-Basic Latin text into legacy
encodings enables smaller code size thanks to the encoder side using the
decode-optimized data tables without having encode-optimized data tables at
all. Even in decoders, smaller lookup table size is preferred over avoiding
multiplication operations.

Additionally, performance is a non-goal for the ASCII-incompatible
ISO-2022-JP encoding, which are rarely used on the Web. Instead of
performance, the decoder for ISO-2022-JP optimizes for ease/clarity
of implementation.

Despite the browser focus, the hope is that non-browser applications
that wish to consume Web content or submit Web forms in a Web-compatible
way will find encoding_rs useful. While encoding_rs does not try to match
Windows behavior, many of the encodings are close enough to legacy
encodings implemented by Windows that applications that need to consume
data in legacy Windows encodins may find encoding_rs useful. The
[codepage](https://crates.io/crates/codepage) crate maps from Windows
code page identifiers onto encoding_rs `Encoding`s and vice versa.

For decoding email, UTF-7 support is needed (unfortunately) in additition
to the encodings defined in the Encoding Standard. The
[charset](https://crates.io/crates/charset) wraps encoding_rs and adds
UTF-7 decoding for email purposes.

For single-byte DOS encodings beyond the ones supported by the Encoding
Standard, there is the [`oem_cp`](https://crates.io/crates/oem_cp) crate.

# Preparing Text for the Encoders

Normalizing text into Unicode Normalization Form C prior to encoding text
into a legacy encoding minimizes unmappable characters. Text can be
normalized to Unicode Normalization Form C using the
[`icu_normalizer`](https://crates.io/crates/icu_normalizer) crate, which
is part of [ICU4X](https://icu4x.unicode.org/).

The exception is windows-1258, which after normalizing to Unicode
Normalization Form C requires tone marks to be decomposed in order to
minimize unmappable characters. Vietnamese tone marks can be decomposed
using the [`detone`](https://crates.io/crates/detone) crate.

# Streaming & Non-Streaming; Rust & C/C++

The API in Rust has two modes of operation: streaming and non-streaming.
The streaming API is the foundation of the implementation and should be
used when processing data that arrives piecemeal from an i/o stream. The
streaming API has an FFI wrapper (as a [separate crate][1]) that exposes it
to C callers. The non-streaming part of the API is for Rust callers only and
is smart about borrowing instead of copying when possible. When
streamability is not needed, the non-streaming API should be preferrer in
order to avoid copying data when a borrow suffices.

There is no analogous C API exposed via FFI, mainly because C doesn't have
standard types for growable byte buffers and Unicode strings that know
their length.

The C API (header file generated at `target/include/encoding_rs.h` when
building encoding_rs) can, in turn, be wrapped for use from C++. Such a
C++ wrapper can re-create the non-streaming API in C++ for C++ callers.
The C binding comes with a [C++17 wrapper][2] that uses standard library +
[GSL][3] types and that recreates the non-streaming API in C++ on top of
the streaming API. A C++ wrapper with XPCOM/MFBT types is available as
[`mozilla::Encoding`][4].

The `Encoding` type is common to both the streaming and non-streaming
modes. In the streaming mode, decoding operations are performed with a
`Decoder` and encoding operations with an `Encoder` object obtained via
`Encoding`. In the non-streaming mode, decoding and encoding operations are
performed using methods on `Encoding` objects themselves, so the `Decoder`
and `Encoder` objects are not used at all.

[1]: https://github.com/hsivonen/encoding_c
[2]: https://github.com/hsivonen/encoding_c/blob/master/include/encoding_rs_cpp.h
[3]: https://github.com/Microsoft/GSL/
[4]: https://searchfox.org/mozilla-central/source/intl/Encoding.h

# Memory management

The non-streaming mode never performs heap allocations (even the methods
that write into a `Vec<u8>` or a `String` by taking them as arguments do
not reallocate the backing buffer of the `Vec<u8>` or the `String`). That
is, the non-streaming mode uses caller-allocated buffers exclusively.

The methods of the streaming mode that return a `Vec<u8>` or a `String`
perform heap allocations but only to allocate the backing buffer of the
`Vec<u8>` or the `String`.

`Encoding` is always statically allocated. `Decoder` and `Encoder` need no
`Drop` cleanup.

# Buffer reading and writing behavior

Based on experience gained with the `java.nio.charset` encoding converter
API and with the Gecko uconv encoding converter API, the buffer reading
and writing behaviors of encoding_rs are asymmetric: input buffers are
fully drained but output buffers are not always fully filled.

When reading from an input buffer, encoding_rs always consumes all input
up to the next error or to the end of the buffer. In particular, when
decoding, even if the input buffer ends in the middle of a byte sequence
for a character, the decoder consumes all input. This has the benefit that
the caller of the API can always fill the next buffer from the start from
whatever source the bytes come from and never has to first copy the last
bytes of the previous buffer to the start of the next buffer. However, when
encoding, the UTF-8 input buffers have to end at a character boundary, which
is a requirement for the Rust `str` type anyway, and UTF-16 input buffer
boundaries falling in the middle of a surrogate pair result in both
suggorates being treated individually as unpaired surrogates.

Additionally, decoders guarantee that they can be fed even one byte at a
time and encoders guarantee that they can be fed even one code point at a
time. This has the benefit of not placing restrictions on the size of
chunks the content arrives e.g. from network.

When writing into an output buffer, encoding_rs makes sure that the code
unit sequence for a character is never split across output buffer
boundaries. This may result in wasted space at the end of an output buffer,
but the advantages are that the output side of both decoders and encoders
is greatly simplified compared to designs that attempt to fill output
buffers exactly even when that entails splitting a code unit sequence and
when encoding_rs methods return to the caller, the output produces thus
far is always valid taken as whole. (In the case of encoding to ISO-2022-JP,
the output needs to be considered as a whole, because the latest output
buffer taken alone might not be valid taken alone if the transition away
from the ASCII state occurred in an earlier output buffer. However, since
the ISO-2022-JP decoder doesn't treat streams that don't end in the ASCII
state as being in error despite the encoder generating a transition to the
ASCII state at the end, the claim about the partial output taken as a whole
being valid is true even for ISO-2022-JP.)

# Error Reporting

Based on experience gained with the `java.nio.charset` encoding converter
API and with the Gecko uconv encoding converter API, the error reporting
behaviors of encoding_rs are asymmetric: decoder errors include offsets
that leave it up to the caller to extract the erroneous bytes from the
input stream if the caller wishes to do so but encoder errors provide the
code point associated with the error without requiring the caller to
extract it from the input on its own.

On the encoder side, an error is always triggered by the most recently
pushed Unicode scalar, which makes it simple to pass the `char` to the
caller. Also, it's very typical for the caller to wish to do something with
this data: generate a numeric escape for the character. Additionally, the
ISO-2022-JP encoder reports U+FFFD instead of the actual input character in
certain cases, so requiring the caller to extract the character from the
input buffer would require the caller to handle ISO-2022-JP details.
Furthermore, requiring the caller to extract the character from the input
buffer would require the caller to implement UTF-8 or UTF-16 math, which is
the job of an encoding conversion library.

On the decoder side, errors are triggered in more complex ways. For
example, when decoding the sequence ESC, '$', _buffer boundary_, 'A' as
ISO-2022-JP, the ESC byte is in error, but this is discovered only after
the buffer boundary when processing 'A'. Thus, the bytes in error might not
be the ones most recently pushed to the decoder and the error might not even
be in the current buffer.

Some encoding conversion APIs address the problem by not acknowledging
trailing bytes of an input buffer as consumed if it's still possible for
future bytes to cause the trailing bytes to be in error. This way, error
reporting can always refer to the most recently pushed buffer. This has the
problem that the caller of the API has to copy the unconsumed trailing
bytes to the start of the next buffer before being able to fill the rest
of the next buffer. This is annoying, error-prone and inefficient.

A possible solution would be making the decoder remember recently consumed
bytes in order to be able to include a copy of the erroneous bytes when
reporting an error. This has two problem: First, callers a rarely
interested in the erroneous bytes, so attempts to identify them are most
often just overhead anyway. Second, the rare applications that are
interested typically care about the location of the error in the input
stream.

To keep the API convenient for common uses and the overhead low while making
it possible to develop applications, such as HTML validators, that care
about which bytes were in error, encoding_rs reports the length of the
erroneous sequence and the number of bytes consumed after the erroneous
sequence. As long as the caller doesn't discard the 6 most recent bytes,
this makes it possible for callers that care about the erroneous bytes to
locate them.

# No Convenience API for Custom Replacements

The Web Platform and, therefore, the Encoding Standard supports only one
error recovery mode for decoders and only one error recovery mode for
encoders. The supported error recovery mode for decoders is emitting the
REPLACEMENT CHARACTER on error. The supported error recovery mode for
encoders is emitting an HTML decimal numeric character reference for
unmappable characters.

Since encoding_rs is Web-focused, these are the only error recovery modes
for which convenient support is provided. Moreover, on the decoder side,
there aren't really good alternatives for emitting the REPLACEMENT CHARACTER
on error (other than treating errors as fatal). In particular, simply
ignoring errors is a
[security problem](http://www.unicode.org/reports/tr36/#Substituting_for_Ill_Formed_Subsequences),
so it would be a bad idea for encoding_rs to provide a mode that encouraged
callers to ignore errors.

On the encoder side, there are plausible alternatives for HTML decimal
numeric character references. For example, when outputting CSS, CSS-style
escapes would seem to make sense. However, instead of facilitating the
output of CSS, JS, etc. in non-UTF-8 encodings, encoding_rs takes the design
position that you shouldn't generate output in encodings other than UTF-8,
except where backward compatibility with interacting with the legacy Web
requires it. The legacy Web requires it only when parsing the query strings
of URLs and when submitting forms, and those two both use HTML decimal
numeric character references.

While encoding_rs doesn't make encoder replacements other than HTML decimal
numeric character references easy, it does make them _possible_.
`encode_from_utf8()`, which emits HTML decimal numeric character references
for unmappable characters, is implemented on top of
`encode_from_utf8_without_replacement()`. Applications that really, really
want other replacement schemes for unmappable characters can likewise
implement them on top of `encode_from_utf8_without_replacement()`.

# No Extensibility by Design

The set of encodings supported by encoding_rs is not extensible by design.
That is, `Encoding`, `Decoder` and `Encoder` are intentionally `struct`s
rather than `trait`s. encoding_rs takes the design position that all future
text interchange should be done using UTF-8, which can represent all of
Unicode. (It is, in fact, the only encoding supported by the Encoding
Standard and encoding_rs that can represent all of Unicode and that has
encoder support. UTF-16LE and UTF-16BE don't have encoder support, and
gb18030 cannot encode U+E5E5.) The other encodings are supported merely for
legacy compatibility and not due to non-UTF-8 encodings having benefits
other than being able to consume legacy content.

Considering that UTF-8 can represent all of Unicode and is already supported
by all Web browsers, introducing a new encoding wouldn't add to the
expressiveness but would add to compatibility problems. In that sense,
adding new encodings to the Web Platform doesn't make sense, and, in fact,
post-UTF-8 attempts at encodings, such as BOCU-1, have been rejected from
the Web Platform. On the other hand, the set of legacy encodings that must
be supported for a Web browser to be able to be successful is not going to
expand. Empirically, the set of encodings specified in the Encoding Standard
is already sufficient and the set of legacy encodings won't grow
retroactively.

Since extensibility doesn't make sense considering the Web focus of
encoding_rs and adding encodings to Web clients would be actively harmful,
it makes sense to make the set of encodings that encoding_rs supports
non-extensible and to take the (admittedly small) benefits arising from
that, such as the size of `Decoder` and `Encoder` objects being known ahead
 of time, which enables stack allocation thereof.

This does have downsides for applications that might want to put encoding_rs
to non-Web uses if those non-Web uses involve legacy encodings that aren't
needed for Web uses. The needs of such applications should not complicate
encoding_rs itself, though. It is up to those applications to provide a
framework that delegates the operations with encodings that encoding_rs
supports to encoding_rs and operations with other encodings to something
else (as opposed to encoding_rs itself providing an extensibility
framework).

# Panics

Methods in encoding_rs can panic if the API is used against the requirements
stated in the documentation, if a state that's supposed to be impossible
is reached due to an internal bug or on integer overflow. When used
according to documentation with buffer sizes that stay below integer
overflow, in the absence of internal bugs, encoding_rs does not panic.

Panics arising from API misuse aren't documented beyond this on individual
methods.

# At-Risk Parts of the API

The foreseeable source of partially backward-incompatible API change is the
way the instances of `Encoding` are made available.

If Rust changes to allow the entries of `[&'static Encoding; N]` to be
initialized with `static`s of type `&'static Encoding`, the non-reference
`FOO_INIT` public `Encoding` instances will be removed from the public API.

If Rust changes to make the referent of `pub const FOO: &'static Encoding`
unique when the constant is used in different crates, the reference-typed
`static`s for the encoding instances will be changed from `static` to
`const` and the non-reference-typed `_INIT` instances will be removed.

# Mapping Spec Concepts onto the API

<table>
<thead>
<tr><th>Spec Concept</th><th>Streaming</th><th>Non-Streaming</th></tr>
</thead>
<tbody>
<tr><td><a href="https://encoding.spec.whatwg.org/#encoding">encoding</a></td><td><code>&amp;'static Encoding</code></td><td><code>&amp;'static Encoding</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#utf-8">UTF-8 encoding</a></td><td><code>UTF_8</code></td><td><code>UTF_8</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#concept-encoding-get">get an encoding</a></td><td><code>Encoding::for_label(<var>label</var>)</code></td><td><code>Encoding::for_label(<var>label</var>)</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#name">name</a></td><td><code><var>encoding</var>.name()</code></td><td><code><var>encoding</var>.name()</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#get-an-output-encoding">get an output encoding</a></td><td><code><var>encoding</var>.output_encoding()</code></td><td><code><var>encoding</var>.output_encoding()</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#decode">decode</a></td><td><code>let d = <var>encoding</var>.new_decoder();<br>let res = d.decode_to_<var>*</var>(<var>src</var>, <var>dst</var>, false);<br>// &hellip;</br>let last_res = d.decode_to_<var>*</var>(<var>src</var>, <var>dst</var>, true);</code></td><td><code><var>encoding</var>.decode(<var>src</var>)</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#utf-8-decode">UTF-8 decode</a></td><td><code>let d = UTF_8.new_decoder_with_bom_removal();<br>let res = d.decode_to_<var>*</var>(<var>src</var>, <var>dst</var>, false);<br>// &hellip;</br>let last_res = d.decode_to_<var>*</var>(<var>src</var>, <var>dst</var>, true);</code></td><td><code>UTF_8.decode_with_bom_removal(<var>src</var>)</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#utf-8-decode-without-bom">UTF-8 decode without BOM</a></td><td><code>let d = UTF_8.new_decoder_without_bom_handling();<br>let res = d.decode_to_<var>*</var>(<var>src</var>, <var>dst</var>, false);<br>// &hellip;</br>let last_res = d.decode_to_<var>*</var>(<var>src</var>, <var>dst</var>, true);</code></td><td><code>UTF_8.decode_without_bom_handling(<var>src</var>)</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#utf-8-decode-without-bom-or-fail">UTF-8 decode without BOM or fail</a></td><td><code>let d = UTF_8.new_decoder_without_bom_handling();<br>let res = d.decode_to_<var>*</var>_without_replacement(<var>src</var>, <var>dst</var>, false);<br>// &hellip; (fail if malformed)</br>let last_res = d.decode_to_<var>*</var>_without_replacement(<var>src</var>, <var>dst</var>, true);<br>// (fail if malformed)</code></td><td><code>UTF_8.decode_without_bom_handling_and_without_replacement(<var>src</var>)</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#encode">encode</a></td><td><code>let e = <var>encoding</var>.new_encoder();<br>let res = e.encode_to_<var>*</var>(<var>src</var>, <var>dst</var>, false);<br>// &hellip;</br>let last_res = e.encode_to_<var>*</var>(<var>src</var>, <var>dst</var>, true);</code></td><td><code><var>encoding</var>.encode(<var>src</var>)</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#utf-8-encode">UTF-8 encode</a></td><td>Use the UTF-8 nature of Rust strings directly:<br><code><var>write</var>(<var>src</var>.as_bytes());<br>// refill src<br><var>write</var>(<var>src</var>.as_bytes());<br>// refill src<br><var>write</var>(<var>src</var>.as_bytes());<br>// &hellip;</code></td><td>Use the UTF-8 nature of Rust strings directly:<br><code><var>src</var>.as_bytes()</code></td></tr>
</tbody>
</table>

# Compatibility with the rust-encoding API

The crate
[encoding_rs_compat](https://github.com/hsivonen/encoding_rs_compat/)
is a drop-in replacement for rust-encoding 0.2.32 that implements (most of)
the API of rust-encoding 0.2.32 on top of encoding_rs.

# Mapping rust-encoding concepts to encoding_rs concepts

The following table provides a mapping from rust-encoding constructs to
encoding_rs ones.

<table>
<thead>
<tr><th>rust-encoding</th><th>encoding_rs</th></tr>
</thead>
<tbody>
<tr><td><code>encoding::EncodingRef</code></td><td><code>&amp;'static encoding_rs::Encoding</code></td></tr>
<tr><td><code>encoding::all::<var>WINDOWS_31J</var></code> (not based on the WHATWG name for some encodings)</td><td><code>encoding_rs::<var>SHIFT_JIS</var></code> (always the WHATWG name uppercased and hyphens replaced with underscores)</td></tr>
<tr><td><code>encoding::all::ERROR</code></td><td>Not available because not in the Encoding Standard</td></tr>
<tr><td><code>encoding::all::ASCII</code></td><td>Not available because not in the Encoding Standard</td></tr>
<tr><td><code>encoding::all::ISO_8859_1</code></td><td>Not available because not in the Encoding Standard</td></tr>
<tr><td><code>encoding::all::HZ</code></td><td>Not available because not in the Encoding Standard</td></tr>
<tr><td><code>encoding::label::encoding_from_whatwg_label(<var>string</var>)</code></td><td><code>encoding_rs::Encoding::for_label(<var>string</var>)</code></td></tr>
<tr><td><code><var>enc</var>.whatwg_name()</code> (always lower case)</td><td><code><var>enc</var>.name()</code> (potentially mixed case)</td></tr>
<tr><td><code><var>enc</var>.name()</code></td><td>Not available because not in the Encoding Standard</td></tr>
<tr><td><code>encoding::decode(<var>bytes</var>, encoding::DecoderTrap::Replace, <var>enc</var>)</code></td><td><code><var>enc</var>.decode(<var>bytes</var>)</code></td></tr>
<tr><td><code><var>enc</var>.decode(<var>bytes</var>, encoding::DecoderTrap::Replace)</code></td><td><code><var>enc</var>.decode_without_bom_handling(<var>bytes</var>)</code></td></tr>
<tr><td><code><var>enc</var>.encode(<var>string</var>, encoding::EncoderTrap::NcrEscape)</code></td><td><code><var>enc</var>.encode(<var>string</var>)</code></td></tr>
<tr><td><code><var>enc</var>.raw_decoder()</code></td><td><code><var>enc</var>.new_decoder_without_bom_handling()</code></td></tr>
<tr><td><code><var>enc</var>.raw_encoder()</code></td><td><code><var>enc</var>.new_encoder()</code></td></tr>
<tr><td><code>encoding::RawDecoder</code></td><td><code>encoding_rs::Decoder</code></td></tr>
<tr><td><code>encoding::RawEncoder</code></td><td><code>encoding_rs::Encoder</code></td></tr>
<tr><td><code><var>raw_decoder</var>.raw_feed(<var>src</var>, <var>dst_string</var>)</code></td><td><code><var>dst_string</var>.reserve(<var>decoder</var>.max_utf8_buffer_length_without_replacement(<var>src</var>.len()));<br><var>decoder</var>.decode_to_string_without_replacement(<var>src</var>, <var>dst_string</var>, false)</code></td></tr>
<tr><td><code><var>raw_encoder</var>.raw_feed(<var>src</var>, <var>dst_vec</var>)</code></td><td><code><var>dst_vec</var>.reserve(<var>encoder</var>.max_buffer_length_from_utf8_without_replacement(<var>src</var>.len()));<br><var>encoder</var>.encode_from_utf8_to_vec_without_replacement(<var>src</var>, <var>dst_vec</var>, false)</code></td></tr>
<tr><td><code><var>raw_decoder</var>.raw_finish(<var>dst</var>)</code></td><td><code><var>dst_string</var>.reserve(<var>decoder</var>.max_utf8_buffer_length_without_replacement(0));<br><var>decoder</var>.decode_to_string_without_replacement(b"", <var>dst</var>, true)</code></td></tr>
<tr><td><code><var>raw_encoder</var>.raw_finish(<var>dst</var>)</code></td><td><code><var>dst_vec</var>.reserve(<var>encoder</var>.max_buffer_length_from_utf8_without_replacement(0));<br><var>encoder</var>.encode_from_utf8_to_vec_without_replacement("", <var>dst</var>, true)</code></td></tr>
<tr><td><code>encoding::DecoderTrap::Strict</code></td><td><code>decode*</code> methods that have <code>_without_replacement</code> in their name (and treating the `Malformed` result as fatal).</td></tr>
<tr><td><code>encoding::DecoderTrap::Replace</code></td><td><code>decode*</code> methods that <i>do not</i> have <code>_without_replacement</code> in their name.</td></tr>
<tr><td><code>encoding::DecoderTrap::Ignore</code></td><td>It is a bad idea to ignore errors due to security issues, but this could be implemented using <code>decode*</code> methods that have <code>_without_replacement</code> in their name.</td></tr>
<tr><td><code>encoding::DecoderTrap::Call(DecoderTrapFunc)</code></td><td>Can be implemented using <code>decode*</code> methods that have <code>_without_replacement</code> in their name.</td></tr>
<tr><td><code>encoding::EncoderTrap::Strict</code></td><td><code>encode*</code> methods that have <code>_without_replacement</code> in their name (and treating the `Unmappable` result as fatal).</td></tr>
<tr><td><code>encoding::EncoderTrap::Replace</code></td><td>Can be implemented using <code>encode*</code> methods that have <code>_without_replacement</code> in their name.</td></tr>
<tr><td><code>encoding::EncoderTrap::Ignore</code></td><td>It is a bad idea to ignore errors due to security issues, but this could be implemented using <code>encode*</code> methods that have <code>_without_replacement</code> in their name.</td></tr>
<tr><td><code>encoding::EncoderTrap::NcrEscape</code></td><td><code>encode*</code> methods that <i>do not</i> have <code>_without_replacement</code> in their name.</td></tr>
<tr><td><code>encoding::EncoderTrap::Call(EncoderTrapFunc)</code></td><td>Can be implemented using <code>encode*</code> methods that have <code>_without_replacement</code> in their name.</td></tr>
</tbody>
</table>

# Relationship with Windows Code Pages

Despite the Web and browser focus, the encodings defined by the Encoding
Standard and implemented by this crate may be useful for decoding legacy
data that uses Windows code pages. The following table names the single-byte
encodings
that have a closely related Windows code page, the number of the closest
code page, a column indicating whether Windows maps unassigned code points
to the Unicode Private Use Area instead of U+FFFD and a remark number
indicating remarks in the list after the table.

<table>
<thead>
<tr><th>Encoding</th><th>Code Page</th><th>PUA</th><th>Remarks</th></tr>
</thead>
<tbody>
<tr><td>Shift_JIS</td><td>932</td><td></td><td></td></tr>
<tr><td>GBK</td><td>936</td><td></td><td></td></tr>
<tr><td>EUC-KR</td><td>949</td><td></td><td></td></tr>
<tr><td>Big5</td><td>950</td><td></td><td></td></tr>
<tr><td>IBM866</td><td>866</td><td></td><td></td></tr>
<tr><td>windows-874</td><td>874</td><td>&bullet;</td><td></td></tr>
<tr><td>UTF-16LE</td><td>1200</td><td></td><td></td></tr>
<tr><td>UTF-16BE</td><td>1201</td><td></td><td></td></tr>
<tr><td>windows-1250</td><td>1250</td><td></td><td></td></tr>
<tr><td>windows-1251</td><td>1251</td><td></td><td></td></tr>
<tr><td>windows-1252</td><td>1252</td><td></td><td></td></tr>
<tr><td>windows-1253</td><td>1253</td><td>&bullet;</td><td></td></tr>
<tr><td>windows-1254</td><td>1254</td><td></td><td></td></tr>
<tr><td>windows-1255</td><td>1255</td><td>&bullet;</td><td></td></tr>
<tr><td>windows-1256</td><td>1256</td><td></td><td></td></tr>
<tr><td>windows-1257</td><td>1257</td><td>&bullet;</td><td></td></tr>
<tr><td>windows-1258</td><td>1258</td><td></td><td></td></tr>
<tr><td>macintosh</td><td>10000</td><td></td><td>1</td></tr>
<tr><td>x-mac-cyrillic</td><td>10017</td><td></td><td>2</td></tr>
<tr><td>KOI8-R</td><td>20866</td><td></td><td></td></tr>
<tr><td>EUC-JP</td><td>20932</td><td></td><td></td></tr>
<tr><td>KOI8-U</td><td>21866</td><td></td><td></td></tr>
<tr><td>ISO-8859-2</td><td>28592</td><td></td><td></td></tr>
<tr><td>ISO-8859-3</td><td>28593</td><td></td><td></td></tr>
<tr><td>ISO-8859-4</td><td>28594</td><td></td><td></td></tr>
<tr><td>ISO-8859-5</td><td>28595</td><td></td><td></td></tr>
<tr><td>ISO-8859-6</td><td>28596</td><td>&bullet;</td><td></td></tr>
<tr><td>ISO-8859-7</td><td>28597</td><td>&bullet;</td><td>3</td></tr>
<tr><td>ISO-8859-8</td><td>28598</td><td>&bullet;</td><td>4</td></tr>
<tr><td>ISO-8859-13</td><td>28603</td><td>&bullet;</td><td></td></tr>
<tr><td>ISO-8859-15</td><td>28605</td><td></td><td></td></tr>
<tr><td>ISO-8859-8-I</td><td>38598</td><td></td><td>5</td></tr>
<tr><td>ISO-2022-JP</td><td>50220</td><td></td><td></td></tr>
<tr><td>gb18030</td><td>54936</td><td></td><td></td></tr>
<tr><td>UTF-8</td><td>65001</td><td></td><td></td></tr>
</tbody>
</table>

1. Windows decodes 0xBD to U+2126 OHM SIGN instead of U+03A9 GREEK CAPITAL LETTER OMEGA.
2. Windows decodes 0xFF to U+00A4 CURRENCY SIGN instead of U+20AC EURO SIGN.
3. Windows decodes the currency signs at 0xA4 and 0xA5 as well as 0xAA,
   which should be U+037A GREEK YPOGEGRAMMENI, to PUA code points. Windows
   decodes 0xA1 to U+02BD MODIFIER LETTER REVERSED COMMA instead of U+2018
   LEFT SINGLE QUOTATION MARK and 0xA2 to U+02BC MODIFIER LETTER APOSTROPHE
   instead of U+2019 RIGHT SINGLE QUOTATION MARK.
4. Windows decodes 0xAF to OVERLINE instead of MACRON and 0xFE and 0xFD to PUA instead
   of LRM and RLM.
5. Remarks from the previous item apply.

The differences between this crate and Windows in the case of multibyte encodings
are not yet fully documented here. The lack of remarks above should not be taken
as indication of lack of differences.

# Notable Differences from IANA Naming

In some cases, the Encoding Standard specifies the popular unextended encoding
name where in IANA terms one of the other labels would be more precise considering
the extensions that the Encoding Standard has unified into the encoding.

<table>
<thead>
<tr><th>Encoding</th><th>IANA</th></tr>
</thead>
<tbody>
<tr><td>Big5</td><td>Big5-HKSCS</td></tr>
<tr><td>EUC-KR</td><td>windows-949</td></tr>
<tr><td>Shift_JIS</td><td>windows-31j</td></tr>
<tr><td>x-mac-cyrillic</td><td>x-mac-ukrainian</td></tr>
</tbody>
</table>

In other cases where the Encoding Standard unifies unextended and extended
variants of an encoding, the encoding gets the name of the extended
variant.

<table>
<thead>
<tr><th>IANA</th><th>Unified into Encoding</th></tr>
</thead>
<tbody>
<tr><td>ISO-8859-1</td><td>windows-1252</td></tr>
<tr><td>ISO-8859-9</td><td>windows-1254</td></tr>
<tr><td>TIS-620</td><td>windows-874</td></tr>
</tbody>
</table>

See the section [_UTF-16LE, UTF-16BE and Unicode Encoding Schemes_](#utf-16le-utf-16be-and-unicode-encoding-schemes)
for discussion about the UTF-16 family.

## Modules

## Module `mem`

Functions for converting between different in-RAM representations of text
and for quickly checking if the Unicode Bidirectional Algorithm can be
avoided.

By using slices for output, the functions here seek to enable by-register
(ALU register or SIMD register as available) operations in order to
outperform iterator-based conversions available in the Rust standard
library.

_Note:_ "Latin1" in this module refers to the Unicode range from U+0000 to
U+00FF, inclusive, and does not refer to the windows-1252 range. This
in-memory encoding is sometimes used as a storage optimization of text
when UTF-16 indexing and length semantics are exposed.

The FFI binding for this module are in the
[encoding_c_mem crate](https://github.com/hsivonen/encoding_c_mem).

```rust
pub mod mem { /* ... */ }
```

### Types

#### Enum `Latin1Bidi`

**Attributes:**

- `#[must_use]`
- `#[repr(C)]`

Classification of text as Latin1 (all code points are below U+0100),
left-to-right with some non-Latin1 characters or as containing at least
some right-to-left characters.

```rust
pub enum Latin1Bidi {
    Latin1 = 0,
    LeftToRight = 1,
    Bidi = 2,
}
```

##### Variants

###### `Latin1`

Every character is below U+0100.

Discriminant: `0`

Discriminant value: `0`

###### `LeftToRight`

There is at least one character that's U+0100 or higher, but there
are no right-to-left characters.

Discriminant: `1`

Discriminant value: `1`

###### `Bidi`

There is at least one right-to-left character.

Discriminant: `2`

Discriminant value: `2`

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **Sync**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Latin1Bidi) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Functions

#### Function `is_ascii`

Checks whether the buffer is all-ASCII.

May read the entire buffer even if it isn't all-ASCII. (I.e. the function
is not guaranteed to fail fast.)

```rust
pub fn is_ascii(buffer: &[u8]) -> bool { /* ... */ }
```

#### Function `is_basic_latin`

Checks whether the buffer is all-Basic Latin (i.e. UTF-16 representing
only ASCII characters).

May read the entire buffer even if it isn't all-ASCII. (I.e. the function
is not guaranteed to fail fast.)

```rust
pub fn is_basic_latin(buffer: &[u16]) -> bool { /* ... */ }
```

#### Function `is_utf8_latin1`

Checks whether the buffer is valid UTF-8 representing only code points
less than or equal to U+00FF.

Fails fast. (I.e. returns before having read the whole buffer if UTF-8
invalidity or code points above U+00FF are discovered.

```rust
pub fn is_utf8_latin1(buffer: &[u8]) -> bool { /* ... */ }
```

#### Function `is_str_latin1`

Checks whether the buffer represents only code points less than or equal
to U+00FF.

Fails fast. (I.e. returns before having read the whole buffer if code
points above U+00FF are discovered.

```rust
pub fn is_str_latin1(buffer: &str) -> bool { /* ... */ }
```

#### Function `is_utf16_latin1`

Checks whether the buffer represents only code point less than or equal
to U+00FF.

May read the entire buffer even if it isn't all-Latin1. (I.e. the function
is not guaranteed to fail fast.)

```rust
pub fn is_utf16_latin1(buffer: &[u16]) -> bool { /* ... */ }
```

#### Function `is_utf8_bidi`

**Attributes:**

- `#[<cfg_attr>(feature = "cargo-clippy",
allow(collapsible_if, cyclomatic_complexity))]`
- `#[inline]`

Checks whether a potentially-invalid UTF-8 buffer contains code points
that trigger right-to-left processing.

The check is done on a Unicode block basis without regard to assigned
vs. unassigned code points in the block. Hebrew presentation forms in
the Alphabetic Presentation Forms block are treated as if they formed
a block on their own (i.e. it treated as right-to-left). Additionally,
the four RIGHT-TO-LEFT FOO controls in General Punctuation are checked
for. Control characters that are technically bidi controls but do not
cause right-to-left behavior without the presence of right-to-left
characters or right-to-left controls are not checked for. As a special
case, U+FEFF is excluded from Arabic Presentation Forms-B.

Returns `true` if the input is invalid UTF-8 or the input contains an
RTL character. Returns `false` if the input is valid UTF-8 and contains
no RTL characters.

```rust
pub fn is_utf8_bidi(buffer: &[u8]) -> bool { /* ... */ }
```

#### Function `is_str_bidi`

**Attributes:**

- `#[<cfg_attr>(feature = "cargo-clippy", allow(collapsible_if))]`
- `#[inline]`

Checks whether a valid UTF-8 buffer contains code points that trigger
right-to-left processing.

The check is done on a Unicode block basis without regard to assigned
vs. unassigned code points in the block. Hebrew presentation forms in
the Alphabetic Presentation Forms block are treated as if they formed
a block on their own (i.e. it treated as right-to-left). Additionally,
the four RIGHT-TO-LEFT FOO controls in General Punctuation are checked
for. Control characters that are technically bidi controls but do not
cause right-to-left behavior without the presence of right-to-left
characters or right-to-left controls are not checked for. As a special
case, U+FEFF is excluded from Arabic Presentation Forms-B.

```rust
pub fn is_str_bidi(buffer: &str) -> bool { /* ... */ }
```

#### Function `is_utf16_bidi`

Checks whether a UTF-16 buffer contains code points that trigger
right-to-left processing.

The check is done on a Unicode block basis without regard to assigned
vs. unassigned code points in the block. Hebrew presentation forms in
the Alphabetic Presentation Forms block are treated as if they formed
a block on their own (i.e. it treated as right-to-left). Additionally,
the four RIGHT-TO-LEFT FOO controls in General Punctuation are checked
for. Control characters that are technically bidi controls but do not
cause right-to-left behavior without the presence of right-to-left
characters or right-to-left controls are not checked for. As a special
case, U+FEFF is excluded from Arabic Presentation Forms-B.

Returns `true` if the input contains an RTL character or an unpaired
high surrogate that could be the high half of an RTL character.
Returns `false` if the input contains neither RTL characters nor
unpaired high surrogates that could be higher halves of RTL characters.

```rust
pub fn is_utf16_bidi(buffer: &[u16]) -> bool { /* ... */ }
```

#### Function `is_char_bidi`

**Attributes:**

- `#[inline(always)]`

Checks whether a scalar value triggers right-to-left processing.

The check is done on a Unicode block basis without regard to assigned
vs. unassigned code points in the block. Hebrew presentation forms in
the Alphabetic Presentation Forms block are treated as if they formed
a block on their own (i.e. it treated as right-to-left). Additionally,
the four RIGHT-TO-LEFT FOO controls in General Punctuation are checked
for. Control characters that are technically bidi controls but do not
cause right-to-left behavior without the presence of right-to-left
characters or right-to-left controls are not checked for. As a special
case, U+FEFF is excluded from Arabic Presentation Forms-B.

```rust
pub fn is_char_bidi(c: char) -> bool { /* ... */ }
```

#### Function `is_utf16_code_unit_bidi`

**Attributes:**

- `#[inline(always)]`

Checks whether a UTF-16 code unit triggers right-to-left processing.

The check is done on a Unicode block basis without regard to assigned
vs. unassigned code points in the block. Hebrew presentation forms in
the Alphabetic Presentation Forms block are treated as if they formed
a block on their own (i.e. it treated as right-to-left). Additionally,
the four RIGHT-TO-LEFT FOO controls in General Punctuation are checked
for. Control characters that are technically bidi controls but do not
cause right-to-left behavior without the presence of right-to-left
characters or right-to-left controls are not checked for. As a special
case, U+FEFF is excluded from Arabic Presentation Forms-B.

Since supplementary-plane right-to-left blocks are identifiable from the
high surrogate without examining the low surrogate, this function returns
`true` for such high surrogates making the function suitable for handling
supplementary-plane text without decoding surrogate pairs to scalar
values. Obviously, such high surrogates are then reported as right-to-left
even if actually unpaired.

```rust
pub fn is_utf16_code_unit_bidi(u: u16) -> bool { /* ... */ }
```

#### Function `check_utf8_for_latin1_and_bidi`

Checks whether a potentially invalid UTF-8 buffer contains code points
that trigger right-to-left processing or is all-Latin1.

Possibly more efficient than performing the checks separately.

Returns `Latin1Bidi::Latin1` if `is_utf8_latin1()` would return `true`.
Otherwise, returns `Latin1Bidi::Bidi` if `is_utf8_bidi()` would return
`true`. Otherwise, returns `Latin1Bidi::LeftToRight`.

```rust
pub fn check_utf8_for_latin1_and_bidi(buffer: &[u8]) -> Latin1Bidi { /* ... */ }
```

#### Function `check_str_for_latin1_and_bidi`

Checks whether a valid UTF-8 buffer contains code points
that trigger right-to-left processing or is all-Latin1.

Possibly more efficient than performing the checks separately.

Returns `Latin1Bidi::Latin1` if `is_str_latin1()` would return `true`.
Otherwise, returns `Latin1Bidi::Bidi` if `is_str_bidi()` would return
`true`. Otherwise, returns `Latin1Bidi::LeftToRight`.

```rust
pub fn check_str_for_latin1_and_bidi(buffer: &str) -> Latin1Bidi { /* ... */ }
```

#### Function `check_utf16_for_latin1_and_bidi`

Checks whether a potentially invalid UTF-16 buffer contains code points
that trigger right-to-left processing or is all-Latin1.

Possibly more efficient than performing the checks separately.

Returns `Latin1Bidi::Latin1` if `is_utf16_latin1()` would return `true`.
Otherwise, returns `Latin1Bidi::Bidi` if `is_utf16_bidi()` would return
`true`. Otherwise, returns `Latin1Bidi::LeftToRight`.

```rust
pub fn check_utf16_for_latin1_and_bidi(buffer: &[u16]) -> Latin1Bidi { /* ... */ }
```

#### Function `convert_utf8_to_utf16`

Converts potentially-invalid UTF-8 to valid UTF-16 with errors replaced
with the REPLACEMENT CHARACTER.

The length of the destination buffer must be at least the length of the
source buffer _plus one_.

Returns the number of `u16`s written.

# Panics

Panics if the destination buffer is shorter than stated above.

```rust
pub fn convert_utf8_to_utf16(src: &[u8], dst: &mut [u16]) -> usize { /* ... */ }
```

#### Function `convert_str_to_utf16`

Converts valid UTF-8 to valid UTF-16.

The length of the destination buffer must be at least the length of the
source buffer.

Returns the number of `u16`s written.

# Panics

Panics if the destination buffer is shorter than stated above.

```rust
pub fn convert_str_to_utf16(src: &str, dst: &mut [u16]) -> usize { /* ... */ }
```

#### Function `convert_utf8_to_utf16_without_replacement`

Converts potentially-invalid UTF-8 to valid UTF-16 signaling on error.

The length of the destination buffer must be at least the length of the
source buffer.

Returns the number of `u16`s written or `None` if the input was invalid.

When the input was invalid, some output may have been written.

# Panics

Panics if the destination buffer is shorter than stated above.

```rust
pub fn convert_utf8_to_utf16_without_replacement(src: &[u8], dst: &mut [u16]) -> Option<usize> { /* ... */ }
```

#### Function `convert_utf16_to_utf8_partial`

**Attributes:**

- `#[inline(always)]`

Converts potentially-invalid UTF-16 to valid UTF-8 with errors replaced
with the REPLACEMENT CHARACTER with potentially insufficient output
space.

Returns the number of code units read and the number of bytes written.

Guarantees that the bytes in the destination beyond the number of
bytes claimed as written by the second item of the return tuple
are left unmodified.

Not all code units are read if there isn't enough output space.

Note  that this method isn't designed for general streamability but for
not allocating memory for the worst case up front. Specifically,
if the input starts with or ends with an unpaired surrogate, those are
replaced with the REPLACEMENT CHARACTER.

Matches the semantics of `TextEncoder.encodeInto()` from the
Encoding Standard.

# Safety

If you want to convert into a `&mut str`, use
`convert_utf16_to_str_partial()` instead of using this function
together with the `unsafe` method `as_bytes_mut()` on `&mut str`.

```rust
pub fn convert_utf16_to_utf8_partial(src: &[u16], dst: &mut [u8]) -> (usize, usize) { /* ... */ }
```

#### Function `convert_utf16_to_utf8`

**Attributes:**

- `#[inline(always)]`

Converts potentially-invalid UTF-16 to valid UTF-8 with errors replaced
with the REPLACEMENT CHARACTER.

The length of the destination buffer must be at least the length of the
source buffer times three.

Returns the number of bytes written.

# Panics

Panics if the destination buffer is shorter than stated above.

# Safety

If you want to convert into a `&mut str`, use `convert_utf16_to_str()`
instead of using this function together with the `unsafe` method
`as_bytes_mut()` on `&mut str`.

```rust
pub fn convert_utf16_to_utf8(src: &[u16], dst: &mut [u8]) -> usize { /* ... */ }
```

#### Function `convert_utf16_to_str_partial`

Converts potentially-invalid UTF-16 to valid UTF-8 with errors replaced
with the REPLACEMENT CHARACTER such that the validity of the output is
signaled using the Rust type system with potentially insufficient output
space.

Returns the number of code units read and the number of bytes written.

Not all code units are read if there isn't enough output space.

Note  that this method isn't designed for general streamability but for
not allocating memory for the worst case up front. Specifically,
if the input starts with or ends with an unpaired surrogate, those are
replaced with the REPLACEMENT CHARACTER.

```rust
pub fn convert_utf16_to_str_partial(src: &[u16], dst: &mut str) -> (usize, usize) { /* ... */ }
```

#### Function `convert_utf16_to_str`

**Attributes:**

- `#[inline(always)]`

Converts potentially-invalid UTF-16 to valid UTF-8 with errors replaced
with the REPLACEMENT CHARACTER such that the validity of the output is
signaled using the Rust type system.

The length of the destination buffer must be at least the length of the
source buffer times three.

Returns the number of bytes written.

# Panics

Panics if the destination buffer is shorter than stated above.

```rust
pub fn convert_utf16_to_str(src: &[u16], dst: &mut str) -> usize { /* ... */ }
```

#### Function `convert_latin1_to_utf16`

Converts bytes whose unsigned value is interpreted as Unicode code point
(i.e. U+0000 to U+00FF, inclusive) to UTF-16.

The length of the destination buffer must be at least the length of the
source buffer.

The number of `u16`s written equals the length of the source buffer.

# Panics

Panics if the destination buffer is shorter than stated above.

```rust
pub fn convert_latin1_to_utf16(src: &[u8], dst: &mut [u16]) { /* ... */ }
```

#### Function `convert_latin1_to_utf8_partial`

Converts bytes whose unsigned value is interpreted as Unicode code point
(i.e. U+0000 to U+00FF, inclusive) to UTF-8 with potentially insufficient
output space.

Returns the number of bytes read and the number of bytes written.

If the output isn't large enough, not all input is consumed.

# Safety

If you want to convert into a `&mut str`, use
`convert_utf16_to_str_partial()` instead of using this function
together with the `unsafe` method `as_bytes_mut()` on `&mut str`.

```rust
pub fn convert_latin1_to_utf8_partial(src: &[u8], dst: &mut [u8]) -> (usize, usize) { /* ... */ }
```

#### Function `convert_latin1_to_utf8`

**Attributes:**

- `#[inline]`

Converts bytes whose unsigned value is interpreted as Unicode code point
(i.e. U+0000 to U+00FF, inclusive) to UTF-8.

The length of the destination buffer must be at least the length of the
source buffer times two.

Returns the number of bytes written.

# Panics

Panics if the destination buffer is shorter than stated above.

# Safety

Note that this function may write garbage beyond the number of bytes
indicated by the return value, so using a `&mut str` interpreted as
`&mut [u8]` as the destination is not safe. If you want to convert into
a `&mut str`, use `convert_utf16_to_str()` instead of this function.

```rust
pub fn convert_latin1_to_utf8(src: &[u8], dst: &mut [u8]) -> usize { /* ... */ }
```

#### Function `convert_latin1_to_str_partial`

**Attributes:**

- `#[inline]`

Converts bytes whose unsigned value is interpreted as Unicode code point
(i.e. U+0000 to U+00FF, inclusive) to UTF-8 such that the validity of the
output is signaled using the Rust type system with potentially insufficient
output space.

Returns the number of bytes read and the number of bytes written.

If the output isn't large enough, not all input is consumed.

```rust
pub fn convert_latin1_to_str_partial(src: &[u8], dst: &mut str) -> (usize, usize) { /* ... */ }
```

#### Function `convert_latin1_to_str`

**Attributes:**

- `#[inline]`

Converts bytes whose unsigned value is interpreted as Unicode code point
(i.e. U+0000 to U+00FF, inclusive) to UTF-8 such that the validity of the
output is signaled using the Rust type system.

The length of the destination buffer must be at least the length of the
source buffer times two.

Returns the number of bytes written.

# Panics

Panics if the destination buffer is shorter than stated above.

```rust
pub fn convert_latin1_to_str(src: &[u8], dst: &mut str) -> usize { /* ... */ }
```

#### Function `convert_utf8_to_latin1_lossy`

If the input is valid UTF-8 representing only Unicode code points from
U+0000 to U+00FF, inclusive, converts the input into output that
represents the value of each code point as the unsigned byte value of
each output byte.

If the input does not fulfill the condition stated above, this function
panics if debug assertions are enabled (and fuzzing isn't) and otherwise
does something that is memory-safe without any promises about any
properties of the output. In particular, callers shouldn't assume the
output to be the same across crate versions or CPU architectures and
should not assume that non-ASCII input can't map to ASCII output.

The length of the destination buffer must be at least the length of the
source buffer.

Returns the number of bytes written.

# Panics

Panics if the destination buffer is shorter than stated above.

If debug assertions are enabled (and not fuzzing) and the input is
not in the range U+0000 to U+00FF, inclusive.

```rust
pub fn convert_utf8_to_latin1_lossy(src: &[u8], dst: &mut [u8]) -> usize { /* ... */ }
```

#### Function `convert_utf16_to_latin1_lossy`

If the input is valid UTF-16 representing only Unicode code points from
U+0000 to U+00FF, inclusive, converts the input into output that
represents the value of each code point as the unsigned byte value of
each output byte.

If the input does not fulfill the condition stated above, does something
that is memory-safe without any promises about any properties of the
output and will probably assert in debug builds in future versions.
In particular, callers shouldn't assume the output to be the same across
crate versions or CPU architectures and should not assume that non-ASCII
input can't map to ASCII output.

The length of the destination buffer must be at least the length of the
source buffer.

The number of bytes written equals the length of the source buffer.

# Panics

Panics if the destination buffer is shorter than stated above.

(Probably in future versions if debug assertions are enabled (and not
fuzzing) and the input is not in the range U+0000 to U+00FF, inclusive.)

```rust
pub fn convert_utf16_to_latin1_lossy(src: &[u16], dst: &mut [u8]) { /* ... */ }
```

#### Function `decode_latin1`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

Converts bytes whose unsigned value is interpreted as Unicode code point
(i.e. U+0000 to U+00FF, inclusive) to UTF-8.

Borrows if input is ASCII-only. Performs a single heap allocation
otherwise.

Only available if the `alloc` feature is enabled (enabled by default).

```rust
pub fn decode_latin1<''a>(bytes: &''a [u8]) -> alloc::borrow::Cow<''a, str> { /* ... */ }
```

#### Function `encode_latin1_lossy`

**Attributes:**

- `#[<cfg>(feature = "alloc")]`

If the input is valid UTF-8 representing only Unicode code points from
U+0000 to U+00FF, inclusive, converts the input into output that
represents the value of each code point as the unsigned byte value of
each output byte.

If the input does not fulfill the condition stated above, this function
panics if debug assertions are enabled (and fuzzing isn't) and otherwise
does something that is memory-safe without any promises about any
properties of the output. In particular, callers shouldn't assume the
output to be the same across crate versions or CPU architectures and
should not assume that non-ASCII input can't map to ASCII output.

Borrows if input is ASCII-only. Performs a single heap allocation
otherwise.

Only available if the `alloc` feature is enabled (enabled by default).

```rust
pub fn encode_latin1_lossy<''a>(string: &''a str) -> alloc::borrow::Cow<''a, [u8]> { /* ... */ }
```

#### Function `utf16_valid_up_to`

Returns the index of the first unpaired surrogate or, if the input is
valid UTF-16 in its entirety, the length of the input.

```rust
pub fn utf16_valid_up_to(buffer: &[u16]) -> usize { /* ... */ }
```

#### Function `utf8_latin1_up_to`

Returns the index of first byte that starts an invalid byte
sequence or a non-Latin1 byte sequence, or the length of the
string if there are neither.

```rust
pub fn utf8_latin1_up_to(buffer: &[u8]) -> usize { /* ... */ }
```

#### Function `str_latin1_up_to`

Returns the index of first byte that starts a non-Latin1 byte
sequence, or the length of the string if there are none.

```rust
pub fn str_latin1_up_to(buffer: &str) -> usize { /* ... */ }
```

#### Function `ensure_utf16_validity`

**Attributes:**

- `#[inline]`

Replaces unpaired surrogates in the input with the REPLACEMENT CHARACTER.

```rust
pub fn ensure_utf16_validity(buffer: &mut [u16]) { /* ... */ }
```

#### Function `copy_ascii_to_ascii`

Copies ASCII from source to destination up to the first non-ASCII byte
(or the end of the input if it is ASCII in its entirety).

The length of the destination buffer must be at least the length of the
source buffer.

Returns the number of bytes written.

# Panics

Panics if the destination buffer is shorter than stated above.

```rust
pub fn copy_ascii_to_ascii(src: &[u8], dst: &mut [u8]) -> usize { /* ... */ }
```

#### Function `copy_ascii_to_basic_latin`

Copies ASCII from source to destination zero-extending it to UTF-16 up to
the first non-ASCII byte (or the end of the input if it is ASCII in its
entirety).

The length of the destination buffer must be at least the length of the
source buffer.

Returns the number of `u16`s written.

# Panics

Panics if the destination buffer is shorter than stated above.

```rust
pub fn copy_ascii_to_basic_latin(src: &[u8], dst: &mut [u16]) -> usize { /* ... */ }
```

#### Function `copy_basic_latin_to_ascii`

Copies Basic Latin from source to destination narrowing it to ASCII up to
the first non-Basic Latin code unit (or the end of the input if it is
Basic Latin in its entirety).

The length of the destination buffer must be at least the length of the
source buffer.

Returns the number of bytes written.

# Panics

Panics if the destination buffer is shorter than stated above.

```rust
pub fn copy_basic_latin_to_ascii(src: &[u16], dst: &mut [u8]) -> usize { /* ... */ }
```

## Types

### Struct `Encoding`

An encoding as defined in the [Encoding Standard][1].

An _encoding_ defines a mapping from a `u8` sequence to a `char` sequence
and, in most cases, vice versa. Each encoding has a name, an output
encoding, and one or more labels.

_Labels_ are ASCII-case-insensitive strings that are used to identify an
encoding in formats and protocols. The _name_ of the encoding is the
preferred label in the case appropriate for returning from the
[`characterSet`][2] property of the `Document` DOM interface.

The _output encoding_ is the encoding used for form submission and URL
parsing on Web pages in the encoding. This is UTF-8 for the replacement,
UTF-16LE and UTF-16BE encodings and the encoding itself for other
encodings.

[1]: https://encoding.spec.whatwg.org/
[2]: https://dom.spec.whatwg.org/#dom-document-characterset

# Streaming vs. Non-Streaming

When you have the entire input in a single buffer, you can use the
methods [`decode()`][3], [`decode_with_bom_removal()`][3],
[`decode_without_bom_handling()`][5],
[`decode_without_bom_handling_and_without_replacement()`][6] and
[`encode()`][7]. (These methods are available to Rust callers only and are
not available in the C API.) Unlike the rest of the API available to Rust,
these methods perform heap allocations. You should the `Decoder` and
`Encoder` objects when your input is split into multiple buffers or when
you want to control the allocation of the output buffers.

[3]: #method.decode
[4]: #method.decode_with_bom_removal
[5]: #method.decode_without_bom_handling
[6]: #method.decode_without_bom_handling_and_without_replacement
[7]: #method.encode

# Instances

All instances of `Encoding` are statically allocated and have the `'static`
lifetime. There is precisely one unique `Encoding` instance for each
encoding defined in the Encoding Standard.

To obtain a reference to a particular encoding whose identity you know at
compile time, use a `static` that refers to encoding. There is a `static`
for each encoding. The `static`s are named in all caps with hyphens
replaced with underscores (and in C/C++ have `_ENCODING` appended to the
name). For example, if you know at compile time that you will want to
decode using the UTF-8 encoding, use the `UTF_8` `static` (`UTF_8_ENCODING`
in C/C++).

Additionally, there are non-reference-typed forms ending with `_INIT` to
work around the problem that `static`s of the type `&'static Encoding`
cannot be used to initialize items of an array whose type is
`[&'static Encoding; N]`.

If you don't know what encoding you need at compile time and need to
dynamically get an encoding by label, use
<code>Encoding::<a href="#method.for_label">for_label</a>(<var>label</var>)</code>.

Instances of `Encoding` can be compared with `==` (in both Rust and in
C/C++).

```rust
pub struct Encoding {
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
  pub fn for_label(label: &[u8]) -> Option<&''static Encoding> { /* ... */ }
  ```
  Implements the

- ```rust
  pub fn for_label_no_replacement(label: &[u8]) -> Option<&''static Encoding> { /* ... */ }
  ```
  This method behaves the same as `for_label()`, except when `for_label()`

- ```rust
  pub fn for_bom(buffer: &[u8]) -> Option<(&''static Encoding, usize)> { /* ... */ }
  ```
  Performs non-incremental BOM sniffing.

- ```rust
  pub fn name(self: &''static Self) -> &''static str { /* ... */ }
  ```
  Returns the name of this encoding.

- ```rust
  pub fn can_encode_everything(self: &''static Self) -> bool { /* ... */ }
  ```
  Checks whether the _output encoding_ of this encoding can encode every

- ```rust
  pub fn is_ascii_compatible(self: &''static Self) -> bool { /* ... */ }
  ```
  Checks whether the bytes 0x00...0x7F map exclusively to the characters

- ```rust
  pub fn is_single_byte(self: &''static Self) -> bool { /* ... */ }
  ```
  Checks whether this encoding maps one byte to one Basic Multilingual

- ```rust
  pub fn output_encoding(self: &''static Self) -> &''static Encoding { /* ... */ }
  ```
  Returns the _output encoding_ of this encoding. This is UTF-8 for

- ```rust
  pub fn decode<''a>(self: &''static Self, bytes: &''a [u8]) -> (Cow<''a, str>, &''static Encoding, bool) { /* ... */ }
  ```
  Decode complete input to `Cow<'a, str>` _with BOM sniffing_ and with

- ```rust
  pub fn decode_with_bom_removal<''a>(self: &''static Self, bytes: &''a [u8]) -> (Cow<''a, str>, bool) { /* ... */ }
  ```
  Decode complete input to `Cow<'a, str>` _with BOM removal_ and with

- ```rust
  pub fn decode_without_bom_handling<''a>(self: &''static Self, bytes: &''a [u8]) -> (Cow<''a, str>, bool) { /* ... */ }
  ```
  Decode complete input to `Cow<'a, str>` _without BOM handling_ and

- ```rust
  pub fn decode_without_bom_handling_and_without_replacement<''a>(self: &''static Self, bytes: &''a [u8]) -> Option<Cow<''a, str>> { /* ... */ }
  ```
  Decode complete input to `Cow<'a, str>` _without BOM handling_ and

- ```rust
  pub fn encode<''a>(self: &''static Self, string: &''a str) -> (Cow<''a, [u8]>, &''static Encoding, bool) { /* ... */ }
  ```
  Encode complete input to `Cow<'a, [u8]>` using the

- ```rust
  pub fn new_decoder(self: &''static Self) -> Decoder { /* ... */ }
  ```
  Instantiates a new decoder for this encoding with BOM sniffing enabled.

- ```rust
  pub fn new_decoder_with_bom_removal(self: &''static Self) -> Decoder { /* ... */ }
  ```
  Instantiates a new decoder for this encoding with BOM removal.

- ```rust
  pub fn new_decoder_without_bom_handling(self: &''static Self) -> Decoder { /* ... */ }
  ```
  Instantiates a new decoder for this encoding with BOM handling disabled.

- ```rust
  pub fn new_encoder(self: &''static Self) -> Encoder { /* ... */ }
  ```
  Instantiates a new encoder for the [_output encoding_](Encoding::output_encoding)

- ```rust
  pub fn utf8_valid_up_to(bytes: &[u8]) -> usize { /* ... */ }
  ```
  Validates UTF-8.

- ```rust
  pub fn ascii_valid_up_to(bytes: &[u8]) -> usize { /* ... */ }
  ```
  Validates ASCII.

- ```rust
  pub fn iso_2022_jp_ascii_valid_up_to(bytes: &[u8]) -> usize { /* ... */ }
  ```
  Validates ISO-2022-JP ASCII-state data.

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
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

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Encoding) -> bool { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **RefUnwindSafe**
### Enum `CoderResult`

**Attributes:**

- `#[must_use]`

Result of a (potentially partial) decode or encode operation with
replacement.

```rust
pub enum CoderResult {
    InputEmpty,
    OutputFull,
}
```

#### Variants

##### `InputEmpty`

The input was exhausted.

If this result was returned from a call where `last` was `true`, the
conversion process has completed. Otherwise, the caller should call a
decode or encode method again with more input.

##### `OutputFull`

The converter cannot produce another unit of output, because the output
buffer does not have enough space left.

The caller must provide more output space upon the next call and re-push
the remaining input to the converter.

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CoderResult) -> bool { /* ... */ }
    ```

- **Eq**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Enum `DecoderResult`

**Attributes:**

- `#[must_use]`

Result of a (potentially partial) decode operation without replacement.

```rust
pub enum DecoderResult {
    InputEmpty,
    OutputFull,
    Malformed(u8, u8),
}
```

#### Variants

##### `InputEmpty`

The input was exhausted.

If this result was returned from a call where `last` was `true`, the
decoding process has completed. Otherwise, the caller should call a
decode method again with more input.

##### `OutputFull`

The decoder cannot produce another unit of output, because the output
buffer does not have enough space left.

The caller must provide more output space upon the next call and re-push
the remaining input to the decoder.

##### `Malformed`

The decoder encountered a malformed byte sequence.

The caller must either treat this as a fatal error or must append one
REPLACEMENT CHARACTER (U+FFFD) to the output and then re-push the
the remaining input to the decoder.

The first wrapped integer indicates the length of the malformed byte
sequence. The second wrapped integer indicates the number of bytes
that were consumed after the malformed sequence. If the second
integer is zero, the last byte that was consumed is the last byte of
the malformed sequence. Note that the malformed bytes may have been part
of an earlier input buffer.

The first wrapped integer can have values 1, 2, 3 or 4. The second
wrapped integer can have values 0, 1, 2 or 3. The worst-case sum
of the two is 6, which happens with ISO-2022-JP.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |
| 1 | `u8` |  |

#### Implementations

##### Trait Implementations

- **UnwindSafe**
- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DecoderResult) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
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

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Struct `Decoder`

A converter that decodes a byte stream into Unicode according to a
character encoding in a streaming (incremental) manner.

The various `decode_*` methods take an input buffer (`src`) and an output
buffer `dst` both of which are caller-allocated. There are variants for
both UTF-8 and UTF-16 output buffers.

A `decode_*` method decodes bytes from `src` into Unicode characters stored
into `dst` until one of the following three things happens:

1. A malformed byte sequence is encountered (`*_without_replacement`
   variants only).

2. The output buffer has been filled so near capacity that the decoder
   cannot be sure that processing an additional byte of input wouldn't
   cause so much output that the output buffer would overflow.

3. All the input bytes have been processed.

The `decode_*` method then returns tuple of a status indicating which one
of the three reasons to return happened, how many input bytes were read,
how many output code units (`u8` when decoding into UTF-8 and `u16`
when decoding to UTF-16) were written (except when decoding into `String`,
whose length change indicates this), and in the case of the
variants performing replacement, a boolean indicating whether an error was
replaced with the REPLACEMENT CHARACTER during the call.

The number of bytes "written" is what's logically written. Garbage may be
written in the output buffer beyond the point logically written to.
Therefore, if you wish to decode into an `&mut str`, you should use the
methods that take an `&mut str` argument instead of the ones that take an
`&mut [u8]` argument. The former take care of overwriting the trailing
garbage to ensure the UTF-8 validity of the `&mut str` as a whole, but the
latter don't.

In the case of the `*_without_replacement` variants, the status is a
[`DecoderResult`][1] enumeration (possibilities `Malformed`, `OutputFull` and
`InputEmpty` corresponding to the three cases listed above).

In the case of methods whose name does not end with
`*_without_replacement`, malformed sequences are automatically replaced
with the REPLACEMENT CHARACTER and errors do not cause the methods to
return early.

When decoding to UTF-8, the output buffer must have at least 4 bytes of
space. When decoding to UTF-16, the output buffer must have at least two
UTF-16 code units (`u16`) of space.

When decoding to UTF-8 without replacement, the methods are guaranteed
not to return indicating that more output space is needed if the length
of the output buffer is at least the length returned by
[`max_utf8_buffer_length_without_replacement()`][2]. When decoding to UTF-8
with replacement, the length of the output buffer that guarantees the
methods not to return indicating that more output space is needed is given
by [`max_utf8_buffer_length()`][3]. When decoding to UTF-16 with
or without replacement, the length of the output buffer that guarantees
the methods not to return indicating that more output space is needed is
given by [`max_utf16_buffer_length()`][4].

The output written into `dst` is guaranteed to be valid UTF-8 or UTF-16,
and the output after each `decode_*` call is guaranteed to consist of
complete characters. (I.e. the code unit sequence for the last character is
guaranteed not to be split across output buffers.)

The boolean argument `last` indicates that the end of the stream is reached
when all the bytes in `src` have been consumed.

A `Decoder` object can be used to incrementally decode a byte stream.

During the processing of a single stream, the caller must call `decode_*`
zero or more times with `last` set to `false` and then call `decode_*` at
least once with `last` set to `true`. If `decode_*` returns `InputEmpty`,
the processing of the stream has ended. Otherwise, the caller must call
`decode_*` again with `last` set to `true` (or treat a `Malformed` result as
 a fatal error).

Once the stream has ended, the `Decoder` object must not be used anymore.
That is, you need to create another one to process another stream.

When the decoder returns `OutputFull` or the decoder returns `Malformed` and
the caller does not wish to treat it as a fatal error, the input buffer
`src` may not have been completely consumed. In that case, the caller must
pass the unconsumed contents of `src` to `decode_*` again upon the next
call.

[1]: enum.DecoderResult.html
[2]: #method.max_utf8_buffer_length_without_replacement
[3]: #method.max_utf8_buffer_length
[4]: #method.max_utf16_buffer_length

# Infinite loops

When converting with a fixed-size output buffer whose size is too small to
accommodate one character or (when applicable) one numeric character
reference of output, an infinite loop ensues. When converting with a
fixed-size output buffer, it generally makes sense to make the buffer
fairly large (e.g. couple of kilobytes).

```rust
pub struct Decoder {
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
  pub fn encoding(self: &Self) -> &''static Encoding { /* ... */ }
  ```
  The `Encoding` this `Decoder` is for.

- ```rust
  pub fn max_utf8_buffer_length(self: &Self, byte_length: usize) -> Option<usize> { /* ... */ }
  ```
  Query the worst-case UTF-8 output size _with replacement_.

- ```rust
  pub fn max_utf8_buffer_length_without_replacement(self: &Self, byte_length: usize) -> Option<usize> { /* ... */ }
  ```
  Query the worst-case UTF-8 output size _without replacement_.

- ```rust
  pub fn decode_to_utf8(self: &mut Self, src: &[u8], dst: &mut [u8], last: bool) -> (CoderResult, usize, usize, bool) { /* ... */ }
  ```
  Incrementally decode a byte stream into UTF-8 with malformed sequences

- ```rust
  pub fn decode_to_str(self: &mut Self, src: &[u8], dst: &mut str, last: bool) -> (CoderResult, usize, usize, bool) { /* ... */ }
  ```
  Incrementally decode a byte stream into UTF-8 with malformed sequences

- ```rust
  pub fn decode_to_string(self: &mut Self, src: &[u8], dst: &mut String, last: bool) -> (CoderResult, usize, bool) { /* ... */ }
  ```
  Incrementally decode a byte stream into UTF-8 with malformed sequences

- ```rust
  pub fn decode_to_utf8_without_replacement(self: &mut Self, src: &[u8], dst: &mut [u8], last: bool) -> (DecoderResult, usize, usize) { /* ... */ }
  ```
  Incrementally decode a byte stream into UTF-8

- ```rust
  pub fn decode_to_str_without_replacement(self: &mut Self, src: &[u8], dst: &mut str, last: bool) -> (DecoderResult, usize, usize) { /* ... */ }
  ```
  Incrementally decode a byte stream into UTF-8 with type system signaling

- ```rust
  pub fn decode_to_string_without_replacement(self: &mut Self, src: &[u8], dst: &mut String, last: bool) -> (DecoderResult, usize) { /* ... */ }
  ```
  Incrementally decode a byte stream into UTF-8 using a `String` receiver.

- ```rust
  pub fn max_utf16_buffer_length(self: &Self, byte_length: usize) -> Option<usize> { /* ... */ }
  ```
  Query the worst-case UTF-16 output size (with or without replacement).

- ```rust
  pub fn decode_to_utf16(self: &mut Self, src: &[u8], dst: &mut [u16], last: bool) -> (CoderResult, usize, usize, bool) { /* ... */ }
  ```
  Incrementally decode a byte stream into UTF-16 with malformed sequences

- ```rust
  pub fn decode_to_utf16_without_replacement(self: &mut Self, src: &[u8], dst: &mut [u16], last: bool) -> (DecoderResult, usize, usize) { /* ... */ }
  ```
  Incrementally decode a byte stream into UTF-16

- ```rust
  pub fn latin1_byte_compatible_up_to(self: &Self, bytes: &[u8]) -> Option<usize> { /* ... */ }
  ```
  Checks for compatibility with storing Unicode scalar values as unsigned

##### Trait Implementations

- **UnwindSafe**
- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Enum `EncoderResult`

**Attributes:**

- `#[must_use]`

Result of a (potentially partial) encode operation without replacement.

```rust
pub enum EncoderResult {
    InputEmpty,
    OutputFull,
    Unmappable(char),
}
```

#### Variants

##### `InputEmpty`

The input was exhausted.

If this result was returned from a call where `last` was `true`, the
decoding process has completed. Otherwise, the caller should call a
decode method again with more input.

##### `OutputFull`

The encoder cannot produce another unit of output, because the output
buffer does not have enough space left.

The caller must provide more output space upon the next call and re-push
the remaining input to the decoder.

##### `Unmappable`

The encoder encountered an unmappable character.

The caller must either treat this as a fatal error or must append
a placeholder to the output and then re-push the remaining input to the
encoder.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `char` |  |

#### Implementations

##### Trait Implementations

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &EncoderResult) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
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

- **Sync**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
### Struct `Encoder`

A converter that encodes a Unicode stream into bytes according to a
character encoding in a streaming (incremental) manner.

The various `encode_*` methods take an input buffer (`src`) and an output
buffer `dst` both of which are caller-allocated. There are variants for
both UTF-8 and UTF-16 input buffers.

An `encode_*` method encode characters from `src` into bytes characters
stored into `dst` until one of the following three things happens:

1. An unmappable character is encountered (`*_without_replacement` variants
   only).

2. The output buffer has been filled so near capacity that the decoder
   cannot be sure that processing an additional character of input wouldn't
   cause so much output that the output buffer would overflow.

3. All the input characters have been processed.

The `encode_*` method then returns tuple of a status indicating which one
of the three reasons to return happened, how many input code units (`u8`
when encoding from UTF-8 and `u16` when encoding from UTF-16) were read,
how many output bytes were written (except when encoding into `Vec<u8>`,
whose length change indicates this), and in the case of the variants that
perform replacement, a boolean indicating whether an unmappable
character was replaced with a numeric character reference during the call.

The number of bytes "written" is what's logically written. Garbage may be
written in the output buffer beyond the point logically written to.

In the case of the methods whose name ends with
`*_without_replacement`, the status is an [`EncoderResult`][1] enumeration
(possibilities `Unmappable`, `OutputFull` and `InputEmpty` corresponding to
the three cases listed above).

In the case of methods whose name does not end with
`*_without_replacement`, unmappable characters are automatically replaced
with the corresponding numeric character references and unmappable
characters do not cause the methods to return early.

When encoding from UTF-8 without replacement, the methods are guaranteed
not to return indicating that more output space is needed if the length
of the output buffer is at least the length returned by
[`max_buffer_length_from_utf8_without_replacement()`][2]. When encoding from
UTF-8 with replacement, the length of the output buffer that guarantees the
methods not to return indicating that more output space is needed in the
absence of unmappable characters is given by
[`max_buffer_length_from_utf8_if_no_unmappables()`][3]. When encoding from
UTF-16 without replacement, the methods are guaranteed not to return
indicating that more output space is needed if the length of the output
buffer is at least the length returned by
[`max_buffer_length_from_utf16_without_replacement()`][4]. When encoding
from UTF-16 with replacement, the the length of the output buffer that
guarantees the methods not to return indicating that more output space is
needed in the absence of unmappable characters is given by
[`max_buffer_length_from_utf16_if_no_unmappables()`][5].
When encoding with replacement, applications are not expected to size the
buffer for the worst case ahead of time but to resize the buffer if there
are unmappable characters. This is why max length queries are only available
for the case where there are no unmappable characters.

When encoding from UTF-8, each `src` buffer _must_ be valid UTF-8. (When
calling from Rust, the type system takes care of this.) When encoding from
UTF-16, unpaired surrogates in the input are treated as U+FFFD REPLACEMENT
CHARACTERS. Therefore, in order for astral characters not to turn into a
pair of REPLACEMENT CHARACTERS, the caller must ensure that surrogate pairs
are not split across input buffer boundaries.

After an `encode_*` call returns, the output produced so far, taken as a
whole from the start of the stream, is guaranteed to consist of a valid
byte sequence in the target encoding. (I.e. the code unit sequence for a
character is guaranteed not to be split across output buffers. However, due
to the stateful nature of ISO-2022-JP, the stream needs to be considered
from the start for it to be valid. For other encodings, the validity holds
on a per-output buffer basis.)

The boolean argument `last` indicates that the end of the stream is reached
when all the characters in `src` have been consumed. This argument is needed
for ISO-2022-JP and is ignored for other encodings.

An `Encoder` object can be used to incrementally encode a byte stream.

During the processing of a single stream, the caller must call `encode_*`
zero or more times with `last` set to `false` and then call `encode_*` at
least once with `last` set to `true`. If `encode_*` returns `InputEmpty`,
the processing of the stream has ended. Otherwise, the caller must call
`encode_*` again with `last` set to `true` (or treat an `Unmappable` result
as a fatal error).

Once the stream has ended, the `Encoder` object must not be used anymore.
That is, you need to create another one to process another stream.

When the encoder returns `OutputFull` or the encoder returns `Unmappable`
and the caller does not wish to treat it as a fatal error, the input buffer
`src` may not have been completely consumed. In that case, the caller must
pass the unconsumed contents of `src` to `encode_*` again upon the next
call.

[1]: enum.EncoderResult.html
[2]: #method.max_buffer_length_from_utf8_without_replacement
[3]: #method.max_buffer_length_from_utf8_if_no_unmappables
[4]: #method.max_buffer_length_from_utf16_without_replacement
[5]: #method.max_buffer_length_from_utf16_if_no_unmappables

# Infinite loops

When converting with a fixed-size output buffer whose size is too small to
accommodate one character of output, an infinite loop ensues. When
converting with a fixed-size output buffer, it generally makes sense to
make the buffer fairly large (e.g. couple of kilobytes).

```rust
pub struct Encoder {
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
  pub fn encoding(self: &Self) -> &''static Encoding { /* ... */ }
  ```
  The `Encoding` this `Encoder` is for.

- ```rust
  pub fn has_pending_state(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if this is an ISO-2022-JP encoder that's not in the

- ```rust
  pub fn max_buffer_length_from_utf8_if_no_unmappables(self: &Self, byte_length: usize) -> Option<usize> { /* ... */ }
  ```
  Query the worst-case output size when encoding from UTF-8 with

- ```rust
  pub fn max_buffer_length_from_utf8_without_replacement(self: &Self, byte_length: usize) -> Option<usize> { /* ... */ }
  ```
  Query the worst-case output size when encoding from UTF-8 without

- ```rust
  pub fn encode_from_utf8(self: &mut Self, src: &str, dst: &mut [u8], last: bool) -> (CoderResult, usize, usize, bool) { /* ... */ }
  ```
  Incrementally encode into byte stream from UTF-8 with unmappable

- ```rust
  pub fn encode_from_utf8_to_vec(self: &mut Self, src: &str, dst: &mut Vec<u8>, last: bool) -> (CoderResult, usize, bool) { /* ... */ }
  ```
  Incrementally encode into byte stream from UTF-8 with unmappable

- ```rust
  pub fn encode_from_utf8_without_replacement(self: &mut Self, src: &str, dst: &mut [u8], last: bool) -> (EncoderResult, usize, usize) { /* ... */ }
  ```
  Incrementally encode into byte stream from UTF-8 _without replacement_.

- ```rust
  pub fn encode_from_utf8_to_vec_without_replacement(self: &mut Self, src: &str, dst: &mut Vec<u8>, last: bool) -> (EncoderResult, usize) { /* ... */ }
  ```
  Incrementally encode into byte stream from UTF-8 _without replacement_.

- ```rust
  pub fn max_buffer_length_from_utf16_if_no_unmappables(self: &Self, u16_length: usize) -> Option<usize> { /* ... */ }
  ```
  Query the worst-case output size when encoding from UTF-16 with

- ```rust
  pub fn max_buffer_length_from_utf16_without_replacement(self: &Self, u16_length: usize) -> Option<usize> { /* ... */ }
  ```
  Query the worst-case output size when encoding from UTF-16 without

- ```rust
  pub fn encode_from_utf16(self: &mut Self, src: &[u16], dst: &mut [u8], last: bool) -> (CoderResult, usize, usize, bool) { /* ... */ }
  ```
  Incrementally encode into byte stream from UTF-16 with unmappable

- ```rust
  pub fn encode_from_utf16_without_replacement(self: &mut Self, src: &[u16], dst: &mut [u8], last: bool) -> (EncoderResult, usize, usize) { /* ... */ }
  ```
  Incrementally encode into byte stream from UTF-16 _without replacement_.

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
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
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Constants and Statics

### Static `BIG5_INIT`

The initializer for the [Big5](static.BIG5.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static BIG5_INIT: Encoding = _;
```

### Static `BIG5`

The Big5 encoding.

This is Big5 with HKSCS with mappings to more recent Unicode assignments
instead of the Private Use Area code points that have been used historically.
It is believed to be able to decode existing Web content in a way that makes
sense.

To avoid form submissions generating data that Web servers don't understand,
the encoder doesn't use the HKSCS byte sequences that precede the unextended
Big5 in the lexical order.

[Index visualization](https://encoding.spec.whatwg.org/big5.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/big5-bmp.html)

This encoding is designed to be suited for decoding the Windows code page 950
and its HKSCS patched "951" variant such that the text makes sense, given
assignments that Unicode has made after those encodings used Private Use
Area characters.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static BIG5: &''static Encoding = _;
```

### Static `EUC_JP_INIT`

The initializer for the [EUC-JP](static.EUC_JP.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static EUC_JP_INIT: Encoding = _;
```

### Static `EUC_JP`

The EUC-JP encoding.

This is the legacy Unix encoding for Japanese.

For compatibility with Web servers that don't expect three-byte sequences
in form submissions, the encoder doesn't generate three-byte sequences.
That is, the JIS X 0212 support is decode-only.

[Index visualization](https://encoding.spec.whatwg.org/euc-jp.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/euc-jp-bmp.html)

This encoding roughly matches the Windows code page 20932. There are error
handling differences and a handful of 2-byte sequences that decode differently.
Additionall, Windows doesn't support 3-byte sequences.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static EUC_JP: &''static Encoding = _;
```

### Static `EUC_KR_INIT`

The initializer for the [EUC-KR](static.EUC_KR.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static EUC_KR_INIT: Encoding = _;
```

### Static `EUC_KR`

The EUC-KR encoding.

This is the Korean encoding for Windows. It extends the Unix legacy encoding
for Korean, based on KS X 1001 (which also formed the base of MacKorean on Mac OS
Classic), with all the characters from the Hangul Syllables block of Unicode.

[Index visualization](https://encoding.spec.whatwg.org/euc-kr.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/euc-kr-bmp.html)

This encoding matches the Windows code page 949, except Windows decodes byte 0x80
to U+0080 and some byte sequences that are error per the Encoding Standard to
the question mark or the Private Use Area.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static EUC_KR: &''static Encoding = _;
```

### Static `GBK_INIT`

The initializer for the [GBK](static.GBK.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static GBK_INIT: Encoding = _;
```

### Static `GBK`

The GBK encoding.

The decoder for this encoding is the same as the decoder for gb18030.
The encoder side of this encoding is GBK with Windows code page 936 euro
sign behavior and with the changes to two-byte sequences made in GB18030-2022.
GBK extends GB2312-80 to cover the CJK Unified Ideographs Unicode block as
well as a handful of ideographs from the CJK Unified Ideographs Extension A
and CJK Compatibility Ideographs blocks.

Unlike e.g. in the case of ISO-8859-1 and windows-1252, GBK encoder wasn't
unified with the gb18030 encoder in the Encoding Standard out of concern
that servers that expect GBK form submissions might not be able to handle
the four-byte sequences.

[Index visualization for the two-byte sequences](https://encoding.spec.whatwg.org/gb18030.html),
[Visualization of BMP coverage of the two-byte index](https://encoding.spec.whatwg.org/gb18030-bmp.html)

The encoder of this encoding roughly matches the Windows code page 936.
The decoder side is a superset.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static GBK: &''static Encoding = _;
```

### Static `IBM866_INIT`

The initializer for the [IBM866](static.IBM866.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static IBM866_INIT: Encoding = _;
```

### Static `IBM866`

The IBM866 encoding.

This the most notable one of the DOS Cyrillic code pages. It has the same
box drawing characters as code page 437, so it can be used for decoding
DOS-era ASCII + box drawing data.

[Index visualization](https://encoding.spec.whatwg.org/ibm866.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/ibm866-bmp.html)

This encoding matches the Windows code page 866.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static IBM866: &''static Encoding = _;
```

### Static `ISO_2022_JP_INIT`

The initializer for the [ISO-2022-JP](static.ISO_2022_JP.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static ISO_2022_JP_INIT: Encoding = _;
```

### Static `ISO_2022_JP`

The ISO-2022-JP encoding.

This the primary pre-UTF-8 encoding for Japanese email. It uses the ASCII
byte range to encode non-Basic Latin characters. It's the only encoding
supported by this crate whose encoder is stateful.

[Index visualization](https://encoding.spec.whatwg.org/jis0208.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/jis0208-bmp.html)

This encoding roughly matches the Windows code page 50220. Notably, Windows
uses U+30FB in place of the REPLACEMENT CHARACTER and otherwise differs in
error handling.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static ISO_2022_JP: &''static Encoding = _;
```

### Static `ISO_8859_10_INIT`

The initializer for the [ISO-8859-10](static.ISO_8859_10.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static ISO_8859_10_INIT: Encoding = _;
```

### Static `ISO_8859_10`

The ISO-8859-10 encoding.

This is the Nordic part of the ISO/IEC 8859 encoding family. This encoding
is also known as Latin 6.

[Index visualization](https://encoding.spec.whatwg.org/iso-8859-10.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-10-bmp.html)

The Windows code page number for this encoding is 28600, but kernel32.dll
does not support this encoding.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static ISO_8859_10: &''static Encoding = _;
```

### Static `ISO_8859_13_INIT`

The initializer for the [ISO-8859-13](static.ISO_8859_13.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static ISO_8859_13_INIT: Encoding = _;
```

### Static `ISO_8859_13`

The ISO-8859-13 encoding.

This is the Baltic part of the ISO/IEC 8859 encoding family. This encoding
is also known as Latin 7.

[Index visualization](https://encoding.spec.whatwg.org/iso-8859-13.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-13-bmp.html)

This encoding matches the Windows code page 28603, except Windows decodes
unassigned code points to the Private Use Area of Unicode.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static ISO_8859_13: &''static Encoding = _;
```

### Static `ISO_8859_14_INIT`

The initializer for the [ISO-8859-14](static.ISO_8859_14.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static ISO_8859_14_INIT: Encoding = _;
```

### Static `ISO_8859_14`

The ISO-8859-14 encoding.

This is the Celtic part of the ISO/IEC 8859 encoding family. This encoding
is also known as Latin 8.

[Index visualization](https://encoding.spec.whatwg.org/iso-8859-14.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-14-bmp.html)

The Windows code page number for this encoding is 28604, but kernel32.dll
does not support this encoding.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static ISO_8859_14: &''static Encoding = _;
```

### Static `ISO_8859_15_INIT`

The initializer for the [ISO-8859-15](static.ISO_8859_15.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static ISO_8859_15_INIT: Encoding = _;
```

### Static `ISO_8859_15`

The ISO-8859-15 encoding.

This is the revised Western European part of the ISO/IEC 8859 encoding
family. This encoding is also known as Latin 9.

[Index visualization](https://encoding.spec.whatwg.org/iso-8859-15.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-15-bmp.html)

This encoding matches the Windows code page 28605.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static ISO_8859_15: &''static Encoding = _;
```

### Static `ISO_8859_16_INIT`

The initializer for the [ISO-8859-16](static.ISO_8859_16.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static ISO_8859_16_INIT: Encoding = _;
```

### Static `ISO_8859_16`

The ISO-8859-16 encoding.

This is the South-Eastern European part of the ISO/IEC 8859 encoding
family. This encoding is also known as Latin 10.

[Index visualization](https://encoding.spec.whatwg.org/iso-8859-16.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-16-bmp.html)

The Windows code page number for this encoding is 28606, but kernel32.dll
does not support this encoding.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static ISO_8859_16: &''static Encoding = _;
```

### Static `ISO_8859_2_INIT`

The initializer for the [ISO-8859-2](static.ISO_8859_2.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static ISO_8859_2_INIT: Encoding = _;
```

### Static `ISO_8859_2`

The ISO-8859-2 encoding.

This is the Central European part of the ISO/IEC 8859 encoding family. This encoding is also known as Latin 2.

[Index visualization](https://encoding.spec.whatwg.org/iso-8859-2.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-2-bmp.html)

This encoding matches the Windows code page 28592.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static ISO_8859_2: &''static Encoding = _;
```

### Static `ISO_8859_3_INIT`

The initializer for the [ISO-8859-3](static.ISO_8859_3.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static ISO_8859_3_INIT: Encoding = _;
```

### Static `ISO_8859_3`

The ISO-8859-3 encoding.

This is the South European part of the ISO/IEC 8859 encoding family. This encoding is also known as Latin 3.

[Index visualization](https://encoding.spec.whatwg.org/iso-8859-3.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-3-bmp.html)

This encoding matches the Windows code page 28593.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static ISO_8859_3: &''static Encoding = _;
```

### Static `ISO_8859_4_INIT`

The initializer for the [ISO-8859-4](static.ISO_8859_4.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static ISO_8859_4_INIT: Encoding = _;
```

### Static `ISO_8859_4`

The ISO-8859-4 encoding.

This is the North European part of the ISO/IEC 8859 encoding family. This encoding is also known as Latin 4.

[Index visualization](https://encoding.spec.whatwg.org/iso-8859-4.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-4-bmp.html)

This encoding matches the Windows code page 28594.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static ISO_8859_4: &''static Encoding = _;
```

### Static `ISO_8859_5_INIT`

The initializer for the [ISO-8859-5](static.ISO_8859_5.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static ISO_8859_5_INIT: Encoding = _;
```

### Static `ISO_8859_5`

The ISO-8859-5 encoding.

This is the Cyrillic part of the ISO/IEC 8859 encoding family.

[Index visualization](https://encoding.spec.whatwg.org/iso-8859-5.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-5-bmp.html)

This encoding matches the Windows code page 28595.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static ISO_8859_5: &''static Encoding = _;
```

### Static `ISO_8859_6_INIT`

The initializer for the [ISO-8859-6](static.ISO_8859_6.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static ISO_8859_6_INIT: Encoding = _;
```

### Static `ISO_8859_6`

The ISO-8859-6 encoding.

This is the Arabic part of the ISO/IEC 8859 encoding family.

[Index visualization](https://encoding.spec.whatwg.org/iso-8859-6.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-6-bmp.html)

This encoding matches the Windows code page 28596, except Windows decodes
unassigned code points to the Private Use Area of Unicode.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static ISO_8859_6: &''static Encoding = _;
```

### Static `ISO_8859_7_INIT`

The initializer for the [ISO-8859-7](static.ISO_8859_7.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static ISO_8859_7_INIT: Encoding = _;
```

### Static `ISO_8859_7`

The ISO-8859-7 encoding.

This is the Greek part of the ISO/IEC 8859 encoding family.

[Index visualization](https://encoding.spec.whatwg.org/iso-8859-7.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-7-bmp.html)

This encoding roughly matches the Windows code page 28597. Windows decodes
unassigned code points, the currency signs at 0xA4 and 0xA5 as well as
0xAA, which should be U+037A GREEK YPOGEGRAMMENI, to the Private Use Area
of Unicode. Windows decodes 0xA1 to U+02BD MODIFIER LETTER REVERSED COMMA
instead of U+2018 LEFT SINGLE QUOTATION MARK and 0xA2 to U+02BC MODIFIER
LETTER APOSTROPHE instead of U+2019 RIGHT SINGLE QUOTATION MARK.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static ISO_8859_7: &''static Encoding = _;
```

### Static `ISO_8859_8_INIT`

The initializer for the [ISO-8859-8](static.ISO_8859_8.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static ISO_8859_8_INIT: Encoding = _;
```

### Static `ISO_8859_8`

The ISO-8859-8 encoding.

This is the Hebrew part of the ISO/IEC 8859 encoding family in visual order.

[Index visualization](https://encoding.spec.whatwg.org/iso-8859-8.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-8-bmp.html)

This encoding roughly matches the Windows code page 28598. Windows decodes
0xAF to OVERLINE instead of MACRON and 0xFE and 0xFD to the Private Use
Area instead of LRM and RLM. Windows decodes unassigned code points to
the private use area.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static ISO_8859_8: &''static Encoding = _;
```

### Static `ISO_8859_8_I_INIT`

The initializer for the [ISO-8859-8-I](static.ISO_8859_8_I.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static ISO_8859_8_I_INIT: Encoding = _;
```

### Static `ISO_8859_8_I`

The ISO-8859-8-I encoding.

This is the Hebrew part of the ISO/IEC 8859 encoding family in logical order.

[Index visualization](https://encoding.spec.whatwg.org/iso-8859-8.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-8-bmp.html)

This encoding roughly matches the Windows code page 38598. Windows decodes
0xAF to OVERLINE instead of MACRON and 0xFE and 0xFD to the Private Use
Area instead of LRM and RLM. Windows decodes unassigned code points to
the private use area.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static ISO_8859_8_I: &''static Encoding = _;
```

### Static `KOI8_R_INIT`

The initializer for the [KOI8-R](static.KOI8_R.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static KOI8_R_INIT: Encoding = _;
```

### Static `KOI8_R`

The KOI8-R encoding.

This is an encoding for Russian from [RFC 1489](https://tools.ietf.org/html/rfc1489).

[Index visualization](https://encoding.spec.whatwg.org/koi8-r.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/koi8-r-bmp.html)

This encoding matches the Windows code page 20866.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static KOI8_R: &''static Encoding = _;
```

### Static `KOI8_U_INIT`

The initializer for the [KOI8-U](static.KOI8_U.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static KOI8_U_INIT: Encoding = _;
```

### Static `KOI8_U`

The KOI8-U encoding.

This is an encoding for Ukrainian adapted from KOI8-R.

[Index visualization](https://encoding.spec.whatwg.org/koi8-u.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/koi8-u-bmp.html)

This encoding matches the Windows code page 21866.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static KOI8_U: &''static Encoding = _;
```

### Static `SHIFT_JIS_INIT`

The initializer for the [Shift_JIS](static.SHIFT_JIS.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static SHIFT_JIS_INIT: Encoding = _;
```

### Static `SHIFT_JIS`

The Shift_JIS encoding.

This is the Japanese encoding for Windows.

[Index visualization](https://encoding.spec.whatwg.org/shift_jis.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/shift_jis-bmp.html)

This encoding matches the Windows code page 932, except Windows decodes some byte
sequences that are error per the Encoding Standard to the question mark or the
Private Use Area and generally uses U+30FB in place of the REPLACEMENT CHARACTER.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static SHIFT_JIS: &''static Encoding = _;
```

### Static `UTF_16BE_INIT`

The initializer for the [UTF-16BE](static.UTF_16BE.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static UTF_16BE_INIT: Encoding = _;
```

### Static `UTF_16BE`

The UTF-16BE encoding.

This decode-only encoding uses 16-bit code units due to Unicode originally
having been designed as a 16-bit reportoire. In the absence of a byte order
mark the big endian byte order is assumed.

There is no corresponding encoder in this crate or in the Encoding
Standard. The output encoding of this encoding is UTF-8.

This encoding matches the Windows code page 1201.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static UTF_16BE: &''static Encoding = _;
```

### Static `UTF_16LE_INIT`

The initializer for the [UTF-16LE](static.UTF_16LE.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static UTF_16LE_INIT: Encoding = _;
```

### Static `UTF_16LE`

The UTF-16LE encoding.

This decode-only encoding uses 16-bit code units due to Unicode originally
having been designed as a 16-bit reportoire. In the absence of a byte order
mark the little endian byte order is assumed.

There is no corresponding encoder in this crate or in the Encoding
Standard. The output encoding of this encoding is UTF-8.

This encoding matches the Windows code page 1200.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static UTF_16LE: &''static Encoding = _;
```

### Static `UTF_8_INIT`

The initializer for the [UTF-8](static.UTF_8.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static UTF_8_INIT: Encoding = _;
```

### Static `UTF_8`

The UTF-8 encoding.

This is the encoding that should be used for all new development it can
represent all of Unicode.

This encoding matches the Windows code page 65001, except Windows differs
in the number of errors generated for some erroneous byte sequences.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static UTF_8: &''static Encoding = _;
```

### Static `GB18030_INIT`

The initializer for the [gb18030](static.GB18030.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static GB18030_INIT: Encoding = _;
```

### Static `GB18030`

The gb18030 encoding.

This encoding matches GB18030-2022 except the two-byte sequence 0xA3 0xA0
maps to U+3000 for compatibility with existing Web content and the four-byte
sequences for the non-PUA characters that got two-byte sequences still decode
to the same non-PUA characters as in GB18030-2005. As a result, this encoding
can represent all of Unicode except for 19 private-use characters.

[Index visualization for the two-byte sequences](https://encoding.spec.whatwg.org/gb18030.html),
[Visualization of BMP coverage of the two-byte index](https://encoding.spec.whatwg.org/gb18030-bmp.html)

This encoding matches the Windows code page 54936.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static GB18030: &''static Encoding = _;
```

### Static `MACINTOSH_INIT`

The initializer for the [macintosh](static.MACINTOSH.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static MACINTOSH_INIT: Encoding = _;
```

### Static `MACINTOSH`

The macintosh encoding.

This is the MacRoman encoding from Mac OS Classic.

[Index visualization](https://encoding.spec.whatwg.org/macintosh.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/macintosh-bmp.html)

This encoding matches the Windows code page 10000, except Windows decodes
0xBD to U+2126 OHM SIGN instead of U+03A9 GREEK CAPITAL LETTER OMEGA.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static MACINTOSH: &''static Encoding = _;
```

### Static `REPLACEMENT_INIT`

The initializer for the [replacement](static.REPLACEMENT.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static REPLACEMENT_INIT: Encoding = _;
```

### Static `REPLACEMENT`

The replacement encoding.

This decode-only encoding decodes all non-zero-length streams to a single
REPLACEMENT CHARACTER. Its purpose is to avoid the use of an
ASCII-compatible fallback encoding (typically windows-1252) for some
encodings that are no longer supported by the Web Platform and that
would be dangerous to treat as ASCII-compatible.

There is no corresponding encoder. The output encoding of this encoding
is UTF-8.

This encoding does not have a Windows code page number.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static REPLACEMENT: &''static Encoding = _;
```

### Static `WINDOWS_1250_INIT`

The initializer for the [windows-1250](static.WINDOWS_1250.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static WINDOWS_1250_INIT: Encoding = _;
```

### Static `WINDOWS_1250`

The windows-1250 encoding.

This is the Central European encoding for Windows.

[Index visualization](https://encoding.spec.whatwg.org/windows-1250.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1250-bmp.html)

This encoding matches the Windows code page 1250.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static WINDOWS_1250: &''static Encoding = _;
```

### Static `WINDOWS_1251_INIT`

The initializer for the [windows-1251](static.WINDOWS_1251.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static WINDOWS_1251_INIT: Encoding = _;
```

### Static `WINDOWS_1251`

The windows-1251 encoding.

This is the Cyrillic encoding for Windows.

[Index visualization](https://encoding.spec.whatwg.org/windows-1251.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1251-bmp.html)

This encoding matches the Windows code page 1251.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static WINDOWS_1251: &''static Encoding = _;
```

### Static `WINDOWS_1252_INIT`

The initializer for the [windows-1252](static.WINDOWS_1252.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static WINDOWS_1252_INIT: Encoding = _;
```

### Static `WINDOWS_1252`

The windows-1252 encoding.

This is the Western encoding for Windows. It is an extension of ISO-8859-1,
which is known as Latin 1.

[Index visualization](https://encoding.spec.whatwg.org/windows-1252.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1252-bmp.html)

This encoding matches the Windows code page 1252.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static WINDOWS_1252: &''static Encoding = _;
```

### Static `WINDOWS_1253_INIT`

The initializer for the [windows-1253](static.WINDOWS_1253.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static WINDOWS_1253_INIT: Encoding = _;
```

### Static `WINDOWS_1253`

The windows-1253 encoding.

This is the Greek encoding for Windows. It is mostly an extension of
ISO-8859-7, but U+0386 is mapped to a different byte.

[Index visualization](https://encoding.spec.whatwg.org/windows-1253.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1253-bmp.html)

This encoding matches the Windows code page 1253, except Windows decodes
unassigned code points to the Private Use Area of Unicode.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static WINDOWS_1253: &''static Encoding = _;
```

### Static `WINDOWS_1254_INIT`

The initializer for the [windows-1254](static.WINDOWS_1254.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static WINDOWS_1254_INIT: Encoding = _;
```

### Static `WINDOWS_1254`

The windows-1254 encoding.

This is the Turkish encoding for Windows. It is an extension of ISO-8859-9,
which is known as Latin 5.

[Index visualization](https://encoding.spec.whatwg.org/windows-1254.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1254-bmp.html)

This encoding matches the Windows code page 1254.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static WINDOWS_1254: &''static Encoding = _;
```

### Static `WINDOWS_1255_INIT`

The initializer for the [windows-1255](static.WINDOWS_1255.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static WINDOWS_1255_INIT: Encoding = _;
```

### Static `WINDOWS_1255`

The windows-1255 encoding.

This is the Hebrew encoding for Windows. It is an extension of ISO-8859-8-I,
except for a currency sign swap.

[Index visualization](https://encoding.spec.whatwg.org/windows-1255.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1255-bmp.html)

This encoding matches the Windows code page 1255, except Windows decodes
unassigned code points to the Private Use Area of Unicode.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static WINDOWS_1255: &''static Encoding = _;
```

### Static `WINDOWS_1256_INIT`

The initializer for the [windows-1256](static.WINDOWS_1256.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static WINDOWS_1256_INIT: Encoding = _;
```

### Static `WINDOWS_1256`

The windows-1256 encoding.

This is the Arabic encoding for Windows.

[Index visualization](https://encoding.spec.whatwg.org/windows-1256.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1256-bmp.html)

This encoding matches the Windows code page 1256.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static WINDOWS_1256: &''static Encoding = _;
```

### Static `WINDOWS_1257_INIT`

The initializer for the [windows-1257](static.WINDOWS_1257.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static WINDOWS_1257_INIT: Encoding = _;
```

### Static `WINDOWS_1257`

The windows-1257 encoding.

This is the Baltic encoding for Windows.

[Index visualization](https://encoding.spec.whatwg.org/windows-1257.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1257-bmp.html)

This encoding matches the Windows code page 1257, except Windows decodes
unassigned code points to the Private Use Area of Unicode.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static WINDOWS_1257: &''static Encoding = _;
```

### Static `WINDOWS_1258_INIT`

The initializer for the [windows-1258](static.WINDOWS_1258.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static WINDOWS_1258_INIT: Encoding = _;
```

### Static `WINDOWS_1258`

The windows-1258 encoding.

This is the Vietnamese encoding for Windows.

[Index visualization](https://encoding.spec.whatwg.org/windows-1258.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1258-bmp.html)

This encoding matches the Windows code page 1258 when used in the
non-normalizing mode. Unlike with the other single-byte encodings, the
result of decoding is not necessarily in Normalization Form C. On the
other hand, input in the Normalization Form C is not encoded without
replacement. In general, it's a bad idea to encode to encodings other
than UTF-8, but this encoding is especially hazardous to encode to.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static WINDOWS_1258: &''static Encoding = _;
```

### Static `WINDOWS_874_INIT`

The initializer for the [windows-874](static.WINDOWS_874.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static WINDOWS_874_INIT: Encoding = _;
```

### Static `WINDOWS_874`

The windows-874 encoding.

This is the Thai encoding for Windows. It is an extension of TIS-620 / ISO-8859-11.

[Index visualization](https://encoding.spec.whatwg.org/windows-874.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-874-bmp.html)

This encoding matches the Windows code page 874, except Windows decodes
unassigned code points to the Private Use Area of Unicode.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static WINDOWS_874: &''static Encoding = _;
```

### Static `X_MAC_CYRILLIC_INIT`

The initializer for the [x-mac-cyrillic](static.X_MAC_CYRILLIC.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static X_MAC_CYRILLIC_INIT: Encoding = _;
```

### Static `X_MAC_CYRILLIC`

The x-mac-cyrillic encoding.

This is the MacUkrainian encoding from Mac OS Classic.

[Index visualization](https://encoding.spec.whatwg.org/x-mac-cyrillic.html),
[Visualization of BMP coverage](https://encoding.spec.whatwg.org/x-mac-cyrillic-bmp.html)

This encoding matches the Windows code page 10017.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static X_MAC_CYRILLIC: &''static Encoding = _;
```

### Static `X_USER_DEFINED_INIT`

The initializer for the [x-user-defined](static.X_USER_DEFINED.html) encoding.

For use only for taking the address of this form when
Rust prohibits the use of the non-`_INIT` form directly,
such as in initializers of other `static`s. If in doubt,
use the corresponding non-`_INIT` reference-typed `static`.

This part of the public API will go away if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate or if Rust starts allowing static arrays
to be initialized with `pub static FOO: &'static Encoding`
items.

```rust
pub static X_USER_DEFINED_INIT: Encoding = _;
```

### Static `X_USER_DEFINED`

The x-user-defined encoding.

This encoding offsets the non-ASCII bytes by `0xF700` thereby decoding
them to the Private Use Area of Unicode. It was used for loading binary
data into a JavaScript string using `XMLHttpRequest` before XHR supported
the `"arraybuffer"` response type.

This encoding does not have a Windows code page number.

This will change from `static` to `const` if Rust changes
to make the referent of `pub const FOO: &'static Encoding`
unique cross-crate, so don't take the address of this
`static`.

```rust
pub static X_USER_DEFINED: &''static Encoding = _;
```

