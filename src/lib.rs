pub mod modules;
pub mod prelude;

#[macro_export]
macro_rules! splat {
    ($($arg: expr),*) => {
        mlua::Variadic::from_iter([$($arg,)*])
    };
}