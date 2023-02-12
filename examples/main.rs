use uniform_type_identifiers::{system_defined, UTType};

fn main() {
    let raw = system_defined::COM_APPLE_DESKTOP_FOLDER;
    let folder = UTType::from(raw);
    let data: UTType = "public.data".into();
    print!("{:?}, {:?}", folder, data);
}
