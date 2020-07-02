// Copyright (c) Microsoft. All rights reserved.

// use config::{Config, File, FileFormat};
use crate::error::Error;

/// This is the default connection string
pub const DEFAULT_CONNECTION_STRING: &str = "<ADD DEVICE CONNECTION STRING HERE>";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct ManualX509Auth {
    iothub_hostname: String,
    device_id: String,
    identity_cert: url::Url,
    identity_pk: url::Url,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct ManualDeviceConnectionString {
    device_connection_string: String,
}

impl ManualDeviceConnectionString {
    pub fn new(device_connection_string: String) -> Self {
        ManualDeviceConnectionString {
            device_connection_string,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "method")]
#[serde(rename_all = "lowercase")]
pub enum ManualAuthMethod {
    #[serde(rename = "device_connection_string")]
    DeviceConnectionString(ManualDeviceConnectionString),
    X509(ManualX509Auth),
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Manual {
    authentication: ManualAuthMethod,
}

impl<'de> serde::Deserialize<'de> for Manual {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, serde::Deserialize)]
        struct Inner {
            #[serde(skip_serializing_if = "Option::is_none")]
            device_connection_string: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            authentication: Option<ManualAuthMethod>,
        }

        let value: Inner = serde::Deserialize::deserialize(deserializer)?;

        let authentication = match (value.device_connection_string, value.authentication) {
            (Some(_), Some(_)) => {
                return Err(serde::de::Error::custom(
                        "Only one of provisioning.device_connection_string or provisioning.authentication must be set in the config.toml.",
                    ));
            }
            (Some(cs), None) => {
                ManualAuthMethod::DeviceConnectionString(ManualDeviceConnectionString::new(cs))
            }
            (None, Some(auth)) => auth,
            (None, None) => {
                return Err(serde::de::Error::custom(
                    "One of provisioning.device_connection_string or provisioning.authentication must be set in the config.toml.",
                ));
            }
        };

        Ok(Manual { authentication })
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "method")]
#[serde(rename_all = "lowercase")]
pub enum AttestationMethod {
    Tpm(TpmAttestationInfo),
    #[serde(rename = "symmetric_key")]
    SymmetricKey(SymmetricKeyAttestationInfo),
    X509(X509AttestationInfo),
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct TpmAttestationInfo {
    registration_id: String,
}

impl TpmAttestationInfo {
    pub fn new(registration_id: String) -> Self {
        TpmAttestationInfo { registration_id }
    }

    pub fn registration_id(&self) -> &str {
        &self.registration_id
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct SymmetricKeyAttestationInfo {
    registration_id: String,
    symmetric_key: String,
}

impl SymmetricKeyAttestationInfo {
    pub fn registration_id(&self) -> &str {
        &self.registration_id
    }

    pub fn symmetric_key(&self) -> &str {
        &self.symmetric_key
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct X509AttestationInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    registration_id: Option<String>,
    identity_cert: url::Url,
    identity_pk: url::Url,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Dps {
    global_endpoint: url::Url,
    scope_id: String,
    attestation: AttestationMethod,
}

impl<'de> serde::Deserialize<'de> for Dps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, serde::Deserialize)]
        struct Inner {
            global_endpoint: url::Url,
            scope_id: String,
            registration_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            attestation: Option<AttestationMethod>,
        }

        let value: Inner = serde::Deserialize::deserialize(deserializer)?;

        let attestation = match (value.attestation, value.registration_id) {
            (Some(_att), Some(_)) => {
                return Err(serde::de::Error::custom(
                    "Provisioning registration_id has to be set only in attestation",
                ));
            }
            (Some(att), None) => att,
            (None, Some(reg_id)) => AttestationMethod::Tpm(TpmAttestationInfo::new(reg_id)),
            (None, None) => {
                return Err(serde::de::Error::custom(
                    "Provisioning registration_id has to be set",
                ));
            }
        };

        Ok(Dps {
            global_endpoint: value.global_endpoint,
            scope_id: value.scope_id,
            attestation,
        })
    }
}

impl Dps {
    pub fn global_endpoint(&self) -> &url::Url {
        &self.global_endpoint
    }

    pub fn scope_id(&self) -> &str {
        &self.scope_id
    }

    pub fn attestation(&self) -> &AttestationMethod {
        &self.attestation
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct External {
    endpoint: url::Url,
}

impl External {
    pub fn new(endpoint: url::Url) -> Self {
        External { endpoint }
    }

    pub fn endpoint(&self) -> &url::Url {
        &self.endpoint
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Provisioning {
    #[serde(flatten)]
    provisioning: ProvisioningType,

    #[serde(default)]
    dynamic_reprovisioning: bool,
}

impl Provisioning {
    pub fn provisioning_type(&self) -> &ProvisioningType {
        &self.provisioning
    }

    pub fn dynamic_reprovisioning(&self) -> bool {
        self.dynamic_reprovisioning
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "source")]
#[serde(rename_all = "lowercase")]
pub enum ProvisioningType {
    Manual(Box<Manual>),
    Dps(Box<Dps>),
    External(External),
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DeviceCertificate {
    device_ca_cert: String,
    device_ca_pk: String,
    trusted_ca_certs: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Settings {
    provisioning: Provisioning,
    hostname: String,
    homedir: std::path::PathBuf,
}

impl Settings {
    pub fn new(filename: &std::path::Path) -> Result<Self, Error> {
        let settings = std::fs::read_to_string(filename).map_err(|err| Error::LoadSettings(err))?;
        let settings = toml::from_str(&settings).map_err(|err| Error::ParseSettings(err))?;

        Ok(settings)
    }
}
