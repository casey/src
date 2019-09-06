use crate::common::*;

pub(crate) struct Repo {
  #[allow(unused)]
  repo: git2::Repository,
  name: String,
  state: git2::RepositoryState,
  files: BTreeMap<String, Status>,
  head: String,
}

impl Repo {
  pub(crate) fn new(path: &Path) -> Result<Repo, Error> {
    let context = error::Git { path };

    let repo = git2::Repository::open(path).context(context)?;

    let context = error::Git { path };

    let name = repo
      .workdir()
      .unwrap_or_else(|| repo.path())
      .file_name()
      .unwrap()
      .to_string_lossy()
      .into_owned();

    let state = repo.state();

    let mut status_options = git2::StatusOptions::new();
    status_options.include_ignored(false);
    status_options.include_untracked(true);

    let files = repo
      .statuses(Some(&mut status_options))
      .context(context)?
      .iter()
      .map(|status_entry| {
        (
          String::from_utf8_lossy(status_entry.path_bytes()).into_owned(),
          Status::new(status_entry.status()),
        )
      })
      .collect();

    fn head(repo: &git2::Repository) -> Result<String, git2::Error> {
      for branch in repo.branches(Some(git2::BranchType::Local))? {
        let (branch, _) = branch?;

        if branch.is_head() {
          return Ok(String::from_utf8_lossy(branch.name_bytes()?).into_owned());
        }
      }

      Ok(repo.head()?.peel_to_commit()?.id().to_string())
    }

    let head = head(&repo).context(context)?;

    Ok(Repo {
      repo,
      name,
      state,
      files,
      head,
    })
  }

  pub(crate) fn clone(provider: &str, url: &str, into: &Path) -> Result<Repo, Error> {
    let command: Vec<OsString> = vec![
      "git".into(),
      "clone".into(),
      "--origin".into(),
      provider.into(),
      url.into(),
      into.into(),
    ];

    let status = Command::new(&command[0])
      .args(&command[1..])
      .status()
      .context(error::CommandInvocation {
        command: command.clone(),
      })?;

    if !status.success() {
      return Err(Error::CommandStatus { command, status });
    }

    Self::new(into)
  }

  pub(crate) fn load_dir(path: &Path) -> Result<Vec<Repo>, Error> {
    let mut repos = Vec::new();

    for result in fs::read_dir(path).context(error::Io { path })? {
      let entry = result.context(error::Io { path })?;

      if entry.file_name() == ".DS_Store" {
        continue;
      }

      let path = entry.path();

      repos.push(Repo::new(&path)?);
    }

    Ok(repos)
  }

  pub(crate) fn name(&self) -> &str {
    &self.name
  }

  pub(crate) fn head(&self) -> &str {
    &self.head
  }

  pub(crate) fn files(&self) -> impl Iterator<Item = (&String, &Status)> {
    self.files.iter()
  }

  pub(crate) fn changes(&self) -> usize {
    self.files().count()
  }

  pub(crate) fn is_dirty(&self) -> bool {
    self.state != git2::RepositoryState::Clean || self.changes() > 0
  }

  pub(crate) fn state(&self) -> &'static str {
    use git2::RepositoryState::*;

    match self.state {
      Clean => "clean",
      Merge => "merge",
      Revert => "revert",
      RevertSequence => "revert sequence",
      CherryPick => "cherry pick",
      CherryPickSequence => "cherry pick sequence",
      Bisect => "bisect",
      Rebase => "rebase",
      RebaseInteractive => "rebase interactive",
      RebaseMerge => "rebase merge",
      ApplyMailbox => "apply mailbox",
      ApplyMailboxOrRebase => "apply mailbox or rebase",
    }
  }
}
