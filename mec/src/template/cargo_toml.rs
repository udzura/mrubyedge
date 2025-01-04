extern crate askama;
use askama::Template;

#[derive(Template)]
#[template(path = "Cargo.toml.tmpl", escape = "none")]
pub struct CargoToml<'a> {
    pub mrubyedge_version: &'a str,
    pub mrubyedge_feature: &'a str,
    pub strip: &'a str,
}

#[derive(Template)]
#[template(path = "Cargo.toml.debug.tmpl", escape = "none")]
pub struct CargoTomlDebug<'a> {
    pub mruby_edge_crate_path: &'a str,
    pub mrubyedge_feature: &'a str,
}
