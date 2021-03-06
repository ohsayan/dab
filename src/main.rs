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

mod workspace;
#[macro_use]
mod macros;
mod errors;
mod module;
mod package;
mod runner;
mod utils;

use {
    errors::{Error, Result},
    std::env,
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    runner::run(args)
        .map_err(|e| exiterr!("Failed with error: {e}"))
        .unwrap();
}
