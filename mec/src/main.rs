#![feature(path_file_prefix)]

extern crate bpaf;
extern crate rand;

const MRUBY_EDGE_DEFAULT_VERSION: &'static str = "0.1.6";

use std::{fs::File, io::Read, path::PathBuf, process::Command};

use askama::Template;
use bpaf::{any, construct, long, Parser};
use rand::distributions::{Alphanumeric, DistString};

use mec::template::{cargo_toml::CargoTomlDebug, CargoToml, LibRs};

#[derive(Debug, Clone)]
struct ParsedOpt {
    fnname: Option<PathBuf>,
    no_wasi: bool,
    skip_cleanup: bool,
    debug_mruby_edge: bool,
    path: PathBuf,
}

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
    let fnname = long("fnname").argument::<PathBuf>("FNNAME").optional();
    let skip_cleanup = long("skip-cleanup").switch();
    let path = any::<PathBuf, _, _>("MRUBY_FILE", |x| {
        (x.to_str().unwrap() != "--help").then_some(x)
    });
    let no_wasi = long("no-wasi").switch();
    let debug_mruby_edge = long("debug-mruby-edge").switch();
    let opts: ParsedOpt = construct!(ParsedOpt {
        fnname,
        no_wasi,
        skip_cleanup,
        debug_mruby_edge,
        path,
    })
    .to_options()
    .descr("mec - An mruby/edge compilation cli")
    .fallback_to_usage()
    .run();

    let mut rng = rand::thread_rng();
    let suffix = Alphanumeric.sample_string(&mut rng, 32);

    let fnname = opts.fnname;
    let path = opts.path;

    let mrubyfile = std::fs::canonicalize(path)?;
    let fname = mrubyfile.as_path().file_prefix().unwrap().to_string_lossy();

    let pwd = std::env::current_dir()?;
    std::env::set_current_dir(std::env::var("TMPDIR").unwrap_or("/tmp".to_string()))?;

    let dirname = format!("work-mrubyedge-{}", suffix);
    std::fs::create_dir(&dirname)?;
    std::env::set_current_dir(format!("./work-mrubyedge-{}", &suffix))?;
    std::fs::create_dir("src")?;

    sh_do(&format!("cp {} src/", mrubyfile.to_str().unwrap()))?;
    sh_do(&format!("mrbc --verbose src/{}.rb", &fname.to_string()))?;

    if opts.debug_mruby_edge {
        let cargo_toml = CargoTomlDebug {
            mruby_edge_crate_path: "/opt/ghq/github.com/udzura/mrubyedge/mrubyedge",
        };
        std::fs::write("Cargo.toml", cargo_toml.render()?)?;
    } else {
        let cargo_toml = CargoToml {
            mrubyedge_version: MRUBY_EDGE_DEFAULT_VERSION,
        };
        std::fs::write("Cargo.toml", cargo_toml.render()?)?;
    }

    let export_rbs_fname = format!("{}.export.rbs", fname);
    let export_rbs = mrubyfile.parent().unwrap().join(&export_rbs_fname);
    let cont: String;

    let mut ftypes_imports = Vec::new();
    let import_rbs_fname = format!("{}.import.rbs", fname);
    let import_rbs = mrubyfile.parent().unwrap().join(&import_rbs_fname);
    if import_rbs.exists() {
        eprintln!(
            "detected import.rbs: {}",
            import_rbs.as_path().to_string_lossy()
        );
        let mut f = File::open(import_rbs)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        let (_, parsed) = mec::rbs_parser::parse(&s).unwrap();
        for def in parsed.leak().iter() {
            ftypes_imports.push(mec::template::RustImportFnTemplate {
                func_name: &def.name,
                args_decl: def.args_decl(),
                rettype_decl: def.rettype_decl(),
                imoprted_body: def.imoprted_body(),
                import_helper_var: def.import_helper_var(),
            })
        }
    }

    if export_rbs.exists() {
        eprintln!(
            "detected export.rbs: {}",
            export_rbs.as_path().to_string_lossy()
        );
        let mut f = File::open(export_rbs)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        let (_, parsed) = mec::rbs_parser::parse(&s).unwrap();
        let mut ftypes = vec![];
        for def in parsed.leak().iter() {
            ftypes.push(mec::template::RustFnTemplate {
                func_name: &def.name,
                args_decl: def.args_decl(),
                args_let_vec: def.args_let_vec(),
                str_args_converter: def.str_args_converter(),
                rettype_decl: def.rettype_decl(),
                handle_retval: def.handle_retval(),
                exported_helper_var: def.exported_helper_var(),
            })
        }

        let lib_rs = LibRs {
            file_basename: &fname,
            ftypes: &&ftypes,
            ftypes_imports: &ftypes_imports,
        };
        let rendered = lib_rs.render()?;
        cont = rendered;
    } else {
        if fnname.is_none() {
            panic!("--fnname FNNAME should be specified when export.rbs does not exist")
        }
        let fnname = fnname.unwrap();

        let ftypes = vec![mec::template::RustFnTemplate {
            func_name: fnname.to_str().unwrap(),
            args_decl: "",
            args_let_vec: "vec![]",
            str_args_converter: "",
            rettype_decl: "-> ()",
            handle_retval: "()",
            exported_helper_var: "",
        }];

        let lib_rs = LibRs {
            file_basename: &fname,
            ftypes: &&ftypes,
            ftypes_imports: &ftypes_imports,
        };
        let rendered = lib_rs.render()?;
        cont = rendered;
    }
    println!("[debug] will generate main.rs:");
    println!("{}", &cont);
    std::fs::write("src/lib.rs", cont)?;

    let target = if opts.no_wasi {
        "wasm32-unknown-unknown"
    } else {
        "wasm32-wasi"
    };

    sh_do("rustup override set nightly 2>/dev/null")?;
    sh_do(&format!("cargo build --target {} --release", target))?;
    sh_do(&format!(
        "cp ./target/{}/release/mywasm.wasm {}/{}.wasm",
        target,
        &pwd.to_str().unwrap(),
        &fname.to_string()
    ))?;
    if opts.skip_cleanup {
        println!(
            "debug: working directory for compile wasm is remained in {}",
            std::env::current_dir()?.as_os_str().to_str().unwrap()
        );
    } else {
        sh_do(&format!("cd .. && rm -rf work-mrubyedge-{}", &suffix))?;
    }

    std::env::set_current_dir(pwd)?;

    println!("[ok] wasm file is generated: {}.wasm", &fname);

    Ok(())
}
