<!--
Copyright © The Daybrite Project
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Changelog

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
