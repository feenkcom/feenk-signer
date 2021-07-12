use clap::{AppSettings, ArgEnum, Clap};
use std::path::PathBuf;

#[derive(Clap, Clone, Debug, Default)]
#[clap(version = "1.0", author = "feenk gmbh <contact@feenk.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct SignOptions {
    /// Base64 encoded certificate
    #[clap(long, env = "CERT", hide_env_values = true)]
    pub(crate) certificate: String,

    /// Certificate password
    #[clap(long, env = "CERT_PASSWORD", hide_env_values = true)]
    pub(crate) password: Option<String>,

    /// Signing identity
    #[clap(long, env = "SIGNING_IDENTITY")]
    pub(crate) singing_identity: String,

    /// The app folder that needs to be signed
    #[clap(long)]
    pub(crate) app: String,

    ///  File path to .entitlements file
    #[clap(long)]
    pub(crate) entitlements: String

}