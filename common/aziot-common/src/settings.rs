// Copyright (c) Microsoft. All rights reserved.

// use config::{Config, File, FileFormat};
use crate::error::Error;

/// This is the default connection string
pub const DEFAULT_CONNECTION_STRING: &str = "<ADD DEVICE CONNECTION STRING HERE>";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct ManualX509Auth {
    pub iothub_hostname: String,
    
    pub device_id: String,
    
    pub identity_cert: url::Url,
    
    pub identity_pk: url::Url,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct ManualSaSAuth {
    pub iothub_hostname: String,
    
    pub device_id: String,
    
    pub device_id_pk: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "method")]
#[serde(rename_all = "lowercase")]
pub enum ManualAuthMethod {
    SaS(ManualSaSAuth),
    X509(ManualX509Auth),
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Manual {
    pub authentication: ManualAuthMethod,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "method")]
#[serde(rename_all = "lowercase")]
pub enum AttestationMethod {
    #[serde(rename = "symmetric_key")]
    SymmetricKey(SymmetricKeyAttestationInfo),
    X509(X509AttestationInfo),
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct SymmetricKeyAttestationInfo {
    pub registration_id: String,
    
    pub symmetric_key: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct X509AttestationInfo {
    pub registration_id: Option<String>,
    
    pub identity_cert: url::Url,
    
    pub identity_pk: url::Url,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Dps {
    pub global_endpoint: url::Url,
    
    pub scope_id: String,
    
    pub attestation: AttestationMethod,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "cert_issuance")]
pub struct CertificateIssuance {
    
    #[serde(rename = "device-id")]
    pub device_identity: CertificateIssuanceType,

    #[serde(rename = "module-id")]
    pub module_identity: CertificateIssuanceType,

    #[serde(rename = "module-server")]
    pub module_server: CertificateIssuanceType,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CertificateIssuanceType {
    Dps,
    Est,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Provisioning {
    #[serde(flatten)]
    pub provisioning: ProvisioningType,

    #[serde(default)]
    pub dynamic_reprovisioning: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "source")]
#[serde(rename_all = "lowercase")]
pub enum ProvisioningType {
    Manual(Box<Manual>),
    Dps(Box<Dps>),
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub cert_issuance: CertificateIssuance,

    pub provisioning: Provisioning,
}

impl Settings {
    pub fn new(filename: &std::path::Path) -> Result<Self, Error> {
        let settings = std::fs::read_to_string(filename).map_err(|err| Error::LoadSettings(err))?;
        let settings = toml::from_str(&settings).map_err(|err| Error::ParseSettings(err))?;

        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::CertificateIssuanceType;
    use crate::settings::{Settings, ProvisioningType};
    
    #[test]
    fn manual_sas_provisioning_settings_succeeds() {
        let s = Settings::new(std::path::Path::new("test/good_sas_config.toml")).unwrap();

        assert_eq!(s.cert_issuance.device_identity, CertificateIssuanceType::Dps);
        assert_eq!(s.cert_issuance.module_identity, CertificateIssuanceType::Dps);
        assert_eq!(s.cert_issuance.module_server, CertificateIssuanceType::Dps);
        assert_eq!(s.provisioning.dynamic_reprovisioning, false);
        
        match s.provisioning.provisioning {
            ProvisioningType::Manual(_) => assert!(true),
            _ => assert!(false, "incorrect provisioning type selected")
        };
    }

    #[test]
    fn manual_dps_provisioning_settings_succeeds() {
        let s = Settings::new(std::path::Path::new("test/good_dps_config.toml")).unwrap();

        assert_eq!(s.cert_issuance.device_identity, CertificateIssuanceType::Dps);
        assert_eq!(s.cert_issuance.module_identity, CertificateIssuanceType::Dps);
        assert_eq!(s.cert_issuance.module_server, CertificateIssuanceType::Dps);
        
        match s.provisioning.provisioning {
            ProvisioningType::Dps(_) => assert!(true),
            _ => assert!(false, "incorrect provisioning type selected")
        };        
    }

    #[test]
    fn bad_provisioning_settings_fails() {
        assert!(Settings::new(std::path::Path::new("test/bad_config.toml")).is_err(), "provisioning settings read should fail");
    }
}