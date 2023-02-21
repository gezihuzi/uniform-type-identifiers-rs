pub mod system_defined;

use std::vec;

use serde::{Deserialize, Serialize};
use system_defined::{OTHER_TYPES, SYSTEM_TYPES};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct UTType<'a> {
    pub identifier: &'a str,
    pub conforms_to: &'a str,
    pub tags: &'a str,
    pub description: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tags {
    #[serde(default, alias = "public.filename-extension")]
    pub filename_extensions: Vec<String>,
    #[serde(default, alias = "public.mime-type")]
    pub mime_types: Vec<String>,
}

impl<'a> UTType<'a> {
    pub fn new(
        identifier: &'a str,
        conforms_to: &'a str,
        tags: &'a str,
        comments: &'a str,
    ) -> UTType<'a> {
        UTType {
            identifier,
            conforms_to,
            tags,
            description: comments,
        }
    }

    /// The string that represents the type.
    pub fn identifier(&self) -> &str {
        self.identifier
    }

    /// The preferred filename extension for the type.
    pub fn preferred_filename_extension(&self) -> Option<String> {
        let tags = self.tags()?;
        let first = tags.filename_extensions.first()?;
        Some(first.clone())
    }

    /// The preferred MIME type for the type.
    pub fn preferred_mime_type(&self) -> Option<String> {
        let tags = self.tags()?;
        let first = tags.mime_types.first()?;
        Some(first.clone())
    }

    /// The tag specification dictionary of the type.
    pub fn tags(&self) -> Option<Tags> {
        let result: Result<Option<Tags>, serde_json::Error> = serde_json::from_str(self.tags);
        if let Ok(Some(it)) = result {
            Some(it)
        } else {
            None
        }
    }

    pub fn conforms_to(&self) -> Vec<String> {
        let result: Result<Option<Vec<String>>, serde_json::Error> =
            serde_json::from_str(self.conforms_to);
        if let Ok(Some(it)) = result {
            it
        } else {
            vec![]
        }
    }

    /// A Boolean value that indicates whether the system declares the type.
    pub fn is_declared(&self) -> bool {
        let items: Vec<&str> = SYSTEM_TYPES.iter().map(|f| f.identifier).collect();
        items.contains(&self.identifier)
    }

    /// A Boolean value that indicates whether the system generates the type.
    pub fn is_dynamic(&self) -> bool {
        !self.is_declared()
    }

    /// A Boolean value that indicates whether the type is in the public domain.
    pub fn is_public(&self) -> bool {
        self.identifier.starts_with("public.")
    }

    /// The set of types the type directly or indirectly conforms to.
    pub fn super_types(&self) -> Vec<UTType> {
        self.conforms_to
            .split("|")
            .filter(|s| !s.is_empty())
            .map(|f| UTType::from(f.clone()))
            .collect()
    }

    /// Returns a Boolean value that indicates whether a type conforms to the type.
    /// true if the type directly or indirectly conforms to type, or if it’s equal to type.
    pub fn conforms(&self, x: Self) -> bool {
        let items: Vec<&str> = x.conforms_to.split("|").filter(|s| !s.is_empty()).collect();
        items.contains(&self.identifier)
    }

    /// Returns a Boolean value that indicates whether a type is higher in a hierarchy than the type.
    pub fn is_subtype(&self) -> bool {
        todo!()
    }

    /// Returns a Boolean value that indicates whether a type is lower in a hierarchy than the type.
    pub fn is_supertype(&self) -> bool {
        todo!()
    }
}

pub struct MIMETypeAndExtension<'a> {
    pub mime_type: &'a str,
    pub extensions: &'a str,
}

impl<'a> From<&'a str> for UTType<'a> {
    fn from(value: &'a str) -> Self {
        if let Some(it) = SYSTEM_TYPES.iter().find(|it| it.identifier == value) {
            return it.clone();
        } else {
            if let Some(it) = OTHER_TYPES.iter().find(|it| it.identifier == value) {
                return it.clone();
            } else {
                return UTType {
                    identifier: value,
                    conforms_to: "",
                    tags: "",
                    description: "",
                };
            }
        }
    }
}
