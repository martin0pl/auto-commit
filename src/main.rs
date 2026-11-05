use chrono::Local;
use std::process::Command;

fn main() {
    let timestamp = Local::now().format("%Y-%m-%d-%H-%M-%S").to_string();
    let message = format!("Auto-commit: {}", timestamp);

    // git add .
    let add = Command::new("git")
        .args(["add", "."])
        .status()
        .expect("Échec de git add");

    if !add.success() {
        eprintln!("❌ git add a échoué");
        std::process::exit(1);
    }

    // git commit -m "Auto-commit: YYYY-MM-DD-HH-MM-SS"
    let commit = Command::new("git")
        .args(["commit", "-m", &message])
        .status()
        .expect("Échec de git commit");

    if commit.success() {
        println!("✅ {}", message);
    } else {
        eprintln!("❌ git commit a échoué (rien à committer ?)");
        std::process::exit(1);
    }
}