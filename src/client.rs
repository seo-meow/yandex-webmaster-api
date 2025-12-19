use reqwest_middleware::ClientBuilder;
use serde_qs::ArrayFormat;
use tracing::instrument;

use crate::{
    dto::*,
    error::{Result, YandexApiErrorResponse, YandexWebmasterError},
    middleware::AuthMiddleware,
};

/// Base URL for the Yandex Webmaster API
const API_BASE_URL: &str = "https://api.webmaster.yandex.net/v4";

/// Client for interacting with the Yandex Webmaster API
#[derive(Debug, Clone)]
pub struct YandexWebmasterClient {
    client: reqwest_middleware::ClientWithMiddleware,
    user_id: i64,
    qs: serde_qs::Config,
}

impl YandexWebmasterClient {
    /// Creates a new Yandex Webmaster API client
    ///
    /// # Arguments
    ///
    /// * `oauth_token` - OAuth token for authentication
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP client cannot be created
    /// - The user information cannot be fetched
    /// - The OAuth token is invalid
    #[instrument(skip(oauth_token))]
    pub async fn new(oauth_token: String) -> Result<Self> {
        // Build the HTTP client with middleware
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(AuthMiddleware::new(oauth_token))
            .build();

        // Fetch user information
        let user_response = Self::fetch_user(&client).await?;

        tracing::info!(
            user_id = user_response.user_id,
            "Successfully authenticated"
        );

        Ok(Self {
            client,
            user_id: user_response.user_id,
            qs: serde_qs::Config::new().array_format(ArrayFormat::Unindexed),
        })
    }

    /// Fetches user information from the API
    #[instrument(skip(client))]
    async fn fetch_user(client: &reqwest_middleware::ClientWithMiddleware) -> Result<UserResponse> {
        let url = format!("{}/user", API_BASE_URL);

        tracing::debug!(url = %url, "Fetching user information");

        let response = client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(Self::parse_error(response).await);
        }

        let user_response: UserResponse = response.json().await?;

        Ok(user_response)
    }

    /// Returns the user ID
    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    // ============================================================================
    // Hosts Management
    // ============================================================================

    /// List all sites for the user
    #[instrument(skip(self))]
    pub async fn get_hosts(&self) -> Result<Vec<HostInfo>> {
        let url = format!("{}/user/{}/hosts", API_BASE_URL, self.user_id);
        let result: HostsResponse = self.get(&url).await?;
        Ok(result.hosts)
    }

    /// Add a new site
    #[instrument(skip(self))]
    pub async fn add_host(&self, request: &AddHostRequest) -> Result<AddHostResponse> {
        let url = format!("{}/user/{}/hosts", API_BASE_URL, self.user_id);
        self.post(&url, request).await
    }

    /// Get information about a specific site
    #[instrument(skip(self))]
    pub async fn get_host(&self, host_id: &str) -> Result<FullHostInfo> {
        let url = format!("{}/user/{}/hosts/{}", API_BASE_URL, self.user_id, host_id);
        self.get(&url).await
    }

    /// Delete a site
    #[instrument(skip(self))]
    pub async fn delete_host(&self, host_id: &str) -> Result<()> {
        let url = format!("{}/user/{}/hosts/{}", API_BASE_URL, self.user_id, host_id);
        self.delete(&url).await
    }

    // ============================================================================
    // Host Verification
    // ============================================================================

    /// Get verification status for a site
    #[instrument(skip(self))]
    pub async fn get_verification_status(&self, host_id: &str) -> Result<HostVerificationResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/verification",
            API_BASE_URL, self.user_id, host_id
        );
        self.get(&url).await
    }

    /// Initiate verification procedure for a site
    #[instrument(skip(self))]
    pub async fn verify_host(
        &self,
        host_id: &str,
        verification_type: ExplicitVerificationType,
    ) -> Result<HostVerificationResponse> {
        let serialized = serde_json::to_value(verification_type)?;
        let verification_type = serialized.clone().as_str().unwrap_or("").to_owned();

        let url = format!(
            "{}/user/{}/hosts/{}/verification?verification_type={}",
            API_BASE_URL, self.user_id, host_id, verification_type
        );
        self.post(&url, &()).await
    }

    /// Get list of verified owners for a site
    #[instrument(skip(self))]
    pub async fn get_owners(&self, host_id: &str) -> Result<Vec<Owner>> {
        let url = format!(
            "{}/user/{}/hosts/{}/owners",
            API_BASE_URL, self.user_id, host_id
        );
        let result: OwnersResponse = self.get(&url).await?;
        Ok(result.users)
    }

    // ============================================================================
    // Site Statistics
    // ============================================================================

    /// Get site summary statistics
    #[instrument(skip(self))]
    pub async fn get_host_summary(&self, host_id: &str) -> Result<HostSummaryResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/summary",
            API_BASE_URL, self.user_id, host_id
        );
        self.get(&url).await
    }

    /// Get site quality index history
    #[instrument(skip(self))]
    pub async fn get_sqi_history(
        &self,
        host_id: &str,
        req: SqiHistoryRequest,
    ) -> Result<Vec<SqiPoint>> {
        let url = format!(
            "{}/user/{}/hosts/{}/sqi-history?{}",
            API_BASE_URL,
            self.user_id,
            host_id,
            self.qs.serialize_string(&req)?
        );
        let result: SqiHistoryResponse = self.get(&url).await?;
        Ok(result.points)
    }

    // ============================================================================
    // Search Queries
    // ============================================================================

    /// Get popular search queries for a site
    #[instrument(skip(self))]
    pub async fn get_popular_queries(
        &self,
        host_id: &str,
        request: &PopularQueriesRequest,
    ) -> Result<PopularQueriesResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/search-queries/popular?{}",
            API_BASE_URL,
            self.user_id,
            host_id,
            self.qs.serialize_string(request)?
        );
        self.get(&url).await
    }

    /// Get overall query statistics history
    #[instrument(skip(self))]
    pub async fn get_query_analytics(
        &self,
        host_id: &str,
        request: &QueryAnalyticsRequest,
    ) -> Result<QueryAnalyticsResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/search-queries/all/history?{}",
            API_BASE_URL,
            self.user_id,
            host_id,
            self.qs.serialize_string(request)?
        );
        self.get(&url).await
    }

    /// Get statistics for a specific query
    #[instrument(skip(self))]
    pub async fn get_query_history(
        &self,
        host_id: &str,
        query_id: &str,
        request: &QueryHistoryRequest,
    ) -> Result<QueryHistoryResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/search-queries/{}/history?{}",
            API_BASE_URL,
            self.user_id,
            host_id,
            query_id,
            self.qs.serialize_string(request)?
        );
        self.get(&url).await
    }

    // ============================================================================
    // Sitemaps
    // ============================================================================

    /// Get list of all sitemap files
    #[instrument(skip(self))]
    pub async fn get_sitemaps(
        &self,
        host_id: &str,
        request: &GetSitemapsRequest,
    ) -> Result<SitemapsResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/sitemaps?{}",
            API_BASE_URL,
            self.user_id,
            host_id,
            self.qs.serialize_string(request)?
        );
        self.get(&url).await
    }

    /// Get details of a specific sitemap
    #[instrument(skip(self))]
    pub async fn get_sitemap(&self, host_id: &str, sitemap_id: &str) -> Result<SitemapInfo> {
        let url = format!(
            "{}/user/{}/hosts/{}/sitemaps/{}",
            API_BASE_URL, self.user_id, host_id, sitemap_id
        );
        self.get(&url).await
    }

    /// Get list of user-submitted sitemaps
    #[instrument(skip(self))]
    pub async fn get_user_sitemaps(
        &self,
        host_id: &str,
        request: &GetUserSitemapsRequest,
    ) -> Result<UserSitemapsResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/user-added-sitemaps?{}",
            API_BASE_URL,
            self.user_id,
            host_id,
            self.qs.serialize_string(request)?
        );
        self.get(&url).await
    }

    /// Add a new sitemap file
    #[instrument(skip(self))]
    pub async fn add_sitemap(
        &self,
        host_id: &str,
        request: &AddSitemapRequest,
    ) -> Result<AddSitemapResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/user-added-sitemaps",
            API_BASE_URL, self.user_id, host_id
        );
        self.post(&url, request).await
    }

    /// Get user-submitted sitemap details
    #[instrument(skip(self))]
    pub async fn get_user_sitemap(
        &self,
        host_id: &str,
        sitemap_id: &str,
    ) -> Result<UserSitemapInfo> {
        let url = format!(
            "{}/user/{}/hosts/{}/user-added-sitemaps/{}",
            API_BASE_URL, self.user_id, host_id, sitemap_id
        );
        self.get(&url).await
    }

    /// Delete a user-submitted sitemap
    #[instrument(skip(self))]
    pub async fn delete_sitemap(&self, host_id: &str, sitemap_id: &str) -> Result<()> {
        let url = format!(
            "{}/user/{}/hosts/{}/user-added-sitemaps/{}",
            API_BASE_URL, self.user_id, host_id, sitemap_id
        );
        self.delete(&url).await
    }

    // ============================================================================
    // Indexing
    // ============================================================================

    /// Get indexing history
    #[instrument(skip(self))]
    pub async fn get_indexing_history(
        &self,
        host_id: &str,
        request: &IndexingHistoryRequest,
    ) -> Result<IndexingHistoryResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/indexing/history?{}",
            API_BASE_URL,
            self.user_id,
            host_id,
            self.qs.serialize_string(request)?
        );
        self.get(&url).await
    }

    /// Get sample indexed pages
    #[instrument(skip(self))]
    pub async fn get_indexing_samples(
        &self,
        host_id: &str,
        request: &GetIndexingSamplesRequest,
    ) -> Result<IndexingSamplesResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/indexing/samples?{}",
            API_BASE_URL,
            self.user_id,
            host_id,
            self.qs.serialize_string(request)?
        );
        self.get(&url).await
    }

    /// Get pages in search history
    #[instrument(skip(self))]
    pub async fn get_search_urls_history(
        &self,
        host_id: &str,
        request: &IndexingHistoryRequest,
    ) -> Result<SearchUrlsHistoryResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/search-urls/in-search/history?{}",
            API_BASE_URL,
            self.user_id,
            host_id,
            self.qs.serialize_string(request)?
        );
        self.get(&url).await
    }

    /// Get sample pages in search
    #[instrument(skip(self))]
    pub async fn get_search_urls_samples(
        &self,
        host_id: &str,
        request: &GetSearchUrlsSamplesRequest,
    ) -> Result<SearchUrlsSamplesResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/search-urls/in-search/samples?{}",
            API_BASE_URL,
            self.user_id,
            host_id,
            self.qs.serialize_string(request)?
        );
        self.get(&url).await
    }

    /// Get page appearance/removal history
    #[instrument(skip(self))]
    pub async fn get_search_events_history(
        &self,
        host_id: &str,
        request: &IndexingHistoryRequest,
    ) -> Result<SearchEventsHistoryResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/search-urls/events/history?{}",
            API_BASE_URL,
            self.user_id,
            host_id,
            self.qs.serialize_string(request)?
        );
        self.get(&url).await
    }

    /// Get sample page changes
    #[instrument(skip(self))]
    pub async fn get_search_events_samples(
        &self,
        host_id: &str,
        request: &GetSearchEventsSamplesRequest,
    ) -> Result<SearchEventsSamplesResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/search-urls/events/samples?{}",
            API_BASE_URL,
            self.user_id,
            host_id,
            self.qs.serialize_string(request)?
        );
        self.get(&url).await
    }

    // ============================================================================
    // Important URLs
    // ============================================================================

    /// Get list of important URLs
    #[instrument(skip(self))]
    pub async fn get_important_urls(&self, host_id: &str) -> Result<ImportantUrlsResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/important-urls",
            API_BASE_URL, self.user_id, host_id
        );
        self.get(&url).await
    }

    /// Get important URLs history
    #[instrument(skip(self))]
    pub async fn get_important_urls_history(
        &self,
        host_id: &str,
        url_param: &str,
    ) -> Result<ImportantUrlHistoryResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/important-urls/history?url={}",
            API_BASE_URL,
            self.user_id,
            host_id,
            urlencoding::encode(url_param)
        );
        self.get(&url).await
    }

    // ============================================================================
    // Recrawl Management
    // ============================================================================

    /// Request page recrawl
    #[instrument(skip(self))]
    pub async fn recrawl_urls(
        &self,
        host_id: &str,
        request: &RecrawlRequest,
    ) -> Result<RecrawlResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/recrawl/queue",
            API_BASE_URL, self.user_id, host_id
        );
        self.post(&url, request).await
    }

    /// Get list of recrawl tasks
    #[instrument(skip(self))]
    pub async fn get_recrawl_tasks(
        &self,
        host_id: &str,
        request: &GetRecrawlTasksRequest,
    ) -> Result<RecrawlTasksResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/recrawl/queue?{}",
            API_BASE_URL,
            self.user_id,
            host_id,
            self.qs.serialize_string(request)?
        );
        self.get(&url).await
    }

    /// Get recrawl task status
    #[instrument(skip(self))]
    pub async fn get_recrawl_task(&self, host_id: &str, task_id: &str) -> Result<RecrawlTask> {
        let url = format!(
            "{}/user/{}/hosts/{}/recrawl/queue/{}",
            API_BASE_URL, self.user_id, host_id, task_id
        );
        self.get(&url).await
    }

    /// Get recrawl quota
    #[instrument(skip(self))]
    pub async fn get_recrawl_quota(&self, host_id: &str) -> Result<RecrawlQuotaResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/recrawl/quota",
            API_BASE_URL, self.user_id, host_id
        );
        self.get(&url).await
    }

    // ============================================================================
    // Links
    // ============================================================================

    /// Get broken internal links samples
    #[instrument(skip(self))]
    pub async fn get_broken_links(&self, host_id: &str) -> Result<BrokenLinksResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/links/internal/broken/samples",
            API_BASE_URL, self.user_id, host_id
        );
        self.get(&url).await
    }

    /// Get broken links history
    #[instrument(skip(self))]
    pub async fn get_broken_links_history(
        &self,
        host_id: &str,
        request: &IndexingHistoryRequest,
    ) -> Result<IndexingHistoryResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/links/internal/broken/history",
            API_BASE_URL, self.user_id, host_id
        );
        self.post(&url, request).await
    }

    /// Get external backlinks samples
    #[instrument(skip(self))]
    pub async fn get_external_links(&self, host_id: &str) -> Result<ExternalLinksResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/links/external/samples",
            API_BASE_URL, self.user_id, host_id
        );
        self.get(&url).await
    }

    /// Get backlinks history
    #[instrument(skip(self))]
    pub async fn get_external_links_history(
        &self,
        host_id: &str,
        request: &IndexingHistoryRequest,
    ) -> Result<IndexingHistoryResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/links/external/history",
            API_BASE_URL, self.user_id, host_id
        );
        self.post(&url, request).await
    }

    // ============================================================================
    // Diagnostics
    // ============================================================================

    /// Get site diagnostic report
    #[instrument(skip(self))]
    pub async fn get_diagnostics(&self, host_id: &str) -> Result<DiagnosticsResponse> {
        let url = format!(
            "{}/user/{}/hosts/{}/diagnostics",
            API_BASE_URL, self.user_id, host_id
        );
        self.get(&url).await
    }

    // ============================================================================
    // Helper Methods
    // ============================================================================

    /// Generic GET request helper
    #[instrument(skip(self))]
    async fn get<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T> {
        tracing::debug!(url = %url, "Making GET request");

        let response = self.client.get(url).send().await?;

        Self::handle_response(response).await
    }

    /// Generic POST request helper
    #[instrument(skip(self, body))]
    async fn post<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<T> {
        tracing::debug!(url = %url, "Making POST request");

        let json_body = serde_json::to_string(body)?;

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(json_body)
            .send()
            .await?;

        Self::handle_response(response).await
    }

    /// Generic DELETE request helper
    #[instrument(skip(self))]
    async fn delete(&self, url: &str) -> Result<()> {
        tracing::debug!(url = %url, "Making DELETE request");

        let response = self.client.delete(url).send().await?;

        if !response.status().is_success() {
            return Err(Self::parse_error(response).await);
        }

        Ok(())
    }

    /// Parse API error response
    #[instrument(skip(response))]
    async fn parse_error(response: reqwest::Response) -> YandexWebmasterError {
        let status = response.status();
        let status_code = status.as_u16();

        // Try to parse structured error response
        match response.text().await {
            Ok(error_text) => {
                // Try to parse as structured Yandex API error
                match serde_json::from_str::<YandexApiErrorResponse>(&error_text) {
                    Ok(api_error) => {
                        tracing::error!(
                            status = %status,
                            error_code = %api_error.error_code,
                            error_message = %api_error.error_message,
                            "Structured API error"
                        );
                        YandexWebmasterError::ApiError {
                            status: status_code,
                            response: api_error,
                        }
                    }
                    Err(_) => {
                        // Fallback to generic error
                        tracing::error!(
                            status = %status,
                            error = %error_text,
                            "API request failed with unstructured error"
                        );
                        YandexWebmasterError::GenericApiError(format!(
                            "Status: {}, Error: {}",
                            status, error_text
                        ))
                    }
                }
            }
            Err(e) => {
                tracing::error!(
                    status = %status,
                    error = %e,
                    "Failed to read error response"
                );
                YandexWebmasterError::GenericApiError(format!(
                    "Status: {}, Failed to read error response: {}",
                    status, e
                ))
            }
        }
    }

    /// Handle API response
    #[instrument(skip(response))]
    async fn handle_response<T: serde::de::DeserializeOwned>(
        response: reqwest::Response,
    ) -> Result<T> {
        if !response.status().is_success() {
            return Err(Self::parse_error(response).await);
        }

        let data: T = response.json().await?;
        Ok(data)
    }
}
