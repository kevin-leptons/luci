use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::str::FromStr;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Configuration {
    pub host: String,
    pub port: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = prepare_example_file()?;
    let files = [file];
    let (config, file) = luci::read_jsonc::<Configuration, _>(files.iter())?;
    println!("Read configuration from {}", file.to_string_lossy());
    println!("> host: {}", config.host);
    println!("> port: {}", config.port);
    Ok(())
}

fn prepare_example_file() -> Result<PathBuf, Box<dyn Error>> {
    let content = r#"
    // A configuration file.

    {
        "host": "127.0.0.1",
        "port": 8086
    }
    "#;
    let path = PathBuf::from_str("/tmp/luci_example.json")?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(&path)?;
    std::fs::write(&path, content)?;
    file.write(content.as_bytes())?;
    println!("Write configuration file to {}", path.to_string_lossy());
    Ok(path)
}
