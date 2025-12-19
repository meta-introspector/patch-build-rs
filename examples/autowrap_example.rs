use autowrap_macros::{autowrap, extract_and_wrap};

fn main() {
    // Auto-wrap a code snippet - will try to compile and add needed imports
    autowrap!("let x = HashMap::new(); x.insert(1, 2);");
    
    // Extract code matching pattern and auto-wrap each match
    extract_and_wrap!("fn calculate");
    
    println!("Autowrap example completed");
}
