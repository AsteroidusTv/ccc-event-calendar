use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{self, Read, Seek};
use crate::read::read_json;

#[derive(Serialize, Deserialize)]
pub struct Event {
    name: String,
    date: String,
    description: String,
}

fn add_content(file_path: &str, new_content: Event) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    let mut existing_content = String::new();
    file.read_to_string(&mut existing_content)?;

    let mut existing_events: Vec<Event> = if existing_content.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&existing_content)?
    };

    existing_events.push(new_content);
    file.seek(std::io::SeekFrom::Start(0))?; 
    file.set_len(0)?;
    serde_json::to_writer(&file, &existing_events)?;

    Ok(())
}

pub fn verify_json_content(proprety_to_find: String) -> bool {
    let json = read_json();
    let found_element = json.iter().find(|&x| x.name.to_lowercase() == proprety_to_find.to_lowercase());

    match found_element {
        Some(_) => true,
        None => false,
    }
} 

pub fn write_json(name: String, date: String, description: String) {
    let new_content = Event {
        name,
        date,
        description,
    };

    if let Err(err) = add_content("./test.json", new_content) {
        eprintln!("Error: {:?}", err);
    } else {
        println!("Success");
    }
}
