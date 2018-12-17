extern crate json;
extern crate rusoto_core;
extern crate rusoto_s3;

use std::process::{Command, exit};
use std::fs::read_dir;
use std::path::Path;
use std::env;

use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, PutObjectRequest};

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

    let bucket = match env::var("BUCKET") {
        Ok(val) => val.to_string(),
        Err(_) => {
            println!("{}", "You must provide the bucket to upload to");
            exit(1);
        }
    };

    let s3_client = S3Client::new(Region::UsEast1);
    s3_client.put_object(PutObjectRequest {
        bucket: bucket,
        key: "@types.json".to_string(),
        body: Some(json::stringify(types_names).into_bytes().into()),
        acl: Some("public-read".to_string()),
        ..Default::default()
    }).sync().expect("could not upload");
}
