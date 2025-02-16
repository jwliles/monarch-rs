use anyhow::{Context, Result};
use futures::stream::{self, StreamExt};
use std::path::{Path, PathBuf};
use tokio::{fs, process::Command};
use tracing::{debug, error, info, warn};

// Changed from derive macro to enum implementation
#[derive(Debug, Clone)]
enum Operation {
    Clone,
    Pull,
    Push,
    Status,
}

// Removed derive macros and implemented struct directly
#[derive(Clone)] // Added Clone trait here
struct Args {
    operation: Operation,
    source_dir: PathBuf,
    target_dir: Option<PathBuf>,
    reverse: bool,
    dry_run: bool,
    verbose: bool,
}

impl Args {
    fn from_args() -> Result<Self> {
        let mut args = pico_args::Arguments::from_env();

        Ok(Args {
            operation: match args.value_from_str::<&str, String>("-o")? {
                op if op == "clone" => Operation::Clone,
                op if op == "pull" => Operation::Pull,
                op if op == "push" => Operation::Push,
                op if op == "status" => Operation::Status,
                op => anyhow::bail!("Invalid operation: {}", op),
            },
            source_dir: PathBuf::from(args.value_from_str::<&str, String>("-s")?),
            target_dir: args
                .opt_value_from_str::<&str, String>("-t")?
                .map(PathBuf::from),
            reverse: args.contains("--reverse"),
            dry_run: args.contains("--dry-run"),
            verbose: args.contains("--verbose"),
        })
    }
}

#[derive(Default)]
struct OperationResults {
    succeeded: Vec<PathBuf>,
    failed: Vec<PathBuf>,
}

struct GitReposManager {
    args: Args,
}

// Increase the default concurrency level
const DEFAULT_CONCURRENCY_LEVEL: usize = 16;

// Add this import
use async_recursion::async_recursion;

impl GitReposManager {
    fn new(args: Args) -> Self {
        Self { args }
    }

    async fn run(&self) -> Result<()> {
        self.validate_options().await?;
        self.process_repositories().await
    }

    async fn validate_options(&self) -> Result<()> {
        if !self.args.source_dir.is_dir() {
            anyhow::bail!(
                "Source directory '{}' does not exist",
                self.args.source_dir.display()
            );
        }

        if matches!(self.args.operation, Operation::Clone)
            && !self.args.reverse
            && self.args.target_dir.is_none()
        {
            anyhow::bail!(
                "Target directory required for clone operation unless --reverse is specified"
            );
        }

        Command::new("git")
            .arg("--version")
            .output()
            .await
            .context("git command not found. Please install Git and try again")?;

        Ok(())
    }

    // Use the async_recursion macro to handle recursive async functions
    #[async_recursion]
    async fn find_git_repositories(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut repos = Vec::new();
        let mut entries = fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.join(".git").is_dir() {
                repos.push(path);
            } else if path.is_dir() {
                let sub_repos = self.find_git_repositories(&path).await?;
                repos.extend(sub_repos);
            }
        }

        Ok(repos)
    }

    async fn process_repositories(&self) -> Result<()> {
        match self.args.operation {
            Operation::Clone => self.handle_clone().await,
            Operation::Pull | Operation::Push | Operation::Status => {
                self.handle_git_operation().await
            }
        }
    }

    async fn handle_clone(&self) -> Result<()> {
        let source = &self.args.source_dir;
        let target = if self.args.reverse {
            PathBuf::from(".")
        } else {
            self.args
                .target_dir
                .clone()
                .expect("Target directory must be specified")
        };

        if !self.args.dry_run {
            fs::create_dir_all(&target).await?;
        }

        let repos = self.find_git_repositories(source).await?;
        if repos.is_empty() {
            warn!("No Git repositories found in {}", source.display());
            return Ok(());
        }

        let mut results = OperationResults::default();

        let concurrency_level = DEFAULT_CONCURRENCY_LEVEL; // Or make this configurable

        let mut tasks = stream::iter(repos)
            .map(|repo_dir| {
                let target = target.clone();
                let source = source.clone();
                let args = self.args.clone();
                async move {
                    let rel_path = repo_dir.strip_prefix(&source).unwrap();
                    let target_repo_dir = target.join(rel_path);

                    if args.dry_run {
                        debug!(
                            "[DRY RUN] Would clone {} to {}",
                            repo_dir.display(),
                            target_repo_dir.display()
                        );
                        return Ok(repo_dir);
                    }

                    if target_repo_dir.join(".git").is_dir() {
                        info!(
                            "Repository already exists at {}, skipping",
                            target_repo_dir.display()
                        );
                        return Ok(repo_dir);
                    }

                    fs::create_dir_all(target_repo_dir.parent().unwrap()).await?;
                    info!(
                        "Cloning {} to {}",
                        repo_dir.display(),
                        target_repo_dir.display()
                    );

                    let output = Command::new("git")
                        .arg("clone")
                        .arg("--shared") // Use the --shared option
                        .arg(&repo_dir)
                        .arg(&target_repo_dir)
                        .output()
                        .await?;

                    if output.status.success() {
                        info!("Successfully cloned {}", repo_dir.display());
                        Ok(repo_dir)
                    } else {
                        error!(
                            "Failed to clone {}: {}",
                            repo_dir.display(),
                            String::from_utf8_lossy(&output.stderr)
                        );
                        Err(anyhow::anyhow!("Clone failed"))
                    }
                }
            })
            .buffer_unordered(concurrency_level);

        while let Some(result) = tasks.next().await {
            match result {
                Ok(repo) => results.succeeded.push(repo),
                Err(_) => results.failed.push(PathBuf::from("failed")),
            }
        }

        self.print_results(&results).await;
        Ok(())
    }

    async fn handle_git_operation(&self) -> Result<()> {
        let repos = self.find_git_repositories(&self.args.source_dir).await?;
        if repos.is_empty() {
            warn!(
                "No Git repositories found in {}",
                self.args.source_dir.display()
            );
            return Ok(());
        }

        let mut results = OperationResults::default();

        let concurrency_level = DEFAULT_CONCURRENCY_LEVEL;

        let mut tasks = stream::iter(repos)
            .map(|repo_dir| {
                let operation = self.args.operation.clone();
                let args = self.args.clone();
                async move {
                    match operation {
                        Operation::Status => {
                            GitReposManager::show_repo_status(&args, &repo_dir).await
                        }
                        op => GitReposManager::perform_git_operation(&args, &repo_dir, &op).await,
                    }
                    .map(|_| repo_dir)
                }
            })
            .buffer_unordered(concurrency_level);

        while let Some(result) = tasks.next().await {
            match result {
                Ok(repo) => results.succeeded.push(repo),
                Err(_) => results.failed.push(PathBuf::from("failed")),
            }
        }

        self.print_results(&results).await;
        Ok(())
    }

    async fn show_repo_status(args: &Args, repo_dir: &Path) -> Result<()> {
        if args.dry_run {
            debug!("[DRY RUN] Would show status of {}", repo_dir.display());
            return Ok(());
        }

        let status_output = Command::new("git")
            .current_dir(repo_dir)
            .args(["status", "--porcelain"])
            .output()
            .await?;

        let branch_output = Command::new("git")
            .current_dir(repo_dir)
            .args(["branch", "--show-current"])
            .output()
            .await?;

        let status = String::from_utf8_lossy(&status_output.stdout);
        let branch = String::from_utf8_lossy(&branch_output.stdout)
            .trim()
            .to_string();

        if !status.is_empty() || args.verbose {
            println!("\n{} [{}]:", repo_dir.display(), branch);
            if status.is_empty() {
                println!("  Clean");
            } else {
                for line in status.lines() {
                    println!("  {}", line);
                }
            }
        }

        Ok(())
    }

    async fn perform_git_operation(
        args: &Args,
        repo_dir: &Path,
        operation: &Operation,
    ) -> Result<()> {
        let op_str = match operation {
            Operation::Pull => "pull",
            Operation::Push => "push",
            _ => unreachable!(),
        };

        if args.dry_run {
            debug!(
                "[DRY RUN] Would perform git {} in {}",
                op_str,
                repo_dir.display()
            );
            return Ok(());
        }

        let output = Command::new("git")
            .current_dir(repo_dir)
            .arg(op_str)
            .output()
            .await?;

        if output.status.success() {
            debug!(
                "Successfully performed git {} in {}",
                op_str,
                repo_dir.display()
            );
            Ok(())
        } else {
            error!(
                "Failed to perform git {} in {}: {}",
                op_str,
                repo_dir.display(),
                String::from_utf8_lossy(&output.stderr)
            );
            Err(anyhow::anyhow!("Git operation failed"))
        }
    }

    async fn print_results(&self, results: &OperationResults) {
        if results.succeeded.is_empty() && results.failed.is_empty() {
            return;
        }

        if !matches!(self.args.operation, Operation::Status) || self.args.verbose {
            info!("\nOperation summary:");
            info!("  Succeeded: {}", results.succeeded.len());
            info!("  Failed: {}", results.failed.len());

            if self.args.verbose && !results.failed.is_empty() {
                info!("\nFailed repositories:");
                for repo in &results.failed {
                    info!("  {}", repo.display());
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::from_args()?;
    GitReposManager::new(args).run().await
}
