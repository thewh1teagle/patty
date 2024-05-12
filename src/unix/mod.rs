use eyre::{Context, ContextCompat, Result};
use std::{fs, path::PathBuf};
mod shell;
use crate::{utils, Options, PathManager};

pub struct Patty {
    pub options: Options,
}

impl Patty {
    pub fn new(options: Options) -> Self {
        Self { options }
    }
}

impl PathManager for Patty {
    fn add(&mut self, bin_folder: PathBuf) -> Result<Vec<PathBuf>> {
        let app_env_path = if self.options.app_env_path.is_some() {
            self.options.app_env_path.clone().unwrap()
        } else {
            bin_folder.join("env")
        };

        for sh in shell::get_available_shells() {
            let app_env_path = app_env_path.clone();
            let source_cmd = sh.source_string(&app_env_path)?;
            let source_cmd = format!("\n{}", &source_cmd); // ensure new line suffix

            for rc in sh.update_rcs() {
                let cmd_to_write = match fs::read_to_string(&rc) {
                    Ok(contents) if contents.contains(&source_cmd) => continue, // Skip if already exists in shell RC file
                    Ok(contents) if !contents.ends_with('\n') => &source_cmd, // Ensure to return RC content with new line
                    _ => &source_cmd,
                };

                let rc_clone = rc.clone();
                let rc_dir = rc_clone.parent().with_context(|| {
                    format!(
                        "parent directory doesn't exist for rcfile path: `{}`",
                        rc.display()
                    )
                })?;
                utils::ensure_dir_exists(rc_dir.to_path_buf())?;
                utils::append_file(rc.clone(), cmd_to_write)
                    .context(format!("could not amend shell profile: '{}'", rc.display()))?;
            }

            // Write env source files
            let mut written = vec![];
            let script = sh.env_script();
            // Only write each possible script once.
            if !written.contains(&script) {
                utils::ensure_dir_exists(app_env_path.parent().context("parent")?.to_path_buf())?;
                script.write(
                    &app_env_path.clone(),
                    bin_folder.clone().as_path(),
                    self.options.description.clone(),
                )?;
                written.push(script);
            }
        }

        let mut new_path = self.get()?;
        new_path.push(bin_folder.into());
        Ok(new_path)
    }

    fn remove(&mut self, bin_folder: PathBuf) -> Result<Vec<PathBuf>> {
        let app_env_path = if self.options.app_env_path.is_some() {
            self.options.app_env_path.clone().unwrap()
        } else {
            bin_folder.join("env")
        };
        if app_env_path.clone().is_file() {
            fs::remove_file(app_env_path.clone())?;
        }

        // Remove parent if is empty
        if fs::read_dir(bin_folder.parent().context("parent")?)?.count() == 0 {
            fs::remove_dir(bin_folder.parent().context("parent")?)?;
        }

        for sh in shell::get_available_shells() {
            let app_env_path = app_env_path.clone();
            let source_cmd = sh.source_string(&app_env_path)?;


            for rc in sh.update_rcs() {
                let cmd_to_write = fs::read_to_string(&rc)?;
                let cmd_to_write = cmd_to_write.replace(&source_cmd, "");

                let rc_clone = rc.clone();
                let rc_dir = rc_clone.parent().with_context(|| {
                    format!(
                        "parent directory doesn't exist for rcfile path: `{}`",
                        rc.display()
                    )
                })?;
                utils::ensure_dir_exists(rc_dir.to_path_buf())?;
                utils::write_file(&rc, &cmd_to_write)
                    .context(format!("could not edit shell profile: '{}'", rc.display()))?
            }
        }

        let mut new_path = self.get()?;
        new_path.push(bin_folder.into());
        Ok(new_path)
    }
}