use crate::{Document, Mesh};

use json::extensions::kittycad_boundary_representation as brep;
use json::extensions::kittycad_part as ext;

/// Part definition.
#[derive(Clone, Debug)]
pub struct Part<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a ext::Part,
}

impl<'a> Part<'a> {
    /// Constructs a `Part`.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a ext::Part) -> Self {
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

    /// Returns the part identifier.
    pub fn id(&self) -> &str {
        &self.json.id
    }

    /// Returns the part version, where applicable.
    pub fn version(&self) -> Option<&str> {
        self.json.version.as_deref()
    }

    /// Returns an `Iterator` visits the part's solid bodies.
    pub fn solids(&self) -> impl ExactSizeIterator<Item = brep::Solid<'a>> {
        self.json
            .solids
            .iter()
            .map(|index| self.document.solids().unwrap().nth(index.value()).unwrap())
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}
