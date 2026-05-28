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

pub trait Role {
    fn name() -> &'static str;
}

// Marcadores de Tipo (Zero-Sized Types)
pub struct Admin;
impl Role for Admin {
    fn name() -> &'static str {
        "admin"
    } // Nome da Role igualzinho ao configurado no Keycloak
}

pub struct UserRole;
impl Role for UserRole {
    fn name() -> &'static str {
        "user"
    }
}

// O nosso extrator modular
pub struct HasRole<R: Role>(pub AuthUser, pub std::marker::PhantomData<R>);

impl<S, R> FromRequestParts<S> for HasRole<R>
where
    S: Send + Sync + AsRef<AppState>,
    R: Role + Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // 1. Executa o extrator do AuthUser padrão para validar o JWT
        let auth_user = AuthUser::from_request_parts(parts, state).await?;

        // 2. Inspeciona as roles que o Keycloak injetou no realm_access
        let target_role = R::name();
        if auth_user
            .0
            .realm_access
            .roles
            .iter()
            .any(|r| r == target_role)
        {
            // Se o usuário tiver a role, autoriza a requisição!
            Ok(HasRole(auth_user, std::marker::PhantomData))
        } else {
            // Se não tiver, barra imediatamente com 403 Forbidden
            Err((StatusCode::FORBIDDEN, "Acesso negado: Role insuficiente").into_response())
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub realm_access: RealmAccess, // Estrutura padrão do Keycloak
}

#[derive(Debug, Deserialize, Clone)]
pub struct RealmAccess {
    pub roles: Vec<String>,
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
