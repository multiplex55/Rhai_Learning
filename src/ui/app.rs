//! eframe/`egui` application displaying and executing Rhai examples.

use crate::examples::{Example, ExampleRegistry};
use eframe::egui;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::{channel, Receiver};

/// Top-level application state for the Rhai learning UI.
pub struct App {
    examples: Vec<Example>,
    selected: Option<usize>,
    console: String,
    logs: String,
    #[allow(dead_code)]
    watcher: RecommendedWatcher,
    watch_rx: Receiver<Event>,
    reload_notice: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        let examples = ExampleRegistry::all();
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        })
        .expect("failed to create watcher");
        watcher
            .watch(std::path::Path::new("examples"), RecursiveMode::Recursive)
            .expect("failed to watch examples");
        Self {
            examples,
            selected: None,
            console: String::new(),
            logs: String::new(),
            watcher,
            watch_rx: rx,
            reload_notice: None,
        }
    }
}

impl App {
    fn run_selected(&mut self) {
        if let Some(idx) = self.selected {
            if let Some(example) = self.examples.get(idx) {
                let result = example.run();
                self.console.clear();
                if !result.stdout.is_empty() {
                    self.console.push_str(&result.stdout);
                }
                self.console.push_str(&format!("=> {}", result.value));

                let log_path = format!("logs/{}.log", example.id);
                self.logs = std::fs::read_to_string(log_path).unwrap_or_default();
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(event) = self.watch_rx.try_recv() {
            if event
                .paths
                .iter()
                .any(|p| p.extension().and_then(|s| s.to_str()) == Some("rhai"))
            {
                let selected_id = self
                    .selected
                    .and_then(|i| self.examples.get(i).map(|e| e.id.clone()));
                self.examples = ExampleRegistry::all();
                self.selected =
                    selected_id.and_then(|id| self.examples.iter().position(|e| e.id == id));
                if self.selected.is_some() {
                    self.run_selected();
                }
                self.reload_notice = Some("Scripts recompiled".to_string());
            }
        }

        // Side panel listing all examples and reload button.
        egui::SidePanel::left("example_list").show(ctx, |ui| {
            if ui.button("Reload scripts").clicked() {
                let selected_id = self
                    .selected
                    .and_then(|i| self.examples.get(i).map(|e| e.id.clone()));
                self.examples = ExampleRegistry::all();
                self.selected =
                    selected_id.and_then(|id| self.examples.iter().position(|e| e.id == id));
                if self.selected.is_some() {
                    self.run_selected();
                }
            }

            ui.separator();

            if let Some(msg) = self.reload_notice.take() {
                ui.label(egui::RichText::new(msg).color(egui::Color32::LIGHT_GREEN));
            }

            for (i, ex) in self.examples.iter().enumerate() {
                if ui
                    .selectable_label(self.selected == Some(i), &ex.name)
                    .clicked()
                {
                    self.selected = Some(i);
                }
            }
        });

        // Optional log viewer on the right.
        if !self.logs.is_empty() {
            egui::SidePanel::right("logs").show(ctx, |ui| {
                ui.heading("Logs");
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.monospace(&self.logs);
                });
            });
        }

        // Console panel at the bottom.
        egui::TopBottomPanel::bottom("console").show(ctx, |ui| {
            ui.label("Console");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.monospace(&self.console);
            });
        });

        // Main central panel with example details.
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(idx) = self.selected {
                let ex = &self.examples[idx];
                ui.heading(&ex.name);
                ui.label(&ex.description);
                if let Some(note) = &ex.note {
                    ui.label(format!("Note: {}", note));
                }
                ui.hyperlink_to("Documentation", ex.doc_html_path.to_string_lossy());
                if ui.button("Run").clicked() {
                    self.run_selected();
                }
            } else {
                ui.label("Select an example from the left");
            }
        });
    }
}
