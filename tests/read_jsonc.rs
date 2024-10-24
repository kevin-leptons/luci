use luci::result::Error;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(deny_unknown_fields)]
struct Configuration {
    pub host: String,
    pub port: u16,
}

#[test]
fn valid() {
    let file = "tests/data/valid.json";
    let (config, readed_file) = luci::read_jsonc::<Configuration, _>([file].iter()).unwrap();
    assert_eq!(
        config,
        Configuration {
            host: "127.0.0.1".into(),
            port: 8000
        }
    );
    assert_eq!(readed_file.to_string_lossy(), file);
}

#[test]
fn not_found() {
    let file = "tests/data/not_existed.json";
    let error = luci::read_jsonc::<Configuration, _>([file].iter()).unwrap_err();
    let matched_error = matches!(error, Error::NotFound);
    assert!(matched_error);
}

#[test]
fn unknown_field() {
    let file = "tests/data/unknown_field.json";
    let error = luci::read_jsonc::<Configuration, _>([file].iter()).unwrap_err();
    let matched_error = matches!(error, Error::Deserialization(_, _));
    assert!(matched_error);
}

#[test]
fn missing_field() {
    let file = "tests/data/missing_field.json";
    let error = luci::read_jsonc::<Configuration, _>([file].iter()).unwrap_err();
    let matched_error = matches!(error, Error::Deserialization(_, _));
    assert!(matched_error);
}

#[test]
fn malformed_format() {
    let file = "tests/data/malformed_jsonc.json";
    let error = luci::read_jsonc::<Configuration, _>([file].iter()).unwrap_err();
    let matched_error = matches!(error, Error::Deserialization(_, _));
    assert!(matched_error);
}
