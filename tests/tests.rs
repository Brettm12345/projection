mod testenv;
use testenv::TestEnv;
#[cfg(test)]
#[test]
fn add_project() {
    let te = TestEnv::new();
    te.assert_snapshot(&["add", "gh:brettm12345/xmonad-config", "--name", "xmonad"]);
    te.assert_temp_dir_diff()
}
