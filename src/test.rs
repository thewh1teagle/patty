use crate::{PathManager, Patty, Settings};
use serial_test::serial;

#[test]
#[serial]
fn append() {
    let mut patty = Patty::new(Settings::default());
    let path = "hello/world";
    patty.append(path).unwrap();
    let new_path = patty.get().unwrap();
    let new_path = new_path.unwrap();
    patty.remove(path).unwrap();
    assert!(new_path.contains(if cfg!(windows) {
        ";hello/world"
    } else {
        ":hello/world"
    }));
}
