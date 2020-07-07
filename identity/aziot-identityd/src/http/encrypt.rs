pub(super) fn handle(
    req: hyper::Request<hyper::Body>,
    _inner: std::sync::Arc<aziot_identityd::Server>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<hyper::Response<hyper::Body>, hyper::Request<hyper::Body>>> + Send>> {
    Box::pin(async move {
        if req.uri().path() != "/encrypt" {
            return Err(req);
        }
        
        //TODO: Execute request
        return Ok(super::err_response(
            hyper::StatusCode::METHOD_NOT_ALLOWED,
            Some((hyper::header::ALLOW, "POST")),
            "method not allowed".into(),
        ));

        }
    )
}
