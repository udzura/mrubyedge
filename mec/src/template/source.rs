extern crate askama;
use askama::Template;

#[derive(Template)]
#[template(path = "lib.rs.tmpl", escape = "none")]
pub struct LibRs<'a> {
    pub func_name: &'a str,
    pub file_basename: &'a str,
}
