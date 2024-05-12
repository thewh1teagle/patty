use patty::{home_dir, Options, PathManager};

fn main() {
    let mut patty = patty::Patty::new(Options::default());
    let bin_path = home_dir().unwrap().join("bin");
    let new_path = patty.add(bin_path).unwrap();
    println!("PATH = {:?}", new_path);
}
