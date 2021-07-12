use execute::{command, Execute};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

pub struct Codesign {
    signing_identity: String,
    entitlements: PathBuf,
}

impl Codesign {
    pub fn new(signing_identity: impl Into<String>, entitlements: impl Into<PathBuf>) -> Codesign {
        Self {
            signing_identity: signing_identity.into(),
            entitlements: entitlements.into(),
        }
    }

    pub fn sign(&mut self, file: &Path) {
        let mut command = command(format!(
            "codesign --entitlements {}  --force -v --options=runtime  --deep --timestamp --file-list - -s '{}' {}",
            &self.entitlements.display(),
            &self.signing_identity,
            file.display()
        ));
        println!("{:?}", &command);

        if !command.status().unwrap().success() {
            panic!("Could not codesign");
        }
    }
}
