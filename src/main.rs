use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        println!("Welcome to Git Wizard!");
        println!("1. List branches");
        println!("2. Create a new branch");
        println!("3. Add changes");
        println!("4. Commit changes");
        println!("5. Push current branch");
        println!("6. Merge current branch into develop");
        println!("7. Merge current branch into main");
        println!("8. Show git status");
        println!("9. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read input");
        let option = option.trim();

        match option {
            "1" => list_branches(),
            "2" => create_branch(),
            "3" => add_changes(),
            "4" => commit_changes(),
            "5" => {
                if let Some(branch_name) = get_current_branch() {
                    push_branch(&branch_name);
                }
            }
            "6" => merge_branch("develop"),
            "7" => merge_branch("main"),
            "8" => show_git_status(),
            "9" => {
                println!("Exiting Git Wizard...");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}

fn list_branches() {
    let branches = Command::new("git")
        .arg("branch")
        .output()
        .expect("Failed to list branches");

    println!("{}", String::from_utf8_lossy(&branches.stdout));
}

fn create_branch() {
    print!("Enter the new branch name: ");
    io::stdout().flush().unwrap();

    let mut branch_name = String::new();
    io::stdin()
        .read_line(&mut branch_name)
        .expect("Failed to read line");

    let branch_name = branch_name.trim();

    let output = Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(branch_name)
        .output()
        .expect("Failed to create branch");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn add_changes() {
    let add = Command::new("git")
        .arg("add")
        .output()
        .expect("Failed to add changes");

    println!("{}", String::from_utf8_lossy(&add.stdout));
}

fn commit_changes() {
    print!("Enter the commit message: ");
    io::stdout().flush().unwrap();

    let mut commit_message = String::new();
    io::stdin()
        .read_line(&mut commit_message)
        .expect("Failed to read line");

    let commit_message = commit_message.trim();

    let commit = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()
        .expect("Failed to commit changes");

    println!("{}", String::from_utf8_lossy(&commit.stdout));
}

fn push_branch(branch_name: &str) {
    let push = Command::new("git")
        .arg("push")
        .arg("--set-upstream")
        .arg("origin")
        .arg(branch_name)
        .output()
        .expect("Failed to push branch");

    println!("{}", String::from_utf8_lossy(&push.stdout));
}

fn merge_branch(target_branch: &str) {
    let merge = Command::new("git")
        .arg("merge")
        .arg(target_branch)
        .output()
        .expect("Failed to merge branch");

    println!("{}", String::from_utf8_lossy(&merge.stdout));
}

fn show_git_status() {
    let status = Command::new("git")
        .arg("status")
        .output()
        .expect("Failed to show git status");

    const RED: &str = "\u{001b}[31m";
    const RESET: &str = "\u{001b}[0m";
    println!(
        "{}{}{}",
        RED,
        String::from_utf8_lossy(&status.stdout),
        RESET
    );

    let stdout = String::from_utf8_lossy(&status.stdout);
    let stderr = String::from_utf8_lossy(&status.stderr);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
}

fn get_current_branch() -> Option<String> {
    let current_branch = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("Failed to get current branch");

    let branch_name = String::from_utf8_lossy(&current_branch.stdout)
        .trim()
        .to_string();

    if !branch_name.is_empty() {
        Some(branch_name)
    } else {
        println!("Failed to retrieve the current branch.");
        None
    }
}
