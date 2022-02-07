use std::path::PathBuf;

use rusty::{FilePath, SourceCode, SourceContainer};

macro_rules! add_std {
    ($src:expr, $($name:expr),* ) => {
        {
            let mut res = Vec::new();
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
