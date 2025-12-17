//! Layer 14: Numeric Attractors as Pure Programs
//! A `PureProgram` is a mathematical representation of a Rust program,
//! encoded as a set of numbers (e.g., Gödel numbers, monster group indices).

use std::collections::BTreeSet;

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
