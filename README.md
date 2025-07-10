## 📂 Fast File Searcher 🚀 (for Windows only)
A super-fast file searcher app for Windows, written in Rust with an easy-to-use GUI powered by egui.

⚡️ Features
✅ Ultra-fast file search using multithreaded parallel scanning (ignore crate + all CPU cores)
✅ Simple desktop GUI — just enter your query & path, click Search
✅ Cancel search any time
✅ See total results found
✅ Open found files directly in Windows Explorer
✅ Shows elapsed time for each search

🖼️ Screenshot
mathematica
Copy
Edit
Fast File Searcher 🚀
[ Search Query:  ___ ]
[ Path to Search: ___ ]
[ Start Search ] [ Cancel ]
Searching... / Search Complete!
Elapsed: 12.4s
Total results: 123
Results:
C:\Users\You\Documents\example.txt   [Open]
...
🗂️ Requirements
Windows OS

Rust installed (cargo + rustc)

Internet connection for cargo build

🛠️ Build & Run
1️⃣ Clone the repo:

bash
Copy
Edit
git clone https://github.com/YOUR-USERNAME/fast-file-searcher.git
cd fast-file-searcher
2️⃣ Build:

bash
Copy
Edit
cargo build --release
3️⃣ Run:

bash
Copy
Edit
cargo run --release
The .exe will be at:

arduino
Copy
Edit
target/release/fast-file-searcher.exe
You can share this .exe with other Windows users.

⚙️ How it works
Uses ignore::WalkBuilder to walk directories, skipping .gitignore files where possible.

Spawns threads equal to num_cpus for fast parallel searching.

Uses a channel to collect matches in real time.

Uses eframe + egui for the desktop GUI.

Shows elapsed time from start to real finish.

❌ Platform
Windows-only!
No Linux/Mac support — by design.
All explorer open logic is pure Windows.

✨ Credits
Built in Rust 🦀

GUI by eframe/egui

Fast file walking by ignore
