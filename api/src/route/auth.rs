use axum::{extract::State, http::StatusCode, Form};
use service::dto::auth::NewOtpRequest;

use crate::AppState;

pub(crate) async fn new_otp(
    state: State<AppState>,
    Form(request): Form<NewOtpRequest>,
) -> Result<String, (StatusCode, String)> {
    match service::auth::new_otp(&state.db, &state.config, &request.secret).await {
        Ok(response) => Ok(response),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
