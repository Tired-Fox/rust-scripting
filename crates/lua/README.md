# Lua Scripting

This crate uses [mlua](https://github.com/mlua-rs/mlua) to create a standalone
lua executable.

It is written custom in the build.rs file, but inspiration from [copy_to_output](https://crates.io/crates/copy_to_output/) for a way of copying lua script files to the target directory when building examples.