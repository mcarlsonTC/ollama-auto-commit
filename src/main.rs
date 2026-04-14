mod models; // Links your new models.rs file to the project

use models::{OllamaOptions, OllamaRequest, OllamaResponse};
use std::process::Command;

fn main() {
    println!("Generating_____________");

    let diff = match git_diff() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error getting diff: {}", e);
            return;
        }
    };

    // Quick safety check so we don't send an empty prompt to the AI
    if diff.trim().is_empty() {
        eprintln!("No staged changes found. Did you forget to 'git add'?");
        return;
    }

    let commit_message = match get_commit_from_ai(diff) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error generating commit: {}", e);
            return;
        }
    };

    println!("{}", commit_message);
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
        Err(String::from("Git returned an error code or no changes"))
    }
}

fn get_commit_from_ai(diff: String) -> Result<String, String> {
    let prompt = format!(
        "Create a commit message. Output ONLY the commit message in Conventional Commits format (e.g., 'feat: add logic'). \
        No explanations, no thinking, no markdown. Diff: {}",
        diff
    );

    let request_data = OllamaRequest {
        model: String::from("gemma4:e4b"),
        prompt,
        stream: false,
        keep_alive: 0, // Dumps the 9.6GB model immediately
        options: OllamaOptions { temperature: 0.0 }, // Zero creativity
    };

    let payload = match serde_json::to_string(&request_data) {
        Ok(json) => json,
        Err(_) => return Err(String::from("Failed to serialize request")),
    };

    let output = Command::new("curl")
        .args([
            "-s", // Silent mode
            "-X",
            "POST",
            "http://localhost:11434/api/generate",
            "-d",
            &payload,
        ])
        .output()
        .expect("Failed to execute curl command");

    if output.status.success() {
        let raw_json = String::from_utf8_lossy(&output.stdout);

        match serde_json::from_str::<OllamaResponse>(&raw_json) {
            Ok(parsed) => Ok(parsed.response.trim().to_string()),
            Err(_) => Err(String::from("Failed to parse Ollama JSON response")),
        }
    } else {
        Err(String::from("Ollama API call failed"))
    }
}

fn commit(commit_message: String) {
    let output = Command::new("git")
        .args(["commit", "-m", &commit_message])
        .output()
        .expect("Failed to execute commit command");

    if output.status.success() {
        println!("_____________________");
    } else {
        // If git fails (e.g., pre-commit hooks block it), print the error
        eprintln!(
            "Git commit failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
