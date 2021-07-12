
use std::process::{Command, Stdio, Output};
use std::path::PathBuf;

pub struct Codesign {
    signing_identity: String,
    entitlements: String
}
pub const KEYCHAIN_NAME: &str = "MyNewKeychain.keychain";
impl Codesign {
    pub fn new(
        signing_identity: impl Into<String>,
        entitlements: impl Into<String>) -> Codesign {
        Self {
            signing_identity: signing_identity.into(),
            entitlements: entitlements.into()
        }
    }
    fn keychain_file_path() -> PathBuf {
        std::env::current_dir().unwrap().join(KEYCHAIN_NAME)
    }

    pub fn sign(&mut self, file_path:String) {
        let keychain = Self::keychain_file_path();

        if !Command::new("codesign")
            .arg("--entitlements")
            .arg(&self.entitlements)
            .arg("--force")
            .arg("-v")
            .arg("--options=runtime")
            .arg("--deep")
            .arg("--timestamp")
            .arg("--file-list")
            .arg("-")
            .arg("-s")
            .arg(&self.signing_identity)
            .arg(file_path)
            .status()
            .unwrap()
            .success()
        {
            panic!("Could not codesign");
        }
    }
}