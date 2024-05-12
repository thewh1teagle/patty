use std::env;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use eyre::Result;
#[cfg(unix)]
use std::{
    fs,
    io::{self, Write},
};

pub fn home_dir() -> Option<PathBuf> {
    if let Ok(home) = env::var(if cfg!(windows) { "USERPROFILE" } else { "HOME" }) {
        return Some(PathBuf::from(home));
    }
    None
}

/// Check if the given program name (without suffix) exists in PATH
/// Return Path or None
#[cfg(unix)]
pub fn which(cmd: &str) -> Option<PathBuf> {
    let cmd = format!("{}{}", cmd, env::consts::EXE_SUFFIX);
    let path_env = env::var("PATH").unwrap_or_default();
    let path_folders = env::split_paths(&path_env).map(|p| p.join(&cmd));
    for folder in path_folders {
        let cmd_path = folder.join(cmd.clone());
        if cmd_path.exists() {
            return Some(cmd_path);
        }
    }
    None
}

#[cfg(unix)]
pub fn ensure_dir_exists(path: PathBuf) -> Result<()> {
    if !path.is_dir() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

#[cfg(unix)]
pub fn append_file(dest: PathBuf, line: &str) -> Result<()> {
    let mut dest_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(dest.clone())?;
    writeln!(dest_file, "{line}")?;

    dest_file.sync_data()?;

    Ok(())
}

#[cfg(unix)]
pub fn write_file(path: &Path, contents: &str) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    io::Write::write_all(&mut file, contents.as_bytes())?;

    file.sync_data()?;

    Ok(())
}

pub fn normalize_path(path: &Path) -> String {
    let path = path.to_str().unwrap();
    #[cfg(unix)]
    {
        let mut path = path.replace('\\', "/");
        if path.ends_with('/') {
            path.pop();
        }
        path
    }
    #[cfg(windows)]
    {
        let mut path = path.replace('/', "\\");
        if path.ends_with('\\') {
            path.pop();
        }
        path
    }
}
