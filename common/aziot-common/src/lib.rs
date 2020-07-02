// Copyright (c) Microsoft. All rights reserved.

use std::path::Path;
use crate::error::Error;
use crate::settings::Settings;

pub mod error;
pub mod settings;

pub fn init(config_file: &Path) -> Result<Settings, Error> {
    let settings = Settings::new(&config_file)?;

    Ok(settings)
}
