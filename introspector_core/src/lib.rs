pub mod audit_macros;
pub mod audit_tickets;
pub mod auto_fix;
pub mod clippy_rules;
pub mod decl_macro;
pub mod decl_wrapper;
pub mod macro_report;
pub mod nix_rustc;
pub mod expr;
pub mod pureprogram;
pub mod new_quote_trait;
pub mod expr_cache;
pub mod header;

pub use audit_macros::{
    AuditEntry, AuditKind, AuditStats, AUDIT_REGISTRY,
    register_audit, print_audit_warning, print_audit_summary,
    get_audit_stats, get_audits_by_kind, clear_audits,
};
pub use audit_tickets::{
    AuditTicket, TicketCategory, Severity, TICKET_REGISTRY,
    get_ticket, get_all_tickets, get_tickets_by_category,
    get_tickets_by_module, get_tickets_by_severity,
    print_ticket, print_all_tickets, generate_clippy_toml,
};
pub use clippy_rules::{
    ClippyRule, LintPattern, Applicability,
    generate_lint_rules, generate_suggested_fixes_report,
    check_code_for_violations, print_violation, generate_rustfix_json,
};
pub use auto_fix::{
    FixAction, FixPattern, FixType,
    get_fix_patterns, scan_file_for_fixes, apply_fixes_to_file,
    apply_fixes_to_file_in_place, preview_fixes, generate_diff,
    scan_directory, print_scan_summary, apply_all_fixes,
};
pub use decl_macro::{
    DeclInfo, DeclRegistry, DECL_REGISTRY,
    register_decl, get_all_declarations, get_declarations_by_type,
    get_declarations_by_module, get_declaration_by_hash,
    print_declaration_summary, generate_rdf_declarations,
};
pub use decl_wrapper::{
    DeclMetadata, NodeType, VisibilityKind, FieldInfo, WrapAction,
    extract_declarations, generate_decl_wrapper, generate_decl_attribute,
    generate_inventory_registration, wrap_public_declarations,
    apply_decl_wrappers, preview_decl_wrappers, generate_declarations_json,
};
pub use nix_rustc::{
    RustcNixInfo, RustcSourceStats,
    find_current_rustc, unpack_rustc_source, analyze_rustc_source,
    apply_decl_wrappers_to_rustc, generate_rustc_introspection_report,
    print_rustc_info,
};
pub use macro_report::{
    MacroReport, MacroDefinition, MacroKind, ModuleStats,
    AuditSummary as MacroAuditSummary,
    generate_macro_report, print_macro_report, generate_rdf_turtle_report,
};
pub use expr::Expr;
pub use pureprogram::PureProgram;
pub use new_quote_trait::NewQuoteTrait;
pub use expr_cache::EXPR_CACHE;

