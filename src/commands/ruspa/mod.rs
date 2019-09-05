use clap::Values;
use std::fs::remove_dir_all;

pub fn run(values: Values) {
  for foo in values.enumerate() {
    let result = remove_dir_all(foo.1);

    result.expect("An error occurred");
  }
}
