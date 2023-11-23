# ValKeyRe
A key-value store, prioritizing ease of use.

# Example
```rs
use std::path::PathBuf;

use valkeyre::store::Store;

fn main() {
  let store = Store::init(PathBuf::from("./"), "Valkeyre");

  let store_a = store.init_room("Table A");
  store_a.set("email", "hello@gmail.com");
  store_a.set("pass", "hello123");

  let store_b = store.init_room("Table B");
  store_b.set("email", "yo@gmail.com");
  store_b.set("pass", "yo123");

  println!(
    "{} - {}",
    store_a.get("email").unwrap(),
    store_a.get("pass").unwrap()
  );
  println!(
    "{} - {}",
    store_b.get("email").unwrap(),
    store_b.get("pass").unwrap()
  );

  // hello@gmail.com - hello123
  // yo@gmail.com - yo123
}
```