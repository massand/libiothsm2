// Copyright (c) Microsoft. All rights reserved.

use std::fmt;
use std::fmt::Display;

use failure::{Backtrace, Context, Fail};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(
        display = "The Connection String is empty. Please update the config.yaml and provide the IoTHub connection information."
    )]
    ConnectionStringEmpty,

    #[fail(display = "The Connection String is missing required parameter {}", _0)]
    ConnectionStringMissingRequiredParameter(&'static str),

    #[fail(
        display = "The Connection String has a malformed value for parameter {}.",
        _0
    )]
    ConnectionStringMalformedParameter(&'static str),

    #[fail(
        display = "Device information is required.\nPlease update config.toml with the IoT Hub connection information.\nSee {} for more details.",
        _0
    )]
    ConnectionStringNotConfigured(&'static str),

    #[fail(display = "The service could not start up successfully: {}", _0)]
    Initialize(InitializeErrorReason),

    #[fail(display = "Invalid log tail {:?}", _0)]
    InvalidLogTail(String),

    #[fail(display = "Invalid URL {:?}", _0)]
    InvalidUrl(String),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InitializeErrorReason {
    LoadSettings,
}

impl fmt::Display for InitializeErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InitializeErrorReason::LoadSettings => write!(f, "Could not load settings"),
        }
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn new(inner: Context<ErrorKind>) -> Self {
        Error { inner }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Self {
        Error { inner }
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
