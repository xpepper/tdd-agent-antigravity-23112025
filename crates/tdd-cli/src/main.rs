mod config;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use config::Config;
use dotenv::dotenv;
use std::env;

use tdd_agents::LlmAgent;
use tdd_core::{Orchestrator, Role, TddOrchestrator, Vcs};
use tdd_exec::{GitVcs, ProcessRunner};
use tdd_llm::LlmClient;
use tokio::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize repo and scaffolding
    Init,
    /// Run N full TDD steps
    Run {
        #[arg(long, default_value_t = 20)]
        steps: u32,
    },
    /// Run a single agent step (debug)
    Step,
    /// Show current agent, step counter, last commit summary
    Status,
    /// Verify tools, versions, environment
    Doctor,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init().await?,
        Commands::Run { steps } => run(steps).await?,
        Commands::Step => run(1).await?, // Step runs 1 step? Or just one agent turn? Orchestrator::next() is one turn.
        Commands::Status => status().await?,
        Commands::Doctor => doctor().await?,
    }

    Ok(())
}

async fn init() -> Result<()> {
    let cwd = env::current_dir()?;
    println!("Initializing TDD workspace in {:?}", cwd);

    // Create tdd.yaml
    if !cwd.join("tdd.yaml").exists() {
        fs::write(cwd.join("tdd.yaml"), Config::default_yaml()).await?;
        println!("Created tdd.yaml");
    }

    // Create kata.md
    if !cwd.join("kata.md").exists() {
        fs::write(
            cwd.join("kata.md"),
            "# Kata Description\n\nDescribe your kata here.",
        )
        .await?;
        println!("Created kata.md");
    }

    // Create .gitignore
    if !cwd.join(".gitignore").exists() {
        fs::write(cwd.join(".gitignore"), "/target\n/.tdd\n.env\n").await?;
        println!("Created .gitignore");
    }

    // Initialize git
    let vcs = GitVcs::new(&cwd);
    vcs.init_if_needed()?;
    println!("Initialized git repo");

    // Create Cargo.toml if not exists (scaffold rust project)
    if !cwd.join("Cargo.toml").exists() {
        let cargo_toml = r#"[package]
name = "tdd-kata"
version = "0.1.0"
edition = "2021"

[dependencies]

[dev-dependencies]
"#;
        fs::write(cwd.join("Cargo.toml"), cargo_toml).await?;
        println!("Created Cargo.toml");

        fs::create_dir_all(cwd.join("src")).await?;
        fs::write(cwd.join("src/lib.rs"), "").await?;
        println!("Created src/lib.rs");
    }

    Ok(())
}

async fn run(steps: u32) -> Result<()> {
    let cwd = env::current_dir()?;
    let config = Config::load(cwd.join("tdd.yaml")).await?;

    let api_key =
        env::var(&config.llm.api_key_env).context(format!("{} not set", config.llm.api_key_env))?;

    let llm_client = LlmClient::new(config.llm.base_url.clone(), api_key);

    let tester = Box::new(LlmAgent::new(
        Role::Tester,
        llm_client.clone(),
        config.roles.get("tester").unwrap().model.clone(),
        config.roles.get("tester").unwrap().temperature,
        cwd.clone(),
    ));

    let implementor = Box::new(LlmAgent::new(
        Role::Implementor,
        llm_client.clone(),
        config.roles.get("implementor").unwrap().model.clone(),
        config.roles.get("implementor").unwrap().temperature,
        cwd.clone(),
    ));

    let refactorer = Box::new(LlmAgent::new(
        Role::Refactorer,
        llm_client.clone(),
        config.roles.get("refactorer").unwrap().model.clone(),
        config.roles.get("refactorer").unwrap().temperature,
        cwd.clone(),
    ));

    let runner = Box::new(ProcessRunner::new(
        config.ci.fmt_cmd,
        config.ci.check_cmd,
        config.ci.test_cmd,
    ));

    let vcs = Box::new(GitVcs::new(&cwd));

    let kata_description = fs::read_to_string(cwd.join(&config.kata_description))
        .await
        .context("Failed to read kata description")?;

    let mut orchestrator = TddOrchestrator::new(
        tester,
        implementor,
        refactorer,
        runner,
        vcs,
        kata_description,
        config.max_attempts_per_agent,
        cwd,
    );

    for i in 0..steps {
        println!("Step {}/{}", i + 1, steps);
        orchestrator.next().await?;
    }

    Ok(())
}

async fn status() -> Result<()> {
    // Read .tdd/logs to find latest status
    // Or just check git log?
    // The requirements say: "Provide `tdd-cli status` to print the latest step summary and failing diagnostics if any."
    // I'll just print a placeholder for now or read the last log file.
    println!("Status command not fully implemented yet.");
    Ok(())
}

async fn doctor() -> Result<()> {
    println!("Checking environment...");

    // Check cargo
    let cargo = tokio::process::Command::new("cargo")
        .arg("--version")
        .output()
        .await;
    match cargo {
        Ok(out) => println!("Cargo: {}", String::from_utf8_lossy(&out.stdout).trim()),
        Err(_) => println!("Cargo: NOT FOUND"),
    }

    // Check git
    let git = tokio::process::Command::new("git")
        .arg("--version")
        .output()
        .await;
    match git {
        Ok(out) => println!("Git: {}", String::from_utf8_lossy(&out.stdout).trim()),
        Err(_) => println!("Git: NOT FOUND"),
    }

    Ok(())
}
