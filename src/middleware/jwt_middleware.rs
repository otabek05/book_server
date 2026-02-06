use axum::{Json, body::Body, extract::State, http::{Request, StatusCode}, middleware::Next, response::Response};

use crate::{app_state::AppState, pkg::api_response::ApiResponse};



pub async fn jwt_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    println!("Request arrived to jwt");
    let auth = req
    .headers()
    .get("Authorization")
    .and_then(|v|v.to_str().ok())
    .and_then(|v| v.strip_prefix("Bearer "))
    .ok_or(  StatusCode::UNAUTHORIZED)?;
    let claims = state.service.jwt.parse(auth).map_err(|err| {
         eprintln!("JWT parsing error: {:?}", err);
          StatusCode::UNAUTHORIZED
    })?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}