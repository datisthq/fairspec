use std::path::Path;

use crate::actions::folder::create::create_folder;

fn create(path: &Path) {
    create_folder(path.to_str().unwrap()).unwrap();
}

#[test]
fn test_creates_simple_folder() {
    let tmp_path = tempfile::tempdir().unwrap();
    let path = tmp_path.path().join("test-folder");
    create(&path);
    assert!(path.is_dir());
}

#[test]
fn test_creates_nested_folders() {
    let tmp_path = tempfile::tempdir().unwrap();
    let path = tmp_path
        .path()
        .join("parent")
        .join("child")
        .join("grandchild");
    create(&path);
    assert!(path.is_dir());
    assert!(tmp_path.path().join("parent").is_dir());
    assert!(tmp_path.path().join("parent").join("child").is_dir());
}

#[test]
fn test_no_error_when_folder_exists() {
    let tmp_path = tempfile::tempdir().unwrap();
    let path = tmp_path.path().join("existing");
    create(&path);
    create(&path);
    assert!(path.is_dir());
}

#[test]
fn test_creates_deeply_nested_directories() {
    let tmp_path = tempfile::tempdir().unwrap();
    let path = tmp_path.path().join("l1/l2/l3/l4/l5");
    create(&path);
    assert!(path.is_dir());
}

#[test]
fn test_creates_folder_with_special_characters() {
    let tmp_path = tempfile::tempdir().unwrap();
    let path = tmp_path.path().join("folder-with_special.chars");
    create(&path);
    assert!(path.is_dir());
}

#[test]
fn test_creates_sibling_folders() {
    let tmp_path = tempfile::tempdir().unwrap();
    let names = ["folder1", "folder2", "folder3"];
    for name in names {
        create(&tmp_path.path().join(name));
    }
    for name in names {
        assert!(tmp_path.path().join(name).is_dir());
    }
}

#[test]
fn test_creates_complex_directory_structure() {
    let tmp_path = tempfile::tempdir().unwrap();
    let dirs = [
        "project/src/components",
        "project/src/utils",
        "project/tests/unit",
        "project/tests/integration",
    ]
    .map(|dir| tmp_path.path().join(dir));
    for dir in &dirs {
        create(dir);
    }
    for dir in &dirs {
        assert!(dir.is_dir());
    }
}
