use super::*;

#[test]
fn the_manifest_names_the_extension() {
    let manifest = SvelteExtension.manifest();
    assert_eq!(manifest.id, "svelte");
    assert_eq!(manifest.name, "Svelte");
    assert_eq!(manifest.schema_version, CURRENT_SCHEMA_VERSION);
}

#[test]
fn nothing_is_contributed_yet() {
    let contributions = SvelteExtension.contributions();
    assert!(contributions.languages.is_empty());
    assert!(contributions.scaffolds.is_empty());
}
