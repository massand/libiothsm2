// Copyright (c) Microsoft. All rights reserved.

pub mod get_caller_identity {
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Response {
        #[serde(flatten)]
        pub identity: aziot_identity_common::Identity,
    }
}


pub mod get_device_identity {
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Request {
        #[serde(rename = "type")]
        pub id_type: aziot_identity_common::IdentityType,
    }

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Response {
        #[serde(flatten)]
        pub identity: aziot_identity_common::Identity,
    }
}


pub mod create_module_identity {
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Request {
        #[serde(rename = "type")]
        pub id_type: aziot_identity_common::IdentityType,
        #[serde(rename = "deviceId")]
        pub device_id: aziot_identity_common::DeviceId,
        #[serde(rename = "moduleId")]
        pub module_id: Option<aziot_identity_common::ModuleId>,
    }
    
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Response {
        #[serde(flatten)]
        pub identity: aziot_identity_common::Identity,
    }
}


pub mod get_module_identities {
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Response {
        pub identities: Vec<aziot_identity_common::Identity>,
    }
}


pub mod get_module_identity {
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Request {
        #[serde(rename = "moduleId")]
        pub module_id: aziot_identity_common::ModuleId,
    }

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Response {
        #[serde(flatten)]
        pub identity: aziot_identity_common::Identity,
    }
}


pub mod delete_module_identity {
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Request {
        #[serde(rename = "moduleId")]
        pub module_id: aziot_identity_common::ModuleId,
    }
}

pub mod get_trust_bundle {
    #[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
    pub struct Pem(pub Vec<u8>);
    pub struct Response {
        pub certificate: Pem,
    }
}


pub mod decrypt {
	#[derive(Debug, serde::Deserialize, serde::Serialize)]
	pub struct Request {
		#[serde(rename = "moduleid")]
		pub module_id: aziot_identity_common::ModuleId,

		#[serde(flatten)]
		pub parameters: Parameters,

		pub ciphertext: http_common::ByteString,
	}

	#[derive(Debug, serde::Deserialize, serde::Serialize)]
	#[serde(tag = "algorithm", content = "parameters")]
	pub enum Parameters {
		#[serde(rename = "AEAD")]
		Aead {
			iv: http_common::ByteString,
			aad: http_common::ByteString,
		},
	}

	#[derive(Debug, serde::Deserialize, serde::Serialize)]
	pub struct Response {
		pub plaintext: http_common::ByteString,
	}
}

pub mod encrypt {
	#[derive(Debug, serde::Deserialize, serde::Serialize)]
	pub struct Request {
		#[serde(rename = "moduleid")]
		pub module_id: aziot_identity_common::ModuleId,

		#[serde(flatten)]
		pub parameters: Parameters,

		pub plaintext: http_common::ByteString,
	}

	#[derive(Debug, serde::Deserialize, serde::Serialize)]
	#[serde(tag = "algorithm", content = "parameters")]
	pub enum Parameters {
		#[serde(rename = "AEAD")]
		Aead {
			iv: http_common::ByteString,
			aad: http_common::ByteString,
		},
	}

	#[derive(Debug, serde::Deserialize, serde::Serialize)]
	pub struct Response {
		pub ciphertext: http_common::ByteString,
	}
}