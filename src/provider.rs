use crate::common::*;

const REMOTE_TEMPLATE: &str = "remote";

pub(crate) struct Provider {
  pub(crate) name: String,
  pub(crate) default_user: Option<String>,
  pub(crate) remote_template: Tera,
}

impl Provider {
  pub(crate) fn from_raw(name: &str, raw: raw::Provider) -> Result<Provider, Error> {
    let mut remote_template = Tera::default();
    remote_template
      .add_raw_template(REMOTE_TEMPLATE, &raw.remote_template)
      .context(error::Template { name })?;

    Ok(Provider {
      name: name.to_owned(),
      default_user: raw.default_user,
      remote_template,
    })
  }

  pub(crate) fn remote(&self, user: &str, project: &str) -> Result<String, Error> {
    let mut context = tera::Context::new();
    context.insert("user", user);
    context.insert("project", project);

    Ok(
      self
        .remote_template
        .render(REMOTE_TEMPLATE, &context)
        .context(error::Render {
          provider: &self.name,
        })?,
    )
  }
}
