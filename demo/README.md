<!--
Copyright © The Daybrite Project
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Lottie Demo

The demo and on-device test app for [`day-piece-lottie`](..): one page with the bundled
animation, the facts the crate's headless reader takes from its file, and a playback-speed
control. It depends on the piece by path (`day-piece-lottie = { path = ".." }`), so a change to
the piece and a change here land in one pull request and one CI run.

## Run it

```sh
day doctor                                            # the iOS and Android toolchains
day launch -p ios-uikit --script dayscript/lottie.yaml
day launch -p android-mdc --script dayscript/lottie.yaml
```

The script is the test: it asserts every fact on the page and captures three screenshots under
`build/day/screenshots/<target>/`. CI runs exactly this on the iOS Simulator and the Android
emulator ([../.github/workflows/ci.yml](../.github/workflows/ci.yml)).

## Build against a local day

No `Cargo.lock` is committed: the first build resolves day at the tip of `main`, and `cargo
update` moves it there again. To build against a checkout of day (or of the piece) instead:

```sh
day patch --local ../../day             # writes .cargo/config.toml, gitignored
day patch --check                       # every day crate now resolves from the checkout
```

Delete `.cargo/config.toml` to go back to the git dependency. The lock is gitignored, so a
patched build cannot leave the checkout's paths behind for anyone else.
