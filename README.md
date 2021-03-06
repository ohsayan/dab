# `dab`: The pursuit of laziness for Rustaceans

![Crates.io](https://img.shields.io/crates/v/dab?style=for-the-badge) ![Crates.io](https://img.shields.io/crates/l/dab?style=for-the-badge) ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ohsayan/dab/test?style=for-the-badge)

`dab` is a command-line tool that is intended for Rust developers to savor their much deserved laziness
after fighting with `async` lifetimes or FFI. Right now, it does one very simple thing: create modules.
Well, I got a little mad creating modules while I was [working on Skytable](https://github.com/skytable/skytable). Silly? [Read this](#background)!

## Installation 🚀

Simply run:

```shell
$ cargo install dab
```

## Usage

- Basic usage example: `dab modname`. This will create a `src/modname/mod.rs` file along with
  a module declaration (like `mod modname`) in the "root file" which is either `lib.rs`
  or `main.rs` depending on your crate type
- Advanced usage example: `dab -cskip --dskip --public mymod`: This will create a `src/mymod.rs`
  file (note no directory creation), skip a license header in the "root file" (if any) and mark
  the module visibility to be `pub`

## Features ✨

- [x] Create modules in binary/library packages
- [x] Choose if module is public/private (private by default)
- [x] Ignore comments on top of file while adding modules ("license headers")
- [ ] Rewrite using `syn`
- [ ] Support full paths to deeply nested modules
- [ ] Enable parent creation if it doesn't exist
- [ ] Auto add file-header comments ("license headers" for example) to newly create modules
- [ ] Provide a `dab.toml` configuration that will be read for determining settings
- [ ] Run `rustfmt` on adding `mod` entry to the root file
- [ ] Support `cfg` attributes
- [ ] Support workspaces:
  - [x] Support creating modules by package name (`skyd::protocol`)
  - [ ] Detect workspace root and operate from any other directory (much like what `cargo` does)
  - [ ] Support creation of packages in workspaces
- [ ] Open code editor to the newly created module
- Have ideas? [Create an issue!](https://github.com/skytable/dab/issues/new)

## Background

Call it my personal itch, in large Rust projects I've been extremely annoyed while creating modules (especially in workspaces). The usual sequence was:

1. `mkdir <package>/src/path/to/module`
2. `touch <package>/src/path/to/module/mod.rs`
3. Edit `main.rs` or `lib.rs` to add the package the name
4. Open the code editor and add code

I wanted to trim this down to one step. Hence, this tool.

## License

This tool is licensed under the [Apache-2.0 License](./LICENSE).
