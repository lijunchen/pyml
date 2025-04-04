pub mod ast;
pub mod compile;
pub mod core;
pub mod env;
pub mod ident;
pub mod pprint;
pub mod tast;
pub mod typer;

lalrpop_util::lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar
);

#[cfg(test)]
mod tests;
