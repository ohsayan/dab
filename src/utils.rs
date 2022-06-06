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
        io::Result as IoResult,
        path::{Path, PathBuf},
    },
};

pub const MAIN_RS: &str = "src/main.rs";
pub const LIB_RS: &str = "src/lib.rs";
const ROOT_FILE: [&str; 2] = [LIB_RS, MAIN_RS];
pub const MOD_RS: &str = "mod.rs";

pub fn get_root_file_name() -> Result<&'static str> {
    let is_lib = Path::new(LIB_RS).is_file();
    let is_bin = Path::new(MAIN_RS).is_file();
    if is_bin && is_lib {
        Err(Error::Other("Current package contains both `lib.rs` and `main.rs`. Unable to determine package type".to_owned()))
    } else {
        Ok(ROOT_FILE[is_bin as usize])
    }
}

pub fn add_mod_rs(path: impl Into<PathBuf>) -> PathBuf {
    let mut p = path.into();
    p.push(MOD_RS);
    p
}

pub fn cowfile(orig: &str, with_open: impl FnOnce(&mut File, &str) -> IoResult<()>) -> Result<()> {
    // read the old file into memory
    let old_file_contents = fs::read_to_string(orig)?;
    // open the COW file
    let new = format!("{}_", orig);
    let mut new_file = OpenOptions::new().write(true).create_new(true).open(&new)?;
    // do whatever the caller wants to
    with_open(&mut new_file, &old_file_contents)?;
    // replace the file
    fs::rename(new, orig)?;
    Ok(())
}

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
