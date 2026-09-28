<!--
Copyright © The Daybrite Project
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Lottie Demo

The demo and on-device test app for [`day-piece-lottie`](..): a navigation app of twelve bundled
animations, one page each, plus a playground page that picks between them, reads out what the
crate's headless reader finds in the selected file, and drives playback speed.

Every page, and every animation, lives in [`day-piece-lottie-gallery`](../gallery), a library of
the pages that any Day app can mount. This app is only the shell around them (a window, a sidebar
and the routes), and [Day-Showcase](https://github.com/daybrite/Day-Showcase) shows the same pages
under its Lottie section, so a change to an example is a change to both. The demo depends on the
gallery by path (`day-piece-lottie-gallery = { path = "../gallery" }`), and the gallery on the
piece, so a change to the piece, its examples and this shell lands in one pull request and one CI
run. The animations are the gallery's data assets (`[package.metadata.day.piece].assets`), which
`day build` stages into whichever app depends on it as `day-piece-lottie-gallery/<file>`, so
neither app copies a file.

Each animation's route is its file's name, which is the URL hash on the web build
(<https://daybrite.github.io/day-piece-lottie/webapp/#pin-jump>), the id of the view on its page,
and the name its screenshot is filed under.

The ten animations under `gallery/animations/lottie/` are Airbnb's own samples, vendored with their
licence and provenance in [the README beside them](../gallery/animations/lottie/README.md);
`hello.json` and `hamburger-arrow.json` are the gallery's own.

## Run it

```sh
day doctor                                            # the toolchains for the targets you want
day launch -p macos-appkit --script dayscript/lottie.yaml --script dayscript/gallery.yaml
day launch -p web-dom      --script dayscript/gallery.yaml
day launch -p ios-uikit    --script dayscript/lottie.yaml
day launch -p android-mdc  --script dayscript/lottie.yaml
```

The scripts are the test. `dayscript/lottie.yaml` asserts every fact on the playground page,
swaps animations through the picker and drives the speed control; `dayscript/gallery.yaml` opens
each animation by its route, asserts the line the reader writes under it, and captures it. Both
save screenshots under `build/day/screenshots/<target>/`. CI runs exactly this on all eight
primary platform-toolkit pairs ([../.github/workflows/ci.yml](../.github/workflows/ci.yml)) and
publishes what they captured to the project website (`website/site.toml`), which is where the
crate's README links for its pictures.

## Build against a local day

No `Cargo.lock` is committed: the first build resolves day at the tip of `main`, and `cargo
update` moves it there again. To build against a checkout of day (or of the piece) instead:

```sh
day patch --local ../../day             # writes .cargo/config.toml, gitignored
day patch --check                       # every day crate now resolves from the checkout
```

Delete `.cargo/config.toml` to go back to the git dependency. The lock is gitignored, so a
patched build cannot leave the checkout's paths behind for anyone else.
