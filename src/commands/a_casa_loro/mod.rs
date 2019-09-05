use std::fs::rename;
use clap::Values;

pub fn run(values: Values) {
  let origin = values.nth(0);
  let destination = values.nth(1);

  if (origin.and(destination).is_some()) {
    rename(origin.unwrap(), destination.unwrap());
  }
}
