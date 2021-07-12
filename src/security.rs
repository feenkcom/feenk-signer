use execute::{command, Execute};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::str;

pub type Certificate = PathBuf;
pub type CertificatePassword = Option<String>;
pub type Keychain = PathBuf;
pub type KeychainPassword = String;

pub const KEYCHAIN_NAME: &str = "MyNewKeychain.keychain";
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
            let mut command = command(format!("security delete-keychain {}", keychain.display()));

            println!("{:?}", &command);

            if !command.status().unwrap().success() {
                panic!("Could not delete a .keychain file");
            }

            self.keychain = None;
        }
    }

    pub fn create_keychain(&mut self) {
        let keychain = Self::keychain_file_path();

        let mut command = command(format!(
            "security create-keychain -p {} {}",
            Self::keychain_password(),
            &keychain.display()
        ));
        println!("{:?}", &command);

        if !command.status().unwrap().success() {
            panic!("Could not create the keychain");
        }

        self.keychain = Some(keychain)
    }
    //security list-keychains -d user
    pub fn list_keychains(&mut self) -> String {
        let mut command = command("security list-keychains -d user");
        println!("{:?}", &command);

        let output = match command.output() {
            Ok(x) => {
                let output = String::from_utf8_lossy(&x.stdout);
                println!("output = {}", &output);
                output
                    .replace("\"", "")
                    .split_whitespace()
                    .nth(0)
                    .unwrap_or("")
                    .to_string()
            }
            Err(_) => panic!("Could not list keychains"),
        };
        output
    }
    //security list-keychains -d user -s "$MY_KEYCHAIN" $(security list-keychains -d user | sed s/\"//g) # Append temp keychain to the user domain
    pub fn add_keychain_to_user_domain(&mut self) {
        let mut command = command(format!(
            "security list-keychains -d user -s {} {}",
            Self::keychain_file_path().display(),
            self.list_keychains()
        ));
        println!("{:?}", &command);

        if !command.status().unwrap().success() {
            panic!("Could add temporary keychain");
        }
    }

    //security set-keychain-settings "$MY_KEYCHAIN" # Remove relock timeout
    pub fn set_keychain_settings(&mut self) {
        let mut command = command(format!(
            "security set-keychain-settings {}",
            Self::keychain_file_path().display()
        ));
        println!("{:?}", &command);
        if !command.status().unwrap().success() {
            panic!("Could not set keychain settings");
        }
    }

    //security unlock-keychain -p "$MY_KEYCHAIN_PASSWORD" "$MY_KEYCHAIN" # Unlock keychain
    pub fn unlock_keychain(&mut self) {
        let mut command = command(format!(
            "security unlock-keychain -p {} {}",
            Self::keychain_password(),
            Self::keychain_file_path().display(),
        ));
        println!("{:?}", &command);
        if !command.status().unwrap().success() {
            panic!("Could not unlock the keychain");
        }
    }
    //security import $CERT -k "$MY_KEYCHAIN" -P "$CERT_PASSWORD" -T "/usr/bin/codesign" # Add certificate to keychain
    pub fn import_keychain(&mut self) {
        let keychain = Self::keychain_file_path();

        assert!(
            keychain.exists(),
            "Keychain file must exist {}",
            &keychain.display()
        );
        assert!(
            &self.certificate.exists(),
            "Certificate file must exist {}",
            &self.certificate.display()
        );

        let mut command = command(format!(
            "security import {} -k {} -P '{}' -T /usr/bin/codesign",
            &self.certificate.display(),
            Self::keychain_file_path().display(),
            &self
                .certificate_password
                .as_ref()
                .unwrap_or(&"".to_string())
        ));
        println!("{:?}", &command);

        if !command.status().unwrap().success() {
            panic!("Could not import the certificate");
        }
    }

    //security find-identity -v -p codesigning "$MY_KEYCHAIN" | head -1 | grep '"' | sed -e 's/[^"]*"//' -e 's/".*//'
    pub fn find_identity(&mut self) -> String {
        let mut command = command(format!(
            "security find-identity -v -p codesigning {}",
            Self::keychain_file_path().display(),
        ));
        println!("{:?}", &command);

        let output = match command.output() {
            Ok(x) => {
                let output = String::from_utf8_lossy(&x.stdout);
                println!("output = {}", &output);
                output
                    .to_string()
                    .lines()
                    .next()
                    .unwrap()
                    .to_string()
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .to_string()
            }
            Err(_) => panic!("Could find identity"),
        };
        output
    }

    //security set-key-partition-list -S apple-tool:,apple: -s -k $MY_KEYCHAIN_PASSWORD -D "$CERT_IDENTITY" -t private $MY_KEYCHAIN # Enable codesigning from a non user interactive shell
    pub fn set_key_partition_list(&mut self) {
        let keychain = Self::keychain_file_path();

        if !Command::new("security")
            .arg("set-key-partition-list")
            .arg("-S")
            .arg("apple-tool:,apple:")
            .arg("-s")
            .arg("-k")
            .arg(
                self.certificate_password
                    .as_ref()
                    .unwrap_or(&"".to_string()),
            )
            .arg("-D")
            .arg(&self.find_identity())
            .arg("-t")
            .arg("private")
            .arg(&keychain)
            // .arg("-P")
            // .arg("")
            .status()
            .unwrap()
            .success()
        {
            panic!("Could not set key partition list");
        }
    }
}
