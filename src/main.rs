use anyhow::Result;
use clap::Parser;
use colored::*;
use std::process::Command;

mod cache;
mod types;

use cache::{get_cache_dir, get_cache_file, get_data};
use types::CratePackage;
use types::Metadata;

/// ratcrate - discover crates in the Ratatui ecosystem
#[derive(Parser)]
#[command(name = "ratcrate")]
#[command(version, about = "Discover crates in the Ratatui ecosystem", long_about = None)]
struct Cli {
    /// Search term to filter packages
    #[arg(short, long)]
    query: Option<String>,

    /// Show cache info
    #[arg(short = 'c', long)]
    cache_info: bool,

    /// Limit number of results to show
    #[arg(short, long, default_value_t = 10usize)]
    limit: usize,

    /// Force re-download of remote JSON to cache
    #[arg(short = 'r', long)]
    refresh: bool,

    /// Use table view (requires compiling with --features table)
    #[arg(long)]
    table: bool,

    /// Use fzf to interactively pick a crate (requires fzf installed)
    #[arg(short = 'f', long)]
    fzf: bool,

    /// use t for showing the total crates
    #[arg(short = 't', long)]
    total: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // print banner
    print_banner();

    if args.cache_info {
        return show_cache_info();
    }

    // fetch data (from cache or remote); pass `args.refresh` to force download if provided
    let crates_data = get_data(args.refresh)?;

    // If fzf flag is provided, launch interactive selector
    if args.fzf {
        if launch_fzf(&crates_data.crates, args.limit, args.query.as_ref())? {
            // if user selected an item and we displayed it, exit
            return Ok(());
        } else {
            // fall through to normal listing if fzf wasn't used or no selection
        }
    }

    if args.total {
        // Display the total crates and further information
        display_total_crates(&crates_data.metadata);
        return Ok(());
    }

    // If table requested and compiled with feature, use table view
    if args.table {
        display_table_wrapper(&crates_data.crates, args.query.as_ref(), args.limit);
    } else {
        // normal pretty listing
        display_results(&crates_data.crates, args.query.as_ref(), args.limit);
    }

    Ok(())
}

fn print_banner() {
    println!(
        "{} {}",
        "ratcrate".bright_cyan().bold(),
        "(discover ratatui crates)".dimmed()
    );
    println!(
        "{}",
        "────────────────────────────────────────────────────────".dimmed()
    );
}

/// Show some basic cache info (delegates to your existing helpers)
fn show_cache_info() -> Result<()> {
    let cache_dir = get_cache_dir()?;
    let cache_file = get_cache_file()?;
    println!("Cache dir: {}", cache_dir.display());
    println!("Cache file: {}", cache_file.display());
    Ok(())
}

/// Main pretty-print function
fn display_results(crates: &[CratePackage], query: Option<&String>, limit: usize) {
    // Lowercased query for case-insensitive search
    let qlower = query.map(|s| s.to_lowercase());

    // Filter first, then collect and take up to `limit`
    let filtered: Vec<&CratePackage> = crates
        .iter()
        .filter(|c| {
            if let Some(q) = &qlower {
                c.name.to_lowercase().contains(q) || c.description.to_lowercase().contains(q)
            } else {
                true
            }
        })
        .collect();

    let total = filtered.len();
    let shown = std::cmp::min(limit, total);

    // Header
    println!(
        "{} {} {} {}",
        "Results:".bright_yellow().bold(),
        shown.to_string().bright_green().bold(),
        "/".dimmed(),
        total.to_string().dimmed()
    );
    if let Some(q) = query {
        println!("{} {}", "Query:".bright_blue(), q.bright_white());
    }
    println!("{}", "─".repeat(60).dimmed());

    // Iterate only shown items
    for (idx, krate) in filtered.into_iter().take(limit).enumerate() {
        // Line 1: number, name (bold green), version (dim)
        println!(
            "{} {} {}",
            format!("{:>2}.", idx + 1).bright_blue(),
            krate.name.bright_green().bold(),
            format!("v{}", krate.version).dimmed()
        );

        // Line 2: description (wrap if long — simple approach)
        let desc = if krate.description.trim().is_empty() {
            "No description".to_string()
        } else {
            krate.description.clone()
        };
        println!("  {}", wrap_and_dim(&desc, 72));

        // Line 3: small metadata row
        let downloads = format!("{}", krate.downloads);
        let repo = krate.repository.as_deref().unwrap_or("No repo");
        println!(
            "  {} {}   {} {}",
            "Downloads:".bright_yellow(),
            downloads.bright_magenta(),
            "Repo:".bright_yellow(),
            repo.bright_blue()
        );

        // Divider
        println!("{}", "─".repeat(60).dimmed());
    }

    println!(
        "{}",
        format!("End of list (showing up to {})", shown)
            .bright_black()
            .italic()
    );
}

/// naive word-wrap and dim the result
fn wrap_and_dim(s: &str, width: usize) -> String {
    let mut out = String::new();
    let mut line_len = 0usize;
    for word in s.split_whitespace() {
        let wl = word.len();
        if line_len + wl + 1 > width && line_len > 0 {
            out.push('\n');
            out.push_str("  ");
            out.push_str(word);
            line_len = 2 + wl;
        } else {
            if line_len > 0 {
                out.push(' ');
                line_len += 1;
            }
            out.push_str(word);
            line_len += wl;
        }
    }
    out.dimmed().to_string()
}

/// Launch fzf with a list of crate lines. Returns Ok(true) if a selection was made and displayed.
fn launch_fzf(crates: &[CratePackage], limit: usize, query: Option<&String>) -> Result<bool> {
    // Build lines: "name — description (downloads)"
    let qlower = query.map(|s| s.to_lowercase());
    let items: Vec<String> = crates
        .iter()
        .filter(|c| {
            if let Some(q) = &qlower {
                c.name.to_lowercase().contains(q) || c.description.to_lowercase().contains(q)
            } else {
                true
            }
        })
        .take(limit)
        .map(|c| {
            // ANSI colored preview in fzf is not recommended; keep plain lines but include enough info
            format!(
                "{} — {} ({})",
                c.name,
                truncate(&c.description, 80),
                c.downloads
            )
        })
        .collect();

    if items.is_empty() {
        println!("{}", "No items to show in fzf".yellow());
        return Ok(false);
    }

    // Spawn fzf
    let mut child = match Command::new("fzf")
        .arg("--ansi")
        .arg("--prompt")
        .arg("Select crate> ")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "{}",
                format!("Failed to launch fzf: {}. Is fzf installed?", e).red()
            );
            return Ok(false);
        }
    };

    // write items to stdin
    {
        use std::io::Write;
        let stdin = child.stdin.as_mut().expect("Failed to open fzf stdin");
        for line in &items {
            writeln!(stdin, "{}", line)?;
        }
    }

    // read selection
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Ok(false);
    }
    let selected = String::from_utf8_lossy(&output.stdout);
    let selected = selected.trim();
    if selected.is_empty() {
        return Ok(false);
    }

    // Extract name (before ' — ')
    let name = selected.split('—').next().unwrap_or(selected).trim();

    // Find crate and display full details
    if let Some(k) = crates.iter().find(|c| c.name == name) {
        display_single_crate(k);
        Ok(true)
    } else {
        println!("{}", "Selection not found in list".yellow());
        Ok(false)
    }
}

/// Show the total crates available in the ratcrate.json
fn display_total_crates(metadata: &Metadata) {
    println!("{}", "Total Crates Overview".bright_cyan().bold());
    println!(
        "{}",
        "────────────────────────────────────────────────────────".dimmed()
    );

    println!(
        "{} {}",
        "Total Crates:".bright_yellow(),
        metadata.total_crates.to_string().bright_green().bold()
    );

    println!(
        "  {} {}",
        "Core Libraries:".bright_blue(),
        metadata.core_libraries.to_string().bright_magenta()
    );

    println!(
        "  {} {}",
        "Community Packages:".bright_blue(),
        metadata.community_packages.to_string().bright_magenta()
    );

    println!(
        "{}",
        "────────────────────────────────────────────────────────".dimmed()
    );
    println!(
        "{}",
        format!("Data generated at: {}", metadata.generated_at).dimmed()
    );
}

/// Show an individual crate's details (used by fzf selection)
fn display_single_crate(k: &CratePackage) {
    println!(
        "{}",
        "────────────────────────────────────────────────────────".dimmed()
    );
    println!(
        "{} {}",
        k.name.bright_green().bold(),
        format!("v{}", k.version).dimmed()
    );
    println!("{}\n", k.description.dimmed());
    println!(
        "{} {}",
        "Downloads:".bright_yellow(),
        k.downloads.to_string().bright_magenta()
    );
    println!(
        "{} {}",
        "Repository:".bright_yellow(),
        k.repository.as_deref().unwrap_or("No repo").bright_blue()
    );
    println!(
        "{} {}",
        "Created:".bright_yellow(),
        k.created_at.bright_black()
    );
    println!(
        "{} {}",
        "Updated:".bright_yellow(),
        k.updated_at.bright_black()
    );
    println!("{} {}", "ID:".bright_yellow(), k.id.bright_black());
    println!(
        "{}",
        "────────────────────────────────────────────────────────".dimmed()
    );
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        let mut t = s[..max].to_string();
        t.push("…");
        t
    }
}

//
// Table view support (optional).
// This module is only compiled when the crate is built with --features=table
// Add this to Cargo.toml:
// [dependencies]
// tabled = { version = "0.6", optional = true }
//
// [features]
// table = ["tabled"]
//
#[cfg(feature = "table")]
mod table_display {
    use super::CratePackage;
    use colored::*;
    use tabled::{Table, Tabled};

    #[derive(Tabled)]
    struct Row {
        name: String,
        version: String,
        description: String,
        downloads: String,
        repo: String,
    }

    pub fn display_table(crates: &[CratePackage], query: Option<&String>, limit: usize) {
        let qlower = query.map(|s| s.to_lowercase());
        let rows: Vec<Row> = crates
            .iter()
            .filter(|c| {
                if let Some(q) = &qlower {
                    c.name.to_lowercase().contains(q) || c.description.to_lowercase().contains(q)
                } else {
                    true
                }
            })
            .take(limit)
            .map(|c| Row {
                name: c.name.clone(),
                version: format!("v{}", c.version),
                description: truncate(&c.description, 60),
                downloads: c.downloads.to_string(),
                repo: c.repository.clone().unwrap_or_else(|| "-".to_string()),
            })
            .collect();

        let table = Table::new(rows).to_string();
        println!("{}", table);
    }

    fn truncate(s: &str, max: usize) -> String {
        if s.len() <= max {
            s.to_string()
        } else {
            let mut t = s[..max].to_string();
            t.push_str("…");
            t
        }
    }
}

#[cfg(not(feature = "table"))]
mod table_display {
    use super::CratePackage;
    pub fn display_table(_crates: &[CratePackage], _query: Option<&String>, _limit: usize) {
        eprintln!("Table feature not enabled. Build with --features=table to enable.");
    }
}

/// Wrapper to call table module (keeps call-site simple)
fn display_table_wrapper(crates: &[CratePackage], query: Option<&String>, limit: usize) {
    table_display::display_table(crates, query, limit);
}
