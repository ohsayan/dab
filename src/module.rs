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
    crate::{
        utils::{self, add_mod_rs},
        Error, Result,
    },
    std::{collections::HashSet, fs, io::Write},
};

#[derive(Debug, Default)]
/// The configuration to use while creating a module
pub struct ModuleOptions {
    /// show the help menu
    pub is_help: bool,
    /// module should be public
    pub is_public: bool,
    /// module entry (`mod <module>`) should be appended at license header end
    pub from_comment_header_bottom: bool,
    /// module should be create as `<module>.rs` instead of `<module>/mod.rs`
    pub no_mod_folder: bool,
}

impl ModuleOptions {
    /// The flag count (inclusive of short and long)
    const FLAG_COUNT: usize = 7;
    /// Check the options from the given hashset
    pub fn process_options(&mut self, flags: &HashSet<&str>) -> Result<()> {
        self.is_public = flags.contains("public") || flags.contains("P"); // 2
        self.is_help = flags.contains("help"); // 1
        self.from_comment_header_bottom = flags.contains("cskip") || flags.contains("C"); // 2
        self.no_mod_folder = flags.contains("fskip") || flags.contains("F"); // 2
        if flags.len() > Self::FLAG_COUNT {
            return Error::other("Unknown flags");
        }
        Ok(())
    }
}

/// Create the module using the provided `root_file_path`, path segments and the module options
pub fn create_module(
    root_file_path: &str,
    path_segments: &[&str],
    options: ModuleOptions,
) -> Result<()> {
    if path_segments
        .iter()
        .any(|segment| utils::validate_module_name(segment).is_err())
    {
        return Error::bad_module_name();
    }
    if path_segments.len() != 1 {
        // TODO(@ohsayan): Support nested modules
        return Error::other(
            "modules other than the root aren't supported yet. this will be implemented in a future version"
            .to_string()
        );
    }

    // create the module directory (src/<mod>/)
    if options.no_mod_folder {
        // just create <module>.rs
        let filepath = format!("src/{}.rs", path_segments[0]);
        fs::File::create(filepath)?;
    } else {
        // this is wrt the package root
        let dirpath = "src/".to_owned() + path_segments[0];
        let filepath = add_mod_rs(&dirpath);
        fs::create_dir(&dirpath)?;
        // create the module file
        fs::File::create(filepath)?;
    }

    // append the module entry to the top of the main.rs file
    utils::cowfile(root_file_path, |file, contents| {
        patch_file(path_segments[0], contents, options, file)
    })?;
    Ok(())
}

/// Patch the file with the updated data
fn patch_file<W: Write>(
    final_module_name: &str,
    contents: &str,
    options: ModuleOptions,
    file: &mut W,
) -> Result<()> {
    let mod_decl = if options.is_public {
        format!("pub mod {};", final_module_name)
    } else {
        format!("mod {};", final_module_name)
    };
    if contents.starts_with("/*") && options.from_comment_header_bottom {
        // starts with a comment and we have to append below it
        let mut comment_end_idx = contents.find("*/").ok_or_else(|| {
            Error::Other("Your source file possibly has a syntax error".to_string())
        })? + 2;

        if contents.as_bytes().get(comment_end_idx + 1) == Some(&b'\n') {
            comment_end_idx += 1;
        }

        // append the comment
        file.write_all(&contents.as_bytes()[..comment_end_idx])?;

        // One LF for the comment end line
        file.write_all(b"\n")?;

        // now write the module declaration
        file.write_all(mod_decl.as_bytes())?;
        // now write the remaining data
        file.write_all(&contents.as_bytes()[comment_end_idx..])?;
    } else {
        file.write_all(mod_decl.as_bytes())?;
        file.write_all(b"\n")?;
        file.write_all(contents.as_bytes())?;
    }
    Ok(())
}

#[test]
#[allow(clippy::field_reassign_with_default)]
fn file_without_comment_patch() {
    let mut options = ModuleOptions::default();
    options.from_comment_header_bottom = true;
    const FILE_WITHOUT_COMMENT: &str = "\
mod x;
mod y;

fn main() {
    println!(\"Hello, World\");
}

";
    const FILE_WITHOUT_COMMENT_PATCHED: &str = "\
mod z;
mod x;
mod y;

fn main() {
    println!(\"Hello, World\");
}

";
    let mut v = Vec::new();
    patch_file("z", FILE_WITHOUT_COMMENT, options, &mut v).unwrap();
    assert_eq!(String::from_utf8_lossy(&v), FILE_WITHOUT_COMMENT_PATCHED);
}

#[test]
#[allow(clippy::field_reassign_with_default)]
fn file_with_comment_patch() {
    let mut options = ModuleOptions::default();
    options.from_comment_header_bottom = true;
    const FILE_WITH_COMMENT: &str = r#"/*
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

mod x;
mod y;

fn main() {
    println!("Hello, World");
}
"#;
    const FILE_WITH_COMMENT_PATCHED: &str = r#"/*
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

mod z;
mod x;
mod y;

fn main() {
    println!("Hello, World");
}
"#;
    let mut v = Vec::new();
    patch_file("z", FILE_WITH_COMMENT, options, &mut v).unwrap();
    fs::File::create("resulting_file.rs")
        .unwrap()
        .write_all(&v)
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&v), FILE_WITH_COMMENT_PATCHED);
}
