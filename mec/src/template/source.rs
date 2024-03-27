extern crate askama;
use askama::Template;

#[derive(Template)]
#[template(path = "lib.rs.tmpl", escape = "none")]
pub struct LibRs<'a> {
    pub file_basename: &'a str,

    pub ftypes: &'a [RustFnTemplate<'a>],
}

pub struct RustFnTemplate<'a> {
    pub func_name: &'a str,
    pub args_decl: &'a str,
    pub args_let_vec: &'a str,
    pub rettype_decl: &'a str,
}
