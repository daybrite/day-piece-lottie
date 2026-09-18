// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! Lottie Demo: the demo and on-device test app for `day-piece-lottie`.
//!
//! A navigation app of one page per bundled animation, and a playground page in front of them.
//! An animation page names the file, plays it, sums up what the headless reader finds inside it,
//! and carries the playback controls. The playground adds the picker, whose bound name swaps the
//! running view's animation, and the reader's full panel, which is the crate's second half
//! ([`LottieModel`]) answering inside a device build.
//!
//! The route is the address: it is the URL hash on web-dom, so <https://…/#pin-jump> opens that
//! animation on a fresh load and the browser's history walks the pages, and it is the name
//! `dayscript`'s `navigate:` step addresses. Every element carries a stable id, so the
//! walkthroughs assert what is on a page as well as reach it.
//!
//! Where the animation is drawn differs by platform: iOS and Android hand the file to Airbnb's
//! own renderer, and every other backend plays it with lottie-web inside a web view. The app
//! above this line is the same code either way.

use day::prelude::*;
use day_piece_lottie::{LottieModel, lottie};

// The mobile and web entry point; a plain cargo desktop build enters through src/main.rs.
day::day_start!(options: window(), root);

// Typed constants for everything under `resource/` (https://daybrite.dev/docs/resources).
day::resources!();

day::routes! {
    /// One route per page (https://daybrite.dev/docs/navigation). The key is the app's route,
    /// which day-dom reflects into the URL hash, so `…/#switch` opens that animation, and it is
    /// what `dayscript`'s `navigate:` addresses. Each animation's key is its file's name under
    /// `resource/assets/lottie/`, so one name is the route, the id of the view on that page, and
    /// the name its screenshot is filed under.
    pub(crate) enum Page {
        Playground => "playground",
        Hello => "hello",
        HamburgerArrow => "hamburger-arrow",
        LottieLogo1 => "lottie-logo1",
        LottieLogo1Masked => "lottie-logo1-masked",
        LottieLogo2 => "lottie-logo2",
        NineSquares => "nine-squares-alboardman",
        MotionCorpse => "motioncorpse-jrcanest",
        PinJump => "pin-jump",
        TwitterHeart => "twitter-heart",
        BoatLoader => "boat-loader",
        IconTransitions => "icon-transitions",
        Switch => "switch",
    }
}

/// One bundled animation: the page that opens it, the file the player loads, what to call it, and
/// the same file's text for the reader.
struct Animation {
    /// The route that opens it, and the id the view on that page carries.
    page: Page,
    /// The name `lottie()` plays: a path under `resource/assets/`, without the extension. The
    /// player resolves it in the app's bundle at run time; `json` below is the same file,
    /// compiled in, so the reader and the player never disagree about which file this is.
    name: &'static str,
    title: fn() -> day::LocalizedText,
    /// The line under the title: what the file is worth looking at for.
    note: fn() -> day::LocalizedText,
    json: &'static str,
}

/// Everything the app ships, in the order the sidebar lists it. The first two are this demo's own
/// files; the rest are Airbnb's samples, vendored under `resource/assets/lottie/` with their
/// provenance and licence recorded in the README beside them.
static ANIMATIONS: [Animation; 12] = [
    Animation {
        page: Page::Hello,
        name: "hello",
        title: res::str::anim_hello,
        note: res::str::note_hello,
        json: include_str!("../resource/assets/hello.json"),
    },
    Animation {
        page: Page::HamburgerArrow,
        name: "hamburger-arrow",
        title: res::str::anim_hamburger,
        note: res::str::note_hamburger,
        json: include_str!("../resource/assets/hamburger-arrow.json"),
    },
    Animation {
        page: Page::LottieLogo1,
        name: "lottie/lottie-logo1",
        title: res::str::anim_logo1,
        note: res::str::note_logo1,
        json: include_str!("../resource/assets/lottie/lottie-logo1.json"),
    },
    Animation {
        page: Page::LottieLogo1Masked,
        name: "lottie/lottie-logo1-masked",
        title: res::str::anim_logo1_masked,
        note: res::str::note_logo1_masked,
        json: include_str!("../resource/assets/lottie/lottie-logo1-masked.json"),
    },
    Animation {
        page: Page::LottieLogo2,
        name: "lottie/lottie-logo2",
        title: res::str::anim_logo2,
        note: res::str::note_logo2,
        json: include_str!("../resource/assets/lottie/lottie-logo2.json"),
    },
    Animation {
        page: Page::NineSquares,
        name: "lottie/nine-squares-alboardman",
        title: res::str::anim_nine_squares,
        note: res::str::note_nine_squares,
        json: include_str!("../resource/assets/lottie/nine-squares-alboardman.json"),
    },
    Animation {
        page: Page::MotionCorpse,
        name: "lottie/motioncorpse-jrcanest",
        title: res::str::anim_motion_corpse,
        note: res::str::note_motion_corpse,
        json: include_str!("../resource/assets/lottie/motioncorpse-jrcanest.json"),
    },
    Animation {
        page: Page::PinJump,
        name: "lottie/pin-jump",
        title: res::str::anim_pin_jump,
        note: res::str::note_pin_jump,
        json: include_str!("../resource/assets/lottie/pin-jump.json"),
    },
    Animation {
        page: Page::TwitterHeart,
        name: "lottie/twitter-heart",
        title: res::str::anim_twitter_heart,
        note: res::str::note_twitter_heart,
        json: include_str!("../resource/assets/lottie/twitter-heart.json"),
    },
    Animation {
        page: Page::BoatLoader,
        name: "lottie/boat-loader",
        title: res::str::anim_boat_loader,
        note: res::str::note_boat_loader,
        json: include_str!("../resource/assets/lottie/boat-loader.json"),
    },
    Animation {
        page: Page::IconTransitions,
        name: "lottie/icon-transitions",
        title: res::str::anim_icon_transitions,
        note: res::str::note_icon_transitions,
        json: include_str!("../resource/assets/lottie/icon-transitions.json"),
    },
    Animation {
        page: Page::Switch,
        name: "lottie/switch",
        title: res::str::anim_switch,
        note: res::str::note_switch,
        json: include_str!("../resource/assets/lottie/switch.json"),
    },
];

/// The playground picker's selection as an entry of [`ANIMATIONS`], clamped so a stale index
/// never panics.
fn animation(index: usize) -> &'static Animation {
    &ANIMATIONS[index.min(ANIMATIONS.len() - 1)]
}

/// The window every entry point opens.
pub fn window() -> day::WindowOptions {
    day::WindowOptions {
        locales: Some((res::locales::DEFAULT, res::locales::CATALOG)),
        title_fn: Some(|| res::str::app_title().format()),
        size: day::prelude::Size::new(480.0, 800.0),
        ..Default::default()
    }
}

/// The whole app: a sidebar of the playground and every bundled animation, and whichever page is
/// open.
pub fn root() -> impl Piece {
    info!("Lottie Demo starting");

    // The open page. The nav writes it, a launch deep link writes it before the first frame (the
    // URL hash on web-dom, `DAY_DEEPLINK` elsewhere), and the browser's back button writes it
    // again.
    let page = Signal::new(Page::Playground);
    let mut nav = nav(page)
        .style(NavStyle::Sidebar)
        .title(res::str::app_title())
        .section(res::str::nav_playground())
        .item(Page::Playground, res::str::nav_picker(), playground_page)
        .section(res::str::nav_animations());
    // One row per bundled animation, in the order [`ANIMATIONS`] declares them.
    for anim in ANIMATIONS.iter() {
        nav = nav.item(anim.page, (anim.title)(), move || animation_page(anim));
    }
    nav.id("nav")
}

/// One animation's page: what it is, the animation playing, what the reader finds in its file,
/// and the controls that drive playback.
fn animation_page(anim: &'static Animation) -> impl Piece {
    // Playback rate, bound two ways: the slider (or a preset button) drives the signal, and
    // `.speed(speed)` pushes it to whatever is playing the file.
    let speed = Signal::new(1.0_f64);
    column((
        label((anim.title)())
            .font(Font::Headline)
            .id("lottie-example-title"),
        label((anim.note)()).font(Font::Callout).id("lottie-note"),
        // The animation takes every point the page has left after the text and the controls, so
        // a window resized in either direction rescales it rather than padding around a fixed
        // frame. The id is the route, which is what the walkthrough waits for after navigating.
        lottie(anim.name)
            .looping(true)
            .autoplay(true)
            .speed(speed)
            .id(anim.page.key())
            .grow(),
        label(summary(anim))
            .font(Font::Caption)
            .id("lottie-summary"),
        playback(speed),
    ))
    .spacing(12.0)
    .padding(16.0)
    .grow()
}

/// The size, length and layers the reader finds in the file that is playing above it, in one
/// line. The playground's [`facts`] panel is the same reader, a row at a time.
fn summary(anim: &'static Animation) -> String {
    match LottieModel::parse(anim.json) {
        Ok(m) => format!(
            "{} \u{d7} {} \u{b7} {} frames @ {} fps \u{b7} {:.1} s \u{b7} {}",
            m.width,
            m.height,
            m.frames(),
            m.frame_rate,
            m.duration_secs(),
            layer_summary(&m),
        ),
        Err(e) => format!("{}: {e}", res::str::model_error().format()),
    }
}

/// The playground: the picker of every bundled animation, the one it selects, the reader's full
/// panel, and the playback controls.
///
/// The picker is what covers the reactive name: `lottie(closure)` reads the signal the picker
/// writes and swaps the running view's animation in place, which the other pages never ask for
/// because navigating to one builds the view afresh.
fn playground_page() -> impl Piece {
    let selected = Signal::new(0usize);
    let name = move || animation(selected.get()).name.to_string();
    let speed = Signal::new(1.0_f64);

    column((
        labeled(
            res::str::animation(),
            picker(
                ANIMATIONS
                    .iter()
                    .map(|a| (a.title)().format())
                    .collect::<Vec<_>>(),
                selected,
            )
            .id("lottie-animation"),
        ),
        lottie(name)
            .looping(true)
            .autoplay(true)
            .speed(speed)
            .id("lottie-view")
            .grow(),
        facts(move || animation(selected.get())),
        playback(speed),
    ))
    .spacing(12.0)
    .padding(16.0)
    .grow()
}

/// The playback controls: a slider over the rate, the readout of it, and three presets that write
/// the same signal, so a tap moves the slider and the readout together.
fn playback(speed: Signal<f64>) -> impl Piece {
    let preset = move |text: &'static str, value: f64, id: &'static str| {
        button(text)
            .bordered()
            .action(move || speed.set(value))
            .id(id)
    };
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
        row((
            preset("\u{bd}\u{d7}", 0.5, "lottie-speed-half"),
            preset("1\u{d7}", 1.0, "lottie-speed-one"),
            preset("2\u{d7}", 2.0, "lottie-speed-double"),
        ))
        .spacing(8.0),
    ))
    .title(res::str::playback_section())
}

/// What the headless reader says about the selected file: one labeled row per fact, each under
/// the id the walkthrough asserts. A file the reader cannot parse shows the error in its name
/// row, so a broken asset is visible on the page rather than a crash at startup.
fn facts(pick: impl Fn() -> &'static Animation + Copy + 'static) -> impl Piece {
    let fact = move |read: fn(&LottieModel) -> String| {
        move || match LottieModel::parse(pick().json) {
            Ok(model) => read(&model),
            Err(e) => format!("{}: {e}", res::str::model_error().format()),
        }
    };
    section((
        labeled(
            res::str::model_name(),
            label(fact(|m| match m.name.as_deref() {
                // Most of Airbnb's samples record no composition name, which is the file saying
                // nothing rather than the reader failing to find it.
                Some("") | None => res::str::model_unnamed().format(),
                Some(name) => name.to_string(),
            }))
            .id("lottie-model-name"),
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
