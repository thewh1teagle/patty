use eyre::Result;
use std::path::{Path, PathBuf};

use crate::utils::{self, home_dir};

mod bash;
mod fish;
mod posix;
mod zsh;

pub type Shell = Box<dyn UnixShell>;

#[derive(Debug, PartialEq)]
pub struct ShellScript {
    content: &'static str,
    name: &'static str,
}

impl ShellScript {
    pub fn write(
        &self,
        path: &Path,
        bin_folder: &Path,
        description: Option<String>,
    ) -> Result<()> {
        let mut content = self
            .content
            .replace("{template_bin_path}", bin_folder.to_str().unwrap());
        match description {
            Some(description) => {
                // Render description
                content = content.replace("{template_description}", &description);
            }
            None => {
                // Remove description from content
                content = content.replace("\n# {template_description}", "");
            }
        }
        let home = home_dir().unwrap_or_default();
        content = content.replace(home.to_str().unwrap(), "$HOME");
        utils::write_file(path, &content)?;
        Ok(())
    }
}

pub trait UnixShell {
    /// Detects if a shell "exists". Users have multiple shells, so an "eager"
    /// heuristic should be used, assuming shells exist if any traces do.
    fn does_exist(&self) -> bool;

    /// Gives all rcfiles of a given shell.
    /// Used primarily in checking rcfiles for cleanup.
    fn rcfiles(&self) -> Vec<PathBuf>;

    /// Gives rcs that should be written to.
    fn update_rcs(&self) -> Vec<PathBuf>;

    /// Writes the relevant env file.
    /// Default env.sh
    fn env_script(&self) -> ShellScript {
        ShellScript {
            name: "env",
            content: include_str!("template/env.sh"),
        }
    }

    /// Source string. will be written to shell RC file
    fn source_string(&self, rc_path: &Path) -> Result<String> {
        Ok(format!(r#"source "{}""#, rc_path.to_str().unwrap()))
    }
}

// TODO: Tcsh (BSD)
// TODO?: Make a decision on Ion Shell, Power Shell, Nushell
// Cross-platform non-POSIX shells have not been assessed for integration yet
fn enumerate_shells() -> Vec<Shell> {
    vec![
        Box::new(posix::Posix),
        Box::new(bash::Bash),
        Box::new(zsh::Zsh),
        Box::new(fish::Fish),
    ]
}

pub fn get_available_shells() -> impl Iterator<Item = Shell> {
    enumerate_shells().into_iter().filter(|sh| sh.does_exist())
}
