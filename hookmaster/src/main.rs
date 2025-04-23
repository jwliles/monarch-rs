use clap::{Arg, Command};
use git2::{Repository, RepositoryInitOptions};
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::process::exit;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

fn main() {
    let matches = Command::new("hookmaster")
        .version("1.0")
        .author("Your Name")
        .about("Clones Git repositories from local or remote sources")
        .arg(
            Arg::new("target")
                .help("Sets the target directory")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("from-remote")
                .long("from-remote")
                .help("Clone from remote origin URLs"),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .help("Perform a dry run without cloning"),
        )
        .get_matches();

    let target_dir = matches.get_one::<String>("target").unwrap();
    let from_remote = matches.contains_id("from-remote");
    let dry_run = matches.contains_id("dry-run");

    // Create the target directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(target_dir) {
        eprintln!("Error creating target directory: {}", e);
        exit(1);
    }

    let target_path = Path::new(target_dir);
    let repos_found = Arc::new(AtomicBool::new(false));
    let success_count = Arc::new(AtomicUsize::new(0));
    let failure_count = Arc::new(AtomicUsize::new(0));

    // Collect entries in the current directory
    let entries: Vec<_> = fs::read_dir(".")
        .expect("Failed to read current directory")
        .filter_map(Result::ok)
        .collect();

    // Process entries in parallel
    entries.par_iter().for_each(|entry| {
        let path = entry.path();
        if path.is_dir() {
            let git_dir = path.join(".git");
            if git_dir.exists() && git_dir.is_dir() {
                repos_found.store(true, Ordering::SeqCst);
                let project_name = path.file_name().unwrap().to_string_lossy();

                println!("Processing {}", project_name);

                if dry_run {
                    println!(
                        "Would clone {} to {}",
                        project_name,
                        target_path.join(&*project_name).display()
                    );
                    success_count.fetch_add(1, Ordering::SeqCst);
                    return;
                }

                let result = if from_remote {
                    clone_from_remote(&path, target_path)
                } else {
                    clone_from_local(&path, target_path)
                };

                match result {
                    Ok(_) => {
                        println!("Successfully cloned {}", project_name);
                        success_count.fetch_add(1, Ordering::SeqCst);
                    }
                    Err(e) => {
                        eprintln!("Failed to clone {}: {}", project_name, e);
                        failure_count.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
        }
    });

    if !repos_found.load(Ordering::SeqCst) {
        println!("No Git repositories found in the current directory.");
        exit(0);
    }

    println!(
        "Cloning complete: {} succeeded, {} failed.",
        success_count.load(Ordering::SeqCst),
        failure_count.load(Ordering::SeqCst)
    );
}

fn clone_from_remote(repo_path: &Path, target_dir: &Path) -> Result<(), git2::Error> {
    let repo = Repository::open(repo_path)?;
    let remote = repo.find_remote("origin")?;
    let url = remote
        .url()
        .ok_or_else(|| git2::Error::from_str("No remote URL"))?;

    let project_name = repo_path.file_name().unwrap();
    let target_path = target_dir.join(project_name);

    println!("Cloning from {} to {}", url, target_path.display());

    Repository::clone(url, &target_path)?;

    Ok(())
}

fn clone_from_local(repo_path: &Path, target_dir: &Path) -> Result<(), git2::Error> {
    let project_name = repo_path.file_name().unwrap();
    let target_path = target_dir.join(project_name);

    println!(
        "Cloning from {} to {}",
        repo_path.display(),
        target_path.display()
    );

    // Initialize a new bare repository at the target location
    let mut opts = RepositoryInitOptions::new();
    opts.bare(true);

    let new_repo = Repository::init_opts(&target_path, &opts)?;

    // Open the source repository
    let src_repo = Repository::open(repo_path)?;

    // Copy all references from the source to the target
    for reference in src_repo.references()? {
        let reference = reference?;
        let name = reference.name().unwrap();
        let target = reference.target().unwrap();
        new_repo.reference(name, target, true, "Mirror reference")?;
    }

    Ok(())
}
