use std::process::Command;

fn main() {
    println!("Generating commit");
    let diff = match git_diff() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error getting diff: {e}");
            return;
        }
    };
    let commit_message = match get_commit_from_ai(diff) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error generating commit: {e}");
            return;
        }
    };
    commit(commit_message);
}

fn git_diff() -> Result<String, String> {
    let output = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .expect("Failed to execute git diff command");

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from("Error or no changes"))
    }
}

fn get_commit_from_ai(diff: String) -> Result<String, String> {
    let prompt = format!(
        "Output ONLY the commit message in Conventional Commits format (e.g., 'feat: add logic' or 'fix: resolve bug'). \
    No explanations, no 'Thinking...', no markdown. Just the message for this diff: {}",
        diff
    );

    let output = Command::new("ollama")
        .args(["run", "gemma4:e2b", &prompt])
        .output()
        .expect("Failed to execute ollama command");

    if output.status.success() {
        // Wrap the successful string in Ok()
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        // Wrap the failure message in Err()
        Err(String::from("Ollama returned an error code"))
    }
}

fn commit(commit_message: String) {
    let output = Command::new("git")
        .args(["commit", "-m", &commit_message])
        .output()
        .expect("Failed to execute commit command");

    if output.status.success() {
        println!("{}", commit_message);
    }
}
