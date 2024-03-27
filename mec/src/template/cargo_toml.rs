extern crate askama;
use askama::Template;

#[derive(Template)]
#[template(path = "Cargo.toml.tmpl", escape = "none")]
pub struct CargoToml<'a> {
    pub mrubyedge_version: &'a str,
}
