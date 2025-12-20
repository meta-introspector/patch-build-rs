use std::fs;
use std::path::Path;
use syn::{
    parse_file, Item, ItemFn, ItemStruct, ItemEnum, ItemTrait, ItemImpl,
    ItemMod, ItemConst, ItemStatic, ItemType, ItemMacro, Visibility,
    Attribute, Generics, FnArg, ReturnType, Type, Fields,
};
use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct DeclMetadata {
    pub node_type: NodeType,
    pub name: String,
    pub visibility: VisibilityKind,
    pub line: usize,
    pub generics: Option<String>,
    pub attributes: Vec<String>,
    pub signature: Option<String>,
    pub fields: Vec<FieldInfo>,
    pub variants: Vec<String>,
    pub methods: Vec<String>,
    pub doc: Option<String>,
    pub semantic_hash: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Module,
    Const,
    Static,
    TypeAlias,
    Macro,
    Use,
}

impl NodeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeType::Function => "fn",
            NodeType::Struct => "struct",
            NodeType::Enum => "enum",
            NodeType::Trait => "trait",
            NodeType::Impl => "impl",
            NodeType::Module => "mod",
            NodeType::Const => "const",
            NodeType::Static => "static",
            NodeType::TypeAlias => "type",
            NodeType::Macro => "macro",
            NodeType::Use => "use",
        }
    }
    
    pub fn emoji(&self) -> &'static str {
        match self {
            NodeType::Function => "🔧",
            NodeType::Struct => "📦",
            NodeType::Enum => "🔀",
            NodeType::Trait => "🎭",
            NodeType::Impl => "⚙️",
            NodeType::Module => "📁",
            NodeType::Const => "🔒",
            NodeType::Static => "📌",
            NodeType::TypeAlias => "🏷️",
            NodeType::Macro => "✨",
            NodeType::Use => "📎",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityKind {
    Public,
    Crate,
    Super,
    Private,
    Restricted,
}

impl VisibilityKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            VisibilityKind::Public => "pub",
            VisibilityKind::Crate => "pub(crate)",
            VisibilityKind::Super => "pub(super)",
            VisibilityKind::Private => "private",
            VisibilityKind::Restricted => "pub(in ...)",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: Option<String>,
    pub ty: String,
    pub visibility: VisibilityKind,
}

#[derive(Debug, Clone)]
pub struct WrapAction {
    pub line_start: usize,
    pub line_end: usize,
    pub metadata: DeclMetadata,
    pub original: String,
    pub wrapped: String,
}

fn extract_visibility(vis: &Visibility) -> VisibilityKind {
    match vis {
        Visibility::Public(_) => VisibilityKind::Public,
        Visibility::Restricted(r) => {
            let path = r.path.to_token_stream().to_string();
            if path == "crate" {
                VisibilityKind::Crate
            } else if path == "super" {
                VisibilityKind::Super
            } else {
                VisibilityKind::Restricted
            }
        }
        Visibility::Inherited => VisibilityKind::Private,
    }
}

fn extract_doc_comment(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = &nv.value {
                    return Some(s.value().trim().to_string());
                }
            }
        }
    }
    None
}

fn extract_attributes(attrs: &[Attribute]) -> Vec<String> {
    attrs.iter()
        .filter(|a| !a.path().is_ident("doc"))
        .map(|a| a.to_token_stream().to_string())
        .collect()
}

fn generics_to_string(generics: &Generics) -> Option<String> {
    if generics.params.is_empty() {
        None
    } else {
        Some(generics.to_token_stream().to_string())
    }
}

fn compute_semantic_hash(content: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn extract_fn_metadata(item: &ItemFn, line: usize) -> DeclMetadata {
    let params: Vec<String> = item.sig.inputs.iter().map(|arg| {
        match arg {
            FnArg::Receiver(r) => {
                if r.mutability.is_some() { "&mut self".to_string() }
                else if r.reference.is_some() { "&self".to_string() }
                else { "self".to_string() }
            }
            FnArg::Typed(t) => {
                format!("{}: {}", 
                    t.pat.to_token_stream(),
                    t.ty.to_token_stream())
            }
        }
    }).collect();
    
    let return_type = match &item.sig.output {
        ReturnType::Default => "()".to_string(),
        ReturnType::Type(_, ty) => ty.to_token_stream().to_string(),
    };
    
    let signature = format!("fn {}({}) -> {}", 
        item.sig.ident, params.join(", "), return_type);
    
    DeclMetadata {
        node_type: NodeType::Function,
        name: item.sig.ident.to_string(),
        visibility: extract_visibility(&item.vis),
        line,
        generics: generics_to_string(&item.sig.generics),
        attributes: extract_attributes(&item.attrs),
        signature: Some(signature),
        fields: vec![],
        variants: vec![],
        methods: vec![],
        doc: extract_doc_comment(&item.attrs),
        semantic_hash: compute_semantic_hash(&item.to_token_stream().to_string()),
    }
}

fn extract_struct_metadata(item: &ItemStruct, line: usize) -> DeclMetadata {
    let fields: Vec<FieldInfo> = match &item.fields {
        Fields::Named(named) => {
            named.named.iter().map(|f| FieldInfo {
                name: f.ident.as_ref().map(|i| i.to_string()),
                ty: f.ty.to_token_stream().to_string(),
                visibility: extract_visibility(&f.vis),
            }).collect()
        }
        Fields::Unnamed(unnamed) => {
            unnamed.unnamed.iter().enumerate().map(|(i, f)| FieldInfo {
                name: Some(format!("{}", i)),
                ty: f.ty.to_token_stream().to_string(),
                visibility: extract_visibility(&f.vis),
            }).collect()
        }
        Fields::Unit => vec![],
    };
    
    DeclMetadata {
        node_type: NodeType::Struct,
        name: item.ident.to_string(),
        visibility: extract_visibility(&item.vis),
        line,
        generics: generics_to_string(&item.generics),
        attributes: extract_attributes(&item.attrs),
        signature: None,
        fields,
        variants: vec![],
        methods: vec![],
        doc: extract_doc_comment(&item.attrs),
        semantic_hash: compute_semantic_hash(&item.to_token_stream().to_string()),
    }
}

fn extract_enum_metadata(item: &ItemEnum, line: usize) -> DeclMetadata {
    let variants: Vec<String> = item.variants.iter()
        .map(|v| v.ident.to_string())
        .collect();
    
    DeclMetadata {
        node_type: NodeType::Enum,
        name: item.ident.to_string(),
        visibility: extract_visibility(&item.vis),
        line,
        generics: generics_to_string(&item.generics),
        attributes: extract_attributes(&item.attrs),
        signature: None,
        fields: vec![],
        variants,
        methods: vec![],
        doc: extract_doc_comment(&item.attrs),
        semantic_hash: compute_semantic_hash(&item.to_token_stream().to_string()),
    }
}

fn extract_trait_metadata(item: &ItemTrait, line: usize) -> DeclMetadata {
    let methods: Vec<String> = item.items.iter()
        .filter_map(|i| {
            if let syn::TraitItem::Fn(m) = i {
                Some(m.sig.ident.to_string())
            } else {
                None
            }
        })
        .collect();
    
    DeclMetadata {
        node_type: NodeType::Trait,
        name: item.ident.to_string(),
        visibility: extract_visibility(&item.vis),
        line,
        generics: generics_to_string(&item.generics),
        attributes: extract_attributes(&item.attrs),
        signature: None,
        fields: vec![],
        variants: vec![],
        methods,
        doc: extract_doc_comment(&item.attrs),
        semantic_hash: compute_semantic_hash(&item.to_token_stream().to_string()),
    }
}

fn extract_impl_metadata(item: &ItemImpl, line: usize) -> DeclMetadata {
    let self_ty = item.self_ty.to_token_stream().to_string();
    let trait_name = item.trait_.as_ref()
        .map(|(_, path, _)| path.to_token_stream().to_string());
    
    let name = if let Some(trait_name) = trait_name {
        format!("{} for {}", trait_name, self_ty)
    } else {
        self_ty
    };
    
    let methods: Vec<String> = item.items.iter()
        .filter_map(|i| {
            if let syn::ImplItem::Fn(m) = i {
                Some(m.sig.ident.to_string())
            } else {
                None
            }
        })
        .collect();
    
    DeclMetadata {
        node_type: NodeType::Impl,
        name,
        visibility: VisibilityKind::Private, // impls don't have visibility
        line,
        generics: generics_to_string(&item.generics),
        attributes: extract_attributes(&item.attrs),
        signature: None,
        fields: vec![],
        variants: vec![],
        methods,
        doc: None,
        semantic_hash: compute_semantic_hash(&item.to_token_stream().to_string()),
    }
}

pub fn extract_declarations(content: &str) -> Vec<DeclMetadata> {
    let ast = match parse_file(content) {
        Ok(ast) => ast,
        Err(_) => return vec![],
    };
    
    let mut decls = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    for item in &ast.items {
        let decl = match item {
            Item::Fn(f) => {
                let line = find_item_line(&lines, &f.sig.ident.to_string(), "fn ");
                Some(extract_fn_metadata(f, line))
            }
            Item::Struct(s) => {
                let line = find_item_line(&lines, &s.ident.to_string(), "struct ");
                Some(extract_struct_metadata(s, line))
            }
            Item::Enum(e) => {
                let line = find_item_line(&lines, &e.ident.to_string(), "enum ");
                Some(extract_enum_metadata(e, line))
            }
            Item::Trait(t) => {
                let line = find_item_line(&lines, &t.ident.to_string(), "trait ");
                Some(extract_trait_metadata(t, line))
            }
            Item::Impl(i) => {
                let line = find_item_line(&lines, "impl", "impl ");
                Some(extract_impl_metadata(i, line))
            }
            Item::Mod(m) => {
                let line = find_item_line(&lines, &m.ident.to_string(), "mod ");
                Some(DeclMetadata {
                    node_type: NodeType::Module,
                    name: m.ident.to_string(),
                    visibility: extract_visibility(&m.vis),
                    line,
                    generics: None,
                    attributes: extract_attributes(&m.attrs),
                    signature: None,
                    fields: vec![],
                    variants: vec![],
                    methods: vec![],
                    doc: extract_doc_comment(&m.attrs),
                    semantic_hash: compute_semantic_hash(&m.to_token_stream().to_string()),
                })
            }
            Item::Const(c) => {
                let line = find_item_line(&lines, &c.ident.to_string(), "const ");
                Some(DeclMetadata {
                    node_type: NodeType::Const,
                    name: c.ident.to_string(),
                    visibility: extract_visibility(&c.vis),
                    line,
                    generics: None,
                    attributes: extract_attributes(&c.attrs),
                    signature: Some(format!("const {}: {}", c.ident, c.ty.to_token_stream())),
                    fields: vec![],
                    variants: vec![],
                    methods: vec![],
                    doc: extract_doc_comment(&c.attrs),
                    semantic_hash: compute_semantic_hash(&c.to_token_stream().to_string()),
                })
            }
            Item::Type(t) => {
                let line = find_item_line(&lines, &t.ident.to_string(), "type ");
                Some(DeclMetadata {
                    node_type: NodeType::TypeAlias,
                    name: t.ident.to_string(),
                    visibility: extract_visibility(&t.vis),
                    line,
                    generics: generics_to_string(&t.generics),
                    attributes: extract_attributes(&t.attrs),
                    signature: Some(format!("type {} = {}", t.ident, t.ty.to_token_stream())),
                    fields: vec![],
                    variants: vec![],
                    methods: vec![],
                    doc: extract_doc_comment(&t.attrs),
                    semantic_hash: compute_semantic_hash(&t.to_token_stream().to_string()),
                })
            }
            _ => None,
        };
        
        if let Some(d) = decl {
            decls.push(d);
        }
    }
    
    decls
}

fn find_item_line(lines: &[&str], name: &str, keyword: &str) -> usize {
    for (i, line) in lines.iter().enumerate() {
        if line.contains(keyword) && line.contains(name) {
            return i + 1;
        }
    }
    1
}

pub fn generate_decl_wrapper(metadata: &DeclMetadata) -> String {
    let fields_json = if metadata.fields.is_empty() {
        "[]".to_string()
    } else {
        let fields: Vec<String> = metadata.fields.iter()
            .map(|f| format!("{{\"name\":\"{}\",\"ty\":\"{}\"}}", 
                f.name.as_deref().unwrap_or("_"), 
                f.ty.replace("\"", "\\\"")))
            .collect();
        format!("[{}]", fields.join(","))
    };
    
    let variants_json = if metadata.variants.is_empty() {
        "[]".to_string()
    } else {
        format!("[{}]", metadata.variants.iter()
            .map(|v| format!("\"{}\"", v))
            .collect::<Vec<_>>()
            .join(","))
    };
    
    let methods_json = if metadata.methods.is_empty() {
        "[]".to_string()
    } else {
        format!("[{}]", metadata.methods.iter()
            .map(|m| format!("\"{}\"", m))
            .collect::<Vec<_>>()
            .join(","))
    };
    
    format!(
        r#"decl! {{
    node_type: "{}",
    name: "{}",
    visibility: "{}",
    line: {},
    generics: {:?},
    hash: "{}",
    fields: {},
    variants: {},
    methods: {},
    doc: {:?},
}}"#,
        metadata.node_type.as_str(),
        metadata.name,
        metadata.visibility.as_str(),
        metadata.line,
        metadata.generics,
        metadata.semantic_hash,
        fields_json,
        variants_json,
        methods_json,
        metadata.doc,
    )
}

pub fn generate_decl_attribute(metadata: &DeclMetadata) -> String {
    format!(
        "#[decl({}, name = \"{}\", vis = \"{}\", hash = \"{}\")]",
        metadata.node_type.as_str(),
        metadata.name,
        metadata.visibility.as_str(),
        &metadata.semantic_hash[..8],
    )
}

pub fn generate_inventory_registration(metadata: &DeclMetadata) -> String {
    format!(
        r#"inventory::submit! {{
    DeclInfo {{
        node_type: NodeType::{:?},
        name: "{}",
        module: module_path!(),
        file: file!(),
        line: {},
        hash: "{}",
    }}
}}"#,
        metadata.node_type,
        metadata.name,
        metadata.line,
        &metadata.semantic_hash[..16],
    )
}

pub fn wrap_public_declarations(content: &str) -> Vec<WrapAction> {
    let decls = extract_declarations(content);
    let lines: Vec<&str> = content.lines().collect();
    
    let mut actions = Vec::new();
    
    for decl in decls {
        // Only wrap public declarations and skip module declarations for now
        if decl.visibility != VisibilityKind::Public || decl.node_type == NodeType::Module {
            continue;
        }
        
        let line_idx = decl.line.saturating_sub(1);
        if line_idx >= lines.len() {
            continue;
        }
        
        let original = lines[line_idx].to_string();
        
        // Skip if already wrapped
        if original.contains("#[decl(") || original.contains("decl!") {
            continue;
        }
        
        let attr = generate_decl_attribute(&decl);
        let wrapped = format!("{}\n{}", attr, original);
        
        actions.push(WrapAction {
            line_start: decl.line,
            line_end: decl.line,
            metadata: decl,
            original,
            wrapped,
        });
    }
    
    actions
}

pub fn apply_decl_wrappers(path: &Path) -> Result<usize, std::io::Error> {
    let content = fs::read_to_string(path)?;
    let actions = wrap_public_declarations(&content);
    
    if actions.is_empty() {
        return Ok(0);
    }
    
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut skip_next = std::collections::HashSet::new();
    
    for action in &actions {
        skip_next.insert(action.line_start);
    }
    
    let action_map: std::collections::HashMap<usize, &WrapAction> = 
        actions.iter().map(|a| (a.line_start, a)).collect();
    
    for (i, line) in lines.iter().enumerate() {
        let line_num = i + 1;
        if let Some(action) = action_map.get(&line_num) {
            result.push(action.wrapped.clone());
        } else {
            result.push(line.to_string());
        }
    }
    
    fs::write(path, result.join("\n"))?;
    Ok(actions.len())
}

pub fn preview_decl_wrappers(path: &Path) -> String {
    let content = fs::read_to_string(path).unwrap_or_default();
    let actions = wrap_public_declarations(&content);
    
    let mut output = String::new();
    output.push_str(&format!("📁 File: {}\n", path.display()));
    output.push_str(&format!("🔍 Found {} public declarations to wrap\n\n", actions.len()));
    
    for action in &actions {
        output.push_str(&format!(
            "┌─ Line {}: {} {} `{}`\n",
            action.line_start,
            action.metadata.node_type.emoji(),
            action.metadata.node_type.as_str(),
            action.metadata.name
        ));
        output.push_str(&format!("│ Hash: {}\n", &action.metadata.semantic_hash[..16]));
        if !action.metadata.fields.is_empty() {
            output.push_str(&format!("│ Fields: {}\n", action.metadata.fields.len()));
        }
        if !action.metadata.methods.is_empty() {
            output.push_str(&format!("│ Methods: {:?}\n", action.metadata.methods));
        }
        output.push_str(&format!("│ Before: {}\n", action.original.trim()));
        output.push_str(&format!("│ After:\n"));
        for line in action.wrapped.lines() {
            output.push_str(&format!("│   {}\n", line));
        }
        output.push_str("└────────────────────────────────────────\n\n");
    }
    
    output
}

pub fn generate_declarations_json(path: &Path) -> String {
    let content = fs::read_to_string(path).unwrap_or_default();
    let decls = extract_declarations(&content);
    
    let items: Vec<String> = decls.iter().map(|d| {
        format!(
            r#"  {{
    "type": "{}",
    "name": "{}",
    "visibility": "{}",
    "line": {},
    "hash": "{}",
    "fields": {},
    "variants": {},
    "methods": {}
  }}"#,
            d.node_type.as_str(),
            d.name,
            d.visibility.as_str(),
            d.line,
            d.semantic_hash,
            d.fields.len(),
            d.variants.len(),
            d.methods.len(),
        )
    }).collect();
    
    format!("[\n{}\n]", items.join(",\n"))
}

#[macro_export]
macro_rules! wrap_decls {
    ($path:expr) => {{
        let path = std::path::Path::new($path);
        let preview = $crate::decl_wrapper::preview_decl_wrappers(path);
        eprintln!("{}", preview);
    }};
    
    ($path:expr, apply) => {{
        let path = std::path::Path::new($path);
        match $crate::decl_wrapper::apply_decl_wrappers(path) {
            Ok(count) => eprintln!("✅ Wrapped {} declarations in {}", count, $path),
            Err(e) => eprintln!("❌ Error: {}", e),
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_function() {
        let code = r#"
pub fn hello_world(name: &str) -> String {
    format!("Hello, {}!", name)
}
"#;
        let decls = extract_declarations(code);
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].node_type, NodeType::Function);
        assert_eq!(decls[0].name, "hello_world");
        assert_eq!(decls[0].visibility, VisibilityKind::Public);
    }

    #[test]
    fn test_extract_struct() {
        let code = r#"
pub struct User {
    pub name: String,
    pub age: u32,
}
"#;
        let decls = extract_declarations(code);
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].node_type, NodeType::Struct);
        assert_eq!(decls[0].fields.len(), 2);
    }

    #[test]
    fn test_extract_enum() {
        let code = r#"
pub enum Status {
    Active,
    Inactive,
    Pending,
}
"#;
        let decls = extract_declarations(code);
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].node_type, NodeType::Enum);
        assert_eq!(decls[0].variants.len(), 3);
    }

    #[test]
    fn test_generate_attribute() {
        let metadata = DeclMetadata {
            node_type: NodeType::Function,
            name: "test_fn".to_string(),
            visibility: VisibilityKind::Public,
            line: 10,
            generics: None,
            attributes: vec![],
            signature: Some("fn test_fn() -> ()".to_string()),
            fields: vec![],
            variants: vec![],
            methods: vec![],
            doc: None,
            semantic_hash: "abc123def456".to_string(),
        };
        
        let attr = generate_decl_attribute(&metadata);
        assert!(attr.contains("#[decl(fn"));
        assert!(attr.contains("test_fn"));
    }
}
