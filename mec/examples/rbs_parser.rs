use mec::rbs_parser::*;

fn main() {
    let def = "
def hoge: (String) -> Integer
def foo_bar: (Integer, Integer) -> Integer

def fooBar: (Integer, Float, Integer) -> void

def poyo123: () -> void
";

    let ret = parse(def).unwrap();
    let rest = ret.0;
    let ftype = ret.1;
    dbg!(ftype);
    dbg!(rest);
}
