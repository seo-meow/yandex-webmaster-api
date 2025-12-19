use rand::distr::{Alphanumeric, SampleString};
use std::fs::File;
use std::io::Read;
use chrono::{Duration, Utc};
use yandex_webmaster_api::{AddHostRequest, ExplicitVerificationType, SqiHistoryRequest, VerificationState, VerificationType, YandexWebmasterClient};

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

    let host = client.add_host(&AddHostRequest {
        host_url: generate_random_host(),
        verification_type: VerificationType::Dns,
    }).await?;

    dbg!(&host);

    let info = client.verify_host(&host.host_id, ExplicitVerificationType::MetaTag).await?;

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

    let host = client.get_hosts().await?
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

    let host = client.get_hosts().await?
        .into_iter()
        .find(|s| s.verified)
        .unwrap();

    let history = client.get_sqi_history(&host.host_id, SqiHistoryRequest {
        date_from: Some(Utc::now()),
        date_to: Some(Utc::now() - Duration::days(10)),
    }).await?;

    dbg!(&history);

    let history = client.get_sqi_history(&host.host_id, SqiHistoryRequest::default()).await?;

    dbg!(&history);

    Ok(())
}