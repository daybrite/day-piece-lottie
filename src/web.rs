// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The web-view arm: Lottie on every backend without a native player.
//!
//! iOS and Android have Airbnb's own renderers, and this crate binds them (src/lib-uikit.rs,
//! src/DayLottie.java). The other six backends have no Lottie library to bind, but every one of
//! them has a web engine that [`day_piece_webview`] already drives, and Airbnb ships lottie-web
//! for exactly this. So the piece composes: one [`day_piece_webview::web_view_inline`] showing
//! `web/index.html` from this crate's own bundled assets, playing the animation the app named.
//!
//! The front end is the same either way — `lottie("hello").looping(false).speed(sig)` — and so is
//! the file: the app ships `hello.json` under `resource/assets/`, and that one file is what plays.
//!
//! How the file reaches the page depends on what the engine allows:
//!
//! * Where the backend evaluates JavaScript ([`eval_support`] — every engine here but WebKitGTK),
//!   this side reads the file through day's own resource opener and hands the page its text. That
//!   is also the only way on WebKit: a page opened from a `file:` URL may not read a file with
//!   `XMLHttpRequest`, not even one beside it, so an animation it had to fetch would never load.
//! * WebKitGTK has no evaluation arm yet, so its page is told the name in its query string and
//!   fetches the file itself, which its own file-URL policy allows once the view asks for the
//!   app's assets (`app_assets`). web-dom fetches too, over http, from the deployed asset tree.
//!
//! Two limits worth knowing, both from the engines rather than from here:
//!
//! * On gtk and web-dom the animation and the rate are what the page was opened with, because
//!   changing either afterwards means evaluating something. A name or speed that changes later
//!   reaches the view on the backends that evaluate, and elsewhere takes effect the next time the
//!   piece is built — which navigation does anyway.
//! * `macos-gtk` and `windows-gtk` have no WebKitGTK at all, so there is no engine to host the
//!   player; the view reports [`day_spec::Support::Unsupported`] and realizes the placeholder,
//!   the same answer `day_piece_webview` gives there.

use day_core::{BuildCx, Piece, RNode};
use day_piece_webview::{EvalError, JsHandle, eval_support, web_view_inline};
use day_pieces::{Reactive, TextSource};
use day_reactive::bind_seeded;
use day_spec::{AssetDir, AssetName, Support};

/// Where `[package.metadata.day.piece].assets` stages this crate's `web/` directory in every
/// app's bundle. The namespace is the crate name, which the CLI chooses, not this file.
const SITE_ROOT: &str = "day-piece-lottie";
/// The page within that site, relative to it.
const SITE_PAGE: &str = "index.html";

/// The page's query string: what a backend that cannot evaluate JavaScript has to be told up
/// front (web/index.html). An empty `name` opens the page with an empty stage, which is what the
/// evaluating backends do before handing it the animation's text.
fn start_page(name: &str, looping: bool, autoplay: bool, speed: f64) -> String {
    format!(
        "{SITE_PAGE}?src={}&loop={}&autoplay={}&speed={}",
        encode(name),
        u8::from(looping),
        u8::from(autoplay),
        speed,
    )
}

/// Percent-encode what a Lottie name can hold. Names are file paths under `resource/assets/`, so
/// this is the small set that would otherwise end the value or open a second parameter, plus the
/// space that a hand-written name occasionally carries.
fn encode(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for c in name.chars() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' | '/' => out.push(c),
            _ => {
                let mut buf = [0u8; 4];
                for b in c.encode_utf8(&mut buf).as_bytes() {
                    out.push_str(&format!("%{b:02X}"));
                }
            }
        }
    }
    out
}

/// A JavaScript string literal for `s`, for the evaluated `dayLottie.…` calls.
fn js_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('\'');
    for c in s.chars() {
        match c {
            '\'' | '\\' => {
                out.push('\\');
                out.push(c);
            }
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            c => out.push(c),
        }
    }
    out.push('\'');
    out
}

/// The animation's text, read through day's resource opener: the same bundled file the native
/// arms hand to Airbnb's renderer, named the way the app named it (`"hello"` → `hello.json`, a
/// path allowed: `"lottie/pin-jump"`).
fn animation_text(name: &str) -> Option<String> {
    let file = if name.ends_with(".json") {
        name.to_string()
    } else {
        format!("{name}.json")
    };
    let res = day_spec::resource(AssetName::dynamic(file))?;
    String::from_utf8(res.to_vec()).ok()
}

/// The call that puts an animation on the page: its text where the file is readable, and the
/// failure in the page's own error box where it is not, because a player that quietly shows
/// nothing looks exactly like one that is between frames.
///
/// The text goes in as a JavaScript object literal, which is what JSON is.
fn load_call(name: &str, looping: bool, autoplay: bool, speed: f64) -> String {
    match animation_text(name) {
        Some(json) => format!(
            "window.dayLottie&&dayLottie.loadData({},{},{},{})",
            json, looping, autoplay, speed,
        ),
        None => format!(
            "window.dayLottie&&dayLottie.fail({})",
            js_string(&format!("no animation named {name}")),
        ),
    }
}

/// How long to wait for the page's own script before giving up on it, as a number of tries and
/// the pause between them. A page this small is ready within a frame or two of the view being
/// realized; the ceiling is for a machine under load, and reaching it means something is wrong
/// rather than slow.
const READY_TRIES: u32 = 60;
const READY_PAUSE_MS: u64 = 50;

/// Run `script` in the page and drop the reply: an eval dispatches nothing until its future is
/// polled, and none of these calls has an answer worth waiting for.
fn tell(js: JsHandle, script: String) {
    let call = js.eval(script);
    day_core::task(async move {
        let _ = call.await;
    });
}

/// Run `script` once the page can run it.
///
/// A view is realized before the page it points at has loaded, and an evaluation that lands in
/// between runs in a document that has no `window.dayLottie` in it yet and is lost without a
/// trace. So ask the page whether it is there, and hand it the animation when it answers.
fn tell_when_ready(js: JsHandle, script: String) {
    day_core::task(async move {
        for _ in 0..READY_TRIES {
            match js.eval("typeof window.dayLottie").await {
                Ok(reply) if reply.contains("object") => {
                    let _ = js.eval(script).await;
                    return;
                }
                // The view is gone: navigation replaced it, and whatever is on screen now is
                // loading its own animation.
                Err(EvalError::ViewGone) => return,
                _ => {}
            }
            let (deliver, waited) = day_async::oneshot::<()>();
            day_async::schedule(
                std::time::Duration::from_millis(READY_PAUSE_MS),
                move || deliver.send(()),
            );
            if waited.await.is_err() {
                return;
            }
        }
        log::warn!("day-piece-lottie: the player page never answered; nothing is playing");
    });
}

/// Build the web-view-backed animation. Mirrors the native arms' contract: a growing leaf that
/// fills what it is given, playing `name` from the app's bundled assets.
pub(crate) fn build(
    cx: &mut BuildCx,
    name: TextSource,
    looping: bool,
    autoplay: bool,
    speed: Reactive<f64>,
) -> RNode {
    let initial_name = name.initial();
    let initial_speed = speed.get_untracked();
    // Where the page is told its animation, it opens with an empty stage and is handed the text
    // below; where it fetches, the name rides in the query string.
    let evaluates = eval_support() == Support::Native;
    let opening = if evaluates { "" } else { &initial_name };
    // The page lives in this crate's own asset directory and the animation in the app's, one
    // level above it, so a fetching page asks for the app's whole asset tree (`app_assets`):
    // that is what makes `../hello.json` reach the file on an engine that confines a bundled
    // site to its own directory (the GTK cache extraction, WebKit's file-URL read access).
    let site = AssetDir::dynamic(SITE_ROOT.to_string());
    let js = JsHandle::new();
    let view = web_view_inline(site)
        .app_assets()
        .start_page(start_page(opening, looping, autoplay, initial_speed))
        .js(js);

    // No `.grow()` here: a web view already fills what it is offered, and a decorator around it
    // would take the id the app puts on `lottie(…)`, leaving the engine unreachable to
    // `web_eval` and to the piece's own patches.
    let node = view.build(cx);

    if evaluates {
        // The animation itself. After the build, because that is what binds the handle to the
        // view: an eval issued before it has nothing to run in and answers `ViewGone`.
        tell_when_ready(
            js,
            load_call(&initial_name, looping, autoplay, initial_speed),
        );
        // A bound rate reaches the running animation; a constant one is already in the call
        // above and never fires here.
        bind_seeded(
            initial_speed,
            move || speed.get(),
            move |v: &f64| tell(js, format!("window.dayLottie&&dayLottie.setSpeed({v})")),
        );
        // A bound name swaps what is playing, the same change the native arms push as a patch.
        if let TextSource::Dyn(read) = name {
            bind_seeded(
                initial_name,
                move || read(),
                move |n: &String| {
                    tell_when_ready(js, load_call(n, looping, autoplay, initial_speed))
                },
            );
        }
    }
    node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_page_carries_every_option() {
        let page = start_page("hello", false, true, 1.5);
        assert_eq!(page, "index.html?src=hello&loop=0&autoplay=1&speed=1.5");
    }

    #[test]
    fn a_told_page_opens_empty() {
        assert_eq!(
            start_page("", true, true, 1.0),
            "index.html?src=&loop=1&autoplay=1&speed=1"
        );
    }

    #[test]
    fn a_name_with_a_path_keeps_its_slashes() {
        // The page resolves `../<src>.json`, so a slash is part of the path, not something to
        // escape; a space is not.
        assert_eq!(encode("lottie/pin jump"), "lottie/pin%20jump");
    }

    #[test]
    fn a_js_string_cannot_be_escaped_out_of() {
        assert_eq!(js_string("it's"), r"'it\'s'");
        assert_eq!(js_string(r"a\b"), r"'a\\b'");
    }

    #[test]
    fn a_missing_animation_reports_itself() {
        // No resource opener and no asset root in a host test, so every name is missing: what
        // matters is that the page is told so rather than left blank.
        let call = load_call("nowhere", true, true, 1.0);
        assert!(
            call.contains("dayLottie.fail('no animation named nowhere')"),
            "{call}"
        );
    }
}
