use eframe::{App, Frame, egui};
use ignore::WalkBuilder;
use num_cpus;
use std::process::Command;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc::channel,
};
use std::thread;
use std::time::{Duration, Instant};

struct FileSearcherApp {
    query: String,
    path: String,
    results: Arc<Mutex<Vec<String>>>,
    is_searching: Arc<AtomicBool>,
    cancel_search: Arc<AtomicBool>,
    elapsed: Arc<Mutex<Option<Duration>>>,
}

impl Default for FileSearcherApp {
    fn default() -> Self {
        Self {
            query: String::new(),
            path: String::new(),
            results: Arc::new(Mutex::new(Vec::new())),
            is_searching: Arc::new(AtomicBool::new(false)),
            cancel_search: Arc::new(AtomicBool::new(false)),
            elapsed: Arc::new(Mutex::new(None)),
        }
    }
}

impl App for FileSearcherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Fast File Searcher 🚀 (Windows)");

            ui.horizontal(|ui| {
                ui.label("Search Query:");
                ui.text_edit_singleline(&mut self.query);
            });

            ui.horizontal(|ui| {
                ui.label("Path to Search:");
                ui.text_edit_singleline(&mut self.path);
            });

            ui.horizontal(|ui| {
                if ui.button("Start Search").clicked() && !self.is_searching.load(Ordering::Relaxed)
                {
                    let query = self.query.to_lowercase();
                    let path = self.path.clone();
                    let results = Arc::clone(&self.results);
                    let is_searching = Arc::clone(&self.is_searching);
                    let cancel_search = Arc::clone(&self.cancel_search);
                    let elapsed = Arc::clone(&self.elapsed);

                    results.lock().unwrap().clear();
                    is_searching.store(true, Ordering::Relaxed);
                    cancel_search.store(false, Ordering::Relaxed);
                    *elapsed.lock().unwrap() = None;

                    thread::spawn(move || {
                        let start = Instant::now();

                        let (tx, rx) = channel();

                        WalkBuilder::new(&path)
                            .threads(num_cpus::get())
                            .build_parallel()
                            .run(|| {
                                let tx = tx.clone();
                                let query = query.clone();
                                let cancel_search = Arc::clone(&cancel_search);

                                Box::new(move |entry| {
                                    if cancel_search.load(Ordering::Relaxed) {
                                        return ignore::WalkState::Quit;
                                    }

                                    if let Ok(entry) = entry {
                                        let name =
                                            entry.file_name().to_string_lossy().to_lowercase();
                                        if name.contains(&query) {
                                            tx.send(entry.path().display().to_string()).unwrap();
                                        }
                                    }
                                    ignore::WalkState::Continue
                                })
                            });

                        drop(tx);

                        for path in rx {
                            if cancel_search.load(Ordering::Relaxed) {
                                break;
                            }
                            results.lock().unwrap().push(path);
                        }

                        // ✅ This is the real end of the whole search
                        *elapsed.lock().unwrap() = Some(start.elapsed());
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

            if let Some(elapsed) = *self.elapsed.lock().unwrap() {
                ui.label(format!("Elapsed: {:.2?}", elapsed));
            }

            let results = self.results.lock().unwrap();
            ui.label(format!("Total results: {}", results.len()));

            ui.separator();
            ui.heading("Results (first 50 shown):");

            egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
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
    // Windows only:
    let _ = Command::new("explorer").arg("/select,").arg(path).spawn();
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Fast File Searcher (Windows)",
        options,
        Box::new(|_cc| Box::new(FileSearcherApp::default())),
    )
}
