use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[decl(fn, name = "nix_event_impl", vis = "pub", hash = "5eadfd4e")]
pub fn nix_event_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let package = input_str.value();
    
    quote! {
        {
            use std::process::Command;
            
            let nix_info = Command::new("nix")
                .args(&["search", #package, "--json"])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                .unwrap_or_else(|_| format!(r#"{{"packages":{{"{}":"found"}}}}"#, #package));
            
            let memory_item = format!(
                "MemoryItem::NixEvent {{ package: '{}', timestamp: {}, data: '{}' }}",
                #package, 
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                nix_info.chars().take(100).collect::<String>()
            );
            
            println!("cargo:warning=📦 Nix event: {}", #package);
            memory_item
        }
    }.into()
}

#[decl(fn, name = "github_event_impl", vis = "pub", hash = "2eb33304")]
pub fn github_event_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let repo = input_str.value();
    
    quote! {
        {
            use std::process::Command;
            
            let github_data = Command::new("curl")
                .args(&["-s", &format!("https://api.github.com/repos/{}", #repo)])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                .unwrap_or_else(|_| format!(r#"{{"name":"{}","stars":0}}"#, #repo));
            
            let memory_item = format!(
                "MemoryItem::GitHubEvent {{ repo: '{}', timestamp: {}, data: '{}' }}",
                #repo,
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                github_data.chars().take(100).collect::<String>()
            );
            
            println!("cargo:warning=🐙 GitHub event: {}", #repo);
            memory_item
        }
    }.into()
}

#[decl(fn, name = "archive_event_impl", vis = "pub", hash = "98d37693")]
pub fn archive_event_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let url = input_str.value();
    
    quote! {
        {
            let memory_item = format!(
                "MemoryItem::ArchiveEvent {{ url: '{}', timestamp: {}, status: 'archived' }}",
                #url,
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
            );
            
            println!("cargo:warning=📚 Archive event: {}", #url);
            memory_item
        }
    }.into()
}

#[decl(fn, name = "huggingface_event_impl", vis = "pub", hash = "05948575")]
pub fn huggingface_event_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let model = input_str.value();
    
    quote! {
        {
            let memory_item = format!(
                "MemoryItem::HuggingFaceEvent {{ model: '{}', timestamp: {}, type: 'model_access' }}",
                #model,
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
            );
            
            println!("cargo:warning=🤗 HuggingFace event: {}", #model);
            memory_item
        }
    }.into()
}

#[decl(fn, name = "twitter_event_impl", vis = "pub", hash = "9e24288b")]
pub fn twitter_event_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let hashtag = input_str.value();
    
    quote! {
        {
            let memory_item = format!(
                "MemoryItem::TwitterEvent {{ hashtag: '{}', timestamp: {}, sentiment: 'neutral' }}",
                #hashtag,
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
            );
            
            println!("cargo:warning=🐦 Twitter event: {}", #hashtag);
            memory_item
        }
    }.into()
}

#[decl(fn, name = "telegram_event_impl", vis = "pub", hash = "c6e98ffc")]
pub fn telegram_event_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let channel = input_str.value();
    
    quote! {
        {
            let memory_item = format!(
                "MemoryItem::TelegramEvent {{ channel: '{}', timestamp: {}, activity: 'message' }}",
                #channel,
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
            );
            
            println!("cargo:warning=✈️ Telegram event: {}", #channel);
            memory_item
        }
    }.into()
}