extern crate proc_macro;

use proc_macro::TokenStream;

mod macros; // Declare the macros module

// figlet!
#[proc_macro]
pub fn figlet(input: TokenStream) -> TokenStream {
    macros::figlet::figlet_impl(input)
}

// codegen!
#[proc_macro]
pub fn codegen(input: TokenStream) -> TokenStream {
    macros::codegen::codegen_impl(input)
}

// ticket!
#[proc_macro]
pub fn ticket(input: TokenStream) -> TokenStream {
    macros::ticket::ticket_impl(input)
}

// pr!
#[proc_macro]
pub fn pr(input: TokenStream) -> TokenStream {
    macros::pr::pr_impl(input)
}

// branch!
#[proc_macro]
pub fn branch(input: TokenStream) -> TokenStream {
    macros::branch::branch_impl(input)
}

// bug!
#[proc_macro]
pub fn bug(input: TokenStream) -> TokenStream {
    macros::bug::bug_impl(input)
}

// compiler_parser_element!
#[proc_macro]
pub fn compiler_parser_element(input: TokenStream) -> TokenStream {
    macros::compiler_parser_element::compiler_parser_element_impl(input)
}

// compiler_type_check!
#[proc_macro]
pub fn compiler_type_check(input: TokenStream) -> TokenStream {
    macros::compiler_type_check::compiler_type_check_impl(input)
}

// cargo_manipulate!
#[proc_macro]
pub fn cargo_manipulate(input: TokenStream) -> TokenStream {
    macros::cargo_manipulate::cargo_manipulate_impl(input)
}

// replace_version!
#[proc_macro]
pub fn replace_version(input: TokenStream) -> TokenStream {
    macros::replace_version::replace_version_impl(input)
}

// use_vendored_submodules!
#[proc_macro]
pub fn use_vendored_submodules(input: TokenStream) -> TokenStream {
    macros::use_vendored_submodules::use_vendored_submodules_impl(input)
}

// fork_all_repos!
#[proc_macro]
pub fn fork_all_repos(input: TokenStream) -> TokenStream {
    macros::fork_all_repos::fork_all_repos_impl(input)
}

// replace_all_git_hub_actions!
#[proc_macro]
pub fn replace_all_git_hub_actions(input: TokenStream) -> TokenStream {
    macros::replace_all_git_hub_actions::replace_all_git_hub_actions_impl(input)
}

// make_everything_a_macro!
#[proc_macro]
pub fn make_everything_a_macro(input: TokenStream) -> TokenStream {
    macros::make_everything_a_macro::make_everything_a_macro_impl(input)
}

// dwim!
#[proc_macro]
pub fn dwim(input: TokenStream) -> TokenStream {
    macros::dwim::dwim_impl(input)
}

// meme!
#[proc_macro]
pub fn meme(input: TokenStream) -> TokenStream {
    macros::meme::meme_impl(input)
}

// videogen!
#[proc_macro]
pub fn videogen(input: TokenStream) -> TokenStream {
    macros::videogen::videogen_impl(input)
}

// llm!
#[proc_macro]
pub fn llm(input: TokenStream) -> TokenStream {
    macros::llm::llm_impl(input)
}

// toolcall!
#[proc_macro]
pub fn toolcall(input: TokenStream) -> TokenStream {
    macros::toolcall::toolcall_impl(input)
}

// results!
#[proc_macro]
pub fn results(input: TokenStream) -> TokenStream {
    macros::results::results_impl(input)
}

// mcp!
#[proc_macro]
pub fn mcp(input: TokenStream) -> TokenStream {
    macros::mcp::mcp_impl(input)
}

// service_finder!
#[proc_macro]
pub fn service_finder(input: TokenStream) -> TokenStream {
    macros::service_finder::service_finder_impl(input)
}

// biosemiotic!
#[proc_macro]
pub fn biosemiotic(input: TokenStream) -> TokenStream {
    macros::biosemiotic::biosemiotic_impl(input)
}

// github!
#[proc_macro]
pub fn github(input: TokenStream) -> TokenStream {
    macros::github::github_impl(input)
}

// huggingface!
#[proc_macro]
pub fn huggingface(input: TokenStream) -> TokenStream {
    macros::huggingface::huggingface_impl(input)
}

// codeberg!
#[proc_macro]
pub fn codeberg(input: TokenStream) -> TokenStream {
    macros::codeberg::codeberg_impl(input)
}

// twitter!
#[proc_macro]
pub fn twitter(input: TokenStream) -> TokenStream {
    macros::twitter::twitter_impl(input)
}

// discord!
#[proc_macro]
pub fn discord(input: TokenStream) -> TokenStream {
    macros::discord::discord_impl(input)
}

// telegram!
#[proc_macro]
pub fn telegram(input: TokenStream) -> TokenStream {
    macros::telegram::telegram_impl(input)
}

// reddit!
#[proc_macro]
pub fn reddit(input: TokenStream) -> TokenStream {
    macros::reddit::reddit_impl(input)
}

// wikipedia!
#[proc_macro]
pub fn wikipedia(input: TokenStream) -> TokenStream {
    macros::wikipedia::wikipedia_impl(input)
}

// wikidata!
#[proc_macro]
pub fn wikidata(input: TokenStream) -> TokenStream {
    macros::wikidata::wikidata_impl(input)
}

// osm!
#[proc_macro]
pub fn osm(input: TokenStream) -> TokenStream {
    macros::osm::osm_impl(input)
}

// foaf!
#[proc_macro]
pub fn foaf(input: TokenStream) -> TokenStream {
    macros::foaf::foaf_impl(input)
}

// owl!
#[proc_macro]
pub fn owl(input: TokenStream) -> TokenStream {
    macros::owl::owl_impl(input)
}

// oil!
#[proc_macro]
pub fn oil(input: TokenStream) -> TokenStream {
    macros::oil::oil_impl(input)
}

// daml!
#[proc_macro]
pub fn daml(input: TokenStream) -> TokenStream {
    macros::daml::daml_impl(input)
}

// cwm!
#[proc_macro]
pub fn cwm(input: TokenStream) -> TokenStream {
    macros::cwm::cwm_impl(input)
}

// euler!
#[proc_macro]
pub fn euler(input: TokenStream) -> TokenStream {
    macros::euler::euler_impl(input)
}