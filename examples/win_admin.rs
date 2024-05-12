use patty::{home_dir, Options, PathManager, RegistryKind};

fn main() {
    let home = home_dir().unwrap();
    let bin_path = home.join(".example/bin");
    let options = Options {
        kind: RegistryKind::System, // Require admin rights
        ..Default::default()
    };
    let mut patty = patty::Patty::new(options);
    patty.add(bin_path).unwrap();
}
