// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! Lottie Demo: the demo and on-device test app for `day-piece-lottie`.
//!
//! A navigation app of one page per bundled animation, and a playground page in front of them.
//! Every page, and every animation file, comes from `day-piece-lottie-gallery`, the crate that
//! holds them for any app that wants to show them (Day-Showcase mounts the same pages), so this
//! app is only the shell: a window, a sidebar, and the routes. The animations arrive in its bundle
//! as the gallery's data assets; nothing is copied here.
//!
//! The route is the address: it is the URL hash on web-dom, so <https://…/#pin-jump> opens that
//! animation on a fresh load and the browser's history walks the pages, and it is the name
//! `dayscript`'s `navigate:` step addresses. Every element carries a stable id, so the
//! walkthroughs assert what is on a page as well as reach it.

use day::prelude::*;
use day_piece_lottie_gallery::{self as gallery, LottiePage};

// The mobile and web entry point; a plain cargo desktop build enters through src/main.rs.
day::day_start!(options: window(), root);

// Typed constants for everything under `resource/` (https://daybrite.dev/docs/how-to/resources).
day::resources!();

/// The window every entry point opens.
pub fn window() -> day::WindowOptions {
    res::window_options().size(480.0, 800.0)
}

/// The whole app: a sidebar of the playground and every bundled animation, and whichever page is
/// open.
pub fn root() -> impl Piece {
    info!("Lottie Demo starting");

    // The open page. The nav writes it, a launch deep link writes it before the first frame (the
    // URL hash on web-dom, `DAY_DEEPLINK` elsewhere), and the browser's back button writes it
    // again.
    let page = Signal::new(LottiePage::Playground);
    // The playground under its own heading, then one row per animation, in the gallery's order.
    let (playground, animations): (Vec<_>, Vec<_>) = gallery::pages()
        .into_iter()
        .partition(|entry| entry.page == LottiePage::Playground);
    let rows = |entries: Vec<gallery::Entry>| -> Vec<NavItem<LottiePage>> {
        entries
            .into_iter()
            .map(|entry| item(entry.page, (entry.title)()).page(move || entry.build()))
            .collect()
    };
    nav(page)
        .style(NavStyle::Sidebar)
        .title(res::str::app_title())
        .section(res::str::nav_playground(), rows(playground))
        .section(res::str::nav_animations(), rows(animations))
        .id(day::id!(nav))
}
