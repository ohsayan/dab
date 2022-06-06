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
    crate::{module, module::ModuleOptions, utils, Error, Result},
    cargo_toml::Workspace,
    std::env,
};

pub fn create_module_in_workspace(
    path: &str,
    options: ModuleOptions,
    workspace: Workspace,
) -> Result<()> {
    let path_segments: Vec<&str> = path.split("::").collect();
    if path_segments.iter().any(|s| s.is_empty()) || path_segments.len() < 2 {
        return Error::other("Bad module path");
    }
    let target_member = path_segments[0];
    if workspace.members.contains(&target_member.to_owned()) {
        // good, now switch to the package directory
        let cd = env::current_dir()?;
        env::set_current_dir(target_member)?;
        // now create the module
        module::create_module(utils::get_root_file_name()?, &path_segments[1..], options)?;
        env::set_current_dir(cd)?;
        Ok(())
    } else {
        // TODO(@ohsayan): Enable package creation in workspace
        Error::other("package not present in workspace `Cargo.toml`. consider adding it there")
    }
}
