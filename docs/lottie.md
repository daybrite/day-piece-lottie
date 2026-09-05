---
title: "Lottie"
description: "Lottie vector animations as an external piece on iOS and Android."
---

<!--
Copyright © The Daybrite Project
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Lottie (external piece, iOS + Android)

> **Status: shipped**, as `day-piece-lottie` in this repository. It is the reference for a piece
> that pulls an external native package on each platform: the lottie-ios SwiftPM package on iOS
> (via `[package.metadata.day.ios]`) and `com.airbnb.android:lottie` on Android. It is also the
> first piece to live outside the `daybrite/day` tree; the [README](../README.md) covers how it
> depends on day and how the `demo/` app tests it on device.

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
the Android `assets/`. `.looping(false)` plays once; `.autoplay(false)` starts paused. `.speed(_)` sets
the playback-rate multiplier (1.0 = normal, 2.0 = double, 0.5 = half) and takes any `IntoReactive<f64>`:
a constant, a `Signal<f64>`, or a `Fn() -> f64`. A reactive value updates the native view's speed live
(the showcase binds it to a slider). `Lottie` implements `Piece`, so `.id()/.a11y()/.frame()` chain via
`Decorate`.

The showcase's `lottie` page is `#[cfg(any(target_os = "ios", target_os = "android"))]`, so the nav item
appears only on those builds; the bundled `hello.json` (this repository's `demo/resource/assets/`, and
`Day-Showcase/resource/assets/`) is a small hand-authored animation (a rotating, pulsing rounded square).

## Per-platform native realization

| | iOS (UIKit) | Android |
|---|---|---|
| control | `LottieAnimationView` (lottie-ios) | `LottieAnimationView` (lottie-android) |
| dependency | SwiftPM `github.com/airbnb/lottie-ios` | Gradle `com.airbnb.android:lottie` |
| declared in | `[package.metadata.day.ios].swift-packages` | `[package.metadata.day.android].gradle-dependencies` |
| shim | `ios/swift/DayLottie.swift` (`@_cdecl`) | `android/java/…/DayLottie.java` (static method) |

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

## The headless half: `model`

`LottieModel` reads a document without a view: `parse(&str)` / `from_slice(&[u8])` take the
top-level facts (`v`, `nm`, `fr`, `ip`, `op`, `w`, `h`, `ddd`), each layer's index, name, type,
timing, and `refId`, the asset ids, and the markers, and ignore the rest so newer exporters parse.
`frames()` and `duration_secs()` derive; `verify()` returns the `Issue`s a player would refuse
(no frames, a zero frame rate, no size, no layers, a missing asset, an unknown layer type, a layer
that ends before it starts), each with a one-sentence `Display`. It is behind no feature, so it
compiles for every target; `tests/model.rs` covers it on the host and `demo/dayscript/lottie.yaml`
asserts the same answers on device.
