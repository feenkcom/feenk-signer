use clap::{AppSettings, Clap};
use rand::Rng;
use std::path::{Path, PathBuf};

const FEENK_SIGNING_IDENTITY: &str = "Developer ID Application: feenk gmbh (77664ZXL29)";
const FEENK_ENTITLEMENTS: &str = include_str!("../resources/Product.entitlements");

#[derive(Clap, Clone, Debug, Default)]
#[clap(version = "1.0", author = "feenk gmbh <contact@feenk.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct SignOptions {
    /// A folder (.app) or a file that needs to be signed
    #[clap(parse(from_os_str))]
    pub(crate) artefact: PathBuf,

    /// Base64 encoded certificate
    #[clap(long, env = "CERT", hide_env_values = true)]
    pub(crate) certificate: String,

    /// Certificate password
    #[clap(long, env = "CERT_PASSWORD", hide_env_values = true)]
    pub(crate) password: Option<String>,

    /// Signing identity
    #[clap(long, env = "SIGNING_IDENTITY", hide_env_values = true)]
    singing_identity: Option<String>,

    ///  File path to .entitlements file
    #[clap(long, parse(from_os_str))]
    entitlements: Option<PathBuf>,
}

impl SignOptions {
    fn singing_identity(&self) -> &str {
        self.singing_identity
            .as_ref()
            .map_or(FEENK_SIGNING_IDENTITY, |identity| identity.as_str())
    }

    pub fn with_signing_identity_and_entitlements(&self, block: impl FnOnce(&str, &Path)) {
        if let Some(ref entitlements) = self.entitlements {
            block(self.singing_identity(), entitlements.as_path())
        } else {
            let mut rng = rand::thread_rng();
            let mut file = PathBuf::from("Product.entitlement");
            while file.exists() {
                let suffix: u16 = rng.gen();
                file = PathBuf::from(format!("Product-{}.entitlement", suffix));
            }

            std::fs::write(&file, FEENK_ENTITLEMENTS)
                .expect(&format!("Could not write {:?}", &file));
            block(self.singing_identity(), &file.as_path());
            if file.exists() {
                std::fs::remove_file(&file).expect(&format!("Could not remove {:?}", &file));
            }
        }
    }
}
