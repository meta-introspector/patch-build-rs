use patch_build_rs_macros::newquote;
use introspector_core::Expr;
use std::collections::BTreeSet;
use std::hash::{DefaultHasher, Hash, Hasher};

extern crate quote; // Required for quote::quote!

#[test]
fn test_newquote_simple_function() {
    let (expr_result, returned_str) = newquote! {
        pub fn calculate_sum(a: i32, b: i32) -> i32 {
            a + b
        }
    };

    // Calculate expected hash from the string returned by the macro
    let mut hasher = DefaultHasher::new();
    returned_str.hash(&mut hasher);
    let expected_hash = hasher.finish();

    let mut expected_set = BTreeSet::new();
    expected_set.insert(expected_hash);

    // Assert that the result is an Expr::PureAttractor
    if let Expr::PureAttractor(pure_program) = expr_result {
        assert_eq!(pure_program.set, expected_set);
        assert!(pure_program.name.starts_with("reflected_program_"));
        // Optionally, print the returned_str to inspect its content
        // eprintln!("Returned String: {}", returned_str);
    } else {
        panic!("newquote! did not produce an Expr::PureAttractor variant.");
    }
}

#[test]
fn test_newquote_different_input() {
    let (expr_result, returned_str) = newquote! {
        struct MyStruct {
            field: String,
        }
    };

    let mut hasher = DefaultHasher::new();
    returned_str.hash(&mut hasher);
    let expected_hash = hasher.finish();

    let mut expected_set = BTreeSet::new();
    expected_set.insert(expected_hash);

    if let Expr::PureAttractor(pure_program) = expr_result {
        assert_eq!(pure_program.set, expected_set);
        assert!(pure_program.name.starts_with("reflected_program_"));
    } else {
        panic!("newquote! did not produce an Expr::PureAttractor variant.");
    }
}