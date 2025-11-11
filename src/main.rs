use anyhow::Result;
use clap::Parser;
use colored::*;

mod cache;
mod types;

use cache::{get_data, get_cache_dir, get_cache_file};
// use types::CratesData;

#[derive(Parser)]
#[command(name = "ratcrate")]
#[command(version, about = "Discover crates in the Ratatui ecosystem", long_about = None)]
struct Cli {
    /// Search term to filter packages
    query: Option<String>,
    
    /// Force refresh data from GitHub
    #[arg(short, long)]
    refresh: bool,
    
    /// Show only core libraries
    #[arg(short, long)]
    core: bool,
    
    /// Show cache information
    #[arg(short='i', long)]
    cache_info: bool,
    
    /// Number of results to show (default: 20)
    #[arg(short, long, default_value = "20")]
    limit: usize,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    
    // Print banner
    print_banner();
    
    // Handle cache-info command
    if args.cache_info {
        return show_cache_info();
    }
    
    // Get data (from cache or download)
    let data = get_data(args.refresh)?;
    
    // Print metadata
    println!("{}", format!("📦 Total packages: {}", data.metadata.total_crates).cyan());
    println!("{}", format!("⭐ Core libraries: {}", data.metadata.core_libraries).yellow());
    println!("{}", format!("🌍 Community: {}", data.metadata.community_packages).green());
    println!("{}", format!("🕒 Last updated: {}", data.metadata.generated_at).dimmed());
    println!();
    
    // Filter crates
    let mut crates = data.crates;
    
    // Filter by core if requested
    if args.core {
        crates.retain(|c| c.is_core_library);
    }
    
    // Filter by search query
    if let Some(query) = &args.query {
        let query_lower = query.to_lowercase();
        crates.retain(|c| {
            c.name.to_lowercase().contains(&query_lower)
                || c.description.to_lowercase().contains(&query_lower)
        });
    }
    
    // Show results
    if crates.is_empty() {
        println!("{}", "No packages found matching your criteria.".red());
        return Ok(());
    }
    
    let total = crates.len();
    let showing = args.limit.min(total);
    
    println!("{}", format!("Showing {} of {} packages:", showing, total).bold());
    println!();
    
    for (i, crate_pkg) in crates.iter().take(args.limit).enumerate() {
        let icon = if crate_pkg.is_core_library {
            "⭐".yellow()
        } else {
            "📦".normal()
        };
        
        let number = format!("{:3}.", i + 1).dimmed();
        let name = crate_pkg.name.bright_cyan().bold();
        let version = format!("v{}", crate_pkg.version).dimmed();
        
        println!("{} {} {} {}", number, icon, name, version);
        println!("    {}", crate_pkg.description.dimmed());
        println!(
            "    {} {} • {} {}",
            "↓".green(),
            format!("{:>8}", format_number(crate_pkg.downloads)).green(),
            "📈".blue(),
            format!("{:>6}", format_number(crate_pkg.recent_downloads)).blue()
        );
        
        if let Some(repo) = &crate_pkg.repository {
            println!("    {} {}", "🔗".dimmed(), repo.dimmed());
        }
        
        println!("    {} {}", "📦".dimmed(), format!("cargo add {}", crate_pkg.name).bright_black());
        println!();
    }
    
    if total > showing {
        println!(
            "{}",
            format!("... and {} more packages", total - showing).dimmed()
        );
    }
    
    Ok(())
}

fn print_banner() {
    println!("{}", "
╔═══════════════════════════════════════════════════════╗
║                                                       ║
║   🐀 RATCRATE                                         ║
║   Ratatui Ecosystem Explorer                          ║
║                                                       ║
╚═══════════════════════════════════════════════════════╝
    ".bright_cyan());
}

fn show_cache_info() -> Result<()> {
    println!("{}", "Cache Information:".bold().cyan());
    println!();
    
    let cache_dir = get_cache_dir()?;
    let cache_file = get_cache_file()?;
    
    println!("  {} {}", "Cache directory:".bold(), cache_dir.display());
    println!("  {} {}", "Cache file:".bold(), cache_file.display());
    
    if cache_file.exists() {
        let metadata = std::fs::metadata(&cache_file)?;
        let size = metadata.len();
        let modified = metadata.modified()?;
        let age = std::time::SystemTime::now()
            .duration_since(modified)?
            .as_secs();
        
        println!("  {} {}", "File size:".bold(), format_bytes(size).green());
        println!(
            "  {} {}",
            "Last updated:".bold(),
            format_duration(age).yellow()
        );
        
        if age > 7 * 24 * 3600 {
            println!(
                "\n  {} Cache is older than 7 days, consider refreshing with --refresh",
                "⚠️".yellow()
            );
        }
    } else {
        println!("\n  {} Cache file doesn't exist. Run without --cache-info to download.", "ℹ️".blue());
    }
    
    Ok(())
}

fn format_number(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes >= 1_000_000 {
        format!("{:.2} MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{:.2} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{} bytes", bytes)
    }
}

fn format_duration(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    
    if days > 0 {
        format!("{} day(s) ago", days)
    } else if hours > 0 {
        format!("{} hour(s) ago", hours)
    } else if minutes > 0 {
        format!("{} minute(s) ago", minutes)
    } else {
        "just now".to_string()
    }
}
