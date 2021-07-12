use std::path::PathBuf;
use std::process::{Command, Stdio, Output};
use std::str;

pub type Certificate = PathBuf;
pub type CertificatePassword = Option<String>;
pub type Keychain = PathBuf;
pub type KeychainPassword = String;

pub const KEYCHAIN_NAME: &str = "MyKeychain.keychain";
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
    //security list-keychains -d user
    pub fn list_keychains(&mut self) -> String {
        let keychain = Self::keychain_file_path();
        let output= match Command::new("security")
            .arg("list-keychains")
            .arg("-d")
            .arg("user")
            .stdout(Stdio::piped())
            .output() {
            Ok(x) => str::from_utf8(x.stdout.as_slice()).unwrap().to_string().replace("\"", ""),
            Err(_) => panic!("Could not list keychains"),
        };
        output
    }
    //security list-keychains -d user -s "$MY_KEYCHAIN" $(security list-keychains -d user | sed s/\"//g) # Append temp keychain to the user domain
    pub fn add_keychain_to_user_domain(&mut self) {
        let keychain = Self::keychain_file_path();

        if !Command::new("security")
            .arg("list-keychains")
            .arg("-d")
            .arg("user")
            .arg("-s")
            .arg(&keychain)
            .arg(self.list_keychains())
            .status()
            .unwrap()
            .success()
        {
            panic!("Could add temporary keychain");
        }

        self.keychain = Some(keychain)
    }

    //security set-keychain-settings "$MY_KEYCHAIN" # Remove relock timeout
    pub fn set_keychain_settings(&mut self) {
        let keychain = Self::keychain_file_path();

        if !Command::new("security")
            .arg("set-keychain-settings")
            .arg(&keychain)
            .status()
            .unwrap()
            .success()
        {
            panic!("Could not create a .keychain file");
        }

        self.keychain = Some(keychain)
    }

    //security unlock-keychain -p "$MY_KEYCHAIN_PASSWORD" "$MY_KEYCHAIN" # Unlock keychain
    pub fn unlock_keychain(&mut self) {
        let keychain = Self::keychain_file_path();

        if !Command::new("security")
            .arg("unlock-keychain")
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
    //security import $CERT -k "$MY_KEYCHAIN" -P "$CERT_PASSWORD" -T "/usr/bin/codesign" # Add certificate to keychain
    pub fn import_keychain(&mut self) {
        let keychain = Self::keychain_file_path();

        if !Command::new("security")
            .arg("import")

            .arg(&self.certificate)
            .arg("-k")
            .arg(&keychain)
            .arg("-T")
            .arg("/usr/bin/codesign")
            .status()
            .unwrap()
            .success()
        {
            panic!("Could not create a .keychain file");
        }

        self.keychain = Some(keychain)
    }

    //security find-identity -v -p codesigning "$MY_KEYCHAIN" | head -1 | grep '"' | sed -e 's/[^"]*"//' -e 's/".*//'
    pub fn find_identity(&mut self) -> String {
        let keychain = Self::keychain_file_path();
        let output= match Command::new("security")
            .arg("find-identity")
            .arg("-v")
            .arg("-p")
            .arg("codesigning")
            .arg(&keychain)
            .stdout(Stdio::piped())
            .output() {
            Ok(x) => str::from_utf8(x.stdout.as_slice()).unwrap().to_string().lines().next().unwrap().to_string().split_whitespace().nth(1).unwrap().to_string(),
            Err(_) => panic!("Could not list keychains"),
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
            .arg(self.certificate_password.as_ref().unwrap_or(&"".to_string()))
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
            panic!("Could not create a .keychain file");
        }
    }
}
