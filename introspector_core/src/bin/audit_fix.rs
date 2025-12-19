use std::env;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        process::exit(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "scan" => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            cmd_scan(path);
        }
        "preview" => {
            if args.len() < 3 {
                eprintln!("Error: preview requires a file path");
                process::exit(1);
            }
            cmd_preview(&args[2]);
        }
        "fix" => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            let dry_run = args.iter().any(|a| a == "--dry-run" || a == "-n");
            cmd_fix(path, dry_run);
        }
        "diff" => {
            if args.len() < 3 {
                eprintln!("Error: diff requires a file path");
                process::exit(1);
            }
            cmd_diff(&args[2]);
        }
        "tickets" => {
            let filter = args.get(2).map(|s| s.as_str());
            cmd_tickets(filter);
        }
        "ticket" => {
            if args.len() < 3 {
                eprintln!("Error: ticket requires a ticket ID");
                process::exit(1);
            }
            cmd_ticket(&args[2]);
        }
        "report" => {
            cmd_report();
        }
        "decl-scan" => {
            if args.len() < 3 {
                eprintln!("Error: decl-scan requires a file path");
                process::exit(1);
            }
            cmd_decl_scan(&args[2]);
        }
        "decl-wrap" => {
            if args.len() < 3 {
                eprintln!("Error: decl-wrap requires a file path");
                process::exit(1);
            }
            let dry_run = args.iter().any(|a| a == "--dry-run" || a == "-n");
            cmd_decl_wrap(&args[2], dry_run);
        }
        "decl-json" => {
            if args.len() < 3 {
                eprintln!("Error: decl-json requires a file path");
                process::exit(1);
            }
            cmd_decl_json(&args[2]);
        }
        "help" | "--help" | "-h" => {
            print_usage(&args[0]);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage(&args[0]);
            process::exit(1);
        }
    }
}

fn print_usage(program: &str) {
    eprintln!(r#"
🔧 Audit Fix Tool - Automatic audit_id! and decl! injection

USAGE:
    {} <COMMAND> [OPTIONS]

AUDIT COMMANDS:
    scan [DIR]          Scan directory for audit issues (default: current dir)
    preview <FILE>      Preview fixes for a single file
    fix [DIR] [--dry-run]  Apply fixes to directory (default: current dir)
    diff <FILE>         Show git-style diff of proposed changes
    tickets [FILTER]    List all tickets (optional: PHO, FKD, ISS, etc.)
    ticket <ID>         Show details for a specific ticket
    report              Generate full macro audit report

DECLARATION COMMANDS:
    decl-scan <FILE>    Scan file for public declarations and show metadata
    decl-wrap <FILE> [--dry-run]  Wrap public declarations with #[decl(...)]
    decl-json <FILE>    Export declarations as JSON

OPTIONS:
    --dry-run, -n       Show what would be fixed without changing files
    --help, -h          Show this help message

EXAMPLES:
    {} scan src/
    {} preview src/solana_lift.rs
    {} fix src/ --dry-run
    {} decl-scan src/lib.rs
    {} decl-wrap src/lib.rs --dry-run
    {} decl-json src/lib.rs > decls.json
"#, program, program, program, program, program, program, program);
}

fn cmd_scan(path: &str) {
    eprintln!("🔍 Scanning {} for audit issues...\n", path);
    
    let path = Path::new(path);
    let results = introspector_core::scan_directory(path, &["rs"]);
    introspector_core::print_scan_summary(&results);
}

fn cmd_preview(path: &str) {
    let path = Path::new(path);
    let preview = introspector_core::preview_fixes(path);
    println!("{}", preview);
}

fn cmd_fix(path: &str, dry_run: bool) {
    let path = Path::new(path);
    
    if dry_run {
        eprintln!("🔍 Dry run mode - no files will be modified\n");
    } else {
        eprintln!("🔧 Applying fixes...\n");
    }
    
    match introspector_core::apply_all_fixes(path, dry_run) {
        Ok(count) => {
            if dry_run {
                eprintln!("\n📊 Would apply {} fixes", count);
            } else {
                eprintln!("\n✅ Applied {} fixes", count);
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            process::exit(1);
        }
    }
}

fn cmd_diff(path: &str) {
    let path = Path::new(path);
    let diff = introspector_core::generate_diff(path);
    if diff.is_empty() {
        eprintln!("No changes needed for {}", path.display());
    } else {
        println!("{}", diff);
    }
}

fn cmd_tickets(filter: Option<&str>) {
    let tickets = introspector_core::get_all_tickets();
    
    let filtered: Vec<_> = if let Some(prefix) = filter {
        tickets.into_iter()
            .filter(|t| t.id.starts_with(prefix))
            .collect()
    } else {
        tickets
    };
    
    eprintln!("\n🎫 Audit Tickets ({})\n", filtered.len());
    eprintln!("{:<10} {:<8} {:<22} {}", "ID", "Sev", "Module", "Title");
    eprintln!("{}", "─".repeat(70));
    
    for ticket in filtered {
        let sev = match ticket.severity {
            introspector_core::Severity::Critical => "🔴 CRIT",
            introspector_core::Severity::High => "🟠 HIGH",
            introspector_core::Severity::Medium => "🟡 MED",
            introspector_core::Severity::Low => "🟢 LOW",
            introspector_core::Severity::Info => "ℹ️ INFO",
        };
        eprintln!("{:<10} {:<8} {:<22} {}", 
            ticket.id, sev, ticket.module, 
            ticket.title.chars().take(35).collect::<String>());
    }
}

fn cmd_ticket(id: &str) {
    if let Some(ticket) = introspector_core::get_ticket(id) {
        introspector_core::print_ticket(&ticket);
    } else {
        eprintln!("❌ Unknown ticket: {}", id);
        eprintln!("   Use 'tickets' command to list all tickets");
        process::exit(1);
    }
}

fn cmd_report() {
    let report = introspector_core::generate_macro_report();
    introspector_core::print_macro_report(&report);
}

fn cmd_decl_scan(path: &str) {
    let path = Path::new(path);
    let preview = introspector_core::preview_decl_wrappers(path);
    println!("{}", preview);
}

fn cmd_decl_wrap(path: &str, dry_run: bool) {
    let path = Path::new(path);
    
    if dry_run {
        let preview = introspector_core::preview_decl_wrappers(path);
        println!("{}", preview);
        eprintln!("🔍 Dry run - no changes made");
    } else {
        match introspector_core::apply_decl_wrappers(path) {
            Ok(count) => eprintln!("✅ Wrapped {} public declarations in {}", count, path.display()),
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn cmd_decl_json(path: &str) {
    let path = Path::new(path);
    let json = introspector_core::generate_declarations_json(path);
    println!("{}", json);
}
