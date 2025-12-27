use crate::audit_tickets::{AuditTicket, TicketCategory, Severity, get_all_tickets};

#[derive(Debug, Clone)]
#[decl(struct, name = "ClippyRule", vis = "pub", hash = "0e255083")]
pub struct ClippyRule {
    pub name: String,
    pub ticket_id: &'static str,
    pub pattern: LintPattern,
    pub message: String,
    pub suggestion: Option<String>,
    pub applicability: Applicability,
}

#[derive(Debug, Clone)]
#[decl(enum, name = "LintPattern", vis = "pub", hash = "dfd653b8")]
pub enum LintPattern {
    StringLiteral(&'static str),
    Regex(&'static str),
    FunctionCall(&'static str),
    MacroInvocation(&'static str),
    ModulePath(&'static str),
}

#[derive(Debug, Clone, Copy)]
#[decl(enum, name = "Applicability", vis = "pub", hash = "85e6c897")]
pub enum Applicability {
    MachineApplicable,
    MaybeIncorrect,
    HasPlaceholders,
    Unspecified,
}

#[decl(fn, name = "generate_lint_rules", vis = "pub", hash = "f80e334e")]
pub fn generate_lint_rules() -> Vec<ClippyRule> {
    vec![
        // PHO-001: Fabricated VFS statistics
        ClippyRule {
            name: "phony_statistics".to_string(),
            ticket_id: "PHO-001",
            pattern: LintPattern::Regex(r#"(1,?247|635|213|399)\s*(items?|functions?|structs?)"#),
            message: "[PHO-001] 🎭 Fabricated statistics detected".to_string(),
            suggestion: Some(
                "// audit_id!(\"PHO-001\", \"Replace with actual count from analysis\");\n\
                 let actual_count = walk_dir_and_count(path)?; // Use real counting".to_string()
            ),
            applicability: Applicability::HasPlaceholders,
        },
        
        // PHO-002: Fabricated percentages
        ClippyRule {
            name: "phony_percentages".to_string(),
            ticket_id: "PHO-002",
            pattern: LintPattern::Regex(r#"55\.1%|18\.9%|15\.2%|12\.3%"#),
            message: "[PHO-002] 🎭 Fabricated percentage detected".to_string(),
            suggestion: Some(
                "// audit_id!(\"PHO-002\", \"Compute percentage from actual analysis\");\n\
                 let percentage = (duplicates as f64 / total as f64) * 100.0;".to_string()
            ),
            applicability: Applicability::HasPlaceholders,
        },

        // FKD-001: Hardcoded hash values
        ClippyRule {
            name: "hardcoded_hash".to_string(),
            ticket_id: "FKD-001",
            pattern: LintPattern::Regex(r#"["\']?[a-f0-9]{8}["\']?\s*(//.*hash|hash)"#),
            message: "[FKD-001] 🧪 Hardcoded hash value detected".to_string(),
            suggestion: Some(
                "// audit_id!(\"FKD-001\", \"Use computed hash\");\n\
                 use sha2::{Sha256, Digest};\n\
                 let hash = format!(\"{:x}\", Sha256::digest(input.as_bytes()));".to_string()
            ),
            applicability: Applicability::HasPlaceholders,
        },

        // FKD-002: Fake blockhash
        ClippyRule {
            name: "fake_blockchain_data".to_string(),
            ticket_id: "FKD-002",
            pattern: LintPattern::StringLiteral("11111111111111111111111111111111"),
            message: "[FKD-002] 🧪 Fake blockhash detected".to_string(),
            suggestion: Some(
                "// audit_id!(\"FKD-002\", \"Return error instead of fake data\");\n\
                 Err(BlockchainError::ApiUnavailable)".to_string()
            ),
            applicability: Applicability::MaybeIncorrect,
        },

        // FKD-003: Placeholder value
        ClippyRule {
            name: "placeholder_value".to_string(),
            ticket_id: "FKD-003",
            pattern: LintPattern::StringLiteral("sample_block_hash"),
            message: "[FKD-003] 🧪 Placeholder value detected".to_string(),
            suggestion: Some(
                "// audit_id!(\"FKD-003\", \"Use Option::None for missing values\");\n\
                 .ok_or(BlockchainError::NoBlockHash)?".to_string()
            ),
            applicability: Applicability::MaybeIncorrect,
        },

        // FKD-005: Always-true stub
        ClippyRule {
            name: "stub_always_true".to_string(),
            ticket_id: "FKD-005",
            pattern: LintPattern::Regex(r#"fn\s+\w+\([^)]*\)\s*->\s*bool\s*\{\s*(true|false)\s*\}"#),
            message: "[FKD-005] 🧪 Stub function always returns constant".to_string(),
            suggestion: Some(
                "// audit_id!(\"FKD-005\", \"Implement actual logic or mark as todo!()\");\n\
                 todo!(\"Implement actual matching logic\")".to_string()
            ),
            applicability: Applicability::HasPlaceholders,
        },

        // FKD-006: Hardcoded prices
        ClippyRule {
            name: "hardcoded_prices".to_string(),
            ticket_id: "FKD-006",
            pattern: LintPattern::Regex(r#"\"(145\.32|43250\.67|147\.89|44123\.89)\""#),
            message: "[FKD-006] 🧪 Hardcoded price data detected".to_string(),
            suggestion: Some(
                "// audit_id!(\"FKD-006\", \"Fetch from API or accept as parameter\");\n\
                 let prices = fetch_prices_from_api(symbol).await?;".to_string()
            ),
            applicability: Applicability::HasPlaceholders,
        },

        // UNV-001: Misleading verify function
        ClippyRule {
            name: "misleading_verify".to_string(),
            ticket_id: "UNV-001",
            pattern: LintPattern::FunctionCall("verify_morphism"),
            message: "[UNV-001] ❓ Function name suggests verification but doesn't verify".to_string(),
            suggestion: Some(
                "// audit_id!(\"UNV-001\", \"Rename to reflect actual behavior\");\n\
                 fn check_sum_equality(&self) -> bool { ... }".to_string()
            ),
            applicability: Applicability::MaybeIncorrect,
        },

        // UNV-002: Metaphorical math
        ClippyRule {
            name: "metaphorical_math".to_string(),
            ticket_id: "UNV-002",
            pattern: LintPattern::StringLiteral("196883"),
            message: "[UNV-002] ❓ Monster group dimension used metaphorically".to_string(),
            suggestion: Some(
                "// audit_id!(\"UNV-002\", \"Document as conceptual reference\");\n\
                 /// Note: 196883 is a conceptual reference to the Monster group,\n\
                 /// not a mathematical claim about this code.".to_string()
            ),
            applicability: Applicability::HasPlaceholders,
        },

        // ISS-001: Misleading terminology
        ClippyRule {
            name: "misleading_terminology".to_string(),
            ticket_id: "ISS-001",
            pattern: LintPattern::Regex(r#"eigenvalue|eigenmatrix"#),
            message: "[ISS-001] ⚠️ Term 'eigenvalue' used for non-eigenvalue computation".to_string(),
            suggestion: Some(
                "// audit_id!(\"ISS-001\", \"Use accurate terminology\");\n\
                 // Rename: eigenvalue -> normalized_frequency\n\
                 // Rename: eigenmatrix -> keyword_frequency_matrix".to_string()
            ),
            applicability: Applicability::MaybeIncorrect,
        },

        // ISS-002: Naive parsing
        ClippyRule {
            name: "naive_parsing".to_string(),
            ticket_id: "ISS-002",
            pattern: LintPattern::Regex(r#"\.matches\(\s*["\']fn\s"#),
            message: "[ISS-002] ⚠️ Naive string matching for code analysis".to_string(),
            suggestion: Some(
                "// audit_id!(\"ISS-002\", \"Use syn for AST-based analysis\");\n\
                 use syn::{parse_file, Item};\n\
                 let ast = parse_file(&content)?;\n\
                 let fn_count = ast.items.iter().filter(|i| matches!(i, Item::Fn(_))).count();".to_string()
            ),
            applicability: Applicability::HasPlaceholders,
        },

        // CON-002: Hardcoded paths
        ClippyRule {
            name: "hardcoded_path".to_string(),
            ticket_id: "CON-002",
            pattern: LintPattern::StringLiteral("/nix/store"),
            message: "[CON-002] 🤔 Hardcoded Nix store path - may fail on non-NixOS".to_string(),
            suggestion: Some(
                "// audit_id!(\"CON-002\", \"Use environment variable or cargo metadata\");\n\
                 let rustc_src = std::env::var(\"RUST_SRC_PATH\")\n\
                     .or_else(|_| find_rustc_src_via_rustup())?;".to_string()
            ),
            applicability: Applicability::HasPlaceholders,
        },

        // Template contract warning
        ClippyRule {
            name: "template_contract".to_string(),
            ticket_id: "PHO-003",
            pattern: LintPattern::MacroInvocation("ca!"),
            message: "[PHO-003] 🎭 Template Solana contract macro".to_string(),
            suggestion: Some(
                "// audit_id!(\"PHO-003\", \"Generated code is template only\");\n\
                 // Add to output: \"// [TEMPLATE] Not production-ready\"".to_string()
            ),
            applicability: Applicability::HasPlaceholders,
        },

        // Fake crypto warning
        ClippyRule {
            name: "fake_crypto".to_string(),
            ticket_id: "PHO-004",
            pattern: LintPattern::Regex(r#"(plonk|stark|snark|zk_witness|zk_proof)"#),
            message: "[PHO-004] 🎭 Template ZK code - not real cryptographic proof".to_string(),
            suggestion: Some(
                "// audit_id!(\"PHO-004\", \"Use real ZK library\");\n\
                 // For real ZK proofs, use: arkworks, bellman, or halo2".to_string()
            ),
            applicability: Applicability::HasPlaceholders,
        },
    ]
}

#[decl(fn, name = "generate_suggested_fixes_report", vis = "pub", hash = "a99fcc5c")]
pub fn generate_suggested_fixes_report() -> String {
    let rules = generate_lint_rules();
    let mut report = String::new();
    
    report.push_str("# 🔧 Suggested Fixes Report\n\n");
    report.push_str("Generated lint rules with automated fix suggestions.\n\n");
    
    for rule in &rules {
        report.push_str(&format!("## {} `{}`\n\n", rule.ticket_id, rule.name));
        report.push_str(&format!("**Message:** {}\n\n", rule.message));
        report.push_str(&format!("**Pattern:** `{:?}`\n\n", rule.pattern));
        
        if let Some(suggestion) = &rule.suggestion {
            report.push_str("**Suggested Fix:**\n```rust\n");
            report.push_str(suggestion);
            report.push_str("\n```\n\n");
        }
        
        report.push_str("---\n\n");
    }
    
    report
}

#[decl(fn, name = "check_code_for_violations", vis = "pub", hash = "b92ae4c3")]
pub fn check_code_for_violations(code: &str) -> Vec<(ClippyRule, usize)> {
    let rules = generate_lint_rules();
    let mut violations = Vec::new();
    
    for (line_num, line) in code.lines().enumerate() {
        for rule in &rules {
            let matches = match &rule.pattern {
                LintPattern::StringLiteral(s) => line.contains(s),
                LintPattern::Regex(pattern) => {
                    regex::Regex::new(pattern)
                        .map(|r| r.is_match(line))
                        .unwrap_or(false)
                }
                LintPattern::FunctionCall(name) => line.contains(&format!("{}(", name)),
                LintPattern::MacroInvocation(name) => line.contains(name),
                LintPattern::ModulePath(path) => line.contains(path),
            };
            
            if matches {
                violations.push((rule.clone(), line_num + 1));
            }
        }
    }
    
    violations
}

#[decl(fn, name = "print_violation", vis = "pub", hash = "33a63a1f")]
pub fn print_violation(rule: &ClippyRule, line: usize, file: &str) {
    let reset = "\x1b[0m";
    let yellow = "\x1b[93m";
    let cyan = "\x1b[96m";
    let green = "\x1b[92m";
    
    eprintln!("{}warning{}: {}", yellow, reset, rule.message);
    eprintln!("  {}-->{} {}:{}", cyan, reset, file, line);
    
    if let Some(suggestion) = &rule.suggestion {
        eprintln!("  {}help{}: {}", green, reset, suggestion.lines().next().unwrap_or(""));
        eprintln!("  {}= note{}: ticket {}", cyan, reset, rule.ticket_id);
    }
    eprintln!();
}

#[decl(fn, name = "generate_rustfix_json", vis = "pub", hash = "61f3285b")]
pub fn generate_rustfix_json(violations: &[(ClippyRule, usize)], file: &str) -> String {
    use std::fmt::Write;
    let mut json = String::from("[\n");
    
    for (i, (rule, line)) in violations.iter().enumerate() {
        let suggestion = rule.suggestion.as_deref().unwrap_or("");
        let first_line = suggestion.lines().next().unwrap_or("");
        
        write!(json, r#"  {{
    "message": "{}",
    "code": {{ "code": "{}", "explanation": null }},
    "level": "warning",
    "spans": [{{
      "file_name": "{}",
      "line_start": {},
      "line_end": {},
      "column_start": 1,
      "column_end": 80,
      "is_primary": true,
      "suggested_replacement": "{}",
      "suggestion_applicability": "{:?}"
    }}]
  }}"#,
            rule.message,
            rule.name,
            file,
            line,
            line,
            first_line.replace("\"", "\\\""),
            rule.applicability
        ).ok();
        
        if i < violations.len() - 1 {
            json.push_str(",\n");
        }
    }
    
    json.push_str("\n]");
    json
}

#[macro_export]
macro_rules! check_file {
    ($path:expr) => {{
        let content = std::fs::read_to_string($path).unwrap_or_default();
        let violations = $crate::clippy_rules::check_code_for_violations(&content);
        for (rule, line) in &violations {
            $crate::clippy_rules::print_violation(rule, *line, $path);
        }
        violations
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_rules() {
        let rules = generate_lint_rules();
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_detect_fake_blockhash() {
        let code = r#"let hash = "11111111111111111111111111111111";"#;
        let violations = check_code_for_violations(code);
        assert!(!violations.is_empty());
        assert!(violations.iter().any(|(r, _)| r.ticket_id == "FKD-002"));
    }

    #[test]
    fn test_detect_hardcoded_path() {
        let code = r#"let path = "/nix/store/abc123";"#;
        let violations = check_code_for_violations(code);
        assert!(violations.iter().any(|(r, _)| r.ticket_id == "CON-002"));
    }
}