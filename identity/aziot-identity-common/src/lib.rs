// Copyright (c) Microsoft. All rights reserved.

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DeviceId(pub String);
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ModuleId(pub String);
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GenId(pub String);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Identity {
    #[serde(rename = "type")]
    pub id_type: IdentityType,
    #[serde(rename = "spec")]
    pub id_spec: IdentitySpec,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum IdentityType {
    Aziot,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IdentitySpec { 
    #[serde(rename = "hubName")]
    pub hub_name: String,
    #[serde(rename = "deviceId")]
    pub device_id: DeviceId,
    #[serde(rename = "moduleId")]
    pub module_id: Option<ModuleId>,
    #[serde(rename = "genId")]
    pub gen_id: Option<GenId>,
    #[serde(rename = "auth")]
    pub auth: AuthenticationInfo
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthenticationInfo {
    #[serde(rename = "type")]
    pub auth_type: AuthenticationType,
    #[serde(rename = "keyHandle")]
    pub key_handle: aziot_key_common::KeyHandle,
    #[serde(rename = "certId")]
    pub cert_id: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthenticationType {
    SaS,
    X509,
}