use crate::common::*;

use Opt::*;

#[derive(StructOpt)]
pub(crate) enum Opt {
  Status,
  Get {
    #[structopt(long = "tmp")]
    tmp: bool,
    #[structopt(min_values = 1, max_values = 3)]
    spec: Vec<String>,
  },
  Remote {
    #[structopt(min_values = 1, max_values = 3)]
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
}

impl Opt {
  pub(crate) fn run(self) -> Result<(), Error> {
    let config = Config::load()?;

    match self {
      Status => Self::status(config),
      Remote { spec } => Self::remote(config, spec),
      Get { tmp, spec } => Self::get(config, tmp, spec),
      Init { force } => Self::init(force),
      Push { force, remote } => Self::push(config, remote, force),
    }
  }

  fn status(config: Config) -> Result<(), Error> {
    let src = Src::load(config.srcdir())?;

    src.print_status();

    Ok(())
  }

  fn remote(config: Config, spec: Vec<String>) -> Result<(), Error> {
    let spec = config.spec(spec)?;

    eprintln!("{}", spec.remote()?);

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
