use crate::configs::COMPILATION_CONFIG_NAME;
use crate::{CompileToolErrors, RuntimeVariables};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

/// # Errors
///
/// Will return `Err` if `filename` does not exist or the user does not have
/// permission to read it.s
pub fn validate_compile_directory(
    runtime_vars: &RuntimeVariables,
) -> Result<bool, CompileToolErrors> {
    if !runtime_vars.compiled_paths.compile_dir.try_exists()? {
        return Err(CompileToolErrors::IOError(Error::new(
            ErrorKind::NotFound,
            format!(
                "path `{:?}` doesn't exist!",
                runtime_vars.compiled_paths.compile_dir
            ),
        )));
    }

    dbg!(
        &runtime_vars.compiled_paths.ucc_exe,
        runtime_vars.compiled_paths.ucc_exe.try_exists()?
    );

    Ok(true)
}

// Source: https://rust-lang-nursery.github.io/rust-cookbook/os/external.html?highlight=stdout#continuously-process-child-process-outputs
/// Actual compilation process
/// - Consumes UCC.exe path.
/// - Start a `Command` with given arguments
///     - Call ucc's `make` commandlet.
///     - Pass our custom, compilation config (kfcompile.ini).
///     - Pass `-EXPORTCACHE` to reliably create `ucl` files.
/// # Errors
///
/// Will return `Err` if `filename` does not exist or the user does not have
/// permission to read it.
pub fn start_compilation(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    let ucc_exe = Command::new(runtime_vars.compiled_paths.ucc_exe.clone())
        .stdout(Stdio::piped())
        .arg("make")
        .arg(format!("ini={COMPILATION_CONFIG_NAME}"))
        .arg("-EXPORTCACHE")
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    // create a BufReader to show the stdout in real time
    let reader = BufReader::new(ucc_exe);
    // show the output in real time
    reader
        .lines()
        .map_while(Result::ok)
        // .filter(|line| line.find("usb").is_some())
        .for_each(|line| println!("{line}"));

    Ok(())
}
