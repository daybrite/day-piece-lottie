// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The day-piece-lottie example pages, and the animations they play, as a library any Day app
//! can mount.
//!
//! The Lottie Demo app is a sidebar of these pages; Day-Showcase shows the same pages under its
//! Lottie section. Both call into this crate, so a fix or a new animation lands in both at once.
//!
//! Two ways in:
//!
//! * [`pages`] lists every page with its route and title, for an app that builds its own
//!   navigation around them (the demo's sidebar is one `nav` item per entry), and [`LottiePage`]
//!   is the route type, so `…/#pin-jump` opens the same page in every host.
//! * [`gallery`] is every animation in one page, the playground, whose one picker chooses among
//!   them, for an app that gives the animations a single slot (Showcase's Lottie section).
//!
//! The animations are this crate's data assets (`animations/`, declared in Cargo.toml), which
//! `day build` stages into the host's bundle as `day-piece-lottie-gallery/<file>`: the host
//! copies nothing. The strings come from this crate's private catalog (`resource/locales`), so
//! the pages read the same in any host and follow its locale; a dayscript names them by their
//! qualified key (`day_piece_lottie_gallery::anim_hello`), which [`register`] makes resolvable.
//!
//! Where an animation is drawn differs by platform: iOS and Android hand the file to Airbnb's own
//! renderer, and every other backend plays it with lottie-web inside a web view. The pages are the
//! same code either way.

use day::prelude::*;
use day_piece_lottie::{LottieModel, lottie};

// Typed accessors for this crate's own strings: `res::str::anim_hello()` and friends.
day_fluent::locales!();

/// Where `day build` stages `animations/` in the host's bundle: this crate's name, which is not
/// configurable (day docs/extending.md "Data assets a piece ships").
macro_rules! asset {
    ($path:literal) => {
        concat!("day-piece-lottie-gallery/", $path)
    };
}

day::routes! {
    /// One route per page (https://daybrite.dev/docs/navigation). The key is the host's route,
    /// which day-dom reflects into the URL hash, so `…/#switch` opens that animation, and it is
    /// what `dayscript`'s `navigate:` addresses. Each animation's key is its file's name under
    /// `animations/`, so one name is the route, the id of the view on that page, and the name its
    /// screenshot is filed under.
    pub enum LottiePage {
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
    page: LottiePage,
    /// The name `lottie()` plays: the file's staged path in the host's bundle, without the
    /// extension. The player resolves it at run time; `json` below is the same file, compiled
    /// in, so the reader and the player never disagree about which file this is.
    name: &'static str,
    title: fn() -> day::LocalizedText,
    /// The line under the title: what the file is worth looking at for.
    note: fn() -> day::LocalizedText,
    json: &'static str,
}

/// Every animation, in the order the pages list them. The first two are this crate's own files;
/// the rest are Airbnb's samples, vendored under `animations/lottie/` with their provenance and
/// licence recorded in the README beside them.
static ANIMATIONS: [Animation; 12] = [
    Animation {
        page: LottiePage::Hello,
        name: asset!("hello"),
        title: res::str::anim_hello,
        note: res::str::note_hello,
        json: include_str!("../animations/hello.json"),
    },
    Animation {
        page: LottiePage::HamburgerArrow,
        name: asset!("hamburger-arrow"),
        title: res::str::anim_hamburger,
        note: res::str::note_hamburger,
        json: include_str!("../animations/hamburger-arrow.json"),
    },
    Animation {
        page: LottiePage::LottieLogo1,
        name: asset!("lottie/lottie-logo1"),
        title: res::str::anim_logo1,
        note: res::str::note_logo1,
        json: include_str!("../animations/lottie/lottie-logo1.json"),
    },
    Animation {
        page: LottiePage::LottieLogo1Masked,
        name: asset!("lottie/lottie-logo1-masked"),
        title: res::str::anim_logo1_masked,
        note: res::str::note_logo1_masked,
        json: include_str!("../animations/lottie/lottie-logo1-masked.json"),
    },
    Animation {
        page: LottiePage::LottieLogo2,
        name: asset!("lottie/lottie-logo2"),
        title: res::str::anim_logo2,
        note: res::str::note_logo2,
        json: include_str!("../animations/lottie/lottie-logo2.json"),
    },
    Animation {
        page: LottiePage::NineSquares,
        name: asset!("lottie/nine-squares-alboardman"),
        title: res::str::anim_nine_squares,
        note: res::str::note_nine_squares,
        json: include_str!("../animations/lottie/nine-squares-alboardman.json"),
    },
    Animation {
        page: LottiePage::MotionCorpse,
        name: asset!("lottie/motioncorpse-jrcanest"),
        title: res::str::anim_motion_corpse,
        note: res::str::note_motion_corpse,
        json: include_str!("../animations/lottie/motioncorpse-jrcanest.json"),
    },
    Animation {
        page: LottiePage::PinJump,
        name: asset!("lottie/pin-jump"),
        title: res::str::anim_pin_jump,
        note: res::str::note_pin_jump,
        json: include_str!("../animations/lottie/pin-jump.json"),
    },
    Animation {
        page: LottiePage::TwitterHeart,
        name: asset!("lottie/twitter-heart"),
        title: res::str::anim_twitter_heart,
        note: res::str::note_twitter_heart,
        json: include_str!("../animations/lottie/twitter-heart.json"),
    },
    Animation {
        page: LottiePage::BoatLoader,
        name: asset!("lottie/boat-loader"),
        title: res::str::anim_boat_loader,
        note: res::str::note_boat_loader,
        json: include_str!("../animations/lottie/boat-loader.json"),
    },
    Animation {
        page: LottiePage::IconTransitions,
        name: asset!("lottie/icon-transitions"),
        title: res::str::anim_icon_transitions,
        note: res::str::note_icon_transitions,
        json: include_str!("../animations/lottie/icon-transitions.json"),
    },
    Animation {
        page: LottiePage::Switch,
        name: asset!("lottie/switch"),
        title: res::str::anim_switch,
        note: res::str::note_switch,
        json: include_str!("../animations/lottie/switch.json"),
    },
];

/// The playground picker's selection as an entry of [`ANIMATIONS`], clamped so a stale index
/// never panics.
fn animation(index: usize) -> &'static Animation {
    &ANIMATIONS[index.min(ANIMATIONS.len() - 1)]
}

/// One page: the route that opens it, its name, and how to build it.
#[derive(Clone, Copy)]
pub struct Entry {
    pub page: LottiePage,
    pub title: fn() -> day::LocalizedText,
    build: fn() -> AnyPiece,
}

impl Entry {
    /// The page as a host mounts it.
    pub fn build(&self) -> AnyPiece {
        register();
        (self.build)()
    }
}

/// Every page, in order: the playground, then one page per animation.
pub fn pages() -> Vec<Entry> {
    let mut all = vec![Entry {
        page: LottiePage::Playground,
        title: res::str::nav_picker,
        build: || playground(Signal::new(LottiePage::Hello)).any(),
    }];
    all.extend(examples());
    all
}

/// The animations' pages alone, without the playground.
pub fn examples() -> Vec<Entry> {
    // A `fn` per page, because an entry holds a plain function pointer: each one finds its own
    // animation by route.
    fn build_for(page: LottiePage) -> AnyPiece {
        ANIMATIONS
            .iter()
            .find(|a| a.page == page)
            .map(|a| animation_page(a).any())
            .unwrap_or_else(|| label("").any())
    }
    macro_rules! entry {
        ($page:expr, $title:expr) => {
            Entry {
                page: $page,
                title: $title,
                build: || build_for($page),
            }
        };
    }
    vec![
        entry!(LottiePage::Hello, res::str::anim_hello),
        entry!(LottiePage::HamburgerArrow, res::str::anim_hamburger),
        entry!(LottiePage::LottieLogo1, res::str::anim_logo1),
        entry!(LottiePage::LottieLogo1Masked, res::str::anim_logo1_masked),
        entry!(LottiePage::LottieLogo2, res::str::anim_logo2),
        entry!(LottiePage::NineSquares, res::str::anim_nine_squares),
        entry!(LottiePage::MotionCorpse, res::str::anim_motion_corpse),
        entry!(LottiePage::PinJump, res::str::anim_pin_jump),
        entry!(LottiePage::TwitterHeart, res::str::anim_twitter_heart),
        entry!(LottiePage::BoatLoader, res::str::anim_boat_loader),
        entry!(LottiePage::IconTransitions, res::str::anim_icon_transitions),
        entry!(LottiePage::Switch, res::str::anim_switch),
    ]
}

/// The page for `page`, as [`Entry::build`] mounts it.
pub fn page(page: LottiePage) -> AnyPiece {
    pages()
        .into_iter()
        .find(|e| e.page == page)
        .map(|e| e.build())
        .unwrap_or_else(|| label("").any())
}

/// Every animation in one page, for a host that gives the animations a single slot in its own
/// navigation: the playground, whose one picker chooses among them. `selected` is the animation
/// playing, owned by the host so it can pick the one the page opens on and remember or deep-link
/// it; the playground's own route ([`LottiePage::Playground`]) selects the first animation.
pub fn gallery(selected: Signal<LottiePage>) -> impl Piece {
    register();
    playground(selected)
}

/// The position in [`ANIMATIONS`] of the animation `page` opens, the first for any page that is
/// not an animation's (the playground).
fn index_of(page: LottiePage) -> usize {
    ANIMATIONS.iter().position(|a| a.page == page).unwrap_or(0)
}

/// Make this crate's strings resolvable by their qualified dayscript keys
/// (`day_piece_lottie_gallery::anim_hello`) without touching the app's locale. Idempotent;
/// every entry point calls it, so a host never has to.
pub fn register() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(res::locales::register);
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

/// The playground: the picker of every bundled animation, what the selected one is, the animation
/// itself, the reader's full panel, and the playback controls. `page` is the animation selected,
/// owned by whoever mounts the view (the demo's sidebar page, a host's [`gallery`]); the picker
/// writes it and follows it.
///
/// The picker is what covers the reactive name: `lottie(closure)` reads the selection and swaps
/// the running view's animation in place, which the other pages never ask for because navigating
/// to one builds the view afresh.
fn playground(page: Signal<LottiePage>) -> impl Piece {
    // The picker speaks indices and the owner speaks pages; each follows the other, so an owner
    // that sets the page (a deep link) moves the picker too.
    let selected = Signal::new(index_of(page.get_untracked()));
    Effect::new(move || {
        let wanted = ANIMATIONS[selected.get().min(ANIMATIONS.len() - 1)].page;
        if untrack(|| page.get()) != wanted {
            page.set(wanted);
        }
    });
    Effect::new(move || {
        let i = index_of(page.get());
        if untrack(|| selected.get()) != i {
            selected.set(i);
        }
    });
    let name = move || animation(selected.get()).name.to_string();
    let speed = Signal::new(1.0_f64);

    // A page that scrolls around a stage of fixed height, not a column the animation grows to
    // fill: under the picker, the description, the reader's six rows and the playback controls,
    // a phone has little height left over, and a script whose lines run taller (Arabic) leaves
    // none, which drew the animation zero points tall. A fixed stage keeps it visible at every
    // size and in every language, and whatever does not fit scrolls into view.
    scroll(
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
                .menu()
                .id("lottie-animation"),
            ),
            label(move || (animation(selected.get()).note)().format())
                .font(Font::Callout)
                .id("lottie-note"),
            lottie(name)
                .looping(true)
                .autoplay(true)
                .speed(speed)
                // The id before the height: a decorator takes the id of what it wraps, and the web
                // arms need it on the engine itself so `web_eval` can reach the player.
                .id("lottie-view")
                .height(PLAYGROUND_STAGE),
            facts(move || animation(selected.get())),
            playback(speed),
        ))
        .spacing(12.0)
        .padding(16.0),
    )
}

/// The playground's animation height in points: room for any of the animations to read at a
/// glance on a phone, with the page's text and controls still in reach below it.
const PLAYGROUND_STAGE: f64 = 300.0;

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
