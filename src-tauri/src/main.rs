// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
fn set_env_var() {
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
}

#[macro_use]
extern crate ini;

use aws_sdk_config::config::Region;
use aws_sdk_s3::client::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use tracing::info;
use tracing_subscriber::EnvFilter;

type Result<T> = std::result::Result<T, String>;

#[derive(Serialize)]
struct StorageObject {
    name: String,
    size: i64,
    last_modified: i64,
}

#[derive(Serialize, Debug)]
struct BucketObject {
    name: String,
    creation_date: i64,
    acl: Option<String>,
    grants: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Profile {
    name: String,
    region: String,
    endpoint: String,
    force_path_style: bool,
}

#[derive(Serialize, Debug)]
struct AWSCredentials {
    name: String,
    aws_access_key_id: String,
    aws_secret_access_key: String,
}

async fn new_client(profile: Profile) -> Client {
    let sdk_config = ::aws_config::load_from_env().await;
    let config = aws_sdk_s3::config::Builder::from(&sdk_config)
        .region(Region::new(profile.region))
        .credentials_provider(
            aws_config::default_provider::credentials::Builder::default()
                .profile_name(&profile.name)
                .build()
                .await,
        )
        .force_path_style(profile.force_path_style)
        .endpoint_url(profile.endpoint)
        .build();

    Client::from_conf(config)
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
        .ok_or("failed to get buckets")?
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
async fn _get_bucket(profile: Profile, bucket: &str) -> Result<BucketObject> {
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

#[tauri::command]
async fn read_aws_config(config_path: Option<PathBuf>) -> Result<HashMap<String, AWSCredentials>> {
    let path = if let Some(ref path) = config_path {
        path.to_str()
            .ok_or_else(|| String::from("failed to convert path to string"))?
            .to_string()
    } else {
        let home = env::var("HOME").map_err(|e| format!("failed to get home directory: {e}"))?;
        Path::new(&home)
            .join(".aws")
            .join("credentials")
            .to_str()
            .map(String::from)
            .ok_or_else(|| String::from("failed to convert path to string"))?
    };

    let mut profiles = HashMap::new();
    for (name, credentials) in ini!(&path).iter() {
        let aws_access_key_id = credentials
            .get("aws_access_key_id")
            .ok_or(format!(
                "failed to get aws_access_key_id for profile {name}"
            ))?
            .to_owned()
            .ok_or(format!(
                "failed to get aws_access_key_id for profile {name}"
            ))?;

        let aws_secret_access_key = credentials
            .get("aws_secret_access_key")
            .ok_or(format!(
                "failed to get aws_secret_access_key for profile {name}"
            ))?
            .to_owned()
            .ok_or(format!(
                "failed to get aws_secret_access_key for profile {name}"
            ))?;

        profiles.insert(
            name.to_string(),
            AWSCredentials {
                name: name.to_string(),
                aws_access_key_id,
                aws_secret_access_key,
            },
        );
    }

    Ok(profiles)
}

fn main() {
    set_env_var();
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
        .invoke_handler(tauri::generate_handler![
            list_buckets,
            list_objects,
            read_aws_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
