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
  pub fn init(path: PathBuf, name: String) -> Database {
    let hexed_name = hex::encode(name.clone());
    let database_dir = Path::new(path.as_path()).join(hexed_name);
    fs_extra::ensure_dir(database_dir.clone());
    Database {
      name,
      database_dir,
      root_dir: path,
    }
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn set_name(&mut self, new_name: String) -> bool {
    self.name = new_name;
    let hexed_name = hex::encode(self.name.clone());
    let new_db_dir = Path::new(self.root_dir.as_path()).join(hexed_name);
    let success = fs::rename(self.database_dir.clone(), new_db_dir.clone());
    self.database_dir = new_db_dir;
    success.is_ok()
  }

  pub fn create_table(&self, name: String) -> Table {
    Table::init(self.database_dir.clone(), name)
  }

  pub fn drop_table(&self, name: String) -> bool {
    let hexed_name = hex::encode(name.clone());
    let table_dir = Path::new(self.database_dir.as_path()).join(hexed_name);
    fs::remove_dir_all(table_dir.clone()).is_ok() && fs::remove_dir(table_dir).is_ok()
  }
}
