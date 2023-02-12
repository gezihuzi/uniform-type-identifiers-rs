pub mod system_defined;

use system_defined::ALL_TYPES;

#[derive(Debug, Clone, Copy)]
pub struct UTInternalType<'a> {
    pub identifier: &'a str,
    pub conforms_to: &'a str,
    pub tags: &'a str,
    pub filename_extension: &'a str,
    pub mime_type: &'a str,
    pub description: &'a str,
}

impl<'a> UTInternalType<'a> {
    pub fn new(
        identifier: &'a str,
        conforms_to: &'a str,
        tags: &'a str,
        filename_extension: &'a str,
        mime_type: &'a str,
        comments: &'a str,
    ) -> UTInternalType<'a> {
        UTInternalType {
            identifier,
            conforms_to,
            tags,
            filename_extension,
            mime_type,
            description: comments,
        }
    }
}

#[derive(Debug)]
pub struct Tag();

#[derive(Debug)]
pub struct UTType {
    pub identifier: String,
    pub conforms_to: Vec<String>,
    pub tags: Tag,
    pub filename_extension: Vec<String>,
    pub mime_type: Vec<String>,
    pub description: Option<String>,
}

impl<'a> From<UTInternalType<'a>> for UTType {
    fn from(internal_type: UTInternalType) -> Self {
        UTType {
            identifier: internal_type.identifier.to_owned(),
            conforms_to: internal_type
                .conforms_to
                .split("|")
                .filter(|s| !s.is_empty())
                .map(|s| s.to_owned())
                .collect(),
            tags: Tag {},
            filename_extension: internal_type
                .filename_extension
                .split("|")
                .filter(|s| !s.is_empty())
                .map(|s| s.to_owned())
                .collect(),
            mime_type: internal_type
                .mime_type
                .split("|")
                .filter(|s| !s.is_empty())
                .map(|s| s.to_owned())
                .collect(),
            description: Some(internal_type.description.to_owned()),
        }
    }
}

impl From<&str> for UTType {
    fn from(value: &str) -> Self {
        let item = ALL_TYPES.iter().find(|item| item.identifier == value);
        match item {
            Some(it) => UTType::from(it.clone()),
            None => UTType {
                identifier: value.into(),
                conforms_to: vec![],
                tags: Tag(),
                filename_extension: vec![],
                mime_type: vec![],
                description: Option::None,
            },
        }
    }
}
