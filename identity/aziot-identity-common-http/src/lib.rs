// Copyright (c) Microsoft. All rights reserved.

pub mod get_caller_identity {
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Response {
        #[serde(rename = "type")]
        pub id_type: aziot_identity_common::IdentityType,
        #[serde(rename = "spec")]
        pub id_spec: aziot_identity_common::IdentitySpec,
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
        #[serde(rename = "type")]
        pub id_type: aziot_identity_common::IdentityType,
        #[serde(rename = "spec")]
        pub id_spec: aziot_identity_common::IdentitySpec,
    }
}


pub mod get_module_identities {
    pub struct Request {}

    pub struct Response {}
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
        #[serde(rename = "type")]
        pub id_type: aziot_identity_common::IdentityType,
        #[serde(rename = "spec")]
        pub id_spec: aziot_identity_common::IdentitySpec,
    }
}


pub mod get_module_identity {
    pub struct Request {}

    pub struct Response {}
}


pub mod delete_module_identity {
    pub struct Request {}

    pub struct Response {}
}


pub mod reprovision_device {
    pub struct Request {}

    pub struct Response {}
}


pub mod get_trust_bundle {
    pub struct Request {}

    pub struct Response {}
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