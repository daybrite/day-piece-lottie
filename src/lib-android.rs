// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

// ---------------------------------------------------------------------------
// Android: a LottieAnimationView from com.airbnb.android:lottie, created by this crate's Java
// (src/DayLottie.java), folded into the app's Gradle build via
// [package.metadata.day.android] (which also declares the Gradle dependency), without touching
// day-mdc. Rust calls its own class through the re-exported `jni`.
// ---------------------------------------------------------------------------

use super::*;
use day_mdc::jni::objects::JString;
use day_mdc::{AHandle, Android, java};
use day_spec::NodeId;

// This piece's Java class (src/DayLottie.java, on the app classpath at build).
day_mdc::java_class! {
    DayLottie => dev.daybrite.day.piece.lottie.DayLottie,
    methods {
        static fn makeLottie(name: JString, looping: jboolean, autoplay: jboolean, speed: jfloat)
            -> android.view.View,
        static fn setSpeed(view: android.view.View, speed: jfloat),
        static fn setAnimation(view: android.view.View, name: JString),
    },
}

fn make(_backend: &mut Android, p: &LottieProps, _id: NodeId) -> AHandle {
    // A Java throw (e.g. the Lottie dependency missing from the app build) must not panic
    // inside realize: the panic unwinds the JNI up-call and aborts. Placeholder instead.
    day_mdc::realize("lottie", |env| {
        let name = JString::from_str(env, &p.name)?;
        DayLottie::makeLottie(env, &name, p.looping, p.autoplay, p.speed as f32)
    })
}

fn update(_backend: &mut Android, h: &AHandle, patch: &LottiePatch) {
    match patch {
        LottiePatch::Speed(s) => {
            java(|env| DayLottie::setSpeed(env, h.0.as_obj(), *s as f32));
        }
        LottiePatch::Name(n) => {
            java(|env| {
                let name = JString::from_str(env, n)?;
                DayLottie::setAnimation(env, h.0.as_obj(), &name)
            });
        }
    }
}

// looping/autoplay are set once at build; `name` and `speed` patch. `fill_measure` gives the uniform
// growing-leaf sizing (which day-mdc's `measure: None` default would otherwise collapse).
day_pieces::renderer!(day_mdc::RENDERERS, Android,
    kind: KIND, props: LottieProps, patch: LottiePatch, make: make, update: update,
    measure: day_pieces::fill_measure);
