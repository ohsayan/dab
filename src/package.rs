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

#[cfg(test)]
use std::{fs, io::Write, path::Path, process::Command};
use {
    crate::{
        module::{self, ModuleOptions},
        utils, Error, Result,
    },
    cargo_toml::Package,
};

/// Create a module in a package (not a workspace)
pub fn create_module_in_package(
    path: &str,
    options: ModuleOptions,
    _package: Package,
) -> Result<()> {
    _create_module_in_package(path, options)
}

fn _create_module_in_package(path: &str, options: ModuleOptions) -> Result<()> {
    // find module directory and file paths
    let path_segments: Vec<&str> = path.split("::").collect();
    let has_empty = path_segments.iter().any(|s| s.is_empty());
    if has_empty {
        // this will handle special cases like: "", "::", "::a", "a::"
        // TODO(@ohsayan): Support full paths starting with "::"
        return Err(Error::EmptyPath);
    }
    let root_file_name = utils::get_root_file_name()?;
    // create the module
    module::create_module(root_file_name, &path_segments, options)
}

#[test]
fn create_module_in_package_test() {
    _create_module_in_package("protocol", ModuleOptions::default()).unwrap();
    assert!(Path::new("src/protocol").is_dir());
    assert!(Path::new("src/protocol/mod.rs").is_file());
    let cmd = Command::new("cargo").arg("build").output().unwrap();
    assert!(
        cmd.status.success(),
        "{}",
        String::from_utf8_lossy(&cmd.stderr)
    );
    utils::cowfile("src/main.rs", |file, contents| {
        let contents = contents.to_owned().replace("mod protocol;\n", "");
        file.write_all(contents.as_bytes())?;
        Ok(())
    })
    .unwrap();
    fs::remove_dir_all("src/protocol").unwrap();
}
