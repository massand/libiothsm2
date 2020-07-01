// Copyright (c) Microsoft. All rights reserved.

use std::fmt;
use std::fmt::Display;

use aziot_common::error::Error as CoreError;
use failure::{Backtrace, Context, Fail};

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Clone, Debug, Fail, PartialEq)]
pub enum ErrorKind {
    #[fail(display = "The service could not start up successfully: {}", _0)]
    Initialize(InitializeErrorReason),
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
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

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<CoreError> for Error {
    fn from(error: CoreError) -> Self {
        let error_kind = ErrorKind::Initialize(InitializeErrorReason::InvalidDeviceConfig);
        Error::from(error.context(error_kind))
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Self {
        Error { inner }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InitializeErrorReason {
    InvalidDeviceConfig,
    LoadSettings,
}

impl fmt::Display for InitializeErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InitializeErrorReason::InvalidDeviceConfig => {
                write!(f, "Invalid device configuration was provided")
            }

            InitializeErrorReason::LoadSettings => write!(f, "Could not load settings"),
        }
    }
}
