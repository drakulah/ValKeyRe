use std::{
  fs,
  path::{Path, PathBuf},
};

use crate::fs_extra;

use self::room::Room;

mod room;

pub struct Store {
  name: String,
  root_dir: PathBuf,
  store_dir: PathBuf,
}

impl Store {
  /**
   * Initializes the store. Creates if the store is not present or if the store is present then uses it.
   */
  pub fn init<S: AsRef<str>>(path: PathBuf, name: S) -> Store {
    let name_ref = name.as_ref();
    let hexed_name = hex::encode(name_ref);
    let store_dir = Path::new(path.as_path()).join(hexed_name);
    fs_extra::ensure_dir(store_dir.clone());
    Store {
      name: name_ref.to_string(),
      store_dir,
      root_dir: path,
    }
  }

  /**
   * Get the name of current store.
   */
  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  /**
   * Set the name of current store.
   */
  pub fn set_name<S: AsRef<str>>(&mut self, new_name: S) -> bool {
    self.name = new_name.as_ref().to_string();
    let hexed_name = hex::encode(self.name.clone());
    let new_db_dir = Path::new(self.root_dir.as_path()).join(hexed_name);
    let success = fs::rename(self.store_dir.clone(), new_db_dir.clone());
    self.store_dir = new_db_dir;
    success.is_ok()
  }

  /**
   * Initializes the room inside the store. Creates if the room is not present or if the room is present then uses it.
   */
  pub fn init_room<S: AsRef<str>>(&self, name: S) -> Room {
    Room::init(self.store_dir.clone(), name)
  }

  /**
   * Completely delete the room from store.
   */
  pub fn drop_room<S: AsRef<str>>(&self, name: S) -> bool {
    let name_ref = name.as_ref();
    let hexed_name = hex::encode(name_ref);
    let room_dir = Path::new(self.store_dir.as_path()).join(hexed_name);
    fs::remove_dir_all(room_dir.clone()).is_ok() && fs::remove_dir(room_dir).is_ok()
  }
}
