pub mod system_defined;

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
    //     pub filename_extensions: Vec<&'a str>,
    //     pub mime_types: Vec<&'a str>,
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
    pub fn preferred_filename_extension(&self) -> Option<&str> {
        // let tags = self.tags();
        // let items = tags?.filename_extensions;
        let first = "";
        Some(first)
    }

    /// The preferred MIME type for the type.
    pub fn preferred_mime_type(&self) -> Option<&str> {
        // let tags = self.tags();
        // let items = tags?.mime_types;
        let first = "";
        Some(first)
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

    /// A Boolean value that indicates whether the system declares the type.
    pub fn is_declared() -> bool {
        false
    }

    /// A Boolean value that indicates whether the system generates the type.
    pub fn is_dynamic() -> bool {
        false
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
    pub fn conforms(&self, x: UTType) -> bool {
        let items: Vec<&str> = x.conforms_to.split("|").filter(|s| !s.is_empty()).collect();
        items.contains(&self.identifier)
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
