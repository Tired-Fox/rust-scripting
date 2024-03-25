# Lua Scripting

## Lua (mlua)

This crate uses [mlua](https://github.com/mlua-rs/mlua) to create a standalone
lua executable.

It is written custom in the build.rs file, but inspiration from [copy_to_output](https://crates.io/crates/copy_to_output/) for a way of copying lua script files to the target directory when building examples.

`.luarc.json` is required for typing as it will allow for specifying the used lua version along with where the lua definition files are located.
This allows it to read and handle the rust provided types.

Also looking at `rhai` as it is more tightly working with rust and could allow for better functionality and typing.

Comparable to [luau](https://luau-lang.org/syntax)

## Rhai

Reference [lua-ls/annotations](https://luals.github.io/wiki/annotations/) for a list of usable annotations while typing.
