//! Model-based definition and product manufacturing information.

use crate::validation::Validate;
use crate::Extras;
use gltf_derive::Validate;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};

/// The drawing area for MBD items.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
pub struct Overlay {
    /// Optional name for this overlay.
    #[cfg(feature = "names")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Feature control frames.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frames: Vec<Frame>,

    /// Datum target points.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<Target>,

    /// Human-readable textual information.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_blocks: Vec<TextBlock>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Extras::is_empty"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

/// Symbol part of a leader line.
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LeaderLineSymbol {
    /// ●
    ///
    /// Filled circle.
    FilledCircle,

    /// ○
    ///
    /// Hollow circle.
    HollowCircle,

    /// ▶
    ///
    /// Filled triangle.
    FilledTriangle,

    /// ▷
    ///
    /// Hollow triangle.
    HollowTriangle,

    /// ➤
    ///
    /// Filled arrow head.
    FilledArrow,
}

impl Validate for LeaderLineSymbol {}

/// Waypoint for a leader line.
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Serialize, Validate)]
pub struct LeaderLinePoint {
    /// X coordinate in the overlay plane.
    pub x: f64,

    /// Y coordinate in the overlay plane.
    pub y: f64,

    /// Endpoint symbol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<LeaderLineSymbol>,
}

/// A feature control frame.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
pub struct Frame {
    /// Optional name for this frame.
    #[cfg(feature = "names")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Text to appear above the feature control box.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_above: Option<String>,

    /// Grouping feature.
    ///
    /// Intended for multi-row feature callout boxes, this specifies a feature that groups
    /// the callouts in the MBD frame.
    ///
    /// For example, two tolerances are grouped the position tolerance feature represented by ⌖ in the diagram below.
    ///
    /// ```
    /// ┌───────┬───────┬───┐
    /// │       │ ⌀ .04 │ A │
    /// │   ⌖   ├───────┼───┤
    /// │       │ ⌀ .02 │ B │
    /// └───────┴───────┴───┘
    /// ```
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Feature>,

    /// Callouts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub callouts: Vec<Callout>,

    /// Text to appear below the feature control box.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_below: Option<String>,

    /// The leader line array begins at the callout boundary and ends at the model boundary.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub leader_line: Vec<LeaderLinePoint>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Extras::is_empty"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

impl Frame {
    pub fn new(callouts: Vec<Callout>) -> Self {
        Self {
            name: None,
            callouts,
            text_above: None,
            text_below: None,
            group: None,
            leader_line: Vec::new(),
            extras: Default::default(),
        }
    }
}

/// A datum target point.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
pub struct Target {
    /// Target area size.
    pub size: String,

    /// The datum target.
    pub datum: Datum,
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Serialize)]
pub enum Modifier {
    /// Diameter tolerance zone.
    #[serde(rename = "D")]
    Diameter,

    /// Only gravity applies to the part.
    #[serde(rename = "F")]
    FreeState,

    /// Perfect form not required at MMC nor LMC.
    #[serde(rename = "I")]
    Independency,

    /// Least material condition.
    #[serde(rename = "L")]
    LeastMaterialCondition,

    /// Maximum material condition.
    #[serde(rename = "M")]
    MaximumMaterialCondition,

    /// Projected tolerance zone.
    #[serde(rename = "P")]
    ProjectedToleranceZone,

    /// Unequally displaced profile tolerance zone.
    #[serde(rename = "U")]
    UnequallyDisplacedProfile,

    /// Tangent plane of a surface.
    #[serde(rename = "T")]
    TangentPlane,
}

impl Validate for Modifier {}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
pub struct Datum {
    /// The ID of the datum feature.
    pub id: String,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Extras::is_empty"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Feature {
    Straightness,
    Flatness,
    Circularity,
    Cylindricity,
    Parallelism,
    Perpendicularity,
    Angularity,

    LineProfile,
    SurfaceProfile,

    Position,
    Concentricity,
    Symmetry,

    Runout,
    TotalRunout,
}

impl Validate for Feature {}

/// A tolerance specification for a particular feature.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
pub struct Tolerance {
    /// Measurable feature.
    pub feature: Feature,

    /// Numerical limit of the feature.
    pub limit: Limit,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Extras::is_empty"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

/// Numerical limits of a particular feature.
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Limit {
    /// Tolerance zone.
    #[serde(rename_all = "camelCase")]
    Zone { max_deviation: f64 },

    /// `min-max`
    #[serde(rename_all = "camelCase")]
    Explicit { min: f64, max: f64 },

    /// `baseline ± max_deviation`
    #[serde(rename_all = "camelCase")]
    Symmetric { baseline: f64, max_deviation: f64 },

    /// `baseline +max / -min`
    #[serde(rename_all = "camelCase")]
    Bilateral { baseline: f64, min: f64, max: f64 },
}

impl Validate for Limit {}

/// Control sequence used to position callouts in a MBD frame.
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Control {
    /// Following callouts are positioned on the next line.
    NextLine,

    /// Groups following datum callouts into a compound datum.
    ///
    /// For example, 'A' followed by 'B' becomes "A-B" in the MBD frame.
    BeginCompoundDatum,

    /// Stops grouping datum callouts.
    EndCompoundDatum,
}

impl Validate for Control {}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CalloutType {
    /// Datum callout.
    Datum,

    /// Feature specification.
    Feature,

    /// Feature modifier.
    Modifier,

    /// Numerical limit of a particular feature.
    Limit,

    /// Callout control sequence.
    Control,
}

impl Validate for CalloutType {}

/// Specific callout in a MBD frame.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
pub struct Callout {
    /// Optional name for this frame.
    #[cfg(feature = "names")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Specifies the callout type.
    #[serde(rename = "type")]
    pub type_: CalloutType,

    /// Datum callout.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datum: Option<Datum>,

    /// Feature specification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<Feature>,

    /// Feature modifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier: Option<Modifier>,

    /// Numerical limit of a particular feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,

    /// Callout control sequence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control: Option<Control>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Extras::is_empty"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

impl Callout {
    /// Datum callout.
    pub fn datum(c: char) -> Self {
        Self {
            name: None,
            type_: CalloutType::Datum,
            datum: Some(Datum {
                id: c.to_string(),
                extras: Default::default(),
            }),
            feature: None,
            modifier: None,
            limit: None,
            control: None,
            extras: Default::default(),
        }
    }

    /// Feature specification.
    pub fn feature(feature: Feature) -> Self {
        Self {
            name: None,
            type_: CalloutType::Feature,
            datum: None,
            feature: Some(feature),
            modifier: None,
            limit: None,
            control: None,
            extras: Default::default(),
        }
    }

    /// Feature modifier.
    pub fn modifier(modifier: Modifier) -> Self {
        Self {
            name: None,
            type_: CalloutType::Modifier,
            datum: None,
            feature: None,
            modifier: Some(modifier),
            limit: None,
            control: None,
            extras: Default::default(),
        }
    }

    /// Numerical limit of a particular feature.
    pub fn limit(limit: Limit) -> Self {
        Self {
            name: None,
            type_: CalloutType::Limit,
            datum: None,
            feature: None,
            modifier: None,
            limit: Some(limit),
            control: None,
            extras: Default::default(),
        }
    }

    /// Callout control sequence.
    pub fn control(control: Control) -> Self {
        Self {
            name: None,
            type_: CalloutType::Control,
            datum: None,
            feature: None,
            modifier: None,
            limit: None,
            control: Some(control),
            extras: Default::default(),
        }
    }
}

/// Block of text.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
pub struct TextBlock {
    /// The text content.
    pub content: String,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Extras::is_empty"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut overlay = Overlay {
            name: None,
            frames: Vec::new(),
            text_blocks: Vec::new(),
            extras: Default::default(),
            targets: Vec::new(),
        };

        {
            let mut callouts = Vec::new();
            callouts.push(Callout::feature(Feature::Position));
            callouts.push(Callout::modifier(Modifier::Diameter));
            callouts.push(Callout::limit(Limit::Zone {
                max_deviation: 0.04,
            }));
            callouts.push(Callout::datum('C'));
            callouts.push(Callout::datum('A'));
            callouts.push(Callout::datum('B'));
            overlay.frames.push(Frame::new(callouts));
        }

        let json = serde_json::to_string_pretty(&overlay).unwrap();
        println!("{json}");
    }

    #[test]
    fn example2() {
        let mut overlay = Overlay {
            name: None,
            frames: Vec::new(),
            text_blocks: Vec::new(),
            extras: Default::default(),
            targets: Vec::new(),
        };

        {
            let mut callouts = Vec::new();
            callouts.push(Callout::limit(Limit::Zone {
                max_deviation: 0.05,
            }));
            callouts.push(Callout::datum('D'));
            callouts.push(Callout::datum('B'));
            callouts.push(Callout::datum('C'));
            callouts.push(Callout::control(Control::NextLine));
            callouts.push(Callout::limit(Limit::Zone {
                max_deviation: 0.01,
            }));
            callouts.push(Callout::datum('D'));

            let mut frame = Frame::new(callouts);
            frame.group = Some(Feature::SurfaceProfile);
            overlay.frames.push(frame);
        }

        let json = serde_json::to_string_pretty(&overlay).unwrap();
        println!("{json}");
    }
}
