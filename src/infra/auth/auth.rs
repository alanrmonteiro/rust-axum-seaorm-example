use axum::{
    extract::FromRequestParts,
    http::StatusCode,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use jwks::Jwks;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Claims {
    pub _sub: String,
    pub _roles: Vec<String>, // Mapeado do Keycloak
    pub _exp: usize,
}

pub struct AuthUser(pub Claims);

// Estrutura para manter o estado do validador (corrigida)
pub struct AppState {
    pub jwks_client: Jwks,
    pub db_conn: DatabaseConnection,
}

impl AsRef<AppState> for AppState {
    fn as_ref(&self) -> &AppState {
        self
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync + AsRef<AppState>,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = state.as_ref();

        // Chamando as funções modulares que agora residem associadas ao AuthUser
        let token = Self::extract_token_from_request_parts(parts)?;
        let decoding_key = Self::extract_decoding_key_from_token(&token, app_state)?;
        let claims = Self::extract_claims(&token, &decoding_key)?;

        Ok(AuthUser(claims))
    }
}

// Bloco separado para suas funções auxiliares modulares
impl AuthUser {
    fn extract_token_from_request_parts(parts: &Parts) -> Result<String, Response> {
        parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(|s| s.to_string())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Token ausente").into_response())
    }

    fn extract_decoding_key_from_token(
        token: &str,
        app_state: &AppState,
    ) -> Result<DecodingKey, Response> {
        // 1. Decodifica apenas o header para ler o 'kid'
        let header = decode_header(token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Header do token inválido").into_response())?;

        let kid = header.kid.ok_or_else(|| {
            (StatusCode::UNAUTHORIZED, "Token sem propriedade kid").into_response()
        })?;

        // 2. Busca o objeto Jwk no cache através do 'kid'
        let jwk = app_state.jwks_client.keys.get(&kid).ok_or_else(|| {
            (StatusCode::UNAUTHORIZED, "Chave pública não encontrada").into_response()
        })?;

        // 3. CORREÇÃO: Basta clonar a decoding_key que o crate já deixou pronta!
        Ok(jwk.decoding_key.clone())
    }

    fn extract_claims(token: &str, decoding_key: &DecodingKey) -> Result<Claims, Response> {
        decode::<Claims>(token, decoding_key, &Validation::new(Algorithm::RS256))
            .map(|data| data.claims)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Token inválido ou expirado").into_response())
    }
}

// Inicializador auxiliar para o seu main.rs
pub async fn get_jwks_client() -> Jwks {
    Jwks::from_jwks_url("http://localhost:8080/realms/myrealm/protocol/openid-connect/certs")
        .await
        .expect("Failed to create JwksClient")
}
