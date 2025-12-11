# 🦀 ratcrate-cli: Ratatui Ecosystem Crate Discoverer

[![CI](https://github.com/ratcrate/ratcrate-cli/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/ratcrate/ratcrate-cli/actions/workflows/ci.yml)
[![Release](https://github.com/ratcrate/ratcrate-cli/actions/workflows/all-release.yml/badge.svg)](https://github.com/ratcrate/ratcrate-cli/actions/workflows/all-release.yml)
[![Documentation](https://docs.rs/doc2quarto/badge.svg)](https://docs.rs/doc2quarto)[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE) 
[![Crates.io](https://img.shields.io/crates/v/ratcrate-cli.svg)](https://crates.io/crates/ratcrate-cli)
[![Downloads](https://img.shields.io/crates/d/ratcrate-cli.svg)](https://crates.io/crates/ratcrate-cli)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)



# 🛠️ Overview

**`ratcrate-cli`** is a fast, terminal-native command-line tool built in **Rust** for discovering, searching, and getting information about crates within the Ratatui ecosystem. It uses a smart caching strategy to provide near-instant results directly in your terminal. The project follows a standard Rust CLI structure, ensuring clear separation of responsibilities:


![ratcrate-cli-demo-small](https://github.com/user-attachments/assets/70f71f30-7076-4c93-aae9-17787475a550)





> [!TIP] 
> ### For more information, read ["The book"](https://qubitai.in/book/rat-cli/)

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
2.  **fzf (Optional):** Required for the interactive mode (`-f`).

### Install from Crates.io 

Once the package is published, you can install it globally:

```bash
$> cargo install ratcrate-cli
```

## Using Homebrew

```bash
$> brew install rvbug/tap/ratcrate-cli
```
> Homebrew cn be installed via `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`


## Build from Source
Clone the repository and build the project using Cargo:

```bash
$> git clone [https://github.com/ratcrate/ratcrate-cli.git](https://github.com/ratcrate/ratcrate-cli.git)
$> cd ratcrate-cli
$> cargo install --path 
```


---

## ѫ Usage

Run `ratcrate-cli --help` for a full list of options.

| Option | Description |
| --- | --- |
| `-h, --help` | Display help information. |
| `-q, --query <QUERY>`  | Search term to filter packages
| `-c, --cache-info`   |  Show cache info |
| `-l, --limit <LIMIT>` |  Limit number of results to show [default: 10] |
| `-r, --refresh`     |   Force re-download of remote JSON to cache |
| `-f, --fzf` | Enable interactive mode with fzf. |
| `-t, --total` | Display total number of crates. |
| `-V, --version` | Display the version number. |
| `-h, --help` | Display help information. |
| `--table `   | Table view (requires compiling with --features table)


### Total View 

```bash
$ ratcrate-cli -t

────────────────────────────────────────────────────────
✓ Loaded from cache
Total Crates Overview
────────────────────────────────────────────────────────
Total Crates: 1686
  Core Libraries: 2
  Community Packages: 1684
────────────────────────────────────────────────────────
Data generated at: 2025-12-04T06:10:38.648089+00:00
```

### Table View
```bash
$> cargo run --features=table -- --table

ratcrate (discover ratatui crates)
────────────────────────────────────────────────────────
✓ Loaded from cache
+-----------------+---------+---------------------------------------------------------------+-----------+-----------------------------------------------------+
| name            | version | description                                                   | downloads | repo                                                |
+-----------------+---------+---------------------------------------------------------------+-----------+-----------------------------------------------------+
| ratatui-macros  | v0.6.0  | Macros for Ratatui                                            | 252104    | https://github.com/ratatui/ratatui                  |
+-----------------+---------+---------------------------------------------------------------+-----------+-----------------------------------------------------+
| ratatui-widgets | v0.2.2  | A collection of Ratatui widgets for building terminal user i… | 123671    | https://github.com/ratatui/ratatui                  |
+-----------------+---------+---------------------------------------------------------------+-----------+-----------------------------------------------------+
| CLI-Rhythm      | v1.0.1  | A simple and functional CLI music player.                     | 5092      | https://github.com/Arklingh/CLI-Rhythm              |
+-----------------+---------+---------------------------------------------------------------+-----------+-----------------------------------------------------+
```
### Query

```bash
$> cargo run -- -q parser

ratcrate (discover ratatui crates)
────────────────────────────────────────────────────────
✓ Loaded from cache
Results: 8 / 8
Query: parser
────────────────────────────────────────────────────────────
 1. codex_usage v0.1.1
  Codex and Claude Code telemetry/usage parser, aggregate JSONL events
  into CodeAnalysis results
  Downloads: 262   Repo: https://github.com/Mai0313/codex_usage
────────────────────────────────────────────────────────────
 2. keybinds v0.2.0
  Platform&Framework-agnostic key binding (keyboard shortcut) dispatcher,
  parser, and generator written in Safe Rust.
  Downloads: 9140   Repo: https://github.com/rhysd/keybinds-rs
```

## Roadmap

- [ ] "Try" mode
- [ ] Debian Support

## 🤝 Contributing
Contributions are welcome! If you have suggestions or find a bug, please open an issue or submit a pull request.

## 📝 License
This project is licensed under the MIT License. See the LICENSE file for details.

---

<br>

<h1 align="center">Support</h1>

<h3 align="center">
  If you find my projects helpful or want to support my research and open-source work:
</h3>
<p align="center">
  <a href="https://ko-fi.com/rvbugged">
    <img src="https://ko-fi.com/img/githubbutton_sm.svg" alt="Ko-fi" height="40">
  </a>
</p>













