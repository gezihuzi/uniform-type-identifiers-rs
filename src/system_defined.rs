use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{MIMETypeAndExtension, UTType};

pub const PUBLIC_ITEM: UTType = UTType {
    identifier: "public.item",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"item"#,
};
pub const PUBLIC_DATA: UTType = UTType {
    identifier: "public.data",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{"public.mime-type": ["application/octet-stream"]}"#,
    description: r#"data"#,
};
pub const PUBLIC_DIRECTORY: UTType = UTType {
    identifier: "public.directory",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: r#"directory"#,
};
pub const PUBLIC_CONTENT: UTType = UTType {
    identifier: "public.content",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"content"#,
};
pub const PUBLIC_COMPOSITE_CONTENT: UTType = UTType {
    identifier: "public.composite-content",
    conforms_to: r#"["public.content"]"#,
    tags: r#"{}"#,
    description: r#"content"#,
};
pub const PUBLIC_NAMED_PIPE: UTType = UTType {
    identifier: "public.named-pipe",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const PUBLIC_CHARACTER_SPECIAL: UTType = UTType {
    identifier: "public.character-special",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const PUBLIC_BLOCK_SPECIAL: UTType = UTType {
    identifier: "public.block-special",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const PUBLIC_SOCKET: UTType = UTType {
    identifier: "public.socket",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const PUBLIC_EXECUTABLE: UTType = UTType {
    identifier: "public.executable",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"executable"#,
};
pub const PUBLIC_UNIX_EXECUTABLE: UTType = UTType {
    identifier: "public.unix-executable",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{}"#,
    description: r#"Unix executable"#,
};
pub const COM_APPLE_APPLICATION: UTType = UTType {
    identifier: "com.apple.application",
    conforms_to: r#"["public.executable"]"#,
    tags: r#"{}"#,
    description: r#"application"#,
};
pub const PUBLIC_ARCHIVE: UTType = UTType {
    identifier: "public.archive",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"file archive"#,
};
pub const PUBLIC_BOOKMARK: UTType = UTType {
    identifier: "public.bookmark",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Bookmark"#,
};
pub const PUBLIC_DATABASE: UTType = UTType {
    identifier: "public.database",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"database"#,
};
pub const COM_APPLE_CSSTORE: UTType = UTType {
    identifier: "com.apple.csstore",
    conforms_to: r#"["public.data", "public.database"]"#,
    tags: r#"{"public.filename-extension": ["csstore"]}"#,
    description: r#""#,
};
pub const PUBLIC_PRESENTATION: UTType = UTType {
    identifier: "public.presentation",
    conforms_to: r#"["public.composite-content"]"#,
    tags: r#"{}"#,
    description: r#"presentation"#,
};
pub const PUBLIC_SPREADSHEET: UTType = UTType {
    identifier: "public.spreadsheet",
    conforms_to: r#"["public.content"]"#,
    tags: r#"{}"#,
    description: r#"Spreadsheet"#,
};
pub const COM_APPLE_ICLOUD: UTType = UTType {
    identifier: "com.apple.iCloud",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: r#"iCloud"#,
};
pub const PUBLIC_3D_CONTENT: UTType = UTType {
    identifier: "public.3d-content",
    conforms_to: r#"["public.content"]"#,
    tags: r#"{}"#,
    description: r#"3D Content"#,
};
pub const PUBLIC_ALEMBIC: UTType = UTType {
    identifier: "public.alembic",
    conforms_to: r#"["public.3d-content", "public.data"]"#,
    tags: r#"{"public.filename-extension": ["abc"]}"#,
    description: r#"Alembic 3D Scene"#,
};
pub const PUBLIC_GEOMETRY_DEFINITION_FORMAT: UTType = UTType {
    identifier: "public.geometry-definition-format",
    conforms_to: r#"["public.3d-content", "public.text"]"#,
    tags: r#"{"public.filename-extension": ["obj"]}"#,
    description: r#"Geometry Definition File Format"#,
};
pub const PUBLIC_STANDARD_TESSELATED_GEOMETRY_FORMAT: UTType = UTType {
    identifier: "public.standard-tesselated-geometry-format",
    conforms_to: r#"["public.3d-content", "public.data"]"#,
    tags: r#"{"public.filename-extension": ["stl"]}"#,
    description: r#"Standard Tesselated Geometry File Format"#,
};
pub const PUBLIC_POLYGON_FILE_FORMAT: UTType = UTType {
    identifier: "public.polygon-file-format",
    conforms_to: r#"["public.3d-content", "public.data"]"#,
    tags: r#"{"public.filename-extension": ["ply"]}"#,
    description: r#"Polygon File Format"#,
};
pub const COM_PIXAR_UNIVERSAL_SCENE_DESCRIPTION: UTType = UTType {
    identifier: "com.pixar.universal-scene-description",
    conforms_to: r#"["public.3d-content", "public.data"]"#,
    tags: r#"{"public.filename-extension": ["usd", "usda", "usdc"]}"#,
    description: r#"Universal Scene Description"#,
};
pub const COM_PIXAR_UNIVERSAL_SCENE_DESCRIPTION_MOBILE: UTType = UTType {
    identifier: "com.pixar.universal-scene-description-mobile",
    conforms_to: r#"["public.3d-content", "public.data"]"#,
    tags: r#"{"public.filename-extension": ["usdz"], "public.mime-type": ["model/vnd.usdz+zip"]}"#,
    description: r#"Universal Scene Description Package"#,
};
pub const COM_APPLE_REALITY: UTType = UTType {
    identifier: "com.apple.reality",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["reality"], "public.mime-type": ["model/vnd.reality"]}"#,
    description: r#"Reality File"#,
};
pub const COM_APPLE_SCENEKIT_SCENE: UTType = UTType {
    identifier: "com.apple.scenekit.scene",
    conforms_to: r#"["public.data", "public.3d-content"]"#,
    tags: r#"{"public.filename-extension": ["scn", "scnz"]}"#,
    description: r#"SceneKit serialized format"#,
};
pub const COM_APPLE_AROBJECT: UTType = UTType {
    identifier: "com.apple.arobject",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["arobject"]}"#,
    description: r#"AR Reference Object"#,
};
pub const PUBLIC_MESSAGE: UTType = UTType {
    identifier: "public.message",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"message"#,
};
pub const PUBLIC_EMAIL_MESSAGE: UTType = UTType {
    identifier: "public.email-message",
    conforms_to: r#"["public.message"]"#,
    tags: r#"{}"#,
    description: r#"email message"#,
};
pub const PUBLIC_TO_DO_ITEM: UTType = UTType {
    identifier: "public.to-do-item",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"to-do item"#,
};
pub const PUBLIC_CALENDAR_EVENT: UTType = UTType {
    identifier: "public.calendar-event",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"calendar event"#,
};
pub const COM_APPLE_ICAL_VCS: UTType = UTType {
    identifier: "com.apple.ical.vcs",
    conforms_to: r#"["public.text", "public.item", "public.calendar-event"]"#,
    tags: r#"{"public.filename-extension": ["vcs", "vcal"], "public.mime-type": ["text/x-vcalendar"]}"#,
    description: r#"VCS File"#,
};
pub const COM_APPLE_ICAL_ICS: UTType = UTType {
    identifier: "com.apple.ical.ics",
    conforms_to: r#"["public.text", "public.item", "public.calendar-event"]"#,
    tags: r#"{"public.filename-extension": ["ics"], "public.mime-type": ["text/calendar"]}"#,
    description: r#"ICS File"#,
};
pub const PUBLIC_CONTACT: UTType = UTType {
    identifier: "public.contact",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"contact information"#,
};
pub const PUBLIC_VCARD: UTType = UTType {
    identifier: "public.vcard",
    conforms_to: r#"["public.text", "public.contact"]"#,
    tags: r#"{"public.filename-extension": ["vcf", "vcard"], "public.mime-type": ["text/vcard", "text/directory", "text/x-vcard"]}"#,
    description: r#"electronic business card"#,
};
pub const COM_APPLE_SHAZAMSIGNATURE: UTType = UTType {
    identifier: "com.apple.shazamsignature",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["shazamsignature"]}"#,
    description: r#"Shazam Signature Data"#,
};
pub const COM_APPLE_SHAZAMCATALOG: UTType = UTType {
    identifier: "com.apple.shazamcatalog",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["shazamcatalog"]}"#,
    description: r#"Shazam Catalog"#,
};
pub const PUBLIC_TEXT: UTType = UTType {
    identifier: "public.text",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{}"#,
    description: r#"text"#,
};
pub const PUBLIC_PLAIN_TEXT: UTType = UTType {
    identifier: "public.plain-text",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["txt", "text"], "public.mime-type": ["text/plain"]}"#,
    description: r#""#,
};
pub const PUBLIC_UTF8_PLAIN_TEXT: UTType = UTType {
    identifier: "public.utf8-plain-text",
    conforms_to: r#"["public.plain-text"]"#,
    tags: r#"{"public.mime-type": ["text/plain;charset=utf-8", "text/plain;charset="utf-8""]}"#,
    description: r#""#,
};
pub const PUBLIC_UTF16_EXTERNAL_PLAIN_TEXT: UTType = UTType {
    identifier: "public.utf16-external-plain-text",
    conforms_to: r#"["public.plain-text"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const PUBLIC_UTF16_PLAIN_TEXT: UTType = UTType {
    identifier: "public.utf16-plain-text",
    conforms_to: r#"["public.plain-text"]"#,
    tags: r#"{"public.mime-type": ["text/plain;charset=utf-16", "text/plain;charset="utf-16""]}"#,
    description: r#""#,
};
pub const COM_APPLE_TRADITIONAL_MAC_PLAIN_TEXT: UTType = UTType {
    identifier: "com.apple.traditional-mac-plain-text",
    conforms_to: r#"["public.plain-text"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const PUBLIC_CASE_INSENSITIVE_TEXT: UTType = UTType {
    identifier: "public.case-insensitive-text",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const PUBLIC_LOG: UTType = UTType {
    identifier: "public.log",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: r#"Log file"#,
};
pub const COM_APPLE_LOG: UTType = UTType {
    identifier: "com.apple.log",
    conforms_to: r#"["public.plain-text", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["log"]}"#,
    description: r#""#,
};
pub const COM_APPLE_SHUTDOWNSTALL: UTType = UTType {
    identifier: "com.apple.shutdownStall",
    conforms_to: r#"["public.utf8-plain-text", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["shutdownStall"]}"#,
    description: r#"Shutdown Stall"#,
};
pub const COM_APPLE_GPURESTART: UTType = UTType {
    identifier: "com.apple.gpuRestart",
    conforms_to: r#"["public.utf8-plain-text", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["gpuRestart"]}"#,
    description: r#"GPU Restart"#,
};
pub const COM_APPLE_CRASHREPORT: UTType = UTType {
    identifier: "com.apple.crashreport",
    conforms_to: r#"["public.utf8-plain-text", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["crash"]}"#,
    description: r#"Crash Report"#,
};
pub const COM_APPLE_HANGREPORT: UTType = UTType {
    identifier: "com.apple.hangreport",
    conforms_to: r#"["public.utf8-plain-text", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["hang"]}"#,
    description: r#"Hang Report"#,
};
pub const COM_APPLE_SPINREPORT: UTType = UTType {
    identifier: "com.apple.spinreport",
    conforms_to: r#"["public.utf8-plain-text", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["spin"]}"#,
    description: r#"Spin Report"#,
};
pub const COM_APPLE_PANICREPORT: UTType = UTType {
    identifier: "com.apple.panicreport",
    conforms_to: r#"["public.utf8-plain-text", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["panic"]}"#,
    description: r#"Panic Report"#,
};
pub const COM_APPLE_KTRACE: UTType = UTType {
    identifier: "com.apple.ktrace",
    conforms_to: r#"["public.data", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["ktrace"]}"#,
    description: r#"Darwin kernel trace file"#,
};
pub const PUBLIC_FILENAME_EXTENSION: UTType = UTType {
    identifier: "public.filename-extension",
    conforms_to: r#"["public.case-insensitive-text"]"#,
    tags: r#"{}"#,
    description: r#"file name extension"#,
};
pub const PUBLIC_MIME_TYPE: UTType = UTType {
    identifier: "public.mime-type",
    conforms_to: r#"["public.case-insensitive-text"]"#,
    tags: r#"{}"#,
    description: r#"MIME type"#,
};
pub const COM_APPLE_OSTYPE: UTType = UTType {
    identifier: "com.apple.ostype",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{}"#,
    description: r#"four-character code"#,
};
pub const COM_APPLE_NSPBOARD_TYPE: UTType = UTType {
    identifier: "com.apple.nspboard-type",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{}"#,
    description: r#"NSPasteboard type"#,
};
pub const COM_APPLE_DEVICE_MODEL_CODE: UTType = UTType {
    identifier: "com.apple.device-model-code",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{}"#,
    description: r#"device model code"#,
};
pub const COM_APPLE_PASTEBOARD_PROMISED_FILE_URL: UTType = UTType {
    identifier: "com.apple.pasteboard.promised-file-url",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Promised file URL"#,
};
pub const COM_APPLE_PASTEBOARD_PROMISED_FILE_CONTENT_TYPE: UTType = UTType {
    identifier: "com.apple.pasteboard.promised-file-content-type",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Promised file content type"#,
};
pub const COM_APPLE_COCOA_PASTEBOARD_COLOR: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.color",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"color"#,
};
pub const COM_APPLE_COCOA_PASTEBOARD_SOUND: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.sound",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"sound"#,
};
pub const COM_APPLE_COCOA_PASTEBOARD_CHARACTER_FORMATTING: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.character-formatting",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"character format"#,
};
pub const COM_APPLE_COCOA_PASTEBOARD_PARAGRAPH_FORMATTING: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.paragraph-formatting",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"paragraph format"#,
};
pub const COM_APPLE_COCOA_PASTEBOARD_MULTIPLE_TEXT_SELECTION: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.multiple-text-selection",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"text selection"#,
};
pub const COM_APPLE_COCOA_PASTEBOARD_FIND_PANEL_SEARCH_OPTIONS: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.find-panel-search-options",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"search options"#,
};
pub const COM_APPLE_MAPKIT_MAP_ITEM: UTType = UTType {
    identifier: "com.apple.mapkit.map-item",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"Map Item"#,
};
pub const COM_APPLE_RESOLVABLE: UTType = UTType {
    identifier: "com.apple.resolvable",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"resolvable"#,
};
pub const PUBLIC_SYMLINK: UTType = UTType {
    identifier: "public.symlink",
    conforms_to: r#"["public.item", "com.apple.resolvable"]"#,
    tags: r#"{}"#,
    description: r#"symbolic link"#,
};
pub const COM_APPLE_MOUNT_POINT: UTType = UTType {
    identifier: "com.apple.mount-point",
    conforms_to: r#"["public.item", "com.apple.resolvable"]"#,
    tags: r#"{}"#,
    description: r#"mount point"#,
};
pub const COM_APPLE_BOOKMARK: UTType = UTType {
    identifier: "com.apple.bookmark",
    conforms_to: r#"["public.data", "com.apple.resolvable"]"#,
    tags: r#"{}"#,
    description: r#"bookmark"#,
};
pub const COM_APPLE_ALIAS_FILE: UTType = UTType {
    identifier: "com.apple.alias-file",
    conforms_to: r#"["public.data", "com.apple.resolvable"]"#,
    tags: r#"{}"#,
    description: r#"alias"#,
};
pub const COM_APPLE_ALIAS_RECORD: UTType = UTType {
    identifier: "com.apple.alias-record",
    conforms_to: r#"["public.data", "com.apple.resolvable"]"#,
    tags: r#"{}"#,
    description: r#"alias data"#,
};
pub const COM_APPLE_ICLOUD_FILE_FAULT: UTType = UTType {
    identifier: "com.apple.icloud-file-fault",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["icloud"]}"#,
    description: r#"iCloud synchronization file"#,
};
pub const COM_APPLE_FINDER_CLIPPING: UTType = UTType {
    identifier: "com.apple.finder.clipping",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"clipping"#,
};
pub const COM_APPLE_FINDER_SOUND_CLIPPING: UTType = UTType {
    identifier: "com.apple.finder.sound-clipping",
    conforms_to: r#"["com.apple.finder.clipping"]"#,
    tags: r#"{"public.filename-extension": ["sndClipping"]}"#,
    description: r#"Sound Clipping"#,
};
pub const COM_APPLE_FINDER_TEXTCLIPPING: UTType = UTType {
    identifier: "com.apple.finder.textclipping",
    conforms_to: r#"["com.apple.finder.clipping"]"#,
    tags: r#"{"public.filename-extension": ["textclipping"]}"#,
    description: r#"text clipping"#,
};
pub const COM_APPLE_FINDER_PICTCLIPPING: UTType = UTType {
    identifier: "com.apple.finder.pictclipping",
    conforms_to: r#"["com.apple.finder.clipping", "public.image"]"#,
    tags: r#"{"public.filename-extension": ["pictclipping"]}"#,
    description: r#"picture clipping"#,
};
pub const COM_APPLE_FINDER_BURN_FOLDER: UTType = UTType {
    identifier: "com.apple.finder.burn-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{"public.filename-extension": ["fpbf"]}"#,
    description: r#"Burn Folder"#,
};
pub const COM_APPLE_ICONSET: UTType = UTType {
    identifier: "com.apple.iconset",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{"public.filename-extension": ["iconset"]}"#,
    description: r#"Icon Set"#,
};
pub const COM_APPLE_FINDER_SMART_FOLDER: UTType = UTType {
    identifier: "com.apple.finder.smart-folder",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["savedSearch"]}"#,
    description: r#"Smart Folder"#,
};
pub const COM_APPLE_FINDER_RECENT_ITEMS: UTType = UTType {
    identifier: "com.apple.finder.recent-items",
    conforms_to: r#"["com.apple.finder.smart-folder"]"#,
    tags: r#"{}"#,
    description: r#"Recent Items"#,
};
pub const PUBLIC_OBJECT_CODE: UTType = UTType {
    identifier: "public.object-code",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["o"]}"#,
    description: r#"object code"#,
};
pub const COM_APPLE_MACH_O_BINARY: UTType = UTType {
    identifier: "com.apple.mach-o-binary",
    conforms_to: r#"["public.unix-executable"]"#,
    tags: r#"{}"#,
    description: r#"Mach-O binary"#,
};
pub const COM_APPLE_MACH_O_OBJECT: UTType = UTType {
    identifier: "com.apple.mach-o-object",
    conforms_to: r#"["com.apple.mach-o-binary"]"#,
    tags: r#"{}"#,
    description: r#"Mach-O object"#,
};
pub const COM_APPLE_MACH_O_EXECUTABLE: UTType = UTType {
    identifier: "com.apple.mach-o-executable",
    conforms_to: r#"["com.apple.mach-o-binary"]"#,
    tags: r#"{}"#,
    description: r#"Mach-O executable"#,
};
pub const COM_APPLE_X11_MACH_O_EXECUTABLE: UTType = UTType {
    identifier: "com.apple.x11-mach-o-executable",
    conforms_to: r#"["com.apple.mach-o-binary"]"#,
    tags: r#"{}"#,
    description: r#"X11 application"#,
};
pub const COM_APPLE_MACH_O_CORE: UTType = UTType {
    identifier: "com.apple.mach-o-core",
    conforms_to: r#"["com.apple.mach-o-binary"]"#,
    tags: r#"{}"#,
    description: r#"Mach-O core"#,
};
pub const COM_APPLE_MACH_O_DYLIB: UTType = UTType {
    identifier: "com.apple.mach-o-dylib",
    conforms_to: r#"["com.apple.mach-o-binary"]"#,
    tags: r#"{"public.filename-extension": ["dylib"]}"#,
    description: r#"Mach-O dynamic library"#,
};
pub const COM_APPLE_MACH_O_BUNDLE: UTType = UTType {
    identifier: "com.apple.mach-o-bundle",
    conforms_to: r#"["com.apple.mach-o-binary"]"#,
    tags: r#"{}"#,
    description: r#"Mach-O bundle"#,
};
pub const COM_APPLE_PEF_BINARY: UTType = UTType {
    identifier: "com.apple.pef-binary",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{}"#,
    description: r#"PEF binary"#,
};
pub const PUBLIC_ELF_BINARY: UTType = UTType {
    identifier: "public.elf-binary",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{}"#,
    description: r#"ELF binary"#,
};
pub const COM_MICROSOFT_WINDOWS_EXECUTABLE: UTType = UTType {
    identifier: "com.microsoft.windows-executable",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["exe"], "public.mime-type": ["application/x-msdownload"]}"#,
    description: r#"Microsoft Windows application"#,
};
pub const COM_MICROSOFT_WINDOWS_DYNAMIC_LINK_LIBRARY: UTType = UTType {
    identifier: "com.microsoft.windows-dynamic-link-library",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["dll"], "public.mime-type": ["application/x-msdownload"]}"#,
    description: r#"Microsoft dynamic link library"#,
};
pub const COM_SUN_JAVA_CLASS: UTType = UTType {
    identifier: "com.sun.java-class",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["class"]}"#,
    description: r#"Java class"#,
};
pub const COM_SUN_JAVA_ARCHIVE: UTType = UTType {
    identifier: "com.sun.java-archive",
    conforms_to: r#"["public.zip-archive", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["jar"], "public.mime-type": ["application/java-archive"]}"#,
    description: r#"Java archive"#,
};
pub const COM_SUN_WEB_APPLICATION_ARCHIVE: UTType = UTType {
    identifier: "com.sun.web-application-archive",
    conforms_to: r#"["com.sun.java-archive"]"#,
    tags: r#"{"public.filename-extension": ["war"]}"#,
    description: r#"web application archive"#,
};
pub const COM_APPLE_QUARTZ_COMPOSER_COMPOSITION: UTType = UTType {
    identifier: "com.apple.quartz-composer-composition",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["qtz"], "public.mime-type": ["application/x-quartzcomposer"]}"#,
    description: r#"Quartz Composer compostion"#,
};
pub const COM_APPLE_BOM_ARCHIVE: UTType = UTType {
    identifier: "com.apple.bom-archive",
    conforms_to: r#"["public.archive"]"#,
    tags: r#"{}"#,
    description: r#"BOM-compatible archive"#,
};
pub const PUBLIC_DISK_IMAGE: UTType = UTType {
    identifier: "public.disk-image",
    conforms_to: r#"["public.archive"]"#,
    tags: r#"{}"#,
    description: r#"disk image"#,
};
pub const ORG_GNU_GNU_TAR_ARCHIVE: UTType = UTType {
    identifier: "org.gnu.gnu-tar-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["gtar"], "public.mime-type": ["application/x-gtar"]}"#,
    description: r#"GNU tar archive"#,
};
pub const PUBLIC_TAR_ARCHIVE: UTType = UTType {
    identifier: "public.tar-archive",
    conforms_to: r#"["org.gnu.gnu-tar-archive"]"#,
    tags: r#"{"public.filename-extension": ["tar"], "public.mime-type": ["application/x-tar", "application/tar"]}"#,
    description: r#"tar archive"#,
};
pub const ORG_GNU_GNU_ZIP_ARCHIVE: UTType = UTType {
    identifier: "org.gnu.gnu-zip-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["gz", "gzip"], "public.mime-type": ["application/x-gzip", "application/gzip"]}"#,
    description: r#"GZip archive"#,
};
pub const ORG_GNU_GNU_ZIP_TAR_ARCHIVE: UTType = UTType {
    identifier: "org.gnu.gnu-zip-tar-archive",
    conforms_to: r#"["org.gnu.gnu-zip-archive"]"#,
    tags: r#"{"public.filename-extension": ["tgz"]}"#,
    description: r#"gzip tar archive"#,
};
pub const PUBLIC_BZIP2_ARCHIVE: UTType = UTType {
    identifier: "public.bzip2-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["bz2", "bz"], "public.mime-type": ["application/x-bzip2", "application/x-bzip", "application/bzip2", "application/bzip", "application/x-bz2"]}"#,
    description: r#"Bzip2 archive"#,
};
pub const PUBLIC_TAR_BZIP2_ARCHIVE: UTType = UTType {
    identifier: "public.tar-bzip2-archive",
    conforms_to: r#"["public.bzip2-archive"]"#,
    tags: r#"{"public.filename-extension": ["tbz2", "tbz"]}"#,
    description: r#"Bzip2 compressed tar archive"#,
};
pub const COM_APPLE_BINHEX_ARCHIVE: UTType = UTType {
    identifier: "com.apple.binhex-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["hqx"], "public.mime-type": ["application/mac-binhex40", "application/mac-binhex", "application/binhex"]}"#,
    description: r#"BinHex archive"#,
};
pub const COM_APPLE_MACBINARY_ARCHIVE: UTType = UTType {
    identifier: "com.apple.macbinary-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["bin"], "public.mime-type": ["application/macbinary", "application/x-macbinary"]}"#,
    description: r#"MacBinary archive"#,
};
pub const COM_APPLE_APPLESINGLE_ARCHIVE: UTType = UTType {
    identifier: "com.apple.applesingle-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["as"]}"#,
    description: r#"AppleSingle archive"#,
};
pub const PUBLIC_UUENCODED_ARCHIVE: UTType = UTType {
    identifier: "public.uuencoded-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["uu"], "public.mime-type": ["text/x-uuencode"]}"#,
    description: r#"UUEncoded archive"#,
};
pub const PUBLIC_Z_ARCHIVE: UTType = UTType {
    identifier: "public.z-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["z"], "public.mime-type": ["application/x-compress"]}"#,
    description: r#"Z archive"#,
};
pub const COM_APPLE_BOM_COMPRESSED_CPIO: UTType = UTType {
    identifier: "com.apple.bom-compressed-cpio",
    conforms_to: r#"["public.data", "com.apple.bom-archive"]"#,
    tags: r#"{"public.filename-extension": ["cpgz"]}"#,
    description: r#"BOM-generated compressed CPIO archive"#,
};
pub const PUBLIC_CPIO_ARCHIVE: UTType = UTType {
    identifier: "public.cpio-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["cpio", "pax"]}"#,
    description: r#"CPIO archive"#,
};
pub const COM_PKWARE_ZIP_ARCHIVE: UTType = UTType {
    identifier: "com.pkware.zip-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{}"#,
    description: r#"PKZip archive"#,
};
pub const PUBLIC_ZIP_ARCHIVE: UTType = UTType {
    identifier: "public.zip-archive",
    conforms_to: r#"["com.pkware.zip-archive"]"#,
    tags: r#"{"public.filename-extension": ["zip"], "public.mime-type": ["application/zip", "application/x-zip-compressed"]}"#,
    description: r#"Zip archive"#,
};
pub const COM_APPLE_XAR_ARCHIVE: UTType = UTType {
    identifier: "com.apple.xar-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["xar"]}"#,
    description: r#"XAR Archive"#,
};
pub const COM_APPLE_XIP_ARCHIVE: UTType = UTType {
    identifier: "com.apple.xip-archive",
    conforms_to: r#"["com.apple.xar-archive"]"#,
    tags: r#"{"public.filename-extension": ["xip"]}"#,
    description: r#"XIP Secure Archive"#,
};
pub const COM_APPLE_INSTALLER_PACKAGE_ARCHIVE: UTType = UTType {
    identifier: "com.apple.installer-package-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["pkg", "mpkg"]}"#,
    description: r#"Installer package archive"#,
};
pub const COM_APPLE_ARCHIVE: UTType = UTType {
    identifier: "com.apple.archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["aar", "yaa"]}"#,
    description: r#"Apple Archive"#,
};
pub const COM_APPLE_ENCRYPTED_ARCHIVE: UTType = UTType {
    identifier: "com.apple.encrypted-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["aea"]}"#,
    description: r#"Apple Encrypted Archive"#,
};
pub const PUBLIC_URL: UTType = UTType {
    identifier: "public.url",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"URL"#,
};
pub const PUBLIC_FILE_URL: UTType = UTType {
    identifier: "public.file-url",
    conforms_to: r#"["public.url"]"#,
    tags: r#"{}"#,
    description: r#"file URL"#,
};
pub const PUBLIC_URL_NAME: UTType = UTType {
    identifier: "public.url-name",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"URL name"#,
};
pub const PUBLIC_STORED_URL: UTType = UTType {
    identifier: "public.stored-url",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.internet-location",
    conforms_to: r#"["public.stored-url", "public.data"]"#,
    tags: r#"{}"#,
    description: r#"internet location"#,
};
pub const COM_APPLE_WEB_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.web-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["webloc"]}"#,
    description: r#"web internet location"#,
};
pub const COM_APPLE_VNC_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.vnc-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["vncloc"]}"#,
    description: r#"VNC Internet Location"#,
};
pub const COM_APPLE_MAIL_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.mail-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["mailloc"]}"#,
    description: r#"mail internet location"#,
};
pub const COM_APPLE_AFP_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.afp-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["afploc"]}"#,
    description: r#"AFP internet location"#,
};
pub const COM_APPLE_FILE_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.file-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["fileloc"]}"#,
    description: r#"file internet location"#,
};
pub const COM_APPLE_FTP_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.ftp-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["ftploc"]}"#,
    description: r#"FTP internet location"#,
};
pub const COM_APPLE_NEWS_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.news-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["newsloc"]}"#,
    description: r#"news internet location"#,
};
pub const COM_APPLE_GENERIC_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.generic-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["inetloc"]}"#,
    description: r#"internet location"#,
};
pub const COM_MICROSOFT_INTERNET_SHORTCUT: UTType = UTType {
    identifier: "com.microsoft.internet-shortcut",
    conforms_to: r#"["public.stored-url", "public.data"]"#,
    tags: r#"{"public.filename-extension": ["url"]}"#,
    description: r#"Windows Internet shortcut"#,
};
pub const COM_APPLE_ITUNES_STORE_URL: UTType = UTType {
    identifier: "com.apple.itunes.store-url",
    conforms_to: r#"["public.url"]"#,
    tags: r#"{"public.filename-extension": ["itms"]}"#,
    description: r#"iTunes store URL"#,
};
pub const PUBLIC_DELIMITED_VALUES_TEXT: UTType = UTType {
    identifier: "public.delimited-values-text",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{}"#,
    description: r#"delimited text values"#,
};
pub const PUBLIC_COMMA_SEPARATED_VALUES_TEXT: UTType = UTType {
    identifier: "public.comma-separated-values-text",
    conforms_to: r#"["public.plain-text", "public.delimited-values-text"]"#,
    tags: r#"{"public.filename-extension": ["csv"], "public.mime-type": ["text/csv", "text/comma-separated-values"]}"#,
    description: r#"comma-separated values"#,
};
pub const PUBLIC_TAB_SEPARATED_VALUES_TEXT: UTType = UTType {
    identifier: "public.tab-separated-values-text",
    conforms_to: r#"["public.plain-text", "public.delimited-values-text"]"#,
    tags: r#"{"public.filename-extension": ["tsv"], "public.mime-type": ["text/tab-separated-values"]}"#,
    description: r#"tab-separated values"#,
};
pub const PUBLIC_UTF8_TAB_SEPARATED_VALUES_TEXT: UTType = UTType {
    identifier: "public.utf8-tab-separated-values-text",
    conforms_to: r#"["public.tab-separated-values-text", "public.utf8-plain-text"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const PUBLIC_RTF: UTType = UTType {
    identifier: "public.rtf",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["rtf"], "public.mime-type": ["text/rtf"]}"#,
    description: r#"rich text (RTF)"#,
};
pub const PUBLIC_HTML: UTType = UTType {
    identifier: "public.html",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["html", "htm", "shtml", "shtm"], "public.mime-type": ["text/html"]}"#,
    description: r#"HTML text"#,
};
pub const PUBLIC_XML: UTType = UTType {
    identifier: "public.xml",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["xml"], "public.mime-type": ["application/xml", "text/xml"]}"#,
    description: r#"XML text"#,
};
pub const PUBLIC_XHTML: UTType = UTType {
    identifier: "public.xhtml",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{"public.filename-extension": ["xhtml", "xhtm", "xht"], "public.mime-type": ["application/xhtml+xml"]}"#,
    description: r#"XHTML"#,
};
pub const PUBLIC_RSS: UTType = UTType {
    identifier: "public.rss",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{"public.filename-extension": ["rss"], "public.mime-type": ["application/rss+xml"]}"#,
    description: r#"RSS web feed"#,
};
pub const PUBLIC_XFD: UTType = UTType {
    identifier: "public.xfd",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{"public.filename-extension": ["xfd"]}"#,
    description: r#"XML Form (XFD)"#,
};
pub const PUBLIC_CSS: UTType = UTType {
    identifier: "public.css",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["css"], "public.mime-type": ["text/css"]}"#,
    description: r#"CSS"#,
};
pub const PUBLIC_PATCH_FILE: UTType = UTType {
    identifier: "public.patch-file",
    conforms_to: r#"["public.plain-text"]"#,
    tags: r#"{"public.filename-extension": ["patch", "diff"]}"#,
    description: r#"patch file"#,
};
pub const PUBLIC_JSON: UTType = UTType {
    identifier: "public.json",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["json"], "public.mime-type": ["application/json"]}"#,
    description: r#"JSON"#,
};
pub const PUBLIC_NDJSON: UTType = UTType {
    identifier: "public.ndjson",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["ndjson"], "public.mime-type": ["application/x-ndjson"]}"#,
    description: r#"NDJSON"#,
};
pub const PUBLIC_YAML: UTType = UTType {
    identifier: "public.yaml",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["yml", "yaml"], "public.mime-type": ["application/x-yaml"]}"#,
    description: r#"YAML"#,
};
pub const COM_SCENARIST_CLOSED_CAPTION: UTType = UTType {
    identifier: "com.scenarist.closed-caption",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["scc"]}"#,
    description: r#"Scenarist Closed Caption"#,
};
pub const ORG_W3_WEBVTT: UTType = UTType {
    identifier: "org.w3.webvtt",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["vtt"], "public.mime-type": ["text/vtt"]}"#,
    description: r#"WebVTT Format"#,
};
pub const COM_APPLE_GENERIC_STATIONERY: UTType = UTType {
    identifier: "com.apple.generic-stationery",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"Stationery"#,
};
pub const COM_APPLE_PROPERTY_LIST: UTType = UTType {
    identifier: "com.apple.property-list",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["plist"]}"#,
    description: r#"property list"#,
};
pub const COM_APPLE_XML_PROPERTY_LIST: UTType = UTType {
    identifier: "com.apple.xml-property-list",
    conforms_to: r#"["public.xml", "com.apple.property-list"]"#,
    tags: r#"{"public.filename-extension": ["plist"]}"#,
    description: r#"XML property list"#,
};
pub const COM_APPLE_BINARY_PROPERTY_LIST: UTType = UTType {
    identifier: "com.apple.binary-property-list",
    conforms_to: r#"["com.apple.property-list"]"#,
    tags: r#"{"public.filename-extension": ["plist"]}"#,
    description: r#"binary property list"#,
};
pub const COM_APPLE_ASCII_PROPERTY_LIST: UTType = UTType {
    identifier: "com.apple.ascii-property-list",
    conforms_to: r#"["public.text", "com.apple.property-list"]"#,
    tags: r#"{"public.filename-extension": ["plist"]}"#,
    description: r#"ascii property list"#,
};
pub const PUBLIC_SOURCE_CODE: UTType = UTType {
    identifier: "public.source-code",
    conforms_to: r#"["public.plain-text"]"#,
    tags: r#"{}"#,
    description: r#"source code"#,
};
pub const PUBLIC_SOURCE_CODE_PREPROCESSED: UTType = UTType {
    identifier: "public.source-code.preprocessed",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{}"#,
    description: r#"preprocessed source code"#,
};
pub const PUBLIC_C_SOURCE: UTType = UTType {
    identifier: "public.c-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["c"]}"#,
    description: r#"C source code"#,
};
pub const PUBLIC_C_SOURCE_PREPROCESSED: UTType = UTType {
    identifier: "public.c-source.preprocessed",
    conforms_to: r#"["public.c-source", "public.source-code.preprocessed"]"#,
    tags: r#"{"public.filename-extension": ["i"]}"#,
    description: r#"preprocessed C source code"#,
};
pub const COM_APPLE_IIG_SOURCE: UTType = UTType {
    identifier: "com.apple.iig-source",
    conforms_to: r#"["public.c-source"]"#,
    tags: r#"{"public.filename-extension": ["iig"]}"#,
    description: r#"IOKit Interface Generator source code"#,
};
pub const PUBLIC_OBJECTIVE_C_SOURCE: UTType = UTType {
    identifier: "public.objective-c-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["m"]}"#,
    description: r#"Objective-C source code"#,
};
pub const PUBLIC_OBJECTIVE_C_SOURCE_PREPROCESSED: UTType = UTType {
    identifier: "public.objective-c-source.preprocessed",
    conforms_to: r#"["public.objective-c-source", "public.source-code.preprocessed"]"#,
    tags: r#"{"public.filename-extension": ["mi"]}"#,
    description: r#"preprocessed Objective-C source code"#,
};
pub const PUBLIC_C_PLUS_PLUS_SOURCE: UTType = UTType {
    identifier: "public.c-plus-plus-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["cp", "cpp", "c++", "cc", "cxx"]}"#,
    description: r#"C++ source code"#,
};
pub const PUBLIC_C_PLUS_PLUS_SOURCE_PREPROCESSED: UTType = UTType {
    identifier: "public.c-plus-plus-source.preprocessed",
    conforms_to: r#"["public.c-plus-plus-source", "public.source-code.preprocessed"]"#,
    tags: r#"{"public.filename-extension": ["ii"]}"#,
    description: r#"preprocessed C++ source code"#,
};
pub const PUBLIC_OBJECTIVE_C_PLUS_PLUS_SOURCE: UTType = UTType {
    identifier: "public.objective-c-plus-plus-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["mm"]}"#,
    description: r#"Objective-C++ source code"#,
};
pub const PUBLIC_OBJECTIVE_C_PLUS_PLUS_SOURCE_PREPROCESSED: UTType = UTType {
    identifier: "public.objective-c-plus-plus-source.preprocessed",
    conforms_to: r#"["public.objective-c-plus-plus-source", "public.source-code.preprocessed"]"#,
    tags: r#"{"public.filename-extension": ["mii"]}"#,
    description: r#"preprocessed Objective-C++ source code"#,
};
pub const PUBLIC_C_HEADER: UTType = UTType {
    identifier: "public.c-header",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["h"]}"#,
    description: r#"C header code"#,
};
pub const PUBLIC_PRECOMPILED_C_HEADER: UTType = UTType {
    identifier: "public.precompiled-c-header",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["pch"]}"#,
    description: r#"precompiled C header"#,
};
pub const PUBLIC_C_PLUS_PLUS_HEADER: UTType = UTType {
    identifier: "public.c-plus-plus-header",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["hh", "hp", "hpp", "hxx", "h++", "ipp"]}"#,
    description: r#"C++ header code"#,
};
pub const PUBLIC_C_PLUS_PLUS_INLINE_HEADER: UTType = UTType {
    identifier: "public.c-plus-plus-inline-header",
    conforms_to: r#"["public.c-plus-plus-header"]"#,
    tags: r#"{"public.filename-extension": ["inl"]}"#,
    description: r#"C++ Inline Header"#,
};
pub const PUBLIC_PRECOMPILED_C_PLUS_PLUS_HEADER: UTType = UTType {
    identifier: "public.precompiled-c-plus-plus-header",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["pch++"]}"#,
    description: r#"precompiled C++ header"#,
};
pub const PUBLIC_SWIFT_SOURCE: UTType = UTType {
    identifier: "public.swift-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["swift"]}"#,
    description: r#"Swift Source Code"#,
};
pub const COM_SUN_JAVA_SOURCE: UTType = UTType {
    identifier: "com.sun.java-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["java", "jav"]}"#,
    description: r#"Java source code"#,
};
pub const PUBLIC_SCRIPT: UTType = UTType {
    identifier: "public.script",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{}"#,
    description: r#"script"#,
};
pub const PUBLIC_ASSEMBLY_SOURCE: UTType = UTType {
    identifier: "public.assembly-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["s"]}"#,
    description: r#"assembly source code"#,
};
pub const COM_APPLE_REZ_SOURCE: UTType = UTType {
    identifier: "com.apple.rez-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["r"]}"#,
    description: r#"Rez source code"#,
};
pub const PUBLIC_LEX_SOURCE: UTType = UTType {
    identifier: "public.lex-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["l", "lm", "lmm", "lpp", "lxx", "ll"]}"#,
    description: r#"Lex source code"#,
};
pub const PUBLIC_YACC_SOURCE: UTType = UTType {
    identifier: "public.yacc-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["y", "ym", "ymm", "ypp", "yxx", "yy"]}"#,
    description: r#"Yacc source code"#,
};
pub const PUBLIC_MIG_SOURCE: UTType = UTType {
    identifier: "public.mig-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["defs", "mig"]}"#,
    description: r#"Mig definition source code"#,
};
pub const COM_APPLE_SYMBOL_EXPORT: UTType = UTType {
    identifier: "com.apple.symbol-export",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["exp"]}"#,
    description: r#"symbol export list"#,
};
pub const PUBLIC_FORTRAN_SOURCE: UTType = UTType {
    identifier: "public.fortran-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["f", "for"]}"#,
    description: r#"Fortran source code"#,
};
pub const PUBLIC_FORTRAN_77_SOURCE: UTType = UTType {
    identifier: "public.fortran-77-source",
    conforms_to: r#"["public.fortran-source"]"#,
    tags: r#"{"public.filename-extension": ["f77"]}"#,
    description: r#""#,
};
pub const PUBLIC_FORTRAN_90_SOURCE: UTType = UTType {
    identifier: "public.fortran-90-source",
    conforms_to: r#"["public.fortran-source"]"#,
    tags: r#"{"public.filename-extension": ["f90"]}"#,
    description: r#""#,
};
pub const PUBLIC_FORTRAN_95_SOURCE: UTType = UTType {
    identifier: "public.fortran-95-source",
    conforms_to: r#"["public.fortran-source"]"#,
    tags: r#"{"public.filename-extension": ["f95"]}"#,
    description: r#""#,
};
pub const PUBLIC_PASCAL_SOURCE: UTType = UTType {
    identifier: "public.pascal-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["pas"]}"#,
    description: r#"Pascal source code"#,
};
pub const PUBLIC_ADA_SOURCE: UTType = UTType {
    identifier: "public.ada-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["ada", "adb", "ads"]}"#,
    description: r#"Ada source code"#,
};
pub const PUBLIC_DYLAN_SOURCE: UTType = UTType {
    identifier: "public.dylan-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["dlyan", "lid"]}"#,
    description: r#"Dylan source code"#,
};
pub const COM_NETSCAPE_JAVASCRIPT_SOURCE: UTType = UTType {
    identifier: "com.netscape.javascript-source",
    conforms_to: r#"["public.script", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["js", "jscript", "javascript", "mjs"], "public.mime-type": ["text/javascript"]}"#,
    description: r#"JavaScript"#,
};
pub const COM_APPLE_XCODE_DSYM: UTType = UTType {
    identifier: "com.apple.xcode.dsym",
    conforms_to: r#"["com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["dsym"]}"#,
    description: r#""#,
};
pub const PUBLIC_SHELL_SCRIPT: UTType = UTType {
    identifier: "public.shell-script",
    conforms_to: r#"["public.script"]"#,
    tags: r#"{"public.filename-extension": ["sh"]}"#,
    description: r#"shell script"#,
};
pub const PUBLIC_BASH_SCRIPT: UTType = UTType {
    identifier: "public.bash-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["bash"]}"#,
    description: r#"Bourne-Again Shell script"#,
};
pub const PUBLIC_CSH_SCRIPT: UTType = UTType {
    identifier: "public.csh-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["csh"]}"#,
    description: r#"C Shell script"#,
};
pub const PUBLIC_KSH_SCRIPT: UTType = UTType {
    identifier: "public.ksh-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["ksh"]}"#,
    description: r#"Korn Shell script"#,
};
pub const PUBLIC_TCSH_SCRIPT: UTType = UTType {
    identifier: "public.tcsh-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["tcsh"]}"#,
    description: r#"Tenex C Shell script"#,
};
pub const PUBLIC_ZSH_SCRIPT: UTType = UTType {
    identifier: "public.zsh-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["zsh"]}"#,
    description: r#"Z Shell script"#,
};
pub const PUBLIC_PERL_SCRIPT: UTType = UTType {
    identifier: "public.perl-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["pl", "pm"], "public.mime-type": ["text/x-perl-script"]}"#,
    description: r#"Perl script"#,
};
pub const PUBLIC_PYTHON_SCRIPT: UTType = UTType {
    identifier: "public.python-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["py"], "public.mime-type": ["text/x-python-script"]}"#,
    description: r#"Python script"#,
};
pub const PUBLIC_RUBY_SCRIPT: UTType = UTType {
    identifier: "public.ruby-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["rb", "rbw"], "public.mime-type": ["text/x-ruby-script"]}"#,
    description: r#"Ruby script"#,
};
pub const PUBLIC_PHP_SCRIPT: UTType = UTType {
    identifier: "public.php-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["php", "php3", "php4", "ph3", "ph4", "phtml"], "public.mime-type": ["text/php", "text/x-php-script", "application/php"]}"#,
    description: r#"PHP script"#,
};
pub const COM_SUN_JAVA_WEB_START: UTType = UTType {
    identifier: "com.sun.java-web-start",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{"public.filename-extension": ["jnlp"], "public.mime-type": ["application/x-java-jnlp-file", "application/jnlp"]}"#,
    description: r#"Java web start"#,
};
pub const PUBLIC_MAKE_SOURCE: UTType = UTType {
    identifier: "public.make-source",
    conforms_to: r#"["public.script"]"#,
    tags: r#"{"public.filename-extension": ["make", "mak", "gmk", "mk"]}"#,
    description: r#"Makefile"#,
};
pub const PUBLIC_IMAGE: UTType = UTType {
    identifier: "public.image",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{}"#,
    description: r#"image"#,
};
pub const COM_APPLE_LIVE_PHOTO: UTType = UTType {
    identifier: "com.apple.live-photo",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Live Photo"#,
};
pub const COM_APPLE_PRIVATE_LIVE_PHOTO_BUNDLE: UTType = UTType {
    identifier: "com.apple.private.live-photo-bundle",
    conforms_to: r#"["com.apple.live-photo", "com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["pvt"]}"#,
    description: r#""#,
};
pub const PUBLIC_FAX: UTType = UTType {
    identifier: "public.fax",
    conforms_to: r#"["public.image", "public.message"]"#,
    tags: r#"{}"#,
    description: r#"fax"#,
};
pub const PUBLIC_CAMERA_RAW_IMAGE: UTType = UTType {
    identifier: "public.camera-raw-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: r#"camera raw image"#,
};
pub const PUBLIC_JPEG: UTType = UTType {
    identifier: "public.jpeg",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["jpeg", "jpg", "jpe"], "public.mime-type": ["image/jpeg", "image/jpg"]}"#,
    description: r#"JPEG image"#,
};
pub const PUBLIC_JPEG_2000: UTType = UTType {
    identifier: "public.jpeg-2000",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["jp2", "jpf", "jpx", "j2k", "j2c"], "public.mime-type": ["image/jp2"]}"#,
    description: r#"JPEG 2000 image"#,
};
pub const PUBLIC_TIFF: UTType = UTType {
    identifier: "public.tiff",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["tiff", "tif"], "public.mime-type": ["image/tiff"]}"#,
    description: r#"TIFF image"#,
};
pub const COM_APPLE_PICT: UTType = UTType {
    identifier: "com.apple.pict",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["pict", "pct", "pic"], "public.mime-type": ["image/pict", "image/x-pict", "image/x-macpict"]}"#,
    description: r#"QuickDraw picture"#,
};
pub const COM_APPLE_MACPAINT_IMAGE: UTType = UTType {
    identifier: "com.apple.macpaint-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["pntg"]}"#,
    description: r#"MacPaint image"#,
};
pub const PUBLIC_PNG: UTType = UTType {
    identifier: "public.png",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["png"], "public.mime-type": ["image/png"]}"#,
    description: r#"PNG image"#,
};
pub const PUBLIC_SVG_IMAGE: UTType = UTType {
    identifier: "public.svg-image",
    conforms_to: r#"["public.image", "public.xml"]"#,
    tags: r#"{"public.filename-extension": ["svg", "svgz"], "public.mime-type": ["image/svg+xml"]}"#,
    description: r#"SVG image"#,
};
pub const COM_APPLE_QUICKTIME_IMAGE: UTType = UTType {
    identifier: "com.apple.quicktime-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["qtif", "qti"], "public.mime-type": ["image/x-quicktime"]}"#,
    description: r#"QuickTime image"#,
};
pub const COM_APPLE_ICNS: UTType = UTType {
    identifier: "com.apple.icns",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["icns"]}"#,
    description: r#"Apple icon image"#,
};
pub const PUBLIC_XBITMAP_IMAGE: UTType = UTType {
    identifier: "public.xbitmap-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["xbm"], "public.mime-type": ["image/x-xbitmap"]}"#,
    description: r#"X bitmap image"#,
};
pub const PUBLIC_MPO_IMAGE: UTType = UTType {
    identifier: "public.mpo-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["mpo"]}"#,
    description: r#"Multiple Picture Object image"#,
};
pub const CA_MCGILL_MNI_BIC_MNC: UTType = UTType {
    identifier: "ca.mcgill.mni.bic.mnc",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["mnc", "minc"]}"#,
    description: r#"MINC Image"#,
};
pub const ORG_NEMA_DICOM: UTType = UTType {
    identifier: "org.nema.dicom",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["dcm", "dicom"], "public.mime-type": ["application/dicom"]}"#,
    description: r#"DICOM"#,
};
pub const GOV_NIH_NIFTI_1: UTType = UTType {
    identifier: "gov.nih.nifti-1",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["nii"]}"#,
    description: r#"NIfTI-1"#,
};
pub const PUBLIC_AUDIOVISUAL_CONTENT: UTType = UTType {
    identifier: "public.audiovisual-content",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{}"#,
    description: r#"audiovisual content"#,
};
pub const PUBLIC_MOVIE: UTType = UTType {
    identifier: "public.movie",
    conforms_to: r#"["public.audiovisual-content"]"#,
    tags: r#"{}"#,
    description: r#"movie"#,
};
pub const PUBLIC_VIDEO: UTType = UTType {
    identifier: "public.video",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{}"#,
    description: r#"video"#,
};
pub const PUBLIC_AUDIO: UTType = UTType {
    identifier: "public.audio",
    conforms_to: r#"["public.audiovisual-content"]"#,
    tags: r#"{}"#,
    description: r#"audio"#,
};
pub const COM_APPLE_QUICKTIME_MOVIE: UTType = UTType {
    identifier: "com.apple.quicktime-movie",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["mov", "qt"], "public.mime-type": ["video/quicktime"]}"#,
    description: r#"QuickTime movie"#,
};
pub const PUBLIC_MPEG: UTType = UTType {
    identifier: "public.mpeg",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["mpg", "mpeg", "mpe", "m75", "m15"], "public.mime-type": ["video/mpeg", "video/mpg", "video/x-mpeg", "video/x-mpg"]}"#,
    description: r#"MPEG movie"#,
};
pub const PUBLIC_MPEG_2_VIDEO: UTType = UTType {
    identifier: "public.mpeg-2-video",
    conforms_to: r#"["public.video"]"#,
    tags: r#"{"public.filename-extension": ["m2v"], "public.mime-type": ["video/mpeg2", "video/mpeg2-video"]}"#,
    description: r#"MPEG-2 video"#,
};
pub const PUBLIC_MPEG_4: UTType = UTType {
    identifier: "public.mpeg-4",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["mp4", "mpg4"], "public.mime-type": ["video/mp4", "video/mp4v-es"]}"#,
    description: r#"MPEG-4 movie"#,
};
pub const COM_APPLE_M4V_VIDEO: UTType = UTType {
    identifier: "com.apple.m4v-video",
    conforms_to: r#"["public.mpeg-4"]"#,
    tags: r#"{"public.filename-extension": ["m4v"], "public.mime-type": ["video/x-m4v"]}"#,
    description: r#"Apple MPEG-4 movie"#,
};
pub const COM_APPLE_PROTECTED_MPEG_4_VIDEO: UTType = UTType {
    identifier: "com.apple.protected-mpeg-4-video",
    conforms_to: r#"["com.apple.m4v-video"]"#,
    tags: r#"{}"#,
    description: r#"protected MPEG-4 movie"#,
};
pub const PUBLIC_DV_MOVIE: UTType = UTType {
    identifier: "public.dv-movie",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["dv", "dif"], "public.mime-type": ["video/x-dv"]}"#,
    description: r#"DV movie"#,
};
pub const PUBLIC_AVI: UTType = UTType {
    identifier: "public.avi",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["avi", "vfw"], "public.mime-type": ["video/avi", "video/msvideo", "video/x-msvideo"]}"#,
    description: r#"AVI movie"#,
};
pub const PUBLIC_3GPP: UTType = UTType {
    identifier: "public.3gpp",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["3gp", "3gpp", "sdv"], "public.mime-type": ["video/3gpp", "audio/3gpp"]}"#,
    description: r#"3GPP movie"#,
};
pub const PUBLIC_3GPP2: UTType = UTType {
    identifier: "public.3gpp2",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["3g2", "3gp2"], "public.mime-type": ["video/3gpp2", "audio/3gpp2"]}"#,
    description: r#"3GPP2 movie"#,
};
pub const PUBLIC_FLC_ANIMATION: UTType = UTType {
    identifier: "public.flc-animation",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["flc", "fli", "cel"], "public.mime-type": ["video/flc"]}"#,
    description: r#"FLC animation"#,
};
pub const PUBLIC_MPEG_2_TRANSPORT_STREAM: UTType = UTType {
    identifier: "public.mpeg-2-transport-stream",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["ts"]}"#,
    description: r#"MPEG-2 Transport Stream"#,
};
pub const PUBLIC_AUDIOVISUAL_CONTENT_COLLECTION: UTType = UTType {
    identifier: "public.audiovisual-content-collection",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Audiovisual Collection"#,
};
pub const PUBLIC_AVCHD_COLLECTION: UTType = UTType {
    identifier: "public.avchd-collection",
    conforms_to: r#"["com.apple.package", "public.audiovisual-content-collection"]"#,
    tags: r#"{"public.filename-extension": ["avchd"]}"#,
    description: r#"AVCHD Collection"#,
};
pub const COM_APPLE_AUDIO_UNIT_PRESET: UTType = UTType {
    identifier: "com.apple.audio-unit-preset",
    conforms_to: r#"["com.apple.xml-property-list"]"#,
    tags: r#"{"public.filename-extension": ["aupreset"]}"#,
    description: r#"audio unit preset"#,
};
pub const PUBLIC_MP2: UTType = UTType {
    identifier: "public.mp2",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["mp2"]}"#,
    description: r#"MP2 audio"#,
};
pub const PUBLIC_MP3: UTType = UTType {
    identifier: "public.mp3",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["mp3", "mpga"], "public.mime-type": ["audio/mpeg", "audio/mpeg3", "audio/mpg", "audio/mp3", "audio/x-mpeg", "audio/x-mpeg3", "audio/x-mpg", "audio/x-mp3"]}"#,
    description: r#"MP3 audio"#,
};
pub const PUBLIC_PLAYLIST: UTType = UTType {
    identifier: "public.playlist",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"playlist"#,
};
pub const PUBLIC_M3U_PLAYLIST: UTType = UTType {
    identifier: "public.m3u-playlist",
    conforms_to: r#"["public.plain-text", "public.playlist"]"#,
    tags: r#"{"public.filename-extension": ["m3u", "m3u8"], "public.mime-type": ["audio/mpegurl", "application/vnd.apple.mpegurl", "audio/x-mpegurl"]}"#,
    description: r#"M3U Playlist"#,
};
pub const PUBLIC_PLS_PLAYLIST: UTType = UTType {
    identifier: "public.pls-playlist",
    conforms_to: r#"["public.text", "public.playlist"]"#,
    tags: r#"{"public.filename-extension": ["pls"], "public.mime-type": ["audio/x-scpls"]}"#,
    description: r#"PLS Playlist"#,
};
pub const PUBLIC_MPEG_4_AUDIO: UTType = UTType {
    identifier: "public.mpeg-4-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["mp4", "mpg4"], "public.mime-type": ["audio/mp4", "audio/mp4a-latm"]}"#,
    description: r#"MPEG-4 audio"#,
};
pub const COM_APPLE_M4A_AUDIO: UTType = UTType {
    identifier: "com.apple.m4a-audio",
    conforms_to: r#"["public.mpeg-4-audio"]"#,
    tags: r#"{"public.filename-extension": ["m4a"], "public.mime-type": ["audio/x-m4a"]}"#,
    description: r#"Apple MPEG-4 audio"#,
};
pub const COM_APPLE_MPEG_4_RINGTONE: UTType = UTType {
    identifier: "com.apple.mpeg-4-ringtone",
    conforms_to: r#"["public.mpeg-4-audio"]"#,
    tags: r#"{"public.filename-extension": ["m4r"], "public.mime-type": ["audio/x-m4r"]}"#,
    description: r#"Ringtone"#,
};
pub const COM_APPLE_PROTECTED_MPEG_4_AUDIO: UTType = UTType {
    identifier: "com.apple.protected-mpeg-4-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["m4p"]}"#,
    description: r#"protected MPEG-4 audio"#,
};
pub const COM_APPLE_PROTECTED_MPEG_4_AUDIO_B: UTType = UTType {
    identifier: "com.apple.protected-mpeg-4-audio-b",
    conforms_to: r#"["com.apple.protected-mpeg-4-audio"]"#,
    tags: r#"{"public.filename-extension": ["m4b"]}"#,
    description: r#"protected MPEG-4 audio"#,
};
pub const PUBLIC_ULAW_AUDIO: UTType = UTType {
    identifier: "public.ulaw-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["ul", "ulw", "ulaw"]}"#,
    description: r#"uLaw audio"#,
};
pub const PUBLIC_AU_AUDIO: UTType = UTType {
    identifier: "public.au-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["au", "snd"], "public.mime-type": ["audio/basic"]}"#,
    description: r#"AU audio"#,
};
pub const PUBLIC_AIFC_AUDIO: UTType = UTType {
    identifier: "public.aifc-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["aifc"]}"#,
    description: r#"AIFF-C audio"#,
};
pub const PUBLIC_AIFF_AUDIO: UTType = UTType {
    identifier: "public.aiff-audio",
    conforms_to: r#"["public.aifc-audio"]"#,
    tags: r#"{"public.filename-extension": ["aiff", "aif"], "public.mime-type": ["audio/aiff", "audio/x-aiff"]}"#,
    description: r#"AIFF audio"#,
};
pub const PUBLIC_CDDA_AUDIO: UTType = UTType {
    identifier: "public.cdda-audio",
    conforms_to: r#"["public.aifc-audio"]"#,
    tags: r#"{"public.filename-extension": ["cdda"]}"#,
    description: r#"CDDA audio"#,
};
pub const PUBLIC_MIDI_AUDIO: UTType = UTType {
    identifier: "public.midi-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["midi", "mid", "smf", "kar"], "public.mime-type": ["audio/midi", "audio/x-midi"]}"#,
    description: r#"MIDI audio"#,
};
pub const PUBLIC_DOWNLOADABLE_SOUND: UTType = UTType {
    identifier: "public.downloadable-sound",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["dls"], "public.mime-type": ["audio/dls"]}"#,
    description: r#"downloadable sound"#,
};
pub const COM_APPLE_COREAUDIO_FORMAT: UTType = UTType {
    identifier: "com.apple.coreaudio-format",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["caf"]}"#,
    description: r#"Apple CoreAudio format"#,
};
pub const PUBLIC_AC3_AUDIO: UTType = UTType {
    identifier: "public.ac3-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["ac3"], "public.mime-type": ["audio/ac3"]}"#,
    description: r#"AC-3 audio"#,
};
pub const PUBLIC_ENHANCED_AC3_AUDIO: UTType = UTType {
    identifier: "public.enhanced-ac3-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["eac3", "ec3"], "public.mime-type": ["audio/eac3"]}"#,
    description: r#"Enhanced AC-3 audio"#,
};
pub const ORG_3GPP_ADAPTIVE_MULTI_RATE_AUDIO: UTType = UTType {
    identifier: "org.3gpp.adaptive-multi-rate-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["amr"], "public.mime-type": ["audio/amr"]}"#,
    description: r#"Adaptive Multi-rate audio"#,
};
pub const PUBLIC_AAC_AUDIO: UTType = UTType {
    identifier: "public.aac-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["aac", "adts"], "public.mime-type": ["audio/aac", "audio/x-aac"]}"#,
    description: r#"AAC audio"#,
};
pub const COM_AUDIBLE_AA_AUDIO: UTType = UTType {
    identifier: "com.audible.aa-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{}"#,
    description: r#"Audible.com Audio"#,
};
pub const COM_AUDIBLE_AA_AUDIOBOOK: UTType = UTType {
    identifier: "com.audible.aa-audiobook",
    conforms_to: r#"["com.audible.aa-audio"]"#,
    tags: r#"{"public.filename-extension": ["aa"], "public.mime-type": ["audio/audible", "audio/x-pn-audibleaudio", "audio/x-audible"]}"#,
    description: r#"Audible.com Audiobook"#,
};
pub const COM_AUDIBLE_AAX_AUDIOBOOK: UTType = UTType {
    identifier: "com.audible.aax-audiobook",
    conforms_to: r#"["com.audible.aa-audio"]"#,
    tags: r#"{"public.filename-extension": ["aax"], "public.mime-type": ["audio/vnd.audible.aax"]}"#,
    description: r#"Audible.com Audiobook"#,
};
pub const COM_SONY_WAVE64: UTType = UTType {
    identifier: "com.sony.wave64",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["w64"]}"#,
    description: r#"Wave64 Audio"#,
};
pub const PUBLIC_FONT: UTType = UTType {
    identifier: "public.font",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"font"#,
};
pub const PUBLIC_TRUETYPE_FONT: UTType = UTType {
    identifier: "public.truetype-font",
    conforms_to: r#"["public.font"]"#,
    tags: r#"{}"#,
    description: r#"TrueType font"#,
};
pub const COM_ADOBE_POSTSCRIPT_FONT: UTType = UTType {
    identifier: "com.adobe.postscript-font",
    conforms_to: r#"["public.font"]"#,
    tags: r#"{}"#,
    description: r#"PostScript font"#,
};
pub const COM_APPLE_TRUETYPE_DATAFORK_SUITCASE_FONT: UTType = UTType {
    identifier: "com.apple.truetype-datafork-suitcase-font",
    conforms_to: r#"["public.truetype-font"]"#,
    tags: r#"{"public.filename-extension": ["dfont"]}"#,
    description: r#"TrueType datafork font"#,
};
pub const PUBLIC_OPENTYPE_FONT: UTType = UTType {
    identifier: "public.opentype-font",
    conforms_to: r#"["public.font"]"#,
    tags: r#"{"public.filename-extension": ["otf"], "public.mime-type": ["font/otf"]}"#,
    description: r#"PostScript OpenType font"#,
};
pub const PUBLIC_OPENTYPE_COLLECTION_FONT: UTType = UTType {
    identifier: "public.opentype-collection-font",
    conforms_to: r#"["public.opentype-font"]"#,
    tags: r#"{"public.filename-extension": ["otc"]}"#,
    description: r#"PostScript OpenType collection font"#,
};
pub const PUBLIC_TRUETYPE_TTF_FONT: UTType = UTType {
    identifier: "public.truetype-ttf-font",
    conforms_to: r#"["public.truetype-font"]"#,
    tags: r#"{"public.filename-extension": ["ttf"], "public.mime-type": ["font/ttf"]}"#,
    description: r#"TrueType OpenType font"#,
};
pub const PUBLIC_TRUETYPE_COLLECTION_FONT: UTType = UTType {
    identifier: "public.truetype-collection-font",
    conforms_to: r#"["public.truetype-font"]"#,
    tags: r#"{"public.filename-extension": ["ttc"]}"#,
    description: r#"TrueType collection font"#,
};
pub const COM_APPLE_FONT_SUITCASE: UTType = UTType {
    identifier: "com.apple.font-suitcase",
    conforms_to: r#"["public.font"]"#,
    tags: r#"{"public.filename-extension": ["suit"]}"#,
    description: r#"font suitcase"#,
};
pub const COM_ADOBE_POSTSCRIPT_LWFN_FONT: UTType = UTType {
    identifier: "com.adobe.postscript-lwfn-font",
    conforms_to: r#"["com.adobe.postscript-font"]"#,
    tags: r#"{}"#,
    description: r#"PostScript Type 1 outline font"#,
};
pub const COM_ADOBE_POSTSCRIPT_PFB_FONT: UTType = UTType {
    identifier: "com.adobe.postscript-pfb-font",
    conforms_to: r#"["com.adobe.postscript-font"]"#,
    tags: r#"{"public.filename-extension": ["pfb"]}"#,
    description: r#"PostScript Type 1 outline font"#,
};
pub const COM_ADOBE_POSTSCRIPT_PFA_FONT: UTType = UTType {
    identifier: "com.adobe.postscript-pfa-font",
    conforms_to: r#"["com.adobe.postscript-font"]"#,
    tags: r#"{"public.filename-extension": ["pfa"]}"#,
    description: r#"PostScript Type 1 outline font"#,
};
pub const COM_APPLE_PROFILE_FONT_ICON: UTType = UTType {
    identifier: "com.apple.profile-font-icon",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: r#"Profile Font"#,
};
pub const COM_APPLE_APPLESCRIPT_TEXT: UTType = UTType {
    identifier: "com.apple.applescript.text",
    conforms_to: r#"["public.script"]"#,
    tags: r#"{"public.filename-extension": ["applescript"]}"#,
    description: r#"AppleScript text"#,
};
pub const COM_APPLE_APPLESCRIPT_SCRIPT: UTType = UTType {
    identifier: "com.apple.applescript.script",
    conforms_to: r#"["public.data", "public.script"]"#,
    tags: r#"{"public.filename-extension": ["scpt"]}"#,
    description: r#"AppleScript"#,
};
pub const COM_APPLE_APPLESCRIPT_SCRIPT_BUNDLE: UTType = UTType {
    identifier: "com.apple.applescript.script-bundle",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["scptd"]}"#,
    description: r#"Script bundle"#,
};
pub const COM_APPLE_SCRIPTING_DEFINITION: UTType = UTType {
    identifier: "com.apple.scripting-definition",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{"public.filename-extension": ["sdef"]}"#,
    description: r#"Scripting Definition"#,
};
pub const PUBLIC_FOLDER: UTType = UTType {
    identifier: "public.folder",
    conforms_to: r#"["public.directory"]"#,
    tags: r#"{}"#,
    description: r#"folder"#,
};
pub const COM_APPLE_DROP_FOLDER: UTType = UTType {
    identifier: "com.apple.drop-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_APPLICATIONS_FOLDER: UTType = UTType {
    identifier: "com.apple.applications-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_SERVER_APPLICATIONS_FOLDER: UTType = UTType {
    identifier: "com.apple.server-applications-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_LIBRARY_FOLDER: UTType = UTType {
    identifier: "com.apple.library-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_DOCUMENT_TYPE_SYSTEM_FOLDER: UTType = UTType {
    identifier: "com.apple.document-type.system-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"System Folder"#,
};
pub const COM_APPLE_TRASH_EMPTY: UTType = UTType {
    identifier: "com.apple.trash-empty",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Trash"#,
};
pub const COM_APPLE_TRASH_FULL: UTType = UTType {
    identifier: "com.apple.trash-full",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Full Trash"#,
};
pub const COM_APPLE_SITES_FOLDER: UTType = UTType {
    identifier: "com.apple.sites-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Sites Folder"#,
};
pub const COM_APPLE_GROUPS_FOLDER: UTType = UTType {
    identifier: "com.apple.groups-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Groups Folder"#,
};
pub const COM_APPLE_SERVERS_FOLDER: UTType = UTType {
    identifier: "com.apple.servers-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_DESKTOP_FOLDER: UTType = UTType {
    identifier: "com.apple.desktop-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Desktop Folder"#,
};
pub const COM_APPLE_DOCUMENTS_FOLDER: UTType = UTType {
    identifier: "com.apple.documents-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Documents Folder"#,
};
pub const COM_APPLE_DOWNLOADS_FOLDER: UTType = UTType {
    identifier: "com.apple.downloads-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Downloads Folder"#,
};
pub const COM_APPLE_MOVIE_FOLDER: UTType = UTType {
    identifier: "com.apple.movie-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Movie Folder"#,
};
pub const COM_APPLE_MUSIC_FOLDER: UTType = UTType {
    identifier: "com.apple.music-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Music Folder"#,
};
pub const COM_APPLE_PICTURES_FOLDER: UTType = UTType {
    identifier: "com.apple.pictures-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Pictures Folder"#,
};
pub const COM_APPLE_PUBLIC_FOLDER: UTType = UTType {
    identifier: "com.apple.public-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Public Folder"#,
};
pub const COM_APPLE_HOME_FOLDER: UTType = UTType {
    identifier: "com.apple.home-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Home Folder"#,
};
pub const COM_APPLE_DEVELOPER_FOLDER: UTType = UTType {
    identifier: "com.apple.developer-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Developer Folder"#,
};
pub const COM_APPLE_USERS_FOLDER: UTType = UTType {
    identifier: "com.apple.users-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Users Folder"#,
};
pub const COM_APPLE_UTILITIES_FOLDER: UTType = UTType {
    identifier: "com.apple.utilities-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"Utilities Folder"#,
};
pub const PUBLIC_VOLUME: UTType = UTType {
    identifier: "public.volume",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: r#"volume"#,
};
pub const PUBLIC_FILE_SHAREPOINT: UTType = UTType {
    identifier: "public.file-sharepoint",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"sharepoint"#,
};
pub const COM_APPLE_NETWORK_NEIGHBORHOOD: UTType = UTType {
    identifier: "com.apple.network-neighborhood",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"neighborhood"#,
};
pub const COM_APPLE_DOT_MAC: UTType = UTType {
    identifier: "com.apple.dot-mac",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#".Mac"#,
};
pub const COM_APPLE_IDISK: UTType = UTType {
    identifier: "com.apple.idisk",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"iDisk"#,
};
pub const COM_APPLE_USER_IDISK: UTType = UTType {
    identifier: "com.apple.user-idisk",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"User iDisk"#,
};
pub const COM_APPLE_PACKAGE: UTType = UTType {
    identifier: "com.apple.package",
    conforms_to: r#"["public.directory"]"#,
    tags: r#"{}"#,
    description: r#"Package"#,
};
pub const COM_APPLE_BUNDLE: UTType = UTType {
    identifier: "com.apple.bundle",
    conforms_to: r#"["public.directory"]"#,
    tags: r#"{}"#,
    description: r#"bundle"#,
};
pub const COM_APPLE_GENERIC_BUNDLE: UTType = UTType {
    identifier: "com.apple.generic-bundle",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["bundle"]}"#,
    description: r#"bundle"#,
};
pub const COM_APPLE_SYSTEMPREFERENCE_PREFPANE: UTType = UTType {
    identifier: "com.apple.systempreference.prefpane",
    conforms_to: r#"["com.apple.package", "com.apple.bundle"]"#,
    tags: r#"{"public.filename-extension": ["prefPane"]}"#,
    description: r#"System Preference pane"#,
};
pub const COM_APPLE_SYSTEMPREFERENCE_SCREEN_SAVER: UTType = UTType {
    identifier: "com.apple.systempreference.screen-saver",
    conforms_to: r#"["com.apple.package", "com.apple.bundle"]"#,
    tags: r#"{"public.filename-extension": ["saver"]}"#,
    description: r#"Screen Saver"#,
};
pub const COM_APPLE_SYSTEMPREFERENCE_SCREEN_SLIDE_SAVER: UTType = UTType {
    identifier: "com.apple.systempreference.screen-slide-saver",
    conforms_to: r#"["com.apple.package", "com.apple.bundle"]"#,
    tags: r#"{"public.filename-extension": ["slideSaver"]}"#,
    description: r#"Screen Slide Saver"#,
};
pub const COM_APPLE_MENU_EXTRA: UTType = UTType {
    identifier: "com.apple.menu-extra",
    conforms_to: r#"["com.apple.package", "com.apple.bundle"]"#,
    tags: r#"{"public.filename-extension": ["menu"]}"#,
    description: r#"System Menu Extra"#,
};
pub const COM_APPLE_LOCALIZABLE_NAME_BUNDLE: UTType = UTType {
    identifier: "com.apple.localizable-name-bundle",
    conforms_to: r#"["com.apple.bundle"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_FRAMEWORK: UTType = UTType {
    identifier: "com.apple.framework",
    conforms_to: r#"["com.apple.bundle"]"#,
    tags: r#"{"public.filename-extension": ["framework"]}"#,
    description: r#"framework"#,
};
pub const COM_APPLE_APPLICATION_BUNDLE: UTType = UTType {
    identifier: "com.apple.application-bundle",
    conforms_to: r#"["com.apple.application", "com.apple.localizable-name-bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["app"]}"#,
    description: r#""#,
};
pub const COM_APPLE_APPLICATION_FILE: UTType = UTType {
    identifier: "com.apple.application-file",
    conforms_to: r#"["com.apple.application", "public.data"]"#,
    tags: r#"{"public.filename-extension": ["app"]}"#,
    description: r#""#,
};
pub const COM_APPLE_DEPRECATED_APPLICATION_FILE: UTType = UTType {
    identifier: "com.apple.deprecated-application-file",
    conforms_to: r#"["com.apple.application-file"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_FLAT_RTFD: UTType = UTType {
    identifier: "com.apple.flat-rtfd",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{}"#,
    description: r#"rich text with attachments (RTFD) data"#,
};
pub const COM_APPLE_INSTALLER_PACKAGE: UTType = UTType {
    identifier: "com.apple.installer-package",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["pkg"]}"#,
    description: r#"Installer package"#,
};
pub const COM_APPLE_INSTALLER_DISTRIBUTION_PACKAGE: UTType = UTType {
    identifier: "com.apple.installer-distribution-package",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["dist", "distz"]}"#,
    description: r#"Installer distribution"#,
};
pub const COM_APPLE_INSTALLER_META_PACKAGE: UTType = UTType {
    identifier: "com.apple.installer-meta-package",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["mpkg"]}"#,
    description: r#"Installer package"#,
};
pub const COM_APPLE_INTELLIGENTSUGGESTIONS_ASSETS: UTType = UTType {
    identifier: "com.apple.intelligentsuggestions.assets",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["suggestionsassets"]}"#,
    description: r#"Intelligent Suggestions assets"#,
};
pub const COM_APPLE_RTFD: UTType = UTType {
    identifier: "com.apple.rtfd",
    conforms_to: r#"["com.apple.package", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["rtfd"]}"#,
    description: r#"rich text with attachments (RTFD)"#,
};
pub const COM_APPLE_APPLICATION_PLACEHOLDER: UTType = UTType {
    identifier: "com.apple.application-placeholder",
    conforms_to: r#"["com.apple.application-bundle"]"#,
    tags: r#"{"public.filename-extension": ["placeholder"]}"#,
    description: r#""#,
};
pub const COM_APPLE_SERVICE_APPLICATION: UTType = UTType {
    identifier: "com.apple.service-application",
    conforms_to: r#"["com.apple.application-bundle"]"#,
    tags: r#"{"public.filename-extension": ["service"]}"#,
    description: r#""#,
};
pub const COM_APPLE_PLUGIN: UTType = UTType {
    identifier: "com.apple.plugin",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["plugin"]}"#,
    description: r#"plug-in"#,
};
pub const COM_APPLE_XPC_SERVICE: UTType = UTType {
    identifier: "com.apple.xpc-service",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["xpc"]}"#,
    description: r#"XPC Service"#,
};
pub const COM_APPLE_KERNEL_EXTENSION: UTType = UTType {
    identifier: "com.apple.kernel-extension",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["kext"]}"#,
    description: r#"Kernel Extension"#,
};
pub const COM_APPLE_APPLICATION_AND_SYSTEM_EXTENSION: UTType = UTType {
    identifier: "com.apple.application-and-system-extension",
    conforms_to: r#"["com.apple.xpc-service"]"#,
    tags: r#"{"public.filename-extension": ["appex"]}"#,
    description: r#"Application and System Extension"#,
};
pub const COM_APPLE_PLUGINKIT: UTType = UTType {
    identifier: "com.apple.pluginkit",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["pluginkit"]}"#,
    description: r#"PlugInKit plug-in"#,
};
pub const COM_APPLE_WEBKIT_PLUGIN: UTType = UTType {
    identifier: "com.apple.webkit-plugin",
    conforms_to: r#"["com.apple.plugin"]"#,
    tags: r#"{"public.filename-extension": ["webplugin"]}"#,
    description: r#"WebKit plug-in"#,
};
pub const COM_APPLE_METADATA_IMPORTER: UTType = UTType {
    identifier: "com.apple.metadata-importer",
    conforms_to: r#"["com.apple.plugin"]"#,
    tags: r#"{"public.filename-extension": ["mdimporter"]}"#,
    description: r#"Spotlight importer"#,
};
pub const COM_APPLE_QUICKLOOK_GENERATOR: UTType = UTType {
    identifier: "com.apple.quicklook-generator",
    conforms_to: r#"["com.apple.plugin"]"#,
    tags: r#"{"public.filename-extension": ["qlgenerator"]}"#,
    description: r#"QuickLook preview generator"#,
};
pub const COM_APPLE_DASHBOARD_WIDGET: UTType = UTType {
    identifier: "com.apple.dashboard-widget",
    conforms_to: r#"["com.apple.localizable-name-bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["wdgt"]}"#,
    description: r#"Dashboard widget"#,
};
pub const COM_APPLE_DRIVER_EXTENSION: UTType = UTType {
    identifier: "com.apple.driver-extension",
    conforms_to: r#"["com.apple.localizable-name-bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["dext"]}"#,
    description: r#"Driver Extension"#,
};
pub const COM_APPLE_SYSTEM_EXTENSION: UTType = UTType {
    identifier: "com.apple.system-extension",
    conforms_to: r#"["com.apple.localizable-name-bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["systemextension"]}"#,
    description: r#"System Extension"#,
};
pub const COM_APPLE_PPP_PLUG_IN: UTType = UTType {
    identifier: "com.apple.ppp-plug-in",
    conforms_to: r#"["com.apple.plugin"]"#,
    tags: r#"{"public.filename-extension": ["ppp"]}"#,
    description: r#"PPP Plug-in"#,
};
pub const COM_APPLE_FILE_SYSTEM_PLUG_IN: UTType = UTType {
    identifier: "com.apple.file-system-plug-in",
    conforms_to: r#"["com.apple.plugin"]"#,
    tags: r#"{"public.filename-extension": ["fs"]}"#,
    description: r#"File System Plug-in"#,
};
pub const COM_APPLE_DATA_CONTAINER: UTType = UTType {
    identifier: "com.apple.data-container",
    conforms_to: r#"["public.folder", "com.apple.localizable-name-bundle"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const ORG_OPENXMLFORMATS_OPENXML: UTType = UTType {
    identifier: "org.openxmlformats.openxml",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"Office Open XML"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT: UTType = UTType {
    identifier: "org.oasis-open.opendocument",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"Open Document"#,
};
pub const COM_RSA_PKCS_12: UTType = UTType {
    identifier: "com.rsa.pkcs-12",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["p12", "pfx"], "public.mime-type": ["application/x-pkcs12"]}"#,
    description: r#"personal information exchange (PKCS#12)"#,
};
pub const PUBLIC_X509_CERTIFICATE: UTType = UTType {
    identifier: "public.x509-certificate",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["cer", "der", "crt", "pem"], "public.mime-type": ["application/x-x509-ca-cert"]}"#,
    description: r#"certificate (X.509)"#,
};
pub const COM_APPLE_ALERT: UTType = UTType {
    identifier: "com.apple.alert",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Alert"#,
};
pub const COM_APPLE_ALERT_NOTE: UTType = UTType {
    identifier: "com.apple.alert-note",
    conforms_to: r#"["com.apple.alert"]"#,
    tags: r#"{}"#,
    description: r#"Alert Note"#,
};
pub const COM_APPLE_ALERT_CAUTION: UTType = UTType {
    identifier: "com.apple.alert-caution",
    conforms_to: r#"["com.apple.alert"]"#,
    tags: r#"{}"#,
    description: r#"Alert Caution"#,
};
pub const COM_APPLE_ALERT_STOP: UTType = UTType {
    identifier: "com.apple.alert-stop",
    conforms_to: r#"["com.apple.alert"]"#,
    tags: r#"{}"#,
    description: r#"Alert Stop"#,
};
pub const COM_APPLE_WEBARCHIVE: UTType = UTType {
    identifier: "com.apple.webarchive",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["webarchive"], "public.mime-type": ["application/x-webarchive"]}"#,
    description: r#"web archive"#,
};
pub const ORG_IDPF_EPUB_CONTAINER: UTType = UTType {
    identifier: "org.idpf.epub-container",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["epub"], "public.mime-type": ["application/epub+zip"]}"#,
    description: r#"EPUB publication"#,
};
pub const COM_APPLE_LOCALIZED_PDF_BUNDLE: UTType = UTType {
    identifier: "com.apple.localized-pdf-bundle",
    conforms_to: r#"["com.apple.localizable-name-bundle", "com.apple.package", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["lpdf"]}"#,
    description: r#"localized PDF"#,
};
pub const ORG_AAFASSOCIATION_ADVANCED_AUTHORING_FORMAT: UTType = UTType {
    identifier: "org.aafassociation.advanced-authoring-format",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["aaf"]}"#,
    description: r#"Advanced Authoring Format"#,
};
pub const COM_APPLE_TXN_TEXT_MULTIMEDIA_DATA: UTType = UTType {
    identifier: "com.apple.txn.text-multimedia-data",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{}"#,
    description: r#"text with multimedia"#,
};
pub const COM_APPLE_COLORSYNC_PROFILE: UTType = UTType {
    identifier: "com.apple.colorsync-profile",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["icc", "icm", "pf"]}"#,
    description: r#"ColorSync profile"#,
};
pub const COM_APPLE_PROFILE_BACKGROUND_COLOR: UTType = UTType {
    identifier: "com.apple.profile-background-color",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"Profile Background Color"#,
};
pub const COM_APPLE_PROFILE_FONT_AND_COLOR: UTType = UTType {
    identifier: "com.apple.profile-font-and-color",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"Profile Font And Color"#,
};
pub const COM_APPLE_COLOR_FILE: UTType = UTType {
    identifier: "com.apple.color-file",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["clr", "ccl"]}"#,
    description: r#"Color File"#,
};
pub const COM_APPLE_INK_INKTEXT: UTType = UTType {
    identifier: "com.apple.ink.inktext",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"writing"#,
};
pub const COM_APPLE_MOBILECONFIG: UTType = UTType {
    identifier: "com.apple.mobileconfig",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{"public.filename-extension": ["mobileconfig", "mobile"], "public.mime-type": ["application/x-apple-aspen-config"]}"#,
    description: r#"mobile configuration"#,
};
pub const COM_APPLE_PROVISIONPROFILE: UTType = UTType {
    identifier: "com.apple.provisionprofile",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{"public.filename-extension": ["provisionprofile"]}"#,
    description: r#"provision profile"#,
};
pub const COM_APPLE_CONFIGPROFILE: UTType = UTType {
    identifier: "com.apple.configprofile",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{"public.filename-extension": ["configprofile"]}"#,
    description: r#"configuration profile"#,
};
pub const COM_APPLE_USER: UTType = UTType {
    identifier: "com.apple.user",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"User"#,
};
pub const COM_APPLE_GUEST_USER: UTType = UTType {
    identifier: "com.apple.guest-user",
    conforms_to: r#"["com.apple.user"]"#,
    tags: r#"{}"#,
    description: r#"Guest User"#,
};
pub const COM_APPLE_HELP_DOCUMENT: UTType = UTType {
    identifier: "com.apple.help-document",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: r#"Help Document"#,
};
pub const COM_APPLE_USER_GROUP: UTType = UTType {
    identifier: "com.apple.user-group",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"User Group"#,
};
pub const COM_APPLE_USER_UNKNOWN: UTType = UTType {
    identifier: "com.apple.user-unknown",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"User Unknown"#,
};
pub const COM_APPLE_AIRDROP: UTType = UTType {
    identifier: "com.apple.airdrop",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"AirDrop"#,
};
pub const COM_APPLE_BONJOUR: UTType = UTType {
    identifier: "com.apple.bonjour",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Bonjour"#,
};
pub const COM_APPLE_NOTIFICATIONS: UTType = UTType {
    identifier: "com.apple.notifications",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Notifications"#,
};
pub const COM_APPLE_MOBILEPROVISION: UTType = UTType {
    identifier: "com.apple.mobileprovision",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["mobileprovision"], "public.mime-type": ["application/x-apple-aspen-mobileprovision"]}"#,
    description: r#"mobile provision"#,
};
pub const COM_APPLE_PKPASS: UTType = UTType {
    identifier: "com.apple.pkpass",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["pkpass"], "public.mime-type": ["application/vnd.apple.pkpass"]}"#,
    description: r#"Pass"#,
};
pub const COM_APPLE_PKPASS_DATA: UTType = UTType {
    identifier: "com.apple.pkpass-data",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["pkpass"]}"#,
    description: r#"Pass"#,
};
pub const COM_APPLE_PKPASSES_DATA: UTType = UTType {
    identifier: "com.apple.pkpasses-data",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["pkpasses"], "public.mime-type": ["application/vnd.apple.pkpasses"]}"#,
    description: r#"Passes"#,
};
pub const COM_APPLE_WATCHFACE: UTType = UTType {
    identifier: "com.apple.watchface",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["watchface"]}"#,
    description: r#"Watchface"#,
};
pub const PUBLIC_DEVICE: UTType = UTType {
    identifier: "public.device",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Device"#,
};
pub const COM_APPLE_VIRTUAL_MACHINE: UTType = UTType {
    identifier: "com.apple.virtual-machine",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: r#"Virtual Machine"#,
};
pub const PUBLIC_DISPLAY: UTType = UTType {
    identifier: "public.display",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Display"#,
};
pub const PUBLIC_SPEAKER: UTType = UTType {
    identifier: "public.speaker",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Speaker"#,
};
pub const PUBLIC_COMPUTER: UTType = UTType {
    identifier: "public.computer",
    conforms_to: r#"["public.device"]"#,
    tags: r#"{}"#,
    description: r#"Computer"#,
};
pub const PUBLIC_GENERIC_PC: UTType = UTType {
    identifier: "public.generic-pc",
    conforms_to: r#"["public.computer"]"#,
    tags: r#"{}"#,
    description: r#"PC"#,
};
pub const COM_APPLE_DEVICE: UTType = UTType {
    identifier: "com.apple.device",
    conforms_to: r#"["public.device"]"#,
    tags: r#"{}"#,
    description: r#"Apple device"#,
};
pub const COM_APPLE_MAC: UTType = UTType {
    identifier: "com.apple.mac",
    conforms_to: r#"["public.computer", "com.apple.device"]"#,
    tags: r#"{}"#,
    description: r#"Mac"#,
};
pub const COM_APPLE_MAC_LAPTOP: UTType = UTType {
    identifier: "com.apple.mac.laptop",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: r#"Laptop"#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-silver",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-space-gray",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-2017",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-silver-2017",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc-silver"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-space-gray-2017",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc-space-gray"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-2017",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver-2017",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-silver"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray-2017",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-space-gray"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-2018",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver-2018",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-silver", "com.apple.macbookpro-13-retina-touchid-2018"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray-2018",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-space-gray", "com.apple.macbookpro-13-retina-touchid-2018"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_2019: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-2019",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER_2019: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-silver-2019",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc-silver", "com.apple.macbookpro-13-retina-usbc-2019"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY_2019: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-space-gray-2019",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc-space-gray", "com.apple.macbookpro-13-retina-usbc-2019"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-silver",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-space-gray",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-2017",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-silver-2017",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid-silver"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-space-gray-2017",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid-space-gray"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-2018",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-silver-2018",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid-silver", "com.apple.macbookpro-15-retina-touchid-2018"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-space-gray-2018",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid-space-gray", "com.apple.macbookpro-15-retina-touchid-2018"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_15_LATE_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-late-2018",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_15_SILVER_LATE_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-silver-late-2018",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid-silver", "com.apple.macbookpro-15-late-2018"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_15_SPACE_GRAY_LATE_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-space-gray-late-2018",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid-space-gray", "com.apple.macbookpro-15-late-2018"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MAC_TOWER: UTType = UTType {
    identifier: "com.apple.mac.tower",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: r#"Tower"#,
};
pub const COM_APPLE_MAC_RACKMOUNT: UTType = UTType {
    identifier: "com.apple.mac.rackmount",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: r#"Rack Mount"#,
};
pub const COM_APPLE_POWERBOOK: UTType = UTType {
    identifier: "com.apple.powerbook",
    conforms_to: r#"["com.apple.mac.laptop"]"#,
    tags: r#"{}"#,
    description: r#"PowerBook"#,
};
pub const COM_APPLE_POWERBOOK_G4_TITANIUM: UTType = UTType {
    identifier: "com.apple.powerbook-g4-titanium",
    conforms_to: r#"["com.apple.powerbook"]"#,
    tags: r#"{}"#,
    description: r#"PowerBook G4"#,
};
pub const COM_APPLE_POWERBOOK_G4_12: UTType = UTType {
    identifier: "com.apple.powerbook-g4-12",
    conforms_to: r#"["com.apple.powerbook"]"#,
    tags: r#"{}"#,
    description: r#"PowerBook G4"#,
};
pub const COM_APPLE_POWERBOOK_G4_15: UTType = UTType {
    identifier: "com.apple.powerbook-g4-15",
    conforms_to: r#"["com.apple.powerbook"]"#,
    tags: r#"{}"#,
    description: r#"PowerBook G4"#,
};
pub const COM_APPLE_POWERBOOK_G4_17: UTType = UTType {
    identifier: "com.apple.powerbook-g4-17",
    conforms_to: r#"["com.apple.powerbook"]"#,
    tags: r#"{}"#,
    description: r#"PowerBook G4"#,
};
pub const COM_APPLE_IBOOK_G4: UTType = UTType {
    identifier: "com.apple.ibook-g4",
    conforms_to: r#"["com.apple.mac.laptop"]"#,
    tags: r#"{}"#,
    description: r#"iBook G4"#,
};
pub const COM_APPLE_POWERMAC: UTType = UTType {
    identifier: "com.apple.powermac",
    conforms_to: r#"["com.apple.mac.tower"]"#,
    tags: r#"{}"#,
    description: r#"Power Mac"#,
};
pub const COM_APPLE_POWERMAC_G4_QUICKSILVER: UTType = UTType {
    identifier: "com.apple.powermac-g4-quicksilver",
    conforms_to: r#"["com.apple.powermac"]"#,
    tags: r#"{}"#,
    description: r#"Power Mac G4"#,
};
pub const COM_APPLE_POWERMAC_G4_MIRRORED_DRIVE_DOORS: UTType = UTType {
    identifier: "com.apple.powermac-g4-mirrored-drive-doors",
    conforms_to: r#"["com.apple.powermac"]"#,
    tags: r#"{}"#,
    description: r#"Power Mac G4"#,
};
pub const COM_APPLE_POWERMAC_G5: UTType = UTType {
    identifier: "com.apple.powermac-g5",
    conforms_to: r#"["com.apple.powermac"]"#,
    tags: r#"{}"#,
    description: r#"Power Mac G5"#,
};
pub const COM_APPLE_XSERVE: UTType = UTType {
    identifier: "com.apple.xserve",
    conforms_to: r#"["com.apple.mac.rackmount"]"#,
    tags: r#"{}"#,
    description: r#"Xserve"#,
};
pub const COM_APPLE_XSERVE_G4: UTType = UTType {
    identifier: "com.apple.xserve-g4",
    conforms_to: r#"["com.apple.xserve"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_XSERVE_G5: UTType = UTType {
    identifier: "com.apple.xserve-g5",
    conforms_to: r#"["com.apple.xserve"]"#,
    tags: r#"{}"#,
    description: r#"Xserve G5"#,
};
pub const COM_APPLE_XSERVE_XEON: UTType = UTType {
    identifier: "com.apple.xserve-xeon",
    conforms_to: r#"["com.apple.xserve"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACMINI: UTType = UTType {
    identifier: "com.apple.macmini",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: r#"Mac mini"#,
};
pub const COM_APPLE_MACMINI_G4: UTType = UTType {
    identifier: "com.apple.macmini-g4",
    conforms_to: r#"["com.apple.macmini"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACMINI_CORE_DUO: UTType = UTType {
    identifier: "com.apple.macmini-core-duo",
    conforms_to: r#"["com.apple.macmini"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACMINI_UNIBODY: UTType = UTType {
    identifier: "com.apple.macmini-unibody",
    conforms_to: r#"["com.apple.macmini"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACMINI_UNIBODY_NO_OPTICAL: UTType = UTType {
    identifier: "com.apple.macmini-unibody-no-optical",
    conforms_to: r#"["com.apple.macmini"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACMINI_2018: UTType = UTType {
    identifier: "com.apple.macmini-2018",
    conforms_to: r#"["com.apple.macmini"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_EMAC: UTType = UTType {
    identifier: "com.apple.emac",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: r#"eMac"#,
};
pub const COM_APPLE_IMAC: UTType = UTType {
    identifier: "com.apple.imac",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: r#"iMac"#,
};
pub const COM_APPLE_IMAC_G4_15: UTType = UTType {
    identifier: "com.apple.imac-g4-15",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_G4_17: UTType = UTType {
    identifier: "com.apple.imac-g4-17",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_G4_20: UTType = UTType {
    identifier: "com.apple.imac-g4-20",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_G5: UTType = UTType {
    identifier: "com.apple.imac-g5",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#"iMac G5"#,
};
pub const COM_APPLE_IMAC_G5_ISIGHT: UTType = UTType {
    identifier: "com.apple.imac-g5-isight",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_CORE_DUO: UTType = UTType {
    identifier: "com.apple.imac-core-duo",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_CORE_2_DUO: UTType = UTType {
    identifier: "com.apple.imac-core-2-duo",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_ISIGHT_24: UTType = UTType {
    identifier: "com.apple.imac-iSight-24",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_ALUMINUM_20: UTType = UTType {
    identifier: "com.apple.imac-aluminum-20",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_ALUMINUM_24: UTType = UTType {
    identifier: "com.apple.imac-aluminum-24",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_UNIBODY_21: UTType = UTType {
    identifier: "com.apple.imac-unibody-21",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_UNIBODY: UTType = UTType {
    identifier: "com.apple.imac-unibody",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL: UTType = UTType {
    identifier: "com.apple.imac-unibody-21-no-optical",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_15_1: UTType = UTType {
    identifier: "com.apple.imac-15-1",
    conforms_to: r#"["com.apple.imac-unibody-27-no-optical"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_MID_2015: UTType = UTType {
    identifier: "com.apple.imac-unibody-21-no-optical.mid-2015",
    conforms_to: r#"["com.apple.imac-unibody-21-no-optical"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_LATE_2015: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical-late-2015",
    conforms_to: r#"["com.apple.imac-unibody-27-no-optical"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_2017: UTType = UTType {
    identifier: "com.apple.imac-unibody-21-no-optical-2017",
    conforms_to: r#"["com.apple.imac-unibody-21-no-optical"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2017: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical-2017",
    conforms_to: r#"["com.apple.imac-unibody-27-no-optical"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_2019: UTType = UTType {
    identifier: "com.apple.imac-unibody-21-no-optical-2019",
    conforms_to: r#"["com.apple.imac-unibody-21-no-optical"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2019: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical-2019",
    conforms_to: r#"["com.apple.imac-unibody-27-no-optical"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2020: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical-2020",
    conforms_to: r#"["com.apple.imac-unibody-27-no-optical"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMACPRO_2017: UTType = UTType {
    identifier: "com.apple.imacpro-2017",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#"iMac Pro"#,
};
pub const COM_APPLE_MACBOOK: UTType = UTType {
    identifier: "com.apple.macbook",
    conforms_to: r#"["com.apple.mac.laptop"]"#,
    tags: r#"{}"#,
    description: r#"MacBook"#,
};
pub const COM_APPLE_MACBOOK_WHITE: UTType = UTType {
    identifier: "com.apple.macbook-white",
    conforms_to: r#"["com.apple.macbook"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOK_BLACK: UTType = UTType {
    identifier: "com.apple.macbook-black",
    conforms_to: r#"["com.apple.macbook"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOK_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbook-unibody",
    conforms_to: r#"["com.apple.macbook"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOK_UNIBODY_PLASTIC: UTType = UTType {
    identifier: "com.apple.macbook-unibody-plastic",
    conforms_to: r#"["com.apple.macbook"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOK_RETINA: UTType = UTType {
    identifier: "com.apple.macbook-retina",
    conforms_to: r#"["com.apple.macbook"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOK_RETINA_SILVER: UTType = UTType {
    identifier: "com.apple.macbook-retina-silver",
    conforms_to: r#"["com.apple.macbook-retina"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOK_RETINA_GOLD: UTType = UTType {
    identifier: "com.apple.macbook-retina-gold",
    conforms_to: r#"["com.apple.macbook-retina"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOK_RETINA_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbook-retina-space-gray",
    conforms_to: r#"["com.apple.macbook-retina"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOK_RETINA_ROSE_GOLD: UTType = UTType {
    identifier: "com.apple.macbook-retina-rose-gold",
    conforms_to: r#"["com.apple.macbook-retina"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOK_RETINA_SILVER_2017: UTType = UTType {
    identifier: "com.apple.macbook-retina-silver-2017",
    conforms_to: r#"["com.apple.macbook-retina-silver"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOK_RETINA_GOLD_2017: UTType = UTType {
    identifier: "com.apple.macbook-retina-gold-2017",
    conforms_to: r#"["com.apple.macbook-retina-gold"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOK_RETINA_SPACE_GRAY_2017: UTType = UTType {
    identifier: "com.apple.macbook-retina-space-gray-2017",
    conforms_to: r#"["com.apple.macbook-retina-space-gray"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOK_RETINA_ROSE_GOLD_2017: UTType = UTType {
    identifier: "com.apple.macbook-retina-rose-gold-2017",
    conforms_to: r#"["com.apple.macbook-retina-rose-gold"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOK_RETINA_GOLD_2018: UTType = UTType {
    identifier: "com.apple.macbook-retina-gold-2018",
    conforms_to: r#"["com.apple.macbook-retina"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookpro-13-unibody",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_DISPLAY: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-display",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO: UTType = UTType {
    identifier: "com.apple.macbookpro",
    conforms_to: r#"["com.apple.mac.laptop"]"#,
    tags: r#"{}"#,
    description: r#"MacBook Pro"#,
};
pub const COM_APPLE_MACBOOKPRO_15: UTType = UTType {
    identifier: "com.apple.macbookpro-15",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_15_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookpro-15-unibody",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_DISPLAY: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-display",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_17_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookpro-17-unibody",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_17: UTType = UTType {
    identifier: "com.apple.macbookpro-17",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_16: UTType = UTType {
    identifier: "com.apple.macbookpro-16",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_16_SILVER: UTType = UTType {
    identifier: "com.apple.macbookpro-16-silver",
    conforms_to: r#"["com.apple.macbookpro-16"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_16_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookpro-16-space-gray",
    conforms_to: r#"["com.apple.macbookpro-16"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-four-usb-ports-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_SILVER_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-four-usb-ports-silver-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-silver", "com.apple.macbookpro-13-retina-four-usb-ports-2020"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_SPACE_GRAY_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-four-usb-ports-space-gray-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-space-gray", "com.apple.macbookpro-13-retina-four-usb-ports-2020"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-silver", "com.apple.macbookpro-13-retina-touchid-2020"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-space-gray", "com.apple.macbookpro-13-retina-touchid-2020"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_16_MID_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-16-mid-2020",
    conforms_to: r#"["com.apple.macbookpro-16"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_16_SILVER_MID_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-16-silver-mid-2020",
    conforms_to: r#"["com.apple.macbookpro-16-silver", "com.apple.macbookpro-16-mid-2020"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_16_SPACE_GRAY_MID_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-16-space-gray-mid-2020",
    conforms_to: r#"["com.apple.macbookpro-16-space-gray", "com.apple.macbookpro-16-mid-2020"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR: UTType = UTType {
    identifier: "com.apple.macbookair",
    conforms_to: r#"["com.apple.mac.laptop"]"#,
    tags: r#"{}"#,
    description: r#"MacBook Air"#,
};
pub const COM_APPLE_MACBOOKAIR_11_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookair-11-unibody",
    conforms_to: r#"["com.apple.macbookair"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_13_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookair-13-unibody",
    conforms_to: r#"["com.apple.macbookair"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_2018: UTType = UTType {
    identifier: "com.apple.macbookair-2018",
    conforms_to: r#"["com.apple.macbookair"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_2018_SILVER: UTType = UTType {
    identifier: "com.apple.macbookair-2018-silver",
    conforms_to: r#"["com.apple.macbookair-2018"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_2018_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookair-2018-space-gray",
    conforms_to: r#"["com.apple.macbookair-2018"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_2018_GOLD: UTType = UTType {
    identifier: "com.apple.macbookair-2018-gold",
    conforms_to: r#"["com.apple.macbookair-2018"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_2019: UTType = UTType {
    identifier: "com.apple.macbookair-2019",
    conforms_to: r#"["com.apple.macbookair-2018"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_2019_SILVER: UTType = UTType {
    identifier: "com.apple.macbookair-2019-silver",
    conforms_to: r#"["com.apple.macbookair-2018-silver"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_2019_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookair-2019-space-gray",
    conforms_to: r#"["com.apple.macbookair-2018-space-gray"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_2019_GOLD: UTType = UTType {
    identifier: "com.apple.macbookair-2019-gold",
    conforms_to: r#"["com.apple.macbookair-2018-gold"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_2020: UTType = UTType {
    identifier: "com.apple.macbookair-2020",
    conforms_to: r#"["com.apple.macbookair-2019"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_2020_SILVER: UTType = UTType {
    identifier: "com.apple.macbookair-2020-silver",
    conforms_to: r#"["com.apple.macbookair-2019-silver"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_2020_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookair-2020-space-gray",
    conforms_to: r#"["com.apple.macbookair-2019-space-gray"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_2020_GOLD: UTType = UTType {
    identifier: "com.apple.macbookair-2020-gold",
    conforms_to: r#"["com.apple.macbookair-2019-gold"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACPRO: UTType = UTType {
    identifier: "com.apple.macpro",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: r#"Mac Pro"#,
};
pub const COM_APPLE_MACPRO_FIREWIRE: UTType = UTType {
    identifier: "com.apple.macpro-firewire",
    conforms_to: r#"["com.apple.macpro", "com.apple.mac.tower"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACPRO_CYLINDER: UTType = UTType {
    identifier: "com.apple.macpro-cylinder",
    conforms_to: r#"["com.apple.macpro", "com.apple.mac.tower"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACPRO_2019: UTType = UTType {
    identifier: "com.apple.macpro-2019",
    conforms_to: r#"["com.apple.macpro", "com.apple.mac.tower"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACPRO_2019_RACKMOUNT: UTType = UTType {
    identifier: "com.apple.macpro-2019-rackmount",
    conforms_to: r#"["com.apple.macpro", "com.apple.mac.rackmount"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACMINI_2020: UTType = UTType {
    identifier: "com.apple.macmini-2020",
    conforms_to: r#"["com.apple.macmini"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_LATE_2020: UTType = UTType {
    identifier: "com.apple.macbookair-late-2020",
    conforms_to: r#"["com.apple.macbookair-2020"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_LATE_2020_SILVER: UTType = UTType {
    identifier: "com.apple.macbookair-late-2020-silver",
    conforms_to: r#"["com.apple.macbookair-2020-silver", "com.apple.macbookair-late-2020"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_LATE_2020_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookair-late-2020-space-gray",
    conforms_to: r#"["com.apple.macbookair-2020-space-gray", "com.apple.macbookair-late-2020"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKAIR_LATE_2020_GOLD: UTType = UTType {
    identifier: "com.apple.macbookair-late-2020-gold",
    conforms_to: r#"["com.apple.macbookair-2020-gold", "com.apple.macbookair-late-2020"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_LATE_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-late-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-2020"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_LATE_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver-late-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-silver-2020", "com.apple.macbookpro-13-retina-touchid-late-2020"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_LATE_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray-late-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-space-gray-2020", "com.apple.macbookpro-13-retina-touchid-late-2020"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_2021: UTType = UTType {
    identifier: "com.apple.imac-2021",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_2021_SILVER: UTType = UTType {
    identifier: "com.apple.imac-2021-silver",
    conforms_to: r#"["com.apple.imac-2021"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_2021_YELLOW: UTType = UTType {
    identifier: "com.apple.imac-2021-yellow",
    conforms_to: r#"["com.apple.imac-2021"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_2021_GREEN: UTType = UTType {
    identifier: "com.apple.imac-2021-green",
    conforms_to: r#"["com.apple.imac-2021"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_2021_BLUE: UTType = UTType {
    identifier: "com.apple.imac-2021-blue",
    conforms_to: r#"["com.apple.imac-2021"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_2021_RED: UTType = UTType {
    identifier: "com.apple.imac-2021-red",
    conforms_to: r#"["com.apple.imac-2021"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_2021_PURPLE: UTType = UTType {
    identifier: "com.apple.imac-2021-purple",
    conforms_to: r#"["com.apple.imac-2021"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IMAC_2021_ORANGE: UTType = UTType {
    identifier: "com.apple.imac-2021-orange",
    conforms_to: r#"["com.apple.imac-2021"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_DEVELOPER_TRANSITION_KIT_2005: UTType = UTType {
    identifier: "com.apple.developer-transition-kit-2005",
    conforms_to: r#"["com.apple.mac.tower"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_DEVELOPER_TRANSITION_KIT_2020: UTType = UTType {
    identifier: "com.apple.developer-transition-kit-2020",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_IOS_DEVICE: UTType = UTType {
    identifier: "com.apple.ios-device",
    conforms_to: r#"["com.apple.device"]"#,
    tags: r#"{}"#,
    description: r#"iOS device"#,
};
pub const COM_APPLE_APPLE_TV: UTType = UTType {
    identifier: "com.apple.apple-tv",
    conforms_to: r#"["com.apple.ios-device"]"#,
    tags: r#"{}"#,
    description: r#"Apple TV"#,
};
pub const COM_APPLE_HOMEPOD: UTType = UTType {
    identifier: "com.apple.homepod",
    conforms_to: r#"["com.apple.ios-device", "public.speaker"]"#,
    tags: r#"{}"#,
    description: r#"HomePod"#,
};
pub const COM_APPLE_IOS_SIMULATOR: UTType = UTType {
    identifier: "com.apple.ios-simulator",
    conforms_to: r#"["com.apple.ios-device"]"#,
    tags: r#"{}"#,
    description: r#"iOS Simulator"#,
};
pub const COM_APPLE_IPHONE: UTType = UTType {
    identifier: "com.apple.iphone",
    conforms_to: r#"["com.apple.ios-device"]"#,
    tags: r#"{}"#,
    description: r#"iPhone"#,
};
pub const COM_APPLE_IPHONE_3G: UTType = UTType {
    identifier: "com.apple.iphone-3g",
    conforms_to: r#"["com.apple.iphone"]"#,
    tags: r#"{}"#,
    description: r#"iPhone 3G"#,
};
pub const COM_APPLE_IPHONE_3GS: UTType = UTType {
    identifier: "com.apple.iphone-3gs",
    conforms_to: r#"["com.apple.iphone"]"#,
    tags: r#"{}"#,
    description: r#"iPhone 3GS"#,
};
pub const COM_APPLE_IPHONE_4: UTType = UTType {
    identifier: "com.apple.iphone-4",
    conforms_to: r#"["com.apple.iphone"]"#,
    tags: r#"{}"#,
    description: r#"iPhone 4"#,
};
pub const COM_APPLE_IPHONE_8: UTType = UTType {
    identifier: "com.apple.iphone-8",
    conforms_to: r#"["com.apple.iphone"]"#,
    tags: r#"{}"#,
    description: r#"iPhone 8"#,
};
pub const COM_APPLE_IPHONE_8_2: UTType = UTType {
    identifier: "com.apple.iphone-8-2",
    conforms_to: r#"["com.apple.iphone-8"]"#,
    tags: r#"{}"#,
    description: r#"iPhone 8 (Model A1863, A1905, A1906, A1907)"#,
};
pub const COM_APPLE_IPHONE_8_7: UTType = UTType {
    identifier: "com.apple.iphone-8-7",
    conforms_to: r#"["com.apple.iphone-8"]"#,
    tags: r#"{}"#,
    description: r#"iPhone 8 (Model A1863, A1905, A1906, A1907)"#,
};
pub const COM_APPLE_IPHONE_8_8: UTType = UTType {
    identifier: "com.apple.iphone-8-8",
    conforms_to: r#"["com.apple.iphone-8"]"#,
    tags: r#"{}"#,
    description: r#"iPhone 8 (Model A1863, A1905, A1906, A1907)"#,
};
pub const COM_APPLE_IPHONE_8_PLUS: UTType = UTType {
    identifier: "com.apple.iphone-8-plus",
    conforms_to: r#"["com.apple.iphone"]"#,
    tags: r#"{}"#,
    description: r#"iPhone 8 Plus"#,
};
pub const COM_APPLE_IPHONE_8_PLUS_2: UTType = UTType {
    identifier: "com.apple.iphone-8-plus-2",
    conforms_to: r#"["com.apple.iphone-8-plus"]"#,
    tags: r#"{}"#,
    description: r#"iPhone 8 Plus (Model A1864, A1897, A1898, A1899)"#,
};
pub const COM_APPLE_IPHONE_8_PLUS_3: UTType = UTType {
    identifier: "com.apple.iphone-8-plus-3",
    conforms_to: r#"["com.apple.iphone-8-plus"]"#,
    tags: r#"{}"#,
    description: r#"iPhone 8 Plus (Model A1864, A1897, A1898, A1899)"#,
};
pub const COM_APPLE_IPHONE_8_PLUS_1: UTType = UTType {
    identifier: "com.apple.iphone-8-plus-1",
    conforms_to: r#"["com.apple.iphone-8-plus"]"#,
    tags: r#"{}"#,
    description: r#"iPhone 8 Plus (Model A1864, A1897, A1898, A1899)"#,
};
pub const COM_APPLE_IPHONE_X: UTType = UTType {
    identifier: "com.apple.iphone-x",
    conforms_to: r#"["com.apple.homebuttonless-iphone"]"#,
    tags: r#"{}"#,
    description: r#"iPhone X"#,
};
pub const COM_APPLE_IPHONE_X_1: UTType = UTType {
    identifier: "com.apple.iphone-x-1",
    conforms_to: r#"["com.apple.iphone-x"]"#,
    tags: r#"{}"#,
    description: r#"iPhone X (Model A1865, A1901, A1902, A1903)"#,
};
pub const COM_APPLE_IPHONE_X_2: UTType = UTType {
    identifier: "com.apple.iphone-x-2",
    conforms_to: r#"["com.apple.iphone-x"]"#,
    tags: r#"{}"#,
    description: r#"iPhone X (Model A1865, A1901, A1902, A1903)"#,
};
pub const COM_APPLE_LEGACY_IPOD: UTType = UTType {
    identifier: "com.apple.legacy-ipod",
    conforms_to: r#"["com.apple.device"]"#,
    tags: r#"{}"#,
    description: r#"iPod"#,
};
pub const COM_APPLE_IPOD_SHUFFLE: UTType = UTType {
    identifier: "com.apple.ipod-shuffle",
    conforms_to: r#"["com.apple.legacy-ipod"]"#,
    tags: r#"{}"#,
    description: r#"iPod Shuffle"#,
};
pub const COM_APPLE_IPOD_SHUFFLE_GEN1: UTType = UTType {
    identifier: "com.apple.ipod-shuffle-gen1",
    conforms_to: r#"["com.apple.ipod-shuffle"]"#,
    tags: r#"{}"#,
    description: r#"iPod Shuffle"#,
};
pub const COM_APPLE_IPOD_SHUFFLE_GEN2: UTType = UTType {
    identifier: "com.apple.ipod-shuffle-gen2",
    conforms_to: r#"["com.apple.ipod-shuffle"]"#,
    tags: r#"{}"#,
    description: r#"iPod Shuffle (2th generation)"#,
};
pub const COM_APPLE_IPOD_SHUFFLE_GEN3: UTType = UTType {
    identifier: "com.apple.ipod-shuffle-gen3",
    conforms_to: r#"["com.apple.ipod-shuffle"]"#,
    tags: r#"{}"#,
    description: r#"iPod Shuffle (3th generation)"#,
};
pub const COM_APPLE_IPOD_SHUFFLE_GEN4: UTType = UTType {
    identifier: "com.apple.ipod-shuffle-gen4",
    conforms_to: r#"["com.apple.ipod-shuffle"]"#,
    tags: r#"{}"#,
    description: r#"iPod Shuffle (4th generation)"#,
};
pub const COM_APPLE_IPOD_NANO: UTType = UTType {
    identifier: "com.apple.ipod-nano",
    conforms_to: r#"["com.apple.legacy-ipod"]"#,
    tags: r#"{}"#,
    description: r#"iPod Nano"#,
};
pub const COM_APPLE_IPOD_MINI: UTType = UTType {
    identifier: "com.apple.ipod-mini",
    conforms_to: r#"["com.apple.legacy-ipod"]"#,
    tags: r#"{}"#,
    description: r#"iPod Nano"#,
};
pub const COM_APPLE_IPOD_CLASSIC: UTType = UTType {
    identifier: "com.apple.ipod-classic",
    conforms_to: r#"["com.apple.legacy-ipod"]"#,
    tags: r#"{}"#,
    description: r#"iPod Classic"#,
};
pub const COM_APPLE_IPOD: UTType = UTType {
    identifier: "com.apple.ipod",
    conforms_to: r#"["com.apple.ios-device"]"#,
    tags: r#"{}"#,
    description: r#"iPod"#,
};
pub const COM_APPLE_IPOD_TOUCH: UTType = UTType {
    identifier: "com.apple.ipod-touch",
    conforms_to: r#"["com.apple.ipod"]"#,
    tags: r#"{}"#,
    description: r#"iPod touch"#,
};
pub const COM_APPLE_IPOD_TOUCH_2: UTType = UTType {
    identifier: "com.apple.ipod-touch-2",
    conforms_to: r#"["com.apple.ipod"]"#,
    tags: r#"{}"#,
    description: r#"iPod touch"#,
};
pub const COM_APPLE_IPOD_TOUCH_3: UTType = UTType {
    identifier: "com.apple.ipod-touch-3",
    conforms_to: r#"["com.apple.ipod"]"#,
    tags: r#"{}"#,
    description: r#"iPod touch"#,
};
pub const COM_APPLE_IPOD_TOUCH_4: UTType = UTType {
    identifier: "com.apple.ipod-touch-4",
    conforms_to: r#"["com.apple.ipod"]"#,
    tags: r#"{}"#,
    description: r#"iPod touch"#,
};
pub const COM_APPLE_IPAD: UTType = UTType {
    identifier: "com.apple.ipad",
    conforms_to: r#"["com.apple.ios-device"]"#,
    tags: r#"{}"#,
    description: r#"iPad"#,
};
pub const COM_APPLE_HOMEBUTTONLESS_IPAD: UTType = UTType {
    identifier: "com.apple.homebuttonless-ipad",
    conforms_to: r#"["com.apple.ipad", "com.apple.homebuttonless-device"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_HOMEBUTTONLESS_IPHONE: UTType = UTType {
    identifier: "com.apple.homebuttonless-iphone",
    conforms_to: r#"["com.apple.iphone", "com.apple.homebuttonless-device"]"#,
    tags: r#"{}"#,
    description: r#""#,
};
pub const COM_APPLE_WATCH: UTType = UTType {
    identifier: "com.apple.watch",
    conforms_to: r#"["com.apple.ios-device"]"#,
    tags: r#"{}"#,
    description: r#"Apple Watch"#,
};
pub const COM_APPLE_AIRPORT_EXPRESS: UTType = UTType {
    identifier: "com.apple.airport-express",
    conforms_to: r#"["com.apple.device"]"#,
    tags: r#"{}"#,
    description: r#"AirPort Express"#,
};
pub const COM_APPLE_AIRPORT: UTType = UTType {
    identifier: "com.apple.airport",
    conforms_to: r#"["com.apple.device"]"#,
    tags: r#"{}"#,
    description: r#"AirPort Extreme"#,
};
pub const COM_APPLE_TIME_CAPSULE: UTType = UTType {
    identifier: "com.apple.time-capsule",
    conforms_to: r#"["com.apple.device"]"#,
    tags: r#"{}"#,
    description: r#"Time Capsule"#,
};
pub const COM_APPLE_AIRPORT_EXTREME_TOWER: UTType = UTType {
    identifier: "com.apple.airport-extreme-tower",
    conforms_to: r#"["com.apple.airport"]"#,
    tags: r#"{}"#,
    description: r#"AirPort Extreme"#,
};
pub const COM_APPLE_AIRPORT_TIME_CAPSULE_TOWER: UTType = UTType {
    identifier: "com.apple.airport-time-capsule-tower",
    conforms_to: r#"["com.apple.time-capsule"]"#,
    tags: r#"{}"#,
    description: r#"Time Capsule"#,
};
pub const COM_APPLE_CINEMA_DISPLAY: UTType = UTType {
    identifier: "com.apple.cinema-display",
    conforms_to: r#"["public.display"]"#,
    tags: r#"{}"#,
    description: r#"Cinema Display"#,
};
pub const COM_APPLE_LED_CINEMA_DISPLAY_24: UTType = UTType {
    identifier: "com.apple.led-cinema-display-24",
    conforms_to: r#"["public.display"]"#,
    tags: r#"{}"#,
    description: r#"LED Cinema Display"#,
};
pub const COM_APPLE_LED_CINEMA_DISPLAY_27: UTType = UTType {
    identifier: "com.apple.led-cinema-display-27",
    conforms_to: r#"["public.display"]"#,
    tags: r#"{}"#,
    description: r#"LED Cinema Display"#,
};
pub const COM_APPLE_PRO_DISPLAY_XDR: UTType = UTType {
    identifier: "com.apple.pro-display-xdr",
    conforms_to: r#"["public.display"]"#,
    tags: r#"{}"#,
    description: r#"Pro Display XDR"#,
};
pub const PUBLIC_STORAGE: UTType = UTType {
    identifier: "public.storage",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Storage"#,
};
pub const COM_APPLE_STORAGE_EXTERNAL: UTType = UTType {
    identifier: "com.apple.storage-external",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: r#"External Disk"#,
};
pub const COM_APPLE_GENERIC_TIME_MACHINE_DISK: UTType = UTType {
    identifier: "com.apple.generic-time-machine-disk",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: r#"Generic Time Machine Disk"#,
};
pub const COM_APPLE_STORAGE_NETBOOT: UTType = UTType {
    identifier: "com.apple.storage-netboot",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: r#"NetBootVolume"#,
};
pub const COM_APPLE_FILE_SERVER: UTType = UTType {
    identifier: "com.apple.file-server",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: r#"File Server"#,
};
pub const COM_APPLE_STORAGE_INTERNAL: UTType = UTType {
    identifier: "com.apple.storage-internal",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: r#"Internal Disk"#,
};
pub const COM_APPLE_STORAGE_REMOVABLE: UTType = UTType {
    identifier: "com.apple.storage-removable",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: r#"Removable Disk"#,
};
pub const COM_APPLE_FILE_VAULT: UTType = UTType {
    identifier: "com.apple.file-vault",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"File Vault"#,
};
pub const COM_APPLE_GENERIC_AIRDISK: UTType = UTType {
    identifier: "com.apple.generic-airdisk",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: r#"Generic AirDisk"#,
};
pub const PUBLIC_OPTICAL_STORAGE_MEDIA: UTType = UTType {
    identifier: "public.optical-storage-media",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: r#"optical storage media"#,
};
pub const PUBLIC_CD_BASED_MEDIA: UTType = UTType {
    identifier: "public.cd-based-media",
    conforms_to: r#"["public.optical-storage-media"]"#,
    tags: r#"{}"#,
    description: r#"CD"#,
};
pub const PUBLIC_CD_MEDIA: UTType = UTType {
    identifier: "public.cd-media",
    conforms_to: r#"["public.cd-based-media"]"#,
    tags: r#"{}"#,
    description: r#"CD"#,
};
pub const PUBLIC_CD_R_MEDIA: UTType = UTType {
    identifier: "public.cd-r-media",
    conforms_to: r#"["public.cd-based-media"]"#,
    tags: r#"{}"#,
    description: r#"CD-R"#,
};
pub const PUBLIC_CD_RW_MEDIA: UTType = UTType {
    identifier: "public.cd-rw-media",
    conforms_to: r#"["public.cd-based-media"]"#,
    tags: r#"{}"#,
    description: r#"CD-RW"#,
};
pub const PUBLIC_DVD_BASED_MEDIA: UTType = UTType {
    identifier: "public.dvd-based-media",
    conforms_to: r#"["public.optical-storage-media"]"#,
    tags: r#"{}"#,
    description: r#"DVD"#,
};
pub const PUBLIC_DVD_MEDIA: UTType = UTType {
    identifier: "public.dvd-media",
    conforms_to: r#"["public.dvd-based-media"]"#,
    tags: r#"{}"#,
    description: r#"DVD"#,
};
pub const PUBLIC_DVD_R_MEDIA: UTType = UTType {
    identifier: "public.dvd-r-media",
    conforms_to: r#"["public.dvd-based-media"]"#,
    tags: r#"{}"#,
    description: r#"DVD-R"#,
};
pub const PUBLIC_DVD_RW_MEDIA: UTType = UTType {
    identifier: "public.dvd-rw-media",
    conforms_to: r#"["public.dvd-based-media"]"#,
    tags: r#"{}"#,
    description: r#"DVD-RW"#,
};
pub const PUBLIC_DVD_RAM_MEDIA: UTType = UTType {
    identifier: "public.dvd-ram-media",
    conforms_to: r#"["public.dvd-based-media"]"#,
    tags: r#"{}"#,
    description: r#"DVD-RAM"#,
};
pub const PUBLIC_DVD_PLUS_R_MEDIA: UTType = UTType {
    identifier: "public.dvd-plus-r-media",
    conforms_to: r#"["public.dvd-based-media"]"#,
    tags: r#"{}"#,
    description: r#"DVD+R"#,
};
pub const PUBLIC_DVD_PLUS_RW_MEDIA: UTType = UTType {
    identifier: "public.dvd-plus-rw-media",
    conforms_to: r#"["public.dvd-based-media"]"#,
    tags: r#"{}"#,
    description: r#"DVD+RW"#,
};
pub const PUBLIC_HD_DVD_BASED_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-based-media",
    conforms_to: r#"["public.optical-storage-media"]"#,
    tags: r#"{}"#,
    description: r#"HD DVD"#,
};
pub const PUBLIC_HD_DVD_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-media",
    conforms_to: r#"["public.hd-dvd-based-media"]"#,
    tags: r#"{}"#,
    description: r#"HD DVD"#,
};
pub const PUBLIC_HD_DVD_R_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-r-media",
    conforms_to: r#"["public.hd-dvd-based-media"]"#,
    tags: r#"{}"#,
    description: r#"HD DVD-R"#,
};
pub const PUBLIC_HD_DVD_RW_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-rw-media",
    conforms_to: r#"["public.hd-dvd-based-media"]"#,
    tags: r#"{}"#,
    description: r#"HD DVD-RW"#,
};
pub const PUBLIC_HD_DVD_RAM_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-ram-media",
    conforms_to: r#"["public.hd-dvd-based-media"]"#,
    tags: r#"{}"#,
    description: r#"HD DVD-RAM"#,
};
pub const PUBLIC_APP_CATEGORY: UTType = UTType {
    identifier: "public.app-category",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Application"#,
};
pub const PUBLIC_APP_CATEGORY_BUSINESS: UTType = UTType {
    identifier: "public.app-category.business",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Business"#,
};
pub const PUBLIC_APP_CATEGORY_DEVELOPER_TOOLS: UTType = UTType {
    identifier: "public.app-category.developer-tools",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Developer Tools"#,
};
pub const PUBLIC_APP_CATEGORY_EDUCATION: UTType = UTType {
    identifier: "public.app-category.education",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Education"#,
};
pub const PUBLIC_APP_CATEGORY_ENTERTAINMENT: UTType = UTType {
    identifier: "public.app-category.entertainment",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Entertainment"#,
};
pub const PUBLIC_APP_CATEGORY_FINANCE: UTType = UTType {
    identifier: "public.app-category.finance",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Finance"#,
};
pub const PUBLIC_APP_CATEGORY_GAMES: UTType = UTType {
    identifier: "public.app-category.games",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Games"#,
};
pub const PUBLIC_APP_CATEGORY_ACTION_GAMES: UTType = UTType {
    identifier: "public.app-category.action-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Action Games"#,
};
pub const PUBLIC_APP_CATEGORY_ADVENTURE_GAMES: UTType = UTType {
    identifier: "public.app-category.adventure-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Adventure Games"#,
};
pub const PUBLIC_APP_CATEGORY_ARCADE_GAMES: UTType = UTType {
    identifier: "public.app-category.arcade-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Arcade Games"#,
};
pub const PUBLIC_APP_CATEGORY_BOARD_GAMES: UTType = UTType {
    identifier: "public.app-category.board-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Board Games"#,
};
pub const PUBLIC_APP_CATEGORY_CARD_GAMES: UTType = UTType {
    identifier: "public.app-category.card-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Card Games"#,
};
pub const PUBLIC_APP_CATEGORY_CASINO_GAMES: UTType = UTType {
    identifier: "public.app-category.casino-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Casino Games"#,
};
pub const PUBLIC_APP_CATEGORY_DICE_GAMES: UTType = UTType {
    identifier: "public.app-category.dice-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Dice Games"#,
};
pub const PUBLIC_APP_CATEGORY_EDUCATIONAL_GAMES: UTType = UTType {
    identifier: "public.app-category.educational-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Educational Games"#,
};
pub const PUBLIC_APP_CATEGORY_FAMILY_GAMES: UTType = UTType {
    identifier: "public.app-category.family-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Family Games"#,
};
pub const PUBLIC_APP_CATEGORY_KIDS_GAMES: UTType = UTType {
    identifier: "public.app-category.kids-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Kids Games"#,
};
pub const PUBLIC_APP_CATEGORY_MUSIC_GAMES: UTType = UTType {
    identifier: "public.app-category.music-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Music Games"#,
};
pub const PUBLIC_APP_CATEGORY_PUZZLE_GAMES: UTType = UTType {
    identifier: "public.app-category.puzzle-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Puzzle Games"#,
};
pub const PUBLIC_APP_CATEGORY_RACING_GAMES: UTType = UTType {
    identifier: "public.app-category.racing-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Racing Games"#,
};
pub const PUBLIC_APP_CATEGORY_ROLE_PLAYING_GAMES: UTType = UTType {
    identifier: "public.app-category.role-playing-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Role Playing Games"#,
};
pub const PUBLIC_APP_CATEGORY_SIMULATION_GAMES: UTType = UTType {
    identifier: "public.app-category.simulation-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Simulation Games"#,
};
pub const PUBLIC_APP_CATEGORY_SPORTS_GAMES: UTType = UTType {
    identifier: "public.app-category.sports-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Sports Games"#,
};
pub const PUBLIC_APP_CATEGORY_STRATEGY_GAMES: UTType = UTType {
    identifier: "public.app-category.strategy-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Strategy Games"#,
};
pub const PUBLIC_APP_CATEGORY_TRIVIA_GAMES: UTType = UTType {
    identifier: "public.app-category.trivia-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Trivia Games"#,
};
pub const PUBLIC_APP_CATEGORY_WORD_GAMES: UTType = UTType {
    identifier: "public.app-category.word-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: r#"Word Games"#,
};
pub const PUBLIC_APP_CATEGORY_GRAPHICS_DESIGN: UTType = UTType {
    identifier: "public.app-category.graphics-design",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Graphics n Design"#,
};
pub const PUBLIC_APP_CATEGORY_HEALTHCARE_FITNESS: UTType = UTType {
    identifier: "public.app-category.healthcare-fitness",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Healthcare n Fitness"#,
};
pub const PUBLIC_APP_CATEGORY_LIFESTYLE: UTType = UTType {
    identifier: "public.app-category.lifestyle",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Lifestyle"#,
};
pub const PUBLIC_APP_CATEGORY_MEDICAL: UTType = UTType {
    identifier: "public.app-category.medical",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Medical"#,
};
pub const PUBLIC_APP_CATEGORY_MUSIC: UTType = UTType {
    identifier: "public.app-category.music",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Music"#,
};
pub const PUBLIC_APP_CATEGORY_NEWS: UTType = UTType {
    identifier: "public.app-category.news",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"News"#,
};
pub const PUBLIC_APP_CATEGORY_PHOTOGRAPHY: UTType = UTType {
    identifier: "public.app-category.photography",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Photography"#,
};
pub const PUBLIC_APP_CATEGORY_PRODUCTIVITY: UTType = UTType {
    identifier: "public.app-category.productivity",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Productivity"#,
};
pub const PUBLIC_APP_CATEGORY_REFERENCE: UTType = UTType {
    identifier: "public.app-category.reference",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Reference"#,
};
pub const PUBLIC_APP_CATEGORY_SOCIAL_NETWORKING: UTType = UTType {
    identifier: "public.app-category.social-networking",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Social Networking"#,
};
pub const PUBLIC_APP_CATEGORY_SPORTS: UTType = UTType {
    identifier: "public.app-category.sports",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Sports"#,
};
pub const PUBLIC_APP_CATEGORY_TRAVEL: UTType = UTType {
    identifier: "public.app-category.travel",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Travel"#,
};
pub const PUBLIC_APP_CATEGORY_UTILITIES: UTType = UTType {
    identifier: "public.app-category.utilities",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Utilities"#,
};
pub const PUBLIC_APP_CATEGORY_VIDEO: UTType = UTType {
    identifier: "public.app-category.video",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Video"#,
};
pub const PUBLIC_APP_CATEGORY_WEATHER: UTType = UTType {
    identifier: "public.app-category.weather",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Weather"#,
};
pub const PUBLIC_APP_CATEGORY_BOOKMARKS: UTType = UTType {
    identifier: "public.app-category.bookmarks",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Bookmarks"#,
};
pub const PUBLIC_APP_CATEGORY_BOOKS: UTType = UTType {
    identifier: "public.app-category.books",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Books"#,
};
pub const PUBLIC_APP_CATEGORY_NAVIGATION: UTType = UTType {
    identifier: "public.app-category.navigation",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Navigation"#,
};
pub const PUBLIC_APP_CATEGORY_PHOTOGRAPHY_AND_VIDEO: UTType = UTType {
    identifier: "public.app-category.photography-and-video",
    conforms_to: r#"["public.app-category.photography", "public.app-category.video"]"#,
    tags: r#"{}"#,
    description: r#"Photo & Video"#,
};
pub const PUBLIC_APP_CATEGORY_FOOD_AND_DRINK: UTType = UTType {
    identifier: "public.app-category.food-and-drink",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Food & Drink"#,
};
pub const PUBLIC_APP_CATEGORY_SHOPPING: UTType = UTType {
    identifier: "public.app-category.shopping",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Shopping"#,
};
pub const PUBLIC_APP_CATEGORY_MAGAZINES_AND_NEWSPAPERS: UTType = UTType {
    identifier: "public.app-category.magazines-and-newspapers",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: r#"Magazines & Newspapers"#,
};
pub const COM_APPLE_STRUCTURED_TEXT: UTType = UTType {
    identifier: "com.apple.structured-text",
    conforms_to: r#"["public.plain-text"]"#,
    tags: r#"{}"#,
    description: r#"Structured Text"#,
};
pub const COM_APPLE_STRUCTURED_TEXT_DATE: UTType = UTType {
    identifier: "com.apple.structured-text.date",
    conforms_to: r#"["com.apple.structured-text"]"#,
    tags: r#"{}"#,
    description: r#"Date (Structured Text)"#,
};
pub const COM_APPLE_STRUCTURED_TEXT_ADDRESS: UTType = UTType {
    identifier: "com.apple.structured-text.address",
    conforms_to: r#"["com.apple.structured-text"]"#,
    tags: r#"{}"#,
    description: r#"Address (Structured Text)"#,
};
pub const COM_APPLE_STRUCTURED_TEXT_TELEPHONE_NUMBER: UTType = UTType {
    identifier: "com.apple.structured-text.telephone-number",
    conforms_to: r#"["com.apple.structured-text"]"#,
    tags: r#"{}"#,
    description: r#"Telephone Number (Structured Text)"#,
};
pub const COM_APPLE_STRUCTURED_TEXT_TRANSIT_INFORMATION: UTType = UTType {
    identifier: "com.apple.structured-text.transit-information",
    conforms_to: r#"["com.apple.structured-text"]"#,
    tags: r#"{}"#,
    description: r#"Transit Information (Structured Text)"#,
};
pub const COM_APPLE_ACTIVE_WEBPAGE: UTType = UTType {
    identifier: "com.apple.active-webpage",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Active Web Page"#,
};
pub const VND_ANDROID_PACKAGE_ARCHIVE: UTType = UTType {
    identifier: "vnd.android.package-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["apk"], "public.mime-type": ["application/vnd.android.package-archive"]}"#,
    description: r#"Android APK file"#,
};
pub const PUBLIC_RUST_SOURCE_CODE: UTType = UTType {
    identifier: "public.rust-source",
    conforms_to: r#"["public.text", "public.source-code"]"#,
    tags: r#"{
        "public.filename-extension": ["rs"],
        "public.mime-type": ["text/x-rust", "text/rust"]
    }"#,
    description: r#"Rust source code file"#,
};
pub const COM_APPLE_IPHONE_PACKAGE_ARCHIVE: UTType = UTType {
    identifier: "com.apple.iphone.package-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["ipa"], "public.mime-type": ["application/octet-stream"]}"#,
    description: r#"iOS application archive"#,
};
lazy_static! {
    pub(crate) static ref SYSTEM_TYPES_MAP: Arc<RwLock<HashMap<&'static str, UTType<'static>>>> =
        Arc::new(RwLock::new(HashMap::from([
            (
                "com.apple.iphone.package-archive",
                COM_APPLE_IPHONE_PACKAGE_ARCHIVE
            ),
            ("vnd.android.package-archive", VND_ANDROID_PACKAGE_ARCHIVE),
            ("public.rust-source", PUBLIC_RUST_SOURCE_CODE),
            ("com.apple.active-webpage", COM_APPLE_ACTIVE_WEBPAGE),
            (
                "com.apple.structured-text.transit-information",
                COM_APPLE_STRUCTURED_TEXT_TRANSIT_INFORMATION
            ),
            (
                "com.apple.structured-text.telephone-number",
                COM_APPLE_STRUCTURED_TEXT_TELEPHONE_NUMBER
            ),
            (
                "com.apple.structured-text.address",
                COM_APPLE_STRUCTURED_TEXT_ADDRESS
            ),
            (
                "com.apple.structured-text.date",
                COM_APPLE_STRUCTURED_TEXT_DATE
            ),
            ("com.apple.structured-text", COM_APPLE_STRUCTURED_TEXT),
            (
                "public.app-category.magazines-and-newspapers",
                PUBLIC_APP_CATEGORY_MAGAZINES_AND_NEWSPAPERS
            ),
            ("public.app-category.shopping", PUBLIC_APP_CATEGORY_SHOPPING),
            (
                "public.app-category.food-and-drink",
                PUBLIC_APP_CATEGORY_FOOD_AND_DRINK
            ),
            (
                "public.app-category.photography-and-video",
                PUBLIC_APP_CATEGORY_PHOTOGRAPHY_AND_VIDEO
            ),
            (
                "public.app-category.navigation",
                PUBLIC_APP_CATEGORY_NAVIGATION
            ),
            ("public.app-category.books", PUBLIC_APP_CATEGORY_BOOKS),
            (
                "public.app-category.bookmarks",
                PUBLIC_APP_CATEGORY_BOOKMARKS
            ),
            ("public.app-category.weather", PUBLIC_APP_CATEGORY_WEATHER),
            ("public.app-category.video", PUBLIC_APP_CATEGORY_VIDEO),
            (
                "public.app-category.utilities",
                PUBLIC_APP_CATEGORY_UTILITIES
            ),
            ("public.app-category.travel", PUBLIC_APP_CATEGORY_TRAVEL),
            ("public.app-category.sports", PUBLIC_APP_CATEGORY_SPORTS),
            (
                "public.app-category.social-networking",
                PUBLIC_APP_CATEGORY_SOCIAL_NETWORKING
            ),
            (
                "public.app-category.reference",
                PUBLIC_APP_CATEGORY_REFERENCE
            ),
            (
                "public.app-category.productivity",
                PUBLIC_APP_CATEGORY_PRODUCTIVITY
            ),
            (
                "public.app-category.photography",
                PUBLIC_APP_CATEGORY_PHOTOGRAPHY
            ),
            ("public.app-category.news", PUBLIC_APP_CATEGORY_NEWS),
            ("public.app-category.music", PUBLIC_APP_CATEGORY_MUSIC),
            ("public.app-category.medical", PUBLIC_APP_CATEGORY_MEDICAL),
            (
                "public.app-category.lifestyle",
                PUBLIC_APP_CATEGORY_LIFESTYLE
            ),
            (
                "public.app-category.healthcare-fitness",
                PUBLIC_APP_CATEGORY_HEALTHCARE_FITNESS
            ),
            (
                "public.app-category.graphics-design",
                PUBLIC_APP_CATEGORY_GRAPHICS_DESIGN
            ),
            (
                "public.app-category.word-games",
                PUBLIC_APP_CATEGORY_WORD_GAMES
            ),
            (
                "public.app-category.trivia-games",
                PUBLIC_APP_CATEGORY_TRIVIA_GAMES
            ),
            (
                "public.app-category.strategy-games",
                PUBLIC_APP_CATEGORY_STRATEGY_GAMES
            ),
            (
                "public.app-category.sports-games",
                PUBLIC_APP_CATEGORY_SPORTS_GAMES
            ),
            (
                "public.app-category.simulation-games",
                PUBLIC_APP_CATEGORY_SIMULATION_GAMES
            ),
            (
                "public.app-category.role-playing-games",
                PUBLIC_APP_CATEGORY_ROLE_PLAYING_GAMES
            ),
            (
                "public.app-category.racing-games",
                PUBLIC_APP_CATEGORY_RACING_GAMES
            ),
            (
                "public.app-category.puzzle-games",
                PUBLIC_APP_CATEGORY_PUZZLE_GAMES
            ),
            (
                "public.app-category.music-games",
                PUBLIC_APP_CATEGORY_MUSIC_GAMES
            ),
            (
                "public.app-category.kids-games",
                PUBLIC_APP_CATEGORY_KIDS_GAMES
            ),
            (
                "public.app-category.family-games",
                PUBLIC_APP_CATEGORY_FAMILY_GAMES
            ),
            (
                "public.app-category.educational-games",
                PUBLIC_APP_CATEGORY_EDUCATIONAL_GAMES
            ),
            (
                "public.app-category.dice-games",
                PUBLIC_APP_CATEGORY_DICE_GAMES
            ),
            (
                "public.app-category.casino-games",
                PUBLIC_APP_CATEGORY_CASINO_GAMES
            ),
            (
                "public.app-category.card-games",
                PUBLIC_APP_CATEGORY_CARD_GAMES
            ),
            (
                "public.app-category.board-games",
                PUBLIC_APP_CATEGORY_BOARD_GAMES
            ),
            (
                "public.app-category.arcade-games",
                PUBLIC_APP_CATEGORY_ARCADE_GAMES
            ),
            (
                "public.app-category.adventure-games",
                PUBLIC_APP_CATEGORY_ADVENTURE_GAMES
            ),
            (
                "public.app-category.action-games",
                PUBLIC_APP_CATEGORY_ACTION_GAMES
            ),
            ("public.app-category.games", PUBLIC_APP_CATEGORY_GAMES),
            ("public.app-category.finance", PUBLIC_APP_CATEGORY_FINANCE),
            (
                "public.app-category.entertainment",
                PUBLIC_APP_CATEGORY_ENTERTAINMENT
            ),
            (
                "public.app-category.education",
                PUBLIC_APP_CATEGORY_EDUCATION
            ),
            (
                "public.app-category.developer-tools",
                PUBLIC_APP_CATEGORY_DEVELOPER_TOOLS
            ),
            ("public.app-category.business", PUBLIC_APP_CATEGORY_BUSINESS),
            ("public.app-category", PUBLIC_APP_CATEGORY),
            ("public.hd-dvd-ram-media", PUBLIC_HD_DVD_RAM_MEDIA),
            ("public.hd-dvd-rw-media", PUBLIC_HD_DVD_RW_MEDIA),
            ("public.hd-dvd-r-media", PUBLIC_HD_DVD_R_MEDIA),
            ("public.hd-dvd-media", PUBLIC_HD_DVD_MEDIA),
            ("public.hd-dvd-based-media", PUBLIC_HD_DVD_BASED_MEDIA),
            ("public.dvd-plus-rw-media", PUBLIC_DVD_PLUS_RW_MEDIA),
            ("public.dvd-plus-r-media", PUBLIC_DVD_PLUS_R_MEDIA),
            ("public.dvd-ram-media", PUBLIC_DVD_RAM_MEDIA),
            ("public.dvd-rw-media", PUBLIC_DVD_RW_MEDIA),
            ("public.dvd-r-media", PUBLIC_DVD_R_MEDIA),
            ("public.dvd-media", PUBLIC_DVD_MEDIA),
            ("public.dvd-based-media", PUBLIC_DVD_BASED_MEDIA),
            ("public.cd-rw-media", PUBLIC_CD_RW_MEDIA),
            ("public.cd-r-media", PUBLIC_CD_R_MEDIA),
            ("public.cd-media", PUBLIC_CD_MEDIA),
            ("public.cd-based-media", PUBLIC_CD_BASED_MEDIA),
            ("public.optical-storage-media", PUBLIC_OPTICAL_STORAGE_MEDIA),
            ("com.apple.generic-airdisk", COM_APPLE_GENERIC_AIRDISK),
            ("com.apple.file-vault", COM_APPLE_FILE_VAULT),
            ("com.apple.storage-removable", COM_APPLE_STORAGE_REMOVABLE),
            ("com.apple.storage-internal", COM_APPLE_STORAGE_INTERNAL),
            ("com.apple.file-server", COM_APPLE_FILE_SERVER),
            ("com.apple.storage-netboot", COM_APPLE_STORAGE_NETBOOT),
            (
                "com.apple.generic-time-machine-disk",
                COM_APPLE_GENERIC_TIME_MACHINE_DISK
            ),
            ("com.apple.storage-external", COM_APPLE_STORAGE_EXTERNAL),
            ("public.storage", PUBLIC_STORAGE),
            ("com.apple.pro-display-xdr", COM_APPLE_PRO_DISPLAY_XDR),
            (
                "com.apple.led-cinema-display-27",
                COM_APPLE_LED_CINEMA_DISPLAY_27
            ),
            (
                "com.apple.led-cinema-display-24",
                COM_APPLE_LED_CINEMA_DISPLAY_24
            ),
            ("com.apple.cinema-display", COM_APPLE_CINEMA_DISPLAY),
            (
                "com.apple.airport-time-capsule-tower",
                COM_APPLE_AIRPORT_TIME_CAPSULE_TOWER
            ),
            (
                "com.apple.airport-extreme-tower",
                COM_APPLE_AIRPORT_EXTREME_TOWER
            ),
            ("com.apple.time-capsule", COM_APPLE_TIME_CAPSULE),
            ("com.apple.airport", COM_APPLE_AIRPORT),
            ("com.apple.airport-express", COM_APPLE_AIRPORT_EXPRESS),
            ("com.apple.watch", COM_APPLE_WATCH),
            (
                "com.apple.homebuttonless-iphone",
                COM_APPLE_HOMEBUTTONLESS_IPHONE
            ),
            (
                "com.apple.homebuttonless-ipad",
                COM_APPLE_HOMEBUTTONLESS_IPAD
            ),
            ("com.apple.ipad", COM_APPLE_IPAD),
            ("com.apple.ipod-touch-4", COM_APPLE_IPOD_TOUCH_4),
            ("com.apple.ipod-touch-3", COM_APPLE_IPOD_TOUCH_3),
            ("com.apple.ipod-touch-2", COM_APPLE_IPOD_TOUCH_2),
            ("com.apple.ipod-touch", COM_APPLE_IPOD_TOUCH),
            ("com.apple.ipod", COM_APPLE_IPOD),
            ("com.apple.ipod-classic", COM_APPLE_IPOD_CLASSIC),
            ("com.apple.ipod-mini", COM_APPLE_IPOD_MINI),
            ("com.apple.ipod-nano", COM_APPLE_IPOD_NANO),
            ("com.apple.ipod-shuffle-gen4", COM_APPLE_IPOD_SHUFFLE_GEN4),
            ("com.apple.ipod-shuffle-gen3", COM_APPLE_IPOD_SHUFFLE_GEN3),
            ("com.apple.ipod-shuffle-gen2", COM_APPLE_IPOD_SHUFFLE_GEN2),
            ("com.apple.ipod-shuffle-gen1", COM_APPLE_IPOD_SHUFFLE_GEN1),
            ("com.apple.ipod-shuffle", COM_APPLE_IPOD_SHUFFLE),
            ("com.apple.legacy-ipod", COM_APPLE_LEGACY_IPOD),
            ("com.apple.iphone-x-2", COM_APPLE_IPHONE_X_2),
            ("com.apple.iphone-x-1", COM_APPLE_IPHONE_X_1),
            ("com.apple.iphone-x", COM_APPLE_IPHONE_X),
            ("com.apple.iphone-8-plus-1", COM_APPLE_IPHONE_8_PLUS_1),
            ("com.apple.iphone-8-plus-3", COM_APPLE_IPHONE_8_PLUS_3),
            ("com.apple.iphone-8-plus-2", COM_APPLE_IPHONE_8_PLUS_2),
            ("com.apple.iphone-8-plus", COM_APPLE_IPHONE_8_PLUS),
            ("com.apple.iphone-8-8", COM_APPLE_IPHONE_8_8),
            ("com.apple.iphone-8-7", COM_APPLE_IPHONE_8_7),
            ("com.apple.iphone-8-2", COM_APPLE_IPHONE_8_2),
            ("com.apple.iphone-8", COM_APPLE_IPHONE_8),
            ("com.apple.iphone-4", COM_APPLE_IPHONE_4),
            ("com.apple.iphone-3gs", COM_APPLE_IPHONE_3GS),
            ("com.apple.iphone-3g", COM_APPLE_IPHONE_3G),
            ("com.apple.iphone", COM_APPLE_IPHONE),
            ("com.apple.ios-simulator", COM_APPLE_IOS_SIMULATOR),
            ("com.apple.homepod", COM_APPLE_HOMEPOD),
            ("com.apple.apple-tv", COM_APPLE_APPLE_TV),
            ("com.apple.ios-device", COM_APPLE_IOS_DEVICE),
            (
                "com.apple.developer-transition-kit-2020",
                COM_APPLE_DEVELOPER_TRANSITION_KIT_2020
            ),
            (
                "com.apple.developer-transition-kit-2005",
                COM_APPLE_DEVELOPER_TRANSITION_KIT_2005
            ),
            ("com.apple.imac-2021-orange", COM_APPLE_IMAC_2021_ORANGE),
            ("com.apple.imac-2021-purple", COM_APPLE_IMAC_2021_PURPLE),
            ("com.apple.imac-2021-red", COM_APPLE_IMAC_2021_RED),
            ("com.apple.imac-2021-blue", COM_APPLE_IMAC_2021_BLUE),
            ("com.apple.imac-2021-green", COM_APPLE_IMAC_2021_GREEN),
            ("com.apple.imac-2021-yellow", COM_APPLE_IMAC_2021_YELLOW),
            ("com.apple.imac-2021-silver", COM_APPLE_IMAC_2021_SILVER),
            ("com.apple.imac-2021", COM_APPLE_IMAC_2021),
            (
                "com.apple.macbookpro-13-retina-touchid-space-gray-late-2020",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_LATE_2020
            ),
            (
                "com.apple.macbookpro-13-retina-touchid-silver-late-2020",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_LATE_2020
            ),
            (
                "com.apple.macbookpro-13-retina-touchid-late-2020",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_LATE_2020
            ),
            (
                "com.apple.macbookair-late-2020-gold",
                COM_APPLE_MACBOOKAIR_LATE_2020_GOLD
            ),
            (
                "com.apple.macbookair-late-2020-space-gray",
                COM_APPLE_MACBOOKAIR_LATE_2020_SPACE_GRAY
            ),
            (
                "com.apple.macbookair-late-2020-silver",
                COM_APPLE_MACBOOKAIR_LATE_2020_SILVER
            ),
            (
                "com.apple.macbookair-late-2020",
                COM_APPLE_MACBOOKAIR_LATE_2020
            ),
            ("com.apple.macmini-2020", COM_APPLE_MACMINI_2020),
            (
                "com.apple.macpro-2019-rackmount",
                COM_APPLE_MACPRO_2019_RACKMOUNT
            ),
            ("com.apple.macpro-2019", COM_APPLE_MACPRO_2019),
            ("com.apple.macpro-cylinder", COM_APPLE_MACPRO_CYLINDER),
            ("com.apple.macpro-firewire", COM_APPLE_MACPRO_FIREWIRE),
            ("com.apple.macpro", COM_APPLE_MACPRO),
            (
                "com.apple.macbookair-2020-gold",
                COM_APPLE_MACBOOKAIR_2020_GOLD
            ),
            (
                "com.apple.macbookair-2020-space-gray",
                COM_APPLE_MACBOOKAIR_2020_SPACE_GRAY
            ),
            (
                "com.apple.macbookair-2020-silver",
                COM_APPLE_MACBOOKAIR_2020_SILVER
            ),
            ("com.apple.macbookair-2020", COM_APPLE_MACBOOKAIR_2020),
            (
                "com.apple.macbookair-2019-gold",
                COM_APPLE_MACBOOKAIR_2019_GOLD
            ),
            (
                "com.apple.macbookair-2019-space-gray",
                COM_APPLE_MACBOOKAIR_2019_SPACE_GRAY
            ),
            (
                "com.apple.macbookair-2019-silver",
                COM_APPLE_MACBOOKAIR_2019_SILVER
            ),
            ("com.apple.macbookair-2019", COM_APPLE_MACBOOKAIR_2019),
            (
                "com.apple.macbookair-2018-gold",
                COM_APPLE_MACBOOKAIR_2018_GOLD
            ),
            (
                "com.apple.macbookair-2018-space-gray",
                COM_APPLE_MACBOOKAIR_2018_SPACE_GRAY
            ),
            (
                "com.apple.macbookair-2018-silver",
                COM_APPLE_MACBOOKAIR_2018_SILVER
            ),
            ("com.apple.macbookair-2018", COM_APPLE_MACBOOKAIR_2018),
            (
                "com.apple.macbookair-13-unibody",
                COM_APPLE_MACBOOKAIR_13_UNIBODY
            ),
            (
                "com.apple.macbookair-11-unibody",
                COM_APPLE_MACBOOKAIR_11_UNIBODY
            ),
            ("com.apple.macbookair", COM_APPLE_MACBOOKAIR),
            (
                "com.apple.macbookpro-16-space-gray-mid-2020",
                COM_APPLE_MACBOOKPRO_16_SPACE_GRAY_MID_2020
            ),
            (
                "com.apple.macbookpro-16-silver-mid-2020",
                COM_APPLE_MACBOOKPRO_16_SILVER_MID_2020
            ),
            (
                "com.apple.macbookpro-16-mid-2020",
                COM_APPLE_MACBOOKPRO_16_MID_2020
            ),
            (
                "com.apple.macbookpro-13-retina-touchid-space-gray-2020",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2020
            ),
            (
                "com.apple.macbookpro-13-retina-touchid-silver-2020",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2020
            ),
            (
                "com.apple.macbookpro-13-retina-touchid-2020",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2020
            ),
            (
                "com.apple.macbookpro-13-retina-four-usb-ports-space-gray-2020",
                COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_SPACE_GRAY_2020
            ),
            (
                "com.apple.macbookpro-13-retina-four-usb-ports-silver-2020",
                COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_SILVER_2020
            ),
            (
                "com.apple.macbookpro-13-retina-four-usb-ports-2020",
                COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_2020
            ),
            (
                "com.apple.macbookpro-16-space-gray",
                COM_APPLE_MACBOOKPRO_16_SPACE_GRAY
            ),
            (
                "com.apple.macbookpro-16-silver",
                COM_APPLE_MACBOOKPRO_16_SILVER
            ),
            ("com.apple.macbookpro-16", COM_APPLE_MACBOOKPRO_16),
            ("com.apple.macbookpro-17", COM_APPLE_MACBOOKPRO_17),
            (
                "com.apple.macbookpro-17-unibody",
                COM_APPLE_MACBOOKPRO_17_UNIBODY
            ),
            (
                "com.apple.macbookpro-15-retina-display",
                COM_APPLE_MACBOOKPRO_15_RETINA_DISPLAY
            ),
            (
                "com.apple.macbookpro-15-unibody",
                COM_APPLE_MACBOOKPRO_15_UNIBODY
            ),
            ("com.apple.macbookpro-15", COM_APPLE_MACBOOKPRO_15),
            ("com.apple.macbookpro", COM_APPLE_MACBOOKPRO),
            (
                "com.apple.macbookpro-13-retina-display",
                COM_APPLE_MACBOOKPRO_13_RETINA_DISPLAY
            ),
            (
                "com.apple.macbookpro-13-unibody",
                COM_APPLE_MACBOOKPRO_13_UNIBODY
            ),
            (
                "com.apple.macbook-retina-gold-2018",
                COM_APPLE_MACBOOK_RETINA_GOLD_2018
            ),
            (
                "com.apple.macbook-retina-rose-gold-2017",
                COM_APPLE_MACBOOK_RETINA_ROSE_GOLD_2017
            ),
            (
                "com.apple.macbook-retina-space-gray-2017",
                COM_APPLE_MACBOOK_RETINA_SPACE_GRAY_2017
            ),
            (
                "com.apple.macbook-retina-gold-2017",
                COM_APPLE_MACBOOK_RETINA_GOLD_2017
            ),
            (
                "com.apple.macbook-retina-silver-2017",
                COM_APPLE_MACBOOK_RETINA_SILVER_2017
            ),
            (
                "com.apple.macbook-retina-rose-gold",
                COM_APPLE_MACBOOK_RETINA_ROSE_GOLD
            ),
            (
                "com.apple.macbook-retina-space-gray",
                COM_APPLE_MACBOOK_RETINA_SPACE_GRAY
            ),
            (
                "com.apple.macbook-retina-gold",
                COM_APPLE_MACBOOK_RETINA_GOLD
            ),
            (
                "com.apple.macbook-retina-silver",
                COM_APPLE_MACBOOK_RETINA_SILVER
            ),
            ("com.apple.macbook-retina", COM_APPLE_MACBOOK_RETINA),
            (
                "com.apple.macbook-unibody-plastic",
                COM_APPLE_MACBOOK_UNIBODY_PLASTIC
            ),
            ("com.apple.macbook-unibody", COM_APPLE_MACBOOK_UNIBODY),
            ("com.apple.macbook-black", COM_APPLE_MACBOOK_BLACK),
            ("com.apple.macbook-white", COM_APPLE_MACBOOK_WHITE),
            ("com.apple.macbook", COM_APPLE_MACBOOK),
            ("com.apple.imacpro-2017", COM_APPLE_IMACPRO_2017),
            (
                "com.apple.imac-unibody-27-no-optical-2020",
                COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2020
            ),
            (
                "com.apple.imac-unibody-27-no-optical-2019",
                COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2019
            ),
            (
                "com.apple.imac-unibody-21-no-optical-2019",
                COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_2019
            ),
            (
                "com.apple.imac-unibody-27-no-optical-2017",
                COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2017
            ),
            (
                "com.apple.imac-unibody-21-no-optical-2017",
                COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_2017
            ),
            (
                "com.apple.imac-unibody-27-no-optical-late-2015",
                COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_LATE_2015
            ),
            (
                "com.apple.imac-unibody-21-no-optical.mid-2015",
                COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_MID_2015
            ),
            ("com.apple.imac-15-1", COM_APPLE_IMAC_15_1),
            (
                "com.apple.imac-unibody-27-no-optical",
                COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL
            ),
            (
                "com.apple.imac-unibody-21-no-optical",
                COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL
            ),
            ("com.apple.imac-unibody", COM_APPLE_IMAC_UNIBODY),
            ("com.apple.imac-unibody-21", COM_APPLE_IMAC_UNIBODY_21),
            ("com.apple.imac-aluminum-24", COM_APPLE_IMAC_ALUMINUM_24),
            ("com.apple.imac-aluminum-20", COM_APPLE_IMAC_ALUMINUM_20),
            ("com.apple.imac-iSight-24", COM_APPLE_IMAC_ISIGHT_24),
            ("com.apple.imac-core-2-duo", COM_APPLE_IMAC_CORE_2_DUO),
            ("com.apple.imac-core-duo", COM_APPLE_IMAC_CORE_DUO),
            ("com.apple.imac-g5-isight", COM_APPLE_IMAC_G5_ISIGHT),
            ("com.apple.imac-g5", COM_APPLE_IMAC_G5),
            ("com.apple.imac-g4-20", COM_APPLE_IMAC_G4_20),
            ("com.apple.imac-g4-17", COM_APPLE_IMAC_G4_17),
            ("com.apple.imac-g4-15", COM_APPLE_IMAC_G4_15),
            ("com.apple.imac", COM_APPLE_IMAC),
            ("com.apple.emac", COM_APPLE_EMAC),
            ("com.apple.macmini-2018", COM_APPLE_MACMINI_2018),
            (
                "com.apple.macmini-unibody-no-optical",
                COM_APPLE_MACMINI_UNIBODY_NO_OPTICAL
            ),
            ("com.apple.macmini-unibody", COM_APPLE_MACMINI_UNIBODY),
            ("com.apple.macmini-core-duo", COM_APPLE_MACMINI_CORE_DUO),
            ("com.apple.macmini-g4", COM_APPLE_MACMINI_G4),
            ("com.apple.macmini", COM_APPLE_MACMINI),
            ("com.apple.xserve-xeon", COM_APPLE_XSERVE_XEON),
            ("com.apple.xserve-g5", COM_APPLE_XSERVE_G5),
            ("com.apple.xserve-g4", COM_APPLE_XSERVE_G4),
            ("com.apple.xserve", COM_APPLE_XSERVE),
            ("com.apple.powermac-g5", COM_APPLE_POWERMAC_G5),
            (
                "com.apple.powermac-g4-mirrored-drive-doors",
                COM_APPLE_POWERMAC_G4_MIRRORED_DRIVE_DOORS
            ),
            (
                "com.apple.powermac-g4-quicksilver",
                COM_APPLE_POWERMAC_G4_QUICKSILVER
            ),
            ("com.apple.powermac", COM_APPLE_POWERMAC),
            ("com.apple.ibook-g4", COM_APPLE_IBOOK_G4),
            ("com.apple.powerbook-g4-17", COM_APPLE_POWERBOOK_G4_17),
            ("com.apple.powerbook-g4-15", COM_APPLE_POWERBOOK_G4_15),
            ("com.apple.powerbook-g4-12", COM_APPLE_POWERBOOK_G4_12),
            (
                "com.apple.powerbook-g4-titanium",
                COM_APPLE_POWERBOOK_G4_TITANIUM
            ),
            ("com.apple.powerbook", COM_APPLE_POWERBOOK),
            ("com.apple.mac.rackmount", COM_APPLE_MAC_RACKMOUNT),
            ("com.apple.mac.tower", COM_APPLE_MAC_TOWER),
            (
                "com.apple.macbookpro-15-space-gray-late-2018",
                COM_APPLE_MACBOOKPRO_15_SPACE_GRAY_LATE_2018
            ),
            (
                "com.apple.macbookpro-15-silver-late-2018",
                COM_APPLE_MACBOOKPRO_15_SILVER_LATE_2018
            ),
            (
                "com.apple.macbookpro-15-late-2018",
                COM_APPLE_MACBOOKPRO_15_LATE_2018
            ),
            (
                "com.apple.macbookpro-15-retina-touchid-space-gray-2018",
                COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY_2018
            ),
            (
                "com.apple.macbookpro-15-retina-touchid-silver-2018",
                COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER_2018
            ),
            (
                "com.apple.macbookpro-15-retina-touchid-2018",
                COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_2018
            ),
            (
                "com.apple.macbookpro-15-retina-touchid-space-gray-2017",
                COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY_2017
            ),
            (
                "com.apple.macbookpro-15-retina-touchid-silver-2017",
                COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER_2017
            ),
            (
                "com.apple.macbookpro-15-retina-touchid-2017",
                COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_2017
            ),
            (
                "com.apple.macbookpro-15-retina-touchid-space-gray",
                COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY
            ),
            (
                "com.apple.macbookpro-15-retina-touchid-silver",
                COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER
            ),
            (
                "com.apple.macbookpro-15-retina-touchid",
                COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID
            ),
            (
                "com.apple.macbookpro-13-retina-usbc-space-gray-2019",
                COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY_2019
            ),
            (
                "com.apple.macbookpro-13-retina-usbc-silver-2019",
                COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER_2019
            ),
            (
                "com.apple.macbookpro-13-retina-usbc-2019",
                COM_APPLE_MACBOOKPRO_13_RETINA_USBC_2019
            ),
            (
                "com.apple.macbookpro-13-retina-touchid-space-gray-2018",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2018
            ),
            (
                "com.apple.macbookpro-13-retina-touchid-silver-2018",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2018
            ),
            (
                "com.apple.macbookpro-13-retina-touchid-2018",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2018
            ),
            (
                "com.apple.macbookpro-13-retina-touchid-space-gray-2017",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2017
            ),
            (
                "com.apple.macbookpro-13-retina-touchid-silver-2017",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2017
            ),
            (
                "com.apple.macbookpro-13-retina-touchid-2017",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2017
            ),
            (
                "com.apple.macbookpro-13-retina-touchid-space-gray",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY
            ),
            (
                "com.apple.macbookpro-13-retina-touchid-silver",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER
            ),
            (
                "com.apple.macbookpro-13-retina-touchid",
                COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID
            ),
            (
                "com.apple.macbookpro-13-retina-usbc-space-gray-2017",
                COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY_2017
            ),
            (
                "com.apple.macbookpro-13-retina-usbc-silver-2017",
                COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER_2017
            ),
            (
                "com.apple.macbookpro-13-retina-usbc-2017",
                COM_APPLE_MACBOOKPRO_13_RETINA_USBC_2017
            ),
            (
                "com.apple.macbookpro-13-retina-usbc-space-gray",
                COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY
            ),
            (
                "com.apple.macbookpro-13-retina-usbc-silver",
                COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER
            ),
            (
                "com.apple.macbookpro-13-retina-usbc",
                COM_APPLE_MACBOOKPRO_13_RETINA_USBC
            ),
            ("com.apple.mac.laptop", COM_APPLE_MAC_LAPTOP),
            ("com.apple.mac", COM_APPLE_MAC),
            ("com.apple.device", COM_APPLE_DEVICE),
            ("public.generic-pc", PUBLIC_GENERIC_PC),
            ("public.computer", PUBLIC_COMPUTER),
            ("public.speaker", PUBLIC_SPEAKER),
            ("public.display", PUBLIC_DISPLAY),
            ("com.apple.virtual-machine", COM_APPLE_VIRTUAL_MACHINE),
            ("public.device", PUBLIC_DEVICE),
            ("com.apple.watchface", COM_APPLE_WATCHFACE),
            ("com.apple.pkpasses-data", COM_APPLE_PKPASSES_DATA),
            ("com.apple.pkpass-data", COM_APPLE_PKPASS_DATA),
            ("com.apple.pkpass", COM_APPLE_PKPASS),
            ("com.apple.mobileprovision", COM_APPLE_MOBILEPROVISION),
            ("com.apple.notifications", COM_APPLE_NOTIFICATIONS),
            ("com.apple.bonjour", COM_APPLE_BONJOUR),
            ("com.apple.airdrop", COM_APPLE_AIRDROP),
            ("com.apple.user-unknown", COM_APPLE_USER_UNKNOWN),
            ("com.apple.user-group", COM_APPLE_USER_GROUP),
            ("com.apple.help-document", COM_APPLE_HELP_DOCUMENT),
            ("com.apple.guest-user", COM_APPLE_GUEST_USER),
            ("com.apple.user", COM_APPLE_USER),
            ("com.apple.configprofile", COM_APPLE_CONFIGPROFILE),
            ("com.apple.provisionprofile", COM_APPLE_PROVISIONPROFILE),
            ("com.apple.mobileconfig", COM_APPLE_MOBILECONFIG),
            ("com.apple.ink.inktext", COM_APPLE_INK_INKTEXT),
            ("com.apple.color-file", COM_APPLE_COLOR_FILE),
            (
                "com.apple.profile-font-and-color",
                COM_APPLE_PROFILE_FONT_AND_COLOR
            ),
            (
                "com.apple.profile-background-color",
                COM_APPLE_PROFILE_BACKGROUND_COLOR
            ),
            ("com.apple.colorsync-profile", COM_APPLE_COLORSYNC_PROFILE),
            (
                "com.apple.txn.text-multimedia-data",
                COM_APPLE_TXN_TEXT_MULTIMEDIA_DATA
            ),
            (
                "org.aafassociation.advanced-authoring-format",
                ORG_AAFASSOCIATION_ADVANCED_AUTHORING_FORMAT
            ),
            (
                "com.apple.localized-pdf-bundle",
                COM_APPLE_LOCALIZED_PDF_BUNDLE
            ),
            ("org.idpf.epub-container", ORG_IDPF_EPUB_CONTAINER),
            ("com.apple.webarchive", COM_APPLE_WEBARCHIVE),
            ("com.apple.alert-stop", COM_APPLE_ALERT_STOP),
            ("com.apple.alert-caution", COM_APPLE_ALERT_CAUTION),
            ("com.apple.alert-note", COM_APPLE_ALERT_NOTE),
            ("com.apple.alert", COM_APPLE_ALERT),
            ("public.x509-certificate", PUBLIC_X509_CERTIFICATE),
            ("com.rsa.pkcs-12", COM_RSA_PKCS_12),
            ("org.oasis-open.opendocument", ORG_OASIS_OPEN_OPENDOCUMENT),
            ("org.openxmlformats.openxml", ORG_OPENXMLFORMATS_OPENXML),
            ("com.apple.data-container", COM_APPLE_DATA_CONTAINER),
            (
                "com.apple.file-system-plug-in",
                COM_APPLE_FILE_SYSTEM_PLUG_IN
            ),
            ("com.apple.ppp-plug-in", COM_APPLE_PPP_PLUG_IN),
            ("com.apple.system-extension", COM_APPLE_SYSTEM_EXTENSION),
            ("com.apple.driver-extension", COM_APPLE_DRIVER_EXTENSION),
            ("com.apple.dashboard-widget", COM_APPLE_DASHBOARD_WIDGET),
            (
                "com.apple.quicklook-generator",
                COM_APPLE_QUICKLOOK_GENERATOR
            ),
            ("com.apple.metadata-importer", COM_APPLE_METADATA_IMPORTER),
            ("com.apple.webkit-plugin", COM_APPLE_WEBKIT_PLUGIN),
            ("com.apple.pluginkit", COM_APPLE_PLUGINKIT),
            (
                "com.apple.application-and-system-extension",
                COM_APPLE_APPLICATION_AND_SYSTEM_EXTENSION
            ),
            ("com.apple.kernel-extension", COM_APPLE_KERNEL_EXTENSION),
            ("com.apple.xpc-service", COM_APPLE_XPC_SERVICE),
            ("com.apple.plugin", COM_APPLE_PLUGIN),
            (
                "com.apple.service-application",
                COM_APPLE_SERVICE_APPLICATION
            ),
            (
                "com.apple.application-placeholder",
                COM_APPLE_APPLICATION_PLACEHOLDER
            ),
            ("com.apple.rtfd", COM_APPLE_RTFD),
            (
                "com.apple.intelligentsuggestions.assets",
                COM_APPLE_INTELLIGENTSUGGESTIONS_ASSETS
            ),
            (
                "com.apple.installer-meta-package",
                COM_APPLE_INSTALLER_META_PACKAGE
            ),
            (
                "com.apple.installer-distribution-package",
                COM_APPLE_INSTALLER_DISTRIBUTION_PACKAGE
            ),
            ("com.apple.installer-package", COM_APPLE_INSTALLER_PACKAGE),
            ("com.apple.flat-rtfd", COM_APPLE_FLAT_RTFD),
            (
                "com.apple.deprecated-application-file",
                COM_APPLE_DEPRECATED_APPLICATION_FILE
            ),
            ("com.apple.application-file", COM_APPLE_APPLICATION_FILE),
            ("com.apple.application-bundle", COM_APPLE_APPLICATION_BUNDLE),
            ("com.apple.framework", COM_APPLE_FRAMEWORK),
            (
                "com.apple.localizable-name-bundle",
                COM_APPLE_LOCALIZABLE_NAME_BUNDLE
            ),
            ("com.apple.menu-extra", COM_APPLE_MENU_EXTRA),
            (
                "com.apple.systempreference.screen-slide-saver",
                COM_APPLE_SYSTEMPREFERENCE_SCREEN_SLIDE_SAVER
            ),
            (
                "com.apple.systempreference.screen-saver",
                COM_APPLE_SYSTEMPREFERENCE_SCREEN_SAVER
            ),
            (
                "com.apple.systempreference.prefpane",
                COM_APPLE_SYSTEMPREFERENCE_PREFPANE
            ),
            ("com.apple.generic-bundle", COM_APPLE_GENERIC_BUNDLE),
            ("com.apple.bundle", COM_APPLE_BUNDLE),
            ("com.apple.package", COM_APPLE_PACKAGE),
            ("com.apple.user-idisk", COM_APPLE_USER_IDISK),
            ("com.apple.idisk", COM_APPLE_IDISK),
            ("com.apple.dot-mac", COM_APPLE_DOT_MAC),
            (
                "com.apple.network-neighborhood",
                COM_APPLE_NETWORK_NEIGHBORHOOD
            ),
            ("public.file-sharepoint", PUBLIC_FILE_SHAREPOINT),
            ("public.volume", PUBLIC_VOLUME),
            ("com.apple.utilities-folder", COM_APPLE_UTILITIES_FOLDER),
            ("com.apple.users-folder", COM_APPLE_USERS_FOLDER),
            ("com.apple.developer-folder", COM_APPLE_DEVELOPER_FOLDER),
            ("com.apple.home-folder", COM_APPLE_HOME_FOLDER),
            ("com.apple.public-folder", COM_APPLE_PUBLIC_FOLDER),
            ("com.apple.pictures-folder", COM_APPLE_PICTURES_FOLDER),
            ("com.apple.music-folder", COM_APPLE_MUSIC_FOLDER),
            ("com.apple.movie-folder", COM_APPLE_MOVIE_FOLDER),
            ("com.apple.downloads-folder", COM_APPLE_DOWNLOADS_FOLDER),
            ("com.apple.documents-folder", COM_APPLE_DOCUMENTS_FOLDER),
            ("com.apple.desktop-folder", COM_APPLE_DESKTOP_FOLDER),
            ("com.apple.servers-folder", COM_APPLE_SERVERS_FOLDER),
            ("com.apple.groups-folder", COM_APPLE_GROUPS_FOLDER),
            ("com.apple.sites-folder", COM_APPLE_SITES_FOLDER),
            ("com.apple.trash-full", COM_APPLE_TRASH_FULL),
            ("com.apple.trash-empty", COM_APPLE_TRASH_EMPTY),
            (
                "com.apple.document-type.system-folder",
                COM_APPLE_DOCUMENT_TYPE_SYSTEM_FOLDER
            ),
            ("com.apple.library-folder", COM_APPLE_LIBRARY_FOLDER),
            (
                "com.apple.server-applications-folder",
                COM_APPLE_SERVER_APPLICATIONS_FOLDER
            ),
            (
                "com.apple.applications-folder",
                COM_APPLE_APPLICATIONS_FOLDER
            ),
            ("com.apple.drop-folder", COM_APPLE_DROP_FOLDER),
            ("public.folder", PUBLIC_FOLDER),
            (
                "com.apple.scripting-definition",
                COM_APPLE_SCRIPTING_DEFINITION
            ),
            (
                "com.apple.applescript.script-bundle",
                COM_APPLE_APPLESCRIPT_SCRIPT_BUNDLE
            ),
            ("com.apple.applescript.script", COM_APPLE_APPLESCRIPT_SCRIPT),
            ("com.apple.applescript.text", COM_APPLE_APPLESCRIPT_TEXT),
            ("com.apple.profile-font-icon", COM_APPLE_PROFILE_FONT_ICON),
            (
                "com.adobe.postscript-pfa-font",
                COM_ADOBE_POSTSCRIPT_PFA_FONT
            ),
            (
                "com.adobe.postscript-pfb-font",
                COM_ADOBE_POSTSCRIPT_PFB_FONT
            ),
            (
                "com.adobe.postscript-lwfn-font",
                COM_ADOBE_POSTSCRIPT_LWFN_FONT
            ),
            ("com.apple.font-suitcase", COM_APPLE_FONT_SUITCASE),
            (
                "public.truetype-collection-font",
                PUBLIC_TRUETYPE_COLLECTION_FONT
            ),
            ("public.truetype-ttf-font", PUBLIC_TRUETYPE_TTF_FONT),
            (
                "public.opentype-collection-font",
                PUBLIC_OPENTYPE_COLLECTION_FONT
            ),
            ("public.opentype-font", PUBLIC_OPENTYPE_FONT),
            (
                "com.apple.truetype-datafork-suitcase-font",
                COM_APPLE_TRUETYPE_DATAFORK_SUITCASE_FONT
            ),
            ("com.adobe.postscript-font", COM_ADOBE_POSTSCRIPT_FONT),
            ("public.truetype-font", PUBLIC_TRUETYPE_FONT),
            ("public.font", PUBLIC_FONT),
            ("com.sony.wave64", COM_SONY_WAVE64),
            ("com.audible.aax-audiobook", COM_AUDIBLE_AAX_AUDIOBOOK),
            ("com.audible.aa-audiobook", COM_AUDIBLE_AA_AUDIOBOOK),
            ("com.audible.aa-audio", COM_AUDIBLE_AA_AUDIO),
            ("public.aac-audio", PUBLIC_AAC_AUDIO),
            (
                "org.3gpp.adaptive-multi-rate-audio",
                ORG_3GPP_ADAPTIVE_MULTI_RATE_AUDIO
            ),
            ("public.enhanced-ac3-audio", PUBLIC_ENHANCED_AC3_AUDIO),
            ("public.ac3-audio", PUBLIC_AC3_AUDIO),
            ("com.apple.coreaudio-format", COM_APPLE_COREAUDIO_FORMAT),
            ("public.downloadable-sound", PUBLIC_DOWNLOADABLE_SOUND),
            ("public.midi-audio", PUBLIC_MIDI_AUDIO),
            ("public.cdda-audio", PUBLIC_CDDA_AUDIO),
            ("public.aiff-audio", PUBLIC_AIFF_AUDIO),
            ("public.aifc-audio", PUBLIC_AIFC_AUDIO),
            ("public.au-audio", PUBLIC_AU_AUDIO),
            ("public.ulaw-audio", PUBLIC_ULAW_AUDIO),
            (
                "com.apple.protected-mpeg-4-audio-b",
                COM_APPLE_PROTECTED_MPEG_4_AUDIO_B
            ),
            (
                "com.apple.protected-mpeg-4-audio",
                COM_APPLE_PROTECTED_MPEG_4_AUDIO
            ),
            ("com.apple.mpeg-4-ringtone", COM_APPLE_MPEG_4_RINGTONE),
            ("com.apple.m4a-audio", COM_APPLE_M4A_AUDIO),
            ("public.mpeg-4-audio", PUBLIC_MPEG_4_AUDIO),
            ("public.pls-playlist", PUBLIC_PLS_PLAYLIST),
            ("public.m3u-playlist", PUBLIC_M3U_PLAYLIST),
            ("public.playlist", PUBLIC_PLAYLIST),
            ("public.mp3", PUBLIC_MP3),
            ("public.mp2", PUBLIC_MP2),
            ("com.apple.audio-unit-preset", COM_APPLE_AUDIO_UNIT_PRESET),
            ("public.avchd-collection", PUBLIC_AVCHD_COLLECTION),
            (
                "public.audiovisual-content-collection",
                PUBLIC_AUDIOVISUAL_CONTENT_COLLECTION
            ),
            (
                "public.mpeg-2-transport-stream",
                PUBLIC_MPEG_2_TRANSPORT_STREAM
            ),
            ("public.flc-animation", PUBLIC_FLC_ANIMATION),
            ("public.3gpp2", PUBLIC_3GPP2),
            ("public.3gpp", PUBLIC_3GPP),
            ("public.avi", PUBLIC_AVI),
            ("public.dv-movie", PUBLIC_DV_MOVIE),
            (
                "com.apple.protected-mpeg-4-video",
                COM_APPLE_PROTECTED_MPEG_4_VIDEO
            ),
            ("com.apple.m4v-video", COM_APPLE_M4V_VIDEO),
            ("public.mpeg-4", PUBLIC_MPEG_4),
            ("public.mpeg-2-video", PUBLIC_MPEG_2_VIDEO),
            ("public.mpeg", PUBLIC_MPEG),
            ("com.apple.quicktime-movie", COM_APPLE_QUICKTIME_MOVIE),
            ("public.audio", PUBLIC_AUDIO),
            ("public.video", PUBLIC_VIDEO),
            ("public.movie", PUBLIC_MOVIE),
            ("public.audiovisual-content", PUBLIC_AUDIOVISUAL_CONTENT),
            ("gov.nih.nifti-1", GOV_NIH_NIFTI_1),
            ("org.nema.dicom", ORG_NEMA_DICOM),
            ("ca.mcgill.mni.bic.mnc", CA_MCGILL_MNI_BIC_MNC),
            ("public.mpo-image", PUBLIC_MPO_IMAGE),
            ("public.xbitmap-image", PUBLIC_XBITMAP_IMAGE),
            ("com.apple.icns", COM_APPLE_ICNS),
            ("com.apple.quicktime-image", COM_APPLE_QUICKTIME_IMAGE),
            ("public.svg-image", PUBLIC_SVG_IMAGE),
            ("public.png", PUBLIC_PNG),
            ("com.apple.macpaint-image", COM_APPLE_MACPAINT_IMAGE),
            ("com.apple.pict", COM_APPLE_PICT),
            ("public.tiff", PUBLIC_TIFF),
            ("public.jpeg-2000", PUBLIC_JPEG_2000),
            ("public.jpeg", PUBLIC_JPEG),
            ("public.camera-raw-image", PUBLIC_CAMERA_RAW_IMAGE),
            ("public.fax", PUBLIC_FAX),
            (
                "com.apple.private.live-photo-bundle",
                COM_APPLE_PRIVATE_LIVE_PHOTO_BUNDLE
            ),
            ("com.apple.live-photo", COM_APPLE_LIVE_PHOTO),
            ("public.image", PUBLIC_IMAGE),
            ("public.make-source", PUBLIC_MAKE_SOURCE),
            ("com.sun.java-web-start", COM_SUN_JAVA_WEB_START),
            ("public.php-script", PUBLIC_PHP_SCRIPT),
            ("public.ruby-script", PUBLIC_RUBY_SCRIPT),
            ("public.python-script", PUBLIC_PYTHON_SCRIPT),
            ("public.perl-script", PUBLIC_PERL_SCRIPT),
            ("public.zsh-script", PUBLIC_ZSH_SCRIPT),
            ("public.tcsh-script", PUBLIC_TCSH_SCRIPT),
            ("public.ksh-script", PUBLIC_KSH_SCRIPT),
            ("public.csh-script", PUBLIC_CSH_SCRIPT),
            ("public.bash-script", PUBLIC_BASH_SCRIPT),
            ("public.shell-script", PUBLIC_SHELL_SCRIPT),
            ("com.apple.xcode.dsym", COM_APPLE_XCODE_DSYM),
            (
                "com.netscape.javascript-source",
                COM_NETSCAPE_JAVASCRIPT_SOURCE
            ),
            ("public.dylan-source", PUBLIC_DYLAN_SOURCE),
            ("public.ada-source", PUBLIC_ADA_SOURCE),
            ("public.pascal-source", PUBLIC_PASCAL_SOURCE),
            ("public.fortran-95-source", PUBLIC_FORTRAN_95_SOURCE),
            ("public.fortran-90-source", PUBLIC_FORTRAN_90_SOURCE),
            ("public.fortran-77-source", PUBLIC_FORTRAN_77_SOURCE),
            ("public.fortran-source", PUBLIC_FORTRAN_SOURCE),
            ("com.apple.symbol-export", COM_APPLE_SYMBOL_EXPORT),
            ("public.mig-source", PUBLIC_MIG_SOURCE),
            ("public.yacc-source", PUBLIC_YACC_SOURCE),
            ("public.lex-source", PUBLIC_LEX_SOURCE),
            ("com.apple.rez-source", COM_APPLE_REZ_SOURCE),
            ("public.assembly-source", PUBLIC_ASSEMBLY_SOURCE),
            ("public.script", PUBLIC_SCRIPT),
            ("com.sun.java-source", COM_SUN_JAVA_SOURCE),
            ("public.swift-source", PUBLIC_SWIFT_SOURCE),
            (
                "public.precompiled-c-plus-plus-header",
                PUBLIC_PRECOMPILED_C_PLUS_PLUS_HEADER
            ),
            (
                "public.c-plus-plus-inline-header",
                PUBLIC_C_PLUS_PLUS_INLINE_HEADER
            ),
            ("public.c-plus-plus-header", PUBLIC_C_PLUS_PLUS_HEADER),
            ("public.precompiled-c-header", PUBLIC_PRECOMPILED_C_HEADER),
            ("public.c-header", PUBLIC_C_HEADER),
            (
                "public.objective-c-plus-plus-source.preprocessed",
                PUBLIC_OBJECTIVE_C_PLUS_PLUS_SOURCE_PREPROCESSED
            ),
            (
                "public.objective-c-plus-plus-source",
                PUBLIC_OBJECTIVE_C_PLUS_PLUS_SOURCE
            ),
            (
                "public.c-plus-plus-source.preprocessed",
                PUBLIC_C_PLUS_PLUS_SOURCE_PREPROCESSED
            ),
            ("public.c-plus-plus-source", PUBLIC_C_PLUS_PLUS_SOURCE),
            (
                "public.objective-c-source.preprocessed",
                PUBLIC_OBJECTIVE_C_SOURCE_PREPROCESSED
            ),
            ("public.objective-c-source", PUBLIC_OBJECTIVE_C_SOURCE),
            ("com.apple.iig-source", COM_APPLE_IIG_SOURCE),
            ("public.c-source.preprocessed", PUBLIC_C_SOURCE_PREPROCESSED),
            ("public.c-source", PUBLIC_C_SOURCE),
            (
                "public.source-code.preprocessed",
                PUBLIC_SOURCE_CODE_PREPROCESSED
            ),
            ("public.source-code", PUBLIC_SOURCE_CODE),
            (
                "com.apple.ascii-property-list",
                COM_APPLE_ASCII_PROPERTY_LIST
            ),
            (
                "com.apple.binary-property-list",
                COM_APPLE_BINARY_PROPERTY_LIST
            ),
            ("com.apple.xml-property-list", COM_APPLE_XML_PROPERTY_LIST),
            ("com.apple.property-list", COM_APPLE_PROPERTY_LIST),
            ("com.apple.generic-stationery", COM_APPLE_GENERIC_STATIONERY),
            ("org.w3.webvtt", ORG_W3_WEBVTT),
            ("com.scenarist.closed-caption", COM_SCENARIST_CLOSED_CAPTION),
            ("public.yaml", PUBLIC_YAML),
            ("public.ndjson", PUBLIC_NDJSON),
            ("public.json", PUBLIC_JSON),
            ("public.patch-file", PUBLIC_PATCH_FILE),
            ("public.css", PUBLIC_CSS),
            ("public.xfd", PUBLIC_XFD),
            ("public.rss", PUBLIC_RSS),
            ("public.xhtml", PUBLIC_XHTML),
            ("public.xml", PUBLIC_XML),
            ("public.html", PUBLIC_HTML),
            ("public.rtf", PUBLIC_RTF),
            (
                "public.utf8-tab-separated-values-text",
                PUBLIC_UTF8_TAB_SEPARATED_VALUES_TEXT
            ),
            (
                "public.tab-separated-values-text",
                PUBLIC_TAB_SEPARATED_VALUES_TEXT
            ),
            (
                "public.comma-separated-values-text",
                PUBLIC_COMMA_SEPARATED_VALUES_TEXT
            ),
            ("public.delimited-values-text", PUBLIC_DELIMITED_VALUES_TEXT),
            ("com.apple.itunes.store-url", COM_APPLE_ITUNES_STORE_URL),
            (
                "com.microsoft.internet-shortcut",
                COM_MICROSOFT_INTERNET_SHORTCUT
            ),
            (
                "com.apple.generic-internet-location",
                COM_APPLE_GENERIC_INTERNET_LOCATION
            ),
            (
                "com.apple.news-internet-location",
                COM_APPLE_NEWS_INTERNET_LOCATION
            ),
            (
                "com.apple.ftp-internet-location",
                COM_APPLE_FTP_INTERNET_LOCATION
            ),
            (
                "com.apple.file-internet-location",
                COM_APPLE_FILE_INTERNET_LOCATION
            ),
            (
                "com.apple.afp-internet-location",
                COM_APPLE_AFP_INTERNET_LOCATION
            ),
            (
                "com.apple.mail-internet-location",
                COM_APPLE_MAIL_INTERNET_LOCATION
            ),
            (
                "com.apple.vnc-internet-location",
                COM_APPLE_VNC_INTERNET_LOCATION
            ),
            (
                "com.apple.web-internet-location",
                COM_APPLE_WEB_INTERNET_LOCATION
            ),
            ("com.apple.internet-location", COM_APPLE_INTERNET_LOCATION),
            ("public.stored-url", PUBLIC_STORED_URL),
            ("public.url-name", PUBLIC_URL_NAME),
            ("public.file-url", PUBLIC_FILE_URL),
            ("public.url", PUBLIC_URL),
            ("com.apple.encrypted-archive", COM_APPLE_ENCRYPTED_ARCHIVE),
            ("com.apple.archive", COM_APPLE_ARCHIVE),
            (
                "com.apple.installer-package-archive",
                COM_APPLE_INSTALLER_PACKAGE_ARCHIVE
            ),
            ("com.apple.xip-archive", COM_APPLE_XIP_ARCHIVE),
            ("com.apple.xar-archive", COM_APPLE_XAR_ARCHIVE),
            ("public.zip-archive", PUBLIC_ZIP_ARCHIVE),
            ("com.pkware.zip-archive", COM_PKWARE_ZIP_ARCHIVE),
            ("public.cpio-archive", PUBLIC_CPIO_ARCHIVE),
            (
                "com.apple.bom-compressed-cpio",
                COM_APPLE_BOM_COMPRESSED_CPIO
            ),
            ("public.z-archive", PUBLIC_Z_ARCHIVE),
            ("public.uuencoded-archive", PUBLIC_UUENCODED_ARCHIVE),
            (
                "com.apple.applesingle-archive",
                COM_APPLE_APPLESINGLE_ARCHIVE
            ),
            ("com.apple.macbinary-archive", COM_APPLE_MACBINARY_ARCHIVE),
            ("com.apple.binhex-archive", COM_APPLE_BINHEX_ARCHIVE),
            ("public.tar-bzip2-archive", PUBLIC_TAR_BZIP2_ARCHIVE),
            ("public.bzip2-archive", PUBLIC_BZIP2_ARCHIVE),
            ("org.gnu.gnu-zip-tar-archive", ORG_GNU_GNU_ZIP_TAR_ARCHIVE),
            ("org.gnu.gnu-zip-archive", ORG_GNU_GNU_ZIP_ARCHIVE),
            ("public.tar-archive", PUBLIC_TAR_ARCHIVE),
            ("org.gnu.gnu-tar-archive", ORG_GNU_GNU_TAR_ARCHIVE),
            ("public.disk-image", PUBLIC_DISK_IMAGE),
            ("com.apple.bom-archive", COM_APPLE_BOM_ARCHIVE),
            (
                "com.apple.quartz-composer-composition",
                COM_APPLE_QUARTZ_COMPOSER_COMPOSITION
            ),
            (
                "com.sun.web-application-archive",
                COM_SUN_WEB_APPLICATION_ARCHIVE
            ),
            ("com.sun.java-archive", COM_SUN_JAVA_ARCHIVE),
            ("com.sun.java-class", COM_SUN_JAVA_CLASS),
            (
                "com.microsoft.windows-dynamic-link-library",
                COM_MICROSOFT_WINDOWS_DYNAMIC_LINK_LIBRARY
            ),
            (
                "com.microsoft.windows-executable",
                COM_MICROSOFT_WINDOWS_EXECUTABLE
            ),
            ("public.elf-binary", PUBLIC_ELF_BINARY),
            ("com.apple.pef-binary", COM_APPLE_PEF_BINARY),
            ("com.apple.mach-o-bundle", COM_APPLE_MACH_O_BUNDLE),
            ("com.apple.mach-o-dylib", COM_APPLE_MACH_O_DYLIB),
            ("com.apple.mach-o-core", COM_APPLE_MACH_O_CORE),
            (
                "com.apple.x11-mach-o-executable",
                COM_APPLE_X11_MACH_O_EXECUTABLE
            ),
            ("com.apple.mach-o-executable", COM_APPLE_MACH_O_EXECUTABLE),
            ("com.apple.mach-o-object", COM_APPLE_MACH_O_OBJECT),
            ("com.apple.mach-o-binary", COM_APPLE_MACH_O_BINARY),
            ("public.object-code", PUBLIC_OBJECT_CODE),
            (
                "com.apple.finder.recent-items",
                COM_APPLE_FINDER_RECENT_ITEMS
            ),
            (
                "com.apple.finder.smart-folder",
                COM_APPLE_FINDER_SMART_FOLDER
            ),
            ("com.apple.iconset", COM_APPLE_ICONSET),
            ("com.apple.finder.burn-folder", COM_APPLE_FINDER_BURN_FOLDER),
            (
                "com.apple.finder.pictclipping",
                COM_APPLE_FINDER_PICTCLIPPING
            ),
            (
                "com.apple.finder.textclipping",
                COM_APPLE_FINDER_TEXTCLIPPING
            ),
            (
                "com.apple.finder.sound-clipping",
                COM_APPLE_FINDER_SOUND_CLIPPING
            ),
            ("com.apple.finder.clipping", COM_APPLE_FINDER_CLIPPING),
            ("com.apple.icloud-file-fault", COM_APPLE_ICLOUD_FILE_FAULT),
            ("com.apple.alias-record", COM_APPLE_ALIAS_RECORD),
            ("com.apple.alias-file", COM_APPLE_ALIAS_FILE),
            ("com.apple.bookmark", COM_APPLE_BOOKMARK),
            ("com.apple.mount-point", COM_APPLE_MOUNT_POINT),
            ("public.symlink", PUBLIC_SYMLINK),
            ("com.apple.resolvable", COM_APPLE_RESOLVABLE),
            ("com.apple.mapkit.map-item", COM_APPLE_MAPKIT_MAP_ITEM),
            (
                "com.apple.cocoa.pasteboard.find-panel-search-options",
                COM_APPLE_COCOA_PASTEBOARD_FIND_PANEL_SEARCH_OPTIONS
            ),
            (
                "com.apple.cocoa.pasteboard.multiple-text-selection",
                COM_APPLE_COCOA_PASTEBOARD_MULTIPLE_TEXT_SELECTION
            ),
            (
                "com.apple.cocoa.pasteboard.paragraph-formatting",
                COM_APPLE_COCOA_PASTEBOARD_PARAGRAPH_FORMATTING
            ),
            (
                "com.apple.cocoa.pasteboard.character-formatting",
                COM_APPLE_COCOA_PASTEBOARD_CHARACTER_FORMATTING
            ),
            (
                "com.apple.cocoa.pasteboard.sound",
                COM_APPLE_COCOA_PASTEBOARD_SOUND
            ),
            (
                "com.apple.cocoa.pasteboard.color",
                COM_APPLE_COCOA_PASTEBOARD_COLOR
            ),
            (
                "com.apple.pasteboard.promised-file-content-type",
                COM_APPLE_PASTEBOARD_PROMISED_FILE_CONTENT_TYPE
            ),
            (
                "com.apple.pasteboard.promised-file-url",
                COM_APPLE_PASTEBOARD_PROMISED_FILE_URL
            ),
            ("com.apple.device-model-code", COM_APPLE_DEVICE_MODEL_CODE),
            ("com.apple.nspboard-type", COM_APPLE_NSPBOARD_TYPE),
            ("com.apple.ostype", COM_APPLE_OSTYPE),
            ("public.mime-type", PUBLIC_MIME_TYPE),
            ("public.filename-extension", PUBLIC_FILENAME_EXTENSION),
            ("com.apple.ktrace", COM_APPLE_KTRACE),
            ("com.apple.panicreport", COM_APPLE_PANICREPORT),
            ("com.apple.spinreport", COM_APPLE_SPINREPORT),
            ("com.apple.hangreport", COM_APPLE_HANGREPORT),
            ("com.apple.crashreport", COM_APPLE_CRASHREPORT),
            ("com.apple.gpuRestart", COM_APPLE_GPURESTART),
            ("com.apple.shutdownStall", COM_APPLE_SHUTDOWNSTALL),
            ("com.apple.log", COM_APPLE_LOG),
            ("public.log", PUBLIC_LOG),
            ("public.case-insensitive-text", PUBLIC_CASE_INSENSITIVE_TEXT),
            (
                "com.apple.traditional-mac-plain-text",
                COM_APPLE_TRADITIONAL_MAC_PLAIN_TEXT
            ),
            ("public.utf16-plain-text", PUBLIC_UTF16_PLAIN_TEXT),
            (
                "public.utf16-external-plain-text",
                PUBLIC_UTF16_EXTERNAL_PLAIN_TEXT
            ),
            ("public.utf8-plain-text", PUBLIC_UTF8_PLAIN_TEXT),
            ("public.plain-text", PUBLIC_PLAIN_TEXT),
            ("public.text", PUBLIC_TEXT),
            ("com.apple.shazamcatalog", COM_APPLE_SHAZAMCATALOG),
            ("com.apple.shazamsignature", COM_APPLE_SHAZAMSIGNATURE),
            ("public.vcard", PUBLIC_VCARD),
            ("public.contact", PUBLIC_CONTACT),
            ("com.apple.ical.ics", COM_APPLE_ICAL_ICS),
            ("com.apple.ical.vcs", COM_APPLE_ICAL_VCS),
            ("public.calendar-event", PUBLIC_CALENDAR_EVENT),
            ("public.to-do-item", PUBLIC_TO_DO_ITEM),
            ("public.email-message", PUBLIC_EMAIL_MESSAGE),
            ("public.message", PUBLIC_MESSAGE),
            ("com.apple.arobject", COM_APPLE_AROBJECT),
            ("com.apple.scenekit.scene", COM_APPLE_SCENEKIT_SCENE),
            ("com.apple.reality", COM_APPLE_REALITY),
            (
                "com.pixar.universal-scene-description-mobile",
                COM_PIXAR_UNIVERSAL_SCENE_DESCRIPTION_MOBILE
            ),
            (
                "com.pixar.universal-scene-description",
                COM_PIXAR_UNIVERSAL_SCENE_DESCRIPTION
            ),
            ("public.polygon-file-format", PUBLIC_POLYGON_FILE_FORMAT),
            (
                "public.standard-tesselated-geometry-format",
                PUBLIC_STANDARD_TESSELATED_GEOMETRY_FORMAT
            ),
            (
                "public.geometry-definition-format",
                PUBLIC_GEOMETRY_DEFINITION_FORMAT
            ),
            ("public.alembic", PUBLIC_ALEMBIC),
            ("public.3d-content", PUBLIC_3D_CONTENT),
            ("com.apple.iCloud", COM_APPLE_ICLOUD),
            ("public.spreadsheet", PUBLIC_SPREADSHEET),
            ("public.presentation", PUBLIC_PRESENTATION),
            ("com.apple.csstore", COM_APPLE_CSSTORE),
            ("public.database", PUBLIC_DATABASE),
            ("public.bookmark", PUBLIC_BOOKMARK),
            ("public.archive", PUBLIC_ARCHIVE),
            ("com.apple.application", COM_APPLE_APPLICATION),
            ("public.unix-executable", PUBLIC_UNIX_EXECUTABLE),
            ("public.executable", PUBLIC_EXECUTABLE),
            ("public.socket", PUBLIC_SOCKET),
            ("public.block-special", PUBLIC_BLOCK_SPECIAL),
            ("public.character-special", PUBLIC_CHARACTER_SPECIAL),
            ("public.named-pipe", PUBLIC_NAMED_PIPE),
            ("public.composite-content", PUBLIC_COMPOSITE_CONTENT),
            ("public.content", PUBLIC_CONTENT),
            ("public.directory", PUBLIC_DIRECTORY),
            ("public.data", PUBLIC_DATA),
            ("public.item", PUBLIC_ITEM)
        ])));
    pub(crate) static ref SYSTEM_FILENAME_EXTENSION_MAP: Arc<RwLock<HashMap<&'static str, &'static str>>> =
        Arc::new(RwLock::new(HashMap::from([
            (r#"ipa"#, "com.apple.iphone.package-archive"),
            (r#"apk"#, "vnd.android.package-archive"),
            (r#"rs"#, "public.rust-source"),
            (r#"watchface"#, "com.apple.watchface"),
            (r#"pkpasses"#, "com.apple.pkpasses-data"),
            (r#"pkpass"#, "com.apple.pkpass-data"),
            (r#"pkpass"#, "com.apple.pkpass"),
            (r#"mobileprovision"#, "com.apple.mobileprovision"),
            (r#"configprofile"#, "com.apple.configprofile"),
            (r#"provisionprofile"#, "com.apple.provisionprofile"),
            (r#"mobile"#, "com.apple.mobileconfig"),
            (r#"mobileconfig"#, "com.apple.mobileconfig"),
            (r#"ccl"#, "com.apple.color-file"),
            (r#"clr"#, "com.apple.color-file"),
            (r#"pf"#, "com.apple.colorsync-profile"),
            (r#"icm"#, "com.apple.colorsync-profile"),
            (r#"icc"#, "com.apple.colorsync-profile"),
            (r#"aaf"#, "org.aafassociation.advanced-authoring-format"),
            (r#"lpdf"#, "com.apple.localized-pdf-bundle"),
            (r#"epub"#, "org.idpf.epub-container"),
            (r#"webarchive"#, "com.apple.webarchive"),
            (r#"pem"#, "public.x509-certificate"),
            (r#"crt"#, "public.x509-certificate"),
            (r#"der"#, "public.x509-certificate"),
            (r#"cer"#, "public.x509-certificate"),
            (r#"pfx"#, "com.rsa.pkcs-12"),
            (r#"p12"#, "com.rsa.pkcs-12"),
            (r#"fs"#, "com.apple.file-system-plug-in"),
            (r#"ppp"#, "com.apple.ppp-plug-in"),
            (r#"systemextension"#, "com.apple.system-extension"),
            (r#"dext"#, "com.apple.driver-extension"),
            (r#"wdgt"#, "com.apple.dashboard-widget"),
            (r#"qlgenerator"#, "com.apple.quicklook-generator"),
            (r#"mdimporter"#, "com.apple.metadata-importer"),
            (r#"webplugin"#, "com.apple.webkit-plugin"),
            (r#"pluginkit"#, "com.apple.pluginkit"),
            (r#"appex"#, "com.apple.application-and-system-extension"),
            (r#"kext"#, "com.apple.kernel-extension"),
            (r#"xpc"#, "com.apple.xpc-service"),
            (r#"plugin"#, "com.apple.plugin"),
            (r#"service"#, "com.apple.service-application"),
            (r#"placeholder"#, "com.apple.application-placeholder"),
            (r#"rtfd"#, "com.apple.rtfd"),
            (
                r#"suggestionsassets"#,
                "com.apple.intelligentsuggestions.assets"
            ),
            (r#"mpkg"#, "com.apple.installer-meta-package"),
            (r#"distz"#, "com.apple.installer-distribution-package"),
            (r#"dist"#, "com.apple.installer-distribution-package"),
            (r#"pkg"#, "com.apple.installer-package"),
            (r#"app"#, "com.apple.application-file"),
            (r#"app"#, "com.apple.application-bundle"),
            (r#"framework"#, "com.apple.framework"),
            (r#"menu"#, "com.apple.menu-extra"),
            (
                r#"slideSaver"#,
                "com.apple.systempreference.screen-slide-saver"
            ),
            (r#"saver"#, "com.apple.systempreference.screen-saver"),
            (r#"prefPane"#, "com.apple.systempreference.prefpane"),
            (r#"bundle"#, "com.apple.generic-bundle"),
            (r#"sdef"#, "com.apple.scripting-definition"),
            (r#"scptd"#, "com.apple.applescript.script-bundle"),
            (r#"scpt"#, "com.apple.applescript.script"),
            (r#"applescript"#, "com.apple.applescript.text"),
            (r#"pfa"#, "com.adobe.postscript-pfa-font"),
            (r#"pfb"#, "com.adobe.postscript-pfb-font"),
            (r#"suit"#, "com.apple.font-suitcase"),
            (r#"ttc"#, "public.truetype-collection-font"),
            (r#"ttf"#, "public.truetype-ttf-font"),
            (r#"otc"#, "public.opentype-collection-font"),
            (r#"otf"#, "public.opentype-font"),
            (r#"dfont"#, "com.apple.truetype-datafork-suitcase-font"),
            (r#"w64"#, "com.sony.wave64"),
            (r#"aax"#, "com.audible.aax-audiobook"),
            (r#"aa"#, "com.audible.aa-audiobook"),
            (r#"adts"#, "public.aac-audio"),
            (r#"aac"#, "public.aac-audio"),
            (r#"amr"#, "org.3gpp.adaptive-multi-rate-audio"),
            (r#"ec3"#, "public.enhanced-ac3-audio"),
            (r#"eac3"#, "public.enhanced-ac3-audio"),
            (r#"ac3"#, "public.ac3-audio"),
            (r#"caf"#, "com.apple.coreaudio-format"),
            (r#"dls"#, "public.downloadable-sound"),
            (r#"kar"#, "public.midi-audio"),
            (r#"smf"#, "public.midi-audio"),
            (r#"mid"#, "public.midi-audio"),
            (r#"midi"#, "public.midi-audio"),
            (r#"cdda"#, "public.cdda-audio"),
            (r#"aif"#, "public.aiff-audio"),
            (r#"aiff"#, "public.aiff-audio"),
            (r#"aifc"#, "public.aifc-audio"),
            (r#"snd"#, "public.au-audio"),
            (r#"au"#, "public.au-audio"),
            (r#"ulaw"#, "public.ulaw-audio"),
            (r#"ulw"#, "public.ulaw-audio"),
            (r#"ul"#, "public.ulaw-audio"),
            (r#"m4b"#, "com.apple.protected-mpeg-4-audio-b"),
            (r#"m4p"#, "com.apple.protected-mpeg-4-audio"),
            (r#"m4r"#, "com.apple.mpeg-4-ringtone"),
            (r#"m4a"#, "com.apple.m4a-audio"),
            (r#"mpg4"#, "public.mpeg-4-audio"),
            (r#"mp4"#, "public.mpeg-4-audio"),
            (r#"pls"#, "public.pls-playlist"),
            (r#"m3u8"#, "public.m3u-playlist"),
            (r#"m3u"#, "public.m3u-playlist"),
            (r#"mpga"#, "public.mp3"),
            (r#"mp3"#, "public.mp3"),
            (r#"mp2"#, "public.mp2"),
            (r#"aupreset"#, "com.apple.audio-unit-preset"),
            (r#"avchd"#, "public.avchd-collection"),
            (r#"ts"#, "public.mpeg-2-transport-stream"),
            (r#"cel"#, "public.flc-animation"),
            (r#"fli"#, "public.flc-animation"),
            (r#"flc"#, "public.flc-animation"),
            (r#"3gp2"#, "public.3gpp2"),
            (r#"3g2"#, "public.3gpp2"),
            (r#"sdv"#, "public.3gpp"),
            (r#"3gpp"#, "public.3gpp"),
            (r#"3gp"#, "public.3gpp"),
            (r#"vfw"#, "public.avi"),
            (r#"avi"#, "public.avi"),
            (r#"dif"#, "public.dv-movie"),
            (r#"dv"#, "public.dv-movie"),
            (r#"m4v"#, "com.apple.m4v-video"),
            (r#"mpg4"#, "public.mpeg-4"),
            (r#"mp4"#, "public.mpeg-4"),
            (r#"m2v"#, "public.mpeg-2-video"),
            (r#"m15"#, "public.mpeg"),
            (r#"m75"#, "public.mpeg"),
            (r#"mpe"#, "public.mpeg"),
            (r#"mpeg"#, "public.mpeg"),
            (r#"mpg"#, "public.mpeg"),
            (r#"qt"#, "com.apple.quicktime-movie"),
            (r#"mov"#, "com.apple.quicktime-movie"),
            (r#"nii"#, "gov.nih.nifti-1"),
            (r#"dicom"#, "org.nema.dicom"),
            (r#"dcm"#, "org.nema.dicom"),
            (r#"minc"#, "ca.mcgill.mni.bic.mnc"),
            (r#"mnc"#, "ca.mcgill.mni.bic.mnc"),
            (r#"mpo"#, "public.mpo-image"),
            (r#"xbm"#, "public.xbitmap-image"),
            (r#"icns"#, "com.apple.icns"),
            (r#"qti"#, "com.apple.quicktime-image"),
            (r#"qtif"#, "com.apple.quicktime-image"),
            (r#"svgz"#, "public.svg-image"),
            (r#"svg"#, "public.svg-image"),
            (r#"png"#, "public.png"),
            (r#"pntg"#, "com.apple.macpaint-image"),
            (r#"pic"#, "com.apple.pict"),
            (r#"pct"#, "com.apple.pict"),
            (r#"pict"#, "com.apple.pict"),
            (r#"tif"#, "public.tiff"),
            (r#"tiff"#, "public.tiff"),
            (r#"j2c"#, "public.jpeg-2000"),
            (r#"j2k"#, "public.jpeg-2000"),
            (r#"jpx"#, "public.jpeg-2000"),
            (r#"jpf"#, "public.jpeg-2000"),
            (r#"jp2"#, "public.jpeg-2000"),
            (r#"jpe"#, "public.jpeg"),
            (r#"jpg"#, "public.jpeg"),
            (r#"jpeg"#, "public.jpeg"),
            (r#"pvt"#, "com.apple.private.live-photo-bundle"),
            (r#"mk"#, "public.make-source"),
            (r#"gmk"#, "public.make-source"),
            (r#"mak"#, "public.make-source"),
            (r#"make"#, "public.make-source"),
            (r#"jnlp"#, "com.sun.java-web-start"),
            (r#"phtml"#, "public.php-script"),
            (r#"ph4"#, "public.php-script"),
            (r#"ph3"#, "public.php-script"),
            (r#"php4"#, "public.php-script"),
            (r#"php3"#, "public.php-script"),
            (r#"php"#, "public.php-script"),
            (r#"rbw"#, "public.ruby-script"),
            (r#"rb"#, "public.ruby-script"),
            (r#"py"#, "public.python-script"),
            (r#"pm"#, "public.perl-script"),
            (r#"pl"#, "public.perl-script"),
            (r#"zsh"#, "public.zsh-script"),
            (r#"tcsh"#, "public.tcsh-script"),
            (r#"ksh"#, "public.ksh-script"),
            (r#"csh"#, "public.csh-script"),
            (r#"bash"#, "public.bash-script"),
            (r#"sh"#, "public.shell-script"),
            (r#"dsym"#, "com.apple.xcode.dsym"),
            (r#"mjs"#, "com.netscape.javascript-source"),
            (r#"javascript"#, "com.netscape.javascript-source"),
            (r#"jscript"#, "com.netscape.javascript-source"),
            (r#"js"#, "com.netscape.javascript-source"),
            (r#"lid"#, "public.dylan-source"),
            (r#"dlyan"#, "public.dylan-source"),
            (r#"ads"#, "public.ada-source"),
            (r#"adb"#, "public.ada-source"),
            (r#"ada"#, "public.ada-source"),
            (r#"pas"#, "public.pascal-source"),
            (r#"f95"#, "public.fortran-95-source"),
            (r#"f90"#, "public.fortran-90-source"),
            (r#"f77"#, "public.fortran-77-source"),
            (r#"for"#, "public.fortran-source"),
            (r#"f"#, "public.fortran-source"),
            (r#"exp"#, "com.apple.symbol-export"),
            (r#"mig"#, "public.mig-source"),
            (r#"defs"#, "public.mig-source"),
            (r#"yy"#, "public.yacc-source"),
            (r#"yxx"#, "public.yacc-source"),
            (r#"ypp"#, "public.yacc-source"),
            (r#"ymm"#, "public.yacc-source"),
            (r#"ym"#, "public.yacc-source"),
            (r#"y"#, "public.yacc-source"),
            (r#"ll"#, "public.lex-source"),
            (r#"lxx"#, "public.lex-source"),
            (r#"lpp"#, "public.lex-source"),
            (r#"lmm"#, "public.lex-source"),
            (r#"lm"#, "public.lex-source"),
            (r#"l"#, "public.lex-source"),
            (r#"r"#, "com.apple.rez-source"),
            (r#"s"#, "public.assembly-source"),
            (r#"jav"#, "com.sun.java-source"),
            (r#"java"#, "com.sun.java-source"),
            (r#"swift"#, "public.swift-source"),
            (r#"pch++"#, "public.precompiled-c-plus-plus-header"),
            (r#"inl"#, "public.c-plus-plus-inline-header"),
            (r#"ipp"#, "public.c-plus-plus-header"),
            (r#"h++"#, "public.c-plus-plus-header"),
            (r#"hxx"#, "public.c-plus-plus-header"),
            (r#"hpp"#, "public.c-plus-plus-header"),
            (r#"hp"#, "public.c-plus-plus-header"),
            (r#"hh"#, "public.c-plus-plus-header"),
            (r#"pch"#, "public.precompiled-c-header"),
            (r#"h"#, "public.c-header"),
            (r#"mii"#, "public.objective-c-plus-plus-source.preprocessed"),
            (r#"mm"#, "public.objective-c-plus-plus-source"),
            (r#"ii"#, "public.c-plus-plus-source.preprocessed"),
            (r#"cxx"#, "public.c-plus-plus-source"),
            (r#"cc"#, "public.c-plus-plus-source"),
            (r#"c++"#, "public.c-plus-plus-source"),
            (r#"cpp"#, "public.c-plus-plus-source"),
            (r#"cp"#, "public.c-plus-plus-source"),
            (r#"mi"#, "public.objective-c-source.preprocessed"),
            (r#"m"#, "public.objective-c-source"),
            (r#"iig"#, "com.apple.iig-source"),
            (r#"i"#, "public.c-source.preprocessed"),
            (r#"c"#, "public.c-source"),
            (r#"plist"#, "com.apple.ascii-property-list"),
            (r#"plist"#, "com.apple.binary-property-list"),
            (r#"plist"#, "com.apple.xml-property-list"),
            (r#"plist"#, "com.apple.property-list"),
            (r#"vtt"#, "org.w3.webvtt"),
            (r#"scc"#, "com.scenarist.closed-caption"),
            (r#"yaml"#, "public.yaml"),
            (r#"yml"#, "public.yaml"),
            (r#"ndjson"#, "public.ndjson"),
            (r#"json"#, "public.json"),
            (r#"diff"#, "public.patch-file"),
            (r#"patch"#, "public.patch-file"),
            (r#"css"#, "public.css"),
            (r#"xfd"#, "public.xfd"),
            (r#"rss"#, "public.rss"),
            (r#"xht"#, "public.xhtml"),
            (r#"xhtm"#, "public.xhtml"),
            (r#"xhtml"#, "public.xhtml"),
            (r#"xml"#, "public.xml"),
            (r#"shtm"#, "public.html"),
            (r#"shtml"#, "public.html"),
            (r#"htm"#, "public.html"),
            (r#"html"#, "public.html"),
            (r#"rtf"#, "public.rtf"),
            (r#"tsv"#, "public.tab-separated-values-text"),
            (r#"csv"#, "public.comma-separated-values-text"),
            (r#"itms"#, "com.apple.itunes.store-url"),
            (r#"url"#, "com.microsoft.internet-shortcut"),
            (r#"inetloc"#, "com.apple.generic-internet-location"),
            (r#"newsloc"#, "com.apple.news-internet-location"),
            (r#"ftploc"#, "com.apple.ftp-internet-location"),
            (r#"fileloc"#, "com.apple.file-internet-location"),
            (r#"afploc"#, "com.apple.afp-internet-location"),
            (r#"mailloc"#, "com.apple.mail-internet-location"),
            (r#"vncloc"#, "com.apple.vnc-internet-location"),
            (r#"webloc"#, "com.apple.web-internet-location"),
            (r#"aea"#, "com.apple.encrypted-archive"),
            (r#"yaa"#, "com.apple.archive"),
            (r#"aar"#, "com.apple.archive"),
            (r#"mpkg"#, "com.apple.installer-package-archive"),
            (r#"pkg"#, "com.apple.installer-package-archive"),
            (r#"xip"#, "com.apple.xip-archive"),
            (r#"xar"#, "com.apple.xar-archive"),
            (r#"zip"#, "public.zip-archive"),
            (r#"pax"#, "public.cpio-archive"),
            (r#"cpio"#, "public.cpio-archive"),
            (r#"cpgz"#, "com.apple.bom-compressed-cpio"),
            (r#"z"#, "public.z-archive"),
            (r#"uu"#, "public.uuencoded-archive"),
            (r#"as"#, "com.apple.applesingle-archive"),
            (r#"bin"#, "com.apple.macbinary-archive"),
            (r#"hqx"#, "com.apple.binhex-archive"),
            (r#"tbz"#, "public.tar-bzip2-archive"),
            (r#"tbz2"#, "public.tar-bzip2-archive"),
            (r#"bz"#, "public.bzip2-archive"),
            (r#"bz2"#, "public.bzip2-archive"),
            (r#"tgz"#, "org.gnu.gnu-zip-tar-archive"),
            (r#"gzip"#, "org.gnu.gnu-zip-archive"),
            (r#"gz"#, "org.gnu.gnu-zip-archive"),
            (r#"tar"#, "public.tar-archive"),
            (r#"gtar"#, "org.gnu.gnu-tar-archive"),
            (r#"qtz"#, "com.apple.quartz-composer-composition"),
            (r#"war"#, "com.sun.web-application-archive"),
            (r#"jar"#, "com.sun.java-archive"),
            (r#"class"#, "com.sun.java-class"),
            (r#"dll"#, "com.microsoft.windows-dynamic-link-library"),
            (r#"exe"#, "com.microsoft.windows-executable"),
            (r#"dylib"#, "com.apple.mach-o-dylib"),
            (r#"o"#, "public.object-code"),
            (r#"savedSearch"#, "com.apple.finder.smart-folder"),
            (r#"iconset"#, "com.apple.iconset"),
            (r#"fpbf"#, "com.apple.finder.burn-folder"),
            (r#"pictclipping"#, "com.apple.finder.pictclipping"),
            (r#"textclipping"#, "com.apple.finder.textclipping"),
            (r#"sndClipping"#, "com.apple.finder.sound-clipping"),
            (r#"icloud"#, "com.apple.icloud-file-fault"),
            (r#"ktrace"#, "com.apple.ktrace"),
            (r#"panic"#, "com.apple.panicreport"),
            (r#"spin"#, "com.apple.spinreport"),
            (r#"hang"#, "com.apple.hangreport"),
            (r#"crash"#, "com.apple.crashreport"),
            (r#"gpuRestart"#, "com.apple.gpuRestart"),
            (r#"shutdownStall"#, "com.apple.shutdownStall"),
            (r#"log"#, "com.apple.log"),
            (r#"text"#, "public.plain-text"),
            (r#"txt"#, "public.plain-text"),
            (r#"shazamcatalog"#, "com.apple.shazamcatalog"),
            (r#"shazamsignature"#, "com.apple.shazamsignature"),
            (r#"vcard"#, "public.vcard"),
            (r#"vcf"#, "public.vcard"),
            (r#"ics"#, "com.apple.ical.ics"),
            (r#"vcal"#, "com.apple.ical.vcs"),
            (r#"vcs"#, "com.apple.ical.vcs"),
            (r#"arobject"#, "com.apple.arobject"),
            (r#"scnz"#, "com.apple.scenekit.scene"),
            (r#"scn"#, "com.apple.scenekit.scene"),
            (r#"reality"#, "com.apple.reality"),
            (r#"usdz"#, "com.pixar.universal-scene-description-mobile"),
            (r#"usdc"#, "com.pixar.universal-scene-description"),
            (r#"usda"#, "com.pixar.universal-scene-description"),
            (r#"usd"#, "com.pixar.universal-scene-description"),
            (r#"ply"#, "public.polygon-file-format"),
            (r#"stl"#, "public.standard-tesselated-geometry-format"),
            (r#"obj"#, "public.geometry-definition-format"),
            (r#"abc"#, "public.alembic"),
            (r#"csstore"#, "com.apple.csstore")
        ])));
    pub(crate) static ref SYSTEM_MIME_MAP: Arc<RwLock<HashMap<&'static str, &'static str>>> =
        Arc::new(RwLock::new(HashMap::from([
            (r#"application/octet-stream"#, "public.data"),
            (
                r#"model/vnd.usdz+zip"#,
                "com.pixar.universal-scene-description-mobile"
            ),
            (r#"model/vnd.reality"#, "com.apple.reality"),
            (r#"text/x-vcalendar"#, "com.apple.ical.vcs"),
            (r#"text/calendar"#, "com.apple.ical.ics"),
            (r#"text/vcard"#, "public.vcard"),
            (r#"text/directory"#, "public.vcard"),
            (r#"text/x-vcard"#, "public.vcard"),
            (r#"text/plain"#, "public.plain-text"),
            (r#"text/plain;charset=utf-8"#, "public.utf8-plain-text"),
            (r#"text/plain;charset="utf-8""#, "public.utf8-plain-text"),
            (r#"text/plain;charset=utf-16"#, "public.utf16-plain-text"),
            (r#"text/plain;charset="utf-16""#, "public.utf16-plain-text"),
            (
                r#"application/x-msdownload"#,
                "com.microsoft.windows-executable"
            ),
            (
                r#"application/x-msdownload"#,
                "com.microsoft.windows-dynamic-link-library"
            ),
            (r#"application/java-archive"#, "com.sun.java-archive"),
            (
                r#"application/x-quartzcomposer"#,
                "com.apple.quartz-composer-composition"
            ),
            (r#"application/x-gtar"#, "org.gnu.gnu-tar-archive"),
            (r#"application/x-tar"#, "public.tar-archive"),
            (r#"application/tar"#, "public.tar-archive"),
            (r#"application/x-gzip"#, "org.gnu.gnu-zip-archive"),
            (r#"application/gzip"#, "org.gnu.gnu-zip-archive"),
            (r#"application/x-bzip2"#, "public.bzip2-archive"),
            (r#"application/x-bzip"#, "public.bzip2-archive"),
            (r#"application/bzip2"#, "public.bzip2-archive"),
            (r#"application/bzip"#, "public.bzip2-archive"),
            (r#"application/x-bz2"#, "public.bzip2-archive"),
            (r#"application/mac-binhex40"#, "com.apple.binhex-archive"),
            (r#"application/mac-binhex"#, "com.apple.binhex-archive"),
            (r#"application/binhex"#, "com.apple.binhex-archive"),
            (r#"application/macbinary"#, "com.apple.macbinary-archive"),
            (r#"application/x-macbinary"#, "com.apple.macbinary-archive"),
            (r#"text/x-uuencode"#, "public.uuencoded-archive"),
            (r#"application/x-compress"#, "public.z-archive"),
            (r#"application/zip"#, "public.zip-archive"),
            (r#"application/x-zip-compressed"#, "public.zip-archive"),
            (r#"text/csv"#, "public.comma-separated-values-text"),
            (
                r#"text/comma-separated-values"#,
                "public.comma-separated-values-text"
            ),
            (
                r#"text/tab-separated-values"#,
                "public.tab-separated-values-text"
            ),
            (r#"text/rtf"#, "public.rtf"),
            (r#"text/html"#, "public.html"),
            (r#"application/xml"#, "public.xml"),
            (r#"text/xml"#, "public.xml"),
            (r#"application/xhtml+xml"#, "public.xhtml"),
            (r#"application/rss+xml"#, "public.rss"),
            (r#"text/css"#, "public.css"),
            (r#"application/json"#, "public.json"),
            (r#"application/x-ndjson"#, "public.ndjson"),
            (r#"application/x-yaml"#, "public.yaml"),
            (r#"text/vtt"#, "org.w3.webvtt"),
            (r#"text/javascript"#, "com.netscape.javascript-source"),
            (r#"text/x-perl-script"#, "public.perl-script"),
            (r#"text/x-python-script"#, "public.python-script"),
            (r#"text/x-ruby-script"#, "public.ruby-script"),
            (r#"text/php"#, "public.php-script"),
            (r#"text/x-php-script"#, "public.php-script"),
            (r#"application/php"#, "public.php-script"),
            (r#"application/x-java-jnlp-file"#, "com.sun.java-web-start"),
            (r#"application/jnlp"#, "com.sun.java-web-start"),
            (r#"image/jpeg"#, "public.jpeg"),
            (r#"image/jpg"#, "public.jpeg"),
            (r#"image/jp2"#, "public.jpeg-2000"),
            (r#"image/tiff"#, "public.tiff"),
            (r#"image/pict"#, "com.apple.pict"),
            (r#"image/x-pict"#, "com.apple.pict"),
            (r#"image/x-macpict"#, "com.apple.pict"),
            (r#"image/png"#, "public.png"),
            (r#"image/svg+xml"#, "public.svg-image"),
            (r#"image/x-quicktime"#, "com.apple.quicktime-image"),
            (r#"image/x-xbitmap"#, "public.xbitmap-image"),
            (r#"application/dicom"#, "org.nema.dicom"),
            (r#"video/quicktime"#, "com.apple.quicktime-movie"),
            (r#"video/mpeg"#, "public.mpeg"),
            (r#"video/mpg"#, "public.mpeg"),
            (r#"video/x-mpeg"#, "public.mpeg"),
            (r#"video/x-mpg"#, "public.mpeg"),
            (r#"video/mpeg2"#, "public.mpeg-2-video"),
            (r#"video/mpeg2-video"#, "public.mpeg-2-video"),
            (r#"video/mp4"#, "public.mpeg-4"),
            (r#"video/mp4v-es"#, "public.mpeg-4"),
            (r#"video/x-m4v"#, "com.apple.m4v-video"),
            (r#"video/x-dv"#, "public.dv-movie"),
            (r#"video/avi"#, "public.avi"),
            (r#"video/msvideo"#, "public.avi"),
            (r#"video/x-msvideo"#, "public.avi"),
            (r#"video/3gpp"#, "public.3gpp"),
            (r#"audio/3gpp"#, "public.3gpp"),
            (r#"video/3gpp2"#, "public.3gpp2"),
            (r#"audio/3gpp2"#, "public.3gpp2"),
            (r#"video/flc"#, "public.flc-animation"),
            (r#"audio/mpeg"#, "public.mp3"),
            (r#"audio/mpeg3"#, "public.mp3"),
            (r#"audio/mpg"#, "public.mp3"),
            (r#"audio/mp3"#, "public.mp3"),
            (r#"audio/x-mpeg"#, "public.mp3"),
            (r#"audio/x-mpeg3"#, "public.mp3"),
            (r#"audio/x-mpg"#, "public.mp3"),
            (r#"audio/x-mp3"#, "public.mp3"),
            (r#"audio/mpegurl"#, "public.m3u-playlist"),
            (r#"application/vnd.apple.mpegurl"#, "public.m3u-playlist"),
            (r#"audio/x-mpegurl"#, "public.m3u-playlist"),
            (r#"audio/x-scpls"#, "public.pls-playlist"),
            (r#"audio/mp4"#, "public.mpeg-4-audio"),
            (r#"audio/mp4a-latm"#, "public.mpeg-4-audio"),
            (r#"audio/x-m4a"#, "com.apple.m4a-audio"),
            (r#"audio/x-m4r"#, "com.apple.mpeg-4-ringtone"),
            (r#"audio/basic"#, "public.au-audio"),
            (r#"audio/aiff"#, "public.aiff-audio"),
            (r#"audio/x-aiff"#, "public.aiff-audio"),
            (r#"audio/midi"#, "public.midi-audio"),
            (r#"audio/x-midi"#, "public.midi-audio"),
            (r#"audio/dls"#, "public.downloadable-sound"),
            (r#"audio/ac3"#, "public.ac3-audio"),
            (r#"audio/eac3"#, "public.enhanced-ac3-audio"),
            (r#"audio/amr"#, "org.3gpp.adaptive-multi-rate-audio"),
            (r#"audio/aac"#, "public.aac-audio"),
            (r#"audio/x-aac"#, "public.aac-audio"),
            (r#"audio/audible"#, "com.audible.aa-audiobook"),
            (r#"audio/x-pn-audibleaudio"#, "com.audible.aa-audiobook"),
            (r#"audio/x-audible"#, "com.audible.aa-audiobook"),
            (r#"audio/vnd.audible.aax"#, "com.audible.aax-audiobook"),
            (r#"font/otf"#, "public.opentype-font"),
            (r#"font/ttf"#, "public.truetype-ttf-font"),
            (r#"application/x-pkcs12"#, "com.rsa.pkcs-12"),
            (r#"application/x-x509-ca-cert"#, "public.x509-certificate"),
            (r#"application/x-webarchive"#, "com.apple.webarchive"),
            (r#"application/epub+zip"#, "org.idpf.epub-container"),
            (
                r#"application/x-apple-aspen-config"#,
                "com.apple.mobileconfig"
            ),
            (
                r#"application/x-apple-aspen-mobileprovision"#,
                "com.apple.mobileprovision"
            ),
            (r#"application/vnd.apple.pkpass"#, "com.apple.pkpass"),
            (
                r#"application/vnd.apple.pkpasses"#,
                "com.apple.pkpasses-data"
            )
        ])));
}
pub const COM_ADOBE_PDF: UTType = UTType {
    identifier: "com.adobe.pdf",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["pdf"], "public.mime-type": ["application/pdf"]}"#,
    description: r#"PDF document"#,
};
pub const COM_ADOBE_EDN: UTType = UTType {
    identifier: "com.adobe.edn",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["edn"]}"#,
    description: r#"Adobe DRM Activation Key (EDN)"#,
};
pub const COM_ADOBE_ETD: UTType = UTType {
    identifier: "com.adobe.etd",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["etd"]}"#,
    description: r#"Adobe Digital Editions (ETD)"#,
};
pub const COM_ADOBE_XFDF: UTType = UTType {
    identifier: "com.adobe.xfdf",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["xfdf"]}"#,
    description: r#"Adobe Acrobat Forms Document (XFDF)"#,
};
pub const COM_ADOBE_FDF: UTType = UTType {
    identifier: "com.adobe.fdf",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["fdf"]}"#,
    description: r#"Adobe Acrobat Forms Document (FDF)"#,
};
pub const COM_ADOBE_POSTSCRIPT: UTType = UTType {
    identifier: "com.adobe.postscript",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["ps"], "public.mime-type": ["application/postscript"]}"#,
    description: r#"PostScript"#,
};
pub const COM_ADOBE_ENCAPSULATED_POSTSCRIPT: UTType = UTType {
    identifier: "com.adobe.encapsulated-postscript",
    conforms_to: r#"["com.adobe.postscript"]"#,
    tags: r#"{"public.filename-extension": ["eps"]}"#,
    description: r#"Encapsulated PostScript"#,
};
pub const COM_COMPUSERVE_GIF: UTType = UTType {
    identifier: "com.compuserve.gif",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["gif"], "public.mime-type": ["image/gif"]}"#,
    description: r#"GIF image"#,
};
pub const COM_MICROSOFT_BMP: UTType = UTType {
    identifier: "com.microsoft.bmp",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["bmp", "dib"], "public.mime-type": ["image/bmp"]}"#,
    description: r#"Windows BMP image"#,
};
pub const COM_MICROSOFT_ICO: UTType = UTType {
    identifier: "com.microsoft.ico",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["ico"], "public.mime-type": ["image/vnd.microsoft.icon"]}"#,
    description: r#"Windows icon image"#,
};
pub const ORG_WEBMPROJECT_WEBP: UTType = UTType {
    identifier: "org.webmproject.webp",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["webp"], "public.mime-type": ["image/webp"]}"#,
    description: r#"WebP Image"#,
};
pub const ORG_WEBMPROJECT_WEBM: UTType = UTType {
    identifier: "org.webmproject.webm",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["webm"], "public.mime-type": ["video/webm", "audio/webm"]}"#,
    description: r#"WebM Media"#,
};
pub const PUBLIC_OFD: UTType = UTType {
    identifier: "public.ofd",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["ofd"], "public.mime-type": ["application/ofd"]}"#,
    description: r#"Open Fixed-layout Document"#,
};
pub const ORG_OPENOFFICE_TEXT: UTType = UTType {
    identifier: "org.openoffice.text",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["sxw", "sdw"], "public.mime-type": ["application/vnd.sun.xml.writer", "application/vnd.stardivision.writer"]}"#,
    description: r#"OpenOffice.org 1.0 Text"#,
};
pub const ORG_OPENOFFICE_TEXT_TEMPLATE: UTType = UTType {
    identifier: "org.openoffice.text-template",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["stw"], "public.mime-type": ["application/vnd.sun.xml.writer.template"]}"#,
    description: r#"OpenOffice.org 1.0 Text Template"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_TEXT: UTType = UTType {
    identifier: "org.oasis-open.opendocument.text",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["odt"], "public.mime-type": ["application/vnd.oasis.opendocument.text"]}"#,
    description: r#"Open Document text"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.text-template",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["ott"], "public.mime-type": ["application/vnd.oasis.opendocument.text-template"]}"#,
    description: r#"Open Document text template"#,
};
pub const ORG_OPENOFFICE_GRAPHICS: UTType = UTType {
    identifier: "org.openoffice.graphics",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["sxd", "sda"], "public.mime-type": ["application/vnd.sun.xml.draw", "application/vnd.stardivision.draw"]}"#,
    description: r#"OpenOffice.org 1.0 Drawing"#,
};
pub const ORG_OPENOFFICE_GRAPHICS_TEMPLATE: UTType = UTType {
    identifier: "org.openoffice.graphics-template",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["std"], "public.mime-type": ["application/vnd.sun.xml.draw.template"]}"#,
    description: r#"OpenOffice.org 1.0 Drawing Template"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_GRAPHICS: UTType = UTType {
    identifier: "org.oasis-open.opendocument.graphics",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["odg"], "public.mime-type": ["application/vnd.oasis.opendocument.graphics"]}"#,
    description: r#"Open Document graphics"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_GRAPHICS_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.graphics-template",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["otg"], "public.mime-type": ["application/vnd.oasis.opendocument.graphics-template"]}"#,
    description: r#"Open Document graphics template"#,
};
pub const ORG_OPENOFFICE_PRESENTATION: UTType = UTType {
    identifier: "org.openoffice.presentation",
    conforms_to: r#"["public.data", "public.content", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["sxi", "sdd", "sdp"], "public.mime-type": ["application/vnd.sun.xml.impress", "application/vnd.stardivision.impress", "application/vnd.stardivision.impress-packed"]}"#,
    description: r#"OpenOffice.org 1.0 Presentation"#,
};
pub const ORG_OPENOFFICE_PRESENTATION_TEMPLATE: UTType = UTType {
    identifier: "org.openoffice.presentation-template",
    conforms_to: r#"["public.data", "public.content", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["sti"], "public.mime-type": ["application/vnd.sun.xml.impress.template"]}"#,
    description: r#"OpenOffice.org 1.0 Presentation Template"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_PRESENTATION: UTType = UTType {
    identifier: "org.oasis-open.opendocument.presentation",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["odp"], "public.mime-type": ["application/vnd.oasis.opendocument.presentation"]}"#,
    description: r#"Open Document presentation"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_PRESENTATION_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.presentation-template",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["otp"], "public.mime-type": ["application/vnd.oasis.opendocument.presentation-template"]}"#,
    description: r#"Open Document presentation template"#,
};
pub const ORG_OPENOFFICE_SPREADSHEET: UTType = UTType {
    identifier: "org.openoffice.spreadsheet",
    conforms_to: r#"["public.data", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["sxc", "sdc"], "public.mime-type": ["application/vnd.sun.xml.calc", "application/vnd.stardivision.calc"]}"#,
    description: r#"OpenOffice.org 1.0 Spreadsheet"#,
};
pub const ORG_OPENOFFICE_SPREADSHEET_TEMPLATE: UTType = UTType {
    identifier: "org.openoffice.spreadsheet-template",
    conforms_to: r#"["public.data", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["stc"], "public.mime-type": ["application/vnd.sun.xml.calc.template"]}"#,
    description: r#"OpenOffice.org 1.0 Spreadsheet Template"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_SPREADSHEET: UTType = UTType {
    identifier: "org.oasis-open.opendocument.spreadsheet",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["ods"], "public.mime-type": ["application/vnd.oasis.opendocument.spreadsheet"]}"#,
    description: r#"Open Document spreadsheet"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_SPREADSHEET_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.spreadsheet-template",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["ots"], "public.mime-type": ["application/vnd.oasis.opendocument.spreadsheet-template"]}"#,
    description: r#"Open Document spreadsheet template"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_CHART: UTType = UTType {
    identifier: "org.oasis-open.opendocument.chart",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["odc"], "public.mime-type": ["application/vnd.oasis.opendocument.chart"]}"#,
    description: r#"Open Document chart"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_CHART_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.chart-template",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["otc"], "public.mime-type": ["application/vnd.oasis.opendocument.chart-template"]}"#,
    description: r#"Open Document chart template"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_IMAGE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.image",
    conforms_to: r#"["org.oasis-open.opendocument", "public.image"]"#,
    tags: r#"{"public.filename-extension": ["odi"], "public.mime-type": ["application/vnd.oasis.opendocument.image"]}"#,
    description: r#"Open Document image"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_IMAGE_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.image-template",
    conforms_to: r#"["org.oasis-open.opendocument", "public.image"]"#,
    tags: r#"{"public.filename-extension": ["oti"], "public.mime-type": ["application/vnd.oasis.opendocument.image-template"]}"#,
    description: r#"Open Document image template"#,
};
pub const ORG_OPENOFFICE_FORMULA: UTType = UTType {
    identifier: "org.openoffice.formula",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["sxm", "smf"], "public.mime-type": ["application/vnd.sun.xml.math", "application/vnd.stardivision.math"]}"#,
    description: r#"OpenOffice.org 1.0 Formula"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_FORMULA: UTType = UTType {
    identifier: "org.oasis-open.opendocument.formula",
    conforms_to: r#"["org.oasis-open.opendocument", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["odf"], "public.mime-type": ["application/vnd.oasis.opendocument.formula"]}"#,
    description: r#"Open Document formula"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_FORMULA_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.formula-template",
    conforms_to: r#"["org.oasis-open.opendocument", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["otf"], "public.mime-type": ["application/vnd.oasis.opendocument.formula-template"]}"#,
    description: r#"Open Document formula template"#,
};
pub const ORG_OPENOFFICE_TEXT_MASTER: UTType = UTType {
    identifier: "org.openoffice.text-master",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["sxg"], "public.mime-type": ["application/vnd.sun.xml.writer.global"]}"#,
    description: r#"OpenOffice.org 1.0 Master"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_MASTER: UTType = UTType {
    identifier: "org.oasis-open.opendocument.text-master",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["odm"], "public.mime-type": ["application/vnd.oasis.opendocument.text-master"]}"#,
    description: r#"Open Document text master"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_WEB: UTType = UTType {
    identifier: "org.oasis-open.opendocument.text-web",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["oth"], "public.mime-type": ["application/vnd.oasis.opendocument.text-web"]}"#,
    description: r#"Open Document HTML template"#,
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_DATABASE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.database",
    conforms_to: r#"["public.data", "public.content", "public.database"]"#,
    tags: r#"{"public.filename-extension": ["odb"], "public.mime-type": ["application/vnd.oasis.opendocument.database"]}"#,
    description: r#"Open Document database"#,
};
pub const COM_MICROSOFT_WORD_WORDML: UTType = UTType {
    identifier: "com.microsoft.word.wordml",
    conforms_to: r#"["public.xml", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["xml"]}"#,
    description: r#"Microsoft Word 2003 XML document"#,
};
pub const ORG_OPENXMLFORMATS_WORDPROCESSINGML_DOCUMENT: UTType = UTType {
    identifier: "org.openxmlformats.wordprocessingml.document",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["docx"], "public.mime-type": ["application/vnd.openxmlformats-officedocument.wordprocessingml.document"]}"#,
    description: r#"Office Open XML word processing document"#,
};
pub const ORG_OPENXMLFORMATS_WORDPROCESSINGML_DOCUMENT_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.wordprocessingml.document.macroenabled",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["docm"], "public.mime-type": ["application/vnd.ms-word.document.macroEnabled.12"]}"#,
    description: r#"Office Open XML word processing document (macros enabled)"#,
};
pub const ORG_OPENXMLFORMATS_WORDPROCESSINGML_TEMPLATE: UTType = UTType {
    identifier: "org.openxmlformats.wordprocessingml.template",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["dotx"], "public.mime-type": ["application/vnd.openxmlformats-officedocument.wordprocessingml.template"]}"#,
    description: r#"Office Open XML word processing template"#,
};
pub const ORG_OPENXMLFORMATS_WORDPROCESSINGML_TEMPLATE_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.wordprocessingml.template.macroenabled",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["dotm"], "public.mime-type": ["application/vnd.ms-word.template.macroEnabled.12"]}"#,
    description: r#"Office Open XML word processing template (macros enabled)"#,
};
pub const ORG_OPENXMLFORMATS_SPREADSHEETML_SHEET: UTType = UTType {
    identifier: "org.openxmlformats.spreadsheetml.sheet",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["xlsx"], "public.mime-type": ["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"]}"#,
    description: r#"Office Open XML spreadsheet"#,
};
pub const ORG_OPENXMLFORMATS_SPREADSHEETML_SHEET_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.spreadsheetml.sheet.macroenabled",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content", "public.spreadsheet", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["xlsm"], "public.mime-type": ["application/vnd.ms-excel.sheet.macroEnabled.12"]}"#,
    description: r#"Office Open XML spreadsheet (macros enabled)"#,
};
pub const COM_MICROSOFT_EXCEL_SHEET_BINARY_MACROENABLED: UTType = UTType {
    identifier: "com.microsoft.excel.sheet.binary.macroenabled",
    conforms_to: r#"["public.zip-archive", "public.composite-content", "public.spreadsheet", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["xlsb"], "public.mime-type": ["application/vnd.ms-excel.sheet.binary.macroEnabled.12"]}"#,
    description: r#"Microsoft Excel 2007 spreadsheet (macros enabled)"#,
};
pub const ORG_OPENXMLFORMATS_SPREADSHEETML_TEMPLATE: UTType = UTType {
    identifier: "org.openxmlformats.spreadsheetml.template",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["xltx"], "public.mime-type": ["application/vnd.openxmlformats-officedocument.spreadsheetml.template"]}"#,
    description: r#"Office Open XML spreadsheet template"#,
};
pub const ORG_OPENXMLFORMATS_SPREADSHEETML_TEMPLATE_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.spreadsheetml.template.macroenabled",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content", "public.spreadsheet", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["xltm"], "public.mime-type": ["application/vnd.ms-excel.template.macroEnabled.12"]}"#,
    description: r#"Office Open XML spreadsheet template (macros enabled)"#,
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_PRESENTATION: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.presentation",
    conforms_to: r#"["org.openxmlformats.openxml", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["pptx"], "public.mime-type": ["application/vnd.openxmlformats-officedocument.presentationml.presentation"]}"#,
    description: r#"Office Open XML presentation"#,
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_PRESENTATION_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.presentation.macroenabled",
    conforms_to: r#"["org.openxmlformats.openxml", "public.presentation", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["pptm"], "public.mime-type": ["application/vnd.ms-powerpoint.presentation.macroEnabled.12"]}"#,
    description: r#"Office Open XML presentation (macros enabled)"#,
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_SLIDESHOW: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.slideshow",
    conforms_to: r#"["org.openxmlformats.openxml", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["ppsx"], "public.mime-type": ["application/vnd.openxmlformats-officedocument.presentationml.slideshow"]}"#,
    description: r#"Office Open XML slide show"#,
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_SLIDESHOW_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.slideshow.macroenabled",
    conforms_to: r#"["org.openxmlformats.openxml", "public.presentation", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["ppsm"], "public.mime-type": ["application/vnd.ms-powerpoint.slideshow.macroEnabled.12"]}"#,
    description: r#"Office Open XML slide show (macros enabled)"#,
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_TEMPLATE: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.template",
    conforms_to: r#"["org.openxmlformats.openxml", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["potx"], "public.mime-type": ["application/vnd.openxmlformats-officedocument.presentationml.template"]}"#,
    description: r#"Office Open XML presentation template"#,
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_TEMPLATE_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.template.macroenabled",
    conforms_to: r#"["org.openxmlformats.openxml", "public.presentation", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["potm"], "public.mime-type": ["application/vnd.ms-powerpoint.template.macroEnabled.12"]}"#,
    description: r#"Office Open XML presentation template (macros enabled)"#,
};
pub const COM_MICROSOFT_WORD_DOC: UTType = UTType {
    identifier: "com.microsoft.word.doc",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["doc"], "public.mime-type": ["application/msword"]}"#,
    description: r#"Microsoft Word 97-2004 document"#,
};
pub const COM_MICROSOFT_WORD_DOT: UTType = UTType {
    identifier: "com.microsoft.word.dot",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["dot"], "public.mime-type": ["application/msword"]}"#,
    description: r#"Microsoft Word 97-2004 template"#,
};
pub const COM_MICROSOFT_EXCEL_XLS: UTType = UTType {
    identifier: "com.microsoft.excel.xls",
    conforms_to: r#"["public.data", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["xls"], "public.mime-type": ["application/vnd.ms-excel", "application/msexcel"]}"#,
    description: r#"Microsoft Excel 97-2004 worksheet"#,
};
pub const COM_MICROSOFT_EXCEL_XLT: UTType = UTType {
    identifier: "com.microsoft.excel.xlt",
    conforms_to: r#"["public.data", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["xlt"], "public.mime-type": ["application/vnd.ms-excel", "application/msexcel"]}"#,
    description: r#"Microsoft Excel 97-2004 template"#,
};
pub const COM_MICROSOFT_EXCEL_XLW: UTType = UTType {
    identifier: "com.microsoft.excel.xlw",
    conforms_to: r#"["public.data", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["xlw"], "public.mime-type": ["application/vnd.ms-excel", "application/msexcel"]}"#,
    description: r#"Microsoft Excel 97-2004 workspace"#,
};
pub const COM_MICROSOFT_EXCEL_XLA: UTType = UTType {
    identifier: "com.microsoft.excel.xla",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["xla"]}"#,
    description: r#"Microsoft Excel add-in"#,
};
pub const COM_MICROSOFT_POWERPOINT_PPT: UTType = UTType {
    identifier: "com.microsoft.powerpoint.ppt",
    conforms_to: r#"["public.data", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["ppt"], "public.mime-type": ["application/vnd.ms-powerpoint", "application/mspowerpoint"]}"#,
    description: r#"Microsoft PowerPoint 97-2004 presentation"#,
};
pub const COM_MICROSOFT_POWERPOINT_POT: UTType = UTType {
    identifier: "com.microsoft.powerpoint.pot",
    conforms_to: r#"["public.data", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["pot"], "public.mime-type": ["application/vnd.ms-powerpoint", "application/mspowerpoint"]}"#,
    description: r#"Microsoft PowerPoint 97-2004 template"#,
};
pub const COM_MICROSOFT_POWERPOINT_PPS: UTType = UTType {
    identifier: "com.microsoft.powerpoint.pps",
    conforms_to: r#"["public.data", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["pps"], "public.mime-type": ["application/vnd.ms-powerpoint", "application/mspowerpoint"]}"#,
    description: r#"Microsoft PowerPoint 97-2004 slide show"#,
};
pub const COM_APPLE_KEYNOTE_KEY: UTType = UTType {
    identifier: "com.apple.keynote.key",
    conforms_to: r#"["com.apple.package", "public.presentation"]"#,
    tags: r#"{}"#,
    description: r#"Keynote document"#,
};
pub const COM_APPLE_KEYNOTE_KTH: UTType = UTType {
    identifier: "com.apple.keynote.kth",
    conforms_to: r#"["com.apple.package", "public.presentation"]"#,
    tags: r#"{}"#,
    description: r#"Keynote theme"#,
};
pub const COM_APPLE_IWORK_KEYNOTE_KEY: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.key",
    conforms_to: r#"["com.apple.package", "public.presentation", "com.apple.keynote.key"]"#,
    tags: r#"{"public.filename-extension": ["key"]}"#,
    description: r#"Keynote document"#,
};
pub const COM_APPLE_IWORK_KEYNOTE_KEY_TEF: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.key-tef",
    conforms_to: r#"["com.apple.package", "public.presentation", "com.apple.keynote.key"]"#,
    tags: r#"{"public.filename-extension": ["key-tef"]}"#,
    description: r#"Keynote document"#,
};
pub const COM_APPLE_IWORK_KEYNOTE_SFFKEY: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.sffkey",
    conforms_to: r#"["public.data", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["key"], "public.mime-type": ["application/x-iwork-keynote-sffkey"]}"#,
    description: r#"Keynote document"#,
};
pub const COM_APPLE_IWORK_KEYNOTE_KTH: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.kth",
    conforms_to: r#"["com.apple.package", "public.presentation", "com.apple.keynote.kth"]"#,
    tags: r#"{"public.filename-extension": ["kth"]}"#,
    description: r#"Keynote theme"#,
};
pub const COM_APPLE_IWORK_KEYNOTE_SFFKTH: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.sffkth",
    conforms_to: r#"["public.data", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["kth"], "public.mime-type": ["application/x-iwork-keynote-sffkth"]}"#,
    description: r#"Keynote theme"#,
};
pub const COM_APPLE_IWORK_PAGES_PAGES: UTType = UTType {
    identifier: "com.apple.iWork.Pages.pages",
    conforms_to: r#"["com.apple.package", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["pages"]}"#,
    description: r#"Pages document"#,
};
pub const COM_APPLE_IWORK_PAGES_PAGES_TEF: UTType = UTType {
    identifier: "com.apple.iWork.Pages.pages-tef",
    conforms_to: r#"["com.apple.package", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["pages-tef"]}"#,
    description: r#"Pages document"#,
};
pub const COM_APPLE_IWORK_PAGES_SFFPAGES: UTType = UTType {
    identifier: "com.apple.iWork.Pages.sffpages",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["pages"], "public.mime-type": ["application/x-iwork-pages-sffpages"]}"#,
    description: r#"Pages document"#,
};
pub const COM_APPLE_IWORK_PAGES_TEMPLATE: UTType = UTType {
    identifier: "com.apple.iWork.Pages.template",
    conforms_to: r#"["com.apple.package", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["template"]}"#,
    description: r#"Pages template"#,
};
pub const COM_APPLE_IWORK_PAGES_SFFTEMPLATE: UTType = UTType {
    identifier: "com.apple.iWork.Pages.sfftemplate",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["template"], "public.mime-type": ["application/x-iwork-pages-sfftemplate"]}"#,
    description: r#"Pages template"#,
};
pub const COM_APPLE_IWORK_NUMBERS_NUMBERS: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.numbers",
    conforms_to: r#"["com.apple.package", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["numbers"]}"#,
    description: r#"Numbers document"#,
};
pub const COM_APPLE_IWORK_NUMBERS_NUMBERS_TEF: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.numbers-tef",
    conforms_to: r#"["com.apple.package", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["numbers-tef"]}"#,
    description: r#"Numbers document"#,
};
pub const COM_APPLE_IWORK_NUMBERS_SFFNUMBERS: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.sffnumbers",
    conforms_to: r#"["public.data", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["numbers"], "public.mime-type": ["application/x-iwork-numbers-sffnumbers"]}"#,
    description: r#"Numbers document"#,
};
pub const COM_APPLE_IWORK_NUMBERS_TEMPLATE: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.template",
    conforms_to: r#"["com.apple.package", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["nmbtemplate"]}"#,
    description: r#"Numbers template"#,
};
pub const COM_APPLE_IWORK_NUMBERS_SFFTEMPLATE: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.sfftemplate",
    conforms_to: r#"["public.data", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["nmbtemplate"], "public.mime-type": ["application/x-iwork-numbers-sfftemplate"]}"#,
    description: r#"Numbers template"#,
};
pub const COM_APPLE_GARAGEBAND_PROJECT: UTType = UTType {
    identifier: "com.apple.garageband.project",
    conforms_to: r#"["com.apple.package", "public.audiovisual-content"]"#,
    tags: r#"{"public.filename-extension": ["band", "gbProj"]}"#,
    description: r#"GarageBand Project"#,
};
pub const COM_ADOBE_PHOTOSHOP_IMAGE: UTType = UTType {
    identifier: "com.adobe.photoshop-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["psd"], "public.mime-type": ["image/vnd.adobe.photoshop", "image/photoshop", "image/x-photoshop", "image/psd", "application/photoshop"]}"#,
    description: r#"Adobe Photoshop document"#,
};
pub const COM_ADOBE_PHOTOSHOP_LARGE_IMAGE: UTType = UTType {
    identifier: "com.adobe.photoshop-large-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["psb"]}"#,
    description: r#"Adobe Photoshop large document"#,
};
pub const COM_ADOBE_ILLUSTRATOR_AI_IMAGE: UTType = UTType {
    identifier: "com.adobe.illustrator.ai-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["ai"]}"#,
    description: r#"Adobe Illustrator document"#,
};
pub const COM_TRUEVISION_TGA_IMAGE: UTType = UTType {
    identifier: "com.truevision.tga-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["tga"], "public.mime-type": ["image/targa", "image/tga", "application/tga"]}"#,
    description: r#"TGA image"#,
};
pub const COM_SGI_SGI_IMAGE: UTType = UTType {
    identifier: "com.sgi.sgi-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["sgi"], "public.mime-type": ["image/sgi"]}"#,
    description: r#"Silicon Graphics image"#,
};
pub const COM_ILM_OPENEXR_IMAGE: UTType = UTType {
    identifier: "com.ilm.openexr-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["exr"]}"#,
    description: r#"OpenEXR image"#,
};
pub const COM_KODAK_FLASHPIX_IMAGE: UTType = UTType {
    identifier: "com.kodak.flashpix-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["fpx"], "public.mime-type": ["image/fpx", "application/vnd.fpx"]}"#,
    description: r#"FlashPix image"#,
};
pub const PUBLIC_HEIF_STANDARD: UTType = UTType {
    identifier: "public.heif-standard",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: r#"HEIF Image"#,
};
pub const PUBLIC_HEIF: UTType = UTType {
    identifier: "public.heif",
    conforms_to: r#"["public.heif-standard"]"#,
    tags: r#"{"public.filename-extension": ["heif", "hif"], "public.mime-type": ["image/heif"]}"#,
    description: r#""#,
};
pub const PUBLIC_HEIC: UTType = UTType {
    identifier: "public.heic",
    conforms_to: r#"["public.heif-standard"]"#,
    tags: r#"{"public.filename-extension": ["heic"], "public.mime-type": ["image/heic"]}"#,
    description: r#""#,
};
pub const PUBLIC_AVCI: UTType = UTType {
    identifier: "public.avci",
    conforms_to: r#"["public.heif-standard"]"#,
    tags: r#"{"public.filename-extension": ["avci"], "public.mime-type": ["image/avci"]}"#,
    description: r#""#,
};
pub const PUBLIC_HEIFS: UTType = UTType {
    identifier: "public.heifs",
    conforms_to: r#"["public.heif-standard"]"#,
    tags: r#"{"public.filename-extension": ["heifs"], "public.mime-type": ["image/heif-sequence"]}"#,
    description: r#"HEIF Image Sequence"#,
};
pub const PUBLIC_HEICS: UTType = UTType {
    identifier: "public.heics",
    conforms_to: r#"["public.heif-standard"]"#,
    tags: r#"{"public.filename-extension": ["heics"], "public.mime-type": ["image/heic-sequence"]}"#,
    description: r#"HEIF Image Sequence"#,
};
pub const PUBLIC_AVCS: UTType = UTType {
    identifier: "public.avcs",
    conforms_to: r#"["public.heif-standard"]"#,
    tags: r#"{"public.filename-extension": ["avcs"], "public.mime-type": ["image/avcs"]}"#,
    description: r#"HEIF Image Sequence"#,
};
pub const COM_APPLE_DRAWING: UTType = UTType {
    identifier: "com.apple.drawing",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["drawing"]}"#,
    description: r#"Apple Drawing Format"#,
};
pub const COM_J2_JFX_FAX: UTType = UTType {
    identifier: "com.j2.jfx-fax",
    conforms_to: r#"["public.fax"]"#,
    tags: r#"{"public.filename-extension": ["jfx"]}"#,
    description: r#"J2 fax"#,
};
pub const COM_J2_EFX_FAX: UTType = UTType {
    identifier: "com.j2.efx-fax",
    conforms_to: r#"["public.fax"]"#,
    tags: r#"{"public.filename-extension": ["efx"], "public.mime-type": ["image/efax"]}"#,
    description: r#"eFax fax"#,
};
pub const COM_DIGIDESIGN_SD2_AUDIO: UTType = UTType {
    identifier: "com.digidesign.sd2-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["sd2"]}"#,
    description: r#"Sound Designer II audio"#,
};
pub const COM_MICROSOFT_WAVEFORM_AUDIO: UTType = UTType {
    identifier: "com.microsoft.waveform-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["wav", "wave", "bwf"], "public.mime-type": ["audio/vnd.wave", "audio/wav", "audio/wave", "audio/x-wav"]}"#,
    description: r#"Waveform audio"#,
};
pub const COM_MICROSOFT_ADVANCED_SYSTEMS_FORMAT: UTType = UTType {
    identifier: "com.microsoft.advanced-systems-format",
    conforms_to: r#"["public.audiovisual-content"]"#,
    tags: r#"{"public.filename-extension": ["asf"], "public.mime-type": ["video/x-ms-asf"]}"#,
    description: r#"Microsoft Advanced Systems Format"#,
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WM: UTType = UTType {
    identifier: "com.microsoft.windows-media-wm",
    conforms_to: r#"["public.movie", "com.microsoft.advanced-systems-format"]"#,
    tags: r#"{"public.filename-extension": ["wm"], "public.mime-type": ["video/x-ms-wm"]}"#,
    description: r#"Windows media"#,
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WMV: UTType = UTType {
    identifier: "com.microsoft.windows-media-wmv",
    conforms_to: r#"["public.movie", "com.microsoft.advanced-systems-format"]"#,
    tags: r#"{"public.filename-extension": ["wmv"], "public.mime-type": ["video/x-ms-wmv"]}"#,
    description: r#"Windows media"#,
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WMP: UTType = UTType {
    identifier: "com.microsoft.windows-media-wmp",
    conforms_to: r#"["public.movie", "com.microsoft.advanced-systems-format"]"#,
    tags: r#"{"public.filename-extension": ["wmp"], "public.mime-type": ["video/x-ms-wmp"]}"#,
    description: r#"Windows media"#,
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WMA: UTType = UTType {
    identifier: "com.microsoft.windows-media-wma",
    conforms_to: r#"["public.audio", "com.microsoft.advanced-systems-format"]"#,
    tags: r#"{"public.filename-extension": ["wma"], "public.mime-type": ["video/x-ms-wma"]}"#,
    description: r#"Windows media audio"#,
};
pub const COM_MICROSOFT_ADVANCED_STREAM_REDIRECTOR: UTType = UTType {
    identifier: "com.microsoft.advanced-stream-redirector",
    conforms_to: r#"["public.audiovisual-content", "public.xml"]"#,
    tags: r#"{"public.filename-extension": ["asx"], "public.mime-type": ["video/x-ms-asx"]}"#,
    description: r#"Advanced Stream Redirector"#,
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WMX: UTType = UTType {
    identifier: "com.microsoft.windows-media-wmx",
    conforms_to: r#"["public.movie", "com.microsoft.advanced-stream-redirector"]"#,
    tags: r#"{"public.filename-extension": ["wmx"], "public.mime-type": ["video/x-ms-wmx"]}"#,
    description: r#"Windows media"#,
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WVX: UTType = UTType {
    identifier: "com.microsoft.windows-media-wvx",
    conforms_to: r#"["public.movie", "com.microsoft.advanced-stream-redirector"]"#,
    tags: r#"{"public.filename-extension": ["wvx"], "public.mime-type": ["video/x-ms-wvx"]}"#,
    description: r#"Windows media"#,
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WAX: UTType = UTType {
    identifier: "com.microsoft.windows-media-wax",
    conforms_to: r#"["public.audio", "com.microsoft.advanced-stream-redirector"]"#,
    tags: r#"{"public.filename-extension": ["wax"], "public.mime-type": ["video/x-ms-wax"]}"#,
    description: r#"Windows media audio"#,
};
pub const COM_REAL_REALMEDIA: UTType = UTType {
    identifier: "com.real.realmedia",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["rm"], "public.mime-type": ["application/vnd.rn-realmedia"]}"#,
    description: r#"RealMedia"#,
};
pub const COM_REAL_REALMEDIA_VBR: UTType = UTType {
    identifier: "com.real.realmedia-vbr",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["rmvb"], "public.mime-type": ["application/vnd.rn-realmedia-vbr"]}"#,
    description: r#"RealMedia Variable Bitrate"#,
};
pub const ORG_SMPTE_MXF: UTType = UTType {
    identifier: "org.smpte.mxf",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["mxf"], "public.mime-type": ["application/mxf"]}"#,
    description: r#"Material eXchange Format"#,
};
pub const COM_REAL_REALAUDIO: UTType = UTType {
    identifier: "com.real.realaudio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["ram", "ra"], "public.mime-type": ["audio/vnd.rn-realaudio", "audio/x-pn-realaudio", "audio/x-realaudio"]}"#,
    description: r#"RealMedia Audio"#,
};
pub const COM_SOUNDBLASTER_SOUNDFONT: UTType = UTType {
    identifier: "com.soundblaster.soundfont",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["sf2"]}"#,
    description: r#"SoundFont audio"#,
};
pub const ORG_XIPH_FLAC: UTType = UTType {
    identifier: "org.xiph.flac",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["flac"], "public.mime-type": ["audio/flac"]}"#,
    description: r#"FLAC audio"#,
};
pub const COM_AVID_OPEN_MEDIA_FRAMEWORK: UTType = UTType {
    identifier: "com.avid.open-media-framework",
    conforms_to: r#"["public.audiovisual-content"]"#,
    tags: r#"{"public.filename-extension": ["omf"]}"#,
    description: r#"Open Media Framework interchange format"#,
};
pub const PUBLIC_MP4A_LOAS: UTType = UTType {
    identifier: "public.mp4a-loas",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["loas", "latm"]}"#,
    description: r#"Low Overhead MPEG-4 Audio Stream"#,
};
pub const PUBLIC_MP4A_LATM: UTType = UTType {
    identifier: "public.mp4a-latm",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.mime-type": ["audio/mp4a-latm"]}"#,
    description: r#"Low-overhead MPEG-4 Audio Transport Multiplex"#,
};
pub const COM_ALLUME_STUFFIT_ARCHIVE: UTType = UTType {
    identifier: "com.allume.stuffit-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{}"#,
    description: r#"StuffIt archive family"#,
};
pub const COM_STUFFIT_ARCHIVE_SITX: UTType = UTType {
    identifier: "com.stuffit.archive.sitx",
    conforms_to: r#"["com.allume.stuffit-archive"]"#,
    tags: r#"{"public.filename-extension": ["sitx"], "public.mime-type": ["application/x-stuffitx", "application/x-sitx"]}"#,
    description: r#"StuffIt X archive"#,
};
pub const COM_STUFFIT_ARCHIVE_SIDX: UTType = UTType {
    identifier: "com.stuffit.archive.sidx",
    conforms_to: r#"["com.allume.stuffit-archive"]"#,
    tags: r#"{"public.filename-extension": ["sidx"], "public.mime-type": ["application/x-stuffitx-index", "application/x-sitx-index"]}"#,
    description: r#"StuffIt X index"#,
};
pub const COM_STUFFIT_ARCHIVE_SIT: UTType = UTType {
    identifier: "com.stuffit.archive.sit",
    conforms_to: r#"["com.allume.stuffit-archive"]"#,
    tags: r#"{"public.filename-extension": ["sit", "sea"], "public.mime-type": ["application/x-stuffit", "application/x-sit"]}"#,
    description: r#"StuffIt archive"#,
};
pub const COM_ADOBE_FLASH_VIDEO: UTType = UTType {
    identifier: "com.adobe.flash.video",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["flv", "f4v", "f4p", "f4a", "f4b"], "public.mime-type": ["video/x-flv"]}"#,
    description: r#"Flash video"#,
};
pub const ORG_KHRONOS_COLLADA_DIGITAL_ASSET_EXCHANGE: UTType = UTType {
    identifier: "org.khronos.collada.digital-asset-exchange",
    conforms_to: r#"["public.xml", "public.audiovisual-content", "public.3d-content"]"#,
    tags: r#"{"public.filename-extension": ["dae"]}"#,
    description: r#"Digital Asset Exchange (DAE)"#,
};
pub const COM_APPLE_IMOVIELIBRARY: UTType = UTType {
    identifier: "com.apple.iMovieLibrary",
    conforms_to: r#"["com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["imovielibrary"]}"#,
    description: r#"iMovie Library"#,
};
pub const COM_APPLE_IMOVIETHEATER: UTType = UTType {
    identifier: "com.apple.iMovieTheater",
    conforms_to: r#"["com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["theater"]}"#,
    description: r#"iMovie Theater"#,
};
pub const ORG_7_ZIP_7_ZIP_ARCHIVE: UTType = UTType {
    identifier: "org.7-zip.7-zip-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["7z"], "public.mime-type": ["application/x-7z-compressed"]}"#,
    description: r#"7-Zip archive"#,
};
pub const ORG_TUKAANI_XZ_ARCHIVE: UTType = UTType {
    identifier: "org.tukaani.xz-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["xz"], "public.mime-type": ["application/x-xz"]}"#,
    description: r#"xz compressed archive"#,
};
pub const ORG_TUKAANI_TAR_XZ_ARCHIVE: UTType = UTType {
    identifier: "org.tukaani.tar-xz-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["txz"]}"#,
    description: r#"xz compressed tar archive"#,
};
pub const COM_MICROSOFT_CAB: UTType = UTType {
    identifier: "com.microsoft.cab",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["cab"]}"#,
    description: r#"Microsoft Cabinet archive"#,
};
pub const PUBLIC_HAPTICS_CONTENT: UTType = UTType {
    identifier: "public.haptics-content",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Haptics content"#,
};
pub const COM_APPLE_HAPTICS_AHAP: UTType = UTType {
    identifier: "com.apple.haptics.ahap",
    conforms_to: r#"["public.haptics-content", "public.json"]"#,
    tags: r#"{"public.filename-extension": ["ahap"]}"#,
    description: r#"Apple Haptics Audio Pattern"#,
};
pub const COM_APPLE_COREML_MODEL: UTType = UTType {
    identifier: "com.apple.coreml.model",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["mlmodel", "mlkitmodel"]}"#,
    description: r#"Core ML Machine Learning Model"#,
};
pub const COM_APPLE_COREML_MLPACKAGE: UTType = UTType {
    identifier: "com.apple.coreml.mlpackage",
    conforms_to: r#"["com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["mlpackage"]}"#,
    description: r#"Core ML Machine Learning Model Package"#,
};
pub const COM_APPLE_GROUPACTIVITIES_ACTIVITY: UTType = UTType {
    identifier: "com.apple.groupactivities.activity",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["groupactivity"]}"#,
    description: r#"Group Activity"#,
};
pub const COM_APPLE_ICON_DECORATION: UTType = UTType {
    identifier: "com.apple.icon-decoration",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration"#,
};
pub const COM_APPLE_ICON_DECORATION_POSITION: UTType = UTType {
    identifier: "com.apple.icon-decoration-position",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Position"#,
};
pub const COM_APPLE_ICON_DECORATION_POSITION_CENTER: UTType = UTType {
    identifier: "com.apple.icon-decoration-position.center",
    conforms_to: r#"["com.apple.icon-decoration-position"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Position Center"#,
};
pub const COM_APPLE_ICON_DECORATION_POSITION_LEADING_BOTTOM: UTType = UTType {
    identifier: "com.apple.icon-decoration-position.leading-bottom",
    conforms_to: r#"["com.apple.icon-decoration-position"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Position Leading Bottom"#,
};
pub const COM_APPLE_ICON_DECORATION_POSITION_TRAILING_BOTTOM: UTType = UTType {
    identifier: "com.apple.icon-decoration-position.trailing-bottom",
    conforms_to: r#"["com.apple.icon-decoration-position"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Position Trailing Bottom"#,
};
pub const COM_APPLE_ICON_DECORATION_POSITION_OVERLAY: UTType = UTType {
    identifier: "com.apple.icon-decoration-position.overlay",
    conforms_to: r#"["com.apple.icon-decoration-position"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Position Overlay"#,
};
pub const COM_APPLE_ICON_DECORATION_BADGE: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Badge"#,
};
pub const COM_APPLE_ICON_DECORATION_EMBOSS: UTType = UTType {
    identifier: "com.apple.icon-decoration.emboss",
    conforms_to: r#"["com.apple.icon-decoration-position.center", "com.apple.icon-decoration"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Emboss"#,
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM: UTType = UTType {
    identifier: "com.apple.icon-decoration.system",
    conforms_to: r#"["com.apple.icon-decoration"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration System"#,
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM_UNSUPPORTED: UTType = UTType {
    identifier: "com.apple.icon-decoration.system.unsupported",
    conforms_to: r#"["com.apple.icon-decoration.system"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Unsupported"#,
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM_CAUTION_ALERT: UTType = UTType {
    identifier: "com.apple.icon-decoration.system.caution-alert",
    conforms_to: r#"["com.apple.icon-decoration.system"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Alert Caution"#,
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM_ALIAS: UTType = UTType {
    identifier: "com.apple.icon-decoration.system.alias",
    conforms_to: r#"["com.apple.icon-decoration.system"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Alias"#,
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM_NEW_FOLDER: UTType = UTType {
    identifier: "com.apple.icon-decoration.system.new-folder",
    conforms_to: r#"["com.apple.icon-decoration.system"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration New Folder"#,
};
pub const COM_APPLE_ICON_DECORATION_BADGE_LOCKED: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.locked",
    conforms_to: r#"["com.apple.icon-decoration-position.leading-bottom", "com.apple.icon-decoration.system"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Locked Badge"#,
};
pub const COM_APPLE_ICON_DECORATION_BADGE_CHECKMARK: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.checkmark",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Checkmark Badge"#,
};
pub const COM_APPLE_ICON_DECORATION_BADGE_COMMENTS: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.comments",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Comments Badge"#,
};
pub const COM_APPLE_ICON_DECORATION_BADGE_DROP_FOLDER: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.drop-folder",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Drop Folder Badge"#,
};
pub const COM_APPLE_ICON_DECORATION_BADGE_HEART: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.heart",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Heart Badge"#,
};
pub const COM_APPLE_ICON_DECORATION_BADGE_IN_REVIEW: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.in-review",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration In Review Badge"#,
};
pub const COM_APPLE_ICON_DECORATION_BADGE_LOCKED_BY_COLLABORATOR: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.locked-by-collaborator",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Locked By Collaborator Badge"#,
};
pub const COM_APPLE_ICON_DECORATION_BADGE_LOCKED_BY_USER: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.locked-by-user",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Locked By User Badge"#,
};
pub const COM_APPLE_ICON_DECORATION_BADGE_PINNED: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.pinned",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Pinned Badge"#,
};
pub const COM_APPLE_ICON_DECORATION_BADGE_PRIVATE_FOLDER: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.private-folder",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Private Folder Badge"#,
};
pub const COM_APPLE_ICON_DECORATION_BADGE_SYNCING: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.syncing",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Syncing Badge"#,
};
pub const COM_APPLE_ICON_DECORATION_BADGE_TRENDING: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.trending",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Trending Badge"#,
};
pub const COM_APPLE_ICON_DECORATION_BADGE_WARNING: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.warning",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: r#"Icon Decoration Warning Badge"#,
};
pub const COM_APPLE_DOCUMENT_TYPE: UTType = UTType {
    identifier: "com.apple.document-type",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: r#"Document Type"#,
};
pub const COM_APPLE_DOCUMENT_TYPE_DICTIONARY: UTType = UTType {
    identifier: "com.apple.document-type.dictionary",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Dictionary"#,
};
pub const COM_APPLE_ACCOUNTS_ICON: UTType = UTType {
    identifier: "com.apple.accounts-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Accounts Icon"#,
};
pub const COM_APPLE_LEGACY_ACTIONS_ICON: UTType = UTType {
    identifier: "com.apple.legacy.actions-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Actions Icon"#,
};
pub const COM_APPLE_EVERYONE_ICON: UTType = UTType {
    identifier: "com.apple.everyone-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Everyone Icon"#,
};
pub const COM_APPLE_LEGACY_GENERAL_ICON: UTType = UTType {
    identifier: "com.apple.legacy.general-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"General Icon"#,
};
pub const COM_APPLE_LEGACY_SIDEBAR_PREFS_ICON: UTType = UTType {
    identifier: "com.apple.legacy.sidebar-prefs-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Sidebar Prefs Icon"#,
};
pub const COM_APPLE_LEGACY_TOOLBAR_ADVANCED_ICON: UTType = UTType {
    identifier: "com.apple.legacy.toolbar-advanced-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Toolbar Advanced Icon"#,
};
pub const COM_APPLE_LEGACY_TOOLBAR_INFO_ICON: UTType = UTType {
    identifier: "com.apple.legacy.toolbar-info-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Toolbar Info Icon"#,
};
pub const COM_APPLE_LEGACY_TOOLBAR_LABELS_ICON: UTType = UTType {
    identifier: "com.apple.legacy.toolbar-labels-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Toolbar Labels Icon"#,
};
pub const COM_APPLE_LEGACY_CLOCK_ICON: UTType = UTType {
    identifier: "com.apple.legacy.clock-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Clock Icon"#,
};
pub const COM_APPLE_LEGACY_SYNCHRONIZE: UTType = UTType {
    identifier: "com.apple.legacy.synchronize",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Synchronize"#,
};
pub const COM_APPLE_ICON_OVERLAY_NEW_FOLDER_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.new-folder-badge",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"New Folder Badge"#,
};
pub const COM_APPLE_LEGACY_FINDER_ICON: UTType = UTType {
    identifier: "com.apple.legacy.finder-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Finder"#,
};
pub const COM_APPLE_UNKNOWN_OBJECT: UTType = UTType {
    identifier: "com.apple.unknown-object",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Unknown Object"#,
};
pub const COM_APPLE_NOT_LOADED: UTType = UTType {
    identifier: "com.apple.not-loaded",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Not Loaded"#,
};
pub const COM_APPLE_LEGACY_WINDOW: UTType = UTType {
    identifier: "com.apple.legacy.window",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Window"#,
};
pub const COM_APPLE_LEGACY_QUESTION_MARK: UTType = UTType {
    identifier: "com.apple.legacy.question-mark",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Question Mark"#,
};
pub const COM_APPLE_LEGACY_EJECT_MEDIA: UTType = UTType {
    identifier: "com.apple.legacy.eject-media",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Eject Media"#,
};
pub const COM_APPLE_LEGACY_BURN: UTType = UTType {
    identifier: "com.apple.legacy.burn",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Burn"#,
};
pub const COM_APPLE_LEGACY_CUSTOMIZE_TOOLBAR: UTType = UTType {
    identifier: "com.apple.legacy.customize-toolbar",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Customize Toolbar"#,
};
pub const COM_APPLE_LEGACY_DELETE_TOOLBAR: UTType = UTType {
    identifier: "com.apple.legacy.delete-toolbar",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Delete Toolbar"#,
};
pub const COM_APPLE_LEGACY_RIGHT_CONTAINER_ARROW: UTType = UTType {
    identifier: "com.apple.legacy.right-container-arrow",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Right Container Arrow"#,
};
pub const COM_APPLE_ICON_OVERLAY_DROP_FOLDER_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.drop-folder-badge",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Drop Folder Badge"#,
};
pub const COM_APPLE_ICON_OVERLAY_PRIVATE_FOLDER_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.private-folder-badge",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Private Folder Badge"#,
};
pub const COM_APPLE_ICON_OVERLAY_PRIVATE_FOLDER: UTType = UTType {
    identifier: "com.apple.icon-overlay.private-folder",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Private Folder"#,
};
pub const COM_APPLE_LEGACY_OPEN_FOLDER: UTType = UTType {
    identifier: "com.apple.legacy.open-folder",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Open Folder"#,
};
pub const COM_APPLE_LEGACY_FAVORITE_ITEMS: UTType = UTType {
    identifier: "com.apple.legacy.favorite-items",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Favorite Items"#,
};
pub const COM_APPLE_LEGACY_LOCKED: UTType = UTType {
    identifier: "com.apple.legacy.locked",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Locked"#,
};
pub const COM_APPLE_LEGACY_UNLOCKED: UTType = UTType {
    identifier: "com.apple.legacy.unlocked",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Unlocked"#,
};
pub const COM_APPLE_LEGACY_NO_WRITE: UTType = UTType {
    identifier: "com.apple.legacy.no-write",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"No Write"#,
};
pub const COM_APPLE_LEGACY_KEEP_ARRANGED: UTType = UTType {
    identifier: "com.apple.legacy.keep-arranged",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Keep Arranged"#,
};
pub const COM_APPLE_LEGACY_GRID: UTType = UTType {
    identifier: "com.apple.legacy.grid",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Grid"#,
};
pub const COM_APPLE_LEGACY_CONNECT_TO: UTType = UTType {
    identifier: "com.apple.legacy.connect-to",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Connect To"#,
};
pub const COM_APPLE_LEGACY_BACKWARD_ARROW: UTType = UTType {
    identifier: "com.apple.legacy.backward-arrow",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Backward Arrow"#,
};
pub const COM_APPLE_LEGACY_FORWARD_ARROW: UTType = UTType {
    identifier: "com.apple.legacy.forward-arrow",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Forward Arrow"#,
};
pub const COM_APPLE_ICON_OVERLAY_LOCKED_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.locked-badge",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Locked Badge"#,
};
pub const COM_APPLE_ICON_OVERLAY_ALIAS_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.alias-badge",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Alias Badge"#,
};
pub const COM_APPLE_ICON_OVERLAY_ALERT_CAUTION_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.alert-caution-badge",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Alert Caution Badge"#,
};
pub const COM_APPLE_ICON_OVERLAY_UNSUPPORTED_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.unsupported-badge",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Unsupported Badge"#,
};
pub const COM_APPLE_LEGACY_MAGNIFYING_GLASS: UTType = UTType {
    identifier: "com.apple.legacy.magnifying-glass",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Magnifying Glass"#,
};
pub const COM_APPLE_LEGACY_ERASING: UTType = UTType {
    identifier: "com.apple.legacy.erasing",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Erasing"#,
};
pub const COM_APPLE_LEGACY_MULTIPLE_ITEMS: UTType = UTType {
    identifier: "com.apple.legacy.multiple-items",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: r#"Multiple Items"#,
};

lazy_static! {
    pub(crate) static ref OTHER_TYPES_MAP: Arc<RwLock<HashMap<&'static str, UTType<'static>>>> =
        Arc::new(RwLock::new(HashMap::from([
            (
                "com.apple.legacy.multiple-items",
                COM_APPLE_LEGACY_MULTIPLE_ITEMS
            ),
            ("com.apple.legacy.erasing", COM_APPLE_LEGACY_ERASING),
            (
                "com.apple.legacy.magnifying-glass",
                COM_APPLE_LEGACY_MAGNIFYING_GLASS
            ),
            (
                "com.apple.icon-overlay.unsupported-badge",
                COM_APPLE_ICON_OVERLAY_UNSUPPORTED_BADGE
            ),
            (
                "com.apple.icon-overlay.alert-caution-badge",
                COM_APPLE_ICON_OVERLAY_ALERT_CAUTION_BADGE
            ),
            (
                "com.apple.icon-overlay.alias-badge",
                COM_APPLE_ICON_OVERLAY_ALIAS_BADGE
            ),
            (
                "com.apple.icon-overlay.locked-badge",
                COM_APPLE_ICON_OVERLAY_LOCKED_BADGE
            ),
            (
                "com.apple.legacy.forward-arrow",
                COM_APPLE_LEGACY_FORWARD_ARROW
            ),
            (
                "com.apple.legacy.backward-arrow",
                COM_APPLE_LEGACY_BACKWARD_ARROW
            ),
            ("com.apple.legacy.connect-to", COM_APPLE_LEGACY_CONNECT_TO),
            ("com.apple.legacy.grid", COM_APPLE_LEGACY_GRID),
            (
                "com.apple.legacy.keep-arranged",
                COM_APPLE_LEGACY_KEEP_ARRANGED
            ),
            ("com.apple.legacy.no-write", COM_APPLE_LEGACY_NO_WRITE),
            ("com.apple.legacy.unlocked", COM_APPLE_LEGACY_UNLOCKED),
            ("com.apple.legacy.locked", COM_APPLE_LEGACY_LOCKED),
            (
                "com.apple.legacy.favorite-items",
                COM_APPLE_LEGACY_FAVORITE_ITEMS
            ),
            ("com.apple.legacy.open-folder", COM_APPLE_LEGACY_OPEN_FOLDER),
            (
                "com.apple.icon-overlay.private-folder",
                COM_APPLE_ICON_OVERLAY_PRIVATE_FOLDER
            ),
            (
                "com.apple.icon-overlay.private-folder-badge",
                COM_APPLE_ICON_OVERLAY_PRIVATE_FOLDER_BADGE
            ),
            (
                "com.apple.icon-overlay.drop-folder-badge",
                COM_APPLE_ICON_OVERLAY_DROP_FOLDER_BADGE
            ),
            (
                "com.apple.legacy.right-container-arrow",
                COM_APPLE_LEGACY_RIGHT_CONTAINER_ARROW
            ),
            (
                "com.apple.legacy.delete-toolbar",
                COM_APPLE_LEGACY_DELETE_TOOLBAR
            ),
            (
                "com.apple.legacy.customize-toolbar",
                COM_APPLE_LEGACY_CUSTOMIZE_TOOLBAR
            ),
            ("com.apple.legacy.burn", COM_APPLE_LEGACY_BURN),
            ("com.apple.legacy.eject-media", COM_APPLE_LEGACY_EJECT_MEDIA),
            (
                "com.apple.legacy.question-mark",
                COM_APPLE_LEGACY_QUESTION_MARK
            ),
            ("com.apple.legacy.window", COM_APPLE_LEGACY_WINDOW),
            ("com.apple.not-loaded", COM_APPLE_NOT_LOADED),
            ("com.apple.unknown-object", COM_APPLE_UNKNOWN_OBJECT),
            ("com.apple.legacy.finder-icon", COM_APPLE_LEGACY_FINDER_ICON),
            (
                "com.apple.icon-overlay.new-folder-badge",
                COM_APPLE_ICON_OVERLAY_NEW_FOLDER_BADGE
            ),
            ("com.apple.legacy.synchronize", COM_APPLE_LEGACY_SYNCHRONIZE),
            ("com.apple.legacy.clock-icon", COM_APPLE_LEGACY_CLOCK_ICON),
            (
                "com.apple.legacy.toolbar-labels-icon",
                COM_APPLE_LEGACY_TOOLBAR_LABELS_ICON
            ),
            (
                "com.apple.legacy.toolbar-info-icon",
                COM_APPLE_LEGACY_TOOLBAR_INFO_ICON
            ),
            (
                "com.apple.legacy.toolbar-advanced-icon",
                COM_APPLE_LEGACY_TOOLBAR_ADVANCED_ICON
            ),
            (
                "com.apple.legacy.sidebar-prefs-icon",
                COM_APPLE_LEGACY_SIDEBAR_PREFS_ICON
            ),
            (
                "com.apple.legacy.general-icon",
                COM_APPLE_LEGACY_GENERAL_ICON
            ),
            ("com.apple.everyone-icon", COM_APPLE_EVERYONE_ICON),
            (
                "com.apple.legacy.actions-icon",
                COM_APPLE_LEGACY_ACTIONS_ICON
            ),
            ("com.apple.accounts-icon", COM_APPLE_ACCOUNTS_ICON),
            (
                "com.apple.document-type.dictionary",
                COM_APPLE_DOCUMENT_TYPE_DICTIONARY
            ),
            ("com.apple.document-type", COM_APPLE_DOCUMENT_TYPE),
            (
                "com.apple.icon-decoration.badge.warning",
                COM_APPLE_ICON_DECORATION_BADGE_WARNING
            ),
            (
                "com.apple.icon-decoration.badge.trending",
                COM_APPLE_ICON_DECORATION_BADGE_TRENDING
            ),
            (
                "com.apple.icon-decoration.badge.syncing",
                COM_APPLE_ICON_DECORATION_BADGE_SYNCING
            ),
            (
                "com.apple.icon-decoration.badge.private-folder",
                COM_APPLE_ICON_DECORATION_BADGE_PRIVATE_FOLDER
            ),
            (
                "com.apple.icon-decoration.badge.pinned",
                COM_APPLE_ICON_DECORATION_BADGE_PINNED
            ),
            (
                "com.apple.icon-decoration.badge.locked-by-user",
                COM_APPLE_ICON_DECORATION_BADGE_LOCKED_BY_USER
            ),
            (
                "com.apple.icon-decoration.badge.locked-by-collaborator",
                COM_APPLE_ICON_DECORATION_BADGE_LOCKED_BY_COLLABORATOR
            ),
            (
                "com.apple.icon-decoration.badge.in-review",
                COM_APPLE_ICON_DECORATION_BADGE_IN_REVIEW
            ),
            (
                "com.apple.icon-decoration.badge.heart",
                COM_APPLE_ICON_DECORATION_BADGE_HEART
            ),
            (
                "com.apple.icon-decoration.badge.drop-folder",
                COM_APPLE_ICON_DECORATION_BADGE_DROP_FOLDER
            ),
            (
                "com.apple.icon-decoration.badge.comments",
                COM_APPLE_ICON_DECORATION_BADGE_COMMENTS
            ),
            (
                "com.apple.icon-decoration.badge.checkmark",
                COM_APPLE_ICON_DECORATION_BADGE_CHECKMARK
            ),
            (
                "com.apple.icon-decoration.badge.locked",
                COM_APPLE_ICON_DECORATION_BADGE_LOCKED
            ),
            (
                "com.apple.icon-decoration.system.new-folder",
                COM_APPLE_ICON_DECORATION_SYSTEM_NEW_FOLDER
            ),
            (
                "com.apple.icon-decoration.system.alias",
                COM_APPLE_ICON_DECORATION_SYSTEM_ALIAS
            ),
            (
                "com.apple.icon-decoration.system.caution-alert",
                COM_APPLE_ICON_DECORATION_SYSTEM_CAUTION_ALERT
            ),
            (
                "com.apple.icon-decoration.system.unsupported",
                COM_APPLE_ICON_DECORATION_SYSTEM_UNSUPPORTED
            ),
            (
                "com.apple.icon-decoration.system",
                COM_APPLE_ICON_DECORATION_SYSTEM
            ),
            (
                "com.apple.icon-decoration.emboss",
                COM_APPLE_ICON_DECORATION_EMBOSS
            ),
            (
                "com.apple.icon-decoration.badge",
                COM_APPLE_ICON_DECORATION_BADGE
            ),
            (
                "com.apple.icon-decoration-position.overlay",
                COM_APPLE_ICON_DECORATION_POSITION_OVERLAY
            ),
            (
                "com.apple.icon-decoration-position.trailing-bottom",
                COM_APPLE_ICON_DECORATION_POSITION_TRAILING_BOTTOM
            ),
            (
                "com.apple.icon-decoration-position.leading-bottom",
                COM_APPLE_ICON_DECORATION_POSITION_LEADING_BOTTOM
            ),
            (
                "com.apple.icon-decoration-position.center",
                COM_APPLE_ICON_DECORATION_POSITION_CENTER
            ),
            (
                "com.apple.icon-decoration-position",
                COM_APPLE_ICON_DECORATION_POSITION
            ),
            ("com.apple.icon-decoration", COM_APPLE_ICON_DECORATION),
            (
                "com.apple.groupactivities.activity",
                COM_APPLE_GROUPACTIVITIES_ACTIVITY
            ),
            ("com.apple.coreml.mlpackage", COM_APPLE_COREML_MLPACKAGE),
            ("com.apple.coreml.model", COM_APPLE_COREML_MODEL),
            ("com.apple.haptics.ahap", COM_APPLE_HAPTICS_AHAP),
            ("public.haptics-content", PUBLIC_HAPTICS_CONTENT),
            ("com.microsoft.cab", COM_MICROSOFT_CAB),
            ("org.tukaani.tar-xz-archive", ORG_TUKAANI_TAR_XZ_ARCHIVE),
            ("org.tukaani.xz-archive", ORG_TUKAANI_XZ_ARCHIVE),
            ("org.7-zip.7-zip-archive", ORG_7_ZIP_7_ZIP_ARCHIVE),
            ("com.apple.iMovieTheater", COM_APPLE_IMOVIETHEATER),
            ("com.apple.iMovieLibrary", COM_APPLE_IMOVIELIBRARY),
            (
                "org.khronos.collada.digital-asset-exchange",
                ORG_KHRONOS_COLLADA_DIGITAL_ASSET_EXCHANGE
            ),
            ("com.adobe.flash.video", COM_ADOBE_FLASH_VIDEO),
            ("com.stuffit.archive.sit", COM_STUFFIT_ARCHIVE_SIT),
            ("com.stuffit.archive.sidx", COM_STUFFIT_ARCHIVE_SIDX),
            ("com.stuffit.archive.sitx", COM_STUFFIT_ARCHIVE_SITX),
            ("com.allume.stuffit-archive", COM_ALLUME_STUFFIT_ARCHIVE),
            ("public.mp4a-latm", PUBLIC_MP4A_LATM),
            ("public.mp4a-loas", PUBLIC_MP4A_LOAS),
            (
                "com.avid.open-media-framework",
                COM_AVID_OPEN_MEDIA_FRAMEWORK
            ),
            ("org.xiph.flac", ORG_XIPH_FLAC),
            ("com.soundblaster.soundfont", COM_SOUNDBLASTER_SOUNDFONT),
            ("com.real.realaudio", COM_REAL_REALAUDIO),
            ("org.smpte.mxf", ORG_SMPTE_MXF),
            ("com.real.realmedia-vbr", COM_REAL_REALMEDIA_VBR),
            ("com.real.realmedia", COM_REAL_REALMEDIA),
            (
                "com.microsoft.windows-media-wax",
                COM_MICROSOFT_WINDOWS_MEDIA_WAX
            ),
            (
                "com.microsoft.windows-media-wvx",
                COM_MICROSOFT_WINDOWS_MEDIA_WVX
            ),
            (
                "com.microsoft.windows-media-wmx",
                COM_MICROSOFT_WINDOWS_MEDIA_WMX
            ),
            (
                "com.microsoft.advanced-stream-redirector",
                COM_MICROSOFT_ADVANCED_STREAM_REDIRECTOR
            ),
            (
                "com.microsoft.windows-media-wma",
                COM_MICROSOFT_WINDOWS_MEDIA_WMA
            ),
            (
                "com.microsoft.windows-media-wmp",
                COM_MICROSOFT_WINDOWS_MEDIA_WMP
            ),
            (
                "com.microsoft.windows-media-wmv",
                COM_MICROSOFT_WINDOWS_MEDIA_WMV
            ),
            (
                "com.microsoft.windows-media-wm",
                COM_MICROSOFT_WINDOWS_MEDIA_WM
            ),
            (
                "com.microsoft.advanced-systems-format",
                COM_MICROSOFT_ADVANCED_SYSTEMS_FORMAT
            ),
            ("com.microsoft.waveform-audio", COM_MICROSOFT_WAVEFORM_AUDIO),
            ("com.digidesign.sd2-audio", COM_DIGIDESIGN_SD2_AUDIO),
            ("com.j2.efx-fax", COM_J2_EFX_FAX),
            ("com.j2.jfx-fax", COM_J2_JFX_FAX),
            ("com.apple.drawing", COM_APPLE_DRAWING),
            ("public.avcs", PUBLIC_AVCS),
            ("public.heics", PUBLIC_HEICS),
            ("public.heifs", PUBLIC_HEIFS),
            ("public.avci", PUBLIC_AVCI),
            ("public.heic", PUBLIC_HEIC),
            ("public.heif", PUBLIC_HEIF),
            ("public.heif-standard", PUBLIC_HEIF_STANDARD),
            ("com.kodak.flashpix-image", COM_KODAK_FLASHPIX_IMAGE),
            ("com.ilm.openexr-image", COM_ILM_OPENEXR_IMAGE),
            ("com.sgi.sgi-image", COM_SGI_SGI_IMAGE),
            ("com.truevision.tga-image", COM_TRUEVISION_TGA_IMAGE),
            (
                "com.adobe.illustrator.ai-image",
                COM_ADOBE_ILLUSTRATOR_AI_IMAGE
            ),
            (
                "com.adobe.photoshop-large-image",
                COM_ADOBE_PHOTOSHOP_LARGE_IMAGE
            ),
            ("com.adobe.photoshop-image", COM_ADOBE_PHOTOSHOP_IMAGE),
            ("com.apple.garageband.project", COM_APPLE_GARAGEBAND_PROJECT),
            (
                "com.apple.iWork.Numbers.sfftemplate",
                COM_APPLE_IWORK_NUMBERS_SFFTEMPLATE
            ),
            (
                "com.apple.iWork.Numbers.template",
                COM_APPLE_IWORK_NUMBERS_TEMPLATE
            ),
            (
                "com.apple.iWork.Numbers.sffnumbers",
                COM_APPLE_IWORK_NUMBERS_SFFNUMBERS
            ),
            (
                "com.apple.iWork.Numbers.numbers-tef",
                COM_APPLE_IWORK_NUMBERS_NUMBERS_TEF
            ),
            (
                "com.apple.iWork.Numbers.numbers",
                COM_APPLE_IWORK_NUMBERS_NUMBERS
            ),
            (
                "com.apple.iWork.Pages.sfftemplate",
                COM_APPLE_IWORK_PAGES_SFFTEMPLATE
            ),
            (
                "com.apple.iWork.Pages.template",
                COM_APPLE_IWORK_PAGES_TEMPLATE
            ),
            (
                "com.apple.iWork.Pages.sffpages",
                COM_APPLE_IWORK_PAGES_SFFPAGES
            ),
            (
                "com.apple.iWork.Pages.pages-tef",
                COM_APPLE_IWORK_PAGES_PAGES_TEF
            ),
            ("com.apple.iWork.Pages.pages", COM_APPLE_IWORK_PAGES_PAGES),
            (
                "com.apple.iWork.Keynote.sffkth",
                COM_APPLE_IWORK_KEYNOTE_SFFKTH
            ),
            ("com.apple.iWork.Keynote.kth", COM_APPLE_IWORK_KEYNOTE_KTH),
            (
                "com.apple.iWork.Keynote.sffkey",
                COM_APPLE_IWORK_KEYNOTE_SFFKEY
            ),
            (
                "com.apple.iWork.Keynote.key-tef",
                COM_APPLE_IWORK_KEYNOTE_KEY_TEF
            ),
            ("com.apple.iWork.Keynote.key", COM_APPLE_IWORK_KEYNOTE_KEY),
            ("com.apple.keynote.kth", COM_APPLE_KEYNOTE_KTH),
            ("com.apple.keynote.key", COM_APPLE_KEYNOTE_KEY),
            ("com.microsoft.powerpoint.pps", COM_MICROSOFT_POWERPOINT_PPS),
            ("com.microsoft.powerpoint.pot", COM_MICROSOFT_POWERPOINT_POT),
            ("com.microsoft.powerpoint.ppt", COM_MICROSOFT_POWERPOINT_PPT),
            ("com.microsoft.excel.xla", COM_MICROSOFT_EXCEL_XLA),
            ("com.microsoft.excel.xlw", COM_MICROSOFT_EXCEL_XLW),
            ("com.microsoft.excel.xlt", COM_MICROSOFT_EXCEL_XLT),
            ("com.microsoft.excel.xls", COM_MICROSOFT_EXCEL_XLS),
            ("com.microsoft.word.dot", COM_MICROSOFT_WORD_DOT),
            ("com.microsoft.word.doc", COM_MICROSOFT_WORD_DOC),
            (
                "org.openxmlformats.presentationml.template.macroenabled",
                ORG_OPENXMLFORMATS_PRESENTATIONML_TEMPLATE_MACROENABLED
            ),
            (
                "org.openxmlformats.presentationml.template",
                ORG_OPENXMLFORMATS_PRESENTATIONML_TEMPLATE
            ),
            (
                "org.openxmlformats.presentationml.slideshow.macroenabled",
                ORG_OPENXMLFORMATS_PRESENTATIONML_SLIDESHOW_MACROENABLED
            ),
            (
                "org.openxmlformats.presentationml.slideshow",
                ORG_OPENXMLFORMATS_PRESENTATIONML_SLIDESHOW
            ),
            (
                "org.openxmlformats.presentationml.presentation.macroenabled",
                ORG_OPENXMLFORMATS_PRESENTATIONML_PRESENTATION_MACROENABLED
            ),
            (
                "org.openxmlformats.presentationml.presentation",
                ORG_OPENXMLFORMATS_PRESENTATIONML_PRESENTATION
            ),
            (
                "org.openxmlformats.spreadsheetml.template.macroenabled",
                ORG_OPENXMLFORMATS_SPREADSHEETML_TEMPLATE_MACROENABLED
            ),
            (
                "org.openxmlformats.spreadsheetml.template",
                ORG_OPENXMLFORMATS_SPREADSHEETML_TEMPLATE
            ),
            (
                "com.microsoft.excel.sheet.binary.macroenabled",
                COM_MICROSOFT_EXCEL_SHEET_BINARY_MACROENABLED
            ),
            (
                "org.openxmlformats.spreadsheetml.sheet.macroenabled",
                ORG_OPENXMLFORMATS_SPREADSHEETML_SHEET_MACROENABLED
            ),
            (
                "org.openxmlformats.spreadsheetml.sheet",
                ORG_OPENXMLFORMATS_SPREADSHEETML_SHEET
            ),
            (
                "org.openxmlformats.wordprocessingml.template.macroenabled",
                ORG_OPENXMLFORMATS_WORDPROCESSINGML_TEMPLATE_MACROENABLED
            ),
            (
                "org.openxmlformats.wordprocessingml.template",
                ORG_OPENXMLFORMATS_WORDPROCESSINGML_TEMPLATE
            ),
            (
                "org.openxmlformats.wordprocessingml.document.macroenabled",
                ORG_OPENXMLFORMATS_WORDPROCESSINGML_DOCUMENT_MACROENABLED
            ),
            (
                "org.openxmlformats.wordprocessingml.document",
                ORG_OPENXMLFORMATS_WORDPROCESSINGML_DOCUMENT
            ),
            ("com.microsoft.word.wordml", COM_MICROSOFT_WORD_WORDML),
            (
                "org.oasis-open.opendocument.database",
                ORG_OASIS_OPEN_OPENDOCUMENT_DATABASE
            ),
            (
                "org.oasis-open.opendocument.text-web",
                ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_WEB
            ),
            (
                "org.oasis-open.opendocument.text-master",
                ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_MASTER
            ),
            ("org.openoffice.text-master", ORG_OPENOFFICE_TEXT_MASTER),
            (
                "org.oasis-open.opendocument.formula-template",
                ORG_OASIS_OPEN_OPENDOCUMENT_FORMULA_TEMPLATE
            ),
            (
                "org.oasis-open.opendocument.formula",
                ORG_OASIS_OPEN_OPENDOCUMENT_FORMULA
            ),
            ("org.openoffice.formula", ORG_OPENOFFICE_FORMULA),
            (
                "org.oasis-open.opendocument.image-template",
                ORG_OASIS_OPEN_OPENDOCUMENT_IMAGE_TEMPLATE
            ),
            (
                "org.oasis-open.opendocument.image",
                ORG_OASIS_OPEN_OPENDOCUMENT_IMAGE
            ),
            (
                "org.oasis-open.opendocument.chart-template",
                ORG_OASIS_OPEN_OPENDOCUMENT_CHART_TEMPLATE
            ),
            (
                "org.oasis-open.opendocument.chart",
                ORG_OASIS_OPEN_OPENDOCUMENT_CHART
            ),
            (
                "org.oasis-open.opendocument.spreadsheet-template",
                ORG_OASIS_OPEN_OPENDOCUMENT_SPREADSHEET_TEMPLATE
            ),
            (
                "org.oasis-open.opendocument.spreadsheet",
                ORG_OASIS_OPEN_OPENDOCUMENT_SPREADSHEET
            ),
            (
                "org.openoffice.spreadsheet-template",
                ORG_OPENOFFICE_SPREADSHEET_TEMPLATE
            ),
            ("org.openoffice.spreadsheet", ORG_OPENOFFICE_SPREADSHEET),
            (
                "org.oasis-open.opendocument.presentation-template",
                ORG_OASIS_OPEN_OPENDOCUMENT_PRESENTATION_TEMPLATE
            ),
            (
                "org.oasis-open.opendocument.presentation",
                ORG_OASIS_OPEN_OPENDOCUMENT_PRESENTATION
            ),
            (
                "org.openoffice.presentation-template",
                ORG_OPENOFFICE_PRESENTATION_TEMPLATE
            ),
            ("org.openoffice.presentation", ORG_OPENOFFICE_PRESENTATION),
            (
                "org.oasis-open.opendocument.graphics-template",
                ORG_OASIS_OPEN_OPENDOCUMENT_GRAPHICS_TEMPLATE
            ),
            (
                "org.oasis-open.opendocument.graphics",
                ORG_OASIS_OPEN_OPENDOCUMENT_GRAPHICS
            ),
            (
                "org.openoffice.graphics-template",
                ORG_OPENOFFICE_GRAPHICS_TEMPLATE
            ),
            ("org.openoffice.graphics", ORG_OPENOFFICE_GRAPHICS),
            (
                "org.oasis-open.opendocument.text-template",
                ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_TEMPLATE
            ),
            (
                "org.oasis-open.opendocument.text",
                ORG_OASIS_OPEN_OPENDOCUMENT_TEXT
            ),
            ("org.openoffice.text-template", ORG_OPENOFFICE_TEXT_TEMPLATE),
            ("org.openoffice.text", ORG_OPENOFFICE_TEXT),
            ("public.ofd", PUBLIC_OFD),
            ("org.webmproject.webm", ORG_WEBMPROJECT_WEBM),
            ("org.webmproject.webp", ORG_WEBMPROJECT_WEBP),
            ("com.microsoft.ico", COM_MICROSOFT_ICO),
            ("com.microsoft.bmp", COM_MICROSOFT_BMP),
            ("com.compuserve.gif", COM_COMPUSERVE_GIF),
            (
                "com.adobe.encapsulated-postscript",
                COM_ADOBE_ENCAPSULATED_POSTSCRIPT
            ),
            ("com.adobe.postscript", COM_ADOBE_POSTSCRIPT),
            ("com.adobe.fdf", COM_ADOBE_FDF),
            ("com.adobe.xfdf", COM_ADOBE_XFDF),
            ("com.adobe.etd", COM_ADOBE_ETD),
            ("com.adobe.edn", COM_ADOBE_EDN),
            ("com.adobe.pdf", COM_ADOBE_PDF)
        ])));
    pub(crate) static ref OTHER_FILENAME_EXTENSION_MAP: Arc<RwLock<HashMap<&'static str, &'static str>>> =
        Arc::new(RwLock::new(HashMap::from([
            (r#"groupactivity"#, "com.apple.groupactivities.activity"),
            (r#"mlpackage"#, "com.apple.coreml.mlpackage"),
            (r#"mlkitmodel"#, "com.apple.coreml.model"),
            (r#"mlmodel"#, "com.apple.coreml.model"),
            (r#"ahap"#, "com.apple.haptics.ahap"),
            (r#"cab"#, "com.microsoft.cab"),
            (r#"txz"#, "org.tukaani.tar-xz-archive"),
            (r#"xz"#, "org.tukaani.xz-archive"),
            (r#"7z"#, "org.7-zip.7-zip-archive"),
            (r#"theater"#, "com.apple.iMovieTheater"),
            (r#"imovielibrary"#, "com.apple.iMovieLibrary"),
            (r#"dae"#, "org.khronos.collada.digital-asset-exchange"),
            (r#"f4b"#, "com.adobe.flash.video"),
            (r#"f4a"#, "com.adobe.flash.video"),
            (r#"f4p"#, "com.adobe.flash.video"),
            (r#"f4v"#, "com.adobe.flash.video"),
            (r#"flv"#, "com.adobe.flash.video"),
            (r#"sea"#, "com.stuffit.archive.sit"),
            (r#"sit"#, "com.stuffit.archive.sit"),
            (r#"sidx"#, "com.stuffit.archive.sidx"),
            (r#"sitx"#, "com.stuffit.archive.sitx"),
            (r#"latm"#, "public.mp4a-loas"),
            (r#"loas"#, "public.mp4a-loas"),
            (r#"omf"#, "com.avid.open-media-framework"),
            (r#"flac"#, "org.xiph.flac"),
            (r#"sf2"#, "com.soundblaster.soundfont"),
            (r#"ra"#, "com.real.realaudio"),
            (r#"ram"#, "com.real.realaudio"),
            (r#"mxf"#, "org.smpte.mxf"),
            (r#"rmvb"#, "com.real.realmedia-vbr"),
            (r#"rm"#, "com.real.realmedia"),
            (r#"wax"#, "com.microsoft.windows-media-wax"),
            (r#"wvx"#, "com.microsoft.windows-media-wvx"),
            (r#"wmx"#, "com.microsoft.windows-media-wmx"),
            (r#"asx"#, "com.microsoft.advanced-stream-redirector"),
            (r#"wma"#, "com.microsoft.windows-media-wma"),
            (r#"wmp"#, "com.microsoft.windows-media-wmp"),
            (r#"wmv"#, "com.microsoft.windows-media-wmv"),
            (r#"wm"#, "com.microsoft.windows-media-wm"),
            (r#"asf"#, "com.microsoft.advanced-systems-format"),
            (r#"bwf"#, "com.microsoft.waveform-audio"),
            (r#"wave"#, "com.microsoft.waveform-audio"),
            (r#"wav"#, "com.microsoft.waveform-audio"),
            (r#"sd2"#, "com.digidesign.sd2-audio"),
            (r#"efx"#, "com.j2.efx-fax"),
            (r#"jfx"#, "com.j2.jfx-fax"),
            (r#"drawing"#, "com.apple.drawing"),
            (r#"avcs"#, "public.avcs"),
            (r#"heics"#, "public.heics"),
            (r#"heifs"#, "public.heifs"),
            (r#"avci"#, "public.avci"),
            (r#"heic"#, "public.heic"),
            (r#"hif"#, "public.heif"),
            (r#"heif"#, "public.heif"),
            (r#"fpx"#, "com.kodak.flashpix-image"),
            (r#"exr"#, "com.ilm.openexr-image"),
            (r#"sgi"#, "com.sgi.sgi-image"),
            (r#"tga"#, "com.truevision.tga-image"),
            (r#"ai"#, "com.adobe.illustrator.ai-image"),
            (r#"psb"#, "com.adobe.photoshop-large-image"),
            (r#"psd"#, "com.adobe.photoshop-image"),
            (r#"gbProj"#, "com.apple.garageband.project"),
            (r#"band"#, "com.apple.garageband.project"),
            (r#"nmbtemplate"#, "com.apple.iWork.Numbers.sfftemplate"),
            (r#"nmbtemplate"#, "com.apple.iWork.Numbers.template"),
            (r#"numbers"#, "com.apple.iWork.Numbers.sffnumbers"),
            (r#"numbers-tef"#, "com.apple.iWork.Numbers.numbers-tef"),
            (r#"numbers"#, "com.apple.iWork.Numbers.numbers"),
            (r#"template"#, "com.apple.iWork.Pages.sfftemplate"),
            (r#"template"#, "com.apple.iWork.Pages.template"),
            (r#"pages"#, "com.apple.iWork.Pages.sffpages"),
            (r#"pages-tef"#, "com.apple.iWork.Pages.pages-tef"),
            (r#"pages"#, "com.apple.iWork.Pages.pages"),
            (r#"kth"#, "com.apple.iWork.Keynote.sffkth"),
            (r#"kth"#, "com.apple.iWork.Keynote.kth"),
            (r#"key"#, "com.apple.iWork.Keynote.sffkey"),
            (r#"key-tef"#, "com.apple.iWork.Keynote.key-tef"),
            (r#"key"#, "com.apple.iWork.Keynote.key"),
            (r#"pps"#, "com.microsoft.powerpoint.pps"),
            (r#"pot"#, "com.microsoft.powerpoint.pot"),
            (r#"ppt"#, "com.microsoft.powerpoint.ppt"),
            (r#"xla"#, "com.microsoft.excel.xla"),
            (r#"xlw"#, "com.microsoft.excel.xlw"),
            (r#"xlt"#, "com.microsoft.excel.xlt"),
            (r#"xls"#, "com.microsoft.excel.xls"),
            (r#"dot"#, "com.microsoft.word.dot"),
            (r#"doc"#, "com.microsoft.word.doc"),
            (
                r#"potm"#,
                "org.openxmlformats.presentationml.template.macroenabled"
            ),
            (r#"potx"#, "org.openxmlformats.presentationml.template"),
            (
                r#"ppsm"#,
                "org.openxmlformats.presentationml.slideshow.macroenabled"
            ),
            (r#"ppsx"#, "org.openxmlformats.presentationml.slideshow"),
            (
                r#"pptm"#,
                "org.openxmlformats.presentationml.presentation.macroenabled"
            ),
            (r#"pptx"#, "org.openxmlformats.presentationml.presentation"),
            (
                r#"xltm"#,
                "org.openxmlformats.spreadsheetml.template.macroenabled"
            ),
            (r#"xltx"#, "org.openxmlformats.spreadsheetml.template"),
            (r#"xlsb"#, "com.microsoft.excel.sheet.binary.macroenabled"),
            (
                r#"xlsm"#,
                "org.openxmlformats.spreadsheetml.sheet.macroenabled"
            ),
            (r#"xlsx"#, "org.openxmlformats.spreadsheetml.sheet"),
            (
                r#"dotm"#,
                "org.openxmlformats.wordprocessingml.template.macroenabled"
            ),
            (r#"dotx"#, "org.openxmlformats.wordprocessingml.template"),
            (
                r#"docm"#,
                "org.openxmlformats.wordprocessingml.document.macroenabled"
            ),
            (r#"docx"#, "org.openxmlformats.wordprocessingml.document"),
            (r#"xml"#, "com.microsoft.word.wordml"),
            (r#"odb"#, "org.oasis-open.opendocument.database"),
            (r#"oth"#, "org.oasis-open.opendocument.text-web"),
            (r#"odm"#, "org.oasis-open.opendocument.text-master"),
            (r#"sxg"#, "org.openoffice.text-master"),
            (r#"otf"#, "org.oasis-open.opendocument.formula-template"),
            (r#"odf"#, "org.oasis-open.opendocument.formula"),
            (r#"smf"#, "org.openoffice.formula"),
            (r#"sxm"#, "org.openoffice.formula"),
            (r#"oti"#, "org.oasis-open.opendocument.image-template"),
            (r#"odi"#, "org.oasis-open.opendocument.image"),
            (r#"otc"#, "org.oasis-open.opendocument.chart-template"),
            (r#"odc"#, "org.oasis-open.opendocument.chart"),
            (r#"ots"#, "org.oasis-open.opendocument.spreadsheet-template"),
            (r#"ods"#, "org.oasis-open.opendocument.spreadsheet"),
            (r#"stc"#, "org.openoffice.spreadsheet-template"),
            (r#"sdc"#, "org.openoffice.spreadsheet"),
            (r#"sxc"#, "org.openoffice.spreadsheet"),
            (
                r#"otp"#,
                "org.oasis-open.opendocument.presentation-template"
            ),
            (r#"odp"#, "org.oasis-open.opendocument.presentation"),
            (r#"sti"#, "org.openoffice.presentation-template"),
            (r#"sdp"#, "org.openoffice.presentation"),
            (r#"sdd"#, "org.openoffice.presentation"),
            (r#"sxi"#, "org.openoffice.presentation"),
            (r#"otg"#, "org.oasis-open.opendocument.graphics-template"),
            (r#"odg"#, "org.oasis-open.opendocument.graphics"),
            (r#"std"#, "org.openoffice.graphics-template"),
            (r#"sda"#, "org.openoffice.graphics"),
            (r#"sxd"#, "org.openoffice.graphics"),
            (r#"ott"#, "org.oasis-open.opendocument.text-template"),
            (r#"odt"#, "org.oasis-open.opendocument.text"),
            (r#"stw"#, "org.openoffice.text-template"),
            (r#"sdw"#, "org.openoffice.text"),
            (r#"sxw"#, "org.openoffice.text"),
            (r#"ofd"#, "public.ofd"),
            (r#"webm"#, "org.webmproject.webm"),
            (r#"webp"#, "org.webmproject.webp"),
            (r#"ico"#, "com.microsoft.ico"),
            (r#"dib"#, "com.microsoft.bmp"),
            (r#"bmp"#, "com.microsoft.bmp"),
            (r#"gif"#, "com.compuserve.gif"),
            (r#"eps"#, "com.adobe.encapsulated-postscript"),
            (r#"ps"#, "com.adobe.postscript"),
            (r#"fdf"#, "com.adobe.fdf"),
            (r#"xfdf"#, "com.adobe.xfdf"),
            (r#"etd"#, "com.adobe.etd"),
            (r#"edn"#, "com.adobe.edn"),
            (r#"pdf"#, "com.adobe.pdf")
        ])));
    pub(crate) static ref OTHER_MIME_MAP: Arc<RwLock<HashMap<&'static str, &'static str>>> =
        Arc::new(RwLock::new(HashMap::from([
            (r#"application/pdf"#, "com.adobe.pdf"),
            (r#"application/postscript"#, "com.adobe.postscript"),
            (r#"image/gif"#, "com.compuserve.gif"),
            (r#"image/bmp"#, "com.microsoft.bmp"),
            (r#"image/vnd.microsoft.icon"#, "com.microsoft.ico"),
            (r#"image/webp"#, "org.webmproject.webp"),
            (r#"video/webm"#, "org.webmproject.webm"),
            (r#"audio/webm"#, "org.webmproject.webm"),
            (r#"application/ofd"#, "public.ofd"),
            (r#"application/vnd.sun.xml.writer"#, "org.openoffice.text"),
            (
                r#"application/vnd.stardivision.writer"#,
                "org.openoffice.text"
            ),
            (
                r#"application/vnd.sun.xml.writer.template"#,
                "org.openoffice.text-template"
            ),
            (
                r#"application/vnd.oasis.opendocument.text"#,
                "org.oasis-open.opendocument.text"
            ),
            (
                r#"application/vnd.oasis.opendocument.text-template"#,
                "org.oasis-open.opendocument.text-template"
            ),
            (r#"application/vnd.sun.xml.draw"#, "org.openoffice.graphics"),
            (
                r#"application/vnd.stardivision.draw"#,
                "org.openoffice.graphics"
            ),
            (
                r#"application/vnd.sun.xml.draw.template"#,
                "org.openoffice.graphics-template"
            ),
            (
                r#"application/vnd.oasis.opendocument.graphics"#,
                "org.oasis-open.opendocument.graphics"
            ),
            (
                r#"application/vnd.oasis.opendocument.graphics-template"#,
                "org.oasis-open.opendocument.graphics-template"
            ),
            (
                r#"application/vnd.sun.xml.impress"#,
                "org.openoffice.presentation"
            ),
            (
                r#"application/vnd.stardivision.impress"#,
                "org.openoffice.presentation"
            ),
            (
                r#"application/vnd.stardivision.impress-packed"#,
                "org.openoffice.presentation"
            ),
            (
                r#"application/vnd.sun.xml.impress.template"#,
                "org.openoffice.presentation-template"
            ),
            (
                r#"application/vnd.oasis.opendocument.presentation"#,
                "org.oasis-open.opendocument.presentation"
            ),
            (
                r#"application/vnd.oasis.opendocument.presentation-template"#,
                "org.oasis-open.opendocument.presentation-template"
            ),
            (
                r#"application/vnd.sun.xml.calc"#,
                "org.openoffice.spreadsheet"
            ),
            (
                r#"application/vnd.stardivision.calc"#,
                "org.openoffice.spreadsheet"
            ),
            (
                r#"application/vnd.sun.xml.calc.template"#,
                "org.openoffice.spreadsheet-template"
            ),
            (
                r#"application/vnd.oasis.opendocument.spreadsheet"#,
                "org.oasis-open.opendocument.spreadsheet"
            ),
            (
                r#"application/vnd.oasis.opendocument.spreadsheet-template"#,
                "org.oasis-open.opendocument.spreadsheet-template"
            ),
            (
                r#"application/vnd.oasis.opendocument.chart"#,
                "org.oasis-open.opendocument.chart"
            ),
            (
                r#"application/vnd.oasis.opendocument.chart-template"#,
                "org.oasis-open.opendocument.chart-template"
            ),
            (
                r#"application/vnd.oasis.opendocument.image"#,
                "org.oasis-open.opendocument.image"
            ),
            (
                r#"application/vnd.oasis.opendocument.image-template"#,
                "org.oasis-open.opendocument.image-template"
            ),
            (r#"application/vnd.sun.xml.math"#, "org.openoffice.formula"),
            (
                r#"application/vnd.stardivision.math"#,
                "org.openoffice.formula"
            ),
            (
                r#"application/vnd.oasis.opendocument.formula"#,
                "org.oasis-open.opendocument.formula"
            ),
            (
                r#"application/vnd.oasis.opendocument.formula-template"#,
                "org.oasis-open.opendocument.formula-template"
            ),
            (
                r#"application/vnd.sun.xml.writer.global"#,
                "org.openoffice.text-master"
            ),
            (
                r#"application/vnd.oasis.opendocument.text-master"#,
                "org.oasis-open.opendocument.text-master"
            ),
            (
                r#"application/vnd.oasis.opendocument.text-web"#,
                "org.oasis-open.opendocument.text-web"
            ),
            (
                r#"application/vnd.oasis.opendocument.database"#,
                "org.oasis-open.opendocument.database"
            ),
            (
                r#"application/vnd.openxmlformats-officedocument.wordprocessingml.document"#,
                "org.openxmlformats.wordprocessingml.document"
            ),
            (
                r#"application/vnd.ms-word.document.macroEnabled.12"#,
                "org.openxmlformats.wordprocessingml.document.macroenabled"
            ),
            (
                r#"application/vnd.openxmlformats-officedocument.wordprocessingml.template"#,
                "org.openxmlformats.wordprocessingml.template"
            ),
            (
                r#"application/vnd.ms-word.template.macroEnabled.12"#,
                "org.openxmlformats.wordprocessingml.template.macroenabled"
            ),
            (
                r#"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"#,
                "org.openxmlformats.spreadsheetml.sheet"
            ),
            (
                r#"application/vnd.ms-excel.sheet.macroEnabled.12"#,
                "org.openxmlformats.spreadsheetml.sheet.macroenabled"
            ),
            (
                r#"application/vnd.ms-excel.sheet.binary.macroEnabled.12"#,
                "com.microsoft.excel.sheet.binary.macroenabled"
            ),
            (
                r#"application/vnd.openxmlformats-officedocument.spreadsheetml.template"#,
                "org.openxmlformats.spreadsheetml.template"
            ),
            (
                r#"application/vnd.ms-excel.template.macroEnabled.12"#,
                "org.openxmlformats.spreadsheetml.template.macroenabled"
            ),
            (
                r#"application/vnd.openxmlformats-officedocument.presentationml.presentation"#,
                "org.openxmlformats.presentationml.presentation"
            ),
            (
                r#"application/vnd.ms-powerpoint.presentation.macroEnabled.12"#,
                "org.openxmlformats.presentationml.presentation.macroenabled"
            ),
            (
                r#"application/vnd.openxmlformats-officedocument.presentationml.slideshow"#,
                "org.openxmlformats.presentationml.slideshow"
            ),
            (
                r#"application/vnd.ms-powerpoint.slideshow.macroEnabled.12"#,
                "org.openxmlformats.presentationml.slideshow.macroenabled"
            ),
            (
                r#"application/vnd.openxmlformats-officedocument.presentationml.template"#,
                "org.openxmlformats.presentationml.template"
            ),
            (
                r#"application/vnd.ms-powerpoint.template.macroEnabled.12"#,
                "org.openxmlformats.presentationml.template.macroenabled"
            ),
            (r#"application/msword"#, "com.microsoft.word.doc"),
            (r#"application/msword"#, "com.microsoft.word.dot"),
            (r#"application/vnd.ms-excel"#, "com.microsoft.excel.xls"),
            (r#"application/msexcel"#, "com.microsoft.excel.xls"),
            (r#"application/vnd.ms-excel"#, "com.microsoft.excel.xlt"),
            (r#"application/msexcel"#, "com.microsoft.excel.xlt"),
            (r#"application/vnd.ms-excel"#, "com.microsoft.excel.xlw"),
            (r#"application/msexcel"#, "com.microsoft.excel.xlw"),
            (
                r#"application/vnd.ms-powerpoint"#,
                "com.microsoft.powerpoint.ppt"
            ),
            (
                r#"application/mspowerpoint"#,
                "com.microsoft.powerpoint.ppt"
            ),
            (
                r#"application/vnd.ms-powerpoint"#,
                "com.microsoft.powerpoint.pot"
            ),
            (
                r#"application/mspowerpoint"#,
                "com.microsoft.powerpoint.pot"
            ),
            (
                r#"application/vnd.ms-powerpoint"#,
                "com.microsoft.powerpoint.pps"
            ),
            (
                r#"application/mspowerpoint"#,
                "com.microsoft.powerpoint.pps"
            ),
            (
                r#"application/x-iwork-keynote-sffkey"#,
                "com.apple.iWork.Keynote.sffkey"
            ),
            (
                r#"application/x-iwork-keynote-sffkth"#,
                "com.apple.iWork.Keynote.sffkth"
            ),
            (
                r#"application/x-iwork-pages-sffpages"#,
                "com.apple.iWork.Pages.sffpages"
            ),
            (
                r#"application/x-iwork-pages-sfftemplate"#,
                "com.apple.iWork.Pages.sfftemplate"
            ),
            (
                r#"application/x-iwork-numbers-sffnumbers"#,
                "com.apple.iWork.Numbers.sffnumbers"
            ),
            (
                r#"application/x-iwork-numbers-sfftemplate"#,
                "com.apple.iWork.Numbers.sfftemplate"
            ),
            (r#"image/vnd.adobe.photoshop"#, "com.adobe.photoshop-image"),
            (r#"image/photoshop"#, "com.adobe.photoshop-image"),
            (r#"image/x-photoshop"#, "com.adobe.photoshop-image"),
            (r#"image/psd"#, "com.adobe.photoshop-image"),
            (r#"application/photoshop"#, "com.adobe.photoshop-image"),
            (r#"image/targa"#, "com.truevision.tga-image"),
            (r#"image/tga"#, "com.truevision.tga-image"),
            (r#"application/tga"#, "com.truevision.tga-image"),
            (r#"image/sgi"#, "com.sgi.sgi-image"),
            (r#"image/fpx"#, "com.kodak.flashpix-image"),
            (r#"application/vnd.fpx"#, "com.kodak.flashpix-image"),
            (r#"image/heif"#, "public.heif"),
            (r#"image/heic"#, "public.heic"),
            (r#"image/avci"#, "public.avci"),
            (r#"image/heif-sequence"#, "public.heifs"),
            (r#"image/heic-sequence"#, "public.heics"),
            (r#"image/avcs"#, "public.avcs"),
            (r#"image/efax"#, "com.j2.efx-fax"),
            (r#"audio/vnd.wave"#, "com.microsoft.waveform-audio"),
            (r#"audio/wav"#, "com.microsoft.waveform-audio"),
            (r#"audio/wave"#, "com.microsoft.waveform-audio"),
            (r#"audio/x-wav"#, "com.microsoft.waveform-audio"),
            (r#"video/x-ms-asf"#, "com.microsoft.advanced-systems-format"),
            (r#"video/x-ms-wm"#, "com.microsoft.windows-media-wm"),
            (r#"video/x-ms-wmv"#, "com.microsoft.windows-media-wmv"),
            (r#"video/x-ms-wmp"#, "com.microsoft.windows-media-wmp"),
            (r#"video/x-ms-wma"#, "com.microsoft.windows-media-wma"),
            (
                r#"video/x-ms-asx"#,
                "com.microsoft.advanced-stream-redirector"
            ),
            (r#"video/x-ms-wmx"#, "com.microsoft.windows-media-wmx"),
            (r#"video/x-ms-wvx"#, "com.microsoft.windows-media-wvx"),
            (r#"video/x-ms-wax"#, "com.microsoft.windows-media-wax"),
            (r#"application/vnd.rn-realmedia"#, "com.real.realmedia"),
            (
                r#"application/vnd.rn-realmedia-vbr"#,
                "com.real.realmedia-vbr"
            ),
            (r#"application/mxf"#, "org.smpte.mxf"),
            (r#"audio/vnd.rn-realaudio"#, "com.real.realaudio"),
            (r#"audio/x-pn-realaudio"#, "com.real.realaudio"),
            (r#"audio/x-realaudio"#, "com.real.realaudio"),
            (r#"audio/flac"#, "org.xiph.flac"),
            (r#"audio/mp4a-latm"#, "public.mp4a-latm"),
            (r#"application/x-stuffitx"#, "com.stuffit.archive.sitx"),
            (r#"application/x-sitx"#, "com.stuffit.archive.sitx"),
            (
                r#"application/x-stuffitx-index"#,
                "com.stuffit.archive.sidx"
            ),
            (r#"application/x-sitx-index"#, "com.stuffit.archive.sidx"),
            (r#"application/x-stuffit"#, "com.stuffit.archive.sit"),
            (r#"application/x-sit"#, "com.stuffit.archive.sit"),
            (r#"video/x-flv"#, "com.adobe.flash.video"),
            (r#"application/x-7z-compressed"#, "org.7-zip.7-zip-archive"),
            (r#"application/x-xz"#, "org.tukaani.xz-archive")
        ])));
}
pub const MIME_TYPE_TO_EXTENSION_VEC: [MIMETypeAndExtension; 89] = [
    MIMETypeAndExtension {
        mime_type: "audio/x-aiff",
        extensions: "aiff",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-msdownload",
        extensions: "dll",
    },
    MIMETypeAndExtension {
        mime_type: "chemical/x-xyz",
        extensions: "xyz",
    },
    MIMETypeAndExtension {
        mime_type: "application/msword",
        extensions: "doc",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.wap.wmlc",
        extensions: "wmlc",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-sv4cpio",
        extensions: "sv4cpio",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-ustar",
        extensions: "ustar",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-wais-source",
        extensions: "src",
    },
    MIMETypeAndExtension {
        mime_type: "text/sgml",
        extensions: "sgml|sgm",
    },
    MIMETypeAndExtension {
        mime_type: "application/octet-stream",
        extensions: "dms|lha|lzh|class|so|iso|fla",
    },
    MIMETypeAndExtension {
        mime_type: "text/vnd.wap.wmlscript",
        extensions: "wmls",
    },
    MIMETypeAndExtension {
        mime_type: "audio/mpeg",
        extensions: "mp3|mpga|mp2",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-csh",
        extensions: "csh",
    },
    MIMETypeAndExtension {
        mime_type: "text/x-setext",
        extensions: "etx",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.fdf",
        extensions: "fdf",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-futuresplash",
        extensions: "spl",
    },
    MIMETypeAndExtension {
        mime_type: "text/qif",
        extensions: "qif",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-portable-anymap",
        extensions: "pnm",
    },
    MIMETypeAndExtension {
        mime_type: "model/vrml",
        extensions: "wrl|vrml",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-sh",
        extensions: "sh",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.ms-excel",
        extensions: "xls",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-troff-ms",
        extensions: "ms",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-bittorrent",
        extensions: "torrent",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-cdlink",
        extensions: "vcd",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-shar",
        extensions: "shar",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-pcx",
        extensions: "pcx",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-troff",
        extensions: "t|tr|roff",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-diskcopy",
        extensions: "dmg",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-chess-pgn",
        extensions: "pgn",
    },
    MIMETypeAndExtension {
        mime_type: "text/html",
        extensions: "html|jhtml",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-xpixmap",
        extensions: "xpm",
    },
    MIMETypeAndExtension {
        mime_type: "application/oda",
        extensions: "oda",
    },
    MIMETypeAndExtension {
        mime_type: "audio/x-m4p",
        extensions: "m4p",
    },
    MIMETypeAndExtension {
        mime_type: "audio/x-pn-realaudio-plugin",
        extensions: "rpm",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-texinfo",
        extensions: "texinfo|texi",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-portable-bitmap",
        extensions: "pbm",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-olympus-or",
        extensions: "orf",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-xwindowdump",
        extensions: "xwd",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-targa",
        extensions: "targa",
    },
    MIMETypeAndExtension {
        mime_type: "audio/aiff",
        extensions: "aiff",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.adobe.xfdf",
        extensions: "xfdf",
    },
    MIMETypeAndExtension {
        mime_type: "application/msexcel",
        extensions: "xls",
    },
    MIMETypeAndExtension {
        mime_type: "image/ief",
        extensions: "ief",
    },
    MIMETypeAndExtension {
        mime_type: "text/xml",
        extensions: "xml|xsl",
    },
    MIMETypeAndExtension {
        mime_type: "model/mesh",
        extensions: "msh|mesh|silo",
    },
    MIMETypeAndExtension {
        mime_type: "text/plain",
        extensions: "txt|asc",
    },
    MIMETypeAndExtension {
        mime_type: "video/mp4",
        extensions: "mp4|mpg4",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-latex",
        extensions: "latex",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.mif",
        extensions: "mif",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-shockwave-flash",
        extensions: "swf",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-rgb",
        extensions: "rgb",
    },
    MIMETypeAndExtension {
        mime_type: "application/xml",
        extensions: "xml",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.wap.wbxml",
        extensions: "wbxml",
    },
    MIMETypeAndExtension {
        mime_type: "text/css",
        extensions: "css",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.adobe.xdp+xml",
        extensions: "xdp",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.wap.wmlscriptc",
        extensions: "wmlsc",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-portable-pixmap",
        extensions: "ppm",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-ms-wmd",
        extensions: "wmd",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-macpaint",
        extensions: "pnt|pntg|mac",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-hdf",
        extensions: "hdf",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-netcdf",
        extensions: "nc|cdf",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-portable-graymap",
        extensions: "pgm",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-cpio",
        extensions: "cpio",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-bcpio",
        extensions: "bcpio",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-tex",
        extensions: "tex",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-koan",
        extensions: "skp|skd|skt|skm",
    },
    MIMETypeAndExtension {
        mime_type: "application/mspowerpoint",
        extensions: "ppt",
    },
    MIMETypeAndExtension {
        mime_type: "x-conference/x-cooltalk",
        extensions: "ice",
    },
    MIMETypeAndExtension {
        mime_type: "text/vnd.wap.wml",
        extensions: "wml",
    },
    MIMETypeAndExtension {
        mime_type: "chemical/x-pdb",
        extensions: "pdb",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-sv4crc",
        extensions: "sv4crc",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-troff-me",
        extensions: "me",
    },
    MIMETypeAndExtension {
        mime_type: "application/mac-compactpro",
        extensions: "cpt",
    },
    MIMETypeAndExtension {
        mime_type: "application/andrew-inset",
        extensions: "ez",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-gzip",
        extensions: "gz|tgz|cpgz",
    },
    MIMETypeAndExtension {
        mime_type: "text/richtext",
        extensions: "rtx",
    },
    MIMETypeAndExtension {
        mime_type: "model/iges",
        extensions: "igs|iges",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-filemaker",
        extensions: "fp6|fp5|fp4|fp3|fp2|fp",
    },
    MIMETypeAndExtension {
        mime_type: "video/x-sgi-movie",
        extensions: "movie",
    },
    MIMETypeAndExtension {
        mime_type: "application/postscript",
        extensions: "ps|eps|ai",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-cmu-raster",
        extensions: "ras",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-tcl",
        extensions: "tcl",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.adobe.xfd+xml",
        extensions: "xfd",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-director",
        extensions: "dcr|dir|dxr",
    },
    MIMETypeAndExtension {
        mime_type: "video/vnd.mpegurl",
        extensions: "mxu",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-troff-man",
        extensions: "man",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.ms-powerpoint",
        extensions: "ppt",
    },
    MIMETypeAndExtension {
        mime_type: "image/vnd.wap.wbmp",
        extensions: "wbmp",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-dvi",
        extensions: "dvi",
    },
];
