use std::{fs, path::PathBuf};

pub fn ensure_dir(path: PathBuf) -> bool {
  if path.exists() && path.is_dir() {
    return true;
  }

  match fs::create_dir_all(path) {
    Ok(_) => true,
    Err(_) => false,
  }
}

pub fn ensure_file(path: PathBuf) -> bool {
  if path.exists() && path.is_file() {
    return true;
  }

  let parent_dir = path.parent();

  if parent_dir.is_none() {
    return false;
  }

  if !ensure_dir(PathBuf::from(parent_dir.unwrap())) {
    return false;
  }

  match fs::File::create(path) {
    Ok(_) => true,
    Err(_) => false,
  }
}
