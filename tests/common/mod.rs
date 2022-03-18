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
        ("LN__REAL", iec61131_std::LN__REAL as usize),
        ("LN__LREAL", iec61131_std::LN__LREAL as usize),
        ("LOG__REAL", iec61131_std::LOG__REAL as usize),
        ("LOG__LREAL", iec61131_std::LOG__LREAL as usize),
        ("EXP__REAL", iec61131_std::EXP__REAL as usize),
        ("EXP__LREAL", iec61131_std::EXP__LREAL as usize),
        ("SIN__REAL", iec61131_std::SIN__REAL as usize),
        ("SIN__LREAL", iec61131_std::SIN__LREAL as usize),
        ("COS__REAL", iec61131_std::COS__REAL as usize),
        ("COS__LREAL", iec61131_std::COS__LREAL as usize),
        ("TAN__REAL", iec61131_std::TAN__REAL as usize),
        ("TAN__LREAL", iec61131_std::TAN__LREAL as usize),
        ("ASIN__REAL", iec61131_std::ASIN__REAL as usize),
        ("ASIN__LREAL", iec61131_std::ASIN__LREAL as usize),
        ("ACOS__REAL", iec61131_std::ACOS__REAL as usize),
        ("ACOS__LREAL", iec61131_std::ACOS__LREAL as usize),
        ("ATAN__REAL", iec61131_std::ATAN__REAL as usize),
        ("ATAN__LREAL", iec61131_std::ATAN__LREAL as usize),
        ("ATAN2__REAL", iec61131_std::ATAN2__REAL as usize),
        ("ATAN2__LREAL", iec61131_std::ATAN2__LREAL as usize),
        ("LWORD_TO_LREAL", iec61131_std::LWORD_TO_LREAL as usize),
        ("DWORD_TO_REAL", iec61131_std::DWORD_TO_REAL as usize),
        ("LREAL_TO_LWORD", iec61131_std::LREAL_TO_LWORD as usize),
        ("REAL_TO_DWORD", iec61131_std::REAL_TO_DWORD as usize),
    ];

    let variables = vec![
        (
            "PI_REAL",
            std::ptr::addr_of!(iec61131_std::PI_REAL) as usize,
        ),
        (
            "PI_LREAL",
            std::ptr::addr_of!(iec61131_std::PI_LREAL) as usize,
        ),
        (
            "FRAC_PI_2_REAL",
            std::ptr::addr_of!(iec61131_std::FRAC_PI_2_REAL) as usize,
        ),
        (
            "FRAC_PI_2_LREAL",
            std::ptr::addr_of!(iec61131_std::FRAC_PI_2_LREAL) as usize,
        ),
        (
            "FRAC_PI_4_REAL",
            std::ptr::addr_of!(iec61131_std::FRAC_PI_4_REAL) as usize,
        ),
        (
            "FRAC_PI_4_LREAL",
            std::ptr::addr_of!(iec61131_std::FRAC_PI_4_LREAL) as usize,
        ),
        ("E_REAL", std::ptr::addr_of!(iec61131_std::E_REAL) as usize),
        (
            "E_LREAL",
            std::ptr::addr_of!(iec61131_std::E_LREAL) as usize,
        ),
    ];
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let (_, code_gen) = compile_module(
        &context,
        source.containers(),
        vec![],
        None,
        Diagnostician::default(),
    )
    .unwrap();
    println!("{}", code_gen.module.print_to_string());
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
    for (var_name, var_address) in variables {
        if let Some(var_value) = code_gen.module.get_global(var_name) {
            exec_engine.add_global_mapping(&var_value, var_address);
        } else {
            println!("No definition for {} in test", var_name)
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
