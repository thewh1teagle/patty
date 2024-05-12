use std::path::PathBuf;

use crate::{Options, PathManager, Patty};
use serial_test::serial;

#[test]
#[serial]
fn add() {
    let mut patty = Patty::new(Options::default());
    let path = "hello/world";
    patty.add(path.into()).unwrap();
    let new_path = patty.get().unwrap();
    patty.remove(path.into()).unwrap();
    let exists = new_path.contains(&PathBuf::from(if cfg!(windows) {
        ";hello/world"
    } else {
        ":hello/world"
    }));
    assert!(exists);
}
