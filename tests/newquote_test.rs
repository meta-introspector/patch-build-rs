use patch_build_rs_macros::__newquote_implementation_default;
use introspector_core::Expr;
use std::collections::BTreeSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use patch_build_rs_macros::{grast, structural_grep}; // Import macros

extern crate quote; // Required for quote::quote!

#[test]
fn test_newquote_simple_function() {
    let (expr_result, returned_str) = __newquote_implementation_default! {
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
        panic!("__newquote_implementation_default! did not produce an Expr::PureAttractor variant.");
    }
}

#[test]
fn test_newquote_different_input() {
    let (expr_result, returned_str) = __newquote_implementation_default! {
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
        panic!("__newquote_implementation_default! did not produce an Expr::PureAttractor variant.");
    }
}

#[test]
fn test_structural_grep_macro() {
    let search_results_public: Vec<String> = structural_grep!(
        grast! {
            pub fn my_public_function() {}
            fn my_private_function() {}
            pub struct MyStruct {}
        },
        "Public"
    );

    let search_results_struct: Vec<String> = structural_grep!(
        grast! {
            pub fn my_public_function() {}
            fn my_private_function() {}
            pub struct MyStruct {}
        },
        ":StructDecl"
    );

    let search_results_private: Vec<String> = structural_grep!(
        grast! {
            pub fn my_public_function() {}
            fn my_private_function() {}
            pub struct MyStruct {}
        },
        "private"
    );


    eprintln!("Search Results (Public):\n{:?}", search_results_public);
    eprintln!("Search Results (StructDecl):\n{:?}", search_results_struct);
    eprintln!("Search Results (Private):\n{:?}", search_results_private);

    assert_eq!(search_results_public.len(), 1);
    assert!(search_results_public[0].contains("my_public_function"));

    assert_eq!(search_results_struct.len(), 1);
    assert!(search_results_struct[0].contains("MyStruct"));

    assert_eq!(search_results_private.len(), 3); // Three lines contain "private" (type, name, visibility for my_private_function)
    // Note: this assertion is based on the current simple grep implementation
    // and will need to be adjusted as structural_grep becomes more sophisticated.
}