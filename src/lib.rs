use std::env;
use std::path::PathBuf;

use eyre::{Ok, Result};
mod utils;

#[cfg(test)]
mod test;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use unix::Patty;

use utils::normalize_path;
#[cfg(target_os = "windows")]
pub use windows::RegistryKind;

#[cfg(target_os = "windows")]
pub use windows::Patty;

pub use utils::home_dir;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Options {
    /// Don't fail if already exists / not found
    pub ignore_errors: bool,

    /// **Platform specific: Windows**
    /// Environment kind. Default to user
    #[cfg(target_os = "windows")]
    pub kind: RegistryKind,

    /// **Platform specific: macOS / Linux**
    /// `env` file
    /// Which is a shell source file that exports the required PATH
    /// This file will later be included in the default shell RC file
    /// Such as `.bashrc`
    /// Used on macOS/Linux platforms
    /// Default to {bin_folder}/env
    #[cfg(unix)]
    pub app_env_path: Option<PathBuf>,

    /// **Platform specific: macOS / Linux**
    /// Description for this path entry. written as comment in env file
    #[cfg(unix)]
    pub description: Option<String>,

    /// **Platform specific: macOS / Linux**
    /// Whether to allow modify env as sudo
    /// Default to false
    #[cfg(unix)]
    pub allow_sudo: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            ignore_errors: true,

            #[cfg(target_os = "windows")]
            kind: RegistryKind::User,

            #[cfg(unix)]
            app_env_path: None,

            #[cfg(unix)]
            description: None,

            #[cfg(unix)]
            allow_sudo: false,
        }
    }
}

pub trait PathManager {
    fn add(&mut self, folder: PathBuf) -> Result<Vec<PathBuf>>;
    fn remove(&mut self, folder: PathBuf) -> Result<Vec<PathBuf>>;

    fn get(&mut self) -> Result<Vec<PathBuf>> {
        let path = std::env::var("PATH")?;
        let folders = env::split_paths(&path).map(|p| p.to_path_buf()).collect();
        Ok(folders)
    }

    fn exists(&mut self, bin_path: PathBuf) -> Result<bool> {
        let folders = self.get()?;
        for folder in folders {
            if normalize_path(bin_path.as_path()) == normalize_path(&folder) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
