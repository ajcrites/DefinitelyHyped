extern crate json;

use std::process::Command;
use std::fs::read_dir;
use std::path::Path;

fn main() {
    const REPO_PATH: &'static str = "DefinitelyTyped";

    // git clone --branch types-2.0 --single-branch --depth=1 \
    //  https://github.com/DefinitelyTyped/DefinitelyTyped
    Command::new("git")
        .arg("clone")
        .arg("--branch")
        .arg("types-2.0")
        .arg("--single-branch")
        .arg("--depth=1")
        .arg(format!("https://github.com/DefinitelyTyped/{}", REPO_PATH))
        .output()
        .expect("clone failed");

    let mut paths: Vec<_> = read_dir(Path::new(REPO_PATH))
        .expect("Could not read DefinitelyTyped directory")
        .map(|dir| dir.unwrap().path())
        .filter(|path| path.is_dir())
        .filter(|path| path.to_str() != Some(&format!("{}/.git", REPO_PATH)))
        .collect();

    paths.sort();
    let types_names: Vec<_> = paths.iter()
        .map(|path| path.to_str().unwrap().replace(&format!("{}/", REPO_PATH), ""))
        .collect();
    println!("{}", json::stringify(types_names));
}
