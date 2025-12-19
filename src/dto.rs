use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// User
// ============================================================================

/// Response from the user endpoint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserResponse {
    /// ID of the user. Required to call any Yandex Webmaster API resources.
    pub user_id: i64,
}

// ============================================================================
// Hosts (Sites)
// ============================================================================

/// Response containing a list of hosts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostsResponse {
    /// List of hosts
    pub hosts: Vec<HostInfo>,
}

/// Site indexing status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HostDataStatus {
    /// The site isn't indexed yet.
    NotIndexed,
    /// The site data isn't uploaded to Yandex.Webmaster yet.
    NotLoaded,
    /// The site is indexed. The data is available in Yandex.Webmaster.
    Ok,
}

/// Information about a host
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostInfo {
    /// Site identifier
    pub host_id: String,
    /// ASCII-encoded site URL
    pub ascii_host_url: String,
    /// UTF-8 encoded site URL
    pub unicode_host_url: String,
    /// Ownership verification status
    pub verified: bool,
    /// Primary site address (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_mirror: Option<Box<HostInfo>>,
}

/// Information about a host from `get_host` method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullHostInfo {
    /// Site identifier
    pub host_id: String,
    /// ASCII-encoded site URL
    pub ascii_host_url: String,
    /// UTF-8 encoded site URL
    pub unicode_host_url: String,
    /// Ownership verification status
    pub verified: bool,
    /// Primary site address (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_mirror: Option<Box<HostInfo>>,
    /// Information about the site (shown if the site is verified).
    pub host_data_status: Option<HostDataStatus>,
    /// The site name to display
    pub host_display_name: Option<String>,
}

/// Request to add a new host
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddHostRequest {
    /// Host URL to add
    pub host_url: String,
    /// Verification type
    pub verification_type: VerificationType,
}

/// Response from adding a new host
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddHostResponse {
    /// Assigned host ID
    pub host_id: String,
}

// ============================================================================
// Host Verification
// ============================================================================

/// Error description if the VERIFICATION_FAILED status is received.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FailInfo {
    /// The reason why verification failed.
    pub message: String,
    /// Error description for users.
    pub reason: VerificationFailReason,
}
/// Host verification status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostVerificationStatusResponse {
    /// Verification state
    pub verification_state: VerificationState,
    /// Verification type
    pub verification_type: VerificationType,
    /// Verification token (for DNS and HTML methods)
    pub verification_uin: String,
    /// The verification methods applied for the given site.
    pub applicable_verifiers: Vec<ExplicitVerificationType>,
    /// The time of the last check (if verification_state isn't NONE).
    pub latest_verification_time: Option<DateTime<Utc>>,
    /// Error description if the VERIFICATION_FAILED status is received.
    pub fail_info: Option<FailInfo>,
}

/// Host verification status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostVerificationResponse {
    /// Verification state
    pub verification_state: VerificationState,
    /// Verification type
    pub verification_type: VerificationType,
    /// Verification token (for DNS and HTML methods)
    pub verification_uin: String,
    /// The verification methods applied for the given site.
    pub applicable_verifiers: Vec<ExplicitVerificationType>,
}

/// Verification state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationState {
    /// Not verified
    None,
    /// Rights confirmed
    Verified,
    /// Verification pending
    InProgress,
    /// Rights not confirmed
    VerificationFailed,
    /// System error during verification
    InternalError,
}

/// Verification type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExplicitVerificationType {
    /// DNS record verification
    Dns,
    /// Meta tag verification
    MetaTag,
    /// HTML file verification
    HtmlFile,
}

/// Verification type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationType {
    /// Automatic rights verification (deprecated; only for *.narod.ru sites).
    Auto,
    /// Rights were delegated.
    Delegated,
    /// Rights verification via Yandex.Mail for Domains.
    Pdd,
    /// Placing a text file in the site's root directory.
    TxtFile,
    /// DNS record verification
    Dns,
    /// Meta tag verification
    MetaTag,
    /// HTML file verification
    HtmlFile,
}

/// Verification failure reason
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationFailReason {
    /// Rights delegation revoked
    DelegationCancelled,
    /// Missing DNS entry
    DnsRecordNotFound,
    /// Missing meta tag in homepage header
    MetaTagNotFound,
    /// Incorrect HTML file content
    WrongHtmlPageContent,
    /// Verification of site management rights via Yandex.Mail for Domain isn't allowed for this site.
    PddVerificationCancelled,
}

/// List of verified owners
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OwnersResponse {
    /// List of owners
    pub users: Vec<Owner>,
}

/// Owner information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Owner {
    /// User login
    pub user_login: String,
    /// Confirmation code
    pub verification_uin: String,
    /// Rights verification method
    pub verification_type: VerificationType,
    /// Verification date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_date: Option<DateTime<Utc>>,
}

// ============================================================================
// Host Summary and Statistics
// ============================================================================

/// Site statistics summary
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HostSummaryResponse {
    /// Site quality index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqi: Option<f64>,
    /// Indexing statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexing: Option<IndexingStats>,
    /// Search query statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_queries: Option<SearchQueriesStats>,
}

/// Indexing statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexingStats {
    /// Total pages indexed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    /// Pages in search results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages_in_search: Option<i64>,
}

/// Search queries statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchQueriesStats {
    /// Total queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_queries: Option<i64>,
    /// Total shows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_shows: Option<i64>,
    /// Total clicks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_clicks: Option<i64>,
}

/// Site quality index history request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SqiHistoryRequest {
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
}

/// Site quality index history
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SqiHistoryResponse {
    /// History points
    pub points: Vec<SqiPoint>,
}

/// Single SQI history point
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SqiPoint {
    /// Date
    pub date: DateTime<Utc>,
    /// SQI value
    pub value: f64,
}

// ============================================================================
// Search Queries
// ============================================================================

/// Query sorting order field
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiQueryOrderField {
    /// Sort by total shows
    TotalShows,
    /// Sort by total clicks
    TotalClicks,
}

/// Query indicators
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiQueryIndicator {
    /// Total number of shows
    TotalShows,
    /// Total number of clicks
    TotalClicks,
    /// Average show position
    AvgShowPosition,
    /// Average click position
    AvgClickPosition,
}

/// Device type indicator
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiDeviceTypeIndicator {
    /// All device types
    #[default]
    All,
    /// Desktop computers
    Desktop,
    /// Mobile phones and tablets
    MobileAndTablet,
    /// Mobile phones only
    Mobile,
    /// Tablets only
    Tablet,
}

/// Popular queries request parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PopularQueriesRequest {
    /// Indicator for sorting requests (required)
    pub order_by: ApiQueryOrderField,
    /// Indicators for displaying requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_indicator: Option<ApiQueryIndicator>,
    /// Device type indicator (default: ALL)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_indicator: Option<ApiDeviceTypeIndicator>,
    /// Start date of the range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<NaiveDate>,
    /// End date of the range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<NaiveDate>,
    /// List offset (minimum: 0, default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Page size (1-500, default: 500)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Popular search queries response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PopularQueriesResponse {
    /// List of queries
    pub queries: Vec<PopularQuery>,
    /// Start date of the range
    pub date_from: NaiveDate,
    /// End date of the range
    pub date_to: NaiveDate,
    /// Total number of search queries available
    pub count: i32,
}

/// Popular query information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PopularQuery {
    /// Query ID
    pub query_id: String,
    /// Query text
    pub query_text: String,
    /// Query indicators (e.g., TOTAL_SHOWS, TOTAL_CLICKS, etc.)
    pub indicators: std::collections::HashMap<ApiQueryIndicator, f64>,
}

/// Query analytics request parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QueryAnalyticsRequest {
    /// Indicators for displaying requests (can specify multiple)
    pub query_indicator: Vec<ApiQueryIndicator>,
    /// Device type indicator (default: ALL)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_indicator: Option<ApiDeviceTypeIndicator>,
    /// Start date of the range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<DateTime<Utc>>,
    /// End date of the range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<DateTime<Utc>>,
}

/// Query analytics response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryAnalyticsResponse {
    /// Map of indicators to their history points
    pub indicators: std::collections::HashMap<ApiQueryIndicator, Vec<IndicatorPoint>>,
}

/// Single indicator history point
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndicatorPoint {
    /// Date
    pub date: DateTime<Utc>,
    /// Value
    pub value: f64,
}

/// Query history request parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QueryHistoryRequest {
    /// Indicators for displaying requests (can specify multiple)
    pub query_indicator: Vec<ApiQueryIndicator>,
    /// Device type indicator (default: ALL)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_indicator: Option<ApiDeviceTypeIndicator>,
    /// Start date of the range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<NaiveDate>,
    /// End date of the range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<NaiveDate>,
}

/// Query history response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryHistoryResponse {
    /// Search query ID
    pub query_id: String,
    /// Search query text
    pub query_text: String,
    /// Map of indicators to their history points
    pub indicators: std::collections::HashMap<ApiQueryIndicator, Vec<IndicatorPoint>>,
}

// ============================================================================
// Sitemaps
// ============================================================================

/// List of sitemaps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SitemapsResponse {
    /// Sitemaps
    pub sitemaps: Vec<SitemapInfo>,
}

/// Sitemap information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SitemapInfo {
    /// Sitemap ID
    pub sitemap_id: String,
    /// Sitemap URL
    pub sitemap_url: String,
    /// Last access date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_date: Option<DateTime<Utc>>,
    /// Number of URLs in sitemap
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls_count: Option<i64>,
}

/// Request to add a sitemap
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddSitemapRequest {
    /// Sitemap URL
    pub url: String,
}

/// Response from adding a sitemap
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddSitemapResponse {
    /// Assigned sitemap ID
    pub sitemap_id: String,
}

// ============================================================================
// Indexing
// ============================================================================

/// Indexing history request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexingHistoryRequest {
    /// Date from (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// Date to (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
}

/// Indexing history response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexingHistoryResponse {
    /// History indicators
    pub indicators: Vec<IndexingIndicator>,
}

/// Indexing indicator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexingIndicator {
    /// Indicator type
    pub indicator: String,
    /// History points
    pub points: Vec<IndexingPoint>,
}

/// Indexing history point
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexingPoint {
    /// Date
    pub date: DateTime<Utc>,
    /// Value
    pub value: i64,
}

/// Indexing samples response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexingSamplesResponse {
    /// Sample URLs
    pub samples: Vec<UrlSample>,
}

/// URL sample
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UrlSample {
    /// URL
    pub url: String,
    /// Last access date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access: Option<DateTime<Utc>>,
}

// ============================================================================
// Recrawl
// ============================================================================

/// Request to recrawl URLs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecrawlRequest {
    /// URLs to recrawl
    pub urls: Vec<String>,
}

/// Response from recrawl request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecrawlResponse {
    /// Task ID
    pub task_id: String,
    /// Number of URLs in queue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_remainder: Option<i64>,
}

/// Recrawl task list response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecrawlTasksResponse {
    /// Tasks
    pub tasks: Vec<RecrawlTask>,
}

/// Recrawl task information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecrawlTask {
    /// Task ID
    pub task_id: String,
    /// Task state
    pub state: RecrawlTaskState,
    /// Added date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_date: Option<DateTime<Utc>>,
}

/// Recrawl task state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecrawlTaskState {
    /// Task in queue
    InQueue,
    /// Task processing
    Processing,
    /// Task completed
    Completed,
    /// Task failed
    Failed,
}

/// Recrawl quota response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecrawlQuotaResponse {
    /// Daily quota
    pub quota: i64,
    /// Quota remainder
    pub quota_remainder: i64,
}

// ============================================================================
// Links
// ============================================================================

/// Broken internal links samples
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BrokenLinksResponse {
    /// Samples
    pub samples: Vec<BrokenLink>,
}

/// Broken link information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BrokenLink {
    /// Source URL
    pub source_url: String,
    /// Destination URL
    pub destination_url: String,
    /// Last check date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_check: Option<DateTime<Utc>>,
}

/// External backlinks samples
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalLinksResponse {
    /// Samples
    pub samples: Vec<ExternalLink>,
}

/// External link information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalLink {
    /// Source URL
    pub source_url: String,
    /// Destination URL
    pub destination_url: String,
    /// Discovery date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovered_date: Option<DateTime<Utc>>,
}

// ============================================================================
// Diagnostics
// ============================================================================

/// Site diagnostics response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagnosticsResponse {
    /// Diagnostic items
    pub items: Vec<DiagnosticItem>,
}

/// Diagnostic item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagnosticItem {
    /// Item type
    pub item_type: String,
    /// Severity level
    pub severity: DiagnosticSeverity,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Diagnostic severity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DiagnosticSeverity {
    /// Critical issue
    Critical,
    /// Warning
    Warning,
    /// Informational
    Info,
}

// ============================================================================
// Important URLs
// ============================================================================

/// Important URLs response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportantUrlsResponse {
    /// URLs
    pub urls: Vec<ImportantUrl>,
}

/// Important URL information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportantUrl {
    /// URL
    pub url: String,
    /// Priority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
