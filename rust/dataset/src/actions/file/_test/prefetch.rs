use std::fs;

use fairspec_metadata::Resource;
use serde_json::json;

use crate::actions::file::prefetch::prefetch_files;
use crate::actions::file::temp::write_temp_file;
use crate::helpers::test_server::serve;

const TABLE_CSV: &str = "id,name\n1,english\n2,中文\n";

fn create_resource(data: serde_json::Value) -> Resource {
    Resource {
        data: Some(data),
        ..Resource::default()
    }
}

#[test]
fn test_prefetches_remote_file() {
    let (url, server) = serve(vec![(200, TABLE_CSV.as_bytes().to_vec())]);
    let resource = create_resource(json!(format!("{url}/table.csv")));
    let result = prefetch_files(&resource, None, None).unwrap();
    server.join().unwrap();
    assert_eq!(result.len(), 1);
    let path = &result[0];
    assert!(fs::exists(path).unwrap());
    assert_eq!(fs::metadata(path).unwrap().len(), 27);
    let content = fs::read_to_string(path).unwrap();
    assert!(content.contains("id,name"));
}

#[test]
fn test_prefetches_remote_file_with_max_bytes() {
    let (url, server) = serve(vec![(200, TABLE_CSV.as_bytes().to_vec())]);
    let resource = create_resource(json!(format!("{url}/table.csv")));
    let max_bytes = 18;
    let result = prefetch_files(&resource, Some(max_bytes), None).unwrap();
    server.join().unwrap();
    assert_eq!(result.len(), 1);
    let path = &result[0];
    assert!(fs::exists(path).unwrap());
    assert_eq!(fs::metadata(path).unwrap().len(), max_bytes as u64);
    let content = fs::read_to_string(path).unwrap();
    assert!(content.contains("id,name"));
    assert!(content.contains("1,english"));
    assert!(!content.contains("中文"));
}

#[test]
fn test_prefetches_local_paths_in_order() {
    let paths: Vec<String> = (0..4)
        .map(|index| write_temp_file(format!("part-{index}\n"), false, None, None).unwrap())
        .collect();
    let resource = create_resource(json!(paths));

    assert_eq!(prefetch_files(&resource, None, Some(1)).unwrap(), paths);
    assert_eq!(prefetch_files(&resource, None, Some(4)).unwrap(), paths);
}
