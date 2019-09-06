use crate::common::*;

#[derive(Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub(crate) struct Config {
  #[serde(rename = "default-provider")]
  pub(crate) default_provider: String,
  #[serde(rename = "default-user")]
  pub(crate) default_user: Option<String>,
  pub(crate) srcdir: PathBuf,
  pub(crate) tmpdir: PathBuf,
  pub(crate) providers: BTreeMap<String, raw::Provider>,
}

#[derive(Deserialize)]
pub(crate) struct Provider {
  #[serde(rename = "default-user")]
  pub(crate) default_user: Option<String>,
  #[serde(rename = "remote-template")]
  pub(crate) remote_template: String,
}
