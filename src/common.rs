// stdlib
pub(crate) use std::{
  collections::BTreeMap,
  env,
  ffi::OsString,
  fmt::{self, Display, Formatter},
  fs, io,
  path::{Path, PathBuf},
  process::{self, Command, ExitStatus},
  rc::Rc,
};

// dependencies
pub(crate) use libc::EXIT_FAILURE;
pub(crate) use serde::Deserialize;
pub(crate) use snafu::{ResultExt, Snafu};
pub(crate) use structopt::StructOpt;
pub(crate) use tera::Tera;

// modules
pub(crate) use crate::{error, raw};

// structs and enums
pub(crate) use crate::{
  config::Config, error::Error, opt::Opt, provider::Provider, repo::Repo, spec::Spec,
  status::Status,
};
