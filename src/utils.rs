use eyre::Result;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub fn home_dir() -> Option<PathBuf> {
    if let Ok(home) = env::var("HOME") {
        return Some(PathBuf::from(home));
    }
    None
}

/// Check if the given program name (without suffix) exists in PATH
/// Return Path or None
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

pub fn ensure_dir_exists(path: PathBuf) -> Result<()> {
    if !path.is_dir() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn append_file(dest: PathBuf, line: &str) -> Result<()> {
    let mut dest_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(dest.clone())?;
    writeln!(dest_file, "{line}")?;

    dest_file.sync_data()?;

    Ok(())
}

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
        let mut path = path.replace("\\", "/");
        if path.ends_with("/") {
            path.pop();
        }
        return path;
    }
    #[cfg(windows)]
    {
        let mut path = path.replace('/', "\\");
        if path.ends_with('\\') {
            path.pop();
        }
        return path;
    }
}
