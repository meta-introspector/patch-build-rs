#[macro_export]
macro_rules! fixme {
    ($($tt:tt)*) => {
        compile_error!(concat!("FIXME: ", stringify!($($tt)*)));
    };
}

// Temporarily disable broken modules
/*
pub mod expr;
pub mod pureprogram;
pub mod new_quote_trait;
pub mod expr_cache;
pub mod header;

pub use expr::Expr;
pub use pureprogram::PureProgram;
pub use new_quote_trait::NewQuoteTrait;
*/

// Minimal working version
pub mod pureprogram;
pub use pureprogram::PureProgram;

// Placeholder types to satisfy dependencies
#[derive(Debug, Clone)]
pub enum Expr {
    Placeholder,
}

pub trait NewQuoteTrait {
    fn to_expr(&self) -> Expr;
}
