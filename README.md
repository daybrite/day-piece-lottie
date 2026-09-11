<!--
Copyright © The Daybrite Project
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# day-piece-lottie

[![ci](https://github.com/daybrite/day-piece-lottie/actions/workflows/ci.yml/badge.svg)](https://github.com/daybrite/day-piece-lottie/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MPL--2.0-green.svg)](LICENSE)

## Overview and capabilities

`day-piece-lottie` plays bundled Lottie JSON animations in Day apps on iOS and Android.
Lottie is an animation format commonly exported from motion-design tools.

[Day](https://github.com/daybrite/day) is a Rust framework for building applications
from a shared codebase using each platform's native UI toolkit. A **piece** is a UI
component you place in a layout; a **part** provides a capability without drawing UI.
Day's `day` command builds and packages the Rust code, resources, and native platform
code together. `Cargo.toml` declares Rust dependencies; `Day.toml` configures the app
and its target platforms.

The `lottie()` piece provides autoplay, looping, playback speed, and live switching
between bundled animations. Its separate `LottieModel` API reads animation metadata
and checks common document problems without creating a view, on any target.

## Platform support and limitations

| Use | Supported targets | Behavior |
|---|---|---|
| Animation playback | `ios-uikit`, `android-mdc` | Uses Airbnb's native Lottie player on each platform. |
| Animation playback elsewhere | Desktop, HarmonyOS, web, mock | No renderer; Day displays a placeholder. Omit the animation or provide alternative UI. |
| Metadata parsing and verification | All Rust targets supported by the crate | No native player or backend feature required. |

Only the animation name and speed update reactively. Looping and autoplay are fixed
when the piece is built; the public API has no seek, completion callback, or separate
play/pause command. The player loads bundled JSON by name, not remote URLs.

`verify()` checks a subset of document structure, not complete player compatibility.
An empty issue list does not guarantee identical rendering on iOS and Android.
On iOS, avoid driving speed updates faster than approximately one display frame:
the shim preserves playback progress while changing speed, and rapid updates can
observe invalid intermediate progress. A stepped speed control is appropriate.
Android requires AndroidX (`android.useAndroidX=true` in a custom host project).

## Add it to a Day project

In an existing Day app's `Cargo.toml`, add:

```toml
[dependencies]
day-piece-lottie = { git = "https://github.com/daybrite/day-piece-lottie.git" }
```

Put an animation at `resource/assets/hello.json`. Return the piece from your UI
function or include it in a layout:

```rust
use day::prelude::*;
use day_piece_lottie::lottie;

fn animation_panel() -> impl Piece {
    let speed = Signal::new(1.0);
    lottie("hello")
        .speed(speed)
        .looping(true)
        .autoplay(true)
        .frame(220.0, 220.0)
}
```

A `Signal` is observable state; updating `speed` from a control changes the native
player's rate. The defaults are looping, autoplay, and speed `1.0`. A string signal
or closure passed as the name switches files live. Use `"lottie/hello"` for
`resource/assets/lottie/hello.json`; leave off the `.json` extension.

Configure `ios-uikit` or `android-mdc` in the app's `Day.toml` and use its normal Day
backend features (see [demo/Cargo.toml](demo/Cargo.toml)). Run
`day build -p android-mdc` or `day launch -p ios-uikit` with the platform SDK installed.
Day enables the piece's matching backend feature, bundles assets, and incorporates
its SwiftPM/Gradle dependencies. A plain Cargo build does not perform that packaging.

For an app with other targets, conditionally include the animation UI on iOS and
Android, for example with `#[cfg(any(target_os = "ios", target_os = "android"))]`
on the relevant UI function and matching call sites. Model reading needs no such gate:

```rust
use day_piece_lottie::{LottieError, LottieModel};

fn inspect_animation(json: &str) -> Result<(), LottieError> {
    let model = LottieModel::parse(json)?;
    println!("{} seconds, {} layers", model.duration_secs(), model.layers.len());
    for issue in model.verify() {
        println!("{issue}");
    }
    Ok(())
}
```

## Architecture and dependencies

[src/lib.rs](src/lib.rs) implements Day's `Piece` trait and creates a leaf with
`LottieProps`. Bindings turn changed name/speed values into `LottiePatch` updates
without replacing the view. Backend adapters in `src/lib-uikit.rs` and
`src/lib-android.rs` register through `linkme` at link time.

On iOS, Rust calls a Swift shim through a C interface and owns the returned UIKit
view. The shim resolves Day's bundled asset path and wraps `LottieAnimationView`.
On Android, the Rust adapter calls a Java shim through JNI to create and update the
Android view. Native sources live under [platform/](platform/); Day discovers them
from package metadata, including the external native libraries.

| Dependency group | What it brings in |
|---|---|
| Shared Rust | `day-core`, `day-spec`, `day-pieces`, and `day-reactive` provide the tree, common types, builders, and bindings. `linkme` 0.3 registers renderers; `log` 0.4 provides diagnostics. |
| Model reader | `serde_json` 1 parses the JSON document. |
| iOS feature `uikit` | `day-uikit`, `objc2` 0.6, `objc2-foundation` 0.3, and `objc2-ui-kit` 0.3; SwiftPM adds the `Lottie` product from `airbnb/lottie-ios`, with a compatible version starting at 4.5.0. |
| Android feature `mdc` | `day-android`; Gradle adds `com.airbnb.android:lottie:6.6.0` and its transitive Android dependencies. |
| Tests | `day-mock` for host-side piece tests. |

[src/model.rs](src/model.rs) extracts timing, dimensions, layers, asset references,
and markers while ignoring unrecognized fields. Verification catches missing
frames, invalid frame rate or dimensions, empty layers, missing assets, unknown
layer types, and invalid layer timing. It does not render or fully decode animation
shapes. See [the implementation notes](docs/lottie.md) for the native boundary.

## Compatibility and development

This checkout requires Rust 1.89 or newer and declares compatibility with Day 0.4 in
[Cargo.toml](Cargo.toml). The crate is consumed from Git, not crates.io. Its Day
dependencies use `https://github.com/daybrite/day.git` without a branch, tag, or
revision. Use the same source in your app and keep its `Cargo.lock` to record the
resolved revisions. Mixing Day source URLs or refs can introduce duplicate framework
crates and incompatible types.

For a local framework checkout, run `day patch --local ../day` from this repository
(adjust the path when running from `demo/`). The [demo](demo/) depends on this crate
by path and is a complete integration example.

Run `cargo test` for the model and host checks. From `demo/`, run
`day launch -p ios-uikit --script dayscript/lottie.yaml` or the same command with
`-p android-mdc`. Inspect playback on both platforms for the animations you ship.
