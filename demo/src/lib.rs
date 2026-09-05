// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! Lottie Demo — the demo and on-device test app for `day-piece-lottie`.
//!
//! One page: a picker of the bundled animations, the selected one playing, the facts the headless
//! reader takes from its file, and a playback-speed control. Every element carries a stable id,
//! so `dayscript/lottie.yaml` can assert all of it on the iOS Simulator and the Android emulator —
//! which is how the crate's CI proves that [`LottieModel`] answers correctly inside a device
//! build, and that a bound name swaps the native view's animation live.

use day::prelude::*;
use day_piece_lottie::{LottieModel, lottie};

// The mobile entry point; a plain cargo desktop build enters through src/main.rs.
day::day_start!(options: window(), root);

// Typed constants for everything under `resource/` (https://daybrite.dev/docs/resources).
day::resources!();

/// The animations the app bundles under `resource/assets/`: the name `lottie()` loads by, and the
/// file's text for the facts panel. The native view loads the same file from the app at run time,
/// so what the panel says and what plays come from one source.
const ANIMATIONS: [(&str, &str); 2] = [
    ("hello", include_str!("../resource/assets/hello.json")),
    (
        "hamburger-arrow",
        include_str!("../resource/assets/hamburger-arrow.json"),
    ),
];

/// The window every entry point opens.
pub fn window() -> day::WindowOptions {
    day::WindowOptions {
        locales: Some((res::locales::DEFAULT, res::locales::CATALOG)),
        title_fn: Some(|| res::str::app_title().format()),
        size: day::prelude::Size::new(480.0, 800.0),
        ..Default::default()
    }
}

/// The whole app: title, picker, animation, the file's facts, playback controls.
pub fn root() -> impl Piece {
    info!("Lottie Demo starting");

    // Which bundled animation plays. The picker writes it; `lottie(closure)` reads it and
    // swaps the native view's animation whenever it changes; the facts panel reads it too.
    let selected = Signal::new(0usize);
    let name = move || ANIMATIONS[selected.get().min(ANIMATIONS.len() - 1)].0.to_string();

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
        labeled(
            res::str::animation(),
            picker(
                [
                    res::str::anim_hello().format(),
                    res::str::anim_hamburger().format(),
                ],
                selected,
            )
            .id("lottie-animation"),
        ),
        column((lottie(name)
            .looping(true)
            .autoplay(true)
            .speed(speed)
            .frame(240.0, 240.0)
            .id("lottie-view"),))
        .align(HAlign::Center)
        .grow_w(),
        facts(selected),
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

/// What the headless reader says about the selected file: one labeled row per fact, each under
/// the id the walkthrough asserts, each following the picker. A file the reader cannot parse
/// shows the error in its name row, so a broken asset is visible on the page rather than a
/// crash at startup.
fn facts(selected: Signal<usize>) -> impl Piece {
    let fact = move |pick: fn(&LottieModel) -> String| {
        move || match LottieModel::parse(ANIMATIONS[selected.get().min(ANIMATIONS.len() - 1)].1)
        {
            Ok(model) => pick(&model),
            Err(e) => format!("{}: {e}", res::str::model_error().format()),
        }
    };
    section((
        labeled(
            res::str::model_name(),
            label(fact(|m| m.name.clone().unwrap_or_default())).id("lottie-model-name"),
        ),
        labeled(
            res::str::model_frames(),
            label(fact(|m| format!("{} @ {} fps", m.frames(), m.frame_rate)))
                .id("lottie-model-frames"),
        ),
        labeled(
            res::str::model_duration(),
            label(fact(|m| format!("{:.1} s", m.duration_secs()))).id("lottie-model-duration"),
        ),
        labeled(
            res::str::model_size(),
            label(fact(|m| format!("{} \u{d7} {}", m.width, m.height))).id("lottie-model-size"),
        ),
        labeled(
            res::str::model_layers(),
            label(fact(layer_summary)).id("lottie-model-layers"),
        ),
        labeled(
            res::str::model_issues(),
            label(fact(|m| m.verify().len().to_string())).id("lottie-model-issues"),
        ),
    ))
    .title(res::str::model_section())
}

/// "4 shape", or "3 shape, 1 null": the layer count, by kind, in order of first appearance.
fn layer_summary(model: &LottieModel) -> String {
    let mut counts: Vec<(String, usize)> = Vec::new();
    for layer in &model.layers {
        let kind = layer.kind.to_string();
        match counts.iter_mut().find(|(k, _)| *k == kind) {
            Some((_, n)) => *n += 1,
            None => counts.push((kind, 1)),
        }
    }
    if counts.is_empty() {
        return "0".to_string();
    }
    counts
        .iter()
        .map(|(k, n)| format!("{n} {k}"))
        .collect::<Vec<_>>()
        .join(", ")
}
