use crate::{utils, Options, PathManager};
use eyre::{bail, Context, Result};
use std::path::PathBuf;
use std::{env, io};
use winreg::enums::{RegType, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WRITE};
use winreg::{RegKey, RegValue};

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum RegistryKind {
    /// HKEY_CURRENT_USER
    User,
    /// HKEY_LOCAL_MACHINE. require admin rights
    System,
}

pub struct Patty {
    pub options: Options,
}

impl Patty {
    pub fn new(options: Options) -> Self {
        Self { options }
    }
}

impl PathManager for Patty {
    fn add(&mut self, folder: PathBuf) -> Result<Vec<PathBuf>> {
        if !self.options.ignore_errors && self.exists(folder.clone())? {
            bail!("already exists")
        }
        let mut path = read_path(&self.options)?;
        let folder_exists = env::split_paths(&path)
            .any(|p| utils::normalize_path(&p) == utils::normalize_path(&folder));
        if folder_exists {
            if !self.options.ignore_errors {
                bail!("path {} already exists!", folder.display());
            }
            return self.get();
        }
        if path.ends_with(';') {
            path.pop();
        }
        path.push_str(&format!(";{}", folder.to_str().unwrap()));
        apply_new_path(path.clone(), &self.options.kind)?;
        self.get()
    }

    // Get the windows PATH variable out of the registry as a String. If
    // this returns None then the PATH variable is not a string and we
    // should not mess with it.
    fn get(&mut self) -> Result<Vec<PathBuf>> {
        let path = read_path(&self.options)?;
        let folders = std::env::split_paths(&path)
            .map(|p| p.to_path_buf())
            .collect();
        Ok(folders)
    }
    fn remove(&mut self, folder: PathBuf) -> Result<Vec<PathBuf>> {
        if !self.options.ignore_errors && !self.exists(folder.clone())? {
            bail!("not found")
        }
        let mut path = read_path(&self.options)?;
        if path.ends_with(';') {
            path.pop();
        }
        let folder = utils::normalize_path(&folder);
        let folders: Vec<&str> = path.split(';').collect();
        let new_folders: Vec<&str> = folders
            .into_iter()
            .filter(|&f| normalize(f) != folder)
            .collect();
        let new_path = new_folders.join(";");
        apply_new_path(new_path.clone(), &self.options.kind)?;
        let new_path = new_folders.iter().map(PathBuf::from).collect();
        Ok(new_path)
    }
}

fn read_path(options: &Options) -> Result<String> {
    let environment_path = match options.kind {
        RegistryKind::User => "Environment",
        RegistryKind::System => "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
    };

    let root = RegKey::predef(if options.kind == RegistryKind::User {
        HKEY_CURRENT_USER
    } else {
        HKEY_LOCAL_MACHINE
    });
    let environment = root
        .open_subkey_with_flags(environment_path, KEY_READ | KEY_WRITE)
        .context("Failed opening Environment key")?;

    let reg_value = environment.get_raw_value("PATH");
    match reg_value {
        Ok(val) => {
            if let Some(s) = from_winreg_value(&val) {
                let path = String::from_utf16(&s).context("decode error")?;
                Ok(path)
            } else {
                log::warn!(
                    "the registry key {}\\PATH is not a string. Not modifying the PATH variable",
                    if options.kind == RegistryKind::User {
                        "HKEY_CURRENT_USER\\Environment"
                    } else {
                        "HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment"
                    }
                );
                Ok(String::new())
            }
        }
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(e).context("unknown"),
    }
}

fn normalize(path: &str) -> String {
    let mut path = path.replace('/', "\\");
    if path.ends_with('\\') {
        path.pop();
    }
    path
}

fn apply_new_path(mut new_path: String, kind: &RegistryKind) -> Result<()> {
    // Don't leave a trailing ; though, we don't want an empty string in the path
    if new_path.ends_with(';') {
        new_path.pop();
    }

    let root = RegKey::predef(match kind {
        RegistryKind::User => HKEY_CURRENT_USER,
        RegistryKind::System => HKEY_LOCAL_MACHINE,
    });

    let environment_path = match kind {
        RegistryKind::User => "Environment",
        RegistryKind::System => "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
    };
    let environment = root.open_subkey_with_flags(environment_path, KEY_READ | KEY_WRITE)?;

    if new_path.is_empty() {
        environment.delete_value("PATH")?;
    } else {
        let reg_value = RegValue {
            bytes: to_winreg_bytes(new_path.encode_utf16().collect()),
            vtype: RegType::REG_EXPAND_SZ,
        };
        environment.set_raw_value("PATH", &reg_value)?;
    }
    notify_path_changed();
    Ok(())
}

/// Convert a vector UCS-2 chars to a null-terminated UCS-2 string in bytes
fn to_winreg_bytes(mut v: Vec<u16>) -> Vec<u8> {
    v.push(0);
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast::<u8>(), v.len() * 2).to_vec() }
}

/// This is used to decode the value of HKCU\Environment\PATH. If that key is
/// not REG_SZ | REG_EXPAND_SZ then this returns None. The winreg library itself
/// does a lossy unicode conversion.
fn from_winreg_value(val: &winreg::RegValue) -> Option<Vec<u16>> {
    use std::slice;

    match val.vtype {
        RegType::REG_SZ | RegType::REG_EXPAND_SZ => {
            // Copied from winreg
            let mut words = unsafe {
                #[allow(clippy::cast_ptr_alignment)]
                slice::from_raw_parts(val.bytes.as_ptr().cast::<u16>(), val.bytes.len() / 2)
                    .to_owned()
            };
            while words.last() == Some(&0) {
                words.pop();
            }
            Some(words)
        }
        _ => None,
    }
}

/// Tell other processes to update their environment
fn notify_path_changed() {
    use windows::Win32::Foundation::*;
    use windows::Win32::UI::WindowsAndMessaging::{
        SendMessageTimeoutA, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE,
    };

    unsafe {
        SendMessageTimeoutA(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            WPARAM(0),
            LPARAM("Environment\0".as_ptr() as _),
            SMTO_ABORTIFHUNG,
            5000,
            None,
        );
    }
}
