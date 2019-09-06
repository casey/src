use crate::common::*;

const FILENAME: &str = "config.yaml";

const DEFAULT: &str = "\
default-provider: github
srcdir:           ~/src
tmpdir:           ~/tmp

providers:
  github:
    remote-template: git@github.com:{{user}}/{{project}}.git
  bitbucket:
    remote-template: git@bitbucket.org:{{user}}/{{project}}.git
";

pub(crate) struct Config {
  default_provider: Rc<Provider>,
  default_user: String,
  srcdir: PathBuf,
  tmpdir: PathBuf,
  providers: BTreeMap<String, Rc<Provider>>,
}

impl Config {
  pub(crate) fn default() -> &'static str {
    DEFAULT
  }

  pub(crate) fn filename() -> &'static str {
    FILENAME
  }

  pub(crate) fn load() -> Result<Config, Error> {
    let path = xdg::BaseDirectories::with_prefix("src")
      .context(error::BaseDirectoriesError)?
      .find_config_file(Self::filename());

    let raw = if let Some(path) = path {
      let path = &path;

      let text = fs::read_to_string(path).context(error::Io { path })?;

      serde_yaml::from_str(&text).context(error::Deserialize { path })?
    } else {
      serde_yaml::from_str(Self::default()).unwrap()
    };

    Self::from_raw(raw)
  }

  fn from_raw(raw: raw::Config) -> Result<Config, Error> {
    let mut providers = BTreeMap::new();
    for (name, provider) in raw.providers {
      let provider = Rc::new(Provider::from_raw(&name, provider)?);
      providers.insert(name, provider);
    }

    let default_provider = raw.default_provider;

    let default_user = if let Some(default_user) = raw.default_user {
      default_user
    } else {
      env::var("USER").context(error::User)?
    };

    Ok(Config {
      default_provider: providers
        .get(&default_provider)
        .ok_or_else(|| Error::DefaultProvider {
          name: default_provider.clone(),
        })?
        .clone(),
      srcdir: Config::expand_tilde(raw.srcdir)?,
      tmpdir: Config::expand_tilde(raw.tmpdir)?,
      default_user,
      providers,
    })
  }

  fn expand_tilde(path: PathBuf) -> Result<PathBuf, Error> {
    if path == Path::new("~") {
      dirs::home_dir().ok_or_else(|| Error::HomeDirectory)
    } else if path.starts_with("~") {
      Ok(
        dirs::home_dir()
          .ok_or_else(|| Error::HomeDirectory)?
          .join(path.strip_prefix("~").unwrap().to_path_buf()),
      )
    } else {
      Ok(path)
    }
  }

  pub(crate) fn srcdir(&self) -> &Path {
    &self.srcdir
  }

  pub(crate) fn tmpdir(&self) -> &Path {
    &self.tmpdir
  }

  fn user<'a>(&'a self, provider: &'a Provider) -> &'a str {
    provider.default_user.as_ref().unwrap_or(&self.default_user)
  }

  pub(crate) fn spec(&self, values: Vec<String>) -> Result<Spec, Error> {
    match values.as_slice() {
      [project] => Ok(Spec {
        provider: self.default_provider.clone(),
        user: self.user(&self.default_provider).to_owned(),
        project: project.clone(),
      }),
      [provider, project] => {
        let provider = self
          .providers
          .get(provider)
          .ok_or_else(|| Error::Provider {
            name: provider.clone(),
          })?;

        let user = self.user(provider).to_owned();

        Ok(Spec {
          project: project.clone(),
          provider: provider.clone(),
          user,
        })
      }
      [provider, user, project] => {
        let provider = self
          .providers
          .get(provider)
          .ok_or_else(|| Error::Provider {
            name: provider.clone(),
          })?;

        Ok(Spec {
          project: project.clone(),
          provider: provider.clone(),
          user: user.clone(),
        })
      }
      _ => Err(Error::internal(format!(
        "incorrect number of values for spec: {:?}",
        values
      ))),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn expand_tilde() {
    assert_eq!(
      Config::expand_tilde(PathBuf::from("~")).unwrap(),
      dirs::home_dir().unwrap()
    );
  }

  #[test]
  fn default() {
    serde_yaml::from_str::<raw::Config>(DEFAULT).unwrap();
  }
}
