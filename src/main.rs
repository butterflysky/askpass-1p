use inquire::Select;
use serde::Deserialize;
use std::env;
use std::process::{exit, Command}; 

#[derive(Debug, Deserialize)]
struct OpItem {
    title: String,
    id: String,
}

fn main() {
    // Check argument count early and exit if invalid
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <prompt>", args[0]);
        exit(1);
    }
    let prompt = &args[1];

    let selected_item_id = fetch_op_item(&prompt);
    let field_value = fetch_selected_field_value(&selected_item_id);

    // Step 3: Output the field value (e.g., username, password)
    println!("{}", field_value);
}

fn fetch_op_item(prompt: &str) -> String {
    let items = list_op_items();
    if items.is_empty() {
        eprintln!("No items found in 1Password.");
        exit(1);
    }

    let titles: Vec<String> = items.iter().map(|item| item.title.clone()).collect();

    // Use inquire::Select to display a selectable list
    let selected_title = Select::new(&prompt, titles.clone())
        .prompt()
        .expect("Failed to display selection prompt.");

    // Match the selected title back to its corresponding ID
    items
        .into_iter()
        .find(|item| item.title == selected_title)
        .map(|item| item.id)
        .unwrap_or_else(|| {
            eprintln!("Failed to map selection back to item ID.");
            exit(1);
        })
}

/// Fetch the list of fields for a 1Password item
fn fetch_fields_for_item(op_item_id: &str) -> Vec<String> {
    let output = Command::new("op")
        .arg("item")
        .arg("get")
        .arg(op_item_id)
        .arg("--format=json")
        .output()
        .unwrap_or_else(|_| {
            eprintln!("Failed to fetch 1Password item fields.");
            exit(1);
        });

    if !output.status.success() {
        eprintln!(
            "Error: Failed to get item fields: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }

    let json_output = String::from_utf8(output.stdout).expect("Invalid UTF-8 in 1Password output.");

    // Parse fields from the JSON response
    let parsed: serde_json::Value = serde_json::from_str(&json_output).expect("Invalid JSON output.");
    let fields = parsed["fields"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|field| field["label"].as_str().map(|s| s.to_string()))
        .collect();

    fields
}

/// Allow the user to select a field from the 1Password entry
fn fetch_selected_field_value(op_item_id: &str) -> String {
    let fields = fetch_fields_for_item(op_item_id);

    if fields.is_empty() {
        eprintln!("No fields available for the selected item.");
        exit(1);
    }

    let selected_field = Select::new("Select a field to fetch", fields)
        .prompt()
        .expect("Failed to display field selection prompt.");

    // Fetch the value for the selected field
    let output = Command::new("op")
        .arg("item")
        .arg("get")
        .arg(op_item_id)
        .arg("--field")
        .arg(selected_field)
        .arg("--reveal")
        .output()
        .unwrap_or_else(|_| {
            eprintln!("Failed to fetch the selected field value.");
            exit(1);
        });

    if !output.status.success() {
        eprintln!(
            "Error: Failed to fetch field value: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }

    String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 in field value.")
        .trim()
        .to_string()
}

/// Fetch the list of 1Password items
fn list_op_items() -> Vec<OpItem> {
    let output = Command::new("op")
        .arg("item")
        .arg("list")
        .arg("--format=json")
        .output()
        .unwrap_or_else(|_| {
            eprintln!(
                "Failed to list 1Password items. Ensure 'op' CLI is installed and authenticated."
            );
            exit(1);
        });

    if output.status.success() {
        let items_json =
            String::from_utf8(output.stdout).expect("Invalid UTF-8 in 1Password output.");
        serde_json::from_str(&items_json).expect("Failed to parse 1Password items JSON.")
    } else {
        eprintln!(
            "Error: Failed to fetch 1Password items: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }
}
