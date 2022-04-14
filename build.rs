use std::env;
use std::path::Path;
use std::process::Command;

use glob::glob;
use glob::PatternError;
use rusty::build;
use rusty::get_target_triple;
use rusty::CompileOptions;
use rusty::FilePath;
use rusty::FormatOption;
use rusty::OptimizationLevel;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").ok();
    let optimization = env::var("PROFILE")
        .map(|it| {
            if it == "release" {
                OptimizationLevel::Default
            } else {
                OptimizationLevel::None
            }
        })
        .unwrap_or(OptimizationLevel::None);
    let files = create_file_paths(&["iec61131-st/*.st".to_string()]).unwrap();
    let compile_options = CompileOptions {
        format: FormatOption::Static,
        output: format!("{out_dir}/st.o"),
        target,
        optimization,
    };
    let target = get_target_triple(compile_options.target.as_deref());
    //Build the object file
    let _ = build(
        files,
        vec![],
        &compile_options,
        None,
        &rusty::ErrorFormat::Rich,
        &target,
    );

    Command::new("ar")
        .args(&["crs", "libst.a", "st.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    //link the object file
    println!("cargo:rustc-link-search=native={out_dir}");
    println!("cargo:rustc-link-lib=static=st");
    //We can link against the st lib gernerated, but this will only be reflected in static libs.
    // The shared lib still has to be generated later.
    // There is a planned feature in rust to allow whole-archive linking, but i could not get it to
    // work (should look something like this : `println!("cargo:rustc-flags=-l static:+whole-archive=st");`)
    // The following clang command is equivalent:  clang -o libiec.so --shared -Wl,--whole-archive -lst -L. -Wl,--no-whole-archive  iec.o
    // https://stackoverflow.com/questions/55886779/how-to-link-a-c-library-without-calling-one-of-its-functions
}

fn create_file_paths(inputs: &[String]) -> Result<Vec<FilePath>, PatternError> {
    let mut sources = Vec::new();
    for input in inputs {
        let paths = glob(input)?;

        for p in paths {
            sources.push(FilePath {
                path: p.unwrap().to_string_lossy().to_string(),
            });
        }
    }
    Ok(sources)
}
