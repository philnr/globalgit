use std::env;
use std::path::Path;
use std::process::{Command, Stdio};
use walkdir::WalkDir;
use std::cmp::Reverse;
use chrono::{Utc, TimeZone};
use colored::*;
use std::io::Write;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <repos|log> <root_directory>", args[0]);
        return;
    }

    let mode = &args[1];
    let root_dir = &args[2];

    match mode.as_str() {
        "repos" => list_repos(root_dir),
        "log" => print_combined_log(root_dir),
        _ => eprintln!("Invalid mode. Use 'repos' or 'log'."),
    }
}

/// Lists all git repositories in the given root directory.
fn list_repos(root_dir: &str) {
    let mut walker = WalkDir::new(root_dir).into_iter();
    while let Some(entry) = walker.next() {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_dir() {
                    let path = entry.path();
                    if is_git_repo(path) {
                        println!("Found git repo {}", get_repo_name(path));
                        walker.skip_current_dir(); // Skip the current directory
                    }
                }
            }
            Err(_) => continue, // Ignore errors like permission issues
        }
    }
}

/// Prints the combined git log of all git repositories in the given root directory.
fn print_combined_log(root_dir: &str) {
    let mut walker = WalkDir::new(root_dir).into_iter();
    let mut log_entries = Vec::new();

    while let Some(entry) = walker.next() {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_dir() {
                    let path = entry.path();
                    if is_git_repo(path) {
                        let repo_name = get_repo_name(path);
                        let output = Command::new("git")
                            .arg("-C")
                            .arg(path)
                            .arg("log")
                            .arg("--pretty=format:%h %ct %s")
                            .output()
                            .expect("Failed to execute git command");

                        let logs = String::from_utf8_lossy(&output.stdout);
                        for line in logs.lines() {
                            let parts: Vec<&str> = line.splitn(3, ' ').collect();
                            if parts.len() == 3 {
                                let timestamp: i64 = parts[1].parse().unwrap_or(0);
                                log_entries.push((timestamp, parts[0].to_string(), parts[2].to_string(), repo_name.clone()));
                            }
                        }
                        walker.skip_current_dir(); // Skip the current directory since git repo has already been found
                    }
                }
            }
            Err(_) => continue, // Ignore errors like permission issues
        }
    }

    // Sort log entries by timestamp in descending order and take the latest 10
    log_entries.sort_by_key(|entry| Reverse(entry.0));
    // Create a command to pipe the output to `less`
    let mut less = Command::new("less")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start less");
    {
        let stdin = less.stdin.as_mut().expect("Failed to open stdin");
        for (timestamp, hash, message, repo_name) in log_entries {
            let datetime = Utc.timestamp_opt(timestamp, 0).single().expect("Invalid timestamp");
            let formatted_date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
            writeln!(
                stdin,
                "{} {} {} \n{}",
                formatted_date.truecolor(114, 135, 253), 
                hash.truecolor(92, 95, 119),           
                repo_name.truecolor(254, 100, 11),     
                message.truecolor(255, 255, 224)       
            ).expect("Failed to write to less");    
        }
    }
    
}

/// Checks if the given path is a Git repository.
fn is_git_repo(path: &Path) -> bool {
    path.join(".git").exists()
}

/// Retrieves the name of the Git repository (i.e., the name of the directory).
fn get_repo_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str()) // Get the name of the directory
        .unwrap_or("Unknown").to_string() // If it's not valid, return "Unknown"
}