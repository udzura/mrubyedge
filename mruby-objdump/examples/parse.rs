extern crate mruby_objdump;

fn main() -> Result<(), mruby_objdump::error::Error> {
    let bin = include_bytes!("./hi.mrb");
    mruby_objdump::format::load(bin)
}
