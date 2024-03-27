#![feature(path_file_prefix)]

extern crate rand;

use std::{fs::File, io::Read, process::Command};

use askama::Template;
use rand::distributions::{Alphanumeric, DistString};

use mec::template::{CargoToml, LibRs};

fn sh_do(sharg: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("running: `{}`", sharg);
    let out = Command::new("/bin/sh").args(["-c", sharg]).output()?;
    if out.stdout.len() != 0 {
        println!(
            "stdout:\n{}",
            String::from_utf8_lossy(&out.stdout).to_string().trim()
        );
    }
    if out.stderr.len() != 0 {
        println!(
            "stderr:\n{}",
            String::from_utf8_lossy(&out.stderr).to_string().trim()
        );
    }
    if !out.status.success() {
        println!("{:?}", out.status);
        panic!("failed to execute command");
    }

    Ok(())
}

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

    let mrubyfile = std::fs::canonicalize(last_matched)?;
    let fname = mrubyfile.as_path().file_prefix().unwrap().to_string_lossy();

    let pwd = std::env::current_dir()?;
    std::env::set_current_dir(std::env::var("TMPDIR").unwrap_or("/tmp".to_string()))?;

    let dirname = format!("work-mrubyedge-{}", suffix);
    std::fs::create_dir(&dirname)?;
    std::env::set_current_dir(format!("./work-mrubyedge-{}", &suffix))?;
    std::fs::create_dir("src")?;

    sh_do(&format!("cp {} src/", mrubyfile.to_str().unwrap()))?;
    sh_do(&format!("mrbc --verbose src/{}.rb", &fname.to_string()))?;

    let cargo_toml = CargoToml {
        mrubyedge_version: "0.1.3",
    };
    std::fs::write("Cargo.toml", cargo_toml.render()?)?;

    let import_rbs_fname = format!("{}.import.rbs", fname);
    let import_rbs = mrubyfile.parent().unwrap().join(&import_rbs_fname);
    let mut cont = String::new();

    if import_rbs.exists() {
        eprintln!(
            "detected import.rbs: {}",
            import_rbs.as_path().to_string_lossy()
        );
        let mut f = File::open(import_rbs)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        let (_, parsed) = mec::rbs_parser::parse(&s).unwrap();
        let mut ftypes = vec![];
        for def in parsed.iter() {
            ftypes.push(mec::template::RustFnTemplate {
                func_name: &def.name,
                args_decl: def.args_decl(),
                args_let_vec: def.args_let_vec(),
                rettype_decl: def.rettype_decl(),
            })
        }

        let lib_rs = LibRs {
            file_basename: &fname,
            ftypes: &&ftypes,
        };
        let rendered = lib_rs.render()?;
        cont = rendered;
    } else {
        let ftypes = vec![mec::template::RustFnTemplate {
            func_name: &fnname,
            args_decl: "",
            args_let_vec: "vec![]",
            rettype_decl: "-> ()",
        }];

        let lib_rs = LibRs {
            file_basename: &fname,
            ftypes: &&ftypes,
        };
        let rendered = lib_rs.render()?;
        cont = rendered;
    }
    println!("[debug] will generate main.rs:");
    println!("{}", &cont);
    std::fs::write("src/lib.rs", cont)?;

    sh_do("rustup override set nightly 2>/dev/null")?;
    sh_do("cargo build --target wasm32-wasi --release")?;
    sh_do(&format!(
        "cp ./target/wasm32-wasi/release/mywasm.wasm {}/{}.wasm",
        &pwd.to_str().unwrap(),
        &fname.to_string()
    ))?;
    sh_do(&format!("cd .. && rm -rf work-mrubyedge-{}", &suffix))?;

    std::env::set_current_dir(pwd)?;

    println!("[ok] wasm file is generated: {}.wasm", &fname);

    Ok(())
}
