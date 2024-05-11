use std::path::PathBuf;

use eyre::Result;
use patty::{PathManager, Settings};
fn main() -> Result<()> {
    let home = std::env::var(if cfg!(target_os = "windows") {
        "USERPROFILE"
    } else {
        "HOME"
    })
    .unwrap();
    let path = PathBuf::from(home).join("append-example");
    let path = path.to_str().unwrap();
    let mut patty = patty::Patty::new(Settings {
        ignore_errors: true,
        ..Default::default()
    });
    patty.append(path)?;
    let new_path = patty.get()?.unwrap();
    println!("PATH = {}", new_path);
    patty.remove(path)?;
    Ok(())
}
