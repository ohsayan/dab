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
    crate::{Error, Result},
    std::{
        fs::{self, File, OpenOptions},
        path::{Path, PathBuf},
    },
};

/// `src/main.rs`
pub const MAIN_RS: &str = "src/main.rs";
/// `src/lib.rs`
pub const LIB_RS: &str = "src/lib.rs";
/// The root file look-up
const ROOT_FILE: [&str; 2] = [LIB_RS, MAIN_RS];
/// `mod.rs`
pub const MOD_RS: &str = "mod.rs";

/// Get the root file (`lib.rs` or `main.rs` depending on binary/library crate type)
pub fn get_root_file_name() -> Result<&'static str> {
    let is_lib = Path::new(LIB_RS).is_file();
    let is_bin = Path::new(MAIN_RS).is_file();
    if is_bin && is_lib {
        Err(Error::Other("Current package contains both `lib.rs` and `main.rs`. Unable to determine package type".to_owned()))
    } else {
        Ok(ROOT_FILE[is_bin as usize])
    }
}

/// Add `mod.rs` to the provided path. This is just for convenience
pub fn add_mod_rs(path: impl Into<PathBuf>) -> PathBuf {
    suffix(path, MOD_RS)
}

/// Add suffix `ext` to the path. This is also for convenience
pub fn suffix(path: impl Into<PathBuf>, ext: impl AsRef<Path>) -> PathBuf {
    let mut p = path.into();
    p.push(ext);
    p
}

/// A COW-style file manipulation function. This fill open the original file, read its contents
/// and create a new file on a separate path, allow the user to modify it and then it will replace
/// the original file with the new file
pub fn cowfile(orig: &str, with_open: impl FnOnce(&mut File, &str) -> Result<()>) -> Result<()> {
    // read the old file into memory
    let old_file_contents = fs::read_to_string(orig)?;
    // open the COW file
    let new = format!("{}_", orig);
    let mut new_file = OpenOptions::new().write(true).create_new(true).open(&new)?;
    // do whatever the caller wants to
    with_open(&mut new_file, &old_file_contents)?;
    // fsync
    new_file.sync_all()?;
    // replace the file
    fs::rename(new, orig)?;
    Ok(())
}

/// Validate a module name. Rules:
/// - Can only start with alphabetic chars
/// - Can start with `_` only if the module name is longer than 2 bytes
/// - Must be completely ASCII
/// - Cannot be empty
pub fn validate_module_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Error::bad_module_name();
    }
    let first_byte = name.as_bytes()[0];
    let mut valid_name: bool = true;
    valid_name &= first_byte.is_ascii_alphabetic() || // first byte must be a Latin alphabet
        (
            first_byte == b'_' && // can be an underscore
            name.len() != 1 // but only if it is followed by something
        );
    // all digits must be ASCII alphanumeric or an `_`
    valid_name &= name
        .bytes()
        .all(|b| u8::is_ascii_alphanumeric(&b) || b == b'_');
    if valid_name {
        Ok(())
    } else {
        Error::bad_module_name()
    }
}
