use uniform_type_identifiers::{
    system_defined::{PUBLIC_DATA, PUBLIC_ITEM, PUBLIC_JSON, PUBLIC_TEXT, PUBLIC_XML, PUBLIC_MPEG_4, PUBLIC_MOVIE, PUBLIC_AUDIO},
    UTType,
};

fn main() {
    let mp4 = UTType::from_filename_extension("mp4");
    println!("MPEG-4: {:?}", mp4);
    let mp4_is_movie = PUBLIC_MPEG_4.is_conforms(&PUBLIC_MOVIE);
    let mp4_is_audio = PUBLIC_MPEG_4.is_conforms(&PUBLIC_AUDIO);
    println!("MPEG-4 is movie: {:?}", mp4_is_movie);
    println!("MPEG-4 is audio: {:?}", mp4_is_audio);

    let mime_type = PUBLIC_JSON.preferred_mime_type().unwrap_or_default();
    let extension = PUBLIC_JSON
        .preferred_filename_extension()
        .unwrap_or_default();
    let super_types = PUBLIC_JSON.super_types();
    println!("Json Identifier: {:?}", PUBLIC_JSON.identifier());
    println!("Json MIME type: {:?}", mime_type);
    println!("Json filename extension: {:?}", extension);
    println!(
        "Json super types: {:?}",
        super_types
            .iter()
            .map(|it| it.identifier())
            .collect::<Vec<_>>()
    );
    println!("-----");
    println!(
        "Json is conforms data: {:}",
        PUBLIC_JSON.is_conforms(&PUBLIC_DATA)
    );
    println!(
        "Json is conforms xml: {:}",
        PUBLIC_JSON.is_conforms(&PUBLIC_XML)
    );
    println!("-----");
    println!(
        "Json is text supertype: {:}",
        PUBLIC_JSON.is_supertype(&PUBLIC_TEXT)
    );
    println!(
        "Json is json supertype: {:}",
        PUBLIC_JSON.is_supertype(&PUBLIC_JSON)
    );
    println!(
        "Text is item supertype: {:}",
        PUBLIC_TEXT.is_supertype(&PUBLIC_ITEM)
    );
    println!(
        "Text is json supertype: {:}",
        PUBLIC_TEXT.is_supertype(&PUBLIC_JSON)
    );
    println!(
        "Item is text supertype: {:}",
        PUBLIC_ITEM.is_supertype(&PUBLIC_TEXT)
    );
    println!("-----");
    println!(
        "Json is text subtype: {:}",
        PUBLIC_JSON.is_subtype(&PUBLIC_TEXT)
    );
    println!(
        "Json is json subtype: {:}",
        PUBLIC_JSON.is_subtype(&PUBLIC_JSON)
    );
    println!(
        "Text is item subtype: {:}",
        PUBLIC_TEXT.is_subtype(&PUBLIC_ITEM)
    );
    println!(
        "Text is json subtype: {:}",
        PUBLIC_TEXT.is_subtype(&PUBLIC_JSON)
    );
    println!(
        "Item is text subtype: {:}",
        PUBLIC_ITEM.is_subtype(&PUBLIC_TEXT)
    );
    println!("-----");
    println!(
        "Get type from identifier: {:?}",
        UTType::from_identifier("public.json")
    );
    println!(
        "Get type from extension: {:?}",
        UTType::from_filename_extension("json")
    );
    println!(
        "Get type from mime type: {:?}",
        UTType::from_mime_type("application/json")
    );
}
