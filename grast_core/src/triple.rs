use patch_build_rs_macros::mkbuildrs;

mkbuildrs! {
    module_name: "grast_triple";
    dependencies: ["std::collections::HashMap"];
    description: "Core grast triple representation for RDF-like AST storage";
}

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
