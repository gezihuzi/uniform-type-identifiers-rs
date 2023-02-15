use crate::{MIMETypeAndExtension, UTType};

pub const PUBLIC_ITEM: UTType = UTType {
    identifier: "public.item",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "item",
};
pub const PUBLIC_DATA: UTType = UTType {
    identifier: "public.data",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{"public.mime-type": ["application/octet-stream"]}"#,
    description: "data",
};
pub const PUBLIC_DIRECTORY: UTType = UTType {
    identifier: "public.directory",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: "directory",
};
pub const PUBLIC_CONTENT: UTType = UTType {
    identifier: "public.content",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "content",
};
pub const PUBLIC_COMPOSITE_CONTENT: UTType = UTType {
    identifier: "public.composite-content",
    conforms_to: r#"["public.content"]"#,
    tags: r#"{}"#,
    description: "content",
};
pub const PUBLIC_NAMED_PIPE: UTType = UTType {
    identifier: "public.named-pipe",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const PUBLIC_CHARACTER_SPECIAL: UTType = UTType {
    identifier: "public.character-special",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const PUBLIC_BLOCK_SPECIAL: UTType = UTType {
    identifier: "public.block-special",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const PUBLIC_SOCKET: UTType = UTType {
    identifier: "public.socket",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const PUBLIC_EXECUTABLE: UTType = UTType {
    identifier: "public.executable",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "executable",
};
pub const PUBLIC_UNIX_EXECUTABLE: UTType = UTType {
    identifier: "public.unix-executable",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{}"#,
    description: "Unix executable",
};
pub const COM_APPLE_APPLICATION: UTType = UTType {
    identifier: "com.apple.application",
    conforms_to: r#"["public.executable"]"#,
    tags: r#"{}"#,
    description: "application",
};
pub const PUBLIC_ARCHIVE: UTType = UTType {
    identifier: "public.archive",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "file archive",
};
pub const PUBLIC_BOOKMARK: UTType = UTType {
    identifier: "public.bookmark",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Bookmark",
};
pub const PUBLIC_DATABASE: UTType = UTType {
    identifier: "public.database",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "database",
};
pub const COM_APPLE_CSSTORE: UTType = UTType {
    identifier: "com.apple.csstore",
    conforms_to: r#"["public.data", "public.database"]"#,
    tags: r#"{"public.filename-extension": ["csstore"]}"#,
    description: "",
};
pub const PUBLIC_PRESENTATION: UTType = UTType {
    identifier: "public.presentation",
    conforms_to: r#"["public.composite-content"]"#,
    tags: r#"{}"#,
    description: "presentation",
};
pub const PUBLIC_SPREADSHEET: UTType = UTType {
    identifier: "public.spreadsheet",
    conforms_to: r#"["public.content"]"#,
    tags: r#"{}"#,
    description: "Spreadsheet",
};
pub const COM_APPLE_ICLOUD: UTType = UTType {
    identifier: "com.apple.iCloud",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: "iCloud",
};
pub const PUBLIC_3D_CONTENT: UTType = UTType {
    identifier: "public.3d-content",
    conforms_to: r#"["public.content"]"#,
    tags: r#"{}"#,
    description: "3D Content",
};
pub const PUBLIC_ALEMBIC: UTType = UTType {
    identifier: "public.alembic",
    conforms_to: r#"["public.3d-content", "public.data"]"#,
    tags: r#"{"public.filename-extension": ["abc"]}"#,
    description: "Alembic 3D Scene",
};
pub const PUBLIC_GEOMETRY_DEFINITION_FORMAT: UTType = UTType {
    identifier: "public.geometry-definition-format",
    conforms_to: r#"["public.3d-content", "public.text"]"#,
    tags: r#"{"public.filename-extension": ["obj"]}"#,
    description: "Geometry Definition File Format",
};
pub const PUBLIC_STANDARD_TESSELATED_GEOMETRY_FORMAT: UTType = UTType {
    identifier: "public.standard-tesselated-geometry-format",
    conforms_to: r#"["public.3d-content", "public.data"]"#,
    tags: r#"{"public.filename-extension": ["stl"]}"#,
    description: "Standard Tesselated Geometry File Format",
};
pub const PUBLIC_POLYGON_FILE_FORMAT: UTType = UTType {
    identifier: "public.polygon-file-format",
    conforms_to: r#"["public.3d-content", "public.data"]"#,
    tags: r#"{"public.filename-extension": ["ply"]}"#,
    description: "Polygon File Format",
};
pub const COM_PIXAR_UNIVERSAL_SCENE_DESCRIPTION: UTType = UTType {
    identifier: "com.pixar.universal-scene-description",
    conforms_to: r#"["public.3d-content", "public.data"]"#,
    tags: r#"{"public.filename-extension": ["usd", "usda", "usdc"]}"#,
    description: "Universal Scene Description",
};
pub const COM_PIXAR_UNIVERSAL_SCENE_DESCRIPTION_MOBILE: UTType = UTType {
    identifier: "com.pixar.universal-scene-description-mobile",
    conforms_to: r#"["public.3d-content", "public.data"]"#,
    tags: r#"{}"#,
    description: "Universal Scene Description Package",
};
pub const COM_APPLE_REALITY: UTType = UTType {
    identifier: "com.apple.reality",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "Reality File",
};
pub const COM_APPLE_SCENEKIT_SCENE: UTType = UTType {
    identifier: "com.apple.scenekit.scene",
    conforms_to: r#"["public.data", "public.3d-content"]"#,
    tags: r#"{"public.filename-extension": ["scn", "scnz"]}"#,
    description: "SceneKit serialized format",
};
pub const COM_APPLE_AROBJECT: UTType = UTType {
    identifier: "com.apple.arobject",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "AR Reference Object",
};
pub const PUBLIC_MESSAGE: UTType = UTType {
    identifier: "public.message",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "message",
};
pub const PUBLIC_EMAIL_MESSAGE: UTType = UTType {
    identifier: "public.email-message",
    conforms_to: r#"["public.message"]"#,
    tags: r#"{}"#,
    description: "email message",
};
pub const PUBLIC_TO_DO_ITEM: UTType = UTType {
    identifier: "public.to-do-item",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "to-do item",
};
pub const PUBLIC_CALENDAR_EVENT: UTType = UTType {
    identifier: "public.calendar-event",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "calendar event",
};
pub const COM_APPLE_ICAL_VCS: UTType = UTType {
    identifier: "com.apple.ical.vcs",
    conforms_to: r#"["public.text", "public.item", "public.calendar-event"]"#,
    tags: r#"{"public.filename-extension": ["vcs", "vcal"], "public.mime-type": ["text/x-vcalendar"]}"#,
    description: "VCS File",
};
pub const COM_APPLE_ICAL_ICS: UTType = UTType {
    identifier: "com.apple.ical.ics",
    conforms_to: r#"["public.text", "public.item", "public.calendar-event"]"#,
    tags: r#"{"public.filename-extension": ["ics"], "public.mime-type": ["text/calendar"]}"#,
    description: "ICS File",
};
pub const PUBLIC_CONTACT: UTType = UTType {
    identifier: "public.contact",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "contact information",
};
pub const PUBLIC_VCARD: UTType = UTType {
    identifier: "public.vcard",
    conforms_to: r#"["public.text", "public.contact"]"#,
    tags: r#"{}"#,
    description: "electronic business card",
};
pub const COM_APPLE_SHAZAMSIGNATURE: UTType = UTType {
    identifier: "com.apple.shazamsignature",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["shazamsignature"]}"#,
    description: "Shazam Signature Data",
};
pub const COM_APPLE_SHAZAMCATALOG: UTType = UTType {
    identifier: "com.apple.shazamcatalog",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["shazamcatalog"]}"#,
    description: "Shazam Catalog",
};
pub const PUBLIC_TEXT: UTType = UTType {
    identifier: "public.text",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{}"#,
    description: "text",
};
pub const PUBLIC_PLAIN_TEXT: UTType = UTType {
    identifier: "public.plain-text",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const PUBLIC_UTF8_PLAIN_TEXT: UTType = UTType {
    identifier: "public.utf8-plain-text",
    conforms_to: r#"["public.plain-text"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const PUBLIC_UTF16_EXTERNAL_PLAIN_TEXT: UTType = UTType {
    identifier: "public.utf16-external-plain-text",
    conforms_to: r#"["public.plain-text"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const PUBLIC_UTF16_PLAIN_TEXT: UTType = UTType {
    identifier: "public.utf16-plain-text",
    conforms_to: r#"["public.plain-text"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_TRADITIONAL_MAC_PLAIN_TEXT: UTType = UTType {
    identifier: "com.apple.traditional-mac-plain-text",
    conforms_to: r#"["public.plain-text"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const PUBLIC_CASE_INSENSITIVE_TEXT: UTType = UTType {
    identifier: "public.case-insensitive-text",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const PUBLIC_LOG: UTType = UTType {
    identifier: "public.log",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: "Log file",
};
pub const COM_APPLE_LOG: UTType = UTType {
    identifier: "com.apple.log",
    conforms_to: r#"["public.plain-text", "public.log"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_SHUTDOWNSTALL: UTType = UTType {
    identifier: "com.apple.shutdownStall",
    conforms_to: r#"["public.utf8-plain-text", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["shutdownStall"]}"#,
    description: "Shutdown Stall",
};
pub const COM_APPLE_GPURESTART: UTType = UTType {
    identifier: "com.apple.gpuRestart",
    conforms_to: r#"["public.utf8-plain-text", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["gpuRestart"]}"#,
    description: "GPU Restart",
};
pub const COM_APPLE_CRASHREPORT: UTType = UTType {
    identifier: "com.apple.crashreport",
    conforms_to: r#"["public.utf8-plain-text", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["crash"]}"#,
    description: "Crash Report",
};
pub const COM_APPLE_HANGREPORT: UTType = UTType {
    identifier: "com.apple.hangreport",
    conforms_to: r#"["public.utf8-plain-text", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["hang"]}"#,
    description: "Hang Report",
};
pub const COM_APPLE_SPINREPORT: UTType = UTType {
    identifier: "com.apple.spinreport",
    conforms_to: r#"["public.utf8-plain-text", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["spin"]}"#,
    description: "Spin Report",
};
pub const COM_APPLE_PANICREPORT: UTType = UTType {
    identifier: "com.apple.panicreport",
    conforms_to: r#"["public.utf8-plain-text", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["panic"]}"#,
    description: "Panic Report",
};
pub const COM_APPLE_KTRACE: UTType = UTType {
    identifier: "com.apple.ktrace",
    conforms_to: r#"["public.data", "public.log"]"#,
    tags: r#"{"public.filename-extension": ["ktrace"]}"#,
    description: "Darwin kernel trace file",
};
pub const PUBLIC_FILENAME_EXTENSION: UTType = UTType {
    identifier: "public.filename-extension",
    conforms_to: r#"["public.case-insensitive-text"]"#,
    tags: r#"{}"#,
    description: "file name extension",
};
pub const PUBLIC_MIME_TYPE: UTType = UTType {
    identifier: "public.mime-type",
    conforms_to: r#"["public.case-insensitive-text"]"#,
    tags: r#"{}"#,
    description: "MIME type",
};
pub const COM_APPLE_OSTYPE: UTType = UTType {
    identifier: "com.apple.ostype",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{}"#,
    description: "four-character code",
};
pub const COM_APPLE_NSPBOARD_TYPE: UTType = UTType {
    identifier: "com.apple.nspboard-type",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{}"#,
    description: "NSPasteboard type",
};
pub const COM_APPLE_DEVICE_MODEL_CODE: UTType = UTType {
    identifier: "com.apple.device-model-code",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{}"#,
    description: "device model code",
};
pub const COM_APPLE_PASTEBOARD_PROMISED_FILE_URL: UTType = UTType {
    identifier: "com.apple.pasteboard.promised-file-url",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Promised file URL",
};
pub const COM_APPLE_PASTEBOARD_PROMISED_FILE_CONTENT_TYPE: UTType = UTType {
    identifier: "com.apple.pasteboard.promised-file-content-type",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Promised file content type",
};
pub const COM_APPLE_COCOA_PASTEBOARD_COLOR: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.color",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "color",
};
pub const COM_APPLE_COCOA_PASTEBOARD_SOUND: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.sound",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "sound",
};
pub const COM_APPLE_COCOA_PASTEBOARD_CHARACTER_FORMATTING: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.character-formatting",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "character format",
};
pub const COM_APPLE_COCOA_PASTEBOARD_PARAGRAPH_FORMATTING: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.paragraph-formatting",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "paragraph format",
};
pub const COM_APPLE_COCOA_PASTEBOARD_MULTIPLE_TEXT_SELECTION: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.multiple-text-selection",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "text selection",
};
pub const COM_APPLE_COCOA_PASTEBOARD_FIND_PANEL_SEARCH_OPTIONS: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.find-panel-search-options",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "search options",
};
pub const COM_APPLE_MAPKIT_MAP_ITEM: UTType = UTType {
    identifier: "com.apple.mapkit.map-item",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "Map Item",
};
pub const COM_APPLE_RESOLVABLE: UTType = UTType {
    identifier: "com.apple.resolvable",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "resolvable",
};
pub const PUBLIC_SYMLINK: UTType = UTType {
    identifier: "public.symlink",
    conforms_to: r#"["public.item", "com.apple.resolvable"]"#,
    tags: r#"{}"#,
    description: "symbolic link",
};
pub const COM_APPLE_MOUNT_POINT: UTType = UTType {
    identifier: "com.apple.mount-point",
    conforms_to: r#"["public.item", "com.apple.resolvable"]"#,
    tags: r#"{}"#,
    description: "mount point",
};
pub const COM_APPLE_BOOKMARK: UTType = UTType {
    identifier: "com.apple.bookmark",
    conforms_to: r#"["public.data", "com.apple.resolvable"]"#,
    tags: r#"{}"#,
    description: "bookmark",
};
pub const COM_APPLE_ALIAS_FILE: UTType = UTType {
    identifier: "com.apple.alias-file",
    conforms_to: r#"["public.data", "com.apple.resolvable"]"#,
    tags: r#"{}"#,
    description: "alias",
};
pub const COM_APPLE_ALIAS_RECORD: UTType = UTType {
    identifier: "com.apple.alias-record",
    conforms_to: r#"["public.data", "com.apple.resolvable"]"#,
    tags: r#"{}"#,
    description: "alias data",
};
pub const COM_APPLE_ICLOUD_FILE_FAULT: UTType = UTType {
    identifier: "com.apple.icloud-file-fault",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["icloud"]}"#,
    description: "iCloud synchronization file",
};
pub const COM_APPLE_FINDER_CLIPPING: UTType = UTType {
    identifier: "com.apple.finder.clipping",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "clipping",
};
pub const COM_APPLE_FINDER_SOUND_CLIPPING: UTType = UTType {
    identifier: "com.apple.finder.sound-clipping",
    conforms_to: r#"["com.apple.finder.clipping"]"#,
    tags: r#"{"public.filename-extension": ["sndClipping"]}"#,
    description: "Sound Clipping",
};
pub const COM_APPLE_FINDER_TEXTCLIPPING: UTType = UTType {
    identifier: "com.apple.finder.textclipping",
    conforms_to: r#"["com.apple.finder.clipping"]"#,
    tags: r#"{"public.filename-extension": ["textclipping"]}"#,
    description: "text clipping",
};
pub const COM_APPLE_FINDER_PICTCLIPPING: UTType = UTType {
    identifier: "com.apple.finder.pictclipping",
    conforms_to: r#"["com.apple.finder.clipping", "public.image"]"#,
    tags: r#"{}"#,
    description: "picture clipping",
};
pub const COM_APPLE_FINDER_BURN_FOLDER: UTType = UTType {
    identifier: "com.apple.finder.burn-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Burn Folder",
};
pub const COM_APPLE_ICONSET: UTType = UTType {
    identifier: "com.apple.iconset",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Icon Set",
};
pub const COM_APPLE_FINDER_SMART_FOLDER: UTType = UTType {
    identifier: "com.apple.finder.smart-folder",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "Smart Folder",
};
pub const COM_APPLE_FINDER_RECENT_ITEMS: UTType = UTType {
    identifier: "com.apple.finder.recent-items",
    conforms_to: r#"["com.apple.finder.smart-folder"]"#,
    tags: r#"{}"#,
    description: "Recent Items",
};
pub const PUBLIC_OBJECT_CODE: UTType = UTType {
    identifier: "public.object-code",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{}"#,
    description: "object code",
};
pub const COM_APPLE_MACH_O_BINARY: UTType = UTType {
    identifier: "com.apple.mach-o-binary",
    conforms_to: r#"["public.unix-executable"]"#,
    tags: r#"{}"#,
    description: "Mach-O binary",
};
pub const COM_APPLE_MACH_O_OBJECT: UTType = UTType {
    identifier: "com.apple.mach-o-object",
    conforms_to: r#"["com.apple.mach-o-binary"]"#,
    tags: r#"{}"#,
    description: "Mach-O object",
};
pub const COM_APPLE_MACH_O_EXECUTABLE: UTType = UTType {
    identifier: "com.apple.mach-o-executable",
    conforms_to: r#"["com.apple.mach-o-binary"]"#,
    tags: r#"{}"#,
    description: "Mach-O executable",
};
pub const COM_APPLE_X11_MACH_O_EXECUTABLE: UTType = UTType {
    identifier: "com.apple.x11-mach-o-executable",
    conforms_to: r#"["com.apple.mach-o-binary"]"#,
    tags: r#"{}"#,
    description: "X11 application",
};
pub const COM_APPLE_MACH_O_CORE: UTType = UTType {
    identifier: "com.apple.mach-o-core",
    conforms_to: r#"["com.apple.mach-o-binary"]"#,
    tags: r#"{}"#,
    description: "Mach-O core",
};
pub const COM_APPLE_MACH_O_DYLIB: UTType = UTType {
    identifier: "com.apple.mach-o-dylib",
    conforms_to: r#"["com.apple.mach-o-binary"]"#,
    tags: r#"{}"#,
    description: "Mach-O dynamic library",
};
pub const COM_APPLE_MACH_O_BUNDLE: UTType = UTType {
    identifier: "com.apple.mach-o-bundle",
    conforms_to: r#"["com.apple.mach-o-binary"]"#,
    tags: r#"{}"#,
    description: "Mach-O bundle",
};
pub const COM_APPLE_PEF_BINARY: UTType = UTType {
    identifier: "com.apple.pef-binary",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{}"#,
    description: "PEF binary",
};
pub const PUBLIC_ELF_BINARY: UTType = UTType {
    identifier: "public.elf-binary",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{}"#,
    description: "ELF binary",
};
pub const COM_MICROSOFT_WINDOWS_EXECUTABLE: UTType = UTType {
    identifier: "com.microsoft.windows-executable",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{}"#,
    description: "Microsoft Windows application",
};
pub const COM_MICROSOFT_WINDOWS_DYNAMIC_LINK_LIBRARY: UTType = UTType {
    identifier: "com.microsoft.windows-dynamic-link-library",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{}"#,
    description: "Microsoft dynamic link library",
};
pub const COM_SUN_JAVA_CLASS: UTType = UTType {
    identifier: "com.sun.java-class",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{}"#,
    description: "Java class",
};
pub const COM_SUN_JAVA_ARCHIVE: UTType = UTType {
    identifier: "com.sun.java-archive",
    conforms_to: r#"["public.zip-archive", "public.executable"]"#,
    tags: r#"{}"#,
    description: "Java archive",
};
pub const COM_SUN_WEB_APPLICATION_ARCHIVE: UTType = UTType {
    identifier: "com.sun.web-application-archive",
    conforms_to: r#"["com.sun.java-archive"]"#,
    tags: r#"{}"#,
    description: "web application archive",
};
pub const COM_APPLE_QUARTZ_COMPOSER_COMPOSITION: UTType = UTType {
    identifier: "com.apple.quartz-composer-composition",
    conforms_to: r#"["public.data", "public.executable"]"#,
    tags: r#"{}"#,
    description: "Quartz Composer compostion",
};
pub const COM_APPLE_BOM_ARCHIVE: UTType = UTType {
    identifier: "com.apple.bom-archive",
    conforms_to: r#"["public.archive"]"#,
    tags: r#"{}"#,
    description: "BOM-compatible archive",
};
pub const PUBLIC_DISK_IMAGE: UTType = UTType {
    identifier: "public.disk-image",
    conforms_to: r#"["public.archive"]"#,
    tags: r#"{}"#,
    description: "disk image",
};
pub const ORG_GNU_GNU_TAR_ARCHIVE: UTType = UTType {
    identifier: "org.gnu.gnu-tar-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["gtar"], "public.mime-type": ["application/x-gtar"]}"#,
    description: "GNU tar archive",
};
pub const PUBLIC_TAR_ARCHIVE: UTType = UTType {
    identifier: "public.tar-archive",
    conforms_to: r#"["org.gnu.gnu-tar-archive"]"#,
    tags: r#"{"public.filename-extension": ["tar"], "public.mime-type": ["application/x-tar", "application/tar"]}"#,
    description: "tar archive",
};
pub const ORG_GNU_GNU_ZIP_ARCHIVE: UTType = UTType {
    identifier: "org.gnu.gnu-zip-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["gz", "gzip"], "public.mime-type": ["application/x-gzip", "application/gzip"]}"#,
    description: "GZip archive",
};
pub const ORG_GNU_GNU_ZIP_TAR_ARCHIVE: UTType = UTType {
    identifier: "org.gnu.gnu-zip-tar-archive",
    conforms_to: r#"["org.gnu.gnu-zip-archive"]"#,
    tags: r#"{"public.filename-extension": ["tgz"]}"#,
    description: "gzip tar archive",
};
pub const PUBLIC_BZIP2_ARCHIVE: UTType = UTType {
    identifier: "public.bzip2-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["bz2", "bz"], "public.mime-type": ["application/x-bzip2", "application/x-bzip", "application/bzip2", "application/bzip", "application/x-bz2"]}"#,
    description: "Bzip2 archive",
};
pub const PUBLIC_TAR_BZIP2_ARCHIVE: UTType = UTType {
    identifier: "public.tar-bzip2-archive",
    conforms_to: r#"["public.bzip2-archive"]"#,
    tags: r#"{"public.filename-extension": ["tbz2", "tbz"]}"#,
    description: "Bzip2 compressed tar archive",
};
pub const COM_APPLE_BINHEX_ARCHIVE: UTType = UTType {
    identifier: "com.apple.binhex-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{}"#,
    description: "BinHex archive",
};
pub const COM_APPLE_MACBINARY_ARCHIVE: UTType = UTType {
    identifier: "com.apple.macbinary-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{}"#,
    description: "MacBinary archive",
};
pub const COM_APPLE_APPLESINGLE_ARCHIVE: UTType = UTType {
    identifier: "com.apple.applesingle-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["as"]}"#,
    description: "AppleSingle archive",
};
pub const PUBLIC_UUENCODED_ARCHIVE: UTType = UTType {
    identifier: "public.uuencoded-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["uu"], "public.mime-type": ["text/x-uuencode"]}"#,
    description: "UUEncoded archive",
};
pub const PUBLIC_Z_ARCHIVE: UTType = UTType {
    identifier: "public.z-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["z"], "public.mime-type": ["application/x-compress"]}"#,
    description: "Z archive",
};
pub const COM_APPLE_BOM_COMPRESSED_CPIO: UTType = UTType {
    identifier: "com.apple.bom-compressed-cpio",
    conforms_to: r#"["public.data", "com.apple.bom-archive"]"#,
    tags: r#"{}"#,
    description: "BOM-generated compressed CPIO archive",
};
pub const PUBLIC_CPIO_ARCHIVE: UTType = UTType {
    identifier: "public.cpio-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["cpio", "pax"]}"#,
    description: "CPIO archive",
};
pub const COM_PKWARE_ZIP_ARCHIVE: UTType = UTType {
    identifier: "com.pkware.zip-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{}"#,
    description: "PKZip archive",
};
pub const PUBLIC_ZIP_ARCHIVE: UTType = UTType {
    identifier: "public.zip-archive",
    conforms_to: r#"["com.pkware.zip-archive"]"#,
    tags: r#"{}"#,
    description: "Zip archive",
};
pub const COM_APPLE_XAR_ARCHIVE: UTType = UTType {
    identifier: "com.apple.xar-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{}"#,
    description: "XAR Archive",
};
pub const COM_APPLE_XIP_ARCHIVE: UTType = UTType {
    identifier: "com.apple.xip-archive",
    conforms_to: r#"["com.apple.xar-archive"]"#,
    tags: r#"{}"#,
    description: "XIP Secure Archive",
};
pub const COM_APPLE_INSTALLER_PACKAGE_ARCHIVE: UTType = UTType {
    identifier: "com.apple.installer-package-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["pkg", "mpkg"]}"#,
    description: "Installer package archive",
};
pub const COM_APPLE_ARCHIVE: UTType = UTType {
    identifier: "com.apple.archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["aar", "yaa"]}"#,
    description: "Apple Archive",
};
pub const COM_APPLE_ENCRYPTED_ARCHIVE: UTType = UTType {
    identifier: "com.apple.encrypted-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{"public.filename-extension": ["aea"]}"#,
    description: "Apple Encrypted Archive",
};
pub const PUBLIC_URL: UTType = UTType {
    identifier: "public.url",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "URL",
};
pub const PUBLIC_FILE_URL: UTType = UTType {
    identifier: "public.file-url",
    conforms_to: r#"["public.url"]"#,
    tags: r#"{}"#,
    description: "file URL",
};
pub const PUBLIC_URL_NAME: UTType = UTType {
    identifier: "public.url-name",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "URL name",
};
pub const PUBLIC_STORED_URL: UTType = UTType {
    identifier: "public.stored-url",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.internet-location",
    conforms_to: r#"["public.stored-url", "public.data"]"#,
    tags: r#"{}"#,
    description: "internet location",
};
pub const COM_APPLE_WEB_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.web-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["webloc"]}"#,
    description: "web internet location",
};
pub const COM_APPLE_VNC_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.vnc-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["vncloc"]}"#,
    description: "VNC Internet Location",
};
pub const COM_APPLE_MAIL_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.mail-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["mailloc"]}"#,
    description: "mail internet location",
};
pub const COM_APPLE_AFP_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.afp-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["afploc"]}"#,
    description: "AFP internet location",
};
pub const COM_APPLE_FILE_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.file-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["fileloc"]}"#,
    description: "file internet location",
};
pub const COM_APPLE_FTP_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.ftp-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["ftploc"]}"#,
    description: "FTP internet location",
};
pub const COM_APPLE_NEWS_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.news-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["newsloc"]}"#,
    description: "news internet location",
};
pub const COM_APPLE_GENERIC_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.generic-internet-location",
    conforms_to: r#"["com.apple.internet-location"]"#,
    tags: r#"{"public.filename-extension": ["inetloc"]}"#,
    description: "internet location",
};
pub const COM_MICROSOFT_INTERNET_SHORTCUT: UTType = UTType {
    identifier: "com.microsoft.internet-shortcut",
    conforms_to: r#"["public.stored-url", "public.data"]"#,
    tags: r#"{"public.filename-extension": ["url"]}"#,
    description: "Windows Internet shortcut",
};
pub const COM_APPLE_ITUNES_STORE_URL: UTType = UTType {
    identifier: "com.apple.itunes.store-url",
    conforms_to: r#"["public.url"]"#,
    tags: r#"{}"#,
    description: "iTunes store URL",
};
pub const PUBLIC_DELIMITED_VALUES_TEXT: UTType = UTType {
    identifier: "public.delimited-values-text",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{}"#,
    description: "delimited text values",
};
pub const PUBLIC_COMMA_SEPARATED_VALUES_TEXT: UTType = UTType {
    identifier: "public.comma-separated-values-text",
    conforms_to: r#"["public.plain-text", "public.delimited-values-text"]"#,
    tags: r#"{"public.filename-extension": ["csv"], "public.mime-type": ["text/csv", "text/comma-separated-values"]}"#,
    description: "comma-separated values",
};
pub const PUBLIC_TAB_SEPARATED_VALUES_TEXT: UTType = UTType {
    identifier: "public.tab-separated-values-text",
    conforms_to: r#"["public.plain-text", "public.delimited-values-text"]"#,
    tags: r#"{"public.filename-extension": ["tsv"], "public.mime-type": ["text/tab-separated-values"]}"#,
    description: "tab-separated values",
};
pub const PUBLIC_UTF8_TAB_SEPARATED_VALUES_TEXT: UTType = UTType {
    identifier: "public.utf8-tab-separated-values-text",
    conforms_to: r#"["public.tab-separated-values-text", "public.utf8-plain-text"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const PUBLIC_RTF: UTType = UTType {
    identifier: "public.rtf",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{}"#,
    description: "rich text (RTF)",
};
pub const PUBLIC_HTML: UTType = UTType {
    identifier: "public.html",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{}"#,
    description: "HTML text",
};
pub const PUBLIC_XML: UTType = UTType {
    identifier: "public.xml",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{}"#,
    description: "XML text",
};
pub const PUBLIC_XHTML: UTType = UTType {
    identifier: "public.xhtml",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{}"#,
    description: "XHTML",
};
pub const PUBLIC_RSS: UTType = UTType {
    identifier: "public.rss",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{"public.filename-extension": ["rss"], "public.mime-type": ["application/rss+xml"]}"#,
    description: "RSS web feed",
};
pub const PUBLIC_XFD: UTType = UTType {
    identifier: "public.xfd",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{}"#,
    description: "XML Form (XFD)",
};
pub const PUBLIC_CSS: UTType = UTType {
    identifier: "public.css",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["css"], "public.mime-type": ["text/css"]}"#,
    description: "CSS",
};
pub const PUBLIC_PATCH_FILE: UTType = UTType {
    identifier: "public.patch-file",
    conforms_to: r#"["public.plain-text"]"#,
    tags: r#"{"public.filename-extension": ["patch", "diff"]}"#,
    description: "patch file",
};
pub const PUBLIC_JSON: UTType = UTType {
    identifier: "public.json",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["json"], "public.mime-type": ["application/json"]}"#,
    description: "JSON",
};
pub const PUBLIC_NDJSON: UTType = UTType {
    identifier: "public.ndjson",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["ndjson"], "public.mime-type": ["application/x-ndjson"]}"#,
    description: "NDJSON",
};
pub const PUBLIC_YAML: UTType = UTType {
    identifier: "public.yaml",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["yml", "yaml"], "public.mime-type": ["application/x-yaml"]}"#,
    description: "YAML",
};
pub const COM_SCENARIST_CLOSED_CAPTION: UTType = UTType {
    identifier: "com.scenarist.closed-caption",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["scc"]}"#,
    description: "Scenarist Closed Caption",
};
pub const ORG_W3_WEBVTT: UTType = UTType {
    identifier: "org.w3.webvtt",
    conforms_to: r#"["public.text"]"#,
    tags: r#"{"public.filename-extension": ["vtt"], "public.mime-type": ["text/vtt"]}"#,
    description: "WebVTT Format",
};
pub const COM_APPLE_GENERIC_STATIONERY: UTType = UTType {
    identifier: "com.apple.generic-stationery",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "Stationery",
};
pub const COM_APPLE_PROPERTY_LIST: UTType = UTType {
    identifier: "com.apple.property-list",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["plist"]}"#,
    description: "property list",
};
pub const COM_APPLE_XML_PROPERTY_LIST: UTType = UTType {
    identifier: "com.apple.xml-property-list",
    conforms_to: r#"["public.xml", "com.apple.property-list"]"#,
    tags: r#"{"public.filename-extension": ["plist"]}"#,
    description: "XML property list",
};
pub const COM_APPLE_BINARY_PROPERTY_LIST: UTType = UTType {
    identifier: "com.apple.binary-property-list",
    conforms_to: r#"["com.apple.property-list"]"#,
    tags: r#"{"public.filename-extension": ["plist"]}"#,
    description: "binary property list",
};
pub const COM_APPLE_ASCII_PROPERTY_LIST: UTType = UTType {
    identifier: "com.apple.ascii-property-list",
    conforms_to: r#"["public.text", "com.apple.property-list"]"#,
    tags: r#"{"public.filename-extension": ["plist"]}"#,
    description: "ascii property list",
};
pub const PUBLIC_SOURCE_CODE: UTType = UTType {
    identifier: "public.source-code",
    conforms_to: r#"["public.plain-text"]"#,
    tags: r#"{}"#,
    description: "source code",
};
pub const PUBLIC_SOURCE_CODE_PREPROCESSED: UTType = UTType {
    identifier: "public.source-code.preprocessed",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{}"#,
    description: "preprocessed source code",
};
pub const PUBLIC_C_SOURCE: UTType = UTType {
    identifier: "public.c-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{}"#,
    description: "C source code",
};
pub const PUBLIC_C_SOURCE_PREPROCESSED: UTType = UTType {
    identifier: "public.c-source.preprocessed",
    conforms_to: r#"["public.c-source", "public.source-code.preprocessed"]"#,
    tags: r#"{}"#,
    description: "preprocessed C source code",
};
pub const COM_APPLE_IIG_SOURCE: UTType = UTType {
    identifier: "com.apple.iig-source",
    conforms_to: r#"["public.c-source"]"#,
    tags: r#"{}"#,
    description: "IOKit Interface Generator source code",
};
pub const PUBLIC_OBJECTIVE_C_SOURCE: UTType = UTType {
    identifier: "public.objective-c-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{}"#,
    description: "Objective-C source code",
};
pub const PUBLIC_OBJECTIVE_C_SOURCE_PREPROCESSED: UTType = UTType {
    identifier: "public.objective-c-source.preprocessed",
    conforms_to: r#"["public.objective-c-source", "public.source-code.preprocessed"]"#,
    tags: r#"{}"#,
    description: "preprocessed Objective-C source code",
};
pub const PUBLIC_C_PLUS_PLUS_SOURCE: UTType = UTType {
    identifier: "public.c-plus-plus-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["cp", "cpp", "c++", "cc", "cxx"]}"#,
    description: "C++ source code",
};
pub const PUBLIC_C_PLUS_PLUS_SOURCE_PREPROCESSED: UTType = UTType {
    identifier: "public.c-plus-plus-source.preprocessed",
    conforms_to: r#"["public.c-plus-plus-source", "public.source-code.preprocessed"]"#,
    tags: r#"{"public.filename-extension": ["ii"]}"#,
    description: "preprocessed C++ source code",
};
pub const PUBLIC_OBJECTIVE_C_PLUS_PLUS_SOURCE: UTType = UTType {
    identifier: "public.objective-c-plus-plus-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{}"#,
    description: "Objective-C++ source code",
};
pub const PUBLIC_OBJECTIVE_C_PLUS_PLUS_SOURCE_PREPROCESSED: UTType = UTType {
    identifier: "public.objective-c-plus-plus-source.preprocessed",
    conforms_to: r#"["public.objective-c-plus-plus-source", "public.source-code.preprocessed"]"#,
    tags: r#"{}"#,
    description: "preprocessed Objective-C++ source code",
};
pub const PUBLIC_C_HEADER: UTType = UTType {
    identifier: "public.c-header",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{}"#,
    description: "C header code",
};
pub const PUBLIC_PRECOMPILED_C_HEADER: UTType = UTType {
    identifier: "public.precompiled-c-header",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{}"#,
    description: "precompiled C header",
};
pub const PUBLIC_C_PLUS_PLUS_HEADER: UTType = UTType {
    identifier: "public.c-plus-plus-header",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["hh", "hp", "hpp", "hxx", "h++", "ipp"]}"#,
    description: "C++ header code",
};
pub const PUBLIC_C_PLUS_PLUS_INLINE_HEADER: UTType = UTType {
    identifier: "public.c-plus-plus-inline-header",
    conforms_to: r#"["public.c-plus-plus-header"]"#,
    tags: r#"{"public.filename-extension": ["inl"]}"#,
    description: "C++ Inline Header",
};
pub const PUBLIC_PRECOMPILED_C_PLUS_PLUS_HEADER: UTType = UTType {
    identifier: "public.precompiled-c-plus-plus-header",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{}"#,
    description: "precompiled C++ header",
};
pub const PUBLIC_SWIFT_SOURCE: UTType = UTType {
    identifier: "public.swift-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["swift"]}"#,
    description: "Swift Source Code",
};
pub const COM_SUN_JAVA_SOURCE: UTType = UTType {
    identifier: "com.sun.java-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["java", "jav"]}"#,
    description: "Java source code",
};
pub const PUBLIC_SCRIPT: UTType = UTType {
    identifier: "public.script",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{}"#,
    description: "script",
};
pub const PUBLIC_ASSEMBLY_SOURCE: UTType = UTType {
    identifier: "public.assembly-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["s"]}"#,
    description: "assembly source code",
};
pub const COM_APPLE_REZ_SOURCE: UTType = UTType {
    identifier: "com.apple.rez-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["r"]}"#,
    description: "Rez source code",
};
pub const PUBLIC_LEX_SOURCE: UTType = UTType {
    identifier: "public.lex-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["l", "lm", "lmm", "lpp", "lxx", "ll"]}"#,
    description: "Lex source code",
};
pub const PUBLIC_YACC_SOURCE: UTType = UTType {
    identifier: "public.yacc-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["y", "ym", "ymm", "ypp", "yxx", "yy"]}"#,
    description: "Yacc source code",
};
pub const PUBLIC_MIG_SOURCE: UTType = UTType {
    identifier: "public.mig-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["defs", "mig"]}"#,
    description: "Mig definition source code",
};
pub const COM_APPLE_SYMBOL_EXPORT: UTType = UTType {
    identifier: "com.apple.symbol-export",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["exp"]}"#,
    description: "symbol export list",
};
pub const PUBLIC_FORTRAN_SOURCE: UTType = UTType {
    identifier: "public.fortran-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["f", "for"]}"#,
    description: "Fortran source code",
};
pub const PUBLIC_FORTRAN_77_SOURCE: UTType = UTType {
    identifier: "public.fortran-77-source",
    conforms_to: r#"["public.fortran-source"]"#,
    tags: r#"{"public.filename-extension": ["f77"]}"#,
    description: "",
};
pub const PUBLIC_FORTRAN_90_SOURCE: UTType = UTType {
    identifier: "public.fortran-90-source",
    conforms_to: r#"["public.fortran-source"]"#,
    tags: r#"{"public.filename-extension": ["f90"]}"#,
    description: "",
};
pub const PUBLIC_FORTRAN_95_SOURCE: UTType = UTType {
    identifier: "public.fortran-95-source",
    conforms_to: r#"["public.fortran-source"]"#,
    tags: r#"{"public.filename-extension": ["f95"]}"#,
    description: "",
};
pub const PUBLIC_PASCAL_SOURCE: UTType = UTType {
    identifier: "public.pascal-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["pas"]}"#,
    description: "Pascal source code",
};
pub const PUBLIC_ADA_SOURCE: UTType = UTType {
    identifier: "public.ada-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["ada", "adb", "ads"]}"#,
    description: "Ada source code",
};
pub const PUBLIC_DYLAN_SOURCE: UTType = UTType {
    identifier: "public.dylan-source",
    conforms_to: r#"["public.source-code"]"#,
    tags: r#"{"public.filename-extension": ["dlyan", "lid"]}"#,
    description: "Dylan source code",
};
pub const COM_NETSCAPE_JAVASCRIPT_SOURCE: UTType = UTType {
    identifier: "com.netscape.javascript-source",
    conforms_to: r#"["public.script", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["js", "jscript", "javascript", "mjs"], "public.mime-type": ["text/javascript"]}"#,
    description: "JavaScript",
};
pub const COM_APPLE_XCODE_DSYM: UTType = UTType {
    identifier: "com.apple.xcode.dsym",
    conforms_to: r#"["com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["dsym"]}"#,
    description: "",
};
pub const PUBLIC_SHELL_SCRIPT: UTType = UTType {
    identifier: "public.shell-script",
    conforms_to: r#"["public.script"]"#,
    tags: r#"{"public.filename-extension": ["sh"]}"#,
    description: "shell script",
};
pub const PUBLIC_BASH_SCRIPT: UTType = UTType {
    identifier: "public.bash-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["bash"]}"#,
    description: "Bourne-Again Shell script",
};
pub const PUBLIC_CSH_SCRIPT: UTType = UTType {
    identifier: "public.csh-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["csh"]}"#,
    description: "C Shell script",
};
pub const PUBLIC_KSH_SCRIPT: UTType = UTType {
    identifier: "public.ksh-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["ksh"]}"#,
    description: "Korn Shell script",
};
pub const PUBLIC_TCSH_SCRIPT: UTType = UTType {
    identifier: "public.tcsh-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["tcsh"]}"#,
    description: "Tenex C Shell script",
};
pub const PUBLIC_ZSH_SCRIPT: UTType = UTType {
    identifier: "public.zsh-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["zsh"]}"#,
    description: "Z Shell script",
};
pub const PUBLIC_PERL_SCRIPT: UTType = UTType {
    identifier: "public.perl-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["pl", "pm"], "public.mime-type": ["text/x-perl-script"]}"#,
    description: "Perl script",
};
pub const PUBLIC_PYTHON_SCRIPT: UTType = UTType {
    identifier: "public.python-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["py"], "public.mime-type": ["text/x-python-script"]}"#,
    description: "Python script",
};
pub const PUBLIC_RUBY_SCRIPT: UTType = UTType {
    identifier: "public.ruby-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["rb", "rbw"], "public.mime-type": ["text/x-ruby-script"]}"#,
    description: "Ruby script",
};
pub const PUBLIC_PHP_SCRIPT: UTType = UTType {
    identifier: "public.php-script",
    conforms_to: r#"["public.shell-script"]"#,
    tags: r#"{"public.filename-extension": ["php", "php3", "php4", "ph3", "ph4", "phtml"], "public.mime-type": ["text/php", "text/x-php-script", "application/php"]}"#,
    description: "PHP script",
};
pub const COM_SUN_JAVA_WEB_START: UTType = UTType {
    identifier: "com.sun.java-web-start",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{}"#,
    description: "Java web start",
};
pub const PUBLIC_MAKE_SOURCE: UTType = UTType {
    identifier: "public.make-source",
    conforms_to: r#"["public.script"]"#,
    tags: r#"{"public.filename-extension": ["make", "mak", "gmk", "mk"]}"#,
    description: "Makefile",
};
pub const PUBLIC_IMAGE: UTType = UTType {
    identifier: "public.image",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{}"#,
    description: "image",
};
pub const COM_APPLE_LIVE_PHOTO: UTType = UTType {
    identifier: "com.apple.live-photo",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Live Photo",
};
pub const COM_APPLE_PRIVATE_LIVE_PHOTO_BUNDLE: UTType = UTType {
    identifier: "com.apple.private.live-photo-bundle",
    conforms_to: r#"["com.apple.live-photo", "com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["pvt"]}"#,
    description: "",
};
pub const PUBLIC_FAX: UTType = UTType {
    identifier: "public.fax",
    conforms_to: r#"["public.image", "public.message"]"#,
    tags: r#"{}"#,
    description: "fax",
};
pub const PUBLIC_CAMERA_RAW_IMAGE: UTType = UTType {
    identifier: "public.camera-raw-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "camera raw image",
};
pub const PUBLIC_JPEG: UTType = UTType {
    identifier: "public.jpeg",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "JPEG image",
};
pub const PUBLIC_JPEG_2000: UTType = UTType {
    identifier: "public.jpeg-2000",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "JPEG 2000 image",
};
pub const PUBLIC_TIFF: UTType = UTType {
    identifier: "public.tiff",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "TIFF image",
};
pub const COM_APPLE_PICT: UTType = UTType {
    identifier: "com.apple.pict",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "QuickDraw picture",
};
pub const COM_APPLE_MACPAINT_IMAGE: UTType = UTType {
    identifier: "com.apple.macpaint-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["pntg"]}"#,
    description: "MacPaint image",
};
pub const PUBLIC_PNG: UTType = UTType {
    identifier: "public.png",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "PNG image",
};
pub const PUBLIC_SVG_IMAGE: UTType = UTType {
    identifier: "public.svg-image",
    conforms_to: r#"["public.image", "public.xml"]"#,
    tags: r#"{"public.filename-extension": ["svg", "svgz"], "public.mime-type": ["image/svg+xml"]}"#,
    description: "SVG image",
};
pub const COM_APPLE_QUICKTIME_IMAGE: UTType = UTType {
    identifier: "com.apple.quicktime-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "QuickTime image",
};
pub const COM_APPLE_ICNS: UTType = UTType {
    identifier: "com.apple.icns",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "Apple icon image",
};
pub const PUBLIC_XBITMAP_IMAGE: UTType = UTType {
    identifier: "public.xbitmap-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "X bitmap image",
};
pub const PUBLIC_MPO_IMAGE: UTType = UTType {
    identifier: "public.mpo-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "Multiple Picture Object image",
};
pub const CA_MCGILL_MNI_BIC_MNC: UTType = UTType {
    identifier: "ca.mcgill.mni.bic.mnc",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["mnc", "minc"]}"#,
    description: "MINC Image",
};
pub const ORG_NEMA_DICOM: UTType = UTType {
    identifier: "org.nema.dicom",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["dcm", "dicom"], "public.mime-type": ["application/dicom"]}"#,
    description: "DICOM",
};
pub const GOV_NIH_NIFTI_1: UTType = UTType {
    identifier: "gov.nih.nifti-1",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["nii"]}"#,
    description: "NIfTI-1",
};
pub const PUBLIC_AUDIOVISUAL_CONTENT: UTType = UTType {
    identifier: "public.audiovisual-content",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{}"#,
    description: "audiovisual content",
};
pub const PUBLIC_MOVIE: UTType = UTType {
    identifier: "public.movie",
    conforms_to: r#"["public.audiovisual-content"]"#,
    tags: r#"{}"#,
    description: "movie",
};
pub const PUBLIC_VIDEO: UTType = UTType {
    identifier: "public.video",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{}"#,
    description: "video",
};
pub const PUBLIC_AUDIO: UTType = UTType {
    identifier: "public.audio",
    conforms_to: r#"["public.audiovisual-content"]"#,
    tags: r#"{}"#,
    description: "audio",
};
pub const COM_APPLE_QUICKTIME_MOVIE: UTType = UTType {
    identifier: "com.apple.quicktime-movie",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{}"#,
    description: "QuickTime movie",
};
pub const PUBLIC_MPEG: UTType = UTType {
    identifier: "public.mpeg",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["mpg", "mpeg", "mpe", "m75", "m15"], "public.mime-type": ["video/mpeg", "video/mpg", "video/x-mpeg", "video/x-mpg"]}"#,
    description: "MPEG movie",
};
pub const PUBLIC_MPEG_2_VIDEO: UTType = UTType {
    identifier: "public.mpeg-2-video",
    conforms_to: r#"["public.video"]"#,
    tags: r#"{"public.filename-extension": ["m2v"], "public.mime-type": ["video/mpeg2", "video/mpeg2-video"]}"#,
    description: "MPEG-2 video",
};
pub const PUBLIC_MPEG_4: UTType = UTType {
    identifier: "public.mpeg-4",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["mp4", "mpg4"], "public.mime-type": ["video/mp4", "video/mp4v-es"]}"#,
    description: "MPEG-4 movie",
};
pub const COM_APPLE_M4V_VIDEO: UTType = UTType {
    identifier: "com.apple.m4v-video",
    conforms_to: r#"["public.mpeg-4"]"#,
    tags: r#"{"public.filename-extension": ["m4v"], "public.mime-type": ["video/x-m4v"]}"#,
    description: "Apple MPEG-4 movie",
};
pub const COM_APPLE_PROTECTED_MPEG_4_VIDEO: UTType = UTType {
    identifier: "com.apple.protected-mpeg-4-video",
    conforms_to: r#"["com.apple.m4v-video"]"#,
    tags: r#"{}"#,
    description: "protected MPEG-4 movie",
};
pub const PUBLIC_DV_MOVIE: UTType = UTType {
    identifier: "public.dv-movie",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["dv", "dif"], "public.mime-type": ["video/x-dv"]}"#,
    description: "DV movie",
};
pub const PUBLIC_AVI: UTType = UTType {
    identifier: "public.avi",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["avi", "vfw"], "public.mime-type": ["video/avi", "video/msvideo", "video/x-msvideo"]}"#,
    description: "AVI movie",
};
pub const PUBLIC_3GPP: UTType = UTType {
    identifier: "public.3gpp",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["3gp", "3gpp", "sdv"], "public.mime-type": ["video/3gpp", "audio/3gpp"]}"#,
    description: "3GPP movie",
};
pub const PUBLIC_3GPP2: UTType = UTType {
    identifier: "public.3gpp2",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["3g2", "3gp2"], "public.mime-type": ["video/3gpp2", "audio/3gpp2"]}"#,
    description: "3GPP2 movie",
};
pub const PUBLIC_FLC_ANIMATION: UTType = UTType {
    identifier: "public.flc-animation",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["flc", "fli", "cel"], "public.mime-type": ["video/flc"]}"#,
    description: "FLC animation",
};
pub const PUBLIC_MPEG_2_TRANSPORT_STREAM: UTType = UTType {
    identifier: "public.mpeg-2-transport-stream",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["ts"]}"#,
    description: "MPEG-2 Transport Stream",
};
pub const PUBLIC_AUDIOVISUAL_CONTENT_COLLECTION: UTType = UTType {
    identifier: "public.audiovisual-content-collection",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Audiovisual Collection",
};
pub const PUBLIC_AVCHD_COLLECTION: UTType = UTType {
    identifier: "public.avchd-collection",
    conforms_to: r#"["com.apple.package", "public.audiovisual-content-collection"]"#,
    tags: r#"{"public.filename-extension": ["avchd"]}"#,
    description: "AVCHD Collection",
};
pub const COM_APPLE_AUDIO_UNIT_PRESET: UTType = UTType {
    identifier: "com.apple.audio-unit-preset",
    conforms_to: r#"["com.apple.xml-property-list"]"#,
    tags: r#"{"public.filename-extension": ["aupreset"]}"#,
    description: "audio unit preset",
};
pub const PUBLIC_MP2: UTType = UTType {
    identifier: "public.mp2",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{}"#,
    description: "MP2 audio",
};
pub const PUBLIC_MP3: UTType = UTType {
    identifier: "public.mp3",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["mp3", "mpga"], "public.mime-type": ["audio/mpeg", "audio/mpeg3", "audio/mpg", "audio/mp3", "audio/x-mpeg", "audio/x-mpeg3", "audio/x-mpg", "audio/x-mp3"]}"#,
    description: "MP3 audio",
};
pub const PUBLIC_PLAYLIST: UTType = UTType {
    identifier: "public.playlist",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "playlist",
};
pub const PUBLIC_M3U_PLAYLIST: UTType = UTType {
    identifier: "public.m3u-playlist",
    conforms_to: r#"["public.plain-text", "public.playlist"]"#,
    tags: r#"{"public.filename-extension": ["m3u", "m3u8"], "public.mime-type": ["audio/mpegurl", "application/vnd.apple.mpegurl", "audio/x-mpegurl"]}"#,
    description: "M3U Playlist",
};
pub const PUBLIC_PLS_PLAYLIST: UTType = UTType {
    identifier: "public.pls-playlist",
    conforms_to: r#"["public.text", "public.playlist"]"#,
    tags: r#"{}"#,
    description: "PLS Playlist",
};
pub const PUBLIC_MPEG_4_AUDIO: UTType = UTType {
    identifier: "public.mpeg-4-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["mp4", "mpg4"], "public.mime-type": ["audio/mp4", "audio/mp4a-latm"]}"#,
    description: "MPEG-4 audio",
};
pub const COM_APPLE_M4A_AUDIO: UTType = UTType {
    identifier: "com.apple.m4a-audio",
    conforms_to: r#"["public.mpeg-4-audio"]"#,
    tags: r#"{"public.filename-extension": ["m4a"], "public.mime-type": ["audio/x-m4a"]}"#,
    description: "Apple MPEG-4 audio",
};
pub const COM_APPLE_MPEG_4_RINGTONE: UTType = UTType {
    identifier: "com.apple.mpeg-4-ringtone",
    conforms_to: r#"["public.mpeg-4-audio"]"#,
    tags: r#"{}"#,
    description: "Ringtone",
};
pub const COM_APPLE_PROTECTED_MPEG_4_AUDIO: UTType = UTType {
    identifier: "com.apple.protected-mpeg-4-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["m4p"]}"#,
    description: "protected MPEG-4 audio",
};
pub const COM_APPLE_PROTECTED_MPEG_4_AUDIO_B: UTType = UTType {
    identifier: "com.apple.protected-mpeg-4-audio-b",
    conforms_to: r#"["com.apple.protected-mpeg-4-audio"]"#,
    tags: r#"{"public.filename-extension": ["m4b"]}"#,
    description: "protected MPEG-4 audio",
};
pub const PUBLIC_ULAW_AUDIO: UTType = UTType {
    identifier: "public.ulaw-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{}"#,
    description: "uLaw audio",
};
pub const PUBLIC_AU_AUDIO: UTType = UTType {
    identifier: "public.au-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["au", "snd"], "public.mime-type": ["audio/basic"]}"#,
    description: "AU audio",
};
pub const PUBLIC_AIFC_AUDIO: UTType = UTType {
    identifier: "public.aifc-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["aifc"]}"#,
    description: "AIFF-C audio",
};
pub const PUBLIC_AIFF_AUDIO: UTType = UTType {
    identifier: "public.aiff-audio",
    conforms_to: r#"["public.aifc-audio"]"#,
    tags: r#"{}"#,
    description: "AIFF audio",
};
pub const PUBLIC_CDDA_AUDIO: UTType = UTType {
    identifier: "public.cdda-audio",
    conforms_to: r#"["public.aifc-audio"]"#,
    tags: r#"{}"#,
    description: "CDDA audio",
};
pub const PUBLIC_MIDI_AUDIO: UTType = UTType {
    identifier: "public.midi-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["midi", "mid", "smf", "kar"], "public.mime-type": ["audio/midi", "audio/x-midi"]}"#,
    description: "MIDI audio",
};
pub const PUBLIC_DOWNLOADABLE_SOUND: UTType = UTType {
    identifier: "public.downloadable-sound",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["dls"], "public.mime-type": ["audio/dls"]}"#,
    description: "downloadable sound",
};
pub const COM_APPLE_COREAUDIO_FORMAT: UTType = UTType {
    identifier: "com.apple.coreaudio-format",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{}"#,
    description: "Apple CoreAudio format",
};
pub const PUBLIC_AC3_AUDIO: UTType = UTType {
    identifier: "public.ac3-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["ac3"], "public.mime-type": ["audio/ac3"]}"#,
    description: "AC-3 audio",
};
pub const PUBLIC_ENHANCED_AC3_AUDIO: UTType = UTType {
    identifier: "public.enhanced-ac3-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["eac3", "ec3"], "public.mime-type": ["audio/eac3"]}"#,
    description: "Enhanced AC-3 audio",
};
pub const ORG_3GPP_ADAPTIVE_MULTI_RATE_AUDIO: UTType = UTType {
    identifier: "org.3gpp.adaptive-multi-rate-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["amr"], "public.mime-type": ["audio/amr"]}"#,
    description: "Adaptive Multi-rate audio",
};
pub const PUBLIC_AAC_AUDIO: UTType = UTType {
    identifier: "public.aac-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["aac", "adts"], "public.mime-type": ["audio/aac", "audio/x-aac"]}"#,
    description: "AAC audio",
};
pub const COM_AUDIBLE_AA_AUDIO: UTType = UTType {
    identifier: "com.audible.aa-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{}"#,
    description: "Audible.com Audio",
};
pub const COM_AUDIBLE_AA_AUDIOBOOK: UTType = UTType {
    identifier: "com.audible.aa-audiobook",
    conforms_to: r#"["com.audible.aa-audio"]"#,
    tags: r#"{"public.filename-extension": ["aa"], "public.mime-type": ["audio/audible", "audio/x-pn-audibleaudio", "audio/x-audible"]}"#,
    description: "Audible.com Audiobook",
};
pub const COM_AUDIBLE_AAX_AUDIOBOOK: UTType = UTType {
    identifier: "com.audible.aax-audiobook",
    conforms_to: r#"["com.audible.aa-audio"]"#,
    tags: r#"{"public.filename-extension": ["aax"], "public.mime-type": ["audio/vnd.audible.aax"]}"#,
    description: "Audible.com Audiobook",
};
pub const COM_SONY_WAVE64: UTType = UTType {
    identifier: "com.sony.wave64",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{}"#,
    description: "Wave64 Audio",
};
pub const PUBLIC_FONT: UTType = UTType {
    identifier: "public.font",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "font",
};
pub const PUBLIC_TRUETYPE_FONT: UTType = UTType {
    identifier: "public.truetype-font",
    conforms_to: r#"["public.font"]"#,
    tags: r#"{}"#,
    description: "TrueType font",
};
pub const COM_ADOBE_POSTSCRIPT_FONT: UTType = UTType {
    identifier: "com.adobe.postscript-font",
    conforms_to: r#"["public.font"]"#,
    tags: r#"{}"#,
    description: "PostScript font",
};
pub const COM_APPLE_TRUETYPE_DATAFORK_SUITCASE_FONT: UTType = UTType {
    identifier: "com.apple.truetype-datafork-suitcase-font",
    conforms_to: r#"["public.truetype-font"]"#,
    tags: r#"{}"#,
    description: "TrueType datafork font",
};
pub const PUBLIC_OPENTYPE_FONT: UTType = UTType {
    identifier: "public.opentype-font",
    conforms_to: r#"["public.font"]"#,
    tags: r#"{}"#,
    description: "PostScript OpenType font",
};
pub const PUBLIC_OPENTYPE_COLLECTION_FONT: UTType = UTType {
    identifier: "public.opentype-collection-font",
    conforms_to: r#"["public.opentype-font"]"#,
    tags: r#"{}"#,
    description: "PostScript OpenType collection font",
};
pub const PUBLIC_TRUETYPE_TTF_FONT: UTType = UTType {
    identifier: "public.truetype-ttf-font",
    conforms_to: r#"["public.truetype-font"]"#,
    tags: r#"{}"#,
    description: "TrueType OpenType font",
};
pub const PUBLIC_TRUETYPE_COLLECTION_FONT: UTType = UTType {
    identifier: "public.truetype-collection-font",
    conforms_to: r#"["public.truetype-font"]"#,
    tags: r#"{}"#,
    description: "TrueType collection font",
};
pub const COM_APPLE_FONT_SUITCASE: UTType = UTType {
    identifier: "com.apple.font-suitcase",
    conforms_to: r#"["public.font"]"#,
    tags: r#"{}"#,
    description: "font suitcase",
};
pub const COM_ADOBE_POSTSCRIPT_LWFN_FONT: UTType = UTType {
    identifier: "com.adobe.postscript-lwfn-font",
    conforms_to: r#"["com.adobe.postscript-font"]"#,
    tags: r#"{}"#,
    description: "PostScript Type 1 outline font",
};
pub const COM_ADOBE_POSTSCRIPT_PFB_FONT: UTType = UTType {
    identifier: "com.adobe.postscript-pfb-font",
    conforms_to: r#"["com.adobe.postscript-font"]"#,
    tags: r#"{}"#,
    description: "PostScript Type 1 outline font",
};
pub const COM_ADOBE_POSTSCRIPT_PFA_FONT: UTType = UTType {
    identifier: "com.adobe.postscript-pfa-font",
    conforms_to: r#"["com.adobe.postscript-font"]"#,
    tags: r#"{}"#,
    description: "PostScript Type 1 outline font",
};
pub const COM_APPLE_PROFILE_FONT_ICON: UTType = UTType {
    identifier: "com.apple.profile-font-icon",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: "Profile Font",
};
pub const COM_APPLE_APPLESCRIPT_TEXT: UTType = UTType {
    identifier: "com.apple.applescript.text",
    conforms_to: r#"["public.script"]"#,
    tags: r#"{}"#,
    description: "AppleScript text",
};
pub const COM_APPLE_APPLESCRIPT_SCRIPT: UTType = UTType {
    identifier: "com.apple.applescript.script",
    conforms_to: r#"["public.data", "public.script"]"#,
    tags: r#"{}"#,
    description: "AppleScript",
};
pub const COM_APPLE_APPLESCRIPT_SCRIPT_BUNDLE: UTType = UTType {
    identifier: "com.apple.applescript.script-bundle",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["scptd"]}"#,
    description: "Script bundle",
};
pub const COM_APPLE_SCRIPTING_DEFINITION: UTType = UTType {
    identifier: "com.apple.scripting-definition",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{"public.filename-extension": ["sdef"]}"#,
    description: "Scripting Definition",
};
pub const PUBLIC_FOLDER: UTType = UTType {
    identifier: "public.folder",
    conforms_to: r#"["public.directory"]"#,
    tags: r#"{}"#,
    description: "folder",
};
pub const COM_APPLE_DROP_FOLDER: UTType = UTType {
    identifier: "com.apple.drop-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_APPLICATIONS_FOLDER: UTType = UTType {
    identifier: "com.apple.applications-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_SERVER_APPLICATIONS_FOLDER: UTType = UTType {
    identifier: "com.apple.server-applications-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_LIBRARY_FOLDER: UTType = UTType {
    identifier: "com.apple.library-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_DOCUMENT_TYPE_SYSTEM_FOLDER: UTType = UTType {
    identifier: "com.apple.document-type.system-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "System Folder",
};
pub const COM_APPLE_TRASH_EMPTY: UTType = UTType {
    identifier: "com.apple.trash-empty",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Trash",
};
pub const COM_APPLE_TRASH_FULL: UTType = UTType {
    identifier: "com.apple.trash-full",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Full Trash",
};
pub const COM_APPLE_SITES_FOLDER: UTType = UTType {
    identifier: "com.apple.sites-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Sites Folder",
};
pub const COM_APPLE_GROUPS_FOLDER: UTType = UTType {
    identifier: "com.apple.groups-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Groups Folder",
};
pub const COM_APPLE_SERVERS_FOLDER: UTType = UTType {
    identifier: "com.apple.servers-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_DESKTOP_FOLDER: UTType = UTType {
    identifier: "com.apple.desktop-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Desktop Folder",
};
pub const COM_APPLE_DOCUMENTS_FOLDER: UTType = UTType {
    identifier: "com.apple.documents-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Documents Folder",
};
pub const COM_APPLE_DOWNLOADS_FOLDER: UTType = UTType {
    identifier: "com.apple.downloads-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Downloads Folder",
};
pub const COM_APPLE_MOVIE_FOLDER: UTType = UTType {
    identifier: "com.apple.movie-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Movie Folder",
};
pub const COM_APPLE_MUSIC_FOLDER: UTType = UTType {
    identifier: "com.apple.music-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Music Folder",
};
pub const COM_APPLE_PICTURES_FOLDER: UTType = UTType {
    identifier: "com.apple.pictures-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Pictures Folder",
};
pub const COM_APPLE_PUBLIC_FOLDER: UTType = UTType {
    identifier: "com.apple.public-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Public Folder",
};
pub const COM_APPLE_HOME_FOLDER: UTType = UTType {
    identifier: "com.apple.home-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Home Folder",
};
pub const COM_APPLE_DEVELOPER_FOLDER: UTType = UTType {
    identifier: "com.apple.developer-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Developer Folder",
};
pub const COM_APPLE_USERS_FOLDER: UTType = UTType {
    identifier: "com.apple.users-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Users Folder",
};
pub const COM_APPLE_UTILITIES_FOLDER: UTType = UTType {
    identifier: "com.apple.utilities-folder",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "Utilities Folder",
};
pub const PUBLIC_VOLUME: UTType = UTType {
    identifier: "public.volume",
    conforms_to: r#"["public.folder"]"#,
    tags: r#"{}"#,
    description: "volume",
};
pub const PUBLIC_FILE_SHAREPOINT: UTType = UTType {
    identifier: "public.file-sharepoint",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "sharepoint",
};
pub const COM_APPLE_NETWORK_NEIGHBORHOOD: UTType = UTType {
    identifier: "com.apple.network-neighborhood",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "neighborhood",
};
pub const COM_APPLE_DOT_MAC: UTType = UTType {
    identifier: "com.apple.dot-mac",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: ".Mac",
};
pub const COM_APPLE_IDISK: UTType = UTType {
    identifier: "com.apple.idisk",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "iDisk",
};
pub const COM_APPLE_USER_IDISK: UTType = UTType {
    identifier: "com.apple.user-idisk",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "User iDisk",
};
pub const COM_APPLE_PACKAGE: UTType = UTType {
    identifier: "com.apple.package",
    conforms_to: r#"["public.directory"]"#,
    tags: r#"{}"#,
    description: "Package",
};
pub const COM_APPLE_BUNDLE: UTType = UTType {
    identifier: "com.apple.bundle",
    conforms_to: r#"["public.directory"]"#,
    tags: r#"{}"#,
    description: "bundle",
};
pub const COM_APPLE_GENERIC_BUNDLE: UTType = UTType {
    identifier: "com.apple.generic-bundle",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{}"#,
    description: "bundle",
};
pub const COM_APPLE_SYSTEMPREFERENCE_PREFPANE: UTType = UTType {
    identifier: "com.apple.systempreference.prefpane",
    conforms_to: r#"["com.apple.package", "com.apple.bundle"]"#,
    tags: r#"{}"#,
    description: "System Preference pane",
};
pub const COM_APPLE_SYSTEMPREFERENCE_SCREEN_SAVER: UTType = UTType {
    identifier: "com.apple.systempreference.screen-saver",
    conforms_to: r#"["com.apple.package", "com.apple.bundle"]"#,
    tags: r#"{}"#,
    description: "Screen Saver",
};
pub const COM_APPLE_SYSTEMPREFERENCE_SCREEN_SLIDE_SAVER: UTType = UTType {
    identifier: "com.apple.systempreference.screen-slide-saver",
    conforms_to: r#"["com.apple.package", "com.apple.bundle"]"#,
    tags: r#"{}"#,
    description: "Screen Slide Saver",
};
pub const COM_APPLE_MENU_EXTRA: UTType = UTType {
    identifier: "com.apple.menu-extra",
    conforms_to: r#"["com.apple.package", "com.apple.bundle"]"#,
    tags: r#"{}"#,
    description: "System Menu Extra",
};
pub const COM_APPLE_LOCALIZABLE_NAME_BUNDLE: UTType = UTType {
    identifier: "com.apple.localizable-name-bundle",
    conforms_to: r#"["com.apple.bundle"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_FRAMEWORK: UTType = UTType {
    identifier: "com.apple.framework",
    conforms_to: r#"["com.apple.bundle"]"#,
    tags: r#"{}"#,
    description: "framework",
};
pub const COM_APPLE_APPLICATION_BUNDLE: UTType = UTType {
    identifier: "com.apple.application-bundle",
    conforms_to: r#"["com.apple.application", "com.apple.localizable-name-bundle", "com.apple.package"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_APPLICATION_FILE: UTType = UTType {
    identifier: "com.apple.application-file",
    conforms_to: r#"["com.apple.application", "public.data"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_DEPRECATED_APPLICATION_FILE: UTType = UTType {
    identifier: "com.apple.deprecated-application-file",
    conforms_to: r#"["com.apple.application-file"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_FLAT_RTFD: UTType = UTType {
    identifier: "com.apple.flat-rtfd",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{}"#,
    description: "rich text with attachments (RTFD) data",
};
pub const COM_APPLE_INSTALLER_PACKAGE: UTType = UTType {
    identifier: "com.apple.installer-package",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{}"#,
    description: "Installer package",
};
pub const COM_APPLE_INSTALLER_DISTRIBUTION_PACKAGE: UTType = UTType {
    identifier: "com.apple.installer-distribution-package",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{}"#,
    description: "Installer distribution",
};
pub const COM_APPLE_INSTALLER_META_PACKAGE: UTType = UTType {
    identifier: "com.apple.installer-meta-package",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{}"#,
    description: "Installer package",
};
pub const COM_APPLE_INTELLIGENTSUGGESTIONS_ASSETS: UTType = UTType {
    identifier: "com.apple.intelligentsuggestions.assets",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{}"#,
    description: "Intelligent Suggestions assets",
};
pub const COM_APPLE_RTFD: UTType = UTType {
    identifier: "com.apple.rtfd",
    conforms_to: r#"["com.apple.package", "public.composite-content"]"#,
    tags: r#"{}"#,
    description: "rich text with attachments (RTFD)",
};
pub const COM_APPLE_APPLICATION_PLACEHOLDER: UTType = UTType {
    identifier: "com.apple.application-placeholder",
    conforms_to: r#"["com.apple.application-bundle"]"#,
    tags: r#"{"public.filename-extension": ["placeholder"]}"#,
    description: "",
};
pub const COM_APPLE_SERVICE_APPLICATION: UTType = UTType {
    identifier: "com.apple.service-application",
    conforms_to: r#"["com.apple.application-bundle"]"#,
    tags: r#"{"public.filename-extension": ["service"]}"#,
    description: "",
};
pub const COM_APPLE_PLUGIN: UTType = UTType {
    identifier: "com.apple.plugin",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{}"#,
    description: "plug-in",
};
pub const COM_APPLE_XPC_SERVICE: UTType = UTType {
    identifier: "com.apple.xpc-service",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{}"#,
    description: "XPC Service",
};
pub const COM_APPLE_KERNEL_EXTENSION: UTType = UTType {
    identifier: "com.apple.kernel-extension",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["kext"]}"#,
    description: "Kernel Extension",
};
pub const COM_APPLE_APPLICATION_AND_SYSTEM_EXTENSION: UTType = UTType {
    identifier: "com.apple.application-and-system-extension",
    conforms_to: r#"["com.apple.xpc-service"]"#,
    tags: r#"{}"#,
    description: "Application and System Extension",
};
pub const COM_APPLE_PLUGINKIT: UTType = UTType {
    identifier: "com.apple.pluginkit",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{}"#,
    description: "PlugInKit plug-in",
};
pub const COM_APPLE_WEBKIT_PLUGIN: UTType = UTType {
    identifier: "com.apple.webkit-plugin",
    conforms_to: r#"["com.apple.plugin"]"#,
    tags: r#"{}"#,
    description: "WebKit plug-in",
};
pub const COM_APPLE_METADATA_IMPORTER: UTType = UTType {
    identifier: "com.apple.metadata-importer",
    conforms_to: r#"["com.apple.plugin"]"#,
    tags: r#"{}"#,
    description: "Spotlight importer",
};
pub const COM_APPLE_QUICKLOOK_GENERATOR: UTType = UTType {
    identifier: "com.apple.quicklook-generator",
    conforms_to: r#"["com.apple.plugin"]"#,
    tags: r#"{}"#,
    description: "QuickLook preview generator",
};
pub const COM_APPLE_DASHBOARD_WIDGET: UTType = UTType {
    identifier: "com.apple.dashboard-widget",
    conforms_to: r#"["com.apple.localizable-name-bundle", "com.apple.package"]"#,
    tags: r#"{}"#,
    description: "Dashboard widget",
};
pub const COM_APPLE_DRIVER_EXTENSION: UTType = UTType {
    identifier: "com.apple.driver-extension",
    conforms_to: r#"["com.apple.localizable-name-bundle", "com.apple.package"]"#,
    tags: r#"{}"#,
    description: "Driver Extension",
};
pub const COM_APPLE_SYSTEM_EXTENSION: UTType = UTType {
    identifier: "com.apple.system-extension",
    conforms_to: r#"["com.apple.localizable-name-bundle", "com.apple.package"]"#,
    tags: r#"{}"#,
    description: "System Extension",
};
pub const COM_APPLE_PPP_PLUG_IN: UTType = UTType {
    identifier: "com.apple.ppp-plug-in",
    conforms_to: r#"["com.apple.plugin"]"#,
    tags: r#"{"public.filename-extension": ["ppp"]}"#,
    description: "PPP Plug-in",
};
pub const COM_APPLE_FILE_SYSTEM_PLUG_IN: UTType = UTType {
    identifier: "com.apple.file-system-plug-in",
    conforms_to: r#"["com.apple.plugin"]"#,
    tags: r#"{"public.filename-extension": ["fs"]}"#,
    description: "File System Plug-in",
};
pub const COM_APPLE_DATA_CONTAINER: UTType = UTType {
    identifier: "com.apple.data-container",
    conforms_to: r#"["public.folder", "com.apple.localizable-name-bundle"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const ORG_OPENXMLFORMATS_OPENXML: UTType = UTType {
    identifier: "org.openxmlformats.openxml",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "Office Open XML",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT: UTType = UTType {
    identifier: "org.oasis-open.opendocument",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "Open Document",
};
pub const COM_RSA_PKCS_12: UTType = UTType {
    identifier: "com.rsa.pkcs-12",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["p12", "pfx"], "public.mime-type": ["application/x-pkcs12"]}"#,
    description: "personal information exchange (PKCS#12)",
};
pub const PUBLIC_X509_CERTIFICATE: UTType = UTType {
    identifier: "public.x509-certificate",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["cer", "der", "crt", "pem"], "public.mime-type": ["application/x-x509-ca-cert"]}"#,
    description: "certificate (X.509)",
};
pub const COM_APPLE_ALERT: UTType = UTType {
    identifier: "com.apple.alert",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Alert",
};
pub const COM_APPLE_ALERT_NOTE: UTType = UTType {
    identifier: "com.apple.alert-note",
    conforms_to: r#"["com.apple.alert"]"#,
    tags: r#"{}"#,
    description: "Alert Note",
};
pub const COM_APPLE_ALERT_CAUTION: UTType = UTType {
    identifier: "com.apple.alert-caution",
    conforms_to: r#"["com.apple.alert"]"#,
    tags: r#"{}"#,
    description: "Alert Caution",
};
pub const COM_APPLE_ALERT_STOP: UTType = UTType {
    identifier: "com.apple.alert-stop",
    conforms_to: r#"["com.apple.alert"]"#,
    tags: r#"{}"#,
    description: "Alert Stop",
};
pub const COM_APPLE_WEBARCHIVE: UTType = UTType {
    identifier: "com.apple.webarchive",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["webarchive"], "public.mime-type": ["application/x-webarchive"]}"#,
    description: "web archive",
};
pub const ORG_IDPF_EPUB_CONTAINER: UTType = UTType {
    identifier: "org.idpf.epub-container",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["epub"], "public.mime-type": ["application/epub+zip"]}"#,
    description: "EPUB publication",
};
pub const COM_APPLE_LOCALIZED_PDF_BUNDLE: UTType = UTType {
    identifier: "com.apple.localized-pdf-bundle",
    conforms_to: r#"["com.apple.localizable-name-bundle", "com.apple.package", "public.composite-content"]"#,
    tags: r#"{}"#,
    description: "localized PDF",
};
pub const ORG_AAFASSOCIATION_ADVANCED_AUTHORING_FORMAT: UTType = UTType {
    identifier: "org.aafassociation.advanced-authoring-format",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["aaf"]}"#,
    description: "Advanced Authoring Format",
};
pub const COM_APPLE_TXN_TEXT_MULTIMEDIA_DATA: UTType = UTType {
    identifier: "com.apple.txn.text-multimedia-data",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{}"#,
    description: "text with multimedia",
};
pub const COM_APPLE_COLORSYNC_PROFILE: UTType = UTType {
    identifier: "com.apple.colorsync-profile",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["icc", "icm", "pf"]}"#,
    description: "ColorSync profile",
};
pub const COM_APPLE_PROFILE_BACKGROUND_COLOR: UTType = UTType {
    identifier: "com.apple.profile-background-color",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "Profile Background Color",
};
pub const COM_APPLE_PROFILE_FONT_AND_COLOR: UTType = UTType {
    identifier: "com.apple.profile-font-and-color",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "Profile Font And Color",
};
pub const COM_APPLE_COLOR_FILE: UTType = UTType {
    identifier: "com.apple.color-file",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["clr", "ccl"]}"#,
    description: "Color File",
};
pub const COM_APPLE_INK_INKTEXT: UTType = UTType {
    identifier: "com.apple.ink.inktext",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "writing",
};
pub const COM_APPLE_MOBILECONFIG: UTType = UTType {
    identifier: "com.apple.mobileconfig",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{"public.filename-extension": ["mobileconfig", "mobile"], "public.mime-type": ["application/x-apple-aspen-config"]}"#,
    description: "mobile configuration",
};
pub const COM_APPLE_PROVISIONPROFILE: UTType = UTType {
    identifier: "com.apple.provisionprofile",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{"public.filename-extension": ["provisionprofile"]}"#,
    description: "provision profile",
};
pub const COM_APPLE_CONFIGPROFILE: UTType = UTType {
    identifier: "com.apple.configprofile",
    conforms_to: r#"["public.xml"]"#,
    tags: r#"{"public.filename-extension": ["configprofile"]}"#,
    description: "configuration profile",
};
pub const COM_APPLE_USER: UTType = UTType {
    identifier: "com.apple.user",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "User",
};
pub const COM_APPLE_GUEST_USER: UTType = UTType {
    identifier: "com.apple.guest-user",
    conforms_to: r#"["com.apple.user"]"#,
    tags: r#"{}"#,
    description: "Guest User",
};
pub const COM_APPLE_HELP_DOCUMENT: UTType = UTType {
    identifier: "com.apple.help-document",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "Help Document",
};
pub const COM_APPLE_USER_GROUP: UTType = UTType {
    identifier: "com.apple.user-group",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "User Group",
};
pub const COM_APPLE_USER_UNKNOWN: UTType = UTType {
    identifier: "com.apple.user-unknown",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "User Unknown",
};
pub const COM_APPLE_AIRDROP: UTType = UTType {
    identifier: "com.apple.airdrop",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "AirDrop",
};
pub const COM_APPLE_BONJOUR: UTType = UTType {
    identifier: "com.apple.bonjour",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Bonjour",
};
pub const COM_APPLE_NOTIFICATIONS: UTType = UTType {
    identifier: "com.apple.notifications",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Notifications",
};
pub const COM_APPLE_MOBILEPROVISION: UTType = UTType {
    identifier: "com.apple.mobileprovision",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["mobileprovision"], "public.mime-type": ["application/x-apple-aspen-mobileprovision"]}"#,
    description: "mobile provision",
};
pub const COM_APPLE_PKPASS: UTType = UTType {
    identifier: "com.apple.pkpass",
    conforms_to: r#"["com.apple.bundle", "com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["pkpass"], "public.mime-type": ["application/vnd.apple.pkpass"]}"#,
    description: "Pass",
};
pub const COM_APPLE_PKPASS_DATA: UTType = UTType {
    identifier: "com.apple.pkpass-data",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["pkpass"]}"#,
    description: "Pass",
};
pub const COM_APPLE_PKPASSES_DATA: UTType = UTType {
    identifier: "com.apple.pkpasses-data",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["pkpasses"], "public.mime-type": ["application/vnd.apple.pkpasses"]}"#,
    description: "Passes",
};
pub const COM_APPLE_WATCHFACE: UTType = UTType {
    identifier: "com.apple.watchface",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["watchface"]}"#,
    description: "Watchface",
};
pub const PUBLIC_DEVICE: UTType = UTType {
    identifier: "public.device",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Device",
};
pub const COM_APPLE_VIRTUAL_MACHINE: UTType = UTType {
    identifier: "com.apple.virtual-machine",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: "Virtual Machine",
};
pub const PUBLIC_DISPLAY: UTType = UTType {
    identifier: "public.display",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Display",
};
pub const PUBLIC_SPEAKER: UTType = UTType {
    identifier: "public.speaker",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Speaker",
};
pub const PUBLIC_COMPUTER: UTType = UTType {
    identifier: "public.computer",
    conforms_to: r#"["public.device"]"#,
    tags: r#"{}"#,
    description: "Computer",
};
pub const PUBLIC_GENERIC_PC: UTType = UTType {
    identifier: "public.generic-pc",
    conforms_to: r#"["public.computer"]"#,
    tags: r#"{}"#,
    description: "PC",
};
pub const COM_APPLE_DEVICE: UTType = UTType {
    identifier: "com.apple.device",
    conforms_to: r#"["public.device"]"#,
    tags: r#"{}"#,
    description: "Apple device",
};
pub const COM_APPLE_MAC: UTType = UTType {
    identifier: "com.apple.mac",
    conforms_to: r#"["public.computer", "com.apple.device"]"#,
    tags: r#"{}"#,
    description: "Mac",
};
pub const COM_APPLE_MAC_LAPTOP: UTType = UTType {
    identifier: "com.apple.mac.laptop",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: "Laptop",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-silver",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-space-gray",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-2017",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-silver-2017",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc-silver"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-space-gray-2017",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc-space-gray"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-2017",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver-2017",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-silver"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray-2017",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-space-gray"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-2018",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver-2018",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-silver", "com.apple.macbookpro-13-retina-touchid-2018"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray-2018",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-space-gray", "com.apple.macbookpro-13-retina-touchid-2018"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_2019: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-2019",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER_2019: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-silver-2019",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc-silver", "com.apple.macbookpro-13-retina-usbc-2019"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY_2019: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-space-gray-2019",
    conforms_to: r#"["com.apple.macbookpro-13-retina-usbc-space-gray", "com.apple.macbookpro-13-retina-usbc-2019"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-silver",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-space-gray",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-2017",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-silver-2017",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid-silver"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-space-gray-2017",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid-space-gray"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-2018",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-silver-2018",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid-silver", "com.apple.macbookpro-15-retina-touchid-2018"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-space-gray-2018",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid-space-gray", "com.apple.macbookpro-15-retina-touchid-2018"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_LATE_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-late-2018",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_SILVER_LATE_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-silver-late-2018",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid-silver", "com.apple.macbookpro-15-late-2018"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_SPACE_GRAY_LATE_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-space-gray-late-2018",
    conforms_to: r#"["com.apple.macbookpro-15-retina-touchid-space-gray", "com.apple.macbookpro-15-late-2018"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MAC_TOWER: UTType = UTType {
    identifier: "com.apple.mac.tower",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: "Tower",
};
pub const COM_APPLE_MAC_RACKMOUNT: UTType = UTType {
    identifier: "com.apple.mac.rackmount",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: "Rack Mount",
};
pub const COM_APPLE_POWERBOOK: UTType = UTType {
    identifier: "com.apple.powerbook",
    conforms_to: r#"["com.apple.mac.laptop"]"#,
    tags: r#"{}"#,
    description: "PowerBook",
};
pub const COM_APPLE_POWERBOOK_G4_TITANIUM: UTType = UTType {
    identifier: "com.apple.powerbook-g4-titanium",
    conforms_to: r#"["com.apple.powerbook"]"#,
    tags: r#"{}"#,
    description: "PowerBook G4",
};
pub const COM_APPLE_POWERBOOK_G4_12: UTType = UTType {
    identifier: "com.apple.powerbook-g4-12",
    conforms_to: r#"["com.apple.powerbook"]"#,
    tags: r#"{}"#,
    description: "PowerBook G4",
};
pub const COM_APPLE_POWERBOOK_G4_15: UTType = UTType {
    identifier: "com.apple.powerbook-g4-15",
    conforms_to: r#"["com.apple.powerbook"]"#,
    tags: r#"{}"#,
    description: "PowerBook G4",
};
pub const COM_APPLE_POWERBOOK_G4_17: UTType = UTType {
    identifier: "com.apple.powerbook-g4-17",
    conforms_to: r#"["com.apple.powerbook"]"#,
    tags: r#"{}"#,
    description: "PowerBook G4",
};
pub const COM_APPLE_IBOOK_G4: UTType = UTType {
    identifier: "com.apple.ibook-g4",
    conforms_to: r#"["com.apple.mac.laptop"]"#,
    tags: r#"{}"#,
    description: "iBook G4",
};
pub const COM_APPLE_POWERMAC: UTType = UTType {
    identifier: "com.apple.powermac",
    conforms_to: r#"["com.apple.mac.tower"]"#,
    tags: r#"{}"#,
    description: "Power Mac",
};
pub const COM_APPLE_POWERMAC_G4_QUICKSILVER: UTType = UTType {
    identifier: "com.apple.powermac-g4-quicksilver",
    conforms_to: r#"["com.apple.powermac"]"#,
    tags: r#"{}"#,
    description: "Power Mac G4",
};
pub const COM_APPLE_POWERMAC_G4_MIRRORED_DRIVE_DOORS: UTType = UTType {
    identifier: "com.apple.powermac-g4-mirrored-drive-doors",
    conforms_to: r#"["com.apple.powermac"]"#,
    tags: r#"{}"#,
    description: "Power Mac G4",
};
pub const COM_APPLE_POWERMAC_G5: UTType = UTType {
    identifier: "com.apple.powermac-g5",
    conforms_to: r#"["com.apple.powermac"]"#,
    tags: r#"{}"#,
    description: "Power Mac G5",
};
pub const COM_APPLE_XSERVE: UTType = UTType {
    identifier: "com.apple.xserve",
    conforms_to: r#"["com.apple.mac.rackmount"]"#,
    tags: r#"{}"#,
    description: "Xserve",
};
pub const COM_APPLE_XSERVE_G4: UTType = UTType {
    identifier: "com.apple.xserve-g4",
    conforms_to: r#"["com.apple.xserve"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_XSERVE_G5: UTType = UTType {
    identifier: "com.apple.xserve-g5",
    conforms_to: r#"["com.apple.xserve"]"#,
    tags: r#"{}"#,
    description: "Xserve G5",
};
pub const COM_APPLE_XSERVE_XEON: UTType = UTType {
    identifier: "com.apple.xserve-xeon",
    conforms_to: r#"["com.apple.xserve"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACMINI: UTType = UTType {
    identifier: "com.apple.macmini",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: "Mac mini",
};
pub const COM_APPLE_MACMINI_G4: UTType = UTType {
    identifier: "com.apple.macmini-g4",
    conforms_to: r#"["com.apple.macmini"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACMINI_CORE_DUO: UTType = UTType {
    identifier: "com.apple.macmini-core-duo",
    conforms_to: r#"["com.apple.macmini"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACMINI_UNIBODY: UTType = UTType {
    identifier: "com.apple.macmini-unibody",
    conforms_to: r#"["com.apple.macmini"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACMINI_UNIBODY_NO_OPTICAL: UTType = UTType {
    identifier: "com.apple.macmini-unibody-no-optical",
    conforms_to: r#"["com.apple.macmini"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACMINI_2018: UTType = UTType {
    identifier: "com.apple.macmini-2018",
    conforms_to: r#"["com.apple.macmini"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_EMAC: UTType = UTType {
    identifier: "com.apple.emac",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: "eMac",
};
pub const COM_APPLE_IMAC: UTType = UTType {
    identifier: "com.apple.imac",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: "iMac",
};
pub const COM_APPLE_IMAC_G4_15: UTType = UTType {
    identifier: "com.apple.imac-g4-15",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_G4_17: UTType = UTType {
    identifier: "com.apple.imac-g4-17",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_G4_20: UTType = UTType {
    identifier: "com.apple.imac-g4-20",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_G5: UTType = UTType {
    identifier: "com.apple.imac-g5",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "iMac G5",
};
pub const COM_APPLE_IMAC_G5_ISIGHT: UTType = UTType {
    identifier: "com.apple.imac-g5-isight",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_CORE_DUO: UTType = UTType {
    identifier: "com.apple.imac-core-duo",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_CORE_2_DUO: UTType = UTType {
    identifier: "com.apple.imac-core-2-duo",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_ISIGHT_24: UTType = UTType {
    identifier: "com.apple.imac-iSight-24",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_ALUMINUM_20: UTType = UTType {
    identifier: "com.apple.imac-aluminum-20",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_ALUMINUM_24: UTType = UTType {
    identifier: "com.apple.imac-aluminum-24",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_21: UTType = UTType {
    identifier: "com.apple.imac-unibody-21",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY: UTType = UTType {
    identifier: "com.apple.imac-unibody",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL: UTType = UTType {
    identifier: "com.apple.imac-unibody-21-no-optical",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_15_1: UTType = UTType {
    identifier: "com.apple.imac-15-1",
    conforms_to: r#"["com.apple.imac-unibody-27-no-optical"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_MID_2015: UTType = UTType {
    identifier: "com.apple.imac-unibody-21-no-optical.mid-2015",
    conforms_to: r#"["com.apple.imac-unibody-21-no-optical"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_LATE_2015: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical-late-2015",
    conforms_to: r#"["com.apple.imac-unibody-27-no-optical"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_2017: UTType = UTType {
    identifier: "com.apple.imac-unibody-21-no-optical-2017",
    conforms_to: r#"["com.apple.imac-unibody-21-no-optical"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2017: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical-2017",
    conforms_to: r#"["com.apple.imac-unibody-27-no-optical"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_2019: UTType = UTType {
    identifier: "com.apple.imac-unibody-21-no-optical-2019",
    conforms_to: r#"["com.apple.imac-unibody-21-no-optical"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2019: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical-2019",
    conforms_to: r#"["com.apple.imac-unibody-27-no-optical"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2020: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical-2020",
    conforms_to: r#"["com.apple.imac-unibody-27-no-optical"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMACPRO_2017: UTType = UTType {
    identifier: "com.apple.imacpro-2017",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "iMac Pro",
};
pub const COM_APPLE_MACBOOK: UTType = UTType {
    identifier: "com.apple.macbook",
    conforms_to: r#"["com.apple.mac.laptop"]"#,
    tags: r#"{}"#,
    description: "MacBook",
};
pub const COM_APPLE_MACBOOK_WHITE: UTType = UTType {
    identifier: "com.apple.macbook-white",
    conforms_to: r#"["com.apple.macbook"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOK_BLACK: UTType = UTType {
    identifier: "com.apple.macbook-black",
    conforms_to: r#"["com.apple.macbook"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOK_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbook-unibody",
    conforms_to: r#"["com.apple.macbook"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOK_UNIBODY_PLASTIC: UTType = UTType {
    identifier: "com.apple.macbook-unibody-plastic",
    conforms_to: r#"["com.apple.macbook"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA: UTType = UTType {
    identifier: "com.apple.macbook-retina",
    conforms_to: r#"["com.apple.macbook"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_SILVER: UTType = UTType {
    identifier: "com.apple.macbook-retina-silver",
    conforms_to: r#"["com.apple.macbook-retina"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_GOLD: UTType = UTType {
    identifier: "com.apple.macbook-retina-gold",
    conforms_to: r#"["com.apple.macbook-retina"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbook-retina-space-gray",
    conforms_to: r#"["com.apple.macbook-retina"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_ROSE_GOLD: UTType = UTType {
    identifier: "com.apple.macbook-retina-rose-gold",
    conforms_to: r#"["com.apple.macbook-retina"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_SILVER_2017: UTType = UTType {
    identifier: "com.apple.macbook-retina-silver-2017",
    conforms_to: r#"["com.apple.macbook-retina-silver"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_GOLD_2017: UTType = UTType {
    identifier: "com.apple.macbook-retina-gold-2017",
    conforms_to: r#"["com.apple.macbook-retina-gold"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_SPACE_GRAY_2017: UTType = UTType {
    identifier: "com.apple.macbook-retina-space-gray-2017",
    conforms_to: r#"["com.apple.macbook-retina-space-gray"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_ROSE_GOLD_2017: UTType = UTType {
    identifier: "com.apple.macbook-retina-rose-gold-2017",
    conforms_to: r#"["com.apple.macbook-retina-rose-gold"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_GOLD_2018: UTType = UTType {
    identifier: "com.apple.macbook-retina-gold-2018",
    conforms_to: r#"["com.apple.macbook-retina"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookpro-13-unibody",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_DISPLAY: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-display",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO: UTType = UTType {
    identifier: "com.apple.macbookpro",
    conforms_to: r#"["com.apple.mac.laptop"]"#,
    tags: r#"{}"#,
    description: "MacBook Pro",
};
pub const COM_APPLE_MACBOOKPRO_15: UTType = UTType {
    identifier: "com.apple.macbookpro-15",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookpro-15-unibody",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_DISPLAY: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-display",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_17_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookpro-17-unibody",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_17: UTType = UTType {
    identifier: "com.apple.macbookpro-17",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_16: UTType = UTType {
    identifier: "com.apple.macbookpro-16",
    conforms_to: r#"["com.apple.macbookpro"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_16_SILVER: UTType = UTType {
    identifier: "com.apple.macbookpro-16-silver",
    conforms_to: r#"["com.apple.macbookpro-16"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_16_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookpro-16-space-gray",
    conforms_to: r#"["com.apple.macbookpro-16"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-four-usb-ports-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_SILVER_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-four-usb-ports-silver-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-silver", "com.apple.macbookpro-13-retina-four-usb-ports-2020"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_SPACE_GRAY_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-four-usb-ports-space-gray-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-space-gray", "com.apple.macbookpro-13-retina-four-usb-ports-2020"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-silver", "com.apple.macbookpro-13-retina-touchid-2020"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-space-gray", "com.apple.macbookpro-13-retina-touchid-2020"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_16_MID_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-16-mid-2020",
    conforms_to: r#"["com.apple.macbookpro-16"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_16_SILVER_MID_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-16-silver-mid-2020",
    conforms_to: r#"["com.apple.macbookpro-16-silver", "com.apple.macbookpro-16-mid-2020"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_16_SPACE_GRAY_MID_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-16-space-gray-mid-2020",
    conforms_to: r#"["com.apple.macbookpro-16-space-gray", "com.apple.macbookpro-16-mid-2020"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR: UTType = UTType {
    identifier: "com.apple.macbookair",
    conforms_to: r#"["com.apple.mac.laptop"]"#,
    tags: r#"{}"#,
    description: "MacBook Air",
};
pub const COM_APPLE_MACBOOKAIR_11_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookair-11-unibody",
    conforms_to: r#"["com.apple.macbookair"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_13_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookair-13-unibody",
    conforms_to: r#"["com.apple.macbookair"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2018: UTType = UTType {
    identifier: "com.apple.macbookair-2018",
    conforms_to: r#"["com.apple.macbookair"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2018_SILVER: UTType = UTType {
    identifier: "com.apple.macbookair-2018-silver",
    conforms_to: r#"["com.apple.macbookair-2018"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2018_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookair-2018-space-gray",
    conforms_to: r#"["com.apple.macbookair-2018"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2018_GOLD: UTType = UTType {
    identifier: "com.apple.macbookair-2018-gold",
    conforms_to: r#"["com.apple.macbookair-2018"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2019: UTType = UTType {
    identifier: "com.apple.macbookair-2019",
    conforms_to: r#"["com.apple.macbookair-2018"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2019_SILVER: UTType = UTType {
    identifier: "com.apple.macbookair-2019-silver",
    conforms_to: r#"["com.apple.macbookair-2018-silver"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2019_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookair-2019-space-gray",
    conforms_to: r#"["com.apple.macbookair-2018-space-gray"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2019_GOLD: UTType = UTType {
    identifier: "com.apple.macbookair-2019-gold",
    conforms_to: r#"["com.apple.macbookair-2018-gold"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2020: UTType = UTType {
    identifier: "com.apple.macbookair-2020",
    conforms_to: r#"["com.apple.macbookair-2019"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2020_SILVER: UTType = UTType {
    identifier: "com.apple.macbookair-2020-silver",
    conforms_to: r#"["com.apple.macbookair-2019-silver"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2020_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookair-2020-space-gray",
    conforms_to: r#"["com.apple.macbookair-2019-space-gray"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2020_GOLD: UTType = UTType {
    identifier: "com.apple.macbookair-2020-gold",
    conforms_to: r#"["com.apple.macbookair-2019-gold"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACPRO: UTType = UTType {
    identifier: "com.apple.macpro",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: "Mac Pro",
};
pub const COM_APPLE_MACPRO_FIREWIRE: UTType = UTType {
    identifier: "com.apple.macpro-firewire",
    conforms_to: r#"["com.apple.macpro", "com.apple.mac.tower"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACPRO_CYLINDER: UTType = UTType {
    identifier: "com.apple.macpro-cylinder",
    conforms_to: r#"["com.apple.macpro", "com.apple.mac.tower"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACPRO_2019: UTType = UTType {
    identifier: "com.apple.macpro-2019",
    conforms_to: r#"["com.apple.macpro", "com.apple.mac.tower"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACPRO_2019_RACKMOUNT: UTType = UTType {
    identifier: "com.apple.macpro-2019-rackmount",
    conforms_to: r#"["com.apple.macpro", "com.apple.mac.rackmount"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACMINI_2020: UTType = UTType {
    identifier: "com.apple.macmini-2020",
    conforms_to: r#"["com.apple.macmini"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_LATE_2020: UTType = UTType {
    identifier: "com.apple.macbookair-late-2020",
    conforms_to: r#"["com.apple.macbookair-2020"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_LATE_2020_SILVER: UTType = UTType {
    identifier: "com.apple.macbookair-late-2020-silver",
    conforms_to: r#"["com.apple.macbookair-2020-silver", "com.apple.macbookair-late-2020"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_LATE_2020_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookair-late-2020-space-gray",
    conforms_to: r#"["com.apple.macbookair-2020-space-gray", "com.apple.macbookair-late-2020"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_LATE_2020_GOLD: UTType = UTType {
    identifier: "com.apple.macbookair-late-2020-gold",
    conforms_to: r#"["com.apple.macbookair-2020-gold", "com.apple.macbookair-late-2020"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_LATE_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-late-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-2020"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_LATE_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver-late-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-silver-2020", "com.apple.macbookpro-13-retina-touchid-late-2020"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_LATE_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray-late-2020",
    conforms_to: r#"["com.apple.macbookpro-13-retina-touchid-space-gray-2020", "com.apple.macbookpro-13-retina-touchid-late-2020"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_2021: UTType = UTType {
    identifier: "com.apple.imac-2021",
    conforms_to: r#"["com.apple.imac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_2021_SILVER: UTType = UTType {
    identifier: "com.apple.imac-2021-silver",
    conforms_to: r#"["com.apple.imac-2021"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_2021_YELLOW: UTType = UTType {
    identifier: "com.apple.imac-2021-yellow",
    conforms_to: r#"["com.apple.imac-2021"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_2021_GREEN: UTType = UTType {
    identifier: "com.apple.imac-2021-green",
    conforms_to: r#"["com.apple.imac-2021"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_2021_BLUE: UTType = UTType {
    identifier: "com.apple.imac-2021-blue",
    conforms_to: r#"["com.apple.imac-2021"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_2021_RED: UTType = UTType {
    identifier: "com.apple.imac-2021-red",
    conforms_to: r#"["com.apple.imac-2021"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_2021_PURPLE: UTType = UTType {
    identifier: "com.apple.imac-2021-purple",
    conforms_to: r#"["com.apple.imac-2021"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IMAC_2021_ORANGE: UTType = UTType {
    identifier: "com.apple.imac-2021-orange",
    conforms_to: r#"["com.apple.imac-2021"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_DEVELOPER_TRANSITION_KIT_2005: UTType = UTType {
    identifier: "com.apple.developer-transition-kit-2005",
    conforms_to: r#"["com.apple.mac.tower"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_DEVELOPER_TRANSITION_KIT_2020: UTType = UTType {
    identifier: "com.apple.developer-transition-kit-2020",
    conforms_to: r#"["com.apple.mac"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_IOS_DEVICE: UTType = UTType {
    identifier: "com.apple.ios-device",
    conforms_to: r#"["com.apple.device"]"#,
    tags: r#"{}"#,
    description: "iOS device",
};
pub const COM_APPLE_APPLE_TV: UTType = UTType {
    identifier: "com.apple.apple-tv",
    conforms_to: r#"["com.apple.ios-device"]"#,
    tags: r#"{}"#,
    description: "Apple TV",
};
pub const COM_APPLE_HOMEPOD: UTType = UTType {
    identifier: "com.apple.homepod",
    conforms_to: r#"["com.apple.ios-device", "public.speaker"]"#,
    tags: r#"{}"#,
    description: "HomePod",
};
pub const COM_APPLE_IOS_SIMULATOR: UTType = UTType {
    identifier: "com.apple.ios-simulator",
    conforms_to: r#"["com.apple.ios-device"]"#,
    tags: r#"{}"#,
    description: "iOS Simulator",
};
pub const COM_APPLE_IPHONE: UTType = UTType {
    identifier: "com.apple.iphone",
    conforms_to: r#"["com.apple.ios-device"]"#,
    tags: r#"{}"#,
    description: "iPhone",
};
pub const COM_APPLE_IPHONE_3G: UTType = UTType {
    identifier: "com.apple.iphone-3g",
    conforms_to: r#"["com.apple.iphone"]"#,
    tags: r#"{}"#,
    description: "iPhone 3G",
};
pub const COM_APPLE_IPHONE_3GS: UTType = UTType {
    identifier: "com.apple.iphone-3gs",
    conforms_to: r#"["com.apple.iphone"]"#,
    tags: r#"{}"#,
    description: "iPhone 3GS",
};
pub const COM_APPLE_IPHONE_4: UTType = UTType {
    identifier: "com.apple.iphone-4",
    conforms_to: r#"["com.apple.iphone"]"#,
    tags: r#"{}"#,
    description: "iPhone 4",
};
pub const COM_APPLE_IPHONE_8: UTType = UTType {
    identifier: "com.apple.iphone-8",
    conforms_to: r#"["com.apple.iphone"]"#,
    tags: r#"{}"#,
    description: "iPhone 8",
};
pub const COM_APPLE_IPHONE_8_2: UTType = UTType {
    identifier: "com.apple.iphone-8-2",
    conforms_to: r#"["com.apple.iphone-8"]"#,
    tags: r#"{}"#,
    description: "iPhone 8 (Model A1863, A1905, A1906, A1907)",
};
pub const COM_APPLE_IPHONE_8_7: UTType = UTType {
    identifier: "com.apple.iphone-8-7",
    conforms_to: r#"["com.apple.iphone-8"]"#,
    tags: r#"{}"#,
    description: "iPhone 8 (Model A1863, A1905, A1906, A1907)",
};
pub const COM_APPLE_IPHONE_8_8: UTType = UTType {
    identifier: "com.apple.iphone-8-8",
    conforms_to: r#"["com.apple.iphone-8"]"#,
    tags: r#"{}"#,
    description: "iPhone 8 (Model A1863, A1905, A1906, A1907)",
};
pub const COM_APPLE_IPHONE_8_PLUS: UTType = UTType {
    identifier: "com.apple.iphone-8-plus",
    conforms_to: r#"["com.apple.iphone"]"#,
    tags: r#"{}"#,
    description: "iPhone 8 Plus",
};
pub const COM_APPLE_IPHONE_8_PLUS_2: UTType = UTType {
    identifier: "com.apple.iphone-8-plus-2",
    conforms_to: r#"["com.apple.iphone-8-plus"]"#,
    tags: r#"{}"#,
    description: "iPhone 8 Plus (Model A1864, A1897, A1898, A1899)",
};
pub const COM_APPLE_IPHONE_8_PLUS_3: UTType = UTType {
    identifier: "com.apple.iphone-8-plus-3",
    conforms_to: r#"["com.apple.iphone-8-plus"]"#,
    tags: r#"{}"#,
    description: "iPhone 8 Plus (Model A1864, A1897, A1898, A1899)",
};
pub const COM_APPLE_IPHONE_8_PLUS_1: UTType = UTType {
    identifier: "com.apple.iphone-8-plus-1",
    conforms_to: r#"["com.apple.iphone-8-plus"]"#,
    tags: r#"{}"#,
    description: "iPhone 8 Plus (Model A1864, A1897, A1898, A1899)",
};
pub const COM_APPLE_IPHONE_X: UTType = UTType {
    identifier: "com.apple.iphone-x",
    conforms_to: r#"["com.apple.homebuttonless-iphone"]"#,
    tags: r#"{}"#,
    description: "iPhone X",
};
pub const COM_APPLE_IPHONE_X_1: UTType = UTType {
    identifier: "com.apple.iphone-x-1",
    conforms_to: r#"["com.apple.iphone-x"]"#,
    tags: r#"{}"#,
    description: "iPhone X (Model A1865, A1901, A1902, A1903)",
};
pub const COM_APPLE_IPHONE_X_2: UTType = UTType {
    identifier: "com.apple.iphone-x-2",
    conforms_to: r#"["com.apple.iphone-x"]"#,
    tags: r#"{}"#,
    description: "iPhone X (Model A1865, A1901, A1902, A1903)",
};
pub const COM_APPLE_LEGACY_IPOD: UTType = UTType {
    identifier: "com.apple.legacy-ipod",
    conforms_to: r#"["com.apple.device"]"#,
    tags: r#"{}"#,
    description: "iPod",
};
pub const COM_APPLE_IPOD_SHUFFLE: UTType = UTType {
    identifier: "com.apple.ipod-shuffle",
    conforms_to: r#"["com.apple.legacy-ipod"]"#,
    tags: r#"{}"#,
    description: "iPod Shuffle",
};
pub const COM_APPLE_IPOD_SHUFFLE_GEN1: UTType = UTType {
    identifier: "com.apple.ipod-shuffle-gen1",
    conforms_to: r#"["com.apple.ipod-shuffle"]"#,
    tags: r#"{}"#,
    description: "iPod Shuffle",
};
pub const COM_APPLE_IPOD_SHUFFLE_GEN2: UTType = UTType {
    identifier: "com.apple.ipod-shuffle-gen2",
    conforms_to: r#"["com.apple.ipod-shuffle"]"#,
    tags: r#"{}"#,
    description: "iPod Shuffle (2th generation)",
};
pub const COM_APPLE_IPOD_SHUFFLE_GEN3: UTType = UTType {
    identifier: "com.apple.ipod-shuffle-gen3",
    conforms_to: r#"["com.apple.ipod-shuffle"]"#,
    tags: r#"{}"#,
    description: "iPod Shuffle (3th generation)",
};
pub const COM_APPLE_IPOD_SHUFFLE_GEN4: UTType = UTType {
    identifier: "com.apple.ipod-shuffle-gen4",
    conforms_to: r#"["com.apple.ipod-shuffle"]"#,
    tags: r#"{}"#,
    description: "iPod Shuffle (4th generation)",
};
pub const COM_APPLE_IPOD_NANO: UTType = UTType {
    identifier: "com.apple.ipod-nano",
    conforms_to: r#"["com.apple.legacy-ipod"]"#,
    tags: r#"{}"#,
    description: "iPod Nano",
};
pub const COM_APPLE_IPOD_MINI: UTType = UTType {
    identifier: "com.apple.ipod-mini",
    conforms_to: r#"["com.apple.legacy-ipod"]"#,
    tags: r#"{}"#,
    description: "iPod Nano",
};
pub const COM_APPLE_IPOD_CLASSIC: UTType = UTType {
    identifier: "com.apple.ipod-classic",
    conforms_to: r#"["com.apple.legacy-ipod"]"#,
    tags: r#"{}"#,
    description: "iPod Classic",
};
pub const COM_APPLE_IPOD: UTType = UTType {
    identifier: "com.apple.ipod",
    conforms_to: r#"["com.apple.ios-device"]"#,
    tags: r#"{}"#,
    description: "iPod",
};
pub const COM_APPLE_IPOD_TOUCH: UTType = UTType {
    identifier: "com.apple.ipod-touch",
    conforms_to: r#"["com.apple.ipod"]"#,
    tags: r#"{}"#,
    description: "iPod touch",
};
pub const COM_APPLE_IPOD_TOUCH_2: UTType = UTType {
    identifier: "com.apple.ipod-touch-2",
    conforms_to: r#"["com.apple.ipod"]"#,
    tags: r#"{}"#,
    description: "iPod touch",
};
pub const COM_APPLE_IPOD_TOUCH_3: UTType = UTType {
    identifier: "com.apple.ipod-touch-3",
    conforms_to: r#"["com.apple.ipod"]"#,
    tags: r#"{}"#,
    description: "iPod touch",
};
pub const COM_APPLE_IPOD_TOUCH_4: UTType = UTType {
    identifier: "com.apple.ipod-touch-4",
    conforms_to: r#"["com.apple.ipod"]"#,
    tags: r#"{}"#,
    description: "iPod touch",
};
pub const COM_APPLE_IPAD: UTType = UTType {
    identifier: "com.apple.ipad",
    conforms_to: r#"["com.apple.ios-device"]"#,
    tags: r#"{}"#,
    description: "iPad",
};
pub const COM_APPLE_HOMEBUTTONLESS_IPAD: UTType = UTType {
    identifier: "com.apple.homebuttonless-ipad",
    conforms_to: r#"["com.apple.ipad", "com.apple.homebuttonless-device"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_HOMEBUTTONLESS_IPHONE: UTType = UTType {
    identifier: "com.apple.homebuttonless-iphone",
    conforms_to: r#"["com.apple.iphone", "com.apple.homebuttonless-device"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const COM_APPLE_WATCH: UTType = UTType {
    identifier: "com.apple.watch",
    conforms_to: r#"["com.apple.ios-device"]"#,
    tags: r#"{}"#,
    description: "Apple Watch",
};
pub const COM_APPLE_AIRPORT_EXPRESS: UTType = UTType {
    identifier: "com.apple.airport-express",
    conforms_to: r#"["com.apple.device"]"#,
    tags: r#"{}"#,
    description: "AirPort Express",
};
pub const COM_APPLE_AIRPORT: UTType = UTType {
    identifier: "com.apple.airport",
    conforms_to: r#"["com.apple.device"]"#,
    tags: r#"{}"#,
    description: "AirPort Extreme",
};
pub const COM_APPLE_TIME_CAPSULE: UTType = UTType {
    identifier: "com.apple.time-capsule",
    conforms_to: r#"["com.apple.device"]"#,
    tags: r#"{}"#,
    description: "Time Capsule",
};
pub const COM_APPLE_AIRPORT_EXTREME_TOWER: UTType = UTType {
    identifier: "com.apple.airport-extreme-tower",
    conforms_to: r#"["com.apple.airport"]"#,
    tags: r#"{}"#,
    description: "AirPort Extreme",
};
pub const COM_APPLE_AIRPORT_TIME_CAPSULE_TOWER: UTType = UTType {
    identifier: "com.apple.airport-time-capsule-tower",
    conforms_to: r#"["com.apple.time-capsule"]"#,
    tags: r#"{}"#,
    description: "Time Capsule",
};
pub const COM_APPLE_CINEMA_DISPLAY: UTType = UTType {
    identifier: "com.apple.cinema-display",
    conforms_to: r#"["public.display"]"#,
    tags: r#"{}"#,
    description: "Cinema Display",
};
pub const COM_APPLE_LED_CINEMA_DISPLAY_24: UTType = UTType {
    identifier: "com.apple.led-cinema-display-24",
    conforms_to: r#"["public.display"]"#,
    tags: r#"{}"#,
    description: "LED Cinema Display",
};
pub const COM_APPLE_LED_CINEMA_DISPLAY_27: UTType = UTType {
    identifier: "com.apple.led-cinema-display-27",
    conforms_to: r#"["public.display"]"#,
    tags: r#"{}"#,
    description: "LED Cinema Display",
};
pub const COM_APPLE_PRO_DISPLAY_XDR: UTType = UTType {
    identifier: "com.apple.pro-display-xdr",
    conforms_to: r#"["public.display"]"#,
    tags: r#"{}"#,
    description: "Pro Display XDR",
};
pub const PUBLIC_STORAGE: UTType = UTType {
    identifier: "public.storage",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Storage",
};
pub const COM_APPLE_STORAGE_EXTERNAL: UTType = UTType {
    identifier: "com.apple.storage-external",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: "External Disk",
};
pub const COM_APPLE_GENERIC_TIME_MACHINE_DISK: UTType = UTType {
    identifier: "com.apple.generic-time-machine-disk",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: "Generic Time Machine Disk",
};
pub const COM_APPLE_STORAGE_NETBOOT: UTType = UTType {
    identifier: "com.apple.storage-netboot",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: "NetBootVolume",
};
pub const COM_APPLE_FILE_SERVER: UTType = UTType {
    identifier: "com.apple.file-server",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: "File Server",
};
pub const COM_APPLE_STORAGE_INTERNAL: UTType = UTType {
    identifier: "com.apple.storage-internal",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: "Internal Disk",
};
pub const COM_APPLE_STORAGE_REMOVABLE: UTType = UTType {
    identifier: "com.apple.storage-removable",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: "Removable Disk",
};
pub const COM_APPLE_FILE_VAULT: UTType = UTType {
    identifier: "com.apple.file-vault",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "File Vault",
};
pub const COM_APPLE_GENERIC_AIRDISK: UTType = UTType {
    identifier: "com.apple.generic-airdisk",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: "Generic AirDisk",
};
pub const PUBLIC_OPTICAL_STORAGE_MEDIA: UTType = UTType {
    identifier: "public.optical-storage-media",
    conforms_to: r#"["public.storage"]"#,
    tags: r#"{}"#,
    description: "optical storage media",
};
pub const PUBLIC_CD_BASED_MEDIA: UTType = UTType {
    identifier: "public.cd-based-media",
    conforms_to: r#"["public.optical-storage-media"]"#,
    tags: r#"{}"#,
    description: "CD",
};
pub const PUBLIC_CD_MEDIA: UTType = UTType {
    identifier: "public.cd-media",
    conforms_to: r#"["public.cd-based-media"]"#,
    tags: r#"{}"#,
    description: "CD",
};
pub const PUBLIC_CD_R_MEDIA: UTType = UTType {
    identifier: "public.cd-r-media",
    conforms_to: r#"["public.cd-based-media"]"#,
    tags: r#"{}"#,
    description: "CD-R",
};
pub const PUBLIC_CD_RW_MEDIA: UTType = UTType {
    identifier: "public.cd-rw-media",
    conforms_to: r#"["public.cd-based-media"]"#,
    tags: r#"{}"#,
    description: "CD-RW",
};
pub const PUBLIC_DVD_BASED_MEDIA: UTType = UTType {
    identifier: "public.dvd-based-media",
    conforms_to: r#"["public.optical-storage-media"]"#,
    tags: r#"{}"#,
    description: "DVD",
};
pub const PUBLIC_DVD_MEDIA: UTType = UTType {
    identifier: "public.dvd-media",
    conforms_to: r#"["public.dvd-based-media"]"#,
    tags: r#"{}"#,
    description: "DVD",
};
pub const PUBLIC_DVD_R_MEDIA: UTType = UTType {
    identifier: "public.dvd-r-media",
    conforms_to: r#"["public.dvd-based-media"]"#,
    tags: r#"{}"#,
    description: "DVD-R",
};
pub const PUBLIC_DVD_RW_MEDIA: UTType = UTType {
    identifier: "public.dvd-rw-media",
    conforms_to: r#"["public.dvd-based-media"]"#,
    tags: r#"{}"#,
    description: "DVD-RW",
};
pub const PUBLIC_DVD_RAM_MEDIA: UTType = UTType {
    identifier: "public.dvd-ram-media",
    conforms_to: r#"["public.dvd-based-media"]"#,
    tags: r#"{}"#,
    description: "DVD-RAM",
};
pub const PUBLIC_DVD_PLUS_R_MEDIA: UTType = UTType {
    identifier: "public.dvd-plus-r-media",
    conforms_to: r#"["public.dvd-based-media"]"#,
    tags: r#"{}"#,
    description: "DVD+R",
};
pub const PUBLIC_DVD_PLUS_RW_MEDIA: UTType = UTType {
    identifier: "public.dvd-plus-rw-media",
    conforms_to: r#"["public.dvd-based-media"]"#,
    tags: r#"{}"#,
    description: "DVD+RW",
};
pub const PUBLIC_HD_DVD_BASED_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-based-media",
    conforms_to: r#"["public.optical-storage-media"]"#,
    tags: r#"{}"#,
    description: "HD DVD",
};
pub const PUBLIC_HD_DVD_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-media",
    conforms_to: r#"["public.hd-dvd-based-media"]"#,
    tags: r#"{}"#,
    description: "HD DVD",
};
pub const PUBLIC_HD_DVD_R_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-r-media",
    conforms_to: r#"["public.hd-dvd-based-media"]"#,
    tags: r#"{}"#,
    description: "HD DVD-R",
};
pub const PUBLIC_HD_DVD_RW_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-rw-media",
    conforms_to: r#"["public.hd-dvd-based-media"]"#,
    tags: r#"{}"#,
    description: "HD DVD-RW",
};
pub const PUBLIC_HD_DVD_RAM_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-ram-media",
    conforms_to: r#"["public.hd-dvd-based-media"]"#,
    tags: r#"{}"#,
    description: "HD DVD-RAM",
};
pub const PUBLIC_APP_CATEGORY: UTType = UTType {
    identifier: "public.app-category",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Application",
};
pub const PUBLIC_APP_CATEGORY_BUSINESS: UTType = UTType {
    identifier: "public.app-category.business",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Business",
};
pub const PUBLIC_APP_CATEGORY_DEVELOPER_TOOLS: UTType = UTType {
    identifier: "public.app-category.developer-tools",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Developer Tools",
};
pub const PUBLIC_APP_CATEGORY_EDUCATION: UTType = UTType {
    identifier: "public.app-category.education",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Education",
};
pub const PUBLIC_APP_CATEGORY_ENTERTAINMENT: UTType = UTType {
    identifier: "public.app-category.entertainment",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Entertainment",
};
pub const PUBLIC_APP_CATEGORY_FINANCE: UTType = UTType {
    identifier: "public.app-category.finance",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Finance",
};
pub const PUBLIC_APP_CATEGORY_GAMES: UTType = UTType {
    identifier: "public.app-category.games",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Games",
};
pub const PUBLIC_APP_CATEGORY_ACTION_GAMES: UTType = UTType {
    identifier: "public.app-category.action-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Action Games",
};
pub const PUBLIC_APP_CATEGORY_ADVENTURE_GAMES: UTType = UTType {
    identifier: "public.app-category.adventure-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Adventure Games",
};
pub const PUBLIC_APP_CATEGORY_ARCADE_GAMES: UTType = UTType {
    identifier: "public.app-category.arcade-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Arcade Games",
};
pub const PUBLIC_APP_CATEGORY_BOARD_GAMES: UTType = UTType {
    identifier: "public.app-category.board-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Board Games",
};
pub const PUBLIC_APP_CATEGORY_CARD_GAMES: UTType = UTType {
    identifier: "public.app-category.card-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Card Games",
};
pub const PUBLIC_APP_CATEGORY_CASINO_GAMES: UTType = UTType {
    identifier: "public.app-category.casino-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Casino Games",
};
pub const PUBLIC_APP_CATEGORY_DICE_GAMES: UTType = UTType {
    identifier: "public.app-category.dice-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Dice Games",
};
pub const PUBLIC_APP_CATEGORY_EDUCATIONAL_GAMES: UTType = UTType {
    identifier: "public.app-category.educational-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Educational Games",
};
pub const PUBLIC_APP_CATEGORY_FAMILY_GAMES: UTType = UTType {
    identifier: "public.app-category.family-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Family Games",
};
pub const PUBLIC_APP_CATEGORY_KIDS_GAMES: UTType = UTType {
    identifier: "public.app-category.kids-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Kids Games",
};
pub const PUBLIC_APP_CATEGORY_MUSIC_GAMES: UTType = UTType {
    identifier: "public.app-category.music-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Music Games",
};
pub const PUBLIC_APP_CATEGORY_PUZZLE_GAMES: UTType = UTType {
    identifier: "public.app-category.puzzle-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Puzzle Games",
};
pub const PUBLIC_APP_CATEGORY_RACING_GAMES: UTType = UTType {
    identifier: "public.app-category.racing-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Racing Games",
};
pub const PUBLIC_APP_CATEGORY_ROLE_PLAYING_GAMES: UTType = UTType {
    identifier: "public.app-category.role-playing-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Role Playing Games",
};
pub const PUBLIC_APP_CATEGORY_SIMULATION_GAMES: UTType = UTType {
    identifier: "public.app-category.simulation-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Simulation Games",
};
pub const PUBLIC_APP_CATEGORY_SPORTS_GAMES: UTType = UTType {
    identifier: "public.app-category.sports-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Sports Games",
};
pub const PUBLIC_APP_CATEGORY_STRATEGY_GAMES: UTType = UTType {
    identifier: "public.app-category.strategy-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Strategy Games",
};
pub const PUBLIC_APP_CATEGORY_TRIVIA_GAMES: UTType = UTType {
    identifier: "public.app-category.trivia-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Trivia Games",
};
pub const PUBLIC_APP_CATEGORY_WORD_GAMES: UTType = UTType {
    identifier: "public.app-category.word-games",
    conforms_to: r#"["public.app-category.games"]"#,
    tags: r#"{}"#,
    description: "Word Games",
};
pub const PUBLIC_APP_CATEGORY_GRAPHICS_DESIGN: UTType = UTType {
    identifier: "public.app-category.graphics-design",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Graphics n Design",
};
pub const PUBLIC_APP_CATEGORY_HEALTHCARE_FITNESS: UTType = UTType {
    identifier: "public.app-category.healthcare-fitness",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Healthcare n Fitness",
};
pub const PUBLIC_APP_CATEGORY_LIFESTYLE: UTType = UTType {
    identifier: "public.app-category.lifestyle",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Lifestyle",
};
pub const PUBLIC_APP_CATEGORY_MEDICAL: UTType = UTType {
    identifier: "public.app-category.medical",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Medical",
};
pub const PUBLIC_APP_CATEGORY_MUSIC: UTType = UTType {
    identifier: "public.app-category.music",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Music",
};
pub const PUBLIC_APP_CATEGORY_NEWS: UTType = UTType {
    identifier: "public.app-category.news",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "News",
};
pub const PUBLIC_APP_CATEGORY_PHOTOGRAPHY: UTType = UTType {
    identifier: "public.app-category.photography",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Photography",
};
pub const PUBLIC_APP_CATEGORY_PRODUCTIVITY: UTType = UTType {
    identifier: "public.app-category.productivity",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Productivity",
};
pub const PUBLIC_APP_CATEGORY_REFERENCE: UTType = UTType {
    identifier: "public.app-category.reference",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Reference",
};
pub const PUBLIC_APP_CATEGORY_SOCIAL_NETWORKING: UTType = UTType {
    identifier: "public.app-category.social-networking",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Social Networking",
};
pub const PUBLIC_APP_CATEGORY_SPORTS: UTType = UTType {
    identifier: "public.app-category.sports",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Sports",
};
pub const PUBLIC_APP_CATEGORY_TRAVEL: UTType = UTType {
    identifier: "public.app-category.travel",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Travel",
};
pub const PUBLIC_APP_CATEGORY_UTILITIES: UTType = UTType {
    identifier: "public.app-category.utilities",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Utilities",
};
pub const PUBLIC_APP_CATEGORY_VIDEO: UTType = UTType {
    identifier: "public.app-category.video",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Video",
};
pub const PUBLIC_APP_CATEGORY_WEATHER: UTType = UTType {
    identifier: "public.app-category.weather",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Weather",
};
pub const PUBLIC_APP_CATEGORY_BOOKMARKS: UTType = UTType {
    identifier: "public.app-category.bookmarks",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Bookmarks",
};
pub const PUBLIC_APP_CATEGORY_BOOKS: UTType = UTType {
    identifier: "public.app-category.books",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Books",
};
pub const PUBLIC_APP_CATEGORY_NAVIGATION: UTType = UTType {
    identifier: "public.app-category.navigation",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Navigation",
};
pub const PUBLIC_APP_CATEGORY_PHOTOGRAPHY_AND_VIDEO: UTType = UTType {
    identifier: "public.app-category.photography-and-video",
    conforms_to: r#"["public.app-category.photography", "public.app-category.video"]"#,
    tags: r#"{}"#,
    description: "Photo & Video",
};
pub const PUBLIC_APP_CATEGORY_FOOD_AND_DRINK: UTType = UTType {
    identifier: "public.app-category.food-and-drink",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Food & Drink",
};
pub const PUBLIC_APP_CATEGORY_SHOPPING: UTType = UTType {
    identifier: "public.app-category.shopping",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Shopping",
};
pub const PUBLIC_APP_CATEGORY_MAGAZINES_AND_NEWSPAPERS: UTType = UTType {
    identifier: "public.app-category.magazines-and-newspapers",
    conforms_to: r#"["public.app-category"]"#,
    tags: r#"{}"#,
    description: "Magazines & Newspapers",
};
pub const COM_APPLE_STRUCTURED_TEXT: UTType = UTType {
    identifier: "com.apple.structured-text",
    conforms_to: r#"["public.plain-text"]"#,
    tags: r#"{}"#,
    description: "Structured Text",
};
pub const COM_APPLE_STRUCTURED_TEXT_DATE: UTType = UTType {
    identifier: "com.apple.structured-text.date",
    conforms_to: r#"["com.apple.structured-text"]"#,
    tags: r#"{}"#,
    description: "Date (Structured Text)",
};
pub const COM_APPLE_STRUCTURED_TEXT_ADDRESS: UTType = UTType {
    identifier: "com.apple.structured-text.address",
    conforms_to: r#"["com.apple.structured-text"]"#,
    tags: r#"{}"#,
    description: "Address (Structured Text)",
};
pub const COM_APPLE_STRUCTURED_TEXT_TELEPHONE_NUMBER: UTType = UTType {
    identifier: "com.apple.structured-text.telephone-number",
    conforms_to: r#"["com.apple.structured-text"]"#,
    tags: r#"{}"#,
    description: "Telephone Number (Structured Text)",
};
pub const COM_APPLE_STRUCTURED_TEXT_TRANSIT_INFORMATION: UTType = UTType {
    identifier: "com.apple.structured-text.transit-information",
    conforms_to: r#"["com.apple.structured-text"]"#,
    tags: r#"{}"#,
    description: "Transit Information (Structured Text)",
};
pub const COM_APPLE_ACTIVE_WEBPAGE: UTType = UTType {
    identifier: "com.apple.active-webpage",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Active Web Page",
};
pub const SYSTEM_TYPES: [UTType; 667] = [
    PUBLIC_ITEM,
    PUBLIC_DATA,
    PUBLIC_DIRECTORY,
    PUBLIC_CONTENT,
    PUBLIC_COMPOSITE_CONTENT,
    PUBLIC_NAMED_PIPE,
    PUBLIC_CHARACTER_SPECIAL,
    PUBLIC_BLOCK_SPECIAL,
    PUBLIC_SOCKET,
    PUBLIC_EXECUTABLE,
    PUBLIC_UNIX_EXECUTABLE,
    COM_APPLE_APPLICATION,
    PUBLIC_ARCHIVE,
    PUBLIC_BOOKMARK,
    PUBLIC_DATABASE,
    COM_APPLE_CSSTORE,
    PUBLIC_PRESENTATION,
    PUBLIC_SPREADSHEET,
    COM_APPLE_ICLOUD,
    PUBLIC_3D_CONTENT,
    PUBLIC_ALEMBIC,
    PUBLIC_GEOMETRY_DEFINITION_FORMAT,
    PUBLIC_STANDARD_TESSELATED_GEOMETRY_FORMAT,
    PUBLIC_POLYGON_FILE_FORMAT,
    COM_PIXAR_UNIVERSAL_SCENE_DESCRIPTION,
    COM_PIXAR_UNIVERSAL_SCENE_DESCRIPTION_MOBILE,
    COM_APPLE_REALITY,
    COM_APPLE_SCENEKIT_SCENE,
    COM_APPLE_AROBJECT,
    PUBLIC_MESSAGE,
    PUBLIC_EMAIL_MESSAGE,
    PUBLIC_TO_DO_ITEM,
    PUBLIC_CALENDAR_EVENT,
    COM_APPLE_ICAL_VCS,
    COM_APPLE_ICAL_ICS,
    PUBLIC_CONTACT,
    PUBLIC_VCARD,
    COM_APPLE_SHAZAMSIGNATURE,
    COM_APPLE_SHAZAMCATALOG,
    PUBLIC_TEXT,
    PUBLIC_PLAIN_TEXT,
    PUBLIC_UTF8_PLAIN_TEXT,
    PUBLIC_UTF16_EXTERNAL_PLAIN_TEXT,
    PUBLIC_UTF16_PLAIN_TEXT,
    COM_APPLE_TRADITIONAL_MAC_PLAIN_TEXT,
    PUBLIC_CASE_INSENSITIVE_TEXT,
    PUBLIC_LOG,
    COM_APPLE_LOG,
    COM_APPLE_SHUTDOWNSTALL,
    COM_APPLE_GPURESTART,
    COM_APPLE_CRASHREPORT,
    COM_APPLE_HANGREPORT,
    COM_APPLE_SPINREPORT,
    COM_APPLE_PANICREPORT,
    COM_APPLE_KTRACE,
    PUBLIC_FILENAME_EXTENSION,
    PUBLIC_MIME_TYPE,
    COM_APPLE_OSTYPE,
    COM_APPLE_NSPBOARD_TYPE,
    COM_APPLE_DEVICE_MODEL_CODE,
    COM_APPLE_PASTEBOARD_PROMISED_FILE_URL,
    COM_APPLE_PASTEBOARD_PROMISED_FILE_CONTENT_TYPE,
    COM_APPLE_COCOA_PASTEBOARD_COLOR,
    COM_APPLE_COCOA_PASTEBOARD_SOUND,
    COM_APPLE_COCOA_PASTEBOARD_CHARACTER_FORMATTING,
    COM_APPLE_COCOA_PASTEBOARD_PARAGRAPH_FORMATTING,
    COM_APPLE_COCOA_PASTEBOARD_MULTIPLE_TEXT_SELECTION,
    COM_APPLE_COCOA_PASTEBOARD_FIND_PANEL_SEARCH_OPTIONS,
    COM_APPLE_MAPKIT_MAP_ITEM,
    COM_APPLE_RESOLVABLE,
    PUBLIC_SYMLINK,
    COM_APPLE_MOUNT_POINT,
    COM_APPLE_BOOKMARK,
    COM_APPLE_ALIAS_FILE,
    COM_APPLE_ALIAS_RECORD,
    COM_APPLE_ICLOUD_FILE_FAULT,
    COM_APPLE_FINDER_CLIPPING,
    COM_APPLE_FINDER_SOUND_CLIPPING,
    COM_APPLE_FINDER_TEXTCLIPPING,
    COM_APPLE_FINDER_PICTCLIPPING,
    COM_APPLE_FINDER_BURN_FOLDER,
    COM_APPLE_ICONSET,
    COM_APPLE_FINDER_SMART_FOLDER,
    COM_APPLE_FINDER_RECENT_ITEMS,
    PUBLIC_OBJECT_CODE,
    COM_APPLE_MACH_O_BINARY,
    COM_APPLE_MACH_O_OBJECT,
    COM_APPLE_MACH_O_EXECUTABLE,
    COM_APPLE_X11_MACH_O_EXECUTABLE,
    COM_APPLE_MACH_O_CORE,
    COM_APPLE_MACH_O_DYLIB,
    COM_APPLE_MACH_O_BUNDLE,
    COM_APPLE_PEF_BINARY,
    PUBLIC_ELF_BINARY,
    COM_MICROSOFT_WINDOWS_EXECUTABLE,
    COM_MICROSOFT_WINDOWS_DYNAMIC_LINK_LIBRARY,
    COM_SUN_JAVA_CLASS,
    COM_SUN_JAVA_ARCHIVE,
    COM_SUN_WEB_APPLICATION_ARCHIVE,
    COM_APPLE_QUARTZ_COMPOSER_COMPOSITION,
    COM_APPLE_BOM_ARCHIVE,
    PUBLIC_DISK_IMAGE,
    ORG_GNU_GNU_TAR_ARCHIVE,
    PUBLIC_TAR_ARCHIVE,
    ORG_GNU_GNU_ZIP_ARCHIVE,
    ORG_GNU_GNU_ZIP_TAR_ARCHIVE,
    PUBLIC_BZIP2_ARCHIVE,
    PUBLIC_TAR_BZIP2_ARCHIVE,
    COM_APPLE_BINHEX_ARCHIVE,
    COM_APPLE_MACBINARY_ARCHIVE,
    COM_APPLE_APPLESINGLE_ARCHIVE,
    PUBLIC_UUENCODED_ARCHIVE,
    PUBLIC_Z_ARCHIVE,
    COM_APPLE_BOM_COMPRESSED_CPIO,
    PUBLIC_CPIO_ARCHIVE,
    COM_PKWARE_ZIP_ARCHIVE,
    PUBLIC_ZIP_ARCHIVE,
    COM_APPLE_XAR_ARCHIVE,
    COM_APPLE_XIP_ARCHIVE,
    COM_APPLE_INSTALLER_PACKAGE_ARCHIVE,
    COM_APPLE_ARCHIVE,
    COM_APPLE_ENCRYPTED_ARCHIVE,
    PUBLIC_URL,
    PUBLIC_FILE_URL,
    PUBLIC_URL_NAME,
    PUBLIC_STORED_URL,
    COM_APPLE_INTERNET_LOCATION,
    COM_APPLE_WEB_INTERNET_LOCATION,
    COM_APPLE_VNC_INTERNET_LOCATION,
    COM_APPLE_MAIL_INTERNET_LOCATION,
    COM_APPLE_AFP_INTERNET_LOCATION,
    COM_APPLE_FILE_INTERNET_LOCATION,
    COM_APPLE_FTP_INTERNET_LOCATION,
    COM_APPLE_NEWS_INTERNET_LOCATION,
    COM_APPLE_GENERIC_INTERNET_LOCATION,
    COM_MICROSOFT_INTERNET_SHORTCUT,
    COM_APPLE_ITUNES_STORE_URL,
    PUBLIC_DELIMITED_VALUES_TEXT,
    PUBLIC_COMMA_SEPARATED_VALUES_TEXT,
    PUBLIC_TAB_SEPARATED_VALUES_TEXT,
    PUBLIC_UTF8_TAB_SEPARATED_VALUES_TEXT,
    PUBLIC_RTF,
    PUBLIC_HTML,
    PUBLIC_XML,
    PUBLIC_XHTML,
    PUBLIC_RSS,
    PUBLIC_XFD,
    PUBLIC_CSS,
    PUBLIC_PATCH_FILE,
    PUBLIC_JSON,
    PUBLIC_NDJSON,
    PUBLIC_YAML,
    COM_SCENARIST_CLOSED_CAPTION,
    ORG_W3_WEBVTT,
    COM_APPLE_GENERIC_STATIONERY,
    COM_APPLE_PROPERTY_LIST,
    COM_APPLE_XML_PROPERTY_LIST,
    COM_APPLE_BINARY_PROPERTY_LIST,
    COM_APPLE_ASCII_PROPERTY_LIST,
    PUBLIC_SOURCE_CODE,
    PUBLIC_SOURCE_CODE_PREPROCESSED,
    PUBLIC_C_SOURCE,
    PUBLIC_C_SOURCE_PREPROCESSED,
    COM_APPLE_IIG_SOURCE,
    PUBLIC_OBJECTIVE_C_SOURCE,
    PUBLIC_OBJECTIVE_C_SOURCE_PREPROCESSED,
    PUBLIC_C_PLUS_PLUS_SOURCE,
    PUBLIC_C_PLUS_PLUS_SOURCE_PREPROCESSED,
    PUBLIC_OBJECTIVE_C_PLUS_PLUS_SOURCE,
    PUBLIC_OBJECTIVE_C_PLUS_PLUS_SOURCE_PREPROCESSED,
    PUBLIC_C_HEADER,
    PUBLIC_PRECOMPILED_C_HEADER,
    PUBLIC_C_PLUS_PLUS_HEADER,
    PUBLIC_C_PLUS_PLUS_INLINE_HEADER,
    PUBLIC_PRECOMPILED_C_PLUS_PLUS_HEADER,
    PUBLIC_SWIFT_SOURCE,
    COM_SUN_JAVA_SOURCE,
    PUBLIC_SCRIPT,
    PUBLIC_ASSEMBLY_SOURCE,
    COM_APPLE_REZ_SOURCE,
    PUBLIC_LEX_SOURCE,
    PUBLIC_YACC_SOURCE,
    PUBLIC_MIG_SOURCE,
    COM_APPLE_SYMBOL_EXPORT,
    PUBLIC_FORTRAN_SOURCE,
    PUBLIC_FORTRAN_77_SOURCE,
    PUBLIC_FORTRAN_90_SOURCE,
    PUBLIC_FORTRAN_95_SOURCE,
    PUBLIC_PASCAL_SOURCE,
    PUBLIC_ADA_SOURCE,
    PUBLIC_DYLAN_SOURCE,
    COM_NETSCAPE_JAVASCRIPT_SOURCE,
    COM_APPLE_XCODE_DSYM,
    PUBLIC_SHELL_SCRIPT,
    PUBLIC_BASH_SCRIPT,
    PUBLIC_CSH_SCRIPT,
    PUBLIC_KSH_SCRIPT,
    PUBLIC_TCSH_SCRIPT,
    PUBLIC_ZSH_SCRIPT,
    PUBLIC_PERL_SCRIPT,
    PUBLIC_PYTHON_SCRIPT,
    PUBLIC_RUBY_SCRIPT,
    PUBLIC_PHP_SCRIPT,
    COM_SUN_JAVA_WEB_START,
    PUBLIC_MAKE_SOURCE,
    PUBLIC_IMAGE,
    COM_APPLE_LIVE_PHOTO,
    COM_APPLE_PRIVATE_LIVE_PHOTO_BUNDLE,
    PUBLIC_FAX,
    PUBLIC_CAMERA_RAW_IMAGE,
    PUBLIC_JPEG,
    PUBLIC_JPEG_2000,
    PUBLIC_TIFF,
    COM_APPLE_PICT,
    COM_APPLE_MACPAINT_IMAGE,
    PUBLIC_PNG,
    PUBLIC_SVG_IMAGE,
    COM_APPLE_QUICKTIME_IMAGE,
    COM_APPLE_ICNS,
    PUBLIC_XBITMAP_IMAGE,
    PUBLIC_MPO_IMAGE,
    CA_MCGILL_MNI_BIC_MNC,
    ORG_NEMA_DICOM,
    GOV_NIH_NIFTI_1,
    PUBLIC_AUDIOVISUAL_CONTENT,
    PUBLIC_MOVIE,
    PUBLIC_VIDEO,
    PUBLIC_AUDIO,
    COM_APPLE_QUICKTIME_MOVIE,
    PUBLIC_MPEG,
    PUBLIC_MPEG_2_VIDEO,
    PUBLIC_MPEG_4,
    COM_APPLE_M4V_VIDEO,
    COM_APPLE_PROTECTED_MPEG_4_VIDEO,
    PUBLIC_DV_MOVIE,
    PUBLIC_AVI,
    PUBLIC_3GPP,
    PUBLIC_3GPP2,
    PUBLIC_FLC_ANIMATION,
    PUBLIC_MPEG_2_TRANSPORT_STREAM,
    PUBLIC_AUDIOVISUAL_CONTENT_COLLECTION,
    PUBLIC_AVCHD_COLLECTION,
    COM_APPLE_AUDIO_UNIT_PRESET,
    PUBLIC_MP2,
    PUBLIC_MP3,
    PUBLIC_PLAYLIST,
    PUBLIC_M3U_PLAYLIST,
    PUBLIC_PLS_PLAYLIST,
    PUBLIC_MPEG_4_AUDIO,
    COM_APPLE_M4A_AUDIO,
    COM_APPLE_MPEG_4_RINGTONE,
    COM_APPLE_PROTECTED_MPEG_4_AUDIO,
    COM_APPLE_PROTECTED_MPEG_4_AUDIO_B,
    PUBLIC_ULAW_AUDIO,
    PUBLIC_AU_AUDIO,
    PUBLIC_AIFC_AUDIO,
    PUBLIC_AIFF_AUDIO,
    PUBLIC_CDDA_AUDIO,
    PUBLIC_MIDI_AUDIO,
    PUBLIC_DOWNLOADABLE_SOUND,
    COM_APPLE_COREAUDIO_FORMAT,
    PUBLIC_AC3_AUDIO,
    PUBLIC_ENHANCED_AC3_AUDIO,
    ORG_3GPP_ADAPTIVE_MULTI_RATE_AUDIO,
    PUBLIC_AAC_AUDIO,
    COM_AUDIBLE_AA_AUDIO,
    COM_AUDIBLE_AA_AUDIOBOOK,
    COM_AUDIBLE_AAX_AUDIOBOOK,
    COM_SONY_WAVE64,
    PUBLIC_FONT,
    PUBLIC_TRUETYPE_FONT,
    COM_ADOBE_POSTSCRIPT_FONT,
    COM_APPLE_TRUETYPE_DATAFORK_SUITCASE_FONT,
    PUBLIC_OPENTYPE_FONT,
    PUBLIC_OPENTYPE_COLLECTION_FONT,
    PUBLIC_TRUETYPE_TTF_FONT,
    PUBLIC_TRUETYPE_COLLECTION_FONT,
    COM_APPLE_FONT_SUITCASE,
    COM_ADOBE_POSTSCRIPT_LWFN_FONT,
    COM_ADOBE_POSTSCRIPT_PFB_FONT,
    COM_ADOBE_POSTSCRIPT_PFA_FONT,
    COM_APPLE_PROFILE_FONT_ICON,
    COM_APPLE_APPLESCRIPT_TEXT,
    COM_APPLE_APPLESCRIPT_SCRIPT,
    COM_APPLE_APPLESCRIPT_SCRIPT_BUNDLE,
    COM_APPLE_SCRIPTING_DEFINITION,
    PUBLIC_FOLDER,
    COM_APPLE_DROP_FOLDER,
    COM_APPLE_APPLICATIONS_FOLDER,
    COM_APPLE_SERVER_APPLICATIONS_FOLDER,
    COM_APPLE_LIBRARY_FOLDER,
    COM_APPLE_DOCUMENT_TYPE_SYSTEM_FOLDER,
    COM_APPLE_TRASH_EMPTY,
    COM_APPLE_TRASH_FULL,
    COM_APPLE_SITES_FOLDER,
    COM_APPLE_GROUPS_FOLDER,
    COM_APPLE_SERVERS_FOLDER,
    COM_APPLE_DESKTOP_FOLDER,
    COM_APPLE_DOCUMENTS_FOLDER,
    COM_APPLE_DOWNLOADS_FOLDER,
    COM_APPLE_MOVIE_FOLDER,
    COM_APPLE_MUSIC_FOLDER,
    COM_APPLE_PICTURES_FOLDER,
    COM_APPLE_PUBLIC_FOLDER,
    COM_APPLE_HOME_FOLDER,
    COM_APPLE_DEVELOPER_FOLDER,
    COM_APPLE_USERS_FOLDER,
    COM_APPLE_UTILITIES_FOLDER,
    PUBLIC_VOLUME,
    PUBLIC_FILE_SHAREPOINT,
    COM_APPLE_NETWORK_NEIGHBORHOOD,
    COM_APPLE_DOT_MAC,
    COM_APPLE_IDISK,
    COM_APPLE_USER_IDISK,
    COM_APPLE_PACKAGE,
    COM_APPLE_BUNDLE,
    COM_APPLE_GENERIC_BUNDLE,
    COM_APPLE_SYSTEMPREFERENCE_PREFPANE,
    COM_APPLE_SYSTEMPREFERENCE_SCREEN_SAVER,
    COM_APPLE_SYSTEMPREFERENCE_SCREEN_SLIDE_SAVER,
    COM_APPLE_MENU_EXTRA,
    COM_APPLE_LOCALIZABLE_NAME_BUNDLE,
    COM_APPLE_FRAMEWORK,
    COM_APPLE_APPLICATION_BUNDLE,
    COM_APPLE_APPLICATION_FILE,
    COM_APPLE_DEPRECATED_APPLICATION_FILE,
    COM_APPLE_FLAT_RTFD,
    COM_APPLE_INSTALLER_PACKAGE,
    COM_APPLE_INSTALLER_DISTRIBUTION_PACKAGE,
    COM_APPLE_INSTALLER_META_PACKAGE,
    COM_APPLE_INTELLIGENTSUGGESTIONS_ASSETS,
    COM_APPLE_RTFD,
    COM_APPLE_APPLICATION_PLACEHOLDER,
    COM_APPLE_SERVICE_APPLICATION,
    COM_APPLE_PLUGIN,
    COM_APPLE_XPC_SERVICE,
    COM_APPLE_KERNEL_EXTENSION,
    COM_APPLE_APPLICATION_AND_SYSTEM_EXTENSION,
    COM_APPLE_PLUGINKIT,
    COM_APPLE_WEBKIT_PLUGIN,
    COM_APPLE_METADATA_IMPORTER,
    COM_APPLE_QUICKLOOK_GENERATOR,
    COM_APPLE_DASHBOARD_WIDGET,
    COM_APPLE_DRIVER_EXTENSION,
    COM_APPLE_SYSTEM_EXTENSION,
    COM_APPLE_PPP_PLUG_IN,
    COM_APPLE_FILE_SYSTEM_PLUG_IN,
    COM_APPLE_DATA_CONTAINER,
    ORG_OPENXMLFORMATS_OPENXML,
    ORG_OASIS_OPEN_OPENDOCUMENT,
    COM_RSA_PKCS_12,
    PUBLIC_X509_CERTIFICATE,
    COM_APPLE_ALERT,
    COM_APPLE_ALERT_NOTE,
    COM_APPLE_ALERT_CAUTION,
    COM_APPLE_ALERT_STOP,
    COM_APPLE_WEBARCHIVE,
    ORG_IDPF_EPUB_CONTAINER,
    COM_APPLE_LOCALIZED_PDF_BUNDLE,
    ORG_AAFASSOCIATION_ADVANCED_AUTHORING_FORMAT,
    COM_APPLE_TXN_TEXT_MULTIMEDIA_DATA,
    COM_APPLE_COLORSYNC_PROFILE,
    COM_APPLE_PROFILE_BACKGROUND_COLOR,
    COM_APPLE_PROFILE_FONT_AND_COLOR,
    COM_APPLE_COLOR_FILE,
    COM_APPLE_INK_INKTEXT,
    COM_APPLE_MOBILECONFIG,
    COM_APPLE_PROVISIONPROFILE,
    COM_APPLE_CONFIGPROFILE,
    COM_APPLE_USER,
    COM_APPLE_GUEST_USER,
    COM_APPLE_HELP_DOCUMENT,
    COM_APPLE_USER_GROUP,
    COM_APPLE_USER_UNKNOWN,
    COM_APPLE_AIRDROP,
    COM_APPLE_BONJOUR,
    COM_APPLE_NOTIFICATIONS,
    COM_APPLE_MOBILEPROVISION,
    COM_APPLE_PKPASS,
    COM_APPLE_PKPASS_DATA,
    COM_APPLE_PKPASSES_DATA,
    COM_APPLE_WATCHFACE,
    PUBLIC_DEVICE,
    COM_APPLE_VIRTUAL_MACHINE,
    PUBLIC_DISPLAY,
    PUBLIC_SPEAKER,
    PUBLIC_COMPUTER,
    PUBLIC_GENERIC_PC,
    COM_APPLE_DEVICE,
    COM_APPLE_MAC,
    COM_APPLE_MAC_LAPTOP,
    COM_APPLE_MACBOOKPRO_13_RETINA_USBC,
    COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER,
    COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY,
    COM_APPLE_MACBOOKPRO_13_RETINA_USBC_2017,
    COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER_2017,
    COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY_2017,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2017,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2017,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2017,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2018,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2018,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2018,
    COM_APPLE_MACBOOKPRO_13_RETINA_USBC_2019,
    COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER_2019,
    COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY_2019,
    COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID,
    COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER,
    COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY,
    COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_2017,
    COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER_2017,
    COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY_2017,
    COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_2018,
    COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER_2018,
    COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY_2018,
    COM_APPLE_MACBOOKPRO_15_LATE_2018,
    COM_APPLE_MACBOOKPRO_15_SILVER_LATE_2018,
    COM_APPLE_MACBOOKPRO_15_SPACE_GRAY_LATE_2018,
    COM_APPLE_MAC_TOWER,
    COM_APPLE_MAC_RACKMOUNT,
    COM_APPLE_POWERBOOK,
    COM_APPLE_POWERBOOK_G4_TITANIUM,
    COM_APPLE_POWERBOOK_G4_12,
    COM_APPLE_POWERBOOK_G4_15,
    COM_APPLE_POWERBOOK_G4_17,
    COM_APPLE_IBOOK_G4,
    COM_APPLE_POWERMAC,
    COM_APPLE_POWERMAC_G4_QUICKSILVER,
    COM_APPLE_POWERMAC_G4_MIRRORED_DRIVE_DOORS,
    COM_APPLE_POWERMAC_G5,
    COM_APPLE_XSERVE,
    COM_APPLE_XSERVE_G4,
    COM_APPLE_XSERVE_G5,
    COM_APPLE_XSERVE_XEON,
    COM_APPLE_MACMINI,
    COM_APPLE_MACMINI_G4,
    COM_APPLE_MACMINI_CORE_DUO,
    COM_APPLE_MACMINI_UNIBODY,
    COM_APPLE_MACMINI_UNIBODY_NO_OPTICAL,
    COM_APPLE_MACMINI_2018,
    COM_APPLE_EMAC,
    COM_APPLE_IMAC,
    COM_APPLE_IMAC_G4_15,
    COM_APPLE_IMAC_G4_17,
    COM_APPLE_IMAC_G4_20,
    COM_APPLE_IMAC_G5,
    COM_APPLE_IMAC_G5_ISIGHT,
    COM_APPLE_IMAC_CORE_DUO,
    COM_APPLE_IMAC_CORE_2_DUO,
    COM_APPLE_IMAC_ISIGHT_24,
    COM_APPLE_IMAC_ALUMINUM_20,
    COM_APPLE_IMAC_ALUMINUM_24,
    COM_APPLE_IMAC_UNIBODY_21,
    COM_APPLE_IMAC_UNIBODY,
    COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL,
    COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL,
    COM_APPLE_IMAC_15_1,
    COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_MID_2015,
    COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_LATE_2015,
    COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_2017,
    COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2017,
    COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_2019,
    COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2019,
    COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2020,
    COM_APPLE_IMACPRO_2017,
    COM_APPLE_MACBOOK,
    COM_APPLE_MACBOOK_WHITE,
    COM_APPLE_MACBOOK_BLACK,
    COM_APPLE_MACBOOK_UNIBODY,
    COM_APPLE_MACBOOK_UNIBODY_PLASTIC,
    COM_APPLE_MACBOOK_RETINA,
    COM_APPLE_MACBOOK_RETINA_SILVER,
    COM_APPLE_MACBOOK_RETINA_GOLD,
    COM_APPLE_MACBOOK_RETINA_SPACE_GRAY,
    COM_APPLE_MACBOOK_RETINA_ROSE_GOLD,
    COM_APPLE_MACBOOK_RETINA_SILVER_2017,
    COM_APPLE_MACBOOK_RETINA_GOLD_2017,
    COM_APPLE_MACBOOK_RETINA_SPACE_GRAY_2017,
    COM_APPLE_MACBOOK_RETINA_ROSE_GOLD_2017,
    COM_APPLE_MACBOOK_RETINA_GOLD_2018,
    COM_APPLE_MACBOOKPRO_13_UNIBODY,
    COM_APPLE_MACBOOKPRO_13_RETINA_DISPLAY,
    COM_APPLE_MACBOOKPRO,
    COM_APPLE_MACBOOKPRO_15,
    COM_APPLE_MACBOOKPRO_15_UNIBODY,
    COM_APPLE_MACBOOKPRO_15_RETINA_DISPLAY,
    COM_APPLE_MACBOOKPRO_17_UNIBODY,
    COM_APPLE_MACBOOKPRO_17,
    COM_APPLE_MACBOOKPRO_16,
    COM_APPLE_MACBOOKPRO_16_SILVER,
    COM_APPLE_MACBOOKPRO_16_SPACE_GRAY,
    COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_2020,
    COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_SILVER_2020,
    COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_SPACE_GRAY_2020,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2020,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2020,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2020,
    COM_APPLE_MACBOOKPRO_16_MID_2020,
    COM_APPLE_MACBOOKPRO_16_SILVER_MID_2020,
    COM_APPLE_MACBOOKPRO_16_SPACE_GRAY_MID_2020,
    COM_APPLE_MACBOOKAIR,
    COM_APPLE_MACBOOKAIR_11_UNIBODY,
    COM_APPLE_MACBOOKAIR_13_UNIBODY,
    COM_APPLE_MACBOOKAIR_2018,
    COM_APPLE_MACBOOKAIR_2018_SILVER,
    COM_APPLE_MACBOOKAIR_2018_SPACE_GRAY,
    COM_APPLE_MACBOOKAIR_2018_GOLD,
    COM_APPLE_MACBOOKAIR_2019,
    COM_APPLE_MACBOOKAIR_2019_SILVER,
    COM_APPLE_MACBOOKAIR_2019_SPACE_GRAY,
    COM_APPLE_MACBOOKAIR_2019_GOLD,
    COM_APPLE_MACBOOKAIR_2020,
    COM_APPLE_MACBOOKAIR_2020_SILVER,
    COM_APPLE_MACBOOKAIR_2020_SPACE_GRAY,
    COM_APPLE_MACBOOKAIR_2020_GOLD,
    COM_APPLE_MACPRO,
    COM_APPLE_MACPRO_FIREWIRE,
    COM_APPLE_MACPRO_CYLINDER,
    COM_APPLE_MACPRO_2019,
    COM_APPLE_MACPRO_2019_RACKMOUNT,
    COM_APPLE_MACMINI_2020,
    COM_APPLE_MACBOOKAIR_LATE_2020,
    COM_APPLE_MACBOOKAIR_LATE_2020_SILVER,
    COM_APPLE_MACBOOKAIR_LATE_2020_SPACE_GRAY,
    COM_APPLE_MACBOOKAIR_LATE_2020_GOLD,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_LATE_2020,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_LATE_2020,
    COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_LATE_2020,
    COM_APPLE_IMAC_2021,
    COM_APPLE_IMAC_2021_SILVER,
    COM_APPLE_IMAC_2021_YELLOW,
    COM_APPLE_IMAC_2021_GREEN,
    COM_APPLE_IMAC_2021_BLUE,
    COM_APPLE_IMAC_2021_RED,
    COM_APPLE_IMAC_2021_PURPLE,
    COM_APPLE_IMAC_2021_ORANGE,
    COM_APPLE_DEVELOPER_TRANSITION_KIT_2005,
    COM_APPLE_DEVELOPER_TRANSITION_KIT_2020,
    COM_APPLE_IOS_DEVICE,
    COM_APPLE_APPLE_TV,
    COM_APPLE_HOMEPOD,
    COM_APPLE_IOS_SIMULATOR,
    COM_APPLE_IPHONE,
    COM_APPLE_IPHONE_3G,
    COM_APPLE_IPHONE_3GS,
    COM_APPLE_IPHONE_4,
    COM_APPLE_IPHONE_8,
    COM_APPLE_IPHONE_8_2,
    COM_APPLE_IPHONE_8_7,
    COM_APPLE_IPHONE_8_8,
    COM_APPLE_IPHONE_8_PLUS,
    COM_APPLE_IPHONE_8_PLUS_2,
    COM_APPLE_IPHONE_8_PLUS_3,
    COM_APPLE_IPHONE_8_PLUS_1,
    COM_APPLE_IPHONE_X,
    COM_APPLE_IPHONE_X_1,
    COM_APPLE_IPHONE_X_2,
    COM_APPLE_LEGACY_IPOD,
    COM_APPLE_IPOD_SHUFFLE,
    COM_APPLE_IPOD_SHUFFLE_GEN1,
    COM_APPLE_IPOD_SHUFFLE_GEN2,
    COM_APPLE_IPOD_SHUFFLE_GEN3,
    COM_APPLE_IPOD_SHUFFLE_GEN4,
    COM_APPLE_IPOD_NANO,
    COM_APPLE_IPOD_MINI,
    COM_APPLE_IPOD_CLASSIC,
    COM_APPLE_IPOD,
    COM_APPLE_IPOD_TOUCH,
    COM_APPLE_IPOD_TOUCH_2,
    COM_APPLE_IPOD_TOUCH_3,
    COM_APPLE_IPOD_TOUCH_4,
    COM_APPLE_IPAD,
    COM_APPLE_HOMEBUTTONLESS_IPAD,
    COM_APPLE_HOMEBUTTONLESS_IPHONE,
    COM_APPLE_WATCH,
    COM_APPLE_AIRPORT_EXPRESS,
    COM_APPLE_AIRPORT,
    COM_APPLE_TIME_CAPSULE,
    COM_APPLE_AIRPORT_EXTREME_TOWER,
    COM_APPLE_AIRPORT_TIME_CAPSULE_TOWER,
    COM_APPLE_CINEMA_DISPLAY,
    COM_APPLE_LED_CINEMA_DISPLAY_24,
    COM_APPLE_LED_CINEMA_DISPLAY_27,
    COM_APPLE_PRO_DISPLAY_XDR,
    PUBLIC_STORAGE,
    COM_APPLE_STORAGE_EXTERNAL,
    COM_APPLE_GENERIC_TIME_MACHINE_DISK,
    COM_APPLE_STORAGE_NETBOOT,
    COM_APPLE_FILE_SERVER,
    COM_APPLE_STORAGE_INTERNAL,
    COM_APPLE_STORAGE_REMOVABLE,
    COM_APPLE_FILE_VAULT,
    COM_APPLE_GENERIC_AIRDISK,
    PUBLIC_OPTICAL_STORAGE_MEDIA,
    PUBLIC_CD_BASED_MEDIA,
    PUBLIC_CD_MEDIA,
    PUBLIC_CD_R_MEDIA,
    PUBLIC_CD_RW_MEDIA,
    PUBLIC_DVD_BASED_MEDIA,
    PUBLIC_DVD_MEDIA,
    PUBLIC_DVD_R_MEDIA,
    PUBLIC_DVD_RW_MEDIA,
    PUBLIC_DVD_RAM_MEDIA,
    PUBLIC_DVD_PLUS_R_MEDIA,
    PUBLIC_DVD_PLUS_RW_MEDIA,
    PUBLIC_HD_DVD_BASED_MEDIA,
    PUBLIC_HD_DVD_MEDIA,
    PUBLIC_HD_DVD_R_MEDIA,
    PUBLIC_HD_DVD_RW_MEDIA,
    PUBLIC_HD_DVD_RAM_MEDIA,
    PUBLIC_APP_CATEGORY,
    PUBLIC_APP_CATEGORY_BUSINESS,
    PUBLIC_APP_CATEGORY_DEVELOPER_TOOLS,
    PUBLIC_APP_CATEGORY_EDUCATION,
    PUBLIC_APP_CATEGORY_ENTERTAINMENT,
    PUBLIC_APP_CATEGORY_FINANCE,
    PUBLIC_APP_CATEGORY_GAMES,
    PUBLIC_APP_CATEGORY_ACTION_GAMES,
    PUBLIC_APP_CATEGORY_ADVENTURE_GAMES,
    PUBLIC_APP_CATEGORY_ARCADE_GAMES,
    PUBLIC_APP_CATEGORY_BOARD_GAMES,
    PUBLIC_APP_CATEGORY_CARD_GAMES,
    PUBLIC_APP_CATEGORY_CASINO_GAMES,
    PUBLIC_APP_CATEGORY_DICE_GAMES,
    PUBLIC_APP_CATEGORY_EDUCATIONAL_GAMES,
    PUBLIC_APP_CATEGORY_FAMILY_GAMES,
    PUBLIC_APP_CATEGORY_KIDS_GAMES,
    PUBLIC_APP_CATEGORY_MUSIC_GAMES,
    PUBLIC_APP_CATEGORY_PUZZLE_GAMES,
    PUBLIC_APP_CATEGORY_RACING_GAMES,
    PUBLIC_APP_CATEGORY_ROLE_PLAYING_GAMES,
    PUBLIC_APP_CATEGORY_SIMULATION_GAMES,
    PUBLIC_APP_CATEGORY_SPORTS_GAMES,
    PUBLIC_APP_CATEGORY_STRATEGY_GAMES,
    PUBLIC_APP_CATEGORY_TRIVIA_GAMES,
    PUBLIC_APP_CATEGORY_WORD_GAMES,
    PUBLIC_APP_CATEGORY_GRAPHICS_DESIGN,
    PUBLIC_APP_CATEGORY_HEALTHCARE_FITNESS,
    PUBLIC_APP_CATEGORY_LIFESTYLE,
    PUBLIC_APP_CATEGORY_MEDICAL,
    PUBLIC_APP_CATEGORY_MUSIC,
    PUBLIC_APP_CATEGORY_NEWS,
    PUBLIC_APP_CATEGORY_PHOTOGRAPHY,
    PUBLIC_APP_CATEGORY_PRODUCTIVITY,
    PUBLIC_APP_CATEGORY_REFERENCE,
    PUBLIC_APP_CATEGORY_SOCIAL_NETWORKING,
    PUBLIC_APP_CATEGORY_SPORTS,
    PUBLIC_APP_CATEGORY_TRAVEL,
    PUBLIC_APP_CATEGORY_UTILITIES,
    PUBLIC_APP_CATEGORY_VIDEO,
    PUBLIC_APP_CATEGORY_WEATHER,
    PUBLIC_APP_CATEGORY_BOOKMARKS,
    PUBLIC_APP_CATEGORY_BOOKS,
    PUBLIC_APP_CATEGORY_NAVIGATION,
    PUBLIC_APP_CATEGORY_PHOTOGRAPHY_AND_VIDEO,
    PUBLIC_APP_CATEGORY_FOOD_AND_DRINK,
    PUBLIC_APP_CATEGORY_SHOPPING,
    PUBLIC_APP_CATEGORY_MAGAZINES_AND_NEWSPAPERS,
    COM_APPLE_STRUCTURED_TEXT,
    COM_APPLE_STRUCTURED_TEXT_DATE,
    COM_APPLE_STRUCTURED_TEXT_ADDRESS,
    COM_APPLE_STRUCTURED_TEXT_TELEPHONE_NUMBER,
    COM_APPLE_STRUCTURED_TEXT_TRANSIT_INFORMATION,
    COM_APPLE_ACTIVE_WEBPAGE,
];
pub const COM_ADOBE_PDF: UTType = UTType {
    identifier: "com.adobe.pdf",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{}"#,
    description: "PDF document",
};
pub const COM_ADOBE_EDN: UTType = UTType {
    identifier: "com.adobe.edn",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "Adobe DRM Activation Key (EDN)",
};
pub const COM_ADOBE_ETD: UTType = UTType {
    identifier: "com.adobe.etd",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "Adobe Digital Editions (ETD)",
};
pub const COM_ADOBE_XFDF: UTType = UTType {
    identifier: "com.adobe.xfdf",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{}"#,
    description: "Adobe Acrobat Forms Document (XFDF)",
};
pub const COM_ADOBE_FDF: UTType = UTType {
    identifier: "com.adobe.fdf",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{}"#,
    description: "Adobe Acrobat Forms Document (FDF)",
};
pub const COM_ADOBE_POSTSCRIPT: UTType = UTType {
    identifier: "com.adobe.postscript",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{}"#,
    description: "PostScript",
};
pub const COM_ADOBE_ENCAPSULATED_POSTSCRIPT: UTType = UTType {
    identifier: "com.adobe.encapsulated-postscript",
    conforms_to: r#"["com.adobe.postscript"]"#,
    tags: r#"{"public.filename-extension": ["eps"]}"#,
    description: "Encapsulated PostScript",
};
pub const COM_COMPUSERVE_GIF: UTType = UTType {
    identifier: "com.compuserve.gif",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "GIF image",
};
pub const COM_MICROSOFT_BMP: UTType = UTType {
    identifier: "com.microsoft.bmp",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "Windows BMP image",
};
pub const COM_MICROSOFT_ICO: UTType = UTType {
    identifier: "com.microsoft.ico",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "Windows icon image",
};
pub const ORG_WEBMPROJECT_WEBP: UTType = UTType {
    identifier: "org.webmproject.webp",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "WebP Image",
};
pub const ORG_WEBMPROJECT_WEBM: UTType = UTType {
    identifier: "org.webmproject.webm",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{}"#,
    description: "WebM Media",
};
pub const PUBLIC_OFD: UTType = UTType {
    identifier: "public.ofd",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{}"#,
    description: "Open Fixed-layout Document",
};
pub const ORG_OPENOFFICE_TEXT: UTType = UTType {
    identifier: "org.openoffice.text",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["sxw", "sdw"], "public.mime-type": ["application/vnd.sun.xml.writer", "application/vnd.stardivision.writer"]}"#,
    description: "OpenOffice.org 1.0 Text",
};
pub const ORG_OPENOFFICE_TEXT_TEMPLATE: UTType = UTType {
    identifier: "org.openoffice.text-template",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["stw"], "public.mime-type": ["application/vnd.sun.xml.writer.template"]}"#,
    description: "OpenOffice.org 1.0 Text Template",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_TEXT: UTType = UTType {
    identifier: "org.oasis-open.opendocument.text",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["odt"], "public.mime-type": ["application/vnd.oasis.opendocument.text"]}"#,
    description: "Open Document text",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.text-template",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["ott"], "public.mime-type": ["application/vnd.oasis.opendocument.text-template"]}"#,
    description: "Open Document text template",
};
pub const ORG_OPENOFFICE_GRAPHICS: UTType = UTType {
    identifier: "org.openoffice.graphics",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["sxd", "sda"], "public.mime-type": ["application/vnd.sun.xml.draw", "application/vnd.stardivision.draw"]}"#,
    description: "OpenOffice.org 1.0 Drawing",
};
pub const ORG_OPENOFFICE_GRAPHICS_TEMPLATE: UTType = UTType {
    identifier: "org.openoffice.graphics-template",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["std"], "public.mime-type": ["application/vnd.sun.xml.draw.template"]}"#,
    description: "OpenOffice.org 1.0 Drawing Template",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_GRAPHICS: UTType = UTType {
    identifier: "org.oasis-open.opendocument.graphics",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["odg"], "public.mime-type": ["application/vnd.oasis.opendocument.graphics"]}"#,
    description: "Open Document graphics",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_GRAPHICS_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.graphics-template",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["otg"], "public.mime-type": ["application/vnd.oasis.opendocument.graphics-template"]}"#,
    description: "Open Document graphics template",
};
pub const ORG_OPENOFFICE_PRESENTATION: UTType = UTType {
    identifier: "org.openoffice.presentation",
    conforms_to: r#"["public.data", "public.content", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["sxi", "sdd", "sdp"], "public.mime-type": ["application/vnd.sun.xml.impress", "application/vnd.stardivision.impress", "application/vnd.stardivision.impress-packed"]}"#,
    description: "OpenOffice.org 1.0 Presentation",
};
pub const ORG_OPENOFFICE_PRESENTATION_TEMPLATE: UTType = UTType {
    identifier: "org.openoffice.presentation-template",
    conforms_to: r#"["public.data", "public.content", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["sti"], "public.mime-type": ["application/vnd.sun.xml.impress.template"]}"#,
    description: "OpenOffice.org 1.0 Presentation Template",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_PRESENTATION: UTType = UTType {
    identifier: "org.oasis-open.opendocument.presentation",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["odp"], "public.mime-type": ["application/vnd.oasis.opendocument.presentation"]}"#,
    description: "Open Document presentation",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_PRESENTATION_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.presentation-template",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["otp"], "public.mime-type": ["application/vnd.oasis.opendocument.presentation-template"]}"#,
    description: "Open Document presentation template",
};
pub const ORG_OPENOFFICE_SPREADSHEET: UTType = UTType {
    identifier: "org.openoffice.spreadsheet",
    conforms_to: r#"["public.data", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["sxc", "sdc"], "public.mime-type": ["application/vnd.sun.xml.calc", "application/vnd.stardivision.calc"]}"#,
    description: "OpenOffice.org 1.0 Spreadsheet",
};
pub const ORG_OPENOFFICE_SPREADSHEET_TEMPLATE: UTType = UTType {
    identifier: "org.openoffice.spreadsheet-template",
    conforms_to: r#"["public.data", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["stc"], "public.mime-type": ["application/vnd.sun.xml.calc.template"]}"#,
    description: "OpenOffice.org 1.0 Spreadsheet Template",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_SPREADSHEET: UTType = UTType {
    identifier: "org.oasis-open.opendocument.spreadsheet",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["ods"], "public.mime-type": ["application/vnd.oasis.opendocument.spreadsheet"]}"#,
    description: "Open Document spreadsheet",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_SPREADSHEET_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.spreadsheet-template",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["ots"], "public.mime-type": ["application/vnd.oasis.opendocument.spreadsheet-template"]}"#,
    description: "Open Document spreadsheet template",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_CHART: UTType = UTType {
    identifier: "org.oasis-open.opendocument.chart",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["odc"], "public.mime-type": ["application/vnd.oasis.opendocument.chart"]}"#,
    description: "Open Document chart",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_CHART_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.chart-template",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["otc"], "public.mime-type": ["application/vnd.oasis.opendocument.chart-template"]}"#,
    description: "Open Document chart template",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_IMAGE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.image",
    conforms_to: r#"["org.oasis-open.opendocument", "public.image"]"#,
    tags: r#"{"public.filename-extension": ["odi"], "public.mime-type": ["application/vnd.oasis.opendocument.image"]}"#,
    description: "Open Document image",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_IMAGE_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.image-template",
    conforms_to: r#"["org.oasis-open.opendocument", "public.image"]"#,
    tags: r#"{"public.filename-extension": ["oti"], "public.mime-type": ["application/vnd.oasis.opendocument.image-template"]}"#,
    description: "Open Document image template",
};
pub const ORG_OPENOFFICE_FORMULA: UTType = UTType {
    identifier: "org.openoffice.formula",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["sxm", "smf"], "public.mime-type": ["application/vnd.sun.xml.math", "application/vnd.stardivision.math"]}"#,
    description: "OpenOffice.org 1.0 Formula",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_FORMULA: UTType = UTType {
    identifier: "org.oasis-open.opendocument.formula",
    conforms_to: r#"["org.oasis-open.opendocument", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["odf"], "public.mime-type": ["application/vnd.oasis.opendocument.formula"]}"#,
    description: "Open Document formula",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_FORMULA_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.formula-template",
    conforms_to: r#"["org.oasis-open.opendocument", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["otf"], "public.mime-type": ["application/vnd.oasis.opendocument.formula-template"]}"#,
    description: "Open Document formula template",
};
pub const ORG_OPENOFFICE_TEXT_MASTER: UTType = UTType {
    identifier: "org.openoffice.text-master",
    conforms_to: r#"["public.data", "public.content"]"#,
    tags: r#"{"public.filename-extension": ["sxg"], "public.mime-type": ["application/vnd.sun.xml.writer.global"]}"#,
    description: "OpenOffice.org 1.0 Master",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_MASTER: UTType = UTType {
    identifier: "org.oasis-open.opendocument.text-master",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["odm"], "public.mime-type": ["application/vnd.oasis.opendocument.text-master"]}"#,
    description: "Open Document text master",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_WEB: UTType = UTType {
    identifier: "org.oasis-open.opendocument.text-web",
    conforms_to: r#"["org.oasis-open.opendocument", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["oth"], "public.mime-type": ["application/vnd.oasis.opendocument.text-web"]}"#,
    description: "Open Document HTML template",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_DATABASE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.database",
    conforms_to: r#"["public.data", "public.content", "public.database"]"#,
    tags: r#"{"public.filename-extension": ["odb"], "public.mime-type": ["application/vnd.oasis.opendocument.database"]}"#,
    description: "Open Document database",
};
pub const COM_MICROSOFT_WORD_WORDML: UTType = UTType {
    identifier: "com.microsoft.word.wordml",
    conforms_to: r#"["public.xml", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["xml"]}"#,
    description: "Microsoft Word 2003 XML document",
};
pub const ORG_OPENXMLFORMATS_WORDPROCESSINGML_DOCUMENT: UTType = UTType {
    identifier: "org.openxmlformats.wordprocessingml.document",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["docx"], "public.mime-type": ["application/vnd.openxmlformats-officedocument.wordprocessingml.document"]}"#,
    description: "Office Open XML word processing document",
};
pub const ORG_OPENXMLFORMATS_WORDPROCESSINGML_DOCUMENT_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.wordprocessingml.document.macroenabled",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["docm"], "public.mime-type": ["application/vnd.ms-word.document.macroEnabled.12"]}"#,
    description: "Office Open XML word processing document (macros enabled)",
};
pub const ORG_OPENXMLFORMATS_WORDPROCESSINGML_TEMPLATE: UTType = UTType {
    identifier: "org.openxmlformats.wordprocessingml.template",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["dotx"], "public.mime-type": ["application/vnd.openxmlformats-officedocument.wordprocessingml.template"]}"#,
    description: "Office Open XML word processing template",
};
pub const ORG_OPENXMLFORMATS_WORDPROCESSINGML_TEMPLATE_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.wordprocessingml.template.macroenabled",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["dotm"], "public.mime-type": ["application/vnd.ms-word.template.macroEnabled.12"]}"#,
    description: "Office Open XML word processing template (macros enabled)",
};
pub const ORG_OPENXMLFORMATS_SPREADSHEETML_SHEET: UTType = UTType {
    identifier: "org.openxmlformats.spreadsheetml.sheet",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["xlsx"], "public.mime-type": ["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"]}"#,
    description: "Office Open XML spreadsheet",
};
pub const ORG_OPENXMLFORMATS_SPREADSHEETML_SHEET_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.spreadsheetml.sheet.macroenabled",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content", "public.spreadsheet", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["xlsm"], "public.mime-type": ["application/vnd.ms-excel.sheet.macroEnabled.12"]}"#,
    description: "Office Open XML spreadsheet (macros enabled)",
};
pub const COM_MICROSOFT_EXCEL_SHEET_BINARY_MACROENABLED: UTType = UTType {
    identifier: "com.microsoft.excel.sheet.binary.macroenabled",
    conforms_to: r#"["public.zip-archive", "public.composite-content", "public.spreadsheet", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["xlsb"], "public.mime-type": ["application/vnd.ms-excel.sheet.binary.macroEnabled.12"]}"#,
    description: "Microsoft Excel 2007 spreadsheet (macros enabled)",
};
pub const ORG_OPENXMLFORMATS_SPREADSHEETML_TEMPLATE: UTType = UTType {
    identifier: "org.openxmlformats.spreadsheetml.template",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["xltx"], "public.mime-type": ["application/vnd.openxmlformats-officedocument.spreadsheetml.template"]}"#,
    description: "Office Open XML spreadsheet template",
};
pub const ORG_OPENXMLFORMATS_SPREADSHEETML_TEMPLATE_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.spreadsheetml.template.macroenabled",
    conforms_to: r#"["org.openxmlformats.openxml", "public.composite-content", "public.spreadsheet", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["xltm"], "public.mime-type": ["application/vnd.ms-excel.template.macroEnabled.12"]}"#,
    description: "Office Open XML spreadsheet template (macros enabled)",
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_PRESENTATION: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.presentation",
    conforms_to: r#"["org.openxmlformats.openxml", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["pptx"], "public.mime-type": ["application/vnd.openxmlformats-officedocument.presentationml.presentation"]}"#,
    description: "Office Open XML presentation",
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_PRESENTATION_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.presentation.macroenabled",
    conforms_to: r#"["org.openxmlformats.openxml", "public.presentation", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["pptm"], "public.mime-type": ["application/vnd.ms-powerpoint.presentation.macroEnabled.12"]}"#,
    description: "Office Open XML presentation (macros enabled)",
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_SLIDESHOW: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.slideshow",
    conforms_to: r#"["org.openxmlformats.openxml", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["ppsx"], "public.mime-type": ["application/vnd.openxmlformats-officedocument.presentationml.slideshow"]}"#,
    description: "Office Open XML slide show",
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_SLIDESHOW_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.slideshow.macroenabled",
    conforms_to: r#"["org.openxmlformats.openxml", "public.presentation", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["ppsm"], "public.mime-type": ["application/vnd.ms-powerpoint.slideshow.macroEnabled.12"]}"#,
    description: "Office Open XML slide show (macros enabled)",
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_TEMPLATE: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.template",
    conforms_to: r#"["org.openxmlformats.openxml", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["potx"], "public.mime-type": ["application/vnd.openxmlformats-officedocument.presentationml.template"]}"#,
    description: "Office Open XML presentation template",
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_TEMPLATE_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.template.macroenabled",
    conforms_to: r#"["org.openxmlformats.openxml", "public.presentation", "public.executable"]"#,
    tags: r#"{"public.filename-extension": ["potm"], "public.mime-type": ["application/vnd.ms-powerpoint.template.macroEnabled.12"]}"#,
    description: "Office Open XML presentation template (macros enabled)",
};
pub const COM_MICROSOFT_WORD_DOC: UTType = UTType {
    identifier: "com.microsoft.word.doc",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{}"#,
    description: "Microsoft Word 97-2004 document",
};
pub const COM_MICROSOFT_WORD_DOT: UTType = UTType {
    identifier: "com.microsoft.word.dot",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{}"#,
    description: "Microsoft Word 97-2004 template",
};
pub const COM_MICROSOFT_EXCEL_XLS: UTType = UTType {
    identifier: "com.microsoft.excel.xls",
    conforms_to: r#"["public.data", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{}"#,
    description: "Microsoft Excel 97-2004 worksheet",
};
pub const COM_MICROSOFT_EXCEL_XLT: UTType = UTType {
    identifier: "com.microsoft.excel.xlt",
    conforms_to: r#"["public.data", "public.spreadsheet"]"#,
    tags: r#"{}"#,
    description: "Microsoft Excel 97-2004 template",
};
pub const COM_MICROSOFT_EXCEL_XLW: UTType = UTType {
    identifier: "com.microsoft.excel.xlw",
    conforms_to: r#"["public.data", "public.spreadsheet"]"#,
    tags: r#"{}"#,
    description: "Microsoft Excel 97-2004 workspace",
};
pub const COM_MICROSOFT_EXCEL_XLA: UTType = UTType {
    identifier: "com.microsoft.excel.xla",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["xla"]}"#,
    description: "Microsoft Excel add-in",
};
pub const COM_MICROSOFT_POWERPOINT_PPT: UTType = UTType {
    identifier: "com.microsoft.powerpoint.ppt",
    conforms_to: r#"["public.data", "public.presentation"]"#,
    tags: r#"{}"#,
    description: "Microsoft PowerPoint 97-2004 presentation",
};
pub const COM_MICROSOFT_POWERPOINT_POT: UTType = UTType {
    identifier: "com.microsoft.powerpoint.pot",
    conforms_to: r#"["public.data", "public.presentation"]"#,
    tags: r#"{}"#,
    description: "Microsoft PowerPoint 97-2004 template",
};
pub const COM_MICROSOFT_POWERPOINT_PPS: UTType = UTType {
    identifier: "com.microsoft.powerpoint.pps",
    conforms_to: r#"["public.data", "public.presentation"]"#,
    tags: r#"{}"#,
    description: "Microsoft PowerPoint 97-2004 slide show",
};
pub const COM_APPLE_KEYNOTE_KEY: UTType = UTType {
    identifier: "com.apple.keynote.key",
    conforms_to: r#"["com.apple.package", "public.presentation"]"#,
    tags: r#"{}"#,
    description: "Keynote document",
};
pub const COM_APPLE_KEYNOTE_KTH: UTType = UTType {
    identifier: "com.apple.keynote.kth",
    conforms_to: r#"["com.apple.package", "public.presentation"]"#,
    tags: r#"{}"#,
    description: "Keynote theme",
};
pub const COM_APPLE_IWORK_KEYNOTE_KEY: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.key",
    conforms_to: r#"["com.apple.package", "public.presentation", "com.apple.keynote.key"]"#,
    tags: r#"{"public.filename-extension": ["key"]}"#,
    description: "Keynote document",
};
pub const COM_APPLE_IWORK_KEYNOTE_KEY_TEF: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.key-tef",
    conforms_to: r#"["com.apple.package", "public.presentation", "com.apple.keynote.key"]"#,
    tags: r#"{"public.filename-extension": ["key-tef"]}"#,
    description: "Keynote document",
};
pub const COM_APPLE_IWORK_KEYNOTE_SFFKEY: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.sffkey",
    conforms_to: r#"["public.data", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["key"], "public.mime-type": ["application/x-iwork-keynote-sffkey"]}"#,
    description: "Keynote document",
};
pub const COM_APPLE_IWORK_KEYNOTE_KTH: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.kth",
    conforms_to: r#"["com.apple.package", "public.presentation", "com.apple.keynote.kth"]"#,
    tags: r#"{"public.filename-extension": ["kth"]}"#,
    description: "Keynote theme",
};
pub const COM_APPLE_IWORK_KEYNOTE_SFFKTH: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.sffkth",
    conforms_to: r#"["public.data", "public.presentation"]"#,
    tags: r#"{"public.filename-extension": ["kth"], "public.mime-type": ["application/x-iwork-keynote-sffkth"]}"#,
    description: "Keynote theme",
};
pub const COM_APPLE_IWORK_PAGES_PAGES: UTType = UTType {
    identifier: "com.apple.iWork.Pages.pages",
    conforms_to: r#"["com.apple.package", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["pages"]}"#,
    description: "Pages document",
};
pub const COM_APPLE_IWORK_PAGES_PAGES_TEF: UTType = UTType {
    identifier: "com.apple.iWork.Pages.pages-tef",
    conforms_to: r#"["com.apple.package", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["pages-tef"]}"#,
    description: "Pages document",
};
pub const COM_APPLE_IWORK_PAGES_SFFPAGES: UTType = UTType {
    identifier: "com.apple.iWork.Pages.sffpages",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["pages"], "public.mime-type": ["application/x-iwork-pages-sffpages"]}"#,
    description: "Pages document",
};
pub const COM_APPLE_IWORK_PAGES_TEMPLATE: UTType = UTType {
    identifier: "com.apple.iWork.Pages.template",
    conforms_to: r#"["com.apple.package", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["template"]}"#,
    description: "Pages template",
};
pub const COM_APPLE_IWORK_PAGES_SFFTEMPLATE: UTType = UTType {
    identifier: "com.apple.iWork.Pages.sfftemplate",
    conforms_to: r#"["public.data", "public.composite-content"]"#,
    tags: r#"{"public.filename-extension": ["template"], "public.mime-type": ["application/x-iwork-pages-sfftemplate"]}"#,
    description: "Pages template",
};
pub const COM_APPLE_IWORK_NUMBERS_NUMBERS: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.numbers",
    conforms_to: r#"["com.apple.package", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["numbers"]}"#,
    description: "Numbers document",
};
pub const COM_APPLE_IWORK_NUMBERS_NUMBERS_TEF: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.numbers-tef",
    conforms_to: r#"["com.apple.package", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["numbers-tef"]}"#,
    description: "Numbers document",
};
pub const COM_APPLE_IWORK_NUMBERS_SFFNUMBERS: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.sffnumbers",
    conforms_to: r#"["public.data", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["numbers"], "public.mime-type": ["application/x-iwork-numbers-sffnumbers"]}"#,
    description: "Numbers document",
};
pub const COM_APPLE_IWORK_NUMBERS_TEMPLATE: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.template",
    conforms_to: r#"["com.apple.package", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["nmbtemplate"]}"#,
    description: "Numbers template",
};
pub const COM_APPLE_IWORK_NUMBERS_SFFTEMPLATE: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.sfftemplate",
    conforms_to: r#"["public.data", "public.composite-content", "public.spreadsheet"]"#,
    tags: r#"{"public.filename-extension": ["nmbtemplate"], "public.mime-type": ["application/x-iwork-numbers-sfftemplate"]}"#,
    description: "Numbers template",
};
pub const COM_APPLE_GARAGEBAND_PROJECT: UTType = UTType {
    identifier: "com.apple.garageband.project",
    conforms_to: r#"["com.apple.package", "public.audiovisual-content"]"#,
    tags: r#"{"public.filename-extension": ["band", "gbProj"]}"#,
    description: "GarageBand Project",
};
pub const COM_ADOBE_PHOTOSHOP_IMAGE: UTType = UTType {
    identifier: "com.adobe.photoshop-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "Adobe Photoshop document",
};
pub const COM_ADOBE_PHOTOSHOP_LARGE_IMAGE: UTType = UTType {
    identifier: "com.adobe.photoshop-large-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "Adobe Photoshop large document",
};
pub const COM_ADOBE_ILLUSTRATOR_AI_IMAGE: UTType = UTType {
    identifier: "com.adobe.illustrator.ai-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "Adobe Illustrator document",
};
pub const COM_TRUEVISION_TGA_IMAGE: UTType = UTType {
    identifier: "com.truevision.tga-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "TGA image",
};
pub const COM_SGI_SGI_IMAGE: UTType = UTType {
    identifier: "com.sgi.sgi-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["sgi"], "public.mime-type": ["image/sgi"]}"#,
    description: "Silicon Graphics image",
};
pub const COM_ILM_OPENEXR_IMAGE: UTType = UTType {
    identifier: "com.ilm.openexr-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["exr"]}"#,
    description: "OpenEXR image",
};
pub const COM_KODAK_FLASHPIX_IMAGE: UTType = UTType {
    identifier: "com.kodak.flashpix-image",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{"public.filename-extension": ["fpx"], "public.mime-type": ["image/fpx", "application/vnd.fpx"]}"#,
    description: "FlashPix image",
};
pub const PUBLIC_HEIF_STANDARD: UTType = UTType {
    identifier: "public.heif-standard",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "HEIF Image",
};
pub const PUBLIC_HEIF: UTType = UTType {
    identifier: "public.heif",
    conforms_to: r#"["public.heif-standard"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const PUBLIC_HEIC: UTType = UTType {
    identifier: "public.heic",
    conforms_to: r#"["public.heif-standard"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const PUBLIC_AVCI: UTType = UTType {
    identifier: "public.avci",
    conforms_to: r#"["public.heif-standard"]"#,
    tags: r#"{}"#,
    description: "",
};
pub const PUBLIC_HEIFS: UTType = UTType {
    identifier: "public.heifs",
    conforms_to: r#"["public.heif-standard"]"#,
    tags: r#"{}"#,
    description: "HEIF Image Sequence",
};
pub const PUBLIC_HEICS: UTType = UTType {
    identifier: "public.heics",
    conforms_to: r#"["public.heif-standard"]"#,
    tags: r#"{}"#,
    description: "HEIF Image Sequence",
};
pub const PUBLIC_AVCS: UTType = UTType {
    identifier: "public.avcs",
    conforms_to: r#"["public.heif-standard"]"#,
    tags: r#"{}"#,
    description: "HEIF Image Sequence",
};
pub const COM_APPLE_DRAWING: UTType = UTType {
    identifier: "com.apple.drawing",
    conforms_to: r#"["public.image"]"#,
    tags: r#"{}"#,
    description: "Apple Drawing Format",
};
pub const COM_J2_JFX_FAX: UTType = UTType {
    identifier: "com.j2.jfx-fax",
    conforms_to: r#"["public.fax"]"#,
    tags: r#"{"public.filename-extension": ["jfx"]}"#,
    description: "J2 fax",
};
pub const COM_J2_EFX_FAX: UTType = UTType {
    identifier: "com.j2.efx-fax",
    conforms_to: r#"["public.fax"]"#,
    tags: r#"{"public.filename-extension": ["efx"], "public.mime-type": ["image/efax"]}"#,
    description: "eFax fax",
};
pub const COM_DIGIDESIGN_SD2_AUDIO: UTType = UTType {
    identifier: "com.digidesign.sd2-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{}"#,
    description: "Sound Designer II audio",
};
pub const COM_MICROSOFT_WAVEFORM_AUDIO: UTType = UTType {
    identifier: "com.microsoft.waveform-audio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["wav", "wave", "bwf"], "public.mime-type": ["audio/vnd.wave", "audio/wav", "audio/wave", "audio/x-wav"]}"#,
    description: "Waveform audio",
};
pub const COM_MICROSOFT_ADVANCED_SYSTEMS_FORMAT: UTType = UTType {
    identifier: "com.microsoft.advanced-systems-format",
    conforms_to: r#"["public.audiovisual-content"]"#,
    tags: r#"{}"#,
    description: "Microsoft Advanced Systems Format",
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WM: UTType = UTType {
    identifier: "com.microsoft.windows-media-wm",
    conforms_to: r#"["public.movie", "com.microsoft.advanced-systems-format"]"#,
    tags: r#"{}"#,
    description: "Windows media",
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WMV: UTType = UTType {
    identifier: "com.microsoft.windows-media-wmv",
    conforms_to: r#"["public.movie", "com.microsoft.advanced-systems-format"]"#,
    tags: r#"{}"#,
    description: "Windows media",
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WMP: UTType = UTType {
    identifier: "com.microsoft.windows-media-wmp",
    conforms_to: r#"["public.movie", "com.microsoft.advanced-systems-format"]"#,
    tags: r#"{}"#,
    description: "Windows media",
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WMA: UTType = UTType {
    identifier: "com.microsoft.windows-media-wma",
    conforms_to: r#"["public.audio", "com.microsoft.advanced-systems-format"]"#,
    tags: r#"{}"#,
    description: "Windows media audio",
};
pub const COM_MICROSOFT_ADVANCED_STREAM_REDIRECTOR: UTType = UTType {
    identifier: "com.microsoft.advanced-stream-redirector",
    conforms_to: r#"["public.audiovisual-content", "public.xml"]"#,
    tags: r#"{}"#,
    description: "Advanced Stream Redirector",
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WMX: UTType = UTType {
    identifier: "com.microsoft.windows-media-wmx",
    conforms_to: r#"["public.movie", "com.microsoft.advanced-stream-redirector"]"#,
    tags: r#"{}"#,
    description: "Windows media",
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WVX: UTType = UTType {
    identifier: "com.microsoft.windows-media-wvx",
    conforms_to: r#"["public.movie", "com.microsoft.advanced-stream-redirector"]"#,
    tags: r#"{}"#,
    description: "Windows media",
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WAX: UTType = UTType {
    identifier: "com.microsoft.windows-media-wax",
    conforms_to: r#"["public.audio", "com.microsoft.advanced-stream-redirector"]"#,
    tags: r#"{}"#,
    description: "Windows media audio",
};
pub const COM_REAL_REALMEDIA: UTType = UTType {
    identifier: "com.real.realmedia",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{}"#,
    description: "RealMedia",
};
pub const COM_REAL_REALMEDIA_VBR: UTType = UTType {
    identifier: "com.real.realmedia-vbr",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{}"#,
    description: "RealMedia Variable Bitrate",
};
pub const ORG_SMPTE_MXF: UTType = UTType {
    identifier: "org.smpte.mxf",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{}"#,
    description: "Material eXchange Format",
};
pub const COM_REAL_REALAUDIO: UTType = UTType {
    identifier: "com.real.realaudio",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{}"#,
    description: "RealMedia Audio",
};
pub const COM_SOUNDBLASTER_SOUNDFONT: UTType = UTType {
    identifier: "com.soundblaster.soundfont",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["sf2"]}"#,
    description: "SoundFont audio",
};
pub const ORG_XIPH_FLAC: UTType = UTType {
    identifier: "org.xiph.flac",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{}"#,
    description: "FLAC audio",
};
pub const COM_AVID_OPEN_MEDIA_FRAMEWORK: UTType = UTType {
    identifier: "com.avid.open-media-framework",
    conforms_to: r#"["public.audiovisual-content"]"#,
    tags: r#"{"public.filename-extension": ["omf"]}"#,
    description: "Open Media Framework interchange format",
};
pub const PUBLIC_MP4A_LOAS: UTType = UTType {
    identifier: "public.mp4a-loas",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{"public.filename-extension": ["loas", "latm"]}"#,
    description: "Low Overhead MPEG-4 Audio Stream",
};
pub const PUBLIC_MP4A_LATM: UTType = UTType {
    identifier: "public.mp4a-latm",
    conforms_to: r#"["public.audio"]"#,
    tags: r#"{}"#,
    description: "Low-overhead MPEG-4 Audio Transport Multiplex",
};
pub const COM_ALLUME_STUFFIT_ARCHIVE: UTType = UTType {
    identifier: "com.allume.stuffit-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{}"#,
    description: "StuffIt archive family",
};
pub const COM_STUFFIT_ARCHIVE_SITX: UTType = UTType {
    identifier: "com.stuffit.archive.sitx",
    conforms_to: r#"["com.allume.stuffit-archive"]"#,
    tags: r#"{"public.filename-extension": ["sitx"], "public.mime-type": ["application/x-stuffitx", "application/x-sitx"]}"#,
    description: "StuffIt X archive",
};
pub const COM_STUFFIT_ARCHIVE_SIDX: UTType = UTType {
    identifier: "com.stuffit.archive.sidx",
    conforms_to: r#"["com.allume.stuffit-archive"]"#,
    tags: r#"{"public.filename-extension": ["sidx"], "public.mime-type": ["application/x-stuffitx-index", "application/x-sitx-index"]}"#,
    description: "StuffIt X index",
};
pub const COM_STUFFIT_ARCHIVE_SIT: UTType = UTType {
    identifier: "com.stuffit.archive.sit",
    conforms_to: r#"["com.allume.stuffit-archive"]"#,
    tags: r#"{"public.filename-extension": ["sit", "sea"], "public.mime-type": ["application/x-stuffit", "application/x-sit"]}"#,
    description: "StuffIt archive",
};
pub const COM_ADOBE_FLASH_VIDEO: UTType = UTType {
    identifier: "com.adobe.flash.video",
    conforms_to: r#"["public.movie"]"#,
    tags: r#"{"public.filename-extension": ["flv", "f4v", "f4p", "f4a", "f4b"], "public.mime-type": ["video/x-flv"]}"#,
    description: "Flash video",
};
pub const ORG_KHRONOS_COLLADA_DIGITAL_ASSET_EXCHANGE: UTType = UTType {
    identifier: "org.khronos.collada.digital-asset-exchange",
    conforms_to: r#"["public.xml", "public.audiovisual-content", "public.3d-content"]"#,
    tags: r#"{"public.filename-extension": ["dae"]}"#,
    description: "Digital Asset Exchange (DAE)",
};
pub const COM_APPLE_IMOVIELIBRARY: UTType = UTType {
    identifier: "com.apple.iMovieLibrary",
    conforms_to: r#"["com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["imovielibrary"]}"#,
    description: "iMovie Library",
};
pub const COM_APPLE_IMOVIETHEATER: UTType = UTType {
    identifier: "com.apple.iMovieTheater",
    conforms_to: r#"["com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["theater"]}"#,
    description: "iMovie Theater",
};
pub const ORG_7_ZIP_7_ZIP_ARCHIVE: UTType = UTType {
    identifier: "org.7-zip.7-zip-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{}"#,
    description: "7-Zip archive",
};
pub const ORG_TUKAANI_XZ_ARCHIVE: UTType = UTType {
    identifier: "org.tukaani.xz-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{}"#,
    description: "xz compressed archive",
};
pub const ORG_TUKAANI_TAR_XZ_ARCHIVE: UTType = UTType {
    identifier: "org.tukaani.tar-xz-archive",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{}"#,
    description: "xz compressed tar archive",
};
pub const COM_MICROSOFT_CAB: UTType = UTType {
    identifier: "com.microsoft.cab",
    conforms_to: r#"["public.data", "public.archive"]"#,
    tags: r#"{}"#,
    description: "Microsoft Cabinet archive",
};
pub const PUBLIC_HAPTICS_CONTENT: UTType = UTType {
    identifier: "public.haptics-content",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Haptics content",
};
pub const COM_APPLE_HAPTICS_AHAP: UTType = UTType {
    identifier: "com.apple.haptics.ahap",
    conforms_to: r#"["public.haptics-content", "public.json"]"#,
    tags: r#"{}"#,
    description: "Apple Haptics Audio Pattern",
};
pub const COM_APPLE_COREML_MODEL: UTType = UTType {
    identifier: "com.apple.coreml.model",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["mlmodel", "mlkitmodel"]}"#,
    description: "Core ML Machine Learning Model",
};
pub const COM_APPLE_COREML_MLPACKAGE: UTType = UTType {
    identifier: "com.apple.coreml.mlpackage",
    conforms_to: r#"["com.apple.package"]"#,
    tags: r#"{"public.filename-extension": ["mlpackage"]}"#,
    description: "Core ML Machine Learning Model Package",
};
pub const COM_APPLE_GROUPACTIVITIES_ACTIVITY: UTType = UTType {
    identifier: "com.apple.groupactivities.activity",
    conforms_to: r#"["public.data"]"#,
    tags: r#"{"public.filename-extension": ["groupactivity"]}"#,
    description: "Group Activity",
};
pub const COM_APPLE_ICON_DECORATION: UTType = UTType {
    identifier: "com.apple.icon-decoration",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Icon Decoration",
};
pub const COM_APPLE_ICON_DECORATION_POSITION: UTType = UTType {
    identifier: "com.apple.icon-decoration-position",
    conforms_to: r#"[]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Position",
};
pub const COM_APPLE_ICON_DECORATION_POSITION_CENTER: UTType = UTType {
    identifier: "com.apple.icon-decoration-position.center",
    conforms_to: r#"["com.apple.icon-decoration-position"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Position Center",
};
pub const COM_APPLE_ICON_DECORATION_POSITION_LEADING_BOTTOM: UTType = UTType {
    identifier: "com.apple.icon-decoration-position.leading-bottom",
    conforms_to: r#"["com.apple.icon-decoration-position"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Position Leading Bottom",
};
pub const COM_APPLE_ICON_DECORATION_POSITION_TRAILING_BOTTOM: UTType = UTType {
    identifier: "com.apple.icon-decoration-position.trailing-bottom",
    conforms_to: r#"["com.apple.icon-decoration-position"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Position Trailing Bottom",
};
pub const COM_APPLE_ICON_DECORATION_POSITION_OVERLAY: UTType = UTType {
    identifier: "com.apple.icon-decoration-position.overlay",
    conforms_to: r#"["com.apple.icon-decoration-position"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Position Overlay",
};
pub const COM_APPLE_ICON_DECORATION_BADGE: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Badge",
};
pub const COM_APPLE_ICON_DECORATION_EMBOSS: UTType = UTType {
    identifier: "com.apple.icon-decoration.emboss",
    conforms_to: r#"["com.apple.icon-decoration-position.center", "com.apple.icon-decoration"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Emboss",
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM: UTType = UTType {
    identifier: "com.apple.icon-decoration.system",
    conforms_to: r#"["com.apple.icon-decoration"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration System",
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM_UNSUPPORTED: UTType = UTType {
    identifier: "com.apple.icon-decoration.system.unsupported",
    conforms_to: r#"["com.apple.icon-decoration.system"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Unsupported",
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM_CAUTION_ALERT: UTType = UTType {
    identifier: "com.apple.icon-decoration.system.caution-alert",
    conforms_to: r#"["com.apple.icon-decoration.system"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Alert Caution",
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM_ALIAS: UTType = UTType {
    identifier: "com.apple.icon-decoration.system.alias",
    conforms_to: r#"["com.apple.icon-decoration.system"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Alias",
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM_NEW_FOLDER: UTType = UTType {
    identifier: "com.apple.icon-decoration.system.new-folder",
    conforms_to: r#"["com.apple.icon-decoration.system"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration New Folder",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_LOCKED: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.locked",
    conforms_to: r#"["com.apple.icon-decoration-position.leading-bottom", "com.apple.icon-decoration.system"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Locked Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_CHECKMARK: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.checkmark",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Checkmark Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_COMMENTS: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.comments",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Comments Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_DROP_FOLDER: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.drop-folder",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Drop Folder Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_HEART: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.heart",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Heart Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_IN_REVIEW: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.in-review",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration In Review Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_LOCKED_BY_COLLABORATOR: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.locked-by-collaborator",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Locked By Collaborator Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_LOCKED_BY_USER: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.locked-by-user",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Locked By User Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_PINNED: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.pinned",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Pinned Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_PRIVATE_FOLDER: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.private-folder",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Private Folder Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_SYNCING: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.syncing",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Syncing Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_TRENDING: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.trending",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Trending Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_WARNING: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.warning",
    conforms_to: r#"["com.apple.icon-decoration-position.trailing-bottom", "com.apple.icon-decoration.badge"]"#,
    tags: r#"{}"#,
    description: "Icon Decoration Warning Badge",
};
pub const COM_APPLE_DOCUMENT_TYPE: UTType = UTType {
    identifier: "com.apple.document-type",
    conforms_to: r#"["public.item"]"#,
    tags: r#"{}"#,
    description: "Document Type",
};
pub const COM_APPLE_DOCUMENT_TYPE_DICTIONARY: UTType = UTType {
    identifier: "com.apple.document-type.dictionary",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Dictionary",
};
pub const COM_APPLE_ACCOUNTS_ICON: UTType = UTType {
    identifier: "com.apple.accounts-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Accounts Icon",
};
pub const COM_APPLE_LEGACY_ACTIONS_ICON: UTType = UTType {
    identifier: "com.apple.legacy.actions-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Actions Icon",
};
pub const COM_APPLE_EVERYONE_ICON: UTType = UTType {
    identifier: "com.apple.everyone-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Everyone Icon",
};
pub const COM_APPLE_LEGACY_GENERAL_ICON: UTType = UTType {
    identifier: "com.apple.legacy.general-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "General Icon",
};
pub const COM_APPLE_LEGACY_SIDEBAR_PREFS_ICON: UTType = UTType {
    identifier: "com.apple.legacy.sidebar-prefs-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Sidebar Prefs Icon",
};
pub const COM_APPLE_LEGACY_TOOLBAR_ADVANCED_ICON: UTType = UTType {
    identifier: "com.apple.legacy.toolbar-advanced-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Toolbar Advanced Icon",
};
pub const COM_APPLE_LEGACY_TOOLBAR_INFO_ICON: UTType = UTType {
    identifier: "com.apple.legacy.toolbar-info-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Toolbar Info Icon",
};
pub const COM_APPLE_LEGACY_TOOLBAR_LABELS_ICON: UTType = UTType {
    identifier: "com.apple.legacy.toolbar-labels-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Toolbar Labels Icon",
};
pub const COM_APPLE_LEGACY_CLOCK_ICON: UTType = UTType {
    identifier: "com.apple.legacy.clock-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Clock Icon",
};
pub const COM_APPLE_LEGACY_SYNCHRONIZE: UTType = UTType {
    identifier: "com.apple.legacy.synchronize",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Synchronize",
};
pub const COM_APPLE_ICON_OVERLAY_NEW_FOLDER_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.new-folder-badge",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "New Folder Badge",
};
pub const COM_APPLE_LEGACY_FINDER_ICON: UTType = UTType {
    identifier: "com.apple.legacy.finder-icon",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Finder",
};
pub const COM_APPLE_UNKNOWN_OBJECT: UTType = UTType {
    identifier: "com.apple.unknown-object",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Unknown Object",
};
pub const COM_APPLE_NOT_LOADED: UTType = UTType {
    identifier: "com.apple.not-loaded",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Not Loaded",
};
pub const COM_APPLE_LEGACY_WINDOW: UTType = UTType {
    identifier: "com.apple.legacy.window",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Window",
};
pub const COM_APPLE_LEGACY_QUESTION_MARK: UTType = UTType {
    identifier: "com.apple.legacy.question-mark",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Question Mark",
};
pub const COM_APPLE_LEGACY_EJECT_MEDIA: UTType = UTType {
    identifier: "com.apple.legacy.eject-media",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Eject Media",
};
pub const COM_APPLE_LEGACY_BURN: UTType = UTType {
    identifier: "com.apple.legacy.burn",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Burn",
};
pub const COM_APPLE_LEGACY_CUSTOMIZE_TOOLBAR: UTType = UTType {
    identifier: "com.apple.legacy.customize-toolbar",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Customize Toolbar",
};
pub const COM_APPLE_LEGACY_DELETE_TOOLBAR: UTType = UTType {
    identifier: "com.apple.legacy.delete-toolbar",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Delete Toolbar",
};
pub const COM_APPLE_LEGACY_RIGHT_CONTAINER_ARROW: UTType = UTType {
    identifier: "com.apple.legacy.right-container-arrow",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Right Container Arrow",
};
pub const COM_APPLE_ICON_OVERLAY_DROP_FOLDER_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.drop-folder-badge",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Drop Folder Badge",
};
pub const COM_APPLE_ICON_OVERLAY_PRIVATE_FOLDER_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.private-folder-badge",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Private Folder Badge",
};
pub const COM_APPLE_ICON_OVERLAY_PRIVATE_FOLDER: UTType = UTType {
    identifier: "com.apple.icon-overlay.private-folder",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Private Folder",
};
pub const COM_APPLE_LEGACY_OPEN_FOLDER: UTType = UTType {
    identifier: "com.apple.legacy.open-folder",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Open Folder",
};
pub const COM_APPLE_LEGACY_FAVORITE_ITEMS: UTType = UTType {
    identifier: "com.apple.legacy.favorite-items",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Favorite Items",
};
pub const COM_APPLE_LEGACY_LOCKED: UTType = UTType {
    identifier: "com.apple.legacy.locked",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Locked",
};
pub const COM_APPLE_LEGACY_UNLOCKED: UTType = UTType {
    identifier: "com.apple.legacy.unlocked",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Unlocked",
};
pub const COM_APPLE_LEGACY_NO_WRITE: UTType = UTType {
    identifier: "com.apple.legacy.no-write",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "No Write",
};
pub const COM_APPLE_LEGACY_KEEP_ARRANGED: UTType = UTType {
    identifier: "com.apple.legacy.keep-arranged",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Keep Arranged",
};
pub const COM_APPLE_LEGACY_GRID: UTType = UTType {
    identifier: "com.apple.legacy.grid",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Grid",
};
pub const COM_APPLE_LEGACY_CONNECT_TO: UTType = UTType {
    identifier: "com.apple.legacy.connect-to",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Connect To",
};
pub const COM_APPLE_LEGACY_BACKWARD_ARROW: UTType = UTType {
    identifier: "com.apple.legacy.backward-arrow",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Backward Arrow",
};
pub const COM_APPLE_LEGACY_FORWARD_ARROW: UTType = UTType {
    identifier: "com.apple.legacy.forward-arrow",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Forward Arrow",
};
pub const COM_APPLE_ICON_OVERLAY_LOCKED_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.locked-badge",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Locked Badge",
};
pub const COM_APPLE_ICON_OVERLAY_ALIAS_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.alias-badge",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Alias Badge",
};
pub const COM_APPLE_ICON_OVERLAY_ALERT_CAUTION_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.alert-caution-badge",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Alert Caution Badge",
};
pub const COM_APPLE_ICON_OVERLAY_UNSUPPORTED_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.unsupported-badge",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Unsupported Badge",
};
pub const COM_APPLE_LEGACY_MAGNIFYING_GLASS: UTType = UTType {
    identifier: "com.apple.legacy.magnifying-glass",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Magnifying Glass",
};
pub const COM_APPLE_LEGACY_ERASING: UTType = UTType {
    identifier: "com.apple.legacy.erasing",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Erasing",
};
pub const COM_APPLE_LEGACY_MULTIPLE_ITEMS: UTType = UTType {
    identifier: "com.apple.legacy.multiple-items",
    conforms_to: r#"["com.apple.document-type"]"#,
    tags: r#"{}"#,
    description: "Multiple Items",
};
pub const OTHER_TYPES: [UTType; 206] = [
    COM_ADOBE_PDF,
    COM_ADOBE_EDN,
    COM_ADOBE_ETD,
    COM_ADOBE_XFDF,
    COM_ADOBE_FDF,
    COM_ADOBE_POSTSCRIPT,
    COM_ADOBE_ENCAPSULATED_POSTSCRIPT,
    COM_COMPUSERVE_GIF,
    COM_MICROSOFT_BMP,
    COM_MICROSOFT_ICO,
    ORG_WEBMPROJECT_WEBP,
    ORG_WEBMPROJECT_WEBM,
    PUBLIC_OFD,
    ORG_OPENOFFICE_TEXT,
    ORG_OPENOFFICE_TEXT_TEMPLATE,
    ORG_OASIS_OPEN_OPENDOCUMENT_TEXT,
    ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_TEMPLATE,
    ORG_OPENOFFICE_GRAPHICS,
    ORG_OPENOFFICE_GRAPHICS_TEMPLATE,
    ORG_OASIS_OPEN_OPENDOCUMENT_GRAPHICS,
    ORG_OASIS_OPEN_OPENDOCUMENT_GRAPHICS_TEMPLATE,
    ORG_OPENOFFICE_PRESENTATION,
    ORG_OPENOFFICE_PRESENTATION_TEMPLATE,
    ORG_OASIS_OPEN_OPENDOCUMENT_PRESENTATION,
    ORG_OASIS_OPEN_OPENDOCUMENT_PRESENTATION_TEMPLATE,
    ORG_OPENOFFICE_SPREADSHEET,
    ORG_OPENOFFICE_SPREADSHEET_TEMPLATE,
    ORG_OASIS_OPEN_OPENDOCUMENT_SPREADSHEET,
    ORG_OASIS_OPEN_OPENDOCUMENT_SPREADSHEET_TEMPLATE,
    ORG_OASIS_OPEN_OPENDOCUMENT_CHART,
    ORG_OASIS_OPEN_OPENDOCUMENT_CHART_TEMPLATE,
    ORG_OASIS_OPEN_OPENDOCUMENT_IMAGE,
    ORG_OASIS_OPEN_OPENDOCUMENT_IMAGE_TEMPLATE,
    ORG_OPENOFFICE_FORMULA,
    ORG_OASIS_OPEN_OPENDOCUMENT_FORMULA,
    ORG_OASIS_OPEN_OPENDOCUMENT_FORMULA_TEMPLATE,
    ORG_OPENOFFICE_TEXT_MASTER,
    ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_MASTER,
    ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_WEB,
    ORG_OASIS_OPEN_OPENDOCUMENT_DATABASE,
    COM_MICROSOFT_WORD_WORDML,
    ORG_OPENXMLFORMATS_WORDPROCESSINGML_DOCUMENT,
    ORG_OPENXMLFORMATS_WORDPROCESSINGML_DOCUMENT_MACROENABLED,
    ORG_OPENXMLFORMATS_WORDPROCESSINGML_TEMPLATE,
    ORG_OPENXMLFORMATS_WORDPROCESSINGML_TEMPLATE_MACROENABLED,
    ORG_OPENXMLFORMATS_SPREADSHEETML_SHEET,
    ORG_OPENXMLFORMATS_SPREADSHEETML_SHEET_MACROENABLED,
    COM_MICROSOFT_EXCEL_SHEET_BINARY_MACROENABLED,
    ORG_OPENXMLFORMATS_SPREADSHEETML_TEMPLATE,
    ORG_OPENXMLFORMATS_SPREADSHEETML_TEMPLATE_MACROENABLED,
    ORG_OPENXMLFORMATS_PRESENTATIONML_PRESENTATION,
    ORG_OPENXMLFORMATS_PRESENTATIONML_PRESENTATION_MACROENABLED,
    ORG_OPENXMLFORMATS_PRESENTATIONML_SLIDESHOW,
    ORG_OPENXMLFORMATS_PRESENTATIONML_SLIDESHOW_MACROENABLED,
    ORG_OPENXMLFORMATS_PRESENTATIONML_TEMPLATE,
    ORG_OPENXMLFORMATS_PRESENTATIONML_TEMPLATE_MACROENABLED,
    COM_MICROSOFT_WORD_DOC,
    COM_MICROSOFT_WORD_DOT,
    COM_MICROSOFT_EXCEL_XLS,
    COM_MICROSOFT_EXCEL_XLT,
    COM_MICROSOFT_EXCEL_XLW,
    COM_MICROSOFT_EXCEL_XLA,
    COM_MICROSOFT_POWERPOINT_PPT,
    COM_MICROSOFT_POWERPOINT_POT,
    COM_MICROSOFT_POWERPOINT_PPS,
    COM_APPLE_KEYNOTE_KEY,
    COM_APPLE_KEYNOTE_KTH,
    COM_APPLE_IWORK_KEYNOTE_KEY,
    COM_APPLE_IWORK_KEYNOTE_KEY_TEF,
    COM_APPLE_IWORK_KEYNOTE_SFFKEY,
    COM_APPLE_IWORK_KEYNOTE_KTH,
    COM_APPLE_IWORK_KEYNOTE_SFFKTH,
    COM_APPLE_IWORK_PAGES_PAGES,
    COM_APPLE_IWORK_PAGES_PAGES_TEF,
    COM_APPLE_IWORK_PAGES_SFFPAGES,
    COM_APPLE_IWORK_PAGES_TEMPLATE,
    COM_APPLE_IWORK_PAGES_SFFTEMPLATE,
    COM_APPLE_IWORK_NUMBERS_NUMBERS,
    COM_APPLE_IWORK_NUMBERS_NUMBERS_TEF,
    COM_APPLE_IWORK_NUMBERS_SFFNUMBERS,
    COM_APPLE_IWORK_NUMBERS_TEMPLATE,
    COM_APPLE_IWORK_NUMBERS_SFFTEMPLATE,
    COM_APPLE_GARAGEBAND_PROJECT,
    COM_ADOBE_PHOTOSHOP_IMAGE,
    COM_ADOBE_PHOTOSHOP_LARGE_IMAGE,
    COM_ADOBE_ILLUSTRATOR_AI_IMAGE,
    COM_TRUEVISION_TGA_IMAGE,
    COM_SGI_SGI_IMAGE,
    COM_ILM_OPENEXR_IMAGE,
    COM_KODAK_FLASHPIX_IMAGE,
    PUBLIC_HEIF_STANDARD,
    PUBLIC_HEIF,
    PUBLIC_HEIC,
    PUBLIC_AVCI,
    PUBLIC_HEIFS,
    PUBLIC_HEICS,
    PUBLIC_AVCS,
    COM_APPLE_DRAWING,
    COM_J2_JFX_FAX,
    COM_J2_EFX_FAX,
    COM_DIGIDESIGN_SD2_AUDIO,
    COM_MICROSOFT_WAVEFORM_AUDIO,
    COM_MICROSOFT_ADVANCED_SYSTEMS_FORMAT,
    COM_MICROSOFT_WINDOWS_MEDIA_WM,
    COM_MICROSOFT_WINDOWS_MEDIA_WMV,
    COM_MICROSOFT_WINDOWS_MEDIA_WMP,
    COM_MICROSOFT_WINDOWS_MEDIA_WMA,
    COM_MICROSOFT_ADVANCED_STREAM_REDIRECTOR,
    COM_MICROSOFT_WINDOWS_MEDIA_WMX,
    COM_MICROSOFT_WINDOWS_MEDIA_WVX,
    COM_MICROSOFT_WINDOWS_MEDIA_WAX,
    COM_REAL_REALMEDIA,
    COM_REAL_REALMEDIA_VBR,
    ORG_SMPTE_MXF,
    COM_REAL_REALAUDIO,
    COM_SOUNDBLASTER_SOUNDFONT,
    ORG_XIPH_FLAC,
    COM_AVID_OPEN_MEDIA_FRAMEWORK,
    PUBLIC_MP4A_LOAS,
    PUBLIC_MP4A_LATM,
    COM_ALLUME_STUFFIT_ARCHIVE,
    COM_STUFFIT_ARCHIVE_SITX,
    COM_STUFFIT_ARCHIVE_SIDX,
    COM_STUFFIT_ARCHIVE_SIT,
    COM_ADOBE_FLASH_VIDEO,
    ORG_KHRONOS_COLLADA_DIGITAL_ASSET_EXCHANGE,
    COM_APPLE_IMOVIELIBRARY,
    COM_APPLE_IMOVIETHEATER,
    ORG_7_ZIP_7_ZIP_ARCHIVE,
    ORG_TUKAANI_XZ_ARCHIVE,
    ORG_TUKAANI_TAR_XZ_ARCHIVE,
    COM_MICROSOFT_CAB,
    PUBLIC_HAPTICS_CONTENT,
    COM_APPLE_HAPTICS_AHAP,
    COM_APPLE_COREML_MODEL,
    COM_APPLE_COREML_MLPACKAGE,
    COM_APPLE_GROUPACTIVITIES_ACTIVITY,
    COM_APPLE_ICON_DECORATION,
    COM_APPLE_ICON_DECORATION_POSITION,
    COM_APPLE_ICON_DECORATION_POSITION_CENTER,
    COM_APPLE_ICON_DECORATION_POSITION_LEADING_BOTTOM,
    COM_APPLE_ICON_DECORATION_POSITION_TRAILING_BOTTOM,
    COM_APPLE_ICON_DECORATION_POSITION_OVERLAY,
    COM_APPLE_ICON_DECORATION_BADGE,
    COM_APPLE_ICON_DECORATION_EMBOSS,
    COM_APPLE_ICON_DECORATION_SYSTEM,
    COM_APPLE_ICON_DECORATION_SYSTEM_UNSUPPORTED,
    COM_APPLE_ICON_DECORATION_SYSTEM_CAUTION_ALERT,
    COM_APPLE_ICON_DECORATION_SYSTEM_ALIAS,
    COM_APPLE_ICON_DECORATION_SYSTEM_NEW_FOLDER,
    COM_APPLE_ICON_DECORATION_BADGE_LOCKED,
    COM_APPLE_ICON_DECORATION_BADGE_CHECKMARK,
    COM_APPLE_ICON_DECORATION_BADGE_COMMENTS,
    COM_APPLE_ICON_DECORATION_BADGE_DROP_FOLDER,
    COM_APPLE_ICON_DECORATION_BADGE_HEART,
    COM_APPLE_ICON_DECORATION_BADGE_IN_REVIEW,
    COM_APPLE_ICON_DECORATION_BADGE_LOCKED_BY_COLLABORATOR,
    COM_APPLE_ICON_DECORATION_BADGE_LOCKED_BY_USER,
    COM_APPLE_ICON_DECORATION_BADGE_PINNED,
    COM_APPLE_ICON_DECORATION_BADGE_PRIVATE_FOLDER,
    COM_APPLE_ICON_DECORATION_BADGE_SYNCING,
    COM_APPLE_ICON_DECORATION_BADGE_TRENDING,
    COM_APPLE_ICON_DECORATION_BADGE_WARNING,
    COM_APPLE_DOCUMENT_TYPE,
    COM_APPLE_DOCUMENT_TYPE_DICTIONARY,
    COM_APPLE_ACCOUNTS_ICON,
    COM_APPLE_LEGACY_ACTIONS_ICON,
    COM_APPLE_EVERYONE_ICON,
    COM_APPLE_LEGACY_GENERAL_ICON,
    COM_APPLE_LEGACY_SIDEBAR_PREFS_ICON,
    COM_APPLE_LEGACY_TOOLBAR_ADVANCED_ICON,
    COM_APPLE_LEGACY_TOOLBAR_INFO_ICON,
    COM_APPLE_LEGACY_TOOLBAR_LABELS_ICON,
    COM_APPLE_LEGACY_CLOCK_ICON,
    COM_APPLE_LEGACY_SYNCHRONIZE,
    COM_APPLE_ICON_OVERLAY_NEW_FOLDER_BADGE,
    COM_APPLE_LEGACY_FINDER_ICON,
    COM_APPLE_UNKNOWN_OBJECT,
    COM_APPLE_NOT_LOADED,
    COM_APPLE_LEGACY_WINDOW,
    COM_APPLE_LEGACY_QUESTION_MARK,
    COM_APPLE_LEGACY_EJECT_MEDIA,
    COM_APPLE_LEGACY_BURN,
    COM_APPLE_LEGACY_CUSTOMIZE_TOOLBAR,
    COM_APPLE_LEGACY_DELETE_TOOLBAR,
    COM_APPLE_LEGACY_RIGHT_CONTAINER_ARROW,
    COM_APPLE_ICON_OVERLAY_DROP_FOLDER_BADGE,
    COM_APPLE_ICON_OVERLAY_PRIVATE_FOLDER_BADGE,
    COM_APPLE_ICON_OVERLAY_PRIVATE_FOLDER,
    COM_APPLE_LEGACY_OPEN_FOLDER,
    COM_APPLE_LEGACY_FAVORITE_ITEMS,
    COM_APPLE_LEGACY_LOCKED,
    COM_APPLE_LEGACY_UNLOCKED,
    COM_APPLE_LEGACY_NO_WRITE,
    COM_APPLE_LEGACY_KEEP_ARRANGED,
    COM_APPLE_LEGACY_GRID,
    COM_APPLE_LEGACY_CONNECT_TO,
    COM_APPLE_LEGACY_BACKWARD_ARROW,
    COM_APPLE_LEGACY_FORWARD_ARROW,
    COM_APPLE_ICON_OVERLAY_LOCKED_BADGE,
    COM_APPLE_ICON_OVERLAY_ALIAS_BADGE,
    COM_APPLE_ICON_OVERLAY_ALERT_CAUTION_BADGE,
    COM_APPLE_ICON_OVERLAY_UNSUPPORTED_BADGE,
    COM_APPLE_LEGACY_MAGNIFYING_GLASS,
    COM_APPLE_LEGACY_ERASING,
    COM_APPLE_LEGACY_MULTIPLE_ITEMS,
];
pub const MIME_TYPE_TO_EXTENSION_VEC: [MIMETypeAndExtension; 89] = [
    MIMETypeAndExtension {
        mime_type: "application/x-koan",
        extensions: "skp|skd|skt|skm",
    },
    MIMETypeAndExtension {
        mime_type: "application/mac-compactpro",
        extensions: "cpt",
    },
    MIMETypeAndExtension {
        mime_type: "image/ief",
        extensions: "ief",
    },
    MIMETypeAndExtension {
        mime_type: "text/vnd.wap.wml",
        extensions: "wml",
    },
    MIMETypeAndExtension {
        mime_type: "video/x-sgi-movie",
        extensions: "movie",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-filemaker",
        extensions: "fp6|fp5|fp4|fp3|fp2|fp",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-wais-source",
        extensions: "src",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-portable-pixmap",
        extensions: "ppm",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-macpaint",
        extensions: "pnt|pntg|mac",
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
        mime_type: "application/x-ustar",
        extensions: "ustar",
    },
    MIMETypeAndExtension {
        mime_type: "text/html",
        extensions: "html|jhtml",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-troff-me",
        extensions: "me",
    },
    MIMETypeAndExtension {
        mime_type: "text/xml",
        extensions: "xml|xsl",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-tcl",
        extensions: "tcl",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-csh",
        extensions: "csh",
    },
    MIMETypeAndExtension {
        mime_type: "audio/aiff",
        extensions: "aiff",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-hdf",
        extensions: "hdf",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-shar",
        extensions: "shar",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.adobe.xfdf",
        extensions: "xfdf",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-ms-wmd",
        extensions: "wmd",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.mif",
        extensions: "mif",
    },
    MIMETypeAndExtension {
        mime_type: "text/vnd.wap.wmlscript",
        extensions: "wmls",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-troff-ms",
        extensions: "ms",
    },
    MIMETypeAndExtension {
        mime_type: "text/x-setext",
        extensions: "etx",
    },
    MIMETypeAndExtension {
        mime_type: "chemical/x-xyz",
        extensions: "xyz",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-sv4crc",
        extensions: "sv4crc",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-xpixmap",
        extensions: "xpm",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-sh",
        extensions: "sh",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-pcx",
        extensions: "pcx",
    },
    MIMETypeAndExtension {
        mime_type: "audio/mpeg",
        extensions: "mp3|mpga|mp2",
    },
    MIMETypeAndExtension {
        mime_type: "application/xml",
        extensions: "xml",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-chess-pgn",
        extensions: "pgn",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-troff",
        extensions: "t|tr|roff",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-cpio",
        extensions: "cpio",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-msdownload",
        extensions: "dll",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-tex",
        extensions: "tex",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-futuresplash",
        extensions: "spl",
    },
    MIMETypeAndExtension {
        mime_type: "application/postscript",
        extensions: "ps|eps|ai",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-shockwave-flash",
        extensions: "swf",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.ms-excel",
        extensions: "xls",
    },
    MIMETypeAndExtension {
        mime_type: "application/mspowerpoint",
        extensions: "ppt",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-portable-bitmap",
        extensions: "pbm",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-troff-man",
        extensions: "man",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.fdf",
        extensions: "fdf",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-dvi",
        extensions: "dvi",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.ms-powerpoint",
        extensions: "ppt",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-olympus-or",
        extensions: "orf",
    },
    MIMETypeAndExtension {
        mime_type: "audio/x-pn-realaudio-plugin",
        extensions: "rpm",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.wap.wmlc",
        extensions: "wmlc",
    },
    MIMETypeAndExtension {
        mime_type: "model/vrml",
        extensions: "wrl|vrml",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.adobe.xfd+xml",
        extensions: "xfd",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-cmu-raster",
        extensions: "ras",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-portable-anymap",
        extensions: "pnm",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-bittorrent",
        extensions: "torrent",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-diskcopy",
        extensions: "dmg",
    },
    MIMETypeAndExtension {
        mime_type: "video/mp4",
        extensions: "mp4|mpg4",
    },
    MIMETypeAndExtension {
        mime_type: "chemical/x-pdb",
        extensions: "pdb",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-targa",
        extensions: "targa",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-portable-graymap",
        extensions: "pgm",
    },
    MIMETypeAndExtension {
        mime_type: "application/msword",
        extensions: "doc",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.wap.wbxml",
        extensions: "wbxml",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-texinfo",
        extensions: "texinfo|texi",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-netcdf",
        extensions: "nc|cdf",
    },
    MIMETypeAndExtension {
        mime_type: "application/oda",
        extensions: "oda",
    },
    MIMETypeAndExtension {
        mime_type: "text/richtext",
        extensions: "rtx",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-director",
        extensions: "dcr|dir|dxr",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-latex",
        extensions: "latex",
    },
    MIMETypeAndExtension {
        mime_type: "application/msexcel",
        extensions: "xls",
    },
    MIMETypeAndExtension {
        mime_type: "model/mesh",
        extensions: "msh|mesh|silo",
    },
    MIMETypeAndExtension {
        mime_type: "model/iges",
        extensions: "igs|iges",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.adobe.xdp+xml",
        extensions: "xdp",
    },
    MIMETypeAndExtension {
        mime_type: "audio/x-m4p",
        extensions: "m4p",
    },
    MIMETypeAndExtension {
        mime_type: "image/vnd.wap.wbmp",
        extensions: "wbmp",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-xwindowdump",
        extensions: "xwd",
    },
    MIMETypeAndExtension {
        mime_type: "x-conference/x-cooltalk",
        extensions: "ice",
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
        mime_type: "application/vnd.wap.wmlscriptc",
        extensions: "wmlsc",
    },
    MIMETypeAndExtension {
        mime_type: "text/qif",
        extensions: "qif",
    },
    MIMETypeAndExtension {
        mime_type: "audio/x-aiff",
        extensions: "aiff",
    },
    MIMETypeAndExtension {
        mime_type: "text/plain",
        extensions: "txt|asc",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-bcpio",
        extensions: "bcpio",
    },
    MIMETypeAndExtension {
        mime_type: "text/css",
        extensions: "css",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-cdlink",
        extensions: "vcd",
    },
    MIMETypeAndExtension {
        mime_type: "video/vnd.mpegurl",
        extensions: "mxu",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-rgb",
        extensions: "rgb",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-sv4cpio",
        extensions: "sv4cpio",
    },
];
