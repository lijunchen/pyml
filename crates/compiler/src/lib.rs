pub mod compile_match;
pub mod core;
pub mod env;
pub mod interpreter;
pub mod pprint;
pub mod tast;
pub mod typer;

lalrpop_util::lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub grammar
);

#[cfg(test)]
mod tests;
