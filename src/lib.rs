use eyre::Result;

#[cfg(test)]
mod test;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
use windows::RegistryKind;

#[cfg(target_os = "windows")]
pub use windows::Patty;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Settings {
    /// Don't fail if already exists / not found
    pub ignore_errors: bool,

    /// **Platform specific: Windows**
    /// Environment kind. Default to user
    pub kind: RegistryKind,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            ignore_errors: true,
            kind: RegistryKind::User,
        }
    }
}

pub trait PathManager {
    fn get(&mut self) -> Result<Option<String>>;
    fn append(&mut self, folder: &str) -> Result<String>;
    fn remove(&mut self, folder: &str) -> Result<String>;
    fn exists(&mut self, folder: &str) -> Result<bool>;
}
