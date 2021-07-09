use std::path::PathBuf;
use std::process::Command;

pub type Certificate = PathBuf;
pub type CertificatePassword = String;
pub type Keychain = PathBuf;
pub type KeychainPassword = String;

const KEYCHAIN_NAME: &str = "MyKeychain.keychain";
const KEYCHAIN_PASSWORD: &str = "temporaryPassword";

pub struct Security {
    /// decoded .p12 certificate file
    certificate: Certificate,
    /// Certificate password
    certificate_password: CertificatePassword,
    /// A path to the .keychain file to be created
    keychain: Option<Keychain>,
}

impl Security {
    pub fn new(
        certificate: impl Into<Certificate>,
        certificate_password: CertificatePassword,
    ) -> Security {
        let keychain = if Self::keychain_file_path().exists() {
            Some(Self::keychain_file_path())
        } else {
            None
        };

        Self {
            certificate: certificate.into(),
            certificate_password,
            keychain,
        }
    }

    /// A path to the temporary keychain file
    fn keychain_file_path() -> PathBuf {
        std::env::current_dir().unwrap().join(KEYCHAIN_NAME)
    }

    fn keychain_password() -> &'static str {
        KEYCHAIN_PASSWORD
    }

    /// Ensure that the temporary keychain file is deleted
    pub fn delete_keychain(&mut self) {
        if let Some(ref keychain) = self.keychain {
            if !Command::new("security")
                .arg("delete-keychain")
                .arg(keychain)
                .status()
                .unwrap()
                .success()
            {
                panic!("Could not delete a .keychain file");
            }

            self.keychain = None;
        }
    }

    pub fn create_keychain(&mut self) {
        let keychain = Self::keychain_file_path();

        if !Command::new("security")
            .arg("create-keychain")
            .arg("-p")
            .arg(Self::keychain_password())
            .arg(&keychain)
            .status()
            .unwrap()
            .success()
        {
            panic!("Could not create a .keychain file");
        }

        self.keychain = Some(keychain)
    }
}
