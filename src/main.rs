mod analyzer;
mod generator;
mod config;

use clap::Parser;
use colored::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "git-commit-gen")]
#[command(about = "Intelligent git commit message generator")]
struct Args {
    /// Path to the git repository
    #[arg(short, long, default_value = ".")]
    repo: PathBuf,

    /// Generate a commit message for staged changes
    #[arg(short, long)]
    stage: bool,

    /// Show suggested commit messages (don't create commit)
    #[arg(short, long)]
    suggest: bool,

    /// Conventional commit type filter
    #[arg(short, long)]
    r#type: Option<String>,

    /// Include breaking change indicator
    #[arg(short, long)]
    breaking: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("{}: {}", "Error".red().bold(), e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🔍 Analyzing changes...".bright_cyan());

    let repo_path = args.repo.canonicalize()?;
    
    // Initialize analyzer
    let analyzer = analyzer::DiffAnalyzer::new(&repo_path)?;
    
    // Analyze the diff
    let analysis = if args.stage {
        analyzer.analyze_staged()?
    } else {
        analyzer.analyze_unstaged()?
    };

    if args.verbose {
        println!("\n{}", "📊 Analysis Summary:".bright_yellow());
        println!("  Files changed: {}", analysis.files_changed);
        println!("  Lines added: {}", analysis.lines_added);
        println!("  Lines removed: {}", analysis.lines_removed);
        println!("  Primary type: {}", analysis.primary_type);
    }

    // Generate commit message
    let generator = generator::CommitGenerator::new();
    let suggestions = if let Some(ref r#type) = args.r#type {
        vec![generator.generate_with_type(&analysis, r#type, args.breaking)]
    } else {
        generator.generate_suggestions(&analysis, args.breaking)
    };

    println!("\n{}", "💡 Suggested Commit Messages:".bright_green().bold());
    for (i, suggestion) in suggestions.iter().enumerate() {
        println!("\n  {}. {}", (i + 1).to_string().bold(), suggestion);
    }

    if args.suggest {
        return Ok(());
    }

    // Create commit if not just suggesting
    if !suggestions.is_empty() {
        let message = suggestions[0].clone();
        analyzer.commit(&message)?;
        println!("\n{}", "✅ Commit created successfully!".bright_green());
        println!("   {}", message);
    }

    Ok(())
}
