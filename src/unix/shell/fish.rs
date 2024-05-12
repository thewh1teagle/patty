use crate::utils;
use std::{env, path::{Path, PathBuf}};

use super::{ShellScript, UnixShell};
use eyre::Result;
pub struct Fish;

impl UnixShell for Fish {
    fn does_exist(&self) -> bool {
        // fish has to either be the shell or be callable for fish setup.
        matches!(env::var("SHELL"), Ok(sh) if sh.contains("fish"))
            || utils::which(&"fish").is_some()
    }

    // > "$XDG_CONFIG_HOME/fish/conf.d" (or "~/.config/fish/conf.d" if that variable is unset) for the user
    // from <https://github.com/fish-shell/fish-shell/issues/3170#issuecomment-228311857>
    fn rcfiles(&self) -> Vec<PathBuf> {
        let p0 = env::var("XDG_CONFIG_HOME").ok().map(|p| {
            let path = PathBuf::from(p);
            path
        });

        let p1 = utils::home_dir().map(|path| path);

        p0.into_iter().chain(p1).collect()
    }

    fn update_rcs(&self) -> Vec<PathBuf> {
        // The first rcfile takes precedence.
        match self.rcfiles().into_iter().next() {
            Some(path) => vec![path],
            None => vec![],
        }
    }

    fn env_script(&self) -> ShellScript {
        ShellScript {
            name: "env.fish",
            content: include_str!("template/env.fish"),
        }
    }

    fn source_string(&self, env_file_path: &Path) -> Result<String> {
        Ok(format!(r#"source "{}""#, env_file_path.to_str().unwrap()))
    }
}
