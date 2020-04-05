use crate::common::*;

pub(crate) struct Src {
  repos: Vec<Repo>,
}

impl Src {
  pub(crate) fn load(path: &Path) -> Result<Src, Error> {
    let mut repos = Vec::new();

    let style = ProgressStyle::default_spinner().template("🧿  {spinner} {msg}");

    let spinner = ProgressBar::new_spinner();
    spinner.set_message(&format!("Searching {} for repositories...", path.display()));
    spinner.set_style(style);

    for result in fs::read_dir(path).context(error::Io { path })? {
      spinner.tick();

      let entry = result.context(error::Io { path })?;

      if entry.file_name() == ".DS_Store" {
        continue;
      }

      let path = entry.path();

      repos.push(Repo::new(&path)?);
    }

    let style = ProgressStyle::default_spinner().template("🧿  {msg}");
    spinner.set_style(style);

    spinner.finish_with_message(&format!("Found {} repositories.", repos.len()));

    Ok(Src { repos })
  }

  pub(crate) fn is_dirty(&self) -> bool {
    self.repos.iter().any(Repo::is_dirty)
  }

  pub(crate) fn print_status(&self) {
    for (i, repo) in self.repos.iter().filter(|repo| repo.is_dirty()).enumerate() {
      if i > 0 {
        eprintln!();
      }

      eprint!("{}", repo.name());

      if repo.head() != "master" {
        eprint!("@{}", repo.head());
      }

      if repo.state() != "clean" {
        eprint!(" {}", repo.state());
      }

      eprintln!(":");

      for (path, status) in repo.files() {
        eprintln!("{} {}", status, path);
      }
    }
  }

  pub(crate) fn push_all(&self, remote: &str) -> Result<(), Error> {
    let style = ProgressStyle::default_bar().template("Pushing: {wide_bar} {pos}/{len}");

    let bar = ProgressBar::new(self.repos.len() as u64);
    bar.set_style(style);

    let errors = self
      .repos
      .par_iter()
      .flat_map(|repo| {
        let result = repo.push(remote);

        bar.inc(1);

        result.err().map(|err| (repo.name(), err))
      })
      .collect::<Vec<(&str, Error)>>();

    bar.finish();

    if errors.is_empty() {
      eprintln!("Successfully pushed all {} repositories!", self.repos.len());
      Ok(())
    } else {
      for (name, error) in errors {
        eprintln!("Failed to push `{}`: {}", name, error);
      }
      Err(Error::PushAll {
        remote: remote.to_owned(),
      })
    }
  }

  pub(crate) fn all(&self, command: &[String]) -> Result<(), Error> {
    for repo in &self.repos {
      repo.run(command.iter().map(OsString::from).collect())?;
    }

    Ok(())
  }
}
