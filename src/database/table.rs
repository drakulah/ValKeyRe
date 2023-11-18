use std::{
  fs,
  path::{Path, PathBuf},
};

use crate::fs_extra;

pub struct Table {
  name: String,
  root_dir: PathBuf,
  table_dir: PathBuf,
}

impl Table {
  pub fn init(root_dir: PathBuf, name: String) -> Table {
    let hexed_name = hex::encode(name.clone());
    let table_dir = Path::new(root_dir.as_path()).join(hexed_name);
    fs_extra::ensure_dir(table_dir.clone());
    Table {
      name,
      table_dir,
      root_dir,
    }
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn set_name(&mut self, new_name: String) -> bool {
    self.name = new_name;
    let hexed_name = hex::encode(self.name.clone());
    let new_table_dir = Path::new(self.root_dir.as_path()).join(hexed_name);
    let success = fs::rename(self.table_dir.clone(), new_table_dir.clone());
    self.table_dir = new_table_dir;
    success.is_ok()
  }

  pub fn truncate(&self) -> bool {
    fs::remove_dir_all(self.table_dir.clone()).is_ok()
  }

  pub fn has(&self, key: String) -> bool {
    let hexed_key = hex::encode(key);
    let data_file_path = Path::new(self.table_dir.as_path()).join(hexed_key);
    data_file_path.exists() && data_file_path.is_file()
  }

  pub fn get(&self, key: String) -> Option<String> {
    let hexed_key = hex::encode(key);
    let data_file_path = Path::new(self.table_dir.as_path()).join(hexed_key);

    if !data_file_path.exists() || !data_file_path.is_file() {
      return None;
    }

    let maybe_file_content = fs::read_to_string(data_file_path.as_path());

    if maybe_file_content.is_err() {
      return None;
    }

    Some(maybe_file_content.unwrap())
  }

  pub fn set(&self, key: String, value: String) -> bool {
    let hexed_key = hex::encode(key);
    let data_file_path = Path::new(self.table_dir.as_path()).join(hexed_key);
    let ensured = fs_extra::ensure_file(data_file_path.clone());

    if !ensured {
      return false;
    }

    fs::write(data_file_path, value).is_ok()
  }

  pub fn remove(&self, key: String) -> bool {
    let hexed_key = hex::encode(key);
    let data_file_path = Path::new(self.table_dir.as_path()).join(hexed_key);

    if !data_file_path.exists() || !data_file_path.is_file() {
      return true;
    }

    fs::remove_file(data_file_path).is_ok()
  }

  pub fn keys(&self) -> Vec<String> {
    let mut keys = Vec::new();
    if !self.table_dir.is_dir() {
      return keys;
    }

    let dir = fs::read_dir(self.table_dir.as_path());

    if dir.is_err() {
      return keys;
    }

    for maybe_dir_entry in dir.unwrap().into_iter() {
      if maybe_dir_entry.is_err() {
        continue;
      }

      let dir_entry = maybe_dir_entry.unwrap();
      let maybe_file_type = dir_entry.file_type();

      if maybe_file_type.is_err() {
        continue;
      }

      if !maybe_file_type.unwrap().is_file() {
        continue;
      }

      let maybe_de_hexed = hex::decode(dir_entry.file_name().to_string_lossy().to_string());

      if maybe_de_hexed.is_err() {
        continue;
      }

      let de_hexed = String::from_utf8_lossy(&maybe_de_hexed.unwrap()).to_string();

      keys.push(de_hexed);
    }

    keys
  }

  pub fn values(&self) -> Vec<String> {
    let mut values = Vec::new();
    if !self.table_dir.is_dir() {
      return values;
    }

    let dir = fs::read_dir(self.table_dir.as_path());

    if dir.is_err() {
      return values;
    }

    for maybe_dir_entry in dir.unwrap().into_iter() {
      if maybe_dir_entry.is_err() {
        continue;
      }

      let dir_entry = maybe_dir_entry.unwrap();
      let maybe_file_type = dir_entry.file_type();

      if maybe_file_type.is_err() {
        continue;
      }

      if !maybe_file_type.unwrap().is_file() {
        continue;
      }

      let maybe_file_content = fs::read_to_string(dir_entry.path());

      if maybe_file_content.is_err() {
        continue;
      }

      values.push(maybe_file_content.unwrap());
    }

    values
  }
}
