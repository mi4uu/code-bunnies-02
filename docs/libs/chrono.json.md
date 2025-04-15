# Crate Documentation

**Version:** 0.4.40

**Format Version:** 43

# Module `chrono`

# Chrono: Date and Time for Rust

Chrono aims to provide all functionality needed to do correct operations on dates and times in
the [proleptic Gregorian calendar]:

* The [`DateTime`] type is timezone-aware by default, with separate timezone-naive types.
* Operations that may produce an invalid or ambiguous date and time return `Option` or
  [`MappedLocalTime`].
* Configurable parsing and formatting with a `strftime` inspired date and time formatting
  syntax.
* The [`Local`] timezone works with the current timezone of the OS.
* Types and operations are implemented to be reasonably efficient.

Timezone data is not shipped with chrono by default to limit binary sizes. Use the companion
crate [Chrono-TZ] or [`tzfile`] for full timezone support.

[proleptic Gregorian calendar]: https://en.wikipedia.org/wiki/Proleptic_Gregorian_calendar
[Chrono-TZ]: https://crates.io/crates/chrono-tz
[`tzfile`]: https://crates.io/crates/tzfile

### Features

Chrono supports various runtime environments and operating systems, and has several features
that may be enabled or disabled.

Default features:

- `alloc`: Enable features that depend on allocation (primarily string formatting).
- `std`: Enables functionality that depends on the standard library. This is a superset of
  `alloc` and adds interoperation with standard library types and traits.
- `clock`: Enables reading the local timezone (`Local`). This is a superset of `now`.
- `now`: Enables reading the system time (`now`).
- `wasmbind`: Interface with the JS Date API for the `wasm32` target.

Optional features:

- `serde`: Enable serialization/deserialization via [serde].
- `rkyv`: Deprecated, use the `rkyv-*` features.
- `rkyv-16`: Enable serialization/deserialization via [rkyv],
   using 16-bit integers for integral `*size` types.
- `rkyv-32`: Enable serialization/deserialization via [rkyv],
   using 32-bit integers for integral `*size` types.
- `rkyv-64`: Enable serialization/deserialization via [rkyv],
   using 64-bit integers for integral `*size` types.
- `rkyv-validation`: Enable rkyv validation support using `bytecheck`.
- `arbitrary`: Construct arbitrary instances of a type with the Arbitrary crate.
- `unstable-locales`: Enable localization. This adds various methods with a `_localized` suffix.
  The implementation and API may change or even be removed in a patch release. Feedback welcome.
- `oldtime`: This feature no longer has any effect; it used to offer compatibility with the
  `time` 0.1 crate.

Note: The `rkyv{,-16,-32,-64}` features are mutually exclusive.

See the [cargo docs] for examples of specifying features.

[serde]: https://github.com/serde-rs/serde
[rkyv]: https://github.com/rkyv/rkyv
[cargo docs]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#choosing-features

## Overview

### Time delta / Duration

Chrono has a [`TimeDelta`] type to represent the magnitude of a time span. This is an "accurate"
duration represented as seconds and nanoseconds, and does not represent "nominal" components
such as days or months.

The [`TimeDelta`] type was previously named `Duration` (and is still available as a type alias
with that name). A notable difference with the similar [`core::time::Duration`] is that it is a
signed value instead of unsigned.

Chrono currently only supports a small number of operations with [`core::time::Duration`].
You can convert between both types with the [`TimeDelta::from_std`] and [`TimeDelta::to_std`]
methods.

### Date and Time

Chrono provides a [`DateTime`] type to represent a date and a time in a timezone.

For more abstract moment-in-time tracking such as internal timekeeping that is unconcerned with
timezones, consider [`std::time::SystemTime`], which tracks your system clock, or
[`std::time::Instant`], which is an opaque but monotonically-increasing representation of a
moment in time.

[`DateTime`] is timezone-aware and must be constructed from a [`TimeZone`] object, which defines
how the local date is converted to and back from the UTC date.
There are three well-known [`TimeZone`] implementations:

* [`Utc`] specifies the UTC time zone. It is most efficient.

* [`Local`] specifies the system local time zone.

* [`FixedOffset`] specifies an arbitrary, fixed time zone such as UTC+09:00 or UTC-10:30.
  This often results from the parsed textual date and time. Since it stores the most information
  and does not depend on the system environment, you would want to normalize other `TimeZone`s
  into this type.

[`DateTime`]s with different [`TimeZone`] types are distinct and do not mix, but can be
converted to each other using the [`DateTime::with_timezone`] method.

You can get the current date and time in the UTC time zone ([`Utc::now()`]) or in the local time
zone ([`Local::now()`]).

```
# #[cfg(feature = "now")] {
use chrono::prelude::*;

let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
# let _ = utc;
# }
```

```
# #[cfg(feature = "clock")] {
use chrono::prelude::*;

let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
# let _ = local;
# }
```

Alternatively, you can create your own date and time. This is a bit verbose due to Rust's lack
of function and method overloading, but in turn we get a rich combination of initialization
methods.

```
use chrono::offset::MappedLocalTime;
use chrono::prelude::*;

# fn doctest() -> Option<()> {

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
assert_eq!(
    dt,
    NaiveDate::from_ymd_opt(2014, 7, 8)?
        .and_hms_opt(9, 10, 11)?
        .and_utc()
);

// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, NaiveDate::from_yo_opt(2014, 189)?.and_hms_opt(9, 10, 11)?.and_utc());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(
    dt,
    NaiveDate::from_isoywd_opt(2014, 28, Weekday::Tue)?.and_hms_opt(9, 10, 11)?.and_utc()
);

let dt = NaiveDate::from_ymd_opt(2014, 7, 8)?
    .and_hms_milli_opt(9, 10, 11, 12)?
    .and_utc(); // `2014-07-08T09:10:11.012Z`
assert_eq!(
    dt,
    NaiveDate::from_ymd_opt(2014, 7, 8)?
        .and_hms_micro_opt(9, 10, 11, 12_000)?
        .and_utc()
);
assert_eq!(
    dt,
    NaiveDate::from_ymd_opt(2014, 7, 8)?
        .and_hms_nano_opt(9, 10, 11, 12_000_000)?
        .and_utc()
);

// dynamic verification
assert_eq!(
    Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33),
    MappedLocalTime::Single(
        NaiveDate::from_ymd_opt(2014, 7, 8)?.and_hms_opt(21, 15, 33)?.and_utc()
    )
);
assert_eq!(Utc.with_ymd_and_hms(2014, 7, 8, 80, 15, 33), MappedLocalTime::None);
assert_eq!(Utc.with_ymd_and_hms(2014, 7, 38, 21, 15, 33), MappedLocalTime::None);

# #[cfg(feature = "clock")] {
// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local
    .from_local_datetime(
        &NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap(),
    )
    .unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600)
    .unwrap()
    .from_local_datetime(
        &NaiveDate::from_ymd_opt(2014, 7, 8)
            .unwrap()
            .and_hms_milli_opt(18, 10, 11, 12)
            .unwrap(),
    )
    .unwrap();
assert_eq!(dt, fixed_dt);
# let _ = local_dt;
# }
# Some(())
# }
# doctest().unwrap();
```

Various properties are available to the date and time, and can be altered individually. Most of
them are defined in the traits [`Datelike`] and [`Timelike`] which you should `use` before.
Addition and subtraction is also supported.
The following illustrates most supported operations to the date and time:

```rust
use chrono::prelude::*;
use chrono::TimeDelta;

// assume this returned `2014-11-28T21:45:59.324310806+09:00`:
let dt = FixedOffset::east_opt(9 * 3600)
    .unwrap()
    .from_local_datetime(
        &NaiveDate::from_ymd_opt(2014, 11, 28)
            .unwrap()
            .and_hms_nano_opt(21, 45, 59, 324310806)
            .unwrap(),
    )
    .unwrap();

// property accessors
assert_eq!((dt.year(), dt.month(), dt.day()), (2014, 11, 28));
assert_eq!((dt.month0(), dt.day0()), (10, 27)); // for unfortunate souls
assert_eq!((dt.hour(), dt.minute(), dt.second()), (21, 45, 59));
assert_eq!(dt.weekday(), Weekday::Fri);
assert_eq!(dt.weekday().number_from_monday(), 5); // Mon=1, ..., Sun=7
assert_eq!(dt.ordinal(), 332); // the day of year
assert_eq!(dt.num_days_from_ce(), 735565); // the number of days from and including Jan 1, 1

// time zone accessor and manipulation
assert_eq!(dt.offset().fix().local_minus_utc(), 9 * 3600);
assert_eq!(dt.timezone(), FixedOffset::east_opt(9 * 3600).unwrap());
assert_eq!(
    dt.with_timezone(&Utc),
    NaiveDate::from_ymd_opt(2014, 11, 28)
        .unwrap()
        .and_hms_nano_opt(12, 45, 59, 324310806)
        .unwrap()
        .and_utc()
);

// a sample of property manipulations (validates dynamically)
assert_eq!(dt.with_day(29).unwrap().weekday(), Weekday::Sat); // 2014-11-29 is Saturday
assert_eq!(dt.with_day(32), None);
assert_eq!(dt.with_year(-300).unwrap().num_days_from_ce(), -109606); // November 29, 301 BCE

// arithmetic operations
let dt1 = Utc.with_ymd_and_hms(2014, 11, 14, 8, 9, 10).unwrap();
let dt2 = Utc.with_ymd_and_hms(2014, 11, 14, 10, 9, 8).unwrap();
assert_eq!(dt1.signed_duration_since(dt2), TimeDelta::try_seconds(-2 * 3600 + 2).unwrap());
assert_eq!(dt2.signed_duration_since(dt1), TimeDelta::try_seconds(2 * 3600 - 2).unwrap());
assert_eq!(
    Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap()
        + TimeDelta::try_seconds(1_000_000_000).unwrap(),
    Utc.with_ymd_and_hms(2001, 9, 9, 1, 46, 40).unwrap()
);
assert_eq!(
    Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap()
        - TimeDelta::try_seconds(1_000_000_000).unwrap(),
    Utc.with_ymd_and_hms(1938, 4, 24, 22, 13, 20).unwrap()
);
```

### Formatting and Parsing

Formatting is done via the [`format`](DateTime::format()) method, which format is equivalent to
the familiar `strftime` format.

See [`format::strftime`](format::strftime#specifiers) documentation for full syntax and list of
specifiers.

The default `to_string` method and `{:?}` specifier also give a reasonable representation.
Chrono also provides [`to_rfc2822`](DateTime::to_rfc2822) and
[`to_rfc3339`](DateTime::to_rfc3339) methods for well-known formats.

Chrono now also provides date formatting in almost any language without the help of an
additional C library. This functionality is under the feature `unstable-locales`:

```toml
chrono = { version = "0.4", features = ["unstable-locales"] }
```

The `unstable-locales` feature requires and implies at least the `alloc` feature.

```rust
# #[allow(unused_imports)]
use chrono::prelude::*;

# #[cfg(all(feature = "unstable-locales", feature = "alloc"))]
# fn test() {
let dt = Utc.with_ymd_and_hms(2014, 11, 28, 12, 0, 9).unwrap();
assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2014-11-28 12:00:09");
assert_eq!(dt.format("%a %b %e %T %Y").to_string(), "Fri Nov 28 12:00:09 2014");
assert_eq!(
    dt.format_localized("%A %e %B %Y, %T", Locale::fr_BE).to_string(),
    "vendredi 28 novembre 2014, 12:00:09"
);

assert_eq!(dt.format("%a %b %e %T %Y").to_string(), dt.format("%c").to_string());
assert_eq!(dt.to_string(), "2014-11-28 12:00:09 UTC");
assert_eq!(dt.to_rfc2822(), "Fri, 28 Nov 2014 12:00:09 +0000");
assert_eq!(dt.to_rfc3339(), "2014-11-28T12:00:09+00:00");
assert_eq!(format!("{:?}", dt), "2014-11-28T12:00:09Z");

// Note that milli/nanoseconds are only printed if they are non-zero
let dt_nano = NaiveDate::from_ymd_opt(2014, 11, 28)
    .unwrap()
    .and_hms_nano_opt(12, 0, 9, 1)
    .unwrap()
    .and_utc();
assert_eq!(format!("{:?}", dt_nano), "2014-11-28T12:00:09.000000001Z");
# }
# #[cfg(not(all(feature = "unstable-locales", feature = "alloc")))]
# fn test() {}
# if cfg!(all(feature = "unstable-locales", feature = "alloc")) {
#    test();
# }
```

Parsing can be done with two methods:

1. The standard [`FromStr`](std::str::FromStr) trait (and [`parse`](str::parse) method on a
   string) can be used for parsing `DateTime<FixedOffset>`, `DateTime<Utc>` and
   `DateTime<Local>` values. This parses what the `{:?}` ([`std::fmt::Debug`] format specifier
   prints, and requires the offset to be present.

2. [`DateTime::parse_from_str`] parses a date and time with offsets and returns
   `DateTime<FixedOffset>`. This should be used when the offset is a part of input and the
   caller cannot guess that. It *cannot* be used when the offset can be missing.
   [`DateTime::parse_from_rfc2822`] and [`DateTime::parse_from_rfc3339`] are similar but for
   well-known formats.

More detailed control over the parsing process is available via [`format`](mod@format) module.

```rust
use chrono::prelude::*;

let dt = Utc.with_ymd_and_hms(2014, 11, 28, 12, 0, 9).unwrap();
let fixed_dt = dt.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());

// method 1
assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));

// method 2
assert_eq!(
    DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"),
    Ok(fixed_dt.clone())
);
assert_eq!(
    DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"),
    Ok(fixed_dt.clone())
);
assert_eq!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"), Ok(fixed_dt.clone()));

// oops, the year is missing!
assert!(DateTime::parse_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y").is_err());
// oops, the format string does not include the year at all!
assert!(DateTime::parse_from_str("Fri Nov 28 12:00:09", "%a %b %e %T").is_err());
// oops, the weekday is incorrect!
assert!(DateTime::parse_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y").is_err());
```

Again: See [`format::strftime`](format::strftime#specifiers) documentation for full syntax and
list of specifiers.

### Conversion from and to EPOCH timestamps

Use [`DateTime::from_timestamp(seconds, nanoseconds)`](DateTime::from_timestamp)
to construct a [`DateTime<Utc>`] from a UNIX timestamp
(seconds, nanoseconds that passed since January 1st 1970).

Use [`DateTime.timestamp`](DateTime::timestamp) to get the timestamp (in seconds)
from a [`DateTime`]. Additionally, you can use
[`DateTime.timestamp_subsec_nanos`](DateTime::timestamp_subsec_nanos)
to get the number of additional number of nanoseconds.

```
# #[cfg(feature = "alloc")] {
// We need the trait in scope to use Utc::timestamp().
use chrono::{DateTime, Utc};

// Construct a datetime from epoch:
let dt: DateTime<Utc> = DateTime::from_timestamp(1_500_000_000, 0).unwrap();
assert_eq!(dt.to_rfc2822(), "Fri, 14 Jul 2017 02:40:00 +0000");

// Get epoch value from a datetime:
let dt = DateTime::parse_from_rfc2822("Fri, 14 Jul 2017 02:40:00 +0000").unwrap();
assert_eq!(dt.timestamp(), 1_500_000_000);
# }
```

### Naive date and time

Chrono provides naive counterparts to `Date`, (non-existent) `Time` and `DateTime` as
[`NaiveDate`], [`NaiveTime`] and [`NaiveDateTime`] respectively.

They have almost equivalent interfaces as their timezone-aware twins, but are not associated to
time zones obviously and can be quite low-level. They are mostly useful for building blocks for
higher-level types.

Timezone-aware `DateTime` and `Date` types have two methods returning naive versions:
[`naive_local`](DateTime::naive_local) returns a view to the naive local time,
and [`naive_utc`](DateTime::naive_utc) returns a view to the naive UTC time.

## Limitations

* Only the proleptic Gregorian calendar (i.e. extended to support older dates) is supported.
* Date types are limited to about +/- 262,000 years from the common epoch.
* Time types are limited to nanosecond accuracy.
* Leap seconds can be represented, but Chrono does not fully support them.
  See [Leap Second Handling](NaiveTime#leap-second-handling).

## Rust version requirements

The Minimum Supported Rust Version (MSRV) is currently **Rust 1.61.0**.

The MSRV is explicitly tested in CI. It may be bumped in minor releases, but this is not done
lightly.

## Relation between chrono and time 0.1

Rust first had a `time` module added to `std` in its 0.7 release. It later moved to
`libextra`, and then to a `libtime` library shipped alongside the standard library. In 2014
work on chrono started in order to provide a full-featured date and time library in Rust.
Some improvements from chrono made it into the standard library; notably, `chrono::Duration`
was included as `std::time::Duration` ([rust#15934]) in 2014.

In preparation of Rust 1.0 at the end of 2014 `libtime` was moved out of the Rust distro and
into the `time` crate to eventually be redesigned ([rust#18832], [rust#18858]), like the
`num` and `rand` crates. Of course chrono kept its dependency on this `time` crate. `time`
started re-exporting `std::time::Duration` during this period. Later, the standard library was
changed to have a more limited unsigned `Duration` type ([rust#24920], [RFC 1040]), while the
`time` crate kept the full functionality with `time::Duration`. `time::Duration` had been a
part of chrono's public API.

By 2016 `time` 0.1 lived under the `rust-lang-deprecated` organisation and was not actively
maintained ([time#136]). chrono absorbed the platform functionality and `Duration` type of the
`time` crate in [chrono#478] (the work started in [chrono#286]). In order to preserve
compatibility with downstream crates depending on `time` and `chrono` sharing a `Duration`
type, chrono kept depending on time 0.1. chrono offered the option to opt out of the `time`
dependency by disabling the `oldtime` feature (swapping it out for an effectively similar
chrono type). In 2019, @jhpratt took over maintenance on the `time` crate and released what
amounts to a new crate as `time` 0.2.

[rust#15934]: https://github.com/rust-lang/rust/pull/15934
[rust#18832]: https://github.com/rust-lang/rust/pull/18832#issuecomment-62448221
[rust#18858]: https://github.com/rust-lang/rust/pull/18858
[rust#24920]: https://github.com/rust-lang/rust/pull/24920
[RFC 1040]: https://rust-lang.github.io/rfcs/1040-duration-reform.html
[time#136]: https://github.com/time-rs/time/issues/136
[chrono#286]: https://github.com/chronotope/chrono/pull/286
[chrono#478]: https://github.com/chronotope/chrono/pull/478

## Security advisories

In November of 2020 [CVE-2020-26235] and [RUSTSEC-2020-0071] were opened against the `time` crate.
@quininer had found that calls to `localtime_r` may be unsound ([chrono#499]). Eventually, almost
a year later, this was also made into a security advisory against chrono as [RUSTSEC-2020-0159],
which had platform code similar to `time`.

On Unix-like systems a process is given a timezone id or description via the `TZ` environment
variable. We need this timezone data to calculate the current local time from a value that is
in UTC, such as the time from the system clock. `time` 0.1 and chrono used the POSIX function
`localtime_r` to do the conversion to local time, which reads the `TZ` variable.

Rust assumes the environment to be writable and uses locks to access it from multiple threads.
Some other programming languages and libraries use similar locking strategies, but these are
typically not shared across languages. More importantly, POSIX declares modifying the
environment in a multi-threaded process as unsafe, and `getenv` in libc can't be changed to
take a lock because it returns a pointer to the data (see [rust#27970] for more discussion).

Since version 4.20 chrono no longer uses `localtime_r`, instead using Rust code to query the
timezone (from the `TZ` variable or via `iana-time-zone` as a fallback) and work with data
from the system timezone database directly. The code for this was forked from the [tz-rs crate]
by @x-hgg-x. As such, chrono now respects the Rust lock when reading the `TZ` environment
variable. In general, code should avoid modifying the environment.

[CVE-2020-26235]: https://nvd.nist.gov/vuln/detail/CVE-2020-26235
[RUSTSEC-2020-0071]: https://rustsec.org/advisories/RUSTSEC-2020-0071
[chrono#499]: https://github.com/chronotope/chrono/pull/499
[RUSTSEC-2020-0159]: https://rustsec.org/advisories/RUSTSEC-2020-0159.html
[rust#27970]: https://github.com/rust-lang/rust/issues/27970
[chrono#677]: https://github.com/chronotope/chrono/pull/677
[tz-rs crate]: https://crates.io/crates/tz-rs

## Removing time 0.1

Because time 0.1 has been unmaintained for years, however, the security advisory mentioned
above has not been addressed. While chrono maintainers were careful not to break backwards
compatibility with the `time::Duration` type, there has been a long stream of issues from
users inquiring about the time 0.1 dependency with the vulnerability. We investigated the
potential breakage of removing the time 0.1 dependency in [chrono#1095] using a crater-like
experiment and determined that the potential for breaking (public) dependencies is very low.
We reached out to those few crates that did still depend on compatibility with time 0.1.

As such, for chrono 0.4.30 we have decided to swap out the time 0.1 `Duration` implementation
for a local one that will offer a strict superset of the existing API going forward. This
will prevent most downstream users from being affected by the security vulnerability in time
0.1 while minimizing the ecosystem impact of semver-incompatible version churn.

[chrono#1095]: https://github.com/chronotope/chrono/pull/1095

## Modules

## Module `prelude`

A convenience module appropriate for glob imports (`use chrono::prelude::*;`).

```rust
pub mod prelude { /* ... */ }
```

### Re-exports

#### Re-export `Date`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use crate::Date;
```

#### Re-export `SubsecRound`

```rust
pub use crate::SubsecRound;
```

#### Re-export `DateTime`

```rust
pub use crate::DateTime;
```

#### Re-export `SecondsFormat`

```rust
pub use crate::SecondsFormat;
```

#### Re-export `Datelike`

```rust
pub use crate::Datelike;
```

#### Re-export `Month`

```rust
pub use crate::Month;
```

#### Re-export `Timelike`

```rust
pub use crate::Timelike;
```

#### Re-export `Weekday`

```rust
pub use crate::Weekday;
```

#### Re-export `FixedOffset`

```rust
pub use crate::FixedOffset;
```

#### Re-export `Utc`

```rust
pub use crate::Utc;
```

#### Re-export `NaiveDate`

```rust
pub use crate::NaiveDate;
```

#### Re-export `NaiveDateTime`

```rust
pub use crate::NaiveDateTime;
```

#### Re-export `NaiveTime`

```rust
pub use crate::NaiveTime;
```

#### Re-export `Offset`

```rust
pub use crate::Offset;
```

#### Re-export `TimeZone`

```rust
pub use crate::TimeZone;
```

## Module `format`

Formatting (and parsing) utilities for date and time.

This module provides the common types and routines to implement,
for example, [`DateTime::format`](../struct.DateTime.html#method.format) or
[`DateTime::parse_from_str`](../struct.DateTime.html#method.parse_from_str) methods.
For most cases you should use these high-level interfaces.

Internally the formatting and parsing shares the same abstract **formatting items**,
which are just an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) of
the [`Item`](./enum.Item.html) type.
They are generated from more readable **format strings**;
currently Chrono supports a built-in syntax closely resembling
C's `strftime` format. The available options can be found [here](./strftime/index.html).

# Example
```
# #[cfg(feature = "alloc")] {
use chrono::{NaiveDateTime, TimeZone, Utc};

let date_time = Utc.with_ymd_and_hms(2020, 11, 10, 0, 1, 32).unwrap();

let formatted = format!("{}", date_time.format("%Y-%m-%d %H:%M:%S"));
assert_eq!(formatted, "2020-11-10 00:01:32");

let parsed = NaiveDateTime::parse_from_str(&formatted, "%Y-%m-%d %H:%M:%S")?.and_utc();
assert_eq!(parsed, date_time);
# }
# Ok::<(), chrono::ParseError>(())
```

```rust
pub mod format { /* ... */ }
```

### Modules

## Module `strftime`

`strftime`/`strptime`-inspired date and time formatting syntax.

## Specifiers

The following specifiers are available both to formatting and parsing.

| Spec. | Example  | Description                                                                |
|-------|----------|----------------------------------------------------------------------------|
|       |          | **DATE SPECIFIERS:**                                                       |
| `%Y`  | `2001`   | The full proleptic Gregorian year, zero-padded to 4 digits. chrono supports years from -262144 to 262143. Note: years before 1 BCE or after 9999 CE, require an initial sign (+/-).|
| `%C`  | `20`     | The proleptic Gregorian year divided by 100, zero-padded to 2 digits. [^1] |
| `%y`  | `01`     | The proleptic Gregorian year modulo 100, zero-padded to 2 digits. [^1]     |
|       |          |                                                                            |
| `%q`  | `1`      | Quarter of year (1-4)                                                      |
| `%m`  | `07`     | Month number (01--12), zero-padded to 2 digits.                            |
| `%b`  | `Jul`    | Abbreviated month name. Always 3 letters.                                  |
| `%B`  | `July`   | Full month name. Also accepts corresponding abbreviation in parsing.       |
| `%h`  | `Jul`    | Same as `%b`.                                                              |
|       |          |                                                                            |
| `%d`  | `08`     | Day number (01--31), zero-padded to 2 digits.                              |
| `%e`  | ` 8`     | Same as `%d` but space-padded. Same as `%_d`.                              |
|       |          |                                                                            |
| `%a`  | `Sun`    | Abbreviated weekday name. Always 3 letters.                                |
| `%A`  | `Sunday` | Full weekday name. Also accepts corresponding abbreviation in parsing.     |
| `%w`  | `0`      | Sunday = 0, Monday = 1, ..., Saturday = 6.                                 |
| `%u`  | `7`      | Monday = 1, Tuesday = 2, ..., Sunday = 7. (ISO 8601)                       |
|       |          |                                                                            |
| `%U`  | `28`     | Week number starting with Sunday (00--53), zero-padded to 2 digits. [^2]   |
| `%W`  | `27`     | Same as `%U`, but week 1 starts with the first Monday in that year instead.|
|       |          |                                                                            |
| `%G`  | `2001`   | Same as `%Y` but uses the year number in ISO 8601 week date. [^3]          |
| `%g`  | `01`     | Same as `%y` but uses the year number in ISO 8601 week date. [^3]          |
| `%V`  | `27`     | Same as `%U` but uses the week number in ISO 8601 week date (01--53). [^3] |
|       |          |                                                                            |
| `%j`  | `189`    | Day of the year (001--366), zero-padded to 3 digits.                       |
|       |          |                                                                            |
| `%D`  | `07/08/01`    | Month-day-year format. Same as `%m/%d/%y`.                            |
| `%x`  | `07/08/01`    | Locale's date representation (e.g., 12/31/99).                        |
| `%F`  | `2001-07-08`  | Year-month-day format (ISO 8601). Same as `%Y-%m-%d`.                 |
| `%v`  | ` 8-Jul-2001` | Day-month-year format. Same as `%e-%b-%Y`.                            |
|       |          |                                                                            |
|       |          | **TIME SPECIFIERS:**                                                       |
| `%H`  | `00`     | Hour number (00--23), zero-padded to 2 digits.                             |
| `%k`  | ` 0`     | Same as `%H` but space-padded. Same as `%_H`.                              |
| `%I`  | `12`     | Hour number in 12-hour clocks (01--12), zero-padded to 2 digits.           |
| `%l`  | `12`     | Same as `%I` but space-padded. Same as `%_I`.                              |
|       |          |                                                                            |
| `%P`  | `am`     | `am` or `pm` in 12-hour clocks.                                            |
| `%p`  | `AM`     | `AM` or `PM` in 12-hour clocks.                                            |
|       |          |                                                                            |
| `%M`  | `34`     | Minute number (00--59), zero-padded to 2 digits.                           |
| `%S`  | `60`     | Second number (00--60), zero-padded to 2 digits. [^4]                      |
| `%f`  | `26490000`    | Number of nanoseconds since last whole second. [^7]                   |
| `%.f` | `.026490`| Decimal fraction of a second. Consumes the leading dot. [^7]               |
| `%.3f`| `.026`        | Decimal fraction of a second with a fixed length of 3.                |
| `%.6f`| `.026490`     | Decimal fraction of a second with a fixed length of 6.                |
| `%.9f`| `.026490000`  | Decimal fraction of a second with a fixed length of 9.                |
| `%3f` | `026`         | Decimal fraction of a second like `%.3f` but without the leading dot. |
| `%6f` | `026490`      | Decimal fraction of a second like `%.6f` but without the leading dot. |
| `%9f` | `026490000`   | Decimal fraction of a second like `%.9f` but without the leading dot. |
|       |               |                                                                       |
| `%R`  | `00:34`       | Hour-minute format. Same as `%H:%M`.                                  |
| `%T`  | `00:34:60`    | Hour-minute-second format. Same as `%H:%M:%S`.                        |
| `%X`  | `00:34:60`    | Locale's time representation (e.g., 23:13:48).                        |
| `%r`  | `12:34:60 AM` | Locale's 12 hour clock time. (e.g., 11:11:04 PM). Falls back to `%X` if the locale does not have a 12 hour clock format. |
|       |          |                                                                            |
|       |          | **TIME ZONE SPECIFIERS:**                                                  |
| `%Z`  | `ACST`   | Local time zone name. Skips all non-whitespace characters during parsing. Identical to `%:z` when formatting. [^8] |
| `%z`  | `+0930`  | Offset from the local time to UTC (with UTC being `+0000`).                |
| `%:z` | `+09:30` | Same as `%z` but with a colon.                                             |
|`%::z`|`+09:30:00`| Offset from the local time to UTC with seconds.                            |
|`%:::z`| `+09`    | Offset from the local time to UTC without minutes.                         |
| `%#z` | `+09`    | *Parsing only:* Same as `%z` but allows minutes to be missing or present.  |
|       |          |                                                                            |
|       |          | **DATE & TIME SPECIFIERS:**                                                |
|`%c`|`Sun Jul  8 00:34:60 2001`|Locale's date and time (e.g., Thu Mar  3 23:05:25 2005).       |
| `%+`  | `2001-07-08T00:34:60.026490+09:30` | ISO 8601 / RFC 3339 date & time format. [^5]     |
|       |               |                                                                       |
| `%s`  | `994518299`   | UNIX timestamp, the number of seconds since 1970-01-01 00:00 UTC. [^6]|
|       |          |                                                                            |
|       |          | **SPECIAL SPECIFIERS:**                                                    |
| `%t`  |          | Literal tab (`\t`).                                                        |
| `%n`  |          | Literal newline (`\n`).                                                    |
| `%%`  |          | Literal percent sign.                                                      |

It is possible to override the default padding behavior of numeric specifiers `%?`.
This is not allowed for other specifiers and will result in the `BAD_FORMAT` error.

Modifier | Description
-------- | -----------
`%-?`    | Suppresses any padding including spaces and zeroes. (e.g. `%j` = `012`, `%-j` = `12`)
`%_?`    | Uses spaces as a padding. (e.g. `%j` = `012`, `%_j` = ` 12`)
`%0?`    | Uses zeroes as a padding. (e.g. `%e` = ` 9`, `%0e` = `09`)

Notes:

[^1]: `%C`, `%y`:
   This is floor division, so 100 BCE (year number -99) will print `-1` and `99` respectively.
   For `%y`, values greater or equal to 70 are interpreted as being in the 20th century,
   values smaller than 70 in the 21st century.

[^2]: `%U`:
   Week 1 starts with the first Sunday in that year.
   It is possible to have week 0 for days before the first Sunday.

[^3]: `%G`, `%g`, `%V`:
   Week 1 is the first week with at least 4 days in that year.
   Week 0 does not exist, so this should be used with `%G` or `%g`.

[^4]: `%S`:
   It accounts for leap seconds, so `60` is possible.

[^5]: `%+`: Same as `%Y-%m-%dT%H:%M:%S%.f%:z`, i.e. 0, 3, 6 or 9 fractional
   digits for seconds and colons in the time zone offset.
   <br>
   <br>
   This format also supports having a `Z` or `UTC` in place of `%:z`. They
   are equivalent to `+00:00`.
   <br>
   <br>
   Note that all `T`, `Z`, and `UTC` are parsed case-insensitively.
   <br>
   <br>
   The typical `strftime` implementations have different (and locale-dependent)
   formats for this specifier. While Chrono's format for `%+` is far more
   stable, it is best to avoid this specifier if you want to control the exact
   output.

[^6]: `%s`:
   This is not padded and can be negative.
   For the purpose of Chrono, it only accounts for non-leap seconds
   so it slightly differs from ISO C `strftime` behavior.

[^7]: `%f`, `%.f`:
   <br>
   `%f` and `%.f` are notably different formatting specifiers.<br>
   `%f` counts the number of nanoseconds since the last whole second, while `%.f` is a fraction of a
   second.<br>
   Example: 7μs is formatted as `7000` with `%f`, and formatted as `.000007` with `%.f`.

[^8]: `%Z`:
   Since `chrono` is not aware of timezones beyond their offsets, this specifier
   **only prints the offset** when used for formatting. The timezone abbreviation
   will NOT be printed. See [this issue](https://github.com/chronotope/chrono/issues/960)
   for more information.
   <br>
   <br>
   Offset will not be populated from the parsed data, nor will it be validated.
   Timezone is completely ignored. Similar to the glibc `strptime` treatment of
   this format code.
   <br>
   <br>
   It is not possible to reliably convert from an abbreviation to an offset,
   for example CDT can mean either Central Daylight Time (North America) or
   China Daylight Time.

```rust
pub mod strftime { /* ... */ }
```

### Types

#### Struct `StrftimeItems`

Parsing iterator for `strftime`-like format strings.

See the [`format::strftime` module](crate::format::strftime) for supported formatting
specifiers.

`StrftimeItems` is used in combination with more low-level methods such as [`format::parse()`]
or [`format_with_items`].

If formatting or parsing date and time values is not performance-critical, the methods
[`parse_from_str`] and [`format`] on types such as [`DateTime`](crate::DateTime) are easier to
use.

[`format`]: crate::DateTime::format
[`format_with_items`]: crate::DateTime::format
[`parse_from_str`]: crate::DateTime::parse_from_str
[`DateTime`]: crate::DateTime
[`format::parse()`]: crate::format::parse()

```rust
pub struct StrftimeItems<''a> {
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
  pub const fn new(s: &''a str) -> StrftimeItems<''a> { /* ... */ }
  ```
  Creates a new parsing iterator from a `strftime`-like format string.

###### Trait Implementations

- **Send**
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> StrftimeItems<''a> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Item<''a>> { /* ... */ }
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

- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
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

### Types

#### Enum `Pad`

Padding characters for numeric items.

```rust
pub enum Pad {
    None,
    Zero,
    Space,
}
```

##### Variants

###### `None`

No padding.

###### `Zero`

Zero (`0`) padding.

###### `Space`

Space padding.

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Freeze**
- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Pad) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Pad { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

#### Enum `Numeric`

**Attributes:**

- `#[non_exhaustive]`

Numeric item types.
They have associated formatting width (FW) and parsing width (PW).

The **formatting width** is the minimal width to be formatted.
If the number is too short, and the padding is not [`Pad::None`](./enum.Pad.html#variant.None),
then it is left-padded.
If the number is too long or (in some cases) negative, it is printed as is.

The **parsing width** is the maximal width to be scanned.
The parser only tries to consume from one to given number of digits (greedily).
It also trims the preceding whitespace if any.
It cannot parse the negative number, so some date and time cannot be formatted then
parsed with the same formatting items.

```rust
pub enum Numeric {
    Year,
    YearDiv100,
    YearMod100,
    IsoYear,
    IsoYearDiv100,
    IsoYearMod100,
    Quarter,
    Month,
    Day,
    WeekFromSun,
    WeekFromMon,
    IsoWeek,
    NumDaysFromSun,
    WeekdayFromMon,
    Ordinal,
    Hour,
    Hour12,
    Minute,
    Second,
    Nanosecond,
    Timestamp,
    Internal(InternalNumeric),
}
```

##### Variants

###### `Year`

Full Gregorian year (FW=4, PW=∞).
May accept years before 1 BCE or after 9999 CE, given an initial sign (+/-).

###### `YearDiv100`

Gregorian year divided by 100 (century number; FW=PW=2). Implies the non-negative year.

###### `YearMod100`

Gregorian year modulo 100 (FW=PW=2). Cannot be negative.

###### `IsoYear`

Year in the ISO week date (FW=4, PW=∞).
May accept years before 1 BCE or after 9999 CE, given an initial sign.

###### `IsoYearDiv100`

Year in the ISO week date, divided by 100 (FW=PW=2). Implies the non-negative year.

###### `IsoYearMod100`

Year in the ISO week date, modulo 100 (FW=PW=2). Cannot be negative.

###### `Quarter`

Quarter (FW=PW=1).

###### `Month`

Month (FW=PW=2).

###### `Day`

Day of the month (FW=PW=2).

###### `WeekFromSun`

Week number, where the week 1 starts at the first Sunday of January (FW=PW=2).

###### `WeekFromMon`

Week number, where the week 1 starts at the first Monday of January (FW=PW=2).

###### `IsoWeek`

Week number in the ISO week date (FW=PW=2).

###### `NumDaysFromSun`

Day of the week, where Sunday = 0 and Saturday = 6 (FW=PW=1).

###### `WeekdayFromMon`

Day of the week, where Monday = 1 and Sunday = 7 (FW=PW=1).

###### `Ordinal`

Day of the year (FW=PW=3).

###### `Hour`

Hour number in the 24-hour clocks (FW=PW=2).

###### `Hour12`

Hour number in the 12-hour clocks (FW=PW=2).

###### `Minute`

The number of minutes since the last whole hour (FW=PW=2).

###### `Second`

The number of seconds since the last whole minute (FW=PW=2).

###### `Nanosecond`

The number of nanoseconds since the last whole second (FW=PW=9).
Note that this is *not* left-aligned;
see also [`Fixed::Nanosecond`](./enum.Fixed.html#variant.Nanosecond).

###### `Timestamp`

The number of non-leap seconds since the midnight UTC on January 1, 1970 (FW=1, PW=∞).
For formatting, it assumes UTC upon the absence of time zone offset.

###### `Internal`

Internal uses only.

This item exists so that one can add additional internal-only formatting
without breaking major compatibility (as enum variants cannot be selectively private).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `InternalNumeric` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Numeric) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Numeric { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
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

- **Sync**
- **StructuralPartialEq**
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

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `InternalNumeric`

An opaque type representing numeric item types for internal uses only.

```rust
pub struct InternalNumeric {
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &InternalNumeric) -> bool { /* ... */ }
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

- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **RefUnwindSafe**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> InternalNumeric { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
#### Enum `Fixed`

**Attributes:**

- `#[non_exhaustive]`

Fixed-format item types.

They have their own rules of formatting and parsing.
Otherwise noted, they print in the specified cases but parse case-insensitively.

```rust
pub enum Fixed {
    ShortMonthName,
    LongMonthName,
    ShortWeekdayName,
    LongWeekdayName,
    LowerAmPm,
    UpperAmPm,
    Nanosecond,
    Nanosecond3,
    Nanosecond6,
    Nanosecond9,
    TimezoneName,
    TimezoneOffsetColon,
    TimezoneOffsetDoubleColon,
    TimezoneOffsetTripleColon,
    TimezoneOffsetColonZ,
    TimezoneOffset,
    TimezoneOffsetZ,
    RFC2822,
    RFC3339,
    Internal(InternalFixed),
}
```

##### Variants

###### `ShortMonthName`

Abbreviated month names.

Prints a three-letter-long name in the title case, reads the same name in any case.

###### `LongMonthName`

Full month names.

Prints a full name in the title case, reads either a short or full name in any case.

###### `ShortWeekdayName`

Abbreviated day of the week names.

Prints a three-letter-long name in the title case, reads the same name in any case.

###### `LongWeekdayName`

Full day of the week names.

Prints a full name in the title case, reads either a short or full name in any case.

###### `LowerAmPm`

AM/PM.

Prints in lower case, reads in any case.

###### `UpperAmPm`

AM/PM.

Prints in upper case, reads in any case.

###### `Nanosecond`

An optional dot plus one or more digits for left-aligned nanoseconds.
May print nothing, 3, 6 or 9 digits according to the available accuracy.
See also [`Numeric::Nanosecond`](./enum.Numeric.html#variant.Nanosecond).

###### `Nanosecond3`

Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 3.

###### `Nanosecond6`

Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 6.

###### `Nanosecond9`

Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 9.

###### `TimezoneName`

Timezone name.

It does not support parsing, its use in the parser is an immediate failure.

###### `TimezoneOffsetColon`

Offset from the local time to UTC (`+09:00` or `-04:00` or `+00:00`).

In the parser, the colon can be omitted and/or surrounded with any amount of whitespace.
The offset is limited from `-24:00` to `+24:00`,
which is the same as [`FixedOffset`](../offset/struct.FixedOffset.html)'s range.

###### `TimezoneOffsetDoubleColon`

Offset from the local time to UTC with seconds (`+09:00:00` or `-04:00:00` or `+00:00:00`).

In the parser, the colon can be omitted and/or surrounded with any amount of whitespace.
The offset is limited from `-24:00:00` to `+24:00:00`,
which is the same as [`FixedOffset`](../offset/struct.FixedOffset.html)'s range.

###### `TimezoneOffsetTripleColon`

Offset from the local time to UTC without minutes (`+09` or `-04` or `+00`).

In the parser, the colon can be omitted and/or surrounded with any amount of whitespace.
The offset is limited from `-24` to `+24`,
which is the same as [`FixedOffset`](../offset/struct.FixedOffset.html)'s range.

###### `TimezoneOffsetColonZ`

Offset from the local time to UTC (`+09:00` or `-04:00` or `Z`).

In the parser, the colon can be omitted and/or surrounded with any amount of whitespace,
and `Z` can be either in upper case or in lower case.
The offset is limited from `-24:00` to `+24:00`,
which is the same as [`FixedOffset`](../offset/struct.FixedOffset.html)'s range.

###### `TimezoneOffset`

Same as [`TimezoneOffsetColon`](#variant.TimezoneOffsetColon) but prints no colon.
Parsing allows an optional colon.

###### `TimezoneOffsetZ`

Same as [`TimezoneOffsetColonZ`](#variant.TimezoneOffsetColonZ) but prints no colon.
Parsing allows an optional colon.

###### `RFC2822`

RFC 2822 date and time syntax. Commonly used for email and MIME date and time.

###### `RFC3339`

RFC 3339 & ISO 8601 date and time syntax.

###### `Internal`

Internal uses only.

This item exists so that one can add additional internal-only formatting
without breaking major compatibility (as enum variants cannot be selectively private).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `InternalFixed` |  |

##### Implementations

###### Trait Implementations

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Fixed { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Fixed) -> bool { /* ... */ }
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

- **StructuralPartialEq**
#### Struct `InternalFixed`

An opaque type representing fixed-format item types for internal uses only.

```rust
pub struct InternalFixed {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

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

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &InternalFixed) -> bool { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> InternalFixed { /* ... */ }
    ```

- **Eq**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Struct `OffsetFormat`

Type for specifying the format of UTC offsets.

```rust
pub struct OffsetFormat {
    pub precision: OffsetPrecision,
    pub colons: Colons,
    pub allow_zulu: bool,
    pub padding: Pad,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `precision` | `OffsetPrecision` | See `OffsetPrecision`. |
| `colons` | `Colons` | Separator between hours, minutes and seconds. |
| `allow_zulu` | `bool` | Represent `+00:00` as `Z`. |
| `padding` | `Pad` | Pad the hour value to two digits. |

##### Implementations

###### Trait Implementations

- **Sync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Freeze**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OffsetFormat { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OffsetFormat) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
#### Enum `OffsetPrecision`

The precision of an offset from UTC formatting item.

```rust
pub enum OffsetPrecision {
    Hours,
    Minutes,
    Seconds,
    OptionalMinutes,
    OptionalSeconds,
    OptionalMinutesAndSeconds,
}
```

##### Variants

###### `Hours`

Format offset from UTC as only hours. Not recommended, it is not uncommon for timezones to
have an offset of 30 minutes, 15 minutes, etc.
Any minutes and seconds get truncated.

###### `Minutes`

Format offset from UTC as hours and minutes.
Any seconds will be rounded to the nearest minute.

###### `Seconds`

Format offset from UTC as hours, minutes and seconds.

###### `OptionalMinutes`

Format offset from UTC as hours, and optionally with minutes.
Any seconds will be rounded to the nearest minute.

###### `OptionalSeconds`

Format offset from UTC as hours and minutes, and optionally seconds.

###### `OptionalMinutesAndSeconds`

Format offset from UTC as hours and optionally minutes and seconds.

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OffsetPrecision) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OffsetPrecision { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
- **Copy**
- **Unpin**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Enum `Colons`

The separator between hours and minutes in an offset.

```rust
pub enum Colons {
    None,
    Colon,
    Maybe,
}
```

##### Variants

###### `None`

No separator

###### `Colon`

Colon (`:`) as separator

###### `Maybe`

No separator when formatting, colon allowed when parsing.

##### Implementations

###### Trait Implementations

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Colons { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Colons) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Eq**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Enum `Item`

A single formatting item. This is used for both formatting and parsing.

```rust
pub enum Item<''a> {
    Literal(&''a str),
    Space(&''a str),
    Numeric(Numeric, Pad),
    Fixed(Fixed),
    Error,
}
```

##### Variants

###### `Literal`

A literally printed and parsed text.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

###### `Space`

Whitespace. Prints literally but reads zero or more whitespace.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

###### `Numeric`

Numeric item. Can be optionally padded to the maximal length (if any) when formatting;
the parser simply ignores any padded whitespace and zeroes.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Numeric` |  |
| 1 | `Pad` |  |

###### `Fixed`

Fixed-format item.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Fixed` |  |

###### `Error`

Issues a formatting error. Used to signal an invalid format string.

##### Implementations

###### Trait Implementations

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Item<''a>) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Item<''a> { /* ... */ }
    ```

- **Sync**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
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

#### Struct `ParseError`

An error from the `parse` function.

```rust
pub struct ParseError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn kind(self: &Self) -> ParseErrorKind { /* ... */ }
  ```
  The category of parse error

###### Trait Implementations

- **Eq**
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
    fn clone(self: &Self) -> ParseError { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ParseError) -> bool { /* ... */ }
    ```

- **Send**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **RefUnwindSafe**
- **Unpin**
#### Enum `ParseErrorKind`

**Attributes:**

- `#[allow(clippy::manual_non_exhaustive)]`

The category of parse error

```rust
pub enum ParseErrorKind {
    OutOfRange,
    Impossible,
    NotEnough,
    Invalid,
    TooShort,
    TooLong,
    BadFormat,
    // Some variants omitted
}
```

##### Variants

###### `OutOfRange`

Given field is out of permitted range.

###### `Impossible`

There is no possible date and time value with given set of fields.

This does not include the out-of-range conditions, which are trivially invalid.
It includes the case that there are one or more fields that are inconsistent to each other.

###### `NotEnough`

Given set of fields is not enough to make a requested date and time value.

Note that there *may* be a case that given fields constrain the possible values so much
that there is a unique possible value. Chrono only tries to be correct for
most useful sets of fields however, as such constraint solving can be expensive.

###### `Invalid`

The input string has some invalid character sequence for given formatting items.

###### `TooShort`

The input string has been prematurely ended.

###### `TooLong`

All formatting items have been read but there is a remaining input.

###### `BadFormat`

There was an error on the formatting string, or there were non-supported formatting items.

*Note: Some variants have been omitted because they are private or hidden.*

##### Implementations

###### Trait Implementations

- **Send**
- **Unpin**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParseErrorKind { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ParseErrorKind) -> bool { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Type Alias `ParseResult`

Same as `Result<T, ParseError>`.

```rust
pub type ParseResult<T> = Result<T, ParseError>;
```

### Re-exports

#### Re-export `SecondsFormat`

```rust
pub use formatting::SecondsFormat;
```

#### Re-export `parse`

```rust
pub use parse::parse;
```

#### Re-export `parse_and_remainder`

```rust
pub use parse::parse_and_remainder;
```

#### Re-export `Parsed`

```rust
pub use parsed::Parsed;
```

#### Re-export `StrftimeItems`

```rust
pub use strftime::StrftimeItems;
```

## Module `naive`

Date and time types unconcerned with timezones.

They are primarily building blocks for other types
(e.g. [`TimeZone`](../offset/trait.TimeZone.html)),
but can be also used for the simpler date and time handling.

```rust
pub mod naive { /* ... */ }
```

### Types

#### Struct `NaiveWeek`

A week represented by a [`NaiveDate`] and a [`Weekday`] which is the first
day of the week.

```rust
pub struct NaiveWeek {
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
  pub const fn first_day(self: &Self) -> NaiveDate { /* ... */ }
  ```
  Returns a date representing the first day of the week.

- ```rust
  pub const fn checked_first_day(self: &Self) -> Option<NaiveDate> { /* ... */ }
  ```
  Returns a date representing the first day of the week or

- ```rust
  pub const fn last_day(self: &Self) -> NaiveDate { /* ... */ }
  ```
  Returns a date representing the last day of the week.

- ```rust
  pub const fn checked_last_day(self: &Self) -> Option<NaiveDate> { /* ... */ }
  ```
  Returns a date representing the last day of the week or

- ```rust
  pub const fn days(self: &Self) -> RangeInclusive<NaiveDate> { /* ... */ }
  ```
  Returns a [`RangeInclusive<T>`] representing the whole week bounded by

- ```rust
  pub const fn checked_days(self: &Self) -> Option<RangeInclusive<NaiveDate>> { /* ... */ }
  ```
  Returns an [`Option<RangeInclusive<T>>`] representing the whole week bounded by

###### Trait Implementations

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &NaiveWeek) -> bool { /* ... */ }
    ```

- **Copy**
- **Eq**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> NaiveWeek { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
#### Struct `Days`

A duration in calendar days.

This is useful because when using `TimeDelta` it is possible that adding `TimeDelta::days(1)`
doesn't increment the day value as expected due to it being a fixed number of seconds. This
difference applies only when dealing with `DateTime<TimeZone>` data types and in other cases
`TimeDelta::days(n)` and `Days::new(n)` are equivalent.

```rust
pub struct Days(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn new(num: u64) -> Self { /* ... */ }
  ```
  Construct a new `Days` from a number of days

###### Trait Implementations

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Days) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **Sub**
  - ```rust
    fn sub(self: Self, days: Days) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, days: Days) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, days: Days) -> <Self as >::Output { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Days) -> $crate::cmp::Ordering { /* ... */ }
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
    fn eq(self: &Self, other: &Days) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, days: Days) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, days: Days) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, days: Days) -> <Self as >::Output { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Days { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
### Re-exports

#### Re-export `MAX_DATE`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use self::date::MAX_DATE;
```

#### Re-export `MIN_DATE`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use self::date::MIN_DATE;
```

#### Re-export `NaiveDate`

```rust
pub use self::date::NaiveDate;
```

#### Re-export `NaiveDateDaysIterator`

```rust
pub use self::date::NaiveDateDaysIterator;
```

#### Re-export `NaiveDateWeeksIterator`

```rust
pub use self::date::NaiveDateWeeksIterator;
```

#### Re-export `MAX_DATETIME`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use self::datetime::MAX_DATETIME;
```

#### Re-export `MIN_DATETIME`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use self::datetime::MIN_DATETIME;
```

#### Re-export `NaiveDateTime`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use self::datetime::NaiveDateTime;
```

#### Re-export `IsoWeek`

```rust
pub use self::isoweek::IsoWeek;
```

#### Re-export `NaiveTime`

```rust
pub use self::time::NaiveTime;
```

## Module `offset`

The time zone, which calculates offsets from the local time to UTC.

There are four operations provided by the `TimeZone` trait:

1. Converting the local `NaiveDateTime` to `DateTime<Tz>`
2. Converting the UTC `NaiveDateTime` to `DateTime<Tz>`
3. Converting `DateTime<Tz>` to the local `NaiveDateTime`
4. Constructing `DateTime<Tz>` objects from various offsets

1 is used for constructors. 2 is used for the `with_timezone` method of date and time types.
3 is used for other methods, e.g. `year()` or `format()`, and provided by an associated type
which implements `Offset` (which then passed to `TimeZone` for actual implementations).
Technically speaking `TimeZone` has a total knowledge about given timescale,
but `Offset` is used as a cache to avoid the repeated conversion
and provides implementations for 1 and 3.
An `TimeZone` instance can be reconstructed from the corresponding `Offset` instance.

```rust
pub mod offset { /* ... */ }
```

### Types

#### Type Alias `MappedLocalTime`

The result of mapping a local time to a concrete instant in a given time zone.

The calculation to go from a local time (wall clock time) to an instant in UTC can end up in
three cases:
* A single, simple result.
* An ambiguous result when the clock is turned backwards during a transition due to for example
  DST.
* No result when the clock is turned forwards during a transition due to for example DST.

When the clock is turned backwards it creates a _fold_ in local time, during which the local
time is _ambiguous_. When the clock is turned forwards it creates a _gap_ in local time, during
which the local time is _missing_, or does not exist.

Chrono does not return a default choice or invalid data during time zone transitions, but has
the `MappedLocalTime` type to help deal with the result correctly.

The type of `T` is usually a [`DateTime`] but may also be only an offset.

```rust
pub type MappedLocalTime<T> = LocalResult<T>;
```

#### Enum `LocalResult`

Old name of [`MappedLocalTime`]. See that type for more documentation.

```rust
pub enum LocalResult<T> {
    Single(T),
    Ambiguous(T, T),
    None,
}
```

##### Variants

###### `Single`

The local time maps to a single unique result.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

###### `Ambiguous`

The local time is _ambiguous_ because there is a _fold_ in the local time.

This variant contains the two possible results, in the order `(earliest, latest)`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |
| 1 | `T` |  |

###### `None`

The local time does not exist because there is a _gap_ in the local time.

This variant may also be returned if there was an error while resolving the local time,
caused by for example missing time zone data files, an error in an OS API, or overflow.

##### Implementations

###### Methods

- ```rust
  pub fn single(self: Self) -> Option<T> { /* ... */ }
  ```
  Returns `Some` if the time zone mapping has a single result.

- ```rust
  pub fn earliest(self: Self) -> Option<T> { /* ... */ }
  ```
  Returns the earliest possible result of the time zone mapping.

- ```rust
  pub fn latest(self: Self) -> Option<T> { /* ... */ }
  ```
  Returns the latest possible result of the time zone mapping.

- ```rust
  pub fn map<U, F: FnMut(T) -> U>(self: Self, f: F) -> MappedLocalTime<U> { /* ... */ }
  ```
  Maps a `MappedLocalTime<T>` into `MappedLocalTime<U>` with given function.

- ```rust
  pub fn and_time(self: Self, time: NaiveTime) -> MappedLocalTime<DateTime<Tz>> { /* ... */ }
  ```
  Makes a new `DateTime` from the current date and given `NaiveTime`.

- ```rust
  pub fn and_hms_opt(self: Self, hour: u32, min: u32, sec: u32) -> MappedLocalTime<DateTime<Tz>> { /* ... */ }
  ```
  Makes a new `DateTime` from the current date, hour, minute and second.

- ```rust
  pub fn and_hms_milli_opt(self: Self, hour: u32, min: u32, sec: u32, milli: u32) -> MappedLocalTime<DateTime<Tz>> { /* ... */ }
  ```
  Makes a new `DateTime` from the current date, hour, minute, second and millisecond.

- ```rust
  pub fn and_hms_micro_opt(self: Self, hour: u32, min: u32, sec: u32, micro: u32) -> MappedLocalTime<DateTime<Tz>> { /* ... */ }
  ```
  Makes a new `DateTime` from the current date, hour, minute, second and microsecond.

- ```rust
  pub fn and_hms_nano_opt(self: Self, hour: u32, min: u32, sec: u32, nano: u32) -> MappedLocalTime<DateTime<Tz>> { /* ... */ }
  ```
  Makes a new `DateTime` from the current date, hour, minute, second and nanosecond.

- ```rust
  pub fn unwrap(self: Self) -> T { /* ... */ }
  ```
  Returns a single unique conversion result or panics.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LocalResult<T> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LocalResult<T>) -> bool { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
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

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
### Traits

#### Trait `Offset`

The offset from the local time to UTC.

```rust
pub trait Offset: Sized + Clone + fmt::Debug {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `fix`: Returns the fixed offset from UTC to the local time stored.

##### Implementations

This trait is implemented for the following types:

- `FixedOffset`
- `Utc`

#### Trait `TimeZone`

The time zone.

The methods here are the primary constructors for the [`DateTime`] type.

```rust
pub trait TimeZone: Sized + Clone {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Offset`: An associated offset type.

###### Required Methods

- `from_offset`: Reconstructs the time zone from the offset.
- `offset_from_local_date`: Creates the offset(s) for given local `NaiveDate` if possible.
- `offset_from_local_datetime`: Creates the offset(s) for given local `NaiveDateTime` if possible.
- `offset_from_utc_date`: Creates the offset for given UTC `NaiveDate`. This cannot fail.
- `offset_from_utc_datetime`: Creates the offset for given UTC `NaiveDateTime`. This cannot fail.

##### Provided Methods

- ```rust
  fn with_ymd_and_hms(self: &Self, year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> MappedLocalTime<DateTime<Self>> { /* ... */ }
  ```
  Make a new `DateTime` from year, month, day, time components and current time zone.

- ```rust
  fn ymd(self: &Self, year: i32, month: u32, day: u32) -> Date<Self> { /* ... */ }
  ```
  Makes a new `Date` from year, month, day and the current time zone.

- ```rust
  fn ymd_opt(self: &Self, year: i32, month: u32, day: u32) -> MappedLocalTime<Date<Self>> { /* ... */ }
  ```
  Makes a new `Date` from year, month, day and the current time zone.

- ```rust
  fn yo(self: &Self, year: i32, ordinal: u32) -> Date<Self> { /* ... */ }
  ```
  Makes a new `Date` from year, day of year (DOY or "ordinal") and the current time zone.

- ```rust
  fn yo_opt(self: &Self, year: i32, ordinal: u32) -> MappedLocalTime<Date<Self>> { /* ... */ }
  ```
  Makes a new `Date` from year, day of year (DOY or "ordinal") and the current time zone.

- ```rust
  fn isoywd(self: &Self, year: i32, week: u32, weekday: Weekday) -> Date<Self> { /* ... */ }
  ```
  Makes a new `Date` from ISO week date (year and week number), day of the week (DOW) and

- ```rust
  fn isoywd_opt(self: &Self, year: i32, week: u32, weekday: Weekday) -> MappedLocalTime<Date<Self>> { /* ... */ }
  ```
  Makes a new `Date` from ISO week date (year and week number), day of the week (DOW) and

- ```rust
  fn timestamp(self: &Self, secs: i64, nsecs: u32) -> DateTime<Self> { /* ... */ }
  ```
  Makes a new `DateTime` from the number of non-leap seconds

- ```rust
  fn timestamp_opt(self: &Self, secs: i64, nsecs: u32) -> MappedLocalTime<DateTime<Self>> { /* ... */ }
  ```
  Makes a new `DateTime` from the number of non-leap seconds

- ```rust
  fn timestamp_millis(self: &Self, millis: i64) -> DateTime<Self> { /* ... */ }
  ```
  Makes a new `DateTime` from the number of non-leap milliseconds

- ```rust
  fn timestamp_millis_opt(self: &Self, millis: i64) -> MappedLocalTime<DateTime<Self>> { /* ... */ }
  ```
  Makes a new `DateTime` from the number of non-leap milliseconds

- ```rust
  fn timestamp_nanos(self: &Self, nanos: i64) -> DateTime<Self> { /* ... */ }
  ```
  Makes a new `DateTime` from the number of non-leap nanoseconds

- ```rust
  fn timestamp_micros(self: &Self, micros: i64) -> MappedLocalTime<DateTime<Self>> { /* ... */ }
  ```
  Makes a new `DateTime` from the number of non-leap microseconds

- ```rust
  fn datetime_from_str(self: &Self, s: &str, fmt: &str) -> ParseResult<DateTime<Self>> { /* ... */ }
  ```
  Parses a string with the specified format string and returns a

- ```rust
  fn from_local_date(self: &Self, local: &NaiveDate) -> MappedLocalTime<Date<Self>> { /* ... */ }
  ```
  Converts the local `NaiveDate` to the timezone-aware `Date` if possible.

- ```rust
  fn from_local_datetime(self: &Self, local: &NaiveDateTime) -> MappedLocalTime<DateTime<Self>> { /* ... */ }
  ```
  Converts the local `NaiveDateTime` to the timezone-aware `DateTime` if possible.

- ```rust
  fn from_utc_date(self: &Self, utc: &NaiveDate) -> Date<Self> { /* ... */ }
  ```
  Converts the UTC `NaiveDate` to the local time.

- ```rust
  fn from_utc_datetime(self: &Self, utc: &NaiveDateTime) -> DateTime<Self> { /* ... */ }
  ```
  Converts the UTC `NaiveDateTime` to the local time.

##### Implementations

This trait is implemented for the following types:

- `FixedOffset`
- `Utc`

### Re-exports

#### Re-export `FixedOffset`

```rust
pub use self::fixed::FixedOffset;
```

#### Re-export `Utc`

```rust
pub use self::utc::Utc;
```

## Module `round`

Functionality for rounding or truncating a `DateTime` by a `TimeDelta`.

```rust
pub mod round { /* ... */ }
```

### Types

#### Enum `RoundingError`

An error from rounding by `TimeDelta`

See: [`DurationRound`]

```rust
pub enum RoundingError {
    DurationExceedsTimestamp,
    DurationExceedsLimit,
    TimestampExceedsLimit,
}
```

##### Variants

###### `DurationExceedsTimestamp`

Error when the TimeDelta exceeds the TimeDelta from or until the Unix epoch.

Note: this error is not produced anymore.

###### `DurationExceedsLimit`

Error when `TimeDelta.num_nanoseconds` exceeds the limit.

``` rust
# use chrono::{DurationRound, TimeDelta, RoundingError, NaiveDate};
let dt = NaiveDate::from_ymd_opt(2260, 12, 31)
    .unwrap()
    .and_hms_nano_opt(23, 59, 59, 1_75_500_000)
    .unwrap()
    .and_utc();

assert_eq!(
    dt.duration_round(TimeDelta::try_days(300 * 365).unwrap()),
    Err(RoundingError::DurationExceedsLimit)
);
```

###### `TimestampExceedsLimit`

Error when `DateTime.timestamp_nanos` exceeds the limit.

``` rust
# use chrono::{DurationRound, TimeDelta, RoundingError, TimeZone, Utc};
let dt = Utc.with_ymd_and_hms(2300, 12, 12, 0, 0, 0).unwrap();

assert_eq!(
    dt.duration_round(TimeDelta::try_days(1).unwrap()),
    Err(RoundingError::TimestampExceedsLimit)
);
```

##### Implementations

###### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RoundingError { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RoundingError) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
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

- **Send**
- **StructuralPartialEq**
- **Freeze**
- **Unpin**
- **Sync**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `SubsecRound`

Extension trait for subsecond rounding or truncation to a maximum number
of digits. Rounding can be used to decrease the error variance when
serializing/persisting to lower precision. Truncation is the default
behavior in Chrono display formatting.  Either can be used to guarantee
equality (e.g. for testing) when round-tripping through a lower precision
format.

```rust
pub trait SubsecRound {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `round_subsecs`: Return a copy rounded to the specified number of subsecond digits. With
- `trunc_subsecs`: Return a copy truncated to the specified number of subsecond

##### Implementations

This trait is implemented for the following types:

- `T` with <T>

#### Trait `DurationRound`

Extension trait for rounding or truncating a DateTime by a TimeDelta.

# Limitations
Both rounding and truncating are done via [`TimeDelta::num_nanoseconds`] and
[`DateTime::timestamp_nanos_opt`]. This means that they will fail if either the
`TimeDelta` or the `DateTime` are too big to represented as nanoseconds. They
will also fail if the `TimeDelta` is bigger than the timestamp, negative or zero.

```rust
pub trait DurationRound: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Err`: Error that can occur in rounding or truncating

###### Required Methods

- `duration_round`: Return a copy rounded by TimeDelta.
- `duration_trunc`: Return a copy truncated by TimeDelta.
- `duration_round_up`: Return a copy rounded **up** by TimeDelta.

##### Implementations

This trait is implemented for the following types:

- `crate::DateTime<Tz>` with <Tz: TimeZone>
- `crate::NaiveDateTime`

## Types

### Type Alias `Duration`

Alias of [`TimeDelta`].

```rust
pub type Duration = TimeDelta;
```

### Struct `OutOfRange`

Out of range error type used in various converting APIs

```rust
pub struct OutOfRange {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Unpin**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
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
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OutOfRange) -> bool { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OutOfRange { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Re-exports

### Re-export `TimeDelta`

```rust
pub use time_delta::TimeDelta;
```

### Re-export `Date`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use date::Date;
```

### Re-export `MAX_DATE`

**Attributes:**

- `#[doc(no_inline)]`
- `#[allow(deprecated)]`

```rust
pub use date::MAX_DATE;
```

### Re-export `MIN_DATE`

**Attributes:**

- `#[doc(no_inline)]`
- `#[allow(deprecated)]`

```rust
pub use date::MIN_DATE;
```

### Re-export `DateTime`

```rust
pub use datetime::DateTime;
```

### Re-export `MAX_DATETIME`

**Attributes:**

- `#[allow(deprecated)]`
- `#[doc(no_inline)]`

```rust
pub use datetime::MAX_DATETIME;
```

### Re-export `MIN_DATETIME`

**Attributes:**

- `#[allow(deprecated)]`
- `#[doc(no_inline)]`

```rust
pub use datetime::MIN_DATETIME;
```

### Re-export `ParseError`

```rust
pub use format::ParseError;
```

### Re-export `ParseResult`

```rust
pub use format::ParseResult;
```

### Re-export `SecondsFormat`

```rust
pub use format::SecondsFormat;
```

### Re-export `Days`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use naive::Days;
```

### Re-export `NaiveDate`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use naive::NaiveDate;
```

### Re-export `NaiveDateTime`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use naive::NaiveDateTime;
```

### Re-export `NaiveTime`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use naive::NaiveTime;
```

### Re-export `IsoWeek`

```rust
pub use naive::IsoWeek;
```

### Re-export `NaiveWeek`

```rust
pub use naive::NaiveWeek;
```

### Re-export `MappedLocalTime`

```rust
pub use offset::MappedLocalTime;
```

### Re-export `FixedOffset`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use offset::FixedOffset;
```

### Re-export `Offset`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use offset::Offset;
```

### Re-export `TimeZone`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use offset::TimeZone;
```

### Re-export `Utc`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use offset::Utc;
```

### Re-export `DurationRound`

```rust
pub use round::DurationRound;
```

### Re-export `RoundingError`

```rust
pub use round::RoundingError;
```

### Re-export `SubsecRound`

```rust
pub use round::SubsecRound;
```

### Re-export `ParseWeekdayError`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use weekday::ParseWeekdayError;
```

### Re-export `Weekday`

```rust
pub use weekday::Weekday;
```

### Re-export `ParseMonthError`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use month::ParseMonthError;
```

### Re-export `Month`

```rust
pub use month::Month;
```

### Re-export `Months`

```rust
pub use month::Months;
```

### Re-export `Datelike`

```rust
pub use traits::Datelike;
```

### Re-export `Timelike`

```rust
pub use traits::Timelike;
```

