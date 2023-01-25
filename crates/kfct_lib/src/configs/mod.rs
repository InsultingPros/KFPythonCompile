pub mod helper;

/// app config's global section name
pub const GLOBAL_SECTION_NAME: &str = "global";
/// config name
pub const COMPILATION_CONFIG_NAME: &str = "kfcompile.ini";
/// included minimal kf.ini for ucc.exe
pub const COMPILATION_CONFIG_TEMPLATE: &str = include_str!("./static/kfcompile.ini");
/// Config name.
pub const APP_CONFIG_NAME: &str = "CompileSettings.ini";
/// Included default config file.
pub const APP_CONFIG_TEMPLATE: &str = include_str!("./static/CompileSettings.ini");
pub const STEAM_APPID_TXT: &str = "steam_appid.txt";

/// `Global` section of config file.
#[derive(Debug)]
pub struct SectionGlobal {
    /// Name of an existing local section.
    pub package_name: String,
    /// Directory of our sources
    pub dir_classes: String,
    /// Where we are compiling on.
    pub dir_compile: String,
    /// Move files to here after successful compilation.
    pub dir_move_to: Option<String>,
    /// Release folder.
    pub dir_release_output: Option<String>,
}

/// Per mod section of config file.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
pub struct SectionLocal {
    /// `EditPackages` of this mod.
    ///
    /// **This is not a list!** Separate dependencies by comma.
    pub edit_packages: String,
    /// Are our sources in the same directory as `dir_compile`.
    pub compile_outsideof_kf: bool,
    /// Are we using alternative source file organization style.
    pub alt_directories: bool,
    /// Move files to `dir_move_to`.
    pub move_files: bool,
    /// Create localization files.
    pub create_int: bool,
    /// Create redirect file.
    pub make_redirect: bool,
    /// Move compiled files to `dir_release_output`.
    pub make_release: bool,
}
