
#[derive(Debug)]
pub struct UTType<'a> {
    pub identifier: &'a str,
    pub conforms_to: Vec<&'a str>,
    pub tags: Vec<&'a str>,
    pub comments: &'a str,
}

impl UTType<'_> {
    pub fn new<'a>(
        identifier: &'a str,
        conforms_to: Vec<&'a str>,
        tags: Vec<&'a str>,
        comments: &'a str,
    ) -> UTType<'a> {
        UTType {
            identifier,
            conforms_to,
            tags,
            comments,
        }
    }
}