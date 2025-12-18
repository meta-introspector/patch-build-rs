use std::collections::BTreeSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use proc_macro2; // Move this import to the top
use quote; // Needed for quote::ToTokens

use lru::LruCache; // New import
use once_cell::sync::Lazy; // New import
use std::sync::Mutex; // New import

pub static EXPR_CACHE: Lazy<Mutex<LruCache<u64, (Expr, String)>>> = Lazy::new(|| {
    Mutex::new(LruCache::new(std::num::NonZeroUsize::new(1024).unwrap())) // Cache capacity of 1024
});

/// Represents a Rust program or a patch as a "numeric attractor".
/// The `set` contains the unique numerical identifiers of the program's components.
/// The `name` is a human-readable identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PureProgram {
    pub set: BTreeSet<u64>,
    pub name: String,
}

impl PureProgram {
    pub fn new(name: &str) -> Self {
        Self {
            set: BTreeSet::new(),
            name: name.to_string(),
        }
    }
}

/// The core inductive type for representing Rust expressions, similar to Lean4's `Expr`.
/// This allows for total reflection of the code into a manipulable data structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// A variable, identified by name.
    Var(String),
    /// A constant, identified by name.
    Const(String),
    /// A lambda abstraction (e.g., `fun x => body`).
    Lam(String, Box<Expr>),
    /// A function application (e.g., `f a b`).
    App(Box<Expr>, Vec<Expr>),
    /// A bridge to the numeric encoding of a Rust program from Layer 14.
    PureAttractor(PureProgram),
}

impl Expr {
    /// Simulates `MetaM.bind`: Chains a computation that returns an `Expr`.
    /// `(m : Expr) → (f : Expr → Expr) → Expr`
    pub fn bind<F>(self, f: F) -> Expr
    where
        F: FnOnce(Expr) -> Expr,
    {
        // This constructs the expression: `bind m (λx. f x)`
        Expr::App(
            Box::new(Expr::Const("bind".to_string())),
            vec![
                self,
                Expr::Lam(
                    "x".to_string(), // The bound variable for the lambda
                    Box::new(f(Expr::Var("x".to_string()))),
                ),
            ],
        )
    }

    /// Simulates function application.
    pub fn app(self, args: Vec<Expr>) -> Expr {
        Expr::App(Box::new(self), args)
    }

    /// Lifts a Rust value that can be represented as tokens into the `Expr` meta-model.
    /// This is the "reflection" step, bridging from syntax to semantics.
    pub fn reflect<T: quote::ToTokens>(input: T) -> Self {
        let mut tokens = proc_macro2::TokenStream::new();
        input.to_tokens(&mut tokens); // Convert T to proc_macro2::TokenStream
        tokens.to_expr() // Use the NewQuoteTrait implementation
    }

    /// Reduces the expression to its normal form via symbolic execution.
    /// This is where the meta-level computation happens.
    pub fn reduce(&self) -> Self {
        match self {
            // The core of reduction: Beta-reduction for lambda calculus.
            // `(λx. body) arg` reduces to `body[x := arg]`
            Expr::App(f, args) if matches!(&**f, Expr::Lam(_, _)) => {
                if let Expr::Lam(var, body) = &**f {
                    if args.len() == 1 {
                        // Substitute the variable in the body with the argument.
                        body.substitute(var, &args[0]).reduce()
                    } else {
                        // Handle partial application or error for incorrect arity.
                        // For now, we'll just leave it un-reduced.
                        self.clone()
                    }
                } else {
                    unreachable!()
                }
            }
            // Recursively reduce arguments of an application.
            Expr::App(f, args) => {
                let reduced_f = f.reduce();
                let reduced_args = args.iter().map(|arg| arg.reduce()).collect();
                Expr::App(Box::new(reduced_f), reduced_args)
            }
            // Constants, variables, and lambdas are already in normal form (or as far as we can go).
            Expr::Var(_) | Expr::Const(_) | Expr::Lam(_, _) | Expr::PureAttractor(_) => {
                self.clone()
            }
        }
    }

    /// Helper function to substitute a variable with an expression.
    fn substitute(&self, var_name: &str, replacement: &Expr) -> Self {
        match self {
            Expr::Var(name) if name == var_name => replacement.clone(),
            Expr::Lam(name, body) if name != var_name => {
                // Avoid capturing the variable if names clash.
                // A full implementation would need alpha-conversion (renaming).
                Expr::Lam(name.clone(), Box::new(body.substitute(var_name, replacement)))
            }
            Expr::App(f, args) => {
                let new_f = f.substitute(var_name, replacement);
                let new_args = args
                    .iter()
                    .map(|arg| arg.substitute(var_name, replacement))
                    .collect();
                Expr::App(Box::new(new_f), new_args)
            }
            // Other cases are terminal or don't contain variables to substitute.
            _ => self.clone(),
        }
    }
}

impl quote::ToTokens for Expr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote; // Import quote macro locally
        

        match self {
            Expr::Var(s) => tokens.extend(quote! { introspector_core::Expr::Var(#s.to_string()) }),
            Expr::Const(s) => tokens.extend(quote! { introspector_core::Expr::Const(#s.to_string()) }),
            Expr::Lam(s, body) => tokens.extend(quote! { introspector_core::Expr::Lam(#s.to_string(), Box::new(#body)) }),
            Expr::App(f, args) => tokens.extend(quote! { introspector_core::Expr::App(Box::new(#f), vec![#(#args),*]) }),
            Expr::PureAttractor(pure_program) => {
                let set_values = pure_program.set.iter().collect::<Vec<_>>();
                let name = &pure_program.name;
                tokens.extend(quote! {
                    introspector_core::Expr::PureAttractor(
                        introspector_core::PureProgram {
                            set: std::collections::BTreeSet::from_iter(vec![#(#set_values),*]),
                            name: #name.to_string(),
                        }
                    )
                })
            }
        }
    }
}

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