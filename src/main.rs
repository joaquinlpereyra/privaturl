use clap::Parser;

use std::fs::{read_to_string, write};
use std::path::PathBuf;
use base64::{self, Engine};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, 
};

const TEMPLATE_HTML: &str = include_str!("../template/index.html");

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, about, version)]
#[command(name = "privaturl")]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    data: PathBuf,

    #[arg(short, long, default_value = "private.html")]
    output: PathBuf,

    #[arg(short, long, help = "[optional] Use an alternative template. See the README for details.")]
    alt_template: Option<PathBuf>,
}

fn encrypt(data: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); 
    let ciphertext = match cipher.encrypt(&nonce, data) {
        Ok(data) => data,
        Err(err) => {
            panic!("{}", err);
        }
    };
    (ciphertext, nonce.to_vec(), key.to_vec())
}

fn base64(data: &[u8]) -> String {
    let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    engine.encode(data)
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let embedded = read_to_string(&args.data)?.into_bytes();
    let (encrypted, iv, key) = encrypt(&embedded);
    let (encoded_site, encoded_iv, encoded_key) = (base64(&encrypted), base64(&iv), base64(&key));
    let mut html_template = match args.alt_template {
        Some(alt) => read_to_string(&alt)?, 
        None => String::from(TEMPLATE_HTML)
    };

    html_template = html_template.replace("---PLACEHOLDER-DATA---", &encoded_site);
    html_template = html_template.replace("---PLACEHOLDER-IV---", &encoded_iv);
    write(&args.output, html_template)?;

    println!("Your key is: {}", encoded_key);
    println!("Your output HTML is at: {}", args.output.to_string_lossy());

    Ok(())
}
