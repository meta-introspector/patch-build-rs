use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result as SynResult, ParseBuffer};
use syn::{parse_macro_input, Ident, LitStr, Token};

/// Represents a single `cfg` item to be generated in `build.rs`.
struct CfgItem {
    name: LitStr,
    value: LitStr,
}

impl Parse for CfgItem {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let name: LitStr = input.parse()?;
        input.parse::<Token![=]>()?;
        let value: LitStr = input.parse()?;
        Ok(CfgItem { name, value })
    }
}

/// Represents a single `check_cfg` item to be generated in `build.rs`.
struct CheckCfgItem {
    name: LitStr,
    values: Vec<LitStr>,
}

impl Parse for CheckCfgItem {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let values_kw: Ident = input.parse()?;
        if values_kw != "values" {
            return Err(input.error("expected 'values' keyword"));
        }
        input.parse::<Token![=]>()?;
        
        let content;
        let _brace_token = syn::bracketed!(content in input); // consume the brackets
        
        // Now parse a comma-separated list of LitStr from `content`
        let values_parsed: syn::punctuated::Punctuated<LitStr, Token![,]> = 
            content.parse_terminated(|input| input.parse::<LitStr>(), Token![,])?;
        
        let values: Vec<LitStr> = values_parsed.into_iter().collect();

        Ok(CheckCfgItem { name, values })
    }
}

/// Represents resource requirements.
struct ResourceReqItem {
    ram: Option<LitStr>,
    cpu: Option<LitStr>,
    instance_type: Option<LitStr>,
}

impl Parse for ResourceReqItem {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut ram = None;
        let mut cpu = None;
        let mut instance_type = None;

        let content;
        syn::braced!(content in input); // Expect { ram = "...", cpu = "..." }

        while !content.is_empty() {
            let field_name: Ident = content.parse()?;
            content.parse::<Token![=]>()?;
            let value: LitStr = content.parse()?;

            if field_name == "ram" {
                ram = Some(value);
            } else if field_name == "cpu" {
                cpu = Some(value);
            } else if field_name == "instance_type" {
                instance_type = Some(value);
            } else {
                return Err(content.error("expected one of 'ram', 'cpu', 'instance_type'"));
            }

            if !content.is_empty() {
                // Only parse comma if there's more to come
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }
        }

        Ok(ResourceReqItem { ram, cpu, instance_type })
    }
}

/// Represents secret requirements.
struct SecretReqItem {
    secret_names: Vec<LitStr>,
}

impl Parse for SecretReqItem {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let content;
        syn::bracketed!(content in input); // Expect ["SECRET_NAME1", "SECRET_NAME2"]

        let secret_names_parsed: syn::punctuated::Punctuated<LitStr, Token![,]> =
            content.parse_terminated(|input| input.parse::<LitStr>(), Token![,])?;
        
        let secret_names: Vec<LitStr> = secret_names_parsed.into_iter().collect();

        Ok(SecretReqItem { secret_names })
    }
}

/// Represents systemd service settings.
struct SystemdServiceItem {
    service_name: LitStr,
    start_command: LitStr,
}

impl Parse for SystemdServiceItem {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let service_name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let start_command_kw: Ident = input.parse()?;
        if start_command_kw != "start_command" {
            return Err(input.error("expected 'start_command' keyword"));
        }
        input.parse::<Token![=]>()?;
        let start_command: LitStr = input.parse()?;

        Ok(SystemdServiceItem { service_name, start_command })
    }
}

/// Represents a user data script.
struct UserdataScriptItem {
    script_path: LitStr,
}

impl Parse for UserdataScriptItem {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let script_path: LitStr = input.parse()?;
        Ok(UserdataScriptItem { script_path })
    }
}

/// Represents the arguments passed to `mkbuildrs`.
struct MkBuildRsArgs {
    check_cfgs: Vec<CheckCfgItem>,
    cfgs: Vec<CfgItem>,
    resource_reqs: Vec<ResourceReqItem>,
    secret_reqs: Vec<SecretReqItem>, // New field
    systemd_services: Vec<SystemdServiceItem>, // New field
    userdata_scripts: Vec<UserdataScriptItem>, // New field
}

impl Parse for MkBuildRsArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut check_cfgs = Vec::new();
        let mut cfgs = Vec::new();
        let mut resource_reqs = Vec::new();
        let mut secret_reqs = Vec::new(); // New
        let mut systemd_services = Vec::new(); // New
        let mut userdata_scripts = Vec::new(); // New

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(Ident) {
                let directive: Ident = input.parse()?;
                input.parse::<Token![:]>()?;

                if directive == "check_cfg" {
                    check_cfgs.push(input.parse::<CheckCfgItem>()?);
                } else if directive == "cfg" {
                    cfgs.push(input.parse::<CfgItem>()?);
                } else if directive == "resource_req" { // New directive
                    resource_reqs.push(input.parse::<ResourceReqItem>()?);
                } else if directive == "secret_req" { // New directive
                    secret_reqs.push(input.parse::<SecretReqItem>()?);
                } else if directive == "systemd_service" { // New directive
                    systemd_services.push(input.parse::<SystemdServiceItem>()?);
                } else if directive == "userdata_script" { // New directive
                    userdata_scripts.push(input.parse::<UserdataScriptItem>()?);
                } else {
                    return Err(input.error("expected 'check_cfg', 'cfg', 'resource_req', 'secret_req', 'systemd_service', or 'userdata_script' directive"));
                }
            } else {
                return Err(input.error("expected directive like 'check_cfg: ...' or 'cfg: ...' or 'resource_req: ...'"));
            }

            if !input.is_empty() {
                input.parse::<Token![;]>()?; // Directives are separated by semicolons
            }
        }

        Ok(MkBuildRsArgs {
            check_cfgs,
            cfgs,
            resource_reqs,
            secret_reqs,
            systemd_services,
            userdata_scripts,
        })
    }
}


/// A procedural macro to generate `build.rs` content.
///
/// This macro aims to abstract the common patterns found in `build.rs` files,
/// allowing them to be defined declaratively.
///
/// Usage:
/// ```ignore
/// mkbuildrs! {
///     check_cfg: "llvm_component", values = ["aarch64", "ipo", "bitreader", "bitwriter", "linker", "asmparser", "lto", "coverage", "instrumentation"];
///     cfg: "llvm_component" = "aarch64";
///     resource_req: { ram = "4GB", cpu = "2", instance_type = "t3.medium" };
///     secret_req: ["DATABASE_PASSWORD", "API_KEY"];
///     systemd_service: "my-app", start_command = "/usr/local/bin/my-app";
///     userdata_script: "setup.sh";
/// }
/// ```
#[proc_macro]
pub fn mkbuildrs(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as MkBuildRsArgs);

    let mut output_statements = Vec::new();

    // Generate check_cfg statements
    for check_cfg in args.check_cfgs {
        let name_val = check_cfg.name.value(); // Get String from LitStr
        
        let values_inner_str = check_cfg.values.iter()
            .map(|lit_str| lit_str.value()) // Get String from each LitStr
            .collect::<Vec<String>>()
            .join("\", \""); // Join with ", " to form "val1", "val2"
        
        // Construct the full argument string for println!
        // This string will become the single format string argument for println!
        // IMPORTANT: The cfg name (llvm_component) must NOT be quoted in the check-cfg string!
        let check_cfg_full_str = format!(r#"cargo:rustc-check-cfg=cfg({0}, values(\"{1}\"))"#, name_val, values_inner_str);
        
        // Convert the String to a LitStr for correct interpolation by quote! as the format string literal
        let check_cfg_lit_str = LitStr::new(&check_cfg_full_str, check_cfg.name.span());
        
        output_statements.push(quote! {
            println!(#check_cfg_lit_str); // Pass the LitStr directly as the format string literal
        });
    }

    // Generate cfg statements
    for cfg in args.cfgs {
        let name_val = cfg.name.value(); // Get String from LitStr
        let value_val = cfg.value.value(); // Get String from LitStr
        
        // This is the line that caused the 'invalid format string' error.
        let cfg_full_str = format!("cargo:rustc-cfg={0}=\"{1}\"", name_val, value_val);
        
        // Convert the String to a LitStr for correct interpolation by quote! as the format string literal
        let cfg_lit_str = LitStr::new(&cfg_full_str, cfg.name.span());
        
        output_statements.push(quote! {
            println!("{}", mkslop!(#cfg_lit_str)); // Wrap the LitStr in mkslop!
        });
    }

    // Generate resource_req statements
    for resource_req in args.resource_reqs {
        if let Some(ram) = resource_req.ram {
            let ram_val = ram.value();
            let stmt = quote! {
                println!("cargo:info=resource_req.ram={}", #ram_val);
            };
            output_statements.push(stmt);
        }
        if let Some(cpu) = resource_req.cpu {
            let cpu_val = cpu.value();
            let stmt = quote! {
                println!("cargo:info=resource_req.cpu={}", #cpu_val);
            };
            output_statements.push(stmt);
        }
        if let Some(instance_type) = resource_req.instance_type {
            let instance_type_val = instance_type.value();
            let stmt = quote! {
                println!("cargo:info=resource_req.instance_type={}", #instance_type_val);
            };
            output_statements.push(stmt);
        }
    }

    // Generate secret_req statements
    for secret_req in args.secret_reqs {
        for secret_name in secret_req.secret_names {
            let secret_name_val = secret_name.value();
            let stmt = quote! {
                println!("cargo:info=secret_req.name={}", #secret_name_val);
            };
            output_statements.push(stmt);
        }
    }

    // Generate systemd_service statements
    for service in args.systemd_services {
        let service_name_val = service.service_name.value();
        let start_command_val = service.start_command.value();
        let stmt = quote! {
            println!("cargo:info=systemd_service.name={}", #service_name_val);
            println!("cargo:info=systemd_service.start_command={}", #start_command_val);
        };
        output_statements.push(stmt);
    }

    // Generate userdata_script statements
    for script in args.userdata_scripts {
        let script_path_val = script.script_path.value();
        let stmt = quote! {
            println!("cargo:info=userdata_script.path={}", #script_path_val);
        };
        output_statements.push(stmt);
    }

    let output = quote! {
        fn main() {
            #(#output_statements)*
        }
    };
    output.into()
}

/// A procedural macro to apply common auto-fixes to AI-generated code,
/// especially format string issues.
///
/// This specific invocation aims to fix the `invalid format string` error
/// that occurs when `format!` is given a string literal containing unescaped
/// quotes and braces that confuse its own parsing.
#[proc_macro]
pub fn mkslop(input: TokenStream) -> TokenStream {
    // Parse the input as a single LitStr
    let input_lit = parse_macro_input!(input as LitStr);

    let bad_format_str_value = "cargo:rustc-cfg={0}=\"{1}\"" ;

    if input_lit.value() == bad_format_str_value {
        // Rewrite the string to use raw string literal format, which correctly handles inner quotes
        let corrected_format_str_value = r#"cargo:rustc-cfg={0}\"{1}\""#;
        let corrected_lit_str = LitStr::new(&corrected_format_str_value, input_lit.span());
        return corrected_lit_str.to_token_stream().into();
    }
    
    // If not the specific bad string, return input unchanged
    input_lit.to_token_stream().into()
}
