#![feature(path_file_prefix)]

extern crate rand;

use std::process::Command;

use rand::distributions::{Alphanumeric, DistString};

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
    println!("{:?}", out.status);

    if !out.status.success() {
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

    if fnname.is_empty() {
        panic!("invalid argument. usage: mec --fnname FNNAME FILE.rb")
    }

    let mrubyfile = std::fs::canonicalize(last_matched)?;
    let fname = mrubyfile.as_path().file_prefix().unwrap().to_string_lossy();

    let pwd = std::env::current_dir()?;
    std::env::set_current_dir(std::env::var("TMPDIR").unwrap_or("/tmp".to_string()))?;

    sh_do(&format!(
        "git clone https://github.com/udzura/mrubyedge-template-rs.git work-mrubyedge-{}",
        &suffix
    ))?;
    std::env::set_current_dir(format!("./work-mrubyedge-{}", &suffix))?;
    sh_do("mkdir tmp")?;
    sh_do(&format!("cp {} src/", mrubyfile.to_str().unwrap()))?;
    sh_do(&format!("mrbc --verbose src/{}.rb", &fname.to_string()))?;
    sh_do(&format!(
        "sed -i.bak \"s/@@FILENAME_BASE@@/{}/g\" src/lib.rs.tmpl",
        fname.to_string()
    ))?;
    sh_do(&format!(
        "sed -i.bak \"s/@@FUNKNAME@@/{}/g\" src/lib.rs.tmpl",
        &fnname
    ))?;
    sh_do("cp -f src/lib.rs.tmpl src/lib.rs")?;
    sh_do("rustup override set nightly")?;
    sh_do("cargo build --target wasm32-wasi --release")?;
    sh_do(&format!(
        "cp -v ./target/wasm32-wasi/release/mywasm.wasm {}/{}.wasm",
        &pwd.to_str().unwrap(),
        &fname.to_string()
    ))?;
    sh_do(&format!("cd .. && rm -rf work-mrubyedge-{}", &suffix))?;

    std::env::set_current_dir(pwd)?;

    println!("[ok] wasm file is generated: {}.wasm", &fname);

    Ok(())
}
