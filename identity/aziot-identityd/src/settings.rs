// Copyright (c) Microsoft. All rights reserved.

use crate::error::{Error, ErrorKind, InitializeErrorReason};
use config::{Config, File, FileFormat};
use failure::{Context, Fail};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;
use std::path::Path;
// use std::path::{Path, PathBuf};
use std::str::FromStr;
use url::Url;
use url_serde;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Settings {
    connect: Connect,
    listen: Listen,
}

impl Settings {
    pub fn new(filename: &Path) -> Result<Self, LoadSettingsError> {
        let mut config = Config::default();
        config.merge(File::new(
            filename
                .to_str()
                .ok_or(ErrorKind::Initialize(InitializeErrorReason::LoadSettings))?,
            FileFormat::Toml,
        ))?;

        let settings = config.try_into()?;

        Ok(settings)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Connect {
    #[serde(with = "url_serde")]
    api_uri: Url,
}

impl Connect {
    pub fn api_uri(&self) -> &Url {
        &self.api_uri
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Listen {
    #[serde(with = "url_serde")]
    api_uri: Url,
    #[serde(default = "Protocol::default")]
    min_tls_version: Protocol,
}

impl Listen {
    pub fn api_uri(&self) -> &Url {
        &self.api_uri
    }

    pub fn min_tls_version(&self) -> Protocol {
        self.min_tls_version
    }
}

//TODO: Keeping this setting around until it is determined HTTPS isn't supported
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Protocol {
    Tls10,
    Tls11,
    Tls12,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::Tls10
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Tls10 => write!(f, "TLS 1.0"),
            Protocol::Tls11 => write!(f, "TLS 1.1"),
            Protocol::Tls12 => write!(f, "TLS 1.2"),
        }
    }
}

impl FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "tls" | "tls1" | "tls10" | "tls1.0" | "tls1_0" | "tlsv10" => Ok(Protocol::Tls10),
            "tls11" | "tls1.1" | "tls1_1" | "tlsv11" => Ok(Protocol::Tls11),
            "tls12" | "tls1.2" | "tls1_2" | "tlsv12" => Ok(Protocol::Tls12),
            _ => Err(format!("Unsupported TLS protocol version: {}", s)),
        }
    }
}

impl<'de> Deserialize<'de> for Protocol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(de::Error::custom)
    }
}

impl Serialize for Protocol {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

#[derive(Debug, Fail)]
#[fail(display = "Could not load settings")]
pub struct LoadSettingsError(#[cause] Context<Box<dyn std::fmt::Display + Send + Sync>>);

impl From<std::io::Error> for LoadSettingsError {
    fn from(err: std::io::Error) -> Self {
        LoadSettingsError(Context::new(Box::new(err)))
    }
}

impl From<config::ConfigError> for LoadSettingsError {
    fn from(err: config::ConfigError) -> Self {
        LoadSettingsError(Context::new(Box::new(err)))
    }
}

impl From<serde_json::Error> for LoadSettingsError {
    fn from(err: serde_json::Error) -> Self {
        LoadSettingsError(Context::new(Box::new(err)))
    }
}

impl From<Error> for LoadSettingsError {
    fn from(err: Error) -> Self {
        LoadSettingsError(Context::new(Box::new(err)))
    }
}

impl From<Context<ErrorKind>> for LoadSettingsError {
    fn from(inner: Context<ErrorKind>) -> Self {
        From::from(Error::from(inner))
    }
}

impl From<ErrorKind> for LoadSettingsError {
    fn from(kind: ErrorKind) -> Self {
        From::from(Error::from(kind))
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("tls", Protocol::Tls10; "when tls provided")]
    #[test_case("tls1", Protocol::Tls10; "when tls1 with dot provided")]
    #[test_case("tls10", Protocol::Tls10; "when tls10 provided")]
    #[test_case("tls1.0", Protocol::Tls10; "when tls10 with dot provided")]
    #[test_case("tls1_0", Protocol::Tls10; "when tls10 with underscore provided")]
    #[test_case("Tlsv10" , Protocol::Tls10; "when Tlsv10 provided")]
    #[test_case("TLS10", Protocol::Tls10; "when uppercase TLS10 Provided")]
    #[test_case("tls11", Protocol::Tls11; "when tls11 provided")]
    #[test_case("tls1.1", Protocol::Tls11; "when tls11 with dot provided")]
    #[test_case("tls1_1", Protocol::Tls11; "when tls11 with underscore provided")]
    #[test_case("Tlsv11" , Protocol::Tls11; "when Tlsv11 provided")]
    #[test_case("TLS11", Protocol::Tls11; "when uppercase TLS11 Provided")]
    #[test_case("tls12", Protocol::Tls12; "when tls12 provided")]
    #[test_case("tls1.2", Protocol::Tls12; "when tls12 with dot provided")]
    #[test_case("tls1_2", Protocol::Tls12; "when tls12 with underscore provided")]
    #[test_case("Tlsv12" , Protocol::Tls12; "when Tlsv12 provided")]
    #[test_case("TLS12", Protocol::Tls12; "when uppercase TLS12 Provided")]
    fn it_parses_protocol(value: &str, expected: Protocol) {
        let actual = Protocol::from_str(value);
        assert_eq!(actual, Ok(expected));
    }

    #[test_case(""; "when empty string provided")]
    #[test_case("Sslv3"; "when unsupported version provided")]
    #[test_case("TLS2"; "when non-existing version provided")]
    fn it_fails_to_parse_protocol(value: &str) {
        let actual = Protocol::from_str(value);
        assert_eq!(
            actual,
            Err(format!("Unsupported TLS protocol version: {}", value))
        )
    }
}