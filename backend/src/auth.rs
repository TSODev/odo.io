use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // ID de l'utilisateur
    pub exp: usize,  // Expiration
}

pub struct AuthenticatedUser(pub uuid::Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Extraire le header Authorization
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Token manquant".to_string()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Format de token invalide".to_string(),
            ));
        }

        let token = &auth_header[7..];

        // 2. Valider le token
        let secret = std::env::var("JWT_SECRET").map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erreur de configuration serveur".into(),
            )
        })?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Token invalide ou expiré".to_string(),
            )
        })?;

        // 3. Convertir le "sub" (string) en UUID
        let user_id = uuid::Uuid::parse_str(&token_data.claims.sub).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "ID utilisateur corrompu".to_string(),
            )
        })?;

        Ok(AuthenticatedUser(user_id))
    }
}
