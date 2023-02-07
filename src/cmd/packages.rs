// Copyright (c) 2017-2018 ETH Zurich
// Fabian Schuiki <fschuiki@iis.ee.ethz.ch>

//! The `packages` subcommand.

use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::error::*;
use crate::sess::Session;

/// Assemble the `packages` subcommand.
pub fn new() -> Command {
    Command::new("packages")
        .about("Information about the dependency graph")
        .arg(Arg::new("graph")
            .short('g')
            .long("graph")
            .num_args(0)
            .action(ArgAction::SetTrue)
            .help("Print the dependencies for each package")
        )
        .arg(Arg::new("flat")
            .short('f')
            .long("flat")
            .num_args(0)
            .action(ArgAction::SetTrue)
            .help("Do not group packages by topological rank. If the `--graph` option is specified, print multiple lines per package, one for each dependency.")
        )
}

/// Execute the `packages` subcommand.
pub fn run(sess: &Session, matches: &ArgMatches) -> Result<()> {
    let graph = matches.get_flag("graph");
    let flat = matches.get_flag("flat");
    if graph {
        for (&pkg, deps) in sess.graph().iter() {
            let pkg_name = sess.dependency_name(pkg);
            let dep_names = deps.iter().map(|&id| sess.dependency_name(id));
            if flat {
                // Print one line per dependency.
                for dep_name in dep_names {
                    println!("{pkg_name}\t{dep_name}");
                }
            } else {
                // Print all dependencies on one line.
                print!("{pkg_name}\t");
                for (i, dep_name) in dep_names.enumerate() {
                    if i > 0 {
                        print!(" {dep_name}");
                    } else {
                        print!("{dep_name}");
                    }
                }
                println!();
            }
        }
    } else {
        for pkgs in sess.packages().iter() {
            let pkg_names = pkgs.iter().map(|&id| sess.dependency_name(id));
            if flat {
                // Print one line per package.
                for pkg_name in pkg_names {
                    println!("{pkg_name}");
                }
            } else {
                // Print all packages per rank on one line.
                for (i, pkg_name) in pkg_names.enumerate() {
                    if i > 0 {
                        print!(" {pkg_name}");
                    } else {
                        print!("{pkg_name}");
                    }
                }
                println!();
            }
        }
    }
    Ok(())
}
