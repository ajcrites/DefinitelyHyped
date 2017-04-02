extern crate json;
extern crate rusoto;

use std::process::{Command, exit};
use std::fs::read_dir;
use std::path::Path;
use std::env;

use rusoto::{ChainProvider, ProfileProvider, Region};
use rusoto::s3::{S3Client, PutObjectRequest};
use rusoto::default_tls_client;

fn main() {
    const REPO_PATH: &'static str = "DefinitelyTyped";

    // git clone --single-branch --depth=1 \
    //  https://github.com/DefinitelyTyped/DefinitelyTyped
    Command::new("git")
        .arg("clone")
        .arg("--single-branch")
        .arg("--depth=1")
        .arg(format!("https://github.com/DefinitelyTyped/{}", REPO_PATH))
        .output()
        .expect("clone failed");

    let mut paths: Vec<_> = read_dir(Path::new(&format!("{}/types/", REPO_PATH)))
        .expect("Could not read DefinitelyTyped directory")
        .map(|dir| dir.unwrap().path())
        .filter(|path| path.is_dir())
        .filter(|path| path.to_str() != Some(&format!("{}/.git", REPO_PATH)))
        .collect();

    paths.sort();
    let types_names: Vec<_> = paths.iter()
        .map(|path| path.to_str().unwrap().replace(&format!("{}/types/", REPO_PATH), ""))
        .collect();

    let profile = match env::var("AWS_PROFILE") {
        Ok(val) => val.to_string(),
        Err(_) => "default".to_string(),
    };

    let region = match env::var("AWS_REGION") {
        Ok(val) => val.to_string(),
        Err(_) => {
            println!("{}", "You must provide an AWS Region");
            exit(1);
        }
    };

    let bucket = match env::var("BUCKET") {
        Ok(val) => val.to_string(),
        Err(_) => {
            println!("{}", "You must provide the bucket to upload to");
            exit(1);
        }
    };

    let mut profile_provider = ProfileProvider::new().unwrap();
    profile_provider.set_profile(profile);
    let chain_provider = ChainProvider::with_profile_provider(profile_provider);

    let s3_client = S3Client::new(default_tls_client().unwrap(), chain_provider, region.parse::<Region>().unwrap());
    s3_client.put_object(&PutObjectRequest {
        bucket: bucket,
        key: "@types.json".to_string(),
        body: Some(json::stringify(types_names).into_bytes()),

        request_payer: None,
        content_encoding: None,
        storage_class: None,
        grant_read_acp: None,
        server_side_encryption: None,
        ssekms_key_id: None,
        content_disposition: None,
        metadata: None,
        sse_customer_key: None,
        website_redirect_location: None,
        expires: None,
        cache_control: None,
        content_length: None,
        grant_read: None,
        grant_write_acp: None,
        acl: None,
        grant_full_control: None,
        sse_customer_algorithm: None,
        content_type: None,
        content_language: None,
        content_md5: None,
        sse_customer_key_md5: None,
        tagging: None,
    }).expect("could not upload");
}
