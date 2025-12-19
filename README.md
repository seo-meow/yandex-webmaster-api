# Yandex Webmaster API Client

Rust client library for the [Yandex Webmaster API](https://yandex.ru/dev/webmaster/).

## Features

- OAuth authentication with middleware support
- Automatic user ID fetching on client creation
- Type-safe DTOs for all API responses
- Comprehensive error handling
- Full coverage of Yandex Webmaster API endpoints
- Built on `reqwest` with middleware support
- Instrumented with tracing for observability

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
yandex-webmaster-api = "0.2.0"
```

## Quick Start

```rust
use yandex_webmaster_api::{YandexWebmasterClient, AddHostRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a new client with your OAuth token
    let client = YandexWebmasterClient::new("your-oauth-token".to_string()).await?;

    // The user ID is automatically fetched and stored
    println!("User ID: {}", client.user_id());

    // List all hosts
    let hosts = client.get_hosts().await?;
    for host in hosts.hosts {
        println!("Host: {}", host.ascii_host_url);
    }

    // Add a new site
    let request = AddHostRequest {
        host_url: "https://example.com".to_string(),
    };
    let new_host = client.add_host(&request).await?;
    println!("Added host with ID: {}", new_host.host_id);

    Ok(())
}
```

## API Methods

### Hosts Management

- `get_hosts()` - List all sites for the user
- `add_host(request)` - Add a new site
- `get_host(host_id)` - Get information about a specific site
- `delete_host(host_id)` - Delete a site

### Host Verification

- `get_verification_status(host_id)` - Get verification status for a site
- `verify_host(host_id, verification_type)` - Initiate verification procedure
- `get_owners(host_id)` - Get list of verified owners

### Site Statistics

- `get_host_summary(host_id)` - Get site summary statistics
- `get_sqi_history(host_id)` - Get site quality index history

### Search Queries

- `get_popular_queries(host_id)` - Get popular search queries
- `get_query_analytics(host_id, request)` - Get overall query statistics
- `get_query_history(host_id, query_id, request)` - Get statistics for a specific query

### Sitemaps

- `get_sitemaps(host_id)` - Get list of all sitemap files
- `get_sitemap(host_id, sitemap_id)` - Get details of a specific sitemap
- `get_user_sitemaps(host_id)` - Get list of user-submitted sitemaps
- `add_sitemap(host_id, request)` - Add a new sitemap file
- `get_user_sitemap(host_id, sitemap_id)` - Get user-submitted sitemap details
- `delete_sitemap(host_id, sitemap_id)` - Delete a user-submitted sitemap

### Indexing

- `get_indexing_history(host_id, request)` - Get indexing history
- `get_indexing_samples(host_id)` - Get sample indexed pages
- `get_search_urls_history(host_id, request)` - Get pages in search history
- `get_search_urls_samples(host_id)` - Get sample pages in search
- `get_search_events_history(host_id, request)` - Get page appearance/removal history
- `get_search_events_samples(host_id)` - Get sample page changes

### Important URLs

- `get_important_urls(host_id)` - Get list of important URLs
- `get_important_urls_history(host_id)` - Get important URLs history

### Recrawl Management

- `recrawl_urls(host_id, request)` - Request page recrawl
- `get_recrawl_tasks(host_id)` - Get list of recrawl tasks
- `get_recrawl_task(host_id, task_id)` - Get recrawl task status
- `get_recrawl_quota(host_id)` - Get recrawl quota

### Links

- `get_broken_links(host_id)` - Get broken internal links samples
- `get_broken_links_history(host_id, request)` - Get broken links history
- `get_external_links(host_id)` - Get external backlinks samples
- `get_external_links_history(host_id, request)` - Get backlinks history

### Diagnostics

- `get_diagnostics(host_id)` - Get site diagnostic report

## Project structure

```
.
├── src/
│   ├── lib.rs          # Main library entry point
│   ├── client.rs       # API client implementation with all methods
│   ├── dto.rs          # Data Transfer Objects (DTOs) for API requests/responses
│   ├── error.rs        # Error types and Result alias
│   └── middleware.rs   # OAuth authentication middleware
├── Cargo.toml          # Project dependencies and metadata
└── README.md           # Project documentation
```

## Authentication

To use this library, you need a valid OAuth token from Yandex. You can obtain one by following the [Yandex OAuth documentation](https://yandex.ru/dev/id/doc/en/).

The client automatically:
1. Adds the OAuth token to all requests via middleware
2. Fetches the user ID on creation
3. Validates authentication by calling the `/user` endpoint

## Examples

### Verification Workflow

```rust
use yandex_webmaster_api::{YandexWebmasterClient, VerificationType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = YandexWebmasterClient::new("your-token".to_string()).await?;

    let host_id = "your-host-id";

    // Initiate verification
    let verification = client.verify_host(host_id, VerificationType::MetaTag).await?;
    println!("Verification token: {:?}", verification.verification_uin);

    // Check verification status
    let status = client.get_verification_status(host_id).await?;
    println!("Status: {:?}", status.verification_state);

    Ok(())
}
```

### Query Analytics

```rust
use yandex_webmaster_api::{YandexWebmasterClient, QueryHistoryRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = YandexWebmasterClient::new("your-token".to_string()).await?;

    let request = QueryHistoryRequest {
        date_from: Some("2024-01-01".to_string()),
        date_to: Some("2024-01-31".to_string()),
    };

    let analytics = client.get_query_analytics("host-id", &request).await?;
    for point in analytics.points {
        println!("Date: {}, Clicks: {:?}", point.date, point.clicks);
    }

    Ok(())
}
```

### Recrawl URLs

```rust
use yandex_webmaster_api::{YandexWebmasterClient, RecrawlRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = YandexWebmasterClient::new("your-token".to_string()).await?;

    let request = RecrawlRequest {
        urls: vec![
            "https://example.com/page1".to_string(),
            "https://example.com/page2".to_string(),
        ],
    };

    let response = client.recrawl_urls("host-id", &request).await?;
    println!("Task ID: {}", response.task_id);
    println!("Quota remaining: {:?}", response.quota_remainder);

    Ok(())
}
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
