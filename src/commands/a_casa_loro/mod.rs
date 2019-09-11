use std::fs::rename;
use std::io::{
  Error,
  ErrorKind,
};
use std::result::Result;
use clap::{
  Values,
};

pub fn run(values: Values) -> Result<(), Error> {
  let mut c = values;
  let origin = c.nth(0);
  let destination = c.nth(1);

  if origin.and(destination).is_some() {
    rename(origin.unwrap(), destination.unwrap())
  } else {
    Err(Error::from(ErrorKind::PermissionDenied))
  }
}
