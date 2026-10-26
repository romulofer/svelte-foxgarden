//! The Svelte language server (`svelte-language-server`, whose program is
//! `svelteserver`).
//!
//! Not bundled: it is an npm package the user installs, in the project
//! (`npm install --save-dev svelte-language-server typescript`) or globally.
//! It is found in the project's `node_modules/.bin` first, then on `PATH`, and
//! a path configured in Settings wins over both.

use std::path::{Path, PathBuf};

use fg_extension::{LanguageServerContribution, ResolvedServerStart, ServerStartContext};

use crate::SVELTE;

pub const SVELTE_SERVER: &str = "svelte-language-server";
const PROGRAM: &str = "svelteserver";

pub(crate) fn contribution() -> LanguageServerContribution {
    LanguageServerContribution {
        id: SVELTE_SERVER.to_string(),
        display_name: "Svelte Language Server".to_string(),
        language_ids: vec![SVELTE.to_string()],
        binary_name: PROGRAM.to_string(),
        args: vec!["--stdio".to_string()],
        initialization_options: None,
        install: None,
        virtual_documents: None,
        host_runtime: None,
    }
}

/// A program installed into the project (`node_modules/.bin/<name>`, with the
/// `.cmd` shim npm writes on Windows).
fn project_program(project_root: &Path, name: &str) -> Option<PathBuf> {
    let bin = project_root.join("node_modules").join(".bin");
    [name.to_string(), format!("{name}.cmd")].into_iter().map(|file| bin.join(file)).find(|candidate| candidate.is_file())
}

/// How to start `server_id` for the project in `context`, or `None` for a server
/// this extension does not own.
pub(crate) fn resolve(server_id: &str, context: &ServerStartContext) -> Option<ResolvedServerStart> {
    if server_id != SVELTE_SERVER {
        return None;
    }
    let configured = context.configured_binary.trim();
    let binary = if !configured.is_empty() {
        PathBuf::from(configured)
    } else {
        context
            .project_root
            .as_deref()
            .and_then(|root| project_program(root, PROGRAM))
            .unwrap_or_else(|| PathBuf::from(PROGRAM))
    };
    Some(ResolvedServerStart { binary, args: contribution().args, initialization_options: None, restart_key: String::new() })
}

#[cfg(test)]
#[path = "server_test.rs"]
mod server_test;
