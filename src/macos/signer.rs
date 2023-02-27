use std::path::{Path, PathBuf};

use clap::Args;
use rand::Rng;

use crate::macos::{Codesign, Security};

const FEENK_SIGNING_IDENTITY: &str = "Developer ID Application: feenk gmbh (77664ZXL29)";
const FEENK_ENTITLEMENTS: &str = include_str!("../../resources/Product.entitlements");

#[derive(Args, Debug, Clone)]
pub struct MacSigner {
    /// A folder (.app) or a .zip file that needs to be signed
    artefact: PathBuf,

    /// Signing identity
    #[clap(long, env = "SIGNING_IDENTITY", hide_env_values = true)]
    singing_identity: Option<String>,

    ///  File path to .entitlements file
    #[clap(long)]
    entitlements: Option<PathBuf>,

    /// Path to .p12 certificate
    #[clap(long, env = "CERT", hide_env_values = true)]
    certificate: PathBuf,

    /// Certificate password
    #[clap(long, env = "CERT_PASSWORD", hide_env_values = true)]
    password: Option<String>,
}

impl MacSigner {
    pub fn sign(&self) {
        let mut security = Security::new(&self.certificate, self.password.clone());

        security.delete_keychain();
        security.create_keychain();

        security.add_keychain_to_user_domain();
        security.set_keychain_settings();
        security.unlock_keychain();
        security.import_keychain();

        security.set_key_partition_list();

        self.with_signing_identity_and_entitlements(|signing_identity, entitlements| {
            let codesign = Codesign::new(signing_identity, entitlements);
            codesign.sign(self.artefact.as_path());
        });

        security.delete_keychain();
    }

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
