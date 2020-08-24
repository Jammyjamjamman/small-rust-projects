use std::io;
use std::fs::{self, DirEntry, File, create_dir_all};
use std::path::Path;
use std::env;
use std::io::prelude::*;


fn walk(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn main() {
    // Process files stored in ./assets/.
    let asset_dir_str = "assets".to_owned();
    let out_dir = format!("target/{}", env::var("PROFILE").expect("Error getting the target directory."));

    let asset_dir = Path::new(&asset_dir_str);

    let mut f = |e: &DirEntry| {
        let path_str = e.path().to_str().expect("error getting path").to_owned();
        // Get path with target dir, not asset dir.
        let target_path = {
            let pattern_str = format!("{}/", &asset_dir_str);
            let without_orig_dir = path_str.trim_start_matches(&pattern_str);
            format!("{}/{}", out_dir, without_orig_dir)
        };

        let target_without_file = target_path.trim_end_matches(e.path().file_name().unwrap().to_str().unwrap());
        create_dir_all(Path::new(&target_without_file)).expect(&format!("Error creating dirs: {}", &target_without_file));
        fs::copy(&path_str, &target_path).expect(&format!("error copying file: {}", &path_str));
    };
    walk(asset_dir, &mut f).expect("error walking assets");
}