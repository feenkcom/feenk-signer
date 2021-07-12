extern crate base64;
extern crate clap;
extern crate execute;

mod codesign;
mod options;
mod security;

use std::fs;
use std::process::Command;
use std::{env, panic};

use crate::options::SignOptions;

use crate::security::Security;
use clap::Clap;
use std::path::Path;

use crate::codesign::Codesign;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let options: SignOptions = SignOptions::parse();

    let decoded_cert = base64::decode(&options.certificate).expect("Could not decode certificate");
    let certificate = Path::new("decoded.p12").to_path_buf();
    fs::write(&certificate, decoded_cert).expect("Could not export decoded certificate");

    let mut security = Security::new(&certificate, options.password);

    security.delete_keychain();
    security.create_keychain();

    security.add_keychain_to_user_domain();
    security.set_keychain_settings();
    security.unlock_keychain();
    security.import_keychain();

    security.set_key_partition_list();

    let mut codesign = Codesign::new(options.singing_identity, options.entitlements);
    codesign.sign(options.artefact.as_path());

    security.delete_keychain();
}
