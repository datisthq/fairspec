use fairspec_metadata::Dataset;
use serde_json::{Value, json};

use crate::helpers::test_server::serve;
use crate::plugins::github::actions::dataset::save::save_dataset_to_github;
use crate::plugins::github::services::github::TEST_BASE_URL;

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/plugins/github/actions/dataset/-test/fixtures"
);

#[test]
fn test_saves_a_dataset() {
    let repository = read_response("repository.json");
    let content = read_response("content.json");
    let (base_url, server) = serve(vec![
        (200, repository),
        (200, content.clone()),
        (200, content),
    ]);
    TEST_BASE_URL.set(Some(base_url));

    let result =
        save_dataset_to_github(&create_dataset(), "test-api-key", "test-repo", None).unwrap();

    assert_eq!(
        Value::Object(result),
        json!({
            "path": "https://raw.githubusercontent.com/test-user/test-repo/refs/heads/main/dataset.json",
            "repo_url": "https://github.com/test-user/test-repo",
        })
    );
    let requests = server.join().unwrap();
    let request_lines: Vec<_> = requests
        .iter()
        .map(|request| request.request_line.trim())
        .collect();
    assert_eq!(
        request_lines,
        vec![
            "POST /user/repos HTTP/1.1",
            "PUT /repos/test-user/test-repo/contents/data.csv HTTP/1.1",
            "PUT /repos/test-user/test-repo/contents/dataset.json HTTP/1.1",
        ]
    );
    assert!(
        requests[0]
            .headers
            .contains(&"authorization: bearer test-api-key".to_string())
    );
}

fn create_dataset() -> Dataset {
    serde_json::from_value(json!({
        "titles": [{"title": "Test Package"}],
        "resources": [{
            "name": "test_resource",
            "data": format!("{FIXTURES}/data.csv"),
            "fileDialect": {"format": "csv"},
        }],
    }))
    .unwrap()
}

fn read_response(name: &str) -> Vec<u8> {
    std::fs::read(format!("{FIXTURES}/responses/{name}")).unwrap()
}
