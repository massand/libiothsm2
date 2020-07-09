use crate::http::test_module_identity;

pub(super) fn handle(
    req: hyper::Request<hyper::Body>,
    _inner: std::sync::Arc<aziot_identityd::Server>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<hyper::Response<hyper::Body>, hyper::Request<hyper::Body>>> + Send>> {
    Box::pin(async move {
        if req.uri().path() != "/identities/modules" {
            return Err(req);
        }

        let (http::request::Parts { method, headers, .. }, body) = req.into_parts();
        let content_type = headers.get(hyper::header::CONTENT_TYPE).and_then(|value| value.to_str().ok());

        if method != hyper::Method::GET {
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

        let _body = match hyper::body::to_bytes(body).await {
            Ok(body) => body,
            Err(err) => return Ok(super::err_response(
                hyper::StatusCode::BAD_REQUEST,
                None,
                super::error_to_message(&err).into(),
            )),
        };

        //TODO: Parse request, execute and respond

        // let _body: aziot_identity_common_http::get_module_identities::Request = match serde_json::from_slice(&body) {
        //     Ok(body) => body,
        //     Err(err) => return Ok(super::err_response(
        //         hyper::StatusCode::UNPROCESSABLE_ENTITY,
        //         None,
        //         super::error_to_message(&err).into(),
        //     )),
        // };

        let res = vec![test_module_identity()];
        let res = aziot_identity_common_http::get_module_identities::Response { identities: res };

        let res = super::json_response(hyper::StatusCode::OK, &res);
        Ok(res)

        }
    )
}
