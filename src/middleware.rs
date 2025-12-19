use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next};
use tracing::instrument;

use crate::error::YandexWebmasterError;

/// Middleware that adds OAuth authentication to requests
#[derive(Debug, Clone)]
pub struct AuthMiddleware {
    oauth_token: String,
}

impl AuthMiddleware {
    /// Creates a new authentication middleware with the provided OAuth token
    pub fn new(oauth_token: String) -> Self {
        Self { oauth_token }
    }
}

#[async_trait::async_trait]
impl Middleware for AuthMiddleware {
    #[instrument(skip(self, req, extensions, next))]
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        // Add Authorization header
        req.headers_mut().insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("OAuth {}", self.oauth_token))
                .map_err(|e| {
                    reqwest_middleware::Error::Middleware(anyhow::anyhow!(
                        YandexWebmasterError::MiddlewareError(format!(
                            "Failed to create authorization header: {}",
                            e
                        ))
                    ))
                })?,
        );

        next.run(req, extensions).await
    }
}
