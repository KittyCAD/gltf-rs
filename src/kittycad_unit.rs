use crate::Document;

use json::extensions::kittycad_unit as unit;

#[doc(inline)]
pub use unit::Measure;

/// Part definition.
#[derive(Clone, Debug)]
pub struct Unit<'a> {
    /// The parent `Document` struct.
    #[allow(unused)]
    pub(crate) document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a unit::Unit,
}

impl<'a> Unit<'a> {
    /// Constructs a `Unit`.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a unit::Unit) -> Self {
        Self {
            document,
            index,
            json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the unit name.
    pub fn name(&self) -> &'a str {
        &self.json.name
    }

    /// Returns the unit symbol, which may be blank.
    pub fn symbol(&self) -> &str {
        &self.json.symbol
    }

    /// Returns the specific kind of measurement.
    pub fn measure(&self) -> Measure {
        self.json.measure
    }

    /// Returns the multiplier in terms of the reference unit.
    pub fn multiplier(&self) -> f64 {
        self.json.multiplier
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}
