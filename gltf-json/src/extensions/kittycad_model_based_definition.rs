//! Model-based definition and product manufacturing information.

use crate::validation::Validate;
use crate::{Extras, Mesh, Index};
use gltf_derive::Validate;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::extensions::kittycad_boundary_representation as brep;
use crate::extensions::kittycad_part as part;

#[doc(inline)]
pub use datum::Datum;

/// Datum specific data structures.
pub mod datum {
    use super::*;

    /// Datum definition.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
    pub struct Datum {
        /// Placement of the datum box in the plane.
        pub placement: Placement,

        /// Identification of the feature the datum is associated with.
        pub pointer: Pointer,

        /// Visual representation of the item.
        pub mesh: Index<Mesh>,

        /// Optional application specific data.
        #[serde(default)]
        #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Extras::is_empty"))]
        #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
        pub extras: Extras,
    }

    /// A datum feature.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
    pub struct Feature {
        /// Datum feature references.
        pub references: Vec<Reference>,

        /// Optional application specific data.
        #[serde(default)]
        #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Extras::is_empty"))]
        #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
        pub extras: Extras,
    }

    /// A datum feature reference.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
    pub struct Reference {
        /// The ID of the referenced datum.
        pub id: String,

        /// Datum modifiers.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub modifiers: Vec<Modifier>,

        /// Optional application specific data.
        #[serde(default)]
        #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Extras::is_empty"))]
        #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
        pub extras: Extras,
    }

    /// A datum target point.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
    pub struct Target {
        /// Target area size.
        pub size: String,

        /// The ID of the datum target.
        pub id: String,

        /// Specific target number.
        pub number: u32,
    }
}

/// The drawing area for MBD items.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Overlay {
    /// Optional name for this overlay.
    #[cfg(feature = "names")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The part this overlay is annotating.
    pub part: Index<part::Part>,

    /// The solid this overlay is annotating.
    pub solid: Index<brep::Solid>,

    /// Datum definitions.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub datums: BTreeMap<String, Datum>,

    /// Feature control frames.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frames: Vec<Frame>,

    /// Datum target points.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<datum::Target>,

    /// Human-readable textual information.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_blocks: Vec<TextBlock>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Extras::is_empty"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

/// Identified feature of a part.
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Aspect {
    ///  Identifies a particular edge.
    Edge,

    /// Identifies a particular face.
    Face,

    /// Identifies a particular feature control frame.
    Frame,
}

impl Validate for Aspect {}

/// Symbol part of a leader line.
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Symbol {
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

    /// ➢
    ///
    /// Hollow arrow head.
    HollowArrow,
}

impl Validate for Symbol {}

/// Association between an annotation and a part feature.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Pointer {
    /// Feature aspect of the pointee, i.e., the index type.
    pub aspect: Aspect,

    /// The index of the identified feature.
    pub index: u32,

    /// The endpoint of the leader line on the annotation plane.
    ///
    /// When `None`, no leader line is drawn.
    pub endpoint: Option<[f64; 2]>,

    /// Extension line.
    pub extension_line: Option<Vec<[f64; 2]>>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Extras::is_empty"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

/// Describes the relationship between an anchor point and its owner.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub enum Attachment {
    /// Anchor is on the left side of the item.
    ///
    ///   ┌───┐
    /// ──W   |
    ///   └───┘
    ///
    /// Applicable to any item.
    #[serde(rename = "W")]
    West,

    /// Anchor is on the right side of the item.
    ///
    /// ┌───┐
    /// |   E──
    /// └───┘
    ///
    /// Applicable to any item.
    #[serde(rename = "E")]
    East,

    /// Anchor is on the top side of the item.
    ///
    ///   |
    /// ┌─N─┐
    /// │   |
    /// └───┘
    ///
    /// Applicable only to text and datum boxes.
    #[serde(rename = "N")]
    North,

    /// Anchor is on the bottom side of the item.
    ///
    /// ┌───┐
    /// │   |
    /// └─S─┘
    ///   |
    ///
    /// Applicable only to text and datum boxes.
    #[serde(rename = "S")]
    South,

    /// Anchor is in the center of the item.
    ///
    /// ┌───┐
    /// │ C |
    /// └───┘
    ///
    /// Applicable only to text.
    #[serde(rename = "C")]
    Center,
}

impl Validate for Attachment {}

/// Describes the placement of a boxed item on an annotation plane.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Placement {
    /// Coordinate of the anchor point in the plane.
    pub anchor: [f64; 2],

    /// Attachment point of the anchor.
    pub attachment: Attachment,
}

/// A feature control frame.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
pub struct Frame {
    /// Optional name for this feature control frame.
    #[cfg(feature = "names")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

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
    pub group: Option<Characteristic>,

    /// Required feature tolerances.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerances: Vec<Tolerance>,

    /// Placement of the frame in the plane.
    pub placement: Placement,

    /// Identification of the feature the tolerances apply to.
    pub pointer: Pointer,

    /// Visual representation of the overlay contents.
    pub mesh: Index<Mesh>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Extras::is_empty"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Serialize)]
pub enum Modifier {
    /// Only gravity applies to the part.
    ///
    /// Represented by the Ⓕ symbol.
    #[serde(rename = "F")]
    FreeState,

    /// Perfect form not required at MMC nor LMC.
    ///
    /// Represented by the Ⓘ symbol.
    #[serde(rename = "I")]
    Independency,

    /// Least material condition.
    ///
    /// Represented by the Ⓛ symbol.
    #[serde(rename = "L")]
    LeastMaterialCondition,

    /// Maximum material condition.
    ///
    /// Represented by the Ⓜ symbol.
    #[serde(rename = "M")]
    MaximumMaterialCondition,

    /// Projected tolerance zone.
    ///
    /// Represented by the Ⓟ symbol.
    #[serde(rename = "P")]
    ProjectedToleranceZone,

    /// Unequally displaced profile tolerance zone.
    ///
    /// Represented by the Ⓤ symbol.
    #[serde(rename = "U")]
    UnequallyDisplacedProfile,

    /// Tangent plane of a surface.
    ///
    /// Represented by the Ⓣ symbol.
    #[serde(rename = "T")]
    TangentPlane,
}

impl Validate for Modifier {}

/// Geometric characteristic.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Characteristic {
    /// Permitted projected deviation from a circle.
    Circularity,

    /// Permitted projected deviation from a cylinder.
    Cylindricity,

    /// Permitted projected deviation from a plane.
    Flatness,

    /// Permitted projected deviation along a curve or cross-section.
    LineProfile,

    /// Permitted deviation of a particular point from a specified datum.
    Position,

    /// Permitted projected deviation from a line.
    Straightness,

    /// Permitted projected deviation across a surface.
    SurfaceProfile,

    // --- All geometric characteristics hereon require at least one datum ---

    /// Permitted angular deviation from a referenced datum.
    Angularity,

    /// Permitted circular deviation of a surface as it is rotated about a referenced datum axis.
    Runout,

    /// Permitted deviation of median points of circular features relative to a referenced datum axis.
    Concentricity,

    /// Permitted projected deviation from a parallel referenced datum.
    Parallelism,
 
    /// Permitted perpendicular deviation from a referenced datum.
    Perpendicularity,

    /// Permitted deviation of median points relative to a referenced datum center plane.
    Symmetry,

    /// Permitted cumulative circular deviation of a surface as it is rotated about a referenced datum axis.
    TotalRunout,
}

impl Validate for Characteristic {}

impl Characteristic {
    /// Returns `true` if the geometric characteristic requires a datum.
    pub fn requires_datum(self) -> bool {
        self as isize >= Self::Angularity as isize
    }
}

/// Tolerance measurement modifier.
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Measurement {
    /// Prefixes the tolerance value with ⌀.
    Diameter,

    /// Prefixes the tolerance value with R.
    Radius,

    /// Prefixes the tolerance value with S⌀.
    SphericalDiameter,

    /// Prefixes the tolerance value with SR.
    SphericalRadius,
}

impl Validate for Measurement {}

/// A tolerance specification for a particular feature.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Tolerance {
    /// Optional name for this overlay.
    #[cfg(feature = "names")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Tolerance measurement modifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurement: Option<Measurement>,

    /// Measurable geometric characteristic.
    ///
    /// This must be `None` when the tolerance belongs to group
    /// and must have a value otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub characteristic: Option<Characteristic>,

    /// Numerical limit of the feature.
    pub limit: Limit,

    /// Tolerance modifiers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifiers: Vec<Modifier>,

    /// Datum features.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<datum::Feature>,

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

    /// Straightness tolerance example.
    ///
    /// ```
    /// ┌───────┬────────┬───┐
    /// │  ───  │ ⌀.04Ⓜ │ A │
    /// └───────┴────────┴───┘
    /// ```
    #[test]
    fn straightness_with_mmc_and_datum_reference() {
        let stub_part_index = Index::new(12);
        let stub_solid_index = Index::new(34);
        let stub_mesh_index = Index::new(56);
        let stub_face_index = 78;
        let overlay = Overlay {
            name: None,
            extras: Default::default(),
            part: stub_part_index,
            solid: stub_solid_index,
            mesh: stub_mesh_index,
            text_blocks: Vec::new(),
            targets: Vec::new(),
            datums: BTreeMap::from([
                (
                    "A".to_owned(),
                    Datum {
                        placement: Placement {
                            anchor: [0.0, 1.0],
                            attachment: Attachment::North,
                        },
                        pointer: Pointer {
                            aspect: Aspect::Face,
                            index: stub_face_index,
                            endpoint: Some([1.0, 1.0]),
                            extras: Default::default(),
                        },
                        primitive: 0,
                        extras: Default::default(),
                    },
                ),
            ]),
            frames: vec![Frame {
                name: None,
                group: None,
                placement: Placement {
                    anchor: [0.0, 1.0],
                    attachment: Attachment::East,
                },
                pointer: Pointer {
                    aspect: Aspect::Face,
                    index: stub_face_index,
                    endpoint: Some([2.0, 2.0]),
                    extras: Default::default(),
                },
                primitive: 0,
                extras: Default::default(),
                tolerances: vec![Tolerance {
                    name: None,
                    extras: Default::default(),
                    measurement: Some(Measurement::Diameter),
                    characteristic: Some(Characteristic::Straightness),
                    limit: Limit::Zone {
                        max_deviation: 0.04,
                    },
                    modifiers: vec![Modifier::MaximumMaterialCondition],
                    features: vec![datum::Feature {
                        extras: Default::default(),
                        references: vec![datum::Reference {
                            id: "A".to_string(),
                            modifiers: Vec::new(),
                            extras: Default::default(),
                        }],
                    }],
                }],
            }],
        };
        let json = serde_json::to_string_pretty(&overlay).unwrap();
        println!("{json}");
    }

    /// Grouped tolerances example.
    ///
    /// ```
    /// ┌───────┬───────┬───┐
    /// │       │ ⌀ .04 │ A │
    /// │   ⌖   ├───────┼───┤
    /// │       │ ⌀ .02 │ B │
    /// └───────┴───────┴───┘
    /// ```
    #[test]
    fn grouped_position_tolerances() {
        let stub_part_index = Index::new(12);
        let stub_solid_index = Index::new(34);
        let stub_mesh_index = Index::new(56);
        let stub_face_index = 78;
        let overlay = Overlay {
            name: None,
            extras: Default::default(),
            part: stub_part_index,
            solid: stub_solid_index,
            text_blocks: Vec::new(),
            targets: Vec::new(),
            datums: BTreeMap::from([
                (
                    "A".to_owned(),
                    Datum {
                        placement: Placement {
                            anchor: [0.0, 1.0],
                            attachment: Attachment::North,
                        },
                        pointer: Pointer {
                            aspect: Aspect::Face,
                            index: stub_face_index,
                            endpoint: Some([1.0, 1.0]),
                            extras: Default::default(),
                        },
                        mesh: stub_mesh_index,
                        extras: Default::default(),
                    },
                ),
                (
                    "B".to_owned(),
                    Datum {
                        placement: Placement {
                            anchor: [0.0, 1.0],
                            attachment: Attachment::North,
                        },
                        pointer: Pointer {
                            aspect: Aspect::Face,
                            index: stub_face_index,
                            endpoint: Some([1.0, 1.0]),
                            extras: Default::default(),
                        },
                        mesh: stub_mesh_index,
                        extras: Default::default(),
                    },
                ),
            ]),
            frames: vec![Frame {
                name: None,
                extras: Default::default(),
                group: Some(Characteristic::Position),
                placement: Placement {
                    anchor: [0.0, 1.0],
                    attachment: Attachment::West,
                },
                pointer: Pointer {
                    aspect: Aspect::Face,
                    index: stub_face_index,
                    endpoint: Some([2.0, 2.0]),
                    extras: Default::default(),
                },
                mesh: stub_mesh_index,
                tolerances: vec![
                    Tolerance {
                        name: None,
                        extras: Default::default(),
                        measurement: Some(Measurement::Diameter),
                        characteristic: None,
                        limit: Limit::Zone {
                            max_deviation: 0.04,
                        },
                        modifiers: Vec::new(),
                        features: vec![datum::Feature {
                            extras: Default::default(),
                            references: vec![datum::Reference {
                                id: "A".to_string(),
                                modifiers: Vec::new(),
                                extras: Default::default(),
                            }],
                        }],
                    },
                    Tolerance {
                        name: None,
                        extras: Default::default(),
                        measurement: Some(Measurement::Diameter),
                        characteristic: None,
                        limit: Limit::Zone {
                            max_deviation: 0.02,
                        },
                        modifiers: Vec::new(),
                        features: vec![datum::Feature {
                            extras: Default::default(),
                            references: vec![datum::Reference {
                                id: "B".to_string(),
                                modifiers: Vec::new(),
                                extras: Default::default(),
                            }],
                        }],
                    },
                ],
            }],
        };
        let json = serde_json::to_string_pretty(&overlay).unwrap();
        println!("{json}");
    }
}
