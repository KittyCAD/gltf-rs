use crate::Document;

use json::extensions::kittycad_model_based_definition::{self as mbd, Measurement};

#[doc(inline)]
pub use mbd::{Characteristic, Limit, Modifier};

/// Overlay definition.
#[derive(Clone, Debug)]
pub struct Overlay<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a mbd::Overlay,
}

impl<'a> Overlay<'a> {
    /// Constructs a `Overlay`.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a mbd::Overlay) -> Self {
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

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Returns an `Iterator` visits the overlay's MBD callout frames.
    pub fn frames(&self) -> impl ExactSizeIterator<Item = Frame<'a>> {
        self.json
            .frames
            .iter()
            .map(|json| Frame::new(self.document, json))
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

/// MBD callout frame definition.
#[derive(Clone, Debug)]
pub struct Frame<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a mbd::Frame,
}

impl<'a> Frame<'a> {
    /// Constructs a `Frame`.
    pub(crate) fn new(document: &'a Document, json: &'a mbd::Frame) -> Self {
        Self { document, json }
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Returns the grouping characteristic.
    pub fn group(&self) -> Option<Characteristic> {
        self.json.group
    }

    /// Returns an `Iterator` visits the frame's tolerances.
    pub fn tolerances(&self) -> impl ExactSizeIterator<Item = Tolerance<'a>> {
        self.json
            .tolerances
            .iter()
            .map(|json| Tolerance::new(self.document, json))
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

/// MBD tolerance specifier.
#[derive(Clone, Debug)]
pub struct Tolerance<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a mbd::Tolerance,
}

impl<'a> Tolerance<'a> {
    /// Constructs a `Tolerance`.
    pub(crate) fn new(document: &'a Document, json: &'a mbd::Tolerance) -> Self {
        Self { document, json }
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Returns the tolerance measurement modifier where applicable.
    pub fn measurement(&self) -> Option<Measurement> {
        self.json.measurement
    }

    /// Returns the geometric characteristic the tolerance applies to, where applicable.
    ///
    /// This will be `None` when the tolerance belongs to a group.
    pub fn characteristic(&self) -> Option<Characteristic> {
        self.json.characteristic
    }

    /// Returns the tolerance limit value.
    pub fn limit(&self) -> Limit {
        self.json.limit
    }

    /// Returns the tolerance modifiers.
    pub fn modifiers(&self) -> &[Modifier] {
        &self.json.modifiers
    }

    /// Returns an `Iterator` that visits the referenced datum features.
    pub fn tolerances(&self) -> impl ExactSizeIterator<Item = Feature<'a>> {
        self.json
            .features
            .iter()
            .map(|json| Feature::new(self.document, json))
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

/// MBD datum feature.
#[derive(Clone, Debug)]
pub struct Feature<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a mbd::datum::Feature,
}

impl<'a> Feature<'a> {
    /// Constructs a `Feature`.
    pub(crate) fn new(document: &'a Document, json: &'a mbd::datum::Feature) -> Self {
        Self { document, json }
    }

    /// Returns an `Iterator` that visits the datum feature references.
    pub fn references(&self) -> impl ExactSizeIterator<Item = Reference<'a>> {
        self.json
            .references
            .iter()
            .map(|json| Reference::new(self.document, json))
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

/// MBD datum feature reference.
#[derive(Clone, Debug)]
pub struct Reference<'a> {
    /// The parent `Document` struct.
    #[allow(unused)]
    pub(crate) document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a mbd::datum::Reference,
}

impl<'a> Reference<'a> {
    /// Constructs a `Feature`.
    pub(crate) fn new(document: &'a Document, json: &'a mbd::datum::Reference) -> Self {
        Self { document, json }
    }

    /// Returns the ID of the datum feature reference.
    pub fn id(&self) -> &str {
        &self.json.id
    }

    /// Returns the modifiers applied to the datum feature reference.
    pub fn modifiers(&self) -> &[Modifier] {
        &self.json.modifiers
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

/// MBD datum callout.
#[derive(Clone, Debug)]
pub struct Datum<'a> {
    /// The parent `Document` struct.
    #[allow(unused)]
    pub(crate) document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a mbd::Datum,
}

impl<'a> Datum<'a> {
    /// Constructs a `Datum`.
    pub(crate) fn new(document: &'a Document, json: &'a mbd::Datum) -> Self {
        Self { document, json }
    }

    /// Returns the ID of the datum.
    pub fn id(&self) -> &'a str {
        &self.json.id
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}
