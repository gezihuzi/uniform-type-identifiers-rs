use crate::{MIMETypeAndExtension, UTType};

pub const PUBLIC_ITEM: UTType = UTType {
    identifier: "public.item",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "item",
};
pub const PUBLIC_DATA: UTType = UTType {
    identifier: "public.data",
    conforms_to: "public.item",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "data",
};
pub const PUBLIC_DIRECTORY: UTType = UTType {
    identifier: "public.directory",
    conforms_to: "public.item",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "directory",
};
pub const PUBLIC_CONTENT: UTType = UTType {
    identifier: "public.content",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "content",
};
pub const PUBLIC_COMPOSITE_CONTENT: UTType = UTType {
    identifier: "public.composite-content",
    conforms_to: "public.content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "content",
};
pub const PUBLIC_NAMED_PIPE: UTType = UTType {
    identifier: "public.named-pipe",
    conforms_to: "public.item",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const PUBLIC_CHARACTER_SPECIAL: UTType = UTType {
    identifier: "public.character-special",
    conforms_to: "public.item",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const PUBLIC_BLOCK_SPECIAL: UTType = UTType {
    identifier: "public.block-special",
    conforms_to: "public.item",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const PUBLIC_SOCKET: UTType = UTType {
    identifier: "public.socket",
    conforms_to: "public.item",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const PUBLIC_EXECUTABLE: UTType = UTType {
    identifier: "public.executable",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "executable",
};
pub const PUBLIC_UNIX_EXECUTABLE: UTType = UTType {
    identifier: "public.unix-executable",
    conforms_to: "public.data|public.executable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Unix executable",
};
pub const COM_APPLE_APPLICATION: UTType = UTType {
    identifier: "com.apple.application",
    conforms_to: "public.executable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "application",
};
pub const PUBLIC_ARCHIVE: UTType = UTType {
    identifier: "public.archive",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "file archive",
};
pub const PUBLIC_BOOKMARK: UTType = UTType {
    identifier: "public.bookmark",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Bookmark",
};
pub const PUBLIC_DATABASE: UTType = UTType {
    identifier: "public.database",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "database",
};
pub const COM_APPLE_CSSTORE: UTType = UTType {
    identifier: "com.apple.csstore",
    conforms_to: "public.data|public.database",
    tags: "",
    filename_extension: "csstore",
    mime_type: "csstore",
    description: "",
};
pub const PUBLIC_PRESENTATION: UTType = UTType {
    identifier: "public.presentation",
    conforms_to: "public.composite-content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "presentation",
};
pub const PUBLIC_SPREADSHEET: UTType = UTType {
    identifier: "public.spreadsheet",
    conforms_to: "public.content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Spreadsheet",
};
pub const COM_APPLE_ICLOUD: UTType = UTType {
    identifier: "com.apple.iCloud",
    conforms_to: "public.item",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iCloud",
};
pub const PUBLIC_3D_CONTENT: UTType = UTType {
    identifier: "public.3d-content",
    conforms_to: "public.content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "3D Content",
};
pub const PUBLIC_ALEMBIC: UTType = UTType {
    identifier: "public.alembic",
    conforms_to: "public.3d-content|public.data",
    tags: "",
    filename_extension: "abc",
    mime_type: "abc",
    description: "Alembic 3D Scene",
};
pub const PUBLIC_GEOMETRY_DEFINITION_FORMAT: UTType = UTType {
    identifier: "public.geometry-definition-format",
    conforms_to: "public.3d-content|public.text",
    tags: "",
    filename_extension: "obj",
    mime_type: "obj",
    description: "Geometry Definition File Format",
};
pub const PUBLIC_STANDARD_TESSELATED_GEOMETRY_FORMAT: UTType = UTType {
    identifier: "public.standard-tesselated-geometry-format",
    conforms_to: "public.3d-content|public.data",
    tags: "",
    filename_extension: "stl",
    mime_type: "stl",
    description: "Standard Tesselated Geometry File Format",
};
pub const PUBLIC_POLYGON_FILE_FORMAT: UTType = UTType {
    identifier: "public.polygon-file-format",
    conforms_to: "public.3d-content|public.data",
    tags: "",
    filename_extension: "ply",
    mime_type: "ply",
    description: "Polygon File Format",
};
pub const COM_PIXAR_UNIVERSAL_SCENE_DESCRIPTION: UTType = UTType {
    identifier: "com.pixar.universal-scene-description",
    conforms_to: "public.3d-content|public.data",
    tags: "",
    filename_extension: "usd|usda|usdc",
    mime_type: "usd|usda|usdc",
    description: "Universal Scene Description",
};
pub const COM_PIXAR_UNIVERSAL_SCENE_DESCRIPTION_MOBILE: UTType = UTType {
    identifier: "com.pixar.universal-scene-description-mobile",
    conforms_to: "public.3d-content|public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Universal Scene Description Package",
};
pub const COM_APPLE_REALITY: UTType = UTType {
    identifier: "com.apple.reality",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Reality File",
};
pub const COM_APPLE_SCENEKIT_SCENE: UTType = UTType {
    identifier: "com.apple.scenekit.scene",
    conforms_to: "public.data|public.3d-content",
    tags: "",
    filename_extension: "scn|scnz",
    mime_type: "scn|scnz",
    description: "SceneKit serialized format",
};
pub const COM_APPLE_AROBJECT: UTType = UTType {
    identifier: "com.apple.arobject",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "AR Reference Object",
};
pub const PUBLIC_MESSAGE: UTType = UTType {
    identifier: "public.message",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "message",
};
pub const PUBLIC_EMAIL_MESSAGE: UTType = UTType {
    identifier: "public.email-message",
    conforms_to: "public.message",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "email message",
};
pub const PUBLIC_TO_DO_ITEM: UTType = UTType {
    identifier: "public.to-do-item",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "to-do item",
};
pub const PUBLIC_CALENDAR_EVENT: UTType = UTType {
    identifier: "public.calendar-event",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "calendar event",
};
pub const COM_APPLE_ICAL_VCS: UTType = UTType {
    identifier: "com.apple.ical.vcs",
    conforms_to: "public.text|public.item|public.calendar-event",
    tags: "",
    filename_extension: "vcs|vcal",
    mime_type: "vcs|vcal",
    description: "VCS File",
};
pub const COM_APPLE_ICAL_ICS: UTType = UTType {
    identifier: "com.apple.ical.ics",
    conforms_to: "public.text|public.item|public.calendar-event",
    tags: "",
    filename_extension: "ics",
    mime_type: "ics",
    description: "ICS File",
};
pub const PUBLIC_CONTACT: UTType = UTType {
    identifier: "public.contact",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "contact information",
};
pub const PUBLIC_VCARD: UTType = UTType {
    identifier: "public.vcard",
    conforms_to: "public.text|public.contact",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "electronic business card",
};
pub const COM_APPLE_SHAZAMSIGNATURE: UTType = UTType {
    identifier: "com.apple.shazamsignature",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "shazamsignature",
    mime_type: "shazamsignature",
    description: "Shazam Signature Data",
};
pub const COM_APPLE_SHAZAMCATALOG: UTType = UTType {
    identifier: "com.apple.shazamcatalog",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "shazamcatalog",
    mime_type: "shazamcatalog",
    description: "Shazam Catalog",
};
pub const PUBLIC_TEXT: UTType = UTType {
    identifier: "public.text",
    conforms_to: "public.data|public.content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "text",
};
pub const PUBLIC_PLAIN_TEXT: UTType = UTType {
    identifier: "public.plain-text",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const PUBLIC_UTF8_PLAIN_TEXT: UTType = UTType {
    identifier: "public.utf8-plain-text",
    conforms_to: "public.plain-text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const PUBLIC_UTF16_EXTERNAL_PLAIN_TEXT: UTType = UTType {
    identifier: "public.utf16-external-plain-text",
    conforms_to: "public.plain-text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const PUBLIC_UTF16_PLAIN_TEXT: UTType = UTType {
    identifier: "public.utf16-plain-text",
    conforms_to: "public.plain-text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_TRADITIONAL_MAC_PLAIN_TEXT: UTType = UTType {
    identifier: "com.apple.traditional-mac-plain-text",
    conforms_to: "public.plain-text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const PUBLIC_CASE_INSENSITIVE_TEXT: UTType = UTType {
    identifier: "public.case-insensitive-text",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const PUBLIC_LOG: UTType = UTType {
    identifier: "public.log",
    conforms_to: "public.item",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Log file",
};
pub const COM_APPLE_LOG: UTType = UTType {
    identifier: "com.apple.log",
    conforms_to: "public.plain-text|public.log",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_SHUTDOWNSTALL: UTType = UTType {
    identifier: "com.apple.shutdownStall",
    conforms_to: "public.utf8-plain-text|public.log",
    tags: "",
    filename_extension: "shutdownStall",
    mime_type: "shutdownStall",
    description: "Shutdown Stall",
};
pub const COM_APPLE_GPURESTART: UTType = UTType {
    identifier: "com.apple.gpuRestart",
    conforms_to: "public.utf8-plain-text|public.log",
    tags: "",
    filename_extension: "gpuRestart",
    mime_type: "gpuRestart",
    description: "GPU Restart",
};
pub const COM_APPLE_CRASHREPORT: UTType = UTType {
    identifier: "com.apple.crashreport",
    conforms_to: "public.utf8-plain-text|public.log",
    tags: "",
    filename_extension: "crash",
    mime_type: "crash",
    description: "Crash Report",
};
pub const COM_APPLE_HANGREPORT: UTType = UTType {
    identifier: "com.apple.hangreport",
    conforms_to: "public.utf8-plain-text|public.log",
    tags: "",
    filename_extension: "hang",
    mime_type: "hang",
    description: "Hang Report",
};
pub const COM_APPLE_SPINREPORT: UTType = UTType {
    identifier: "com.apple.spinreport",
    conforms_to: "public.utf8-plain-text|public.log",
    tags: "",
    filename_extension: "spin",
    mime_type: "spin",
    description: "Spin Report",
};
pub const COM_APPLE_PANICREPORT: UTType = UTType {
    identifier: "com.apple.panicreport",
    conforms_to: "public.utf8-plain-text|public.log",
    tags: "",
    filename_extension: "panic",
    mime_type: "panic",
    description: "Panic Report",
};
pub const COM_APPLE_KTRACE: UTType = UTType {
    identifier: "com.apple.ktrace",
    conforms_to: "public.data|public.log",
    tags: "",
    filename_extension: "ktrace",
    mime_type: "ktrace",
    description: "Darwin kernel trace file",
};
pub const PUBLIC_FILENAME_EXTENSION: UTType = UTType {
    identifier: "public.filename-extension",
    conforms_to: "public.case-insensitive-text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "file name extension",
};
pub const PUBLIC_MIME_TYPE: UTType = UTType {
    identifier: "public.mime-type",
    conforms_to: "public.case-insensitive-text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "MIME type",
};
pub const COM_APPLE_OSTYPE: UTType = UTType {
    identifier: "com.apple.ostype",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "four-character code",
};
pub const COM_APPLE_NSPBOARD_TYPE: UTType = UTType {
    identifier: "com.apple.nspboard-type",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "NSPasteboard type",
};
pub const COM_APPLE_DEVICE_MODEL_CODE: UTType = UTType {
    identifier: "com.apple.device-model-code",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "device model code",
};
pub const COM_APPLE_PASTEBOARD_PROMISED_FILE_URL: UTType = UTType {
    identifier: "com.apple.pasteboard.promised-file-url",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Promised file URL",
};
pub const COM_APPLE_PASTEBOARD_PROMISED_FILE_CONTENT_TYPE: UTType = UTType {
    identifier: "com.apple.pasteboard.promised-file-content-type",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Promised file content type",
};
pub const COM_APPLE_COCOA_PASTEBOARD_COLOR: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.color",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "color",
};
pub const COM_APPLE_COCOA_PASTEBOARD_SOUND: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.sound",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "sound",
};
pub const COM_APPLE_COCOA_PASTEBOARD_CHARACTER_FORMATTING: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.character-formatting",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "character format",
};
pub const COM_APPLE_COCOA_PASTEBOARD_PARAGRAPH_FORMATTING: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.paragraph-formatting",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "paragraph format",
};
pub const COM_APPLE_COCOA_PASTEBOARD_MULTIPLE_TEXT_SELECTION: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.multiple-text-selection",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "text selection",
};
pub const COM_APPLE_COCOA_PASTEBOARD_FIND_PANEL_SEARCH_OPTIONS: UTType = UTType {
    identifier: "com.apple.cocoa.pasteboard.find-panel-search-options",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "search options",
};
pub const COM_APPLE_MAPKIT_MAP_ITEM: UTType = UTType {
    identifier: "com.apple.mapkit.map-item",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Map Item",
};
pub const COM_APPLE_RESOLVABLE: UTType = UTType {
    identifier: "com.apple.resolvable",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "resolvable",
};
pub const PUBLIC_SYMLINK: UTType = UTType {
    identifier: "public.symlink",
    conforms_to: "public.item|com.apple.resolvable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "symbolic link",
};
pub const COM_APPLE_MOUNT_POINT: UTType = UTType {
    identifier: "com.apple.mount-point",
    conforms_to: "public.item|com.apple.resolvable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "mount point",
};
pub const COM_APPLE_BOOKMARK: UTType = UTType {
    identifier: "com.apple.bookmark",
    conforms_to: "public.data|com.apple.resolvable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "bookmark",
};
pub const COM_APPLE_ALIAS_FILE: UTType = UTType {
    identifier: "com.apple.alias-file",
    conforms_to: "public.data|com.apple.resolvable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "alias",
};
pub const COM_APPLE_ALIAS_RECORD: UTType = UTType {
    identifier: "com.apple.alias-record",
    conforms_to: "public.data|com.apple.resolvable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "alias data",
};
pub const COM_APPLE_ICLOUD_FILE_FAULT: UTType = UTType {
    identifier: "com.apple.icloud-file-fault",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "icloud",
    mime_type: "icloud",
    description: "iCloud synchronization file",
};
pub const COM_APPLE_FINDER_CLIPPING: UTType = UTType {
    identifier: "com.apple.finder.clipping",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "clipping",
};
pub const COM_APPLE_FINDER_SOUND_CLIPPING: UTType = UTType {
    identifier: "com.apple.finder.sound-clipping",
    conforms_to: "com.apple.finder.clipping",
    tags: "",
    filename_extension: "sndClipping",
    mime_type: "sndClipping",
    description: "Sound Clipping",
};
pub const COM_APPLE_FINDER_TEXTCLIPPING: UTType = UTType {
    identifier: "com.apple.finder.textclipping",
    conforms_to: "com.apple.finder.clipping",
    tags: "",
    filename_extension: "textclipping",
    mime_type: "textclipping",
    description: "text clipping",
};
pub const COM_APPLE_FINDER_PICTCLIPPING: UTType = UTType {
    identifier: "com.apple.finder.pictclipping",
    conforms_to: "com.apple.finder.clipping|public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "picture clipping",
};
pub const COM_APPLE_FINDER_BURN_FOLDER: UTType = UTType {
    identifier: "com.apple.finder.burn-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Burn Folder",
};
pub const COM_APPLE_ICONSET: UTType = UTType {
    identifier: "com.apple.iconset",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Set",
};
pub const COM_APPLE_FINDER_SMART_FOLDER: UTType = UTType {
    identifier: "com.apple.finder.smart-folder",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Smart Folder",
};
pub const COM_APPLE_FINDER_RECENT_ITEMS: UTType = UTType {
    identifier: "com.apple.finder.recent-items",
    conforms_to: "com.apple.finder.smart-folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Recent Items",
};
pub const PUBLIC_OBJECT_CODE: UTType = UTType {
    identifier: "public.object-code",
    conforms_to: "public.data|public.executable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "object code",
};
pub const COM_APPLE_MACH_O_BINARY: UTType = UTType {
    identifier: "com.apple.mach-o-binary",
    conforms_to: "public.unix-executable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Mach-O binary",
};
pub const COM_APPLE_MACH_O_OBJECT: UTType = UTType {
    identifier: "com.apple.mach-o-object",
    conforms_to: "com.apple.mach-o-binary",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Mach-O object",
};
pub const COM_APPLE_MACH_O_EXECUTABLE: UTType = UTType {
    identifier: "com.apple.mach-o-executable",
    conforms_to: "com.apple.mach-o-binary",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Mach-O executable",
};
pub const COM_APPLE_X11_MACH_O_EXECUTABLE: UTType = UTType {
    identifier: "com.apple.x11-mach-o-executable",
    conforms_to: "com.apple.mach-o-binary",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "X11 application",
};
pub const COM_APPLE_MACH_O_CORE: UTType = UTType {
    identifier: "com.apple.mach-o-core",
    conforms_to: "com.apple.mach-o-binary",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Mach-O core",
};
pub const COM_APPLE_MACH_O_DYLIB: UTType = UTType {
    identifier: "com.apple.mach-o-dylib",
    conforms_to: "com.apple.mach-o-binary",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Mach-O dynamic library",
};
pub const COM_APPLE_MACH_O_BUNDLE: UTType = UTType {
    identifier: "com.apple.mach-o-bundle",
    conforms_to: "com.apple.mach-o-binary",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Mach-O bundle",
};
pub const COM_APPLE_PEF_BINARY: UTType = UTType {
    identifier: "com.apple.pef-binary",
    conforms_to: "public.data|public.executable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PEF binary",
};
pub const PUBLIC_ELF_BINARY: UTType = UTType {
    identifier: "public.elf-binary",
    conforms_to: "public.data|public.executable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "ELF binary",
};
pub const COM_MICROSOFT_WINDOWS_EXECUTABLE: UTType = UTType {
    identifier: "com.microsoft.windows-executable",
    conforms_to: "public.data|public.executable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Microsoft Windows application",
};
pub const COM_MICROSOFT_WINDOWS_DYNAMIC_LINK_LIBRARY: UTType = UTType {
    identifier: "com.microsoft.windows-dynamic-link-library",
    conforms_to: "public.data|public.executable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Microsoft dynamic link library",
};
pub const COM_SUN_JAVA_CLASS: UTType = UTType {
    identifier: "com.sun.java-class",
    conforms_to: "public.data|public.executable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Java class",
};
pub const COM_SUN_JAVA_ARCHIVE: UTType = UTType {
    identifier: "com.sun.java-archive",
    conforms_to: "public.zip-archive|public.executable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Java archive",
};
pub const COM_SUN_WEB_APPLICATION_ARCHIVE: UTType = UTType {
    identifier: "com.sun.web-application-archive",
    conforms_to: "com.sun.java-archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "web application archive",
};
pub const COM_APPLE_QUARTZ_COMPOSER_COMPOSITION: UTType = UTType {
    identifier: "com.apple.quartz-composer-composition",
    conforms_to: "public.data|public.executable",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Quartz Composer compostion",
};
pub const COM_APPLE_BOM_ARCHIVE: UTType = UTType {
    identifier: "com.apple.bom-archive",
    conforms_to: "public.archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "BOM-compatible archive",
};
pub const PUBLIC_DISK_IMAGE: UTType = UTType {
    identifier: "public.disk-image",
    conforms_to: "public.archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "disk image",
};
pub const ORG_GNU_GNU_TAR_ARCHIVE: UTType = UTType {
    identifier: "org.gnu.gnu-tar-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "gtar",
    mime_type: "gtar",
    description: "GNU tar archive",
};
pub const PUBLIC_TAR_ARCHIVE: UTType = UTType {
    identifier: "public.tar-archive",
    conforms_to: "org.gnu.gnu-tar-archive",
    tags: "",
    filename_extension: "tar",
    mime_type: "tar",
    description: "tar archive",
};
pub const ORG_GNU_GNU_ZIP_ARCHIVE: UTType = UTType {
    identifier: "org.gnu.gnu-zip-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "gz|gzip",
    mime_type: "gz|gzip",
    description: "GZip archive",
};
pub const ORG_GNU_GNU_ZIP_TAR_ARCHIVE: UTType = UTType {
    identifier: "org.gnu.gnu-zip-tar-archive",
    conforms_to: "org.gnu.gnu-zip-archive",
    tags: "",
    filename_extension: "tgz",
    mime_type: "tgz",
    description: "gzip tar archive",
};
pub const PUBLIC_BZIP2_ARCHIVE: UTType = UTType {
    identifier: "public.bzip2-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "bz2|bz",
    mime_type: "bz2|bz",
    description: "Bzip2 archive",
};
pub const PUBLIC_TAR_BZIP2_ARCHIVE: UTType = UTType {
    identifier: "public.tar-bzip2-archive",
    conforms_to: "public.bzip2-archive",
    tags: "",
    filename_extension: "tbz2|tbz",
    mime_type: "tbz2|tbz",
    description: "Bzip2 compressed tar archive",
};
pub const COM_APPLE_BINHEX_ARCHIVE: UTType = UTType {
    identifier: "com.apple.binhex-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "BinHex archive",
};
pub const COM_APPLE_MACBINARY_ARCHIVE: UTType = UTType {
    identifier: "com.apple.macbinary-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "MacBinary archive",
};
pub const COM_APPLE_APPLESINGLE_ARCHIVE: UTType = UTType {
    identifier: "com.apple.applesingle-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "as",
    mime_type: "as",
    description: "AppleSingle archive",
};
pub const PUBLIC_UUENCODED_ARCHIVE: UTType = UTType {
    identifier: "public.uuencoded-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "uu",
    mime_type: "uu",
    description: "UUEncoded archive",
};
pub const PUBLIC_Z_ARCHIVE: UTType = UTType {
    identifier: "public.z-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "z",
    mime_type: "z",
    description: "Z archive",
};
pub const COM_APPLE_BOM_COMPRESSED_CPIO: UTType = UTType {
    identifier: "com.apple.bom-compressed-cpio",
    conforms_to: "public.data|com.apple.bom-archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "BOM-generated compressed CPIO archive",
};
pub const PUBLIC_CPIO_ARCHIVE: UTType = UTType {
    identifier: "public.cpio-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "cpio|pax",
    mime_type: "cpio|pax",
    description: "CPIO archive",
};
pub const COM_PKWARE_ZIP_ARCHIVE: UTType = UTType {
    identifier: "com.pkware.zip-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PKZip archive",
};
pub const PUBLIC_ZIP_ARCHIVE: UTType = UTType {
    identifier: "public.zip-archive",
    conforms_to: "com.pkware.zip-archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Zip archive",
};
pub const COM_APPLE_XAR_ARCHIVE: UTType = UTType {
    identifier: "com.apple.xar-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "XAR Archive",
};
pub const COM_APPLE_XIP_ARCHIVE: UTType = UTType {
    identifier: "com.apple.xip-archive",
    conforms_to: "com.apple.xar-archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "XIP Secure Archive",
};
pub const COM_APPLE_INSTALLER_PACKAGE_ARCHIVE: UTType = UTType {
    identifier: "com.apple.installer-package-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "pkg|mpkg",
    mime_type: "pkg|mpkg",
    description: "Installer package archive",
};
pub const COM_APPLE_ARCHIVE: UTType = UTType {
    identifier: "com.apple.archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "aar|yaa",
    mime_type: "aar|yaa",
    description: "Apple Archive",
};
pub const COM_APPLE_ENCRYPTED_ARCHIVE: UTType = UTType {
    identifier: "com.apple.encrypted-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "aea",
    mime_type: "aea",
    description: "Apple Encrypted Archive",
};
pub const PUBLIC_URL: UTType = UTType {
    identifier: "public.url",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "URL",
};
pub const PUBLIC_FILE_URL: UTType = UTType {
    identifier: "public.file-url",
    conforms_to: "public.url",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "file URL",
};
pub const PUBLIC_URL_NAME: UTType = UTType {
    identifier: "public.url-name",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "URL name",
};
pub const PUBLIC_STORED_URL: UTType = UTType {
    identifier: "public.stored-url",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.internet-location",
    conforms_to: "public.stored-url|public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "internet location",
};
pub const COM_APPLE_WEB_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.web-internet-location",
    conforms_to: "com.apple.internet-location",
    tags: "",
    filename_extension: "webloc",
    mime_type: "webloc",
    description: "web internet location",
};
pub const COM_APPLE_VNC_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.vnc-internet-location",
    conforms_to: "com.apple.internet-location",
    tags: "",
    filename_extension: "vncloc",
    mime_type: "vncloc",
    description: "VNC Internet Location",
};
pub const COM_APPLE_MAIL_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.mail-internet-location",
    conforms_to: "com.apple.internet-location",
    tags: "",
    filename_extension: "mailloc",
    mime_type: "mailloc",
    description: "mail internet location",
};
pub const COM_APPLE_AFP_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.afp-internet-location",
    conforms_to: "com.apple.internet-location",
    tags: "",
    filename_extension: "afploc",
    mime_type: "afploc",
    description: "AFP internet location",
};
pub const COM_APPLE_FILE_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.file-internet-location",
    conforms_to: "com.apple.internet-location",
    tags: "",
    filename_extension: "fileloc",
    mime_type: "fileloc",
    description: "file internet location",
};
pub const COM_APPLE_FTP_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.ftp-internet-location",
    conforms_to: "com.apple.internet-location",
    tags: "",
    filename_extension: "ftploc",
    mime_type: "ftploc",
    description: "FTP internet location",
};
pub const COM_APPLE_NEWS_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.news-internet-location",
    conforms_to: "com.apple.internet-location",
    tags: "",
    filename_extension: "newsloc",
    mime_type: "newsloc",
    description: "news internet location",
};
pub const COM_APPLE_GENERIC_INTERNET_LOCATION: UTType = UTType {
    identifier: "com.apple.generic-internet-location",
    conforms_to: "com.apple.internet-location",
    tags: "",
    filename_extension: "inetloc",
    mime_type: "inetloc",
    description: "internet location",
};
pub const COM_MICROSOFT_INTERNET_SHORTCUT: UTType = UTType {
    identifier: "com.microsoft.internet-shortcut",
    conforms_to: "public.stored-url|public.data",
    tags: "",
    filename_extension: "url",
    mime_type: "url",
    description: "Windows Internet shortcut",
};
pub const COM_APPLE_ITUNES_STORE_URL: UTType = UTType {
    identifier: "com.apple.itunes.store-url",
    conforms_to: "public.url",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iTunes store URL",
};
pub const PUBLIC_DELIMITED_VALUES_TEXT: UTType = UTType {
    identifier: "public.delimited-values-text",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "delimited text values",
};
pub const PUBLIC_COMMA_SEPARATED_VALUES_TEXT: UTType = UTType {
    identifier: "public.comma-separated-values-text",
    conforms_to: "public.plain-text|public.delimited-values-text",
    tags: "",
    filename_extension: "csv",
    mime_type: "csv",
    description: "comma-separated values",
};
pub const PUBLIC_TAB_SEPARATED_VALUES_TEXT: UTType = UTType {
    identifier: "public.tab-separated-values-text",
    conforms_to: "public.plain-text|public.delimited-values-text",
    tags: "",
    filename_extension: "tsv",
    mime_type: "tsv",
    description: "tab-separated values",
};
pub const PUBLIC_UTF8_TAB_SEPARATED_VALUES_TEXT: UTType = UTType {
    identifier: "public.utf8-tab-separated-values-text",
    conforms_to: "public.tab-separated-values-text|public.utf8-plain-text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const PUBLIC_RTF: UTType = UTType {
    identifier: "public.rtf",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "rich text (RTF)",
};
pub const PUBLIC_HTML: UTType = UTType {
    identifier: "public.html",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "HTML text",
};
pub const PUBLIC_XML: UTType = UTType {
    identifier: "public.xml",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "XML text",
};
pub const PUBLIC_XHTML: UTType = UTType {
    identifier: "public.xhtml",
    conforms_to: "public.xml",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "XHTML",
};
pub const PUBLIC_RSS: UTType = UTType {
    identifier: "public.rss",
    conforms_to: "public.xml",
    tags: "",
    filename_extension: "rss",
    mime_type: "rss",
    description: "RSS web feed",
};
pub const PUBLIC_XFD: UTType = UTType {
    identifier: "public.xfd",
    conforms_to: "public.xml",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "XML Form (XFD)",
};
pub const PUBLIC_CSS: UTType = UTType {
    identifier: "public.css",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "css",
    mime_type: "css",
    description: "CSS",
};
pub const PUBLIC_PATCH_FILE: UTType = UTType {
    identifier: "public.patch-file",
    conforms_to: "public.plain-text",
    tags: "",
    filename_extension: "patch|diff",
    mime_type: "patch|diff",
    description: "patch file",
};
pub const PUBLIC_JSON: UTType = UTType {
    identifier: "public.json",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "json",
    mime_type: "json",
    description: "JSON",
};
pub const PUBLIC_NDJSON: UTType = UTType {
    identifier: "public.ndjson",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "ndjson",
    mime_type: "ndjson",
    description: "NDJSON",
};
pub const PUBLIC_YAML: UTType = UTType {
    identifier: "public.yaml",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "yml|yaml",
    mime_type: "yml|yaml",
    description: "YAML",
};
pub const COM_SCENARIST_CLOSED_CAPTION: UTType = UTType {
    identifier: "com.scenarist.closed-caption",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "scc",
    mime_type: "scc",
    description: "Scenarist Closed Caption",
};
pub const ORG_W3_WEBVTT: UTType = UTType {
    identifier: "org.w3.webvtt",
    conforms_to: "public.text",
    tags: "",
    filename_extension: "vtt",
    mime_type: "vtt",
    description: "WebVTT Format",
};
pub const COM_APPLE_GENERIC_STATIONERY: UTType = UTType {
    identifier: "com.apple.generic-stationery",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Stationery",
};
pub const COM_APPLE_PROPERTY_LIST: UTType = UTType {
    identifier: "com.apple.property-list",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "plist",
    mime_type: "plist",
    description: "property list",
};
pub const COM_APPLE_XML_PROPERTY_LIST: UTType = UTType {
    identifier: "com.apple.xml-property-list",
    conforms_to: "public.xml|com.apple.property-list",
    tags: "",
    filename_extension: "plist",
    mime_type: "plist",
    description: "XML property list",
};
pub const COM_APPLE_BINARY_PROPERTY_LIST: UTType = UTType {
    identifier: "com.apple.binary-property-list",
    conforms_to: "com.apple.property-list",
    tags: "",
    filename_extension: "plist",
    mime_type: "plist",
    description: "binary property list",
};
pub const COM_APPLE_ASCII_PROPERTY_LIST: UTType = UTType {
    identifier: "com.apple.ascii-property-list",
    conforms_to: "public.text|com.apple.property-list",
    tags: "",
    filename_extension: "plist",
    mime_type: "plist",
    description: "ascii property list",
};
pub const PUBLIC_SOURCE_CODE: UTType = UTType {
    identifier: "public.source-code",
    conforms_to: "public.plain-text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "source code",
};
pub const PUBLIC_SOURCE_CODE_PREPROCESSED: UTType = UTType {
    identifier: "public.source-code.preprocessed",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "preprocessed source code",
};
pub const PUBLIC_C_SOURCE: UTType = UTType {
    identifier: "public.c-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "C source code",
};
pub const PUBLIC_C_SOURCE_PREPROCESSED: UTType = UTType {
    identifier: "public.c-source.preprocessed",
    conforms_to: "public.c-source|public.source-code.preprocessed",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "preprocessed C source code",
};
pub const COM_APPLE_IIG_SOURCE: UTType = UTType {
    identifier: "com.apple.iig-source",
    conforms_to: "public.c-source",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "IOKit Interface Generator source code",
};
pub const PUBLIC_OBJECTIVE_C_SOURCE: UTType = UTType {
    identifier: "public.objective-c-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Objective-C source code",
};
pub const PUBLIC_OBJECTIVE_C_SOURCE_PREPROCESSED: UTType = UTType {
    identifier: "public.objective-c-source.preprocessed",
    conforms_to: "public.objective-c-source|public.source-code.preprocessed",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "preprocessed Objective-C source code",
};
pub const PUBLIC_C_PLUS_PLUS_SOURCE: UTType = UTType {
    identifier: "public.c-plus-plus-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "cp|cpp|c++|cc|cxx",
    mime_type: "cp|cpp|c++|cc|cxx",
    description: "C++ source code",
};
pub const PUBLIC_C_PLUS_PLUS_SOURCE_PREPROCESSED: UTType = UTType {
    identifier: "public.c-plus-plus-source.preprocessed",
    conforms_to: "public.c-plus-plus-source|public.source-code.preprocessed",
    tags: "",
    filename_extension: "ii",
    mime_type: "ii",
    description: "preprocessed C++ source code",
};
pub const PUBLIC_OBJECTIVE_C_PLUS_PLUS_SOURCE: UTType = UTType {
    identifier: "public.objective-c-plus-plus-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Objective-C++ source code",
};
pub const PUBLIC_OBJECTIVE_C_PLUS_PLUS_SOURCE_PREPROCESSED: UTType = UTType {
    identifier: "public.objective-c-plus-plus-source.preprocessed",
    conforms_to: "public.objective-c-plus-plus-source|public.source-code.preprocessed",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "preprocessed Objective-C++ source code",
};
pub const PUBLIC_C_HEADER: UTType = UTType {
    identifier: "public.c-header",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "C header code",
};
pub const PUBLIC_PRECOMPILED_C_HEADER: UTType = UTType {
    identifier: "public.precompiled-c-header",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "precompiled C header",
};
pub const PUBLIC_C_PLUS_PLUS_HEADER: UTType = UTType {
    identifier: "public.c-plus-plus-header",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "hh|hp|hpp|hxx|h++|ipp",
    mime_type: "hh|hp|hpp|hxx|h++|ipp",
    description: "C++ header code",
};
pub const PUBLIC_C_PLUS_PLUS_INLINE_HEADER: UTType = UTType {
    identifier: "public.c-plus-plus-inline-header",
    conforms_to: "public.c-plus-plus-header",
    tags: "",
    filename_extension: "inl",
    mime_type: "inl",
    description: "C++ Inline Header",
};
pub const PUBLIC_PRECOMPILED_C_PLUS_PLUS_HEADER: UTType = UTType {
    identifier: "public.precompiled-c-plus-plus-header",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "precompiled C++ header",
};
pub const PUBLIC_SWIFT_SOURCE: UTType = UTType {
    identifier: "public.swift-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "swift",
    mime_type: "swift",
    description: "Swift Source Code",
};
pub const COM_SUN_JAVA_SOURCE: UTType = UTType {
    identifier: "com.sun.java-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "java|jav",
    mime_type: "java|jav",
    description: "Java source code",
};
pub const PUBLIC_SCRIPT: UTType = UTType {
    identifier: "public.script",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "script",
};
pub const PUBLIC_ASSEMBLY_SOURCE: UTType = UTType {
    identifier: "public.assembly-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "s",
    mime_type: "s",
    description: "assembly source code",
};
pub const COM_APPLE_REZ_SOURCE: UTType = UTType {
    identifier: "com.apple.rez-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "r",
    mime_type: "r",
    description: "Rez source code",
};
pub const PUBLIC_LEX_SOURCE: UTType = UTType {
    identifier: "public.lex-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "l|lm|lmm|lpp|lxx|ll",
    mime_type: "l|lm|lmm|lpp|lxx|ll",
    description: "Lex source code",
};
pub const PUBLIC_YACC_SOURCE: UTType = UTType {
    identifier: "public.yacc-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "y|ym|ymm|ypp|yxx|yy",
    mime_type: "y|ym|ymm|ypp|yxx|yy",
    description: "Yacc source code",
};
pub const PUBLIC_MIG_SOURCE: UTType = UTType {
    identifier: "public.mig-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "defs|mig",
    mime_type: "defs|mig",
    description: "Mig definition source code",
};
pub const COM_APPLE_SYMBOL_EXPORT: UTType = UTType {
    identifier: "com.apple.symbol-export",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "exp",
    mime_type: "exp",
    description: "symbol export list",
};
pub const PUBLIC_FORTRAN_SOURCE: UTType = UTType {
    identifier: "public.fortran-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "f|for",
    mime_type: "f|for",
    description: "Fortran source code",
};
pub const PUBLIC_FORTRAN_77_SOURCE: UTType = UTType {
    identifier: "public.fortran-77-source",
    conforms_to: "public.fortran-source",
    tags: "",
    filename_extension: "f77",
    mime_type: "f77",
    description: "",
};
pub const PUBLIC_FORTRAN_90_SOURCE: UTType = UTType {
    identifier: "public.fortran-90-source",
    conforms_to: "public.fortran-source",
    tags: "",
    filename_extension: "f90",
    mime_type: "f90",
    description: "",
};
pub const PUBLIC_FORTRAN_95_SOURCE: UTType = UTType {
    identifier: "public.fortran-95-source",
    conforms_to: "public.fortran-source",
    tags: "",
    filename_extension: "f95",
    mime_type: "f95",
    description: "",
};
pub const PUBLIC_PASCAL_SOURCE: UTType = UTType {
    identifier: "public.pascal-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "pas",
    mime_type: "pas",
    description: "Pascal source code",
};
pub const PUBLIC_ADA_SOURCE: UTType = UTType {
    identifier: "public.ada-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "ada|adb|ads",
    mime_type: "ada|adb|ads",
    description: "Ada source code",
};
pub const PUBLIC_DYLAN_SOURCE: UTType = UTType {
    identifier: "public.dylan-source",
    conforms_to: "public.source-code",
    tags: "",
    filename_extension: "dlyan|lid",
    mime_type: "dlyan|lid",
    description: "Dylan source code",
};
pub const COM_NETSCAPE_JAVASCRIPT_SOURCE: UTType = UTType {
    identifier: "com.netscape.javascript-source",
    conforms_to: "public.script|public.executable",
    tags: "",
    filename_extension: "js|jscript|javascript|mjs",
    mime_type: "js|jscript|javascript|mjs",
    description: "JavaScript",
};
pub const COM_APPLE_XCODE_DSYM: UTType = UTType {
    identifier: "com.apple.xcode.dsym",
    conforms_to: "com.apple.package",
    tags: "",
    filename_extension: "dsym",
    mime_type: "dsym",
    description: "",
};
pub const PUBLIC_SHELL_SCRIPT: UTType = UTType {
    identifier: "public.shell-script",
    conforms_to: "public.script",
    tags: "",
    filename_extension: "sh",
    mime_type: "sh",
    description: "shell script",
};
pub const PUBLIC_BASH_SCRIPT: UTType = UTType {
    identifier: "public.bash-script",
    conforms_to: "public.shell-script",
    tags: "",
    filename_extension: "bash",
    mime_type: "bash",
    description: "Bourne-Again Shell script",
};
pub const PUBLIC_CSH_SCRIPT: UTType = UTType {
    identifier: "public.csh-script",
    conforms_to: "public.shell-script",
    tags: "",
    filename_extension: "csh",
    mime_type: "csh",
    description: "C Shell script",
};
pub const PUBLIC_KSH_SCRIPT: UTType = UTType {
    identifier: "public.ksh-script",
    conforms_to: "public.shell-script",
    tags: "",
    filename_extension: "ksh",
    mime_type: "ksh",
    description: "Korn Shell script",
};
pub const PUBLIC_TCSH_SCRIPT: UTType = UTType {
    identifier: "public.tcsh-script",
    conforms_to: "public.shell-script",
    tags: "",
    filename_extension: "tcsh",
    mime_type: "tcsh",
    description: "Tenex C Shell script",
};
pub const PUBLIC_ZSH_SCRIPT: UTType = UTType {
    identifier: "public.zsh-script",
    conforms_to: "public.shell-script",
    tags: "",
    filename_extension: "zsh",
    mime_type: "zsh",
    description: "Z Shell script",
};
pub const PUBLIC_PERL_SCRIPT: UTType = UTType {
    identifier: "public.perl-script",
    conforms_to: "public.shell-script",
    tags: "",
    filename_extension: "pl|pm",
    mime_type: "pl|pm",
    description: "Perl script",
};
pub const PUBLIC_PYTHON_SCRIPT: UTType = UTType {
    identifier: "public.python-script",
    conforms_to: "public.shell-script",
    tags: "",
    filename_extension: "py",
    mime_type: "py",
    description: "Python script",
};
pub const PUBLIC_RUBY_SCRIPT: UTType = UTType {
    identifier: "public.ruby-script",
    conforms_to: "public.shell-script",
    tags: "",
    filename_extension: "rb|rbw",
    mime_type: "rb|rbw",
    description: "Ruby script",
};
pub const PUBLIC_PHP_SCRIPT: UTType = UTType {
    identifier: "public.php-script",
    conforms_to: "public.shell-script",
    tags: "",
    filename_extension: "php|php3|php4|ph3|ph4|phtml",
    mime_type: "php|php3|php4|ph3|ph4|phtml",
    description: "PHP script",
};
pub const COM_SUN_JAVA_WEB_START: UTType = UTType {
    identifier: "com.sun.java-web-start",
    conforms_to: "public.xml",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Java web start",
};
pub const PUBLIC_MAKE_SOURCE: UTType = UTType {
    identifier: "public.make-source",
    conforms_to: "public.script",
    tags: "",
    filename_extension: "make|mak|gmk|mk",
    mime_type: "make|mak|gmk|mk",
    description: "Makefile",
};
pub const PUBLIC_IMAGE: UTType = UTType {
    identifier: "public.image",
    conforms_to: "public.data|public.content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "image",
};
pub const COM_APPLE_LIVE_PHOTO: UTType = UTType {
    identifier: "com.apple.live-photo",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Live Photo",
};
pub const COM_APPLE_PRIVATE_LIVE_PHOTO_BUNDLE: UTType = UTType {
    identifier: "com.apple.private.live-photo-bundle",
    conforms_to: "com.apple.live-photo|com.apple.bundle|com.apple.package",
    tags: "",
    filename_extension: "pvt",
    mime_type: "pvt",
    description: "",
};
pub const PUBLIC_FAX: UTType = UTType {
    identifier: "public.fax",
    conforms_to: "public.image|public.message",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "fax",
};
pub const PUBLIC_CAMERA_RAW_IMAGE: UTType = UTType {
    identifier: "public.camera-raw-image",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "camera raw image",
};
pub const PUBLIC_JPEG: UTType = UTType {
    identifier: "public.jpeg",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "JPEG image",
};
pub const PUBLIC_JPEG_2000: UTType = UTType {
    identifier: "public.jpeg-2000",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "JPEG 2000 image",
};
pub const PUBLIC_TIFF: UTType = UTType {
    identifier: "public.tiff",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "TIFF image",
};
pub const COM_APPLE_PICT: UTType = UTType {
    identifier: "com.apple.pict",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "QuickDraw picture",
};
pub const COM_APPLE_MACPAINT_IMAGE: UTType = UTType {
    identifier: "com.apple.macpaint-image",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "pntg",
    mime_type: "pntg",
    description: "MacPaint image",
};
pub const PUBLIC_PNG: UTType = UTType {
    identifier: "public.png",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PNG image",
};
pub const PUBLIC_SVG_IMAGE: UTType = UTType {
    identifier: "public.svg-image",
    conforms_to: "public.image|public.xml",
    tags: "",
    filename_extension: "svg|svgz",
    mime_type: "svg|svgz",
    description: "SVG image",
};
pub const COM_APPLE_QUICKTIME_IMAGE: UTType = UTType {
    identifier: "com.apple.quicktime-image",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "QuickTime image",
};
pub const COM_APPLE_ICNS: UTType = UTType {
    identifier: "com.apple.icns",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Apple icon image",
};
pub const PUBLIC_XBITMAP_IMAGE: UTType = UTType {
    identifier: "public.xbitmap-image",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "X bitmap image",
};
pub const PUBLIC_MPO_IMAGE: UTType = UTType {
    identifier: "public.mpo-image",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Multiple Picture Object image",
};
pub const CA_MCGILL_MNI_BIC_MNC: UTType = UTType {
    identifier: "ca.mcgill.mni.bic.mnc",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "mnc|minc",
    mime_type: "mnc|minc",
    description: "MINC Image",
};
pub const ORG_NEMA_DICOM: UTType = UTType {
    identifier: "org.nema.dicom",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "dcm|dicom",
    mime_type: "dcm|dicom",
    description: "DICOM",
};
pub const GOV_NIH_NIFTI_1: UTType = UTType {
    identifier: "gov.nih.nifti-1",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "nii",
    mime_type: "nii",
    description: "NIfTI-1",
};
pub const PUBLIC_AUDIOVISUAL_CONTENT: UTType = UTType {
    identifier: "public.audiovisual-content",
    conforms_to: "public.data|public.content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "audiovisual content",
};
pub const PUBLIC_MOVIE: UTType = UTType {
    identifier: "public.movie",
    conforms_to: "public.audiovisual-content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "movie",
};
pub const PUBLIC_VIDEO: UTType = UTType {
    identifier: "public.video",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "video",
};
pub const PUBLIC_AUDIO: UTType = UTType {
    identifier: "public.audio",
    conforms_to: "public.audiovisual-content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "audio",
};
pub const COM_APPLE_QUICKTIME_MOVIE: UTType = UTType {
    identifier: "com.apple.quicktime-movie",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "QuickTime movie",
};
pub const PUBLIC_MPEG: UTType = UTType {
    identifier: "public.mpeg",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "mpg|mpeg|mpe|m75|m15",
    mime_type: "mpg|mpeg|mpe|m75|m15",
    description: "MPEG movie",
};
pub const PUBLIC_MPEG_2_VIDEO: UTType = UTType {
    identifier: "public.mpeg-2-video",
    conforms_to: "public.video",
    tags: "",
    filename_extension: "m2v",
    mime_type: "m2v",
    description: "MPEG-2 video",
};
pub const PUBLIC_MPEG_4: UTType = UTType {
    identifier: "public.mpeg-4",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "mp4|mpg4",
    mime_type: "mp4|mpg4",
    description: "MPEG-4 movie",
};
pub const COM_APPLE_M4V_VIDEO: UTType = UTType {
    identifier: "com.apple.m4v-video",
    conforms_to: "public.mpeg-4",
    tags: "",
    filename_extension: "m4v",
    mime_type: "m4v",
    description: "Apple MPEG-4 movie",
};
pub const COM_APPLE_PROTECTED_MPEG_4_VIDEO: UTType = UTType {
    identifier: "com.apple.protected-mpeg-4-video",
    conforms_to: "com.apple.m4v-video",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "protected MPEG-4 movie",
};
pub const PUBLIC_DV_MOVIE: UTType = UTType {
    identifier: "public.dv-movie",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "dv|dif",
    mime_type: "dv|dif",
    description: "DV movie",
};
pub const PUBLIC_AVI: UTType = UTType {
    identifier: "public.avi",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "avi|vfw",
    mime_type: "avi|vfw",
    description: "AVI movie",
};
pub const PUBLIC_3GPP: UTType = UTType {
    identifier: "public.3gpp",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "3gp|3gpp|sdv",
    mime_type: "3gp|3gpp|sdv",
    description: "3GPP movie",
};
pub const PUBLIC_3GPP2: UTType = UTType {
    identifier: "public.3gpp2",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "3g2|3gp2",
    mime_type: "3g2|3gp2",
    description: "3GPP2 movie",
};
pub const PUBLIC_FLC_ANIMATION: UTType = UTType {
    identifier: "public.flc-animation",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "flc|fli|cel",
    mime_type: "flc|fli|cel",
    description: "FLC animation",
};
pub const PUBLIC_MPEG_2_TRANSPORT_STREAM: UTType = UTType {
    identifier: "public.mpeg-2-transport-stream",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "ts",
    mime_type: "ts",
    description: "MPEG-2 Transport Stream",
};
pub const PUBLIC_AUDIOVISUAL_CONTENT_COLLECTION: UTType = UTType {
    identifier: "public.audiovisual-content-collection",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Audiovisual Collection",
};
pub const PUBLIC_AVCHD_COLLECTION: UTType = UTType {
    identifier: "public.avchd-collection",
    conforms_to: "com.apple.package|public.audiovisual-content-collection",
    tags: "",
    filename_extension: "avchd",
    mime_type: "avchd",
    description: "AVCHD Collection",
};
pub const COM_APPLE_AUDIO_UNIT_PRESET: UTType = UTType {
    identifier: "com.apple.audio-unit-preset",
    conforms_to: "com.apple.xml-property-list",
    tags: "",
    filename_extension: "aupreset",
    mime_type: "aupreset",
    description: "audio unit preset",
};
pub const PUBLIC_MP2: UTType = UTType {
    identifier: "public.mp2",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "MP2 audio",
};
pub const PUBLIC_MP3: UTType = UTType {
    identifier: "public.mp3",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "mp3|mpga",
    mime_type: "mp3|mpga",
    description: "MP3 audio",
};
pub const PUBLIC_PLAYLIST: UTType = UTType {
    identifier: "public.playlist",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "playlist",
};
pub const PUBLIC_M3U_PLAYLIST: UTType = UTType {
    identifier: "public.m3u-playlist",
    conforms_to: "public.plain-text|public.playlist",
    tags: "",
    filename_extension: "m3u|m3u8",
    mime_type: "m3u|m3u8",
    description: "M3U Playlist",
};
pub const PUBLIC_PLS_PLAYLIST: UTType = UTType {
    identifier: "public.pls-playlist",
    conforms_to: "public.text|public.playlist",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PLS Playlist",
};
pub const PUBLIC_MPEG_4_AUDIO: UTType = UTType {
    identifier: "public.mpeg-4-audio",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "mp4|mpg4",
    mime_type: "mp4|mpg4",
    description: "MPEG-4 audio",
};
pub const COM_APPLE_M4A_AUDIO: UTType = UTType {
    identifier: "com.apple.m4a-audio",
    conforms_to: "public.mpeg-4-audio",
    tags: "",
    filename_extension: "m4a",
    mime_type: "m4a",
    description: "Apple MPEG-4 audio",
};
pub const COM_APPLE_MPEG_4_RINGTONE: UTType = UTType {
    identifier: "com.apple.mpeg-4-ringtone",
    conforms_to: "public.mpeg-4-audio",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Ringtone",
};
pub const COM_APPLE_PROTECTED_MPEG_4_AUDIO: UTType = UTType {
    identifier: "com.apple.protected-mpeg-4-audio",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "m4p",
    mime_type: "m4p",
    description: "protected MPEG-4 audio",
};
pub const COM_APPLE_PROTECTED_MPEG_4_AUDIO_B: UTType = UTType {
    identifier: "com.apple.protected-mpeg-4-audio-b",
    conforms_to: "com.apple.protected-mpeg-4-audio",
    tags: "",
    filename_extension: "m4b",
    mime_type: "m4b",
    description: "protected MPEG-4 audio",
};
pub const PUBLIC_ULAW_AUDIO: UTType = UTType {
    identifier: "public.ulaw-audio",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "uLaw audio",
};
pub const PUBLIC_AU_AUDIO: UTType = UTType {
    identifier: "public.au-audio",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "au|snd",
    mime_type: "au|snd",
    description: "AU audio",
};
pub const PUBLIC_AIFC_AUDIO: UTType = UTType {
    identifier: "public.aifc-audio",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "aifc",
    mime_type: "aifc",
    description: "AIFF-C audio",
};
pub const PUBLIC_AIFF_AUDIO: UTType = UTType {
    identifier: "public.aiff-audio",
    conforms_to: "public.aifc-audio",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "AIFF audio",
};
pub const PUBLIC_CDDA_AUDIO: UTType = UTType {
    identifier: "public.cdda-audio",
    conforms_to: "public.aifc-audio",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "CDDA audio",
};
pub const PUBLIC_MIDI_AUDIO: UTType = UTType {
    identifier: "public.midi-audio",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "midi|mid|smf|kar",
    mime_type: "midi|mid|smf|kar",
    description: "MIDI audio",
};
pub const PUBLIC_DOWNLOADABLE_SOUND: UTType = UTType {
    identifier: "public.downloadable-sound",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "dls",
    mime_type: "dls",
    description: "downloadable sound",
};
pub const COM_APPLE_COREAUDIO_FORMAT: UTType = UTType {
    identifier: "com.apple.coreaudio-format",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Apple CoreAudio format",
};
pub const PUBLIC_AC3_AUDIO: UTType = UTType {
    identifier: "public.ac3-audio",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "ac3",
    mime_type: "ac3",
    description: "AC-3 audio",
};
pub const PUBLIC_ENHANCED_AC3_AUDIO: UTType = UTType {
    identifier: "public.enhanced-ac3-audio",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "eac3|ec3",
    mime_type: "eac3|ec3",
    description: "Enhanced AC-3 audio",
};
pub const ORG_3GPP_ADAPTIVE_MULTI_RATE_AUDIO: UTType = UTType {
    identifier: "org.3gpp.adaptive-multi-rate-audio",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "amr",
    mime_type: "amr",
    description: "Adaptive Multi-rate audio",
};
pub const PUBLIC_AAC_AUDIO: UTType = UTType {
    identifier: "public.aac-audio",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "aac|adts",
    mime_type: "aac|adts",
    description: "AAC audio",
};
pub const COM_AUDIBLE_AA_AUDIO: UTType = UTType {
    identifier: "com.audible.aa-audio",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Audible.com Audio",
};
pub const COM_AUDIBLE_AA_AUDIOBOOK: UTType = UTType {
    identifier: "com.audible.aa-audiobook",
    conforms_to: "com.audible.aa-audio",
    tags: "",
    filename_extension: "aa",
    mime_type: "aa",
    description: "Audible.com Audiobook",
};
pub const COM_AUDIBLE_AAX_AUDIOBOOK: UTType = UTType {
    identifier: "com.audible.aax-audiobook",
    conforms_to: "com.audible.aa-audio",
    tags: "",
    filename_extension: "aax",
    mime_type: "aax",
    description: "Audible.com Audiobook",
};
pub const COM_SONY_WAVE64: UTType = UTType {
    identifier: "com.sony.wave64",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Wave64 Audio",
};
pub const PUBLIC_FONT: UTType = UTType {
    identifier: "public.font",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "font",
};
pub const PUBLIC_TRUETYPE_FONT: UTType = UTType {
    identifier: "public.truetype-font",
    conforms_to: "public.font",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "TrueType font",
};
pub const COM_ADOBE_POSTSCRIPT_FONT: UTType = UTType {
    identifier: "com.adobe.postscript-font",
    conforms_to: "public.font",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PostScript font",
};
pub const COM_APPLE_TRUETYPE_DATAFORK_SUITCASE_FONT: UTType = UTType {
    identifier: "com.apple.truetype-datafork-suitcase-font",
    conforms_to: "public.truetype-font",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "TrueType datafork font",
};
pub const PUBLIC_OPENTYPE_FONT: UTType = UTType {
    identifier: "public.opentype-font",
    conforms_to: "public.font",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PostScript OpenType font",
};
pub const PUBLIC_OPENTYPE_COLLECTION_FONT: UTType = UTType {
    identifier: "public.opentype-collection-font",
    conforms_to: "public.opentype-font",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PostScript OpenType collection font",
};
pub const PUBLIC_TRUETYPE_TTF_FONT: UTType = UTType {
    identifier: "public.truetype-ttf-font",
    conforms_to: "public.truetype-font",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "TrueType OpenType font",
};
pub const PUBLIC_TRUETYPE_COLLECTION_FONT: UTType = UTType {
    identifier: "public.truetype-collection-font",
    conforms_to: "public.truetype-font",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "TrueType collection font",
};
pub const COM_APPLE_FONT_SUITCASE: UTType = UTType {
    identifier: "com.apple.font-suitcase",
    conforms_to: "public.font",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "font suitcase",
};
pub const COM_ADOBE_POSTSCRIPT_LWFN_FONT: UTType = UTType {
    identifier: "com.adobe.postscript-lwfn-font",
    conforms_to: "com.adobe.postscript-font",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PostScript Type 1 outline font",
};
pub const COM_ADOBE_POSTSCRIPT_PFB_FONT: UTType = UTType {
    identifier: "com.adobe.postscript-pfb-font",
    conforms_to: "com.adobe.postscript-font",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PostScript Type 1 outline font",
};
pub const COM_ADOBE_POSTSCRIPT_PFA_FONT: UTType = UTType {
    identifier: "com.adobe.postscript-pfa-font",
    conforms_to: "com.adobe.postscript-font",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PostScript Type 1 outline font",
};
pub const COM_APPLE_PROFILE_FONT_ICON: UTType = UTType {
    identifier: "com.apple.profile-font-icon",
    conforms_to: "public.item",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Profile Font",
};
pub const COM_APPLE_APPLESCRIPT_TEXT: UTType = UTType {
    identifier: "com.apple.applescript.text",
    conforms_to: "public.script",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "AppleScript text",
};
pub const COM_APPLE_APPLESCRIPT_SCRIPT: UTType = UTType {
    identifier: "com.apple.applescript.script",
    conforms_to: "public.data|public.script",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "AppleScript",
};
pub const COM_APPLE_APPLESCRIPT_SCRIPT_BUNDLE: UTType = UTType {
    identifier: "com.apple.applescript.script-bundle",
    conforms_to: "com.apple.bundle|com.apple.package",
    tags: "",
    filename_extension: "scptd",
    mime_type: "scptd",
    description: "Script bundle",
};
pub const COM_APPLE_SCRIPTING_DEFINITION: UTType = UTType {
    identifier: "com.apple.scripting-definition",
    conforms_to: "public.xml",
    tags: "",
    filename_extension: "sdef",
    mime_type: "sdef",
    description: "Scripting Definition",
};
pub const PUBLIC_FOLDER: UTType = UTType {
    identifier: "public.folder",
    conforms_to: "public.directory",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "folder",
};
pub const COM_APPLE_DROP_FOLDER: UTType = UTType {
    identifier: "com.apple.drop-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_APPLICATIONS_FOLDER: UTType = UTType {
    identifier: "com.apple.applications-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_SERVER_APPLICATIONS_FOLDER: UTType = UTType {
    identifier: "com.apple.server-applications-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_LIBRARY_FOLDER: UTType = UTType {
    identifier: "com.apple.library-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_DOCUMENT_TYPE_SYSTEM_FOLDER: UTType = UTType {
    identifier: "com.apple.document-type.system-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "System Folder",
};
pub const COM_APPLE_TRASH_EMPTY: UTType = UTType {
    identifier: "com.apple.trash-empty",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Trash",
};
pub const COM_APPLE_TRASH_FULL: UTType = UTType {
    identifier: "com.apple.trash-full",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Full Trash",
};
pub const COM_APPLE_SITES_FOLDER: UTType = UTType {
    identifier: "com.apple.sites-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Sites Folder",
};
pub const COM_APPLE_GROUPS_FOLDER: UTType = UTType {
    identifier: "com.apple.groups-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Groups Folder",
};
pub const COM_APPLE_SERVERS_FOLDER: UTType = UTType {
    identifier: "com.apple.servers-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_DESKTOP_FOLDER: UTType = UTType {
    identifier: "com.apple.desktop-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Desktop Folder",
};
pub const COM_APPLE_DOCUMENTS_FOLDER: UTType = UTType {
    identifier: "com.apple.documents-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Documents Folder",
};
pub const COM_APPLE_DOWNLOADS_FOLDER: UTType = UTType {
    identifier: "com.apple.downloads-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Downloads Folder",
};
pub const COM_APPLE_MOVIE_FOLDER: UTType = UTType {
    identifier: "com.apple.movie-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Movie Folder",
};
pub const COM_APPLE_MUSIC_FOLDER: UTType = UTType {
    identifier: "com.apple.music-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Music Folder",
};
pub const COM_APPLE_PICTURES_FOLDER: UTType = UTType {
    identifier: "com.apple.pictures-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Pictures Folder",
};
pub const COM_APPLE_PUBLIC_FOLDER: UTType = UTType {
    identifier: "com.apple.public-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Public Folder",
};
pub const COM_APPLE_HOME_FOLDER: UTType = UTType {
    identifier: "com.apple.home-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Home Folder",
};
pub const COM_APPLE_DEVELOPER_FOLDER: UTType = UTType {
    identifier: "com.apple.developer-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Developer Folder",
};
pub const COM_APPLE_USERS_FOLDER: UTType = UTType {
    identifier: "com.apple.users-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Users Folder",
};
pub const COM_APPLE_UTILITIES_FOLDER: UTType = UTType {
    identifier: "com.apple.utilities-folder",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Utilities Folder",
};
pub const PUBLIC_VOLUME: UTType = UTType {
    identifier: "public.volume",
    conforms_to: "public.folder",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "volume",
};
pub const PUBLIC_FILE_SHAREPOINT: UTType = UTType {
    identifier: "public.file-sharepoint",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "sharepoint",
};
pub const COM_APPLE_NETWORK_NEIGHBORHOOD: UTType = UTType {
    identifier: "com.apple.network-neighborhood",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "neighborhood",
};
pub const COM_APPLE_DOT_MAC: UTType = UTType {
    identifier: "com.apple.dot-mac",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: ".Mac",
};
pub const COM_APPLE_IDISK: UTType = UTType {
    identifier: "com.apple.idisk",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iDisk",
};
pub const COM_APPLE_USER_IDISK: UTType = UTType {
    identifier: "com.apple.user-idisk",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "User iDisk",
};
pub const COM_APPLE_PACKAGE: UTType = UTType {
    identifier: "com.apple.package",
    conforms_to: "public.directory",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Package",
};
pub const COM_APPLE_BUNDLE: UTType = UTType {
    identifier: "com.apple.bundle",
    conforms_to: "public.directory",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "bundle",
};
pub const COM_APPLE_GENERIC_BUNDLE: UTType = UTType {
    identifier: "com.apple.generic-bundle",
    conforms_to: "com.apple.bundle|com.apple.package",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "bundle",
};
pub const COM_APPLE_SYSTEMPREFERENCE_PREFPANE: UTType = UTType {
    identifier: "com.apple.systempreference.prefpane",
    conforms_to: "com.apple.package|com.apple.bundle",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "System Preference pane",
};
pub const COM_APPLE_SYSTEMPREFERENCE_SCREEN_SAVER: UTType = UTType {
    identifier: "com.apple.systempreference.screen-saver",
    conforms_to: "com.apple.package|com.apple.bundle",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Screen Saver",
};
pub const COM_APPLE_SYSTEMPREFERENCE_SCREEN_SLIDE_SAVER: UTType = UTType {
    identifier: "com.apple.systempreference.screen-slide-saver",
    conforms_to: "com.apple.package|com.apple.bundle",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Screen Slide Saver",
};
pub const COM_APPLE_MENU_EXTRA: UTType = UTType {
    identifier: "com.apple.menu-extra",
    conforms_to: "com.apple.package|com.apple.bundle",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "System Menu Extra",
};
pub const COM_APPLE_LOCALIZABLE_NAME_BUNDLE: UTType = UTType {
    identifier: "com.apple.localizable-name-bundle",
    conforms_to: "com.apple.bundle",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_FRAMEWORK: UTType = UTType {
    identifier: "com.apple.framework",
    conforms_to: "com.apple.bundle",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "framework",
};
pub const COM_APPLE_APPLICATION_BUNDLE: UTType = UTType {
    identifier: "com.apple.application-bundle",
    conforms_to: "com.apple.application|com.apple.localizable-name-bundle|com.apple.package",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_APPLICATION_FILE: UTType = UTType {
    identifier: "com.apple.application-file",
    conforms_to: "com.apple.application|public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_DEPRECATED_APPLICATION_FILE: UTType = UTType {
    identifier: "com.apple.deprecated-application-file",
    conforms_to: "com.apple.application-file",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_FLAT_RTFD: UTType = UTType {
    identifier: "com.apple.flat-rtfd",
    conforms_to: "public.data|public.composite-content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "rich text with attachments (RTFD) data",
};
pub const COM_APPLE_INSTALLER_PACKAGE: UTType = UTType {
    identifier: "com.apple.installer-package",
    conforms_to: "com.apple.bundle|com.apple.package",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Installer package",
};
pub const COM_APPLE_INSTALLER_DISTRIBUTION_PACKAGE: UTType = UTType {
    identifier: "com.apple.installer-distribution-package",
    conforms_to: "com.apple.bundle|com.apple.package",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Installer distribution",
};
pub const COM_APPLE_INSTALLER_META_PACKAGE: UTType = UTType {
    identifier: "com.apple.installer-meta-package",
    conforms_to: "com.apple.bundle|com.apple.package",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Installer package",
};
pub const COM_APPLE_INTELLIGENTSUGGESTIONS_ASSETS: UTType = UTType {
    identifier: "com.apple.intelligentsuggestions.assets",
    conforms_to: "com.apple.bundle|com.apple.package",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Intelligent Suggestions assets",
};
pub const COM_APPLE_RTFD: UTType = UTType {
    identifier: "com.apple.rtfd",
    conforms_to: "com.apple.package|public.composite-content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "rich text with attachments (RTFD)",
};
pub const COM_APPLE_APPLICATION_PLACEHOLDER: UTType = UTType {
    identifier: "com.apple.application-placeholder",
    conforms_to: "com.apple.application-bundle",
    tags: "",
    filename_extension: "placeholder",
    mime_type: "placeholder",
    description: "",
};
pub const COM_APPLE_SERVICE_APPLICATION: UTType = UTType {
    identifier: "com.apple.service-application",
    conforms_to: "com.apple.application-bundle",
    tags: "",
    filename_extension: "service",
    mime_type: "service",
    description: "",
};
pub const COM_APPLE_PLUGIN: UTType = UTType {
    identifier: "com.apple.plugin",
    conforms_to: "com.apple.bundle|com.apple.package",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "plug-in",
};
pub const COM_APPLE_XPC_SERVICE: UTType = UTType {
    identifier: "com.apple.xpc-service",
    conforms_to: "com.apple.bundle|com.apple.package",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "XPC Service",
};
pub const COM_APPLE_KERNEL_EXTENSION: UTType = UTType {
    identifier: "com.apple.kernel-extension",
    conforms_to: "com.apple.bundle|com.apple.package",
    tags: "",
    filename_extension: "kext",
    mime_type: "kext",
    description: "Kernel Extension",
};
pub const COM_APPLE_APPLICATION_AND_SYSTEM_EXTENSION: UTType = UTType {
    identifier: "com.apple.application-and-system-extension",
    conforms_to: "com.apple.xpc-service",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Application and System Extension",
};
pub const COM_APPLE_PLUGINKIT: UTType = UTType {
    identifier: "com.apple.pluginkit",
    conforms_to: "com.apple.bundle|com.apple.package",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PlugInKit plug-in",
};
pub const COM_APPLE_WEBKIT_PLUGIN: UTType = UTType {
    identifier: "com.apple.webkit-plugin",
    conforms_to: "com.apple.plugin",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "WebKit plug-in",
};
pub const COM_APPLE_METADATA_IMPORTER: UTType = UTType {
    identifier: "com.apple.metadata-importer",
    conforms_to: "com.apple.plugin",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Spotlight importer",
};
pub const COM_APPLE_QUICKLOOK_GENERATOR: UTType = UTType {
    identifier: "com.apple.quicklook-generator",
    conforms_to: "com.apple.plugin",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "QuickLook preview generator",
};
pub const COM_APPLE_DASHBOARD_WIDGET: UTType = UTType {
    identifier: "com.apple.dashboard-widget",
    conforms_to: "com.apple.localizable-name-bundle|com.apple.package",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Dashboard widget",
};
pub const COM_APPLE_DRIVER_EXTENSION: UTType = UTType {
    identifier: "com.apple.driver-extension",
    conforms_to: "com.apple.localizable-name-bundle|com.apple.package",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Driver Extension",
};
pub const COM_APPLE_SYSTEM_EXTENSION: UTType = UTType {
    identifier: "com.apple.system-extension",
    conforms_to: "com.apple.localizable-name-bundle|com.apple.package",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "System Extension",
};
pub const COM_APPLE_PPP_PLUG_IN: UTType = UTType {
    identifier: "com.apple.ppp-plug-in",
    conforms_to: "com.apple.plugin",
    tags: "",
    filename_extension: "ppp",
    mime_type: "ppp",
    description: "PPP Plug-in",
};
pub const COM_APPLE_FILE_SYSTEM_PLUG_IN: UTType = UTType {
    identifier: "com.apple.file-system-plug-in",
    conforms_to: "com.apple.plugin",
    tags: "",
    filename_extension: "fs",
    mime_type: "fs",
    description: "File System Plug-in",
};
pub const COM_APPLE_DATA_CONTAINER: UTType = UTType {
    identifier: "com.apple.data-container",
    conforms_to: "public.folder|com.apple.localizable-name-bundle",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const ORG_OPENXMLFORMATS_OPENXML: UTType = UTType {
    identifier: "org.openxmlformats.openxml",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Office Open XML",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT: UTType = UTType {
    identifier: "org.oasis-open.opendocument",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Open Document",
};
pub const COM_RSA_PKCS_12: UTType = UTType {
    identifier: "com.rsa.pkcs-12",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "p12|pfx",
    mime_type: "p12|pfx",
    description: "personal information exchange (PKCS#12)",
};
pub const PUBLIC_X509_CERTIFICATE: UTType = UTType {
    identifier: "public.x509-certificate",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "cer|der|crt|pem",
    mime_type: "cer|der|crt|pem",
    description: "certificate (X.509)",
};
pub const COM_APPLE_ALERT: UTType = UTType {
    identifier: "com.apple.alert",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Alert",
};
pub const COM_APPLE_ALERT_NOTE: UTType = UTType {
    identifier: "com.apple.alert-note",
    conforms_to: "com.apple.alert",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Alert Note",
};
pub const COM_APPLE_ALERT_CAUTION: UTType = UTType {
    identifier: "com.apple.alert-caution",
    conforms_to: "com.apple.alert",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Alert Caution",
};
pub const COM_APPLE_ALERT_STOP: UTType = UTType {
    identifier: "com.apple.alert-stop",
    conforms_to: "com.apple.alert",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Alert Stop",
};
pub const COM_APPLE_WEBARCHIVE: UTType = UTType {
    identifier: "com.apple.webarchive",
    conforms_to: "public.data|public.composite-content",
    tags: "",
    filename_extension: "webarchive",
    mime_type: "webarchive",
    description: "web archive",
};
pub const ORG_IDPF_EPUB_CONTAINER: UTType = UTType {
    identifier: "org.idpf.epub-container",
    conforms_to: "public.data|public.composite-content",
    tags: "",
    filename_extension: "epub",
    mime_type: "epub",
    description: "EPUB publication",
};
pub const COM_APPLE_LOCALIZED_PDF_BUNDLE: UTType = UTType {
    identifier: "com.apple.localized-pdf-bundle",
    conforms_to: "com.apple.localizable-name-bundle|com.apple.package|public.composite-content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "localized PDF",
};
pub const ORG_AAFASSOCIATION_ADVANCED_AUTHORING_FORMAT: UTType = UTType {
    identifier: "org.aafassociation.advanced-authoring-format",
    conforms_to: "public.data|public.composite-content",
    tags: "",
    filename_extension: "aaf",
    mime_type: "aaf",
    description: "Advanced Authoring Format",
};
pub const COM_APPLE_TXN_TEXT_MULTIMEDIA_DATA: UTType = UTType {
    identifier: "com.apple.txn.text-multimedia-data",
    conforms_to: "public.data|public.composite-content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "text with multimedia",
};
pub const COM_APPLE_COLORSYNC_PROFILE: UTType = UTType {
    identifier: "com.apple.colorsync-profile",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "icc|icm|pf",
    mime_type: "icc|icm|pf",
    description: "ColorSync profile",
};
pub const COM_APPLE_PROFILE_BACKGROUND_COLOR: UTType = UTType {
    identifier: "com.apple.profile-background-color",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Profile Background Color",
};
pub const COM_APPLE_PROFILE_FONT_AND_COLOR: UTType = UTType {
    identifier: "com.apple.profile-font-and-color",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Profile Font And Color",
};
pub const COM_APPLE_COLOR_FILE: UTType = UTType {
    identifier: "com.apple.color-file",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "clr|ccl",
    mime_type: "clr|ccl",
    description: "Color File",
};
pub const COM_APPLE_INK_INKTEXT: UTType = UTType {
    identifier: "com.apple.ink.inktext",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "writing",
};
pub const COM_APPLE_MOBILECONFIG: UTType = UTType {
    identifier: "com.apple.mobileconfig",
    conforms_to: "public.xml",
    tags: "",
    filename_extension: "mobileconfig|mobile",
    mime_type: "mobileconfig|mobile",
    description: "mobile configuration",
};
pub const COM_APPLE_PROVISIONPROFILE: UTType = UTType {
    identifier: "com.apple.provisionprofile",
    conforms_to: "public.xml",
    tags: "",
    filename_extension: "provisionprofile",
    mime_type: "provisionprofile",
    description: "provision profile",
};
pub const COM_APPLE_CONFIGPROFILE: UTType = UTType {
    identifier: "com.apple.configprofile",
    conforms_to: "public.xml",
    tags: "",
    filename_extension: "configprofile",
    mime_type: "configprofile",
    description: "configuration profile",
};
pub const COM_APPLE_USER: UTType = UTType {
    identifier: "com.apple.user",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "User",
};
pub const COM_APPLE_GUEST_USER: UTType = UTType {
    identifier: "com.apple.guest-user",
    conforms_to: "com.apple.user",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Guest User",
};
pub const COM_APPLE_HELP_DOCUMENT: UTType = UTType {
    identifier: "com.apple.help-document",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Help Document",
};
pub const COM_APPLE_USER_GROUP: UTType = UTType {
    identifier: "com.apple.user-group",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "User Group",
};
pub const COM_APPLE_USER_UNKNOWN: UTType = UTType {
    identifier: "com.apple.user-unknown",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "User Unknown",
};
pub const COM_APPLE_AIRDROP: UTType = UTType {
    identifier: "com.apple.airdrop",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "AirDrop",
};
pub const COM_APPLE_BONJOUR: UTType = UTType {
    identifier: "com.apple.bonjour",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Bonjour",
};
pub const COM_APPLE_NOTIFICATIONS: UTType = UTType {
    identifier: "com.apple.notifications",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Notifications",
};
pub const COM_APPLE_MOBILEPROVISION: UTType = UTType {
    identifier: "com.apple.mobileprovision",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "mobileprovision",
    mime_type: "mobileprovision",
    description: "mobile provision",
};
pub const COM_APPLE_PKPASS: UTType = UTType {
    identifier: "com.apple.pkpass",
    conforms_to: "com.apple.bundle|com.apple.package",
    tags: "",
    filename_extension: "pkpass",
    mime_type: "pkpass",
    description: "Pass",
};
pub const COM_APPLE_PKPASS_DATA: UTType = UTType {
    identifier: "com.apple.pkpass-data",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "pkpass",
    mime_type: "pkpass",
    description: "Pass",
};
pub const COM_APPLE_PKPASSES_DATA: UTType = UTType {
    identifier: "com.apple.pkpasses-data",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "pkpasses",
    mime_type: "pkpasses",
    description: "Passes",
};
pub const COM_APPLE_WATCHFACE: UTType = UTType {
    identifier: "com.apple.watchface",
    conforms_to: "public.data|public.content",
    tags: "",
    filename_extension: "watchface",
    mime_type: "watchface",
    description: "Watchface",
};
pub const PUBLIC_DEVICE: UTType = UTType {
    identifier: "public.device",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Device",
};
pub const COM_APPLE_VIRTUAL_MACHINE: UTType = UTType {
    identifier: "com.apple.virtual-machine",
    conforms_to: "com.apple.mac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Virtual Machine",
};
pub const PUBLIC_DISPLAY: UTType = UTType {
    identifier: "public.display",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Display",
};
pub const PUBLIC_SPEAKER: UTType = UTType {
    identifier: "public.speaker",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Speaker",
};
pub const PUBLIC_COMPUTER: UTType = UTType {
    identifier: "public.computer",
    conforms_to: "public.device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Computer",
};
pub const PUBLIC_GENERIC_PC: UTType = UTType {
    identifier: "public.generic-pc",
    conforms_to: "public.computer",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PC",
};
pub const COM_APPLE_DEVICE: UTType = UTType {
    identifier: "com.apple.device",
    conforms_to: "public.device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Apple device",
};
pub const COM_APPLE_MAC: UTType = UTType {
    identifier: "com.apple.mac",
    conforms_to: "public.computer|com.apple.device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Mac",
};
pub const COM_APPLE_MAC_LAPTOP: UTType = UTType {
    identifier: "com.apple.mac.laptop",
    conforms_to: "com.apple.mac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Laptop",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc",
    conforms_to: "com.apple.macbookpro",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-silver",
    conforms_to: "com.apple.macbookpro-13-retina-usbc",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-space-gray",
    conforms_to: "com.apple.macbookpro-13-retina-usbc",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-2017",
    conforms_to: "com.apple.macbookpro-13-retina-usbc",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-silver-2017",
    conforms_to: "com.apple.macbookpro-13-retina-usbc-silver",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-space-gray-2017",
    conforms_to: "com.apple.macbookpro-13-retina-usbc-space-gray",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid",
    conforms_to: "com.apple.macbookpro",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver",
    conforms_to: "com.apple.macbookpro-13-retina-touchid",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray",
    conforms_to: "com.apple.macbookpro-13-retina-touchid",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-2017",
    conforms_to: "com.apple.macbookpro-13-retina-touchid",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver-2017",
    conforms_to: "com.apple.macbookpro-13-retina-touchid-silver",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray-2017",
    conforms_to: "com.apple.macbookpro-13-retina-touchid-space-gray",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-2018",
    conforms_to: "com.apple.macbookpro-13-retina-touchid",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver-2018",
    conforms_to:
        "com.apple.macbookpro-13-retina-touchid-silver|com.apple.macbookpro-13-retina-touchid-2018",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray-2018",
    conforms_to: "com.apple.macbookpro-13-retina-touchid-space-gray|com.apple.macbookpro-13-retina-touchid-2018",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: ""
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_2019: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-2019",
    conforms_to: "com.apple.macbookpro-13-retina-usbc",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SILVER_2019: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-silver-2019",
    conforms_to:
        "com.apple.macbookpro-13-retina-usbc-silver|com.apple.macbookpro-13-retina-usbc-2019",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_USBC_SPACE_GRAY_2019: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-usbc-space-gray-2019",
    conforms_to:
        "com.apple.macbookpro-13-retina-usbc-space-gray|com.apple.macbookpro-13-retina-usbc-2019",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid",
    conforms_to: "com.apple.macbookpro",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-silver",
    conforms_to: "com.apple.macbookpro-15-retina-touchid",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-space-gray",
    conforms_to: "com.apple.macbookpro-15-retina-touchid",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-2017",
    conforms_to: "com.apple.macbookpro-15-retina-touchid",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-silver-2017",
    conforms_to: "com.apple.macbookpro-15-retina-touchid-silver",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY_2017: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-space-gray-2017",
    conforms_to: "com.apple.macbookpro-15-retina-touchid-space-gray",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-2018",
    conforms_to: "com.apple.macbookpro-15-retina-touchid",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SILVER_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-silver-2018",
    conforms_to:
        "com.apple.macbookpro-15-retina-touchid-silver|com.apple.macbookpro-15-retina-touchid-2018",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_TOUCHID_SPACE_GRAY_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-touchid-space-gray-2018",
    conforms_to: "com.apple.macbookpro-15-retina-touchid-space-gray|com.apple.macbookpro-15-retina-touchid-2018",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: ""
};
pub const COM_APPLE_MACBOOKPRO_15_LATE_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-late-2018",
    conforms_to: "com.apple.macbookpro-15-retina-touchid",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_SILVER_LATE_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-silver-late-2018",
    conforms_to: "com.apple.macbookpro-15-retina-touchid-silver|com.apple.macbookpro-15-late-2018",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_SPACE_GRAY_LATE_2018: UTType = UTType {
    identifier: "com.apple.macbookpro-15-space-gray-late-2018",
    conforms_to:
        "com.apple.macbookpro-15-retina-touchid-space-gray|com.apple.macbookpro-15-late-2018",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MAC_TOWER: UTType = UTType {
    identifier: "com.apple.mac.tower",
    conforms_to: "com.apple.mac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Tower",
};
pub const COM_APPLE_MAC_RACKMOUNT: UTType = UTType {
    identifier: "com.apple.mac.rackmount",
    conforms_to: "com.apple.mac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Rack Mount",
};
pub const COM_APPLE_POWERBOOK: UTType = UTType {
    identifier: "com.apple.powerbook",
    conforms_to: "com.apple.mac.laptop",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PowerBook",
};
pub const COM_APPLE_POWERBOOK_G4_TITANIUM: UTType = UTType {
    identifier: "com.apple.powerbook-g4-titanium",
    conforms_to: "com.apple.powerbook",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PowerBook G4",
};
pub const COM_APPLE_POWERBOOK_G4_12: UTType = UTType {
    identifier: "com.apple.powerbook-g4-12",
    conforms_to: "com.apple.powerbook",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PowerBook G4",
};
pub const COM_APPLE_POWERBOOK_G4_15: UTType = UTType {
    identifier: "com.apple.powerbook-g4-15",
    conforms_to: "com.apple.powerbook",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PowerBook G4",
};
pub const COM_APPLE_POWERBOOK_G4_17: UTType = UTType {
    identifier: "com.apple.powerbook-g4-17",
    conforms_to: "com.apple.powerbook",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PowerBook G4",
};
pub const COM_APPLE_IBOOK_G4: UTType = UTType {
    identifier: "com.apple.ibook-g4",
    conforms_to: "com.apple.mac.laptop",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iBook G4",
};
pub const COM_APPLE_POWERMAC: UTType = UTType {
    identifier: "com.apple.powermac",
    conforms_to: "com.apple.mac.tower",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Power Mac",
};
pub const COM_APPLE_POWERMAC_G4_QUICKSILVER: UTType = UTType {
    identifier: "com.apple.powermac-g4-quicksilver",
    conforms_to: "com.apple.powermac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Power Mac G4",
};
pub const COM_APPLE_POWERMAC_G4_MIRRORED_DRIVE_DOORS: UTType = UTType {
    identifier: "com.apple.powermac-g4-mirrored-drive-doors",
    conforms_to: "com.apple.powermac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Power Mac G4",
};
pub const COM_APPLE_POWERMAC_G5: UTType = UTType {
    identifier: "com.apple.powermac-g5",
    conforms_to: "com.apple.powermac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Power Mac G5",
};
pub const COM_APPLE_XSERVE: UTType = UTType {
    identifier: "com.apple.xserve",
    conforms_to: "com.apple.mac.rackmount",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Xserve",
};
pub const COM_APPLE_XSERVE_G4: UTType = UTType {
    identifier: "com.apple.xserve-g4",
    conforms_to: "com.apple.xserve",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_XSERVE_G5: UTType = UTType {
    identifier: "com.apple.xserve-g5",
    conforms_to: "com.apple.xserve",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Xserve G5",
};
pub const COM_APPLE_XSERVE_XEON: UTType = UTType {
    identifier: "com.apple.xserve-xeon",
    conforms_to: "com.apple.xserve",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACMINI: UTType = UTType {
    identifier: "com.apple.macmini",
    conforms_to: "com.apple.mac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Mac mini",
};
pub const COM_APPLE_MACMINI_G4: UTType = UTType {
    identifier: "com.apple.macmini-g4",
    conforms_to: "com.apple.macmini",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACMINI_CORE_DUO: UTType = UTType {
    identifier: "com.apple.macmini-core-duo",
    conforms_to: "com.apple.macmini",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACMINI_UNIBODY: UTType = UTType {
    identifier: "com.apple.macmini-unibody",
    conforms_to: "com.apple.macmini",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACMINI_UNIBODY_NO_OPTICAL: UTType = UTType {
    identifier: "com.apple.macmini-unibody-no-optical",
    conforms_to: "com.apple.macmini",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACMINI_2018: UTType = UTType {
    identifier: "com.apple.macmini-2018",
    conforms_to: "com.apple.macmini",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_EMAC: UTType = UTType {
    identifier: "com.apple.emac",
    conforms_to: "com.apple.mac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "eMac",
};
pub const COM_APPLE_IMAC: UTType = UTType {
    identifier: "com.apple.imac",
    conforms_to: "com.apple.mac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iMac",
};
pub const COM_APPLE_IMAC_G4_15: UTType = UTType {
    identifier: "com.apple.imac-g4-15",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_G4_17: UTType = UTType {
    identifier: "com.apple.imac-g4-17",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_G4_20: UTType = UTType {
    identifier: "com.apple.imac-g4-20",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_G5: UTType = UTType {
    identifier: "com.apple.imac-g5",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iMac G5",
};
pub const COM_APPLE_IMAC_G5_ISIGHT: UTType = UTType {
    identifier: "com.apple.imac-g5-isight",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_CORE_DUO: UTType = UTType {
    identifier: "com.apple.imac-core-duo",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_CORE_2_DUO: UTType = UTType {
    identifier: "com.apple.imac-core-2-duo",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_ISIGHT_24: UTType = UTType {
    identifier: "com.apple.imac-iSight-24",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_ALUMINUM_20: UTType = UTType {
    identifier: "com.apple.imac-aluminum-20",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_ALUMINUM_24: UTType = UTType {
    identifier: "com.apple.imac-aluminum-24",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_21: UTType = UTType {
    identifier: "com.apple.imac-unibody-21",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY: UTType = UTType {
    identifier: "com.apple.imac-unibody",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL: UTType = UTType {
    identifier: "com.apple.imac-unibody-21-no-optical",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_15_1: UTType = UTType {
    identifier: "com.apple.imac-15-1",
    conforms_to: "com.apple.imac-unibody-27-no-optical",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_MID_2015: UTType = UTType {
    identifier: "com.apple.imac-unibody-21-no-optical.mid-2015",
    conforms_to: "com.apple.imac-unibody-21-no-optical",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_LATE_2015: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical-late-2015",
    conforms_to: "com.apple.imac-unibody-27-no-optical",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_2017: UTType = UTType {
    identifier: "com.apple.imac-unibody-21-no-optical-2017",
    conforms_to: "com.apple.imac-unibody-21-no-optical",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2017: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical-2017",
    conforms_to: "com.apple.imac-unibody-27-no-optical",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_21_NO_OPTICAL_2019: UTType = UTType {
    identifier: "com.apple.imac-unibody-21-no-optical-2019",
    conforms_to: "com.apple.imac-unibody-21-no-optical",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2019: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical-2019",
    conforms_to: "com.apple.imac-unibody-27-no-optical",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_UNIBODY_27_NO_OPTICAL_2020: UTType = UTType {
    identifier: "com.apple.imac-unibody-27-no-optical-2020",
    conforms_to: "com.apple.imac-unibody-27-no-optical",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMACPRO_2017: UTType = UTType {
    identifier: "com.apple.imacpro-2017",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iMac Pro",
};
pub const COM_APPLE_MACBOOK: UTType = UTType {
    identifier: "com.apple.macbook",
    conforms_to: "com.apple.mac.laptop",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "MacBook",
};
pub const COM_APPLE_MACBOOK_WHITE: UTType = UTType {
    identifier: "com.apple.macbook-white",
    conforms_to: "com.apple.macbook",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOK_BLACK: UTType = UTType {
    identifier: "com.apple.macbook-black",
    conforms_to: "com.apple.macbook",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOK_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbook-unibody",
    conforms_to: "com.apple.macbook",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOK_UNIBODY_PLASTIC: UTType = UTType {
    identifier: "com.apple.macbook-unibody-plastic",
    conforms_to: "com.apple.macbook",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA: UTType = UTType {
    identifier: "com.apple.macbook-retina",
    conforms_to: "com.apple.macbook",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_SILVER: UTType = UTType {
    identifier: "com.apple.macbook-retina-silver",
    conforms_to: "com.apple.macbook-retina",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_GOLD: UTType = UTType {
    identifier: "com.apple.macbook-retina-gold",
    conforms_to: "com.apple.macbook-retina",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbook-retina-space-gray",
    conforms_to: "com.apple.macbook-retina",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_ROSE_GOLD: UTType = UTType {
    identifier: "com.apple.macbook-retina-rose-gold",
    conforms_to: "com.apple.macbook-retina",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_SILVER_2017: UTType = UTType {
    identifier: "com.apple.macbook-retina-silver-2017",
    conforms_to: "com.apple.macbook-retina-silver",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_GOLD_2017: UTType = UTType {
    identifier: "com.apple.macbook-retina-gold-2017",
    conforms_to: "com.apple.macbook-retina-gold",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_SPACE_GRAY_2017: UTType = UTType {
    identifier: "com.apple.macbook-retina-space-gray-2017",
    conforms_to: "com.apple.macbook-retina-space-gray",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_ROSE_GOLD_2017: UTType = UTType {
    identifier: "com.apple.macbook-retina-rose-gold-2017",
    conforms_to: "com.apple.macbook-retina-rose-gold",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOK_RETINA_GOLD_2018: UTType = UTType {
    identifier: "com.apple.macbook-retina-gold-2018",
    conforms_to: "com.apple.macbook-retina",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookpro-13-unibody",
    conforms_to: "com.apple.macbookpro",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_DISPLAY: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-display",
    conforms_to: "com.apple.macbookpro",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO: UTType = UTType {
    identifier: "com.apple.macbookpro",
    conforms_to: "com.apple.mac.laptop",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "MacBook Pro",
};
pub const COM_APPLE_MACBOOKPRO_15: UTType = UTType {
    identifier: "com.apple.macbookpro-15",
    conforms_to: "com.apple.macbookpro",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookpro-15-unibody",
    conforms_to: "com.apple.macbookpro",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_15_RETINA_DISPLAY: UTType = UTType {
    identifier: "com.apple.macbookpro-15-retina-display",
    conforms_to: "com.apple.macbookpro",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_17_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookpro-17-unibody",
    conforms_to: "com.apple.macbookpro",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_17: UTType = UTType {
    identifier: "com.apple.macbookpro-17",
    conforms_to: "com.apple.macbookpro",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_16: UTType = UTType {
    identifier: "com.apple.macbookpro-16",
    conforms_to: "com.apple.macbookpro",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_16_SILVER: UTType = UTType {
    identifier: "com.apple.macbookpro-16-silver",
    conforms_to: "com.apple.macbookpro-16",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_16_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookpro-16-space-gray",
    conforms_to: "com.apple.macbookpro-16",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-four-usb-ports-2020",
    conforms_to: "com.apple.macbookpro-13-retina-touchid",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_SILVER_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-four-usb-ports-silver-2020",
    conforms_to: "com.apple.macbookpro-13-retina-touchid-silver|com.apple.macbookpro-13-retina-four-usb-ports-2020",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: ""
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_FOUR_USB_PORTS_SPACE_GRAY_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-four-usb-ports-space-gray-2020",
    conforms_to: "com.apple.macbookpro-13-retina-touchid-space-gray|com.apple.macbookpro-13-retina-four-usb-ports-2020",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: ""
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-2020",
    conforms_to: "com.apple.macbookpro-13-retina-touchid",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver-2020",
    conforms_to:
        "com.apple.macbookpro-13-retina-touchid-silver|com.apple.macbookpro-13-retina-touchid-2020",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray-2020",
    conforms_to: "com.apple.macbookpro-13-retina-touchid-space-gray|com.apple.macbookpro-13-retina-touchid-2020",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: ""
};
pub const COM_APPLE_MACBOOKPRO_16_MID_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-16-mid-2020",
    conforms_to: "com.apple.macbookpro-16",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_16_SILVER_MID_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-16-silver-mid-2020",
    conforms_to: "com.apple.macbookpro-16-silver|com.apple.macbookpro-16-mid-2020",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_16_SPACE_GRAY_MID_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-16-space-gray-mid-2020",
    conforms_to: "com.apple.macbookpro-16-space-gray|com.apple.macbookpro-16-mid-2020",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR: UTType = UTType {
    identifier: "com.apple.macbookair",
    conforms_to: "com.apple.mac.laptop",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "MacBook Air",
};
pub const COM_APPLE_MACBOOKAIR_11_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookair-11-unibody",
    conforms_to: "com.apple.macbookair",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_13_UNIBODY: UTType = UTType {
    identifier: "com.apple.macbookair-13-unibody",
    conforms_to: "com.apple.macbookair",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2018: UTType = UTType {
    identifier: "com.apple.macbookair-2018",
    conforms_to: "com.apple.macbookair",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2018_SILVER: UTType = UTType {
    identifier: "com.apple.macbookair-2018-silver",
    conforms_to: "com.apple.macbookair-2018",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2018_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookair-2018-space-gray",
    conforms_to: "com.apple.macbookair-2018",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2018_GOLD: UTType = UTType {
    identifier: "com.apple.macbookair-2018-gold",
    conforms_to: "com.apple.macbookair-2018",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2019: UTType = UTType {
    identifier: "com.apple.macbookair-2019",
    conforms_to: "com.apple.macbookair-2018",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2019_SILVER: UTType = UTType {
    identifier: "com.apple.macbookair-2019-silver",
    conforms_to: "com.apple.macbookair-2018-silver",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2019_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookair-2019-space-gray",
    conforms_to: "com.apple.macbookair-2018-space-gray",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2019_GOLD: UTType = UTType {
    identifier: "com.apple.macbookair-2019-gold",
    conforms_to: "com.apple.macbookair-2018-gold",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2020: UTType = UTType {
    identifier: "com.apple.macbookair-2020",
    conforms_to: "com.apple.macbookair-2019",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2020_SILVER: UTType = UTType {
    identifier: "com.apple.macbookair-2020-silver",
    conforms_to: "com.apple.macbookair-2019-silver",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2020_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookair-2020-space-gray",
    conforms_to: "com.apple.macbookair-2019-space-gray",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_2020_GOLD: UTType = UTType {
    identifier: "com.apple.macbookair-2020-gold",
    conforms_to: "com.apple.macbookair-2019-gold",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACPRO: UTType = UTType {
    identifier: "com.apple.macpro",
    conforms_to: "com.apple.mac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Mac Pro",
};
pub const COM_APPLE_MACPRO_FIREWIRE: UTType = UTType {
    identifier: "com.apple.macpro-firewire",
    conforms_to: "com.apple.macpro|com.apple.mac.tower",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACPRO_CYLINDER: UTType = UTType {
    identifier: "com.apple.macpro-cylinder",
    conforms_to: "com.apple.macpro|com.apple.mac.tower",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACPRO_2019: UTType = UTType {
    identifier: "com.apple.macpro-2019",
    conforms_to: "com.apple.macpro|com.apple.mac.tower",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACPRO_2019_RACKMOUNT: UTType = UTType {
    identifier: "com.apple.macpro-2019-rackmount",
    conforms_to: "com.apple.macpro|com.apple.mac.rackmount",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACMINI_2020: UTType = UTType {
    identifier: "com.apple.macmini-2020",
    conforms_to: "com.apple.macmini",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_LATE_2020: UTType = UTType {
    identifier: "com.apple.macbookair-late-2020",
    conforms_to: "com.apple.macbookair-2020",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_LATE_2020_SILVER: UTType = UTType {
    identifier: "com.apple.macbookair-late-2020-silver",
    conforms_to: "com.apple.macbookair-2020-silver|com.apple.macbookair-late-2020",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_LATE_2020_SPACE_GRAY: UTType = UTType {
    identifier: "com.apple.macbookair-late-2020-space-gray",
    conforms_to: "com.apple.macbookair-2020-space-gray|com.apple.macbookair-late-2020",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKAIR_LATE_2020_GOLD: UTType = UTType {
    identifier: "com.apple.macbookair-late-2020-gold",
    conforms_to: "com.apple.macbookair-2020-gold|com.apple.macbookair-late-2020",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_LATE_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-late-2020",
    conforms_to: "com.apple.macbookpro-13-retina-touchid-2020",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SILVER_LATE_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-silver-late-2020",
    conforms_to: "com.apple.macbookpro-13-retina-touchid-silver-2020|com.apple.macbookpro-13-retina-touchid-late-2020",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: ""
};
pub const COM_APPLE_MACBOOKPRO_13_RETINA_TOUCHID_SPACE_GRAY_LATE_2020: UTType = UTType {
    identifier: "com.apple.macbookpro-13-retina-touchid-space-gray-late-2020",
    conforms_to: "com.apple.macbookpro-13-retina-touchid-space-gray-2020|com.apple.macbookpro-13-retina-touchid-late-2020",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: ""
};
pub const COM_APPLE_IMAC_2021: UTType = UTType {
    identifier: "com.apple.imac-2021",
    conforms_to: "com.apple.imac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_2021_SILVER: UTType = UTType {
    identifier: "com.apple.imac-2021-silver",
    conforms_to: "com.apple.imac-2021",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_2021_YELLOW: UTType = UTType {
    identifier: "com.apple.imac-2021-yellow",
    conforms_to: "com.apple.imac-2021",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_2021_GREEN: UTType = UTType {
    identifier: "com.apple.imac-2021-green",
    conforms_to: "com.apple.imac-2021",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_2021_BLUE: UTType = UTType {
    identifier: "com.apple.imac-2021-blue",
    conforms_to: "com.apple.imac-2021",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_2021_RED: UTType = UTType {
    identifier: "com.apple.imac-2021-red",
    conforms_to: "com.apple.imac-2021",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_2021_PURPLE: UTType = UTType {
    identifier: "com.apple.imac-2021-purple",
    conforms_to: "com.apple.imac-2021",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IMAC_2021_ORANGE: UTType = UTType {
    identifier: "com.apple.imac-2021-orange",
    conforms_to: "com.apple.imac-2021",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_DEVELOPER_TRANSITION_KIT_2005: UTType = UTType {
    identifier: "com.apple.developer-transition-kit-2005",
    conforms_to: "com.apple.mac.tower",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_DEVELOPER_TRANSITION_KIT_2020: UTType = UTType {
    identifier: "com.apple.developer-transition-kit-2020",
    conforms_to: "com.apple.mac",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_IOS_DEVICE: UTType = UTType {
    identifier: "com.apple.ios-device",
    conforms_to: "com.apple.device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iOS device",
};
pub const COM_APPLE_APPLE_TV: UTType = UTType {
    identifier: "com.apple.apple-tv",
    conforms_to: "com.apple.ios-device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Apple TV",
};
pub const COM_APPLE_HOMEPOD: UTType = UTType {
    identifier: "com.apple.homepod",
    conforms_to: "com.apple.ios-device|public.speaker",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "HomePod",
};
pub const COM_APPLE_IOS_SIMULATOR: UTType = UTType {
    identifier: "com.apple.ios-simulator",
    conforms_to: "com.apple.ios-device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iOS Simulator",
};
pub const COM_APPLE_IPHONE: UTType = UTType {
    identifier: "com.apple.iphone",
    conforms_to: "com.apple.ios-device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone",
};
pub const COM_APPLE_IPHONE_3G: UTType = UTType {
    identifier: "com.apple.iphone-3g",
    conforms_to: "com.apple.iphone",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone 3G",
};
pub const COM_APPLE_IPHONE_3GS: UTType = UTType {
    identifier: "com.apple.iphone-3gs",
    conforms_to: "com.apple.iphone",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone 3GS",
};
pub const COM_APPLE_IPHONE_4: UTType = UTType {
    identifier: "com.apple.iphone-4",
    conforms_to: "com.apple.iphone",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone 4",
};
pub const COM_APPLE_IPHONE_8: UTType = UTType {
    identifier: "com.apple.iphone-8",
    conforms_to: "com.apple.iphone",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone 8",
};
pub const COM_APPLE_IPHONE_8_2: UTType = UTType {
    identifier: "com.apple.iphone-8-2",
    conforms_to: "com.apple.iphone-8",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone 8 (Model A1863, A1905, A1906, A1907)",
};
pub const COM_APPLE_IPHONE_8_7: UTType = UTType {
    identifier: "com.apple.iphone-8-7",
    conforms_to: "com.apple.iphone-8",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone 8 (Model A1863, A1905, A1906, A1907)",
};
pub const COM_APPLE_IPHONE_8_8: UTType = UTType {
    identifier: "com.apple.iphone-8-8",
    conforms_to: "com.apple.iphone-8",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone 8 (Model A1863, A1905, A1906, A1907)",
};
pub const COM_APPLE_IPHONE_8_PLUS: UTType = UTType {
    identifier: "com.apple.iphone-8-plus",
    conforms_to: "com.apple.iphone",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone 8 Plus",
};
pub const COM_APPLE_IPHONE_8_PLUS_2: UTType = UTType {
    identifier: "com.apple.iphone-8-plus-2",
    conforms_to: "com.apple.iphone-8-plus",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone 8 Plus (Model A1864, A1897, A1898, A1899)",
};
pub const COM_APPLE_IPHONE_8_PLUS_3: UTType = UTType {
    identifier: "com.apple.iphone-8-plus-3",
    conforms_to: "com.apple.iphone-8-plus",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone 8 Plus (Model A1864, A1897, A1898, A1899)",
};
pub const COM_APPLE_IPHONE_8_PLUS_1: UTType = UTType {
    identifier: "com.apple.iphone-8-plus-1",
    conforms_to: "com.apple.iphone-8-plus",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone 8 Plus (Model A1864, A1897, A1898, A1899)",
};
pub const COM_APPLE_IPHONE_X: UTType = UTType {
    identifier: "com.apple.iphone-x",
    conforms_to: "com.apple.homebuttonless-iphone",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone X",
};
pub const COM_APPLE_IPHONE_X_1: UTType = UTType {
    identifier: "com.apple.iphone-x-1",
    conforms_to: "com.apple.iphone-x",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone X (Model A1865, A1901, A1902, A1903)",
};
pub const COM_APPLE_IPHONE_X_2: UTType = UTType {
    identifier: "com.apple.iphone-x-2",
    conforms_to: "com.apple.iphone-x",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPhone X (Model A1865, A1901, A1902, A1903)",
};
pub const COM_APPLE_LEGACY_IPOD: UTType = UTType {
    identifier: "com.apple.legacy-ipod",
    conforms_to: "com.apple.device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPod",
};
pub const COM_APPLE_IPOD_SHUFFLE: UTType = UTType {
    identifier: "com.apple.ipod-shuffle",
    conforms_to: "com.apple.legacy-ipod",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPod Shuffle",
};
pub const COM_APPLE_IPOD_SHUFFLE_GEN1: UTType = UTType {
    identifier: "com.apple.ipod-shuffle-gen1",
    conforms_to: "com.apple.ipod-shuffle",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPod Shuffle",
};
pub const COM_APPLE_IPOD_SHUFFLE_GEN2: UTType = UTType {
    identifier: "com.apple.ipod-shuffle-gen2",
    conforms_to: "com.apple.ipod-shuffle",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPod Shuffle (2th generation)",
};
pub const COM_APPLE_IPOD_SHUFFLE_GEN3: UTType = UTType {
    identifier: "com.apple.ipod-shuffle-gen3",
    conforms_to: "com.apple.ipod-shuffle",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPod Shuffle (3th generation)",
};
pub const COM_APPLE_IPOD_SHUFFLE_GEN4: UTType = UTType {
    identifier: "com.apple.ipod-shuffle-gen4",
    conforms_to: "com.apple.ipod-shuffle",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPod Shuffle (4th generation)",
};
pub const COM_APPLE_IPOD_NANO: UTType = UTType {
    identifier: "com.apple.ipod-nano",
    conforms_to: "com.apple.legacy-ipod",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPod Nano",
};
pub const COM_APPLE_IPOD_MINI: UTType = UTType {
    identifier: "com.apple.ipod-mini",
    conforms_to: "com.apple.legacy-ipod",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPod Nano",
};
pub const COM_APPLE_IPOD_CLASSIC: UTType = UTType {
    identifier: "com.apple.ipod-classic",
    conforms_to: "com.apple.legacy-ipod",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPod Classic",
};
pub const COM_APPLE_IPOD: UTType = UTType {
    identifier: "com.apple.ipod",
    conforms_to: "com.apple.ios-device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPod",
};
pub const COM_APPLE_IPOD_TOUCH: UTType = UTType {
    identifier: "com.apple.ipod-touch",
    conforms_to: "com.apple.ipod",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPod touch",
};
pub const COM_APPLE_IPOD_TOUCH_2: UTType = UTType {
    identifier: "com.apple.ipod-touch-2",
    conforms_to: "com.apple.ipod",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPod touch",
};
pub const COM_APPLE_IPOD_TOUCH_3: UTType = UTType {
    identifier: "com.apple.ipod-touch-3",
    conforms_to: "com.apple.ipod",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPod touch",
};
pub const COM_APPLE_IPOD_TOUCH_4: UTType = UTType {
    identifier: "com.apple.ipod-touch-4",
    conforms_to: "com.apple.ipod",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPod touch",
};
pub const COM_APPLE_IPAD: UTType = UTType {
    identifier: "com.apple.ipad",
    conforms_to: "com.apple.ios-device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "iPad",
};
pub const COM_APPLE_HOMEBUTTONLESS_IPAD: UTType = UTType {
    identifier: "com.apple.homebuttonless-ipad",
    conforms_to: "com.apple.ipad|com.apple.homebuttonless-device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_HOMEBUTTONLESS_IPHONE: UTType = UTType {
    identifier: "com.apple.homebuttonless-iphone",
    conforms_to: "com.apple.iphone|com.apple.homebuttonless-device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const COM_APPLE_WATCH: UTType = UTType {
    identifier: "com.apple.watch",
    conforms_to: "com.apple.ios-device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Apple Watch",
};
pub const COM_APPLE_AIRPORT_EXPRESS: UTType = UTType {
    identifier: "com.apple.airport-express",
    conforms_to: "com.apple.device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "AirPort Express",
};
pub const COM_APPLE_AIRPORT: UTType = UTType {
    identifier: "com.apple.airport",
    conforms_to: "com.apple.device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "AirPort Extreme",
};
pub const COM_APPLE_TIME_CAPSULE: UTType = UTType {
    identifier: "com.apple.time-capsule",
    conforms_to: "com.apple.device",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Time Capsule",
};
pub const COM_APPLE_AIRPORT_EXTREME_TOWER: UTType = UTType {
    identifier: "com.apple.airport-extreme-tower",
    conforms_to: "com.apple.airport",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "AirPort Extreme",
};
pub const COM_APPLE_AIRPORT_TIME_CAPSULE_TOWER: UTType = UTType {
    identifier: "com.apple.airport-time-capsule-tower",
    conforms_to: "com.apple.time-capsule",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Time Capsule",
};
pub const COM_APPLE_CINEMA_DISPLAY: UTType = UTType {
    identifier: "com.apple.cinema-display",
    conforms_to: "public.display",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Cinema Display",
};
pub const COM_APPLE_LED_CINEMA_DISPLAY_24: UTType = UTType {
    identifier: "com.apple.led-cinema-display-24",
    conforms_to: "public.display",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "LED Cinema Display",
};
pub const COM_APPLE_LED_CINEMA_DISPLAY_27: UTType = UTType {
    identifier: "com.apple.led-cinema-display-27",
    conforms_to: "public.display",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "LED Cinema Display",
};
pub const COM_APPLE_PRO_DISPLAY_XDR: UTType = UTType {
    identifier: "com.apple.pro-display-xdr",
    conforms_to: "public.display",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Pro Display XDR",
};
pub const PUBLIC_STORAGE: UTType = UTType {
    identifier: "public.storage",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Storage",
};
pub const COM_APPLE_STORAGE_EXTERNAL: UTType = UTType {
    identifier: "com.apple.storage-external",
    conforms_to: "public.storage",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "External Disk",
};
pub const COM_APPLE_GENERIC_TIME_MACHINE_DISK: UTType = UTType {
    identifier: "com.apple.generic-time-machine-disk",
    conforms_to: "public.storage",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Generic Time Machine Disk",
};
pub const COM_APPLE_STORAGE_NETBOOT: UTType = UTType {
    identifier: "com.apple.storage-netboot",
    conforms_to: "public.storage",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "NetBootVolume",
};
pub const COM_APPLE_FILE_SERVER: UTType = UTType {
    identifier: "com.apple.file-server",
    conforms_to: "public.storage",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "File Server",
};
pub const COM_APPLE_STORAGE_INTERNAL: UTType = UTType {
    identifier: "com.apple.storage-internal",
    conforms_to: "public.storage",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Internal Disk",
};
pub const COM_APPLE_STORAGE_REMOVABLE: UTType = UTType {
    identifier: "com.apple.storage-removable",
    conforms_to: "public.storage",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Removable Disk",
};
pub const COM_APPLE_FILE_VAULT: UTType = UTType {
    identifier: "com.apple.file-vault",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "File Vault",
};
pub const COM_APPLE_GENERIC_AIRDISK: UTType = UTType {
    identifier: "com.apple.generic-airdisk",
    conforms_to: "public.storage",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Generic AirDisk",
};
pub const PUBLIC_OPTICAL_STORAGE_MEDIA: UTType = UTType {
    identifier: "public.optical-storage-media",
    conforms_to: "public.storage",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "optical storage media",
};
pub const PUBLIC_CD_BASED_MEDIA: UTType = UTType {
    identifier: "public.cd-based-media",
    conforms_to: "public.optical-storage-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "CD",
};
pub const PUBLIC_CD_MEDIA: UTType = UTType {
    identifier: "public.cd-media",
    conforms_to: "public.cd-based-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "CD",
};
pub const PUBLIC_CD_R_MEDIA: UTType = UTType {
    identifier: "public.cd-r-media",
    conforms_to: "public.cd-based-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "CD-R",
};
pub const PUBLIC_CD_RW_MEDIA: UTType = UTType {
    identifier: "public.cd-rw-media",
    conforms_to: "public.cd-based-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "CD-RW",
};
pub const PUBLIC_DVD_BASED_MEDIA: UTType = UTType {
    identifier: "public.dvd-based-media",
    conforms_to: "public.optical-storage-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "DVD",
};
pub const PUBLIC_DVD_MEDIA: UTType = UTType {
    identifier: "public.dvd-media",
    conforms_to: "public.dvd-based-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "DVD",
};
pub const PUBLIC_DVD_R_MEDIA: UTType = UTType {
    identifier: "public.dvd-r-media",
    conforms_to: "public.dvd-based-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "DVD-R",
};
pub const PUBLIC_DVD_RW_MEDIA: UTType = UTType {
    identifier: "public.dvd-rw-media",
    conforms_to: "public.dvd-based-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "DVD-RW",
};
pub const PUBLIC_DVD_RAM_MEDIA: UTType = UTType {
    identifier: "public.dvd-ram-media",
    conforms_to: "public.dvd-based-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "DVD-RAM",
};
pub const PUBLIC_DVD_PLUS_R_MEDIA: UTType = UTType {
    identifier: "public.dvd-plus-r-media",
    conforms_to: "public.dvd-based-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "DVD+R",
};
pub const PUBLIC_DVD_PLUS_RW_MEDIA: UTType = UTType {
    identifier: "public.dvd-plus-rw-media",
    conforms_to: "public.dvd-based-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "DVD+RW",
};
pub const PUBLIC_HD_DVD_BASED_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-based-media",
    conforms_to: "public.optical-storage-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "HD DVD",
};
pub const PUBLIC_HD_DVD_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-media",
    conforms_to: "public.hd-dvd-based-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "HD DVD",
};
pub const PUBLIC_HD_DVD_R_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-r-media",
    conforms_to: "public.hd-dvd-based-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "HD DVD-R",
};
pub const PUBLIC_HD_DVD_RW_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-rw-media",
    conforms_to: "public.hd-dvd-based-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "HD DVD-RW",
};
pub const PUBLIC_HD_DVD_RAM_MEDIA: UTType = UTType {
    identifier: "public.hd-dvd-ram-media",
    conforms_to: "public.hd-dvd-based-media",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "HD DVD-RAM",
};
pub const PUBLIC_APP_CATEGORY: UTType = UTType {
    identifier: "public.app-category",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Application",
};
pub const PUBLIC_APP_CATEGORY_BUSINESS: UTType = UTType {
    identifier: "public.app-category.business",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Business",
};
pub const PUBLIC_APP_CATEGORY_DEVELOPER_TOOLS: UTType = UTType {
    identifier: "public.app-category.developer-tools",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Developer Tools",
};
pub const PUBLIC_APP_CATEGORY_EDUCATION: UTType = UTType {
    identifier: "public.app-category.education",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Education",
};
pub const PUBLIC_APP_CATEGORY_ENTERTAINMENT: UTType = UTType {
    identifier: "public.app-category.entertainment",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Entertainment",
};
pub const PUBLIC_APP_CATEGORY_FINANCE: UTType = UTType {
    identifier: "public.app-category.finance",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Finance",
};
pub const PUBLIC_APP_CATEGORY_GAMES: UTType = UTType {
    identifier: "public.app-category.games",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Games",
};
pub const PUBLIC_APP_CATEGORY_ACTION_GAMES: UTType = UTType {
    identifier: "public.app-category.action-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Action Games",
};
pub const PUBLIC_APP_CATEGORY_ADVENTURE_GAMES: UTType = UTType {
    identifier: "public.app-category.adventure-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Adventure Games",
};
pub const PUBLIC_APP_CATEGORY_ARCADE_GAMES: UTType = UTType {
    identifier: "public.app-category.arcade-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Arcade Games",
};
pub const PUBLIC_APP_CATEGORY_BOARD_GAMES: UTType = UTType {
    identifier: "public.app-category.board-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Board Games",
};
pub const PUBLIC_APP_CATEGORY_CARD_GAMES: UTType = UTType {
    identifier: "public.app-category.card-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Card Games",
};
pub const PUBLIC_APP_CATEGORY_CASINO_GAMES: UTType = UTType {
    identifier: "public.app-category.casino-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Casino Games",
};
pub const PUBLIC_APP_CATEGORY_DICE_GAMES: UTType = UTType {
    identifier: "public.app-category.dice-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Dice Games",
};
pub const PUBLIC_APP_CATEGORY_EDUCATIONAL_GAMES: UTType = UTType {
    identifier: "public.app-category.educational-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Educational Games",
};
pub const PUBLIC_APP_CATEGORY_FAMILY_GAMES: UTType = UTType {
    identifier: "public.app-category.family-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Family Games",
};
pub const PUBLIC_APP_CATEGORY_KIDS_GAMES: UTType = UTType {
    identifier: "public.app-category.kids-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Kids Games",
};
pub const PUBLIC_APP_CATEGORY_MUSIC_GAMES: UTType = UTType {
    identifier: "public.app-category.music-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Music Games",
};
pub const PUBLIC_APP_CATEGORY_PUZZLE_GAMES: UTType = UTType {
    identifier: "public.app-category.puzzle-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Puzzle Games",
};
pub const PUBLIC_APP_CATEGORY_RACING_GAMES: UTType = UTType {
    identifier: "public.app-category.racing-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Racing Games",
};
pub const PUBLIC_APP_CATEGORY_ROLE_PLAYING_GAMES: UTType = UTType {
    identifier: "public.app-category.role-playing-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Role Playing Games",
};
pub const PUBLIC_APP_CATEGORY_SIMULATION_GAMES: UTType = UTType {
    identifier: "public.app-category.simulation-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Simulation Games",
};
pub const PUBLIC_APP_CATEGORY_SPORTS_GAMES: UTType = UTType {
    identifier: "public.app-category.sports-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Sports Games",
};
pub const PUBLIC_APP_CATEGORY_STRATEGY_GAMES: UTType = UTType {
    identifier: "public.app-category.strategy-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Strategy Games",
};
pub const PUBLIC_APP_CATEGORY_TRIVIA_GAMES: UTType = UTType {
    identifier: "public.app-category.trivia-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Trivia Games",
};
pub const PUBLIC_APP_CATEGORY_WORD_GAMES: UTType = UTType {
    identifier: "public.app-category.word-games",
    conforms_to: "public.app-category.games",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Word Games",
};
pub const PUBLIC_APP_CATEGORY_GRAPHICS_DESIGN: UTType = UTType {
    identifier: "public.app-category.graphics-design",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Graphics n Design",
};
pub const PUBLIC_APP_CATEGORY_HEALTHCARE_FITNESS: UTType = UTType {
    identifier: "public.app-category.healthcare-fitness",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Healthcare n Fitness",
};
pub const PUBLIC_APP_CATEGORY_LIFESTYLE: UTType = UTType {
    identifier: "public.app-category.lifestyle",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Lifestyle",
};
pub const PUBLIC_APP_CATEGORY_MEDICAL: UTType = UTType {
    identifier: "public.app-category.medical",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Medical",
};
pub const PUBLIC_APP_CATEGORY_MUSIC: UTType = UTType {
    identifier: "public.app-category.music",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Music",
};
pub const PUBLIC_APP_CATEGORY_NEWS: UTType = UTType {
    identifier: "public.app-category.news",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "News",
};
pub const PUBLIC_APP_CATEGORY_PHOTOGRAPHY: UTType = UTType {
    identifier: "public.app-category.photography",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Photography",
};
pub const PUBLIC_APP_CATEGORY_PRODUCTIVITY: UTType = UTType {
    identifier: "public.app-category.productivity",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Productivity",
};
pub const PUBLIC_APP_CATEGORY_REFERENCE: UTType = UTType {
    identifier: "public.app-category.reference",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Reference",
};
pub const PUBLIC_APP_CATEGORY_SOCIAL_NETWORKING: UTType = UTType {
    identifier: "public.app-category.social-networking",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Social Networking",
};
pub const PUBLIC_APP_CATEGORY_SPORTS: UTType = UTType {
    identifier: "public.app-category.sports",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Sports",
};
pub const PUBLIC_APP_CATEGORY_TRAVEL: UTType = UTType {
    identifier: "public.app-category.travel",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Travel",
};
pub const PUBLIC_APP_CATEGORY_UTILITIES: UTType = UTType {
    identifier: "public.app-category.utilities",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Utilities",
};
pub const PUBLIC_APP_CATEGORY_VIDEO: UTType = UTType {
    identifier: "public.app-category.video",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Video",
};
pub const PUBLIC_APP_CATEGORY_WEATHER: UTType = UTType {
    identifier: "public.app-category.weather",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Weather",
};
pub const PUBLIC_APP_CATEGORY_BOOKMARKS: UTType = UTType {
    identifier: "public.app-category.bookmarks",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Bookmarks",
};
pub const PUBLIC_APP_CATEGORY_BOOKS: UTType = UTType {
    identifier: "public.app-category.books",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Books",
};
pub const PUBLIC_APP_CATEGORY_NAVIGATION: UTType = UTType {
    identifier: "public.app-category.navigation",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Navigation",
};
pub const PUBLIC_APP_CATEGORY_PHOTOGRAPHY_AND_VIDEO: UTType = UTType {
    identifier: "public.app-category.photography-and-video",
    conforms_to: "public.app-category.photography|public.app-category.video",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Photo & Video",
};
pub const PUBLIC_APP_CATEGORY_FOOD_AND_DRINK: UTType = UTType {
    identifier: "public.app-category.food-and-drink",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Food & Drink",
};
pub const PUBLIC_APP_CATEGORY_SHOPPING: UTType = UTType {
    identifier: "public.app-category.shopping",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Shopping",
};
pub const PUBLIC_APP_CATEGORY_MAGAZINES_AND_NEWSPAPERS: UTType = UTType {
    identifier: "public.app-category.magazines-and-newspapers",
    conforms_to: "public.app-category",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Magazines & Newspapers",
};
pub const COM_APPLE_STRUCTURED_TEXT: UTType = UTType {
    identifier: "com.apple.structured-text",
    conforms_to: "public.plain-text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Structured Text",
};
pub const COM_APPLE_STRUCTURED_TEXT_DATE: UTType = UTType {
    identifier: "com.apple.structured-text.date",
    conforms_to: "com.apple.structured-text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Date (Structured Text)",
};
pub const COM_APPLE_STRUCTURED_TEXT_ADDRESS: UTType = UTType {
    identifier: "com.apple.structured-text.address",
    conforms_to: "com.apple.structured-text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Address (Structured Text)",
};
pub const COM_APPLE_STRUCTURED_TEXT_TELEPHONE_NUMBER: UTType = UTType {
    identifier: "com.apple.structured-text.telephone-number",
    conforms_to: "com.apple.structured-text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Telephone Number (Structured Text)",
};
pub const COM_APPLE_STRUCTURED_TEXT_TRANSIT_INFORMATION: UTType = UTType {
    identifier: "com.apple.structured-text.transit-information",
    conforms_to: "com.apple.structured-text",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Transit Information (Structured Text)",
};
pub const COM_APPLE_ACTIVE_WEBPAGE: UTType = UTType {
    identifier: "com.apple.active-webpage",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
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
    conforms_to: "public.data|public.composite-content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PDF document",
};
pub const COM_ADOBE_EDN: UTType = UTType {
    identifier: "com.adobe.edn",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Adobe DRM Activation Key (EDN)",
};
pub const COM_ADOBE_ETD: UTType = UTType {
    identifier: "com.adobe.etd",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Adobe Digital Editions (ETD)",
};
pub const COM_ADOBE_XFDF: UTType = UTType {
    identifier: "com.adobe.xfdf",
    conforms_to: "public.data|public.composite-content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Adobe Acrobat Forms Document (XFDF)",
};
pub const COM_ADOBE_FDF: UTType = UTType {
    identifier: "com.adobe.fdf",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Adobe Acrobat Forms Document (FDF)",
};
pub const COM_ADOBE_POSTSCRIPT: UTType = UTType {
    identifier: "com.adobe.postscript",
    conforms_to: "public.data|public.composite-content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "PostScript",
};
pub const COM_ADOBE_ENCAPSULATED_POSTSCRIPT: UTType = UTType {
    identifier: "com.adobe.encapsulated-postscript",
    conforms_to: "com.adobe.postscript",
    tags: "",
    filename_extension: "eps",
    mime_type: "eps",
    description: "Encapsulated PostScript",
};
pub const COM_COMPUSERVE_GIF: UTType = UTType {
    identifier: "com.compuserve.gif",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "GIF image",
};
pub const COM_MICROSOFT_BMP: UTType = UTType {
    identifier: "com.microsoft.bmp",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Windows BMP image",
};
pub const COM_MICROSOFT_ICO: UTType = UTType {
    identifier: "com.microsoft.ico",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Windows icon image",
};
pub const ORG_WEBMPROJECT_WEBP: UTType = UTType {
    identifier: "org.webmproject.webp",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "WebP Image",
};
pub const ORG_WEBMPROJECT_WEBM: UTType = UTType {
    identifier: "org.webmproject.webm",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "WebM Media",
};
pub const PUBLIC_OFD: UTType = UTType {
    identifier: "public.ofd",
    conforms_to: "public.data|public.composite-content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Open Fixed-layout Document",
};
pub const ORG_OPENOFFICE_TEXT: UTType = UTType {
    identifier: "org.openoffice.text",
    conforms_to: "public.data|public.content",
    tags: "",
    filename_extension: "sxw|sdw",
    mime_type: "sxw|sdw",
    description: "OpenOffice.org 1.0 Text",
};
pub const ORG_OPENOFFICE_TEXT_TEMPLATE: UTType = UTType {
    identifier: "org.openoffice.text-template",
    conforms_to: "public.data|public.content",
    tags: "",
    filename_extension: "stw",
    mime_type: "stw",
    description: "OpenOffice.org 1.0 Text Template",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_TEXT: UTType = UTType {
    identifier: "org.oasis-open.opendocument.text",
    conforms_to: "org.oasis-open.opendocument|public.composite-content",
    tags: "",
    filename_extension: "odt",
    mime_type: "odt",
    description: "Open Document text",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.text-template",
    conforms_to: "org.oasis-open.opendocument|public.composite-content",
    tags: "",
    filename_extension: "ott",
    mime_type: "ott",
    description: "Open Document text template",
};
pub const ORG_OPENOFFICE_GRAPHICS: UTType = UTType {
    identifier: "org.openoffice.graphics",
    conforms_to: "public.data|public.content",
    tags: "",
    filename_extension: "sxd|sda",
    mime_type: "sxd|sda",
    description: "OpenOffice.org 1.0 Drawing",
};
pub const ORG_OPENOFFICE_GRAPHICS_TEMPLATE: UTType = UTType {
    identifier: "org.openoffice.graphics-template",
    conforms_to: "public.data|public.content",
    tags: "",
    filename_extension: "std",
    mime_type: "std",
    description: "OpenOffice.org 1.0 Drawing Template",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_GRAPHICS: UTType = UTType {
    identifier: "org.oasis-open.opendocument.graphics",
    conforms_to: "org.oasis-open.opendocument|public.composite-content",
    tags: "",
    filename_extension: "odg",
    mime_type: "odg",
    description: "Open Document graphics",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_GRAPHICS_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.graphics-template",
    conforms_to: "org.oasis-open.opendocument|public.composite-content",
    tags: "",
    filename_extension: "otg",
    mime_type: "otg",
    description: "Open Document graphics template",
};
pub const ORG_OPENOFFICE_PRESENTATION: UTType = UTType {
    identifier: "org.openoffice.presentation",
    conforms_to: "public.data|public.content|public.presentation",
    tags: "",
    filename_extension: "sxi|sdd|sdp",
    mime_type: "sxi|sdd|sdp",
    description: "OpenOffice.org 1.0 Presentation",
};
pub const ORG_OPENOFFICE_PRESENTATION_TEMPLATE: UTType = UTType {
    identifier: "org.openoffice.presentation-template",
    conforms_to: "public.data|public.content|public.presentation",
    tags: "",
    filename_extension: "sti",
    mime_type: "sti",
    description: "OpenOffice.org 1.0 Presentation Template",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_PRESENTATION: UTType = UTType {
    identifier: "org.oasis-open.opendocument.presentation",
    conforms_to: "org.oasis-open.opendocument|public.composite-content|public.presentation",
    tags: "",
    filename_extension: "odp",
    mime_type: "odp",
    description: "Open Document presentation",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_PRESENTATION_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.presentation-template",
    conforms_to: "org.oasis-open.opendocument|public.composite-content|public.presentation",
    tags: "",
    filename_extension: "otp",
    mime_type: "otp",
    description: "Open Document presentation template",
};
pub const ORG_OPENOFFICE_SPREADSHEET: UTType = UTType {
    identifier: "org.openoffice.spreadsheet",
    conforms_to: "public.data|public.spreadsheet",
    tags: "",
    filename_extension: "sxc|sdc",
    mime_type: "sxc|sdc",
    description: "OpenOffice.org 1.0 Spreadsheet",
};
pub const ORG_OPENOFFICE_SPREADSHEET_TEMPLATE: UTType = UTType {
    identifier: "org.openoffice.spreadsheet-template",
    conforms_to: "public.data|public.spreadsheet",
    tags: "",
    filename_extension: "stc",
    mime_type: "stc",
    description: "OpenOffice.org 1.0 Spreadsheet Template",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_SPREADSHEET: UTType = UTType {
    identifier: "org.oasis-open.opendocument.spreadsheet",
    conforms_to: "org.oasis-open.opendocument|public.composite-content|public.spreadsheet",
    tags: "",
    filename_extension: "ods",
    mime_type: "ods",
    description: "Open Document spreadsheet",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_SPREADSHEET_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.spreadsheet-template",
    conforms_to: "org.oasis-open.opendocument|public.composite-content|public.spreadsheet",
    tags: "",
    filename_extension: "ots",
    mime_type: "ots",
    description: "Open Document spreadsheet template",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_CHART: UTType = UTType {
    identifier: "org.oasis-open.opendocument.chart",
    conforms_to: "org.oasis-open.opendocument|public.composite-content",
    tags: "",
    filename_extension: "odc",
    mime_type: "odc",
    description: "Open Document chart",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_CHART_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.chart-template",
    conforms_to: "org.oasis-open.opendocument|public.composite-content",
    tags: "",
    filename_extension: "otc",
    mime_type: "otc",
    description: "Open Document chart template",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_IMAGE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.image",
    conforms_to: "org.oasis-open.opendocument|public.image",
    tags: "",
    filename_extension: "odi",
    mime_type: "odi",
    description: "Open Document image",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_IMAGE_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.image-template",
    conforms_to: "org.oasis-open.opendocument|public.image",
    tags: "",
    filename_extension: "oti",
    mime_type: "oti",
    description: "Open Document image template",
};
pub const ORG_OPENOFFICE_FORMULA: UTType = UTType {
    identifier: "org.openoffice.formula",
    conforms_to: "public.data|public.content",
    tags: "",
    filename_extension: "sxm|smf",
    mime_type: "sxm|smf",
    description: "OpenOffice.org 1.0 Formula",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_FORMULA: UTType = UTType {
    identifier: "org.oasis-open.opendocument.formula",
    conforms_to: "org.oasis-open.opendocument|public.content",
    tags: "",
    filename_extension: "odf",
    mime_type: "odf",
    description: "Open Document formula",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_FORMULA_TEMPLATE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.formula-template",
    conforms_to: "org.oasis-open.opendocument|public.content",
    tags: "",
    filename_extension: "otf",
    mime_type: "otf",
    description: "Open Document formula template",
};
pub const ORG_OPENOFFICE_TEXT_MASTER: UTType = UTType {
    identifier: "org.openoffice.text-master",
    conforms_to: "public.data|public.content",
    tags: "",
    filename_extension: "sxg",
    mime_type: "sxg",
    description: "OpenOffice.org 1.0 Master",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_MASTER: UTType = UTType {
    identifier: "org.oasis-open.opendocument.text-master",
    conforms_to: "org.oasis-open.opendocument|public.composite-content",
    tags: "",
    filename_extension: "odm",
    mime_type: "odm",
    description: "Open Document text master",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_TEXT_WEB: UTType = UTType {
    identifier: "org.oasis-open.opendocument.text-web",
    conforms_to: "org.oasis-open.opendocument|public.composite-content",
    tags: "",
    filename_extension: "oth",
    mime_type: "oth",
    description: "Open Document HTML template",
};
pub const ORG_OASIS_OPEN_OPENDOCUMENT_DATABASE: UTType = UTType {
    identifier: "org.oasis-open.opendocument.database",
    conforms_to: "public.data|public.content|public.database",
    tags: "",
    filename_extension: "odb",
    mime_type: "odb",
    description: "Open Document database",
};
pub const COM_MICROSOFT_WORD_WORDML: UTType = UTType {
    identifier: "com.microsoft.word.wordml",
    conforms_to: "public.xml|public.composite-content",
    tags: "",
    filename_extension: "xml",
    mime_type: "xml",
    description: "Microsoft Word 2003 XML document",
};
pub const ORG_OPENXMLFORMATS_WORDPROCESSINGML_DOCUMENT: UTType = UTType {
    identifier: "org.openxmlformats.wordprocessingml.document",
    conforms_to: "org.openxmlformats.openxml|public.composite-content",
    tags: "",
    filename_extension: "docx",
    mime_type: "docx",
    description: "Office Open XML word processing document",
};
pub const ORG_OPENXMLFORMATS_WORDPROCESSINGML_DOCUMENT_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.wordprocessingml.document.macroenabled",
    conforms_to: "org.openxmlformats.openxml|public.composite-content|public.executable",
    tags: "",
    filename_extension: "docm",
    mime_type: "docm",
    description: "Office Open XML word processing document (macros enabled)",
};
pub const ORG_OPENXMLFORMATS_WORDPROCESSINGML_TEMPLATE: UTType = UTType {
    identifier: "org.openxmlformats.wordprocessingml.template",
    conforms_to: "org.openxmlformats.openxml|public.composite-content",
    tags: "",
    filename_extension: "dotx",
    mime_type: "dotx",
    description: "Office Open XML word processing template",
};
pub const ORG_OPENXMLFORMATS_WORDPROCESSINGML_TEMPLATE_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.wordprocessingml.template.macroenabled",
    conforms_to: "org.openxmlformats.openxml|public.composite-content|public.executable",
    tags: "",
    filename_extension: "dotm",
    mime_type: "dotm",
    description: "Office Open XML word processing template (macros enabled)",
};
pub const ORG_OPENXMLFORMATS_SPREADSHEETML_SHEET: UTType = UTType {
    identifier: "org.openxmlformats.spreadsheetml.sheet",
    conforms_to: "org.openxmlformats.openxml|public.composite-content|public.spreadsheet",
    tags: "",
    filename_extension: "xlsx",
    mime_type: "xlsx",
    description: "Office Open XML spreadsheet",
};
pub const ORG_OPENXMLFORMATS_SPREADSHEETML_SHEET_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.spreadsheetml.sheet.macroenabled",
    conforms_to:
        "org.openxmlformats.openxml|public.composite-content|public.spreadsheet|public.executable",
    tags: "",
    filename_extension: "xlsm",
    mime_type: "xlsm",
    description: "Office Open XML spreadsheet (macros enabled)",
};
pub const COM_MICROSOFT_EXCEL_SHEET_BINARY_MACROENABLED: UTType = UTType {
    identifier: "com.microsoft.excel.sheet.binary.macroenabled",
    conforms_to: "public.zip-archive|public.composite-content|public.spreadsheet|public.executable",
    tags: "",
    filename_extension: "xlsb",
    mime_type: "xlsb",
    description: "Microsoft Excel 2007 spreadsheet (macros enabled)",
};
pub const ORG_OPENXMLFORMATS_SPREADSHEETML_TEMPLATE: UTType = UTType {
    identifier: "org.openxmlformats.spreadsheetml.template",
    conforms_to: "org.openxmlformats.openxml|public.composite-content|public.spreadsheet",
    tags: "",
    filename_extension: "xltx",
    mime_type: "xltx",
    description: "Office Open XML spreadsheet template",
};
pub const ORG_OPENXMLFORMATS_SPREADSHEETML_TEMPLATE_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.spreadsheetml.template.macroenabled",
    conforms_to:
        "org.openxmlformats.openxml|public.composite-content|public.spreadsheet|public.executable",
    tags: "",
    filename_extension: "xltm",
    mime_type: "xltm",
    description: "Office Open XML spreadsheet template (macros enabled)",
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_PRESENTATION: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.presentation",
    conforms_to: "org.openxmlformats.openxml|public.presentation",
    tags: "",
    filename_extension: "pptx",
    mime_type: "pptx",
    description: "Office Open XML presentation",
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_PRESENTATION_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.presentation.macroenabled",
    conforms_to: "org.openxmlformats.openxml|public.presentation|public.executable",
    tags: "",
    filename_extension: "pptm",
    mime_type: "pptm",
    description: "Office Open XML presentation (macros enabled)",
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_SLIDESHOW: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.slideshow",
    conforms_to: "org.openxmlformats.openxml|public.presentation",
    tags: "",
    filename_extension: "ppsx",
    mime_type: "ppsx",
    description: "Office Open XML slide show",
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_SLIDESHOW_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.slideshow.macroenabled",
    conforms_to: "org.openxmlformats.openxml|public.presentation|public.executable",
    tags: "",
    filename_extension: "ppsm",
    mime_type: "ppsm",
    description: "Office Open XML slide show (macros enabled)",
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_TEMPLATE: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.template",
    conforms_to: "org.openxmlformats.openxml|public.presentation",
    tags: "",
    filename_extension: "potx",
    mime_type: "potx",
    description: "Office Open XML presentation template",
};
pub const ORG_OPENXMLFORMATS_PRESENTATIONML_TEMPLATE_MACROENABLED: UTType = UTType {
    identifier: "org.openxmlformats.presentationml.template.macroenabled",
    conforms_to: "org.openxmlformats.openxml|public.presentation|public.executable",
    tags: "",
    filename_extension: "potm",
    mime_type: "potm",
    description: "Office Open XML presentation template (macros enabled)",
};
pub const COM_MICROSOFT_WORD_DOC: UTType = UTType {
    identifier: "com.microsoft.word.doc",
    conforms_to: "public.data|public.composite-content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Microsoft Word 97-2004 document",
};
pub const COM_MICROSOFT_WORD_DOT: UTType = UTType {
    identifier: "com.microsoft.word.dot",
    conforms_to: "public.data|public.composite-content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Microsoft Word 97-2004 template",
};
pub const COM_MICROSOFT_EXCEL_XLS: UTType = UTType {
    identifier: "com.microsoft.excel.xls",
    conforms_to: "public.data|public.composite-content|public.spreadsheet",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Microsoft Excel 97-2004 worksheet",
};
pub const COM_MICROSOFT_EXCEL_XLT: UTType = UTType {
    identifier: "com.microsoft.excel.xlt",
    conforms_to: "public.data|public.spreadsheet",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Microsoft Excel 97-2004 template",
};
pub const COM_MICROSOFT_EXCEL_XLW: UTType = UTType {
    identifier: "com.microsoft.excel.xlw",
    conforms_to: "public.data|public.spreadsheet",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Microsoft Excel 97-2004 workspace",
};
pub const COM_MICROSOFT_EXCEL_XLA: UTType = UTType {
    identifier: "com.microsoft.excel.xla",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "xla",
    mime_type: "xla",
    description: "Microsoft Excel add-in",
};
pub const COM_MICROSOFT_POWERPOINT_PPT: UTType = UTType {
    identifier: "com.microsoft.powerpoint.ppt",
    conforms_to: "public.data|public.presentation",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Microsoft PowerPoint 97-2004 presentation",
};
pub const COM_MICROSOFT_POWERPOINT_POT: UTType = UTType {
    identifier: "com.microsoft.powerpoint.pot",
    conforms_to: "public.data|public.presentation",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Microsoft PowerPoint 97-2004 template",
};
pub const COM_MICROSOFT_POWERPOINT_PPS: UTType = UTType {
    identifier: "com.microsoft.powerpoint.pps",
    conforms_to: "public.data|public.presentation",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Microsoft PowerPoint 97-2004 slide show",
};
pub const COM_APPLE_KEYNOTE_KEY: UTType = UTType {
    identifier: "com.apple.keynote.key",
    conforms_to: "com.apple.package|public.presentation",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Keynote document",
};
pub const COM_APPLE_KEYNOTE_KTH: UTType = UTType {
    identifier: "com.apple.keynote.kth",
    conforms_to: "com.apple.package|public.presentation",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Keynote theme",
};
pub const COM_APPLE_IWORK_KEYNOTE_KEY: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.key",
    conforms_to: "com.apple.package|public.presentation|com.apple.keynote.key",
    tags: "",
    filename_extension: "key",
    mime_type: "key",
    description: "Keynote document",
};
pub const COM_APPLE_IWORK_KEYNOTE_KEY_TEF: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.key-tef",
    conforms_to: "com.apple.package|public.presentation|com.apple.keynote.key",
    tags: "",
    filename_extension: "key-tef",
    mime_type: "key-tef",
    description: "Keynote document",
};
pub const COM_APPLE_IWORK_KEYNOTE_SFFKEY: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.sffkey",
    conforms_to: "public.data|public.presentation",
    tags: "",
    filename_extension: "key",
    mime_type: "key",
    description: "Keynote document",
};
pub const COM_APPLE_IWORK_KEYNOTE_KTH: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.kth",
    conforms_to: "com.apple.package|public.presentation|com.apple.keynote.kth",
    tags: "",
    filename_extension: "kth",
    mime_type: "kth",
    description: "Keynote theme",
};
pub const COM_APPLE_IWORK_KEYNOTE_SFFKTH: UTType = UTType {
    identifier: "com.apple.iWork.Keynote.sffkth",
    conforms_to: "public.data|public.presentation",
    tags: "",
    filename_extension: "kth",
    mime_type: "kth",
    description: "Keynote theme",
};
pub const COM_APPLE_IWORK_PAGES_PAGES: UTType = UTType {
    identifier: "com.apple.iWork.Pages.pages",
    conforms_to: "com.apple.package|public.composite-content",
    tags: "",
    filename_extension: "pages",
    mime_type: "pages",
    description: "Pages document",
};
pub const COM_APPLE_IWORK_PAGES_PAGES_TEF: UTType = UTType {
    identifier: "com.apple.iWork.Pages.pages-tef",
    conforms_to: "com.apple.package|public.composite-content",
    tags: "",
    filename_extension: "pages-tef",
    mime_type: "pages-tef",
    description: "Pages document",
};
pub const COM_APPLE_IWORK_PAGES_SFFPAGES: UTType = UTType {
    identifier: "com.apple.iWork.Pages.sffpages",
    conforms_to: "public.data|public.composite-content",
    tags: "",
    filename_extension: "pages",
    mime_type: "pages",
    description: "Pages document",
};
pub const COM_APPLE_IWORK_PAGES_TEMPLATE: UTType = UTType {
    identifier: "com.apple.iWork.Pages.template",
    conforms_to: "com.apple.package|public.composite-content",
    tags: "",
    filename_extension: "template",
    mime_type: "template",
    description: "Pages template",
};
pub const COM_APPLE_IWORK_PAGES_SFFTEMPLATE: UTType = UTType {
    identifier: "com.apple.iWork.Pages.sfftemplate",
    conforms_to: "public.data|public.composite-content",
    tags: "",
    filename_extension: "template",
    mime_type: "template",
    description: "Pages template",
};
pub const COM_APPLE_IWORK_NUMBERS_NUMBERS: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.numbers",
    conforms_to: "com.apple.package|public.composite-content|public.spreadsheet",
    tags: "",
    filename_extension: "numbers",
    mime_type: "numbers",
    description: "Numbers document",
};
pub const COM_APPLE_IWORK_NUMBERS_NUMBERS_TEF: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.numbers-tef",
    conforms_to: "com.apple.package|public.composite-content|public.spreadsheet",
    tags: "",
    filename_extension: "numbers-tef",
    mime_type: "numbers-tef",
    description: "Numbers document",
};
pub const COM_APPLE_IWORK_NUMBERS_SFFNUMBERS: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.sffnumbers",
    conforms_to: "public.data|public.composite-content|public.spreadsheet",
    tags: "",
    filename_extension: "numbers",
    mime_type: "numbers",
    description: "Numbers document",
};
pub const COM_APPLE_IWORK_NUMBERS_TEMPLATE: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.template",
    conforms_to: "com.apple.package|public.composite-content|public.spreadsheet",
    tags: "",
    filename_extension: "nmbtemplate",
    mime_type: "nmbtemplate",
    description: "Numbers template",
};
pub const COM_APPLE_IWORK_NUMBERS_SFFTEMPLATE: UTType = UTType {
    identifier: "com.apple.iWork.Numbers.sfftemplate",
    conforms_to: "public.data|public.composite-content|public.spreadsheet",
    tags: "",
    filename_extension: "nmbtemplate",
    mime_type: "nmbtemplate",
    description: "Numbers template",
};
pub const COM_APPLE_GARAGEBAND_PROJECT: UTType = UTType {
    identifier: "com.apple.garageband.project",
    conforms_to: "com.apple.package|public.audiovisual-content",
    tags: "",
    filename_extension: "band|gbProj",
    mime_type: "band|gbProj",
    description: "GarageBand Project",
};
pub const COM_ADOBE_PHOTOSHOP_IMAGE: UTType = UTType {
    identifier: "com.adobe.photoshop-image",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Adobe Photoshop document",
};
pub const COM_ADOBE_PHOTOSHOP_LARGE_IMAGE: UTType = UTType {
    identifier: "com.adobe.photoshop-large-image",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Adobe Photoshop large document",
};
pub const COM_ADOBE_ILLUSTRATOR_AI_IMAGE: UTType = UTType {
    identifier: "com.adobe.illustrator.ai-image",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Adobe Illustrator document",
};
pub const COM_TRUEVISION_TGA_IMAGE: UTType = UTType {
    identifier: "com.truevision.tga-image",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "TGA image",
};
pub const COM_SGI_SGI_IMAGE: UTType = UTType {
    identifier: "com.sgi.sgi-image",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "sgi",
    mime_type: "sgi",
    description: "Silicon Graphics image",
};
pub const COM_ILM_OPENEXR_IMAGE: UTType = UTType {
    identifier: "com.ilm.openexr-image",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "exr",
    mime_type: "exr",
    description: "OpenEXR image",
};
pub const COM_KODAK_FLASHPIX_IMAGE: UTType = UTType {
    identifier: "com.kodak.flashpix-image",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "fpx",
    mime_type: "fpx",
    description: "FlashPix image",
};
pub const PUBLIC_HEIF_STANDARD: UTType = UTType {
    identifier: "public.heif-standard",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "HEIF Image",
};
pub const PUBLIC_HEIF: UTType = UTType {
    identifier: "public.heif",
    conforms_to: "public.heif-standard",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const PUBLIC_HEIC: UTType = UTType {
    identifier: "public.heic",
    conforms_to: "public.heif-standard",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const PUBLIC_AVCI: UTType = UTType {
    identifier: "public.avci",
    conforms_to: "public.heif-standard",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "",
};
pub const PUBLIC_HEIFS: UTType = UTType {
    identifier: "public.heifs",
    conforms_to: "public.heif-standard",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "HEIF Image Sequence",
};
pub const PUBLIC_HEICS: UTType = UTType {
    identifier: "public.heics",
    conforms_to: "public.heif-standard",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "HEIF Image Sequence",
};
pub const PUBLIC_AVCS: UTType = UTType {
    identifier: "public.avcs",
    conforms_to: "public.heif-standard",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "HEIF Image Sequence",
};
pub const COM_APPLE_DRAWING: UTType = UTType {
    identifier: "com.apple.drawing",
    conforms_to: "public.image",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Apple Drawing Format",
};
pub const COM_J2_JFX_FAX: UTType = UTType {
    identifier: "com.j2.jfx-fax",
    conforms_to: "public.fax",
    tags: "",
    filename_extension: "jfx",
    mime_type: "jfx",
    description: "J2 fax",
};
pub const COM_J2_EFX_FAX: UTType = UTType {
    identifier: "com.j2.efx-fax",
    conforms_to: "public.fax",
    tags: "",
    filename_extension: "efx",
    mime_type: "efx",
    description: "eFax fax",
};
pub const COM_DIGIDESIGN_SD2_AUDIO: UTType = UTType {
    identifier: "com.digidesign.sd2-audio",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Sound Designer II audio",
};
pub const COM_MICROSOFT_WAVEFORM_AUDIO: UTType = UTType {
    identifier: "com.microsoft.waveform-audio",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "wav|wave|bwf",
    mime_type: "wav|wave|bwf",
    description: "Waveform audio",
};
pub const COM_MICROSOFT_ADVANCED_SYSTEMS_FORMAT: UTType = UTType {
    identifier: "com.microsoft.advanced-systems-format",
    conforms_to: "public.audiovisual-content",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Microsoft Advanced Systems Format",
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WM: UTType = UTType {
    identifier: "com.microsoft.windows-media-wm",
    conforms_to: "public.movie|com.microsoft.advanced-systems-format",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Windows media",
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WMV: UTType = UTType {
    identifier: "com.microsoft.windows-media-wmv",
    conforms_to: "public.movie|com.microsoft.advanced-systems-format",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Windows media",
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WMP: UTType = UTType {
    identifier: "com.microsoft.windows-media-wmp",
    conforms_to: "public.movie|com.microsoft.advanced-systems-format",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Windows media",
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WMA: UTType = UTType {
    identifier: "com.microsoft.windows-media-wma",
    conforms_to: "public.audio|com.microsoft.advanced-systems-format",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Windows media audio",
};
pub const COM_MICROSOFT_ADVANCED_STREAM_REDIRECTOR: UTType = UTType {
    identifier: "com.microsoft.advanced-stream-redirector",
    conforms_to: "public.audiovisual-content|public.xml",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Advanced Stream Redirector",
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WMX: UTType = UTType {
    identifier: "com.microsoft.windows-media-wmx",
    conforms_to: "public.movie|com.microsoft.advanced-stream-redirector",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Windows media",
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WVX: UTType = UTType {
    identifier: "com.microsoft.windows-media-wvx",
    conforms_to: "public.movie|com.microsoft.advanced-stream-redirector",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Windows media",
};
pub const COM_MICROSOFT_WINDOWS_MEDIA_WAX: UTType = UTType {
    identifier: "com.microsoft.windows-media-wax",
    conforms_to: "public.audio|com.microsoft.advanced-stream-redirector",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Windows media audio",
};
pub const COM_REAL_REALMEDIA: UTType = UTType {
    identifier: "com.real.realmedia",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "RealMedia",
};
pub const COM_REAL_REALMEDIA_VBR: UTType = UTType {
    identifier: "com.real.realmedia-vbr",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "RealMedia Variable Bitrate",
};
pub const ORG_SMPTE_MXF: UTType = UTType {
    identifier: "org.smpte.mxf",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Material eXchange Format",
};
pub const COM_REAL_REALAUDIO: UTType = UTType {
    identifier: "com.real.realaudio",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "RealMedia Audio",
};
pub const COM_SOUNDBLASTER_SOUNDFONT: UTType = UTType {
    identifier: "com.soundblaster.soundfont",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "sf2",
    mime_type: "sf2",
    description: "SoundFont audio",
};
pub const ORG_XIPH_FLAC: UTType = UTType {
    identifier: "org.xiph.flac",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "FLAC audio",
};
pub const COM_AVID_OPEN_MEDIA_FRAMEWORK: UTType = UTType {
    identifier: "com.avid.open-media-framework",
    conforms_to: "public.audiovisual-content",
    tags: "",
    filename_extension: "omf",
    mime_type: "omf",
    description: "Open Media Framework interchange format",
};
pub const PUBLIC_MP4A_LOAS: UTType = UTType {
    identifier: "public.mp4a-loas",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "loas|latm",
    mime_type: "loas|latm",
    description: "Low Overhead MPEG-4 Audio Stream",
};
pub const PUBLIC_MP4A_LATM: UTType = UTType {
    identifier: "public.mp4a-latm",
    conforms_to: "public.audio",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Low-overhead MPEG-4 Audio Transport Multiplex",
};
pub const COM_ALLUME_STUFFIT_ARCHIVE: UTType = UTType {
    identifier: "com.allume.stuffit-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "StuffIt archive family",
};
pub const COM_STUFFIT_ARCHIVE_SITX: UTType = UTType {
    identifier: "com.stuffit.archive.sitx",
    conforms_to: "com.allume.stuffit-archive",
    tags: "",
    filename_extension: "sitx",
    mime_type: "sitx",
    description: "StuffIt X archive",
};
pub const COM_STUFFIT_ARCHIVE_SIDX: UTType = UTType {
    identifier: "com.stuffit.archive.sidx",
    conforms_to: "com.allume.stuffit-archive",
    tags: "",
    filename_extension: "sidx",
    mime_type: "sidx",
    description: "StuffIt X index",
};
pub const COM_STUFFIT_ARCHIVE_SIT: UTType = UTType {
    identifier: "com.stuffit.archive.sit",
    conforms_to: "com.allume.stuffit-archive",
    tags: "",
    filename_extension: "sit|sea",
    mime_type: "sit|sea",
    description: "StuffIt archive",
};
pub const COM_ADOBE_FLASH_VIDEO: UTType = UTType {
    identifier: "com.adobe.flash.video",
    conforms_to: "public.movie",
    tags: "",
    filename_extension: "flv|f4v|f4p|f4a|f4b",
    mime_type: "flv|f4v|f4p|f4a|f4b",
    description: "Flash video",
};
pub const ORG_KHRONOS_COLLADA_DIGITAL_ASSET_EXCHANGE: UTType = UTType {
    identifier: "org.khronos.collada.digital-asset-exchange",
    conforms_to: "public.xml|public.audiovisual-content|public.3d-content",
    tags: "",
    filename_extension: "dae",
    mime_type: "dae",
    description: "Digital Asset Exchange (DAE)",
};
pub const COM_APPLE_IMOVIELIBRARY: UTType = UTType {
    identifier: "com.apple.iMovieLibrary",
    conforms_to: "com.apple.package",
    tags: "",
    filename_extension: "imovielibrary",
    mime_type: "imovielibrary",
    description: "iMovie Library",
};
pub const COM_APPLE_IMOVIETHEATER: UTType = UTType {
    identifier: "com.apple.iMovieTheater",
    conforms_to: "com.apple.package",
    tags: "",
    filename_extension: "theater",
    mime_type: "theater",
    description: "iMovie Theater",
};
pub const ORG_7_ZIP_7_ZIP_ARCHIVE: UTType = UTType {
    identifier: "org.7-zip.7-zip-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "7-Zip archive",
};
pub const ORG_TUKAANI_XZ_ARCHIVE: UTType = UTType {
    identifier: "org.tukaani.xz-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "xz compressed archive",
};
pub const ORG_TUKAANI_TAR_XZ_ARCHIVE: UTType = UTType {
    identifier: "org.tukaani.tar-xz-archive",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "xz compressed tar archive",
};
pub const COM_MICROSOFT_CAB: UTType = UTType {
    identifier: "com.microsoft.cab",
    conforms_to: "public.data|public.archive",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Microsoft Cabinet archive",
};
pub const PUBLIC_HAPTICS_CONTENT: UTType = UTType {
    identifier: "public.haptics-content",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Haptics content",
};
pub const COM_APPLE_HAPTICS_AHAP: UTType = UTType {
    identifier: "com.apple.haptics.ahap",
    conforms_to: "public.haptics-content|public.json",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Apple Haptics Audio Pattern",
};
pub const COM_APPLE_COREML_MODEL: UTType = UTType {
    identifier: "com.apple.coreml.model",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "mlmodel|mlkitmodel",
    mime_type: "mlmodel|mlkitmodel",
    description: "Core ML Machine Learning Model",
};
pub const COM_APPLE_COREML_MLPACKAGE: UTType = UTType {
    identifier: "com.apple.coreml.mlpackage",
    conforms_to: "com.apple.package",
    tags: "",
    filename_extension: "mlpackage",
    mime_type: "mlpackage",
    description: "Core ML Machine Learning Model Package",
};
pub const COM_APPLE_GROUPACTIVITIES_ACTIVITY: UTType = UTType {
    identifier: "com.apple.groupactivities.activity",
    conforms_to: "public.data",
    tags: "",
    filename_extension: "groupactivity",
    mime_type: "groupactivity",
    description: "Group Activity",
};
pub const COM_APPLE_ICON_DECORATION: UTType = UTType {
    identifier: "com.apple.icon-decoration",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration",
};
pub const COM_APPLE_ICON_DECORATION_POSITION: UTType = UTType {
    identifier: "com.apple.icon-decoration-position",
    conforms_to: "",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Position",
};
pub const COM_APPLE_ICON_DECORATION_POSITION_CENTER: UTType = UTType {
    identifier: "com.apple.icon-decoration-position.center",
    conforms_to: "com.apple.icon-decoration-position",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Position Center",
};
pub const COM_APPLE_ICON_DECORATION_POSITION_LEADING_BOTTOM: UTType = UTType {
    identifier: "com.apple.icon-decoration-position.leading-bottom",
    conforms_to: "com.apple.icon-decoration-position",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Position Leading Bottom",
};
pub const COM_APPLE_ICON_DECORATION_POSITION_TRAILING_BOTTOM: UTType = UTType {
    identifier: "com.apple.icon-decoration-position.trailing-bottom",
    conforms_to: "com.apple.icon-decoration-position",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Position Trailing Bottom",
};
pub const COM_APPLE_ICON_DECORATION_POSITION_OVERLAY: UTType = UTType {
    identifier: "com.apple.icon-decoration-position.overlay",
    conforms_to: "com.apple.icon-decoration-position",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Position Overlay",
};
pub const COM_APPLE_ICON_DECORATION_BADGE: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge",
    conforms_to: "com.apple.icon-decoration-position.trailing-bottom|com.apple.icon-decoration",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Badge",
};
pub const COM_APPLE_ICON_DECORATION_EMBOSS: UTType = UTType {
    identifier: "com.apple.icon-decoration.emboss",
    conforms_to: "com.apple.icon-decoration-position.center|com.apple.icon-decoration",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Emboss",
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM: UTType = UTType {
    identifier: "com.apple.icon-decoration.system",
    conforms_to: "com.apple.icon-decoration",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration System",
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM_UNSUPPORTED: UTType = UTType {
    identifier: "com.apple.icon-decoration.system.unsupported",
    conforms_to: "com.apple.icon-decoration.system",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Unsupported",
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM_CAUTION_ALERT: UTType = UTType {
    identifier: "com.apple.icon-decoration.system.caution-alert",
    conforms_to: "com.apple.icon-decoration.system",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Alert Caution",
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM_ALIAS: UTType = UTType {
    identifier: "com.apple.icon-decoration.system.alias",
    conforms_to: "com.apple.icon-decoration.system",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Alias",
};
pub const COM_APPLE_ICON_DECORATION_SYSTEM_NEW_FOLDER: UTType = UTType {
    identifier: "com.apple.icon-decoration.system.new-folder",
    conforms_to: "com.apple.icon-decoration.system",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration New Folder",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_LOCKED: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.locked",
    conforms_to:
        "com.apple.icon-decoration-position.leading-bottom|com.apple.icon-decoration.system",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Locked Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_CHECKMARK: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.checkmark",
    conforms_to:
        "com.apple.icon-decoration-position.trailing-bottom|com.apple.icon-decoration.badge",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Checkmark Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_COMMENTS: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.comments",
    conforms_to:
        "com.apple.icon-decoration-position.trailing-bottom|com.apple.icon-decoration.badge",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Comments Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_DROP_FOLDER: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.drop-folder",
    conforms_to:
        "com.apple.icon-decoration-position.trailing-bottom|com.apple.icon-decoration.badge",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Drop Folder Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_HEART: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.heart",
    conforms_to:
        "com.apple.icon-decoration-position.trailing-bottom|com.apple.icon-decoration.badge",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Heart Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_IN_REVIEW: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.in-review",
    conforms_to:
        "com.apple.icon-decoration-position.trailing-bottom|com.apple.icon-decoration.badge",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration In Review Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_LOCKED_BY_COLLABORATOR: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.locked-by-collaborator",
    conforms_to:
        "com.apple.icon-decoration-position.trailing-bottom|com.apple.icon-decoration.badge",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Locked By Collaborator Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_LOCKED_BY_USER: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.locked-by-user",
    conforms_to:
        "com.apple.icon-decoration-position.trailing-bottom|com.apple.icon-decoration.badge",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Locked By User Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_PINNED: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.pinned",
    conforms_to:
        "com.apple.icon-decoration-position.trailing-bottom|com.apple.icon-decoration.badge",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Pinned Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_PRIVATE_FOLDER: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.private-folder",
    conforms_to:
        "com.apple.icon-decoration-position.trailing-bottom|com.apple.icon-decoration.badge",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Private Folder Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_SYNCING: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.syncing",
    conforms_to:
        "com.apple.icon-decoration-position.trailing-bottom|com.apple.icon-decoration.badge",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Syncing Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_TRENDING: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.trending",
    conforms_to:
        "com.apple.icon-decoration-position.trailing-bottom|com.apple.icon-decoration.badge",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Trending Badge",
};
pub const COM_APPLE_ICON_DECORATION_BADGE_WARNING: UTType = UTType {
    identifier: "com.apple.icon-decoration.badge.warning",
    conforms_to:
        "com.apple.icon-decoration-position.trailing-bottom|com.apple.icon-decoration.badge",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Icon Decoration Warning Badge",
};
pub const COM_APPLE_DOCUMENT_TYPE: UTType = UTType {
    identifier: "com.apple.document-type",
    conforms_to: "public.item",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Document Type",
};
pub const COM_APPLE_DOCUMENT_TYPE_DICTIONARY: UTType = UTType {
    identifier: "com.apple.document-type.dictionary",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Dictionary",
};
pub const COM_APPLE_ACCOUNTS_ICON: UTType = UTType {
    identifier: "com.apple.accounts-icon",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Accounts Icon",
};
pub const COM_APPLE_LEGACY_ACTIONS_ICON: UTType = UTType {
    identifier: "com.apple.legacy.actions-icon",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Actions Icon",
};
pub const COM_APPLE_EVERYONE_ICON: UTType = UTType {
    identifier: "com.apple.everyone-icon",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Everyone Icon",
};
pub const COM_APPLE_LEGACY_GENERAL_ICON: UTType = UTType {
    identifier: "com.apple.legacy.general-icon",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "General Icon",
};
pub const COM_APPLE_LEGACY_SIDEBAR_PREFS_ICON: UTType = UTType {
    identifier: "com.apple.legacy.sidebar-prefs-icon",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Sidebar Prefs Icon",
};
pub const COM_APPLE_LEGACY_TOOLBAR_ADVANCED_ICON: UTType = UTType {
    identifier: "com.apple.legacy.toolbar-advanced-icon",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Toolbar Advanced Icon",
};
pub const COM_APPLE_LEGACY_TOOLBAR_INFO_ICON: UTType = UTType {
    identifier: "com.apple.legacy.toolbar-info-icon",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Toolbar Info Icon",
};
pub const COM_APPLE_LEGACY_TOOLBAR_LABELS_ICON: UTType = UTType {
    identifier: "com.apple.legacy.toolbar-labels-icon",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Toolbar Labels Icon",
};
pub const COM_APPLE_LEGACY_CLOCK_ICON: UTType = UTType {
    identifier: "com.apple.legacy.clock-icon",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Clock Icon",
};
pub const COM_APPLE_LEGACY_SYNCHRONIZE: UTType = UTType {
    identifier: "com.apple.legacy.synchronize",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Synchronize",
};
pub const COM_APPLE_ICON_OVERLAY_NEW_FOLDER_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.new-folder-badge",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "New Folder Badge",
};
pub const COM_APPLE_LEGACY_FINDER_ICON: UTType = UTType {
    identifier: "com.apple.legacy.finder-icon",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Finder",
};
pub const COM_APPLE_UNKNOWN_OBJECT: UTType = UTType {
    identifier: "com.apple.unknown-object",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Unknown Object",
};
pub const COM_APPLE_NOT_LOADED: UTType = UTType {
    identifier: "com.apple.not-loaded",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Not Loaded",
};
pub const COM_APPLE_LEGACY_WINDOW: UTType = UTType {
    identifier: "com.apple.legacy.window",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Window",
};
pub const COM_APPLE_LEGACY_QUESTION_MARK: UTType = UTType {
    identifier: "com.apple.legacy.question-mark",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Question Mark",
};
pub const COM_APPLE_LEGACY_EJECT_MEDIA: UTType = UTType {
    identifier: "com.apple.legacy.eject-media",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Eject Media",
};
pub const COM_APPLE_LEGACY_BURN: UTType = UTType {
    identifier: "com.apple.legacy.burn",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Burn",
};
pub const COM_APPLE_LEGACY_CUSTOMIZE_TOOLBAR: UTType = UTType {
    identifier: "com.apple.legacy.customize-toolbar",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Customize Toolbar",
};
pub const COM_APPLE_LEGACY_DELETE_TOOLBAR: UTType = UTType {
    identifier: "com.apple.legacy.delete-toolbar",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Delete Toolbar",
};
pub const COM_APPLE_LEGACY_RIGHT_CONTAINER_ARROW: UTType = UTType {
    identifier: "com.apple.legacy.right-container-arrow",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Right Container Arrow",
};
pub const COM_APPLE_ICON_OVERLAY_DROP_FOLDER_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.drop-folder-badge",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Drop Folder Badge",
};
pub const COM_APPLE_ICON_OVERLAY_PRIVATE_FOLDER_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.private-folder-badge",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Private Folder Badge",
};
pub const COM_APPLE_ICON_OVERLAY_PRIVATE_FOLDER: UTType = UTType {
    identifier: "com.apple.icon-overlay.private-folder",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Private Folder",
};
pub const COM_APPLE_LEGACY_OPEN_FOLDER: UTType = UTType {
    identifier: "com.apple.legacy.open-folder",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Open Folder",
};
pub const COM_APPLE_LEGACY_FAVORITE_ITEMS: UTType = UTType {
    identifier: "com.apple.legacy.favorite-items",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Favorite Items",
};
pub const COM_APPLE_LEGACY_LOCKED: UTType = UTType {
    identifier: "com.apple.legacy.locked",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Locked",
};
pub const COM_APPLE_LEGACY_UNLOCKED: UTType = UTType {
    identifier: "com.apple.legacy.unlocked",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Unlocked",
};
pub const COM_APPLE_LEGACY_NO_WRITE: UTType = UTType {
    identifier: "com.apple.legacy.no-write",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "No Write",
};
pub const COM_APPLE_LEGACY_KEEP_ARRANGED: UTType = UTType {
    identifier: "com.apple.legacy.keep-arranged",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Keep Arranged",
};
pub const COM_APPLE_LEGACY_GRID: UTType = UTType {
    identifier: "com.apple.legacy.grid",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Grid",
};
pub const COM_APPLE_LEGACY_CONNECT_TO: UTType = UTType {
    identifier: "com.apple.legacy.connect-to",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Connect To",
};
pub const COM_APPLE_LEGACY_BACKWARD_ARROW: UTType = UTType {
    identifier: "com.apple.legacy.backward-arrow",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Backward Arrow",
};
pub const COM_APPLE_LEGACY_FORWARD_ARROW: UTType = UTType {
    identifier: "com.apple.legacy.forward-arrow",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Forward Arrow",
};
pub const COM_APPLE_ICON_OVERLAY_LOCKED_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.locked-badge",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Locked Badge",
};
pub const COM_APPLE_ICON_OVERLAY_ALIAS_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.alias-badge",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Alias Badge",
};
pub const COM_APPLE_ICON_OVERLAY_ALERT_CAUTION_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.alert-caution-badge",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Alert Caution Badge",
};
pub const COM_APPLE_ICON_OVERLAY_UNSUPPORTED_BADGE: UTType = UTType {
    identifier: "com.apple.icon-overlay.unsupported-badge",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Unsupported Badge",
};
pub const COM_APPLE_LEGACY_MAGNIFYING_GLASS: UTType = UTType {
    identifier: "com.apple.legacy.magnifying-glass",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Magnifying Glass",
};
pub const COM_APPLE_LEGACY_ERASING: UTType = UTType {
    identifier: "com.apple.legacy.erasing",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
    description: "Erasing",
};
pub const COM_APPLE_LEGACY_MULTIPLE_ITEMS: UTType = UTType {
    identifier: "com.apple.legacy.multiple-items",
    conforms_to: "com.apple.document-type",
    tags: "",
    filename_extension: "",
    mime_type: "",
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
        mime_type: "image/x-portable-bitmap",
        extensions: "pbm",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-tcl",
        extensions: "tcl",
    },
    MIMETypeAndExtension {
        mime_type: "text/vnd.wap.wml",
        extensions: "wml",
    },
    MIMETypeAndExtension {
        mime_type: "video/vnd.mpegurl",
        extensions: "mxu",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-msdownload",
        extensions: "dll",
    },
    MIMETypeAndExtension {
        mime_type: "model/vrml",
        extensions: "wrl|vrml",
    },
    MIMETypeAndExtension {
        mime_type: "text/richtext",
        extensions: "rtx",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-futuresplash",
        extensions: "spl",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-macpaint",
        extensions: "pnt|pntg|mac",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-cmu-raster",
        extensions: "ras",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-cpio",
        extensions: "cpio",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.fdf",
        extensions: "fdf",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-targa",
        extensions: "targa",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-diskcopy",
        extensions: "dmg",
    },
    MIMETypeAndExtension {
        mime_type: "text/sgml",
        extensions: "sgml|sgm",
    },
    MIMETypeAndExtension {
        mime_type: "audio/x-pn-realaudio-plugin",
        extensions: "rpm",
    },
    MIMETypeAndExtension {
        mime_type: "x-conference/x-cooltalk",
        extensions: "ice",
    },
    MIMETypeAndExtension {
        mime_type: "audio/mpeg",
        extensions: "mp3|mpga|mp2",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.mif",
        extensions: "mif",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.wap.wmlscriptc",
        extensions: "wmlsc",
    },
    MIMETypeAndExtension {
        mime_type: "application/mspowerpoint",
        extensions: "ppt",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-troff-me",
        extensions: "me",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-shar",
        extensions: "shar",
    },
    MIMETypeAndExtension {
        mime_type: "application/octet-stream",
        extensions: "dms|lha|lzh|class|so|iso|fla",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-sh",
        extensions: "sh",
    },
    MIMETypeAndExtension {
        mime_type: "model/iges",
        extensions: "igs|iges",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-sv4cpio",
        extensions: "sv4cpio",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-director",
        extensions: "dcr|dir|dxr",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-sv4crc",
        extensions: "sv4crc",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-cdlink",
        extensions: "vcd",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.wap.wmlc",
        extensions: "wmlc",
    },
    MIMETypeAndExtension {
        mime_type: "text/html",
        extensions: "html|jhtml",
    },
    MIMETypeAndExtension {
        mime_type: "application/oda",
        extensions: "oda",
    },
    MIMETypeAndExtension {
        mime_type: "application/xml",
        extensions: "xml",
    },
    MIMETypeAndExtension {
        mime_type: "model/mesh",
        extensions: "msh|mesh|silo",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-xpixmap",
        extensions: "xpm",
    },
    MIMETypeAndExtension {
        mime_type: "image/ief",
        extensions: "ief",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-troff",
        extensions: "t|tr|roff",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-netcdf",
        extensions: "nc|cdf",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-csh",
        extensions: "csh",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-tex",
        extensions: "tex",
    },
    MIMETypeAndExtension {
        mime_type: "chemical/x-pdb",
        extensions: "pdb",
    },
    MIMETypeAndExtension {
        mime_type: "text/vnd.wap.wmlscript",
        extensions: "wmls",
    },
    MIMETypeAndExtension {
        mime_type: "video/x-sgi-movie",
        extensions: "movie",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-gzip",
        extensions: "gz|tgz|cpgz",
    },
    MIMETypeAndExtension {
        mime_type: "text/x-setext",
        extensions: "etx",
    },
    MIMETypeAndExtension {
        mime_type: "text/xml",
        extensions: "xml|xsl",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-dvi",
        extensions: "dvi",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-pcx",
        extensions: "pcx",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.adobe.xfdf",
        extensions: "xfdf",
    },
    MIMETypeAndExtension {
        mime_type: "video/mp4",
        extensions: "mp4|mpg4",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.adobe.xfd+xml",
        extensions: "xfd",
    },
    MIMETypeAndExtension {
        mime_type: "application/msword",
        extensions: "doc",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.ms-powerpoint",
        extensions: "ppt",
    },
    MIMETypeAndExtension {
        mime_type: "application/msexcel",
        extensions: "xls",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-portable-pixmap",
        extensions: "ppm",
    },
    MIMETypeAndExtension {
        mime_type: "chemical/x-xyz",
        extensions: "xyz",
    },
    MIMETypeAndExtension {
        mime_type: "text/css",
        extensions: "css",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-xwindowdump",
        extensions: "xwd",
    },
    MIMETypeAndExtension {
        mime_type: "audio/aiff",
        extensions: "aiff",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-texinfo",
        extensions: "texinfo|texi",
    },
    MIMETypeAndExtension {
        mime_type: "application/postscript",
        extensions: "ps|eps|ai",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-bcpio",
        extensions: "bcpio",
    },
    MIMETypeAndExtension {
        mime_type: "image/vnd.wap.wbmp",
        extensions: "wbmp",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-ustar",
        extensions: "ustar",
    },
    MIMETypeAndExtension {
        mime_type: "audio/x-m4p",
        extensions: "m4p",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-wais-source",
        extensions: "src",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-troff-man",
        extensions: "man",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-shockwave-flash",
        extensions: "swf",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-koan",
        extensions: "skp|skd|skt|skm",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-latex",
        extensions: "latex",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-filemaker",
        extensions: "fp6|fp5|fp4|fp3|fp2|fp",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-portable-graymap",
        extensions: "pgm",
    },
    MIMETypeAndExtension {
        mime_type: "text/qif",
        extensions: "qif",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.wap.wbxml",
        extensions: "wbxml",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-chess-pgn",
        extensions: "pgn",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-hdf",
        extensions: "hdf",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-ms-wmd",
        extensions: "wmd",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-rgb",
        extensions: "rgb",
    },
    MIMETypeAndExtension {
        mime_type: "application/andrew-inset",
        extensions: "ez",
    },
    MIMETypeAndExtension {
        mime_type: "text/plain",
        extensions: "txt|asc",
    },
    MIMETypeAndExtension {
        mime_type: "audio/x-aiff",
        extensions: "aiff",
    },
    MIMETypeAndExtension {
        mime_type: "application/mac-compactpro",
        extensions: "cpt",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.adobe.xdp+xml",
        extensions: "xdp",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-olympus-or",
        extensions: "orf",
    },
    MIMETypeAndExtension {
        mime_type: "image/x-portable-anymap",
        extensions: "pnm",
    },
    MIMETypeAndExtension {
        mime_type: "application/vnd.ms-excel",
        extensions: "xls",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-bittorrent",
        extensions: "torrent",
    },
    MIMETypeAndExtension {
        mime_type: "application/x-troff-ms",
        extensions: "ms",
    },
];
