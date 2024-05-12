# Patty

Cross platform system PATH manager.

# Introduction

`Patty` enables you to manage and modify the system `PATH` variable on `Windows`, `Linux`, and `macOS`.

It's particularly useful for `CLI` apps that need to be accessible in `PATH`.

`Patty` achieves this by modifying the registry in `Windows` and adjusting `RC` files in `Linux`/`macOS`, such as `zshrc` or `bashrc`.

# Usage

```rust
use patty::{home_dir, Options, PathManager};

fn main() {
    let mut patty = patty::Patty::new(Options::default());
    let home = home_dir().unwrap().join("bin");
    let path = patty.add(home).unwrap();
    println!("PATH = {:?}", path);
}
```

# Examples

see [examples](examples)
