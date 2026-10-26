# svelte-foxgarden

Svelte support for [FoxGarden](../foxgarden), as an extension rather than as
part of the editor.

It reaches the editor only through `fg-extension`, like
[`spring-foxgarden`](../spring-foxgarden), [`kmp-foxgarden`](../kmp-foxgarden),
[`python-foxgarden`](../python-foxgarden) and [`ts-foxgarden`](../ts-foxgarden).
This checkout must sit beside `foxgarden`.

## What it contributes

| | |
|---|---|
| Language | `svelte` (`.svelte`) |
| Grammar | [`tree-sitter-svelte-ng`](https://crates.io/crates/tree-sitter-svelte-ng); highlights are HTML's query followed by Svelte's (blocks and their keywords) |
| Folding | Elements, `<script>` / `<style>`, and `{#if}` / `{#each}` / `{#await}` / `{#key}` / `{#snippet}` blocks |
| Language server | `svelte-language-server` (`svelteserver --stdio`), not bundled: found in the project's `node_modules/.bin`, then on `PATH`; a path set in Settings ▸ Language Servers… wins |

```bash
npm install --save-dev svelte-language-server typescript
```

| Injections | `<script>` is JavaScript (TypeScript with `lang="ts"`), `<style>` is CSS and the expression in `{...}` is JavaScript, each highlighted as that language |

## Not here yet

`lang="scss"` and other preprocessors are highlighted as CSS. Also missing: a
component scaffold and an installer for the server.

## Development

```sh
cargo test --workspace
```

The real-server test lives in `foxgarden`
(`svelte_completion_against_a_real_server_offers_html_tags`, ignored by
default; point `FOXGARDEN_NODE_MODULES` at an npm install of
`svelte-language-server` and `typescript`).
