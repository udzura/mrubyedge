extern crate bpaf;
extern crate rand;

const MRUBY_EDGE_DEFAULT_VERSION: &'static str = "1.0.0-rc1";

use std::{fs::File, io::Read, path::{Path, PathBuf}, process::Command, str};

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
    verbose: bool,
    path: PathBuf,
}

fn sh_do(sharg: &str, debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("running: `{}`", sharg);
    let out = Command::new("/bin/sh").args(["-c", sharg]).output()?;
    if debug && out.stdout.len() != 0 {
        println!(
            "stdout:\n{}",
            String::from_utf8_lossy(&out.stdout).to_string().trim()
        );
    }
    if debug && out.stderr.len() != 0 {
        println!(
            "stderr:\n{}",
            String::from_utf8_lossy(&out.stderr).to_string().trim()
        );
    }
    if debug && !out.status.success() {
        println!("{:?}", out.status);
        panic!("failed to execute command");
    }

    Ok(())
}

fn file_prefix_of(file: &Path) -> Option<String> {
    file.file_name()?
        .to_str()?
        .split('.')
        .next()
        .map(|s| s.to_string())
}

fn debug_println(debug: bool, msg: &str) {
    if debug {
        eprintln!("{}", msg);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fnname = long("fnname").argument::<PathBuf>("FNNAME").optional();
    let skip_cleanup = long("skip-cleanup").switch();
    let path = any::<PathBuf, _, _>("MRUBY_FILE", |x| {
        (x.to_str().unwrap() != "--help").then_some(x)
    });
    let no_wasi = long("no-wasi").switch();
    let debug_mruby_edge = long("debug-mruby-edge").switch();
    let verbose = long("verbose").switch();
    let opts: ParsedOpt = construct!(ParsedOpt {
        fnname,
        no_wasi,
        skip_cleanup,
        debug_mruby_edge,
        verbose,
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
    let fname = file_prefix_of(mrubyfile.as_path()).unwrap();

    let pwd = std::env::current_dir()?;
    std::env::set_current_dir(std::env::var("TMPDIR").unwrap_or("/tmp".to_string()))?;

    let dirname = format!("work-mrubyedge-{}", suffix);
    std::fs::create_dir(&dirname)?;
    std::env::set_current_dir(format!("./work-mrubyedge-{}", &suffix))?;
    std::fs::create_dir("src")?;

    sh_do(&format!("cp {} src/", mrubyfile.to_str().unwrap()), opts.verbose)?;
    if opts.verbose {
        sh_do(&format!("mrbc --verbose src/{}.rb", &fname.to_string()), opts.verbose)?;
    } else {
        sh_do(&format!("mrbc src/{}.rb", &fname.to_string()), opts.verbose)?;
    }

    let feature = if opts.no_wasi { "no-wasi" } else { "default" };

    if opts.debug_mruby_edge {
        let cargo_toml = CargoTomlDebug {
            mruby_edge_crate_path: "/Users/udzura/ghq/github.com/udzura/mrubyedge/mrubyedge",
            mrubyedge_feature: feature,
        };
        std::fs::write("Cargo.toml", cargo_toml.render()?)?;
    } else {
        let cargo_toml = CargoToml {
            mrubyedge_version: MRUBY_EDGE_DEFAULT_VERSION,
            mrubyedge_feature: feature,
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
        debug_println(
            opts.verbose,
            &format!("detected import.rbs: {}", import_rbs.as_path().to_string_lossy()),
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
                imported_body: def.imported_body(),
                import_helper_var: def.import_helper_var(),
            })
        }
    }

    if export_rbs.exists() {
        debug_println(opts.verbose, &format!(
            "detected export.rbs: {}",
            export_rbs.as_path().to_string_lossy()
        ));
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
    debug_println(opts.verbose, "[debug] will generate main.rs:");
    debug_println(opts.verbose, &format!("{}", &cont));
    std::fs::write("src/lib.rs", cont)?;

    let target = if opts.no_wasi {
        "wasm32-unknown-unknown"
    } else {
        "wasm32-wasip1"
    };

    sh_do(&format!("cargo build --target {} --release", target), opts.verbose)?;
    sh_do(&format!(
        "cp ./target/{}/release/mywasm.wasm {}/{}.wasm",
        target,
        &pwd.to_str().unwrap(),
        &fname.to_string()
    ), opts.verbose)?;
    if opts.skip_cleanup {
        println!(
            "debug: working directory for compile wasm is remained in {}",
            std::env::current_dir()?.as_os_str().to_str().unwrap()
        );
    } else {
        sh_do(&format!("cd .. && rm -rf work-mrubyedge-{}", &suffix), opts.verbose)?;
    }

    std::env::set_current_dir(pwd)?;

    println!("[ok] wasm file is generated: {}.wasm", &fname);

    Ok(())
}
