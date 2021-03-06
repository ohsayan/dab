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

use std::{
    error::Error as StdErrTrait,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
/// Errors arising from running `dab`
pub enum Error {
    /// The module path has empty elements
    EmptyPath,
    /// Some other custom erro
    Other(String),
    /// Error from parsing `Cargo.toml`
    CargoTomlError(cargo_toml::Error),
    /// An I/O error
    IoError(IoError),
    /// The module name was illegal
    BadModuleName,
}

impl Error {
    /// Shorthand for a result with `Self::Other`
    pub fn other<T>(e: impl ToString) -> Result<T> {
        Err(Self::Other(e.to_string()))
    }
    /// Shorthand for a result with `Self::BadModuleName`
    pub fn bad_module_name<T>() -> Result<T> {
        Err(Self::BadModuleName)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::EmptyPath => write!(f, "one or more modules have empty names"),
            Error::Other(oe) => write!(f, "{}", oe),
            Error::CargoTomlError(cargo) => write!(f, "failed to read `Cargo.toml`: {}", cargo),
            Error::IoError(ioe) => write!(f, "I/O error: {ioe}"),
            Error::BadModuleName => write!(f, "bad module name"),
        }
    }
}

impl StdErrTrait for Error {}

impl From<cargo_toml::Error> for Error {
    fn from(e: cargo_toml::Error) -> Self {
        Self::CargoTomlError(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}
