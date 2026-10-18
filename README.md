# ytm-play

A small terminal YouTube Music client I wrote in Rust while learning async
networking and JSON parsing. It searches YouTube Music and plays the chosen
track's audio through `mpv`. It's a hands-on learning project that I'm still
refactoring.

## What it does

- Queries the YouTube Music internal (`innertube`) search API.
- Parses the nested JSON response using `Option` combinators
  (`and_then` / `or_else`) so missing fields don't panic.
- Lists up to 10 results in a continuous search loop.
- Plays the selected track's audio by launching `mpv --no-video`.

Built with `tokio` and `reqwest` (async), and `serde` / `serde_json` for parsing.

## Prerequisites

- Rust and Cargo (latest stable toolchain)
- `mpv` installed and on your PATH

## Usage

```bash
git clone https://github.com/JayasuryaNair01/ytm-play.git
cd ytm-play
cargo run
```

Type a song or artist at the prompt, pick a number to play, or `exit` to quit.

## Limitations / notes

- It relies on YouTube Music's internal API, which can change without notice
  and may break parsing at any time.
- The "artist" field is just the search query, not the real track artist.
- No tests or error-recovery beyond skipping failed requests yet.
- Built to learn Rust; expect rough edges.
