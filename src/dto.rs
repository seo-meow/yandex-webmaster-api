use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    /// Number of searchable pages
    #[serde(default)]
    pub searchable_pages_count: i64,
    /// Number of excluded pages
    #[serde(default)]
    pub excluded_pages_count: i64,
    /// Site problems grouped by severity
    #[serde(default)]
    pub site_problems: HashMap<SiteProblemSeverityEnum, i32>,
}

/// Excluded pages statistics by status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExcludedPagesStatistics {
    /// Statistics by status
    pub statuses: HashMap<ApiExcludedUrlStatus, i64>,
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

/// Source of the Sitemap file
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiSitemapSource {
    /// Sitemap is specified in the site's robots.txt file
    RobotsTxt,
    /// The user added the Sitemap in Yandex.Webmaster
    Webmaster,
    /// Sitemap found in another (index) Sitemap file
    IndexSitemap,
}

/// Type of Sitemap file
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiSitemapType {
    /// Normal Sitemap file that contains the URLs of site pages
    Sitemap,
    /// The Sitemap index file that contains the URLs of other Sitemap files
    IndexSitemap,
}

/// Request parameters for getting sitemaps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct GetSitemapsRequest {
    /// Parent sitemap ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// Page size (1-100, default: 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Get sitemaps starting after this ID (not including it)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}

/// List of sitemaps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SitemapsResponse {
    /// Sitemaps
    pub sitemaps: Vec<SitemapInfo>,
}

/// Sitemap information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SitemapInfo {
    /// Sitemap ID
    pub sitemap_id: String,
    /// Sitemap URL
    pub sitemap_url: String,
    /// Last access date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_date: Option<DateTime<Utc>>,
    /// Number of errors in the file
    pub errors_count: i32,
    /// Number of URLs in the file
    pub urls_count: i64,
    /// Number of child Sitemap files
    pub children_count: i32,
    /// Sources that led the robot to this file
    pub sources: Vec<ApiSitemapSource>,
    /// Type of the Sitemap file
    pub sitemap_type: ApiSitemapType,
}

/// Request parameters for getting user-added sitemaps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct GetUserSitemapsRequest {
    /// Get files starting from the specified one (not including it, default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Page size (1-100, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// List of user-added sitemaps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserSitemapsResponse {
    /// Sitemaps
    pub sitemaps: Vec<UserSitemapInfo>,
    /// Total number of Sitemap files added by the user
    pub count: i32,
}

/// User-added sitemap information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserSitemapInfo {
    /// Sitemap ID
    pub sitemap_id: String,
    /// Sitemap URL
    pub sitemap_url: String,
    /// Date the file was added
    pub added_date: DateTime<Utc>,
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

/// Indexing status by HTTP code
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndexingStatusEnum {
    /// HTTP 2xx responses
    #[serde(rename = "HTTP_2XX")]
    Http2xx,
    /// HTTP 3xx responses
    #[serde(rename = "HTTP_3XX")]
    Http3xx,
    /// HTTP 4xx responses
    #[serde(rename = "HTTP_4XX")]
    Http4xx,
    /// HTTP 5xx responses
    #[serde(rename = "HTTP_5XX")]
    Http5xx,
    /// Other statuses
    Other,
}

/// Site problem severity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SiteProblemSeverityEnum {
    /// Fatal problems
    Fatal,
    /// Critical problems
    Critical,
    /// Possible problems
    PossibleProblem,
    /// Recommendations
    Recommendation,
}

/// Excluded URL status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiExcludedUrlStatus {
    /// No exclusion found - robot doesn't know about page or it was unavailable
    NothingFound,
    /// Could not connect to server when accessing site
    HostError,
    /// Page redirects to another page (target page is indexed)
    RedirectNotsearchable,
    /// HTTP error occurred when accessing page
    HttpError,
    /// Page indexed by canonical URL specified in rel="canonical"
    NotCanonical,
    /// Page belongs to secondary site mirror
    NotMainMirror,
    /// Robot couldn't get page content
    ParserError,
    /// Site indexing prohibited in robots.txt
    RobotsHostError,
    /// Page indexing prohibited in robots.txt
    RobotsUrlError,
    /// Page duplicates a site page already in search
    Duplicate,
    /// Page excluded after robot processed Clean-param directive
    CleanParams,
    /// Page excluded because robots meta tag has noindex value
    NoIndex,
    /// Forbidden by robots.txt (legacy)
    ForbiddenByRobotsTxt,
    /// URL not allowed (legacy)
    UrlNotAllowed,
    /// Contains noindex meta tag (legacy)
    ContainsNoindexMetaTag,
    /// Contains noindex X-Robots-Tag header (legacy)
    ContainsNoindexXRobotsTagHeader,
    /// Sitemap forbidden (legacy)
    SitemapForbidden,
    /// Sitemap not allowed (legacy)
    SitemapNotAllowed,
    /// Low quality - removed due to low quality
    LowQuality,
    /// Alternative duplicate (legacy)
    AlternativeDuplicate,
    /// User duplicate (legacy)
    UserDuplicate,
    /// Canonical duplicate (legacy)
    CanonicalDuplicate,
    /// Redirect duplicate (legacy)
    RedirectDuplicate,
    /// Moved permanently (legacy)
    MovedPermanently,
    /// Moved temporarily (legacy)
    MovedTemporarily,
    /// Malware detected (legacy)
    MalwareDetected,
    /// Phishing detected (legacy)
    PhishingDetected,
    /// Adult content (legacy)
    AdultContent,
    /// Other reason - robot doesn't have updated data
    Other,
}

/// Important URL change indicator
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiImportantUrlChangeIndicator {
    /// Indexing HTTP code
    IndexingHttpCode,
    /// Search status
    SearchStatus,
    /// Page title
    Title,
    /// Page description
    Description,
}

/// Indexing history request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct IndexingHistoryRequest {
    /// Date from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<DateTime<Utc>>,
    /// Date to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<DateTime<Utc>>,
}

/// Indexing history response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexingHistoryResponse {
    /// History indicators by status
    pub indicators: HashMap<IndexingStatusEnum, Vec<IndexingHistoryPoint>>,
}

/// Indexing history point
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexingHistoryPoint {
    /// Date
    pub date: DateTime<Utc>,
    /// Value
    pub value: f64,
}

/// Get indexing samples request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct GetIndexingSamplesRequest {
    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Limit for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Indexing samples response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexingSamplesResponse {
    /// Sample URLs
    pub samples: Vec<IndexingSample>,
    /// Total count
    pub count: i32,
}

/// Indexing sample
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexingSample {
    /// URL
    pub url: String,
    /// HTTP status code
    pub http_code: i32,
    /// Last access date
    pub access_date: DateTime<Utc>,
}

// ============================================================================
// Search URLs (Pages in Search)
// ============================================================================

/// Search event type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiSearchEventEnum {
    /// Page appeared in search results
    AppearedInSearch,
    /// Page removed from search results
    RemovedFromSearch,
}

/// Search URLs history response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchUrlsHistoryResponse {
    /// History points
    pub history: Vec<SearchUrlsHistoryPoint>,
}

/// Search URLs history point
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchUrlsHistoryPoint {
    /// Date and time when search output was updated
    pub date: DateTime<Utc>,
    /// Number of pages in search
    pub value: i64,
}

/// Get search URLs samples request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct GetSearchUrlsSamplesRequest {
    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Limit for pagination (1-100, default 50)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Search URLs samples response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchUrlsSamplesResponse {
    /// Total number of available examples
    pub count: i32,
    /// Sample pages
    pub samples: Vec<SearchUrlsSample>,
}

/// Search URLs sample
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchUrlsSample {
    /// Page URL
    pub url: String,
    /// Date of the page version in search
    pub last_access: DateTime<Utc>,
    /// Page heading
    pub title: String,
}

/// Search events history response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchEventsHistoryResponse {
    /// History indicators by event type
    pub indicators: HashMap<ApiSearchEventEnum, Vec<SearchUrlsHistoryPoint>>,
}

/// Get search events samples request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct GetSearchEventsSamplesRequest {
    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Limit for pagination (1-100, default 50)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Search events samples response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchEventsSamplesResponse {
    /// Total number of available examples
    pub count: i32,
    /// Sample pages
    pub samples: Vec<SearchEventsSample>,
}

/// Search events sample
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchEventsSample {
    /// Page URL
    pub url: String,
    /// Page heading
    pub title: String,
    /// Date when page appeared or was excluded
    pub event_date: DateTime<Utc>,
    /// Date when page was last crawled before appearing or being excluded
    pub last_access: DateTime<Utc>,
    /// The appearance or removal of the page
    pub event: ApiSearchEventEnum,
    /// Reason the page was excluded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_url_status: Option<ApiExcludedUrlStatus>,
    /// Page's HTTP response code for HTTP_ERROR status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bad_http_status: Option<i32>,
    /// Another address of the page (redirect target, canonical, or duplicate)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
}

// ============================================================================
// Recrawl (Reindexing)
// ============================================================================

/// Request to recrawl a URL
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecrawlRequest {
    /// URL of the page to be sent for reindexing
    pub url: String,
}

/// Response from recrawl request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecrawlResponse {
    /// Task ID
    pub task_id: String,
}

/// Get recrawl tasks request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetRecrawlTasksRequest {
    /// Offset in the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Page size (minimum 1, default 50)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Start of the date range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<DateTime<Utc>>,
    /// End of the date range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<DateTime<Utc>>,
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
    /// URL of the page sent for reindexing
    pub url: String,
    /// Date the reindexing request was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_time: Option<DateTime<Utc>>,
    /// Status of the reindexing request
    pub state: RecrawlTaskState,
}

/// Recrawl task state (reindexing request status)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecrawlTaskState {
    /// Request is being processed
    InProgress,
    /// Robot crawled the URL
    Done,
    /// Robot failed to crawl the page
    Failed,
}

/// Recrawl quota response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecrawlQuotaResponse {
    /// Daily quota
    pub daily_quota: i32,
    /// Remainder of daily quota
    pub quota_remainder: i32,
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

/// Site problem type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiSiteProblemTypeEnum {
    // FATAL
    /// Robots couldn't visit the site (server settings or high load)
    ConnectFailed,
    /// Site prohibited for indexing in robots.txt
    DisallowedInRobots,
    /// Failed to connect to server due to DNS error
    DnsError,
    /// Site's home page returns an error
    MainPageError,
    /// Security threats or issues detected
    Threats,

    // CRITICAL
    /// Some pages with GET parameters duplicate content of other pages
    InsignificantCgiParameter,
    /// Slow server response
    SlowAvgResponseTime,
    /// Invalid SSL certificate settings
    SslCertificateError,
    /// Some pages respond with 4xx HTTP code for an hour
    #[serde(rename = "URL_ALERT_4XX")]
    UrlAlert4xx,
    /// Some pages respond with 5xx HTTP code for an hour
    #[serde(rename = "URL_ALERT_5XX")]
    UrlAlert5xx,

    // POSSIBLE_PROBLEM
    /// Useful pages found that are closed from indexing
    DisallowedUrlsAlert,
    /// Many pages missing Description meta tag
    DocumentsMissingDescription,
    /// Title element missing on many pages
    DocumentsMissingTitle,
    /// Some pages have identical title and Description
    DuplicateContentAttrs,
    /// Some pages contain identical content
    DuplicatePages,
    /// Errors in robots.txt file
    ErrorInRobotsTxt,
    /// Errors found in Sitemap file
    ErrorsInSitemaps,
    /// Favicon file unavailable on site
    FaviconError,
    /// Main mirror doesn't use HTTPS protocol
    MainMirrorIsNotHttps,
    /// Main page redirects to another site
    MainPageRedirects,
    /// No Yandex.Metrica counter linked to site
    NoMetrikaCounterBinding,
    /// Site crawling using Yandex.Metrica counters not enabled
    NoMetrikaCounterCrawlEnabled,
    /// robots.txt file not found
    NoRobotsTxt,
    /// No Sitemap files used by robot
    NoSitemaps,
    /// Sitemap files haven't been updated for a long time
    NoSitemapModifications,
    /// Robot failed to index marked videos on site
    NonWorkingVideo,
    /// Display of non-existent files and pages configured incorrectly
    #[serde(rename = "SOFT_404")]
    Soft404,
    /// Site subdomains found in search results
    TooManyDomainsOnSearch,
    /// User agreement for video display added to Webmaster was rejected
    VideohostOfferFailed,
    /// User agreement for video display missing for site
    VideohostOfferIsNeeded,
    /// Special agreement with Yandex needed for site cooperation
    VideohostOfferNeedPaper,

    // RECOMMENDATION
    /// Add favicon in SVG format or 120×120 pixels size
    BigFaviconAbsent,
    /// Favicon file not found - robot couldn't load image for browser tab
    FaviconProblem,
    /// Yandex.Metrica counter error
    NoMetrikaCounter,
    /// Site region not set
    NoRegions,
    /// Site not registered in Yandex.Directory
    NotInSprav,
    /// Site not optimized for mobile devices
    NotMobileFriendly,
    /// Yandex.Vygoda not connected to site
    VygodaPossibleActivation,
}

/// Site problem state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiSiteProblemState {
    /// Present on the site
    Present,
    /// Missing
    Absent,
    /// Not enough data to determine if there are issues
    Undefined,
}

/// Site diagnostics response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagnosticsResponse {
    /// Problems by type
    pub problems: HashMap<ApiSiteProblemTypeEnum, SiteProblemInfo>,
}

/// Site problem information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SiteProblemInfo {
    /// Issue type (severity)
    pub severity: SiteProblemSeverityEnum,
    /// State of the issue
    pub state: ApiSiteProblemState,
    /// Date the issue status was last changed
    pub last_state_update: Option<DateTime<Utc>>,
}

// ============================================================================
// Important URLs
// ============================================================================

/// Important URLs response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportantUrlsResponse {
    /// URLs
    pub urls: Vec<ImportantUrl>,
}

/// Important URL information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportantUrl {
    /// Site page URL
    pub url: String,
    /// Date and time the page status information was updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<DateTime<Utc>>,
    /// Indicator of changes from previous check
    #[serde(default)]
    pub change_indicators: Vec<ApiImportantUrlChangeIndicator>,
    /// Information about page indexing by the robot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexing_status: Option<IndexingStatus>,
    /// State of the page in search results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_status: Option<SearchStatus>,
}

/// Page indexing status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexingStatus {
    /// Generalized status of the HTTP code
    pub status: IndexingStatusEnum,
    /// HTTP code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_code: Option<i32>,
    /// Date the page was crawled
    pub access_date: DateTime<Utc>,
}

/// Page search status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchStatus {
    /// Page heading
    pub title: String,
    /// Description meta tag content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Date when page was last crawled before appearing or being excluded
    pub last_access: DateTime<Utc>,
    /// Reason the page was excluded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_url_status: Option<ApiExcludedUrlStatus>,
    /// Page's HTTP response code for HTTP_ERROR status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bad_http_status: Option<i32>,
    /// Whether page is present in search results
    pub searchable: bool,
    /// Another address of the page (redirect target, canonical, or duplicate)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
}

/// Important URL history response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportantUrlHistoryResponse {
    /// History of changes to the page
    pub history: Vec<ImportantUrl>,
}
