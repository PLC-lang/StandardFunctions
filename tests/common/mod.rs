use std::path::PathBuf;

use inkwell::{
    context::Context,
    execution_engine::ExecutionEngine,
    targets::{InitializationConfig, Target},
};
use rusty::{
    compile_module,
    diagnostics::Diagnostician,
    runner::{run, Compilable},
    FilePath, SourceCode, SourceContainer,
};

macro_rules! add_std {
    ($src:expr, $($name:expr),* ) => {
        {
            let mut res = vec![$src.into()];
            res.push($src.into());
            $(
               res.push(crate::common::get_st_file($name));
            )*
            res
        }
    };
}
pub(crate) use add_std;

/// Gets a file from the ST defined standard functions
pub fn get_st_file(name: &str) -> SourceCode {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("iec61131-st");
    data_path.push(name);

    assert!(data_path.exists());

    let path: FilePath = data_path.display().to_string().into();
    path.load_source(None).expect("Could not load source")
}

/// Compiles code with all native functions included
/// Should be updated for each native function we add
pub fn compile_with_native<T: Compilable>(context: &Context, source: T) -> ExecutionEngine {
    let functions = vec![
        ("ROUND__REAL", iec61131_std::ROUND__REAL as usize),
        ("ROUND__LREAL", iec61131_std::ROUND__LREAL as usize),
        ("SQRT__REAL", iec61131_std::SQRT__REAL as usize),
        ("SQRT__LREAL", iec61131_std::SQRT__LREAL as usize),
    ];
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let code_gen = compile_module(
        &context,
        source.containers(),
        vec![],
        None,
        Diagnostician::default(),
    )
    .unwrap();
    let exec_engine = code_gen
        .module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();

    for (fn_name, fn_addr) in functions {
        if let Some(fn_value) = code_gen.module.get_function(fn_name) {
            exec_engine.add_global_mapping(&fn_value, fn_addr);
        } else {
            println!("No definition for {} in test", fn_name)
        }
    }

    exec_engine
}

///
/// A Convenience method to compile and then run the given source
///
pub fn compile_and_run<T, U, S: Compilable>(source: S, params: &mut T) -> U {
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    run::<T, U>(&exec_engine, "main", params)
}
