use patch_build_rs_macros::mkbuildrs;
use crate::triple::GrastTriple;
use std::collections::HashMap;

mkbuildrs! {
    module_name: "grast_database";
    dependencies: ["std::collections::HashMap", "std::fs", "std::io", "std::path::Path"];
    description: "Database operations for grast triples with indexing and file I/O";
}

#[decl(struct, name = "GrastDb", vis = "pub", hash = "508c1233")]
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

    pub fn add_triple(&mut self, subject: &str, predicate: &str, object: &str) {
        let triple = GrastTriple {
            subject: subject.to_string(),
            predicate: predicate.to_string(),
            object: object.to_string(),
        };
        
        let index = self.triples.len();
        self.triples.push(triple);
        
        // Index by subject
        self.index.entry(subject.to_string())
            .or_insert_with(Vec::new)
            .push(index);
    }

    pub fn to_turtle(&self) -> String {
        self.triples.iter()
            .map(|t| t.to_turtle())
            .collect::<Vec<_>>()
            .join("\n")
    }
}