use patty::{Options, PathManager};

fn main() {
    let mut patty = patty::Patty::new(Options::default());
    let path = patty.get().unwrap();
    println!("PATH = {:?}", path);
}
