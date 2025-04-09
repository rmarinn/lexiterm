# Lexiterm

Lexiterm is a terminal-based TUI tool for searching words given a set of letters and optional regex constraints.

![Demo GIF](./assets/lexiterm-demo-1.gif) <!-- Replace with your demo gif path -->

---

You can use Lexiterm to search with powerful Regex patterns like:

1. **Words that start with an `S` and end with an `e`:**  
   Regex: `^s.*e$`  
   _Matches examples like `spare`, `scale`, `settle`_  
   ![Starts with S and ends with E](./assets/lexiterm-demo-regex-1.gif)

2. **Words that contain the letters `cl` together:**  
   Regex: `cl`  
   _Matches examples like `clear`, `clash`, `decline`_  
   ![Contains CL](./assets/lexiterm-demo-regex-2.gif)

3. **Words with an `r` and an `s` separated by exactly 3 letters:**  
   Regex: `r.{3}s`  
   _Matches examples like `rails`, `rents`, `reaps`_  
   ![R...S](./assets/lexiterm-demo-regex-3.gif)

4. **And much more...**  
   If you can write it in [Regex](https://regexone.com/), you can search for it.

## Installation

### Pre-built Binaries

1. Head over to the [Releases section](https://github.com/rmarinn/lexiterm/releases)
2. Download the binary for your OS

### Build from Source

1. Clone the repo:  
   ```bash
   git clone https://github.com/your-username/lexiterm.git
   cd lexiterm
   ```

2. Build it:  
   ```bash
   cargo build --release
   ```

3. Run it:  
   ```bash
   ./target/release/lexiterm
   ```

## Features

- Real-time filtering with full Regex support
- Match highlighting for easy scanning
- Fast, responsive, and fully keyboard-driven
- Works offline
- Minimalist TUI built with `crossterm` and `ratatui`
- Customizable word list — just edit `words.txt`
- Adjustable letter scoring — tweak `char_scores.txt` to your liking

## Contributing

Issues and PRs are welcome! Feel free to open an issue with ideas, bugs, or feature requests.

## License

Lexiterm is licensed under the [MIT License](./LICENSE)
