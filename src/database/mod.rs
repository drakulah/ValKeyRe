use std::{
  fs,
  path::{Path, PathBuf},
};

use crate::fs_extra;

use self::table::Table;

mod table;

pub struct Database {
  name: String,
  root_dir: PathBuf,
  database_dir: PathBuf,
}

impl Database {
  pub fn init<S: AsRef<str>>(path: PathBuf, name: S) -> Database {
    let name_ref = name.as_ref();
    let hexed_name = hex::encode(name_ref);
    let database_dir = Path::new(path.as_path()).join(hexed_name);
    fs_extra::ensure_dir(database_dir.clone());
    Database {
      name: name_ref.to_string(),
      database_dir,
      root_dir: path,
    }
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn set_name<S: AsRef<str>>(&mut self, new_name: S) -> bool {
    self.name = new_name.as_ref().to_string();
    let hexed_name = hex::encode(self.name.clone());
    let new_db_dir = Path::new(self.root_dir.as_path()).join(hexed_name);
    let success = fs::rename(self.database_dir.clone(), new_db_dir.clone());
    self.database_dir = new_db_dir;
    success.is_ok()
  }

  pub fn init_table<S: AsRef<str>>(&self, name: S) -> Table {
    Table::init(self.database_dir.clone(), name)
  }

  pub fn drop_table<S: AsRef<str>>(&self, name: S) -> bool {
    let name_ref = name.as_ref();
    let hexed_name = hex::encode(name_ref);
    let table_dir = Path::new(self.database_dir.as_path()).join(hexed_name);
    fs::remove_dir_all(table_dir.clone()).is_ok() && fs::remove_dir(table_dir).is_ok()
  }
}
