use configs::{SectionGlobal, SectionLocal, COMPILATION_CONFIG_NAME, STEAM_APPID_TXT};
use std::path::{PathBuf, StripPrefixError};
pub mod configs;
pub mod file_helper;
pub mod ucc_wrapper;

/// KF1 file extensions.
pub const UNREAL_PACKAGES: [&str; 4] = [".u", ".ucl", ".u.uz2", ".int"];
/// Filter for files-directories, so we copy-paste only source files.
pub const IGNORE_LIST: [&str; 4] = [".git", "*.md", "Docs", "LICENSE"];

#[derive(thiserror::Error, Debug)]
pub enum CompileToolErrors {
    // #[error("Some IO error?")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    WriteError(#[from] std::fmt::Error),
    #[error("{0}")]
    StringErrors(String),
    #[error(transparent)]
    ZipErrors(#[from] zip::result::ZipError),
    #[error(transparent)]
    WalkDirErrors(#[from] walkdir::Error),
    #[error(transparent)]
    PathErrors(#[from] StripPrefixError),
}

/// Define application exit codes, specific to each platforms
///
/// Reference: <https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499->
#[cfg(target_family = "windows")]
pub mod exit_codes {
    pub const ERROR_SUCCESS: u8 = 0;
    pub const ERROR_CANNOT_MAKE: u8 = 82;
    pub const ARGUMENT_PARSING_ERROR: u8 = 2;
    pub const ERROR_BAD_ARGUMENTS: u8 = 160;
}

/// Define application exit codes, specific to each platform
///
/// Reference: <https://unix.stackexchange.com/a/254747>
#[cfg(target_family = "unix")]
pub mod exit_codes {
    pub const ERROR_SUCCESS: u8 = 0;
    pub const ERROR_CANNOT_MAKE: u8 = 1;
    pub const ARGUMENT_PARSING_ERROR: u8 = 2;
    pub const ERROR_BAD_ARGUMENTS: u8 = 128;
}

#[derive(Debug, Default)]
/// some stupid superstruct for all important variables
pub struct RuntimeVariables {
    pub compile_options: CompileOptions,
    pub compiled_paths: CompilationPaths,
    pub path_where_to_copy: Option<PathBuf>,
    pub release_options: Option<ReleaseOptions>,
}

impl RuntimeVariables {
    fn new(global_section: &SectionGlobal, local_section: &SectionLocal) -> Self {
        Self {
            compile_options: CompileOptions::new(global_section, local_section),
            compiled_paths: CompilationPaths::new(global_section),
            path_where_to_copy: global_section.dir_move_to.clone().map(PathBuf::from),
            release_options: {
                global_section
                    .dir_release_output
                    .as_ref()
                    .map(ReleaseOptions::new)
            },
        }
    }
}

#[derive(Debug, Default)]
pub struct CompileOptions {
    /// Mod package name.
    pub package_name: String,
    /// Mod's `EditPackages`.
    pub edit_packages: Vec<String>,
    /// Create localization file.
    pub create_int: Option<bool>,
    /// Source files are somewhere else.
    pub i_compile_outsideof_kf: Option<bool>,
    /// Where do our source files lay.
    pub path_source_files: PathBuf,
    /// Use alternative source file organization.
    pub alt_directories: Option<bool>,
    /// Copy default config or no?
    pub copy_default_ini: Option<bool>,
}

impl CompileOptions {
    fn new(global_section: &SectionGlobal, local_section: &SectionLocal) -> Self {
        Self {
            package_name: global_section.package_name.clone(),
            edit_packages: local_section
                .edit_packages
                .split(',')
                .map(std::string::ToString::to_string)
                .collect(),
            create_int: Some(local_section.create_int),
            i_compile_outsideof_kf: Some(local_section.compile_outsideof_kf),
            path_source_files: global_section.dir_classes.clone().into(),
            alt_directories: Some(local_section.alt_directories),
            copy_default_ini: Some(true),
        }
    }
}

#[derive(Debug, Default)]
pub struct ReleaseOptions {
    pub make_zip: bool,
    pub release_path: PathBuf,
}

impl ReleaseOptions {
    fn new(dir: &String) -> Self {
        Self {
            make_zip: true,
            release_path: PathBuf::from(dir),
        }
    }
}

/// compiled files
#[derive(Debug, Default)]
pub struct CompilationPaths {
    /// Compilation main directory. For example "D:\\Dedicated Server".
    pub compile_dir: PathBuf,
    /// Compilation `System` directory. For example "D:\\Dedicated Server\\System".
    pub compile_dir_system: PathBuf,

    /// Path to `UCC.exe`. For example "D:\\Dedicated Server\\System\\UCC.exe".
    pub ucc_exe: PathBuf,
    /// Path to temporary kf.ini. For example "D:\\Dedicated Server\\System\\kfcompile.ini".
    pub temp_compilation_file: Option<PathBuf>,
    /// Path to hacked `steam_appid.txt`. For example "D:\\Dedicated Server\\System\\`steam_appid.txt`".
    /// # Should be deleted after compilation attempt!
    pub hacky_steam_appid_file: PathBuf,

    /// Path to compiled binary file. For example "D:\\Dedicated Server\\System\\`PACKAGE_NAME`.u".
    pub package_u: PathBuf,
    /// Path to compiled `ucl` file. For example "D:\\Dedicated Server\\System\\`PACKAGE_NAME`.ucl".
    pub package_ucl: PathBuf,
    /// Path to compiled localization file. For example "D:\\Dedicated Server\\System\\`PACKAGE_NAME`.int".
    pub package_int: Option<PathBuf>,
    /// Path to compiled mod's redirect file. For example "D:\\Dedicated Server\\System\\`PACKAGE_NAME`.uz2".
    pub package_uz2: Option<PathBuf>,
    /// Path to compiled mod's redirect file. For example "D:\\Dedicated Server\\System\\`PACKAGE_NAME`.ini".
    pub package_ini: Option<PathBuf>,

    /// Path to directory from where we copy-paste source files.
    /// "D:\\Mods\\`PACKAGE_NAME`"
    pub sources_path: Option<PathBuf>,
}

impl CompilationPaths {
    fn new(global_section: &SectionGlobal) -> Self {
        let compile_dir: PathBuf = PathBuf::from(global_section.dir_compile.clone());

        Self {
            compile_dir: compile_dir.clone(),
            compile_dir_system: compile_dir.join("System"),
            temp_compilation_file: Some(compile_dir.join("System").join(COMPILATION_CONFIG_NAME)),
            ucc_exe: compile_dir.join("System").join("UCC.exe"),
            sources_path: global_section.dir_move_to.as_ref().map(PathBuf::from),
            package_u: compile_dir.join(format!("System\\{}.u", global_section.package_name)),
            package_ucl: compile_dir.join(format!("System\\{}.ucl", global_section.package_name)),
            package_int: Some(
                compile_dir.join(format!("System\\{}.int", global_section.package_name)),
            ),
            package_uz2: Some(
                compile_dir.join(format!("System\\{}.uz2", global_section.package_name)),
            ),
            package_ini: Some(
                compile_dir.join(format!("System\\{}.ini", global_section.package_name)),
            ),
            hacky_steam_appid_file: compile_dir.join(format!("System\\{STEAM_APPID_TXT}")),
        }
    }
}
