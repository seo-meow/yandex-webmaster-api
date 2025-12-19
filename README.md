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
yandex-webmaster-api = "1.0.0"
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

## TODOs
- Add local e2e tests
- Add RSS feeds supports
- Add builders to DTOs