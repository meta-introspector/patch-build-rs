// change: introspector_core/src/lib.rs
// target: PureProgram
// old_string: #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// new_string: #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]

// change: introspector_core/src/lib.rs
// target: Expr
// old_string: #[derive(Debug, Clone, PartialEq, Eq)]
// new_string: #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]