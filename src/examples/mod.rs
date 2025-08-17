//! Utilities for loading, running, and documenting Rhai example scripts.

use rand::Rng;
use rhai::{Dynamic, Engine, module_resolvers::FileModuleResolver};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn length(&mut self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }
}

fn http_get(url: &str) -> Dynamic {
    match reqwest::blocking::get(url) {
        Ok(resp) => match resp.json::<Dynamic>() {
            Ok(json) => json,
            Err(e) => format!("Error parsing JSON: {e}").into(),
        },
        Err(e) => format!("Request error: {e}").into(),
    }
}

fn to_json(value: Dynamic) -> String {
    serde_json::to_string(&value).unwrap_or_default()
}

fn from_json(s: &str) -> Dynamic {
    serde_json::from_str::<Dynamic>(s).unwrap_or(Dynamic::UNIT)
}

fn assert_fn(cond: bool) {
    if !cond {
        panic!("assertion failed");
    }
}

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| format!("Error reading file: {e}"))
}

fn sleep_ms(ms: i64) {
    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
}

fn rand_int(min: i64, max: i64) -> i64 {
    rand::thread_rng().gen_range(min..=max)
}

/// Metadata and execution support for a single Rhai example.
#[derive(Clone, Debug)]
pub struct Example {
    /// Unique identifier for the example.
    pub id: String,
    /// Human‑readable display name.
    pub name: String,
    /// Short description shown in the UI.
    pub description: String,
    /// Optional additional note.
    pub note: Option<String>,
    /// Path to the Markdown documentation.
    #[allow(dead_code)]
    pub doc_path: PathBuf,
    /// Path to the rendered HTML documentation.
    pub doc_html_path: PathBuf,
    /// Path to the Rhai script file.
    pub script_path: PathBuf,
}

/// Result of running a Rhai example.
pub struct RunResult {
    /// Captured standard output from the script.
    pub stdout: String,
    /// Final value returned by the script.
    pub value: Dynamic,
}

impl Example {
    /// Run this example's script, capturing any printed output and returning the result.
    pub fn run(&self) -> RunResult {
        let stdout = Arc::new(Mutex::new(String::new()));
        let mut engine = Engine::new();
        let mut resolver = FileModuleResolver::new();
        if let Some(parent) = self.script_path.parent() {
            resolver.set_base_path(parent);
        }
        engine.set_module_resolver(resolver);
        // Capture calls to `print` into our stdout buffer.
        let out = stdout.clone();
        engine.on_print(move |s| {
            if let Ok(mut buf) = out.lock() {
                buf.push_str(s);
                buf.push('\n');
            }
        });

        // Capture debug output as well.
        let out_dbg = stdout.clone();
        engine.on_debug(move |s, _, _| {
            if let Ok(mut buf) = out_dbg.lock() {
                buf.push_str("DEBUG: ");
                buf.push_str(s);
                buf.push('\n');
            }
        });

        // Register custom Rust types and helper functions.
        engine.register_type::<Point>();
        engine.register_fn("Point", Point::new);
        engine.register_fn("length", Point::length);
        engine.register_fn("http_get", http_get);
        engine.register_fn("to_json", to_json);
        engine.register_fn("from_json", from_json);
        engine.register_fn("assert", assert_fn);
        engine.register_fn("read_file", read_file);
        engine.register_fn("sleep_ms", sleep_ms);
        engine.register_fn("rand_int", rand_int);

        // Evaluate the script file so relative imports work.
        let value = engine
            .eval_file::<Dynamic>(self.script_path.clone())
            .unwrap_or_else(|e| format!("Error: {e}").into());

        let stdout = stdout.lock().map(|s| s.clone()).unwrap_or_default();

        if !stdout.is_empty() {
            let log_dir = std::path::Path::new("logs");
            let _ = std::fs::create_dir_all(log_dir);
            let log_path = log_dir.join(format!("{}.log", self.id));
            let _ = std::fs::write(log_path, &stdout);
        }

        RunResult { stdout, value }
    }

    /// Run a provided script text for this example instead of reading from file.
    ///
    /// The script is executed with the same engine configuration as [`run`].
    pub fn run_script(&self, script: &str) -> RunResult {
        let stdout = Arc::new(Mutex::new(String::new()));
        let mut engine = Engine::new();
        let mut resolver = FileModuleResolver::new();
        if let Some(parent) = self.script_path.parent() {
            resolver.set_base_path(parent);
        }
        engine.set_module_resolver(resolver);
        // Capture calls to `print` into our stdout buffer.
        let out = stdout.clone();
        engine.on_print(move |s| {
            if let Ok(mut buf) = out.lock() {
                buf.push_str(s);
                buf.push('\n');
            }
        });

        // Capture debug output as well.
        let out_dbg = stdout.clone();
        engine.on_debug(move |s, _, _| {
            if let Ok(mut buf) = out_dbg.lock() {
                buf.push_str("DEBUG: ");
                buf.push_str(s);
                buf.push('\n');
            }
        });

        // Register custom Rust types and helper functions.
        engine.register_type::<Point>();
        engine.register_fn("Point", Point::new);
        engine.register_fn("length", Point::length);
        engine.register_fn("http_get", http_get);
        engine.register_fn("to_json", to_json);
        engine.register_fn("from_json", from_json);
        engine.register_fn("assert", assert_fn);
        engine.register_fn("read_file", read_file);
        engine.register_fn("sleep_ms", sleep_ms);
        engine.register_fn("rand_int", rand_int);

        // Evaluate the provided script text.
        let value = engine
            .eval::<Dynamic>(script)
            .unwrap_or_else(|e| format!("Error: {e}").into());

        let stdout = stdout.lock().map(|s| s.clone()).unwrap_or_default();

        if !stdout.is_empty() {
            let log_dir = std::path::Path::new("logs");
            let _ = std::fs::create_dir_all(log_dir);
            let log_path = log_dir.join(format!("{}.log", self.id));
            let _ = std::fs::write(log_path, &stdout);
        }

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
                let doc_html_path = PathBuf::from(&m.doc);
                let doc_path = doc_html_path.with_extension("md");
                let script_path = PathBuf::from(&m.script);
                let (description, note) = parse_doc(&doc_path);
                Example {
                    id: m.id,
                    name: m.name,
                    description,
                    note,
                    doc_path,
                    doc_html_path,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_new_examples() {
        let ids = [
            "basic-arith",
            "use-struct",
            "http-request",
            "serde-demo",
            "perf-loop",
            "unit-tests",
            "hot-swap",
            "custom-module",
            "async-sim",
        ];

        let registry = ExampleRegistry::all();
        for id in ids {
            let ex = registry
                .iter()
                .find(|e| e.id == id)
                .expect("example not found");
            let result = ex.run();
            println!("{} => {} | {:?}", id, result.stdout.trim(), result.value);
        }
    }
}
