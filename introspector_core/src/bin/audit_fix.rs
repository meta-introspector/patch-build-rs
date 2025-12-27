use std::env;
use std::path::{Path, PathBuf};
use std::process;
use walkdir::WalkDir; // Add walkdir import

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        process::exit(1);
    }
    
    let command = &args[1];
    let mut path_arg: Option<&str> = None;
    let mut dry_run = false;
    let mut recursive = false;

    // Parse global options and subcommand arguments
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--dry-run" | "-n" => dry_run = true,
            "--recursive" | "-r" => recursive = true,
            _ => {
                if path_arg.is_none() {
                    path_arg = Some(&args[i]);
                } else {
                    // This command already has a path, so this must be an unknown argument
                    eprintln!("Error: Unexpected argument '{}'", args[i]);
                    print_usage(&args[0]);
                    process::exit(1);
                }
            }
        }
        i += 1;
    }

    let current_path = path_arg.unwrap_or(".");
    
    match command.as_str() {
        "scan" => {
            cmd_scan(current_path);
        }
        "preview" => {
            if path_arg.is_none() {
                eprintln!("Error: preview requires a file path");
                process::exit(1);
            }
            cmd_preview(current_path);
        }
        "fix" => {
            cmd_fix(current_path, dry_run);
        }
        "diff" => {
            if path_arg.is_none() {
                eprintln!("Error: diff requires a file path");
                process::exit(1);
            }
            cmd_diff(current_path);
        }
        "tickets" => {
            let filter = path_arg;
            cmd_tickets(filter);
        }
        "ticket" => {
            if path_arg.is_none() {
                eprintln!("Error: ticket requires a ticket ID");
                process::exit(1);
            }
            cmd_ticket(current_path);
        }
        "report" => {
            cmd_report();
        }
        "decl-scan" => {
            if path_arg.is_none() {
                eprintln!("Error: decl-scan requires a file path");
                process::exit(1);
            }
            cmd_decl_scan(current_path);
        }
        "decl-wrap" => {
            cmd_decl_wrap(current_path, dry_run, recursive);
        }
        "decl-json" => {
            if path_arg.is_none() {
                eprintln!("Error: decl-json requires a file path");
                process::exit(1);
            }
            cmd_decl_json(current_path);
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
    eprintln!(
r#"#,
"🔧 Audit Fix Tool - Automatic audit_id! and decl! injection

USAGE:
    {} <COMMAND> [PATH] [OPTIONS] 

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
    decl-wrap [PATH] [--dry-run] [--recursive]  Wrap public declarations with #[decl(...)]
    decl-json <FILE>    Export declarations as JSON

OPTIONS:
    --dry-run, -n       Show what would be fixed without changing files
    --recursive, -r     Apply command recursively to all .rs files in a directory
    --help, -h          Show this help message

EXAMPLES:
    {} scan src/
    {} preview src/solana_lift.rs
    {} fix src/ --dry-run
    {} decl-scan src/lib.rs
    {} decl-wrap src/lib.rs --dry-run
    {} decl-wrap . --recursive
    {} decl-json src/lib.rs > decls.json
"#, program, program, program, program, program, program, program, program);
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

fn cmd_decl_wrap(root_path: &str, dry_run: bool, recursive: bool) {
    let path = PathBuf::from(root_path);
    let mut files_processed = 0;

    if recursive {
        if !path.is_dir() {
            eprintln!("Error: --recursive can only be used with a directory path.");
            process::exit(1);
        }
        eprintln!("🔍 {} recursively in {}...\n", if dry_run { "Previewing" } else { "Wrapping" }, path.display());
        for entry in WalkDir::new(&path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "rs"))
        {
            let file_path = entry.path();
            if dry_run {
                let preview = introspector_core::preview_decl_wrappers(file_path);
                println!("{}", preview);
            } else {
                match introspector_core::apply_decl_wrappers(file_path) {
                    Ok(count) => {
                        eprintln!("✅ Wrapped {} public declarations in {}", count, file_path.display());
                        files_processed += 1;
                    }
                    Err(e) => {
                        eprintln!("❌ Error wrapping {}: {}", file_path.display(), e);
                    }
                }
            }
        }
        if dry_run {
            eprintln!("🔍 Dry run - no changes made across {} files", files_processed);
        } else {
            eprintln!("✅ Completed wrapping declarations in {} files", files_processed);
        }
    } else {
        if !path.is_file() {
            eprintln!("Error: {} is not a file. Use --recursive for directories.", path.display());
            process::exit(1);
        }
        if dry_run {
            let preview = introspector_core::preview_decl_wrappers(&path);
            println!("{}", preview);
            eprintln!("🔍 Dry run - no changes made");
        } else {
            match introspector_core::apply_decl_wrappers(&path) {
                Ok(count) => eprintln!("✅ Wrapped {} public declarations in {}", count, path.display()),
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}


fn cmd_decl_json(path: &str) {
    let path = Path::new(path);
    let json = introspector_core::generate_declarations_json(path);
    println!("{}", json);
}