---
title: "Lottie"
description: "Lottie vector animations as an external piece, on every platform Day builds for."
---

<!--
Copyright © The Daybrite Project
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Lottie (external piece)

> **Status: shipped**, as `day-piece-lottie` in this repository. It is the reference for a piece
> that pulls an external native package on each platform: the lottie-ios SwiftPM package on iOS
> (via `[package.metadata.day.ios]`) and `com.airbnb.android:lottie` on Android. It is also the
> first piece to live outside the `daybrite/day` tree; the [README](../README.md) covers how it
> depends on day and how the `demo/` app tests it on device.
>
> Since the web-view arm below, it is also the reference for a piece **composed out of another
> piece**: where there is no native Lottie player, it plays the same file with lottie-web inside
> a `day-piece-webview`, and the app above it writes the same line either way.

## Authoring

```rust
use day_piece_lottie::lottie;

let speed = Signal::new(1.0);
lottie("hello")                 // renders the bundled hello.json (looping, autoplaying)
    .speed(speed)               // playback rate; reactive, follows the signal live
    .frame(220.0, 220.0)        // it's a growing leaf, so constrain it
    .id("lottie-view")
```

`lottie(name)` loads `name`(.json), bundled with the app: the iOS app bundle (`Bundle.main`) and
the Android `assets/`. `name` is an `IntoText` like a label's: a `&str`, a `Signal<String>`, or a
closure, and a reactive one pushes a `Name` patch that loads the other file into the same view,
rewinds, and keeps playing (the Showcase's picker of sample animations). `.looping(false)` plays once; `.autoplay(false)` starts paused. `.speed(_)` sets
the playback-rate multiplier (1.0 = normal, 2.0 = double, 0.5 = half) and takes any `IntoReactive<f64>`:
a constant, a `Signal<f64>`, or a `Fn() -> f64`. A reactive value updates the native view's speed live
(the showcase binds it to a slider). `Lottie` implements `Piece`, so `.id()/.a11y()/.frame()` chain via
`Decorate`.

An app needs no `cfg` around its animation: every target plays it, one way or the other. (The
showcase's `lottie` page still carries `#[cfg(any(target_os = "ios", target_os = "android"))]`
from before the web arm, so its nav item appears only on those builds.) The bundled `hello.json`
(this repository's `demo/resource/assets/`, and `Day-Showcase/resource/assets/`) is a small
hand-authored animation: a rotating, pulsing rounded square.

## Per-platform native realization

| | iOS (UIKit) | Android |
|---|---|---|
| control | `LottieAnimationView` (lottie-ios) | `LottieAnimationView` (lottie-android) |
| dependency | SwiftPM `github.com/airbnb/lottie-ios` | Gradle `com.airbnb.android:lottie` |
| declared in | `[package.metadata.day.ios].swift-packages` | `[package.metadata.day.android].gradle-dependencies` |
| shim | `platform/ios/swift/DayLottie.swift` (`@_cdecl`) | `src/DayLottie.java` (static method) |

Both shims wrap a `LottieAnimationView` behind a flat interface the piece's Rust calls; the iOS shim
returns a `UIView` Rust wraps via `Retained::from_raw`, and the Android shim returns a `View` through JNI.

## The iOS mechanism this piece introduces

A piece can't drive a Swift library from Rust directly, and it ships as a SwiftPM package, neither of
which Day supported before. `[package.metadata.day.ios]` (see [extending.md](extending.md)) adds it: at
build time the CLI generates a local SwiftPM package (`build/day/ios/DayPieces`) whose `Package.swift`
depends on every piece's `swift-packages` and compiles every piece's staged Swift shims. The app's
`.xcodeproj` depends on that one local package, so adding an iOS piece is pure `Cargo.toml` data, with
no `.xcodeproj` edits. This mirrors the Android `day-pieces.json` → Gradle scaffold flow.

## Notes / gotchas

- **AndroidX**: Lottie's `LottieAnimationView` extends `androidx.appcompat.widget.AppCompatImageView`,
  so the app must set `android.useAndroidX=true` (a non-fatal warning is logged about the framework
  theme not being an AppCompat theme; it renders regardless).
- **Gradle configuration cache**: the scaffold reads the generated `day-pieces.json` at configuration
  time, which the config cache can't track, so it ships disabled (otherwise a newly added piece's Gradle
  dependency is silently dropped from the build).

## Everywhere else: the web-view arm

The six backends with no Lottie library to bind all have a web engine that
[day-piece-webview](https://github.com/daybrite/day-piece-webview) already drives, and Airbnb
ships lottie-web for exactly this case. So `src/web.rs` builds one inline web view instead of a
leaf: `web/index.html` and `web/lottie.min.js` (MIT, vendored) are staged into every app's bundle
by `[package.metadata.day.piece].assets = ["web"]`, under the crate's own name, and the page
plays what it is given.

How the animation reaches the page is where the engines differ:

| | Told | Fetches |
|---|---|---|
| backends | `macos-appkit`, `macos-qt`, `linux-qt`, `windows-xaml`, `windows-qt`, `harmony-arkui` | `linux-gtk`, `web-dom` |
| how | this side reads the file through Day's resource opener and evaluates `dayLottie.loadData(<the file's text>, …)` in the page | the name rides in the page's query string and the page fetches `../<name>.json` itself |
| why | `eval_support()` is `Native` there, and on WebKit it is the only way: a page opened from a `file:` URL may not read a file with `XMLHttpRequest`, not even one beside it | no JavaScript-evaluation arm yet; WebKitGTK reaches the file because the view asks for the app's assets (`app_assets`), and web-dom serves the asset tree over http |

A told page is handed its animation after the view is realized, so the arm waits for the page's
own script to answer before evaluating anything: an evaluation that lands mid-load runs in a
document that has no `window.dayLottie` yet and is lost without a trace.

What the two fetching backends give up is live updates: a name or speed that changes after the
view was built reaches the player only where the engine evaluates. Navigation rebuilds the view,
so a route per animation (what `demo/` does) behaves the same everywhere.

`macos-gtk` and `windows-gtk` have no WebKitGTK to host the player, so the view reports
`Unsupported` and realizes Day's placeholder, which is the answer `day_piece_webview` gives there
for any web view.

## The headless half: `model`

`LottieModel` reads a document without a view: `parse(&str)` / `from_slice(&[u8])` take the
top-level facts (`v`, `nm`, `fr`, `ip`, `op`, `w`, `h`, `ddd`), each layer's index, name, type,
timing, and `refId`, the asset ids, and the markers, and ignore the rest so newer exporters parse.
`frames()` and `duration_secs()` derive; `verify()` returns the `Issue`s a player would refuse
(no frames, a zero frame rate, no size, no layers, a missing asset, an unknown layer type, a layer
that ends before it starts), each with a one-sentence `Display`. It is behind no feature, so it
compiles for every target; `tests/model.rs` covers it on the host and `demo/dayscript/lottie.yaml`
asserts the same answers on device.
