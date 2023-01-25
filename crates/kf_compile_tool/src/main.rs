use kfct_lib::{
    configs::helper::{create_kf_config, create_runtime_vars, parse_app_config},
    exit_codes,
    file_helper::{create_hacky_steamappid, new_compress_dir_to_zip, remove_compilation_garbage},
    ucc_wrapper, RuntimeVariables,
};
use std::{
    path::PathBuf,
    process::ExitCode,
    time::{Duration, Instant},
};

fn main() -> ExitCode {
    let now: Instant = Instant::now();

    // 1. read internal config file and create all required variables
    // if not found - create new, default config and start over
    let runtime_vars: RuntimeVariables = match parse_app_config() {
        Ok(result) => create_runtime_vars(&result.0, &result.1),
        // println!("RUNTIME VARS: {x:#?}");}(result.0, result.1);
        Err(e) => {
            eprintln!("Terminated with error: {e}");
            std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE));
        }
    };
    let elapsed: Duration = now.elapsed();
    println!("> #1 Elapsed: {elapsed:.2?}");

    // 2. check compile dir
    ucc_wrapper::validate_compile_directory(&runtime_vars).unwrap_or_else(|e| {
        eprintln!("Terminated with error: {e}");
        std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE));
    });
    let elapsed: Duration = now.elapsed();
    println!("> #2 Elapsed: {elapsed:.2?}");

    // 3. create `kfcompile.ini`
    create_kf_config(&runtime_vars).unwrap_or_else(|e| {
        eprintln!("Terminated with error: {e}");
        std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE));
    });
    let elapsed: Duration = now.elapsed();
    println!("> #3 Elapsed: {elapsed:.2?}");

    // 4. start compilation
    let _ = create_hacky_steamappid(&runtime_vars);

    // ucc_wrapper::start_compilation(&runtime_vars).unwrap_or_else(|e| {
    //     eprintln!("Terminated with error: {e}");
    //     std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE));
    // });
    // let elapsed: Duration = now.elapsed();
    // println!("> #4 Elapsed: {elapsed:.2?}");

    match new_compress_dir_to_zip(
        &PathBuf::from("D:\\Games\\KF Dedicated Server\\BitCore"),
        &PathBuf::from("result.zip"),
    ) {
        Ok(()) => {}
        Err(e) => println!("{e}"),
    };
    let elapsed: Duration = now.elapsed();
    println!("> #4 Elapsed: {elapsed:.2?}");

    match remove_compilation_garbage(&runtime_vars) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Terminated with error: {e}");
            std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE));
        }
    };
    let elapsed: Duration = now.elapsed();
    println!("> #5 Elapsed: {elapsed:.2?}");

    ExitCode::from(exit_codes::ERROR_SUCCESS)
}
