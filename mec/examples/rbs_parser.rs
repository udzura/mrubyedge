use mec::rbs_parser::*;

fn main() {
    let ret = parse("def hoge: (String) -> Integer");
    let ftype = ret.unwrap().1;
    dbg!(ftype);

    let ret = parse("def foo_bar: (Integer, Integer) -> Integer");
    let ftype = ret.unwrap().1;
    dbg!(ftype);

    let ret = parse("def fooBar: (Integer, Float, Integer) -> void");
    let ftype = ret.unwrap().1;
    dbg!(ftype);

    let ret = parse("def poyo123: () -> void");
    let ftype = ret.unwrap().1;
    dbg!(ftype);
}
