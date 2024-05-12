use crate::{home_dir, Options, PathManager, Patty};
use serial_test::serial;
use std::path::PathBuf;

#[test]
#[serial]
fn add() {
    let mut patty = Patty::new(Options::default());
    let path: PathBuf = home_dir().unwrap().join("hello/world");
    let new_path = patty.add(path.clone()).unwrap();
    let exists = new_path.contains(&path);
    patty.remove(path.clone()).unwrap();
    assert!(exists);
}
