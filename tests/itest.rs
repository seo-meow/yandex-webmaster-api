use chrono::{Duration, Utc};
use rand::distr::{Alphanumeric, SampleString};
use std::fs::File;
use std::io::Read;
use yandex_webmaster_api::{
    AddHostRequest, AddSitemapRequest, ApiQueryIndicator, ApiQueryOrderField,
    ExplicitVerificationType, GetIndexingSamplesRequest, GetRecrawlTasksRequest,
    GetSearchEventsSamplesRequest, GetSearchUrlsSamplesRequest, GetSitemapsRequest,
    GetUserSitemapsRequest, IndexingHistoryRequest, PopularQueriesRequest, QueryAnalyticsRequest,
    QueryHistoryRequest, RecrawlRequest, SqiHistoryRequest, VerificationState, VerificationType,
    YandexWebmasterClient,
};

async fn new_client() -> anyhow::Result<YandexWebmasterClient> {
    let mut str = String::new();
    File::open("tests/token")?.read_to_string(&mut str)?;

    Ok(YandexWebmasterClient::new(str).await?)
}

fn generate_random_host() -> String {
    let s: String = Alphanumeric.sample_string(&mut rand::rng(), 8);
    format!("example-{s}.com")
}

#[tokio::test]
#[ignore]
async fn should_get_user() -> anyhow::Result<()> {
    let client = new_client().await?;

    assert!(client.user_id() > 0);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn should_get_host() -> anyhow::Result<()> {
    let client = new_client().await?;

    let hosts = client.get_hosts().await?;
    dbg!(&hosts);

    assert!(hosts.len() > 0);
    let first = hosts.first().unwrap();

    let info = client.get_host(&first.host_id).await?;

    dbg!(&info);

    assert_eq!(info.host_id, first.host_id);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn should_create_and_verify_host() -> anyhow::Result<()> {
    let client = new_client().await?;

    let host = client
        .add_host(&AddHostRequest {
            host_url: generate_random_host(),
            verification_type: VerificationType::Dns,
        })
        .await?;

    dbg!(&host);

    let info = client
        .verify_host(&host.host_id, ExplicitVerificationType::MetaTag)
        .await?;

    assert_eq!(info.verification_state, VerificationState::InProgress);
    assert_eq!(info.verification_type, VerificationType::MetaTag);

    dbg!(&info);

    let uin = info.verification_uin;

    let info = client.get_verification_status(&host.host_id).await?;

    assert_eq!(info.verification_state, VerificationState::InProgress);
    assert_eq!(info.verification_type, VerificationType::MetaTag);
    assert_eq!(info.verification_uin, uin);

    dbg!(&info);

    client.delete_host(&host.host_id).await?;

    Ok(())
}

#[tokio::test]
#[ignore]
async fn get_host_owners() -> anyhow::Result<()> {
    let client = new_client().await?;

    let host = client
        .get_hosts()
        .await?
        .into_iter()
        .find(|s| s.verified)
        .unwrap();

    let owners = client.get_owners(&host.host_id).await?;

    dbg!(&owners);

    assert!(owners.len() > 0);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn get_sqi() -> anyhow::Result<()> {
    let client = new_client().await?;

    let host = client
        .get_hosts()
        .await?
        .into_iter()
        .find(|s| s.verified)
        .unwrap();

    let history = client
        .get_sqi_history(
            &host.host_id,
            SqiHistoryRequest {
                date_from: Some(Utc::now()),
                date_to: Some(Utc::now() - Duration::days(10)),
            },
        )
        .await?;

    dbg!(&history);

    let history = client
        .get_sqi_history(&host.host_id, SqiHistoryRequest::default())
        .await?;

    dbg!(&history);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn get_search_queries() -> anyhow::Result<()> {
    let client = new_client().await?;

    let host = client
        .get_hosts()
        .await?
        .into_iter()
        .find(|s| s.verified)
        .unwrap();

    dbg!(&host);

    let queries = client
        .get_popular_queries(
            &host.host_id,
            &PopularQueriesRequest {
                order_by: ApiQueryOrderField::TotalShows,
                query_indicator: None,
                device_type_indicator: None,
                date_from: None,
                date_to: None,
                offset: None,
                limit: None,
            },
        )
        .await?;

    dbg!(&queries);

    let history = client
        .get_query_analytics(
            &host.host_id,
            &QueryAnalyticsRequest {
                query_indicator: vec![
                    ApiQueryIndicator::AvgClickPosition,
                    ApiQueryIndicator::AvgShowPosition,
                    ApiQueryIndicator::TotalClicks,
                    ApiQueryIndicator::TotalShows,
                ],
                device_type_indicator: None,
                date_from: Some(Utc::now() - Duration::days(90)),
                date_to: None,
            },
        )
        .await?;

    dbg!(&history);

    let query = queries
        .queries
        .into_iter()
        .map(|s| s.query_id)
        .next()
        .unwrap();

    let history = client
        .get_query_history(
            &host.host_id,
            &query,
            &QueryHistoryRequest {
                query_indicator: vec![
                    ApiQueryIndicator::AvgClickPosition,
                    ApiQueryIndicator::AvgShowPosition,
                    ApiQueryIndicator::TotalClicks,
                    ApiQueryIndicator::TotalShows,
                ],
                device_type_indicator: None,
                date_from: None,
                date_to: None,
            },
        )
        .await?;

    dbg!(&history);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn work_with_sitemaps() -> anyhow::Result<()> {
    let client = new_client().await?;

    let host = client
        .get_hosts()
        .await?
        .into_iter()
        .find(|s| s.verified)
        .unwrap();

    dbg!(&host);

    let sm = client
        .add_sitemap(
            &host.host_id,
            &AddSitemapRequest {
                url: format!("{}sitemap-test.xml", &host.ascii_host_url),
            },
        )
        .await?;

    dbg!(&sm);

    let sitemaps = client
        .get_sitemaps(
            &host.host_id,
            &GetSitemapsRequest {
                parent_id: None,
                limit: None,
                from: None,
            },
        )
        .await?;

    dbg!(&sitemaps);

    let sitemap = client
        .get_sitemap(
            &host.host_id,
            &sitemaps.sitemaps.first().unwrap().sitemap_id,
        )
        .await?;

    dbg!(&sitemap);

    let user_sitemaps = client
        .get_user_sitemaps(
            &host.host_id,
            &GetUserSitemapsRequest {
                offset: None,
                limit: None,
            },
        )
        .await?;
    dbg!(&user_sitemaps);

    let user_sitemap = client
        .get_user_sitemap(
            &host.host_id,
            &user_sitemaps.sitemaps.first().unwrap().sitemap_id,
        )
        .await?;

    dbg!(&user_sitemap);

    client.delete_sitemap(&host.host_id, &sm.sitemap_id).await?;

    Ok(())
}

#[tokio::test]
#[ignore]
async fn get_indexing() -> anyhow::Result<()> {
    let client = new_client().await?;

    let host = client
        .get_hosts()
        .await?
        .into_iter()
        .find(|s| s.verified)
        .unwrap();

    let stats = client.get_host_summary(&host.host_id).await?;

    dbg!(&stats);

    let index_history = client
        .get_indexing_history(
            &host.host_id,
            &IndexingHistoryRequest {
                date_from: None,
                date_to: None,
            },
        )
        .await?;

    dbg!(&index_history);

    let examples = client
        .get_indexing_samples(
            &host.host_id,
            &GetIndexingSamplesRequest {
                offset: None,
                limit: None,
            },
        )
        .await?;

    dbg!(&examples);

    let important = client.get_important_urls(&host.host_id).await?;

    dbg!(&important);

    let hist = client
        .get_important_urls_history(
            &host.host_id,
            &important.urls.first().map(|s| s.url.clone()).unwrap(),
        )
        .await?;
    dbg!(&hist);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn search_methods() -> anyhow::Result<()> {
    let client = new_client().await?;

    let host = client
        .get_hosts()
        .await?
        .into_iter()
        .find(|s| s.verified)
        .unwrap();

    let history = client
        .get_search_urls_history(
            &host.host_id,
            &IndexingHistoryRequest {
                date_from: None,
                date_to: None,
            },
        )
        .await?;

    dbg!(&history);

    let samples = client
        .get_search_urls_samples(
            &host.host_id,
            &GetSearchUrlsSamplesRequest {
                offset: None,
                limit: None,
            },
        )
        .await?;

    dbg!(&samples);

    let history = client
        .get_search_events_history(
            &host.host_id,
            &IndexingHistoryRequest {
                date_from: None,
                date_to: None,
            },
        )
        .await?;

    dbg!(&history);

    let samples = client
        .get_search_events_samples(
            &host.host_id,
            &GetSearchEventsSamplesRequest {
                offset: None,
                limit: None,
            },
        )
        .await?;

    dbg!(&samples);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn reindex() -> anyhow::Result<()> {
    let client = new_client().await?;

    let host = client
        .get_hosts()
        .await?
        .into_iter()
        .find(|s| s.verified)
        .unwrap();

    let task = client
        .recrawl_urls(
            &host.host_id,
            &RecrawlRequest {
                url: "https://seomeow.com".to_string(),
            },
        )
        .await?;

    dbg!(&task);

    let status = client
        .get_recrawl_task(&host.host_id, &task.task_id)
        .await?;

    dbg!(&status);

    let tasks = client
        .get_recrawl_tasks(
            &host.host_id,
            &GetRecrawlTasksRequest {
                offset: None,
                limit: None,
                date_from: None,
                date_to: None,
            },
        )
        .await?;

    dbg!(&tasks);

    let quota = client.get_recrawl_quota(&host.host_id).await?;

    dbg!(&quota);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn site_diagnostics() -> anyhow::Result<()> {
    let client = new_client().await?;

    let host = client
        .get_hosts()
        .await?
        .into_iter()
        .find(|s| s.verified)
        .unwrap();

    let diagnostics = client.get_diagnostics(&host.host_id).await?;

    dbg!(&diagnostics);

    Ok(())
}
