use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn main() {
    // Does not recursively traverse.
    let asset_dir_str = "test".to_owned();
    let asset_dir = Path::new(&asset_dir_str);
    let f = |e: &DirEntry| {
        let path_str = e.path().to_str().unwrap().to_owned();
        let pattern_str = format!("{}/", &asset_dir_str);
        let without_orig_dir = path_str.trim_start_matches(&pattern_str);
        let without_file = path_str.trim_end_matches(e.path().file_name().unwrap().to_str().unwrap());
        println!("{}", without_orig_dir);
        println!("{}", without_file);
    };

    visit_dirs(asset_dir, &f).unwrap();
}
