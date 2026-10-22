use fairspec_dataset::get_temp_file_path;
use fairspec_metadata::Resource;
use polars::prelude::*;
use serde_json::json;

use crate::plugins::_shared::collect_records;
use crate::plugins::parquet::actions::table::load::load_parquet_table;

pub(crate) fn write_frame(path: &str) {
    let mut frame = df!("id" => [1i64, 2], "name" => ["english", "中文"]).unwrap();
    ParquetWriter::new(std::fs::File::create(path).unwrap())
        .finish(&mut frame)
        .unwrap();
}

fn create_resource(data: serde_json::Value) -> Resource {
    serde_json::from_value(json!({"data": data, "fileDialect": {"format": "parquet"}})).unwrap()
}

#[test]
fn test_should_load_local_file() {
    let path = get_temp_file_path(false, None, None).unwrap();
    write_frame(&path);

    let table = load_parquet_table(&create_resource(json!(path)), &Default::default()).unwrap();

    assert_eq!(
        collect_records(table),
        json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
    );
}

#[test]
fn test_should_load_local_file_multipart() {
    let path1 = get_temp_file_path(false, None, None).unwrap();
    let path2 = get_temp_file_path(false, None, None).unwrap();
    write_frame(&path1);
    write_frame(&path2);

    let table =
        load_parquet_table(&create_resource(json!([path1, path2])), &Default::default()).unwrap();

    assert_eq!(
        collect_records(table),
        json!([
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ])
    );
}
