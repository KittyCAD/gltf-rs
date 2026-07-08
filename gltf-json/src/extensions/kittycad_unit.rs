use crate::{validation::Validate, Extras};
use gltf_derive::Validate;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};

/// Specific kind of measurement.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Measure {
    /// Reference unit: meter.
    Length,

    /// Reference unit: second.
    Time,

    /// Reference unit: radian.
    PlanarAngle,

    /// Reference unit: steradian.
    SolidAngle,

    /// Reference unit: gram.
    Mass,
}

impl Validate for Measure {}

/// The unit of measurement.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
pub struct Unit {
    /// Unit name.
    ///
    /// N.B., unlike other named objects, this is a required attribute.
    pub name: String,

    /// Unit symbol.
    ///
    /// N.B., the symbol may be blank.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub symbol: String,

    /// Specific kind of measurement.
    pub measure: Measure,

    /// Number of units required to represent the reference unit.
    ///
    /// For example, a 10 decimeters are required to represent the reference
    /// unit of 1 meter.
    pub multiplier: f64,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Extras::is_empty"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,
}

/// Meter length unit.
pub fn meter() -> Unit {
    Unit {
        name: "meter".to_owned(),
        symbol: "m".to_owned(),
        measure: Measure::Length,
        multiplier: 1.0,
        extras: Default::default(),
    }
}

/// Millimeter length unit.
pub fn millimeter() -> Unit {
    Unit {
        name: "millimeter".to_owned(),
        symbol: "mm".to_owned(),
        measure: Measure::Length,
        multiplier: 1000.0,
        extras: Default::default(),
    }
}

/// Inch length unit.
pub fn inch() -> Unit {
    Unit {
        name: "inch".to_owned(),
        symbol: "\"".to_owned(),
        measure: Measure::Length,
        multiplier: 0.0254,
        extras: Default::default(),
    }
}

/// Second time unit.
pub fn second() -> Unit {
    Unit {
        name: "second".to_owned(),
        symbol: "s".to_owned(),
        measure: Measure::Time,
        multiplier: 1.0,
        extras: Default::default(),
    }
}

/// Radian planar angle unit.
pub fn radian() -> Unit {
    Unit {
        name: "radian".to_owned(),
        symbol: "".to_owned(),
        measure: Measure::PlanarAngle,
        multiplier: 1.0,
        extras: Default::default(),
    }
}

/// Degree planar angle unit.
pub fn degree() -> Unit {
    Unit {
        name: "degree".to_owned(),
        symbol: "°".to_owned(),
        measure: Measure::PlanarAngle,
        multiplier: 180.0 * std::f64::consts::FRAC_1_PI,
        extras: Default::default(),
    }
}

/// Steradian solid angle unit.
pub fn steradian() -> Unit {
    Unit {
        name: "steradian".to_owned(),
        symbol: "Ω".to_owned(),
        measure: Measure::SolidAngle,
        multiplier: 1.0,
        extras: Default::default(),
    }
}

/// Gram mass unit.
pub fn gram() -> Unit {
    Unit {
        name: "gram".to_owned(),
        symbol: "g".to_owned(),
        measure: Measure::Mass,
        multiplier: 1.0,
        extras: Default::default(),
    }
}

/// Kilogram mass unit.
pub fn kilogram() -> Unit {
    Unit {
        name: "kilogram".to_owned(),
        symbol: "kg".to_owned(),
        measure: Measure::Mass,
        multiplier: 1000.0,
        extras: Default::default(),
    }
}
