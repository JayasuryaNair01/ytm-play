# ytm-play 🎵🦀

A blazing-fast, terminal-based YouTube Music client written in Rust. 

Designed to query the YouTube Music internal API directly, parse deep nested JSON structures safely using functional option chaining, and stream audio locally using system commands.

## Features
- **Async HTTP Requests:** Built with `reqwest` and `tokio` for high-performance non-blocking network calls.
- **Defensive JSON Parsing:** Utilizes Rust's pattern matching and `Option` combinators (`and_then`, `or_else`) to safely traverse complex, changing API payloads without risk of runtime panics.
- **Interactive TUI Flow:** Continuous search loop with dynamic track selection.
- **System Integration:** Seamlessly spawns external media processes (`mpv`) for lightweight audio playback.

## Prerequisites
Ensure you have the following installed on your system before running the binary:
- **Rust & Cargo** (Latest stable toolchain)
- **mpv** (Required for audio playback backend)

### Installation & Usage

1. Clone the repository:
   ```bash
   git clone [https://github.com/JayasuryaNair01/ytm-play.git](https://github.com/JayasuryaNair01/ytm-play.git)
   cd ytm-play
