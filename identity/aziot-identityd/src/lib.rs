// Copyright (c) Microsoft. All rights reserved.

pub mod app;
pub mod error;
mod logging;
pub mod settings;

pub use error::Error;

pub struct Server {
    //ProvisioningManager
    //HubIdentityManager
    //LocalIdentityManager
    //KeyClient
    //CertificateClient
}

impl Server {
    pub fn new() -> Result<Self, Error> {
        Ok(Server {})
    }
}

impl Server {
    pub async fn get_module_identity() -> Result<aziot_identity_common::Identity, Error> {

        //TODO: match identity type based on uid configuration and get identity from appropriate identity manager (Hub or local)
        let i = aziot_identity_common::Identity::Aziot (
            aziot_identity_common::AzureIoTSpec {
                hub_name: "dummyHubName".to_string(),
                device_id: aziot_identity_common::DeviceId("dummyDeviceId".to_string()),
                module_id: Some(aziot_identity_common::ModuleId("dummyModuleId".to_string())),
                gen_id: Some(aziot_identity_common::GenId("dummyGenId".to_string())),
                auth: aziot_identity_common::AuthenticationInfo {
                    auth_type: aziot_identity_common::AuthenticationType::SaS,
                    key_handle: aziot_key_common::KeyHandle("dummyKeyHandle".to_string()),
                    cert_id: None,
                }});
        Ok(i)
    }

    pub async fn get_module_identities(&self, _idtype: String) -> Result<Vec<aziot_identity_common::Identity>, Error> {

        //TODO: get identity type and get identities from appropriate identity manager (Hub or local)
        Ok(vec![aziot_identity_common::Identity::Aziot (
            aziot_identity_common::AzureIoTSpec {
                hub_name: "dummyHubName".to_string(),
                device_id: aziot_identity_common::DeviceId("dummyDeviceId".to_string()),
                module_id: Some(aziot_identity_common::ModuleId("dummyModuleId".to_string())),
                gen_id: Some(aziot_identity_common::GenId("dummyGenId".to_string())),
                auth: aziot_identity_common::AuthenticationInfo {
                    auth_type: aziot_identity_common::AuthenticationType::SaS,
                    key_handle: aziot_key_common::KeyHandle("dummyKeyHandle".to_string()),
                    cert_id: None,
                }})])
    }

    pub async fn get_device_identity(_idtype: String) -> Result<Vec<aziot_identity_common::Identity>, Error> {

        //TODO: validate identity type for device is always Hub and get identities from provisioning manager (Hub)
        Ok(vec![aziot_identity_common::Identity::Aziot(
            aziot_identity_common::AzureIoTSpec {
                hub_name: "dummyHubName".to_string(),
                device_id: aziot_identity_common::DeviceId("dummyDeviceId".to_string()),
                module_id: None,
                gen_id: None,
                auth: aziot_identity_common::AuthenticationInfo {
                    auth_type: aziot_identity_common::AuthenticationType::SaS,
                    key_handle: aziot_key_common::KeyHandle("dummyKeyHandle".to_string()),
                    cert_id: None,
                }})])
    }

    pub async fn create_identity(id: aziot_identity_common::Identity) -> Result<aziot_identity_common::Identity, Error> {

        //TODO: match identity type based on uid configuration and create and get identity from appropriate identity manager (Hub or local)
        Ok(id)
    }

    pub async fn delete_identity(_id: aziot_identity_common::Identity) -> Result<bool, Error> {

        //TODO: match identity type based on uid configuration and create and get identity from appropriate identity manager (Hub or local)
        Ok(Default::default())
    }

    pub async fn encrypt() -> Result<http_common::ByteString, Error> {

        //TODO: match identity type based on uid configuration and create and get identity from appropriate identity manager (Hub or local)
        Ok(http_common::ByteString::default())
    }

    pub async fn decrypt() -> Result<http_common::ByteString, Error> {

        //TODO: match identity type based on uid configuration and create and get identity from appropriate identity manager (Hub or local)
        Ok(http_common::ByteString::default())
    }
}
