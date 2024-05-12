# Patty

[![Crates](https://img.shields.io/crates/v/patty?logo=rust)](https://crates.io/crates/patty/)
[![License](https://img.shields.io/github/license/thewh1teagle/rookie?color=00aaaa&logo=license)](https://github.com/thewh1teagle/rookie/blob/main/rookie-rs/MIT-LICENSE.txt)

Cross platform system PATH manager.

# Introduction

`Patty` enables you to manage and modify the system `PATH` variable on `Windows`, `Linux`, and `macOS`.

It's particularly useful for `CLI` apps that need to be accessible in `PATH`.

`Patty` achieves this by modifying the registry in `Windows` and adjusting `RC` files in `Linux`/`macOS`, such as `zshrc` or `bashrc`.

# Install

```console
cargo add patty
```

# Usage

```rust
use patty::{PathManager, Options, home_dir};

fn main() {
    let mut patty = patty::Patty::new(Options::default());
    let bin_path = home_dir().unwrap().join("bin");
    let new_path = patty.add(bin_path).unwrap();
    println!("PATH = {:?}", new_path);
}
```

# Examples

see [examples](examples)

# Credits

Inspired by [github.com/rust-lang/rustup](https://github.com/rust-lang/rustup)
