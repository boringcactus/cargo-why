#[macro_use]
extern crate failure;

use std::collections::{HashMap, HashSet};

use cargo_metadata::{Metadata, PackageId, Resolve};
use failure::Error;

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
    let resolve = match &metadata.resolve {
        Some(x) => x,
        None => bail!("No dependency resolution found"),
    };

    let mut chains = Vec::new();

    for root in &metadata.workspace_members {
        search(vec![root], resolve, &target, &mut chains);
    }

    print_chains(&chains, &metadata);

    Ok(())
}

fn usage() {
    eprintln!(concat!(
        "cargo-why ",
        env!("CARGO_PKG_VERSION"),
        r#"

USAGE:
    cargo why <target crate> [other cargo flags (features, offline, etc)...]

FLAGS:
    -h, --help       Prints help information
"#
    ));
}

fn print_chains(chains: &Vec<Vec<&PackageId>>, metadata: &Metadata) {
    let mut print_versions: HashSet<&PackageId> = HashSet::new();
    let mut name_to_pkgid: HashMap<&str, &PackageId> = HashMap::new();

    for chain in chains {
        for pkg_id in chain {
            let pkg = &metadata[pkg_id];

            let found_id = name_to_pkgid.entry(&pkg.name[..]).or_insert(pkg_id);

            if pkg_id != found_id {
                print_versions.insert(pkg_id);
                print_versions.insert(found_id);
            }
        }
    }

    for chain in chains {
        let mut chain_iter = chain.iter();

        let pkg_id = chain_iter.next().unwrap();
        let pkg = &metadata[pkg_id];

        print!("{}", pkg.name);
        if print_versions.contains(pkg_id) {
            print!(" {}", pkg.version);
        }
        for pkg_id in chain_iter {
            let pkg = &metadata[pkg_id];

            print!(" -> {}", pkg.name);
            if print_versions.contains(pkg_id) {
                print!(" {}", pkg.version);
            }
        }
        println!();
    }
}

fn search<'a>(
    history: Vec<&'a PackageId>,
    resolve: &'a Resolve,
    target: &str,
    chains: &mut Vec<Vec<&'a PackageId>>,
) {
    let curr = match history.last() {
        Some(&x) => x,
        None => return,
    };
    if history[0..history.len() - 1].contains(&curr) {
        // avoid infinite recursion
        return;
    }
    let node = resolve.nodes.iter().find(|node| node.id == *curr);
    let node = match node {
        Some(x) => x,
        None => return,
    };
    for dep in &node.dependencies {
        if dep.repr.contains(&format!("{} ", target)) {
            let mut chain = Vec::new();

            chain.extend(&history);
            chain.push(dep);

            chains.push(chain);
        } else {
            let mut history = history.clone();
            history.push(dep);
            search(history, resolve, target, chains);
        }
    }
}
