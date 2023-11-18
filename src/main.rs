use std::path::PathBuf;

use database::Database;

pub mod database;
pub mod fs_extra;
pub mod utils;

fn main() {
  let db = Database::init(PathBuf::from("./"), "Valkeyre".to_string());
}
