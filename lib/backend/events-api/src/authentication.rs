use axum::{
    extract::{FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

use crate::configuration::CONTENT_CREATORS_GROUP_NAME;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Token is invalid")]
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match self {
            AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
        };
        (status, "Invalid token").into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    #[serde(rename = "cognito:groups")]
    pub groups: Vec<String>,
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let key = DecodingKey::from_secret(&[]);
        let mut validation = Validation::default();
        validation.insecure_disable_signature_validation();
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &key, &validation)
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

pub async fn content_creator_authorizer_middleware(
    claims: Claims,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if claims.groups.contains(&CONTENT_CREATORS_GROUP_NAME) {
        tracing::debug!(
            "User has the required role {:?}. Letting request through.",
            CONTENT_CREATORS_GROUP_NAME
        );
        return Ok(next.run(req).await);
    }

    tracing::debug!(
        "User does not have the required role {:?}. Rejecting request.",
        CONTENT_CREATORS_GROUP_NAME
    );
    Err(StatusCode::FORBIDDEN)
}
