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
}

impl Opt {
  pub(crate) fn run(self) -> Result<(), Error> {
    let config = Config::load()?;

    match self {
      Status => Self::status(config),
      Remote { spec } => Self::remote(config, spec),
      Get { tmp, spec } => Self::get(config, tmp, spec),
      Init { force } => Self::init(force),
    }
  }

  fn status(config: Config) -> Result<(), Error> {
    let repos = Repo::load_dir(config.srcdir())?;

    for (i, repo) in repos.into_iter().filter(Repo::is_dirty).enumerate() {
      if i > 0 {
        println!();
      }

      print!("{}", repo.name());

      if repo.head() != "master" {
        print!("@{}", repo.head());
      }

      if repo.state() != "clean" {
        print!(" {}", repo.state());
      }

      println!(":");

      for (path, status) in repo.files() {
        println!("{} {}", status, path);
      }
    }

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

    println!("Successfully wrote default config to {}.", path.display());

    Ok(())
  }
}
