<!--
Copyright © The Daybrite Project
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# day-piece-lottie

[![ci](https://github.com/daybrite/day-piece-lottie/actions/workflows/ci.yml/badge.svg)](https://github.com/daybrite/day-piece-lottie/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MPL--2.0-green.svg)](LICENSE)

## Overview and capabilities

`day-piece-lottie` plays bundled Lottie JSON animations in Day apps, on every platform Day
builds for. Lottie is an animation format commonly exported from motion-design tools.

[Day](https://github.com/daybrite/day) is a Rust framework for building applications
from a shared codebase using each platform's native UI toolkit. A **piece** is a UI
component you place in a layout; a **part** provides a capability without drawing UI.
Day's `day` command builds and packages the Rust code, resources, and native platform
code together. `Cargo.toml` declares Rust dependencies; `Day.toml` configures the app
and its target platforms.

The `lottie()` piece provides autoplay, looping, playback speed, and live switching
between bundled animations. Its separate `LottieModel` API reads animation metadata
and checks common document problems without creating a view, on any target.

The demo app plays twelve animations, one per page:
**[open it in a browser](https://daybrite.github.io/day-piece-lottie/webapp/)**, or browse
[what it looks like on each platform](https://daybrite.github.io/day-piece-lottie/gallery/)
— screenshots CI takes on all eight primary platform-toolkit pairs on every push.

<p align="center">
  <kbd><img src="https://daybrite.github.io/day-piece-lottie/gallery/web-dom/default/lottie-logo1.png" width="760" alt="The demo playing Airbnb's Lottie logo animation, one route per animation in the sidebar"></kbd>
</p>

## Platform support and limitations

Two renderers sit behind one API. iOS and Android have Airbnb's own players and this crate
binds them. Every other backend plays the same file with
[lottie-web](https://github.com/airbnb/lottie-web), bundled with this crate and shown in a web
view through [day-piece-webview](https://github.com/daybrite/day-piece-webview), which composes
the piece out of an engine each platform already ships.

| Use | Supported targets | Behavior |
|---|---|---|
| Animation playback | `ios-uikit` | [Lottie for iOS](https://github.com/airbnb/lottie-ios): SwiftPM `Lottie` product, compatible versions starting at 4.5.0. |
| Animation playback | `android-mdc` | [Lottie for Android](https://github.com/airbnb/lottie-android): Gradle `com.airbnb.android:lottie:6.6.0`. |
| Animation playback | `macos-appkit`, `macos-qt`, `linux-gtk`, `linux-qt`, `windows-xaml`, `windows-qt`, `harmony-arkui`, `web-dom` | lottie-web 5.13.0 in a web view: WebKit, WebKitGTK, QtWebEngine, WebView2, ArkWeb, or the browser's own frame. |
| Animation playback | `macos-gtk`, `windows-gtk` | No WebKitGTK on those hosts, so there is no engine to host the player and the view realizes Day's placeholder. |
| Metadata parsing and verification | All Rust targets supported by the crate | No native player or backend feature required. |

Only the animation name and speed update reactively. Looping and autoplay are fixed
when the piece is built; the public API has no seek, completion callback, or separate
play/pause command. The player loads bundled JSON by name, not remote URLs.

On the web-view backends, a name or speed that changes after the view is built reaches the
player through JavaScript evaluation, which `linux-gtk` and `web-dom` do not have yet: there the
animation and the rate are what the view was built with, and a change takes effect the next time
the piece is built, which navigating to another page does anyway. A first frame also arrives a
moment later than a native player's, because the engine loads the page first.

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

List the targets in the app's `Day.toml` and give the app the matching Day backend
features (see [demo/Cargo.toml](demo/Cargo.toml)). Run `day build -p android-mdc` or
`day launch -p macos-appkit` with that platform's SDK installed. Day enables the piece's
matching backend feature, bundles the animation and the player's own web assets, and
incorporates its SwiftPM and Gradle dependencies. A plain Cargo build does not perform that
packaging.

The same call site works on every target, so an app needs no `cfg` around its animation. Model
reading needs no backend at all:

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

Dependency links below lead to upstream source repositories or official API
documentation. Version requirements describe this checkout's [Cargo.toml](Cargo.toml),
not necessarily the newest upstream releases.

[src/lib.rs](src/lib.rs) implements Day's `Piece` trait and creates a leaf with
`LottieProps`. Bindings turn changed name/speed values into `LottiePatch` updates
without replacing the view. Backend adapters in `src/lib-uikit.rs` and
`src/lib-android.rs` register through [linkme](https://github.com/dtolnay/linkme) at link time.

On iOS, Rust calls a Swift shim through a C interface and owns the returned UIKit
view. The shim resolves Day's bundled asset path and wraps `LottieAnimationView`.
On Android, the Rust adapter calls a Java shim through JNI to create and update the
Android view. Native sources live under [platform/](platform/); Day discovers them
from package metadata, including the external native libraries.

Everywhere else, [src/web.rs](src/web.rs) builds a `day-piece-webview` inline site instead of a
leaf: the page in [web/](web/) and lottie-web beside it, staged into the app's bundle by
`[package.metadata.day.piece].assets` and opened as a local file. How the animation reaches that
page depends on the engine. Where the backend evaluates JavaScript, this side reads the file
through Day's resource opener and hands the page its text, which is also the only way that works
on WebKit — a page opened from a `file:` URL may not read a file with `XMLHttpRequest`, not even
one beside it. WebKitGTK has no evaluation arm yet, so its page is told the name and fetches the
file itself, as does the browser frame on `web-dom`, where the asset tree is served over http.

| Dependency group | What it brings in |
|---|---|
| Shared Rust | [day-core](https://github.com/daybrite/day/tree/main/crates/day-core), [day-spec](https://github.com/daybrite/day/tree/main/crates/day-spec), [day-pieces](https://github.com/daybrite/day/tree/main/crates/day-pieces), and [day-reactive](https://github.com/daybrite/day/tree/main/crates/day-reactive) provide the tree, common types, builders, and bindings. [linkme](https://github.com/dtolnay/linkme) 0.3 registers renderers; [log](https://github.com/rust-lang/log) 0.4 provides diagnostics. |
| Model reader | [serde_json](https://github.com/serde-rs/json) 1 parses the JSON document. |
| Web-view backends | [day-piece-webview](https://github.com/daybrite/day-piece-webview) binds each platform's engine; [day-async](https://github.com/daybrite/day/tree/main/crates/day-async) times the wait for the player page to load. [lottie-web](https://github.com/airbnb/lottie-web) 5.13.0 (MIT) is vendored under [web/](web/) with its licence and provenance. |
| iOS feature `uikit` | [day-uikit](https://github.com/daybrite/day/tree/main/toolkits/day-uikit), [objc2](https://github.com/madsmtm/objc2) 0.6, [objc2-foundation](https://github.com/madsmtm/objc2) 0.3, and [objc2-ui-kit](https://github.com/madsmtm/objc2) 0.3; SwiftPM adds the `Lottie` product from [airbnb/lottie-ios](https://github.com/airbnb/lottie-ios), with a compatible version starting at 4.5.0. |
| Android feature `mdc` | [day-android](https://github.com/daybrite/day/tree/main/toolkits/day-android); Gradle adds [com.airbnb.android:lottie:6.6.0](https://github.com/airbnb/lottie-android) and its transitive Android dependencies. |
| Tests | [day-mock](https://github.com/daybrite/day/tree/main/crates/day-mock) for host-side piece tests. |

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
`day launch -p macos-appkit --script dayscript/lottie.yaml --script dayscript/gallery.yaml`,
or the same command with `-p ios-uikit`, `-p android-mdc`, `-p web-dom`, or any other target in
[demo/Day.toml](demo/Day.toml). CI runs both scripts on all eight primary platform-toolkit
pairs, publishes the captures to the project website, and deploys the web build beside them.
Inspect playback on the platforms you ship for the animations you ship.
