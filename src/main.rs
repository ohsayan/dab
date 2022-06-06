/*
 * Copyright (c) 2022, Sayan Nandan <nandansayan@outlook.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

mod errors;
mod package;
mod utils;
#[macro_use]
mod macros;

use errors::{Error, Result};
use {
    cargo_toml::Manifest,
    std::{env, fs},
};

const HELP: &str = "\
dab 0.1.0
Sayan Nandan <ohsayan@outlook.com>
dab is a command-line tool for Rust developers that can be used to create modules by paths.

Example usage:
- `dab errors`: Will create a file under src/errors/mod.rs (along with the directory) while
also adding `mod errors.rs` to the root file (`lib.rs` or `main.rs` depending on the package
type)

USAGE:
    dab [FLAGS]

FLAGS:
    --help  Prints help information
";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        exiterr!("Incorrect number of arguments. Run `--help` for usage");
    }
    run(args)
        .map_err(|e| exiterr!("Failed with error: {e}"))
        .unwrap();
}

fn run(args: Vec<String>) -> Result<()> {
    if args[0].as_str() == "--help" {
        println!("{HELP}");
        return Ok(());
    }
    // read Cargo.toml
    let read_file = fs::read_to_string("Cargo.toml")
        .map_err(|_| Error::Other("Couldn't read `Cargo.toml`".to_owned()))?;
    // FIXME(@ohsayan): This is for future validation
    let crate_cfg = Manifest::from_str(&read_file)?;
    if crate_cfg.package.is_some() {
        package::create_module_in_package(&args[0])
    } else {
        Err(Error::Other("workspaces are not supported yet".to_owned()))
    }
}
