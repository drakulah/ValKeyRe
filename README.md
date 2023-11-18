# ValKeyRe
A high-performance key-value database library in Rust, prioritizing ease of use.

# Example
```rs
use std::path::PathBuf;

use valkeyre::database::Database;

fn main() {
  let db = Database::init(PathBuf::from("./"), "Valkeyre");

  let table_a = db.init_table("Table A");
  table_a.set("email", "hello@gmail.com");
  table_a.set("pass", "hello123");

  let table_b = db.init_table("Table B");
  table_b.set("email", "yo@gmail.com");
  table_b.set("pass", "yo123");

  println!("{} - {}", table_a.get("email").unwrap(), table_a.get("pass").unwrap());
  println!("{} - {}", table_b.get("email").unwrap(), table_b.get("pass").unwrap());

  // hello@gmail.com - hello123
  // yo@gmail.com - yo123
}
```