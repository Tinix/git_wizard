use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        // Mostrar el menú
        println!("Welcome to Git Wizard!");
        println!("1. List branches");
        println!("2. Create a new branch");
        println!("3. Add changes");
        println!("4. Commit changes");
        println!("5. Push current branch");
        println!("6. Merge current branch into develop");
        println!("7. Merge current branch into main");
        println!("8. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap(); // Limpia la línea para que el prompt aparezca correctamente

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read input");
        let option = option.trim();

        match option {
            "1" => list_branches(),
            "2" => create_branch(),
            "3" => {
                if let Some(branch_name) = get_current_branch() {
                    push_branch(&branch_name);
                }
            }
            "4" => merge_branch("develop"),
            "5" => {
                println!("Exiting Git Wizard...");
                break; // Salir del bucle
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
