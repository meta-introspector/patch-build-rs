// grast: Greppable AST - Line-oriented AST representation
// Inspired by gron's approach to making JSON greppable

use std::collections::HashMap;
use std::fs;
use std::io::{self};
use std::path::{Path};
use syn::{self, Expr, Item, Stmt, Visibility};
use quote::quote; // Import quote macro for token stream conversion


/// Core grast representation: flat triple format
/// Format: subject predicate object
/// Example: node_0 :type :FunctionDecl
///          node_0 :name "main"
///          node_0 :child node_1

#[derive(Debug, Clone)]
pub struct GrastTriple {
    pub subject: String,
    pub predicate: String,
    pub object: String,
}

impl GrastTriple {
    pub fn to_turtle(&self) -> String {
        format!("{} {} {} .", self.subject, self.predicate, self.object)
    }
    
    pub fn from_turtle(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.trim_end_matches('.').split_whitespace().collect();
        if parts.len() >= 3 {
            Some(GrastTriple {
                subject: parts[0].to_string(),
                predicate: parts[1].to_string(),
                object: parts[2..].join(" "),
            })
        } else {
            None
        }
    }
}

pub struct GrastDb {
    pub triples: Vec<GrastTriple>,
    pub index: HashMap<String, Vec<usize>>,
}

impl GrastDb {
    pub fn new() -> Self {
        GrastDb {
            triples: Vec::new(),
            index: HashMap::new(),
        }
    }
    
    /// Flatten AST to line-oriented format
    pub fn flatten(&mut self, ast: &syn::File) {
        let mut node_counter = 0;
        self.flatten_file(ast, &mut node_counter);
    }
    
    fn flatten_file(&mut self, file: &syn::File, counter: &mut usize) {
        let file_id = format!("node_{{}}", counter);
        *counter += 1;
        
        self.add_triple(&file_id, ":type", ":File");
        
        for (idx, item) in file.items.iter().enumerate() {
            let item_id = self.flatten_item(item, counter);
            self.add_triple(&file_id, &format!(":item_{{}}", idx), &item_id);
        }
    }
    
    fn flatten_item(&mut self, item: &syn::Item, counter: &mut usize) -> String {
        let item_id = format!("node_{{}}", counter);
        *counter += 1;
        
        match item {
            Item::Fn(func) => { // Use Item::Fn directly
                self.add_triple(&item_id, ":type", ":FunctionDecl");
                self.add_triple(&item_id, ":name", &format!("\"{}\"", func.sig.ident));
                self.add_triple(&item_id, ":visibility", &format!("{:?}", func.vis));
                
                // Add parameters
                for (idx, input) in func.sig.inputs.iter().enumerate() {
                    let param_id = format!("node_{{}}", counter);
                    *counter += 1;
                    self.add_triple(&param_id, ":type", ":Parameter");
                    self.add_triple(&item_id, &format!(":param_{{}}", idx), &param_id);
                    // Add parameter details (name, type)
                    if let syn::FnArg::Typed(pat_type) = input {
                        if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                            self.add_triple(&param_id, ":name", &format!("\"{}\"", pat_ident.ident));
                        }
                        self.add_triple(&param_id, ":type_name", &format!("\"{}\"", quote!{#pat_type.ty}));
                    }
                }
                // Add return type
                if let syn::ReturnType::Type(_, ty) = &func.sig.output {
                    let return_type_id = format!("node_{{}}", counter);
                    *counter += 1;
                    self.add_triple(&return_type_id, ":type", ":ReturnType");
                    self.add_triple(&item_id, ":return_type", &return_type_id);
                    self.add_triple(&return_type_id, ":type_name", &format!("\"{}\"", quote!{#ty}));
                }

                // Flatten function body statements
                for (idx, stmt) in func.block.stmts.iter().enumerate() {
                    let stmt_id = self.flatten_stmt(stmt, counter);
                    self.add_triple(&item_id, &format!(":stmt_{{}}", idx), &stmt_id);
                }
            }
            Item::Struct(s) => { // Use Item::Struct directly
                self.add_triple(&item_id, ":type", ":StructDecl");
                self.add_triple(&item_id, ":name", &format!("\"{}\"", s.ident));
                self.add_triple(&item_id, ":visibility", &format!("{:?}", s.vis));
                
                // Add fields
                if let syn::Fields::Named(fields) = &s.fields {
                    for (idx, field) in fields.named.iter().enumerate() {
                        let field_id = format!("node_{{}}", counter);
                        *counter += 1;
                        self.add_triple(&field_id, ":type", ":Field");
                        self.add_triple(&item_id, &format!(":field_{{}}", idx), &field_id);
                        if let Some(ident) = &field.ident {
                            self.add_triple(&field_id, ":name", &format!("\"{}\"", ident));
                        }
                        self.add_triple(&field_id, ":type_name", &format!("\"{}\"", quote!{#field.ty}));
                    }
                }
            }
            Item::Enum(e) => { // Use Item::Enum directly
                self.add_triple(&item_id, ":type", ":EnumDecl");
                self.add_triple(&item_id, ":name", &format!("\"{}\"", e.ident));
                self.add_triple(&item_id, ":visibility", &format!("{:?}", e.vis));
                
                // Add variants
                for (idx, variant) in e.variants.iter().enumerate() {
                    let variant_id = format!("node_{{}}", counter);
                    *counter += 1;
                    self.add_triple(&variant_id, ":type", ":Variant");
                    self.add_triple(&item_id, &format!(":variant_{{}}", idx), &variant_id);
                    self.add_triple(&variant_id, ":name", &format!("\"{}\"", variant.ident));
                }
            }
            Item::Trait(t) => { // Use Item::Trait directly
                self.add_triple(&item_id, ":type", ":TraitDecl");
                self.add_triple(&item_id, ":name", &format!("\"{}\"", t.ident));
                self.add_triple(&item_id, ":visibility", &format!("{:?}", t.vis));
            }
            Item::Impl(_imp) => { // Use Item::Impl directly, _imp to suppress warning
                self.add_triple(&item_id, ":type", ":ImplBlock");
            }
            Item::Const(c) => { // Use Item::Const directly
                self.add_triple(&item_id, ":type", ":ConstDecl");
                self.add_triple(&item_id, ":name", &format!("\"{}\"", c.ident));
                self.add_triple(&item_id, ":visibility", &format!("{:?}", c.vis));
                if let Some(expr_id) = self.flatten_expr(&c.expr, counter) {
                    self.add_triple(&item_id, ":value", &expr_id);
                }
            }
            Item::Static(s_item) => { // Use Item::Static directly
                self.add_triple(&item_id, ":type", ":StaticDecl");
                self.add_triple(&item_id, ":name", &format!("\"{}\"", s_item.ident));
                self.add_triple(&item_id, ":visibility", &format!("{:?}", s_item.vis));
                if let Some(expr_id) = self.flatten_expr(&s_item.expr, counter) {
                    self.add_triple(&item_id, ":value", &expr_id);
                }
            }
            Item::Mod(m) => { // Use Item::Mod directly
                self.add_triple(&item_id, ":type", ":ModDecl");
                self.add_triple(&item_id, ":name", &format!("\"{}\"", m.ident));
                self.add_triple(&item_id, ":visibility", &format!("{:?}", m.vis));
            }
            Item::Use(u) => { // Use Item::Use directly
                self.add_triple(&item_id, ":type", ":UseDecl");
                self.add_triple(&item_id, ":path", &format!("\"{}\"", quote!{#u.tree}));
            }
            Item::Macro(m_item) => { // Use Item::Macro directly (for macro definitions)
                self.add_triple(&item_id, ":type", ":MacroDecl");
                self.add_triple(&item_id, ":name", &format!("\"{}\"", quote!{#m_item.mac.path}));
                self.add_triple(&item_id, ":tokens", &format!("\"{}\"", quote!{#m_item.mac.tokens}));
            }
            Item::ExternCrate(ec) => { // Use Item::ExternCrate directly
                self.add_triple(&item_id, ":type", ":ExternCrateDecl");
                self.add_triple(&item_id, ":name", &format!("\"{}\"", ec.ident));
            }
            Item::ForeignMod(_fm) => { // Use Item::ForeignMod directly, _fm to suppress warning
                self.add_triple(&item_id, ":type", ":ForeignModDecl");
            }
            Item::TraitAlias(ta) => { // Use Item::TraitAlias directly
                self.add_triple(&item_id, ":type", ":TraitAliasDecl");
                self.add_triple(&item_id, ":name", &format!("\"{}\"", ta.ident));
            }
            Item::Type(ty_item) => { // Use Item::Type directly
                self.add_triple(&item_id, ":type", ":TypeAliasDecl");
                self.add_triple(&item_id, ":name", &format!("\"{}\"", ty_item.ident));
                self.add_triple(&item_id, ":visibility", &format!("{:?}", ty_item.vis));
            }
            _ => {
                self.add_triple(&item_id, ":type", ":OtherItem");
                self.add_triple(&item_id, ":debug_str", &format!("\"{}\"", quote!{#item}));
            }
        }
        
        item_id
    }

    fn flatten_stmt(&mut self, stmt: &syn::Stmt, counter: &mut usize) -> String {
        let stmt_id = format!("node_{{}}", counter);
        *counter += 1;

        match stmt {
            Stmt::Local(local) => {
                self.add_triple(&stmt_id, ":type", ":LocalStmt");
                if let Some(expr_id) = local.init.as_ref().and_then(|init| self.flatten_expr(&init.expr, counter)) {
                    self.add_triple(&stmt_id, ":init_expr", &expr_id);
                }
            }
            Stmt::Item(item) => {
                let item_child_id = self.flatten_item(item, counter);
                self.add_triple(&stmt_id, ":type", ":ItemStmt");
                self.add_triple(&stmt_id, ":item", &item_child_id);
            }
            Stmt::Expr(expr) => {
                let expr_id = self.flatten_expr(expr, counter).unwrap_or_else(|| {
                    let dummy_id = format!("node_{{}}", counter);
                    *counter += 1;
                    self.add_triple(&dummy_id, ":type", ":UnknownExpr");
                    dummy_id
                });
                self.add_triple(&stmt_id, ":type", ":ExprStmt");
                self.add_triple(&stmt_id, ":expr", &expr_id);
            }
            Stmt::Semi(expr, _semi) => {
                let expr_id = self.flatten_expr(expr, counter).unwrap_or_else(|| {
                    let dummy_id = format!("node_{{}}", counter);
                    *counter += 1;
                    self.add_triple(&dummy_id, ":type", ":UnknownExpr");
                    dummy_id
                });
                self.add_triple(&stmt_id, ":type", ":SemiStmt");
                self.add_triple(&stmt_id, ":expr", &expr_id);
            }
            _ => {
                self.add_triple(&stmt_id, ":type", ":OtherStmt");
                self.add_triple(&stmt_id, ":debug_str", &format!("\"{}\"", quote!{#stmt}));
            }
        }
        stmt_id
    }

    fn flatten_expr(&mut self, expr: &syn::Expr, counter: &mut usize) -> Option<String> {
        let expr_id = format!("node_{{}}", counter);
        *counter += 1;

        match expr {
            Expr::Macro(expr_macro) => {
                self.add_triple(&expr_id, ":type", ":MacroInvocation");
                self.add_triple(&expr_id, ":macro_path", &format!("\"{}\"", quote!{#expr_macro.mac.path}));
                self.add_triple(&expr_id, ":macro_args", &format!("\"{}\"", quote!{#expr_macro.mac.tokens}));
            }
            Expr::Path(expr_path) => {
                self.add_triple(&expr_id, ":type", ":PathExpr");
                self.add_triple(&expr_id, ":path", &format!("\"{}\"", quote!{#expr_path.path}));
            }
            Expr::Lit(expr_lit) => {
                self.add_triple(&expr_id, ":type", ":LiteralExpr");
                self.add_triple(&expr_id, ":value", &format!("\"{}\"", quote!{#expr_lit.lit}));
            }
            Expr::Call(expr_call) => {
                self.add_triple(&expr_id, ":type", ":CallExpr");
                if let Some(func_id) = self.flatten_expr(&expr_call.func, counter) {
                    self.add_triple(&expr_id, ":function", &func_id);
                }
                for (idx, arg) in expr_call.args.iter().enumerate() {
                    if let Some(arg_id) = self.flatten_expr(arg, counter) {
                        self.add_triple(&expr_id, &format!(":arg_{{}}", idx), &arg_id);
                    }
                }
            }
            Expr::MethodCall(expr_method_call) => {
                self.add_triple(&expr_id, ":type", ":MethodCallExpr");
                self.add_triple(&expr_id, ":method_name", &format!("\"{}\"", expr_method_call.method));
                if let Some(receiver_id) = self.flatten_expr(&expr_method_call.receiver, counter) {
                    self.add_triple(&expr_id, ":receiver", &receiver_id);
                }
                for (idx, arg) in expr_method_call.args.iter().enumerate() {
                    if let Some(arg_id) = self.flatten_expr(arg, counter) {
                        self.add_triple(&expr_id, &format!(":arg_{{}}", idx), &arg_id);
                    }
                }
            }
            // Handle other expression types as needed for more comprehensive AST flattening
            _ => {
                self.add_triple(&expr_id, ":type", ":OtherExpr");
                self.add_triple(&expr_id, ":debug_str", &format!("\"{}\"", quote!{#expr}));
            }
        }
        Some(expr_id)
    }

    
    pub fn add_triple(&mut self, subject: &str, predicate: &str, object: &str) {
        let idx = self.triples.len();
        self.triples.push(GrastTriple {
            subject: subject.to_string(),
            predicate: predicate.to_string(),
            object: object.to_string(),
        });
        
        self.index.entry(subject.to_string())
            .or_insert_with(Vec::new)
            .push(idx);
    }
    
    /// Output as greppable text
    pub fn to_turtle(&self) -> String {
        self.triples.iter()
            .map(|t| t.to_turtle())
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    /// Reconstruct from greppable text (grast -u / ungrast)
    pub fn from_turtle(text: &str) -> Self {
        let mut db = GrastDb::new();
        
        for line in text.lines() {
            if let Some(triple) = GrastTriple::from_turtle(line) {
                db.add_triple(&triple.subject, &triple.predicate, &triple.object);
            }
        }
        
        db
    }
    
    /// Export as virtual filesystem structure
    pub fn to_vfs(&self, base_path: &Path) -> io::Result<()> {
        // Create directory structure mirroring AST
        for triple in &self.triples {
            let node_dir = base_path.join(&triple.subject);
            fs::create_dir_all(&node_dir)?;
            
            // Each predicate becomes a file in the node directory
            let pred_file = node_dir.join(format!("{}.txt", triple.predicate.trim_start_matches(':')))); 
            fs::write(pred_file, &triple.object)?;
        }
        
        Ok(())
    }
    
    /// Import from virtual filesystem
    pub fn from_vfs(base_path: &Path) -> io::Result<Self> {
        let mut db = GrastDb::new();
        
        // Walk directory structure
        for entry in fs::read_dir(base_path)? {
            let entry = entry?;
            let node_name = entry.file_name().to_string_lossy().to_string();
            
            if entry.file_type()?.is_dir() {
                // Each subdirectory is a node
                for pred_file in fs::read_dir(entry.path())? {
                    let pred_file = pred_file?;
                    if pred_file.file_type()?.is_file() {
                        let predicate = format!(
                            ":{}",
                            pred_file.file_name().to_string_lossy().trim_end_matches(".txt")
                        );
                        let object = fs::read_to_string(pred_file.path())?;
                        db.add_triple(&node_name, &predicate, object.trim());
                    } // closes if pred_file.file_type()?.is_file()
                } // closes for pred_file in ...
            } // closes if entry.file_type()?.is_dir()
        } // closes for entry in ...
        
        Ok(db)
    } // closes pub fn from_vfs
    
    /// Query support for grep-like operations
    pub fn query(&self, pattern: &str) -> Vec<&GrastTriple> {
        self.triples.iter()
            .filter(|t| {
                t.subject.contains(pattern) ||
                t.predicate.contains(pattern) ||
                t.object.contains(pattern)
            })
            .collect()
    }
} // closes impl GrastDb

/// Macro for embedding shell-like transformations
/// Usage: grast_transform! { "grep ':name' | sed 's/foo/bar/'" }
// Removed #[macro_export] as it's a library now, not directly exported as macro from lib.rs
pub struct GrastTransform {
    operations: Vec<Box<dyn Fn(&str) -> String>>,
}

impl GrastTransform {
    pub fn from_shell(cmd: &str) -> Self {
        // Parse shell-like pipeline syntax
        // This is a simplified version - full implementation would handle:
        // grep, sed, awk, sort, uniq, etc.
        let mut ops: Vec<Box<dyn Fn(&str) -> String>> = Vec::new();
        
        for part in cmd.split('|') {
            let part = part.trim();
            if part.starts_with("grep") {
                let pattern = part.split_whitespace().nth(1).unwrap_or("");
                let p = pattern.to_string();
                ops.push(Box::new(move |input: &str| {
                    input.lines()
                        .filter(|line| line.contains(&p))
                        .collect::<Vec<_>>()
                        .join("\n")
                }));
            } else if part.starts_with("sed") {
                // Simple sed s/pattern/replacement/ support
                // This is a placeholder for a more robust sed implementation
                let args: Vec<&str> = part.split_whitespace().skip(1).collect();
                if let Some(sed_cmd_str) = args.first() {
                    let sed_cmd = sed_cmd_str.to_string();
                    ops.push(Box::new(move |input: &str| {
                        // Very basic "sed" for s/pat/rep/g
                        // This needs actual regex and replacement logic
                        if let Some(s_idx) = sed_cmd.find("s/") {
                            let parts: Vec<&str> = sed_cmd[s_idx + 2..].split('/').collect();
                            if parts.len() >= 2 {
                                let pattern = parts[0];
                                let replacement = parts[1];
                                let mut output_lines = Vec::new();
                                for line in input.lines() {
                                    output_lines.push(line.replace(pattern, replacement));
                                }
                                return output_lines.join("\n");
                            }
                        }
                        input.to_string() // Fallback if sed command is not supported
                    }));
                }
            }
        }
        
        GrastTransform { operations: ops }
    }
    
    pub fn apply(&self, input: &str) -> String {
        let mut result = input.to_string();
        for op in &self.operations {
            result = op(&result);
        }
        result
    }
}
