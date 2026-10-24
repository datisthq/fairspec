use fairspec_metadata::Result;
use serde_json::{Value, json};

use crate::helpers::test_server::serve;
use crate::plugins::github::models::tree::GithubTree;
use crate::plugins::github::services::github::{TEST_BASE_URL, make_github_api_request};

fn request_tree(body: Value) -> Result<GithubTree> {
    let (base_url, _server) = serve(vec![(200, body.to_string().into_bytes())]);
    TEST_BASE_URL.set(Some(base_url));
    make_github_api_request("/repos/o/r/git/trees/main", None, None, None)
}

#[test]
fn test_returns_the_response_parsed_by_the_model() {
    let entry = json!({
        "path": "data",
        "mode": "040000",
        "type": "tree",
        "sha": "abc",
        "url": "https://api.github.com/repos/o/r/git/trees/abc",
    });
    let tree = request_tree(json!({"sha": "def", "tree": [entry], "truncated": false})).unwrap();

    assert_eq!(tree.tree.len(), 1);
    assert_eq!(tree.tree[0].path, "data");
    assert_eq!(tree.tree[0].size, None);
}

#[test]
fn test_rejects_a_response_that_does_not_match_the_model() {
    let error = request_tree(json!({"message": "Not Found"}))
        .unwrap_err()
        .to_string();

    assert!(
        error.starts_with(
            "Github API returned an unexpected response from /repos/o/r/git/trees/main"
        )
    );
    assert!(error.contains("tree"), "{error}");
}
