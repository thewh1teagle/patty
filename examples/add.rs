use patty::{home_dir, Options, PathManager};

fn main() {
    let mut patty = patty::Patty::new(Options::default());
    let home = home_dir().unwrap().join("bin");
    let path = patty.add(home).unwrap();
    println!("PATH = {:?}", path);
}
