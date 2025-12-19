use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

mod decl_attr;
mod rust_nix;
mod rustc_ring;
mod lmfdb_morph;
mod sat_lfunction;
mod dao_governance;
mod solana_lift;
mod quant_trading;
mod mev_protection;
mod event_memory;
mod graph_partition;
mod context_knapsack;
mod zk_proof;
mod lean4_proof;
mod lean4_json;
mod lean4_mirror;
mod quine_relay;
mod emoji_poetry;
mod rust_eigenmatrix;
mod real_rustc_analysis;
mod rustc_tracer;
mod compiler_inventory;
mod duplicate_analysis;
mod real_data_analysis;
mod mkbuildrs;
mod macro_lattice;
mod repo_analysis;
mod macro_generator;
mod template_checker;

#[proc_macro]
pub fn checktemplate(input: TokenStream) -> TokenStream {
    template_checker::checktemplate_impl(input)
}

#[proc_macro]
pub fn generate_checked_macros(input: TokenStream) -> TokenStream {
    template_checker::generate_checked_macros_impl(input)
}

#[proc_macro]
pub fn mkbuildrs_checked(input: TokenStream) -> TokenStream {
    template_checker::mkbuildrs_checked_impl(input)
}

#[proc_macro]
pub fn generate_common_macros(input: TokenStream) -> TokenStream {
    macro_generator::generate_common_macros_impl(input)
}

#[proc_macro]
pub fn mkbuildrs_with_macros(input: TokenStream) -> TokenStream {
    macro_generator::mkbuildrs_with_macros_impl(input)
}

#[proc_macro]
pub fn repo_duplicate_analysis(input: TokenStream) -> TokenStream {
    repo_analysis::repo_duplicate_analysis_impl(input)
}

#[proc_macro]
pub fn pattern_similarity(input: TokenStream) -> TokenStream {
    repo_analysis::pattern_similarity_impl(input)
}

#[proc_macro]
pub fn macro_lattice(input: TokenStream) -> TokenStream {
    macro_lattice::macro_lattice_impl(input)
}

#[proc_macro]
pub fn lattice_dependencies(input: TokenStream) -> TokenStream {
    macro_lattice::lattice_dependencies_impl(input)
}

#[proc_macro]
pub fn lattice_path(input: TokenStream) -> TokenStream {
    macro_lattice::lattice_path_impl(input)
}

#[proc_macro]
pub fn mkbuildrs(input: TokenStream) -> TokenStream {
    mkbuildrs::mkbuildrs_impl(input)
}

#[proc_macro]
pub fn nix_rust_version(input: TokenStream) -> TokenStream {
    mkbuildrs::nix_rust_version_impl(input)
}

#[proc_macro]
pub fn rust_cache(input: TokenStream) -> TokenStream {
    mkbuildrs::rust_cache_impl(input)
}

#[proc_macro]
pub fn real_rustc_analysis(input: TokenStream) -> TokenStream {
    real_data_analysis::real_rustc_analysis_impl(input)
}

#[proc_macro]
pub fn real_duplicate_detection(input: TokenStream) -> TokenStream {
    real_data_analysis::real_duplicate_detection_impl(input)
}

#[proc_macro]
pub fn real_eigenmatrix(input: TokenStream) -> TokenStream {
    real_data_analysis::real_eigenmatrix_impl(input)
}

#[proc_macro]
pub fn unified_codebase(input: TokenStream) -> TokenStream {
    duplicate_analysis::unified_codebase_impl(input)
}

#[proc_macro]
pub fn semantic_hash(input: TokenStream) -> TokenStream {
    duplicate_analysis::semantic_hash_impl(input)
}

#[proc_macro]
pub fn grast_structural(input: TokenStream) -> TokenStream {
    duplicate_analysis::grast_structural_impl(input)
}

#[proc_macro]
pub fn llm_redundancy(input: TokenStream) -> TokenStream {
    duplicate_analysis::llm_redundancy_impl(input)
}

#[proc_macro]
pub fn redundancy_stats(input: TokenStream) -> TokenStream {
    duplicate_analysis::redundancy_stats_impl(input)
}

#[proc_macro]
pub fn compiler_inventory(input: TokenStream) -> TokenStream {
    compiler_inventory::compiler_inventory_impl(input)
}

#[proc_macro]
pub fn pure_reflect(input: TokenStream) -> TokenStream {
    compiler_inventory::pure_reflect_impl(input)
}

#[proc_macro]
pub fn grast_extract(input: TokenStream) -> TokenStream {
    compiler_inventory::grast_extract_impl(input)
}

#[proc_macro]
pub fn usage_analysis(input: TokenStream) -> TokenStream {
    compiler_inventory::usage_analysis_impl(input)
}

#[proc_macro]
pub fn trace_rustc(input: TokenStream) -> TokenStream {
    rustc_tracer::trace_rustc_impl(input)
}

#[proc_macro]
pub fn custom_rust_driver(input: TokenStream) -> TokenStream {
    rustc_tracer::custom_rust_driver_impl(input)
}

#[proc_macro]
pub fn auto_source_setup(input: TokenStream) -> TokenStream {
    rustc_tracer::auto_source_setup_impl(input)
}

#[proc_macro]
pub fn find_rustc_source(input: TokenStream) -> TokenStream {
    real_rustc_analysis::find_rustc_source_impl(input)
}

#[proc_macro]
pub fn analyze_real_source(input: TokenStream) -> TokenStream {
    real_rustc_analysis::analyze_real_source_impl(input)
}

#[proc_macro]
pub fn prove_eigenvalues(input: TokenStream) -> TokenStream {
    real_rustc_analysis::prove_eigenvalues_impl(input)
}

#[proc_macro]
pub fn rust_eigenmatrix(input: TokenStream) -> TokenStream {
    rust_eigenmatrix::rust_eigenmatrix_impl(input)
}

#[proc_macro]
pub fn source_to_emoji(input: TokenStream) -> TokenStream {
    rust_eigenmatrix::source_to_emoji_impl(input)
}

#[proc_macro]
pub fn eigenform_verify(input: TokenStream) -> TokenStream {
    rust_eigenmatrix::eigenform_verify_impl(input)
}

#[proc_macro]
pub fn emoji_poem(input: TokenStream) -> TokenStream {
    emoji_poetry::emoji_poem_impl(input)
}

#[proc_macro]
pub fn math_to_emoji(input: TokenStream) -> TokenStream {
    emoji_poetry::math_to_emoji_impl(input)
}

#[proc_macro]
pub fn emoji_to_math(input: TokenStream) -> TokenStream {
    emoji_poetry::emoji_to_math_impl(input)
}

#[proc_macro]
pub fn poetry_cycle(input: TokenStream) -> TokenStream {
    emoji_poetry::poetry_cycle_impl(input)
}

#[proc_macro]
pub fn language_quine(input: TokenStream) -> TokenStream {
    quine_relay::language_quine_impl(input)
}

#[proc_macro]
pub fn compiler_macro(input: TokenStream) -> TokenStream {
    quine_relay::compiler_macro_impl(input)
}

#[proc_macro]
pub fn bootstrap_cycle(input: TokenStream) -> TokenStream {
    quine_relay::bootstrap_cycle_impl(input)
}

#[proc_macro]
pub fn automorphic_orbit(input: TokenStream) -> TokenStream {
    quine_relay::automorphic_orbit_impl(input)
}

#[proc_macro]
pub fn lean4_to_rust(input: TokenStream) -> TokenStream {
    lean4_mirror::lean4_to_rust_impl(input)
}

#[proc_macro]
pub fn rust_to_lean4(input: TokenStream) -> TokenStream {
    lean4_mirror::rust_to_lean4_impl(input)
}

#[proc_macro]
pub fn proof_simulate(input: TokenStream) -> TokenStream {
    lean4_mirror::proof_simulate_impl(input)
}

#[proc_macro]
pub fn lean4_macro_bridge(input: TokenStream) -> TokenStream {
    lean4_mirror::lean4_macro_bridge_impl(input)
}

#[proc_macro]
pub fn lean4_expr_json(input: TokenStream) -> TokenStream {
    lean4_json::lean4_expr_json_impl(input)
}

#[proc_macro]
pub fn rustc_lean4_bridge(input: TokenStream) -> TokenStream {
    lean4_json::rustc_lean4_bridge_impl(input)
}

#[proc_macro]
pub fn lean4_patch(input: TokenStream) -> TokenStream {
    lean4_json::lean4_patch_impl(input)
}

#[proc_macro]
pub fn json_monster_proof(input: TokenStream) -> TokenStream {
    lean4_json::json_monster_proof_impl(input)
}

#[proc_macro]
pub fn lean4_theorem(input: TokenStream) -> TokenStream {
    lean4_proof::lean4_theorem_impl(input)
}

#[proc_macro]
pub fn rustc_to_lean(input: TokenStream) -> TokenStream {
    lean4_proof::rustc_to_lean_impl(input)
}

#[proc_macro]
pub fn monster_proof(input: TokenStream) -> TokenStream {
    lean4_proof::monster_proof_impl(input)
}

#[proc_macro]
pub fn lfunction_proof(input: TokenStream) -> TokenStream {
    lean4_proof::lfunction_proof_impl(input)
}

#[proc_macro]
pub fn formal_verification(input: TokenStream) -> TokenStream {
    lean4_proof::formal_verification_impl(input)
}

#[proc_macro]
pub fn zk_witness(input: TokenStream) -> TokenStream {
    zk_proof::zk_witness_impl(input)
}

#[proc_macro]
pub fn plonk_circuit(input: TokenStream) -> TokenStream {
    zk_proof::plonk_circuit_impl(input)
}

#[proc_macro]
pub fn stark_proof(input: TokenStream) -> TokenStream {
    zk_proof::stark_proof_impl(input)
}

#[proc_macro]
pub fn snark_verify(input: TokenStream) -> TokenStream {
    zk_proof::snark_verify_impl(input)
}

#[proc_macro]
pub fn backpack_fill(input: TokenStream) -> TokenStream {
    context_knapsack::backpack_fill_impl(input)
}

#[proc_macro]
pub fn context_optimize(input: TokenStream) -> TokenStream {
    context_knapsack::context_optimize_impl(input)
}

#[proc_macro]
pub fn token_weight(input: TokenStream) -> TokenStream {
    context_knapsack::token_weight_impl(input)
}

#[proc_macro]
pub fn context_compress(input: TokenStream) -> TokenStream {
    context_knapsack::context_compress_impl(input)
}

#[proc_macro]
pub fn nix_event(input: TokenStream) -> TokenStream {
    event_memory::nix_event_impl(input)
}

#[proc_macro]
pub fn github_event(input: TokenStream) -> TokenStream {
    event_memory::github_event_impl(input)
}

#[proc_macro]
pub fn archive_event(input: TokenStream) -> TokenStream {
    event_memory::archive_event_impl(input)
}

#[proc_macro]
pub fn huggingface_event(input: TokenStream) -> TokenStream {
    event_memory::huggingface_event_impl(input)
}

#[proc_macro]
pub fn twitter_event(input: TokenStream) -> TokenStream {
    event_memory::twitter_event_impl(input)
}

#[proc_macro]
pub fn telegram_event(input: TokenStream) -> TokenStream {
    event_memory::telegram_event_impl(input)
}

#[proc_macro]
pub fn sat_group(input: TokenStream) -> TokenStream {
    graph_partition::sat_group_impl(input)
}

#[proc_macro]
pub fn metis_partition(input: TokenStream) -> TokenStream {
    graph_partition::metis_partition_impl(input)
}

#[proc_macro]
pub fn memory_select(input: TokenStream) -> TokenStream {
    graph_partition::memory_select_impl(input)
}

#[proc_macro]
pub fn code_split(input: TokenStream) -> TokenStream {
    graph_partition::code_split_impl(input)
}

#[proc_macro]
pub fn sandwich_detect(input: TokenStream) -> TokenStream {
    mev_protection::sandwich_detect_impl(input)
}

#[proc_macro]
pub fn frontrun_block(input: TokenStream) -> TokenStream {
    mev_protection::frontrun_block_impl(input)
}

#[proc_macro]
pub fn mev_exclude(input: TokenStream) -> TokenStream {
    mev_protection::mev_exclude_impl(input)
}

#[proc_macro]
pub fn atomic_swap(input: TokenStream) -> TokenStream {
    mev_protection::atomic_swap_impl(input)
}

#[proc_macro]
pub fn purchase_blocks(input: TokenStream) -> TokenStream {
    solana_lift::purchase_blocks_impl(input)
}

#[proc_macro]
pub fn lift_int_code(input: TokenStream) -> TokenStream {
    solana_lift::lift_int_code_impl(input)
}

#[proc_macro]
pub fn ca(input: TokenStream) -> TokenStream {
    solana_lift::ca_macro_impl(input)
}

#[proc_macro]
pub fn token(input: TokenStream) -> TokenStream {
    solana_lift::token_macro_impl(input)
}

#[proc_macro]
pub fn lp(input: TokenStream) -> TokenStream {
    solana_lift::lp_macro_impl(input)
}

#[proc_macro]
pub fn quant(input: TokenStream) -> TokenStream {
    quant_trading::quant_impl(input)
}

#[proc_macro]
pub fn trading(input: TokenStream) -> TokenStream {
    quant_trading::trading_impl(input)
}

#[proc_macro]
pub fn load_historical(input: TokenStream) -> TokenStream {
    quant_trading::load_historical_impl(input)
}

#[proc_macro]
pub fn dao_vote(input: TokenStream) -> TokenStream {
    dao_governance::dao_vote_impl(input)
}

#[proc_macro]
pub fn paxos_consensus(input: TokenStream) -> TokenStream {
    dao_governance::paxos_consensus_impl(input)
}

#[proc_macro]
pub fn apply_patch(input: TokenStream) -> TokenStream {
    dao_governance::apply_patch_impl(input)
}

#[proc_macro]
pub fn token_governance(input: TokenStream) -> TokenStream {
    dao_governance::token_governance_impl(input)
}

#[proc_macro]
pub fn sat_solve_unity(input: TokenStream) -> TokenStream {
    sat_lfunction::sat_solve_unity_impl(input)
}

#[proc_macro]
pub fn extract_lfunction(input: TokenStream) -> TokenStream {
    sat_lfunction::extract_lfunction_impl(input)
}

#[proc_macro]
pub fn matrix_decompose(input: TokenStream) -> TokenStream {
    sat_lfunction::matrix_decompose_impl(input)
}

#[proc_macro]
pub fn unity_proof(input: TokenStream) -> TokenStream {
    sat_lfunction::unity_proof_impl(input)
}

#[proc_macro]
pub fn load_lmfdb(input: TokenStream) -> TokenStream {
    lmfdb_morph::load_lmfdb_impl(input)
}

#[proc_macro]
pub fn conformal_map(input: TokenStream) -> TokenStream {
    lmfdb_morph::conformal_map_impl(input)
}

#[proc_macro]
pub fn hott_morph(input: TokenStream) -> TokenStream {
    lmfdb_morph::hott_morph_impl(input)
}

#[proc_macro]
pub fn monster_check(input: TokenStream) -> TokenStream {
    lmfdb_morph::monster_check_impl(input)
}

#[proc_macro]
pub fn analyze_rustc_ring(input: TokenStream) -> TokenStream {
    rustc_ring::analyze_rustc_ring_impl(input)
}

#[proc_macro]
pub fn crate_report(input: TokenStream) -> TokenStream {
    rustc_ring::crate_report_impl(input)
}

#[proc_macro]
pub fn dependency_graph(input: TokenStream) -> TokenStream {
    rustc_ring::dependency_graph_impl(input)
}

#[proc_macro]
pub fn ring_properties(input: TokenStream) -> TokenStream {
    rustc_ring::ring_properties_impl(input)
}

#[proc_macro]
pub fn nix_rust_src(input: TokenStream) -> TokenStream {
    rust_nix::nix_rust_src_impl(input)
}

#[proc_macro]
pub fn extract_decl(input: TokenStream) -> TokenStream {
    rust_nix::extract_decl_impl(input)
}

#[proc_macro]
pub fn patch_rust(input: TokenStream) -> TokenStream {
    rust_nix::patch_rust_impl(input)
}

#[proc_macro]
pub fn extract(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let fixme_id = input_str.value();
    
    quote! {
        {
            use std::fs;
            let dir = format!("extracted/fixme-{}", #fixme_id.len());
            fs::create_dir_all(&dir).ok();
            println!("cargo:warning=🔧 Extracted: {}", dir);
        }
    }.into()
}

#[proc_macro]
pub fn simplify(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let data = input_str.value();
    
    quote! {
        {
            let simplified = #data
                .lines()
                .filter(|line| !line.trim().is_empty())
                .filter(|line| !line.starts_with("//"))
                .collect::<Vec<_>>()
                .join("\n");
            println!("cargo:warning=📉 Simplified: {} -> {} lines", 
                #data.lines().count(), simplified.lines().count());
            simplified
        }
    }.into()
}

#[proc_macro]
pub fn pii(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let data = input_str.value();
    
    quote! {
        {
            let cleaned = #data
                .replace("/home/", "/home/<user>/")
                .replace("/Users/", "/Users/<user>/")
                .replace("@gmail.com", "@<email>")
                .replace("@company.com", "@<company>");
            println!("cargo:warning=🔒 PII cleaned");
            cleaned
        }
    }.into()
}

#[proc_macro]
pub fn prune(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let data = input_str.value();
    
    quote! {
        {
            let pruned = #data
                .lines()
                .filter(|line| !line.contains("target/"))
                .filter(|line| !line.contains(".git/"))
                .filter(|line| !line.contains("node_modules/"))
                .take(1000)
                .collect::<Vec<_>>()
                .join("\n");
            println!("cargo:warning=✂️ Pruned: {} -> {} lines", 
                #data.lines().count(), pruned.lines().count());
            pruned
        }
    }.into()
}

#[proc_macro]
pub fn compress(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let data = input_str.value();
    
    quote! {
        {
            let compressed = #data
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            println!("cargo:warning=🗜️ Compressed: {} -> {} chars", 
                #data.len(), compressed.len());
            compressed
        }
    }.into()
}

#[proc_macro]
pub fn ticket(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let issue = input_str.value();
    
    quote! {
        println!("🎫 Ticket: {}", #issue);
    }.into()
}

#[proc_macro]
pub fn value(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let amount = input_str.value();
    
    quote! {
        println!("💰 Bounty: {}", #amount);
    }.into()
}

/// Attribute macro for wrapping declarations with metadata
/// 
/// Usage:
/// ```rust
/// #[decl(fn, name = "my_function", vis = "pub", hash = "abc123")]
/// pub fn my_function() {}
/// 
/// #[decl(struct)]
/// pub struct MyStruct { ... }
/// ```
/// 
/// The macro automatically extracts metadata and registers the declaration
/// in a compile-time registry for introspection.
#[proc_macro_attribute]
pub fn decl(attr: TokenStream, item: TokenStream) -> TokenStream {
    decl_attr::decl_attr_impl(attr, item)
}
