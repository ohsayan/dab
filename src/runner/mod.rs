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
use {
    crate::{package, utils, Error, Result},
    cargo_toml::Manifest,
    std::{collections::HashSet, fs},
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
    --help       Prints help information
    --public,-P  Make the new module public
";

#[derive(Debug, Default)]
pub struct ModuleOptions {
    pub is_public: bool,
    pub is_help: bool,
}

impl ModuleOptions {
    const FLAG_COUNT: usize = 2;
    fn process_options(&mut self, flags: &HashSet<&str>) -> Result<()> {
        self.is_public = flags.contains("public") || flags.contains("P");
        self.is_help = flags.contains("help");
        if flags.len() > Self::FLAG_COUNT {
            return Error::other("Unknown flags");
        }
        Ok(())
    }
}

pub fn run(args: Vec<String>) -> Result<()> {
    if args.is_empty() {
        return Error::other("Incorrect number of arguments. Run `--help` for usage");
    }

    // process module options
    let mut options = HashSet::new();
    let mut module = None;
    for arg in args.iter() {
        if let Some(stripped) = arg.strip_prefix("--") {
            if !options.insert(stripped) {
                return Error::other("duplicate options specified");
            }
        } else if module.is_none() {
            module = Some(arg.as_str());
        } else {
            return Error::other("expected one module name");
        }
    }
    let mut modoption = ModuleOptions::default();
    modoption.process_options(&options)?;

    // handle help message case
    if modoption.is_help {
        println!("{HELP}");
        return Ok(());
    } else if module.is_none() {
        // all options; no module? that's broken
        return Error::other("Expected module name. Only found options. Run `--help` for usage");
    }

    // validate module name
    let module = module.unwrap();
    utils::validate_module_name(module)?;

    // read Cargo.toml
    let read_file = fs::read_to_string("Cargo.toml")
        .map_err(|_| Error::Other("Couldn't read `Cargo.toml`".to_owned()))?;
    let crate_cfg = Manifest::from_str(&read_file)?;
    if crate_cfg.package.is_some() {
        package::create_module_in_package(module, modoption)
    } else {
        Err(Error::Other("workspaces are not supported yet".to_owned()))
    }
}
