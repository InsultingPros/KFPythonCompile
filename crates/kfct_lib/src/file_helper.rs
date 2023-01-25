use crate::{CompileToolErrors, RuntimeVariables};
use std::path::PathBuf;
use std::{
    fs::{self, OpenOptions},
    io::Write as _,
};
use walkdir::{DirEntry, WalkDir};
use zip::{write::SimpleFileOptions, CompressionMethod};
use zip_extensions::zip_create_from_directory_with_options;

/// a.
/// # Errors
///
/// Will return `Err` if failed to write to file.
pub fn create_hacky_steamappid(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    let mut hack_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(runtime_vars.compiled_paths.hacky_steam_appid_file.clone())?;
    hack_file.write_all(b"3")?;

    let metadata = hack_file.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_readonly(true);
    hack_file.set_permissions(permissions)?;

    Ok(())
}

/// a
/// # Errors
/// a
#[allow(clippy::permissions_set_readonly_false)]
pub fn remove_compilation_garbage(
    runtime_vars: &RuntimeVariables,
) -> Result<(), CompileToolErrors> {
    // kf.ini
    if let Some(kf_ini) = runtime_vars.compiled_paths.temp_compilation_file.clone() {
        fs::remove_file(kf_ini)?;
    }
    // steamappid.txt
    if runtime_vars.compiled_paths.hacky_steam_appid_file.exists() {
        let steamapp_id = runtime_vars.compiled_paths.hacky_steam_appid_file.clone();

        let metadata = fs::metadata(&steamapp_id)?;
        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);
        fs::set_permissions(&steamapp_id, permissions)?;
        fs::remove_file(runtime_vars.compiled_paths.hacky_steam_appid_file.clone())?;
    }

    Ok(())
}

/// a.
/// # Errors
///
/// Will return `Err` if failed to write to file.
pub fn get_walkdir_iterator(
    input_path: &PathBuf,
) -> Result<impl IntoIterator<Item = DirEntry>, CompileToolErrors> {
    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .is_some_and(|s| s.starts_with('.'))
    }

    let walker = WalkDir::new(input_path).into_iter();
    let result_iter = walker.filter_entry(|e| !is_hidden(e)).flatten();

    Ok(result_iter)
}

/// Compress `input_directory` to a zip file.
/// # Errors
///
/// Will return `Err` if failed to write to file.
pub fn new_compress_dir_to_zip(
    input_directory: &PathBuf,
    archive_file: &PathBuf,
) -> Result<(), CompileToolErrors> {
    let zip_options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    zip_create_from_directory_with_options(archive_file, input_directory, |_| zip_options)?;

    Ok(())
}

// pub fn prepare_sources_for_compilation(
//     runtime_vars: &RuntimeVariables,
// ) -> Result<(), CompileToolErrors> {
//     // do nothing
//     if runtime_vars
//         .compile_options
//         .i_compile_outsideof_kf
//         .is_none()
//     {
//         return Ok(());
//     }

//     // copy-files otherwise
//     if let Some(path) = &runtime_vars.compiled_paths.sources_path {
//         if let Ok(path_iterator) =
//             get_walkdir_iterator(&path.join(&runtime_vars.compile_options.package_name))
//         {
//             dbg!(&path.join(&runtime_vars.compile_options.package_name));
//             for file in path_iterator {
//                 println!("{file:#?}");
//             }
//         }
//     };

//     Ok(())
// }
