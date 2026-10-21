use crate::actions::dataset::basepath::get_common_local_basepath;

#[test]
fn test_get_common_local_basepath() {
    let cases: [(&str, &[&str], Option<&str>); 8] = [
        (
            "same directory different files",
            &["data/table1.csv", "data/table2.csv"],
            Some("data"),
        ),
        (
            "nested directories",
            &[
                "data/nested/file1.csv",
                "data/nested/file2.csv",
                "data/file3.csv",
            ],
            Some("data"),
        ),
        ("single path", &["data/file.csv"], Some("data")),
        ("root level files", &["file1.csv", "file2.csv"], Some("")),
        (
            "different top-level directories",
            &["data1/file1.csv", "data2/file2.csv"],
            Some(""),
        ),
        ("empty paths array", &[], None),
        (
            "some paths are remote",
            &["https://example.com/table.csv", "data/table.csv"],
            Some("data"),
        ),
        (
            "all paths are remote",
            &[
                "https://example.com/table1.csv",
                "https://example.com/table2.csv",
            ],
            None,
        ),
    ];
    for (description, paths, expected) in cases {
        let paths: Vec<String> = paths.iter().map(|path| path.to_string()).collect();
        let result = get_common_local_basepath(&paths).unwrap();
        assert_eq!(result.as_deref(), expected, "{description}");
    }
}
