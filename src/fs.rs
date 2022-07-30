use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub fn find_executable_in_path(path: &str) -> Option<PathBuf> {
    let fullpath = PathBuf::from(path);
    if is_executable(&fullpath) {
        return Some(fullpath);
    }

    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let fullpath = dir.join(path);
                if is_executable(&fullpath) {
                    Some(fullpath)
                } else {
                    None
                }
            })
            .next()
    })
}

fn is_executable(path: &Path) -> bool {
    let metadata = match path.metadata() {
        Ok(metadata) => metadata,
        Err(_) => return false,
    };
    let permissions = metadata.permissions();
    metadata.is_file() && permissions.mode() & 0o111 != 0
}
