// grast: Greppable AST - Line-oriented AST representation
// Inspired by gron's approach to making JSON greppable

#[macro_export]
macro_rules! fixme {
    ($msg:literal) => {
        {
            use patch_build_rs_macros::extract;
            
            // Extract the fixme to isolated environment
            extract!($msg);
            
            // Then fail compilation with clear message
            compile_error!(concat!("FIXME: ", $msg));
        }
    };
}

pub mod triple;
pub mod database;
pub mod flatten;

pub use triple::GrastTriple;
pub use database::GrastDb;

// Re-export for convenience
pub use syn::{self, Expr, Item, Stmt};
pub use quote::quote;
