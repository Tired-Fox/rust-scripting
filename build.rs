use std::fs;
use std::path::{Path, PathBuf};

fn copy_dir<A: AsRef<Path>, B: AsRef<Path>>(from: A, to: B) {
    let from = from.as_ref();
    let to = to.as_ref();

    for entry in from.read_dir().unwrap().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            if entry.file_name() != "types" {
                if !to.join(entry.path()).exists() {
                    fs::create_dir_all(to.join(entry.path())).unwrap();
                }
                copy_dir(entry.path(), to.join(entry.path()));
            }
        } else {
            fs::copy(entry.path(), to.join(entry.file_name())).unwrap();
        }
    }
}

fn main() {
    let out_dir = PathBuf::from("target").join(std::env::var_os("PROFILE").unwrap());

    let lua = PathBuf::from("lua/");
    if lua.exists() {
        if !out_dir.join(&lua).exists() {
            fs::create_dir_all(out_dir.join(&lua)).unwrap();
        }
        copy_dir(&lua, out_dir.join(&lua));
    }

    let example_lua = PathBuf::from("examples/lua/");
    if example_lua.exists() {
        if !out_dir.join(&example_lua).exists() {
            fs::create_dir_all(out_dir.join(&example_lua)).unwrap();
        }
        copy_dir(&example_lua, out_dir.join(&example_lua));
    }

    // Rerun if lua/ or examples/lua/ has any file changes
    println!("cargo:rerun-if-changed=lua/");
    println!("cargo:rerun-if-changed=examples/lua/");
}