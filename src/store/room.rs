use std::{
  fs,
  path::{Path, PathBuf},
};

use crate::fs_extra;

pub struct Room {
  name: String,
  root_dir: PathBuf,
  room_dir: PathBuf,
}

impl Room {
  /**
   * Initializes the room inside the store. Creates if the room is not present or if the room is present then uses it.
   */
  pub fn init<S: AsRef<str>>(root_dir: PathBuf, name: S) -> Room {
    let name_ref = name.as_ref();
    let hexed_name = hex::encode(name_ref);
    let room_dir = Path::new(root_dir.as_path()).join(hexed_name);
    fs_extra::ensure_dir(room_dir.clone());
    Room {
      name: name_ref.to_string(),
      room_dir,
      root_dir,
    }
  }

  /**
   * Get the name of current room.
   */
  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  /**
   * Set the name of current room.
   */
  pub fn set_name<S: AsRef<str>>(&mut self, new_name: S) -> bool {
    let new_name_ref = new_name.as_ref();
    self.name = new_name_ref.to_string();
    let hexed_name = hex::encode(self.name.clone());
    let new_room_dir = Path::new(self.root_dir.as_path()).join(hexed_name);
    let success = fs::rename(self.room_dir.clone(), new_room_dir.clone());
    self.room_dir = new_room_dir;
    success.is_ok()
  }

  /**
   * Remove all the maps from the room.
   */
  pub fn truncate(&self) -> bool {
    fs::remove_dir_all(self.room_dir.clone()).is_ok()
  }

  /**
   * Check if the key is present.
   */
  pub fn has<S: AsRef<str>>(&self, k: S) -> bool {
    let key = k.as_ref();
    let hexed_key = hex::encode(key);
    let data_file_path = Path::new(self.room_dir.as_path()).join(hexed_key);
    data_file_path.exists() && data_file_path.is_file()
  }

  /**
   * Get the value from the key.
   */
  pub fn get<S: AsRef<str>>(&self, k: S) -> Option<String> {
    let key = k.as_ref();
    let hexed_key = hex::encode(key);
    let data_file_path = Path::new(self.room_dir.as_path()).join(hexed_key);

    if !data_file_path.exists() || !data_file_path.is_file() {
      return None;
    }

    let maybe_file_content = fs::read_to_string(data_file_path.as_path());

    if maybe_file_content.is_err() {
      return None;
    }

    Some(maybe_file_content.unwrap())
  }

  /**
   * Set the key and it's value.
   */
  pub fn set<S: AsRef<str>>(&self, k: S, v: S) -> bool {
    let key = k.as_ref();
    let value = v.as_ref();
    let hexed_key = hex::encode(key);
    let data_file_path = Path::new(self.room_dir.as_path()).join(hexed_key);
    let ensured = fs_extra::ensure_file(data_file_path.clone());

    if !ensured {
      return false;
    }

    fs::write(data_file_path, value).is_ok()
  }

  /**
   * Remove the map from the key.
   */
  pub fn remove<S: AsRef<str>>(&self, k: S) -> bool {
    let key = k.as_ref();
    let hexed_key = hex::encode(key);
    let data_file_path = Path::new(self.room_dir.as_path()).join(hexed_key);

    if !data_file_path.exists() || !data_file_path.is_file() {
      return true;
    }

    fs::remove_file(data_file_path).is_ok()
  }

  /**
   * Get all the keys in the room.
   */
  pub fn keys(&self) -> Vec<String> {
    let mut keys = Vec::new();
    if !self.room_dir.is_dir() {
      return keys;
    }

    let dir = fs::read_dir(self.room_dir.as_path());

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

  /**
   * Get all the values in the room.
   */
  pub fn values(&self) -> Vec<String> {
    let mut values = Vec::new();
    if !self.room_dir.is_dir() {
      return values;
    }

    let dir = fs::read_dir(self.room_dir.as_path());

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
