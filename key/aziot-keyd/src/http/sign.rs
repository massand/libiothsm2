pub(super) fn handle(
	req: hyper::Request<hyper::Body>,
	inner: std::sync::Arc<aziot_keyd::Server>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<hyper::Response<hyper::Body>, hyper::Request<hyper::Body>>> + Send>> {
	Box::pin(async move {
		if req.uri().path() != "/sign" {
			return Err(req);
		}

		let (http::request::Parts { method, headers, .. }, body) = req.into_parts();
		let content_type = headers.get(hyper::header::CONTENT_TYPE).and_then(|value| value.to_str().ok());

		if method != hyper::Method::POST {
			return Ok(super::err_response(
				hyper::StatusCode::METHOD_NOT_ALLOWED,
				Some((hyper::header::ALLOW, "POST")),
				"method not allowed".into(),
			));
		}

		if content_type.as_deref() != Some("application/json") {
			return Ok(super::err_response(
				hyper::StatusCode::UNSUPPORTED_MEDIA_TYPE,
				None,
				"request body must be application/json".into(),
			));
		}

		let body = match hyper::body::to_bytes(body).await {
			Ok(body) => body,
			Err(err) => return Ok(super::err_response(
				hyper::StatusCode::BAD_REQUEST,
				None,
				super::error_to_message(&err).into(),
			)),
		};
		let body: aziot_key_common_http::sign::Request = match serde_json::from_slice(&body) {
			Ok(body) => body,
			Err(err) => return Ok(super::err_response(
				hyper::StatusCode::UNPROCESSABLE_ENTITY,
				None,
				super::error_to_message(&err).into(),
			)),
		};
		let (mechanism, digest) = match body.parameters {
			aziot_key_common_http::sign::Parameters::Ecdsa { digest } => (aziot_key_common::SignMechanism::Ecdsa, digest),

			aziot_key_common_http::sign::Parameters::RsaPkcs1 { message_digest_algorithm, message } => {
				let message_digest = match &*message_digest_algorithm {
					"sha1" => aziot_key_common::RsaPkcs1MessageDigest::Sha1,
					"sha224" => aziot_key_common::RsaPkcs1MessageDigest::Sha224,
					"sha256" => aziot_key_common::RsaPkcs1MessageDigest::Sha256,
					"sha384" => aziot_key_common::RsaPkcs1MessageDigest::Sha384,
					"sha512" => aziot_key_common::RsaPkcs1MessageDigest::Sha512,
					message_digest_algorithm => return Ok(super::err_response(
						hyper::StatusCode::UNPROCESSABLE_ENTITY,
						None,
						format!("invalid value of parameters.messageDigestAlgorithm {:?}", message_digest_algorithm).into(),
					)),
				};

				(aziot_key_common::SignMechanism::RsaPkcs1 { message_digest }, message)
			},

			aziot_key_common_http::sign::Parameters::HmacSha256 { message } => (aziot_key_common::SignMechanism::HmacSha256, message),
		};

		let signature = match inner.sign(&body.key_handle, mechanism, &digest.0) {
			Ok(signature) => signature,
			Err(err) => return Ok(super::ToHttpResponse::to_http_response(&err)),
		};

		let res = aziot_key_common_http::sign::Response {
			signature: http_common::ByteString(signature),
		};
		let res = super::json_response(hyper::StatusCode::OK, &res);
		Ok(res)
	})
}
