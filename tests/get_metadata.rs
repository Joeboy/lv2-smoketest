use std::process::Command;

#[test]
fn missing_plugin_uri_returns_failure() {
    let binary = env!("CARGO_BIN_EXE_lv2-smoketest");
    let output = Command::new(binary)
        .args([
            "show-plugin-metadata",
            "http://example.invalid/plugins/this-plugin-should-not-exist",
        ])
        .output()
        .expect("failed to run lv2-smoketest");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Plugin not found"));
}

#[test]
fn list_installed_plugins_returns_json_array() {
    let binary = env!("CARGO_BIN_EXE_lv2-smoketest");
    let output = Command::new(binary)
        .arg("list-installed-plugins")
        .output()
        .expect("failed to run lv2-smoketest");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("list-installed-plugins should emit valid JSON");

    assert!(value.is_array());
}

#[test]
fn test_plugin_run_missing_plugin_returns_failure() {
    let binary = env!("CARGO_BIN_EXE_lv2-smoketest");
    let output = Command::new(binary)
        .args([
            "test-plugin-run",
            "http://example.invalid/plugins/this-plugin-should-not-exist",
        ])
        .output()
        .expect("failed to run lv2-smoketest");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Plugin not found"));
}
