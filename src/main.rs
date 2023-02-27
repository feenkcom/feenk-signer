use clap::{Parser, Subcommand};

use crate::android::AndroidSigner;
use crate::macos::MacSigner;

mod android;
mod macos;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Sign using Apple developer certificate
    Mac(MacSigner),
    /// Sign Android .apk using keystore
    Android(AndroidSigner),
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

fn main() {
    let options: Cli = Cli::parse();
    match options.command {
        Commands::Mac(signer) => signer.sign(),
        Commands::Android(signer) => signer.sign(),
    }
}
