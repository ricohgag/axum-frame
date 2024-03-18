use axum::{extract::Request, middleware::Next, response::Response};
use http::header::AUTHORIZATION;
use service::identity::Identity;

pub async fn handle(mut request: Request, next: Next) -> Response {
    let token = request.headers().get(AUTHORIZATION);

    let identity = match token {
        None => Identity::empty(),
        Some(v) => match v.to_str() {
            Ok(v) => Identity::from_auth_token(v.to_string()),
            Err(err) => {
                tracing::error!(error = ?err, "err get header(authorization)");
                Identity::empty()
            }
        },
    };

    request.extensions_mut().insert(identity);

    next.run(request).await
}
