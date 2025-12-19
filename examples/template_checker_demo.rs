// Template Syntax Checker Demo - Compile-time validation of generated code
// Shows checktemplate! macro ensuring generated templates are syntactically correct

use patch_build_rs_macros::{
    checktemplate, generate_checked_macros, mkbuildrs_checked
};

fn main() {
    println!("🔧 Template Syntax Checker Demo");
    
    // Test 1: Check a valid template
    let valid_template = r#"
fn hello_world() {
    println!("Hello, world!");
}
    "#;
    
    println!("✅ Testing valid template...");
    let _checked_valid = checktemplate!(valid_template);
    
    // Test 2: Generate syntax-checked common macros
    let checked_macros = generate_checked_macros!("standard");
    
    // Test 3: Generate syntax-checked build system
    let checked_build = mkbuildrs_checked!("syntax_validated");
    
    println!("🔧 Checked macros: {} lines", checked_macros.lines().count());
    println!("🏗️ Checked build: {} lines", checked_build.lines().count());
    
    // Save the syntax-checked templates
    std::fs::create_dir_all("checked_templates").ok();
    std::fs::write("checked_templates/validated_macros.rs", &checked_macros).ok();
    std::fs::write("checked_templates/validated_build.rs", &checked_build).ok();
    
    // Create a demonstration of the syntax checking
    let demo_code = r#"
// Demo: Template Syntax Checking at Compile Time

use patch_build_rs_macros::checktemplate;

fn demo_syntax_checking() {
    // This template will be syntax-checked at compile time
    let valid_rust_code = checktemplate!(r#"
        fn example_function(x: i32) -> i32 {
            x * 2
        }
        
        macro_rules! example_macro {
            ($x:expr) => {
                println!("Value: {}", $x)
            };
        }
    "#);
    
    println!("Template passed syntax check: {}", valid_rust_code.len());
}

// Example of what happens with invalid syntax:
/*
fn demo_invalid_syntax() {
    // This would cause a compile-time error:
    let invalid_code = checktemplate!(r#"
        fn broken_function( {  // Missing parameter and brace
            println!("This won't compile");
        // Missing closing brace
    "#);
}
*/
    "#;
    
    std::fs::write("checked_templates/demo_usage.rs", demo_code).ok();
    
    // Create comprehensive documentation
    std::fs::write("TEMPLATE_CHECKER.md", format!(
        r#"# 🔧 Template Syntax Checker - Compile-time Validation

## Overview

The `checktemplate!` macro provides compile-time syntax validation for generated template code, ensuring that all generated Rust code is syntactically correct before compilation.

## Syntax-Checked Macros

{}

## Syntax-Checked Build System

{}

## How It Works

### 1. **checktemplate!** - Core Validation Macro
```rust
checktemplate!(r#"
    fn example() {{
        println!("This will be syntax-checked");
    }}
"#)
```

- Parses template code using `syn::parse_str::<syn::File>`
- Returns the template if syntax is valid
- Emits `compile_error!` if syntax is invalid
- Provides detailed error messages for debugging

### 2. **generate_checked_macros!** - Validated Utility Macros
- Generates common subexpression macros
- Each macro template is syntax-checked at compile time
- Ensures generated macros are syntactically correct
- Prevents runtime syntax errors in generated code

### 3. **mkbuildrs_checked!** - Validated Build System
- Generates build.rs with syntax validation
- All template code is checked before generation
- Guarantees syntactically correct build scripts
- Eliminates build-time syntax errors

## Benefits

### Compile-time Safety
- ✅ **Early Error Detection**: Syntax errors caught at compile time
- ✅ **Template Validation**: All generated code is pre-validated
- ✅ **Error Prevention**: Eliminates runtime syntax failures
- ✅ **Developer Experience**: Clear error messages for debugging

### Template Quality
- ✅ **Syntax Correctness**: Guaranteed valid Rust syntax
- ✅ **Macro Validation**: Generated macros are syntactically sound
- ✅ **Build Script Safety**: Build systems are pre-validated
- ✅ **Code Generation**: Reliable template-based code generation

## Usage Examples

### Basic Template Checking
```rust
let template = checktemplate!(r#"
    macro_rules! my_macro {{
        ($x:expr) => {{
            println!("Value: {{}}", $x)
        }};
    }}
"#);
```

### Macro Generation with Validation
```rust
let macros = generate_checked_macros!("standard");
// All generated macros are syntax-checked
```

### Build System with Validation
```rust
let build_system = mkbuildrs_checked!("validated");
// Generated build.rs is guaranteed to be syntactically correct
```

## Error Handling

### Valid Template
- Template passes `syn::parse_str` validation
- Code is returned as-is for use
- Compile-time warning: "✅ Template syntax check passed"

### Invalid Template
- Template fails `syn::parse_str` validation
- `compile_error!` is emitted with detailed error message
- Compilation stops with clear syntax error description
- Developer can fix template and retry

## Integration

The template checker integrates seamlessly with:
- **mkbuildrs system**: Validates all generated build scripts
- **Common macros**: Ensures utility macros are syntactically correct
- **Code generation**: Provides safety for template-based generation
- **Development workflow**: Catches errors early in the process

**🎯 Compile-time template validation ensures generated code is always syntactically correct!**
        "#,
        checked_macros.lines().take(30).collect::<Vec<_>>().join("\n"),
        checked_build.lines().take(20).collect::<Vec<_>>().join("\n")
    )).ok();
    
    println!("💾 Template syntax checker demo complete!");
    println!("🔧 Validated macros: checked_templates/validated_macros.rs");
    println!("🏗️ Validated build: checked_templates/validated_build.rs");
    println!("📋 Demo usage: checked_templates/demo_usage.rs");
    println!("📖 Documentation: TEMPLATE_CHECKER.md");
    println!("✅ All templates syntax-checked at compile time!");
}
