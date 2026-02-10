use assert_cmd::cargo::*;
use assert_fs::prelude::PathChild;

#[test]
#[ignore = "this test calls the real Zotero API, so it is excluded from tests in the CI"]
pub fn smoketest_export_snapshot() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let export_path = temp_dir.child("export.bib");
    let api_key_from_env = std::env::var("ZOTERO_API_KEY")
        .expect("Environment variable ZOTERO_API_KEY needs to be set for the smoke test");
    let mut cmd = cargo_bin_cmd!("zotexon");
    cmd.arg("--api-key")
        .arg(api_key_from_env)
        .arg("-o")
        .arg(export_path.to_str().unwrap())
        .assert()
        .success();

    let export_content =
        std::fs::read_to_string(export_path).expect("failed to read exported file");
    insta::assert_snapshot!(export_content);
}
