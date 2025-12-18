extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr, Ident, Token, parse::{Parse, ParseStream}, Result as SynResult};
use std::fs;
use std::path::PathBuf;
use brush_parser::{parse, ast::TopLevelCommand}; // Import brush-parser
// Import rustsat components for SAT solving
use rustsat::{
    instances::{Cnf, SatInstance},
    solvers::{
        batsat::{BatSat, BatSatConfig},
        Solve,
    },
    types::{Assignment, Lit, Var},
};
// Import asdi components for Datalog
use asdi::{Engine, Program}; // Assuming asdi is used for Datalog

// Represents the arguments for the shebling! macro
struct SheblingArgs {
    path_key: Ident, // "path"
    _colon1: Token![:],
    path: LitStr,
}

impl Parse for SheblingArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let path_key: Ident = input.parse()?;
        if path_key != "path" {
            return Err(input.error("Expected 'path' as the argument"));
        }
        let _colon1 = input.parse()?;
        let path = input.parse()?;
        Ok(SheblingArgs { path_key, _colon1, path })
    }
}

#[proc_macro]
pub fn shebling(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as SheblingArgs);
    let script_path = args.path.value();
    let span = args.path.span();

    let full_path = PathBuf::from(&script_path);

    let script_content = match fs::read_to_string(&full_path) {
        Ok(content) => content,
        Err(e) => {
            return quote_spanned! {span =>
                compile_error!(format!("Failed to read shell script at {}: {}", #script_path, #e));
            }
            .into();
        }
    };

    let parsed_ast_string = match parse(&script_content) {
        Ok(ast) => format!("{:#?}", ast), // Pretty-print the AST
        Err(e) => {
            return quote_spanned! {span =>
                compile_error!(format!("Failed to parse shell script at {}: {}", #script_path, #e));
            }
            .into();
        }
    };

    let expanded = quote! {
        println!("shebling! macro invoked for script: {}", #script_path);
        println!("--- AST for {}: ---\n{}", #script_path, #parsed_ast_string);
        println!("------------------------------------");
        
        #[allow(dead_code)]
        const SHELL_SCRIPT_CONTENT: &str = #script_content;

        #[allow(dead_code)]
        pub struct SheblingAnalysis {
            pub script_path: &'static str,
            pub content_len: usize,
            pub ast_dump: &'static str,
        }

        #[allow(dead_code)]
        pub const SHEBLING_ANALYSIS: SheblingAnalysis = SheblingAnalysis {
            script_path: #script_path,
            content_len: #script_content.len(),
            ast_dump: #parsed_ast_string,
        };
    };

    expanded.into()
}

// --- bash! macro definition ---

#[proc_macro]
pub fn bash(input: TokenStream) -> TokenStream {
    let script_literal = parse_macro_input!(input as LitStr);
    let script_content = script_literal.value();
    let span = script_literal.span();

    let parsed_ast_string = match parse(&script_content) {
        Ok(ast) => format!("{:#?}", ast), // Pretty-print the AST
        Err(e) => {
            return quote_spanned! {span =>
                compile_error!(format!("Failed to parse bash snippet: {}", #e));
            }
            .into();
        }
    };

    // --- Placeholder for OWL Ontology to CNF Conversion ---
    let mut cnf = Cnf::new();
    let mut instance = SatInstance::new();

    let var_script_valid = instance.new_var();
    let var_pipe_correct = instance.new_var();
    let var_has_grep_filter = instance.new_var();

    cnf.add_clause([Lit::new(!var_script_valid), Lit::new(var_pipe_correct)]);
    cnf.add_clause([Lit::new(!var_pipe_correct), Lit::new(!var_has_grep_filter)]);

    cnf.add_unit(Lit::new(var_script_valid));
    cnf.add_unit(Lit::new(var_pipe_correct));
    cnf.add_unit(Lit::new(var_has_grep_filter));

    // --- SAT Solver Invocation ---
    let mut solver = BatSat::new(BatSatConfig::default());
    let sat_result = solver.solve(&cnf); // Solve the CNF

    let is_simulated_unsat = script_content.contains("grep") && script_content.contains("echo");

    let sat_check_code = if is_simulated_unsat {
        quote_spanned! {span =>
            compile_error!(format!(
                "Bash script is UNSAT! Logical contradiction detected by SAT solver. (Simulated UNSAT for 'grep' and 'echo' together). Raw SAT result: {:?}",
                #sat_result
            ));
        }
    } else {
        quote! {
            match #sat_result {
                Ok(Some(Assignment::Sat)) => {
                    println!("Bash script is SAT: Ontology axioms are satisfiable for this AST.");
                },
                Ok(Some(Assignment::Unsat)) => {
                    compile_error!("Bash script is UNSAT: Logical contradiction detected by SAT solver.");
                },
                Ok(None) => {
                    compile_error!("SAT solver returned unknown result (e.g., interrupted).");
                },
                Err(e) => {
                    compile_error!(format!("SAT solver encountered an error: {:?}", e));
                },
            }
        }
    };

    // --- Datalog Integration Placeholder ---
    // Here, we would extract Datalog facts from the parsed AST and potentially the SAT solver's witness.
    // For this example, let's assume we extract a fact about the presence of a pipeline.

    let mut datalog_program = Program::new();
    let mut datalog_engine = Engine::new();

    // Conceptual Datalog facts based on script_content
    let has_pipeline_fact = if script_content.contains("|") {
        quote! { datalog_engine.add_fact("has_pipeline", &[]); }
    } else {
        quote! {}
    };
    
    // Conceptual Datalog rules
    let datalog_rules = r#"
        valid_script :- has_pipeline.
        valid_script :- !has_pipeline, empty_script.
        empty_script :- "True". // Placeholder for a rule indicating an empty script is valid
    "#;

    // Load conceptual program and rules
    datalog_program.load_str(datalog_rules).expect("Failed to load Datalog rules.");
    datalog_engine.load_program(datalog_program);

    // Conceptual Datalog query
    let datalog_query_result = datalog_engine.query("?- valid_script.").expect("Failed to query Datalog engine.");
    let is_datalog_valid = !datalog_query_result.is_empty();

    let datalog_check_code = if !is_datalog_valid {
        quote_spanned! {span =>
            compile_error!("Bash script is Datalog UNSAT: Datalog query 'valid_script' returned no results. (Simulated check)");
        }
    } else {
        quote! {
            println!("Bash script is Datalog SAT: Datalog query 'valid_script' returned results.");
        }
    };

    let expanded = quote! {
        println!("bash! macro invoked for snippet:");
        println!("--- AST for snippet: ---\n{}", #parsed_ast_string);
        println!("------------------------------------");

        #sat_check_code // Include the SAT check logic
        #datalog_check_code // Include the Datalog check logic

        #[allow(dead_code)]
        const BASH_SNIPPET_CONTENT: &str = #script_content;

        #[allow(dead_code)]
        pub struct BashSnippetAnalysis {
            pub content: &'static str,
            pub ast_dump: &'static str,
        }

        #[allow(dead_code)]
        pub const BASH_SNIPPET_ANALYSIS: BashSnippetAnalysis = BashSnippetAnalysis {
            content: #script_content,
            ast_dump: #parsed_ast_string,
        };
    };

    expanded.into()
}
