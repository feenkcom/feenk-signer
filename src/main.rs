extern crate base64;
extern crate clap;

mod options;
mod security;

use std::env;
use std::fs;
use std::process::Command;

use crate::options::SignOptions;

use crate::security::Security;
use clap::Clap;
use std::path::Path;

fn main() {
    let options: SignOptions = SignOptions::parse();

    let decoded_cert = base64::decode(&options.certificate).expect("Could not decode certificate");

    let certificate = Path::new("decoded.p12").to_path_buf();

    fs::write(&certificate, decoded_cert).expect("Could not export decoded certificate");

    let mut security = Security::new(&certificate, options.password);
    security.delete_keychain();
    security.create_keychain();


    // let env_variables: [&str; 3] = ["CERT", "MY_KEYCHAIN", "MY_KEYCHAIN_PASSWORD"];
    //security delete-keychain "$MY_KEYCHAIN" "Delete also initially"
    // let my_keychain = env::var("MY_KEYCHAIN").unwrap();

    // let status = Command::new("security")
    //     .arg("delete-keychain")
    //     .arg(&my_keychain)
    //     .arg("Delete also initially")
    //     .status()
    //     .unwrap();
    //
    // if !status.success() {
    //     panic!("Could not delete keychain {}", &my_keychain);
    // }
}
