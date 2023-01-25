use crate::configs::{
    SectionGlobal, SectionLocal, APP_CONFIG_NAME, APP_CONFIG_TEMPLATE, COMPILATION_CONFIG_NAME,
    COMPILATION_CONFIG_TEMPLATE, GLOBAL_SECTION_NAME,
};
use crate::{CompileToolErrors, RuntimeVariables};
use configparser::ini::Ini;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

/// Check if our settings file exists in the same directory.
/// If not - create a default one and warn the user.
/// # Errors
///
/// Will return `Err` if failed to write to file.
pub fn check_app_config() -> Result<(), CompileToolErrors> {
    if !Path::new(APP_CONFIG_NAME).exists() {
        println!("{APP_CONFIG_NAME} DOESNT exist! Creating it!");
        fs::write(APP_CONFIG_NAME, APP_CONFIG_TEMPLATE)?;
    }

    Ok(())
}

/// Creates temporary kf.ini for compilation and adds required `Editpackages`.
/// # Errors
///
/// Will return `Err` if failed to write to file.
pub fn create_kf_config(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    let mut new_content: String = COMPILATION_CONFIG_TEMPLATE.to_string();

    for package in &runtime_vars.compile_options.edit_packages.clone() {
        dbg!(package);
        let _ = writeln!(&mut new_content, "EditPackages={package}")
            .map_err(CompileToolErrors::WriteError);
    }

    fs::write(
        runtime_vars
            .compiled_paths
            .compile_dir_system
            .join(COMPILATION_CONFIG_NAME),
        &new_content,
    )?;

    Ok(())
}

/// Parse internal config file and get the variables
/// # Errors
///
/// Will return `Err` if fail to read the app config file / find the proper sections.
pub fn parse_app_config() -> Result<(SectionGlobal, SectionLocal), CompileToolErrors> {
    let mut my_config: Ini = Ini::new();

    match my_config.load(APP_CONFIG_NAME) {
        Ok(_) => {
            let result_global: SectionGlobal = get_global_section(&my_config)?;
            let result_local: SectionLocal =
                get_local_section(&my_config, &result_global.package_name)?;
            dbg!(&result_global);
            dbg!(&result_local);

            Ok((result_global, result_local))
        }
        Err(e) => {
            check_app_config()?;
            Err(CompileToolErrors::StringErrors(e))
        }
    }
}

#[inline]
/// Placeholder
/// # Errors
/// # Panics
pub fn get_global_section(app_config: &Ini) -> Result<SectionGlobal, CompileToolErrors> {
    let result: SectionGlobal = SectionGlobal {
        // these variables are important
        package_name: app_config
            .get(GLOBAL_SECTION_NAME, "mutatorName")
            .expect("'mutatorName' isn't specified in the config, aborting!"),
        dir_compile: app_config
            .get(GLOBAL_SECTION_NAME, "dir_Compile")
            .expect("'dir_Compile' path isn't specified in the config, aborting!"),
        dir_classes: app_config
            .get(GLOBAL_SECTION_NAME, "dir_Classes")
            .expect("'dir_Classes' path isn't specified in the config, aborting!"),

        // all these are optional
        dir_move_to: app_config.get(GLOBAL_SECTION_NAME, "dir_MoveTo"),
        dir_release_output: app_config.get(GLOBAL_SECTION_NAME, "dir_ReleaseOutput"),
    };

    Ok(result)
}

#[inline]
/// Placeholder
/// # Errors
/// # Panics
pub fn get_local_section(
    app_config: &Ini,
    package_name: &str,
) -> Result<SectionLocal, CompileToolErrors> {
    let result: SectionLocal = SectionLocal {
        // this one is important
        edit_packages: app_config
            .get(package_name, "EditPackages")
            .expect("`EditPackages` variable is empty, aborting!"),

        // everything else is optional and can be set to default (false) values on fail
        compile_outsideof_kf: app_config
            .getbool(package_name, "bICompileOutsideofKF")
            .map_err(CompileToolErrors::StringErrors)?
            .unwrap_or_default(),
        alt_directories: app_config
            .getbool(package_name, "bAltDirectories")
            .map_err(CompileToolErrors::StringErrors)?
            .unwrap_or_default(),
        move_files: app_config
            .getbool(package_name, "bMoveFiles")
            .map_err(CompileToolErrors::StringErrors)?
            .unwrap_or_default(),
        create_int: app_config
            .getbool(package_name, "bCreateINT")
            .map_err(CompileToolErrors::StringErrors)?
            .unwrap_or_default(),
        make_redirect: app_config
            .getbool(package_name, "bMakeRedirect")
            .map_err(CompileToolErrors::StringErrors)?
            .unwrap_or_default(),
        make_release: app_config
            .getbool(package_name, "bMakeRelease")
            .map_err(CompileToolErrors::StringErrors)?
            .unwrap_or_default(),
    };

    Ok(result)
}

#[must_use]
pub fn create_runtime_vars(
    global_section: &SectionGlobal,
    local_section: &SectionLocal,
) -> RuntimeVariables {
    let res = RuntimeVariables::new(global_section, local_section);

    dbg!(&res);
    res
}
