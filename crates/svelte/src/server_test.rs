use super::*;

fn context(root: &Path, configured: &str) -> ServerStartContext {
    ServerStartContext {
        configured_binary: configured.to_string(),
        project_root: Some(root.to_path_buf()),
        ..ServerStartContext::default()
    }
}

#[test]
fn the_server_serves_svelte_over_stdio_and_is_not_installed_for_the_user() {
    let server = contribution();
    assert_eq!(server.language_ids, ["svelte"]);
    assert_eq!(server.binary_name, "svelteserver");
    assert_eq!(server.args, ["--stdio"]);
    assert!(server.install.is_none());
}

#[test]
fn without_an_install_in_the_project_the_server_is_looked_up_on_the_path() {
    let dir = tempfile::tempdir().unwrap();
    let started = resolve(SVELTE_SERVER, &context(dir.path(), "")).unwrap();
    assert_eq!(started.binary, PathBuf::from("svelteserver"));
    assert_eq!(started.args, ["--stdio"]);
}

#[test]
fn a_server_installed_in_the_project_is_preferred_to_the_path() {
    let dir = tempfile::tempdir().unwrap();
    let program = dir.path().join("node_modules/.bin/svelteserver");
    std::fs::create_dir_all(program.parent().unwrap()).unwrap();
    std::fs::write(&program, "").unwrap();
    assert_eq!(resolve(SVELTE_SERVER, &context(dir.path(), "")).unwrap().binary, program);
}

#[test]
fn a_path_configured_in_settings_wins() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(resolve(SVELTE_SERVER, &context(dir.path(), "/opt/svelteserver")).unwrap().binary, PathBuf::from("/opt/svelteserver"));
}

#[test]
fn another_extensions_server_is_not_answered() {
    let dir = tempfile::tempdir().unwrap();
    assert!(resolve("typescript-language-server", &context(dir.path(), "")).is_none());
}
