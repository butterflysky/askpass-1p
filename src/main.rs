use std::env;
use std::io::{Write};
use std::process::{Command, Stdio};
use serde::Deserialize;

#[derive(Deserialize)]
struct Item {
    id: String,
    title: String,
}

fn main() {
    let prompt = env::args().nth(1).unwrap_or_default();

    // Fetch list of Login items from 1Password
    let op_list = Command::new("op")
        .args(&["item", "list", "--categories", "Login", "--format", "json"])
        .output()
        .expect("Failed to run op item list");
    if !op_list.status.success() {
        eprintln!("op item list failed, ensure you're signed in");
        std::process::exit(1);
    }

    let items_json = String::from_utf8_lossy(&op_list.stdout);
    let items: Vec<Item> = serde_json::from_str(&items_json).expect("Failed to parse JSON from op");

    // Prepare just the titles for rofi
    let titles: Vec<&str> = items.iter().map(|item| item.title.as_str()).collect();

    // Run rofi with titles on stdin, returning an index
    let mut rofi_child = Command::new("rofi")
        .args(&["-dmenu", "-i", "-p", &prompt, "-format", "i"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn rofi");

    {
        let rofi_stdin = rofi_child.stdin.as_mut().expect("Failed to open rofi stdin");
        for title in &titles {
            writeln!(rofi_stdin, "{}", title).expect("Failed writing to rofi stdin");
        }
    }

    let rofi_output = rofi_child.wait_with_output().expect("Failed to read rofi output");
    if !rofi_output.status.success() {
        // User likely cancelled selection
        std::process::exit(1);
    }

    let selected_index_str = String::from_utf8_lossy(&rofi_output.stdout).trim().to_string();
    if selected_index_str.is_empty() {
        // User cancelled
        std::process::exit(1);
    }

    let selected_index: usize = selected_index_str.parse().expect("Invalid index from rofi");
    if selected_index >= items.len() {
        eprintln!("Selected index out of range");
        std::process::exit(1);
    }

    let selected_id = &items[selected_index].id;

    // Determine which field to get based on prompt
    let field = if prompt.to_lowercase().contains("username") {
        "username"
    } else {
        "password"
    };

    let op_get = Command::new("op")
        .args(&["item", "get", selected_id, "--fields", field, "--reveal"])
        .output()
        .expect("Failed to run op item get");
    if !op_get.status.success() {
        eprintln!("Failed to retrieve {} from op item get", field);
        std::process::exit(1);
    }

    let field_value = String::from_utf8_lossy(&op_get.stdout).trim().to_string();
    println!("{}", field_value);
}
