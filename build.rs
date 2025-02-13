use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=res/*");

    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;

    let paths_to_copy = vec!["res"];
    let out_dir = env::var("OUT_DIR")?;
    let out_path = Path::new(&out_dir);

    fs_extra::dir::copy("res", out_path, &copy_options)?;

    Ok(())
}