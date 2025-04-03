pub mod ast;
pub mod compile;
pub mod core;
pub mod env;
pub mod pprint;
pub mod tast;

lalrpop_util::lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar
);

#[cfg(test)]
mod tests;
