//! Part definitions.

use crate::{Extras, Index};
use gltf_derive::Validate;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};

use crate::extensions::kittycad_boundary_representation::Solid;

/// Solid boundary representation structure.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "part")]
pub struct Part {
    /// Solid bodies that form the part shape.
    pub solids: Vec<Index<Solid>>,

    /// Part identifier.
    pub id: String,

    /// Optional name for this part.
    #[cfg(feature = "names")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional version for this part.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Extras::is_empty"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}
