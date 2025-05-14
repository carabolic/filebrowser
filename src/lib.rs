/// functions on files, file listings, paths, etc.
use std::{
    fs,
    iter::zip,
    path::{Path, PathBuf},
};

pub fn list_dir(path: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    match fs::read_dir(path) {
        Ok(child_paths) => {
            for child_path in child_paths {
                match child_path {
                    Ok(p) => {
                        result.push(p.path());
                    }
                    Err(e) => println!("error: child path {}", e),
                }
            }
        }
        Err(e) => println!("{}", e),
    };
    result
}

trait PathExt {
    fn starts_with_incomplete<P: AsRef<Path>>(&self, base: P) -> bool;
}

impl PathExt for Path {
    /// Checks if given path is a prefix of this path
    /// _uses the underlying bytes_
    fn starts_with_incomplete<P: AsRef<Path>>(&self, base: P) -> bool {
        let self_bytes = self.as_os_str().as_encoded_bytes();
        let base_bytes = base.as_ref().as_os_str().as_encoded_bytes();
        for (b, s) in zip(base_bytes, self_bytes) {
            if b != s {
                return false;
            }
        }
        return true;
    }
}

pub fn autocomplete_dir(incomplete_path: &String) -> Vec<PathBuf> {
    let mut possible_paths = Vec::new();

    let p = Path::new(incomplete_path);
    let parent_dir = if p.is_dir() && p.to_str().expect("should not happen").ends_with("/") {
        p
    } else {
        p.parent().unwrap_or(Path::new("/"))
    };

    match parent_dir.read_dir() {
        Ok(listing) => {
            possible_paths = listing
                .flatten()
                .map(|e| e.path())
                .filter(|c| c.starts_with_incomplete(p))
                .collect()
        }
        Err(e) => {
            println!("error: no listing for {}, {}", p.display(), e);
        }
    }

    possible_paths
}
