use super::*;

const COMPONENT: &str = "<script lang=\"ts\">\n  let n: number = 0;\n</script>\n\n{#if n > 1}\n  <p class=\"a\" on:click={() => n++}>{n}</p>\n{:else}\n  <Foo bind:value={n} />\n{/if}\n\n<style>\n  p { color: red }\n</style>\n";

fn grammar() -> GrammarContribution {
    SvelteExtension.contributions().grammars.into_iter().next().unwrap()
}

fn language() -> tree_sitter::Language {
    match grammar().source {
        GrammarSource::Builtin(language) => language.into(),
        _ => panic!("the grammar is compiled in"),
    }
}

#[test]
fn the_manifest_names_the_extension() {
    let manifest = SvelteExtension.manifest();
    assert_eq!(manifest.id, "svelte");
    assert_eq!(manifest.name, "Svelte");
    assert_eq!(manifest.schema_version, CURRENT_SCHEMA_VERSION);
}

#[test]
fn svelte_files_are_the_svelte_language_with_an_icon() {
    let contributions = SvelteExtension.contributions();
    assert_eq!(contributions.languages.len(), 1);
    assert_eq!(contributions.languages[0].file_extensions, ["svelte"]);
    assert_eq!(contributions.file_icons[0].language_id, SVELTE);
}

#[test]
fn starting_the_server_needs_the_process_capability() {
    assert_eq!(SvelteExtension.capabilities(), [Capability::ProcessExec]);
}

#[test]
fn a_component_parses_without_errors() {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language()).unwrap();
    let tree = parser.parse(COMPONENT, None).unwrap();
    assert!(!tree.root_node().has_error(), "{}", tree.root_node().to_sexp());
}

#[test]
fn the_highlight_query_compiles_and_colors_tags_and_block_keywords() {
    use tree_sitter::StreamingIterator;

    let language = language();
    let query = tree_sitter::Query::new(&language, &grammar().highlight_query.unwrap()).expect("HTML's query and Svelte's compile together");
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language).unwrap();
    let tree = parser.parse(COMPONENT, None).unwrap();

    let mut cursor = tree_sitter::QueryCursor::new();
    let mut captures = cursor.captures(&query, tree.root_node(), COMPONENT.as_bytes());
    let mut seen = Vec::new();
    while let Some((m, index)) = captures.next() {
        let capture = m.captures[*index];
        seen.push((query.capture_names()[capture.index as usize], &COMPONENT[capture.node.byte_range()]));
    }
    assert!(seen.iter().any(|(name, text)| name.starts_with("tag") && *text == "p"), "{seen:?}");
    assert!(seen.iter().any(|(name, text)| name.starts_with("keyword") && *text == "if"), "{seen:?}");
    assert!(seen.iter().any(|(name, text)| name.starts_with("keyword") && *text == "else"), "{seen:?}");
}

#[test]
fn every_foldable_kind_is_a_node_of_the_grammar() {
    let language = language();
    for kind in node_kinds().foldable {
        assert_ne!(language.id_for_node_kind(&kind, true), 0, "{kind} is not a named node of the grammar");
    }
}
