use uniform_type_identifiers::{system_defined::{self, PUBLIC_CALENDAR_EVENT}, UTType};

fn main() {
    let raw = system_defined::COM_APPLE_DESKTOP_FOLDER;
    let folder = UTType::from(raw);
    let data: UTType = "public.data".into();
    let json = PUBLIC_CALENDAR_EVENT;
    let mime_type = json.preferred_mime_type();
    let extension = json.preferred_filename_extension();
    println!("{:?}, {:?}", folder, data);
    println!("{:?}, {:?}, {:?}", json, mime_type, extension);
}
