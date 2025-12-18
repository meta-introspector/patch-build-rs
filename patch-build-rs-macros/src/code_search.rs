use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result as SynResult};
use syn::{Ident, LitBool, LitInt, LitStr, Token};

struct FindCodeArgs {
    pattern: LitStr,
    file_types: Option<Vec<LitStr>>,
    case_sensitive: Option<LitBool>,
    context_lines: Option<LitInt>,
    dir: Option<LitStr>,
}

impl Parse for FindCodeArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut pattern = None;
        let mut file_types = None;
        let mut case_sensitive = None;
        let mut context_lines = None;
        let mut dir = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            if key == "pattern" {
                pattern = Some(input.parse()?);
            } else if key == "file_types" {
                let content;
                syn::bracketed!(content in input);
                let types_parsed: syn::punctuated::Punctuated<LitStr, Token![,]> =
                    content.parse_terminated(LitStr::parse, Token![,])?;
                file_types = Some(types_parsed.into_iter().collect());
            } else if key == "case_sensitive" {
                case_sensitive = Some(input.parse()?);
            } else if key == "context_lines" {
                context_lines = Some(input.parse()?);
            } else if key == "dir" {
                dir = Some(input.parse()?);
            } else {
                return Err(input.error(format!("unexpected argument: {}", key)));
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(FindCodeArgs {
            pattern: pattern.ok_or_else(|| input.error("missing 'pattern' argument"))?,
            file_types,
            case_sensitive,
            context_lines,
            dir,
        })
    }
}

#[proc_macro]
pub fn findcode(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as FindCodeArgs);

    let mut command_parts = vec!["grep -R".to_string()];

    // Case sensitivity
    if !args.case_sensitive.map_or(false, |l| l.value()) {
        command_parts.push("-i".to_string());
    }

    // Context lines
    if let Some(lines) = args.context_lines {
        command_parts.push(format!("-C {}", lines.base10_parse::<u32>().unwrap()));
    }

    // File types (include)
    if let Some(types) = args.file_types {
        for ft in types {
            command_parts.push(format!("--include='*.{}'", ft.value()));
        }
    }

    // Pattern
    command_parts.push(format!("-- '{}'", args.pattern.value()));

    // Directory
    let search_dir = args.dir.map_or(".".to_string(), |d| d.value());
    command_parts.push(search_dir);

    let grep_command = command_parts.join(" ");

    // For now, expand to a println! that shows the grep command.
    // In a more advanced scenario, this might be executed by a helper.
    let output = quote! {
        // You would typically execute this command in a shell, not directly in Rust code.
        // This macro just expands to a print statement that gives you the command to run.
        eprintln!("Generated grep command: {}", #grep_command);
    };

    output.into()
}
