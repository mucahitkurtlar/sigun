use crate::error::ToErrorResponse;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Form,
};
use service::{dto::delete::DeleteRequest, error::ServiceError};

use crate::AppState;

pub async fn upload(
    state: State<AppState>,
    mut multipart: Multipart,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut dir = state.config.file.root.to_string();
    if let Some(field) = multipart.next_field().await.unwrap() {
        let name = match field.name() {
            Some(name) => name.to_string(),
            None => return Err((StatusCode::BAD_REQUEST, "Invalid field name".to_string())),
        };
        if &name != "token" {
            return Err((StatusCode::BAD_REQUEST, "Invalid field name".to_string()));
        }
        let token = match field.text().await {
            Ok(data) => data,
            Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
        };
        if !service::auth::auth(&state.db, &state.config, &token).await {
            return Err(ServiceError::InvalidToken.to_response());
        }
    }

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = match field.name() {
            Some(name) => name.to_string(),
            None => return Err((StatusCode::BAD_REQUEST, "Invalid field name".to_string())),
        };

        if &name == "subdir" {
            let subdir = match field.text().await {
                Ok(data) => data,
                Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
            };
            dir = format!("{}/{}", &state.config.file.root, subdir);

            continue;
        } else {
            let data = match field.bytes().await {
                Ok(data) => data,
                Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
            };

            let mut file = tokio::fs::File::create(format!("{}/{}", dir, name))
                .await
                .unwrap();
            tokio::io::copy(&mut &data[..], &mut file).await.unwrap();
        }
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete(
    state: State<AppState>,
    Form(request): Form<DeleteRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    match service::media::delete_media(&state.db, &state.config, &request.token, &request.path)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.to_response()),
    }
}
