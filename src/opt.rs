use crate::common::*;

use Opt::*;

#[derive(StructOpt)]
pub(crate) enum Opt {
  Add {
    #[structopt(required = true, min_values = 1, max_values = 3)]
    spec: Vec<String>,
    #[structopt(long = "name")]
    name: Option<String>,
  },
  Status,
  Get {
    #[structopt(long = "tmp")]
    tmp: bool,
    #[structopt(required = true, min_values = 1, max_values = 3)]
    spec: Vec<String>,
  },
  Remote {
    #[structopt(required = true, min_values = 1, max_values = 3)]
    spec: Vec<String>,
  },
  Init {
    #[structopt(long = "force")]
    force: bool,
  },
  Push {
    remote: String,
    #[structopt(long = "force")]
    force: bool,
  },
  All {
    command: Vec<String>,
  },
}

impl Opt {
  pub(crate) fn run(self) -> Result<(), Error> {
    let config = Config::load()?;

    match self {
      Add { spec, name } => Self::add(config, spec, name),
      All { command } => Self::all(config, &command),
      Status => Self::status(config),
      Remote { spec } => Self::remote(config, spec),
      Get { tmp, spec } => Self::get(config, tmp, spec),
      Init { force } => Self::init(force),
      Push { force, remote } => Self::push(config, remote, force),
    }
  }

  fn add(config: Config, spec: Vec<String>, name: Option<String>) -> Result<(), Error> {
    let spec = config.spec(spec)?;

    let remote = spec.remote()?;

    let name = name.as_deref().unwrap_or(&spec.provider.name);

    let command = &["git", "remote", "add", name, &remote];

    Repo::command_status(command.iter().map(|arg| arg.into()).collect())?;

    Ok(())
  }

  fn all(config: Config, command: &[String]) -> Result<(), Error> {
    let src = Src::load(config.srcdir())?;

    src.all(command)?;

    Ok(())
  }

  fn status(config: Config) -> Result<(), Error> {
    let src = Src::load(config.srcdir())?;

    src.print_status();

    Ok(())
  }

  fn remote(config: Config, spec: Vec<String>) -> Result<(), Error> {
    let spec = config.spec(spec)?;

    println!("{}", spec.remote()?);

    Ok(())
  }

  fn get(config: Config, tmp: bool, spec: Vec<String>) -> Result<(), Error> {
    let spec = config.spec(spec)?;

    let dst = if tmp {
      config.tmpdir().join(&spec.project)
    } else {
      config.srcdir().join(&spec.project)
    };

    if dst.exists() {
      return Err(Error::DestinationExists { destination: dst });
    }

    Repo::clone(&spec.provider.name, &spec.remote()?, &dst)?;

    Ok(())
  }

  fn init(force: bool) -> Result<(), Error> {
    let path = xdg::BaseDirectories::with_prefix("src")
      .context(error::BaseDirectoriesError)?
      .place_config_file(Config::filename())
      .context(error::ConfigPlace)?;

    if path.exists() && !force {
      return Err(Error::ConfigExists { path });
    }

    fs::write(&path, Config::default()).context(error::Io { path: &path })?;

    eprintln!("Successfully wrote default config to {}.", path.display());

    Ok(())
  }

  fn push(config: Config, remote: String, force: bool) -> Result<(), Error> {
    let src = Src::load(config.srcdir())?;

    if !force && src.is_dirty() {
      src.print_status();
      return Err(Error::PushDirty);
    }

    src.push_all(&remote)?;

    Ok(())
  }
}
