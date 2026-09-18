<!--
Copyright © The Daybrite Project
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Lottie Demo

The demo and on-device test app for [`day-piece-lottie`](..): a navigation app of twelve bundled
animations, one page each, plus a playground page that picks between them, reads out what the
crate's headless reader finds in the selected file, and drives playback speed. It depends on the
piece by path (`day-piece-lottie = { path = ".." }`), so a change to the piece and a change here
land in one pull request and one CI run.

Each animation's route is its file's name, which is the URL hash on the web build
(<https://daybrite.github.io/day-piece-lottie/webapp/#pin-jump>), the id of the view on its page,
and the name its screenshot is filed under.

The ten animations under `resource/assets/lottie/` are Airbnb's own samples, vendored with their
licence and provenance in [the README beside them](resource/assets/lottie/README.md);
`hello.json` and `hamburger-arrow.json` are this app's own.

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
