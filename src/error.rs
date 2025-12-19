use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Yandex API error codes
///
/// This enum represents all possible error codes that can be returned by the Yandex Webmaster API.
/// Each variant corresponds to a specific error condition documented in the API specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum YandexErrorCode {
    // 400 Bad Request
    EmptyDates,
    EmptyPaths,
    EntityValidationError,
    FieldValidationError,
    InvalidUrl,
    NoChanges,
    SomeDatesAreUnavailable,
    UrlsAreCorrupted,
    WrongRegion,

    // 403 Forbidden
    AccessForbidden,
    InvalidOauthToken,
    InvalidUserId,
    HostsLimitExceeded,
    FeedsLimitExceeded,
    BatchLimitExceeded,
    FeedsCategoryBan,
    LimitsExceeded,

    // 404 Not Found
    ResourceNotFound,
    HostNotIndexed,
    HostNotLoaded,
    HostNotVerified,
    HostNotFound,
    SitemapNotFound,
    SitemapNotAdded,
    TaskNotFound,
    QueryIdNotFound,
    BadHttpCode,
    BadMimeType,
    RequestNotFound,
    TimedOut,
    FeedAlreadyAdded,
    OnlyHttps,
    ManyUrlsForRemove,
    IncorrectUrl,
    NotExist,

    // 405 Method Not Allowed
    MethodNotAllowed,

    // 406 Not Acceptable
    ContentTypeUnsupported,

    // 409 Conflict
    UrlAlreadyAdded,
    HostAlreadyAdded,
    VerificationAlreadyInProgress,
    TextAlreadyAdded,
    SitemapAlreadyAdded,

    // 410 Gone
    UploadAddressExpired,

    // 413 Request Entity Too Large
    RequestEntityTooLarge,
    PayloadTooLarge,

    // 415 Unsupported Media Type
    ContentEncodingUnsupported,

    // 422 Unprocessable Entity
    TextLengthConstraintsViolation,
    NoVerificationRecord,

    // 429 Too Many Requests
    QuotaExceeded,
    TooManyRequestsError,

    /// Unknown error code not listed in the API documentation
    #[serde(untagged)]
    Unknown(String),
}

impl fmt::Display for YandexErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YandexErrorCode::Unknown(s) => write!(f, "{}", s),
            _ => {
                let json = serde_json::to_string(self).unwrap_or_else(|_| "UNKNOWN".to_string());
                write!(f, "{}", json.trim_matches('"'))
            }
        }
    }
}

/// Response structure for Yandex API errors
///
/// This struct represents the error response format returned by the Yandex Webmaster API.
/// It includes the error code and a human-readable error message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YandexApiErrorResponse {
    /// Error code identifying the specific error condition
    pub error_code: YandexErrorCode,

    /// Human-readable error message providing additional context
    pub error_message: String,

    /// Optional list of acceptable content types (for 406 errors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptable_types: Option<Vec<String>>,

    /// Optional expiration date (for 410 errors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
}

/// Errors that can occur when interacting with the Yandex Webmaster API
#[derive(Debug, Error)]
pub enum YandexWebmasterError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Middleware request failed
    #[error("Middleware request failed: {0}")]
    MiddlewareHttpError(#[from] reqwest_middleware::Error),

    /// Failed to parse response
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),

    /// Failed to serialize url
    #[error("Failed serialize url: {0}")]
    SerdeQsError(#[from] serde_qs::Error),

    /// Middleware error
    #[error("Middleware error: {0}")]
    MiddlewareError(String),

    /// Authentication failed
    #[error("Authentication failed: missing or invalid OAuth token")]
    AuthenticationError,

    /// API returned a structured error (RFC 7807 compliant)
    #[error("API error ({error_code}): {error_message}", error_code = .response.error_code, error_message = .response.error_message)]
    ApiError {
        /// HTTP status code
        status: u16,
        /// Yandex API error response
        response: YandexApiErrorResponse,
    },

    /// API returned an unstructured error
    #[error("API error: {0}")]
    GenericApiError(String),
}

/// Result type alias for Yandex Webmaster API operations
pub type Result<T> = std::result::Result<T, YandexWebmasterError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entity_validation_error() {
        let json = r#"{
            "error_code": "ENTITY_VALIDATION_ERROR",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::EntityValidationError);
        assert_eq!(result.error_message, "some string");
        assert!(result.acceptable_types.is_none());
        assert!(result.valid_until.is_none());
    }

    #[test]
    fn test_parse_invalid_url() {
        let json = r#"{
            "error_code": "INVALID_URL",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::InvalidUrl);
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_no_changes() {
        let json = r#"{
            "error_code": "NO_CHANGES",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::NoChanges);
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_wrong_region() {
        let json = r#"{
            "error_code": "WRONG_REGION",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::WrongRegion);
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_access_forbidden() {
        let json = r#"{
            "error_code": "ACCESS_FORBIDDEN",
            "error_message": "explicit error message"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::AccessForbidden);
        assert_eq!(result.error_message, "explicit error message");
    }

    #[test]
    fn test_parse_invalid_oauth_token() {
        let json = r#"{
            "error_code": "INVALID_OAUTH_TOKEN",
            "error_message": "explicit error message"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::InvalidOauthToken);
        assert_eq!(result.error_message, "explicit error message");
    }

    #[test]
    fn test_parse_hosts_limit_exceeded() {
        let json = r#"{
            "error_code": "HOSTS_LIMIT_EXCEEDED",
            "limit": 1,
            "error_message": "explicit error message"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::HostsLimitExceeded);
        assert_eq!(result.error_message, "explicit error message");
    }

    #[test]
    fn test_parse_feeds_limit_exceeded() {
        let json = r#"{
            "error_code": "FEEDS_LIMIT_EXCEEDED",
            "limit": 1,
            "error_message": "explicit error message"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::FeedsLimitExceeded);
        assert_eq!(result.error_message, "explicit error message");
    }

    #[test]
    fn test_parse_resource_not_found() {
        let json = r#"{
            "error_code": "RESOURCE_NOT_FOUND",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::ResourceNotFound);
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_host_not_indexed() {
        let json = r#"{
            "error_code": "HOST_NOT_INDEXED",
            "host_id": "http:ya.ru:80",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::HostNotIndexed);
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_host_not_found() {
        let json = r#"{
            "error_code": "HOST_NOT_FOUND",
            "host_id": "http:ya.ru:80",
            "error_message": "explicit error message"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::HostNotFound);
        assert_eq!(result.error_message, "explicit error message");
    }

    #[test]
    fn test_parse_sitemap_not_found() {
        let json = r#"{
            "error_code": "SITEMAP_NOT_FOUND",
            "host_id": "http:ya.ru:80",
            "sitemap_id": "c7-fe:80-c0",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::SitemapNotFound);
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_task_not_found() {
        let json = r#"{
            "error_code": "TASK_NOT_FOUND",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::TaskNotFound);
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_query_id_not_found() {
        let json = r#"{
            "error_code": "QUERY_ID_NOT_FOUND",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::QueryIdNotFound);
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_method_not_allowed() {
        let json = r#"{
            "error_code": "METHOD_NOT_ALLOWED",
            "error_message": "explicit error message"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::MethodNotAllowed);
        assert_eq!(result.error_message, "explicit error message");
    }

    #[test]
    fn test_parse_content_type_unsupported_with_acceptable_types() {
        let json = r#"{
            "error_code": "CONTENT_TYPE_UNSUPPORTED",
            "acceptable_types": [
                "application/json",
                "application/xml"
            ],
            "error_message": "explicit error message"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::ContentTypeUnsupported);
        assert_eq!(result.error_message, "explicit error message");
        assert_eq!(
            result.acceptable_types,
            Some(vec![
                "application/json".to_string(),
                "application/xml".to_string()
            ])
        );
    }

    #[test]
    fn test_parse_url_already_added() {
        let json = r#"{
            "error_code": "URL_ALREADY_ADDED",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::UrlAlreadyAdded);
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_host_already_added() {
        let json = r#"{
            "error_code": "HOST_ALREADY_ADDED",
            "host_id": "http:ya.ru:80",
            "verified": false,
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::HostAlreadyAdded);
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_verification_already_in_progress() {
        let json = r#"{
            "error_code": "VERIFICATION_ALREADY_IN_PROGRESS",
            "verification_type": "META_TAG",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            result.error_code,
            YandexErrorCode::VerificationAlreadyInProgress
        );
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_sitemap_already_added() {
        let json = r#"{
            "error_code": "SITEMAP_ALREADY_ADDED",
            "sitemap_id": "c7-fe:80-c0",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::SitemapAlreadyAdded);
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_upload_address_expired_with_valid_until() {
        let json = r#"{
            "error_code": "UPLOAD_ADDRESS_EXPIRED",
            "valid_until": "2016-01-01T00:00:00,000+0300",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::UploadAddressExpired);
        assert_eq!(result.error_message, "some string");
        assert_eq!(
            result.valid_until,
            Some("2016-01-01T00:00:00,000+0300".to_string())
        );
    }

    #[test]
    fn test_parse_request_entity_too_large() {
        let json = r#"{
            "error_code": "REQUEST_ENTITY_TOO_LARGE",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::RequestEntityTooLarge);
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_content_encoding_unsupported() {
        let json = r#"{
            "error_code": "CONTENT_ENCODING_UNSUPPORTED",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            result.error_code,
            YandexErrorCode::ContentEncodingUnsupported
        );
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_quota_exceeded() {
        let json = r#"{
            "error_code": "QUOTA_EXCEEDED",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::QuotaExceeded);
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_too_many_requests() {
        let json = r#"{
            "error_code": "TOO_MANY_REQUESTS_ERROR",
            "error_message": "some string"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::TooManyRequestsError);
        assert_eq!(result.error_message, "some string");
    }

    #[test]
    fn test_parse_unknown_error_code() {
        let json = r#"{
            "error_code": "SOME_UNKNOWN_ERROR",
            "error_message": "unknown error occurred"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            result.error_code,
            YandexErrorCode::Unknown("SOME_UNKNOWN_ERROR".to_string())
        );
        assert_eq!(result.error_message, "unknown error occurred");
    }

    #[test]
    fn test_error_code_display() {
        assert_eq!(YandexErrorCode::InvalidUrl.to_string(), "INVALID_URL");
        assert_eq!(YandexErrorCode::HostNotFound.to_string(), "HOST_NOT_FOUND");
        assert_eq!(
            YandexErrorCode::Unknown("CUSTOM_ERROR".to_string()).to_string(),
            "CUSTOM_ERROR"
        );
    }

    #[test]
    fn test_error_display() {
        let error = YandexWebmasterError::ApiError {
            status: 404,
            response: YandexApiErrorResponse {
                error_code: YandexErrorCode::HostNotFound,
                error_message: "Host not found in user's list".to_string(),
                acceptable_types: None,
                valid_until: None,
            },
        };

        let error_string = error.to_string();
        assert!(error_string.contains("HOST_NOT_FOUND"));
        assert!(error_string.contains("Host not found in user's list"));
    }

    #[test]
    fn test_parse_with_extra_fields_ignored() {
        // Test that extra fields in the JSON are ignored
        let json = r#"{
            "error_code": "HOST_NOT_FOUND",
            "host_id": "http:example.com:80",
            "extra_field": "should be ignored",
            "error_message": "Host not found"
        }"#;

        let result: YandexApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.error_code, YandexErrorCode::HostNotFound);
        assert_eq!(result.error_message, "Host not found");
    }
}
