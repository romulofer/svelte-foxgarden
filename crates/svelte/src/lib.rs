//! Svelte support for FoxGarden, as an extension.
//!
//! Contributes the `svelte` language (`.svelte`) with its tree-sitter grammar,
//! a highlight query, node kinds for folding, a file icon, and the Svelte
//! language server (found in the project's `node_modules` or on `PATH`).
//!
//! A component embeds JavaScript or TypeScript in `<script>` and CSS in
//! `<style>`, which are highlighted as the languages they are.

use fg_extension::{
    CURRENT_SCHEMA_VERSION, Capability, Contributions, Extension, ExtensionManifest, FileIcon, GrammarContribution, GrammarSource,
    InjectionContribution, LanguageContribution, LanguageId, NodeKinds, ResolvedServerStart, ServerStartContext,
};

mod server;

/// Named the way the Language Server Protocol names it, since the id is also the
/// `languageId` a server is told a document has.
pub const SVELTE: &str = "svelte";

pub struct SvelteExtension;

impl Extension for SvelteExtension {
    fn manifest(&self) -> ExtensionManifest {
        ExtensionManifest {
            id: "svelte".to_string(),
            name: "Svelte".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            schema_version: CURRENT_SCHEMA_VERSION,
        }
    }

    /// The language server is a program this extension starts.
    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability::ProcessExec]
    }

    fn resolve_server_start(&self, server_id: &str, context: &ServerStartContext) -> Option<Result<ResolvedServerStart, String>> {
        server::resolve(server_id, context).map(Ok)
    }

    fn contributions(&self) -> Contributions {
        Contributions {
            languages: vec![LanguageContribution {
                id: LanguageId::from(SVELTE),
                display_name: "Svelte".to_string(),
                file_extensions: vec!["svelte".to_string()],
                filename_patterns: Vec::new(),
            }],
            grammars: vec![GrammarContribution {
                language_id: SVELTE.to_string(),
                source: GrammarSource::Builtin(tree_sitter_svelte_ng::LANGUAGE),
                highlight_query: Some(highlight_query()),
                node_kinds: node_kinds(),
            }],
            injections: vec![InjectionContribution { language_id: LanguageId::from(SVELTE), query: INJECTIONS.to_string() }],
            language_servers: vec![server::contribution()],
            // The Nerd Fonts seti Svelte glyph.
            file_icons: vec![FileIcon { language_id: SVELTE.to_string(), glyph: '\u{e697}' }],
            ..Contributions::default()
        }
    }
}

/// A `<script>` is JavaScript, or TypeScript with `lang="ts"`; a `<style>` is
/// CSS; the expression in `{...}` is JavaScript. The general pattern comes
/// first because the last one to capture a range wins.
const INJECTIONS: &str = r#"
((script_element (raw_text) @injection.content)
 (#set! injection.language "javascript"))

((script_element
   (start_tag (attribute (attribute_name) @_name (quoted_attribute_value (attribute_value) @_value)))
   (raw_text) @injection.content)
 (#eq? @_name "lang")
 (#any-of? @_value "ts" "typescript")
 (#set! injection.language "typescript"))

((style_element (raw_text) @injection.content)
 (#set! injection.language "css"))

((expression (svelte_raw_text) @injection.content)
 (#set! injection.language "javascript"))
"#;

/// Svelte's grammar extends HTML's and its own query only covers what Svelte
/// adds (blocks and their keywords), so HTML's comes first.
fn highlight_query() -> String {
    [tree_sitter_html::HIGHLIGHTS_QUERY, tree_sitter_svelte_ng::HIGHLIGHTS_QUERY].join("\n")
}

/// An element, a script or style block, and each `{#...}` block fold from its
/// opening to its closing tag; nothing is pinned, as in HTML.
fn node_kinds() -> NodeKinds {
    NodeKinds {
        scopes: Vec::new(),
        foldable: [
            "element",
            "script_element",
            "style_element",
            "if_statement",
            "each_statement",
            "await_statement",
            "key_statement",
            "snippet_statement",
            "comment",
        ]
        .map(str::to_string)
        .to_vec(),
        import: None,
    }
}

#[cfg(test)]
#[path = "lib_test.rs"]
mod lib_test;
