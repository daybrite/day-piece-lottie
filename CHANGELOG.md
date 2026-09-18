<!--
Copyright © The Daybrite Project
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Changelog

## Unreleased

- New: the web-view arm. Every backend without a native Lottie player — macOS, Linux, Windows,
  HarmonyOS and the web — now plays the same file with Airbnb's lottie-web, vendored under
  `web/` and shown through `day-piece-webview`. `lottie("hello")` is the whole API on all of
  them, unchanged. `macos-gtk` and `windows-gtk` have no WebKitGTK and still realize the
  placeholder. Needs a `day` CLI that stages `[package.metadata.day.piece].assets`.
- New: the demo is a navigation app of twelve animations, one route each, ten of them Airbnb's
  own samples vendored under `demo/resource/assets/lottie/` (Apache-2.0, with their provenance
  recorded beside them). `dayscript/gallery.yaml` opens every one by its route and captures it;
  `dayscript/lottie.yaml` drives the playground page it opens on. CI runs both on all eight
  primary platform-toolkit pairs and publishes the captures and the web build to the project
  website (`demo/website/`).
- Changed: the Android factory moved from `platform/android/java/…/DayLottie.java` to
  `src/DayLottie.java`, beside the Rust arms. Building for Android now needs a `day` CLI that
  links single-file `java` entries; an older one skips the file, and the app fails when it first
  creates the view.

## 0.1.0

The first release from its own repository. The crate moved out of `daybrite/day`
(`pieces/day-piece-lottie`) with its history.

- Tested against day 0.4 at the revision in `demo/Cargo.lock`.
- New: `model`, the headless reader. `LottieModel::parse` reads a document's version, name,
  frame rate, in and out points, size, layers, assets, and markers; `verify()` lists what a
  player would refuse.
- New: the `demo/` app and its `dayscript/lottie.yaml`, run by CI on the iOS Simulator and the
  Android emulator.
- New: `lottie(name)` takes any `IntoText` — a `Signal<String>` or closure swaps the animation
  live through a `Name` patch (`day_lottie_set_animation` on iOS, `DayLottie.setAnimation` on
  Android). A `/` path under `resource/assets/` works as a name.
- Unchanged: `.looping()`, `.autoplay()`, `.speed()`, and the two renderers otherwise.
