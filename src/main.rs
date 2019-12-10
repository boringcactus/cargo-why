#[macro_use]
extern crate failure;

use failure::Error;
use cargo_metadata::{PackageId, Resolve};

fn main() -> Result<(), Error> {
    let (target, other_flags) = {
        let mut args = std::env::args();
        let _ = args.next(); // we don't care about the name of the binary
        if args.next() != Some("why".to_string()) {
            usage();
            ::std::process::exit(1);
        }
        let target = match args.next() {
            Some(target) => {
                if target == "-h" || target == "--help" {
                    usage();
                    ::std::process::exit(1);
                }
                target
            }
            None => {
                usage();
                ::std::process::exit(1);
            }
        };
        let other_flags: Vec<String> = args.collect();
        (target, other_flags)
    };
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.other_options(other_flags);
    let metadata = cmd.exec()?;
    let resolve = match metadata.resolve {
        Some(x) => x,
        None => bail!("No dependency resolution found"),
    };
    let root = resolve.root.clone().ok_or_else(|| format_err!("No dependency resolution root"))?;
    search(vec![&root], &resolve, &target);
    Ok(())
}

fn usage() {
    eprintln!(concat!("cargo-why ", env!("CARGO_PKG_VERSION"), r#"

USAGE:
    cargo why <target crate> [other cargo flags (features, offline, etc)...]

FLAGS:
    -h, --help       Prints help information
"#));
}

fn search(history: Vec<&PackageId>, resolve: &Resolve, target: &str) {
    let curr = match history.last() {
        Some(&x) => x,
        None => return,
    };
    let node = resolve.nodes.iter().find(|node| node.id == *curr);
    let node = match node {
        Some(x) => x,
        None => return,
    };
    for dep in &node.dependencies {
        if dep.repr.contains(&format!("{} ", target)) {
            for pkg in &history {
                let pkg = pkg.repr.split(' ').nth(0).unwrap();
                print!("{} -> ", pkg);
            }
            println!("{}", target);
        } else {
            let mut history = history.clone();
            history.push(dep);
            search(history, resolve, target);
        }
    }
}
