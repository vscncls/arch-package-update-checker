use notify_rust::Notification;
use std::process::{exit, Command};

const MESSAGE_SIZE_LIMIT: usize = 15;

fn main() {
    let output = match Command::new("pacman").arg("-Qu").output() {
        Ok(output) => output,
        Err(error) => {
            eprintln!("Error calling pacman: {}", error);
            exit(1);
        }
    };

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");

    let pening_updates_raw: Vec<&str> = stdout.lines().collect();
    let updates = pening_updates_raw.len();

    let mut program_names = pening_updates_raw
        .iter()
        .filter(|line| line.len() > 0)
        .map(|line| line.split(" ").into_iter().next().unwrap())
        .take(MESSAGE_SIZE_LIMIT)
        .fold(String::new(), |acc, s| {
            if acc.is_empty() {
                s.to_string()
            } else {
                format!("{}, {}", acc, s)
            }
        });

    if updates > MESSAGE_SIZE_LIMIT {
        program_names = format!("{}...", program_names)
    }

    Notification::new()
        .summary(&format!("{} updates available", updates))
        .body(&program_names)
        .show()
        .expect("Failed creating notification");

    println!("{} pending updates", updates);
}
