use std::process::Command;

pub fn show_git_status() -> Result<String, String> {
    let output = Command::new("git").arg("status").output();

    match output {
        Ok(status) => {
            let output_str = String::from_utf8_lossy(&status.stdout).to_string();
            Ok(output_str)
        }
        Err(e) => Err(format!("Error ejecutando git status: {}", e)),
    }
}

pub fn create_branch(branch_name: &str) -> Result<String, String> {
    let output = Command::new("git")
        .args(&["checkout", "-b", branch_name])
        .output();

    match output {
        Ok(_) => Ok(format!("Branch '{}' created successfully.", branch_name)),
        Err(e) => Err(format!("Error creating branch '{}': {}", branch_name, e)),
    }
}

pub fn merge_with_develop() -> Result<String, String> {
    let output = Command::new("git").args(&["checkout", "develop"]).output();

    if output.is_err() {
        return Err("Error checking out to develop branch.".to_string());
    }

    let output = Command::new("git")
        .args(&["merge", "--no-ff", "-m", "Merge branch 'develop'"])
        .output();

    match output {
        Ok(_) => Ok("Successfully merged with develop.".to_string()),
        Err(e) => Err(format!("Error merging with develop: {}", e)),
    }
}
