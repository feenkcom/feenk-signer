use std::env;
use std::process::Command;

fn main() {

    // let env_variables: [&str; 3] = ["CERT", "MY_KEYCHAIN", "MY_KEYCHAIN_PASSWORD"];
    //security delete-keychain "$MY_KEYCHAIN" "Delete also initially"
    let my_keychain = env::var("MY_KEYCHAIN").unwrap();
    let mut command = Command::new("security");
    command.arg("delete-keychain")
        .arg(&my_keychain)
        .arg("Delete also initially");
    let status = command.status().unwrap();

    if !status.success() {
        panic!("Could not delete keychain {}", &my_keychain);
    }
}
