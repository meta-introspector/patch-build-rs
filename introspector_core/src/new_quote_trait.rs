use crate::Expr;
use crate::PureProgram;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::collections::BTreeSet;
use proc_macro2::TokenStream; // Explicitly import TokenStream

#[decl(trait, name = "NewQuoteTrait", vis = "pub", hash = "852a05ae")]
pub trait NewQuoteTrait {
    fn to_expr(&self) -> Expr;
}

impl NewQuoteTrait for proc_macro2::TokenStream {
    fn to_expr(&self) -> Expr {
        let input_str = self.to_string();

        let mut hasher = DefaultHasher::new();
        input_str.hash(&mut hasher);
        let hashed_value = hasher.finish();

        let mut set = BTreeSet::new();
        set.insert(hashed_value);

        let program_name = format!("reflected_program_{}", hashed_value);
        let pure_program = PureProgram { set, name: program_name };

        Expr::PureAttractor(pure_program)
    }
}