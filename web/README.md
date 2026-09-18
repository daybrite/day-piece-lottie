<!--
Copyright © The Daybrite Project
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# The web-view arm's bundled site

What every backend without a native Lottie renderer shows: `index.html` hosts
[lottie-web](https://github.com/airbnb/lottie-web) and plays the animation the app asked for.
`src/web.rs` builds the view, and `[package.metadata.day.piece].assets` in `Cargo.toml` is what
stages this directory into an app's bundle under `day-piece-lottie/`.

## Vendored

| File | Origin | Version | Licence |
| --- | --- | --- | --- |
| `lottie.min.js` | [airbnb/lottie-web](https://github.com/airbnb/lottie-web), `build/player/lottie.min.js` | v5.13.0 (`bede03d25d23`) | MIT — `LICENSE.lottie-web.md` beside it |

`sha256 2eb762973aec914d981f426123040bfac9d26217239605e225ddc7cee17618ac`

It is committed rather than fetched at build time so an app's build needs no network and every
build of a given commit ships the same player. To update it, download the same path at the new
tag, refresh the version, hash and licence file here, and run the demo's walkthrough on
`macos-appkit` and `web-dom` before committing: a player regression shows up as a blank frame,
which nothing but a screenshot catches.

`index.html` is ours (MPL-2.0, like the rest of the crate).
