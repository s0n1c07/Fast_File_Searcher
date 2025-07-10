# 🚀 Fast File Searcher (Windows Only)

A blazing-fast file searcher app for **Windows**, written in **Rust** with an intuitive GUI using `egui`.

---

## ⚡️ Features

- ✅ Super-fast multi-threaded file searching (`ignore` crate + all CPU cores)
- ✅ Simple desktop GUI — just enter your query & path, click **Search**
- ✅ Cancel search any time
- ✅ Shows **total results**
- ✅ Open found files directly in Windows Explorer
- ✅ Displays **accurate elapsed time** for the whole search

---

## 📸 App Structure

Fast File Searcher 🚀
[ Search Query: __________ ]
[ Path to Search: __________ ]
[ Start Search ] [ Cancel ]
Searching... / Search Complete!
Elapsed: 12.4s
Total results: 123

Results:
C:\Users\You\Documents\example.txt [Open]
...

---

## 🗂️ Requirements

- **Windows OS**
- [Rust](https://www.rust-lang.org/tools/install) installed (`cargo` + `rustc`)

---

## ⚙️ Build & Run

1️⃣ **Clone this repo:**
```bash
git clone https://github.com/YOUR-USERNAME/fast-file-searcher.git
cd fast-file-searcher
```
2️⃣ **Build:**

```bash
cargo build --release
```
3️⃣ **Run:**
```bash
cargo run --release
```
✅ **The .exe will be at:**

 - target/release/fast-file-searcher.exe
 - You can share this .exe with other Windows users — they don’t need Rust installed.

## ⚙️ How It Works
 - Uses ignore::WalkBuilder for efficient file walking.

 - Automatically uses all CPU cores (num_cpus).

 - Uses threads & channels for real-time streaming of results.

 - Built with eframe + egui for a modern, native desktop UI.

 - Measures & displays real elapsed time from start to final result.

## ❌ Platform Support
 - Windows only.
 - Explorer open functionality is Windows-specific.

## ✨ Credits
 - Written in Rust 🦀
 - GUI by eframe
 - Fast directory walking by ignore
