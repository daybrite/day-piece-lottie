<!--
Copyright © The Daybrite Project
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# day-piece-lottie

[![ci](https://github.com/daybrite/day-piece-lottie/actions/workflows/ci.yml/badge.svg)](https://github.com/daybrite/day-piece-lottie/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MPL--2.0-green.svg)](LICENSE)

Play Lottie animations in [Day](https://daybrite.dev) apps on iOS and Android, and read what is
inside a Lottie file on any platform.

The piece wraps each platform's standard player, [lottie-ios](https://github.com/airbnb/lottie-ios)
and [lottie-android](https://github.com/airbnb/lottie-android). Nothing is vendored: the crate
declares those dependencies in its metadata, and `day build` fetches them through SwiftPM and
Gradle as part of the normal build. It is also the reference for a piece that pulls third-party
native libraries, if you are planning one of your own.

## Use it

Add the dependency and build. `day` reads the piece's backend list from its metadata, so there is
no per-backend feature to name in your app.

```toml
[dependencies]
day-piece-lottie = { git = "https://github.com/daybrite/day-piece-lottie.git" }
```

```rust
use day_piece_lottie::lottie;

let speed = Signal::new(1.0);
lottie("hello")                 // plays the bundled hello.json, looping and autoplaying
    .speed(speed)               // playback rate; reactive, follows the signal live
    .frame(220.0, 220.0)        // a growing leaf, so constrain it
    .id("lottie-view")
```

`lottie(name)` loads `name.json` from the app: the iOS bundle, or Android's `assets/`. Put the
file under `resource/assets/` and `day build` bundles it (a `/` path such as
`"lottie/pin-jump"` reaches a subfolder). `.looping(false)` plays once, `.autoplay(false)` starts
paused, and `.speed(_)` takes a constant, a `Signal<f64>`, or a closure.

The name takes what `label` takes, so a reactive one swaps the animation live:

```rust
let files = ["hello", "pin-jump", "watermelon"];
let selected = Signal::new(0usize);
column((
    picker(files, selected),
    lottie(move || files[selected.get()].to_string()).frame(220.0, 220.0),
))
```

The piece has renderers for `ios-uikit` and `android-mdc`. On every other target it draws Day's
placeholder for an unrendered piece, so gate the page that shows it:

```rust
#[cfg(any(target_os = "ios", target_os = "android"))]
```

### Read the file

`model` is the headless half: it parses a Lottie document, reports its facts, and says what a
player would refuse. It runs anywhere, with no view involved.

```rust
use day_piece_lottie::LottieModel;

let model = LottieModel::parse(include_str!("../resource/assets/hello.json"))?;
model.frame_rate;        // 30.0
model.frames();          // 60.0, out point minus in point
model.duration_secs();   // 2.0
model.layers.len();      // 1
model.verify();          // Vec<Issue>: empty when a player will accept the file
```

## Compatibility

| This crate | Tested against day | Toolkits |
|---|---|---|
| 0.1 | 0.4 (`main` at the revision in `demo/Cargo.lock`) | `ios-uikit`, `android-mdc` |

Every day dependency names the bare canonical URL with no branch or tag, and your app's
`Cargo.lock` picks one day revision for the whole graph. Cargo unifies a git dependency only when
URL and ref match, so a crate that pinned a tag would double every day crate in an app on `main`.
`[package.metadata.day] compat = "0.4"` records the minor this release was tested against, and
`day build` notes a mismatch before compiling.

To build against a fork of day, patch the canonical URL once in your app and this crate follows:

```sh
day patch --git https://github.com/acme/day.git@acme
```

## Develop it

```sh
cargo test                                            # the headless reader, on the host
cd demo && day launch -p ios-uikit --script dayscript/lottie.yaml
cd demo && day launch -p android-mdc --script dayscript/lottie.yaml
```

The [demo app](demo/) depends on this crate by path and its walkthrough asserts the reader's
answers on device; CI runs it on the iOS Simulator and the Android emulator on every push, and
daily against day's newest `main`. To work against a local day checkout, `day patch --local
../day` in either directory writes a gitignored patch table.

Extending Day is documented at [daybrite.dev/docs/extending](https://daybrite.dev/docs/extending);
[docs/lottie.md](docs/lottie.md) covers this piece's native halves.

## Part of Day

This crate is one piece of [Day](https://daybrite.dev), a Rust framework for building apps out of
each platform's own widgets — AppKit, UIKit, Android's Material widgets, GTK 4, Qt 6, XAML, and
ArkUI — from one codebase. When you write `button("Save")`, macOS shows an `NSButton` and Android
shows a Material button.

New to Day? Start at [daybrite.dev](https://daybrite.dev), or browse the
[source repository](https://github.com/daybrite/day).
