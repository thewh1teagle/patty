use patty::{home_dir, Options, PathManager};

fn main() {
    let home = home_dir().unwrap();
    let bin_path = home.join(".example/bin");
    let mut patty = patty::Patty::new(Options::default());
    patty.add(bin_path).unwrap();
}
