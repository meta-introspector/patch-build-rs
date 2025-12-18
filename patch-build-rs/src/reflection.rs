// patch-build-rs/src/reflection.rs

pub struct DeclInfo {
    pub name: String,
    pub kind: &'static str,
    pub signature: String,
    pub ast_node: String,
}

inventory::collect!(DeclInfo);

pub fn all_decls() -> impl Iterator<Item = &'static DeclInfo> {
    inventory::iter::<DeclInfo>
}