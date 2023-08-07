// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use aws_sdk_config::config::Region;
use aws_sdk_s3::client::Client;
use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;

type Result<T> = std::result::Result<T, String>;

#[derive(Serialize)]
struct StorageObject {
    name: String,
    size: i64,
    last_modified: i64,
}

#[derive(Serialize)]
struct BucketObject {
    name: String,
    creation_date: i64,
    acl: Option<String>,
    grants: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    name: String,
    region: String,
    endpoint: String,
}

async fn new_client(profile: Profile) -> Client {
    Client::from_conf(
        aws_sdk_s3::config::Builder::new()
            .region(Region::new(profile.region.to_string()))
            .credentials_provider(
                aws_config::default_provider::credentials::Builder::default()
                    .profile_name(&profile.name)
                    .build()
                    .await,
            )
            // .force_path_style(true)
            .endpoint_url(profile.endpoint)
            .build(),
    )
}

#[tauri::command]
async fn list_buckets(profile: Profile) -> Result<Vec<BucketObject>> {
    let client = new_client(profile).await;

    let bucket_output = client
        .list_buckets()
        .send()
        .await
        .map_err(|e| format!("failed to list buckets: {e}"))?;

    let buckets = bucket_output
        .buckets()
        .unwrap()
        .into_iter()
        .map(|bucket| BucketObject {
            name: bucket.name().unwrap().to_string(),
            creation_date: bucket.creation_date().unwrap().secs(),
            acl: None,
            grants: None,
        })
        .collect::<Vec<_>>();

    Ok(buckets)
}

#[tauri::command]
async fn list_objects(profile: Profile, bucket: &str) -> Result<Vec<StorageObject>> {
    print!("profile: {:#?}", profile);
    let client = new_client(profile).await;
    let object_output = client
        .list_objects_v2()
        .bucket(bucket)
        .send()
        .await
        .map_err(|e| format!("failed to list objects for {bucket}: {e}"))?;

    let objects = object_output
        .contents()
        .ok_or("{bucket}: failed to get objects")?
        .iter()
        .map(|object| StorageObject {
            name: object.key().unwrap().to_string(),
            size: object.size(),
            last_modified: object.last_modified().unwrap().secs(),
        })
        .collect::<Vec<_>>();

    Ok(objects)
}

#[tauri::command]
async fn get_bucket(profile: Profile, bucket: &str) -> Result<BucketObject> {
    let client = new_client(profile).await;
    let resp = client
        .get_bucket_acl()
        .bucket(bucket)
        .send()
        .await
        .map_err(|e| format!("failed to get bucket acl for {bucket}: {e}"))?;

    Ok(BucketObject {
        name: bucket.to_string(),
        creation_date: 0,
        acl: None,
        grants: None,
    })
}

fn main() {
    let env_filter = EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_ansi(true)
        .with_writer(std::io::stderr)
        .finish();

    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        eprintln!("failed to set global default subscriber: {e}");
    };

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_buckets, list_objects])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
