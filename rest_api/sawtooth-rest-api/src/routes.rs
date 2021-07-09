use actix_web::{
    error::ContentTypeError, get, post, web::Payload, App, Error, HttpMessage, HttpRequest,
    HttpResponse,
};

use crate::errors::{RestApiError, RestApiErrorKind};

#[post("/batches")]
async fn submit_batches(
    request: HttpRequest,
    mut payload: Payload,
) -> Result<HttpResponse, RestApiError> {
    if request.content_type() != "application/octet-stream" {
        return Err(RestApiErrorKind::SubmissionWrongContentType)?;
    }

    // let mut body = web

    todo!()
}
