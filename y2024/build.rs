use regex::{self, Regex};
use std::io::Write;
use std::{env, fs::OpenOptions, path::PathBuf};

fn main() {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("No manifest dir");
    let cargo_manifest_dir = PathBuf::from(cargo_manifest_dir);

    let day_regex = Regex::new(r"(day\d+)\.rs").unwrap();
    let mut mods = Vec::new();
    for file in cargo_manifest_dir.join("src").read_dir().unwrap().flatten() {
        if let Some(captures) = day_regex.captures(&file.file_name().into_string().unwrap()) {
            let (_, [mod_name]) = captures.extract();
            dbg!(mod_name);
            mods.push(mod_name.to_owned())
        }
    }

    let out_dir = env::var("OUT_DIR").expect("No out dir");
    let out_dir = PathBuf::from(out_dir);
    dbg!(&out_dir);
    let mut lib_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(out_dir.join("days.rs"))
        .expect("Could not append to days.rs");
    for mod_name in mods {
        writeln!(
            lib_file,
            "pub mod {} {{ include!(\"{}\"); }}",
            mod_name,
            cargo_manifest_dir
                .join("src")
                .join(format!("{}.rs", mod_name))
                .to_str()
                .unwrap()
        )
        .expect("Failed to append mod to file");
    }

    println!("cargo:rerun-if-changed=src");
}
