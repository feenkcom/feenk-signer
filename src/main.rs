extern crate base64;
extern crate clap;
extern crate execute;
extern crate rand;

mod codesign;
mod options;
mod security;

use std::fs;

use crate::options::SignOptions;

use crate::security::Security;
use clap::Clap;
use std::path::Path;

use crate::codesign::Codesign;

fn main() {
    let options: SignOptions = SignOptions::parse();

    let decoded_cert = base64::decode(&options.certificate).expect("Could not decode certificate");
    let certificate = Path::new("decoded.p12").to_path_buf();
    fs::write(&certificate, decoded_cert).expect("Could not export decoded certificate");

    let mut security = Security::new(&certificate, options.password.clone());

    security.delete_keychain();
    security.create_keychain();

    security.add_keychain_to_user_domain();
    security.set_keychain_settings();
    security.unlock_keychain();
    security.import_keychain();

    security.set_key_partition_list();

    options.with_signing_identity_and_entitlements(|signing_identity, entitlements| {
        let codesign = Codesign::new(signing_identity, entitlements);
        codesign.sign(options.artefact.as_path());
    });

    security.delete_keychain();
}
