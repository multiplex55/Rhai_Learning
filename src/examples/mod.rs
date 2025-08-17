use rhai::{Dynamic, Engine};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Metadata and execution support for a single Rhai example.
#[derive(Clone, Debug)]
pub struct Example {
    pub id: String,
    pub name: String,
    pub description: String,
    pub note: Option<String>,
    pub doc_path: PathBuf,
    pub script_path: PathBuf,
}

/// Result of running a Rhai example.
pub struct RunResult {
    pub stdout: String,
    pub value: Dynamic,
}

impl Example {
    /// Run this example's script, capturing any printed output and returning the result.
    pub fn run(&self) -> RunResult {
        let stdout = Arc::new(Mutex::new(String::new()));
        let mut engine = Engine::new();
        // Capture calls to `print` into our stdout buffer.
        let out = stdout.clone();
        engine.on_print(move |s| {
            if let Ok(mut buf) = out.lock() {
                buf.push_str(s);
                buf.push('\n');
            }
        });

        // Custom Rust types/functions could be registered on `engine` here.
        let value = match std::fs::read_to_string(&self.script_path) {
            Ok(code) => engine
                .eval::<Dynamic>(&code)
                .unwrap_or_else(|e| format!("Error: {e}").into()),
            Err(e) => format!("Error loading script: {e}").into(),
        };

        let stdout = stdout.lock().map(|s| s.clone()).unwrap_or_default();
        RunResult { stdout, value }
    }
}

#[derive(Deserialize)]
struct Manifest {
    examples: Vec<ManifestEntry>,
}

#[derive(Deserialize)]
struct ManifestEntry {
    id: String,
    name: String,
    script: String,
    doc: String,
}

/// Registry of examples loaded from the manifest file.
pub struct ExampleRegistry {
    examples: Vec<Example>,
}

impl ExampleRegistry {
    /// Load the example registry from `examples/manifest.toml`.
    pub fn load() -> Self {
        let manifest_path = Path::new("examples/manifest.toml");
        let data =
            std::fs::read_to_string(manifest_path).expect("failed to read examples manifest");
        let manifest: Manifest = toml::from_str(&data).expect("failed to parse examples manifest");

        let examples = manifest
            .examples
            .into_iter()
            .map(|m| {
                let doc_path = PathBuf::from(&m.doc);
                let script_path = PathBuf::from(&m.script);
                let (description, note) = parse_doc(&doc_path);
                Example {
                    id: m.id,
                    name: m.name,
                    description,
                    note,
                    doc_path,
                    script_path,
                }
            })
            .collect();

        Self { examples }
    }
}

/// Parse a markdown document for description and optional note.
fn parse_doc(path: &Path) -> (String, Option<String>) {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let mut description = String::new();
    let mut note = None;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if description.is_empty() {
            description = line.to_string();
        } else if line.to_lowercase().starts_with("note:") {
            note = Some(line[5..].trim().to_string());
            break;
        }
    }

    (description, note)
}

impl ExampleRegistry {
    /// Return all examples sorted by id.
    pub fn all() -> Vec<Example> {
        let mut registry = Self::load();
        registry.examples.sort_by(|a, b| a.id.cmp(&b.id));
        registry.examples
    }
}
