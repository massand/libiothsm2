// Copyright (c) Microsoft. All rights reserved.

use crate::error::{Error, ErrorKind, InitializeErrorReason};
use crate::settings::Settings;
use failure::ResultExt;
use std::path::Path;

pub mod error;
pub mod settings;

/// This is the default auto generated certificate life
pub const DEFAULT_AUTO_GENERATED_CA_LIFETIME_DAYS: u16 = 90;

pub fn init(config_file: &Path) -> Result<Settings, Error> {
    let settings = Settings::new(&config_file)
        .context(ErrorKind::Initialize(InitializeErrorReason::LoadSettings))?;

    Ok(settings)
}
