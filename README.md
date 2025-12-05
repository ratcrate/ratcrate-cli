# 🦀 ratcrate-cli: Ratatui Ecosystem Crate Discoverer


[![Rust Build Status](https://github.com/rvbug/ratcrate-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/rvbug/ratcrate-cli/actions/workflows/rust.yml)
[![CI](https://github.com/rvbug/ratcrate-cli/workflows/CI/badge.svg)](https://github.com/rvbug/ratcrate-cli/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/ratcrate.svg)](https://crates.io/crates/ratcrate)
[![Downloads](https://img.shields.io/crates/d/doc2quarto.svg)](https://crates.io/crates/doc2quarto)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

**`ratcrate-cli`** is a fast, terminal-native command-line tool built in **Rust** for discovering, searching, and getting information about crates within the Ratatui ecosystem. It uses a smart caching strategy to provide near-instant results directly in your terminal.

## ✨ Features

* **Offline First:** Caches data locally and only downloads when stale (older than 1 day) or forced.
* **Fuzzy Search:** Filter crates by name or description using a command-line query.
* **Interactive Mode:** Seamless integration with **`fzf`** for interactive selection and viewing of crate details.
* **Statistical Breakdown:** Quick summary of total, core, and community packages (`-t`).
* **Flexible Output:** Supports both a clean, colored list view and an optional structured table view (`--table`).

---

## 🚀 Installation

### Prerequisites

1.  **Rust Toolchain:** You need the latest stable Rust version installed via [rustup](https://rustup.rs/).
2.  **fzf (Optional):** Required for the interactive mode (`-f`). Install it using your system's package manager (e.g., `sudo apt install fzf` or `brew install fzf`).

### Install from Crates.io (Recommended)

Once the package is published, you can install it globally:

```bash
$> cargo install ratcrate-cli



## Build from Source
Clone the repository and build the project using Cargo:

```bash
$> git clone [https://github.com/your-username/ratcrate-cli.git](https://github.com/your-username/ratcrate-cli.git)
$> cd ratcrate-cli
$> cargo install --path 
```
```

## Usage

Run `ratcrate-cli --help` for a full list of options.

| Option | Description |
| --- | --- |
| `-h, --help` | Display help information. |
| `-f, --fzf` | Enable interactive mode with fzf. |
| `-t, --total` | Display total number of crates. |
| `-v, --version` | Display the version number. |
| `-h, --help` | Display help information. |

## 

```bash
$ ratcrate-cli -t

────────────────────────────────────────────────────────
Total Crates Overview
────────────────────────────────────────────────────────
Total Crates: 125
  Core Libraries: 8
  Community Packages: 117
────────────────────────────────────────────────────────
Data generated at: 2025-12-04T12:00:00Z
```


# 🛠️ Architecture Overview

I can certainly create a comprehensive README for your ratcrate-cli project, formatted in standard GitHub Markdown (.md).Since I cannot directly generate a downloadable file link, I will provide the complete content here. You can copy and paste this into a file named README.md in the root of your repository.This README is structured to be production-grade, covering the project's purpose, installation, usage, and architectural details.Markdown# 🦀 ratcrate-cli: Ratatui Ecosystem Crate Discoverer

[![Rust Build Status](https://github.com/your-username/ratcrate-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/your-username/ratcrate-cli/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/ratcrate.svg)](https://crates.io/crates/ratcrate)

**`ratcrate-cli`** is a fast, terminal-native command-line tool built in **Rust** for discovering, searching, and getting information about crates within the Ratatui ecosystem. It uses a smart caching strategy to provide near-instant results directly in your terminal.

## ✨ Features

* **Offline First:** Caches data locally and only downloads when stale (older than 1 day) or forced.
* **Fuzzy Search:** Filter crates by name or description using a command-line query.
* **Interactive Mode:** Seamless integration with **`fzf`** for interactive selection and viewing of crate details.
* **Statistical Breakdown:** Quick summary of total, core, and community packages (`-t`).
* **Flexible Output:** Supports both a clean, colored list view and an optional structured table view (`--table`).

---

## 🚀 Installation

### Prerequisites

1.  **Rust Toolchain:** You need the latest stable Rust version installed via [rustup](https://rustup.rs/).
2.  **fzf (Optional):** Required for the interactive mode (`-f`). Install it using your system's package manager (e.g., `sudo apt install fzf` or `brew install fzf`).

### Install from Crates.io (Recommended)

Once the package is published, you can install it globally:

```bash
cargo install ratcrate-cli
Build from SourceClone the repository and build the project using Cargo:Bashgit clone [https://github.com/your-username/ratcrate-cli.git](https://github.com/your-username/ratcrate-cli.git)
cd ratcrate-cli
cargo install --path .
💡 UsageRun ratcrate-cli --help for a full list of options.Basic CommandsCommandDescriptionratcrate-cliLists the top 20 crates by default.ratcrate-cli -q "widgets"Searches for crates containing "widgets" in the name or description.ratcrate-cli -l 50 -q "app"Limits the search results to the top 50 matching "app".ratcrate-cli -tShows the total count and breakdown of Core vs. Community crates.ratcrate-cli -fLaunches interactive fuzzy search using fzf.ratcrate-cli -rForces a refresh of the cached data from the remote source.Example: Displaying Total CratesBash$ ratcrate-cli -t

────────────────────────────────────────────────────────
Total Crates Overview
────────────────────────────────────────────────────────
Total Crates: 125
  Core Libraries: 8
  Community Packages: 117
────────────────────────────────────────────────────────
Data generated at: 2025-12-04T12:00:00Z
Example: Interactive Mode (-f)The interactive mode allows you to quickly filter and select a crate to view its full details (repository, documentation, downloads, etc.).Bash$ ratcrate-cli -f
🛠️ Architecture OverviewThe project follows a standard Rust CLI structure, ensuring clear separation of responsibilities:1. main.rs: Application Logic and CLIHandles argument parsing using clap.Manages the top-level execution flow, prioritizing immediate exit commands (-t, -c).Implements the core filtering and display logic.Manages interaction with the external fzf command.2. cache.rs: Data PersistenceManages the data source, which is a JSON file hosted remotely.Calculates the cache age (set to 1 day maximum).Handles fetching data via reqwest and saving/loading data from the OS-specific cache directory using dirs.3. types.rs: Data ModelingDefines the data contract for the remote JSON.Uses serde to deserialize the data into structured CratePackage and CratesData structs.


## 🤝 Contributing
Contributions are welcome! If you have suggestions or find a bug, please open an issue or submit a pull request.

## 📝 License
This project is licensed under the MIT License. See the LICENSE file for details.











