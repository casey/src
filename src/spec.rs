use crate::common::*;

pub(crate) struct Spec {
  pub(crate) provider: Rc<Provider>,
  pub(crate) user: String,
  pub(crate) project: String,
}

impl Spec {
  pub(crate) fn remote(&self) -> Result<String, Error> {
    self.provider.remote(&self.user, &self.project)
  }
}
