use std::path::PathBuf;

use clap::Args;
use ndk_build::ndk::Ndk;

macro_rules! bat {
    ($bat:expr) => {
        if cfg!(target_os = "windows") {
            concat!($bat, ".bat")
        } else {
            $bat
        }
    };
}

#[derive(Args, Debug, Clone)]
pub struct AndroidSigner {
    /// An .apk file that needs to be signed
    artefact: PathBuf,
    /// Keystore file
    #[clap(long, env = "CARGO_APK_RELEASE_KEYSTORE", hide_env_values = true)]
    keystore: PathBuf,
    /// Keystore password
    #[clap(
        long,
        env = "CARGO_APK_RELEASE_KEYSTORE_PASSWORD",
        hide_env_values = true
    )]
    password: String,
}

impl AndroidSigner {
    pub fn sign(&self) {
        let ndk = Ndk::from_env().unwrap();

        let mut apksigner = ndk.build_tool(bat!("apksigner")).unwrap();
        apksigner
            .arg("sign")
            .arg("--ks")
            .arg(&self.keystore)
            .arg("--ks-pass")
            .arg(format!("pass:{}", &self.password))
            .arg(self.artefact.as_path());

        if !apksigner.status().unwrap().success() {
            panic!(
                "Failed to sign an apk: {}",
                self.artefact.as_path().display()
            )
        }
    }
}
