extern crate proc_macro;
use patch_build_rs_macros::decl;
use proc_macro::TokenStream;

mod macros; // Declare the macros module

// figlet!
#[proc_macro]
#[decl(fn, name = "figlet", vis = "pub", hash = "557915a5")]
pub fn figlet(input: TokenStream) -> TokenStream {
    macros::figlet::figlet_impl(input)
}

// codegen!
#[proc_macro]
#[decl(fn, name = "codegen", vis = "pub", hash = "7a67ad6e")]
pub fn codegen(input: TokenStream) -> TokenStream {
    macros::codegen::codegen_impl(input)
}

// ticket!
#[proc_macro]
#[decl(fn, name = "ticket", vis = "pub", hash = "7a2e21f6")]
pub fn ticket(input: TokenStream) -> TokenStream {
    macros::ticket::ticket_impl(input)
}

// pr!
#[proc_macro]
#[decl(fn, name = "pr", vis = "pub", hash = "439ade7e")]
pub fn pr(input: TokenStream) -> TokenStream {
    macros::pr::pr_impl(input)
}

// branch!
#[proc_macro]
#[decl(fn, name = "branch", vis = "pub", hash = "c2bf67ba")]
pub fn branch(input: TokenStream) -> TokenStream {
    macros::branch::branch_impl(input)
}

// bug!
#[proc_macro]
#[decl(fn, name = "bug", vis = "pub", hash = "363cb3e8")]
pub fn bug(input: TokenStream) -> TokenStream {
    macros::bug::bug_impl(input)
}

// compiler_parser_element!
#[proc_macro]
#[decl(fn, name = "compiler_parser_element", vis = "pub", hash = "df0f80a4")]
pub fn compiler_parser_element(input: TokenStream) -> TokenStream {
    macros::compiler_parser_element::compiler_parser_element_impl(input)
}

// compiler_type_check!
#[proc_macro]
#[decl(fn, name = "compiler_type_check", vis = "pub", hash = "a74e17ed")]
pub fn compiler_type_check(input: TokenStream) -> TokenStream {
    macros::compiler_type_check::compiler_type_check_impl(input)
}

// cargo_manipulate!
#[proc_macro]
#[decl(fn, name = "cargo_manipulate", vis = "pub", hash = "807cd9e0")]
pub fn cargo_manipulate(input: TokenStream) -> TokenStream {
    macros::cargo_manipulate::cargo_manipulate_impl(input)
}

// replace_version!
#[proc_macro]
#[decl(fn, name = "replace_version", vis = "pub", hash = "9e3d56df")]
pub fn replace_version(input: TokenStream) -> TokenStream {
    macros::replace_version::replace_version_impl(input)
}

// use_vendored_submodules!
#[proc_macro]
#[decl(fn, name = "use_vendored_submodules", vis = "pub", hash = "927fa3d4")]
pub fn use_vendored_submodules(input: TokenStream) -> TokenStream {
    macros::use_vendored_submodules::use_vendored_submodules_impl(input)
}

// fork_all_repos!
#[proc_macro]
#[decl(fn, name = "fork_all_repos", vis = "pub", hash = "4ac3f475")]
pub fn fork_all_repos(input: TokenStream) -> TokenStream {
    macros::fork_all_repos::fork_all_repos_impl(input)
}

// replace_all_git_hub_actions!
#[proc_macro]
#[decl(fn, name = "replace_all_git_hub_actions", vis = "pub", hash = "8dce1993")]
pub fn replace_all_git_hub_actions(input: TokenStream) -> TokenStream {
    macros::replace_all_git_hub_actions::replace_all_git_hub_actions_impl(input)
}

// make_everything_a_macro!
#[proc_macro]
#[decl(fn, name = "make_everything_a_macro", vis = "pub", hash = "69a030ad")]
pub fn make_everything_a_macro(input: TokenStream) -> TokenStream {
    macros::make_everything_a_macro::make_everything_a_macro_impl(input)
}

// dwim!
#[proc_macro]
#[decl(fn, name = "dwim", vis = "pub", hash = "20bca028")]
pub fn dwim(input: TokenStream) -> TokenStream {
    macros::dwim::dwim_impl(input)
}

// meme!
#[proc_macro]
#[decl(fn, name = "meme", vis = "pub", hash = "17e0d2e0")]
pub fn meme(input: TokenStream) -> TokenStream {
    macros::meme::meme_impl(input)
}

// videogen!
#[proc_macro]
#[decl(fn, name = "videogen", vis = "pub", hash = "6e0880ff")]
pub fn videogen(input: TokenStream) -> TokenStream {
    macros::videogen::videogen_impl(input)
}

// llm!
#[proc_macro]
#[decl(fn, name = "llm", vis = "pub", hash = "16e02522")]
pub fn llm(input: TokenStream) -> TokenStream {
    macros::llm::llm_impl(input)
}

// toolcall!
#[proc_macro]
#[decl(fn, name = "toolcall", vis = "pub", hash = "e3d5663a")]
pub fn toolcall(input: TokenStream) -> TokenStream {
    macros::toolcall::toolcall_impl(input)
}

// results!
#[proc_macro]
#[decl(fn, name = "results", vis = "pub", hash = "5f3457b2")]
pub fn results(input: TokenStream) -> TokenStream {
    macros::results::results_impl(input)
}

// mcp!
#[proc_macro]
#[decl(fn, name = "mcp", vis = "pub", hash = "b9e0e3db")]
pub fn mcp(input: TokenStream) -> TokenStream {
    macros::mcp::mcp_impl(input)
}

// service_finder!
#[proc_macro]
#[decl(fn, name = "service_finder", vis = "pub", hash = "7a15222d")]
pub fn service_finder(input: TokenStream) -> TokenStream {
    macros::service_finder::service_finder_impl(input)
}

// biosemiotic!
#[proc_macro]
#[decl(fn, name = "biosemiotic", vis = "pub", hash = "81f81afc")]
pub fn biosemiotic(input: TokenStream) -> TokenStream {
    macros::biosemiotic::biosemiotic_impl(input)
}

// github!
#[proc_macro]
#[decl(fn, name = "github", vis = "pub", hash = "b2981013")]
pub fn github(input: TokenStream) -> TokenStream {
    macros::github::github_impl(input)
}

// huggingface!
#[proc_macro]
#[decl(fn, name = "huggingface", vis = "pub", hash = "039cf2da")]
pub fn huggingface(input: TokenStream) -> TokenStream {
    macros::huggingface::huggingface_impl(input)
}

// codeberg!
#[proc_macro]
#[decl(fn, name = "codeberg", vis = "pub", hash = "d565a312")]
pub fn codeberg(input: TokenStream) -> TokenStream {
    macros::codeberg::codeberg_impl(input)
}

// twitter!
#[proc_macro]
#[decl(fn, name = "twitter", vis = "pub", hash = "758954c5")]
pub fn twitter(input: TokenStream) -> TokenStream {
    macros::twitter::twitter_impl(input)
}

// discord!
#[proc_macro]
#[decl(fn, name = "discord", vis = "pub", hash = "c4bfd671")]
pub fn discord(input: TokenStream) -> TokenStream {
    macros::discord::discord_impl(input)
}

// telegram!
#[proc_macro]
#[decl(fn, name = "telegram", vis = "pub", hash = "378965a5")]
pub fn telegram(input: TokenStream) -> TokenStream {
    macros::telegram::telegram_impl(input)
}

// reddit!
#[proc_macro]
#[decl(fn, name = "reddit", vis = "pub", hash = "ae6f96ee")]
pub fn reddit(input: TokenStream) -> TokenStream {
    macros::reddit::reddit_impl(input)
}

// wikipedia!
#[proc_macro]
#[decl(fn, name = "wikipedia", vis = "pub", hash = "616a7bb3")]
pub fn wikipedia(input: TokenStream) -> TokenStream {
    macros::wikipedia::wikipedia_impl(input)
}

// wikidata!
#[proc_macro]
#[decl(fn, name = "wikidata", vis = "pub", hash = "168c75a6")]
pub fn wikidata(input: TokenStream) -> TokenStream {
    macros::wikidata::wikidata_impl(input)
}

// osm!
#[proc_macro]
#[decl(fn, name = "osm", vis = "pub", hash = "ca96da81")]
pub fn osm(input: TokenStream) -> TokenStream {
    macros::osm::osm_impl(input)
}

// foaf!
#[proc_macro]
#[decl(fn, name = "foaf", vis = "pub", hash = "f6f07caf")]
pub fn foaf(input: TokenStream) -> TokenStream {
    macros::foaf::foaf_impl(input)
}

// owl!
#[proc_macro]
#[decl(fn, name = "owl", vis = "pub", hash = "1ecc44fa")]
pub fn owl(input: TokenStream) -> TokenStream {
    macros::owl::owl_impl(input)
}

// oil!
#[proc_macro]
#[decl(fn, name = "oil", vis = "pub", hash = "a1e2d08a")]
pub fn oil(input: TokenStream) -> TokenStream {
    macros::oil::oil_impl(input)
}

// daml!
#[proc_macro]
#[decl(fn, name = "daml", vis = "pub", hash = "e8455144")]
pub fn daml(input: TokenStream) -> TokenStream {
    macros::daml::daml_impl(input)
}

// cwm!
#[proc_macro]
#[decl(fn, name = "cwm", vis = "pub", hash = "b40e68a5")]
pub fn cwm(input: TokenStream) -> TokenStream {
    macros::cwm::cwm_impl(input)
}

// euler!
#[proc_macro]
#[decl(fn, name = "euler", vis = "pub", hash = "e63b3b8a")]
pub fn euler(input: TokenStream) -> TokenStream {
    macros::euler::euler_impl(input)
}