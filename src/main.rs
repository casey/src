mod common;
mod config;
mod error;
mod opt;
mod provider;
mod raw;
mod repo;
mod spec;
mod status;

use crate::common::*;

fn main() {
  if let Err(error) = Opt::from_args().run() {
    println!("error: {}", error);

    process::exit(EXIT_FAILURE);
  }
}
