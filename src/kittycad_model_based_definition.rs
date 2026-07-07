use crate::Document;

use json::extensions::kittycad_model_based_definition as mbd;

pub use mbd::{Control, Feature, Limit, Modifier};

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
        Self {
            document,
            json,
        }
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Returns an `Iterator` visits the frame's callouts.
    pub fn frames(&self) -> impl ExactSizeIterator<Item = Callout<'a>> {
        self.json
            .callouts
            .iter()
            .map(|json| Callout::new(self.document, json))
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

/// MBD callout.
#[derive(Clone, Debug)]
pub struct Callout<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a mbd::Callout,
}

impl<'a> Callout<'a> {
    /// Constructs a `Callout`.
    pub(crate) fn new(document: &'a Document, json: &'a mbd::Callout) -> Self {
        Self {
            document,
            json,
        }
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Returns the callout element.
    pub fn element(&self) -> Element<'a> {
        match self.json.type_ {
            mbd::CalloutType::Datum => {
                let json = self.json.datum.as_ref().unwrap()
                Element::Datum(Datum::new(self.document, json))
            }
            mbd::CalloutType::Feature => {
                Element::Feature(self.json.feature.unwrap())
            }
            mbd::CalloutType::Modifier => {
                Element::Modifier(self.json.modifier.unwrap())
            }
            mbd::CalloutType::Limit => {
                Element::Limit(self.json.limit.unwrap())
            }
            mbd::CalloutType::Control => {
                Element::Control(self.json.control.unwrap())
            }
        }
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

/// A particular element of an MBD callout frame.
#[derive(Clone, Debug)]
pub enum Element<'a> {
    /// Datum callout.
    Datum(Datum<'a>),

    /// Feature callout.
    Feature(Feature),

    /// Modifier callout.
    Modifier(Modifier),

    /// Limit callout.
    Limit(Limit),

    /// Control sequence.
    Control(Control),
}

/// MBD datum callout.
#[derive(Clone, Debug)]
pub struct Datum<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a mbd::Datum,
}

impl<'a> Datum<'a> {
    /// Constructs a `Datum`.
    pub(crate) fn new(document: &'a Document, json: &'a mbd::Datum) -> Self {
        Self {
            document,
            json,
        }
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
