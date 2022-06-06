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
use std::{path::Path, process::Command};
use {
    crate::{
        utils::{self, add_mod_rs},
        Error, Result,
    },
    std::{fs, io::Write},
};

pub fn create_module_in_package(path: &str) -> Result<()> {
    // find module directory and file paths
    let path_segments: Vec<&str> = path.split("::").collect();
    let has_empty = path_segments.iter().any(|s| s.is_empty());
    if has_empty {
        // this will handle special cases like: "", "::", "::a", "a::"
        // TODO(@ohsayan): Support full paths starting with "::"
        return Err(Error::EmptyPath);
    }
    let root_file_name = utils::get_root_file_name()?;

    if path_segments.len() == 1 {
        // this is wrt the package root
        let dirpath = "src/".to_owned() + path_segments[0];
        let filepath = add_mod_rs(&dirpath);

        // create the module directory (src/<mod>/)
        fs::create_dir(&dirpath)?;
        // create the module file
        fs::File::create(filepath)?;
        // append the module entry to the top of the main.rs file
        utils::cowfile(root_file_name, |file, contents| {
            let mod_decl = format!("mod {};\n", path_segments[0]);
            file.write_all(mod_decl.as_bytes())?;
            file.write_all(contents.as_bytes())?;
            Ok(())
        })?;
        Ok(())
    } else {
        Err(Error::Other("modules other than the root aren't supported yet. this will be implemented in a future version".to_string()))
    }
}

#[test]
fn create_module_in_package_test() {
    create_module_in_package("protocol").unwrap();
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
        file.write_all(contents.as_bytes())
    })
    .unwrap();
    fs::remove_dir_all("src/protocol").unwrap();
}
