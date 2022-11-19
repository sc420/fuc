use std::{fs, fs::File, io, num::NonZeroUsize};

use ftzz::generator::{Generator, NumFilesWithRatio};
use rstest::rstest;
use tempfile::tempdir;

#[test]
fn one_file() {
    let root = tempdir().unwrap();
    let file = root.path().join("file");
    File::create(&file).unwrap();
    assert!(file.exists());

    fuc_engine::remove_dir_all(&file).unwrap();

    assert!(!file.exists());
    assert!(root.as_ref().exists());
}

#[test]
fn one_dir() {
    let root = tempdir().unwrap();
    let dir = root.path().join("dir");
    fs::create_dir(&dir).unwrap();
    assert!(dir.exists());

    fuc_engine::remove_dir_all(&dir).unwrap();

    assert!(!dir.exists());
    assert!(root.as_ref().exists());
}

#[rstest]
fn uniform(#[values(1_000, 100_000, 1_000_000)] num_files: usize) {
    let root = tempdir().unwrap();
    let dir = root.path().join("dir");
    Generator::builder()
        .root_dir(dir.clone())
        .num_files_with_ratio(NumFilesWithRatio::from_num_files(
            NonZeroUsize::new(num_files).unwrap(),
        ))
        .build()
        .generate(&mut io::sink())
        .unwrap();

    fuc_engine::remove_dir_all(&dir).unwrap();

    assert!(!dir.exists());
    assert!(root.as_ref().exists());
}
