// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! Lottie Demo — the demo and on-device test app for `day-piece-lottie`.
//!
//! One page: the bundled animation, the facts the headless reader takes from its file, and a
//! playback-speed control. Every element carries a stable id, so `dayscript/lottie.yaml` can
//! assert all of it on the iOS Simulator and the Android emulator — which is how the crate's CI
//! proves that [`LottieModel`] answers correctly inside a device build, not only on the host.

use day::prelude::*;
use day_piece_lottie::{LottieModel, lottie};

// The mobile entry point; a plain cargo desktop build enters through src/main.rs.
day::day_start!(options: window(), root);

// Typed constants for everything under `resource/` (https://daybrite.dev/docs/resources).
day::resources!();

/// The animation the app bundles (`resource/assets/hello.json`), read at compile time for the
/// facts panel. The native view loads the same file from the app bundle at run time, so what the
/// panel says and what plays come from one source.
const HELLO_JSON: &str = include_str!("../resource/assets/hello.json");

/// The window every entry point opens.
pub fn window() -> day::WindowOptions {
    day::WindowOptions {
        locales: Some((res::locales::DEFAULT, res::locales::CATALOG)),
        title_fn: Some(|| res::str::app_title().format()),
        size: day::prelude::Size::new(480.0, 800.0),
        ..Default::default()
    }
}

/// The whole app: title, animation, the file's facts, playback controls.
pub fn root() -> impl Piece {
    info!("Lottie Demo starting");

    // Playback rate, bound two ways: the slider (or a preset button) drives the signal, and
    // `.speed(speed)` pushes it to the native `LottieAnimationView` live.
    let speed = Signal::new(1.0_f64);
    let preset = |text: &'static str, value: f64, id: &'static str| {
        button(text)
            .bordered()
            .action(move || speed.set(value))
            .id(id)
    };

    column((
        label(res::str::app_title())
            .font(Font::Title)
            .id("lottie-title"),
        column((lottie("hello")
            .looping(true)
            .autoplay(true)
            .speed(speed)
            .frame(240.0, 240.0)
            .id("lottie-view"),))
        .align(HAlign::Center)
        .grow_w(),
        facts(),
        section((
            labeled(
                res::str::speed(),
                row((
                    slider(speed)
                        .range(0.25..=3.0)
                        .step(0.25)
                        .id("lottie-speed-slider"),
                    label(move || format!("{:.2}\u{d7}", speed.get())).id("lottie-speed-value"),
                ))
                .spacing(8.0),
            ),
            // The same signal the slider writes, so a tap moves the slider and the readout
            // together — and gives a script a deterministic value to assert.
            row((
                preset("\u{bd}\u{d7}", 0.5, "lottie-speed-half"),
                preset("1\u{d7}", 1.0, "lottie-speed-one"),
                preset("2\u{d7}", 2.0, "lottie-speed-double"),
            ))
            .spacing(8.0),
        ))
        .title(res::str::playback_section()),
    ))
    .spacing(12.0)
    .padding(16.0)
}

/// What the headless reader says about the bundled file: one labeled row per fact, each under
/// the id the walkthrough asserts. A file the reader cannot parse shows the error instead, so a
/// broken asset is visible on the page rather than a crash at startup.
fn facts() -> AnyPiece {
    let model = match LottieModel::parse(HELLO_JSON) {
        Ok(model) => model,
        Err(e) => {
            return section((label(format!("{}: {e}", res::str::model_error().format()))
                .id("lottie-model-error"),))
            .title(res::str::model_section())
            .any();
        }
    };
    let kinds = model
        .layers
        .iter()
        .map(|l| l.kind.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let issues = model.verify();
    section((
        labeled(
            res::str::model_name(),
            label(model.name.clone().unwrap_or_default()).id("lottie-model-name"),
        ),
        labeled(
            res::str::model_frames(),
            label(format!("{} @ {} fps", model.frames(), model.frame_rate))
                .id("lottie-model-frames"),
        ),
        labeled(
            res::str::model_duration(),
            label(format!("{:.1} s", model.duration_secs())).id("lottie-model-duration"),
        ),
        labeled(
            res::str::model_size(),
            label(format!("{} \u{d7} {}", model.width, model.height)).id("lottie-model-size"),
        ),
        labeled(
            res::str::model_layers(),
            label(format!("{} ({kinds})", model.layers.len())).id("lottie-model-layers"),
        ),
        labeled(
            res::str::model_issues(),
            label(issues.len().to_string()).id("lottie-model-issues"),
        ),
    ))
    .title(res::str::model_section())
    .any()
}
