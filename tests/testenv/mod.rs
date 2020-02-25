use std::path::PathBuf;

use assert_cmd::Command;
use insta::assert_debug_snapshot;
use std::ffi;
use tempfile::{tempdir, TempDir};

pub struct TestEnv {
    /// Temporary working directory.
    temp_dir: TempDir,
}

impl TestEnv {
    pub fn new() -> TestEnv {
        TestEnv {
            temp_dir: tempdir().unwrap(),
        }
    }
    pub fn assert_snapshot<I, S>(&self, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<ffi::OsStr>,
    {
        assert_debug_snapshot!(Command::cargo_bin("projection")
            .unwrap()
            .arg("-d")
            .arg(&format!("{}", self.temp_dir.path().display()))
            .args(args)
            .assert())
    }
    pub fn assert_temp_dir_diff(&self) {
        assert!(dir_diff::is_different(self.test_root(), tempdir().unwrap().path()).unwrap())
    }
    /// Get the root directory for the tests.
    pub fn test_root(&self) -> PathBuf {
        self.temp_dir.path().to_path_buf()
    }
}
