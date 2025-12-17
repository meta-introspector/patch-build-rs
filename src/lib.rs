//! The `patch-build-rs` library.
//!
//! This crate provides the core data structures and logic for the
//! Lean4-inspired metaprogramming and semantic patching system.

pub mod expr;
pub mod pure_program;

// Re-export the main types for easier access.
pub use expr::Expr;
pub use pure_program::PureProgram;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    /// Tests the beta-reduction logic in `Expr::reduce`.
    #[test]
    fn test_beta_reduction() {
        // Create an expression for `(λx. x) y`
        let identity_fn = Expr::Lam("x".to_string(), Box::new(Expr::Var("x".to_string())));
        let argument = Expr::Const("y".to_string());
        
        let app = Expr::App(Box::new(identity_fn), vec![argument.clone()]);
        
        // The reduced form should be just "y"
        let reduced = app.reduce();
        
        assert_eq!(reduced, argument);
    }

    /// Tests a more complex reduction with substitution.
    #[test]
    fn test_nested_reduction() {
        // Create `(λy. (λx. y)) z`
        let const_fn_inner = Expr::Lam("x".to_string(), Box::new(Expr::Var("y".to_string())));
        let const_fn_outer = Expr::Lam("y".to_string(), Box::new(const_fn_inner));
        let argument = Expr::Const("z".to_string());

        let app = Expr::App(Box::new(const_fn_outer), vec![argument.clone()]);

        // It should reduce to `λx. z`
        let expected_reduced_inner = Expr::Lam("x".to_string(), Box::new(Expr::Const("z".to_string())));
        
        let reduced = app.reduce();

        assert_eq!(reduced, expected_reduced_inner);
    }

    /// Tests the `bind` and `app` meta-model functions.
    #[test]
    fn test_bind_and_apply() {
        // Simulate `my_expr.bind(|x| x.app(vec![const_arg]))`
        
        let my_expr = Expr::PureAttractor(PureProgram {
            set: BTreeSet::from([1, 2, 3]),
            name: "my_program".to_string(),
        });
        
        let const_arg = Expr::Const("config_value".to_string());

        // The meta-level computation
        let meta_computation = my_expr.clone().bind(|x| {
            x.app(vec![const_arg.clone()])
        });

        // Expected structure: bind(my_expr, λx. apply(x, const_arg))
        let expected = Expr::App(
            Box::new(Expr::Const("bind".to_string())),
            vec![
                my_expr.clone(),
                Expr::Lam(
                    "x".to_string(),
                    Box::new(
                        Expr::App(
                            Box::new(Expr::Var("x".to_string())),
                            vec![const_arg.clone()]
                        )
                    )
                )
            ]
        );

        assert_eq!(meta_computation, expected);

        // Note: reducing this `meta_computation` would require a `reduce` implementation
        // that understands the semantics of "bind" and "apply" at the meta-level.
        // The current `reduce` only handles beta-reduction.
    }
}
