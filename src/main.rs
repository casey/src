mod color;
mod common;
mod config;
mod error;
mod opt;
mod provider;
mod raw;
mod repo;
mod spec;
mod src;
mod status;

use crate::common::*;

fn main() {
  if let Err(error) = Opt::from_args().run() {
    let color = Color::new().stderr();

    if color.active() {
      eprintln!(
        "{} {}",
        color.error().paint("error:"),
        color.message().wrap(error),
      );
    } else {
      eprintln!("error: {}", error);
    }

    process::exit(EXIT_FAILURE);
  }
}
