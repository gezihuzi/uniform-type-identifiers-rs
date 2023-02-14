pub mod system_defined;

use std::collections::HashSet;

use system_defined::SYSTEM_TYPES;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UTType<'a> {
    pub identifier: &'a str,
    pub conforms_to: &'a str,
    pub tags: &'a str,
    pub filename_extension: &'a str,
    pub mime_type: &'a str,
    pub description: &'a str,
}

impl<'a> UTType<'a> {
    pub fn new(
        identifier: &'a str,
        conforms_to: &'a str,
        tags: &'a str,
        filename_extension: &'a str,
        mime_type: &'a str,
        comments: &'a str,
    ) -> UTType<'a> {
        UTType {
            identifier,
            conforms_to,
            tags,
            filename_extension,
            mime_type,
            description: comments,
        }
    }

    pub fn preferred_filename_extension(&self) -> Option<&str> {
        let items: Vec<&str> = self
            .filename_extension
            .split("|")
            .filter(|s| !s.is_empty())
            .collect();
        let first = items.first()?;
        Some(*first)
    }

    pub fn preferred_mime_type(&self) -> Option<&str> {
        let items: Vec<&str> = self
            .mime_type
            .split("|")
            .filter(|s| !s.is_empty())
            .collect();
        let first = items.first()?;
        Some(*first)
    }

    pub fn is_dynamic() -> bool {
        false
    }

    pub fn is_declared() -> bool {
        false
    }

    pub fn is_public(&self) -> bool {
        self.identifier.starts_with("public.")
    }

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
        let option = SYSTEM_TYPES.iter().find(|it| it.identifier == value);
        match option {
            Some(it) => it.clone(),
            None => UTType {
                identifier: value,
                conforms_to: "",
                tags: "",
                filename_extension: "",
                mime_type: "",
                description: "",
            },
        }
    }
}
