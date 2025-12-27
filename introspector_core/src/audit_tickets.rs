use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
#[decl(struct, name = "AuditTicket", vis = "pub", hash = "e44f1a4e")]
pub struct AuditTicket {
    pub id: &'static str,
    pub category: TicketCategory,
    pub severity: Severity,
    pub module: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub suggested_fix: Option<&'static str>,
    pub clippy_lint: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[decl(enum, name = "TicketCategory", vis = "pub", hash = "941989a2")]
pub enum TicketCategory {
    Phony,
    FakeData,
    Unverified,
    Issue,
    Concern,
    Todo,
    Security,
    Performance,
}

impl TicketCategory {
    pub fn prefix(&self) -> &'static str {
        match self {
            TicketCategory::Phony => "PHO",
            TicketCategory::FakeData => "FKD",
            TicketCategory::Unverified => "UNV",
            TicketCategory::Issue => "ISS",
            TicketCategory::Concern => "CON",
            TicketCategory::Todo => "TDO",
            TicketCategory::Security => "SEC",
            TicketCategory::Performance => "PRF",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            TicketCategory::Phony => "🎭",
            TicketCategory::FakeData => "🧪",
            TicketCategory::Unverified => "❓",
            TicketCategory::Issue => "⚠️",
            TicketCategory::Concern => "🤔",
            TicketCategory::Todo => "📝",
            TicketCategory::Security => "🔒",
            TicketCategory::Performance => "⚡",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[decl(enum, name = "Severity", vis = "pub", hash = "bb5f55f0")]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn emoji(&self) -> &'static str {
        match self {
            Severity::Info => "ℹ️",
            Severity::Low => "🟢",
            Severity::Medium => "🟡",
            Severity::High => "🟠",
            Severity::Critical => "🔴",
        }
    }
}

pub static TICKET_REGISTRY: Lazy<Mutex<HashMap<&'static str, AuditTicket>>> = 
    Lazy::new(|| Mutex::new(create_ticket_registry()));

fn create_ticket_registry() -> HashMap<&'static str, AuditTicket> {
    let mut registry = HashMap::new();

    // ═══════════════════════════════════════════════════════════════════
    // PHONY TICKETS (PHO-XXX) - Illustrative/Template Code
    // ═══════════════════════════════════════════════════════════════════
    
    registry.insert("PHO-001", AuditTicket {
        id: "PHO-001",
        category: TicketCategory::Phony,
        severity: Severity::High,
        module: "duplicate_analysis",
        title: "Fabricated VFS statistics",
        description: "VFS mapping shows invented numbers (47 functions, 234 functions, 1247 items) that are not from actual analysis",
        suggested_fix: Some("Replace with actual file system traversal and counting, or clearly mark output as '[EXAMPLE]'"),
        clippy_lint: Some("phony_statistics"),
    });

    registry.insert("PHO-002", AuditTicket {
        id: "PHO-002",
        category: TicketCategory::Phony,
        severity: Severity::High,
        module: "duplicate_analysis",
        title: "Fabricated reduction percentages",
        description: "Claims like '55.1% code reduction possible' are not based on actual measurements",
        suggested_fix: Some("Remove percentage claims or compute from actual duplicate analysis"),
        clippy_lint: Some("phony_percentages"),
    });

    registry.insert("PHO-003", AuditTicket {
        id: "PHO-003",
        category: TicketCategory::Phony,
        severity: Severity::Medium,
        module: "solana_lift",
        title: "Template Solana contracts",
        description: "Generated contract code is skeleton template, not functional Solana program",
        suggested_fix: Some("Add disclaimer: '// [TEMPLATE] This is illustrative code, not production-ready'"),
        clippy_lint: Some("template_contract"),
    });

    registry.insert("PHO-004", AuditTicket {
        id: "PHO-004",
        category: TicketCategory::Phony,
        severity: Severity::Critical,
        module: "zk_proof",
        title: "Template ZK proofs",
        description: "PLONK/STARK/SNARK implementations are structural templates, not real cryptographic proofs",
        suggested_fix: Some("Use arkworks, bellman, or halo2 for real ZK proofs, or rename to 'zk_template'"),
        clippy_lint: Some("fake_crypto"),
    });

    registry.insert("PHO-005", AuditTicket {
        id: "PHO-005",
        category: TicketCategory::Phony,
        severity: Severity::Medium,
        module: "dao_governance",
        title: "Simulated DAO voting",
        description: "Voting simulation with hardcoded counts, not actual on-chain governance",
        suggested_fix: Some("Integrate with actual DAO framework (e.g., Realms, Governor) or mark as simulation"),
        clippy_lint: Some("simulated_governance"),
    });

    registry.insert("PHO-006", AuditTicket {
        id: "PHO-006",
        category: TicketCategory::Phony,
        severity: Severity::Medium,
        module: "mev_protection",
        title: "Naive MEV detection",
        description: "Pattern detection uses simplistic string matching, not actual mempool analysis",
        suggested_fix: Some("Use flashbots-like protection or clearly document limitations"),
        clippy_lint: Some("naive_detection"),
    });

    registry.insert("PHO-007", AuditTicket {
        id: "PHO-007",
        category: TicketCategory::Phony,
        severity: Severity::Medium,
        module: "quant_trading",
        title: "Template trading system",
        description: "Trading code is illustrative template, not a real trading system",
        suggested_fix: Some("Add prominent disclaimer: 'NOT FINANCIAL ADVICE - ILLUSTRATIVE ONLY'"),
        clippy_lint: Some("template_trading"),
    });

    registry.insert("PHO-008", AuditTicket {
        id: "PHO-008",
        category: TicketCategory::Phony,
        severity: Severity::Low,
        module: "rustc_ring",
        title: "Conceptual ring properties",
        description: "Mathematical ring properties are conceptual analogy, not proven algebraic structure",
        suggested_fix: Some("Rename to 'conceptual_properties' or formalize the mathematics"),
        clippy_lint: Some("conceptual_math"),
    });

    registry.insert("PHO-009", AuditTicket {
        id: "PHO-009",
        category: TicketCategory::Phony,
        severity: Severity::Medium,
        module: "lean4_proof",
        title: "Unverified Lean4 templates",
        description: "Generates Lean4 syntax templates that are not verified by Lean4 compiler",
        suggested_fix: Some("Add build step to verify generated Lean4 code, or mark as template"),
        clippy_lint: Some("unverified_proof"),
    });

    registry.insert("PHO-010", AuditTicket {
        id: "PHO-010",
        category: TicketCategory::Phony,
        severity: Severity::Low,
        module: "quine_relay",
        title: "Template quines",
        description: "Quine implementations are demonstration templates",
        suggested_fix: Some("Document as educational examples"),
        clippy_lint: None,
    });

    // ═══════════════════════════════════════════════════════════════════
    // FAKEDATA TICKETS (FKD-XXX) - Hardcoded Mock Data
    // ═══════════════════════════════════════════════════════════════════

    registry.insert("FKD-001", AuditTicket {
        id: "FKD-001",
        category: TicketCategory::FakeData,
        severity: Severity::High,
        module: "duplicate_analysis",
        title: "Hardcoded hash values",
        description: "Hash values (a7f3b2c1, d8e9f4a6, f2b8c4d6) are static examples, not computed",
        suggested_fix: Some("Use sha2 crate to compute actual semantic hashes from input"),
        clippy_lint: Some("hardcoded_hash"),
    });

    registry.insert("FKD-002", AuditTicket {
        id: "FKD-002",
        category: TicketCategory::FakeData,
        severity: Severity::High,
        module: "solana_lift",
        title: "Fake blockhash fallback",
        description: "Fallback blockhash '11111111111111111111111111111111' is not a real blockchain hash",
        suggested_fix: Some("Return Result::Err when API fails instead of fake data"),
        clippy_lint: Some("fake_blockchain_data"),
    });

    registry.insert("FKD-003", AuditTicket {
        id: "FKD-003",
        category: TicketCategory::FakeData,
        severity: Severity::Medium,
        module: "solana_lift",
        title: "Placeholder block hash",
        description: "'sample_block_hash' is a placeholder string, not real blockchain data",
        suggested_fix: Some("Propagate Option::None or use descriptive error"),
        clippy_lint: Some("placeholder_value"),
    });

    registry.insert("FKD-004", AuditTicket {
        id: "FKD-004",
        category: TicketCategory::FakeData,
        severity: Severity::Medium,
        module: "dao_governance",
        title: "Hardcoded governance counts",
        description: "Senator/Representative/Lobbyist counts (1000, 500, 100) are arbitrary constants",
        suggested_fix: Some("Accept counts as macro parameters or fetch from on-chain state"),
        clippy_lint: Some("hardcoded_governance"),
    });

    registry.insert("FKD-005", AuditTicket {
        id: "FKD-005",
        category: TicketCategory::FakeData,
        severity: Severity::High,
        module: "mev_protection",
        title: "Always-true transaction matcher",
        description: "transaction_matches() always returns true - placeholder implementation",
        suggested_fix: Some("Implement actual pattern matching or remove function"),
        clippy_lint: Some("stub_always_true"),
    });

    registry.insert("FKD-006", AuditTicket {
        id: "FKD-006",
        category: TicketCategory::FakeData,
        severity: Severity::Medium,
        module: "quant_trading",
        title: "Hardcoded sample prices",
        description: "Historical prices (145.32, 43250.67, etc.) are hardcoded sample data",
        suggested_fix: Some("Fetch from real API (CoinGecko, Yahoo Finance) or accept as input"),
        clippy_lint: Some("hardcoded_prices"),
    });

    registry.insert("FKD-007", AuditTicket {
        id: "FKD-007",
        category: TicketCategory::FakeData,
        severity: Severity::Low,
        module: "rustc_ring",
        title: "Static DOT graph",
        description: "Dependency graph is hardcoded DOT example, not dynamically generated",
        suggested_fix: Some("Use cargo-metadata or syn to generate actual dependency graph"),
        clippy_lint: Some("static_example"),
    });

    registry.insert("FKD-008", AuditTicket {
        id: "FKD-008",
        category: TicketCategory::FakeData,
        severity: Severity::Medium,
        module: "zk_proof",
        title: "Illustrative witness values",
        description: "ZK witness values and execution traces are not cryptographically sound",
        suggested_fix: Some("Use proper witness generation from arkworks or similar"),
        clippy_lint: Some("fake_witness"),
    });

    // ═══════════════════════════════════════════════════════════════════
    // UNVERIFIED TICKETS (UNV-XXX) - Unverified Claims
    // ═══════════════════════════════════════════════════════════════════

    registry.insert("UNV-001", AuditTicket {
        id: "UNV-001",
        category: TicketCategory::Unverified,
        severity: Severity::High,
        module: "zk_proof",
        title: "Fake morphism verification",
        description: "verify_morphism() just checks sum equality, not cryptographic verification",
        suggested_fix: Some("Rename to 'check_sum_equality' or implement real verification"),
        clippy_lint: Some("misleading_verify"),
    });

    registry.insert("UNV-002", AuditTicket {
        id: "UNV-002",
        category: TicketCategory::Unverified,
        severity: Severity::Medium,
        module: "zk_proof",
        title: "Metaphorical Monster group reference",
        description: "Monster group dimension 196883 is used metaphorically, not mathematically",
        suggested_fix: Some("Document as conceptual/artistic reference, not mathematical claim"),
        clippy_lint: Some("metaphorical_math"),
    });

    registry.insert("UNV-003", AuditTicket {
        id: "UNV-003",
        category: TicketCategory::Unverified,
        severity: Severity::Medium,
        module: "lean4_proof",
        title: "Uncompiled Lean4 code",
        description: "Generated Lean4 proofs not verified by actual Lean4 compiler",
        suggested_fix: Some("Add CI step: 'lake build' to verify generated Lean4"),
        clippy_lint: Some("uncompiled_proof"),
    });

    registry.insert("UNV-004", AuditTicket {
        id: "UNV-004",
        category: TicketCategory::Unverified,
        severity: Severity::Medium,
        module: "sat_lfunction",
        title: "Illustrative unity proof",
        description: "Unity proof is conceptual illustration, not mathematical proof",
        suggested_fix: Some("Rename to 'unity_concept' or implement actual SAT solving"),
        clippy_lint: Some("illustrative_proof"),
    });

    // ═══════════════════════════════════════════════════════════════════
    // ISSUE TICKETS (ISS-XXX) - Bugs/Problems
    // ═══════════════════════════════════════════════════════════════════

    registry.insert("ISS-001", AuditTicket {
        id: "ISS-001",
        category: TicketCategory::Issue,
        severity: Severity::High,
        module: "real_data_analysis",
        title: "Misleading eigenvalue terminology",
        description: "'Eigenvalue' calculation is actually normalized frequency, not linear algebra",
        suggested_fix: Some("Rename to 'normalized_frequency' or 'keyword_density'"),
        clippy_lint: Some("misleading_terminology"),
    });

    registry.insert("ISS-002", AuditTicket {
        id: "ISS-002",
        category: TicketCategory::Issue,
        severity: Severity::Medium,
        module: "real_data_analysis",
        title: "Naive pattern counting",
        description: "Pattern counting uses string matching (fn , struct ) which may over/undercount",
        suggested_fix: Some("Use syn crate for proper AST parsing"),
        clippy_lint: Some("naive_parsing"),
    });

    registry.insert("ISS-003", AuditTicket {
        id: "ISS-003",
        category: TicketCategory::Issue,
        severity: Severity::Medium,
        module: "mev_protection",
        title: "Not production-ready protection",
        description: "MEV protection logic is too simplistic for real-world use",
        suggested_fix: Some("Add clear WARNING in output or integrate with flashbots"),
        clippy_lint: Some("incomplete_security"),
    });

    // ═══════════════════════════════════════════════════════════════════
    // CONCERN TICKETS (CON-XXX) - Questionable Patterns
    // ═══════════════════════════════════════════════════════════════════

    registry.insert("CON-001", AuditTicket {
        id: "CON-001",
        category: TicketCategory::Concern,
        severity: Severity::Low,
        module: "dao_governance",
        title: "Conceptual Paxos",
        description: "Paxos consensus is conceptual single-node simulation, not distributed",
        suggested_fix: Some("Rename to 'paxos_simulation' or use actual consensus library"),
        clippy_lint: Some("simulated_consensus"),
    });

    registry.insert("CON-002", AuditTicket {
        id: "CON-002",
        category: TicketCategory::Concern,
        severity: Severity::Low,
        module: "rustc_ring",
        title: "Environment-dependent paths",
        description: "Searches /nix/store which may fail in non-NixOS environments",
        suggested_fix: Some("Use RUSTUP_HOME or cargo metadata for portability"),
        clippy_lint: Some("hardcoded_path"),
    });

    registry.insert("CON-003", AuditTicket {
        id: "CON-003",
        category: TicketCategory::Concern,
        severity: Severity::Low,
        module: "rust_eigenmatrix",
        title: "Informal eigenmatrix concept",
        description: "Eigenmatrix concept needs formal mathematical definition",
        suggested_fix: Some("Document the mathematical basis or rename to 'keyword_matrix'"),
        clippy_lint: Some("informal_math"),
    });

    // ═══════════════════════════════════════════════════════════════════
    // TODO TICKETS (TDO-XXX) - Incomplete Implementations
    // ═══════════════════════════════════════════════════════════════════

    registry.insert("TDO-001", AuditTicket {
        id: "TDO-001",
        category: TicketCategory::Todo,
        severity: Severity::Medium,
        module: "real_data_analysis",
        title: "Replace string matching with AST parsing",
        description: "Current implementation uses naive string matching for code analysis",
        suggested_fix: Some("Use syn::parse_file() for accurate AST-based counting"),
        clippy_lint: None,
    });

    registry.insert("TDO-002", AuditTicket {
        id: "TDO-002",
        category: TicketCategory::Todo,
        severity: Severity::Low,
        module: "context_knapsack",
        title: "Implement actual knapsack optimization",
        description: "Knapsack algorithm is not fully implemented",
        suggested_fix: Some("Implement dynamic programming solution for token optimization"),
        clippy_lint: None,
    });

    registry
}

#[decl(fn, name = "get_ticket", vis = "pub", hash = "7bd660cc")]
pub fn get_ticket(id: &str) -> Option<AuditTicket> {
    TICKET_REGISTRY.lock().ok()?.get(id).cloned()
}

#[decl(fn, name = "get_all_tickets", vis = "pub", hash = "2666359a")]
pub fn get_all_tickets() -> Vec<AuditTicket> {
    TICKET_REGISTRY.lock()
        .map(|r| r.values().cloned().collect())
        .unwrap_or_default()
}

#[decl(fn, name = "get_tickets_by_category", vis = "pub", hash = "09511110")]
pub fn get_tickets_by_category(category: TicketCategory) -> Vec<AuditTicket> {
    get_all_tickets().into_iter()
        .filter(|t| t.category == category)
        .collect()
}

#[decl(fn, name = "get_tickets_by_module", vis = "pub", hash = "aa99f800")]
pub fn get_tickets_by_module(module: &str) -> Vec<AuditTicket> {
    get_all_tickets().into_iter()
        .filter(|t| t.module == module)
        .collect()
}

#[decl(fn, name = "get_tickets_by_severity", vis = "pub", hash = "bf2ba8f9")]
pub fn get_tickets_by_severity(min_severity: Severity) -> Vec<AuditTicket> {
    get_all_tickets().into_iter()
        .filter(|t| t.severity >= min_severity)
        .collect()
}

#[decl(fn, name = "print_ticket", vis = "pub", hash = "098e04ed")]
pub fn print_ticket(ticket: &AuditTicket) {
    let reset = "\x1b[0m";
    let cyan = "\x1b[96m";
    let yellow = "\x1b[93m";
    let gray = "\x1b[90m";
    
    eprintln!("\n{}┌─────────────────────────────────────────────────────────────────┐{}", cyan, reset);
    eprintln!("{}│ {} {} {} │{}", 
        cyan, ticket.category.emoji(), ticket.id, ticket.severity.emoji(), reset);
    eprintln!("{}├─────────────────────────────────────────────────────────────────┤{}", cyan, reset);
    eprintln!("{}│ Title: {}{}", gray, reset, ticket.title);
    eprintln!("{}│ Module: {}{}", gray, reset, ticket.module);
    eprintln!("{}│ Description:{}", gray, reset);
    eprintln!("{}│   {}{}", gray, reset, ticket.description);
    if let Some(fix) = ticket.suggested_fix {
        eprintln!("{}│ Suggested Fix:{}", yellow, reset);
        eprintln!("{}│   {}{}", yellow, reset, fix);
    }
    if let Some(lint) = ticket.clippy_lint {
        eprintln!("{}│ Clippy Lint: #{}{}", gray, reset, lint);
    }
    eprintln!("{}└─────────────────────────────────────────────────────────────────┘{}\n", cyan, reset);
}

#[decl(fn, name = "print_all_tickets", vis = "pub", hash = "ea2f9c0c")]
pub fn print_all_tickets() {
    let mut tickets = get_all_tickets();
    tickets.sort_by(|a, b| {
        b.severity.cmp(&a.severity)
            .then_with(|| a.id.cmp(b.id))
    });
    
    eprintln!("\n╔═══════════════════════════════════════════════════════════════════╗");
    eprintln!("║                    🎫 AUDIT TICKET REGISTRY                        ║");
    eprintln!("╚═══════════════════════════════════════════════════════════════════╝\n");
    
    eprintln!("{:<10} {:<6} {:<20} {:<40}", "ID", "Sev", "Module", "Title");
    eprintln!("{}", "─".repeat(76));
    
    for ticket in &tickets {
        eprintln!("{} {:<10} {:<6} {:<20} {:<40}", 
            ticket.category.emoji(),
            ticket.id, 
            format!("{}", ticket.severity.emoji()),
            ticket.module,
            ticket.title.chars().take(38).collect::<String>());
    }
    
    eprintln!("\n{} Total tickets: {}", "📊", tickets.len());
}

#[decl(fn, name = "generate_clippy_toml", vis = "pub", hash = "727e4dba")]
pub fn generate_clippy_toml() -> String {
    let tickets = get_all_tickets();
    let mut toml = String::new();
    
    toml.push_str("# Auto-generated clippy configuration for audit tickets\n");
    toml.push_str("# Place in .clippy.toml or clippy.toml\n\n");
    
    toml.push_str("[lints]\n");
    
    for ticket in tickets {
        if let Some(lint) = ticket.clippy_lint {
            let level = match ticket.severity {
                Severity::Critical | Severity::High => "deny",
                Severity::Medium => "warn",
                Severity::Low | Severity::Info => "allow",
            };
            toml.push_str(&format!("# {} - {}\n", ticket.id, ticket.title));
            toml.push_str(&format!("# {} = \"{}\"\n\n", lint, level));
        }
    }
    
    toml
}

#[macro_export]
macro_rules! ticket {
    ($id:literal) => {{
        if let Some(ticket) = $crate::audit_tickets::get_ticket($id) {
            $crate::audit_tickets::print_ticket(&ticket);
        } else {
            eprintln!("⚠️ Unknown ticket ID: {}", $id);
        }
    }};
}

#[macro_export]
macro_rules! audit_id {
    ($id:literal, $($msg:tt)*) => {{
        eprint!("[{}] ", $id);
        $crate::audit_macros::print_audit_warning(
            $crate::audit_macros::AuditKind::Issue,
            &format!($($msg)*),
            file!(),
            line!(),
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticket_registry() {
        let tickets = get_all_tickets();
        assert!(!tickets.is_empty());
        assert!(tickets.len() >= 20);
    }

    #[test]
    fn test_get_ticket() {
        let ticket = get_ticket("PHO-001");
        assert!(ticket.is_some());
        assert_eq!(ticket.unwrap().module, "duplicate_analysis");
    }

    #[test]
    fn test_filter_by_category() {
        let phony = get_tickets_by_category(TicketCategory::Phony);
        assert!(!phony.is_empty());
        assert!(phony.iter().all(|t| t.category == TicketCategory::Phony));
    }

    #[test]
    fn test_filter_by_severity() {
        let high = get_tickets_by_severity(Severity::High);
        assert!(!high.is_empty());
        assert!(high.iter().all(|t| t.severity >= Severity::High));
    }
}