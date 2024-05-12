use patty::{home_dir, Options, PathManager};

fn main() {
    let home = home_dir().unwrap();
    let bin_path = home.join("bin");
    let options = Options {
        // macOS / Linux
        #[cfg(unix)]
        allow_sudo: false, // Don't mess with env files as sudo
        #[cfg(unix)]
        description: Some("Example added with patty".into()),

        // Windows / macOS / Linux
        ignore_errors: true,
        ..Default::default()
    };
    let mut patty = patty::Patty::new(options);
    // patty.add(bin_path).unwrap();
    patty.remove(bin_path).unwrap();
}
