use patch_build_rs_macros::{pure_reflect, commit_cache};

// Define some functions and structs with common subexpressions
pure_reflect! {
    fn common_subexpression_a() -> u32 {
        let x = 1 + 2;
        x * 3
    }
}

pure_reflect! {
    fn common_subexpression_b() -> u32 {
        let x = 1 + 2; // Common subexpression with common_subexpression_a
        x * 4
    }
}

pure_reflect! {
    struct MyStruct {
        field1: u32,
        field2: u32,
    }
}

pure_reflect! {
    fn another_function(a: u32, b: u32) -> u32 {
        common_subexpression_a() + a + b
    }
}

pure_reflect! {
    fn yet_another_function() {
        let _ = common_subexpression_a(); // Another use of the common subexpression
    }
}

// Even more expressions to populate the cache
pure_reflect! { fn func1() { /* ... */ } }
pure_reflect! { fn func2() { /* ... */ } }
pure_reflect! { fn func3() { /* ... */ } }
pure_reflect! { fn func4() { /* ... */ } }
pure_reflect! { fn func5() { /* ... */ } }
pure_reflect! { fn func6() { /* ... */ } }
pure_reflect! { fn func7() { /* ... */ } }
pure_reflect! { fn func8() { /* ... */ } }
pure_reflect! { fn func9() { /* ... */ } }
pure_reflect! { fn func10() { /* ... */ } } // This should trigger the println! if it's the 10th common_subexpression_a
pure_reflect! { fn func11() { /* ... */ } }
pure_reflect! { fn func12() { /* ... */ } }

// Invoke commit_cache to save the current state of caches to JSON files
// Using default paths for now.
commit_cache!();

fn main() {
    println!("Full analysis demo complete. Check /tmp/expr_cache.json, /tmp/expr_counts.json, and /tmp/expr_lattice.json");
    // To actually run the functions defined by pure_reflect!, they would need to be in scope
    // and called explicitly. The purpose here is just to trigger macro expansion.
    let _ = common_subexpression_a();
    let _ = common_subexpression_b();
    let _ = MyStruct { field1: 1, field2: 2 };
    let _ = another_function(1, 2);
    let _ = yet_another_function();
    let _ = func1(); let _ = func2(); let _ = func3(); let _ = func4(); let _ = func5(); let _ = func6(); let _ = func7(); let _ = func8(); let _ = func9(); let _ = func10(); let _ = func11(); let _ = func12();
}
