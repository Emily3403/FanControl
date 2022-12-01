use std::borrow::Borrow;
use std::fs;
use std::path::Path;

extern crate shellexpand;

fn main() {
    let ectool_src = Path::new("bin/ectool");
    let dest = shellexpand::tilde("~/.local/bin/ectool");
    let dest: &Path = Path::new(dest.borrow() as &str);

    fs::create_dir_all(dest.parent().unwrap()).unwrap();
    fs::copy(ectool_src, dest).unwrap();
}