use std::process;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Configuration {
    pub host: String,
    pub port: u16,
}

fn main() {
    let files = ["examples/foo.json", "~/.config/foo.json", "/etc/foo.json"];
    let (config, file) = match luci::read_jsonc::<Configuration, _>(files.iter()) {
        Ok(v) => v,
        Err(e) => {
            println!("Can not read configuration file.\n{}", e);
            process::exit(1);
        }
    };
    println!("Read configuration from {}", file.to_string_lossy());
    println!("> host={}", config.host);
    println!("> port={}", config.port);
}
