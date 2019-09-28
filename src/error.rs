use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum Error {
  #[snafu(display("Git failure at `{}`: {}", path.display(), source))]
  Git { source: git2::Error, path: PathBuf },
  #[snafu(display("I/O failure at `{}`: {}", path.display(), source))]
  Io { source: io::Error, path: PathBuf },
  #[snafu(display("Deserialization of text at `{}` failed: {}", path.display(), source))]
  Deserialize {
    source: serde_yaml::Error,
    path: PathBuf,
  },
  #[snafu(display("Failed to access XDG base directories: {}", source))]
  BaseDirectoriesError { source: xdg::BaseDirectoriesError },
  #[snafu(display("Could not determine home directory"))]
  HomeDirectory,
  #[snafu(display("Unknown default provider: {}", name))]
  DefaultProvider { name: String },
  #[snafu(display("Unknown provider: {}", name))]
  Provider { name: String },
  #[snafu(display("Failed to parse template for `{}`: {}", name, source))]
  Template { name: String, source: tera::Error },
  #[snafu(display("Failed to render template for `{}`: {}", provider, source))]
  Render {
    provider: String,
    source: tera::Error,
  },
  #[snafu(display("Internal error, this is a bug: {}", message))]
  Internal { message: String },
  #[snafu(display("Failed to retrieve username: {}", source))]
  User { source: env::VarError },
  #[snafu(display(
    "Command `{}` invocation failed: {}",
    command.into_iter().map(|os_string| {
      os_string.to_string_lossy().into_owned() 
    }).collect::<Vec<String>>().join(" "),
    source
  ))]
  CommandInvocation {
    command: Vec<OsString>,
    source: io::Error,
  },
  #[snafu(display(
    "Command `{}` failed: {}",
    command.into_iter().map(|os_string| {
      os_string.to_string_lossy().into_owned() 
    }).collect::<Vec<String>>().join(" "),
    status
  ))]
  CommandStatus {
    command: Vec<OsString>,
    status: ExitStatus
  },
  #[snafu(display(
    "Command `{}` failed: {}\n{}\n{}",
    command.into_iter().map(|os_string| {
      os_string.to_string_lossy().into_owned() 
    }).collect::<Vec<String>>().join(" "),
    status,
    stdout,
    stderr,
  ))]
  CommandOutput {
    command: Vec<OsString>,
    status: ExitStatus,
    stdout: String,
    stderr: String,
  },
  #[snafu(display("Failed to place config file: {}", source))]
  ConfigPlace {
    source: io::Error,
  },
  #[snafu(display(
    "Config already exists at path `{}`.\n(Use the `--force` flag to overwite it.)",
    path.display()
  ))]
  ConfigExists { path: PathBuf },
  #[snafu(display("Refusing to push modified repositories.\n(Use the `--force` flag to push anyways.)"))]
  PushDirty,
  #[snafu(display("Failed to push all repositories to `{}`", remote))]
  PushAll { remote: String },
  #[snafu(display("Destination already exists: {}", destination.display()))]
  DestinationExists { destination: PathBuf },
}

impl Error {
  pub(crate) fn internal(message: impl Display) -> Error {
    Error::Internal {
      message: message.to_string(),
    }
  }
}
