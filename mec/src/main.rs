#![feature(path_file_prefix)]

#[macro_use]
extern crate run_shell;
extern crate rand;

use rand::distributions::{Alphanumeric, DistString};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let suffix = Alphanumeric.sample_string(&mut rng, 32);

    let mut fnname = "".to_string();
    let mut last_matched = "".to_string();
    let mut in_fnname = false;
    for arg in std::env::args() {
        match arg.as_str() {
            "--fnname" | "-f" => in_fnname = true,
            v => {
                if in_fnname {
                    fnname = v.to_string();
                    in_fnname = false;
                } else {
                    last_matched = v.to_string();
                }
            }
        }
    }

    if last_matched.is_empty() {
        panic!("invalid argument. usage: mec --fnname FNNAME FILE.rb")
    }

    if fnname.is_empty() {
        panic!("invalid argument. usage: mec --fnname FNNAME FILE.rb")
    }

    let mrubyfile = std::fs::canonicalize(last_matched)?;
    let fname = mrubyfile.as_path().file_prefix().unwrap().to_string_lossy();

    let pwd = std::env::current_dir()?;
    std::env::set_current_dir(std::env::var("TMPDIR").unwrap_or("/tmp".to_string()))?;

    cmd!(
        "git clone https://github.com/udzura/mrubyedge-template-rs.git work-mrubyedge-{}",
        &suffix
    )
    .run()
    .unwrap();
    std::env::set_current_dir(format!("./work-mrubyedge-{}", &suffix))?;
    cmd!("mkdir tmp").run().unwrap();
    cmd!("cp {} src/", mrubyfile.to_str().unwrap())
        .run()
        .unwrap();
    cmd!("mrbc --verbose src/{}.rb", &fname.to_string())
        .run()
        .unwrap();
    cmd!(&format!(
        "sed -i.bak \"s/@@FILENAME_BASE@@/{}/g\" src/lib.rs.tmpl",
        fname.to_string()
    ))
    .run()
    .unwrap();
    cmd!(&format!(
        "sed -i.bak \"s/@@FUNKNAME@@/{}/g\" src/lib.rs.tmpl",
        &fnname
    ))
    .run()
    .unwrap();
    cmd!("cp -f src/lib.rs.tmpl src/lib.rs").run().unwrap();
    cmd!("rustup override set nightly").run().unwrap();
    cmd!("cargo build --target wasm32-wasi --release")
        .run()
        .unwrap();
    cmd!(&format!(
        "cp -v ./target/wasm32-wasi/release/mywasm.wasm {}/{}.wasm",
        &pwd.to_str().unwrap(),
        &fname.to_string()
    ))
    .run()
    .unwrap();
    cmd!(&format!(
        "sh -c \"cd .. && rm -rf work-mrubyedge-{}\"",
        &suffix
    ))
    .run()
    .unwrap();

    std::env::set_current_dir(pwd)?;
    Ok(())
}
