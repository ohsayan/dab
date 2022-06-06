# `dab`: The pursuit of laziness for Rust developers

`dab` is a command-line tool that is intended for Rust developers to savor their much deserved laziness after fighting with `async` lifetimes or FFI. Right now,
it does one very simple thing: create modules.

## Features

- [x] Create modules in binary/library packages
- [x] Choose if module is public/private (private by default)
- [x] Ignore comments on top of file while adding modules ("license headers")
- [ ] Support full paths to deeply nested modules
- [ ] Enable parent creation if it doesn't exist
- [ ] Auto add file-header comments ("license headers" for example) to newly create modules
- [ ] Provide a `dab.toml` configuration that will be read for determining settings
- [ ] Run `rustfmt` on adding `mod` entry to the root file
- [ ] Support workspaces:
  - [ ] Support creating modules by package name (`skyd::protocol`)
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
