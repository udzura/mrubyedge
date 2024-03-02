use askama::Template;
use mec::rbs_parser::*;
use mec::template::LibRs;

fn main() {
    let def = "
def foo_bar: (Integer) -> Integer
";

    let ret = parse(def).unwrap();
    let ftype = ret.1;
    let ftypes = vec![mec::template::RustFnTemplate {
        func_name: &ftype[0].name,
        args_decl: "a: i32",
        args_let_vec: "vec![std::rc::Rc::new(RObject::RInteger(a as i64))]",
        rettype_decl: "-> i32",
        rettype_convert: "0",
    }];

    let lib_rs = LibRs {
        file_basename: "world",
        ftypes: &ftypes,
    };

    println!("{}", lib_rs.render().unwrap());
}
