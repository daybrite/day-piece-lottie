<!--
Copyright © The Daybrite Project
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# The bundled animations

What the demo plays, vendored so the app needs no network and every build of a commit shows the
same thing. Each file is committed as downloaded; nothing here is ours.

## Provenance

All ten come from [airbnb/lottie-ios](https://github.com/airbnb/lottie-ios), `Tests/Samples/`,
which that repository distributes under the **Apache License 2.0** — `LICENSE.lottie-ios.txt`
beside them is its copy. Two carry their artist's name in the original file name, kept here.

| File | Source name | Size | Shows |
| --- | --- | --- | --- |
| `lottie-logo1.json` | `LottieLogo1.json` | 375×667, 179f @ 30fps, 48 layers | shape morphing, the animation Lottie launched with |
| `lottie-logo1-masked.json` | `LottieLogo1_masked.json` | 375×667, 179f @ 30fps, 49 layers | the same logo behind a mask |
| `lottie-logo2.json` | `LottieLogo2.json` | 281×500, 378f @ 30fps, 74 layers | the heaviest of the set: 74 layers |
| `nine-squares-alboardman.json` | `9squares_AlBoardman.json` (Al Boardman) | 180×180, 176f @ 30fps, 21 layers | a looping grid, precise easing |
| `motioncorpse-jrcanest.json` | `MotionCorpse_Jrcanest.json` (Jr.canest) | 1920×1080, 82f @ 24fps, 26 layers | a wide canvas scaled into the view |
| `pin-jump.json` | `PinJump.json` | 150×150, 93f @ 30fps, 9 layers | squash and stretch |
| `twitter-heart.json` | `TwitterHeart.json` | 100×100, 116f @ 60fps, 18 layers | a burst at 60fps |
| `boat-loader.json` | `Boat_Loader.json` | 1334×1600, 888f @ 60fps, 5 layers | the longest: 888 frames |
| `icon-transitions.json` | `IconTransitions.json` | 140×140, 158f @ 30fps, 13 layers | icons morphing into one another |
| `switch.json` | `Switch.json` | 100×100, 150f @ 60fps, 5 layers | a control-sized animation |

`hello.json` and `hamburger-arrow.json`, one directory up, are the demo's own originals.

## Updating

Download the same path at the newer revision, keep the renaming (lower case, hyphens), and refresh
the table above from `day lottie info` or the demo's own readout. Run the walkthrough on
`macos-appkit` and `web-dom` afterwards: a file the player cannot parse still lays out, so only a
screenshot catches it.
