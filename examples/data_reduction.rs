use patch_build_rs_macros::{simplify, pii, prune, compress, pipeline};

fn main() {
    // Simulate massive declaration tree data
    let massive_data = r#"
        // This is a comment
        :node_0 :type :FunctionDecl .
        :node_0 :name "process_user_data" .
        :node_0 :file "/home/john.doe@company.com/project/src/lib.rs" .
        
        // Another comment
        :node_1 :type :StructDecl .
        :node_1 :name "UserData" .
        :node_1 :file "/Users/jane.smith@gmail.com/target/debug/build.rs" .
        
        :node_2 :type :EnumDecl .
        :node_2 :path "/home/developer/.git/hooks/pre-commit" .
        :node_3 :cargo_lock "Cargo.lock content here" .
    "#;
    
    println!("Original size: {} chars", massive_data.len());
    
    // Test individual reductions
    let simplified = simplify!(massive_data);
    let pii_cleaned = pii!(&simplified);
    let pruned = prune!(&pii_cleaned);
    let compressed = compress!(&pruned);
    
    println!("Final size: {} chars", compressed.len());
    
    // Test pipeline (all at once)
    let pipeline_result = pipeline!(massive_data);
    println!("Pipeline result size: {} chars", pipeline_result.len());
}
