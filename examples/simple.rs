use patty::{PathManager, Settings};

fn main() {
    let mut patty = patty::Patty::new(Settings::default());
    let path = patty.get().unwrap();
    match path {
        Some(path) => {
            println!("PATH = {}", path);
        }
        None => {
            println!("PATH is empty")
        }
    }
}
