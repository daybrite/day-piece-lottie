// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The headless reader, against the files under `tests/data/`: the animation the demo app ships,
//! a richer one that exercises every arm of the reader, and four deliberately broken siblings.

use day_piece_lottie::model::{AssetKind, Issue, LayerKind, LottieError, LottieModel};

const HELLO: &str = include_str!("data/hello.json");
const RICH: &str = include_str!("data/rich.json");
const NO_FRAMES: &str = include_str!("data/no-frames.json");
const EMPTY_LAYERS: &str = include_str!("data/empty-layers.json");
const MISSING_ASSET: &str = include_str!("data/missing-asset.json");
const TRUNCATED: &str = include_str!("data/truncated.json");
const HAMBURGER: &str = include_str!("data/hamburger-arrow.json");

/// The demo's animation, as the demo and the Showcase display it: what the on-device labels show
/// is exactly what this test asserts on the host.
#[test]
fn hello_reads_back_its_facts() {
    let model = LottieModel::parse(HELLO).expect("hello.json parses");
    assert_eq!(model.name.as_deref(), Some("hello"));
    assert_eq!(model.version.as_deref(), Some("5.9.0"));
    assert_eq!(model.frame_rate, 30.0);
    assert_eq!((model.in_point, model.out_point), (0.0, 60.0));
    assert_eq!(model.frames(), 60.0);
    assert_eq!(model.duration_secs(), 2.0);
    assert_eq!((model.width, model.height), (200.0, 200.0));
    assert!(!model.three_d);
    assert_eq!(model.layers.len(), 1);
    assert_eq!(model.layers[0].kind, LayerKind::Shape);
    assert_eq!(model.layers[0].name.as_deref(), Some("square"));
    assert!(model.assets.is_empty());
    assert!(model.markers.is_empty());
    assert!(model.verify().is_empty(), "{:?}", model.verify());
}

/// Bytes and text are the same reader.
#[test]
fn bytes_and_text_agree() {
    let a = LottieModel::parse(HELLO).expect("text");
    let b = LottieModel::from_slice(HELLO.as_bytes()).expect("bytes");
    assert_eq!(a, b);
}

/// Assets, precomps, markers, and layer references all read back.
#[test]
fn rich_reads_assets_layers_and_markers() {
    let model = LottieModel::parse(RICH).expect("rich.json parses");
    assert_eq!(model.layers.len(), 3);
    assert_eq!(model.layers[1].kind, LayerKind::Image);
    assert_eq!(model.layers[1].ref_id.as_deref(), Some("img_0"));
    assert_eq!(model.layers[2].kind, LayerKind::Precomp);
    assert_eq!(model.layers[2].in_point, 5.0);

    let img = model.asset("img_0").expect("image asset");
    assert_eq!(
        img.kind,
        AssetKind::Image {
            file: Some("photo.png".into()),
            embedded: false
        }
    );
    let comp = model.asset("comp_0").expect("precomp asset");
    assert_eq!(comp.kind, AssetKind::Precomp { layers: 1 });
    assert!(model.asset("nope").is_none());

    assert_eq!(model.markers.len(), 2);
    assert_eq!(model.markers[1].name, "loop");
    assert_eq!(
        (model.markers[1].frame, model.markers[1].duration),
        (30.0, 30.0)
    );
    assert!(model.verify().is_empty(), "{:?}", model.verify());
}

/// Each broken file names its one defect, in words a label can show.
#[test]
fn broken_files_name_their_defect() {
    let model = LottieModel::parse(NO_FRAMES).expect("parses; the defect is semantic");
    let issues = model.verify();
    assert_eq!(
        issues,
        [Issue::NoFrames {
            in_point: 0.0,
            out_point: 0.0
        }]
    );
    assert_eq!(model.duration_secs(), 0.0);
    assert_eq!(
        issues[0].to_string(),
        "no frames to play: the out point (0) is not after the in point (0)"
    );

    let model = LottieModel::parse(EMPTY_LAYERS).expect("parses");
    assert_eq!(model.verify(), [Issue::EmptyLayers]);

    let model = LottieModel::parse(MISSING_ASSET).expect("parses");
    assert_eq!(
        model.verify(),
        [Issue::MissingAsset {
            layer: 1,
            id: "img_0".into()
        }]
    );
    assert_eq!(
        model.verify()[0].to_string(),
        "layer 1 references the asset `img_0`, which is not in the file"
    );
}

/// What cannot be read at all is an error, not a model with issues.
#[test]
fn unreadable_documents_are_errors() {
    assert!(matches!(
        LottieModel::parse(TRUNCATED),
        Err(LottieError::Json(_))
    ));
    assert_eq!(LottieModel::parse("[1, 2]"), Err(LottieError::NotAnObject));
    assert_eq!(
        LottieModel::parse(r#"{"fr": 30, "ip": 0, "op": 60, "w": 10}"#),
        Err(LottieError::MissingField("h"))
    );
    let err = LottieModel::parse(r#"{"fr": "fast", "ip": 0, "op": 60, "w": 10, "h": 10}"#)
        .expect_err("fr is a string");
    assert_eq!(err.to_string(), "the field `fr` is not a number");
}

/// Layer-level checks: an unknown type number and a layer that ends before it starts.
#[test]
fn layer_defects_are_reported_by_index() {
    let text = r#"{"fr": 24, "ip": 0, "op": 48, "w": 100, "h": 100, "layers": [
        {"ty": 4, "ip": 0, "op": 48},
        {"ty": 99, "ip": 0, "op": 48},
        {"ty": 4, "ip": 20, "op": 10}
    ]}"#;
    let model = LottieModel::parse(text).expect("parses");
    assert_eq!(model.layers[1].kind, LayerKind::Unknown(99));
    assert_eq!(model.layers[1].kind.to_string(), "unknown type 99");
    assert_eq!(
        model.verify(),
        [
            Issue::UnknownLayerType { layer: 1, ty: 99 },
            Issue::LayerNeverShows { layer: 2 },
        ]
    );
}

/// The second animation the demo bundles (Airbnb's `HamburgerArrow.json`, Apache-2.0): the facts
/// `demo/dayscript/lottie.yaml` asserts after the picker swaps to it.
#[test]
fn hamburger_arrow_reads_back_its_facts() {
    let model = LottieModel::parse(HAMBURGER).expect("hamburger-arrow.json parses");
    assert_eq!(
        model.name, None,
        "the exporter recorded no composition name"
    );
    assert_eq!(model.frames(), 180.0);
    assert_eq!(model.frame_rate, 30.0);
    assert_eq!(model.duration_secs(), 6.0);
    assert_eq!((model.width, model.height), (400.0, 300.0));
    assert_eq!(model.layers.len(), 4);
    let shapes = model
        .layers
        .iter()
        .filter(|l| l.kind == LayerKind::Shape)
        .count();
    let nulls = model
        .layers
        .iter()
        .filter(|l| l.kind == LayerKind::Null)
        .count();
    assert_eq!((shapes, nulls), (3, 1), "three shapes under a null parent");
    assert!(model.verify().is_empty(), "{:?}", model.verify());
}
