extern crate askama;
use askama::Template;

#[derive(Template)]
#[template(path = "lib.rs.tmpl", escape = "none")]
pub struct LibRs<'a> {
    pub file_basename: &'a str,

    pub ftypes: &'a [RustFnTemplate<'a>],
    pub ftypes_imports: &'a [RustImportFnTemplate<'a>],
}

pub struct RustFnTemplate<'a> {
    pub func_name: &'a str,
    pub args_decl: &'a str,
    pub args_let_vec: &'a str,
    pub str_args_converter: &'a str,
    pub rettype_decl: &'a str,
    pub handle_retval: &'a str,
    pub exported_helper_var: &'a str,
}

pub struct RustImportFnTemplate<'a> {
    pub func_name: &'a str,
    pub args_decl: &'a str,
    pub imported_body: &'a str,
    pub rettype_decl: &'a str,
    pub import_helper_var: &'a str,
}
