use std::fmt::{self, Write};

use crate::fields::Fields;
use crate::formatter::Formatter;
use crate::docs::Docs;
use crate::r#type::Type;

/// Defines an enum variant.
#[derive(Debug, Clone)]
pub struct Variant {
    name: String,
    fields: Fields,

    /// Variant documentation
    docs: Option<Docs>,
}

impl Variant {
    /// Return a new enum variant with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fields: Fields::Empty,
            docs: None,
        }
    }

    /// Add a named field to the variant.
    pub fn named<T>(&mut self, name: &str, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.fields.named(name, ty);
        self
    }

    /// Add a tuple field to the variant.
    pub fn tuple(&mut self, ty: &str) -> &mut Self {
        self.fields.tuple(ty);
        self
    }

    /// Set the variant documentation.
    pub fn doc(&mut self, docs: &str) -> &mut Self {
        self.docs = Some(Docs::new(docs));
        self
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.docs {
            docs.fmt(fmt)?;
        }
        write!(fmt, "{}", self.name)?;
        self.fields.fmt(fmt)?;
        writeln!(fmt, ",")?;

        Ok(())
    }
}
