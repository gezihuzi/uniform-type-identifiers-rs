pub mod system_defined;

use std::{fmt::Display, path::Path, vec};

use serde::{Deserialize, Serialize};
use system_defined::{
    OTHER_FILENAME_EXTENSION_MAP, OTHER_MIME_MAP, OTHER_TYPES_MAP, SYSTEM_FILENAME_EXTENSION_MAP,
    SYSTEM_MIME_MAP, SYSTEM_TYPES_MAP,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct UTType<'a> {
    pub identifier: &'a str,
    pub conforms_to: &'a str,
    pub tags: &'a str,
    pub description: &'a str,
}

impl Display for UTType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier)
    }
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
}

pub struct MIMETypeAndExtension<'a> {
    pub mime_type: &'a str,
    pub extensions: &'a str,
}

impl UTType<'_> {
    pub fn from_identifier(value: &str) -> Option<Self> {
        let item = SYSTEM_TYPES_MAP.get(value);
        let item = match item {
            Some(item) => item,
            None => OTHER_TYPES_MAP.get(value)?,
        };
        Some(*item)
    }

    pub fn from_mime_type(value: &str) -> Option<Self> {
        let item = SYSTEM_MIME_MAP.get(value);
        let item = match item {
            Some(item) => item,
            None => OTHER_MIME_MAP.get(value)?,
        };
        Self::from_identifier(item)
    }

    pub fn from_filename_extension(value: &str) -> Option<Self> {
        let item = SYSTEM_FILENAME_EXTENSION_MAP.get(value);
        let item = match item {
            Some(item) => item,
            None => OTHER_FILENAME_EXTENSION_MAP.get(value)?,
        };
        Self::from_identifier(item)
    }

    pub fn from_path(path: &Path) -> Option<UTType> {
        if path.is_absolute() {
            if path.is_file() {
                let extension = path.extension()?.to_string_lossy().to_lowercase();
                UTType::from_filename_extension(&extension)
            } else if path.is_dir() {
                Some(system_defined::PUBLIC_DIRECTORY)
            } else if path.is_symlink() {
                Some(system_defined::PUBLIC_SYMLINK)
            } else {
                None
            }
        } else {
            None
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
        let key = self.identifier;
        SYSTEM_TYPES_MAP.contains_key(key)
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
        super_types(*self)
    }

    /// Returns a Boolean value that indicates whether a type conforms to the type.
    /// true if the type directly or indirectly conforms to type, or if it’s equal to type.
    pub fn is_conforms(&self, x: &Self) -> bool {
        self == x || self.super_types().contains(x)
    }

    /// Returns a Boolean value that indicates whether a type is higher in a hierarchy than the type.
    pub fn is_subtype(&self, x: &Self) -> bool {
        self == x || self.super_types().iter().any(|t| t.is_subtype(x))
    }

    /// Returns a Boolean value that indicates whether a type is lower in a hierarchy than the type.
    pub fn is_supertype(&self, x: &Self) -> bool {
        x.is_subtype(self)
    }
}

fn super_types(x: UTType<'_>) -> Vec<UTType<'_>> {
    let types = serde_json::from_str::<Vec<&str>>(x.conforms_to)
        .unwrap_or_default()
        .iter()
        .filter(|it| !it.is_empty())
        .filter_map(|it| UTType::from_identifier(it))
        .collect::<Vec<_>>();
    let mut values: Vec<UTType> = types.clone();
    let mut i = 0;
    while i < types.len() {
        let item = types[i];
        let super_types = super_types(item);
        values.extend(super_types);
        i += 1;
    }
    values
}

#[cfg(test)]
mod tests {
    use crate::UTType;

    #[test]
    fn test_plain_text() {
        let text = UTType::from_identifier("public.text").unwrap();
        let plain_text = UTType::from_identifier("public.plain-text").unwrap();
        let utf8_text = UTType::from_identifier("public.utf8-plain-text").unwrap();
        let utf16_text = UTType::from_identifier("public.utf16-plain-text").unwrap();
        let html_text = UTType::from_identifier("public.html").unwrap();
        let rtf_text = UTType::from_identifier("public.rtf").unwrap();
        assert!(plain_text.is_conforms(&text));
        assert!(utf8_text.is_conforms(&plain_text));
        assert!(utf16_text.is_conforms(&plain_text));
        assert!(html_text.is_conforms(&text));
        assert!(rtf_text.is_conforms(&text));
    }
}
