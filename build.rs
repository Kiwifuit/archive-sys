use std::{env::var, path::PathBuf};

use anyhow::Context;
use bindgen::Builder;
use pkg_config::probe_library;

fn main() -> anyhow::Result<()> {
    let lib = probe_library("libarchive").context("cannot find library: `libarchive`")?;

    for path in &lib.include_paths {
        println!("cargo:include={}", path.to_string_lossy());
    }

    let bindings = Builder::default()
        .header("wrapper.h")
        .clang_args(
            lib.include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy())),
        )
        .generate()
        .context("while generating low-level bindings for libarchive")?;

    let out_path = PathBuf::from(var("OUT_DIR").context("while creating bindings")?);

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .context("while writing bindings")?;

    Ok(())
}
