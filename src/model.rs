// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! The headless half of the piece: read a Lottie document, report what is in it, and say what a
//! player would refuse — with no toolkit in the loop.
//!
//! [`lottie`](crate::lottie) hands a bundled file to the platform's `LottieAnimationView` and
//! learns nothing about it. Everything here runs before that, or instead of it: an app can show
//! an animation's length beside it, a build step can check every file under `resource/assets/`,
//! and a test can assert what a file contains on any host, including the iOS and Android
//! binaries a dayscript drives.
//!
//! The reader is deliberately shallow. It takes the top-level facts (version, name, frame rate,
//! in and out points, size), each layer's identity and timing, the asset ids, and the markers, and
//! ignores everything else — shapes, keyframes, expressions — so files written by a newer
//! exporter parse. A document is small, so it is read whole from a string or a byte slice; the
//! app knows where its assets live and this crate does not.

use std::fmt;

use serde_json::Value;

/// A Lottie document's top-level facts, plus its layers, assets, and markers.
#[derive(Clone, Debug, PartialEq)]
pub struct LottieModel {
    /// The exporter version (`v`), e.g. `"5.9.0"`, when the file records one.
    pub version: Option<String>,
    /// The composition name (`nm`), when the file records one.
    pub name: Option<String>,
    /// Frames per second (`fr`).
    pub frame_rate: f64,
    /// The first frame (`ip`).
    pub in_point: f64,
    /// The frame after the last one (`op`); `op - ip` is the frame count.
    pub out_point: f64,
    /// Composition width in pixels (`w`).
    pub width: f64,
    /// Composition height in pixels (`h`).
    pub height: f64,
    /// Whether the composition is flagged as 3D (`ddd`).
    pub three_d: bool,
    /// The root layers, in file order.
    pub layers: Vec<Layer>,
    /// The assets a layer may reference by id (`assets`).
    pub assets: Vec<Asset>,
    /// Named points on the timeline (`markers`), in file order.
    pub markers: Vec<Marker>,
}

/// One root layer: what it is and when it plays.
#[derive(Clone, Debug, PartialEq)]
pub struct Layer {
    /// The layer's index in the file (`ind`), when it records one.
    pub index: Option<u64>,
    /// The layer name (`nm`), when it records one.
    pub name: Option<String>,
    /// The layer type (`ty`).
    pub kind: LayerKind,
    /// The frame the layer appears on (`ip`).
    pub in_point: f64,
    /// The frame the layer disappears on (`op`).
    pub out_point: f64,
    /// The asset this layer draws (`refId`), for image and precomp layers.
    pub ref_id: Option<String>,
}

/// The layer types the format defines (`ty`), plus a catch-all for a number this crate does
/// not know.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayerKind {
    Precomp,
    Solid,
    Image,
    Null,
    Shape,
    Text,
    Audio,
    VideoPlaceholder,
    ImageSequence,
    Video,
    ImagePlaceholder,
    Guide,
    Adjustment,
    Camera,
    Light,
    Data,
    Unknown(u64),
}

impl LayerKind {
    fn from_ty(ty: u64) -> Self {
        match ty {
            0 => LayerKind::Precomp,
            1 => LayerKind::Solid,
            2 => LayerKind::Image,
            3 => LayerKind::Null,
            4 => LayerKind::Shape,
            5 => LayerKind::Text,
            6 => LayerKind::Audio,
            7 => LayerKind::VideoPlaceholder,
            8 => LayerKind::ImageSequence,
            9 => LayerKind::Video,
            10 => LayerKind::ImagePlaceholder,
            11 => LayerKind::Guide,
            12 => LayerKind::Adjustment,
            13 => LayerKind::Camera,
            14 => LayerKind::Light,
            15 => LayerKind::Data,
            other => LayerKind::Unknown(other),
        }
    }
}

impl fmt::Display for LayerKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LayerKind::Precomp => f.write_str("precomp"),
            LayerKind::Solid => f.write_str("solid"),
            LayerKind::Image => f.write_str("image"),
            LayerKind::Null => f.write_str("null"),
            LayerKind::Shape => f.write_str("shape"),
            LayerKind::Text => f.write_str("text"),
            LayerKind::Audio => f.write_str("audio"),
            LayerKind::VideoPlaceholder => f.write_str("video placeholder"),
            LayerKind::ImageSequence => f.write_str("image sequence"),
            LayerKind::Video => f.write_str("video"),
            LayerKind::ImagePlaceholder => f.write_str("image placeholder"),
            LayerKind::Guide => f.write_str("guide"),
            LayerKind::Adjustment => f.write_str("adjustment"),
            LayerKind::Camera => f.write_str("camera"),
            LayerKind::Light => f.write_str("light"),
            LayerKind::Data => f.write_str("data"),
            LayerKind::Unknown(n) => write!(f, "unknown type {n}"),
        }
    }
}

/// An entry in the document's asset list.
#[derive(Clone, Debug, PartialEq)]
pub struct Asset {
    /// The id layers reference (`id`).
    pub id: String,
    /// What the asset is.
    pub kind: AssetKind,
}

/// The two asset shapes a player loads: a bitmap, or a nested composition.
#[derive(Clone, Debug, PartialEq)]
pub enum AssetKind {
    /// An image: its file name (`p`), and whether the file is embedded as a data URI (`e`).
    Image {
        file: Option<String>,
        embedded: bool,
    },
    /// A precomposition: how many layers it holds.
    Precomp { layers: usize },
}

/// A named point on the timeline.
#[derive(Clone, Debug, PartialEq)]
pub struct Marker {
    /// The marker's comment (`cm`), which is what a player exposes as its name.
    pub name: String,
    /// The frame the marker sits on (`tm`).
    pub frame: f64,
    /// How many frames it spans (`dr`); zero for a point.
    pub duration: f64,
}

/// Why a document could not be read at all. Anything a reader can parse is a [`LottieModel`],
/// and what is wrong with it is a matter for [`LottieModel::verify`].
#[derive(Clone, Debug, PartialEq)]
pub enum LottieError {
    /// The text is not JSON.
    Json(String),
    /// The JSON is not an object.
    NotAnObject,
    /// A required top-level number is absent.
    MissingField(&'static str),
    /// A field is present with the wrong shape.
    BadField {
        field: &'static str,
        expected: &'static str,
    },
}

impl fmt::Display for LottieError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LottieError::Json(e) => write!(f, "not valid JSON: {e}"),
            LottieError::NotAnObject => f.write_str("the document is not a JSON object"),
            LottieError::MissingField(name) => {
                write!(f, "the required field `{name}` is missing")
            }
            LottieError::BadField { field, expected } => {
                write!(f, "the field `{field}` is not {expected}")
            }
        }
    }
}

impl std::error::Error for LottieError {}

/// Something a player will refuse or render badly. `Display` gives one plain sentence, so a
/// label or a test assertion can show it as is.
#[derive(Clone, Debug, PartialEq)]
pub enum Issue {
    /// `op` is not after `ip`: the animation has no frames to play.
    NoFrames { in_point: f64, out_point: f64 },
    /// `fr` is zero or negative: no frame lasts any time at all.
    ZeroFrameRate { frame_rate: f64 },
    /// `w` or `h` is zero or negative: nothing can be drawn.
    NoSize { width: f64, height: f64 },
    /// The layer list is empty: the composition is blank.
    EmptyLayers,
    /// A layer references an asset id the asset list does not carry.
    MissingAsset { layer: usize, id: String },
    /// A layer's type number is one this crate (and likely the player) does not know.
    UnknownLayerType { layer: usize, ty: u64 },
    /// A layer's out point is not after its in point: it never shows.
    LayerNeverShows { layer: usize },
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Issue::NoFrames {
                in_point,
                out_point,
            } => write!(
                f,
                "no frames to play: the out point ({out_point}) is not after the in point ({in_point})"
            ),
            Issue::ZeroFrameRate { frame_rate } => {
                write!(
                    f,
                    "the frame rate is {frame_rate}, so no frame lasts any time"
                )
            }
            Issue::NoSize { width, height } => {
                write!(
                    f,
                    "the composition is {width} × {height}, so nothing can be drawn"
                )
            }
            Issue::EmptyLayers => f.write_str("the composition has no layers"),
            Issue::MissingAsset { layer, id } => {
                write!(
                    f,
                    "layer {layer} references the asset `{id}`, which is not in the file"
                )
            }
            Issue::UnknownLayerType { layer, ty } => {
                write!(f, "layer {layer} has the unknown type {ty}")
            }
            Issue::LayerNeverShows { layer } => {
                write!(f, "layer {layer} ends before it starts, so it never shows")
            }
        }
    }
}

impl LottieModel {
    /// Read a document from its JSON text.
    pub fn parse(text: &str) -> Result<Self, LottieError> {
        let value: Value =
            serde_json::from_str(text).map_err(|e| LottieError::Json(e.to_string()))?;
        Self::from_value(&value)
    }

    /// Read a document from its JSON bytes (a file read into memory, an asset fetched at run time).
    pub fn from_slice(bytes: &[u8]) -> Result<Self, LottieError> {
        let value: Value =
            serde_json::from_slice(bytes).map_err(|e| LottieError::Json(e.to_string()))?;
        Self::from_value(&value)
    }

    fn from_value(value: &Value) -> Result<Self, LottieError> {
        let obj = value.as_object().ok_or(LottieError::NotAnObject)?;
        let number = |field: &'static str| -> Result<f64, LottieError> {
            match obj.get(field) {
                None => Err(LottieError::MissingField(field)),
                Some(v) => v.as_f64().ok_or(LottieError::BadField {
                    field,
                    expected: "a number",
                }),
            }
        };
        let text = |field: &'static str| -> Result<Option<String>, LottieError> {
            match obj.get(field) {
                None | Some(Value::Null) => Ok(None),
                Some(v) => v
                    .as_str()
                    .map(|s| Some(s.to_string()))
                    .ok_or(LottieError::BadField {
                        field,
                        expected: "a string",
                    }),
            }
        };
        let list = |field: &'static str| -> Result<&[Value], LottieError> {
            match obj.get(field) {
                None | Some(Value::Null) => Ok(&[]),
                Some(v) => v
                    .as_array()
                    .map(Vec::as_slice)
                    .ok_or(LottieError::BadField {
                        field,
                        expected: "an array",
                    }),
            }
        };

        let layers = list("layers")?.iter().map(Layer::from_value).collect();
        let assets = list("assets")?
            .iter()
            .filter_map(Asset::from_value)
            .collect();
        let markers = list("markers")?
            .iter()
            .filter_map(Marker::from_value)
            .collect();
        Ok(LottieModel {
            version: text("v")?,
            name: text("nm")?,
            frame_rate: number("fr")?,
            in_point: number("ip")?,
            out_point: number("op")?,
            width: number("w")?,
            height: number("h")?,
            three_d: obj.get("ddd").and_then(Value::as_f64).unwrap_or(0.0) != 0.0,
            layers,
            assets,
            markers,
        })
    }

    /// How many frames the animation plays: `op - ip`.
    pub fn frames(&self) -> f64 {
        self.out_point - self.in_point
    }

    /// How long one pass takes at the file's frame rate, in seconds. Zero for a document whose
    /// frame rate is not positive, rather than infinity, so a label can still show it.
    pub fn duration_secs(&self) -> f64 {
        if self.frame_rate > 0.0 {
            self.frames() / self.frame_rate
        } else {
            0.0
        }
    }

    /// The asset with this id, if the document carries one.
    pub fn asset(&self, id: &str) -> Option<&Asset> {
        self.assets.iter().find(|a| a.id == id)
    }

    /// Everything a player would refuse or render badly, in file order. Empty means the document
    /// is playable as far as its structure goes; it says nothing about how it looks.
    pub fn verify(&self) -> Vec<Issue> {
        let mut issues = Vec::new();
        if self.out_point <= self.in_point {
            issues.push(Issue::NoFrames {
                in_point: self.in_point,
                out_point: self.out_point,
            });
        }
        if self.frame_rate <= 0.0 {
            issues.push(Issue::ZeroFrameRate {
                frame_rate: self.frame_rate,
            });
        }
        if self.width <= 0.0 || self.height <= 0.0 {
            issues.push(Issue::NoSize {
                width: self.width,
                height: self.height,
            });
        }
        if self.layers.is_empty() {
            issues.push(Issue::EmptyLayers);
        }
        for (i, layer) in self.layers.iter().enumerate() {
            if let LayerKind::Unknown(ty) = layer.kind {
                issues.push(Issue::UnknownLayerType { layer: i, ty });
            }
            if let Some(id) = &layer.ref_id
                && self.asset(id).is_none()
            {
                issues.push(Issue::MissingAsset {
                    layer: i,
                    id: id.clone(),
                });
            }
            if layer.out_point <= layer.in_point {
                issues.push(Issue::LayerNeverShows { layer: i });
            }
        }
        issues
    }
}

impl Layer {
    fn from_value(v: &Value) -> Self {
        let ty = v.get("ty").and_then(Value::as_u64).unwrap_or(u64::MAX);
        Layer {
            index: v.get("ind").and_then(Value::as_u64),
            name: v.get("nm").and_then(Value::as_str).map(str::to_string),
            kind: LayerKind::from_ty(ty),
            in_point: v.get("ip").and_then(Value::as_f64).unwrap_or(0.0),
            out_point: v.get("op").and_then(Value::as_f64).unwrap_or(0.0),
            ref_id: v.get("refId").and_then(Value::as_str).map(str::to_string),
        }
    }
}

impl Asset {
    fn from_value(v: &Value) -> Option<Self> {
        let id = v.get("id")?.as_str()?.to_string();
        let kind = match v.get("layers").and_then(Value::as_array) {
            Some(layers) => AssetKind::Precomp {
                layers: layers.len(),
            },
            None => AssetKind::Image {
                file: v.get("p").and_then(Value::as_str).map(str::to_string),
                embedded: v.get("e").and_then(Value::as_f64).unwrap_or(0.0) != 0.0,
            },
        };
        Some(Asset { id, kind })
    }
}

impl Marker {
    fn from_value(v: &Value) -> Option<Self> {
        Some(Marker {
            name: v.get("cm")?.as_str()?.to_string(),
            frame: v.get("tm").and_then(Value::as_f64).unwrap_or(0.0),
            duration: v.get("dr").and_then(Value::as_f64).unwrap_or(0.0),
        })
    }
}
