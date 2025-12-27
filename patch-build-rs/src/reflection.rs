// patch-build-rs/src/reflection.rs

#[decl(struct, name = "DeclInfo", vis = "pub", hash = "4af5c11e")]
pub struct DeclInfo {
    pub name: String,
    pub kind: &'static str,
    pub signature: String,
    pub ast_node: String,
}

inventory::collect!(DeclInfo);

#[decl(fn, name = "all_decls", vis = "pub", hash = "60ab6717")]
pub fn all_decls() -> impl Iterator<Item = &'static DeclInfo> {
    inventory::iter::<DeclInfo>
}