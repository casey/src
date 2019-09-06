use crate::common::*;

use ansi_term::Colour::{Green, Red};

#[derive(Debug)]
pub(crate) struct Status {
  status: git2::Status,
}

impl Status {
  pub(crate) fn new(status: git2::Status) -> Status {
    Status { status }
  }
}

impl Display for Status {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    if self.status.is_conflicted() {
      write!(f, "{}", Red.paint("XX"))
    } else {
      let index = if self.status.is_index_new() {
        "A"
      } else if self.status.is_index_modified() {
        "M"
      } else if self.status.is_index_renamed() {
        "R"
      } else if self.status.is_index_typechange() {
        "T"
      } else if self.status.is_index_deleted() {
        "D"
      } else {
        " "
      };

      let worktree = if self.status.is_wt_new() {
        "A"
      } else if self.status.is_wt_modified() {
        "M"
      } else if self.status.is_wt_renamed() {
        "R"
      } else if self.status.is_wt_typechange() {
        "T"
      } else if self.status.is_wt_deleted() {
        "D"
      } else {
        " "
      };

      write!(f, "{}{}", Green.paint(index), Red.paint(worktree))
    }
  }
}
