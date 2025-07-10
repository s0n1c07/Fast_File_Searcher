use eframe::{egui, App, Frame};
use walkdir::WalkDir;
use rayon::prelude::*;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::process::Command;

struct FileSearcherApp {
    query: String,
    path: String,
    results: Arc<Mutex<Vec<String>>>,
    is_searching: Arc<AtomicBool>,
    cancel_search: Arc<AtomicBool>,
}

impl Default for FileSearcherApp {
    fn default() -> Self {
        Self {
            query: String::new(),
            path: String::new(),
            results: Arc::new(Mutex::new(Vec::new())),
            is_searching: Arc::new(AtomicBool::new(false)),
            cancel_search: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl App for FileSearcherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Fast File Searcher 🚀");

            ui.horizontal(|ui| {
                ui.label("Search Query:");
                ui.text_edit_singleline(&mut self.query);
            });

            ui.horizontal(|ui| {
                ui.label("Path to Search:");
                ui.text_edit_singleline(&mut self.path);
            });

            ui.horizontal(|ui| {
                if ui.button("Start Search").clicked() && !self.is_searching.load(Ordering::Relaxed) {
                    let query = self.query.clone();
                    let path = self.path.clone();
                    let results = Arc::clone(&self.results);
                    let is_searching = Arc::clone(&self.is_searching);
                    let cancel_search = Arc::clone(&self.cancel_search);

                    results.lock().unwrap().clear();
                    is_searching.store(true, Ordering::Relaxed);
                    cancel_search.store(false, Ordering::Relaxed);

                    thread::spawn(move || {
                        WalkDir::new(path)
                            .into_iter()
                            .par_bridge()
                            .filter_map(Result::ok)
                            .filter(|entry| {
                                !cancel_search.load(Ordering::Relaxed) && entry
                                    .file_name()
                                    .to_string_lossy()
                                    .to_lowercase()
                                    .contains(&query.to_lowercase())
                            })
                            .map(|entry| entry.path().display().to_string())
                            .for_each(|path| {
                                if !cancel_search.load(Ordering::Relaxed) {
                                    results.lock().unwrap().push(path);
                                }
                            });

                        is_searching.store(false, Ordering::Relaxed);
                    });
                }

                if self.is_searching.load(Ordering::Relaxed) {
                    if ui.button("Cancel").clicked() {
                        self.cancel_search.store(true, Ordering::Relaxed);
                    }
                }
            });

            if self.is_searching.load(Ordering::Relaxed) {
                ui.label("Searching... Please wait.");
            } else {
                ui.label("Search complete!");
            }

            let results = self.results.lock().unwrap();
            ui.label(format!("Total results: {}", results.len()));

            ui.separator();
            ui.heading("Results (first 50 shown):");

            egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                for path in results.iter().take(50) {
                    ui.horizontal(|ui| {
                        ui.label(path);
                        if ui.button("Open").clicked() {
                            open_in_explorer(path);
                        }
                    });
                }
            });
        });
    }
}

fn open_in_explorer(path: &str) {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("explorer").arg("/select,").arg(path).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg("-R").arg(path).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("xdg-open").arg(path).spawn();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Fast File Searcher",
        options,
        Box::new(|_cc| Box::new(FileSearcherApp::default())),
    )
}
// new file